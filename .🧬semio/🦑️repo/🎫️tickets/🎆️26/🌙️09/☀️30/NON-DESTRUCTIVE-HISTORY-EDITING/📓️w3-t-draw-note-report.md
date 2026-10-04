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

## Session 2 — 2026-10-01

Successor executor (S2-DRAW). Scratch: `🗑️generated/s2-draw/`. Aliases as in §1/§3; `DT` = `DR/🏅️standards/🔖️1/🪆️subsets/🔀️transform`,
`NB` = `NO/../🧱️block` (the note `🧱️block` subset).

### S2.1 Repair (rule 21)

- No half-finished edit in owned files. Note had compiled after its last edit (07:54); draw had **never been compiled** after its
  02:32–03:02 edits. First compile after the session-2 peer breaks cleared (kernel paged-ledger / per-viewer head, base64,
  stdio semio drawing split): `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-draw-drawing --tests` **Finished**
  (0 errors, 41 warnings, 13:30).
- Stale test repaired: `DA/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` `dispatch_registers_semantic_descriptors` still expected 18
  kinds and the `layer` entity for every kind; now `KINDS.len()` and the `layers` / `path-points` entities of the four new leaves.
- Warnings in files this WP touched removed: unnecessary qualifications (`canvas-pointer-down/🦀️.rs:879`,
  `✏️editor/🧪️tests/🔬️canvas-tool/🦀️.rs:86,97`, `✏️editor/🧪️tests/🔬️unit/🦀️.rs` session tests), dead `transform_world_point`
  in `DA/🧬️schema/🦀️.rs` (its users were the deleted fsm gesture paths).

### S2.2 Changes

- Lint finding fixed: `DT/🧬️schema/🧬️mutations/📍️drag-path-points/🧬️schema/🔣️.json` `/targets` is an array of structured
  `{layerId, index, point}` records, so it is `role: value`; the reference (`role: target`, `ref {layer, strokes, stroke}`) stays
  on `items.layerId`.
- Note `drag-blocks` (the leaf the drag gesture and every nudge now yield):
  - label carries the offset and a singular form: "Drag 2 blocks by (30, 15)" / "2 Blöcke um (30; 15) ziehen", "Drag 1 block
    by …" / "1 Block um … ziehen" (`NB/🧬️schema/🧬️mutations/🤏️drag-blocks/🦀️.rs`, ink law and editor docstring updated);
  - schema hard bounds `ids: minItems 1, uniqueItems true` (the edit UI can no longer author an empty or repeated target set);
  - TS twin parser `NB/🧬️schema/🧬️mutations/🤏️drag-blocks/🟦️.ts` `parseDragBlocks()` (type stays in the aggregate
    `NO/🧬️schema/🧬️mutations/🟦️.ts`), oracle test `…/🤏️drag-blocks/🧪️tests/🔬️unit/🟦️.ts` (strict Ajv `semioSchemaAjvV1` vs the
    parser on the committed witness `🤏️nudges` + 12 hostile variants), wired into the note TS package `test` list
    (`NO/../../📦️packages/🟦️typescript/📜️script.ts`).
- Replay laws (part c), store level, same pattern as wfc/puzzle 3d:
  - draw `DA/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` region `⏪️TimeTravel`:
    `every_selection_leaf_edited_in_history_replays_its_downstream` (log drag → rotate → scale → drag-path-points → drag; each of the
    four relative leaves superseded in turn: `state_before` = fold of the prefix, nothing downstream; Report replay = fresh fold of
    the edited log; report never blocks; one outcome per replayed op; Overwrite finalizes exactly that state) and
    `a_drag_retargeted_onto_a_missing_layer_blocks_finalizing` (`mutation.target-missing` on the edited leaf blocks finalize).
  - note `NO/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` region `⏪️TimeTravel`: `a_block_drag_edited_in_history_replays_its_downstream`,
    `a_block_drag_retargeted_onto_a_missing_block_blocks_finalizing`, `a_block_drag_label_reads_the_count_and_offset`.
- Hygiene: the two bridge lockfiles re-resolved with `cargo metadata --offline` (`✏️s/🔌️plugins/🖍️draw/🏭️bridge/Cargo.lock`: the
  `semio-s-plugin-draw-fsm{,-macros}` entries are gone, machine/tool-machine/time-travel added; `✏️s/🔌️plugins/🗒️note/🏭️bridge/Cargo.lock`
  likewise). The `draw-fsm` rows left in `🔌️plugin/📇️registry/🧫️fixtures/📖️generated-projection.json` and
  `📚️library/🧫️fixtures/🖍️draw-source-scenario/🔣️.json` are synthetic scenarios of other owners (phantom-sub-crate laws), not stale.

### S2.3 Verification (so far)

| command | result |
|---|---|
| `bun ./📜️script.ts schema mutation-inputs --under ✏️s/🔌️plugins/🖍️draw` (cwd `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`) | 1 finding → **0** (52/52 inputs of 22 leaves) |
| `… schema mutation-payloads --under ✏️s/🔌️plugins/🖍️draw` | **0** (22/22 payloads, 22/22 leaves witnessed) |
| `… schema mutation-inputs --under ✏️s/🔌️plugins/🗒️note` | **0** (60/60 inputs of 34 leaves), also after the `drag-blocks` bounds |
| `… schema mutation-payloads --under ✏️s/🔌️plugins/🗒️note` | **0** (35/35 payloads, 34/34 leaves) after the `drag-blocks` bounds |
| draw TS package `bun ./📜️script.ts test` (`✏️s/🔌️plugins/🖍️draw/📦️packages/🟦️typescript`) | 280 pass, 2 fail — both pre-existing fill pixel-sampling cases (W3-G triage 09-30 18:55), canvas-tool + 4 leaf Ajv tests green |
| note TS package `bun ./📜️script.ts test` | 3 pass, 0 fail (incl. the new `drag-blocks` oracle) |
| `cargo check --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-draw-drawing --tests` | Finished, 0 errors |
| `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s2-draw cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-draw-drawing --lib` (13:58) | **462 passed, 13 failed** of 475; no abort. Green: both new replay laws, `dispatch_registers_semantic_descriptors`, every gesture law (`two_gestures_are_two_transactions_and_a_cancel_is_zero_trace`, `selected_group_and_layer_drag_preserves_selection_and_one_history_edit` — the 22:54 label failure is fixed by `mutation_label` —, node drag, handles, nudges). The 13 reds are triaged in S2.4 |
| same, `-j 2`, after the S2.4 fixes (23:24) | **474 passed, 1 failed** of 475 (the one red fixed in source since, see S2.4) |
| vitest React (`SEMIO_TEST_LEVEL=long`, React pkg): surface-behavior, ink-canvas-{clipboard,editing,domain-interaction}, scene-pointer-cancellation | 6 pass, 1 fail (pre-existing source scan: winit `handle_pointer_cancel(pointer)` vs expected `(pointer.id)`, not this WP); the three ink-canvas files fail to LOAD in `🔌️plugin/📇️registry/🤖️generated/🧩️plugins` (`moduleDirectoryName` reads `undefined.filter`) — generated registry output of the coordinator's rebuild chain |
| `cargo check -p semio-framework-os-renderer-wgpu --tests` | blocked at 12:50 by a peer (`🕸️dag` `DslValue::Bytes` arm); rerun pending |

### S2.4 Triage of the 13 draw reds (13:58 run) and fixes (17:00)

| cluster | tests | root cause | fix |
|---|---|---|---|
| demo example does not parse (`expected Bool, found Absent`) | `archive_load…demo_example_load…`, `renders_canvas_scene_with_segments`, `set_active_example_resolves_the_registered_catalogue`, pdf 1.4 `demo_exports_a_pdf…`, `pack_round_trips_and_agrees_with_dsl`, `dsl_round_trips_semio_example_fixture`, `dsl_rejects_invalid_stroke_caps_and_joins` | the handcrafted demo `DA/🖼️assets/🎬️demo/🗣️.dsl.semio` group lacks `isolation`; the DSL record derive has no default for a non-`Option` bool (pre-existing since `isolation` landed 09-29) | `group isolation=false` in the demo asset |
| live envelope `schema-json.missing-required-field` | `drawing_live_envelope_submit…`, `drawing_live_initializer_candidate…` | peer change (per-viewer head / branch provenance): every `Edit` now requires `line` (explicit `null` = trunk) | `"line": null` in the hand-built wire of `DA/✏️editor/🧪️tests/🔬️unit/🦀️.rs` `drawing_envelope_wire` |
| route count `31 ≠ 30` | `retained_route_dispositions_are_exact_and_exhaustive` | stale magic number (31 bounded routes since ≤ 09-29; the routes == declared commands join below it is the real law) | 31 |
| catalog `21 ≠ 22` | `kinds_match_the_enum_and_the_catalog` | session 1 added `set-group-isolation` to `KINDS` but not to the 🎨️style subset oracle catalog | `DR/…/🎨️style/🔮️oracles/🔣️.json`: oracle `immer-drawing-group-isolation-edit` (immer 10.2.0, the leaf's existing Immer twin), catalog kind + vector (`🧩️pass`), mutation manifest entry |
| owned store `mutation-overlay-destination-capacity` | `retained_blend_mutations_validate_vocabulary…` | fixture `nested_snapshot` pre-admits string destinations of the group only, the blend case edits root layer 0 ("normal" → "multiply" needs 8 B) | admit every root layer |
| `garbage` blend "applies" | `set_layer_blend_mode::…invalid_blend_edits_are_rejected…` | the test read `apply_drawing_mutation(..).is_ok()` as the refusal, but a refused leaf folds as a no-op `Ok` by contract (doc of `apply_drawing_mutation`) | the refusal is now read from the outcome (Error/Fatal message); document unchanged; the raw delta still must reject |

Reruns: 17:05 and 21:40 were SIGKILLed (exit 137, swap ≥ 94 %) while compiling dependencies; the third (`-j 2`, 22:46 → 23:24)
completed: **474 passed, 1 failed** of 475. All twelve fixed tests, both replay laws and `dispatch_registers_semantic_descriptors`
green. The remaining red, `retained_blend_mutations_validate_vocabulary_and_return_unchanged_rejections`, still hit
`mutation-overlay-destination-capacity`: the test applied `source.clone()`, and `String::clone` drops the pre-admitted capacity. Fixed
(02:50) by applying a fresh `nested_snapshot()` (deterministic ids) and comparing against `source`: WRITTEN BUT UNVERIFIED (fleet rule 26
cargo hold).

### S2.4b Policy anchor (root `📜️script.ts`, `toolJobDrawingGestureOperationOwnerExact`)

Probed on the live sources (scratch `🗑️generated/s2-draw/policy-probe*.ts`): every required owner/framework anchor present, but the
predicate was **false** because its blanket `!includes("command.dispatch(&doc")` also matched the bounded-command route
`DrawingBoundedCommandWork` (`input.command.dispatch(&doc, &cfg, &mut session)`, present since before this ticket; gesture commands are
kept off it by the `DRAWING_GESTURE_TOOL_IDS` refusal anchor). The check now excludes exactly that one bounded call and still forbids
any other generic dispatch. Live predicate **true**; self-tests (`🔌️plugin/🧪️tests/🔬️tool-job-drawing-gesture-operation-owner/🟦️.ts`)
**21/21** (valid + 20 hostile, incl. `fn dispatch() { command.dispatch(&doc); }`).

The two red draw TS cases (fill sampling `solid-alpha`, `zero-radius-gradient`) are environmental: `sharp` returns an empty pixel buffer
for the 1×1 SVG probe (`alpha  vs 0.2,0.4,0.6,0.5`). Because the draw package `test`/`publication-authority-audit` commands run that
bun list first, the publication-authority audit half did not execute in S2.3 — it needs `sharp`/librsvg fixed first (not this WP).

### S2.5 Open

- After the hold: `cargo test … -p semio-s-artifact-draw-drawing --lib -- retained_blend_mutations` (the one fix above), note lib tests
  (`semio-s-artifact-note-note --lib`: ink laws, the three new `⏪️TimeTravel` laws, the label law), `cargo check -p
  semio-framework-os-renderer-wgpu --tests` (wgpu ink edits never compiled), wasm32-wasip2 checks of `semio-hub-draw` / `semio-hub-note`.
- Taxonomy report for the new `NB/🧬️schema/🧬️mutations/🤏️drag-blocks/🧪️tests/🔬️unit` dir: WRITTEN BUT UNVERIFIED (17:10 the taxonomy
  was invalid — `generatorContracts["repo-entity-kinds"]` / `["schema-entity-catalog"]` inputPatterns unsorted, a peer's edit).

## Session 3 — 2026-10-02

Successor executor (S3-DRAW). Scratch: `🗑️generated/s3-draw/`. Aliases as in §1/§3 and S2.

### S3.1 Repair (rule 28)

- Files in the draw/note trees newer than the S2 section (02:43): 20, all peer refactors (dsl `RecordSpecProducer` `.ordinary`,
  `semio_s_2d` → `semio-framework-2d`, `EngineHandles` move, note sqlite native encoding) plus the S2.4 `retained_blend_mutations`
  fix (`apply(nested_snapshot(), …)`). No half-finished edit of this WP.

### S3.2 Path budget (item 4)

- Re-census with `🧪️s3-draw-path-budget-repair.py` (reuses `🧪️s2-controls-path-budget-repair.py`, ROOTS = draw + note):
  **0 dangling** `#[path]` / `include_*` / JSON `path` refs, 0 moves, 0 rewrites (the 36 of the S2 census, e.g.
  `✳️any/🧪️tests/🎨️mutate-drawing-1-any-style/🦀️.rs`, were already re-pointed at `🎨️style/🧫️fixtures/…` by 2026-10-01 20:57).
  The 11 "unresolved" rows are scanner false positives (TS `.js` specifiers resolving to `🟦️.ts`, extensionless `./…/🟦️`
  imports, one `\u`-escaped import in `✳️any/🧬️schema/🎨️fill/🌀️rule/🧪️tests/🔬️unit/🟦️.ts`); every target exists.

### S3.3 Taxonomy (item 2)

- `bun ./📜️script.ts verify taxonomy report --scope NB/🧬️schema/🧬️mutations/🤏️drag-blocks` found 2 errors this WP introduced:
  `projection-catalog-coverage` ("Physical mutation 🤏️drag-blocks does not have an exact one-to-one vector registry") on
  `🧪️tests/🔬️unit` and `🧪️tests/🤏️nudges` — the canonical mutation-case pair counts EVERY directory under `<leaf>/🧪️tests/` as a
  scenario that needs one catalog vector. The four draw leaves of session 1 (`DT/…/{✋️drag-layers,🧭️rotate-layers,📐️scale-layers,
  📍️drag-path-points}/🧪️tests/🔬️unit`) had the same finding.
- Fix (`🧪️s3-draw-leaf-oracle-tests.py --apply`): the five leaf Ajv-vs-parser oracle tests moved to the aggregate's open-pattern
  cases `DA/🧬️schema/🧬️mutations/🧪️tests/🧪️{drag-layers,rotate-layers,scale-layers,drag-path-points}/🟦️.ts` and
  `NO/🧬️schema/🧬️mutations/🧪️tests/🧪️drag-blocks/🟦️.ts` (precedent: layout `🧪️frame-selection`, clean), relative imports
  rebased, both package test lists updated.
- After: the five new `🧪️<leaf>` dirs report **clean** (0 findings); `projection-catalog-coverage` gone on all five leaves. What remains
  per leaf (`mutation-fixture-invalid … scenario root is absent` on the fixture bundle, `directory-kind-unresolved` on the vector
  scenario dir) is the repo-wide scoped-report baseline — identical on untouched reference leaves (puzzle 2d `✋️drag-selection`: 12
  such errors, note `🌫️change-grid-opacity`: 1). Pre-existing, not this WP: `DA/…/🧪️tests/🔬️kinds-catalog` kind-unresolved and the
  four older `🔬️unit` dirs under draw leaves (`🌀️set-layer-fill-rule`, `🧩️set-group-isolation`, `📝️update-text`,
  `✏️update-path-geometry`, 09-26..09-28, they also carry `🦀️.rs` mounted by `#[path]`).
- `bun test` of the five moved files: **5 pass, 0 fail** (56 expects). Note TS package `bun ./📜️script.ts test`: **3 pass, 0 fail**.

### S3.4 Setting labels (item 3)

- `change-grid-opacity` printed "Change grid opacity to Some(0.76)". The same `{:?}`-of-`Option` defect sat in eight more note leaves
  (`change-grid-{spacing,subdivisions,visible}`, `change-snap-{enabled,grid-spacing}`, `change-{pencil-width,eraser-radius}`,
  `rename-note`); all nine fixed:
  - `NO/🧬️schema/🧬️mutations/🦀️.rs` region `🔖️Helpers`: `note_label_number` (moved from `🤏️drag-blocks`, now shared) and
    `note_setting_label((en, de), Option<(en, de)>)` — "Change grid spacing to 12.5" / "Rasterabstand auf 12,5 ändern", `None` →
    "Reset grid spacing" / "Rasterabstand zurücksetzen".
  - opacity in percent ("Change grid opacity to 76%" / "Rasterdeckkraft auf 76 % ändern"); switches as verbs ("Show grid" / "Raster
    einblenden", "Enable snapping" / "Fangfunktion einschalten"); "Rename note to \"Plan\"" / "Notiz in \"Plan\" umbenennen",
    `None` → "Remove note title" / "Notiztitel entfernen".
  - Law `document_setting_labels_read_the_value_not_a_debug_option` (note mutations unit tests, region `⏪️TimeTravel`): exact en/de
    strings + no label of the 14 setting variants contains `Some(` / `None` / `true` / `false`.

### S3.5 Closure items (coordinator 12:0x, census `📓️s3-closure-census.md` §(b) S3-DRAW; item 5)

- Gestures: census and this pass agree — draw 1 tool machine (canvas tool), note 1 (ink tool), **0 unconverted gesture families**.
- §16.2 / D4: the 10 hand-labelled `Emit::commit(mutations, "label")` in draw (`🎮️commands/{🎛️edit-selection ×2, ✏️edit-path,
  📥️drop-layer-kind, 🧵️patch-layers, ⌫️delete-selection, 🎨️edit-fill, 📋️duplicate-layer, 🚚️move-layer, ➕️add-layer}/🦀️.rs`) are now
  `Emit::mutations(…)`: no English-only description, the row reads the verb's registry label, else the leaves'
  `SemanticMutation::label` (en/de). draw + note have **0** `Emit::commit(` left.
- Note retained accumulator (census M ×5, `✏️editor/🧵️retained/🦀️.rs`): the `coalesce_key` merge (adopt / compare / release / closed)
  is gone; one refusal remains — a note semantic unit carrying a coalesce key is refused (`note.retained.coalesce`). CLOSURE deletes
  that one check with the field.
- Left for CLOSURE (mechanical, die with the API): draw `✏️editor/🦀️.rs` `protocol::Edit { … coalesce_key: None }` literal, the two
  test assertions `coalesce_key.is_none()` (`🕹️nudge-selection/🧪️tests/🔬️unit`, `✏️editor/🧪️tests/🔬️unit`). V (view lane, D1):
  draw viewer camera `👁️viewer/🦀️.rs` window-config coalesce. F: draw folds through `drawing_inverse_rows` (derived from the leaf);
  window lanes `work_items: 1`.
- 19:16–19:20 S3-CLOSURE wave 5a (`Emit.coalesce_key`, `Emit::amend_config`, config-lane `AmendLast` deleted) swept the rest: the note
  refusal above, the draw viewer camera window-config coalesce, the draw `protocol::Edit` literal and both test assertions are gone —
  draw + note have **0** `coalesce_key` / `amend` references.
- `bun ./📜️script.ts schema mutation-labels --under ✏️s/🔌️plugins/{🖍️draw,🗒️note}` (G7 incl. `labelHandwritten`): **0 / 0** findings
  (draw 22/22 native labels; note 28 native + 6 forwarding of 34).

### S3.6 Reference chips and "Use selection" (coordinator N3 hook)

- Chips: the generic default (`time_travel_entity_names`: first object with that `id` → its `label`/`name`/`title`/`text`) reads
  draw layers by `base.name` (layer kinds are created named: "Rectangle", "Path", "Text", …) and note blocks by their `name` (the
  ink host names each new block, localized at creation). No raw ids; no `entity_label` override needed. Same-named entities are
  told apart by the preview highlight (G3), not by the chip.
- "Use selection": every draw layer reference (`drag-layers`, `rotate-layers`, `scale-layers`, `drag-path-points/targets/*/layerId`
  and the 19 older layer leaves) declares `{domain: "strokes", granularity: "stroke"}` = the canvas selection
  (`DRAWING_INTERACTION_DOMAIN`/`_GRANULARITY`); every note block reference (21) declares `{domain: "blocks", granularity: "block"}`
  = `NOTE_INTERACTION_BLOCKS`/`_GRANULARITY`; asset references (2) have no domain (assets are not selectable).
- Laws (region `⏪️TimeTravel`): draw `layer_references_read_their_name_and_take_the_canvas_selection` (chip of a layer named
  "Rect" reads "Rect"; every ref of the four selection leaves is the canvas domain + granularity), note
  `block_references_read_their_name_and_take_the_block_selection`. WRITTEN, compile pending (below).
- Also: draw `Cargo.toml` (peer rename 05:58 set `semio-framework-2d` `default-features = false`, dropping `booleans` + `trace` that
  `resolve_boolean_layer_segments` / `resolve_trace_layer_segments` use → 6 errors `DA/🧬️schema/🦀️.rs:1125,1182`): now
  `features = ["booleans", "trace"]`. Hub draw `Cargo.toml`: unused legacy alias `semio_s_2d` removed. Oracle catalog prose
  (`DA/🔮️oracles`, `DT/🔮️oracles`) names `semio_framework_2d`.

### S3.7 Verification (running log)

| time | command | result |
|---|---|---|
| 11:06–11:38 | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-draw cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-draw-drawing --lib` | **compile red, 6 errors** — `DA/🧬️schema/🦀️.rs:1125,1182` `semio_framework_2d::{booleans,trace}` missing (peer `default-features = false`) → fixed in draw `Cargo.toml` |
| 12:00–12:28 | same | red in peers: stdio xml/pdf `semio_framework_schema_registry` unlinked (schema split in flight) |
| 12:33 | same + `cargo check -p semio-framework-os-renderer-wgpu --tests` (12:34) | red in peer `🧬️schema/📇️registry/🦀️.rs` (duplicate `ArtifactSchemaRegistry` :304/:349, `SchemaDescriptorRegistryError` :400/:511) — reported to coordinator |
| 12:41–12:46 | same | red in peer `🛂️manifest/🦀️.rs:1266-1269` (`semio_framework_schema::with_schema_export_registry` etc. moved to the registry crate) — reported |
| 12:2x | `bun test` the five moved leaf oracle tests | **5 pass, 0 fail** |
| 12:2x | note TS package `bun ./📜️script.ts test` | **3 pass, 0 fail** |
| 12:3x | draw TS package `bun ./📜️script.ts test` | **280 pass, 2 fail** of 283 (32 files) — the 2 pre-existing `sharp` fill-sampling cases (S2.4b) |
| 12:3x | `schema mutation-inputs` / `mutation-payloads` `--under ✏️s/🔌️plugins/🖍️draw` | **0 / 0** (52/52 inputs, 22/22 leaves witnessed) |
| 12:3x | same `--under ✏️s/🔌️plugins/🗒️note` | **0 / 0** (60/60 inputs, 35/35 payloads, 34/34 leaves) |
| 13:05–13:09 | draw lib test (same command) | red in peer `🔌️plugin/🕹️interaction/🧬️mutations/🦀️.rs:15` (`semio_framework_schema_state` not linked into `semio-framework-plugin`) — schema split (S3-INFRA) |
| 18:40 (after the 17:00 reboot) | `bun test` the five moved leaf oracle tests | **5 pass, 0 fail** (56 expects); all edits of S3.3–S3.6 intact on disk |
| 18:4x | `verify taxonomy report --scope` each of the five `🧪️<leaf>` dirs | **5 × clean=true, 0 errors** |
| 18:51–19:18 | draw lib test (same command), 3 runs | killed twice (SIGTERM 19:01, SIGKILL 19:14 — coordinator flock-cycle kill, rule 33), then red in peers: os-kernel `🔨️modules/🚪️io/🦀️.rs:7` `dsl::Diagnostic` unresolved, `📡️spr/🧵️channel/🦀️.rs:216-248` `crate::Fault*` missing (peer DSL extraction, S3-INFRA watching) |
| 10-03 05:48–06:17 | draw lib test after "TREE GREEN (core)" | stuck in a ✏️s flock cycle (0.7 s CPU in 28 min, ~10 idle cargos fleet-wide) → killed my own cargo only |
| 10-03 06:18–06:21 | draw lib test (re-run) | red in peers: `semio-s-artifact-stdio-zip` 52 errors (`🎒️zip/…/📸️snapshot/🪶️sqlite/🦀️.rs:11-71`, `🚦️native/🦀️.rs:41,45`) and `semio-s-artifact-stdio-svg` 6 errors (`🎨️svg/…/🔰️basic/🧬️schema/🦀️.rs:197-201`): no `From<ValueError>` for `String`/`IoError` (DSL/value extraction) — reported |
| 10-03 06:21–06:36 | `CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=…/target-nde-s3-draw cargo test --manifest-path ✏️s/Cargo.toml -p semio-s-artifact-note-note --lib` | red in framework peer `semio-framework-artifact-workflow-workflow` (206 errors, `OS/🔁️workflow/🗿️artifacts/🔁️workflow/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs`, same `ValueError` class) — reported |
| 10-03 06:37 | `cargo check -p semio-framework-os-renderer-wgpu --tests` | red in core again: `semio-framework-os-kernel` 52 errors (`OS/🚪️io/🦀️.rs` ×36 `IoError` field/`From<String>`, `OS/🏪️store/🦀️.rs` ×12, `📜️space-history/…/🪶️sqlite` ×4) — reported; waiting for TREE GREEN |
| 10-03 06:57 | `cargo check -p semio-framework-os-kernel --lib` (one cheap check) | kernel converging: 52 → 11 errors (peer extraction still active) |

### S3.8 Open items and coordinator actions (as of 10-03 10:45)

- State 10-03 10:45: no edit in flight; every source item of this session is on disk (S3.2–S3.6). OWED, blocked by the peer
  DSL/value extraction (kernel, workflow, stdio zip/svg; coordinator "TREE GREEN" pending): draw `--lib` tests (incl. `retained_blend_mutations`,
  both `⏪️TimeTravel` laws, the new `layer_references_read_their_name_and_take_the_canvas_selection`, the 9 `Emit::mutations` handlers);
  note `--lib` tests (ink laws, three `⏪️TimeTravel` laws, label law, `document_setting_labels_read_the_value_not_a_debug_option`,
  `block_references_read_their_name_and_take_the_block_selection`); `cargo check -p semio-framework-os-renderer-wgpu --tests` (also
  waits on S3-W2C's locale wave); `cargo check -p semio-hub-draw -p semio-hub-note --target wasm32-wasip2`. All S3 Rust edits are
  WRITTEN BUT UNVERIFIED until then.
- Coordinator actions: re-activation of draw (React 6064 / wgpu 6164) and note after green — the component carries the label,
  `Emit::mutations` and `Cargo.toml` feature changes; no descriptor, launch or central schema regeneration needed (no action, command,
  schema or fixture change this session).
- Not this WP (pre-existing, reported): draw TS `sharp` fill-sampling 2 reds; taxonomy findings of the four older draw leaf `🔬️unit`
  dirs and `DA/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog`.
- Scratch outputs: `🗑️generated/s3-draw/` (left for the coordinator; ticket input scripts `🧪️s3-draw-path-budget-repair.py`,
  `🧪️s3-draw-leaf-oracle-tests.py` stay).


## Session 4 — 2026-10-04

Continued by S4-TOOLS-A (draw + note + layout + fem/lowpoly/shooting) in `📓️s4-tools-a-report.md` § Session 4.
