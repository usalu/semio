# Prepared Scene And Overlay Stacking

## Source defect

`DrawList::push_scene_pass` anchors the scene to its current layer and creates the following layer. An inline overlay authored before that scene therefore belongs to the pre-scene layer. The prepared walker currently emits all six channels of that layer, including its three overlay channels, before it advances to the anchored scene pass. The command order is `main base → inline overlay → scene → following layer`.

The retained overlay route is reachable for generic overlay nodes. Its intended layer-scoped order is `main base → scene → inline overlay → following layer`: the inline overlay must cover the scene associated with its layer, while a later covering popup or glass boundary must still cover that overlay. Globally replaying every overlay after every later layer would reproduce the immediate renderer's broad overlay pass and violate React stacking.

## Fail-first contract

The language-neutral schema and fixture are:

- `🧰️framework/🔨️modules/🖱️ui/🧬️schema/🪜️prepared-scene-overlay-stacking/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🧫️fixtures/🪜️prepared-scene-overlay-stacking/🔣️.json`

The source sequence is a normal base, an inline overlay, a scene pass, and a normal covering layer. Its required prepared order is base, scene, overlay, covering layer. Four independent pixel samples prove that the covering layer remains above the inline overlay, the inline overlay remains above the scene, the scene remains above the base, and untouched base remains visible.

`an_inline_overlay_follows_its_layer_scene_and_precedes_the_following_layer` builds the real `DrawList`, completes the real `PreparedRenderJob`, reads retained command-page cursors, and compares their identities to that order. Dev-only `tiny-skia` independently paints the fixture's required order and checks the four pixel samples.

UI17 executed the exact law and recorded the intended RED: one selected test, zero passed, one failed, 664 outside the filter, 0.048 seconds of test execution, and 16.1 seconds through Nx. Production emitted the inline overlay before the scene pass.

## Repair

`PreparedRenderJob::layer_channel_cursor` now treats each layer as two bounded channel groups. The normal UI, vector, and raster group advances into every scene pass anchored to that layer. The last anchored scene resumes the same layer's overlay UI, vector, and raster group. Only that overlay group's terminal advances to the following layer. Layers without a scene still preserve normal-before-overlay order, and multiple passes remain contiguous before their layer overlay.

The source and fixture JSON parse cleanly. The Rust source parses under `rustfmt --check`; the included test file and prepared module retain unrelated existing formatting differences. A strict Ajv validation command was attempted through Nx, but Nx stopped before execution on the workspace's concurrent circular project-graph dependency, so no Ajv result is claimed from that attempt. Root owns the UI18 acceptance run.

Review found that the normal raster channel's terminal still entered overlay channel three directly. The fixture now includes a normal raster before the scene, requires `base → raster → scene → overlay → following layer`, and adds an independent raster-visible pixel sample. `next_layer_raster` now enters the anchored scene ladder for a normal raster and advances directly past overlay channel six only for an overlay raster.

UI18 completed the full UI native census with 665 of 665 tests passing, zero skipped, 2.643 seconds of test execution, and 25.5 seconds through Nx. The complete run includes the normal-raster, scene, scoped-overlay, and following-layer regression plus the adjacent glass regressions.
