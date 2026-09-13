# 🫳️ wgpu NODE-GRAPH GESTURES — a plain move is not a mutation, and a gesture belongs to the path that started it

Lane `wgpu-node-graph-gestures`, ticket `26/09/09/PROCEDURAL-3D-END-TO-END`, 2026-09-13.

Continues `📓️wgpu-node-graph-surface-retention-2026-09-13.md` §6.4/§7 — the three things that lane could
name but not prove.

---

## 1. TL;DR

All five gestures the brief asked for are now proven on
`http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column`, in ONE run
(`🗑️generated/wgpu-node-gestures/run-12`):

| # | gesture | the runtime's own witness |
|---|---|---|
| 1 | **a node selected** | `action=interactionSelect args={"domainId":"graph","targets":"[{\"granularity\":\"node\",\"id\":\"height\"}]","merge":"replace","method":"pick"}` |
| 2 | **a node dragged** | `wgpu node-graph bounded gesture … edits=[Move { node_id: "height", x: -146.755…, y: -69.083… }]` → `action=nodeGraphEdit args={"operations":[{"operation":"move","nodeId":"height","x":-146.755…,"y":-69.083…}]}`, and the next geometry census puts `height` at `[130.0, 444.7, 71.4, 25.0]` where it had been `[40.0, 384.7, 71.4, 25.0]` — exactly the +90/+60 px the drag asked for |
| 3 | **a wire drawn** | `dag edge connected id=106 source=10 target=16` → `screen gesture … edits=[Connect { height/number → extrusion-axis/vector }]` → `action=nodeGraphEdit {"operations":[{"operation":"connect","sourceNodeId":"height","sourcePortId":"number","targetNodeId":"extrusion-axis","targetPortId":"vector"}]}` |
| 3b | **a wire cut** | `dag edge removed id=2` → `edits=[Disconnect { synapse_id: "e2" }]` → `action=nodeGraphEdit {"operations":[{"operation":"disconnect","synapseId":"e2"}]}` |
| 4 | **minimap moves the camera** | `minimap widget pointer down sx=419.4 sy=698.6 on_viewport=false` → `action=nodeGraphViewport {"viewport":{"x":132.990…,"y":-284.102…}}` |
| 5 | **a dense hover sweep never faults** | 224 of 224 swept points acknowledged by `os_host handle_event PointerMove`; `ItemCredits` **0**, `BoundedActionFault` **0**, `graph move fault` **0**, `panicked` **0** |

Three defects were found by reaching them, each fixed law-first at the layer that owns it:

1. **A plain pointer move was priced like a mutation.** Every move over a node graph published
   `interactionSelect` + `interactionHover` + `nodeGraphViewport` whether or not any of the three had
   changed. The bounded action queue holds 256 items; a hover sweep filled it faster than the frame
   loop drained it and the session then published nothing at all for the rest of its life. Measured
   before the fix (`🗑️generated/wgpu-node-gestures/base-1`): `[DEBUG] wgpu-shell graph move fault
   surface=procedural-main fault=ItemCredits`, after which **0 of 440** sweep points and **0 of 6**
   presses reached the host.
2. **A bounded gesture was stolen mid-flight.** A node drag whose pointer crossed a port, a wire
   handle or an inline widget was handed to the screen pointer path, whose
   `resync_interaction_projection` rebuilds the interaction projection as IDLE — so the drag's own
   release saw no gesture to close. No `move` was ever journalled, and the undo baseline the drag had
   armed stayed armed.
3. **A re-armed undo baseline was dropped, not retired.** `begin_gesture`/`begin_change` ASSIGNED over
   `pending_history_baseline`; a `FlowFixture` owns the fail-closed `OrderedMap<WidgetLayout>` root, so
   the overwrite aborted the whole pool worker with `ordered-map root must be explicitly retired
   before drop`. Measured on 6118 as the **fourth** middle-button pan of one session (`run-9`).

A fourth change is not a defect fix but the thing that made the runtime proof possible at all: the
renderer now publishes a **geometry census** of the graph — see §5.

---

## 2. Root causes, with file:line

### 2.1 A move that changes nothing still paid three credits

`⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`, before this lane — identically in
`node_graph_pointer_{down,move,up}_into` and in `node_graph_wheel_into`:

```rust
let mut reservation = input.reserve_actions(3, 3 * ACTION_ITEM_BYTE_CAPACITY)?;
let planned = plan_node_graph_pointer(surface_id, intent)?;
…
write_graph_interaction_actions(&mut reservation, surface_id, controller_id, snapshot)?;
```

The reservation came FIRST and was unconditional, and `write_graph_interaction_actions` wrote all three
actions unconditionally. `BoundedActionQueue::reserve_batch`
(`🖱️ui/🎯️targets/🧊️wgpu/🎬️action/🦀️.rs:642`) answers `ItemCredits` once
`len + claimed + item_credits > ACTION_QUEUE_ITEM_CAPACITY` (256, `:6`), and the queue drains one item
per host turn — so ~85 hover moves ahead of the drain is the whole budget. The fault is recorded on
`InputState` (`record_action_fault`) and returned from `take_action_step` forever after, which is why
the session went silent rather than merely dropping one hover.

React never had this problem structurally: `emitIntent` answers `undefined` for an intent it has
already sent, and `NodeGraphHost`'s hover effect is keyed on the hovered id rather than on the pointer.

### 2.2 A drag that crossed a port lost its own gesture

`⚙️EngineCanvas/…/🦀️.rs`'s `node_graph_gesture_is_screen_path` asked only
`dag.screen_pointer_gesture_active() || dag.screen_pointer_gesture_begins_at(sx, sy)`. A bounded
gesture is not a `DagHost` interaction at all — it lives in `FlowHost::interaction_projection` — so a
drag in flight was invisible to the discriminator. The moment the pointer crossed a connector,
`begins_at` answered true, the move went to `node_graph_screen_pointer_into`, and that path calls
`resync_interaction_projection()` (`🌊️flow/🖥️host/🦀️.rs:561`) whose
`bounded_interaction_projection` (`🕸️dag/🦀️.rs`) always builds
`gesture: DagProjectionGesture::Idle`. The drag's release then derived a plan with
`previous_active == false`: no `commit_gesture_history`, no journalled move.

### 2.3 Overwriting an armed baseline

`🌊️flow/🖥️host/🦀️.rs`, before this lane:

```rust
fn begin_gesture(&mut self) {
    self.flush_pending_change();
    self.pending_history_baseline = Some(self.fixture.clone());   // ← drops whatever was armed
    self.gesture_active = true;
}
```

`flush_pending_change` only takes the baseline when `pending_change` is set, so a gesture that armed
one and never closed it (§2.2 is exactly such a gesture) left it armed, and the next press dropped it.
`begin_change` (`:2207`) and the history reset inside `apply_fixture` (`:305`) had the same shape.

---

## 3. The fixes, per file

### 3.1 `//#region 📌️GraphInteractionChange` — `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`

| item | what it is |
|---|---|
| `PublishedGraphInteraction { select, hover, viewport }` | three FNV-1a fingerprints, 24 bytes, stored on the surface itself |
| `graph_payload_digest(parts)` | FNV-1a over the payload's own bytes — the change witness is the dispatched payload, never a hand-maintained shadow of the fields that feed it |
| `graph_interaction_dispatch(published, snapshot)` | builds the three payloads once and says which of them this surface has not already told the guest |
| `write_graph_interaction_actions(batch, …, &dispatch)` | writes only the changed ones |
| `node_graph_bounded_publish(…)` | the bounded path's ONE dispatch point: plan → decide → reserve exactly `items` → publish atomically with the commit. **`items == 0` takes no reservation at all** and simply commits the plan |
| `node_graph_screen_pointer_into(…)` | keeps its 4-credit PRE-reservation for every phase that can still mutate the graph (a press, a release, or a move while a wire draw / minimap drag / widget drag is in flight) and pays after the fact only for a plain hover, which can produce no edit |

The invariant the change detection must not weaken — *a bounded dispatch never mutates what it cannot
then publish* — is therefore intact where a mutation is possible, and simply does not apply where none
is.

### 3.2 A gesture belongs to the path that started it

- `🕸️dag/🦀️.rs` — `DagInteractionProjection::gesture_active()`.
- `🌊️flow/🖥️host/🦀️.rs` and `🗺️surface/🕸️node-graph/🦀️.rs` — `bounded_pointer_gesture_active()`.
- `⚙️EngineCanvas/…/🦀️.rs` — `node_graph_gesture_is_screen_path` returns **false** while a bounded
  gesture is in flight, before it ever asks about the point.

### 3.3 `arm_history_baseline` — `🌊️flow/🖥️host/🦀️.rs`

```rust
fn arm_history_baseline(&mut self) {
    if let Some(stale) = self.pending_history_baseline.take() {
        stale.retire_cold();
    }
    self.pending_history_baseline = Some(self.fixture.clone());
}
```

Called by `begin_change` and `begin_gesture`; `apply_fixture`'s history reset retires the same way.

### 3.4 The `move` edit the guest already understood

`DagGraphEdit` gains `Move { node_id, x, y }` (`🕸️dag/🦀️.rs`), journalled by `plan_graph_edits(plan)`
from the PLAN — never from the committed host, because a bounded dispatch has to know every byte it
will publish before the mutation is allowed to happen. It fires once, on the release that ends a drag,
and a plan whose pointer never left the drag's own origin (`DagPointerPlan::dragged`) is not an edit at
all, so a plain click does not spend a guest mutation. `write_graph_edit_action` writes it as
`{"operation":"move","nodeId","x","y"}` — the very shape generation3d's `node-graph-edit` command reads
back, and the one React's SSR `Diagram` fallback dispatches from `onNodeDragStop`.
`FlowHost::take_graph_edits_json` carries the same variant.

The bounded path now writes `nodeGraphEdit` too; before this lane only the screen path did, so a wire
could reach the guest and a drag could not.

---

## 4. Laws

### 4.1 Node-graph gestures (new)

- **Oracle**: `📺️renderer/🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json` — 10 declared rules, 9
  cases over generation3d's own `hexagonal-mushroom-column` payload (the same graph the serve boots).
- **Rust law**: `⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs`, driving the REAL
  `node_graph_pointer_{down,move,up}_into` over a REAL `FlowHost`. Every coordinate is DERIVED from the
  live host (`DagScreenHit::is_draggable_body`, the published connector rect), never authored.
  `cargo test -p semio-framework-os-renderer-wgpu --lib -- node_graph` → **13 passed; 0 failed**
  (552 filtered), the 4 pre-existing attach laws included.

| test | what it proves |
|---|---|
| `a_hover_that_changes_nothing_publishes_nothing_and_never_exhausts_the_action_queue` | the first hover over a node publishes ≤3 actions; each of the next **400** publishes **0** and none faults |
| `a_dense_sweep_across_the_whole_surface_survives_an_action_queue_that_is_never_drained` | a 24×24 sweep on ONE never-drained `InputState` is admitted at every point and leaves the queue under its 256-item capacity |
| `a_press_on_a_node_body_selects_that_node_and_a_released_drag_publishes_its_move` | the press selects exactly `extrude`; the 72×48 drag publishes exactly one `move` carrying `nodeId`/`x`/`y` and the node's published rect really moved; a zero-delta click publishes none |
| `a_drag_that_crosses_a_port_keeps_the_bounded_path_and_still_publishes_its_move` | §2.2 — the crossed point is asserted to really be a connector first |
| `an_output_to_input_drag_publishes_connect` | `sides@number` → `extrusion-axis@y` publishes one `connect` with all four ids |
| `detaching_a_wired_input_from_its_port_publishes_disconnect` | dragging `extrusion-axis@z` off its port publishes `disconnect{synapseId:"e1"}` and the synapse leaves the host's own fixture |
| `a_minimap_press_publishes_the_camera_it_moved` | a press inside the panel and outside its viewport rectangle re-centres the camera and publishes exactly that camera |

- **TypeScript twin**: `📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts` — **10 pass; 0 fail;
  53 expect() calls**. It pins the two halves a Rust law cannot: every sub-operation's field names are
  read out of generation3d's own `node-graph-edit` match arms and compared against what the wgpu action
  builder writes, and the ADMISSION RULE is pinned as a property of the dispatch code's shape (the
  change decision precedes the reservation; `items == 0` reserves nothing; the screen path keeps its
  pre-reservation behind `may_mutate`).

Both fixes were written law-first and the laws were **run RED before the fix**:

- with `if bounded_gesture { return false; }` disabled →
  `a_drag_that_crosses_a_port…` fails with `the drag survived crossing extrude@wire and published its move, got []`.
- with `arm_history_baseline` replaced by the old assignment →
  `arming_a_second_history_baseline_retires_the_first` aborts with
  `panicked at 🗂️ordered/🦀️.rs:81:57: ordered-map root must be explicitly retired before drop`.

### 4.2 Baseline retirement (extended)

`🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs` —
`arming_a_second_history_baseline_retires_the_first` drives eight bounded pan presses with a
`resync_interaction_projection()` between them, the exact interleaving §2.2 produces.
`cargo test -p semio-framework-os-flow --lib -- retirement` → **10 passed; 0 failed** (212 filtered).

### 4.3 Neighbours kept green

| command | result |
|---|---|
| `cargo test -p semio-framework-os-infinite --lib -- wire_edit` | **4 passed; 0 failed** |
| `cargo test -p semio-framework-os-renderer-wgpu --lib -- node_graph` | **13 passed; 0 failed** |
| `cargo test -p semio-framework-os-flow --lib -- retirement` | **10 passed; 0 failed** |

The renderer suite and the twins were re-run last, after every fix, and are green on the tree as it
stands. The `wire_edit` and `retirement` suites were green on the same code twenty minutes earlier and
could not be re-run at the very end: `semio-framework-ui` stopped compiling on a peer's live
`World3dSceneLaneRef` rename (`🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:360`), which twelve consecutive polls
did not clear. Neither suite's code path was touched after its green run.
| `bun test` (gestures, wire-edit, retained-control-commit, engine-surface-retention twins) | **45 pass; 0 fail; 189 expect() calls** |
| `nx run @semio-tech/framework-renderer-wgpu:wasm` | `Successfully ran target`, dist `2026-09-13 23:35` |

The wire-edit twin's discriminator assertion was updated rather than forked: it now pins **three**
phase questions asked with `Some((sx, sy))` plus **one** asked with `None` — the screen-path
dispatcher's "is a gesture already in flight?", which is what tells a plain hover from a move inside a
live wire draw.

---

## 5. The geometry census — why the probe stopped sweeping

`DagHost::screen_geometry_census_json()` (`🕸️dag/🦀️.rs`) reports every node and port this graph paints,
each with the screen geometry `entity_screen_json` publishes for it, plus two answers only the host can
give: the point inside each node that a press would actually DRAG (`DagScreenHit::is_draggable_body`,
scanned over the node's own rect by `first_screen_point_in`) and the minimap point that NAVIGATES
rather than grabbing the viewport rectangle. The renderer emits it as
`[DEBUG] wgpu node-graph geometry surface=… entities=[…]`, once per CHANGE of that geometry
(`log_graph_geometry_census`, digest-gated exactly like the hit trace).

That was not a convenience. Finding the graph by sweeping the pointer across it **does not work on this
serve**: measured four times running — `run-2`, `run-3`, `run-4`, and the PREVIOUS lane's own
`🐍️wgpu-node-graph-retention-probe.mjs` re-run unchanged minutes later — a 224-440 point hover sweep had
**0-1** of its moves acknowledged and found no port and no node at all, while
`🐍️wgpu-press-admission-probe.mjs` (boot, stand still, five clicks, 54 s) had **5/5** presses admitted on
the same serve in the same minutes. A sweep spends the host's whole attention budget on looking.

With the census the probe spends its budget on gestures instead, and the sweep moves to the END of the
run where it is a measurement rather than a prerequisite.

The second diagnostic, `[DEBUG] wgpu node-graph hit surface=… sx=… sy=… trace={…}`, publishes the ONE
classification the path discriminator itself routes by — node / draggable / handle+direction / widget /
minimap / minimapViewport / portInsert — once per CHANGE under the pointer. Both lines carry their own
coordinates, because the worker's console is batched and lagging: a 105 s run's stream had only reached
t=44 s when the browser closed, so a per-point `lines.slice(mark)` window sees almost nothing.

---

## 6. The probe, and how each gesture's target is derived

`🐍️wgpu-node-gestures-probe.mjs` (ticket root; the derivation is documented in its header).

1. **Port direction.** The hover channel the runtime publishes is `{"granularity":"handle","id":"…"}`
   and carries no direction — React's `nodeGraphHoverActionArgs` does not either, so the wgpu action
   must not invent one. Directions come from the live census, cross-checked against the committed
   fixture `⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json` by two purely structural rules: a
   widget's `inputPorts[]` are that node's inputs, and a synapse `{from, fromPort, to, toPort}` names
   `from@fromPort` an output and `to@toPort` an input.
2. **Connect candidates** are `output@A → input@B` with `A ≠ B`, no `B → … → A` path in the synapse
   graph (the relation `Board::is_valid_connection` enforces), and both connectors at least 9 px wide —
   a narrower one is not reliably grabbable, and `run-7` proved it: eight wheel-outs left every
   connector ~5 px and every "wire" gesture started as a node drag.
3. **The draggable body point** comes straight off the census.
4. **Bringing the graph into view.** `Fit graph` is clicked first, then the probe PANS with the
   surface's own middle-button verb by exactly the translation the census says is needed
   (`run-9`/`run-12`: dx 353 → 193 → 97 → 33, nodes on screen 1 → 6). Zooming out until everything
   fits overshoots, and the wheel sometimes moves nothing at all (`run-8`).
5. **Every gesture re-reads a census published AFTER the last camera move** (`freshCensus`). Without
   that, `run-10` aimed ~30 px stale and the press that should have selected `radius` landed on empty
   canvas and cleared the selection instead.

---

## 7. 6118 — the numbers

`🗑️generated/wgpu-node-gestures/{base-1 … run-12, liveness-1, retention-now}`; each holds `console.txt`,
`verdict.json` and `shot.png`.

| | before (`base-1`) | after (`run-12`) |
|---|---|---|
| sweep points acknowledged | 118 of 440, then the fault | **224 of 224** |
| `ItemCredits` | **1** (at t=44.19 s) | **0** |
| `graph move fault` | 1 | **0** |
| `handle_event PointerDown` after the sweep | **0 of 6** | **9 of 9** |
| `action=interactionSelect` with a node | 0 | **1** (`height`) |
| `action=nodeGraphEdit` | 0 | **3** (`move`, `connect`, `disconnect`) |
| `dag edge connected` / `removed` | 0 / 0 | **1 / 1** |
| `shell node-graph fit` | 0 | **1** |
| `panicked` | 0 | **0** |

`run-9` is the run that found §2.3: `panicked: 1` —
`wgpu-worker panicked: panicked at 🗂️ordered/🦀️.rs:81:57: ordered-map root must be explicitly retired
before drop`, immediately after `os_host dispatch_normalized_event PointerDown { … button: Middle }`,
on the fourth pan of the session. `run-10`, on the same choreography with the fix, has `panicked: 0`
and ten admitted presses.

---

## 8. What is NOT claimed

- **The `Fit graph` control does not re-fit a graph that has been re-laid-out.** Measured in `run-6`
  and again in `run-12`: it publishes `{"x":94.7558…,"y":-97.5083…,"zoom":1.7844…}` — the scene's own
  fixture camera — while five of seven nodes sit at negative surface x because the graph grew after the
  guest's first evaluation. The control WORKS (it dispatches, and the camera it names is applied); what
  it computes is stale. Not diagnosed further and not touched; it is the camera/fit owner's call.
- **`extrusion-axis` declares `vector`, `x`, `y` and `z` as BOTH inputs and outputs.** The census emits
  each of those channels twice, once per direction, with identical rects — so the live `math.vector`
  operator's port lists overlap. Two engine handles then share one public `"{nodeId}@{portId}"` key,
  one `HandleRole::Target` and one `Source`, and `decode_channel_ref` resolves the ambiguity by asking
  `inputs()` first. That is a generation3d operator-catalogue matter, not a renderer one, and nothing
  here depends on it beyond the probe agreeing with `decode_channel_ref`.
- **The committed renderer scene fixture carries no operator catalogue**, so in the LAW's graph the
  three neuron OUTPUT ports (`profile@wire`, `extrusion-axis@vector`, `extrude@solid`) and the three
  synapses that use them do not exist at all — a `NodeGraphScene` carries `fixture_json` but the
  registered operator catalogue rides `framework.section.catalogue`. Every law case is chosen to stand
  on the handles a catalogue-less scene really has; the oracle says so in
  `graph.neuronOutputsNeedTheCatalogue`. The 6118 serve, which does deliver the catalogue, hovers all
  six.
- **Nothing is claimed about React's own node graph.** The React lane's laws were run and are green;
  no React source was changed.

---

## 9. Peer boundaries, and reds that are not this lane's

- **`engine_canvas_slot_tables_are_heap_first_and_fit_a_bounded_thread_stack` is RED.**
  `EngineSurfaceSlot` measures **71 896** bytes against a committed budget of 67 528. The previous lane
  recorded **71 848** on a tree this lane did not touch — a peer's +4 320-byte growth — so **48 bytes of
  the 4 368 are this lane's** (`published_graph_interaction` 24 + `published_graph_hit` 8 +
  `published_graph_geometry` 8, plus alignment). Re-committing the budget would launder the peer's
  growth along with mine, so it is reported instead: whoever owns the +4 320 should commit
  `⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` at 71 896 and name both.
- **`world3d_pointer_down_emits_the_graph_domain_selection_react_dispatches` is RED and is not this
  lane's.** It fails at `🧩️wgpu-engine-surfaces/🦀️.rs:311` and then aborts the test process in a
  destructor (`♾️infinite/🌍️world/🦀️.rs:1664`). `🚩️619` (20:45) touched both files; this lane changed
  no World3d path.
- **One peer break was fixed forward.** `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3124` called
  `chrome_group_border`, which a peer was mid-way through making `#[cfg(test)]`; the file flip-flopped
  between compiling and not for ~20 minutes of polling. The call is now
  `ui_wgpu::wgpu::push_chrome_group_border(draw, rect, theme)` — the same primitive the one-line wrapper
  forwards to, fully qualified so it survives either state of that import. One line, no behaviour
  change; the wrapper and its other call site are untouched.
- **Two peer waves blocked the build and were waited out, not patched**: `Component::Progress` /
  `UiNode::Progress` across `🖱️ui` (non-exhaustive matches, missing `layout` exports) and
  `WindowMeasure::Progress` / `MeasureProgressStep` between `🔌️plugin` and the puzzle-3d plugin.
- `🐍️wgpu-node-graph-retention-probe.mjs` was re-run unchanged as a CONTROL (`retention-now`) and finds
  0 ports and 0 bodies on today's serve, exactly as this lane's sweep-based attempts did — which is
  what established that §5's sweep failure is the serve's, not this probe's.

---

## 10. Files

**Changed**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — `//#region 📌️GraphInteractionChange`, `node_graph_bounded_publish`, `plan_node_graph_edits`, the reworked screen-path dispatch, the `move` sub-operation, `log_graph_geometry_census`, the hit-trace diagnostic, three new `EngineSurface` digest fields
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — `DagGraphEdit::Move`, `DAG_GRAPH_EDIT_CAPACITY` 8 → 64, `plan_graph_edits`, `DagPointerPlan::dragged`, `DagScreenHit` + `screen_hit`/`screen_hit_json`, `screen_geometry_census_json` + `entity_screen_rect`/`first_screen_point_in`, `DagInteractionProjection::gesture_active`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `arm_history_baseline`, `bounded_pointer_gesture_active`, the `Move` arm of `take_graph_edits_json`, the retiring history reset
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs` — `bounded_pointer_gesture_active`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧹️retirement/🧪️tests/🧹️retirement/🦀️.rs` — the re-arm law
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs` — helpers made `pub(super)`, each law serialized behind the registry lock
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧊️wgpu-standalone/🦀️.rs` — `ENGINE_SURFACE_LAW_LOCK`/`engine_surface_law_guard`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts` — the discriminator assertion now covers the fourth, point-less question
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — one line, the peer fix-forward of §9

**Created**

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🫳️node-graph-gestures/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts`
- `<ticket>/🐍️wgpu-node-gestures-probe.mjs`
- this report

**Diagnostics left in place** (each fires once per CHANGE, never once per frame or once per move):
`wgpu node-graph geometry`, `wgpu node-graph hit`, `wgpu node-graph bounded gesture`. The previous
lane's `wgpu node-graph screen gesture`, `wgpu-shell pointer button` and `wgpu-shell retained press`
are untouched.
