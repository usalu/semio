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

