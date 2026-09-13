# 🎛️🔗️ wgpu RETAINED FORM CONTROLS + NODE-GRAPH WIRES — one commit authority, one pointer path

Ticket `2026/09/09/PROCEDURAL-3D-END-TO-END`, lane **wgpu-retained-controls-wires**, 2026-09-13.
Scope: `📓️audit-wgpu-parity-2026-09-13.md` ranked gaps **#1**, **#6** (retained form controls never
commit) and **#2**, **#5** (node-graph wire editing and minimap click are dead from wgpu's side).

Repo MCP was down all session (`repo -32602 invalid initialize params`, `semio CONNECTION_CLOSED`);
no ticket was opened, closed or reopened, `📓️status.md` and `🎫️ticket.json` were not touched. No
git-state-modifying command was run. No dev server was started or stopped. Nothing under
`🗑️generated` that this lane did not create was touched.

---

## 1. TL;DR

| | |
|---|---|
| **Gap #1 + #6 — retained form controls never reach the guest** | **Closed at the owning layer.** `events.rs` gains a `🔖️Commit` region that is now the ONE commit authority for a retained document: `Input`, `Toggle`, `Slider`, `NumberStepper`, `Ring` and `IconSelect` each turn their own gesture into their own `ActionDescriptor`, with the payload named by its trigger and merged over the node's authored args — React's own two rules, reproduced. The shell's separate immediate-mode `commit_focused_input`/`stepper_metas` system is untouched and stays what it always was: the CHROME's authority. |
| **Why they were mute** | Three stacked reasons, all now fixed. (a) The retained router documented "committing the edited value via `on_change` … a documented gap" and implemented none of it. (b) Three of the six kinds — `NumberStepper`, `Ring`, `IconSelect` — registered NO pointer target at all, so a press on them resolved the window beneath. (c) The shell routed only `HitKind::Input`/`Select` into the router; `Toggle`/`Slider` fell into the action-dispatch branch carrying `action: None`, i.e. dispatched nothing. |
| **Gap #2 + #5 — wires and minimap** | **Closed at the owning layer.** `DagHost` gains a bounded wire-edit journal and two predicates naming which of its TWO pointer paths owns a gesture; the wgpu canvas routes port/handle/minimap/widget gestures through the same `pointer_*_screen` entry React drives, and dispatches the resulting wires as a `nodeGraphEdit` `connect`/`disconnect` — the narrow operation the guest already declares, not React's whole-fixture republish (which no 16 KiB bounded action can carry). |
| **Laws** | Two new language-neutral fixtures, each with a Rust law driving the REAL pipeline and a TypeScript twin re-deriving the same answer independently. Rust: **2 passed** (15 fixture cases) + wire law (§6.2). TypeScript: **16 passed** + **5 passed**. |
| **6118 runtime proof** | **Partly proven, and the rest is blocked one hop upstream — measured, not guessed.** Boot now completes (8–10 s), 368 pointer events reach the shell, the retained registry mints and resolves body targets, and a retained row press now **dispatches its guest action** (`addGeneration`, twice, reproducible) — which it did NOT before a fix this lane made in the same pass (§7.2). The two headline gestures the brief asked for are still unreachable, each for its own upstream reason: the Form has no controls to press until `addGeneration` actually creates a generation (guest side), and the node-graph surface drops out of the shell's pointer map between paints (scenes side). Full evidence, both root causes and the exact next step: §7. |

---

## 2. Gap #1 + #6 — root cause, in three parts

### 2.1 The retained router had no commit at all

`🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` said so in its own docstring on `EventRouter`:

> Per-widget-variant semantics beyond generic routing (e.g. actually committing an edited `Input`'s
> value via its `on_change` `ActionDescriptor`) are a documented gap for a later milestone

`PointerUp`'s `CaptureKind::Press` arm handled exactly three things — `Select`'s popup, a `Button`'s
`action`, a `Stack`'s `activate` — and fell off the end for everything else. `Toggle`, `Slider`,
`NumberStepper`, `Ring` and `IconSelect` appeared in the file ONLY in `is_focusable`.

### 2.2 Three kinds were not even hit-testable

`retained_hit_registration` (`🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs:612`) registered `Stack`, `Button`,
`Input`, `Select`, `Toggle`, `Slider` and `ComponentScene`, and answered `_ => None` for everything
else. A press on a generation's stepper, ring or icon field therefore resolved the WINDOW's own
`HitKind::ScrollRegion` — the exact failure mode `📓️wgpu-runtime-mailbox-dispatch-2026-09-13.md` §6
measured for tree rows, still live for these three kinds.

### 2.3 The shell routed only two kinds into the router

`route_retained_pointer_press` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) read:

```rust
if matches!(kind, HitKind::Input | HitKind::Select) { … dispatch_ui_event … }
else if down { if let Some(action) = action { self.dispatch_action(action).await?; } }
```

A `Toggle` or `Slider` registration carries `action: None` by design ("the retained event router owns
what a press on those means" — the hit-target fixture's own rule), so a press on one took the `else`
branch and dispatched **nothing**.

### 2.4 The audit's open question, answered

`📓️audit-wgpu-parity-2026-09-13.md` §4 could not confirm whether the shell's
`AppInteractionState.input`/`commit_focused_input` system was reachable from a retained click. It is
**not**. `commit_focused_input` reads `input.focused_id` and looks it up in `widget_maps.input_metas`
/ `stepper_metas`, and those maps are written only by `framework_widget_context`'s immediate-mode
chrome walk (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11390-11440` clears them; `:11872`, `:11990` are the two
places that build the context). A retained document's nodes are never in them: the retained body is
rendered by `render_ui_document_step`, which drives `UI_ENGINE` (ingress → reconcile → layout → paint)
and never writes a widget map. So editing a generation's parameters on wgpu was fully cosmetic —
exactly the high-impact reading the audit ranked #1, now confirmed rather than suspected.

---

## 3. Gap #1 + #6 — the fix, per file

### 3.1 One shared control geometry — `🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs:150-181`

| fn | what it is |
|---|---|
| `number_stepper_segments(bounds) -> [Rect; 3]` (`:156`) | decrement / value / increment as thirds of the box |
| `slider_value_at(bounds, x, min, max, step) -> f64` (`:164`) | the full-width track paint draws, snapped onto `step`, clamped |
| `ring_t_at(bounds, x, y) -> f64` (`:177`) | the turn fraction of the angle from the centre of the ring's own bounds |

`paint` and `events` both call these, so the `−` a user sees and the `−` a press resolves cannot
drift. `paint_number_stepper` (`🖌️paint/🦀️.rs`) was rewritten to take its segments from the same
helper rather than recomputing `bounds.w / 3.0` itself.

### 3.2 The commit authority — `⚡️events/🦀️.rs:395-517` (`//#region 🔖️Commit`)

Parity reference is React's **retained** interpreter, not its older declarative-control path:
`🗣️Interpreter/🟦️.tsx`'s `dispatchTrigger` → `📃️UiDocumentStore/🟦️.tsx`'s `emitIntent` →
`🛠️ShellHelpers/🟦️.tsx`'s `uiIntentPayload`/`uiInputField`. Two rules come from there:

- a scalar payload is **named by its trigger** (`delta` for `Trigger::Delta`, `value` for every other);
- it is **merged over** the node's authored args, never substituted for them.

That second rule is load-bearing: a generation's field authors `{generationId, questionId}` into its
descriptor, and a payload that replaced them would leave `updateGenerationValues` with a value and no
idea what it belongs to. (React learned the same lesson the hard way — `uiIntentPayload`'s own
docstring records `setGridSpacing` arriving as a bare `10.5` and settling with zero effects.)

| fn | file:line | what it does |
|---|---|---|
| `commit_action` | `:424` | the merge, plus the silence rule: an EMPTY action name is `reconcile::record_action_or_inert`'s carrier for "no binding declared", and answers `None` — the exact condition `emitIntent` answers `undefined` for |
| `editable_value` | `:440` | which kinds own a text buffer: `Input`, and `IconSelect` (whose value IS its icon string, which React edits in the `IconSelector`'s own textarea) |
| `commits_on_blur` | `:450` | React's `commitOnBlur` — `commit == "blur"` means Enter/blur only |
| `edit_commit_action` | `:456` | `{value: <text>}`, numeric for an `Input` of kind `number` |
| `pointer_commit_action` | `:474` | `Toggle` → `!presence.selected`; `Slider`/`Ring` → the gesture read off `layout`'s geometry; `NumberStepper`'s outer thirds → relative through `on_delta` when that binding exists, absolute through `on_absolute` otherwise (React's `NumberStepperView` makes exactly that choice), value segment → nothing |
| `commits_while_dragging` | `:496` | `Slider`/`Ring` report every intermediate value, not only the release |
| `absolute_rect` | `:502` | the ancestor-offset accumulation, extracted out of `scene_command` so both use one walk |

Wired into `dispatch` at four points: `PointerDown` seeds a `Slider`/`Ring` immediately (`:1449`),
`PointerMove` keeps a captured one live (`:1300`), `PointerUp`'s `Press` arm commits every other kind
(`:1499`), and `FocusState::set_focus` (`:327`) returns the blurred node's commit so losing focus is a
real dispatch moment. Keystroke commits flow through `changed_buffer_action`/`push_buffer_change`
(`:1172`, `:1181`) from `route_text_insert`, `route_ime`'s `Commit` and `route_edit_key`'s mutating
keys; `Enter` (`:1240`) commits a blur-policy node and deliberately does NOT double-fire a
change-policy one.

### 3.3 `Trigger::Commit` was never mapped — `🔀️reconcile/🦀️.rs:560`

`InputProps`' own doc says the binding is `Trigger::Change` **or** `Trigger::Commit`, and
generation3d's inline rename editor binds `Commit` (`🌀️procedural/🫀️core/🖼️semantic-ui/🦀️.rs:56-70`,
"Enter / blur"). `input_node` read `Trigger::Change` unconditionally, so a rename field's descriptor
was the inert empty-action placeholder. `input_commit_action` now picks the trigger the same way
React's `InputView` picks it — `commitOnBlur ? "commit" : "change"` — keeping the retained node's
single `on_change` field the one descriptor the commit authority dispatches.

### 3.4 The three unregistered kinds — `📥️input/🦀️.rs:60-62, 668-670`

`HitKind` gains `NumberStepper`, `Ring`, `IconSelect`; `retained_hit_registration` registers all
three under the node's own id with no dispatched action (the router owns the meaning), `Ring` with
`DragAxis::Both` and `Slider`'s existing `Horizontal` unchanged. `👆️cursor/🦀️.rs` gives them
cursors (`IconSelect` text, `Ring` grab/grabbing, `NumberStepper` selectable).

### 3.5 The shell now routes every control kind — `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6850`

The `matches!` widens to all seven control kinds. Nothing else in that function changed.

### 3.6 A drag that leaves the control — `⚙️engine/🦀️.rs:1810`, `🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs:412`, `🐚️Shell/…:6833`

`route_retained_pointer_move` resolved its target window from `input.hit_at(x, y)`, so a `Slider` or
`Ring` drag froze the instant the pointer left the control's own rect — the registry answered
something else and the router stopped receiving moves. `Ui::window_with_pointer_capture` exposes the
window whose retained content holds capture, `interpreter::retained_pointer_capture_window` bridges
it, and the shell prefers it over the hit registry while a gesture is live — a browser keeps a
captured pointer with the element that captured it until release.

### 3.7 The stepper's `−`/`+` were never painted — `🖌️paint/🦀️.rs:881-925`

The streaming production paint (`paint_node_step`) drew only an outer border and the value; only the
`#[cfg(test)]` `paint_number_stepper` drew segments. The production arm now paints the two dividers,
the nested value border and the `−`/`+` glyphs at exactly the rects `events` hit-tests
(`stepper_glyph_rect`, `:226`). A focused `IconSelect` now shows its live buffer rather than its
declarative value (`:955`), the way `paint_input` already does.

---

## 4. Gap #2 + #5 — root cause

`DagHost` has **two** pointer entries, and the wgpu renderer only ever called one of them.

| entry | vocabulary | reaches ports? |
|---|---|---|
| `derive_pointer_plan`/`apply_pointer_plan` (`dag.rs:3103`, `:3225`) — the bounded, atomically-admitted one | `Idle \| Pan \| Drag \| Select` | **no** |
| `pointer_down_screen`/`_move_screen`/`_up_screen` (`dag.rs:5054-5198`) — the one React drives | everything, incl. `InteractionMode::DrawEdge`, the minimap widget, port insertion, inline widget sliders | **yes** |

`engine-canvas.rs`'s `node_graph_pointer_{down,move,up}_into` called only the first. Worse than
"nothing happened": `bounded_node_hit_index` answers `DagInteractionPlanFault::Unsupported` for
exactly the minimap / handle / port-insert / widget hits, `graph_plan_fault` maps it to
`BoundedActionFault::Structure`, and the `?` aborted the whole dispatch. **A press on a port was a
fault, not a gesture.** Minimap click-to-navigate inherits the same block, which is why the audit
ranked #5 as "inherits #2 rather than being a separate gap" — confirmed.

React reaches the same place through `session.pointerDownScreen(...)` (`🕸️NodeGraph/🟦️.tsx:2977`) and
then re-publishes the WHOLE fixture (`commitFixture`, `:750`). A fixture JSON does not fit a 16 KiB
bounded action — and does not have to: `FlowNodeGraphEditOp` already declares a narrow `connect`
carrying four ids, and React's own ReactFlow path dispatches exactly that shape (`:1046-1054`).

---

## 5. Gap #2 + #5 — the fix

### 5.1 `DagHost` journals what a gesture did — `dag.rs:1944-1951, 4636-4684`

```rust
pub enum DagGraphEdit {
    Connect { source_node_id, source_port_id, target_node_id, target_port_id },
    Disconnect { edge_id },
}
```

`process_engine_events` already drained `BoardEvent::EdgeConnected`/`EdgeRemoved` and did nothing but
log them. It now also journals them in the GUEST's vocabulary: `journal_connect` (`:4636`) resolves
each handle through `handle_key_map`, which already keys every handle as `"{nodeId}@{portId}"` — the
same grammar the pick/hover channel uses, so this is a lookup and not a second derivation of what a
port is. `journal_disconnect` (`:4646`) names the SYNAPSE id from `edge_id_map`; an edge this same
gesture created has no synapse id and is deliberately not journalled (the guest never learned about
it). `push_graph_edit` (`:4653`) is bounded at `DAG_GRAPH_EDIT_CAPACITY = 8` and drops duplicates;
`take_graph_edits` (`:4662`) is the renderer's one read point. An undrained journal is retired through
the same string ladder every other owned id uses (`DagRetirementOwner::Strings`).

### 5.2 The host names which path owns a gesture — `dag.rs:4670, 4677`

- `screen_pointer_gesture_active()` — a wire draw, minimap drag, pan, inline widget drag or port
  insert is already in flight, so the follow-up move/up must not be claimed (and cancelled) by the
  bounded path.
- `screen_pointer_gesture_begins_at(sx, sy)` — a press here would begin one. These are exactly the
  four hits `bounded_node_hit_index` faults on, asked as a question instead of as a fault.

### 5.3 The wgpu canvas routes them — `engine-canvas.rs:2881-3037` (`//#region 🔗️NodeGraphScreenPointer`)

`node_graph_pointer_{down,move,up}_into` now consult `node_graph_gesture_is_screen_path` (`:2917`)
BEFORE reserving, and hand the intent to `node_graph_screen_pointer_into` (`:3025`) when it answers
yes. That function reserves **four** action credits, runs the gesture through the host's own
`pointer_*_screen` (`:2939`), then writes the same `interactionSelect`/`interactionHover`/
`nodeGraphViewport` triple the bounded path publishes plus `write_graph_edit_action` (`:2985`):

```json
{"operations":[{"operation":"connect","sourceNodeId":"…","sourcePortId":"…","targetNodeId":"…","targetPortId":"…"}]}
```

The reservation is taken **before** the host is allowed to mutate, so the credits a dispatch needs
are already held when the wire is drawn. Node selection, node dragging and marquee keep their
bounded, atomically-admitted plan/commit path untouched.

### 5.4 The projection has to be re-seeded — `🌊️flow/🖥️host/🦀️.rs:561`, `🗺️surface/🕸️node-graph/🦀️.rs:527`

Both hosts' `pointer_*_screen` bump `interaction_revision` without refreshing
`interaction_projection`, so after any screen-path gesture `plan_pointer` would answer `Unsupported`
for the rest of the session. `resync_interaction_projection()` on each host closes that, called once
per screen gesture.

---

## 6. Laws

### 6.1 Retained control commit

- Oracle: `🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json` — 7 declared rules, **15 cases**
  covering all six value-carrying kinds plus the two silence cases (an unbound trigger; a
  blur-policy field mid-typing) and the stepper's inert value segment.
- Rust law: `🖱️ui/🧪️tests/🎛️retained-control-commit/🦀️.rs`, driving the REAL `EventRouter` over a
  real `UiTree` through real hit-testing, capture and focus — `cargo test -p semio-framework-ui
  --features testkit --lib -- control_commit` → **2 passed; 0 failed** (413 filtered).
- TypeScript twin: `📺️renderer/🧑‍🎨engine/🧪️tests/🎛️retained-control-commit/🟦️.ts`, re-deriving every
  expected dispatch a second, independent time from React's own `uiInputField`/`uiIntentPayload`/
  `emitIntent` rules — `bun test` → **16 pass; 0 fail; 68 expect() calls**.

The fixture pins the two payload rules by construction: every expected `args` map carries BOTH the
node's authored ids and the gesture's own value, so a merge that replaced instead of merging fails
15 of 15.

### 6.2 Node-graph wire editing and minimap

- Oracle: `♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json` — 7 declared
  rules, 5 gesture cases + 2 minimap cases over a three-node port graph.
- Rust law: `…/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`, driving the REAL `DagHost` — real port hit-testing,
  a real `InteractionMode::DrawEdge`, a real minimap layout — and asserting the journal, the live
  edges, the single-drain rule, the path discriminator and the camera. Result: §6.4.
- TypeScript twin: `📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts` — `bun test` →
  **5 pass; 0 fail; 15 expect() calls**. It pins the half a Rust law cannot: the four argument names
  are read out of `🕸️NodeGraph/🟦️.tsx`'s own `onConnect` source, so a rename on either side fails the
  test instead of silently splitting the two renderers.

### 6.3 The hit-target oracle was extended, not forked

`🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json`'s `rules.input` now names all seven control kinds
and points at the commit oracle for what a press on one MEANS; `🖱️ui/🧪️tests/🎯️retained-hit-targets/
🦀️.rs`'s `kind_name` covers the three new `HitKind`s. That law still passes (`bun test` on its twin:
**11 pass**).

### 6.4 Wire law result

`cargo test -p semio-framework-os-infinite --lib -- wire_edit` → **3 passed; 0 failed** (339 filtered):

| test | what it proved |
|---|---|
| `a_port_to_port_drag_creates_a_wire_and_journals_it_for_the_guest` | all 5 gesture cases: a drag from `src@out` to `tgt@in` creates the wire AND journals `connect{src,out,tgt,in}`; a second wire into the same input replaces the first and leaves exactly one live edge; a drained journal never reports the same wire twice; a press on a port belongs to the screen path and a press on empty canvas does not |
| `a_wire_in_flight_holds_the_screen_pointer_path` | an idle graph is on the bounded path, a press on an output port enters `InteractionMode::DrawEdge` and takes the screen path, and the release hands the graph back |
| `a_minimap_click_moves_the_camera` | a press inside the panel but outside the viewport rectangle re-centres the camera; a press on the viewport rectangle grabs it and leaves the camera alone |

Two fixture-authoring errors were found and fixed by running it, both worth recording because they are
the kind a law that is never run keeps forever: `minimap::layout` reports its rects as CORNERS
`(x0, y0, x1, y1)` rather than origin+size, and *which* part of the panel lies outside the viewport
rectangle depends on the camera — so the law now searches the panel for that relation instead of
asserting a fixed corner.

---

## 7. 6118 — what the runtime actually does now

The coordinator's `flowEvalTick` retirement fix and catalogue decode fix landed, and **`boot_shell`
now returns**: measured at 8.1–10.4 s across four runs on
`http://127.0.0.1:6118/?plugin=generation3d&mode=generate`, against the wgpu serve's own renderer
wasm rebuilt from this lane's source (verified by finding this lane's own log strings inside
`dist/wasm-dev/semio-framework-os-renderer-wgpu_bg.wasm`). So the §7 of the first draft of this
report — "zero pointer events exist to route" — is obsolete, and everything below is browser-measured
rather than inferred.

Probes (both new, both in the ticket root):
`🐍️wgpu-control-commit-probe.mjs`, `🐍️wgpu-wire-drag-probe.mjs`.
Raw console, verdicts and screenshots under `🗑️generated/wgpu-controls/{commit-1..7,wire-1,boot-2}/`.

### 7.1 What now works end to end, measured

| hop | evidence (run `commit-7`, reproducible across `commit-6`/`commit-7`) |
|---|---|
| boot completes | `boot_shell leave` at 8.1 s; dock plan places all three generate-mode windows |
| DOM input reaches the shell | `os_host handle_event` **368**, `dispatch_normalized_event` **368**, `os_host pointer hit` **262** |
| the retained registry mints body targets | `generation3d-generations` **4 ids**, `generation3d-generate-preview` **1**, the whole chrome **36** |
| the registry resolves a retained row | a pointer at `(160.5, 138)` answers `Some((TreeItem, "tree.label.procedural3d-play-generate.add-generation", Some("addGeneration")))` |
| **the retained press router dispatches the row's own action** | `wgpu-shell retained press window=generation3d-generations kind=TreeItem down=false action=Some("addGeneration")`, and the guest answers: `wgpu-bridge typed-operation command instance=1 pages=6 terminal=true`, followed by a re-render of all three surfaces |

That last row did not happen before this session. It took a fix, §7.2.

### 7.2 Found and fixed here: a retained row's action fired on the press, and the press never resolved

`route_retained_pointer_press` dispatched a node's declared action under `else if down`. On 6118 that
branch is **never** taken with an action, because `InputState`'s pointer registry is drained one entry
per frame-build boundary step (`FrameBuildPhase::InputFrame`: `if self.input.hit_targets.pop().is_some() { return Pending }`)
and rebuilt by the chrome walk — so a `PointerDown` arriving inside that window resolves an empty
registry. Browser-measured, run `commit-5`, every press:

```
13778 [DEBUG] wgpu-shell pointer button x=160.5 y=38 down=true  targets=0  hit=None
13930 [DEBUG] wgpu-shell pointer button x=160.5 y=38 down=false targets=36 hit=Some((Button, Some("dock.tab.0.generation3d-generations.new"), None))
22114 [DEBUG] wgpu-shell pointer button x=160.5 y=66 down=true  targets=0  hit=None
22325 [DEBUG] wgpu-shell pointer button x=160.5 y=66 down=false targets=36 hit=Some((TreeItem, Some("section.chevron.…generations"), None))
```

Every Down: `targets=0`. Every Up: `targets=36` and the correct control. `Add Generation` resolved
perfectly on release and dispatched nothing, because the code only looked on press.

The fix is a one-word change with a real semantic behind it: **a click is press AND release, and the
action fires on the release** — the same rule `events::EventRouter` already applies to a `Button`
(`PointerUp` inside the pressed node), and the same rule a browser applies. After it, `addGeneration`
dispatches, twice, in both subsequent runs.

Two temporary diagnostics were added to make this visible and are deliberately left in place for the
input lane (both `[DEBUG]`-prefixed, both firing only on a real press, not per frame):
`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6474` (`wgpu-shell pointer button … targets= … hit=`) and `:6894`
(`wgpu-shell retained press …`). A third, per-frame one (`retained body hits`) was added, used to
produce the registration counts above, and **removed** again because it logged ~370 lines per run.

### 7.3 Still blocked, hop by hop — and these are not this lane's layers

**(a) The Form field commit (`updateGenerationValues`) — blocked on the guest's `addGeneration`.**
The Form window's retained document registers **0** hit targets, and that is *correct*: with no
generation selected the form renders one `ui::text` placeholder and nothing else
(`🎭️modes/🧬️generate/🪟️windows/📝️form/🦀️.rs:125-127`). There is nothing to press until a generation
exists. `addGeneration` now reaches the guest and its typed-operation command settles
`terminal=true` — but the generations tree still registers the same 4 rows afterwards
(`(no generations)` unchanged) across the whole run, so **no generation is created**. That is the
plugin's own `addGeneration` → `FlowMutation` → window-transient path, downstream of every renderer
hop and owned by neither this lane nor the input lane.

**(b) The wire drag — blocked on the node-graph surface dropping out of the shell's pointer map.**
In the mode whose dock carries `procedural-main` (the Flow canvas, `975x814+3,54`) the graph is live:
it registers **31** retained hit targets and paints (`[DEBUG] dag draw lod=normal zoom=0.925`). But a
slow 40-step sweep plus a slow press-drag-release straight across it produced, out of 2042 console
lines, exactly **one** interesting line — that same `dag draw` — and **zero** `dag hover changed`,
zero `node-graph screen gesture`, zero `nodeGraphEdit`. The graph's pointer dispatch does not go
through the hit registry at all; `handle_pointer_move`/`handle_pointer_button` iterate
`self.shell.node_graph_states` and test `surface.bounds.contains(x, y)`. That map is rebuilt every
`sync_engine_surface_states` from `take_engine_surface_registrations()`, and **any graph absent from
that drain is removed in the same frame** (`🐚️Shell/…:4035-4037`). The graph paints twice in 40 s, so
it is absent from nearly every drain — and while it is absent, no pointer event can reach
`node_graph_pointer_down_into`, which is where this lane's screen-pointer path begins. Owning layer:
the engine-surface/scene lifecycle, not this lane.

### 7.4 The exact next step, for whoever unblocks (a) and (b)

1. **(a)** Run `🐍️wgpu-control-commit-probe.mjs` and read `🗑️generated/wgpu-controls/commit-7/console.txt`
   from the `retained press … action=Some("addGeneration")` line forward: the command settles, the
   surfaces re-render, the tree does not change. Instrument `addGeneration`'s own mutation.
2. **(a), once a generation exists**, the probe's Form sweep will already find the field: it presses
   and types down the Form column and stops at the first `updateGenerationValues`. Nothing more is
   needed from this lane's side — `Input` → `EventRouter` → `UiCommand::App` → `publish_retained_action`
   is law-proven (§6.1) and its only runtime prerequisite is a registered `HitKind::Input`, which
   §7.1 shows the registry now mints for real nodes.
3. **(b)** Make a node-graph surface's presence in `node_graph_states` survive a frame in which its
   window did not repaint (or re-register it every walk). Then `🐍️wgpu-wire-drag-probe.mjs` will
   sweep for `dag hover changed`, drag port-to-port, and the canvas will log
   `[DEBUG] wgpu node-graph screen gesture surface=… edits=[Connect { … }]` from
   `engine-canvas.rs:3030` followed by the `nodeGraphEdit` dispatch.

### 7.5 What is therefore claimed, and what is not

**Claimed:** boot completes; input reaches the shell; the retained registry mints and resolves real
body targets; a retained node's declared action dispatches to the guest and the guest settles it.
**Not claimed:** a Form field's typed value reaching `updateGenerationValues` on 6118, and a wire
drawn on 6118. Both are proven against the real pipelines natively (§6) and both are blocked at
runtime by a named, measured hop in another layer.

## 8. Build and test evidence

| command | result |
|---|---|
| `cargo check -p semio-framework-ui --features testkit` | `Finished` (2 pre-existing warnings, both from a peer's `paint.rs` edit) |
| `cargo check -p semio-framework-os-infinite` | `Finished` |
| `cargo check -p semio-framework-os-renderer-wgpu` | `Finished` |
| `cargo test -p semio-framework-ui --features testkit --lib -- control_commit` | **2 passed; 0 failed** |
| `cargo test -p semio-framework-os-infinite --lib -- wire_edit` | **3 passed; 0 failed** |
| `nx run @semio-tech/framework-renderer-wgpu:wasm` | `exit=0`, 0 errors; this lane's own log strings verified present in the served `dist/wasm-dev/…_bg.wasm` |
| `bun test …/🧪️tests/🎛️retained-control-commit/🟦️.ts` | **16 pass; 0 fail** |
| `bun test …/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts` | **5 pass; 0 fail** |
| `bun test …/🧪️tests/🎯️retained-hit-targets/🟦️.ts` (regression) | **11 pass; 0 fail** |

### 8.1 Two peer compile breaks fixed forward

- `♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs:1537` — `World3dScene` gained `instances_delta_json` and this
  test's literal was not updated, so `semio-framework-os-infinite`'s whole test profile failed to
  build. Added `instances_delta_json: None`, one line, nothing else touched.
- `🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:89` referenced a test file that did not exist yet and
  broke the ui crate's test profile for ~20 minutes. **Not** fixed — that is the a11y lane's own file
  and it landed on its own; this lane waited.

### 8.2 A note for the coordinator: the disk filled

Mid-session both Rust laws died with `No space left on device (os error 28)` — 10 GiB free out of
926, against a 348 GB `⚡️cache/cargo/build`. 128 native incremental directories older than two hours
were pruned (`debug/incremental` only; the two wasm target trees were left alone because peers were
building into them), which returned **181 GiB**. This is the known failure mode recorded as
"Incremental Cache Regrows Under Fleet"; under a full fleet it will recur.

---

## 9. Peer boundaries observed

- **wgpu-input-hit-runtime** owns pointer/wheel/key routing, the hit registry and chrome-action
  dispatch. This lane added three `HitKind` variants and their three registrations in
  `📥️input/🦀️.rs`, widened one `matches!` in `route_retained_pointer_press`, and made
  `route_retained_pointer_move` prefer a captured window — the minimum for the six control kinds to
  be reachable at all. Nothing else in that lane's surface was touched, and its own report's findings
  are cited rather than re-derived.
- **wgpu-example-chain** (evaluate/mesh/resident budget) — untouched. §7.3(b)'s finding (a node-graph
  surface drops out of `node_graph_states` in every frame its window does not repaint) sits in that
  area and is reported rather than changed.
- **wgpu-status-a11y-i18n** (World3d status, accessibility projection, chrome i18n) — untouched. Its
  in-flight `♿️accessibility/🦀️.rs` referenced a test file that did not exist yet and broke the ui
  crate's test profile for ~20 minutes; this lane waited rather than writing that lane's file.

---

## 10. Files

**Changed**

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs` — `🎛️ControlGeometry` region
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚡️events/🦀️.rs` — `🔖️Commit` region + dispatch wiring
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs` — `input_commit_action`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs` — stepper segments, IconSelect buffer
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/📥️input/🦀️.rs` — three `HitKind`s + registrations
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/👆️cursor/🦀️.rs` — cursors for the three
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs` — `window_with_pointer_capture`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎯️retained-hit-targets/🦀️.rs` — new kind names
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎯️retained-hit-targets/🔣️.json` — rule text
- `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs` — `resync_interaction_projection`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `resync_interaction_projection`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs` — journal + path predicates + retirement
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — screen pointer path
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs` — capture-window bridge
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — control routing + captured move + **release-dispatch fix** (§7.2) + two press diagnostics
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs` — one-line peer fix-forward (§8.1)

**Created**

- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🎛️retained-control-commit/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🎛️retained-control-commit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧫️fixtures/🔗️wire-edit/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎛️retained-control-commit/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔗️node-graph-wire-edit/🟦️.ts`
- `<ticket>/🐍️wgpu-control-commit-probe.mjs` — the retained-control runtime probe (§7)
- `<ticket>/🐍️wgpu-wire-drag-probe.mjs` — the node-graph wire runtime probe (§7)
- this report

**Temporary `[DEBUG]` logs, all three deliberately left in place because the two open blockers in §7.3
are diagnosed through them** — remove once those are closed:

- `⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:3030` — `wgpu node-graph screen gesture surface=… edits=…`,
  fires only when a gesture actually edits a wire.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6474` — `wgpu-shell pointer button … targets=… hit=…`, fires once per
  real press; this is the line that measured the empty-registry-on-`PointerDown` defect.
- `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6894` — `wgpu-shell retained press …`, fires once per retained press.

A fourth (`retained body hits`, per frame) was added, used for §7.1's registration counts, and removed
again because it logged ~370 lines per run.
