# Research: Missing Epoch Deadline in WasmPluginRuntime

## Overview

In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`, `WasmPluginRuntime::build_engine()` enables epoch interruption (`config.epoch_interruption(true)`).
However, `WasmPluginRuntime::prepare_call` only calls `store.set_fuel(PLUGIN_FUEL_BUDGET).ok()` and never sets an epoch deadline via `store.set_epoch_deadline(...)`.
Furthermore, `read_manifest` and `instantiate` did not invoke `prepare_call` prior to instantiating or executing wasm calls on a fresh `Store`.

Since a fresh Wasmtime `Store` defaults to an epoch deadline of `0` and fuel of `0`, any Wasm call execution hitting an epoch checkpoint traps immediately with `wasm trap: interrupt` (or `all fuel consumed`).

## Sibling Prior Art (`ExtensionRuntime`)

In ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`, the sibling struct `ExtensionRuntime` (in the same file) encountered the exact same bug where `epoch_interruption(true)` caused WASM calls to trap with `wasm trap: interrupt`.
The bug was resolved in `ExtensionRuntime` by:
1. Creating `ExtensionRuntime::prepare_call(&mut Store)` which sets both fuel (`PLUGIN_FUEL_BUDGET`) and epoch deadline (`store.set_epoch_deadline(u64::MAX)`).
2. Invoking `prepare_call` prior to every WASM execution path in `ExtensionRuntime` (`load_bytes`, `unload`, `extension_invoke`).

## Identified Gaps in `WasmPluginRuntime`

1. `WasmPluginRuntime::prepare_call` missing `store.set_epoch_deadline(u64::MAX)`:
   - Needs `store.set_epoch_deadline(u64::MAX)` added beside `set_fuel`.
2. `WasmPluginRuntime::read_manifest` missing `prepare_call`:
   - `read_manifest` creates a fresh `Store` and immediately calls `PluginWorld::instantiate(&mut store, ...)` and `bindings.semio_framework_plugin().call_manifest(&mut store)`.
   - `prepare_call(&mut store)` should be called before `instantiate` and `call_manifest`.
3. `WasmPluginRuntime::instantiate` missing `prepare_call`:
   - `instantiate` receives a fresh `Store` and calls `PluginWorld::instantiate(&mut store, ...)` which can execute WASM start/init code.
   - `prepare_call(&mut store)` should be called inside `instantiate`.

## Verification & Resolution Plan

1. Verify runtime behavior with a real plugin component before fix.
2. Update `WasmPluginRuntime::prepare_call` and all WASM call sites in `WasmPluginRuntime`.
3. Add a unit test in `mod tests` of `component.rs` exercising `WasmPluginRuntime::load` and WASM calls.
4. Verify tests pass cleanly using `cargo test -p semio-framework-plugin-host`.
