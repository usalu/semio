# Wave 3 — Node-Graph Hover Plumbing (framework/renderer side)

## Objective
Let a plugin paint node-graph hover, including per-port (channel) hover, and let a plugin push a
`highlighted` id list (transitive hover from another window) into the graph's chrome.

## Rust — `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs`
- `NodeGraphHover` gained `port_id: Option<String>` (`#[serde(default, skip_serializing_if =
  "Option::is_none")]`, camelCase wire name `portId`) — docstring `🔌️`. When set, the graph paints
  the hovered channel on `node_id`, matching the `"{nodeId}@{portId}"` pick id.
- `NodeGraphScene` gained `highlighted: Vec<String>` (`#[serde(default, skip_serializing_if =
  "Vec::is_empty")]`, wire name `highlighted`) — docstring `✨️`. Ids — nodes, edges, or
  `"{nodeId}@{portId}"` ports — a plugin wants highlighted (e.g. transitively hovered from another
  window). `NodeGraphScene::base()` updated to set `highlighted: Vec::new()`.
- `grep -rn 'NodeGraphHover {' --include='*.rs' .` (excl. `/target/`) found only the struct
  definition itself — no literal construction sites anywhere in the repo needed fixing.
- `grep -rn 'NodeGraphScene {' --include='*.rs' .` (excl. `/target/`) found 13 non-test literal
  sites; every one already uses `..NodeGraphScene::base(...)` struct-update syntax, so the new
  `highlighted` field defaults through `base()` for free — no call site needed editing. (Two sites
  live under `✏️s/🔌️plugins/🌀️procedural/**` — flow window builders for `procedural2d`/
  `procedural3d` — left untouched per the "don't touch procedural" constraint; they also already
  use `..NodeGraphScene::base(...)` so they compile unchanged.)
- Added two unit tests in `scenes.rs`'s `mod tests`: `node_graph_hover_port_id_round_trips_...`
  and `node_graph_scene_highlighted_round_trips_...` — both assert camelCase wire names, full
  round-trip via `serde_json::to_value`/`from_value`, and omission when `None`/empty.

## TypeScript mirror — `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`
Hand-maintained parallel type (not codegen'd): added `portId?: string` to `NodeGraphHover` and
`highlighted?: readonly string[]` to `NodeGraphScene`, matching the Rust wire shape.

## Renderer — `.../NodeGraph/🟦️component.tsx`
1. **`applyNodeGraphHoverFromScene`** (was ~:1805): now calls `session.setHoverChannel(nodeId,
   portId)` when `hover.portId` is present, else falls back to the existing `session.setHover(nodeId)`.
2. **Highlighted chrome**: `DagLabelOverlayInteraction` gained `highlightIds?: readonly string[]`;
   `dagElementInteractionChrome(selectionIds, preselection, extraHighlightIds = [])` now unions
   `extraHighlightIds` into the returned `highlightedIds` set (used by `dagOverlayLabelFill`/
   `dagOverlayLabelFillHex`). Both `paintDagLabelOverlays` call sites (`WasmGraphSurface`'s
   `paintOverlays` and `NodeGraphHost`'s `paintOverlays`) now pass `highlightIds:
   sceneRef.current.highlighted ?? []` — added a `sceneRef` to `WasmGraphSurface` (mirroring the
   one already in `NodeGraphHost`) so the callback (stable `useCallback([])` identity) always reads
   the latest `scene` prop instead of closing over a stale one.
3. **Port hover → `interactionHover`**: `nodeGraphHoverActionArgs(nodeId, portId?)` now takes an
   optional `portId`; when present it emits `{ granularity: "handle", id: "{nodeId}@{portId}" }`
   instead of `{ granularity: "node", id: nodeId }`. Node-only hover (no `portId`) is unchanged.
   Both `onHoverFocus` handlers that resolve a channel via `nodeGraphPickChannel` now forward the
   port:
   - `WasmGraphSurface`'s `onHoverFocus` (~:761): `nodeGraphHoverActionArgs(hovered,
     channel?.portId)`.
   - `NodeGraphHost`'s `onHoverFocus` (~:2160, the async/wasm-task variant): added a
     `parseDagChannelRefJson` helper that decodes the `hoveredChannelJson()` `DagChannelRef`
     payload (`{widgetId, port, direction}` or `"null"`, see
     `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️component.rs:2002`)
     into `{nodeId, portId}`, and passes `hoveredChannel?.portId` to `nodeGraphHoverActionArgs`.
   Node hover keeps working unchanged when the pointer is over the node body (no channel resolved
   → `portId` is `undefined` → same "node" granularity target as before).

All signature changes are additive/optional; existing call sites (`nodeGraphHoverActionArgs("node-a")`
in `🧪️index.test.ts:2534`, `dagElementInteractionChrome` 1-arg-preselection callers, `World3dHost`'s
own hover path which is untouched) compile unchanged.

## Verification (raw output under `🗑️generated/`)
- `cargo check -p semio-framework-ui-scene --all-targets --keep-going` → **clean** (only 2
  pre-existing warnings unrelated to this change: `terminal_is_empty` dead code in `math.rs`, and
  macro-expansion warnings in `ui-contract`'s `compare/component.rs`). Saved to
  `🗑️generated/cargo-check-ui-scene-wave3-hover.txt`.
- `cargo check --workspace --keep-going 2>&1 | grep -E '^error'` → 92 errors, **all** pre-existing
  and unrelated (grep for `scenes.rs|NodeGraphHover|NodeGraphScene|ui-scene|ui_scene` over the
  error log returns zero matches). The errors are concurrent breakage in an unrelated `db_storage`/
  `DbIoAsyncDriverFuture`/`SnapshotRef` area (other devs' in-progress work — see project memory on
  concurrent workspace churn). Saved to `🗑️generated/cargo-check-workspace-errors-wave3-hover.txt`.
- `nx run @semio-tech/framework-renderer-react:typecheck` → fails, but every error is pre-existing
  and unrelated: `TutorialUiSnapshot`/`TutorialArtifactEvent`/`PluginWasmHandle` shape mismatches in
  `♻️mit-bestand/🧺️demonstrator/🟦️brand.ts`, `🧱️elements/ShellHelpers`, `🧱️elements/ShellHost`,
  `🧱️elements/WasmSessionLoader`, `🧱️elements/World3dHost`, and `🧪️index.test.ts` — none reference
  `NodeGraph/🟦️component.tsx`, `NodeGraphHover`, `NodeGraphScene`, `portId`, `highlighted`,
  `dagElementInteractionChrome`, `paintDagLabelOverlays`, or `nodeGraphHoverActionArgs` (grep over
  the log for those tokens returns zero matches). Consistent with a concurrent Tutorial/
  `PluginWasmHandle` refactor in progress elsewhere in the same project. Saved to
  `🗑️generated/typecheck-framework-renderer-react-wave3-hover.txt`.

## Files changed
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust/🦀️scenes.rs`
- `🧰️framework/🔨️modules/🔺️mesh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/NodeGraph/🟦️component.tsx`

## Out of scope (left untouched)
- `EngineCanvas/🧊️component.rs:1874-1875` — the wgpu-native node-graph host has a stale comment
  ("`NodeGraphHover { nodeId }`-only record ... nothing to sync here yet") and does not sync hover
  at all yet; not part of this wave's task list, flagged here rather than silently expanded.
- `✏️s/🔌️plugins/🌀️procedural/**` and `World3dHost/🟦️component.tsx` — owned by other agents per
  the task constraints.
