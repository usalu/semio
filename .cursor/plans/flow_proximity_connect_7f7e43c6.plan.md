---
name: Flow Proximity Connect
overview: "Add proximity-based auto-connect to the flow DAG: while dragging a node or an edge endpoint, the nearest compatible channel within a configurable world-space distance is previewed as a highlighted edge and committed on release, replacing any existing 1-to-1 edge on the target channel. The distance is exposed as a flow window option where 0 disables it."
todos:
  - id: engine-distance
    content: Add proximity_distance_world to GraphEngine and make wire-snap tolerance world-based (0 disables) in mathematical/graph/lib.rs
    status: in_progress
  - id: engine-replace
    content: Add allow_target_replace to is_valid_connection and remove existing target edge on commit (1-to-1 replace)
    status: pending
  - id: engine-nodedrag
    content: Implement node-drag proximity detection, pending_edge preview, hover highlight, and commit in pointer_move/up
    status: pending
  - id: plumb-rust
    content: Add set_proximity_distance to DagHost, FlowState, and FlowSession (setProximityDistance wasm method)
    status: pending
  - id: canvas-prop
    content: Add proximityDistance prop to FlowCanvas and apply it to session via effect (mirror automaticLod)
    status: pending
  - id: window-option
    content: Add Proximity slider WindowMeasure + setProximityDistance command in FlowPlayController and pass prop from FlowPlayPaneSurfaceHost
    status: pending
  - id: tests
    content: Extend engine, DAG host, and flow tests for proximity connect, replace semantics, and 0-disables
    status: pending
isProject: false
---

## Flow Proximity Connect

Edges are already strictly 1-to-1 in the engine (`is_valid_connection` rejects an occupied target). This adds proximity connect for both gestures (node drag + edge-endpoint drag), with replace-on-occupied semantics and a world-unit distance window option (0 = disabled).

### 1. Engine: configurable world-space proximity distance
File: [`mathematical/graph/lib.rs`](mathematical/graph/lib.rs)
- Add field `pub proximity_distance_world: f64` to `GraphEngine` (default a sensible value, e.g. `48.0`; `0.0` disables all snapping/proximity). Init in `new`.
- Change `wire_snap_drag_tolerance_world` to return `self.proximity_distance_world` (+ `handle.radius`) instead of the px-based `WIRE_SNAP_HIT_TOLERANCE_PX + WIRE_SNAP_EXTRA_PX`. When `proximity_distance_world <= 0`, snapping is disabled (callers short-circuit to `None`).

### 2. Engine: replace-on-occupied (keep 1-to-1)
File: [`mathematical/graph/lib.rs`](mathematical/graph/lib.rs)
- Add `allow_target_replace: bool` parameter to `is_valid_connection`; when `true`, skip the "target already has an incoming edge" rejection (line ~1464) but keep same-node/role/duplicate/acyclic checks.
- Pass `true` from proximity paths (`nearest_wire_snap_handle` and the new node-drag scan) so snapping onto an occupied input is allowed.
- At commit time (DrawEdge `pointer_up` and node-drag commit), if the chosen target already has an incoming edge (not the one being reconnected), remove it first (`self.edges.remove`, `selection.edge_ids.remove`, emit `BoardEvent::EdgeRemoved`) before `create_edge`.

### 3. Engine: node-drag proximity connect (new)
File: [`mathematical/graph/lib.rs`](mathematical/graph/lib.rs)
- Add `proximity_connection: Option<ProximityConnection>` to `GraphEngine` where `ProximityConnection { source: HandleId, target: HandleId, replacing: Option<EdgeId> }`.
- In `pointer_move_screen`, in the `DragNode`/`DragNodes` arms (after nodes move), call a new `update_node_drag_proximity(dragged_node_ids)`:
  - Dragged set = `{node_id}` for `DragNode`, or `drag_start_positions.keys()` for `DragNodes`.
  - For each handle on a dragged node and each handle on a non-dragged node, build the (source, target) pair by role (dragged out vs other in, or dragged in vs other out — this is the "changes source or target" behavior), check `is_valid_connection(.., allow_target_replace=true)`, and keep the globally nearest pair within `proximity_distance_world`.
  - Store result in `proximity_connection` (with `replacing` = existing incoming edge on target, if any); set `update_hover` to the candidate target handle for highlight.
- In `render_snapshot`, when `proximity_connection` is set, populate `pending_edge` with the bezier between the two handle positions (reuse `wire_bezier_between`) so the DAG paints the highlighted candidate edge.
- In `pointer_up_screen`, in the `DragNode`/`DragNodes` arm, if `proximity_connection` is set, remove the `replacing` edge, `create_edge`, emit `EdgeConnected`. Clear `proximity_connection` on every pointer up.

### 4. DagHost + FlowState + wasm plumbing
- [`mathematical/graph/port/directed/dag/lib.rs`](mathematical/graph/port/directed/dag/lib.rs): add `pub fn set_proximity_distance(&mut self, world: f64)` forwarding to `self.engine.proximity_distance_world`. Node-drag pointer flow already calls `process_engine_events` (handles `EdgeConnected`/`EdgeRemoved` → `sync_edges_from_engine`) and paints `snap.pending_edge`, so no extra paint code needed.
- [`flow/core/lib.rs`](flow/core/lib.rs): add `FlowState::set_proximity_distance(world)` → `self.dag.set_proximity_distance(world)`; add wasm `#[wasm_bindgen(js_name = setProximityDistance)] pub fn set_proximity_distance(&self, world: f64)` on `FlowSession` (in the `#[wasm_bindgen] impl FlowSession` region).

### 5. FlowCanvas option prop
File: [`flow/react/index.tsx`](flow/react/index.tsx)
- Add `readonly proximityDistance?: number` to `FlowCanvasProps`, destructure with a sensible default.
- Mirror the existing `automaticLod` effect (around line 2062): keep a `lastProximityRef`, and when it changes call `session.setProximityDistance(proximityDistance)`.

### 6. Flow window option (the "window options")
File: [`flow/play/index.ts`](flow/play/index.ts)
- Add state `private proximityDistance = <default>;` and `proximityDistanceValue(): number` getter.
- Add `proximityMeasure(): WindowMeasure` of `kind: "slider"` (label "Proximity", `min: 0`, `max: 240`, `step: 4`, `value: this.proximityDistance`, `onChange: { controllerId: FLOW_PLAY_CONTROLLER_ID, command: "setProximityDistance" }`) and include it in `windowMeasures()` next to `lodMeasure(...)`.
- Handle `setProximityDistance` in `run(...)`: read numeric `value`, clamp `>= 0`, store, `rebuildShellMode()`, `emit()`.

File: [`framework/product/playground/renderer/react/index.tsx`](framework/product/playground/renderer/react/index.tsx)
- In `FlowPlayPaneSurfaceHost`, pass `proximityDistance={ctrl?.proximityDistanceValue() ?? <default>}` to `<FlowCanvas />`.

### 7. Tests (extend existing files only)
- [`mathematical/graph/lib.rs`](mathematical/graph/lib.rs) tests: update existing wire-snap tests to the world-based tolerance; add: node-drag forms a proximity edge within distance; `0` distance disables proximity + endpoint snap; proximity onto an occupied input replaces the old edge (1-to-1); proximity respects acyclic/role/same-node rules.
- [`mathematical/graph/port/directed/dag/lib.rs`](mathematical/graph/port/directed/dag/lib.rs) tests: extend `dag_host_reconnects_edge_endpoint` area — dragging a node near a compatible channel yields `pending_edge` preview and commits a `fixture.edges` entry; `set_proximity_distance(0)` disables it.
- [`flow/react/index.tsx`](flow/react/index.tsx) / [`flow/play/index.ts`](flow/play/index.ts) tests: `setProximityDistance` command updates the window measure value; FlowCanvas forwards the prop to the session.

### Notes / decisions
- Per repo rules: open/reopen a ticket via repo MCP before editing; keep temp logs under the ticket folder; use `[DEBUG]` prefix for temporary logs; structure new code in existing files with regions.
- Default distance: pick a sensible world value (~48) so it works out of the box; the option can set 0 to disable.
- Replace-on-occupied is applied to proximity paths (both gestures), preserving the 1-to-1 invariant on input channels.