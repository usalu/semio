# W2 — Wire Up `extension-world` Progress

Started: 2026-08-11

## Task
Wire up the declared-but-unused WIT `extension-world` so extensions can be instantiated as their
own wasm components, instead of only ever piggybacking on `plugin-world` components (the previous
workaround). Additive wave — nothing wired into any boot sequence yet.

File ownership for this task:
- `🔌️plugin/📦️packages/🦀️rust/Cargo.toml`
- `🔌️plugin/🦀️component.rs` (guest SDK)
- `🔌️plugin/🖥️host/🦀️component.rs` (native wasmtime host)
- `🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit` (read-only reference)

## 1. WIT confirmed
`📜️wit/📜️world.wit` already declares:
```
interface extension {
  use types.{plugin-error};
  manifest: func() -> list<u8>;
  activate: func() -> result<_, plugin-error>;
  deactivate: func();
  invoke: func(capability: string, request: list<u8>) -> result<list<u8>, plugin-error>;
}
world extension-world {
  import host;
  export extension;
}
```
No edits made to this file (read-only per task).

## 2. Guest side — `🔌️plugin/🦀️component.rs`

### New feature flag
`component-extension-guest = []` added to `🔌️plugin/📦️packages/🦀️rust/Cargo.toml` `[features]`,
alongside the existing `component-guest = []`.

### Mutual exclusion guard
Added near the top of the file:
```rust
#[cfg(all(feature = "component-guest", feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2"))]
compile_error!("`component-guest` and `component-extension-guest` are mutually exclusive for wasm32-wasip2 targets");
```
Verified it actually fires: `cargo check -p semio-framework-plugin --features component-guest,component-extension-guest --target wasm32-wasip2` errors with exactly this message (confirmed, then re-ran with only `component-extension-guest` for the real verification below).

### Second `wit_bindgen::generate!` invocation
Added `pub mod extension_component { ... }` (gated `#[cfg(all(feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2"))]`), mirroring the existing `pub mod component` (plugin-world) block structurally:
- `wit_bindgen::generate!({ world: "extension-world", path: "📜️wit" })`, isolated in its own module so its generated `semio::framework::*` tree doesn't collide with `component`'s `plugin-world` tree (wit-bindgen can't generate two worlds at the same module scope).
- `pub struct ExtensionComponentGuest;` implementing the generated `exports::semio::framework::extension::Guest` trait:
  - `manifest()` → delegates to `crate::plugin_runtime::extension_manifest()`, pack-encoded the same way `plugin-world`'s `manifest()` does (`store::pack_rt::encode_wire_value(&dsl::to_dsl_value(&extension_manifest()).unwrap_or(dsl::DslValue::Null))`).
  - `activate()` → delegates to `extension_activate()`, mapping `Fault` → `PluginError::Fault(dsl::encode_fault_bytes(...))`.
  - `deactivate()` → delegates to `extension_deactivate()`.
  - `invoke(capability, request)` → delegates to `extension_invoke(&capability, &request)`, same fault-encoding pattern.
- `export!(ExtensionComponentGuest);`
- `pub fn extension_component_export_anchor() {}` — the real force-link anchor (see below), same pattern as `component::component_export_anchor`.

At the crate root (next to the existing `component_export_anchor` re-export/fallback pair):
```rust
#[cfg(all(feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2"))]
pub use extension_component::extension_component_export_anchor;

#[cfg(not(all(feature = "component-extension-guest", target_arch = "wasm32", target_env = "p2")))]
pub fn extension_component_export_anchor() {}
```

### Retired: the old anchor workaround
Removed the `//#region 🧩️ExtensionGuest` block (the `extension_guest_export_anchor` fn pair + its
doc comment about "Dual-world `wit-bindgen` is not generated alongside `plugin-world` yet") — that
workaround is exactly what this ticket implements for real, per the task's explicit instruction to
retire it now that the second bindgen exists.

`extension_exports!` macro's force-link static was rewired from the retired anchor to the new one:
```rust
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
#[used]
static _SEMIO_EXTENSION_COMPONENT_LINK: fn() = $crate::extension_component_export_anchor;
```
(mirrors `plugin_exports!`'s `_SEMIO_PLUGIN_COMPONENT_LINK` pattern exactly — unconditional on
target, with the anchor itself resolving to a real fn or a no-op stub depending on the feature.)

Removed `extension_guest_export_anchor` from the crate-root `pub use plugin_runtime::{...}` list
(grepped the whole `🔌️plugin/` tree first — only referenced inside this one file, safe to drop).

**Nothing else was touched or renamed** — `ExtensionBundle`, `ExtensionManifest` (guest-side),
`extension_manifest`/`extension_activate`/`extension_deactivate`/`extension_invoke`,
`extension_exports!` all keep their existing names/signatures; the guest-side `plugin-world`
workaround (a plugin's `manifest.contributions`/apps mechanism doubling as extension delivery) is
untouched — that removal is explicitly a later wave's job once producers migrate.

## 3. Host side — `🔌️plugin/🖥️host/🦀️component.rs`

Added a `//#region 🔖️ExtensionRuntime` block after `WasmPluginRuntime`'s region, structured as:

### `mod extension_bindings`
A second `wasmtime::component::bindgen!({ world: "extension-world", path: "../../../📦️packages/🦀️rust/📜️wit", async: false })`, nested in its own module (same reasoning as the guest side — two `bindgen!` invocations can't share a scope because both generate a `semio` module).

### `ExtensionManifest` (host-side)
A host-local mirror of the guest `ExtensionManifest` struct — the guest SDK crate
(`semio-framework-plugin`) that owns the real `ExtensionManifest` type is NOT a dependency of this
host crate (`semio-framework-plugin-host`), so it can't be reused directly. Decoded from the same
wire bytes via `dsl::from_dsl_value`/`store::pack_rt::decode_wire_value`, reusing
`semio_framework::Contribution` and `semio_framework::kernel::CapabilityRequirement` (both already
`Deserialize`) for its `contributions`/`capabilities` fields — no new types invented for those.
(Added `Contribution` to the existing `use semio_framework::{...}` import at the top of the file.)

### `ExtensionHostState` + `Host` impl
A dedicated Wasi-capable state struct (not reusing `HostState` — that type carries a lot of
plugin-specific machinery: `ArtifactSession` table, blob store, `IoRouter`, backbone map — none of
which extensions are wired to yet). Implements
`extension_bindings::semio::framework::host::Host` with all 17 `host` interface methods; `log`/
`now_ms`/`backbone_status` behave for real, everything else (`read-artifact`, `write-artifact`,
`open-window`, `invoke-action`, `network-fetch`, `write-blob`, `read-blob`, `backbone-send`,
`backbone-poll`, `engine-derive`, `engine-read`, `io-dialects`, `io-compose`) faults as
not-implemented — matching the existing `Host for HostState` impl's own pattern for its still-stubbed
methods (`read_artifact`/`write_artifact`/`open_window`/`invoke_action`/`network_fetch` are already
not-implemented there today).

### `ExtensionRuntime` — the requested API surface
```rust
pub struct ExtensionRuntime {
    engine: Engine,
    linker: Linker<ExtensionHostState>,
    instances: Mutex<HashMap<String, Arc<LoadedExtension>>>,
}

impl ExtensionRuntime {
    pub fn new() -> Result<Self, PluginHostError>;
    pub fn load(&self, path: impl AsRef<Path>) -> Result<String, PluginHostError>;       // reads file, delegates to load_bytes
    pub fn load_bytes(&self, wasm_bytes: &[u8]) -> Result<String, PluginHostError>;       // instantiate + manifest() + activate(), keys by manifest.extension_id, returns that id
    pub fn unload(&self, extension_id: &str) -> Result<(), PluginHostError>;              // deactivate() + drop from table
    pub fn manifest(&self, extension_id: &str) -> Option<ExtensionManifest>;              // decoded manifest of a loaded extension
    pub fn extension_invoke(&self, extension_id: &str, capability: &str, request: &[u8]) -> Result<Vec<u8>, dsl::Fault>;
}
```
`extension_invoke` matches the exact signature requested in the task (`Result<Vec<u8>, Fault>` —
this crate's `Fault` type is `dsl::Fault`, the same one `host_fault_bytes`/`PluginHostError::Plugin`
already decode into elsewhere in this file). Unlike `WasmPluginRuntime`'s methods (which surface
`PluginHostError`), `extension_invoke` returns `dsl::Fault` directly — it's one layer closer to the
WIT ABI's own fault channel, and the task explicitly asked for `Fault`, not `PluginHostError`.

`load`/`load_bytes`/`unload`/`manifest` return `PluginHostError` (matching `WasmPluginRuntime`'s own
convention for its equivalent lifecycle methods — `load`, `hot_reload`, etc.).

Mirrors `WasmPluginRuntime`'s `load`/`load_bytes` split and its `build_engine`/`build_linker`
patterns (`consume_fuel(true)`, `epoch_interruption(true)`, `wasm_component_model(true)`,
`PLUGIN_FUEL_BUDGET` reused for the per-call fuel budget in `extension_invoke`).

**Purely additive**: `ExtensionRuntime` is not constructed or referenced anywhere else in the boot
sequence — it's a standalone type ready for a later wave to wire in.

## 4. Verification

### Guest side (wasm32-wasip2 target)
```
cargo check -p semio-framework-plugin --features component-extension-guest --target wasm32-wasip2
```
**Succeeded** — `Finished` dev profile, 23 warnings (all pre-existing, unrelated to this change —
hidden-lifetime-parameter/unnecessary-qualification/unused-variable lints on code this ticket didn't
touch). Confirmed by diffing warning content against a run without the new feature — same warning
set, same line numbers outside my additions.

Also confirmed the mutual-exclusion guard actually compiles-error when both features are enabled:
```
cargo check -p semio-framework-plugin --features component-guest,component-extension-guest --target wasm32-wasip2
```
→ `error: \`component-guest\` and \`component-extension-guest\` are mutually exclusive for wasm32-wasip2 targets` at the `compile_error!` line — exactly as intended.

### Guest side (default features, sanity)
```
cargo check -p semio-framework-plugin
```
**Succeeded**, unaffected (no new warnings).

### Host side (native target, default features)
```
cargo check -p semio-framework-plugin-host
```
**Succeeded** — `Finished` dev profile, 3 warnings, all pre-existing (`unused extern crate`
`dsl_core`/`vcs` in `📦️glue.rs`, and an unused `error` binding at line 104 of this same file, in code
this ticket didn't touch — pre-existing before my edit, confirmed by checking blame context around
that line, which is inside `apply_emit_ops`'s pre-existing `Err(error) => { self.pending_binary_ops = ops; }` arm).

### Host side tests
```
cargo test -p semio-framework-plugin-host
```
**2 passed, 0 failed**:
- `component::tests::wasm_plugin_runtime_api_exists` (pre-existing)
- `component::tests::extension_runtime_constructs_engine_and_linker` (new) — builds a real
  `ExtensionRuntime` (real wasmtime `Engine`+`Linker`, confirming `add_to_linker` for the new
  `extension_bindings::semio::framework::host::Host` impl actually type-checks and links), asserts
  `manifest("nonexistent")` is `None`, and asserts `extension_invoke("nonexistent", "noop", &[])`
  faults with code `extension.unknown`.

### Environment limitations
`wasm32-wasip2` target **was already installed** in this environment (confirmed via
`rustup target list --installed`) — no gap there.

**Skipped**: a full round-trip integration test that instantiates a real compiled `extension-world`
wasm component and calls `manifest()`+`invoke()` through wasmtime end-to-end. This would require
building a trivial extension guest crate with `component-extension-guest` enabled, cross-compiling it
to a `wasm32-wasip2` *component* binary (`cargo build --target wasm32-wasip2` produces a wasm
*module*; turning it into a component additionally needs `wasm-tools component new` or the
`cargo-component` toolchain to adapt/embed the component-model metadata), then loading those bytes
in the host test. That's build tooling beyond `cargo test` (a second cargo invocation targeting a
different triple, plus a component-adapter step) — per the task's own carve-out, skipped and noted
here. The unit test added instead proves the host-side wiring (bindgen, linker, trait impl, instance
table, fault path) compiles and behaves correctly for the one path that doesn't need real wasm bytes
(the not-found path); the guest-side `Guest` trait impl was proven to compile+link for the real
target in the `wasm32-wasip2` check above.

## Files touched
- `🔌️plugin/📦️packages/🦀️rust/Cargo.toml` — added `component-extension-guest = []` feature.
- `🔌️plugin/🦀️component.rs` — added mutual-exclusion `compile_error!`; added `pub mod
  extension_component` (second `wit_bindgen::generate!` for `extension-world` + `Guest` impl +
  `export!`); added crate-root `extension_component_export_anchor` re-export/fallback pair; retired
  the old `extension_guest_export_anchor` anchor workaround block; rewired `extension_exports!`'s
  force-link static to the new anchor; dropped `extension_guest_export_anchor` from the crate-root
  `pub use plugin_runtime::{...}` list.
- `🔌️plugin/🖥️host/🦀️component.rs` — added `Contribution` to the top `use semio_framework::{...}`
  import; added `//#region 🔖️ExtensionRuntime` (`mod extension_bindings` with the host-side
  `bindgen!` for `extension-world`, host-side `ExtensionManifest`, `ExtensionHostState` + `Host`
  impl, `LoadedExtension`, `ExtensionRuntime` with `new`/`load`/`load_bytes`/`unload`/`manifest`/
  `extension_invoke`); added `extension_runtime_constructs_engine_and_linker` test to the existing
  `#[cfg(test)] mod tests`.

## What a later wave needs to find (removal candidates, NOT done here)
- The `plugin-world`-as-extension workaround itself (however extensions are actually delivered
  today via a plugin's own manifest/contributions machinery) — untouched, still the only live path
  until a later wave migrates producers to real `extension-world` components and wires
  `ExtensionRuntime` into an actual boot sequence.
