# Generate-Add · Flow-Wire · Quiet Tick — lane `generate-add-flow-wire-quiet-tick`, port 6026 (2026-09-14)

Owns the three reds `📓️react-oracle-hardening-2026-09-14.md` §6 handed on: the React battery's
`generate-mode` step `add-2`, `flow-wire` step `redraw restores the wire`, and `keyboard-verbs` step
`baseline`. Everything below ran on `http://127.0.0.1:6026/?plugin=generation3d` (direct vite serve,
`SEMIO_VITE_HMR=0`, `screen -S g3dreact6026`). Evidence under `🗑️generated/react-gen-wire/`.

---

## 0. The one-line version

**One of the three is a product defect, and it is fixed; the other two are probe flakes on a healthy
tree, and both probes are now hardened so neither can pass or fail by luck again.**

- `keyboard-verbs` `baseline` — **a product defect, fixed.** Every plain CLICK on the node-graph canvas
  dispatched a whole-fixture `nodeGraphEdit`, and a landed document mutation owes every attached
  preview a fresh evaluation, so a quiet shell invoked `["nodeGraphEdit","flowEvalTick","flowEvalTick"]`
  against a preview already settled at 7/7 nodes. The host has always owned the predicate that answers
  this (`commit_gesture_history`'s `content_changed`); it now PUBLISHES it, and the renderer commits
  only when it says content moved.
- `generate-mode` `add-2` — **not a product defect.** Both generations really were added; the row's
  fixed 2.5 s wait expired before the second row painted, and the very next step in the SAME run
  (`add`) then found both. The row now polls and reports the latency.
- `flow-wire` `redraw restores the wire` — **not a graph that refuses to be rewired.** The press that
  the row was graded on never entered a wire draw at all: the geometry the host PUBLISHED for
  `height@number` was 6.4 px above where the graph then was, so the press landed on empty canvas and
  panned the camera. The row now requires — and reports — a landed press.

---

## 1. Baseline, before anything was changed

`SEMIO_BATTERY_URL=http://127.0.0.1:6026/?plugin=generation3d SEMIO_BATTERY_ROOT=react-gen-wire bun 🐍️react-battery.mjs --only=generate-mode,flow-wire,keyboard-verbs`
(`🗑️generated/react-gen-wire/baseline-run.txt`, 18:57–19:00), on the guest wasm staged at **18:12** —
the same build the handing-on lane measured its three reds on, and a healthy one: `exceeds 64 pages`
and `No loaded plugin contributes the flow extension "brep"` are **0** in all three probe consoles, and
the edit preview delivered the hexagonal-mushroom fixture's real geometry (`meshesLen 3644`,
`nodesDone 7/7`). It is NOT the faulted 19:00 wasm the coordinator's 19:05 broadcast warned about.

| probe | ok | reds |
|---|---|---|
| generate-mode | **✓** | — (13/13 steps, `add-1`, `add-2`, `add`, `select`, `rename`, `remove`, `form`, `gumball-*` all green) |
| flow-wire | **✓** | — (`graph publishes wires` 6, `cut removes the wire`, `redraw restores the wire`) |
| keyboard-verbs | ✗ | **`baseline`** — `invoked: ["nodeGraphEdit","flowEvalTick","flowEvalTick"]` |

Page errors across the run: **0**. Shell faults: **0**.

**So two of the three handed-on reds did not reproduce on the tree they were reported on.** §3 and §4
say why each one was red once and green once, from the two runs' own consoles — neither is "a peer
fixed it in between": both runs are the same wasm.

---

## 2. `keyboard-verbs` `baseline` — the product defect

### 2.1 What the shell did with no gesture

`🐍️editor-verbs-keyboard-probe.mjs` focuses the shell with one click at (720, 500) — which on
generation3d's edit mode is inside the Flow window's node-graph canvas — waits 1.5 s, marks the console,
waits another 2.5 s and requires that nothing was invoked. Measured
(`🗑️generated/react-gen-wire/keyboard-verbs/console.txt`):

| t (ms) | line |
|---:|---|
| 9136–9188 | the focus click: `interactionHover`, `interactionSelect`, `nodeGraphViewport` |
| 10417 | `nodeGraphViewport` settles |
| **10629** | **`performInvocation {"actionId":"nodeGraphEdit"}`** — 1.44 s after the click, inside the baseline window |
| 10775 | `plugin_exchange actionId=nodeGraphEdit branch=catalog` |
| 10809 | `flowEvalTick` |
| 11478 | `flowEvalTick` |

The preview's own published status at the end of the window is `phase: "idle"`, `nodesDone: 7`,
`nodesTotal: 7`, `ratio: 1` — a fully settled evaluation, re-armed twice by a shell nobody touched.

There is no `node graph wire edit dispatch` line before it, so it is not the wire path. The 1.44 s lag
is the two async round trips between the pointer-up and the dispatch (`pointerUpScreen`, then
`documentJson:commit`), which is exactly why it lands past the probe's 1.5 s mark rather than with the
click's own dispatches.

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
(`🕸️dag/🦀️.rs`'s `process_engine_events` → `journal_connect`/`journal_disconnect`), so a node drag,
an inline slider drag and a plain click are all indistinguishable from each other in that answer.

The consequence is not only a wasted retained command per click. `setFixture` lands in
`✏️node-graph-edit/🦀️.rs`, whose `commit_fixture(before, target)` diffs the two; any drift at all
between the session's fixture and the document's becomes real artifact mutations, and
`owe_attached_previews_for_mutations` (`🧵️preview-eval/🦀️.rs:216`) owes every attached preview window a
fresh evaluation the moment `emit.artifact_mutations` is non-empty — carrying the `toolRunStart` when no
live run can be woken. That is the `flowEvalTick` pair, and it is the mechanism lane
`preview-rearm-after-inspector-edit` deliberately installed: the defect is the CLICK claiming to be a
document mutation, not the re-arm reacting to one.

### 2.3 The fix — the host already knew the answer, it just never said it

`FlowHost::commit_gesture_history` (`🌊️flow/🖥️host/🦀️.rs`) has decided this exact question since the
retention lane wrote it: `content_changed(&baseline, &self.fixture)` — `widgets || synapses || layout`,
camera deliberately excluded — is what tells an undo entry from a no-op, and its own docstring already
names the case: *"A gesture that changed nothing — every plain CLICK on the graph"*. It was simply never
published.

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` | new `gesture_changed_content: bool`, set by `commit_gesture_history` from the same `content_changed` call it already made; `take_graph_edits_json` answers `{"operations":[…],"fixtureChanged":bool}`; new reader `gesture_changed_content()`; **removed an unreachable duplicate `DagGraphEdit::Move` match arm** a peer had left in that same function |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` | `graphEditOperations` → `graphGestureAnswer`, which reads both halves; `onPointerUp` now dispatches the narrow operations, **else commits only `if (fixtureChanged)`**, else nothing |

The rule this lands: **a released gesture that changed no widget, no synapse and no layout dispatches
nothing at all.** A node drag, an inline slider drag and a port insert still commit the fixture, because
the screen path's narrow vocabulary does not carry them — that is the second law below, and it is why
the fallback was not simply deleted.

### 2.4 Laws, with output

**Rust** — `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`, two new laws over
the real `FlowHost` and its real screen pointer path.
`RUST_MIN_STACK=33554432 CARGO_INCREMENTAL=0 cargo test -p semio-framework-os-flow --lib -- host::tests::a_gesture_that_changed_nothing host::tests::a_drag_that_moved_a_node --test-threads=1 --nocapture`

```
test host::tests::a_drag_that_moved_a_node_answers_a_fixture_commit ... [DEBUG] dag selection changed: add
[DEBUG] dag node moved id=2 x=260.0 y=40.0
[DEBUG] node drag answer operations=0 fixtureChanged=true
ok
test host::tests::a_gesture_that_changed_nothing_answers_no_operations_and_no_fixture_commit ... [DEBUG] dag selection changed:
[DEBUG] quiet click answer operations=0 fixtureChanged=false
ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 234 filtered out
```

**TypeScript twin** — `🧰️framework/…/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts`, new rule
`aContentlessGestureDispatchesNothing` in the shared fixture
`🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json`, new `it("lets the HOST decide whether a released
screen gesture owes a document write at all")`. It pins both sides of the contract in the two files that
own them, so a rename on either side fails instead of silently splitting the renderers.
`./node_modules/.bin/vitest run --config <ticket>/🔍️node-graph-gestures.config.ts`

```
Test Files  1 passed (1)
     Tests  11 passed (11)
```

**The twin is red before the fix.** Counterproof taken by toggling the one line back to
`else commitFixture();` and re-running (then restoring it):

```
FAIL  …🫳️node-graph-gestures/🟦️.ts > node graph gestures > lets the HOST decide whether a released screen gesture owes a document write at all
AssertionError: expected '…' to contain 'else if (fixtureChanged) commitFixtur…'
     Tests  1 failed | 10 passed (11)
```

**Regression, same crate, same filter:**
`cargo test -p semio-framework-os-flow --lib -- host:: --test-threads=1` →
**118 passed, 4 failed** (`🗑️generated/react-gen-wire/laws-flow-host.txt`). The four are a
**byte-identical failure set** to the one lane `flow-tick-coalescing` recorded on this tree before this
lane touched anything (`🗑️generated/flow-coalesce/laws-flow-host.txt`: 107 passed, 4 failed) —
`connect_ports_replaces_existing_incoming_on_same_input`,
`delete_selection_removes_edge_selected_by_synapse_id_domain`,
`hexagonal_mushroom_fixture_reports_extruded_solid_output`,
`rectangle_extrude_fixture_evaluates_solid_output`. The last two were already red on 09-13
(`🗑️generated/suite-reds/flow-host-2.txt`). +11 passes over that record are this lane's two laws plus
peers'.

`@semio-tech/framework-renderer-react:typecheck` reports **801 pre-existing errors** across the tree,
two of them in `🕸️NodeGraph/🟦️.tsx` — both at line 1111, the peer-owned `Viewport2d` rename, neither at
the two lines this lane changed (2329, 3153).

---

## 3. `generate-mode` `add-2` — a 2.5 s wait, not a dropped add

`🐍️generate-mode-probe.mjs` clicked `Add Generation` twice and graded each click after a FIXED
`waitForTimeout(2500)`.

The handing-on lane's own evidence disproves the reading it was given
(`🗑️generated/react-oracle/generate-mode/results.json`):

| step | ok | roster it saw |
|---|---|---|
| `add-1` | true | `["generation-1"]` |
| `add-2` | **false** | `["generation-1"]` — still one row 2.5 s after the second click |
| `add` | **true** | `["generation-1","generation-2"]` — both rows, moments later |

`add` is the same run, the same session, a few seconds further on, and it is the step that carries the
user-visible sentence ("a user must get two generations, each selectable/renamable with its own
preview"): it was green in that run, and `select`, `rename` and `remove` then operated on
`generation-2` by id. **Nothing was dropped or coalesced; the second row simply painted later than the
wait.** In this lane's run every step including `add-2` was green on the same wasm.

**Hardened** (`🐍️generate-mode-probe.mjs`): each round now polls for the roster length it requires, up
to `SEMIO_PROBE_ADD_WAIT` (default 60 s), and records `seconds` in the row's detail — so a genuinely
dropped add stays red while a slow one is reported as the latency it is.

---

## 4. `flow-wire` `redraw restores the wire` — the press never entered a wire draw

Both runs cut `height@number -> extrusion-axis@z` successfully and then dragged output-centre →
input-centre to redraw it. The consoles say what actually happened.

| | the red run (`🗑️generated/react-oracle/flow-wire/`, 18:28) | this lane's run (`🗑️generated/react-gen-wire/flow-wire/`, 19:00) |
|---|---|---|
| `dag port press … extrusion-axis@z` (the cut) | `interaction=draw-edge(reconnect)` ✓ | `interaction=draw-edge(reconnect)` ✓ |
| `dag port press … height@number` (the redraw) | **absent** | `interaction=draw-edge(new)` ✓ |
| `node graph wire edit dispatch … connect` | only at 131 648 ms, from the LATER low-zoom phase | 86 077 ms, from the redraw itself |
| published `height@number` rect | `(542.86, 482.63) 11.92×12.83` | `(542.86, 489.03) 11.92×12.83` |
| published `extrusion-axis@z` rect | `(629.87, 482.86) **19.25**×12.83` | `(629.87, 489.26) **11.92**×12.83` |
| `redrawRestoredTheWire` | false | true |

The x's are identical and the y's differ by **6.4 px** in both ports at once, while the sink's WIDTH
differs by 7.3 px — the graph had moved and one port row was published in a different draw state. The
red run pressed 6.4 px above the port; the host answered the press as empty canvas and panned the camera
(`nodeGraphViewport` every ~500 ms from 113 016 to 121 485 ms, with `dag hover changed: extrusion-axis`
riding along as the graph slid under the pointer), and no wire draw ever started. **A press that never
reached a port is not evidence that a graph refuses to be rewired**, and the row graded it as exactly
that.

**Why the published rect was stale.** `dagIntroductionResolver`
(`🕸️NodeGraph/🟦️.tsx:1638`) answers `entity("handle", …)` out of a `Map<string,string>` cache and only
*starts* the real read in the background; the cache entry carries no age and the answer carries no
staleness, so a caller that trusts the published geometry cannot tell a live rect from one several
frames old. The probe's guard — two equal reads 400 ms apart — cannot see through that: two reads of the
same stale entry are equal.

**Hardened** (`🐍️flow-window-probe.mjs`, `🐍️react-battery.mjs`):
- `publishedPort` now requires **three** consecutive equal reads (≥1.2 s of stability) instead of two;
- the redraw asserts the host's own `dag port press … interaction=draw-edge` line for the port it aimed
  at, and re-aims **once** at freshly published geometry when the press did not land;
- `redrawPressLanded` and `staleAim` are published in `wire-result.json` and carried into the battery's
  step detail and `key`, so a stale aim is a visible number rather than a silent red.

**Handed on, not fixed here:** the product half is `dagIntroductionResolver`'s undated cache. Two clean
designs exist — key the cache by the paint state it was read under (a miss then returns `null` and the
caller polls), or carry a `stale` flag on `IntroductionResolvedGeometry` — and both change the
introduction geometry contract that the guided-tour/demonstration surface owns, not this lane's. The
risk that decides between them is real and must be measured first: a state-keyed cache on a graph that
is animating never answers, which would leave a guided tour with no target at all.

---

## 5. Runtime verification after the fix

<!--RUNTIME-->

---

## 6. Files changed

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `gesture_changed_content`, published as
  `fixtureChanged` by `take_graph_edits_json`; the duplicate unreachable `Move` arm removed.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx` —
  `graphGestureAnswer`; the pointer-up fallback gated on `fixtureChanged`.

Laws and fixtures:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — two laws plus
  `world_screen_point` / `empty_canvas_world_point` / `gesture_answer` helpers.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫳️node-graph-gestures/🔣️.json` —
  rule `aContentlessGestureDispatchesNothing`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🫳️node-graph-gestures/🟦️.ts` — the
  TS twin of that rule; the rule-coverage filter names it.

Probes and battery (ticket folder):
- `🐍️generate-mode-probe.mjs` — `add-N` polls and reports `seconds`.
- `🐍️flow-window-probe.mjs` — three-read port geometry, landed-press assertion, one re-aim,
  `redrawPressLanded`/`staleAim`.
- `🐍️react-battery.mjs` — the `flow-wire` verdict carries those two.
- `🔍️node-graph-gestures.config.ts` — **new**; the only way to run the `🟦️.ts` twin under vitest.
- `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md` — this report.

---

## 7. What is NOT claimed

<!--NOTCLAIMED-->
