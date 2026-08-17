# W5B — Extension-World Real Round-Trip Prototype

## Verdict

**Full success, after fixing three genuine bugs.** A real `cargo build --target wasm32-wasip2` compiled `extension-world` component, loaded through the actual `ExtensionRuntime` (native `wasmtime`), round-trips correctly: `manifest()` → `activate()` → `invoke("add", {"a":19,"b":23})` → `{"sum":42}`, plus correct fault propagation for an unknown capability. The mechanism was **not** working end-to-end before this session — it type-checked but traps/faults on the very first real call, for three independent reasons detailed below. All three are fixed directly in the framework files (in scope per this ticket's explicit "fix a genuine bug if confident" carve-out), and the fixes are verified both by this prototype and by the pre-existing `semio-framework-plugin-host` test suite (`cargo test -p semio-framework-plugin-host extension_runtime` still passes).

## What was built (scratch crate, this ticket folder only)

```
w5b-extension-prototype/
  Cargo.toml              — virtual workspace, isolated ([workspace] table), NOT a root workspace member
  guest/                  — package "w5b-extension-echo", crate-type=["cdylib"]
    Cargo.toml             — depends on semio-framework-plugin (feature "component-extension-guest") + semio-framework
    src/lib.rs              — simplest possible real extension: one capability "add", sums two i64s from JSON
  host_test/              — package "w5b-extension-host-test"
    Cargo.toml              — depends on semio-framework-plugin-host (real ExtensionRuntime) + wasmtime/wasmtime-wasi
    src/main.rs              — the actual round-trip proof: ExtensionRuntime::load() → manifest() → extension_invoke()
    src/bin/debug.rs         — diagnostic-only harness (raw wasmtime::component::bindgen!, inherit_stdio, full error Debug) used to localize each trap; not part of the proof, kept for reproducibility
```

Build commands used (both succeed cleanly):
```
cargo build -p w5b-extension-echo --target wasm32-wasip2 --release
cargo run   -p w5b-extension-host-test --release
```

The compiled artifact `target/wasm32-wasip2/release/w5b_extension_echo.wasm` (421 KB) is a genuine **component**, confirmed by its binary header `00 61 73 6d 0d 00 01 00` — version `0x0001000d` is the Component Model marker (a plain core module would be version `0x00000001`). `cargo-component` was not actually needed: `wit-bindgen` 0.36's `generate!`/`export!` macros, targeting `wasm32-wasip2` with the bundled `wasm-component-ld` linker (ships inside the rustup toolchain), produce a real component directly — no `wasm-tools component new` post-processing step required.

## Final passing run (real `ExtensionRuntime`, not the debug harness)

```
[w5b] building ExtensionRuntime (engine + linker)...
[w5b] loading component: .../target/wasm32-wasip2/release/w5b_extension_echo.wasm
[w5b] loaded extension_id = "w5b.echo"
[w5b] manifest = ExtensionManifest { extension_id: "w5b.echo", label: "W5B Echo Extension", version: "0.1.0", extends: "w5b.host", capabilities: [], topic_contributions: [] }
[w5b] invoking capability "add" with request = "{\"a\":19,\"b\":23}"
[w5b] invoke result = {"sum":42}
[w5b] invoking unknown capability "nope" (expect a fault, not a panic)...
[w5b] unknown-capability fault = Fault { origin: Plugin, code: FaultCode("extension.unknown-capability"), severity: Error, message: "unknown extension capability 'nope'", ... }
[w5b] ALL ROUND-TRIP ASSERTIONS PASSED — extension-world ABI works end-to-end with a real compiled component.
```

## The three bugs found, each isolated by direct reproduction (not guessed)

Diagnosis method: `host_test/src/bin/debug.rs` reimplements the same `wasmtime::component::bindgen!({world: "extension-world", ...})` call by hand, with `inherit_stdio()` and `Config::wasm_backtrace_details(Enable)`, so trap causes are visible instead of being collapsed into `PluginHostError::Wasmtime(error.to_string())`'s single-line message. Config knobs were then toggled one at a time (fuel off → on, epoch off → on) to attribute each observed trap to its exact cause before touching any framework file.

### Bug 1 — `ExtensionRuntime::load_bytes` never sets store fuel before its first two wasm calls
File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`, `ExtensionRuntime::load_bytes` / `unload`.

`build_engine()` sets `config.consume_fuel(true)`. A fresh `wasmtime::Store` defaults to **zero** fuel when fuel consumption is enabled, so the very first wasm instruction traps with `all fuel consumed by WebAssembly`. `extension_invoke` already called `store.set_fuel(PLUGIN_FUEL_BUDGET)` before its call — but `load_bytes` (which calls `manifest()` then `activate()`) and `unload` (which calls `deactivate()`) never did. Reproduced directly: with `consume_fuel(true)` and no `set_fuel`, `manifest()` traps with exactly `wasm trap: all fuel consumed by WebAssembly`.

### Bug 2 — `epoch_interruption(true)` is enabled but no epoch deadline is EVER set, anywhere
Same file/struct. `build_engine()` also sets `config.epoch_interruption(true)`. A fresh `Store`'s epoch deadline defaults to `0`, and nothing anywhere in this repository calls `Store::set_epoch_deadline` or `Engine::increment_epoch` (`grep -rn "set_epoch_deadline|increment_epoch"` across the whole repo returns zero hits, in *either* `ExtensionRuntime` or the parallel `WasmPluginRuntime`). With fuel fixed but no deadline set, the exact same call traps with `wasm trap: interrupt` instead — reproduced directly. `WasmPluginRuntime` (the `plugin-world` counterpart, same file) has the structurally identical gap; it is *not* touched by this fix since it's outside this ticket's assigned surface (`ExtensionRuntime` only) — flagged below as a related follow-up.

**Fix applied** (both bugs, one helper): added `ExtensionRuntime::prepare_call(&mut Store<ExtensionHostState>)` — sets `PLUGIN_FUEL_BUDGET` fuel and `u64::MAX` epoch deadline (nothing increments the engine epoch anywhere, so `u64::MAX` is "never interrupt" rather than a real cooperative budget, matching the codebase's current de-facto behavior once it isn't broken) — and call it before every wasm-executing call: in `load_bytes` (before `instantiate`+`manifest()`, and again before `activate()`), in `unload` (before `deactivate()`), and in `extension_invoke` (replacing its previous bare `set_fuel` line).

### Bug 3 — the guest-side `ExtensionBundle` is never actually installed
File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`, `plugin_runtime::ensure_extension_initialized`.

The parallel `plugin-world` path (`ensure_plugin_initialized`) uses a documented weak-linkage trick: a `#[linkage = "weak"] extern "C" fn semio_plugin_bundle_installer_link_shim` no-op default lives in the shared framework crate; the concrete plugin crate's `plugin_exports!` macro provides a **strong** override of the same symbol name; `ensure_plugin_initialized` explicitly calls `semio_plugin_bundle_installer_link_shim()` (whichever definition won the link) to trigger `register_plugin_bundle_installer`, then reads the registered installer out of a `OnceLock` and runs it.

`ensure_extension_initialized` (the `extension-world` equivalent) copies the second half of this pattern — `extension_exports!` generates the identically-shaped `semio_extension_bundle_installer_link_shim` / `register_extension_bundle_installer` pair — but was missing **both** the weak default declaration *and* the explicit call to trigger it. `EXTENSION_BUNDLE_INSTALLER.get()` was therefore always `None`, so `extension_manifest()` silently fell through to the hardcoded empty-string `ExtensionManifest` default, and `extension_activate()` would then always fault `extension.missing` ("extension bundle not installed") — even though the extension crate had correctly declared its bundle via `extension_exports!`. Reproduced directly: before this fix, the debug harness's `manifest()` call (once fuel/epoch were separately worked around) returned wire bytes decoding to an all-empty `ExtensionManifest` (`extensionId`, `label`, `version`, `extends` all length-0 strings) instead of the guest crate's real `"w5b.echo"` / `"W5B Echo Extension"` / `"0.1.0"` / `"w5b.host"`.

**Fix applied**: added the missing `#[cfg(feature = "component-extension-guest")] #[unsafe(no_mangle)] #[linkage = "weak"] pub extern "C" fn semio_extension_bundle_installer_link_shim() {}` default, and the explicit `unsafe { semio_extension_bundle_installer_link_shim(); }` call inside `ensure_extension_initialized`, exactly mirroring `ensure_plugin_initialized`. Also extended `📦️glue.rs`'s `#![cfg_attr(..., feature(linkage))]` gate from `feature = "component-guest"` to `any(feature = "component-guest", feature = "component-extension-guest")`, since the new weak-linkage attribute needs the same nightly `feature(linkage)` opt-in the plugin path already required.

## Files touched (framework fixes, all three confirmed-bug fixes, no unrelated changes)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — `ExtensionRuntime::prepare_call` added; called from `load_bytes` (x2), `unload`, `extension_invoke`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` — `ensure_extension_initialized` now mirrors `ensure_plugin_initialized`'s weak-linkage shim-call pattern; new weak-default `semio_extension_bundle_installer_link_shim`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📦️glue.rs` — `feature(linkage)` cfg_attr extended to cover `component-extension-guest`.

Verified not to regress: `cargo check -p semio-framework-plugin -p semio-framework-plugin-host` (clean, only pre-existing unrelated warnings) and `cargo test -p semio-framework-plugin-host extension_runtime` (`extension_runtime_constructs_engine_and_linker` — pass).

## Scratch files (this ticket folder, kept per repo convention)

- `w5b-extension-prototype/` — the full scratch crate (guest + host_test), isolated `[workspace]`, does not join the root workspace.
- `w5b-extension-prototype/target/wasm32-wasip2/release/w5b_extension_echo.wasm` — the compiled proof artifact.

## Out-of-scope related finding (not fixed here — flagged only)

`WasmPluginRuntime` (same host file, the `plugin-world` counterpart to `ExtensionRuntime`) has the identical `epoch_interruption(true)`-without-`set_epoch_deadline` gap (its `prepare_call` sets fuel but not an epoch deadline, and `read_manifest`'s own initial `manifest()`/instantiate call path wasn't audited here). If `plugin-world` components are being loaded through wasmtime with actual epoch interruption engaged anywhere the same "trap: interrupt" on first call is likely reproducible — worth a dedicated pass, but it's a different struct than what this ticket's `ExtensionRuntime`-scoped assignment covers, so it was left untouched.
