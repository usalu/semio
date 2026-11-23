# FEM 2D artifact Rust workspace: status of agent b

Workspace: `C:\semio\✏️s\🔌️plugins\🏗️fem\🗿️artifacts\◻️2d\Cargo.toml`.
Env: `CARGO_TARGET_DIR=C:\t\2d-a CARGO_BUILD_BUILD_DIR=C:\t\2d-ab TMP/TEMP/TMPDIR=C:\t\tmp CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0`, `--config 'build.rustc="rustc"' --config 'build.rustc-wrapper=""'`, `-j 6`.

## State
- `cargo check --workspace --all-targets --keep-going` is green (`C:\t\2dw-check3.log`).
- Last full `cargo test --workspace --no-fail-fast -j 6`: `C:\t\2dw-test6.log`. 68 targets reported: 1276 passed, 56 failed, 60 ignored. Four more tests of the snapshot-tests target never reported because I killed that target's process after it ran 70 minutes (listed below, so 60 names in total).
- The failure list below was generated from that log. Some of it is stale: the test6 editor-lib binary predates my last edits (for example `canvas_pointer_down::an_unaddressed_pointer_event_faults` passes when I run the newest editor lib binary directly). Rerun before trusting a name.
- Not mine, skip: `window-transient-mismatch` / `result_animation_*` (3D editor agent, root cause window_id snake_case in `command_from_action`, reported fixed), `history_edit_acceptance` and `context::*history_edits_end_to_end` / `editor_unit::two_*` replay failures (FEM replay-preparation agent, `fem-history-replay.md`).

## Suspected causes of the remaining groups
1. Sealed value dropped without handback (`sealed source requires original granted handback`, panic at framework `sealed` Drop): a test holds a bare `Fem2dSnapshot` (or a value built from one). Fix pattern: wrap in `Fem2dSnapshotRetiring::new(...)` (it now has Deref and DerefMut), or retire through `Fem2dSnapshot::retire_decoded`. Seen in editor lib / `editor_app` / `editor_app_2` / viewer / model `inference_default_laws` / io component tests (`json_round_trips_the_example` etc.), `verify_protocol_bytes_against_encoded_pack`, `bounds_default_law`.
2. Demo document not what the test expects (`load-case-missing`, `twelve nodes`, `remove_selection_covers_regions` index 0, `set_analysis_settings_partial` 3 vs 4, `inspection ... first_2 json.contains id`): suspect the app test host applies `Effect::LoadDocument` from `SetActiveExample` but the loaded demo differs from `FEM2D_EXAMPLE_TEXT` (ids now scoped, or the load does not replace the document). Start at `🧪️tests/🧰️context/🦀️.rs` `finish_dispatch` and `🎮️commands/📚️set-active-example`.
3. Pointer commands: `canvas_pointer_down_*` assert `args.get("targets")` is a string, but the replay args now carry a JSON array of `{"granularity","id":"scope:..."}`. Update tests (or the command) to the one chosen wire shape.
4. `every_command_matches_declared_wire_bytes`: wire bytes changed for `removeSelection` (command payload shape changed to scoped ids). Regenerate the declared wire fixture or restore the shape.
5. `admitted_document_mutations_make_bounded_progress_and_retire` (`🧪️tests/🔬️fem2d-artifact-preparation-laws`): `released == allocated + capacity` is 2422 vs 2390 (32 bytes extra released); my test now drives `close_step` through `semio_framework_value_clone_source_testing::drive`; probably the owner Box / `OneItemOwners` setup bytes released by the close but allocated outside the measured region.
6. sqlite snapshot tests (`semio-s-artifact-fem-2d` lib, `io/sqlite/snapshot/tests`): fixture `🧫️fixtures/🔣️.json` made graph valid (see fixes); last run still said `FEM reference graph or row is invalid` because the area load pointed at a missing region, fixed afterwards and not re-run.
7. `semio-s-artifact-fem-2d-snapshot-tests` (4 tests, shared with the 3D core agent): `native_tests::actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work`, `actual_chain_receiving_identity_and_reference_work_scales_below_threefold`, `receiving_charges_below_sixty_four_kibibytes_of_retained_capacity_per_node`, `native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix` failed or aborted; and `load_tests::every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication` blocked the run for 70 minutes (on the default stack it overflows immediately; with `RUST_MIN_STACK=268435456` it ran more than 3 minutes without finishing). The identical test exists in the 3D twin (`🧊️3d/.../🧪️tests/loads/receiver-tests.rs`). It loops `for cut in 0..calls` with one full decode per cut; check `calls` first, then the stack use of the decode path.
8. Misc singles: `surface_corpus_exports_obj_and_stl_beside_the_neutral_surface` (io: `owning surface contract command must declare its output path`), `clock_publication_and_retirement_obey_tiny_grants` (transient: `ticket close requires its original producer invocation`), `renders_mesh_edge_preview_for_the_default_example` (viewer, no mesh-edge layers), `mounted_camera_and_interaction_presence_reach_peers...` (`dispatch model camera` failure), window-config ownership law (2 tests).

## Fixes made (files, relative to the 2D workspace root)
- `🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📸️snapshot/cold-tests.rs`: `nodes2d` / `materials2d` return `SealedShared<PersistentList<..>>`; added `unsealed` helper (`try_unwrap`) before `cold_dismantle`, import `RetainedCloneGrant`.
- `.../✏️editor/🧪️tests/🔬️fem2d-artifact-preparation-laws/🦀️.rs` and `.../✏️editor/📦️packages/🦀️rust/Cargo.toml`: `close_step` driven through `semio_framework_value_clone_source_testing::drive/warm`; dev-dependency added (path); `last_work: 0` field.
- `.../✏️editor/🧪️tests/🔬️scoped-identity/🦀️.rs` (new) mounted from `.../✏️editor/🦀️.rs`: scoped entity identity collision law on sealed `Fem2dSnapshotRetiring` (replaces the old interaction-crate test); `ArtifactView::new(&*document, ..)`; arena fixture.
- `.../✏️editor/🕹️interaction/🧪️tests/🔬️unit/🦀️.rs`: demo and identity fixtures built through `Fem2dSnapshotValue` and `Fem2dSnapshotRetiring`; `Fem2dSnapshot::validate_interaction_record` imports; `public_json_snapshot_admission` test also rewrites element material references when it renames a material. `.../🕹️interaction/📦️packages/🦀️rust/Cargo.toml`: dev-dependencies now `semio-s-artifact-fem-2d-model` (testkit) and `semio-plugin` (artifact-app-testing); removed editor, editor-commands, editor-model-window dev-dependencies.
- `.../✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🔬️unit/🦀️.rs`: demo is `Fem2dSnapshotRetiring`; `to_reference()` for folding; `close_step_cold` for the store disposer; preview through `Fem2dSnapshot::to_reference/from_reference`.
- `demo()` converted to `Fem2dSnapshotRetiring` (and `ArtifactView::new(&*doc, ..)`) in: `🎮️commands/🎯️focus-entity`, `🎮️commands/🩹️patch-load-case`, `🎮️commands/🖱️canvas-pointer-down`, `🕹️interaction/🖱️canvas-gesture`, `📌️panels/🔍️inspection` (each `🧪️tests/🔬️unit/🦀️.rs`).
- UI value arena: `let _arena = semio_ui_contract_values::testkit::UiArenaFixture::new();` at the start of 32 panel tests in `📌️panels/🗿️artifact`, `📌️panels/🔍️inspection`, `📌️panels/📊️results` (`🧪️tests/🔬️unit/🦀️.rs`) and in the scoped-identity test; dev-dependency `semio-ui-contract-values` (feature `testkit`) in `📌️panels/📦️packages/🦀️rust/Cargo.toml` and `✏️editor/📦️packages/🦀️rust/Cargo.toml`.
- sqlite fixture `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json`: distinct non-empty node / element / load ids, resolved node, material, section, region and element references, distinct combination case ids (mirrors the 3D twin).

## Failing tests of `C:\t\2dw-test6.log` by target
### semio-s-artifact-fem-2d :: semio_s_artifact_fem_2d (unittests ..\..\.rs) - 7 failing
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_independent_bun_file_queries_and_edits`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_all_words_and_count_widths`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_complete_physical_state`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_actual_declaration_erased_words_both_encodings`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_invalid_graph_and_storage`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_real_interior_copy_controls_and_budgets`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro
- `standards::v1::subsets::any::io::sqlite::snapshot::tests::sqlite_snapshot_fem2d_native_decode_actual_interior_owned_work`: called `Result::unwrap()` on an `Err` value: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_pro

### semio-s-artifact-fem-2d-editor :: semio_s_artifact_fem_2d_editor (unittests .rs) - 4 failing
- `editor::fem2d::commands::canvas_pointer_down::tests::canvas_pointer_down_on_a_node_requests_interaction_select`: [{"granularity":"node","id":"scope:1:4:node2:n1"}]
- `editor::fem2d::commands::canvas_pointer_down::tests::the_results_window_picks_with_its_own_camera`: assertion failed: replay(&emit).1.get("targets").and_then(semio_framework_value::DslValue::as_str).is_some_and(|raw|
- `editor::fem2d::component::fem2d_artifact_preparation_laws::admitted_document_mutations_make_bounded_progress_and_retire`: assertion `left == right` failed
- `editor::fem2d::commands::canvas_pointer_down::tests::canvas_pointer_move_requests_hover_and_pointer_up_refreshes_the_window`: assertion failed: args.get("targets").and_then(semio_framework_value::DslValue::as_str).is_some_and(|raw|

### semio-s-artifact-fem-2d-editor :: editor_app (tests\app\.rs) - 10 failing
- `editor_unit::every_command_matches_declared_wire_bytes`: assertion `left == right` failed: wire bytes changed for removeSelection
- `editor_unit::mounted_camera_and_interaction_presence_reach_peers_without_document_edits`: dispatch model camera: OriginalTurnFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_bytes: 0, 
- `editor_unit::two_actors_reconnect_after_local_edits_without_freezing_or_duplicate_replay`: sealed source requires original granted handback
- `editor_unit::two_instances_converge_on_disjoint_edits`: sealed source requires original granted handback
- `context::conflict_history_edits_end_to_end`: sealed source requires original granted handback
- `context::history_edits_end_to_end`: sealed source requires original granted handback
- `inspection_panel::a_multi_selection_headers_the_count_and_inspects_the_first_2d`: assertion failed: json.contains(&format!("\"{id}\""))
- `patch_load_case::patch_load_case_renames_and_toggles_self_weight_on_the_demo_2d`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `inspection_panel::the_body_key_routes_to_this_panel_on_the_live_app_2d`: the demo's twelve nodes are counted
- `gumball::a_streamed_gesture_is_one_transaction_with_a_preview`: sealed source requires original granted handback

### semio-s-artifact-fem-2d-editor :: editor_app_2 (tests\app-2\.rs) - 16 failing
- `remove_selection::remove_selection_orders_dependencies_and_restores_exact_snapshot`: sealed source requires original granted handback
- `result_animation_tick::result_animation_tick_parks_the_clock_when_stopped`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `context::conflict_history_edits_end_to_end`: sealed source requires original granted handback
- `result_animation_tick::result_animation_frames_never_touch_the_window_config_ledger`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `remove_selection::remove_selection_covers_regions_2d`: index out of bounds: the len is 0 but the index is 0
- `result_animation_tick::result_animation_tick_advances_the_clock_and_rearms_while_playing`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `context::history_edits_end_to_end`: sealed source requires original granted handback
- `remove_selection::remove_selection_covers_nodes_elements_materials_sections_supports_load_cases_2d`: sealed source requires original granted handback
- `set_analysis_settings::set_analysis_settings_partial_args_keep_current_2d`: assertion `left == right` failed
- `result_animation_tick::long::result_animation_frame_cost_stays_flat_across_a_long_run`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `window_config_ownership::fem2d_window_config_runtime_isolates_same_kind_instances_and_restores_packs`: FEM window ownership law thread: Any { .. }
- `result_animation_tick::result_animation_tick_stops_itself_at_the_end_of_a_once_run`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `result_animation_tick::tagged_transport_gestures_rest_where_the_clock_runs`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `set_result_animation::set_result_animation_never_touches_the_document`: sealed source requires original granted handback
- `remove_selection::remove_selection_mounted_publication_is_atomic_for_selected_and_unselected_referrers`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by
- `result_animation_tick::playback_presses_are_one_config_edit_and_never_a_history_row`: settle the typed operation: PluginLifecycleFailure { effects: RetainedCloneProgress { copied_items: 0, copied_bytes: 0, retained_capacity_by

### semio-s-artifact-fem-2d-editor-interaction :: semio_s_artifact_fem_2d_editor_interaction (unittests .rs) - 2 failing
- `editor::fem2d::interaction::gumball::tests::a_gumball_move_edited_in_history_replays_its_downstream`: sealed source requires original granted handback
- `editor::fem2d::interaction::tests::public_json_snapshot_admission_refuses_invalid_owners_and_accepts_distinct_namespaces`: separate namespaces admit: ValueError { kind: InvalidValue, message: "FEM reference graph or row is invalid", retained_progress: RetainedClo

### semio-s-artifact-fem-2d-io :: semio_s_artifact_fem_2d_io (unittests .rs) - 7 failing
- `examples::demo::tests::inference_determinism_law`: sealed source requires original granted handback
- `standards::v1::subsets::any::io::component::tests::csv_is_the_node_table_read_by_a_third_party_reader`: sealed source requires original granted handback
- `standards::v1::subsets::any::io::component::tests::txt_round_trips_the_example`: sealed source requires original granted handback
- `examples::demo::tests::inference_default_law`: sealed source requires original granted handback
- `standards::v1::subsets::any::io::component::tests::surface_corpus_exports_obj_and_stl_beside_the_neutral_surface`: owning surface contract command must declare its output path
- `standards::v1::subsets::any::io::component::tests::geometry_exports_are_real`: sealed source requires original granted handback
- `standards::v1::subsets::any::io::component::tests::json_round_trips_the_example`: sealed source requires original granted handback

### semio-s-artifact-fem-2d-model :: dialect_conformance (io\tests\dialect-conformance\.rs) - 1 failing
- `verify_protocol_bytes_against_encoded_pack`: sealed source requires original granted handback

### semio-s-artifact-fem-2d-model :: inference_default_laws (inferences\tests\default-laws\.rs) - 2 failing
- `inference_default_law`: sealed source requires original granted handback
- `bounds_default_law`: sealed source requires original granted handback

### semio-s-artifact-fem-2d-model :: original_fem_receiving (snapshot\original-receiving-tests.rs) - 2 failing
- `original_job_receives_typed_record_into_caller_slot`: assertion `left == right` failed
- `original_job_receives_and_retires_fem_rows`: assertion `left == right` failed

### semio-s-artifact-fem-2d-mutations :: semio_s_artifact_fem_2d_mutations (unittests .rs) - 1 failing
- `standards::v1::subsets::any::io::binary::mutations::tests::fem2d_document_text_round_trips_through_the_store`: sealed source requires original granted handback

### semio-s-artifact-fem-2d-results-state :: semio_s_artifact_fem_2d_results_state (unittests .rs) - 1 failing
- `transient::tests::clock_publication_and_retirement_obey_tiny_grants`: an unaliased clock publication must close: OriginalRetirementFailure { primary: Some(ValueError { kind: InvariantViolated, message: "ticket 

### semio-s-artifact-fem-2d-snapshot-tests :: semio_s_artifact_fem_2d_snapshot_tests (unittests ..\..\.rs) - 4 failing
- `native_tests::native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix`: (no panic line: aborted or killed)
- `native_tests::actual_chain_receiving_identity_and_reference_work_scales_below_threefold`: (no panic line: aborted or killed)
- `native_tests::receiving_charges_below_sixty_four_kibibytes_of_retained_capacity_per_node`: (no panic line: aborted or killed)
- `native_tests::actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work`: (no panic line: aborted or killed)

### semio-s-artifact-fem-2d-viewer :: semio_s_artifact_fem_2d_viewer (unittests .rs) - 3 failing
- `viewer::fem2d::modes::view::windows::model::tests::renders_mesh_edge_preview_for_the_default_example`: expected mesh-edge preview layers in the view scene: []
- `viewer::fem2d::component::tests::initial_snapshot_is_non_empty`: sealed source requires original granted handback
- `viewer::fem2d::modes::view::windows::model::tests::renders_a_canvas_2d_scene_for_the_default_document`: sealed source requires original granted handback

## Handoff: snapshot-tests findings (now owned by `fem-snapshot-receiving.md`)
Crate `semio-s-artifact-fem-2d-snapshot-tests`, test binary `semio_s_artifact_fem_2d_snapshot_tests`. The 3D twin has the same loop in `🧊️3d/.../🧪️tests/loads/receiver-tests.rs`.
- `load_tests::every_actual_checkpoint_prefix_and_original_grant_denial_closes_without_publication` (2D `🏅️standards/🔖️1/🪆️subsets/🌐️any/🧪️tests/loads/receiver-tests.rs:201`): in the full run it ran 70 minutes and blocked the whole `cargo test` (I killed the process). Run alone, the default 2 MB test-thread stack overflows within 4 seconds ("has overflowed its stack"). With `RUST_MIN_STACK=268435456` it ran 7m43s wall time with only 0.12 s user CPU before it was stopped: the thread is idle, not computing. So it is blocked (deadlock, lock or wait that never ends), not merely quadratic. The test calls `receive(&bytes, grant, cut)` for every `cut in 0..calls` (cancellation-callback count) plus five zero-grant variants; `receive` uses `original_owners::with_decoding_granted` and `ArtifactPackReceiving::receive_pack`. Unknown: the value of `calls`, and which `cut` blocks.
- Same crate, reported FAILED without a panic line (aborted when I killed the process): `native_tests::native_owner_receives_actual_persistent_rows_and_closes_every_cancelled_prefix`, `native_tests::actual_chain_receiving_identity_and_reference_work_scales_below_threefold`, `native_tests::receiving_charges_below_sixty_four_kibibytes_of_retained_capacity_per_node`, `native_tests::actual_owner_receives_a_ten_thousand_node_fixture_with_bounded_identity_work`. Also `original_fem_receiving` (model crate, `snapshot/original-receiving-tests.rs:29`): `original_job_receives_typed_record_into_caller_slot` and `original_job_receives_and_retires_fem_rows` fail an `assert_eq` on line 29.
- [DEBUG] prints: I had added `eprintln!("[DEBUG] calls=..")` and a per-50-cuts print to `loads/receiver-tests.rs`; the rebuild never finished (other agents' in-flight edits broke `semio-plugin-command`, `semio-plugin-window-kits-rich`, `semio-s-artifact-fem-2d-model` and others at that moment), so no [DEBUG] output exists. The prints are removed again; the file is back to its ported state.
- Tip: run the binary directly: `C:\t\2d-ab\debug\build\semio-s-artifact-fem-2d-snapshot-tests\<hash>\out\semio_s_artifact_fem_2d_snapshot_tests-<hash>.exe every_actual_checkpoint_prefix --test-threads=1` from the crate dir, with a `timeout`.

## Final state of my work
- No cargo or test processes of mine are running.
- Untouched by me and still to be verified after the sqlite fixture fix and the `&*doc` edits: groups 1 to 6 and 8 above; the last full run is `C:\t\2dw-test6.log`.
