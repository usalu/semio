# FEM 3D artifact: native Rust state (editor crate owner)

Workspace: `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\🧊️3d`. Env: `CARGO_TARGET_DIR=C:\t\3d`, `CARGO_BUILD_BUILD_DIR=C:\t\3db`, `TMP/TEMP/TMPDIR=C:\t\tmp`, `CARGO_INCREMENTAL=0`, debug=0, `-j 6`. Scripts: `a3d-test.sh`, `a3d-test-editor.sh <tag>`, `a3d-editor-check.sh`, `a3d-editor-errors.py`.

## Compile state (cargo check --all-targets, check7 and per-target r2)
- Framework and all 3D library crates compile. Earlier framework errors (`semio-os-store-document`, `folder_archive_load`, `collect_window_kind_ids`) were mid-edit snapshots and did not recur.
- Editor crate (`semio-s-artifact-fem-3d-editor`): lib test and `editor_app_2` now compile. `editor_app` belongs to another agent (56 errors at r2). Other 3D crates belong to the core-tests and interaction-panels agents.

## Test state, editor crate (last run that reached the tests: e2)
- `statik_pane_smoke`: 3 passed.
- Lib: 5 passed, 1 failed (`window_config_ownership::fem3d_window_config_runtime_isolates_same_kind_instances_and_restores_packs`).
- `editor_app_2`: 32 passed, 18 failed (first full run had about 35 failed).
- Runs e3 and e4 did not reach the tests: `semio-plugin-instance-app` (framework, another agent's in-flight `Box<...>` field change) does not compile (4 then 8 E0308 errors in `📥️staging/🚪️ingress/🦀️.rs` lines 177-202). Not mine; retry when that agent finishes. Fixes made after e2 are therefore unverified.

## Fixed in the editor crate (tests ported to the current API, assertions preserved)
- Harness `🧪️tests/🔬️app-2/🦀️.rs`: local `edit::windows::{model,results}` alias, `Arc<ViewModel>` view state, `dispatch_rows` driven by the `settle_registered_typed_operation` receipt, `retire` and `Retiring` helpers (sealed snapshots must be retired through an original fixture), `#![recursion_limit = "256"]`.
- Command tests: `patch_node`, `patch_section`, `patch_solid`, `patch_support` (handle/ConfigView imports, `Fem3dSnapshotValue` + `from_reference`, `ColdMutationDiff::<..>::absorb`, `ColdDiffAlgebra::inverse`, replay on the value type), `remove_selection` (imports, value-type replay, fixture DOF names `Tx/Ty/Tz` and `dof: Tz`), `set_active_example`, `set_result_animation`, `result_animation_tick` (retire, boot/demo snapshot `Result`, `.as_str()` ids).
- `results_window` test: authority identity via `WindowConfigAuthorityIdentity::issue()`, `close_step` through a fixture scope, cache check via the new public accessor `results_cache_resident()` (added to `🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`, because the old test read a private static), retire of built snapshots.
- Result source ids are now encoded (`semio_s_fem_analyses_model::encode_result_source_parts("loadCase", "dead")`, as in the 2D tests) in the results-window, set-result-display and window-ownership tests (unverified).
- `🪟️window-config-ownership`: ported to `WindowConfigPack.source.pair()` and `restore_fixture_window_config` (like the 2D test), actor `LOCAL_ACTOR_ID`, retire of admitted values.

## Open (editor crate)
- 6 `result_animation_tick` tests fail with `fem.result-animation-tick.window-transient-mismatch`; window and kind ids look consistent in the code, cause not found (harness issue or product bug, undetermined).
- `context::history_edits_end_to_end` and `conflict_history_edits_end_to_end` fail inside the framework's shared history-edit acceptance suite ("original report requires its installed semantic and retirement issuers"); looks framework-side.
- `results_window` tests that wait on solver jobs show "Analysis queued" / "Result not found" captions (`results_scene_captions_name_mode_and_factor_3d`, `results_window_surfaces_solver_error_without_panicking_3d`, `playback_phase_moves_the_deformed_scene`, `mounted_results_frames_never_invoke_synchronous_solvers`); may be fixed by the source-id encoding, unverified.
- Framework `[DEBUG]` prints in the logs are another agent's temporary logging.

## Not mine (split by the coordinator)
Engine, io, mesh, model, snapshot-tests, viewer: see `artifact-3d-core-tests.md`. Interaction, panels: see `artifact-3d-interaction-panels-tests.md`. `editor_app`: other agent.

## Rules followed
Nothing under `C:\t\3d` or `3db` deleted after the coordinator's instruction (one earlier `rm` of my partial build dir happened before it, and it was rebuilt). No git state touched. Generated logs are in `🗑️generated/`.

## Editor crate (round 2)
Run r2b (lib + editor_app_2): lib 5 passed / 1 failed (`window_config_ownership` runtime isolation, not investigated); editor_app_2 46 passed / 4 failed (was 32/18). `statik_pane_smoke` not rerun.

Fixes:
- ROOT CAUSE of all `result_animation_tick` mismatches (2D and 3D): the fixture encodes a typed command payload with snake_case keys (`window_id`), `command_from_action` decoded only camelCase (`windowId`), so every tick arrived with `window_id == ""`, the retained route captured the focused window instead of the addressed one and `required_clock` refused. Fix: `command_from_action` now looks a key up by its camelCase and its snake_case spelling (`lookup` helper) in `🧊️3d/.../✏️editor/🖥️play-app/🦀️.rs` and `◻️2d/.../✏️editor/🎛️play-app-editor/🦀️.rs`. 5 of 6 tick tests pass now.
- `🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs`: engine results are keyed by encoded result-source identity, so `render_static` now falls back to the encoded first load case and captions and buckling decode the plain id (`source_label`). Removed the test-only `results_cache_resident()` accessor; the cache law now observes eviction through `results_solve_count()` (re-solve counts) using the existing API.
- Results-window unit tests: "Analysis queued" captions came from rendering the mounted app (jobs pending); `captions_name_mode_and_factor` and `surfaces_solver_error` now render the app's captured window config directly (`rendered` helper); `mounted_results_frames_never_invoke_synchronous_solvers` no longer asserts a Progress step on an unregistered instance.

Open:
- `playback_presses_are_one_config_edit_and_never_a_history_row`: fixture settlement stops with WorkLimit, maintenance demand copy 20456 B > remaining grant 20278 B (INTERACTIVE_COMPONENT 32 KiB per turn); framework budget vs retained-owner size, not fixed.
- `playback_phase_moves_the_deformed_scene`: phase 1 vs 0 poses equal after the encoded-source fix; cause undetermined (default waveform is Ramp).
- `history_edits_end_to_end` / `conflict_history_edits_end_to_end`: cause 1 found: `ArtifactEditor` for FEM never overrides `document_store_owners_source_demands`/`build_document_store_owners`, so the store has no replay-preparation owner (message "installed semantic and retirement issuers"); `process3d`'s editor-impl shows the opt-in (`cold_replay_document_store_owners` + `operation_wire` with `whole_document_target`). Trying it moved the failure on to "no authored conflict path issuer" (without wire), then to a stuck Replaying stage and "Replay: truncated at offset 14" (with wire): FEM also needs an envelope decode owner bundle like `process3d_envelope_decode_owner_bundle`. That is a sizeable per-plugin integration; override was reverted, no product change left.

### Editor crate (round 2, continued)
Run r3h (lib + editor_app_2 + statik_pane_smoke): lib 5/6, editor_app_2 47/50 (was 46), statik_pane_smoke 3/3.
- `playback_phase_moves_the_deformed_scene` fixed (test): uses the demo snapshot and compares the read-out through the label (`Phase 0.42`, not `phase 0.42`).
- Last tick test passes; with the unit-test fixture budget unchanged, `playback_presses_are_one_config_edit_and_never_a_history_row` now fails earlier or later depending on grant: at the stock grant it stops at maintenance demand 20456 B > remaining 20278 B; with a 4x maintenance copy grant it moves on to "terminal ticket has no explicit native header lifecycle" (framework native-header gap). Left at the stock grant; open, framework side.
- `window_config_ownership`: added native pair/pack receiving for both 3D window configs (`OriginalArtifactPackReceiving`, `native_pair_demands`/`admit_native_pair`, `native_operation_wire`, operation targets, `NativeReceive`/`DslReceiving` derives; new workspace deps `semio-os-store-codec-registry`, `semio-os-store-operation-wire`; `NativeReceive` also added to `Viewport3dOrbit` (framework), `FemResultsAnimation`/`ResultMode`/`FemWaveform`/`FemLoopMode` (fem app-surface) and `Fem3dGumballConfig`). Failure moved from "must declare its original native Pack receiver" through "no installed native operation receiver" to "original JSON operation wire header differs" (copied_bytes 3): the operation wire header of the update variant disagrees with the default JSON header; still open.
- History replay tests: owned by the fem-history-replay agent.
