mod utils;

use rvemu_core::bus::DRAM_BASE;
use rvemu_core::emulator;

use wasm_bindgen::prelude::*;

/*
#[wasm_bindgen(module = "/public/input.js")]
extern "C" {
fn write_to_buffer(byte: u8);
}
*/

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Wrapper for rvemu::emulator::Emulator to connect to WebAssembly.
#[wasm_bindgen]
pub struct Emulator {
    emu: emulator::Emulator,
}

/// Output a message to the emulator console.
pub fn stdout(message: &str) {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let buffer = document
        .get_element_by_id("buffer")
        .expect("should have a element with a `buffer` id");

    let span = document
        .create_element("span")
        .expect("span element should be created successfully");
    span.set_inner_html(message);
    let result = buffer.append_child(&span);
    if result.is_err() {
        panic!("can't append a span node to a buffer node")
    }
}

/// Output a message to both the browser console and the emulator console.
pub fn stdout_log(message: &str) {
    log(message);
    //stdout(message);
    //for c in message.chars() {
     //   write_to_buffer(c as u8);
    //}
}

#[wasm_bindgen]
impl Emulator {
    /// Constructor for the emulator.
    pub fn new() -> Emulator {
        // Initialize for putting error messages to a browser console.
        utils::set_panic_hook();

        Self {
            emu: emulator::Emulator::new(),
        }
    }

    /// Reset the emulator.
    pub fn reset(&mut self) {
        self.emu.reset();
    }

    /// Set binary data to the beginning of the DRAM from the emulator console of a browser.
    pub fn set_dram(&mut self, data: Vec<u8>) {
        self.emu.set_dram(data);
    }

    /// Set binary data to the virtio disk from the emulator console.
    pub fn set_disk(&mut self, data: Vec<u8>) {
        self.emu.set_disk(data);
    }

    /// Start executing.
    pub fn start(&mut self) {
        self.emu.set_pc(DRAM_BASE);
        self.emu.start();
    }

    /// Output current registers.
    pub fn dump_registers(&self) {
        stdout_log(&format!("{}", self.emu.cpu.xregs));
        stdout_log(&format!(
            "-------------------------------------------------------------------------------------------"
        ));
        stdout_log(&format!("{}", self.emu.cpu.fregs));
        stdout_log(&format!(
            "-------------------------------------------------------------------------------------------"
        ));
        stdout_log(&format!("{}", self.emu.cpu.state));
        stdout_log(&format!(
            "-------------------------------------------------------------------------------------------"
        ));
        stdout_log(&format!("pc: {}", self.emu.cpu.pc));
    }
}
