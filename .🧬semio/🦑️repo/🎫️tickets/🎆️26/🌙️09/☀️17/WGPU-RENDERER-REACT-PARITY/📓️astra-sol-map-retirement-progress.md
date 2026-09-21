# Map Retirement Progress

## Fail-first evidence

Renderer Native135 executed `engine_canvas::engine_surface_attach_tests::retained_map_key_replacement_and_removal_retire_the_old_gesture_without_input` and failed at the strict assertion that the bounded retirement lane clears the old keyed Map gesture. The receipt is `🗑️generated/astra-runtime/renderer-native135-full/failures.json`.

The retired component was already absent from live hit routing. Its `UiRetiredComponentScene` remained at the UI queue head because `Ui::step_document_reconcile` stops while that queue is nonempty, while the Interpreter formerly forwarded the head only after the frame reached its terminal phase. The frame therefore could not reach the only code that made reconcile eligible to continue.

## Repair

`Interpreter::render_ui_document_step` now forwards at most one exact retired component immediately after advancing an existing scene-surface retirement and before document ingress or reconcile.

The handoff:

1. clones only the current UI retirement head under the UI borrow;
2. retains the head when the sole bounded scene-retirement owner cannot admit it;
3. calls `Scenes::retire_scene_identity` outside the UI borrow;
4. acknowledges the same exact UI head after admission; and
5. returns the frame opportunity after one handoff.

A stale head with no matching scene state is still acknowledged, matching the prior terminal-frame behavior. Capacity refusal is the only outcome that leaves it queued. The old post-frame forwarding block was removed, so ingress and reconcile cannot recreate the dependency cycle.

## Validation boundary

`rustfmt --edition 2021 --check` passes for the edited Interpreter source. Root owns the native Cargo lane and will rerun the existing exact filter in Native136.
# Native136 follow-up

Native136 still reached the strict old-gesture assertion after the pre-reconcile handoff. The remaining production seam was that `retire_scene_identity` moved the whole state into a later bounded close but never invoked the existing exact `retire_tiled_map_scene_identity` path. It now clears only the matching Map interaction/hover owner before moving the state, while the same bounded external state and engine-token close continues. The no-synthetic-input and exact old-token assertions remain unchanged. Native138 verification is pending.
