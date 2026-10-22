---
name: Channel Preview Mechanism
overview: Introduce a general per-channel preview mechanism across flow and procedural so every geometry component input and output channel previews its value (driven by per-port and per-node hover/selection), and make geometry operations like offset robust on profile inputs so offset-on-circle computes a real result.
todos:
 - id: engine-inputs
   content: "neural/engine/lib.rs: return per-neuron resolved inputs alongside outputs from the evaluator; extend tests."
   status: completed
 - id: flow-core-channels
   content: "flow/core/lib.rs + dag/lib.rs: emit channel-structured eval JSON; add hovered_channel()/selected_channels() decoding handle ids; expose on WASM session; extend Rust tests."
   status: completed
 - id: flow-react
   content: "flow/react/index.tsx: forward channel JSON via onEvalOutputs and emit hovered/selected channels through interaction callbacks."
   status: completed
 - id: procedural-registry
   content: "procedural/react/index.tsx: add port/direction to ProceduralPreviewItem, build preview registry + extractChannelPreviewItems, update ProceduralPreview filter for node/port/default; extend vitest."
   status: completed
 - id: play-controller
   content: "procedural/play/index.ts + playground renderer: track hovered/selected channel, extend setHover/setSelection, feed ProceduralPreview props, use extractChannelPreviewItems; update play tests."
   status: completed
 - id: robust-offset
   content: "geometry/brep/js/index.ts: make offsetSync accept drawing/edge/wire profiles like extrude; extend brep vitest for offset on circle/sketch."
   status: completed
 - id: ticket
   content: Open/reopen repo ticket associated with the most appropriate goal; keep temp artifacts in the ticket folder; close with summary when done.
   status: completed
isProject: false
---

## Problem

Preview today is per-widget, output-only, and error-skipping:

- `extractPreviewItems` ([procedural/react/index.tsx:1794](procedural/react/index.tsx)) reads only each widget's output dict and `continue`s on any `error` (line 1803).
- Eval (`evaluate_internal`, [flow/core/lib.rs:1250](flow/core/lib.rs)) exposes only `outputs` keyed by widget id; per-port input values from `collect_neuron_input` ([neural/engine/lib.rs:315](neural/engine/lib.rs)) are discarded.
- Hover/selection is node-level only (`hovered_node_id()` [dag/lib.rs:1795](mathematical/graph/port/directed/dag/lib.rs)); the controller's `setHover`/`setSelection` ([procedural/play/index.ts:1341](procedural/play/index.ts)) track widget ids only.
- `brep.solid.offset` ([geometry/brep/js/index.ts:979](geometry/brep/js/index.ts)) calls `require(shape,"solid","face","wire")`, but circles are `drawing`/`edge` handles, so it throws, the widget is error-skipped, and nothing previews on input, output, or component.

## Mechanism: Channels + Preview Registry

Define a **Channel** = `{ widgetId, port, direction: "in" | "out" }`. Every channel carries a `Dictionary` value. A **preview registry** maps a channel value shape to preview primitives (geometry handle(s), point, vector). Preview is rendered per channel; hover/selection at port-level shows that one channel, node-level shows all the node's channels, and the default (`everything`) shows all output channels.

### 1. Neural engine: expose per-neuron inputs ([neural/engine/lib.rs](neural/engine/lib.rs))

- In the evaluator loop, capture the `collect_neuron_input` result per neuron (already computed) alongside the output. Return both, e.g. an `EvalChannels { outputs, inputs }` (maps of `widgetId -> Dictionary`) instead of only `outputs`. Keep `outputs` map intact for existing callers.
- Extend the `#[cfg(test)]` block to assert inputs are returned per neuron.

### 2. Flow core: channel JSON + channel hover/selection ([flow/core/lib.rs](flow/core/lib.rs), [dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs))

- In `evaluate_internal` (line 1250), build a channel-structured map `{ widgetId: { in: { port: value }, out: { port: value }, error?: string } }`, splitting inputs by declared input ports (kind `inputs`) and outputs by declared output ports (kind `outputs`, value keyed under port `"out"`). Serialize this as the JSON returned by `session.evaluate()` (line 1850). Keep `self.outputs` + `apply_preview_outputs` (line 1316) unchanged for `preview_text()` and `OutputPreview`.
- Add to the DAG (`#region`): `hovered_channel()` and `selected_channels()` decoding `engine.hover` / selection handle ids via `handle_key_map` (`"node_id:port_id"`, [dag/lib.rs:2442](mathematical/graph/port/directed/dag/lib.rs)) into `{ widgetId, port, direction }`, where direction is resolved by checking `node.inputs()` vs `node.outputs()`. Expose matching `hoveredChannel()` / `selectedChannels()` on the WASM `FlowSession` (near `hovered_widget_id` line 1864).
- Extend existing Rust tests for the new channel JSON shape and `hovered_channel()` decode.

### 3. Flow react: emit channels to host ([flow/react/index.tsx](flow/react/index.tsx))

- `evaluate()` (line 2299) now forwards the channel-structured JSON via `onEvalOutputs`.
- Add `onChannelHoverChange?(channel | null)` and include selected channels in the interaction-state emit, sourced from the new session accessors, wired into the pointer/hover path that already calls `onHoverChange`.

### 4. Procedural react: registry + channel-aware items ([procedural/react/index.tsx](procedural/react/index.tsx))

- Extend `ProceduralPreviewItem` (line 960) with `port: string` and `direction: "in" | "out"`.
- Replace `extractPreviewItems` (line 1794) with a registry: `registerPreviewExtractor(fn)` + `extractChannelPreviewItems(channelJson)` that walks each widget's `in`/`out` channels and emits items tagged with `widgetId/port/direction`. Geometry-ref, point, and vector extractors are the built-in registrations (this is the reusable "register preview" mechanism).
- Update `ProceduralPreview` filter (line 1618): default `everything` shows `direction==="out"` items; node hover/select shows all channels for that widgetId; port hover/select shows the single matching channel. Add `hoveredChannel`/`selectedChannels` props (alongside existing `hoveredNodeId`/`selectedNodeIds`).
- Extend the `import.meta.vitest` block for registry extraction and channel filtering.

### 5. Procedural play controller + playground wiring ([procedural/play/index.ts](procedural/play/index.ts), [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx))

- Store `hoveredChannel` / `selectedChannels`; extend `setHover`/`setSelection` (lines 1341, 1275) to accept optional channel payloads; add getters and feed them into `ProceduralPreview` props. `setEvalOutputs` (line 1264) uses `extractChannelPreviewItems`.
- Update the existing play tests (e.g. `setEvalOutputs stores preview items per widget`, line 1717) to the new channel JSON shape and item fields.

### 6. Robust geometry operations ([geometry/brep/js/index.ts](geometry/brep/js/index.ts))

- Make `offsetSync` (line 979) accept profile inputs the way `extrudeSync` special-cases drawings (line 929): if the entry is a `drawing` (or `edge`/`wire`), derive its wire/face (`profileDrawingWire` / `asDrawing`, lines 644-668) and offset that, returning the appropriate kind; keep the solid/face/wire path. So circle -> offset yields a real offset profile that previews on the output channel.
- Extend the brep `import.meta.vitest` tests to cover `offsetSync` on `sketchCircle` and `curve.circle`.

## Notes / decisions

- `session.evaluate()` JSON shape changes to channel-structured; all consumers and fixtures/tests are updated in one pass (no compatibility layer), per greenfield rules.
- All additions go into existing files using `#region`/subregion structure; no new files.
- Work proceeds inside a repo ticket (read `repo://goals`, open/reopen ticket) per workspace rules; temp logs use the `[DEBUG]` prefix.
