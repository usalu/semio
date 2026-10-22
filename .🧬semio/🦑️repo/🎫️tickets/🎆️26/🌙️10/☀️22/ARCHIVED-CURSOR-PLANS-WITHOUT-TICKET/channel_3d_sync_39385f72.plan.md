---
name: Channel 3D Sync
overview: Add channel-level (per input/output port) hover and selection to the flow editor and synchronize it bidirectionally with the 3D BREP preview in the procedural app, so hovering/selecting a channel emphasizes its geometry in 3D (input channel -> upstream geometry, output channel -> own geometry, bare node -> all outputs), and picking geometry in 3D emphasizes the matching channel in flow.
todos:
 - id: flow-engine-channels
   content: Add channel-level hover/selection getters+setters to DagHost (mathematical/graph/port/directed/dag/lib.rs) using handle_key_map + handle.role, with LOD fallback; expose via FlowHost/FlowSession in flow/core/lib.rs.
   status: completed
 - id: flowcanvas-surface
   content: Define ChannelRef in dag/react, re-export from @semio-tech/flow-react; emit onChannelHoverChange/onChannelSelectionChange and accept hoveredChannel/selectedChannels controlled props in FlowCanvas.
   status: completed
 - id: preview-per-port
   content: Add portId to ProceduralPreviewItem, make extractPreviewItems per output port, key items by (widgetId, portId), and drive chrome from hovered/selectedGeometryTargets; thread portId through pointer handlers.
   status: completed
 - id: controller-resolution
   content: Add hoveredChannel/selectedChannels state, fixture edge parsing, and resolveGeometryTargets (input->upstream, output->self, node->all outputs) plus setHoverChannel/setSelectChannels commands in ProceduralPlayController.
   status: completed
 - id: playground-wiring
   content: Wire channel props/callbacks in ProceduralPlayPaneSurfaceHost and geometry-target props + channel-aware onHover/onPick in ProceduralPreviewSurfaceHost.
   status: completed
 - id: tests
   content: Extend existing tests in flow/core, dag/lib, procedural/react, and procedural/play to cover channel sync and geometry-target resolution.
   status: completed
isProject: false
---

# Channel <-> 3D Sync

## Goal

On LODs where channels (input/output ports) are individually hoverable/selectable (DAG `detail`/`micro`), synchronize channel hover/selection with the 3D preview, bidirectionally:

- Hover/select an **output** channel -> emphasize that node's output geometry for that port.
- Hover/select an **input** channel -> emphasize the geometry feeding it (the upstream output wired into it via an edge).
- Hover/select just the **node** (body, or any lower LOD) -> emphasize **all** of its output channels' geometry.
- Pick/hover a geometry in 3D -> hover/select the matching channel in the flow (output port), falling back to node-level at low LOD.

## Architecture (where it belongs)

- `flow` (Rust + React) exposes **generic channel-level interaction** (port granularity). It must not know about geometry.
- `procedural` (React + play) owns the **channel <-> geometry mapping**, since only here do graph edges (from the flow fixture) and per-port geometry (`outputsJson`) meet.

The shared unit is `ChannelRef = { widgetId, portId }` (geometry is owned by output channels). Geometry emphasis in 3D is a set of resolved output `ChannelRef`s ("geometry targets").

## Step 1 - Flow engine: channel-level interaction (Rust)

File: [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) (`DagHost`)

- Reuse existing `handle_key_map: HandleId -> "widgetId:portId"` and `engine.handles[hid].role` (`Target`=input, `Source`=output) and existing `handle_id_for_port` (line ~2452).
- Add a serde `ChannelRef { widget_id, port_id, role }` (camelCase, role `"in"`/`"out"`).
- `hovered_channel_json()` -> the literally hovered channel when `engine.hover` is a handle and `draw_lod_for_frame().uses_channel_row_pick()`; else `null`.
- `selected_channels_json()` -> map `engine.selection.handle_ids` via `handle_key_map`.
- `set_hover_channel(widget_id, port_id: Option)`: at channel LOD set `engine.hover = handle_id_for_port(...)`; at lower LOD fall back to node hover (`set_hover(widget_id)`); `None` clears.
- `set_selected_channels_json(json)`: at channel LOD set `engine.selection.handle_ids`; at lower LOD fall back to node selection. Keep LOD fallback logic inside flow.

File: [flow/core/lib.rs](flow/core/lib.rs) (`FlowHost` + `FlowSession`)

- `FlowHost` thin delegates to the new `DagHost` methods (near `hovered_widget_id` ~1393, `set_hover` ~1418).
- `FlowSession` `#[wasm_bindgen]` exports: `hoveredChannelJson()`, `selectedChannelsJson()`, `setHoverChannel(widgetId, portId)`, `setSelectedChannels(json)` (near existing `hoveredWidgetId` ~1864, `setHover` ~1880).

## Step 2 - FlowCanvas: surface channels to React

Files: [flow/react/index.tsx](flow/react/index.tsx), [mathematical/graph/port/directed/dag/react/index.tsx](mathematical/graph/port/directed/dag/react/index.tsx)

- Define canonical `ChannelRef` type in the DAG react module and re-export from `@semio-tech/flow-react` (same pattern as `DagDrawLodKind`).
- In `emitInteractionState` (~2245) also emit `onChannelHoverChange?(ChannelRef | null)` from `session.hoveredChannelJson()` and `onChannelSelectionChange?(ChannelRef[])` from `session.selectedChannelsJson()`.
- Accept controlled props `hoveredChannel?`, `selectedChannels?` and apply them in the controlled-sync effect (~2441) via `session.setHoverChannel` / `session.setSelectedChannels`. After applying inbound props, re-run `emitInteractionState` so derived state stays consistent (controller guards prevent loops).

## Step 3 - Procedural preview: per-port geometry + channel chrome

File: [procedural/react/index.tsx](procedural/react/index.tsx)

- `ProceduralPreviewItem`: add `portId: string` to all variants.
- `extractPreviewItems` (~1794): iterate **per output port** (`outputs[widgetId] = { portId: value }`), collecting geometry refs / point / vector per `(widgetId, portId)` instead of flattening per widget. Update `previewItemKey` (~1247) to include `portId`.
- `ProceduralPreviewProps`: add `hoveredGeometryTargets?: ChannelRef[]` and `selectedGeometryTargets?: ChannelRef[]`.
- Render loop chrome (~1753): `chrome.hovered = membership(hoveredGeometryTargets, entry.widgetId, entry.portId)`; same for `selected`. Keep node-level props for marquee/gumball/preview-off only.
- Thread `portId` through `createPreviewPointerHandlers` (~1199) and `BrepPreviewLayer` so `onHover(widgetId, portId)` and `onPick(widgetId, portId, mode)` carry the output channel picked in 3D.
- `ProceduralFlowEditor` (~1852): pass through new channel props (`hoveredChannel`, `selectedChannels`, `onChannelHoverChange`, `onChannelSelectionChange`) to `FlowCanvas`.

## Step 4 - Procedural controller: resolution + bidirectional state

File: [procedural/play/index.ts](procedural/play/index.ts) (`ProceduralPlayController`)

- Parse and store `edges` (`"widget:port" -> "widget:port"`) when the fixture changes (alongside existing fixture handling) so input->upstream resolution is possible.
- New state: `hoveredChannel: ChannelRef | null`, `selectedChannels: ChannelRef[]`; getters `getHoveredChannel`, `getSelectedChannels`, `getHoveredGeometryTargets`, `getSelectedGeometryTargets`.
- `resolveGeometryTargets(channels, nodeFallbackId)`:
  - output channel -> itself `(widget, port)`.
  - input channel -> edge whose target `== widget:port` -> source `(srcWidget, srcPort)`.
  - no channel but node hovered/selected -> every output `portId` present in `previewItems` for that widget (the "all outputs" rule).
- Commands: `setHoverChannel`, `setSelectChannels` (used by both the flow pane and the 3D pane); keep node-level `setHover`/`setSelection` for node-body and other UI. Prefer channel over node-body when a channel is present. Bump `interactionRevision`; guard no-operation updates.

## Step 5 - Playground hosts wiring

File: [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)

- `ProceduralPlayPaneSurfaceHost` (~6435): pass `hoveredChannel`/`selectedChannels` from ctrl; wire `onChannelHoverChange`/`onChannelSelectionChange` -> `ctrl.run("setHoverChannel" | "setSelectChannels")`.
- `ProceduralPreviewSurfaceHost` (~6533): pass `hoveredGeometryTargets`/`selectedGeometryTargets` from ctrl; change `onHover`/`onSelect(ionChange)` to channel-aware `(widgetId, portId)` -> `ctrl.run("setHoverChannel" | "setSelectChannels")`.

## Step 6 - Tests (extend existing only)

- [flow/core/lib.rs](flow/core/lib.rs) tests (~2934): channel hover/selection getters + setters.
- [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) tests (~4713): handle role -> `ChannelRef` json, `set_hover_channel` LOD fallback.
- [procedural/react/index.tsx](procedural/react/index.tsx) tests (~1991): `extractPreviewItems` per-port output (expectations include `portId`); geometry-target chrome membership.
- [procedural/play/index.ts](procedural/play/index.ts) tests (~1673): `setHoverChannel`/`setSelectChannels`, `resolveGeometryTargets` for input->upstream, output->self, node->all outputs.

## Notes

- Greenfield: change `ProceduralPreviewItem`/`extractPreviewItems` shape directly (no back-compat), update all call sites and fixtures at once.
- First implementation action: read `repo://goals` and open/reopen a ticket via the repo MCP; keep any scratch files inside the ticket folder.
- No new `launch.json`/`project.json` commands required; validate via existing Rust tests and the procedural/flow vitest targets.
