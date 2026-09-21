# Prepared Scene And UI Stacking

## Confirmed defect

`DrawList::push_scene_pass` records the active layer and two channel watermarks, but leaves later UI in that same layer. `PreparedRenderJob::next_draw_usage` walks every layer and only then starts scene passes. The real prepared command pages therefore flatten `shell base → Actions pane → World` when authored order was `shell base → World → Actions pane`.

## Fail-first contract

The schema-first neutral contract is `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🌌️prepared-scene-ui-stacking/🔣️.json`, with its fixture at `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🌌️prepared-scene-ui-stacking/🔣️.json`.

The native law `a_scene_pass_is_prepared_between_the_ui_scalars_authored_around_it` builds the real `DrawList`, completes a real `PreparedRenderJob`, and reads the exact retained draw cursors from the published command pages. It requires `shell-base → world → actions-pane`. The same authored fixture is painted through dev-only `tiny-skia`; the independent pixel samples require the opaque Actions pane over World, World outside the pane, and Shell outside the scene viewport.

Production remains unchanged until the native law records the expected RED.

UI14 stopped before execution because the first law draft tried to use a nonexistent `ScenePass3d.background` field. The corrected law identifies the pass by its unique authored viewport; the fixture color remains the independent raster oracle's scene color. No behavioral evidence is claimed from that compile-only run (`🗑️generated/astra-runtime/ui14-scene-stacking-red/run.log`).

The corrected UI14 rerun recorded the intended behavioral RED: 1 selected, 0 passed, 1 failed, 665 outside the filter, 0.043 seconds of test execution and 18 seconds through Nx. The real prepared order was `shell-base → actions-pane → world`; the fixture requires `shell-base → world → actions-pane`. The tiny-skia samples completed before that order assertion. This authorizes the production repair.

## Intended repair boundary

`push_scene_pass` will terminate the active layer by appending one following layer that preserves the current scissor, silhouette clip, and glass foreground owner. The scene pass stays anchored to the terminated layer. The prepared walker will emit each layer's six channels, then every scene pass anchored to that layer, then the following layer. This is a single forward walk: no per-scalar scan and no second global scene ladder.

The implemented producer charges the scene pass and following layer atomically before changing either collection. The prepared walker uses the same sorted `partition_point` boundary lookup as the adjacent glass stream, and pass completion advances directly to the next same-layer pass or the following layer. The existing immediate-renderer watermark law now also proves that a post-pass vector lands in the following layer while the pass retains the complete preceding watermark.

Root owns the adjacent glass boundary stream. Its `GlassRegion.layer_index` transition shares `LayerHeader`, while scene completion remains owned by this packet.

## Transfer audit

The WGPU `DrawList` is painted in place by `Ui::frame_into_step`, or moved and swapped whole into its window and prepared packet owners. No production path concatenates `layers`, `scene_passes`, or `glass_regions`, so their indices require no transfer-time rebasing. `DrawList::clear` resets all three collections together. The similarly named `ui/render/Scene::finish` owns a separate render model and already remaps its own scene anchors; it does not transfer this WGPU `DrawList`.

## Green receipt

UI16 completed the full UI native census with 664 of 664 tests passing, zero skipped, 1.777 seconds of test execution, and 17 seconds through Nx. Both the scene/UI authored-order regression and the adjacent glass/foreground stacking regression passed in that complete run.
