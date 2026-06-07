---
name: flow catalogue dnd
overview: Complete the flow procedural language by adding a module-sectioned function catalogue with drag-and-drop node creation, port-to-port wiring, multiple modules (math/text/logic), and fixture persistence, building on the existing neural-engine + DAG + wasm-core + React slice.
todos:
  - id: catalogue-meta
    content: Add NeuronKindInfo + Registry catalogue() to neural/engine/lib.rs; update register signature and engine tests.
    status: completed
  - id: modules
    content: Update flow/modules/math register with metadata; add flow/modules/text and flow/modules/logic crates (lib.rs+Cargo.toml) with NeuronKinds; register in root Cargo.toml members and flow/core deps.
    status: completed
  - id: core-edit-api
    content: "Flow core: add layout positions, catalogue_json, add/remove/move/connect/disconnect, world_from_screen, camera/wheel, and pointer wiring; expose via FlowSession wasm."
    status: completed
  - id: core-render
    content: "Flow core: render rectangle IO nodes by delegating render_frame_gpu to dag::DagHost rebuilt from widgets."
    status: completed
  - id: react-catalogue
    content: "flow/react: FlowCatalogue panel with module/Inputs/Outputs sections, draggable items, and canvas drop -> addWidget."
    status: completed
  - id: react-interaction
    content: "flow/react: forward pointer/wheel for pan/zoom/select/move and port-to-port wiring; re-evaluate on change."
    status: completed
  - id: persistence
    content: "flow/react: FlowStore localStorage interface; load on mount, save on mutation, reset action."
    status: completed
  - id: wiring
    content: Update flow/core/Cargo.toml deps, root Cargo.toml members, and launch.json cargo entries; verify framework exports build.
    status: completed
  - id: validate
    content: Run cargo tests, wasm build, vitest, and extend @flow/play:validate playwright probe for catalogue + drag-drop + wiring + persistence.
    status: completed
isProject: false
---

# Finish flow: catalogue, drag-and-drop, wiring, persistence

## Current state (verified)
- The vertical slice exists and evaluates: `neural/engine/lib.rs` (Dictionary/Tree/Registry/Evaluator), `flow/modules/math/lib.rs` (`math.add/multiply/passThrough`), `mathematical/graph/port/directed/dag/lib.rs` (rect IO nodes + layered layout), `flow/core/lib.rs` (`FlowHost`/`FlowSession` wasm), `flow/react/index.tsx` (`FlowCanvas`), `flow/play/index.ts`.
- Gaps: `Registry` has no module/label metadata; only the `math` module exists; `flow/core` renders nodes as plain circles via `render_frame_gpu` and never forwards pointer/drop input; React `FlowCanvas` only has a slider+preview overlay; no catalogue; no add/connect/persist.

## 1. Catalogue metadata (neural engine + modules)
- `neural/engine/lib.rs` region `NeuronKind`: add `NeuronKindInfo { id, module, name, summary, inputs: Vec<String>, outputs: Vec<String> }`. Change `Registry` to store `(NeuronKindInfo, Box<dyn Function>)`; `register(info, fn)`; add `Registry::catalogue() -> Vec<NeuronKindInfo>` and keep `get(id)`. Update engine tests.
- `flow/modules/math/lib.rs`: pass `NeuronKindInfo` (module `"math"`, names/summaries, port keys) in `register`.
- New crates mirroring math (new `lib.rs` + `Cargo.toml`, added to root [Cargo.toml](Cargo.toml) `members` and to `flow/core` deps):
  - `flow/modules/text` (`flow_module_text`): e.g. `text.concat`, `text.upper`.
  - `flow/modules/logic` (`flow_module_logic`): e.g. `logic.greater`, `logic.not`.

## 2. Flow core: positions, editing API, rectangle rendering (`flow/core/lib.rs`)
- Fixture: add a `layout: BTreeMap<String,{x,y}>` to `FlowFixtureV1` (serde default). Auto-DAG-layout only fills ids missing from `layout`; persisted/dropped positions win.
- Register all modules in `evaluate_internal` (`math`+`text`+`logic`) and add `catalogue_json()` returning sections: one per module (from `Registry::catalogue()` grouped by `module`) plus synthetic `Inputs` (slider, note) and `Outputs` (preview, action) sections.
- Editing methods on `FlowHost` + matching `#[wasm_bindgen]` on `FlowSession`:
  - `add_widget(descriptor_json, world_x, world_y)` (descriptor = catalogue item: neuron kind, or input/output type) -> new unique id at position; `remove_widget(id)`; `move_widget(id, x, y)`.
  - `connect(from_id, to_id)` enforcing port rules + `would_create_cycle`; `disconnect(synapse_id)`.
  - `world_from_screen(sx, sy)` (camera inverse), `set_camera`, `wheel`/zoom; route pointer to select/move nodes and to draw edges output->input port.
- Rendering: replace the circle drawing in `render_frame_gpu` by delegating to a `dag::DagHost` rebuilt from widgets/synapses (reuses `paint_scene` rectangle IO nodes, port labels, camera affine, hit-testing). Keep the neural `Tree` separate per `flow/AGENTS.md`.

## 3. React renderer (`flow/react/index.tsx`)
- New region `FlowCatalogue`: read `session.catalogueJson()`, render collapsible sections (module function sections + Inputs + Outputs); each item is `draggable` and sets a drag payload (descriptor).
- `FlowCanvas`: handle `onDragOver`/`onDrop` -> `session.worldFromScreen(x,y)` -> `session.addWidget(descriptor, wx, wy)` -> `evaluate()` + render. Forward `pointerdown/move/up` and `wheel` to the session for pan/zoom/select/move and output->input edge wiring (re-evaluate on connect/disconnect). Keep slider/preview overlay.
- Persistence behind an interface: add a small `FlowStore` wrapper over `localStorage` (per repo "external libs behind interface" rule). Load saved fixture on mount, save on every fixture mutation, plus a Reset action.

## 4. Framework + build wiring
- `FlowPlayPaneSurfaceHost` in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx) already renders `<FlowCanvas/>`; the catalogue is an in-canvas overlay so framework changes are minimal (verify exports still build).
- Add `flow/modules/text`, `flow/modules/logic` to root [Cargo.toml](Cargo.toml) members and `flow/core/Cargo.toml` deps. Optionally add `🦀rs` cargo-test launch entries near the existing `🌊flow` group in [.vscode/launch.json](.vscode/launch.json) (existing `dev:flow`/validate entries stay).

## 5. Validation (must confirm at runtime)
- `cargo test -p neural_engine -p flow_module_math -p flow_module_text -p flow_module_logic -p flow_core -p mathematical_graph_port_directed_dag`.
- `bun nx run @flow/core:wasm`; `bun nx run @flow/react:test` and `@flow/play:test`.
- `bun nx run @flow/play:validate` (extend the playwright probe): catalogue shows module/Inputs/Outputs sections; drag a `math.add` onto canvas creates a node; wire ports; preview updates; reload restores persisted fixture; `[DEBUG]` logs confirm evaluate.

## Execution notes (repo rules)
- At start: read `repo://goals`, `ticket_reopen` the existing `Flow Language Vertical Slice` ticket (`2603...`) or `ticket_open` a new one; keep temp/logs in the ticket folder. Do not edit any `AGENTS.md`. Use regions, concise code, emoji docstrings; permanent commands only via each package `script.ts`.
