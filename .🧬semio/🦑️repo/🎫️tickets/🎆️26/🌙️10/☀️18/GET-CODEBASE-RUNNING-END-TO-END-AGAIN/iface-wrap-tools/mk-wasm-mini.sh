#!/bin/bash
# Compiles the staged wasmtime port against the pinned wasmtime/wasmtime-wasi, runs the port tests, and expands the real `bindgen!` world through `generated_code_support` (cargo check). Throw-away crate under /private/tmp; never touches the repository.
set -eu
A="$(cd "$(dirname "$0")" && pwd)"
M=/private/tmp/iface-wrap/wasm-mini
rm -rf "$M"; mkdir -p "$M/src"
cat > "$M/Cargo.toml" <<TOML
[package]
name = "wasm-mini"
version = "0.1.0"
edition = "2021"
[workspace]
[dependencies]
wasmtime = { version = "47.0.3", features = ["cranelift", "component-model", "component-model-async"] }
wasmtime-wasi = "47.0.3"
TOML
WIT="$A/../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema"
cat > "$M/src/lib.rs" <<RS
#![allow(warnings)]
#[path = "$A/src/wasm/port.rs"]
pub mod wasm;
pub mod bindings {
    use crate::wasm::component::bindgen;
    bindgen!({
        world: "actor",
        path: "$(cd "$WIT" && pwd)",
        wasmtime_crate: crate::wasm::generated_code_support,
    });
}
pub fn minimal_component_without_actor_world() -> &'static [u8] { b"\0asm\x0d\0\x01\0" }
#[cfg(test)]
#[path = "$A/src/wasm/port-test.rs"]
mod wasm_port_tests;
RS
cd "$M"
CARGO_TARGET_DIR=/private/tmp/iface-wrap/target-nightly nice -n 10 cargo +nightly-2026-07-20 test --offline -j 2 2>&1 | grep -E "^test |test result|^error"
