# Node-Graph Wire Drag — Port-to-Port Create and Cut on React (2026-09-13)

Lane: `node-graph-wire-drag`. Scope: gap #3's renderer half — the port-to-port wire drag that
`📓️generate-mode-interactions-2026-09-13.md` §4 measured dead on the React Flow canvas ("12 grab
points across both ports' rects, zoomed in and out, produced no fixture change").

## TL;DR

Wire creation and wire deletion now work from a user's pointer on the React Flow canvas, and dispatch
the guest's own narrow vocabulary instead of a whole-fixture blob. **Five defects, not one** — the
world-unit tolerance the handoff suspected was only the first of them, and it was never the whole
story:

1. **The host published one geometry for a port and grabbed a different one.**
   `entity_screen_json("handle", …)` published the port ROW rect (20 world units wide);
   `rim_handle_anchor_hit` accepted only a disc of `handle.radius + 1.5 = 6.5` world units at the
   row's outer edge. The published CENTRE sits 10 units in — **outside the grab zone at every zoom**,
   which is exactly why twelve grab points derived from the published rect never wired.
2. **Below the Normal band a port could not be pressed at all.**
   `allows_connection_hit_picking()` was false for `minimap | overview | compact`, i.e. every zoom
   below 0.80 — while the host kept publishing those ports as `visible: true`.
3. **There was no wire-deletion gesture in the engine at all.** Releasing a reconnect drag over empty
   canvas dropped the interaction and left the wire attached, so "drag a wire off its port" — the
   gesture the earlier probe was written around — could never have cut anything.
4. **The release never reached the host when the drag left the canvas.** The React graph canvas took
   no pointer capture: a press entered `InteractionMode::DrawEdge` and the release, landing over the
   outline tree beside the canvas, was delivered to that tree. Measured live before the fix.
5. **`'{nodeId}@{portId}'` does not name one handle.** A node may carry an input AND an output under
   the same port id — `extrusion-axis` carries `vector`, `x`, `y`, `z` on both sides — and
   `handle_id_for_port` answered whichever came first in the map. A press could snap to the handle 40
   world units away on the other side of the node and start a wire from the wrong end. This is what
   made the gesture flaky across probe runs even after (1)–(4).

Plus one cross-renderer vocabulary break found in passing: the wgpu renderer dispatched a removal as
`edgeId` while the guest's `disconnect` reads `synapseId` — a well-formed command the guest silently
dropped. Both sides now spell it `synapseId`.

**Laws: 4 Rust (`wire_edit`, all green, 348 filtered) + 105 green in the surrounding `directed_dag`
suite; 10 TypeScript twin assertions (`bun test`, 31 `expect()` calls).**
**Runtime: proven on 6018** — cut → `disconnect{synapseId:"e1"}`, wire gone, downstream re-evaluates;
reconnect → four-id `connect`, wire back, chain `ok`. Reproduced on four consecutive runs.

---

## 1. Root cause, file:line

### 1.1 Two authorities for where a port is

| authority | what it said | where |
|---|---|---|
| published | the port ROW rect, `x0 … divider_x` (20 world units × 14) | `🕸️dag/🦀️.rs` `entity_screen_json`, `handle_world_bounds` (was `input_port_row_hit_bounds`) |
| grabbable | a disc of `(handle.radius + 1.5).max(3.0)` world units at the row's outer anchor | `🕸️dag/🦀️.rs:4750` `rim_handle_anchor_hit` (removed) |

A 40-world-unit node splits into a 20-unit input column and a 20-unit output column, so the published
rect's centre is 10 units from the anchor and the grab disc reached 6.5. **No zoom changes that
ratio** — both scale together — which is why "zoomed in and out" made no difference and why the
handoff's world-unit hypothesis, while real, could not have been fixed by widening the tolerance
alone: at `handle.radius + 6.0 = 11` units the disc reaches *past* the row centre and steals the node
drag instead (measured: `normal_lod_input_row_drags_node_handle_anchor_starts_edge_draw` fails).

### 1.2 The LOD gate

`🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:545`

```rust
pub fn allows_connection_hit_picking(self) -> bool {
    self.uses_input_row_connection_hitbox() || self.shows_handles()   // Normal | Detail | Micro
}
```

With `LOD_ZOOM_SHIFT = 0.25` the bands are `minimap < 0.40 ≤ overview < 0.60 ≤ compact < 0.80 ≤
normal < 1.50 ≤ detail < 2.75 ≤ micro`. So **zoom 0.5 had no port hit-testing whatsoever**, while
`entity_screen_json` answered `visible: true` for every port at every zoom — the same two-authority
shape as (1.1), one layer up.

### 1.3 No cut gesture

`♾️infinite/🎲️board/🦀️.rs:1228` — the `InteractionMode::DrawEdge` release arm connected when the
release hit an endpoint and did nothing otherwise. A reconnect drag (`reconnecting: Some(edge)`)
released over empty canvas therefore left the wire exactly as it was.

### 1.4 No pointer capture

`🕸️NodeGraph/🟦️.tsx` — the graph canvas's `onPointerDown` never called `setPointerCapture`. Measured
on 6018 with a temporary host-side trace:

```
58857 [DEBUG] dag port press … handle=Some("extrusion-axis@z") interaction=draw-edge
        (no `dag pointer up` line — the release landed on the outline tree)
```

Cutting a wire means dragging it *away*, so the one gesture that most needs to leave the canvas was
the one that could never finish.

### 1.5 A port id names two handles

`🕸️dag/🦀️.rs` `handle_id_for_port` searched `handle_key_map` (hid → `"{nodeId}@{portId}"`) by name
only. Live anchors for one node, read off the running host:

```
extrusion-axis@vector#16@(-85.3,-144.5)  x#17  y#18  z#19      ← inputs,  left edge
extrusion-axis@vector#20@(-45.3,-144.5)  x#21  y#22  z#23  errors#24   ← outputs, right edge
```

Four port ids exist on both sides. A name-only lookup answers the lower id, so an OUTPUT rect
resolved to the INPUT handle, `connection_hit_world` snapped the press 40 world units across the
node, and the engine began a wire from the wrong end — journalled against the wrong port
(`targetPortId: "y"` for a press on `z`, observed twice before this was fixed).

---

## 2. The fix, at the owning layer

Everything except the pointer capture is in the shared `DagHost`/board engine, so both renderers get
it.

| file | change |
|---|---|
| `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs:202` | new `DAG_PORT_CONNECTOR_ROW_SHARE = 0.4` + `input_port_connector_bounds` / `output_port_connector_bounds` — the outer 40 % of a port row, grown outward past the node edge by the painted cap's radius. **One rect**, used by both the hit test and the published geometry. The row's remaining 60 % stays node body: a computation node has NO header row (`DAG_COMPUTATION_HEADER_ROWS` is 0, its name is painted above the rectangle), so making the whole row grabbable would leave nothing to drag the node by. |
| same, `port_pointer_handle_hit` (replaces `rim_handle_anchor_hit`) | the ONE port hit: the connector rect. The `handle.radius + 6.0` anchor disc is gone from the press path — it reached 11 units into a 20-unit column and swallowed the node drag. |
| same, `entity_screen_json` → `handle_world_bounds` | publishes the CONNECTOR rect, not the row |
| same, `handle_id_for_port_side` | resolves a port rect to the handle on ITS side, through the `HandleRole` the engine already records — no second map, no new retained state |
| same, `port_connector_handle_hit`, `port_row_handle_hit`, the two row-chrome paint sites | all side-aware now (the paint sites read the *other* side's selection/hover chrome before) |
| same, `connection_hit_world_while_wiring` | wire snapping applies only while a wire is in flight. Snapping during a node DRAG teleported the node onto a port anchor (caught by `dag_host_drags_node_in_world_space`) |
| `🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs:537` | `allows_connection_hit_picking()` is now `self != Minimap`; `uses_input_row_connection_hitbox()` deleted (its only two readers are gone). Only the whole-graph silhouette withholds ports — that tier owns the bounded selection-AABB drag instead. |
| `♾️infinite/🎲️board/🦀️.rs:1247` | releasing a reconnect drag over no endpoint REMOVES the wire — the reconnect gesture's other outcome, and the only pointer route to deleting one |
| `🌊️flow/🖥️host/🦀️.rs:1081` | new `take_graph_edits_json()` — the journal in the guest's sub-operation vocabulary |
| `🌊️flow/🕸️wasm/🦀️.rs` `FlowAction2587` | `pointerUpScreen` now RESOLVES with that payload. No new ABI operation, no schema/argument change, no second round trip: the gesture answers with what it did. |
| `🕸️NodeGraph/🟦️.tsx` `onPointerUp` | dispatches those operations as `nodeGraphEdit` when the gesture touched a wire, and falls back to `commitFixture()` only when it did not — React stops re-publishing the whole fixture for a wire |
| `🕸️NodeGraph/🟦️.tsx` `onPointerDown`/`onPointerUp` | `setPointerCapture` / `releasePointerCapture` |
| `🕸️dag/🦀️.rs` `DagGraphEdit::Disconnect` + `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` `write_graph_edit_action` | `edge_id` → `synapse_id`, wire key `edgeId` → **`synapseId`**, the name the guest's own `disconnect` reads |

### Deliberate behaviour changes

- A press on a port row's outer 40 % now begins a wire instead of dragging the node. Three unit tests
  pinned the old split and were rewritten or re-titled, not deleted: `only_the_silhouette_lod_withholds_port_hit_picking`
  (was `hidden_lod_connection_hit_picking_disabled`, which asserted overview/compact refuse ports),
  and the two row-centre drag tests now carry a note saying the row splits in two.
- `try_node_rectangle_pointer_down`'s channel-row select branch is gone: a port-row press is claimed
  by the handle path first, and the engine's own `HitObject::Endpoint` arm already selects that
  channel before it begins the wire. It was a second implementation of the same pick.

---

## 3. Laws

### Oracle — `♾️infinite/…/🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json` (extended, not forked)

Three new declared rules — `publishedPortGeometryIsGrabbable`, `detachCutsTheWire`,
`portSideIsPartOfAPortsIdentity` — plus `disconnectNamesTheSynapse`; a `grab` section with three zoom
bands (0.5 / 1 / 2) × five cases; and a fourth graph node `dual` carrying an input and an output
under the same port id `v`.

### Rust — `…/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`

`cargo test -p semio-framework-os-infinite --lib -- wire_edit` → **4 passed; 0 failed** (348 filtered).

| test | what it proves |
|---|---|
| `the_port_geometry_the_host_publishes_is_the_geometry_that_grabs_a_wire` **(new)** | at zoom 0.5, 1 and 2: a press at the centre of the rect `entity_screen_json("handle", …)` publishes resolves `DrawEdge`; a drag between two published centres journals `connect{src,out,tgt,in}` and leaves one live wire; dragging a wired input onto empty canvas journals a `disconnect` and leaves none; dragging an unwired port there changes nothing; and each side of the shared port id `dual@v` grabs the handle on its own side (asserted through `HandleRole`) |
| `a_port_to_port_drag_creates_a_wire_and_journals_it_for_the_guest` | unchanged, still green |
| `a_wire_in_flight_holds_the_screen_pointer_path` | unchanged, still green |
| `a_minimap_click_moves_the_camera` | unchanged, still green |

The law drives the REAL `DagHost` — real published geometry, real hit test, a real
`InteractionMode::DrawEdge`, the real journal. It presses the published rect's CENTRE, which is the
only point a caller that trusts the host can derive; that single choice is what made the old
behaviour fail and the new behaviour pass.

### Surrounding suite

`cargo test -p semio-framework-os-infinite --lib -- directed_dag` → **105 passed; 2 failed**. The two
failures are **pre-existing and untouched by this lane**:
`dag_host_slider_overlay_preserves_language_neutral_field_labels` (`Number(Float(0.0))` vs
`Number(UInt(0))` — an `os_pack` JSON number-typing change) and
`selected_nodes_cursor_censuses_and_emits_one_byte_per_grant` (census 20 vs 16 bytes). Neither
touches pointer input, hit geometry or the wire journal; this lane changed no code they reach.

### TypeScript twin — `📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts`

`bun test` → **10 pass; 0 fail; 31 expect() calls**. Four assertions are new, and each reads the
rule out of live source rather than restating it, so a rename on either side fails the test:

- the released gesture dispatches its own wire edits and falls back to the fixture commit only when
  it made none (read out of `🕸️NodeGraph/🟦️.tsx`'s `pointerUpScreen` handler);
- the canvas takes and releases pointer capture around the gesture;
- both renderers spell a removal the way the guest reads it — the field name is extracted from the
  guest's `"disconnect" =>` arm and then required of the wgpu builder and of the oracle's rule text;
- the grab section covers zooms 0.5 / 1 / 2 and both gesture kinds.

### wgpu laws kept green

`cargo test -p semio-framework-ui --features testkit --lib -- control_commit` → 2 passed;
`… -- retained_hit` → 2 passed; `cargo check -p semio-framework-os-renderer-wgpu --lib` → clean.

---

## 4. Runtime proof — 6018

`http://127.0.0.1:6018/?plugin=generation3d`, `🐍️flow-window-probe.mjs` mode `wire`, artifacts in
`🗑️generated/flow-window-audit/wire-drag-final/`. The probe aims **only** through
`window.__semioFlowGraphProbe[surfaceId].entity("handle", …)` — the host's own published geometry —
and presses the rect's centre.

Graph: `height@number → extrusion-axis@z` (synapse `e1`), at the boot camera (LOD `normal`,
zoom 0.9165). Published connector rect: `11.92 × 12.83` px (13 world units × zoom), where the same
port published `18.33 × 12.83` before the fix — the row.

```
[DEBUG] dag port press port=Some("extrusion-axis@z") interaction=draw-edge(reconnect)
[DEBUG] dag edge removed id=1
[DEBUG] node graph wire edit dispatch [{"operation":"disconnect","synapseId":"e1"}]
        wires: 6 → 5   (height@number -> extrusion-axis@z is gone)
        downstream: extrusion-axis "computing", extrude "queued"   ← the chain re-evaluated
[DEBUG] dag port press port=Some("height@number") interaction=draw-edge(new)
[DEBUG] dag edge connected id=106
[DEBUG] node graph wire edit dispatch
        [{"operation":"connect","sourceNodeId":"height","sourcePortId":"number",
          "targetNodeId":"extrusion-axis","targetPortId":"z"}]
        wires: 5 → 6, the wire re-appended at the END of the list — rebuilt by the guest's
        `connect`, not restored from a stale fixture
        downstream: every node back to "ok"
```

Both dispatches carry the guest's four-id / synapse-id vocabulary, and the wire list is read from the
GUEST's published fixture, so the guest applied them. Reproduced on four consecutive runs after the
side-resolution fix (1.5); before it, two of five runs journalled the wrong port.

### Two workflow traps worth recording

1. **The React Flow session is NOT in the plugin wasm.** `createFlowSession` loads
   `@semio-tech/flow-core` — `🌊️flow/🫀️core/🕸️bindings/flow_core_bg.wasm`, its own 41 MB module built
   by `nx run semio-framework-os-flow-core:wasm`. `activate-generation3d-react-dev` (14m 47s) does
   **not** rebuild it, and that target's nx inputs glob only `🌊️flow/📦️packages/🦀️rust/**/*.rs`, so
   edits under `🌊️flow/🖥️host/`, `🌊️flow/🕸️wasm/` or `♾️infinite/` do not invalidate its cache:
   `--skip-nx-cache` is required. Rebuild is ~60–95 s, and the running vite serves the new bytes
   without a restart.
2. **The 6018 vite did not see a NodeGraph.tsx write.** Two probe runs measured the pre-edit module
   (verified by fetching the transformed module from the dev server and grepping for the new source).
   `touch`-ing the file invalidated the transform; the edits were served on the next request. Fetch
   the module and grep it before trusting a React-side runtime result.

---

## 5. Files

Changed:
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`;
`…/♾️infinite/🎲️board/🦀️.rs`;
`…/♾️infinite/🗿️artifacts/🕸️dag/🧬️schema/📸️snapshot/🦀️.rs`;
`…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs`;
`…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`;
`…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json`;
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`;
`…/🌊️flow/🕸️wasm/🦀️.rs`;
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`;
`…/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`;
`…/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts`;
`T/🐍️flow-window-probe.mjs`.

Added: `T/📓️node-graph-wire-drag-2026-09-13.md` (this file).
Removed: nothing.

Left in place deliberately: two `[DEBUG]` lines that are the only trace a wire gesture leaves inside a
Worker — `dag port press port=… interaction=draw-edge(reconnect|new)` in `DagHost` (fires only when a
port is actually grabbed) and `node graph wire edit dispatch …` in `NodeGraph`. They match the
existing permanent `dag edge connected` / `dag edge removed` lines beside them.

---

## 6. What is NOT claimed

- **Zoom coverage at runtime.** The browser proof ran in the `normal` band (zoom 0.9165). The
  probe's wheel crosses LOD bands faster than it can be stopped in one — the host pins the tier for
  the duration of a wheel gesture — so the low-zoom phase landed in `minimap` (0.11–0.12), where
  ports are withheld **by design** and the gesture correctly did nothing. Zoom 0.5 / 1 / 2 are
  covered deterministically by the Rust law over the real `DagHost`, not by the browser.
- **The wgpu side of the `synapseId` rename is not runtime-proven.** It compiles, and the TS twin
  pins renderer and guest to the same field name, but no 6118 run was taken by this lane.
- **Node dragging by a port row is intentionally gone** for the outer 40 % of the row. The interior
  still drags, and `dag_host_drags_node_in_world_space` plus the two row-centre tests cover it, but no
  human has driven a node drag on 6018 since the change.
- The two pre-existing `directed_dag` failures in §3 were not investigated beyond establishing that
  this lane does not touch the code they exercise.
- `moveMediaNode`, the `Diagram` SSR fallback's `onConnect`, and the minimap gestures were not
  re-measured; they were already covered by the two lanes named in §1.
