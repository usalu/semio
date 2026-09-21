# Per-Pointer Retained UI and Canvas Cancellation

## Observed defect

Renderer Native83 completed the two actual `AppInteractionState` Canvas laws under Nextest run `6209827b-6e4d-4d7a-9983-ea6eb68ec387`: one passed, one failed, 1,279 were outside the filter, and the selected tests ran in 63 ms. The ordinary Canvas down/outside-up path passed, disproving the suspected generic App Canvas dispatch gap. `renderer_foreign_pointer_cancel_preserves_the_retained_canvas_sequence` failed because P78 cancellation published P77's terminal Canvas action before P77 released. Generic scene routing therefore remains unchanged.

Renderer Native84 later ran 1,281 tests: 1,275 passed, six failed, and zero were skipped in 35.432 seconds. The foreign-cancel law remained RED. This is the actual production failure authorizing the pointer-identity repair.

The independent Chromium pointer oracle in the checkpoint report passed 11/11 tests with 74 assertions in 2.04 seconds. It records the per-pointer `[290, 180]` movement across foreign touch release/cancel. That receipt establishes the browser's pointer-addressed capture and coordinate behavior; it is not presented as a Canvas component lifecycle receipt.

## Contract

Retained UI capture remains a single admitted UI drag, but its owner is an exact pointer ID. A foreign pointer move, release, or cancel cannot mutate or release that capture. Global Escape and window retirement may still retire the active owner through an explicit bounded lifecycle path.

Canvas supports up to the existing fixed sixteen-pointer authority. Each active slot owns its pointer ID, window, document generation, surface, controller, bounds, viewport, and last pointer coordinates. A pointer can replace only its own slot. Independent pointers on independent Canvas documents proceed independently. Move deltas come from each slot's last coordinates rather than surface-global edge state.

Canvas document gestures keep their terminal `canvasPointerUp { cancelled: true }` on owner cancellation. Middle-button and primary-plus-transform Canvas pan cancellation is local: it removes the exact pointer slot and local drag state without publishing a document action. That exact distinction has a test registered and awaits a Native85 runtime receipt before the production pan-cancel branch changes.

Passive scene-list transfer remains a single transfer session, stamped with its pointer ID. A foreign pointer cannot update, drop, or cancel it. Chrome drag/resize/slider state is similarly stamped; foreign pointer events return without clearing the current owner's input state. Unknown cancellation is a no-op.

## Production seams

- UI WGPU `CaptureState` records `(pointer_id, node_id, capture_kind)`. `EventRouter::dispatch_pointer` rejects foreign pointer events before routing or release. Existing general dispatch retains its established pointer-one entry; renderer pointer dispatch uses the required first-party pointer-aware entry.
- UI engine exposes exact `dispatch_pointer_event`, `window_with_pointer_capture(pointer_id)`, and `any_pointer_capture()` queries.
- Interpreter propagates the already-required `SceneInteractionIntent.pointer_id` into UI dispatch, Canvas down/move, and passive transfer operations. Missing pointer identity refuses the operation rather than falling back to a shared owner.
- Scenes owns a heap-first fixed `CanvasGestureSlots` table using the existing capacity authority. Exact pointer cancellation and stale lifecycle draining remove only matching slots. Canvas motion derives its delta from the slot.
- Shell resolves retained capture, Canvas, passive transfer, and chrome cancellation for the cancelling pointer and clears `InputState` only if an exact owner was actually retired.

## Registered laws and gates

Renderer filters:

```text
renderer_foreign_pointer_cancel_preserves_the_retained_canvas_sequence
foreign_pointer_cancel_preserves_the_canvas_gesture_owner
two_canvas_documents_keep_independent_pointer_gestures
canvas2d_middle_pan_cancellation_retires_locally_without_a_document_action
```

UI filter:

```text
foreign_pointer_terminal_cannot_release_retained_capture
```

The direct Canvas laws prove that foreign cancellation preserves the owner, that P77 and P78 can own two Canvas documents independently, and that cancelling P78 does not finish P77. The UI law proves P78 release/cancel cannot clear P77 retained capture, while P77 cancellation releases it. Root owns Cargo/Nextest execution; no Cargo command was run from this task.

Before Native85, `rustfmt --edition 2021 --emit stdout` parsed the UI events, UI engine, Interpreter, Scenes, and Shell source, `git diff --check` passed, and a changed-signature call-site sweep found no old Canvas or passive-transfer calls. These are source checks only. Native85 is the first compile/runtime gate for the integrated pointer packet.

The prior retirement-only `UiSurfaceRegistry` measurement was 161,944 bytes. Adding the capture pointer ID changes the `CaptureState` representation, so a fresh exact UI inventory measurement is required; the report does not claim the retirement-only number as the current final census.

## Native86 follow-up

Renderer Native86 ran 1,287 tests: 1,281 passed, six failed, zero were skipped, and the selected execution finished in 31.862 seconds under Nextest run `5258bb55-e9a9-4ae7-a4a6-3f8ab8415da6`.

The new middle-pan law produced the expected RED: cancellation still published document semantics for a local camera pan. Production now stamps each Canvas slot with `CanvasGestureKind::{Document, Pan}`. Exact pan cancellation removes its slot and local drag state without an action; exact document cancellation retains its cancelled terminal action. Move and release consult the slot kind rather than the surface-global drag flag, and a remaining pan slot on the same exact scene keeps the aggregate local drag active.

Two older Interpreter fixture laws also failed because their direct `apply_ui_commands` calls supplied no pointer identity after pointer identity became required for Canvas admission. They now pass `Some(ui_render::PointerId(1))`, matching the mounted input boundary rather than restoring a shared fallback:

```text
canvas2d_pointer_payload_is_react_shaped_screen_logical_with_a_world_lane
scene_command_dispatches_a_canvas2d_pointer_down_action
```

`rustfmt --edition 2021 --emit stdout` parsed both changed Rust files and the scoped diff check passed. Root's Native87 full renderer run passed the middle-pan law and both corrected Interpreter fixture laws. The remaining secondary-button failure belongs to the separate Canvas pan packet; it does not invalidate these three receipts.
