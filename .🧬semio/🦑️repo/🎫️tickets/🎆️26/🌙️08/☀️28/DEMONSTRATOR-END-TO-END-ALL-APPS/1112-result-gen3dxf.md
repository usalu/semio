# gen3dxf (11/12) generation3d reds on the hub component walk

## Verified
- W-gen3dxf-2 (hub wasm): semio-s-artifact-procedural-3d-transforms rc0 (37 -> 0).
- W-gen3dxf-5 (hub wasm): semio-s-artifact-procedural-3d-io + -preview-eval rc0.

## Transforms
- Reuses `semio_plugin::MediaConstruction` through crate-local `📥️source/🏗️birth` (`Birth`): unit steps settle through the frame, owned results via `into_slot`, close via `close_step(cx) -> Result<bool,_>` (birth first, then owners through `close_original_effect_step`). `GumballGestures::close_original(body, effect)`; `retire_cold` is cold `OrderedMap::retire_cold`.

## Ruling: preview mesh retention threads the real effect
- `evaluate_tick`, `preview_tessellate_invocations`, `retain_preview_meshes`, viewer `generation3d_view_continue_inline` and editor `flow_eval_tick::{evaluate,continue_inline}` take `&mut RetainedAllocationEffect` (other consumers read `effect.grant()`).
- Both turn owners (editor `Generation3dFlowEvalWindowWork::step`, viewer `Generation3dViewFlowEvalWindowWork::step`) open it with `cx.with_original_effect`, charge `cx.consume_work(performed)`, return a Progress step when the effect is not admitted.
- Editor `flow_eval_tick::handle` (no turn) now refuses `generation3d-flow-eval-tick-requires-retained-route`.

## Other fixes
- 3d io envelope RetireOwned (tuple seed); preview-eval tessellation close effect-shaped; tool-run unused closure removed.
- viewer set-contributions: register + invalidate(effect) like the editor.
- `flow_neuron_kind_info_map()` is a SharedOwner alias: `.get().get(kind)` (set-widget-input, panels inspection, edit-mesh-selection, add-widget).
- `terminal_frame_bound!` removed from 8 viewer/editor works (they already declare `terminal_frame_release_bytes`).
- viewer retained-view-commands close ladder effect-shaped; session `close_step(effect)` in both `maintenance_step`s through `close_original_retained_effect_step` + lifecycle loan.
- Transient preparation (viewer + editor): `bounded_transient_preparation_factory` needs `SnapshotProjection<Output=P>`, derive emits `Box<Self>`; used `ArtifactEphemeralTransferPreparationFactory` with footprint/transfer fns (same as gen2d play-app-editor).

## Later steps
- Editor crate: document-io text export ported (RetireOwned derive, tuple pause errors, retirement and RetainedTextEnvelope steps inside `cx.with_original_effect`, close via `close_original_effect_step`), artifact-store-preparation (advance/recovery stubs/effect close, copied from gen2d), command-work and gumball-job closes effect-shaped.
- `🌎️hub/🧩️compositions/🌀️procedural` Cargo.toml: added `semio-framework-value` (LOCK NEEDED logged; chk passed without a manual lock).

## Verified
- W-gen3dxf-12: `-p semio-hub-procedural-component` hub wasm rc 0, 0 errors.

## Not verified
- `--tests` / tst.sh never run; crate test close loops rewritten but uncompiled.
- Runtime: preview mesh retention runs in the real effect; `flow_eval_tick::handle` fallback refuses by design.
- Other lanes' reds not examined (hub is green for procedural).
