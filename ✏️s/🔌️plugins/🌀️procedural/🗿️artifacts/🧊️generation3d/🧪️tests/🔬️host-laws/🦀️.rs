//! 🧪️ Host laws: retained publication authority, envelope decode and store initialization, and the guest-memory footprint of the contributions install path.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../🔨️modules/🏠️host/🧪️tests/🔬️retained-authority-laws/🦀️.rs"]
mod retained_authority_laws;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../📈️guest-memory/🦀️.rs"]
mod guest_memory_tests;
