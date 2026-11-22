# L1b Rust 3D test targets

Result: `cargo check --workspace --all-targets` of the Puzzle 3D workspace passes with zero errors (log `🗑️generated/l1b-check3.log`, 0 `error` lines, 14 failing test targets and 123 error sites at the start). Runner: `l1b-native.sh` (`--jobs 2`, tmpfs size via `L1B_TMPFS`, default 4g).

`3d/🏛️ownership/🧮️compute` is a TypeScript project (`script.ts`, `project.json`, no `Cargo.toml`), so it has no Rust check target.

## Fixes (all under `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/`)

Editor tests (`✏️editor/🧪️tests/...`):
- `crate::editor::puzzle3d::*` and `crate::retained_command::*` paths replaced by the owning crates (`semio_s_artifact_puzzle_3d_commands_document::{export_snapshot,import_snapshot}`, `..._editor_snapshot`, `..._precompute`, `..._model`, `semio_s_plugin_puzzle_retained::{PUZZLE_*,testing}`).
- `🧰️prelude`: imports for the session registry types and `puzzle3d_action_uses_precompute` / `PUZZLE3D_MESH_PAGE_SCAN_CHARS`.
- `with_puzzle3d_app_for` to `with_puzzle3d_session_for`; `protocol::Mutation::<T>` to `protocol::ColdMutation::<T>`; `ViewModel` args wrapped in `Arc`; `BoundedFirstStepCommandWork::new(.., puzzle3d_retained_reduce::<Puzzle3dPlayApp>, ..)`.
- `command_from_action` to the 3-arg `ArtifactEditor` form; `take_fixture_result_turn(&mut *app ..)` derefs; window-config reload via `advance_window_config_load` (new dev-dep `semio-plugin-window-config` in the editor crate).
- `object_flag` and `object_cores` deduplicated from 4 test files into `🧰️context/🖼️render-and-scene-probes`.
- Removed duplicate `SET_ACTIVE_UTILITY_ACTION_ID` import in `🔬️panels/🦀️.rs`.
- Fixture helpers: `precompute_model_tests::context::*` (model crate `testing` feature) imported in precompute and precompute-geometry unit tests; `use protocol::Inference as _` in inferences test.

Production visibility widened so tests in other crates compile (they probe internals; no behaviour change):
- `🎟️session/🗃️session-registry`: `pub(super)` to `pub` (registry, slot, state, lease, constants).
- `⏳️precompute/🪣️fill/{🏗️builder-state,🛑️fill-contract}`: `pub(super)` to `pub`.
- `🎮️commands` `puzzle3d_action_uses_precompute`, `🧠️precompute-command-work` `PUZZLE3D_MESH_PAGE_SCAN_CHARS`, `🎯️focus-selection-work` `frames_source`: now `pub`.

Other crates:
- `🎮️commands/📤️jobs` (`semio-s-plugin-puzzle-jobs` `testing`): new `OriginalFixture::step_job_observed(job, budget, &mut verdict)`; `step_job` delegates to it. fill-run test uses it instead of the removed 5-arg `drive_step` call.
- fill-run test: `ColdMutation`, snapshot import path, `semio-framework-job-payload` dev-dep in session crate.
- `🎟️session` brush test: `parry3d::shape::Shape` and `query::PointQuery` trait imports.
- `🪟️window` test: retirement via `semio_framework_value_clone_source_testing::drive` (new path dev-dep).
- `🚪️io/🪶️sqlite/📸️snapshot` test (puzzle-3d `snapshot-sqlite`): 6 sites moved to `semio_framework_test_original_snapshot_owners::{export_sqlite,import_sqlite,encode_native,decode_native}` (same as the 2D lane).
- modes window test: imports for `mesh_lane`, `world_meshes_json`, mesh kind constants.

## Test run (compiled crates, `--no-fail-fast`, `--all-features`)

`inferences, editor-window, retained-action, precompute-geometry, editor-modes, precompute, editor-session`: 159 passed, 4 failed (`🗑️generated/l1b-test1.log`). Failures are runtime, not compile:
- `editor-modes` lib: `the_brush_suggestions_reconciler_starts_wakes_and_aborts_exactly_once_per_answer` (panic in `retirement/allocation-return`).
- `editor-session` `precompute-fill`: `original_fill_builder_close_denials_and_receipts_match_actual_heap`, `original_fill_run_and_revalidation_close_denials_match_actual_heap` (assert at fill tests 180/192), `fill_run_job_step_and_overlay_append_stay_below_the_interactive_ceiling_for_nakagin` (panic at `jobs/🦀️.rs:517`, original-fixture step).

## Not run

`-p semio-s-artifact-puzzle-3d-editor -p semio-s-artifact-puzzle-3d` tests (editor-defects, manifest, panels, budgets, surface, snapshot-sqlite): runs aborted because concurrent edits by other lanes broke the shared tree (procedural `graph-ports` manifest missing, then `framework/pack/json` `u32_array` file moved). Rerun: `L1B_TMPFS=7g L1_LOCK=-q ./l1b-native.sh 3d test --no-fail-fast -p semio-s-artifact-puzzle-3d-editor -p semio-s-artifact-puzzle-3d`.

Note: the mutated-source laws in `🔬️unit/🧵️cursorized-route-laws` and `🎛️dispatch-and-operation-laws` still replace old `Box::new(crate::retained_command::BoundedFirstStepCommandWork ...)` source text that production no longer contains; they compile, but may now assert nothing or fail at runtime.
