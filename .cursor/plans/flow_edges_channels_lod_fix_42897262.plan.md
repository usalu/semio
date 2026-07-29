---
name: Flow Edges Channels Lod Fix
overview: Restore neuron ports/channels (and therefore edges) and LOD text rendering for Flow across both renderers by wiring the never-connected operator-metadata sync, fixing a field-shape mismatch in React's label overlay, and adding an equivalent label-overlay renderer for wgpu (which has no DOM/2D-canvas overlay layer).
todos: []
isProject: false
---

## Root causes (verified by direct grep/read)

This is broader than a wgpu-only gap: two of the three causes affect **both** renderers today.

### 1. Neuron ports/channels never get metadata, so most edges cannot wire (affects React AND wgpu)

`FlowHost::kind_infos` (`flow/core/rs/lib.rs:1734`) drives `neuron_io_layout` (`flow/core/rs/lib.rs:729-765`), which computes each neuron's input/output ports. For a neuron widget with empty explicit `input_ports`/`output_ports` (e.g. the default fixture's `math.add`, `flow/core/rs/lib.rs:303`: `input_ports: vec![], output_ports: vec![]`) and **no** `kind_infos` entry, `neuron_io_layout` returns `(vec![], vec![], false, false)` — **zero ports**. `DagHost::rebuild_engine_with_layout` (`mathematical/graph/port/directed/dag/rs/lib.rs:2775-2831`) can only wire an edge into `self.engine.edges` when both endpoint port handles exist; with zero ports, edges silently fail to wire, and `paint_scene`'s port-handle painting (`dag/rs/lib.rs:3645-3650`) and edge painting (`dag/rs/lib.rs:4607-4624`) both have nothing to draw.

`kind_infos` is only ever populated via `set_neuron_kind_infos_json` (`flow/core/rs/lib.rs:1828-1831`), which is called **only from Rust tests** (confirmed via repo-wide grep — zero call sites in any `.tsx`/`.ts` file, despite an old ticket claiming otherwise). The flow plugin's scene builder explicitly sets `operators_json: None` (`flow/program/rs/lib.rs:720`), so neither renderer ever receives operator metadata to forward. Note this is separate from `catalogue_json`/`set_host_catalogue_json` (`flow/core/rs/lib.rs:1824-1826`), which only feeds the drag-and-drop sidebar and has nothing to do with port layout.

The registry already exposes exactly the right shape: `flow_registry().operator_catalogue()` (`flow/core/rs/lib.rs:1316-1331`, `neural/engine/rs/lib.rs:997-1001`) returns `Vec<OperatorInfo>` — the exact type `set_neuron_kind_infos_json` deserializes.

### 2. React's label overlay reads a field shape that doesn't match what Rust emits (React-only bug, pre-existing)

`label_overlay_paint_state_json` (`mathematical/graph/port/directed/dag/rs/lib.rs:3867-3888`) emits rows shaped `{ id, text, layout: "horizontal"|"vertical", x, y, nodeW, nodeH, fontScreenPx, ghost, kind?, align? }` with **world-space** `x`/`y` (e.g. `computation_name_world_center` returns `node.x`/`node.y`-based world coordinates, `dag/rs/lib.rs:1021-1029`). But `graph-canvas-overlays.tsx`'s `DagLabelOverlayRow` type and `paintDagLabelOverlays` (`framework/renderer/react/components/graph-canvas-overlays.tsx:4-18,138-177`) expect `width`/`height`/`vertical: boolean` fields and use `row.x`/`row.y` **directly as canvas pixel coordinates** with no world-to-screen projection. This mismatch means React's own node-name/port-label text overlay is not currently rendering correctly either.

### 3. wgpu has no label-overlay renderer at all (wgpu-only gap)

`engine_canvas::paint_node_graph` (`framework/renderer/wgpu/rs/engine_canvas.rs:254-323`) only pushes the Vello raster quad; nothing calls `label_overlay_paint_state_json()` or paints any text on top, unlike React's dedicated `labelCanvas` + `paintDagLabelOverlays` (`flow-graph-canvas-host.tsx:476-477,258-274`). Since node names, port/channel names, and most LOD-tier text are intentionally emitted only through this overlay JSON in Rust (`node_caption_delegated_to_js_overlay`/`port_labels_delegated_to_js_overlay`, `dag/rs/lib.rs:4171-4177`), wgpu shows blank rectangles with no text regardless of item 1's fix.

### 4. Forced-LOD-label field name mismatch (three different names, minor, same code paths)

The program emits `"forced"` (`flow/program/rs/lib.rs:729`), wgpu's `sync_flow_host` reads `"lod"` (`engine_canvas.rs:114`), and React reads `"forcedLabel"` (`flow-graph-canvas-host.tsx:43,45`). Manually forcing an LOD tier from the "LOD Mode" dropdown currently has no effect in either renderer.

## Fix plan

### A. Wire operator metadata end-to-end (fixes ports/channels/edges in both renderers)

- `flow/core/rs/lib.rs`: add a small public helper, e.g. `pub fn flow_neuron_kind_infos_json() -> String { serde_json::to_string(&flow_registry().operator_catalogue()).unwrap_or_else(|_| "[]".into()) }`, next to `flow_operator_catalogue_json`.
- `flow/program/rs/lib.rs:720`: change `operators_json: None` to `operators_json: Some(flow_core::flow_neuron_kind_infos_json())`.
- `framework/renderer/wgpu/rs/engine_canvas.rs`'s `sync_flow_host`: add a diff-gated `operators_json` field to `NodeGraphSyncCache` and call `host.set_neuron_kind_infos_json(json)` when it changes (mirroring the existing `fixture_json`/`catalogue_json` pattern added previously).
- `framework/renderer/react/components/flow-graph-canvas-host.tsx`'s `syncFlowSessionFromScene`: add `if (scene.operatorsJson) session.setNeuronKindInfosJson(scene.operatorsJson);`.

### B. Fix React's label overlay row shape (`framework/renderer/react/components/graph-canvas-overlays.tsx`)

- Update `DagLabelOverlayRow` to match the real Rust shape: `nodeW`/`nodeH` (world units) instead of `width`/`height`, `layout: "horizontal" | "vertical"` instead of `vertical: boolean`.
- In `paintDagLabelOverlays`, accept the camera (already parsed via `parseDagOverlayCamera`) and project each row's world `x`/`y` to screen with the existing `worldToScreen` helper (`graph-canvas-overlays.tsx:79-84`) before drawing; scale `nodeW`/`nodeH` by `camera.zoom` for on-screen box sizing.
- Update the call site in `flow-graph-canvas-host.tsx` (`paintOverlays`, ~line 258-274) to pass the camera through.

### C. Add a wgpu label-overlay renderer (`framework/renderer/wgpu/rs/engine_canvas.rs`, `scenes.rs`)

- Add `paint_node_graph_labels(ctx, scene, inner)` in `engine_canvas.rs`: look up the `EngineSurface`, call `host.label_overlay_paint_state_json()` (Flow) or the equivalent `GraphHost` method for non-Flow graphs, parse `{ camera, width, height, labels }`.
- For each row, project world `x`/`y` to screen using the same convention as React's `worldToScreen` (`screen = inner.origin + (world - camera) * camera.zoom + rowSpaceCenter`), and call the existing `ui_wgpu::draw_text`/`draw_text` helper (already used throughout `scenes.rs`) to push glyphs into `ctx.draw`, on top of the already-pushed raster quad.
- Render all rows left-aligned/horizontally (skip true 90-degree rotation for `layout: "vertical"` rows as an explicit, acceptable simplification — wgpu's `push_glyph` only supports axis-aligned quads; rotated text would require extending the glyph pipeline, out of scope here since it's a cosmetic orientation difference, not a missing-content bug).
- Call `paint_node_graph_labels` from `render_node_graph` in `scenes.rs` right after `engine_canvas::paint_node_graph(...)`.

### D. Fix forced-LOD-label field name mismatch

- Standardize on `forcedLabel` everywhere: change `flow/program/rs/lib.rs:729` to emit `"forcedLabel"` instead of `"forced"`, and update wgpu's `sync_flow_host` (`engine_canvas.rs:114`) to read `"forcedLabel"` instead of `"lod"`. React already reads `"forcedLabel"` — no change needed there.

### E. Verification

- `cargo test -p flow-program`, `cargo test -p flow_core`, `cargo test -p mathematical-graph-port-directed-dag` (or workspace equivalents) and `cargo build --target wasm32-unknown-unknown` for `flow-program` and `semio-framework-renderer-wgpu`.
- Manually check both the React and wgpu Flow playgrounds with a neuron-to-neuron chain (e.g. two connected `math.add`/`math.multiply` nodes): confirm port/channel labels are visible, connecting edge lines render, node/port text density changes across zoom levels (LOD), and that picking a specific tier from "LOD Mode" actually pins that tier's appearance in both renderers.

## Explicitly out of scope

- True rotated-text rendering for vertical-layout label rows in wgpu (slider/note/image/preview/variable/cluster widget names) — will render horizontally instead of sideways as a first pass.
- Inline param/stepper DOM editors (`GraphParamOverlays`/`GraphStepperOverlays` in React) and their wgpu equivalent — separate interactive-editing feature, not required for edges/channels/LOD visibility.
- Per-node `HitTarget`s / double-click-to-open-instance in wgpu (already tracked as out of scope from the prior pan/zoom/select fix).
