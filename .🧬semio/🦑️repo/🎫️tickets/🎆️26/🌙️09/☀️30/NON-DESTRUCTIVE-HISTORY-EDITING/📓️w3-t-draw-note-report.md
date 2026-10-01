# 📓️ W3-T-DRAW Report: Draw and Note Tool-Machine Conversion

Executor W3-T-DRAW. Scope: the draw plugin (`✏️s/🔌️plugins/🖍️draw`) and the note plugin (`✏️s/🔌️plugins/🗒️note`), plus the
hosts that drive their gestures. Brief: `🧭️plan.md` "W3-T brief"; contract: `📋️design.md` §5, §7, §10, §11.

Aliases: `DR` = `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing`, `DA` = `DR/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`NO` = `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any`,
`RE` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

## 1. Census (before this work)

### 1.1 Draw

| # | Gesture | Entry | State machine | Commit today | Cancel today |
|---|---|---|---|---|---|
| D1 | layer drag (selectDirect press on a layer, drag, release) | `canvasPointerDown/Move/Up` → retained `DrawingInstanceOperationOwner` | vendored `fsm::statechart! drawing_gesture` (`moving_layer`) + `DrawingSession.layer_move` scratch | `finish_layer_move` → `Emit::commit(update_layer_transform × N, "Move selection")` — absolute final transforms | `canvasPointerUp{cancelled}` / `canvasEscape` → fsm `Escape`, scratch dropped |
| D2 | resize handle (0–7) | same | same, `LayerMove.handle` | same, `"Resize selection"`, absolute local matrices | same |
| D3 | rotate handle (8) | same | same | same, `"Rotate selection"` | same |
| D4 | node drag (editNodes press on a selected point) | same | `moving_layer` + `DrawingSession.node_move` scratch | `finish_node_move` → `plan_selection` → `Emit::commit(update_path_geometry × N, "Move path points")` — absolute segments | same |
| D5 | marquee / lasso select (selectMarquee, selectLasso, node marquee) | same | `marqueeing`, effect `CommitMarquee` | interaction only (`interactionSelect`), no mutation | Escape |
| D6 | click pick (selectDirect/editNodes release) | same | `idle` effect `PickPoint` | interaction only | – |
| D7 | shape drag (shapeRect/Ellipse/Line) | same | `shape_dragging`, effect `CommitShape` | `commit_with_utility_reset(create_layer, "Add shape")` | Escape → nothing |
| D8 | pen / polygon draft (click sequence, double-click / Enter) | `canvasPointerDown`, `canvasDoubleClick`, `canvasCommitDraft` | `drafting`, effect `CommitDraft` → `DrawingDraftQuery` (one point per dispatch) | `commit_with_utility_reset(create_layer, "Commit draft")` | Escape |
| D9 | trace (trace utility click) | `canvasPointerDown` + self re-dispatch continuation | `idle` effect `CommitTrace` (ignored) + `TracePointerJob` | `commit_with_utility_reset(create_layer trace, "Trace image")` | `canvasEscape` cancels job |
| D10 | keyboard nudge (8 verbs) | `nudgeSelection*` | none | `plan_selection` → `Emit::commit(update_layer_transform / update_path_geometry, "Nudge …")` absolute | – |
| D11 | inspector opacity slider | `setSelectedOpacity` | none | `Emit::amend(set_layer_opacity × N, "opacity")` per tick (pattern B) | none |
| D12 | inspector field edits (`patchLayer(s)`, `editPath`, `editFill`, `editSelection` align/distribute/stack/group) | commands | none | `Emit::mutations` / `Emit::commit` of absolute field sets — the user's intent IS the absolute value | – |

Vendored copy: `DR/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm` (crates `semio-s-plugin-draw-fsm`,
`semio-s-plugin-draw-fsm-macros`; 1554 + 1251 lines, 9 test files) — a diverged private copy of `🔄️machine`.

### 1.2 Note

| # | Gesture | Entry | Commit today | Cancel today |
|---|---|---|---|---|
| N1 | block move (selectDirect/selectMarquee drag) | `inkApplyEvents` phases `begin`/`live`/`commit` (state events `updateBlock`) | per-tick diff → absolute `move-block`, `coalesce_key: "note-gesture"` amend (pattern B) | host drops the tail; begin/live ticks already in the document (`artifactAfterCancel: retain-accepted-begin-live`) |
| N2 | resize handles | same | absolute `move-block` + `resize-block` (+ `edit-block-ink-stroke` for strokes) amended | same |
| N3 | pencil stroke | `begin` (`addBlock`) + `live` + `commit` | `create-block` then `edit-block-ink-stroke` ticks amended | same |
| N4 | eraser (stroke / point) | `begin`/`live`/`commit` | `delete-block` / `edit-block-ink-stroke` amended | same |
| N5 | place text/image/table/math, clipboard, inline edits | `inkApplyEvents` `atomic` | one plain edit | – |
| N6 | keyboard nudge (9 verbs) | `nudgeSelection*` | retained work splits the selection into one unit per block → N `drag-blocks` leaves | – |
| N7 | marquee / pick | host only | interaction only | – |


## 2. Design

### 2.1 Draw: the canvas tool

- The vendored `fsm` copy is gone. The canvas gesture is now `canvas_tool`, a `machine::statechart!` whose effects are
  `ToolYield<DrawingMutation>`, driven by `semio_framework_tool_machine::ToolMachineRunner` (wrapper `DrawingTool`).
  - States: `idle`, `pressing`, `dragging`, `marqueeing`, `shaping`, `drafting`, `tracing`.
  - Events: `PointerDown`, `PointerMove`, `PointerUp`, `Grab`, `NodeMarquee`, `CommitDraft`, `Traced`, `Once`, `Escape`.
  - Tool id: `s.draw.drawing@1/*#editor#<utility or verb>` (for example `…#selectDirect`, `…#pen`, `…#nudgeSelectionRight`).
  - Clock: wall time plus a process-monotone logical tick (`drawing_tool_clock`), so two gestures in one millisecond
    still mint distinct `TransactionRef`s.
- Every document-changing gesture is ONE `ToolTransaction` published with `Emit::commit_transaction` (no description,
  no coalesce key; the history row reads the leaf label through the new `ArtifactEditor::mutation_label`).
  - Layer drag → `drag-layers {targets, dx, dy}` (relative, world offset).
  - Rotate handle → `rotate-layers {targets, pivotX, pivotY, angle}`; resize handles → `scale-layers {targets, pivotX, pivotY, scaleX, scaleY}`.
  - Node drag and node nudges → `drag-path-points {targets:[{layerId,index,point}], dx, dy}`.
  - Layer nudges (8 verbs) → one `drag-layers` one-shot (`Once`) per keypress.
  - Shape drag, pen/polygon draft, trace → one `create-layer` per transaction (created layer selected, default utility restored as host effects).
  - Every drag tick upserts the one net leaf under the key `gesture`; an identity motion retracts it; Escape and capture loss abort with zero trace.
- Selection gestures (marquee, lasso, click pick, node marquee) stay interaction, not mutations: the tool only tracks
  them; the session derives their bounded queries from the tool context at release.
- Where the tool lives: in the retained `DrawingSession` of the draw gesture owner (one per app instance, in memory).
  That owner already retires on a base-revision change or a utility change — the equivalent of the `baseMoved` and
  `retired` aborts — so draw does NOT persist the runner in the window transient (puzzle 2d's model). Preview is derived
  from the tool context (`DrawingSession::preview`).
- Retained store: draw's bespoke owned store engine (`🧰️owned`) applies the four new leaves through their diffs; the
  one-item fold declaration is now per mutation (`drawing_inverse_rows`): a selection transform restores one absolute
  row per addressed layer (`update-layer-transform` / `update-path-geometry`, exact, base-derived), every other leaf is
  point-invertible.
- Opacity slider (D11): `setSelectedOpacity` returns `Emit::mutations` of absolute `set-layer-opacity` leaves; the
  framework scrub machine (W3-T2-CONTROLS, `📓️api-scrub-machine.md`) folds a drag into one transaction.

### 2.2 Note: the ink tool

- `ink_tool` statechart (`idle`, `streaming`; events `Once`, `Stream`, `Finish`, `Cancel`) with
  `ToolYield<NoteMutation>` effects, wrapped by `NoteInkTool` and driven by `note_ink_dispatch`.
- Wire (`inkApplyEvents`): `phase` = absent (one-shot) | `stream` | `commit` | `abort` (+ `reason`); optional
  `gestureJson` drag record `{kind:"drag", ids, dx, dy}`. The old `begin`/`live`/`atomic` phases are gone.
- Yields:
  - A block drag yields ONE relative `drag-blocks` under the key `gesture` (net offset from the press).
  - Strokes, erasers, resizes and placements yield, per touched block, the net leaves the whole gesture makes against
    the committed document (`create-block` / `delete-block` / field updates), keyed `block:<id>#<n>` (stale keys
    retracted); assets keyed `asset:<key>`.
- Between dispatches the open transaction rides `NoteCompositeWindowTransient.inkTool` (`NoteInkToolState`: states by
  stable id, verb, authoring seed, base revision, `TransactionRef`, entries in value form) exactly like puzzle 2d's
  select tool. `baseMoved` / `captureLost` (another verb or a one-shot) abort the open gesture.
- The composite window paints committed ⊕ provisional (`note_ink_tool_preview` in `render_with_request_context`).
- Nudges (9 verbs) are ONE `drag-blocks` over every unlocked selected block, one transaction per keypress; the retained
  work no longer splits nudges per block, and `inkApplyEvents` is one semantic unit.
- `coalesce_key: "note-gesture"` is gone; the retained `NoteCommandWork` carries the emit's `transaction`.

### 2.3 Hosts

- React `🖋️InkCanvasHost`: `stream` / `commit` / one-shot; block moves send the drag record (local draft still applies
  the per-block events); a live frame publishes every event since the previous frame (eraser removals were dropped
  before) and the commit carries the pending frame; pointer cancel and a pinch abort a streamed gesture
  (`phase:"abort", reason:"captureLost"`).
- wgpu `🎞️Scenes`: `write_ink_events_action` takes `phase: Option<&str>` and the drag record; moves no longer scan
  blocks; `ink_gesture_abort_into` publishes the abort on pointer cancel when a gesture streamed
  (`SceneSurfaceState.ink_gesture_open`); the test-only standalone reference mirrors it.
- Draw hosts are unchanged on the wire (`canvasPointer*`): the tool runs plugin-side and the release is the commit.

## 3. Changes

Aliases from §1 plus `DT` = `DR/🏅️standards/🔖️1/🪆️subsets/🔀️transform`, `NB` = the note subset `🧱️block`,
`EN` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine`, `LIB` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library`.

### 3.1 Draw

- Deleted: `DR/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🔄️fsm` (17 files, crates `semio-s-plugin-draw-fsm`,
  `semio-s-plugin-draw-fsm-macros`); their root `Cargo.toml` members and `🔒️dependencies.json` users.
- New leaves (schema-first, each with descriptor, `🧬️schema`, `🦠️mutation/{🦀️.rs,🟦️.ts}`, `🔺️diff`, `↩️inverse`, one test
  case, and a fixture quintet under `DT/🧫️fixtures/🧬️mutations/` generated by the independent Python implementation
  `🧪️w3-t-draw-selection-leaves.py`): `DT/🧬️schema/🧬️mutations/{✋️drag-layers,🧭️rotate-layers,📐️scale-layers,📍️drag-path-points}`,
  plus a TS oracle test per leaf (`🧪️tests/🔬️unit/🟦️.ts`, Ajv vs the TS parser).
- `DA/🧬️schema/🧬️mutations/🦀️.rs`: enum variants, KINDS (also the missing `set-group-isolation`), region
  `🔖️SelectionTransform` (placement walk, diff/inverse helpers, rotation/scaling matrices, labels, `drawing_inverse_rows`).
- `DA/🧬️schema/🔺️diff/🦀️.rs` (multi-entry diff builders), `DA/🧬️schema/🧮️geometry/🎛️handles/🦀️.rs` (`HandleMotion`).
- `DA/🧬️schema/🧰️owned/🦀️.rs`: the owned store applies, digests and retires the new leaves.
- Aggregate `DA/🧬️schema/🧬️mutations/{🔣️.json,🔗️.graphql,🟦️.ts}`, `DT/🔮️oracles/🔣️.json`, `DT/🧪️tests/🔀️mutate-drawing-1-transform/{🥒️.feature,🦀️.rs}`, lib wiring `DR/🦀️.rs`.
- `DR/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🦀️.rs`: the canvas tool, `DrawingTool`, `drawing_tool_emit`,
  the rewritten `DrawingSession` (press/sample/escape/cancel/finish_draft/release/prepare_grab), grab preparation.
  Its unit tests and fixtures moved to `DA/✏️editor/🧪️tests/🔬️canvas-tool/{🦀️.rs,🟦️.ts}` and
  `DA/✏️editor/🧫️fixtures/{🎯️selection,🎯️point-publication}` (child module through `#[path]`), so the projected
  command directory is exactly its `🦀️.rs`.
- `DA/✏️editor/🦀️.rs`: owner `new(utility, seed)`, dispatch over the session API, per-mutation fold declaration,
  `mutation_label`. Handlers `↔️canvas-pointer-move`, `⬆️canvas-pointer-up`, `🚪️canvas-escape`, `🖱️canvas-double-click`,
  `✅️canvas-commit-draft`, `🕹️nudge-selection` (one-shot tool transactions), `🌫️set-selected-opacity` (absolute
  leaves for the scrub machine), canvas render.
- Tests: draw editor unit tests (one gesture = one row with `TransactionRef`, cancel = zero trace, two gestures = two
  transactions, en/de row labels), gesture-owner tests, nudge tests, canvas tool tests.
- Draw package TS test list `✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript/📜️script.ts`: moved canvas test + 4 leaf tests.
- Policy: root `📜️script.ts` `toolJobDrawingGestureOperationOwnerExact` anchors (tool runner, `commit_transaction`,
  no `coalesce_key: Some(`, no `mod fsm`, and the five anchors that had drifted before this ticket) and its self-test
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts`.
- Taxonomy and library: `LIB/🔣️taxonomy.json` (2-node `draw-editor-command-bundle-v1`, `fsm` kind and its two member
  kinds removed, `🎯️point-publication`/`🎯️selection` registered as fixtures), `LIB/🧹️normalization/🟦️.ts` (loader:
  1 source-named rust node); `🧫️fixtures/🦀️rust-warnings/🔣️.json` (fsm packages); stale comments in
  `🧰️framework/🔨️modules/🔀️dispatch/🦀️.rs` and `…/🧑‍💻dev/🧪️tests/🧹️host-handle-policy/🟦️.ts`.

### 3.2 Note

- `NO/✏️editor/🎮️commands/🖊️ink-apply-events/🦀️.rs`: the ink tool, `NoteInkTool`, `NoteInkToolState`, `note_ink_dispatch`,
  `note_ink_tool_preview`, event and gesture yields; new wire fields `phase?`, `reason?`, `gestureJson?`.
- `NO/✏️editor/🪟️window/🦀️.rs` and `🧬️schema/{🔣️.json,🟦️.ts,🛰️.proto,🔗️.graphql,🦀️.rs}`: `inkTool` in the composite
  window transient (retirement, footprint).
- `NO/✏️editor/🦀️.rs`: args bridge (`phase`, `reason`, `gestureJson`), preview paint, `mutation_label`.
- `NO/✏️editor/🧵️retained/🦀️.rs`: `inkApplyEvents` one unit, nudges one unit, transaction carried, WindowTransient lane
  for `inkApplyEvents` and the 9 nudges; `✏️s/🔌️plugins/🗒️note/🧫️fixtures/🧪️action-cohort/🔣️.json` lanes to match.
- Nudges: `🕹️nudge-selection` owns the shared `nudge`; the 8 directional verbs call it (duplicated helpers removed).
- `✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📦️packages/🦀️rust/Cargo.toml`: `machine` + `semio-framework-tool-machine`.
- Tests: ink-apply-events laws (stream → one transaction and one undo; abort = zero trace; drag = one relative
  `drag-blocks` with en/de label; nudge = one transaction and interrupts an open gesture), editor wire tests, window tests.

### 3.3 Hosts and shared contracts

- React: `EN/🧱️elements/🖋️InkCanvasHost/🟦️.tsx` (stream/commit/one-shot/abort, drag record, frame accumulation),
  `…/📖️stories/🧪️.story.tsx` (story reducer applies drag records at commit).
- wgpu: `EN/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs` (`write_ink_events_action`, `ink_move_gesture`,
  `ink_gesture_abort_into`, `ink_gesture_open`), `EN/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`
  (`cancel_scene_pointer` publishes the ink abort), test-only reference `EN/🧱️elements/🎞️Scenes/🧪️tests/🧊️wgpu-standalone/🦀️.rs`.
- Shared contracts: `EN/🧫️fixtures/🎬️surface-behavior/🔣️.json` + `EN/🧬️schema/🎬️surface-behavior/🔣️.json` (note case:
  `cancelled-action-discards-draft`, stream/stream, abort on cancel, `unchanged`), `EN/🧫️fixtures/🛑️scene-pointer-cancellation`
  + schema (ink: `discard`, `publishOnCancel: ["gesture-abort"]`), `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🖋️ink-canvas-editing/🔣️.json`
  (inline edits are one-shots).
- Tests: `EN/🧪️tests/{🎬️surface-behavior,🖋️ink-canvas-clipboard,🛑️scene-pointer-cancellation}/🟦️.ts(x)`,
  `EN/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-ui-command-wiring/🦀️.rs`.
