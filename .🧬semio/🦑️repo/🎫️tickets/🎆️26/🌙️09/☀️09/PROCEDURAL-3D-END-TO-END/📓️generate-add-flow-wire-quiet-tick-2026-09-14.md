# Generate-Add · Flow-Wire · Quiet Tick — lane `generate-add-flow-wire-quiet-tick`, port 6026 (2026-09-14/15)

Owns the three reds `📓️react-oracle-hardening-2026-09-14.md` §6 handed on: the React battery's
`generate-mode` step `add-2`, `flow-wire` step `redraw restores the wire`, and `keyboard-verbs` step
`baseline`. Everything ran on `http://127.0.0.1:6026/?plugin=generation3d` (direct vite serve,
`SEMIO_VITE_HMR=0`, `screen -S g3dreact6026`). Evidence under `🗑️generated/react-gen-wire/` and
`🗑️generated/react-gen-wire-confirm/`.

**Final: `green=3/3 red=[] pageerrors=0`, twice consecutively.**

---

## 0. The one-line version

Three rows, **four product defects**, all four fixed on the layer that owns them:

1. **A plain CLICK on the node graph wrote the document.** The renderer read "this gesture wired
   nothing" as permission to re-publish the WHOLE fixture, so every click dispatched a `setFixture`
   `nodeGraphEdit` — and since a landed document mutation owes every attached preview a fresh
   evaluation, a QUIET shell invoked `["nodeGraphEdit","flowEvalTick","flowEvalTick"]` against a
   preview already settled at 7/7 nodes. The host has always owned the predicate that answers this; it
   now publishes it. (`keyboard-verbs` `baseline`.)
2. **The host published geometry for a port you cannot aim at.** `entity_screen_json`'s `visible` meant
   "this entity exists", never "you can press here": a port the camera had scrolled past was published
   as `visible: true` with its centre 5 px OUTSIDE the canvas, so a caller that trusts the host pressed
   off-surface and reached nothing. (`flow-wire` `redraw restores the wire`.)
3. **A refused flow control message was thrown at whoever called `cancel`.** `observeFlowTask` cancels
   the previous task per feature key on every pointer move, so a drag over a saturated guest raised an
   uncaught `FlowMessageRejected` page error mid-gesture. A refused control is backpressure; the pump
   re-sends it, exactly as it re-sends a session close. (`flow-wire`'s page-error gate.)
4. **`add-2` was never a dropped generation** — it was a fixed 2.5 s wait in the probe, and the probe's
   own next step found both rows. Not a product defect; the row now polls and REPORTS the latency
   (0.5 s and 1.1 s on the final tree). This is the one of the four that is a measurement fix.

---

## 1. Baseline, before anything was changed

`SEMIO_BATTERY_URL=… SEMIO_BATTERY_ROOT=react-gen-wire bun 🐍️react-battery.mjs --only=generate-mode,flow-wire,keyboard-verbs`
(`🗑️generated/react-gen-wire/baseline-run.txt`, 18:57–19:00) on the guest wasm staged at **18:12** —
the same build the handing-on lane measured its three reds on, and a healthy one: `exceeds 64 pages`
and `No loaded plugin contributes the flow extension "brep"` are **0** in all three probe consoles and
the edit preview delivered the hexagonal-mushroom fixture's real geometry (`meshesLen 3644`,
`nodesDone 7/7`). It is NOT the faulted 19:00 wasm the coordinator's 19:05 broadcast warned about.

| probe | ok | reds |
|---|---|---|
| generate-mode | **✓** | — (13/13) |
| flow-wire | **✓** | — |
| keyboard-verbs | ✗ | **`baseline`** — `invoked: ["nodeGraphEdit","flowEvalTick","flowEvalTick"]` |

**Two of the three handed-on reds did not reproduce on the tree they were reported on** — and neither
was "fixed in between": both runs are the same wasm. §3 and §4 say what each red actually was, from the
two runs' own consoles.

---

## 2. `keyboard-verbs` `baseline` — the quiet shell wrote the document

### 2.1 What the shell did with no gesture

The probe focuses the shell with one click at (720, 500) — which in generation3d's edit mode is inside
the Flow window's node-graph canvas — waits 1.5 s, marks the console, waits 2.5 s, and requires that
nothing was invoked. Measured (`🗑️generated/react-gen-wire/keyboard-verbs/console.txt`):

| t (ms) | line |
|---:|---|
| 9136–9188 | the focus click: `interactionHover`, `interactionSelect`, `nodeGraphViewport` |
| **10629** | **`performInvocation {"actionId":"nodeGraphEdit"}`** — 1.44 s after the click, inside the baseline window |
| 10775 | `plugin_exchange actionId=nodeGraphEdit branch=catalog` |
| 10809, 11478 | `flowEvalTick`, `flowEvalTick` |

The preview's own published status at the end of that window is `phase: "idle"`, `nodesDone: 7`,
`nodesTotal: 7`, `ratio: 1` — a fully settled evaluation, re-armed twice by a shell nobody touched. No
`node graph wire edit dispatch` precedes it, so it is not the wire path; the 1.44 s lag is the two async
round trips between pointer-up and dispatch (`pointerUpScreen`, then `documentJson:commit`), which is
why it lands past the probe's 1.5 s mark instead of with the click's own dispatches.

### 2.2 Root cause, at file:line

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx`, the flow
canvas's `onPointerUp`:

```tsx
const operations = graphEditOperations(value);
if (operations.length > 0) dispatch(nodeGraphActions.edit, { operations });
else commitFixture();                           // ← every click, marquee, pan and missed press
```

`commitFixture()` dispatches `nodeGraphEdit` with `{operation:"setFixture", fixtureJson}` — the WHOLE
graph. The empty `operations` was read as permission to commit, but it only ever meant "this gesture
wired nothing": the screen pointer path journals `Connect`/`Disconnect` and nothing else
(`🕸️dag/🦀️.rs`'s `process_engine_events` → `journal_connect`/`journal_disconnect`), so a node drag, an
inline slider drag and a plain click are indistinguishable in that answer.

The cost is not only a retained command per click. `setFixture` lands in `✏️node-graph-edit/🦀️.rs`,
whose `commit_fixture(before, target)` diffs the two; any drift between the session's fixture and the
document's becomes real artifact mutations, and `owe_attached_previews_for_mutations`
(`🧵️preview-eval/🦀️.rs:216`) owes every attached preview window a fresh evaluation the moment
`emit.artifact_mutations` is non-empty — carrying the `toolRunStart` when no live run can be woken.
That is the `flowEvalTick` pair, and it is the mechanism lane `preview-rearm-after-inspector-edit`
deliberately installed: the defect is the CLICK claiming to be a document mutation, not the re-arm
reacting to one.

### 2.3 The fix — the host already knew the answer, it just never said it

`FlowHost::commit_gesture_history` (`🌊️flow/🖥️host/🦀️.rs`) has decided this exact question since the
retention lane wrote it: `content_changed(&baseline, &self.fixture)` — `widgets || synapses || layout`,
camera deliberately excluded — is what tells an undo entry from a no-op, and its own docstring already
names the case: *"A gesture that changed nothing — every plain CLICK on the graph"*. It was never
published.

| file | change |
|---|---|
| `🌊️flow/🖥️host/🦀️.rs` | new `gesture_changed_content: bool`, set by `commit_gesture_history` from the `content_changed` call it already made; `take_graph_edits_json` answers `{"operations":[…],"fixtureChanged":bool}`; new reader `gesture_changed_content()`; **an unreachable duplicate `DagGraphEdit::Move` match arm a peer had left in that same function removed** |
| `🕸️NodeGraph/🟦️.tsx` | `graphEditOperations` → `graphGestureAnswer`, reading both halves; `onPointerUp` dispatches the narrow operations, **else commits only `if (fixtureChanged)`**, else nothing |

The rule: **a released gesture that changed no widget, no synapse and no layout dispatches nothing at
all.** A node drag, an inline slider drag and a port insert still commit the fixture, because the screen
path's narrow vocabulary does not carry them — that is the second law below, and it is why the fallback
was not simply deleted.

### 2.4 Laws, with output

**Rust** — `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`, two new laws over the real `FlowHost` and its real
screen pointer path:
`RUST_MIN_STACK=33554432 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-flow --lib -- host::tests::a_gesture_that_changed_nothing host::tests::a_drag_that_moved_a_node --test-threads=1 --nocapture`

```
test host::tests::a_drag_that_moved_a_node_answers_a_fixture_commit ... [DEBUG] dag node moved id=2 x=260.0 y=40.0
[DEBUG] node drag answer operations=0 fixtureChanged=true
ok
test host::tests::a_gesture_that_changed_nothing_answers_no_operations_and_no_fixture_commit ...
[DEBUG] quiet click answer operations=0 fixtureChanged=false
ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 234 filtered out
```

**TypeScript twin** — `🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts`, new rule
`aContentlessGestureDispatchesNothing` in the shared fixture
`🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json`, new
`it("lets the HOST decide whether a released screen gesture owes a document write at all")`. It pins
both sides of the contract in the two files that own them, so a rename on either side fails instead of
silently splitting the renderers.
`./node_modules/.bin/vitest run --config <ticket>/🔍️node-graph-gestures.config.ts` → **11 passed (11)**.

**The twin is red before the fix.** Counterproof taken by toggling the one line back to
`else commitFixture();` and re-running (then restoring it):

```
FAIL  …🫳️node-graph-gestures/🟦️.ts > lets the HOST decide whether a released screen gesture owes a document write at all
AssertionError: expected '…' to contain 'else if (fixtureChanged) commitFixtur…'
     Tests  1 failed | 10 passed (11)
```

---

## 3. `generate-mode` `add-2` — a 2.5 s wait, not a dropped add

The probe clicked `Add Generation` twice and graded each click after a FIXED `waitForTimeout(2500)`.
The handing-on lane's own evidence disproves the reading it was given
(`🗑️generated/react-oracle/generate-mode/results.json`):

| step | ok | roster it saw |
|---|---|---|
| `add-1` | true | `["generation-1"]` |
| `add-2` | **false** | `["generation-1"]` — still one row 2.5 s after the second click |
| `add` | **true** | `["generation-1","generation-2"]` — both rows, moments later |

`add` is the same run, the same session, a few seconds on, and it carries the user-visible sentence
("two generations, each selectable/renamable with its own preview"): it was green in that run, and
`select`, `rename` and `remove` then operated on `generation-2` by id. **Nothing was dropped or
coalesced.**

**Hardened** (`🐍️generate-mode-probe.mjs`): each round polls for the roster length it requires, up to
`SEMIO_PROBE_ADD_WAIT` (default 60 s), and records `seconds` — so a genuinely dropped add stays red
while a slow one is reported as the latency it is. On the final tree: **`add-1` 0.5 s, `add-2` 1.1 s.**

---

## 4. `flow-wire` `redraw restores the wire` — the press never entered a wire draw

### 4.1 What the two runs recorded

Both cut `height@number -> extrusion-axis@z` successfully, then dragged output-centre → input-centre to
redraw it. The consoles say what actually happened.

| | the red run (`🗑️generated/react-oracle/flow-wire/`, 18:28) | the green run (`🗑️generated/react-gen-wire/flow-wire/`, 19:00) |
|---|---|---|
| `dag port press … extrusion-axis@z` (the cut) | `interaction=draw-edge(reconnect)` ✓ | `interaction=draw-edge(reconnect)` ✓ |
| `dag port press … height@number` (the redraw) | **absent** | `interaction=draw-edge(new)` ✓ |
| `… wire edit dispatch … connect` | only at 131 648 ms, from the LATER low-zoom phase | 86 077 ms, from the redraw itself |
| published `height@number` rect | `(542.86, 482.63) 11.92×12.83` | `(542.86, 489.03) 11.92×12.83` |
| published `extrusion-axis@z` rect | `(629.87, 482.86) **19.25**×12.83` | `(629.87, 489.26) **11.92**×12.83` |
| `redrawRestoredTheWire` | false | true |

The x's are identical and the y's differ by **6.4 px** in both ports at once, while the sink's WIDTH
differs by 7.3 px — the graph had moved and one port row was published in a different draw state. The
red run pressed 6.4 px above the port; the host answered as empty canvas and panned the camera
(`nodeGraphViewport` every ~500 ms from 113 016 to 121 485 ms), and no wire draw ever started. **A press
that never reached a port is not evidence that a graph refuses to be rewired**, and the row graded it as
exactly that.

### 4.2 The product defect underneath: `visible` meant "exists", not "you can aim at this"

Re-running on 6026 after the restage made it deterministic and much sharper. The graph canvas spans
page x ∈ [6.4, 972.7]; the host published `height@number` at **page x = 1.43**, rect from **x = −10.17**
— entirely left of the canvas — with `visible: true`, **twice, byte-identically** (so not a cache race):

```
[DEBUG] wire: the press did not enter a wire draw — the published rect is stale, re-aiming
  {"aim":0,"aimedAt":{"x":1.4269637218488072,…},"publishedSource":{"rect":{"x":-10.171847928558407,…}}}
  {"aim":1,…identical…}
```

`DagHost::entity_screen_json` (`🕸️dag/🦀️.rs:3973`) stamped `visible: true` on every entity it could
FIND, whatever the camera — the viewport was never consulted. Its own docstring says the published
centre "is where anything that trusts the host aims"; for a scrolled-past port that centre is off the
surface, and a demonstration, an assistive caller or a scripted drag presses outside the canvas.

**Fix** (`🕸️dag/🦀️.rs`): new `screen_point_is_on_surface`; the `handle`/`node` path and the `edge`
midpoint publish the unresolved answer (`visible: false`, no rect) when the point a caller would aim at
is not on the surface. `visible` now means "you can press here".

**Law** — `🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`, new
`a_port_the_camera_has_scrolled_past_publishes_no_geometry_to_aim_at`: the endpoint publishes a rect on
the boot camera, publishes **nothing** once the camera is moved two viewport widths away, and publishes
it again when the camera returns.

The existing law `the_port_geometry_the_host_publishes_is_the_geometry_that_grabs_a_wire` went red on
the new rule at zoom 2, and for a real reason: with the camera fixed at (0, 0) its `tgt` row left the
1280-wide viewport entirely, so the case had been pressing a point no browser would route to the canvas.
Fixed forward — a new `case_endpoint_centre` frames the camera on the very endpoints each case names, so
a zoom band tests the LOD, not the framing.

```
cargo test -p semio-framework-os-infinite --lib -- wire_edit --test-threads=1
test …::a_minimap_click_moves_the_camera ... ok
test …::a_port_the_camera_has_scrolled_past_publishes_no_geometry_to_aim_at ... ok
test …::a_port_to_port_drag_creates_a_wire_and_journals_it_for_the_guest ... ok
test …::a_wire_in_flight_holds_the_screen_pointer_path ... ok
test …::the_port_geometry_the_host_publishes_is_the_geometry_that_grabs_a_wire ... ok
test result: ok. 5 passed; 0 failed
```

### 4.3 The page-error gate: a refused control was thrown at its caller

With the wire steps green, one thing still failed the battery's central gate — an intermittent
`pageerror FlowMessageRejected: Flow message rejected` (3 of 5 runs), always mid-drag inside a burst of
`dag hover changed` lines.

`🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js`: `transfer()` throws `FlowMessageRejected` when
`flow_bridge_send` answers "no credit right now". Every other control send goes through
`transferControl`, which treats that as backpressure and retries — `cancel()` was the one that called
`transfer` bare:

```js
const cancel = (requestId) => { if (!state.pending.has(requestId)) return false; transfer(encodeCancel(…)); pump(); return true; };
```

`observeFlowTask` (`🕸️NodeGraph/🟦️.tsx:2021`) keeps ONE task per feature key and **cancels the previous
one**, so a pointer-move burst over a saturated guest throws out of a React event handler → uncaught →
`pageerror`.

**Fix**: `cancel` marks the pending entry `cancelRequested` and the pump re-sends the cancel with
`transferControl` until the guest takes it — the identical idiom the pump already uses for
`owner.closing && !owner.controlSent`. The caller is answered "this request is cancelling" and is never
handed a control rejection.

**Law, changed forward** — `🕸️wasm/🧪️tests/🖥️host/🟨️.js`'s `rejected-control` case asserted the OLD
behaviour (nine `cancel()` calls that must each THROW). It now asserts that none of the nine throws,
that each answers `true`, and that the request still terminates `cancelled` after all nine refusals —
i.e. the pump delivered it. `bun nx run semio-framework-os-flow-core:test-browser` → **green**.

### 4.4 The probe, hardened three ways

- `publishedPort` requires **three** consecutive equal reads (≥1.2 s) instead of two — two reads of the
  same undated `dagIntroductionResolver` cache entry are equal and prove nothing.
- New `bringPortsIntoView`: before the cut AND before the redraw, wheel out at the canvas centre until
  the host publishes geometry for BOTH endpoints — the user-truthful way to reach a port the boot camera
  left off-screen. On the final run: `portsInView.beforeCut = {onSurface: true, zoomOuts: 2}`, i.e. the
  boot camera really did leave an endpoint off the surface.
- The redraw asserts the host's own `dag port press … interaction=draw-edge` line for the port it aimed
  at, re-aims once at fresh geometry when the press did not land, and publishes `redrawPressLanded` and
  `staleAim` into `wire-result.json` and the battery's step detail.
- The low-zoom phase no longer drags in the `minimap` band, which withholds ports **by design**: the
  drag there grabbed a NODE and hurled it across the graph — a gesture this row never tests, and the
  source of one run's page error.

---

## 5. Runtime verification after the fixes

Guest restaged (`🗑️generated/react-gen-wire/restage-3.txt`, 23:53), flow CORE browser wasm rebuilt
(`flow-core-wasm-3.txt`, `flow-core-wasm-4.txt` — `🕸️wasm` is `cfg`'d OUT of the wasm32-wasip2 plugin
components, so **no plugin restage rebuilds it**; that is why `fixtureChanged` was absent from every
plugin wasm and the change only reached the browser via `semio-framework-os-flow-core:wasm`). Served
module verified carrying the edit before trusting any verdict:
`curl --path-as-is 'http://127.0.0.1:6026/@fs/…/🕸️NodeGraph/🟦️.tsx'` → 419 638 bytes, `fixtureChanged` ×4.

`SEMIO_BATTERY_URL=http://127.0.0.1:6026/?plugin=generation3d SEMIO_BATTERY_ROOT=react-gen-wire bun 🐍️react-battery.mjs --only=generate-mode,flow-wire,keyboard-verbs`

| probe | before (handed on, 18:28) | baseline (18:57) | **after** | steps | seconds | pageerrors |
|---|---|---|---|---|---|---|
| generate-mode | ✗ `add-2` | ✓ | **✓** | 13/13 | 32–38 | 0 |
| flow-wire | ✗ `redraw restores the wire` | ✓ | **✓** | 5/5 | 103 | 0 |
| keyboard-verbs | ✗ `baseline` | ✗ `baseline` | **✓** | 12/12 | 28–29 | 0 |
| **battery** | — | `green=2/3` | **`green=3/3 red=[] pageerrors=0`** | — | 163–170 | 0 |

Run twice consecutively, both clean: `🗑️generated/react-gen-wire/final-run-all-3.txt` (00:24) and
`🗑️generated/react-gen-wire-confirm/scoreboard.json` (00:29,
`{"probes":3,"green":3,"red":[],"totalSeconds":163,"pageerrors":0}`).

The numbers the rows now carry:

- `baseline` → `invoked: []` (the scoreboard's own `chords` map reads `"baseline": "(nothing)"`) against
  a preview publishing `nodesDone 7/7`, `ratio 1`. Zero `nodeGraphEdit`, zero `flowEvalTick`.
- `add-1` **0.5 s**, `add-2` **1.1 s**, roster `["generation-1","generation-2"]`, then `select`,
  `rename` ("Balcony Study"), `remove`, `form`, `gumball-rail` `["Move","Rotate","Scale"]`,
  `gumball-lane` all green.
- `flow-wire`: 6 wires before → 5 after the cut → 6 after the redraw, `redrawPressLanded: true`,
  `staleAim: []`, `portsInView {beforeCut:{onSurface:true,zoomOuts:2}, beforeRedraw:{onSurface:true,zoomOuts:0}}`,
  both narrow edits dispatched (`disconnect e1` at 74 423 ms, `connect height@number → extrusion-axis@z`
  at 84 667 ms), preview `meshes 3689`.

### 5.1 Regression suites

| suite | result | note |
|---|---|---|
| `cargo test -p semio-framework-os-flow --lib -- host::` | **118 passed, 4 failed** | a **byte-identical** failure set to lane `flow-tick-coalescing`'s record on this tree before this lane touched anything (`🗑️generated/flow-coalesce/laws-flow-host.txt`: 107/4) — `connect_ports_replaces_existing_incoming_on_same_input`, `delete_selection_removes_edge_selected_by_synapse_id_domain`, `hexagonal_mushroom_fixture_reports_extruded_solid_output`, `rectangle_extrude_fixture_evaluates_solid_output`; the last two already red on 09-13 (`🗑️generated/suite-reds/flow-host-2.txt`). Output: `🗑️generated/react-gen-wire/laws-flow-host.txt` |
| `cargo test -p semio-framework-os-infinite --lib -- board::` | **163 passed, 4 failed** | counterproof taken: with the visibility gate temporarily forced to `true` the SAME 4 fail and my own new law fails as a fifth (164 → 162 passed, 5 failed), so the 4 are peer/pre-existing and the new law is red without the fix. Output: `🗑️generated/react-gen-wire/laws-dag.txt` |
| `cargo test -p semio-framework-os-infinite --lib -- wire_edit` | **5 passed, 0 failed** | includes the new visibility law and the reframed grab law |
| `nx run semio-framework-os-flow-core:test-browser` | **green** | the forward-changed `rejected-control` law |
| `./node_modules/.bin/vitest run --config <ticket>/🔍️node-graph-gestures.config.ts` | **11 passed** | red before the fix (§2.4) |
| `@semio-tech/framework-renderer-react:typecheck` | 801 pre-existing errors tree-wide | two in `🕸️NodeGraph/🟦️.tsx`, both at line 1111 (the peer-owned `Viewport2d` rename); **none at the lines this lane changed** |

### 5.2 wgpu twin

<!--WGPU-->

---

## 6. Files changed

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `gesture_changed_content`, published as
  `fixtureChanged` by `take_graph_edits_json`; duplicate unreachable `Move` arm removed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` —
  `graphGestureAnswer`; the pointer-up fallback gated on `fixtureChanged`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` —
  `screen_point_is_on_surface`; `entity_screen_json` publishes nothing for a point off the surface.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🖥️host/🏃️runtime/🟨️.js` — a refused `cancel` is
  backpressure the pump re-sends, never an exception at the caller.

Laws and fixtures:
- `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — two laws plus `world_screen_point` /
  `empty_canvas_world_point` / `gesture_answer` helpers.
- `🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs` — the visibility law and `case_endpoint_centre`.
- `🕸️wasm/🧪️tests/🖥️host/🟨️.js` — the `rejected-control` case changed forward.
- `🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json` — rule `aContentlessGestureDispatchesNothing`.
- `🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts` — its TS twin; the rule-coverage filter names it.

Probes and battery (ticket folder):
- `🐍️generate-mode-probe.mjs` — `add-N` polls and reports `seconds`.
- `🐍️flow-window-probe.mjs` — three-read port geometry, `bringPortsIntoView`, landed-press assertion,
  one re-aim, no drags in the `minimap` band, `redrawPressLanded`/`staleAim`/`portsInView`.
- `🐍️react-battery.mjs` — the `flow-wire` verdict carries those.
- `🔍️node-graph-gestures.config.ts` — **new**; the only way to run the `🟦️.ts` twin under vitest.
- `📜️flow-core-wasm-retry.sh` — **new**; rebuilds the flow CORE browser wasm through peer churn.
- `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md` — this report.

---

## 7. What is NOT claimed

- **That the `keyboard-verbs` row has only ever had one cause.** Between 19:57 and the 20:14 stage a peer
  lane landed a `toolRunPace` wake, and the row went red again with `invoked` = 23 × `toolRunPace` in the
  2.5 s window (290 in a 28 s session, one per 100 ms — `TOOL_RUN_PACE_PRESENTATION_RETRY_MS`) against a
  preview publishing `phase: "idle"`, `nodesDone 7/7`, `ratio 1`. That mechanism was REMOVED from the tree
  by its own lane at ~23:23 (`toolRunPace` no longer exists in any `🦀️.rs`), and the 23:53 restage cleared
  it from the runtime. This lane neither caused nor fixed it; what this lane's fix owns is the
  `nodeGraphEdit` + `flowEvalTick` pair, which is absent from every run after §2.3 — including the runs
  where `toolRunPace` was storming.
- **That the four pre-existing `semio-framework-os-flow --lib -- host::` failures and the four
  `semio-framework-os-infinite --lib -- board::` failures are understood.** They are shown identical
  before and after this lane's changes (the dag set by an explicit toggle-off counterproof, the flow set
  against lane `flow-tick-coalescing`'s own recorded run), and two of the flow ones were already red on
  09-13. Why they fail is another lane's question.
- **That `dagIntroductionResolver`'s cache is fixed.** It still answers `entity(…)` out of an UNDATED
  `Map<string,string>` and starts the real read in the background (`🕸️NodeGraph/🟦️.tsx:1638`), so an
  answer can be several frames old and says nothing about its own age. What changed is that the HOST no
  longer publishes a point you cannot press, which removes the failure mode this row hit; a rect that is
  on-surface but stale is still possible. The two clean designs — key the cache by the paint state it was
  read under, or carry `stale` on `IntroductionResolvedGeometry` — both change the introduction geometry
  contract that the guided-tour/demonstration surface owns, and the risk that decides between them must
  be measured first: a state-keyed cache on an animating graph never answers, which would leave a guided
  tour with no target at all.
- **That the `visible` rule is right for every consumer.** It is asserted for `handle`, `node` and `edge`
  through `entity_screen_json`, and `entity_census_json` inherits it. A consumer that WANTS to know where
  an off-surface entity is (to scroll it into view) now gets `visible: false` and no rect, and would need
  a separate off-surface answer. No such consumer was found in this tree; none was searched for beyond
  the two `entity_screen_json` call sites in `♾️infinite`.
- **That the `FlowMessageRejected` page error is gone for every gesture.** It is gone from the five post-fix
  `flow-wire` runs and its one throwing call site is fixed; `transfer` is still bare in `start()`, where a
  refused REQUEST rejects that request's own task (which every caller awaits), which is a different and
  correct shape. No other surface was exercised.
- **Nothing about the other thirteen React battery probes.** Only `generate-mode`, `flow-wire` and
  `keyboard-verbs` were run. The `interact` lane's `fit`/`select`/`selection-reset` reds and the
  `cancel-preview` red from `📓️react-oracle-hardening-2026-09-14.md` §4.3/§4.4 are untouched.
- **That the React `typecheck` is clean.** It reports 801 pre-existing errors tree-wide; this lane only
  claims that none of them is at a line it changed, and that the two in `🕸️NodeGraph/🟦️.tsx` are the
  peer-owned `Viewport2d` rename at line 1111.
- **That the 20:14-era tree was measurable at all.** Every extension evaluation on it failed
  `invokeExtension dispatch failed … SemioFaultError: targeted window transient capture requires an exact
  ViewModel roster`, so the graph never converged (`profile: queued`, `extrusion-axis: computing`,
  `extrude: blocked` for 70 s) and the generate preview published `phase: "cancelled"` with 0 meshes. Runs
  taken in that window are recorded here only to explain why they are not evidence.
