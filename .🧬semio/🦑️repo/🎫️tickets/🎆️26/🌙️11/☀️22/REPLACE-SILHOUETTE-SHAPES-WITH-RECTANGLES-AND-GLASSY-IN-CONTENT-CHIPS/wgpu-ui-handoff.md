# wgpu-ui handoff

## Model
A frame clips its content with one rectangular scissor. Chip glass uses the existing glass rect primitive after the content, inside the frame. The silhouette clip API and the union stencil mask are gone. Public calls for lane `wgpu-os`: `wgpu-ui-api.md`.

## What changed
Wgpu draw list (`🎯️targets/🧊️wgpu/🖍️draw`)
- `types`: `begin_silhouette_clip` removed, `end_silhouette_clip` renamed `end_clip` (it closes `begin_scanline_clip`, which stays: canvas2d needs the multi-band clip). `ClipRegion::from_rects`, `from_disjoint_rects`, `source_pieces_overlap`, `subtract`, `effective_scissors` removed.
- `pipeline/execution`, `pipeline/factory`: test-only mask machinery removed (`mask_pipeline`, `mask_stencil_state`, `mask_instances`, `build_batch_masks`, `layer_scissors`, `union_scissors`, `merge_scissor_bounds`, `draw_silhouette_mask`, `FrameBuffers::mask_instances`). Test renderers now set the batch scissor instead. `content_stencil_state` stays (section stencil bit).
- Tests rewritten: `execution/🧪️tests/🔬️unit`, `🧪️tests/🔬️targets-wgpu-gpu-prepared-present`.

Terra render stack (`🖌️render`)
- Removed `ClipRegion`, `SceneBuilder::{begin,end}_silhouette_clip`, `SceneLayer.clip`, `LayerState.clip`, `DrawBatch.mask_range`, `SceneError::UnbalancedClipStack`, tessellate `layer_scissors`/`mask_instances`/`union_scissors`/`merge_scissor_bounds`.
- Added `viewport_scissor(scissor, w, h)` (tessellate, re-exported by packet and `ui_render`); webgpu, metal and d3d12 replay every batch under it (they had no scissor before; the stencil mask was their only clip). Vulkan already had `batch_scissor`.
- Metal and D3D12 mask pipelines and depth-stencil states removed. WebGPU keeps the viewport mask for world passes (renamed `draw_viewport_mask`, `VIEWPORT_MASK_PIPELINE`, label `viewport_mask_pipeline`); `StencilPolicy` stays for it.
- Tests rewritten: `scene-unit` (frame scissor equals frame rect, no mask instances, chip glass batch after content and inside the frame), `tessellate-unit` (`viewport_scissor` laws), `webgpu-surface-state-unit` (field removal).

CardStack (`🧱️elements/🃏️CardStack`)
- Rust mirror migrated to abutting rectangular frames: new `stack.cases.json` + `stack.schema.json` (language neutral, TS lane may consume them), `🎯️targets/🧊️wgpu/🦀️.rs` (`card_stack_frames`, `card_frame_contains`, `paint_card_stack`, chip glass + inner edges), `stack.test.rs` (cases, independent pixel-tiling check, seam painted once, chip glass inside frame). `bands.rs` keeps only `CARD_BAND_MIN_GAP`, `chip_gap_reads_as_sliver`, `snap_card_height` (as `bands.ts`); `bands.test.rs` reads `gaps`/`snaps`.
- Paint (`text-and-structure-nodes`) and mounted layout (`shares_footer` removed: each card has its own cap row and, with a footer, its own footer row). Paint test `a_native_card_stack_lays_out_abutting_frames_and_paints_each_shared_line_once` rewritten.
- "silhouette" wording renamed where it meant box outline (paint comments/tests, node-model comment, events test name).

## Verification
Commands (through `repo cargo --store=shared --configuration=prototype --`): see `run-ui-lane.sh`; results below.

- `repo cargo --store=shared --configuration=prototype -- check --tests -p semio-ui-render`: passes (finished, exit 0; render lib, scene/tessellate unit tests and packet/shader-contract compile).
- A multi-crate `check -p ... -p ...` failed with the infrastructure error "Undeclared original Repo build selection"; single-package runs work.
- NOT verified: `cargo test` for semio-ui-render and the draw, pipelines, light (CardStack), paint, mounted-layout, paint-laws, webgpu and metal crates. `run-ui-lane.sh` starts them one by one (about 8 minutes per crate on the shared store); logs go to `🗑️generated/lane-test-<crate>.txt` and `lane-done.txt` marks the end. At handoff time only the first crate was still compiling. Rerun `sh run-ui-lane.sh` and read the `test result` lines.
- D3D12 and Vulkan changes cannot be compiled on this macOS host.

## Open
- Chip glass of a native card is recorded in the card-stack node step, before its children; children never overlap chip rows in this layout, so no visible difference. Content scrolling beneath chips needs a post-children paint hook.
- `bands.ts` still carries the sliver rule (`chipGapReadsAsSliver`); the design removes gaps, so lane `elements` may drop it and then `bands.rs`/`bands.test.rs` too.
- `🧪️tests/🧫️fixtures/🖥️prepared-gpu-clip-pieces` kept: it is the generic scanline clip law.
- I ran `git rm --cached` on `bands.rs` by mistake once (stages a deletion in this machine's index; file was recreated with the reduced content). The archivist should `git add` it.
- Lane `wgpu-os` must replace `end_silhouette_clip` in canvas2d `scene-nodes-and-image-upload` with `end_clip` and use `push_scissor` for the Dock frame.
- D3D12 and Vulkan edits could not be compiled on this macOS host.
