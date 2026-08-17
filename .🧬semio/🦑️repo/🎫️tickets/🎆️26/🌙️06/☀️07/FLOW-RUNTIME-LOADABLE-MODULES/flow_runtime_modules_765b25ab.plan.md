---
name: flow runtime modules
overview: Convert flow modules from compile-time Rust crates linked into flow/core into independently compiled WASM extensions, loaded/unloaded at runtime by a VSCode-style JS extension host with a full manifest (neuron kinds, widgets, commands, activation events, settings).
todos:
 - id: neural-dispatch
   content: Add Evaluator::evaluate_with(dispatch) to neural/engine/lib.rs; delegate evaluate() to it; add test.
   status: completed
 - id: module-wasm-abi
   content: Give each flow/modules/{math,text,logic,dictionary} its own cdylib + wasm-bindgen ABI (manifest/evaluate/command/activate/deactivate), script.ts, project.json, package.json.
   status: completed
 - id: core-bridge
   content: Remove module deps from flow/core; add eval bridge + host catalogue; rewrite evaluate_internal/catalogue_json; add setEvalBridge/setCatalogueJson to FlowSession.
   status: completed
 - id: extension-host
   content: Implement FlowExtensionHost in flow/react (loader map, activate/deactivate, evaluate routing, catalogue aggregation) and wire FlowCanvas startup async.
   status: completed
 - id: build-aliases
   content: Build all module wasms from flow/react & flow/play scripts; add @flow/module-* vite/vitest aliases.
   status: completed
 - id: extensions-panel
   content: Add VSCode-like Extensions panel + command palette in flow/play and framework renderer; toggle activate/deactivate and refresh catalogue.
   status: completed
 - id: infra
   content: Register module wasm builds in launch.json following existing flow grouping; verify Cargo workspace members.
   status: completed
 - id: tests-validate
   content: Extend existing tests (neural, modules, core, react, play) and the flow play runtime validator for enable/disable behavior.
   status: completed
isProject: false
---

# Flow Runtime-Loadable Modules (VSCode-extension style)

## Goal

Each flow module compiles to its own `.wasm` with a standard extension ABI. A JS "extension host" installs, activates, and deactivates modules at runtime, aggregates their manifest contributions into the catalogue, and bridges neuron evaluation from `flow/core` back to the owning module WASM. Modules can be enabled/disabled live from a VSCode-like Extensions panel.

## Architecture decisions

- **Evaluation bridge over compile-time registry.** `flow/core` stops linking `flow_extension_*`. Its evaluator dispatches each neuron kind through a JS callback (`(kindId, inputJson) -> outputJson`) routed by the host to the active module WASM. This is synchronous (wasm to JS to wasm), matching the existing synchronous `FlowSession.evaluate()`.
- **Single source of truth per module.** Each module keeps its canonical `register(&mut Registry)`; the new wasm wrapper builds a local `Registry` from it and serves `manifest()` + `evaluate()` from that. No metadata duplication.
- **Catalogue from host, not core.** Neuron catalogue sections come from aggregated active manifests pushed into `FlowSession`; `flow/core` keeps only the static Inputs/Outputs sections.
- **Send/Sync avoidance.** `js_sys::Function` is `!Send`, so instead of registering host functions into `neural::Registry` (whose `Function: Send + Sync`), add `Evaluator::evaluate_with(tree, seeds, dispatch)` taking a non-Send closure. The existing registry path delegates to it.

## Manifest schema (`flow.extension/v1`, returned by each module's `manifest()`)

- `id`, `name`, `version`
- `activationEvents`: e.g. `["onStartup"]`, `["onNeuronKind:math.add"]`
- `contributes`:
  - `neuronKinds[]`: `{ id, name, summary, inputs[], outputs[] }` (from `NeuronKindInfo`)
  - `widgets[]`: declared widget contributions (rendered generically via existing widget kinds in v1; custom Vello chrome per extension is out of scope and noted below)
  - `commands[]`: `{ id, title }`
  - `settings[]`: `{ id, type, default, description }`

## 1. neural/engine — dispatch-based evaluation

File: `[neural/engine/lib.rs](neural/engine/lib.rs)` (region `🔖️Evaluator`)

- Add `Evaluator::evaluate_with(&self, tree, seeds, dispatch: &mut dyn FnMut(&str, &Dictionary) -> Result<Dictionary, EvalError>)` containing the current topo logic (lines 219-234), calling `dispatch(&neuron.kind, &input.merge(&params))` instead of `self.registry.get(...)`.
- Reimplement `evaluate()` to call `evaluate_with` with a registry-lookup closure. Existing tests (lines 339-374) stay green.

## 2. Per-module WASM extensions

For each of `flow/modules/{math,text,logic,dictionary}`:

- `Cargo.toml`: change `crate-type` to `["rlib", "cdylib"]`; add `wasm-bindgen`, `serde_json`, and `getrandom` (js feature) gated to wasm; keep `neural_engine` dep. Mirror `[flow/core/Cargo.toml](flow/core/Cargo.toml)` wasm setup.
- `lib.rs`: add `#[cfg(target_arch = "wasm32")]` region exposing wasm-bindgen functions backed by a `Registry` built from the existing `register()`:
  - `manifest() -> String` (builds `flow.extension/v1` JSON from `registry.catalogue()` plus hand-authored commands/settings/activationEvents for that module)
  - `evaluate(kind_id: &str, input_json: &str) -> String` (parse `Dictionary`, dispatch via `registry.get(kind_id)`, serialize result or `{ "error": ... }`)
  - `command(command_id: &str, args_json: &str) -> String` (no-op/log stub returning JSON for v1)
  - `activate()` / `deactivate()` lifecycle stubs (console log)
- `script.ts`: new router calling `runWasmPackWebBuild` (copy `[flow/core/script.ts](flow/core/script.ts)`, set `wasmBaseName`/pkg name `@flow/module-<name>`).
- `project.json`: nx `wasm` target calling `bun ./📜️script.ts wasm` (copy `[flow/core/project.json](flow/core/project.json)`).
- `package.json`: `nx`-invoking wrapper consistent with other crates.

## 3. flow/core — drop module linkage, add bridge + host catalogue

File: `[flow/core/lib.rs](flow/core/lib.rs)`

- Remove `pub use flow_extension_*` (lines 3-6) and the four `register(...)` calls in `build_registry()` (lines 163-170). Delete the neuron half of `build_catalogue()`; keep only Inputs/Outputs sections.
- `FlowHost`: add fields `eval_bridge: Option<EvalBridge>` and `host_catalogue_json: String`.
  - `evaluate_internal()` (lines 443-457): build tree+seeds, then `Evaluator::evaluate_with(&tree, &seeds, &mut |kind, input| self.eval_bridge dispatch)`. The dispatch serializes `input` to JSON, calls the JS function, parses the returned `Dictionary`.
  - `catalogue_json()` merges `host_catalogue_json` (neuron sections) with the static Inputs/Outputs sections.
- `FlowSession` (region `🔖️WasmSession`): add `#[wasm_bindgen]` methods:
  - `setEvalBridge(cb: js_sys::Function)` — stored as the dispatch source.
  - `setCatalogueJson(json: &str)` — host-aggregated neuron sections.
  - keep `evaluate()`, `catalogueJson()` signatures.
- `Cargo.toml`: remove the four `flow_extension_*` path deps.
- Bridge type: a small wasm-only struct holding `js_sys::Function`, called with `(kindId, inputJson)`; non-wasm builds use a no-op so native cargo tests still compile (flow/core tests that rely on math.add move to host-driven react/play tests, or use a test bridge closure via `FlowHost` direct API).

## 4. flow/react — the extension host

File: `[flow/react/index.tsx](flow/react/index.tsx)` (new region `🔖️ExtensionHost`)

- Static module loader map (vite-analyzable):
  `{ math: () => import("@semio-tech/flow-module-math"), text: ..., logic: ..., dictionary: ... }`.
- `FlowExtensionHost` class:
  - `installed`: ids from the loader map (the "marketplace").
  - `activate(id)`: dynamic import glue, `await init()`, read `manifest()`, register contributions (neuronKinds, commands, settings), fire `activationEvents`.
  - `deactivate(id)`: call `deactivate()`, drop the instance reference and contributions.
  - `evaluate(kindId, inputJson)`: route to the active module owning `kindId`, call its wasm `evaluate`; unknown kind returns `{ "error": "no module for kind" }`.
  - `catalogueJson()`: aggregate active manifests into `CatalogueSection[]`.
  - `commands` / `settings` registries with getters/execute.
- `FlowCanvas` effect (lines 416-474): make startup async — `await host.activate(default modules)`, then `session.setEvalBridge((k,i)=>host.evaluate(k,i))`, `session.setCatalogueJson(host.catalogueJson())`, then `loadFixtureJson` + `evaluate`. On activate/deactivate, re-push catalogue + re-evaluate + re-render.
- Export `flowExtensionHost` singleton + types for the play UI.

## 5. flow/react & flow/play build — compile all module wasms

Files: `[flow/react/script.ts](flow/react/script.ts)`, `[flow/play/script.ts](flow/play/script.ts)`

- Before dev/test/validate, invoke each `../modules/<name>/script.ts wasm` (in addition to `../core/script.ts wasm`), so module pkgs exist for dynamic import.

## 6. Vite/vitest aliases

Files: `[ui/styling/vite-elements-assets.ts](ui/styling/vite-elements-assets.ts)` (`playgroundRendererResolveAliases`, lines 750-753), `[flow/play/vite.config.ts](flow/play/vite.config.ts)`, `[flow/react/vitest.config.ts](flow/react/vitest.config.ts)`, `[flow/play/vitest.config.ts](flow/play/vitest.config.ts)`

- Add `@flow/module-<name>` to `flow/modules/<name>/pkg/flow_extension_<name>.js` for each module.

## 7. VSCode-like Extensions panel (full extension UX)

Files: `[flow/play/index.ts](flow/play/index.ts)`, `[framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)`

- Add a "Extensions" workbench tab/section in `FlowPlayController` listing installed modules with enable/disable toggles, version, and contributed counts; wire toggle commands to `flowExtensionHost.activate/deactivate`, then refresh catalogue (`setCatalogueSections`) and trigger re-evaluate.
- Add a simple command palette listing active modules' contributed commands, executing via the host.
- Update `FLOW_NEURON_MODULE_IDS` usage (line 59) so catalogue checks reflect active (not compiled-in) modules.

## 8. Infra registration (launch.json, root Cargo)

Files: `[.vscode/launch.json](.vscode/launch.json)` (flow group, lines 776-826), `[Cargo.toml](Cargo.toml)` (members lines 14-18)

- launch.json: add per-module `🛠️dev🌊️flow🦀️module-<name>` wasm-build entries and update `🛠️dev🌊️flow🦀️test` to include all module crates (already lists them). Follow existing order/grouping.
- Cargo workspace members already include the modules; no removal needed (they stay buildable as standalone cdylibs).

## 9. Tests (extend existing files only)

- `[neural/engine/lib.rs](neural/engine/lib.rs)`: test `evaluate_with` dispatch path.
- Each module `lib.rs`: extend `#[cfg(test)]` to assert `manifest()` JSON contains expected kind ids and `evaluate()` round-trips (native-testable via the registry the wasm wrapper uses).
- `[flow/core/lib.rs](flow/core/lib.rs)`: test `evaluate_internal` via a Rust test bridge closure (host-driven), and catalogue merge of Inputs/Outputs with an injected host catalogue.
- `[flow/react/index.tsx](flow/react/index.tsx)`: vitest for `FlowExtensionHost` activate/deactivate/evaluate routing + catalogue aggregation (mock module glue).
- `[flow/play/index.ts](flow/play/index.ts)`: vitest for Extensions panel toggle building active-module catalogue.

## Runtime validation (per repo rules)

- `bun nx run @semio-tech/flow-core:wasm` and each module wasm build succeed.
- `bun nx run @semio-tech/flow-play:validate` (extend `[flow/play/validate-flow-runtime.mjs]`) to assert: default modules activate, catalogue lists their kinds, disabling `math` removes `math.add` from catalogue and makes the default fixture report an evaluation error, re-enabling restores preview `3`. Confirm via console logs (`[DEBUG]` prefixed).

## Scope notes / decisions

- Custom per-extension Vello widget rendering is declared in the manifest but rendered with existing generic widget kinds in v1 (true custom canvas chrome per extension is a separate large effort).
- "Install/uninstall" in v1 means enable/disable from a fixed built marketplace (the static loader map); fetching arbitrary remote module wasm is a follow-up but the ABI/host are designed to allow it.
- Work proceeds under a new ticket; all temp artifacts kept under the ticket folder.
