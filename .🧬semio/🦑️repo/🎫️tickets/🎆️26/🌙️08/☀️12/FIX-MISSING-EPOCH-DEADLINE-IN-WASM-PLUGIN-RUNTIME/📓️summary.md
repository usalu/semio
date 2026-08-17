# Summary: Fix Missing Epoch Deadline in WasmPluginRuntime

## Problem Statement

In `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`, `WasmPluginRuntime::build_engine()` configures Wasmtime with `config.epoch_interruption(true)`. However, `WasmPluginRuntime` never called `Store::set_epoch_deadline` anywhere, nor called `prepare_call` on fresh stores created in `read_manifest` or `instantiate`.

A fresh Wasmtime `Store` defaults to an epoch deadline of `0` and fuel of `0`. When epoch interruption is enabled without setting an epoch deadline, any Wasm call execution hitting an epoch checkpoint traps immediately with `wasm trap: interrupt`.

## Changes Made

1. **`WasmPluginRuntime::prepare_call`**:
   - Updated `prepare_call(&mut Store<HostState>)` to set `store.set_epoch_deadline(u64::MAX)` alongside `store.set_fuel(PLUGIN_FUEL_BUDGET)`.
   - Matches the fix applied to `ExtensionRuntime::prepare_call` in ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

2. **Wasm Execution Call Paths**:
   - Added `Self::prepare_call(&mut store)` inside `read_manifest` before `PluginWorld::instantiate` and before `call_manifest`.
   - Added `Self::prepare_call(&mut store)` inside `instantiate` before `PluginWorld::instantiate`.
   - Ensured all existing call paths (`create_app`, `exchange`, `list_artifact_dialects`, `artifact_compose`, `migrate_artifact`, `clear_instance_guard`) continue to run `prepare_call`.

3. **Test Extension**:
   - Added `wasm_plugin_runtime_loads_real_plugin_component_if_present` to `mod tests` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`, verifying loading a real plugin wasm component (`puzzle`) succeeds end-to-end without fuel or epoch interruption traps.

## Verification

- Executed `cargo test -p semio-framework-plugin-host`.
- All 3 tests passed cleanly (`wasm_plugin_runtime_api_exists`, `extension_runtime_constructs_engine_and_linker`, `wasm_plugin_runtime_loads_real_plugin_component_if_present`).

## Files Touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/FIX-MISSING-EPOCH-DEADLINE-IN-WASM-PLUGIN-RUNTIME/📓️research.md`
- `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/FIX-MISSING-EPOCH-DEADLINE-IN-WASM-PLUGIN-RUNTIME/📓️summary.md`
