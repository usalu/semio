# Scene Cancellation Ownership Audit

Read-only source audit on 2026-09-20. No build, test, activation, or production edit was run.

## Corrected Canvas/Ink boundary

**Proven — prior Canvas2d coverage statement was incorrect.** Shell cancellation calls `scenes::cancel_canvas_interactions`, which only drains `CANVAS_GESTURE` and `CANVAS_CATALOGUE_HOVER`: [Scenes WGPU](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs) lines 1335–1359, 1676–1699, and 1736–1739. Ink is separate state and is not named by that route.

## Proven gaps

### Ink: cancel neither reaches the retained job nor clears its state

- `InkInteractionEvent` accepts only Down, Move, and Up at [Scenes WGPU](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs) lines 6358–6362. Its normal Up emits `inkApplyEvents` at phase `commit` for move, resize, stroke, and eraser, or commits marquee selection (6744–6757). A cancel cannot be translated to Up without manufacturing exactly that terminal.
- The existing noncommitting state primitive is `clear_ink_pointer_state(surface_id)`, which clears `drag`, `pointer_was_down`, and `ink_marquee_points` (6111–6117). It is private and is called from normal Up, not from `ShellState::handle_pointer_cancel` (Shell lines 12293–12311).
- This primitive deliberately does **not** clear `ink_overrides`. Pencil Down seeds an override and a `begin` action (6853–6864); stroke Move updates the override after a `live` action (6894–6909); paint reads those overrides (7322–7326). Whether cancellation should discard that optimistic stroke requires an Ink action-phase contract: the React host has no `onPointerCancel`; it deliberately maps `onPointerLeave` to its normal committing `handlePointerUp` ([InkCanvasHost](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖋️InkCanvasHost/🟦️.tsx) lines 1252–1299 and 1537–1541). Do not silently clear overrides until that semantics is chosen.
- The staged owner is an additional necessary target. `SceneIntentQueue` retains `InkInteractionJob` (Interpreter lines 305–325); it creates and later steps it independently (1433–1474). `UiEvent::PointerCancel` is discarded by `apply_scene_ui_command` because only Down/Move/Up/Scroll are converted (1014–1021). Thus an already queued Ink job can still publish after renderer cancellation. Marking a queue record retiring only releases job buffers (393–404); it does not call the state primitive.

**Required repair shape:** add a typed cancel path at the Interpreter queue boundary for the captured window/surface, retire matching queued and in-progress Ink intent(s), then call an Ink-owned cancellation function. That function must state explicitly whether `ink_overrides` are retained or reverted. Do not route cancel through `PointerUp`.

**Required tests:** extend [wgpu-ink-canvas](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-ink-canvas/🦀️.rs) with a started marquee and a started stroke; assert no commit/selection action, cleared pointer/marquee state, and a subsequent press is accepted. Add an Interpreter queue law with an Ink job paused before its Publish phase, then cancel; assert it cannot emit any later `inkApplyEvents` or `setSelection`. The expected override assertion must follow the chosen phase contract.

Confidence: high for missing route and states; medium for override disposition because React supplies no pointer-cancel oracle.

### NodeGraph: no noncommitting terminal is exposed

- NodeGraph is a bespoke route and bypasses the Interpreter queue ([Interpreter](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs) lines 1002–1008). The renderer directly sends Down and Up to the graph host (Renderer lines 15600–15612 and 15649–15664), but `handle_pointer_cancel` does neither (15502–15510).
- The only EngineCanvas terminals are `node_graph_pointer_down_into`, `_move_into`, and `_up_into` ([EngineCanvas WGPU](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs) lines 3351–3434). Its lower dispatch maps both `DagPointerPhase::Up` and `Leave` to `pointer_up_screen` for Flow and Dag (3599–3613). Neither represents a noncommitting cancel.
- React's direct graph canvas also has only Down/Move/Up and a hover-only Leave; Up calls `session.pointerUpScreen` then commits the fixture ([NodeGraph](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx) lines 981–1017). It has no pointer-cancel handler, so React does not provide a cancellation terminal to reuse.

**Required repair shape:** create a distinct cancel operation in both Flow and Dag host APIs, plumb it through `NodeGraphEngine` and an EngineCanvas `node_graph_pointer_cancel_into`; renderer cancellation must invoke it for the graph surface that owns the sequence. Reusing Up/Leave is invalid because their current implementation is committing.

**Required tests:** add a cancellation law beside [wgpu-node-graph](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs): Down + Move + Cancel leaves no edit/selection publication and the next Down starts a new gesture. Add the corresponding Flow/Dag host-level test that pending wire/node/slider drag state is cleared without its Up output.

Confidence: high.

### Board: releasing the action claim is not a gesture cancel

- The only general EngineCanvas helper, `release_board_pointer_claim`, clears `board_pointer_controller_id` and releases the action claim ([EngineCanvas WGPU](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs) lines 4833–4843). It does not touch the BoardHost interaction. It is currently reached only after the frame authority reports `Cancelled` or `Fault` (Renderer lines 12939–12956), not from pointer cancellation.
- The BoardHost has `cancel_area_select`, but it is deliberately limited: it aborts region, transform, and selection interactions; for every other interaction it restores the captured `other` state and returns false ([BoardHost](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs) lines 12791–12831). It therefore cannot cancel node drag or link drag.
- Normal Board Up performs commits such as region paint, transform, link creation, and `nodeDragEnd` (12653–12768), so it is not a cancellation substitute. A Board pointer authority already has a cancellation mechanism through its `StepContext` before phase zero (12084–12117), but renderer pointer cancel does not signal it. `close_pointer_authority_step` is lifecycle retirement, not an input gesture API (12155–12177).
- React Board's `onPointerCancel` removes the pointer from the gesture/pinch set and releases DOM capture only; it does not call `session.pointerUpScreen` or flush board events ([Board2dHost](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖥️Board2dHost/🟦️.tsx) lines 1306–1310).

**Required repair shape:** the Board domain needs an owned all-gesture cancel terminal that resets or restores node/link/brush/region/transform/selection gesture state and cancels queued/pending pointer authority, followed by `release_board_pointer_claim`. `cancel_area_select` may be one internal branch but is insufficient as the public cancellation primitive.

**Required tests:** extend the Board host cancellation law already present at [BoardHost tests](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs) lines 209–218 to cover a live node drag and link drag via the public cancel terminal; assert original node geometry/no edge/no `nodeDragEnd`, terminal authority, and no claim. Add an EngineCanvas/renderer cancellation test proving `applyBoardEvents` is absent and a new press succeeds.

Confidence: high for the route and limited primitive; high for React behavior.

### TiledMap: React treats cancel as an Up; WGPU leaves the gesture live

- React TiledMap's `onPointerCancel` explicitly calls `session.pointerUpScreen`, clears both pressed flags and marquee, ends the continuous pan, releases capture, and mirrors the camera ([TiledMapHost](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧭️TiledMapHost/🟦️.tsx) lines 1183–1193).
- WGPU marks primary marquee and middle pan in `SceneDragMode` (Scenes lines 7509–7539); `tiled_map_drag_active` stays true while either remains (7659–7661). It clears both only on normal `tiled_map_pointer_up_into` (7591–7653). Renderer cancellation calls no tiled-map function (Renderer lines 15502–15510).

**Required repair shape:** call the existing map Up terminal for the active map with the cancellation point or retained last pointer point, then clear the local map gesture state as React does. This is a reference-defined committing terminal, unlike Ink/Graph/Board.

**Required test:** add a cancel after middle-pan and a cancel after primary marquee to the map parity fixture/law; assert no stuck `tiled_map_drag_active`, the next press works, and the camera/selection output matches the React cancellation trace.

Confidence: high.

### Paint2d: WGPU retains marquee or navigator pan beyond a cancel

- WGPU stores per-surface `Paint2dMarquee { tracking, active, points, pan_last }` in `PAINT2D_MARQUEES` ([EngineCanvas WGPU](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs) lines 5359–5385). Down sets it, and normal Up resets it (5457–5463 and 5467–5489). There is no Paint2d cancellation entry point and Shell cancellation does not call one.
- React also has no `onPointerCancel`; its overlay only routes `onPointerLeave` to hover cleanup ([Paint2dHost](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🖌️Paint2dHost/🟦️.tsx) lines 594–602). Thus retaining/reverting an active brush or marquee is not determined by the React host.

**Required repair shape:** add a Paint2d-owned state reset for `PAINT2D_MARQUEES`; decide RasterHost brush cancellation separately before calling a normal Up because Up commits selection/brush work. Add a producer cancellation oracle if commit-versus-revert is meant to be parity-defined.

Confidence: high for retained WGPU state; medium for final semantics.

### TextEditor: drag selection has no cancel terminal

- WGPU forwards normal Down/Move/Up through the staged Interpreter path (Interpreter lines 1328–1355), but its queue accepts no cancel event (1014–1021) and Shell cancellation does not target the focused editor.
- EditorHost only drops `drag_selecting` on normal `pointer_up_screen` ([EditorHost](../../../../../../🧰️framework/🔨️modules/✍️editor/🦀️.rs) lines 823–827). The React host likewise has an Up handler but no cancel handler ([TextEditor](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🟦️.tsx) lines 383–390).

**Required repair shape:** add an EditorHost cancel-selection primitive that ends `drag_selecting` without an Up selection side effect, and invoke it when cancelling matching queued editor pointer work. No React cancel policy exists to establish whether the current selection should be retained.

Confidence: high for missing route; medium for end-selection policy.

## Ingress residues

The browser wire carries `pointercancel` and browser boot listens for it ([browser transport](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts) lines 138–139 and 217–219; [browser boot](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts) lines 203–206). That boot source has no `lostpointercapture` listener. The native dispatch maps received `PointerCancel` to the renderer (Winit app lines 355–358), but a source scan found no `WindowEvent::Focused(false)` cancellation route. These are ingress coverage gaps distinct from the host-owner gaps above.

Confidence: high for absence in the scanned source; no runtime claim.
