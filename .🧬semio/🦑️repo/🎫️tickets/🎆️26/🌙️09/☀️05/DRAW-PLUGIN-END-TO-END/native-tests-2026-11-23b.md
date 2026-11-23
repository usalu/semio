# Draw drawing native tests, 2026-11-23 (second pass)

Result: PARTIAL. Source admission and the scene_surface compile blockers are gone; the drawing test crates now compile and run (model-tests excepted). Remaining failures are test-versus-implementation regressions, not environment problems.

## Commands (cwd = drawing workspace, SEMIO_BUILD_MODE=prototype SEMIO_LANGUAGE=en)
`repo cargo --store=isolated --configuration=prototype -- test [--no-fail-fast] -p <crate>...`
Must run from `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing` (from the repo root the crates are not workspace members). Builds take 15-35 min on 1 admitted core; run in the background.

## Blocker 1: source admission
- Editor: moved `🪆️child/🧪️tests` (child registration test, 1.5 KB with fixture) into the `editor-tests-2` companion crate (mount `child_editor::tests`, explicit imports, two new dev-deps). Admission now passes; editor lib tests run.
- Model: admission passes (another agent's concurrent migration already reduced its closure to 31 files).

## Blocker 2: compile errors
- `scene_surface` callers: no compile error surfaced in any crate these tests depend on.
- Fixed a syntax error (extra `)`) in `has_nonempty_lang`, pdf plugin `♿️ua/🚪️io/🦀️.rs`, which broke the whole workspace build.

## Results
| crate | result |
|---|---|
| semio-s-artifact-draw-drawing-editor (lib) | 4 passed, 0 failed |
| semio-s-artifact-draw-drawing-examples | 5 passed, 0 failed |
| semio-s-artifact-draw-drawing-editor-tests-2 | 26 passed, 5 failed (was 25/6; fixed empty-close fixture path) |
| semio-s-artifact-draw-drawing-editor-tests | ABORTS (SIGTRAP): "controlled retirement abandoned before terminal-empty" (value-retirement `🎮️controlled/🦀️.rs:389`) in canvas_laws tests; fixtures drop apps without paid close. 1 of 82 ran green first |
| semio-s-artifact-draw-drawing-model-tests | does not compile, 71 errors (see below) |
| semio-s-artifact-draw-drawing-model (lib) | does not compile its test cfg: E0282 in `📸️snapshot/🔎️lookup/🫴️original/🦀️.rs:35` (`test_step`) |

### editor-tests-2 remaining failures
- `args_bridge_tests::host_pointer_numbers_and_nullable_export_fields_are_typed`: `app_instance_id` decodes to None from `appInstanceId: 4.0` (Option<u32> from float JSON).
- `completion::tests::completion_original_alias_and_payload_physical_release`: 1488 vs expected 1424.
- `path_job::tests::retained_path_simplification_shared_cases`: `progressed && completed` false.
- `export_work_tests::export_retires_the_original_segmented_transport`: `terminal_is_empty` false.
- `export_work_tests::original_export_read_returns_under_exact_physical_grants`: "captured shared original changed its Work or destination".

### model-tests compile errors (companion crate is mid-migration by another agent; those files are uncommitted-modified live)
Missing dev-deps (`semio_framework_artifact_reference`, `_pixels`, `_2d`, `_value_persistent`, `semio_s_artifact_draw_drawing_io`); `test_step` is `#[cfg(test)]` in the model lib so the companion cannot import it (move it into the companion); removed APIs (`RetainedCloneSource::admit_owned`, `DrawingVectorSource::read/read_demand`, `NativeReceive::begin_receive/receive_birth_demand`, `unwrap` on `RetainedWorkOutcome/Poll`, extra args on several calls); lost private names in `🧪️tests/🔬️unit` (`DrawingSnapshot`, `resolve_*_layer_segments`, `drawing_image_asset_luma`, `CURVE_LINE_SAMPLE_STEPS`, `font_family`).

## Files changed
Under `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/`:
- `🪆️child/🦀️.rs`, `🪆️child/🧪️tests/🦀️.rs`
- `🧪️tests/🍰️companion-2/🦀️.rs`, `🧪️tests/🍰️companion-2/📦️packages/🦀️rust/Cargo.toml`
- `🧪️tests/🔬️gesture-operation-owner/🦀️.rs` (fixture path relative to the companion manifest)
Plus `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua/🚪️io/🦀️.rs` (syntax fix).
