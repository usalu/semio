STATUS: PARTIAL. GREEN: framework `semio-framework-actor-retained-turn` lib tests 11/11 after the `.min(64)` removal (`🗑️generated/1101/pngfix-fw.txt`, rc 0), and `semio-framework-os-infinite-world-components` native check rc 0 with the PNG depth fix (`pngfix-world.txt`). Size split: Cargo Source admission passes for scene (232,641 B) and the new scene-paint (110,992 B) against 262,144 (`pngfix-2.txt`). UNVERIFIED: compile of scene, scene-paint, draw io/editor callers and the 4-line draw depth edits, blocked by foreign `draw-drawing-model` (56 errors); layout export edit blocked by foreign `semio-framework-value-dictionary` (4 `RetainedWorkOutcome::map_err` errors, `pngfix-layout.txt`).

## 1. Size blocker (draw-drawing-scene)
Cause: closure was ~330 KB (non-test 299 KB + cfg(test) mods), limit 262,144. The old error text only named the file where the running total crossed the limit (`🔤️text/🦀️.rs`), not the culprit.
Split by domain (no file moved, no limit raised): the painting domain becomes its own crate.
- New crate `semio-s-artifact-draw-drawing-scene-paint` at `…/🧬️schema/🎬️scene/🎨️paint/📦️packages/🦀️rust/Cargo.toml`; its lib root is the existing `🎨️paint/🦀️.rs`, which now mounts `picking` (`🧮️geometry/🎯️picking/🎨️paint`) and `placement` (`📍️placement`) next to `scene` (`📋️prepare`).
- The scene crate lost the mounts `paint`, `picking`, `placement` in `🎬️scene/🦀️.rs`. No cycle: nothing left in scene referenced them (prepare/raster/trace/booleans/text/view/identity/retire only use `preparation`, `raster`, ...).
- Paint sources import `preparation`, `identity` from `semio_s_artifact_draw_drawing_scene`; `crate::paint::scene` became `crate::scene` (lasso, placement).
- `testkit` feature of the new crate forwards to scene's `testkit` (placement derives serde under it). Dev-deps mirror what the in-crate `stroke_custody_tests` needs.
- Registered in `🖍️drawing/Cargo.toml` (member + workspace dependency).
- Callers rewritten by exact path (22 files): `drawing_scene::paint::X` to `drawing_scene_paint::X`, `::picking::`/`::placement::` to `drawing_scene_paint::picking::`/`::placement::`; the shared glob `use` line of the 14 unit-test files now imports from both crates. Dependency added to exactly the six crates whose closures use it: editor-tool (canvas-pointer-down), write (export), scene-tests companion, editor-tests companion, editor-interaction (+ its `testkit` feature), editor.
- Source-admission ran past both new/changed units; module path bug found by it (`../🧮️geometry` needed `../../`) was fixed and re-run.

## 2. Depth grant audit (PNG range quote >= 3)
Host grants: both hosts self-issue depth 64 (`OS…/💼️job-crossing/🦀️.rs` `SELF_ISSUED_GRANT`, TS `ORIGINAL_SELF_ISSUED_GRANT`), so layout export steps are granted 64.
| caller | finding | action |
|---|---|---|
| framework `decode_encoded_blocking` (raster panes, canvas icon codec `🪪️icon-codec`, wgpu packet-image, ui-image-loading, world brush-lod, face-overlay) | self-grants the quoted depth (pixtest) | none |
| `decode_png` (png-encoding-core; raster-host, terrain, tiled-map, note, ui-image-sources, remodel, layout canvas) | independent scanline codec, no job | none |
| draw `DrawingImageAdmissionJob` + import-image work quote | quote and gate use `next_depth_demand` (pixtest); command grants 128 | none; compile UNVERIFIED (model blocker) |
| layout export `🎨️faithful-primitives/🦀️.rs` `prepare_image_one` (ImageDecodeJob::advance with `context.retained_grant()`) | stage got 64, so fine today, but a lower grant would no-op silently forever | now refuses explicitly: `layout-export-image-depth-grant` when `next_depth_demand() > grant.maximum_depth`. Compile UNVERIFIED (layout workspace check not yet run) |
| world `🪄️placeholder-mesh-and-gumball` PNG texture job | hard-coded `maximum_depth:1`: `advance` gated on the new quote and stalled forever (real bug) | `maximum_depth: job.next_depth_demand()`. Compile UNVERIFIED |
| wgpu `glb-decode/🧵️materialize-cursor-step` | still the pre-migration API (`advance(4096)`, `into_result`), in no Cargo.toml (quarantined renderer) | not touched; cannot compile either way |

## 3. `.min(64)` at `🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs:72`
It is in `OriginalJobSourceCursor::advance`, the copy of the job SOURCE (kind, input, checkpoint) before the crossing, not the job output. The output cap fixed by jobpage (`call/🦀️.rs` guest `.min(64)` plus `OUTPUT_PAGE_BYTES`) is a different place, so the audit-open "conflict" mixed two sites. Decision: still a hard-coded step cap with no schema or grant behind it (destination is allocated exactly beforehand, the grant already bounds the copy), so every job input is copied 64 bytes per turn (one fuel per 64 B). Removed: `count = grant.maximum_copy_bytes.min(length - position)`. Test `original_job_source_keeps_actual_source…` sweep extended `[1,3,64]` to `[1,3,64,128]` (input is 97 bytes, so 128 now copies it in one step; 64 previously needed two). The transport twin `📦️transport/📥️source` has no cap. Remaining 64 B pagers noted by jobpage (`reserved/🦀️.rs:134`, `completed/🦀️.rs:37`) untouched.

## Open blocker (owner needed): draw model not ported to the work-accounting API
`semio-s-artifact-draw-drawing-model` fails with 56 errors, all in `🧬️schema/🛬️receive/🦀️.rs` (old `NativeDefault::native_default*`, now `type Cursor`/`default_cursor`), `📸️snapshot/🔎️lookup/**` (`?` and `.map_err` on `RetainedWorkOutcome`, `step.progress()`), `📸️snapshot/🦀️.rs`, `✅️validation/🦀️.rs` (lines 137, 148). No lane touches it (no result file mentions it). Every draw crate that depends on it, scene and scene-paint included, cannot get a compile verdict until it is ported. Not mechanical (NativeDefault now needs a phased cursor), so not attempted here.

## Files changed
Size split: `…/🎬️scene/🦀️.rs`, `🎨️paint/🦀️.rs`, `🎨️paint/📋️prepare/🦀️.rs`, `🎨️paint/📋️prepare/🎯️query/🦀️.rs`, `…/🪢️lasso/🦀️.rs`, `📍️placement/🦀️.rs`, `🧮️geometry/🎯️picking/🎨️paint/🦀️.rs`, new `🎨️paint/📦️packages/🦀️rust/Cargo.toml`, `🖍️drawing/Cargo.toml`, six caller Cargo.toml, 22 caller/test sources.
Others: `🧰️framework/…/🎭️actor/🎟️retained-turn/💼️job/📥️source/🦀️.rs` and its `🧪️tests/🦀️.rs`, layout `🎨️faithful-primitives/🦀️.rs`, world `🪄️placeholder-mesh-and-gumball/🦀️.rs`. Scripts: `1101-tools/pngfix/fw.sh`.

## Verification
- `pngfix-1.txt` rc 1: scene-paint module path error (fixed, `../../🧮️geometry`). `pngfix-2.txt` rc 1: admission passes for all units, then `draw-drawing-model` 56 errors (blocker above) and `stdio-pdf-schema-4` 14 errors (foreign).
- `pngfix-fw-1.txt`/`-2.txt`: retained-turn test target did not compile because of stale tests after the outcome migration (6 sites: `JobPayloadAuthority::admit`, `OriginalCallerSourceParent::admit`, `close_step` need `.result`); fixed in `retained-turn/🧪️tests/🦀️.rs` and `💼️job/📥️source/🧪️tests/🦀️.rs`. `pngfix-fw.txt` (rc 0): 11 passed, 0 failed, including `original_job_source_keeps_actual_source_and_absolute_deadline_without_refill` with copy grants 1, 3, 64, 128.
- `pngfix-world.txt` rc 0 (root `Cargo.toml`, `-p semio-framework-os-infinite-world-components`, native). The first attempt used the framework manifest where that crate is excluded (`pngfix-world-1.txt`).
- `pngfix-layout.txt` rc 1: `semio-framework-value-dictionary` fails (`map_err` on `RetainedWorkOutcome`), so `semio-s-artifact-layout-layout-engine` was never reached.
- Also removed the stale `paint::*,picking::*,placement::*` from the in-crate raster unit test glob (`📷️raster/🧪️tests/🔬️unit/🦀️.rs`), since those modules left the crate.

## Next
1. Someone ports `draw-drawing-model` (`🛬️receive`, `📸️snapshot/🔎️lookup/**`, `📸️snapshot`, `✅️validation`) to `NativeDefault::Cursor` and `RetainedWorkOutcome`; then re-run `C/1101-tools/chk.sh pngfix-3 …/🖍️drawing/Cargo.toml native -p semio-s-artifact-draw-drawing-{scene,scene-paint,io,write,editor-tool,editor,editor-commands,editor-interaction,viewer}`.
2. `semio-framework-value-dictionary` `map_err` sites (framework owner), then re-run the layout check.
