# W0-D — WIT Spine Report

Lane 0-D, contract freeze §6. Scope: `📜️world.wit`, guest bindgen glue (`component`/`extension_component` regions in `🔌️plugin/🦀️component.rs`), host bindgen call-through on `WasmPluginRuntime` in `🔌️plugin/🖥️host/🦀️component.rs`, jco wiring in `🌐plugin-web-materialize.ts`.

## 1. WIT file state found, and what changed

On first read, `📜️world.wit` already reflected the FULL-STDIO ticket's completed move to typed inference records (`artifact-inference-request`/`artifact-inference-result` records in `interface types`, no byte-envelope). No mid-edit/inconsistent state was ever observed — every re-read before each edit showed a stable, coherent file. Changes made (region-local `Edit`s, re-reading first each time):

1. **`dependencies` field added** to `artifact-inference-request` (the record existed already, so per the ticket instructions the field went there, not onto a byte-envelope carrier):
   ```wit
   dependencies: list<tuple<string, list<u8>>>,
   ```
   Tuple is `(plugin-id, payload)` where `payload` is that dependency's `store::pack_rt::encode_wire_value` bytes.

2. **New `interface contributor`**, exporting:
   - `list-artifact-mutations: func() -> result<list<u8>, plugin-error>`
   - `artifact-mutation-plan: func(request: list<u8>) -> result<list<u8>, plugin-error>`
   - `list-artifact-inferences: func() -> result<list<u8>, plugin-error>` — **moved** out of `plugin`
   - `artifact-infer: func(request: artifact-inference-request) -> result<artifact-inference-result, plugin-error>` — **moved** out of `plugin`

   `interface plugin`'s `use types.{...}` list was trimmed (no longer needs `artifact-inference-request`/`-result`); `interface contributor` picks up its own `use types.{artifact-inference-request, artifact-inference-result, plugin-error}`. No duplicate exports remain anywhere.

3. **Worlds**:
   ```wit
   world plugin-world { import host; export plugin; export contributor; }
   world extension-world { import host; export extension; export contributor; }
   ```
   No new `host` imports on either world.

## 2. Guest bindgen glue (`🔌️plugin/🦀️component.rs`)

### `pub mod component` (plugin-world)
- `use exports::semio::framework::contributor::Guest as ContributorGuest;` added alongside the existing `plugin::Guest` import.
- `impl Guest for ComponentGuest` keeps `manifest`, `instantiate_app`, `exchange`, `migrate_artifact`, `clear_instance_guard`, `list_artifact_dialects`, `artifact_compose`.
- New `impl ContributorGuest for ComponentGuest` holds the moved `list_artifact_inferences`/`artifact_infer` (unchanged bodies, just relocated) plus two new stubs:
  ```rust
  fn list_artifact_mutations() -> Result<Vec<u8>, PluginError> {
      ensure_plugin_initialized();
      Ok(wire_list_artifact_mutations())
  }
  fn artifact_mutation_plan(request: Vec<u8>) -> Result<Vec<u8>, PluginError> {
      ensure_plugin_initialized();
      wire_artifact_mutation_plan(&request).map_err(|fault| PluginError::Fault(dsl::encode_fault_bytes(&fault)))
  }
  ```
- `export!(ComponentGuest);` unchanged — wit_bindgen's `export!` picks up both `Guest` impls automatically since both traits are implemented on the same type.

### `pub mod extension_component` (extension-world)
`extension-world` now also exports `contributor`, so `ExtensionComponentGuest` needed a `ContributorGuest` impl too (no extension ever wired inference/mutation services before). Added:
```rust
impl ContributorGuest for ExtensionComponentGuest {
    fn list_artifact_inferences() -> Result<Vec<u8>, PluginError> { Ok(extension_wire_list_artifact_inferences()) }
    fn artifact_infer(_request: ExtensionInferenceRequest) -> Result<ExtensionInferenceResult, PluginError> {
        Err(PluginError::Fault(dsl::encode_fault_bytes(&extension_wire_artifact_infer())))
    }
    fn list_artifact_mutations() -> Result<Vec<u8>, PluginError> { Ok(wire_list_artifact_mutations()) }
    fn artifact_mutation_plan(request: Vec<u8>) -> Result<Vec<u8>, PluginError> {
        wire_artifact_mutation_plan(&request).map_err(|fault| PluginError::Fault(dsl::encode_fault_bytes(&fault)))
    }
}
```

### `plugin_runtime` region — placeholders (⚠️ **W1-A's to fill in**)

`wire_list_artifact_mutations`/`wire_artifact_mutation_plan` did not exist before this lane; added as minimal placeholders right after `plugin_manifest`, each with a `🚧️ PLACEHOLDER for W1-A` doc comment:
```rust
pub fn wire_list_artifact_mutations() -> Vec<u8> {
    store::pack_rt::encode_wire_value(&dsl::DslValue::Array(Vec::new()))
}
pub fn wire_artifact_mutation_plan(_request: &[u8]) -> Result<Vec<u8>, Fault> {
    Err(plugin_internal_fault("artifact-mutation-plan not implemented (W1-A placeholder)"))
}
```
Deterministic EMPTY roster / typed `Fault`, exactly as instructed. Two more placeholders were needed for the extension side (no prior wiring existed at all there), added near `extension_invoke`, also marked `🚧️ PLACEHOLDER for W1-A`:
```rust
pub fn extension_wire_list_artifact_inferences() -> Vec<u8> { store::pack_rt::encode_wire_value(&dsl::DslValue::Array(Vec::new())) }
pub fn extension_wire_artifact_infer() -> Fault { plugin_internal_fault("artifact-infer not implemented for extensions (W1-A placeholder)") }
```

## 3. Host bindgen wiring (`🔌️plugin/🖥️host/🦀️component.rs`)

`WasmPluginRuntime::list_artifact_inferences`/`artifact_infer` repointed from `bindings.semio_framework_plugin()` to `bindings.semio_framework_contributor()` (same call idiom, new accessor — auto-generated by `wasmtime::component::bindgen!` once the WIT moved the interface). Two new methods added mirroring the same idiom:
```rust
pub fn list_artifact_mutations(&self) -> Result<Vec<u8>, PluginHostError> { .. call_list_artifact_mutations .. }
pub fn artifact_mutation_plan(&self, request: &[u8]) -> Result<Vec<u8>, PluginHostError> { .. call_artifact_mutation_plan(&mut *store, request) .. }
```
No routers/graph/coordinator built — out of this lane's scope per the ticket brief. No `with:`/`bindgen!` macro changes were needed; interface accessors regenerate from the WIT automatically.

## 4. jco / `🌐plugin-web-materialize.ts`

No code change needed. `pluginComponentBridgeSource` does `import { plugin } from "./${componentBase}.js"` and only destructures the specific methods it calls (`manifest`, `instantiateApp`, `exchange`, `clearInstanceGuard`) — it never enumerates the full export set, so jco emitting an additional `contributor` named export is additive and harmless. The `--map semio:framework/host=./🟨️host-shim.js` flag only maps **imports**; unaffected by new exports. Searched this package, `🏪️store/📜️store.ts`, and `🧑️‍💻️dev/📜️script.ts` for anything that enumerates/asserts the exported-interface set — found none (no checked-in generated `.d.ts`/bindings to go stale either).

## 5. Gates

### `cargo check -p semio-framework-plugin -p semio-framework-plugin-host` (native target)
**PASS** (confirmed on a stable retry — see §6 on churn):
```
    Finished `dev` profile [unoptimized] target(s) in 21.29s
```
Only pre-existing warnings, no errors.

### Deeper validation: `cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest`
This is the target that actually compiles the `component`/`extension_component` modules (both are `#[cfg(target_arch = "wasm32", target_env = "p2")]`-gated, so the native check above never touches them). Result: 3 errors, **all three pre-existing, at lines I did not touch, and all attributable to `semio_framework::io::IoWireError`** (an error-type change from `String` to `IoWireError` mid-flight in `🚪️io/🦀️component.rs`, confirmed `git status --short` = `MM` for that exact file — outside every W0 lease, explicitly flagged in `📋️ownership-and-handoffs.md` §1 as owned by ticket `26/08/16/FULL-STDIO-…`). The three call sites (`artifact_compose`'s `map_err`, and two spots inside the untouched `install_io_fallback_dispatcher`) predate this lane's diff entirely — verified against the file content read at the start of this task, byte-identical at those lines.

### `cargo check -p semio-s-plugin-flow -p semio-s-plugin-cad`
**Blocked**, not by this lane's diff. Two independent, unrelated concurrent breakages observed across retries (error signatures changed between retries, confirming live flapping edits, not a stable state):
- `error[E0063]: missing field 'origin' in initializer of 'command::MutationMeta'` / `cannot find type ForeignStep` / `cannot find type MutationOrigin` in `📡️spr/🎮️command/🦀️component.rs` and `🏪️store/🦀️component.rs` — lane **0-A**'s in-flight §1 composite-mutation spine work (`git status --short` = `MM`/` M` on both files; exactly 0-A's lease per the ownership doc, which explicitly calls out these `origin:` fixups as in-progress).
- `error[E0425]: cannot find function 'preflight_format_descriptors' in crate 'semio_framework'` (and siblings) — a separate, unrelated flap in the `semio-framework` crate (0-C's lease territory), gone on a later retry.
- `semio-s-plugin-stdio` (a transitive dep of flow/cad): 39 errors, `cannot find value 'assembly' in module crate::artifacts::<kind>` across ~24 stdio artifact modules plus several `cannot find value 'definition'` — an unrelated in-flight refactor of the stdio plugin's artifact-registration macros, nothing this lane touches.

### `cargo check -p semio-s-plugin-cad-aec-building` (extension crate sample)
**Blocked** by the same 0-A in-flight work, this time hitting `📡️spr/📜️history/🦀️component.rs`'s `HistoryOpMeta` (also 0-A's lease): `error[E0063]: missing field 'origin' in initializer of 'history::HistoryOpMeta'`.

**Conclusion:** every crate/file this lane actually leased and edited (`📜️world.wit`, `component`/`extension_component` regions, `WasmPluginRuntime` contributor methods, `🌐plugin-web-materialize.ts`) checks clean on its own; every remaining red gate traces to a different, still-in-flight lane's file (`🚪️io/🦀️component.rs`, `📡️spr/🎮️command`, `🏪️store`, `📡️spr/📜️history`, `semio-framework`, `semio-s-plugin-stdio` artifacts) — confirmed via `git log --date=iso -1` + `git status --short` against start commit `7ad8955884` for each, not via commit-message text. None of these files are in this lane's lease or were touched by this lane's diff. A final re-run of the plugin/plugin-host gate immediately before writing this report caught a *third*, different error (`E0062: field 'origin' specified more than once` in `semio-framework-os-kernel`) — confirming 0-A's spine work is still actively flapping at the time of writing, not stabilized. A post-edit integrity check confirms none of this lane's own changes were clobbered by the churn (`grep` counts for `ContributorGuest`/`wire_list_artifact_mutations`/`wire_artifact_mutation_plan`/`extension_wire_*` in the guest file, `semio_framework_contributor` in the host file, and `interface contributor` in the WIT file all match expectations). Recommend the coordinator re-run these gates once 0-A (and whichever lane owns `io`/`semio-framework`/stdio) land their in-flight work.

## Files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/📜️wit/📜️world.wit`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (`component`, `extension_component`, `plugin_runtime` regions only)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` (`WasmPluginRuntime` impl block only)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts` — inspected, no change required
