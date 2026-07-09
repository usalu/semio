---
name: Finish DAG Interactions
overview: Make DAG nodes draggable and DAG edge endpoints (in/out ports) reconnectable by extending the shared generic GraphEngine with rectangle ports + a wire-drag/reconnect interaction, wiring screen->world + fixture sync into DagHost, and extending the demo fixture with a richer multi-input/output diamond DAG.
todos:
 - id: engine
   content: "Extend GraphEngine (mathematical/graph/lib.rs): NodeShape+rect node, HandleRole, shape-aware handle_position/hit_test, enforce_acyclic + cycle check, remove_edge, DrawEdge interaction (down/move/up commit+reconnect), pending_edge in RenderSnapshot; update engine tests."
   status: completed
 - id: daghost
   content: "DagHost: store viewport + screen_to_world in pointer handlers; rect nodes + handle roles + acyclic; rectangle-convention port angles; engine<->fixture sync (node x/y + edges); paint port dots + preview wire; [DEBUG] logs."
   status: completed
 - id: session-react
   content: Forward viewport from DagSession set_size/attach to host.set_viewport; add [DEBUG] fixtureJson log after pointerup in DagCanvas.
   status: completed
 - id: fixture
   content: Extend demo fixture to 6-node multi-IO diamond in demo.dag.json and DAG_DEFAULT_FIXTURE; update count assertions in dag/lib.rs, dag/play/index.ts, dag/react/index.tsx.
   status: completed
 - id: validate
   content: Extend validate-dag-runtime.mjs to drive drag + reconnect via pointer events and assert fixture mutation; run cargo test, dag vitest, and runtime probe.
   status: completed
isProject: false
---

## Finish DAG: Draggable Nodes + Reconnectable Ports

### Context / root causes found

- `DagHost::pointer_down/move/up` ([dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) lines 389-399) forward **raw screen pixels** to the engine, but `GraphEngine::hit_test` works in **world space** and `DagHost` stores no viewport -> clicks never hit nodes (drag is effectively dead).
- Node rectangles/labels are painted from `self.fixture.nodes` (static) while the engine's `DragNode` only mutates `engine.nodes[].center` -> rectangles never move.
- `InteractionMode` ([mathematical/graph/lib.rs](mathematical/graph/lib.rs) line 93) is only `DragNode | Idle` -> **no reconnection mechanism exists**.
- Ports use circle geometry (`handle_position_on_circle`) floating on an invisible radius, not on the rectangle left/right edges where input/output labels are drawn.
- `GraphEngine` is consumed only by DAG and `flow/core` (`DagBoardEngine`) + round-trip tests; puzzle-2d `BoardHost` is independent. So engine changes are safe **if additive** (keep `create_node`, `create_handle`, `create_edge`, `handles` map intact for flow).

### 1. Extend generic engine — [mathematical/graph/lib.rs](mathematical/graph/lib.rs)

- `Node`: add `shape: NodeShape` (`Circle | Rectangle`), `width`, `height`. Keep `radius`. `create_node(...)` stays circle (additive); add `create_rect_node(id,x,y,w,h,draggable)`.
- `Handle`: add `role: HandleRole` (`Source | Target | Any`, default `Any`); add `set_handle_role(id, role)` (keep `create_handle` signature so flow is unaffected).
- Make `handle_position` and `hit_test` shape-aware: rectangles use `handle_position_on_rectangle` and rect containment instead of circle distance.
- Add `pub enforce_acyclic: bool` (default false). Add internal node-level cycle check (derive node ids via `handle.node_id`) reused for live edits.
- Add `remove_edge(id)`.
- New interaction variant `DrawEdge { anchor_handle: HandleId, anchor_is_source: bool, cursor: Point, reconnecting: Option<EdgeId> }`:
  - `pointer_down` on a handle endpoint: if it's a `Target` (input) with an existing incoming edge -> start `DrawEdge` anchored at that edge's `Source`, `reconnecting=Some(edge)`; otherwise start a fresh `DrawEdge` from the clicked handle.
  - `pointer_move`: update `cursor`, refresh hover candidate.
  - `pointer_up`: hit-test handle under cursor; commit only if `is_valid_connection` (Source->Target, different nodes, no duplicate, and if `enforce_acyclic` not a cycle). On commit: remove `reconnecting` edge if set, then `create_edge`. On invalid drop: cancel (existing edge restored). Emit a `BoardEvent::EdgeConnected/EdgeRemoved`.
- Expose preview: snapshot/`interaction` already `pub`; add the draft segment endpoints to `RenderSnapshot` (e.g. `pending_edge: Option<(Point, Point)>`) for the host to draw.
- Update the engine's exhaustive `match self.interaction` arms and `#[cfg(test)]` tests (add reconnect + acyclic-reject cases).

### 2. DagHost wiring — [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs)

- Store viewport (`width,height,dpr`); add `set_viewport(...)`. Convert pointer coords with `cavas::camera::screen_to_world` inside `pointer_down/move/up` (treat incoming args as screen px).
- In `rebuild_engine`: use `create_rect_node(... node.width, node.height ...)`, set `enforce_acyclic = true`, assign handle roles (inputs=`Target`, outputs=`Source`). Add a **rectangle-convention** port-angle helper (new fn, leave `io_node_handle_angles` untouched so flow keeps working) so port dots land exactly on the left/right edges at the label rows.
- Keep maps: `engine NodeId -> fixture index` and `HandleId -> "nodeId:portId"`. After `pointer_move`/`pointer_up`, sync engine node centers back into `fixture.nodes[i].x/y`, and rebuild `fixture.edges` from `engine.edges` so `fixture_json()` reflects drags + reconnects.
- Paint: draw port dots at rect-edge positions, color source/target, draw the in-progress `pending_edge` preview during a wire drag.
- Add `[DEBUG]` logs (node moved, edge reconnected/created/rejected) for runtime validation.

### 3. WASM session + React — [dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) `wasm_session`, [dag/react/index.tsx](mathematical/graph/port/directed/dag/react/index.tsx)

- `DagSession::set_size`/`attach_canvas` forward viewport to `host.set_viewport`. Pointer methods already forward screen coords.
- React canvas pointer handlers already wired; add a `[DEBUG]` log of `session.fixtureJson()` after `pointerup` to confirm drag/reconnect mutated the fixture at runtime.

### 4. Extend the demo fixture (richer DAG) — keep all 3 sources in sync

New 6-node diamond with multi-input/output and fan-in/out: `source` -> `scale` & `offset` (fan-out) -> `combine` (2 inputs a,b) -> `split` (2 outputs lo,hi) -> `sink`; `split:hi` left free.

- [fixture/demo.dag.json](mathematical/graph/port/directed/dag/fixture/demo.dag.json) and `DAG_DEFAULT_FIXTURE` in [dag/react/index.tsx](mathematical/graph/port/directed/dag/react/index.tsx) updated identically (6 nodes / 6 edges).
- Update count assertions: `dag_host_loads_demo_fixture` in [dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs), the test in [dag/play/index.ts](mathematical/graph/port/directed/dag/play/index.ts), and the test in [dag/react/index.tsx](mathematical/graph/port/directed/dag/react/index.tsx) to the new counts.

### 5. Validation (ticket folder only)

- Extend [validate-dag-runtime.mjs](.repo/🎫/26/06/07/EXTRACT-GENERIC-GRAPH-CANVAS-FROM-PUZZLE-2D-AND-ADD-DAG/validate-dag-runtime.mjs) to drive Playwright pointer events: drag a node and reconnect an edge endpoint, then assert the logged `[DEBUG]` fixture JSON shows changed node coords and changed edge wiring.
- Run `cargo test -p mathematical_graph_port_directed_dag` and the dag vitest via the existing `script.ts test` route; run the runtime probe against `dev:dag`.

### Risks / notes

- `InteractionMode`/`Node`/`Handle` are `pub` + re-exported; changes are additive (new variant/fields/methods) — flow uses `create_node/create_handle/create_edge/handles` only, so it keeps compiling unchanged.
- World-space hit tolerance is kept (fine at zoom ~1); can convert to screen-space later if needed.
