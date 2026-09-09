# Combined Native Runtime Verification

The ticket script now supports a combined build of catalog package targets to avoid recompiling common framework dependencies for each package. Library targets and named integration targets are built separately. It preserves Cargo JSON, validates each executable against its package and target, hashes before and after each exact assertion, records individual failures, and supports cancellation through a signal or the run directory cancel.json. This is temporary ticket infrastructure; the repository runtime helper and its behavior are unchanged.

Changed file: 📜️script.ts in this ticket. Bun parsed the updated TypeScript. Execution is pending completion of native800.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded the three groups already freshly verified (WFC, Block 3D, Equation). Results: {"groups":64,"laws":376,"batches":[{"target":{"kind":"lib"},"groups":62},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while native800 remains active.

## Launch Registration

Added the combined regression command beside the existing zero-warning regression launch entry at order 411.51. Replaced the temporary native800-specific prerequisite with detection of a live unfinished native worker owned by this ticket, so the command remains usable after generated receipts are removed. Parsed both the ticket script and the launch JSONC object using Bun's TypeScript parser before guarded writes.

Files: .vscode/launch.json and this ticket's 📜️script.ts.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded WFC and Block 3D, which remain freshly verified. Equation is included again after its window-state changes. Results: {"groups":65,"laws":400,"batches":[{"target":{"kind":"lib"},"groups":63},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while native800 remains active.

## Ownership Regression Coverage

Registered three existing causal DAG tests for duplicate ownership, causal drain order, and exact identity return, plus the two current Puzzle 3D window-isolation and app-serialization boundary tests. Verified each Rust function exists and parsed the updated ticket runner before the guarded write. Runtime execution remains pending.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded WFC and Block 3D, which remain freshly verified. Equation is included again after its window-state changes. Results: {"groups":65,"laws":411,"batches":[{"target":{"kind":"lib"},"groups":63},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while the native strict worker is active.

## Ownership Regression Coverage

Registered six existing actor bridge laws and two UI list allocation-counter laws for the compiler pass 835 changes. Verified each Rust function exists and parsed the updated ticket runner before the guarded write. Runtime execution remains pending.

## Kernel Runtime Ownership

Cargo metadata confirms semio-framework-os is the host wrapper and semio-framework-os-kernel owns the directory/store tests. Extended the existing kernel library group with explicit sync and nine additional existing laws covering the boxed actor lifecycle, rejected-page retirement, directory descriptors, and canonical bounds. Updated two never-ready actor futures to return the same boxed actor type as production. Rust and TypeScript parsing passed; execution remains pending.

## UI Engine Runtime Coverage

Added the UI engine as an explicit library test target with wgpu-engine enabled. Six existing tests cover stale scene-cursor refusal, one-step close, Unicode layout publication, cancellation, partial close, and complete layout identity validation. The catalog parsed and every source function was located before the guarded write. Runtime remains pending.

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. Excluded WFC and Block 3D, which remain freshly verified. Equation is included again after its window-state changes. Results: {"groups":67,"laws":437,"batches":[{"target":{"kind":"lib"},"groups":65},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while the native strict worker is active.

## Freshness Audit

Current source modification times for the two previously verified package areas, used to decide whether their exact runtime laws need repeating:

[
  {
    "root": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine",
    "latest": [
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪶️soft/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:10.011Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪞️symmetry/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:10.008Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪜️hierarchy/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:09.999Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🩺️diag/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:09.996Z"
      },
      {
        "f": "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧵️parallel/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:52:09.993Z"
      }
    ]
  },
  {
    "root": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor",
    "latest": [
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T04:46:29.279Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs",
        "mtime": "2026-09-09T04:15:23.021Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️testkit/🦀️.rs",
        "mtime": "2026-09-09T02:15:37.747Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧫️fixtures/🎮️command-roster/🔣️.json",
        "mtime": "2026-09-09T01:54:39.096Z"
      },
      {
        "f": "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🌐️world/🫧️transient/🧪️tests/🔬️unit/🦀️.rs",
        "mtime": "2026-09-09T01:53:59.250Z"
      }
    ]
  }
]

## Scope Validation

Executed the new scope-only command and checked every package, target and feature against Cargo metadata. All catalog groups are now included. The freshness audit found WFC and Block 3D source changes after their earlier passing runs; Equation is also included after its window-state changes. Results: {"groups":69,"laws":478,"batches":[{"target":{"kind":"lib"},"groups":67},{"target":{"kind":"test","name":"native_codecs"},"groups":2}],"issues":[]}. No test compilation was started while the focused WASI strict check is active.

## Action Bus and Return-Owner Laws

Added seven existing framework tests to the shared library target: retained page admission/transfer/retirement, typed factory dispatch, saturation handback, exact prefix rejection, independent return-message byte parity, borrowed cursor cancellation, and one-byte UI-patch retirement. All selectors were located in their current Rust source and the catalog parsed before its guarded write. Runtime is pending.

## Current Dispatch Scope

The native worker selection exactly matches the root shipping-scope oracle. The current runtime catalog contains 485 laws in 69 groups, including all previously verified packages whose sources have since changed. Native compilation and combined runtime execution remain pending.

## World and Reactor Regression Coverage

Extended the infinite library target with twelve existing pool, terrain, marquee, gumball, and asset ownership assertions and added the reactor bounded close assertion. These exercise the current interfaces touched by the warning repairs. Runtime execution remains pending.

- mesh_pool_release_clears_at_zero_refcount
- terrain_writer_matches_legacy_bands_and_closes_interrupted_authority
- world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases
- world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba
- world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry
- world_gumball_update_validates_one_selected_aba_token_per_turn
- world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority
- asset_authority_rejects_request_and_byte_capacity_plus_one_before_string_ownership
- asset_response_rejects_page_plus_one_and_retires_partial_stream_one_page_per_step
- asset_decode_resume_and_stale_generation_keep_claimed_pages_until_terminal_return
- asset_unknown_length_releases_unused_aggregate_credit_at_seal
- asset_completed_cursor_advances_one_fixed_slot_per_grant_and_hands_back_exact_owner

## Launch Registration Revalidated

The current shared launch file retained the sequential regression entry but no longer contained the combined entry. Restored the combined launch alongside it with order 411.51; validated JSONC before the guarded write.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":70,"laws":506,"host":5,"shell":3,"infinite":19}.


## Synchronous Boundaries and Relay Coverage

Registered 35 existing assertions covering neutral relay traces, cancellation and panic recovery, completion subscriptions, deterministic plugin graphs, IO routing, and bounded shared-value cloning. This covers the latest warning and borrow repairs. Runtime execution remains pending.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":70,"laws":541,"host":22,"shell":3,"infinite":19}.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":72,"laws":545,"host":22,"shell":3,"infinite":19}.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":74,"laws":554,"host":22,"shell":3,"infinite":19}.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":74,"laws":555,"host":22,"shell":3,"infinite":19}.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":74,"laws":557,"host":22,"shell":3,"infinite":19}.


## Current Scope

Validated unique package targets and exact selectors after the host, shell, world, and reactor additions: {"batches":2,"groups":74,"laws":564,"host":22,"shell":3,"infinite":19}.

## Combined Runtime Dispatch

Artifact directory: 🗑️generated/native-laws/fleet-qVt8M9. Compiling 74 explicit package targets in 2 shared Cargo builds. Features are the union of the selected catalog's declared features; this verifies the combined application configuration. Each native assertion is discovered exactly and bound to its executable hash.

Completed runtime attempt: 0 exact assertions passed; 564 failed or could not run. Overall success: false.

- semio-s-artifact-fem-3d::retained_command_fixture_matches_exact_routes_and_value_codec_boundaries: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::standard_mounts_exactly_one_subset: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::subset_dialect_is_the_canonical_writer_dialect: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::subset_declares_ten_io_entries: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::writer_viewer_never_mutates: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::writer_editor_and_viewer_share_dialect: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::command_ids_are_unique_and_cover_every_row: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::every_command_round_trips_text_and_binary: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::leave_surface_text_and_binary_match_the_command_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::retained_route_dispositions_are_exact_and_exhaustive: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::block3d_world_preview_codecs_and_inverse_match_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::preview_partition_matches_language_neutral_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation3d::diff_absorb_prefers_incoming_fixture_and_preserves_generation: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation3d::generation_preview_is_one_app_transient_shared_by_two_generation_windows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation3d::preview_lifecycle_matches_language_neutral_third_party_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation2d::diff_absorb_prefers_incoming_fixture_and_preserves_generation: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation2d::generation_preview_is_one_app_transient_shared_by_two_generation_windows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation2d::preview_state_matches_language_neutral_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework::io_compose_via_chains_two_registered_hops: Error: Expected one completed Cargo executable, got 0
- semio-framework::io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry: Error: Expected one completed Cargo executable, got 0
- semio-framework::retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes: Error: Expected one completed Cargo executable, got 0
- semio-framework::production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job: Error: Expected one completed Cargo executable, got 0
- semio-framework::retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation: Error: Expected one completed Cargo executable, got 0
- semio-framework::maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix: Error: Expected one completed Cargo executable, got 0
- semio-framework::return_content_message_all_endpoints_match_independent_bytes_without_payload_parsing: Error: Expected one completed Cargo executable, got 0
- semio-framework::return_content_message_large_payload_and_cancel_keep_original_source_allocation: Error: Expected one completed Cargo executable, got 0
- semio-framework::ui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_toposorts_a_diamond: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_is_deterministic_regardless_of_input_order: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_reports_missing_dependency: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_reports_version_mismatch: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_names_every_member_of_a_cycle: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_accepts_a_self_satisfying_empty_graph: Error: Expected one completed Cargo executable, got 0
- semio-framework::dependents_returns_direct_dependents_sorted: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-contract::standard_base64_matches_the_reference_implementation: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-contract::artifact_assembly_layout_and_identity_match_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-stdio::selected_contribution_identities_are_unique_and_schema_owned: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-stdio::full_catalog_preserves_definition_codec_and_ledger_counts: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::create_binary_editor_builds_a_definition_for_the_editor_role: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::create_binary_viewer_builds_a_definition_for_the_viewer_role: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::parse_hex_dump_round_trips_a_rendered_snapshot: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::parse_hex_dump_rejects_odd_length_hex: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gif::registered_migration_runs_end_to_end_through_the_store_registry: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::mutation_rejection_messages_match_the_language_neutral_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::mutation_restore_preserves_the_language_neutral_wire_and_inverse: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::bind_node_child::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_node_name::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::create_scene::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::delete_scene::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-space-space::folds_visibility_and_members_for_this_space_into_config: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-space-space::open_artifact_relays_with_document_and_space_ids: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-space-space::open_artifact_with_relays_the_explicit_choice: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-en1990::qk_working_table_is_owned_by_the_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din18599::climate_working_data_is_owned_by_the_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din4108::set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din4108::selected_check_index_is_a_config_only_edit: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din4108::the_proof_catalog_covers_exactly_the_shared_retained_tool_ids: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-contract::render_report_falls_back_to_a_placeholder_when_nothing_was_computed: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-contract::render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-process-process3d::host_contributions_resolve_to_the_event_sourced_config_lane: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-process-process3d::process_machine_contributions_are_configuration_owned: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-process-process3d::registry_enforced_app_accepts_a_declared_operation_action: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::payload_ledger_identity_must_match_the_exact_step_context: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::retained_state_and_output_have_separate_credits_and_close_one_page_per_grant: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::retained_writer_and_reader_advance_exactly_one_page_per_opportunity: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::worker_pool_rejection_returns_exact_job_before_resume: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::canonical_bytes_match_serde_json_for_typical_documents: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::language_neutral_retained_law_ledger_is_complete: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::retained_anchor_rejects_hostile_crc_and_requires_explicit_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::retained_identity_chunk_fragment_parity_exact_boundary_and_interrupted_finish: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::identity_chunk_cursor_retains_fragment_progress_and_terminal_verification: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_places_exact_pages_and_preserves_wire_and_payload_pointer: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_rejects_duplicate_without_consuming_input_and_cancels_exact_backing: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_and_read_alias_do_not_wait_on_contended_arena: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_reports_metadata_initialization_separately_from_empty_payload_capacity: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_nine_surfaces_share_one_aggregate: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_last_reader_keeps_credit_and_typed_payload: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_cancel_and_contended_final_return_keep_exact_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_reader_pressure_refuses_then_retries_exact_slot: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_seal_transfers_output_without_detaching_root_credit: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_fixed_list_pages_counter_refuses_unaddressable_ownership_before_allocation: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_fixed_list_pages_counter_keeps_actual_failed_allocation_until_release: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::scene_paint_cursor_rejects_stale_node_without_consuming_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_hashes_match_neutral_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_frames_match_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_rejects_malformed_transfers_atomically: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_cancellation_is_atomic_and_restartable: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::server_frame_welcome_round_trips_for_every_bootstrap_variant: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::causal_insert_rejects_oversized_identity_without_losing_the_envelope_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::insert_already_applied_operation_returns_already_applied_without_erroring: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::drains_applied_envelopes_in_causal_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::duplicate_seed_returns_the_exact_unadopted_identity_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::fault_wire_projection_matches_language_neutral_serde_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::fault_inline_layout_stays_within_the_language_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_matches_neutral_vectors_and_serde_json: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_grants_bound_utf8_progress_and_cancellation_returns_the_exact_source: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_drop_rejects_live_recursive_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_rejects_capacity_and_depth_without_losing_the_source: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::gradient_checkerboard_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::scanline_decoder_matches_batch_decode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::oracle_decodes_our_encode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::our_decode_reads_oracle_encode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::our_decode_reads_oracle_palette_encode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::zlib_compress_decompress_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-framework-deflate::reads_miniz_oxide_dynamic_huffman_blocks: Error: Expected one completed Cargo executable, got 0
- semio-framework-deflate::ours_inflates_miniz_oxide_output_and_vice_versa: Error: Expected one completed Cargo executable, got 0
- semio-framework-deflate::stream_produces_the_same_bytes_as_one_shot_inflate: Error: Expected one completed Cargo executable, got 0
- semio-framework-hash::sha256_matches_nist_vectors_and_segmented_input: Error: Expected one completed Cargo executable, got 0
- semio-framework-hash::hash_bytes_agrees_with_the_blake3_oracle_across_lengths: Error: Expected one completed Cargo executable, got 0
- semio-framework-hash::hasher_agrees_with_the_blake3_oracle_for_segmented_updates: Error: Expected one completed Cargo executable, got 0
- semio-framework-trace::clock_is_monotonically_non_decreasing: Error: Expected one completed Cargo executable, got 0
- semio-framework-raster::align_bytes_per_row_pads_to_wgpu_alignment: Error: Expected one completed Cargo executable, got 0
- semio-framework-raster::scene_rasterizer_renders_expected_pixel_count: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::schema_and_language_neutral_fixture_cover_every_operation: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::valid_pack_and_dsl_are_equivalent_deterministic_paged_replies: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::workflow_pack_and_dsl_accept_every_byte_and_field_split: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::deadline_interruption_and_zero_credit_do_not_advance_any_structural_cursor: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::creates_and_lists_space_catalog_entries: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-shell::value_round_trip_matches_serde_shape: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-shell::constructed_cases_match_committed_fixtures: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-shell::fixtures_produce_expected_output: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::exclusive_selection_never_crosses_a_lifecycle_barrier: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::fixed_owner_ring_hands_back_items_and_bytes_at_the_exact_boundary: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::host_error_layout_matches_language_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_register_plugin_rejects_conflicting_io_entry_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::neutral_relay_lifecycle_traces_drive_production_machines: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::mounted_relay_stack_authority_matches_the_neutral_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::retained_pool_future_retries_saturation_once_and_terminalizes_shutdown: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::pending_guest_releases_the_only_worker_and_admits_no_duplicate_step: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::cancellation_race_admits_one_guest_cancel_and_one_terminal_outcome: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::mounted_start_panic_restores_the_instance_and_the_next_route_progresses: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::mounted_step_panic_restores_the_instance_and_terminalizes_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::cancel_panic_quarantines_instance_releases_permit_and_faults_once_on_one_worker: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::context_cancellation_failure_faults_once_quarantines_and_releases_one_worker: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::revoked_capability_cancels_only_its_own_operations_and_actor_survives: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::stale_generation_completion_is_dropped_current_generation_is_delivered: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::park_buffers_completions_and_resume_delivers_them_in_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::completion_burst_while_parked_is_bounded_not_unbounded: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_route_is_deterministic_across_load_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_route_respects_max_hops: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::id_index_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::id_serde_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::new_full_has_all_patterns_and_correct_sums: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::restrict_reduces_and_updates_caches: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::sum_over_matches_manual: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::assembly_cursor_compiler_matches_canonical_builder: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::assembly_cursor_compiler_matches_canonical_csr_order_and_multiplicity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_resume_preserves_rng_trail_and_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_restore_rejects_foreign_operation_and_topology: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_resume_preserves_preview_sequence: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cancellation_interrupts_checkpoint_and_commit_materialization_without_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::minimum_checkpoint_is_exactly_the_fixed_header_and_restores: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_restore_rejects_size_arithmetic_overflow: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::grid2d::tests::node_at_and_coords_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::grid3d::tests::node_at_and_coords_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::custom_stencil_validation_matches_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::from_coords_dedups_and_assigns_stable_first_seen_ids: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::custom_half_turn_groups_match_neutral_offset_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cube_rotation_group_has_exactly_24_elements: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cube_full_symmetry_group_has_exactly_48_elements: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::budget_exceeded_reports_partial_state: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cancellation_stops_search_and_reports_partial: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::restart_only_never_proves_unsat_on_unsatisfiable_instance: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::best_of_n_keeps_the_highest_scoring_attempt: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::best_of_n_keeps_the_lowest_scoring_attempt: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::weight_field_identity_is_all_ones: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::solver_grid2d::tests::grid_domain_constraints_enumeration_and_cancellation_match_neutral_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::solver_grid3d::tests::grid_domain_constraints_enumeration_and_cancellation_match_neutral_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::tagged_and_explicit_selectors_respect_neutral_scoped_cardinality: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::periodic_sample_solves_on_a_same_size_wrapped_grid: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::graph_view_conversion_preserves_neutral_directed_and_undirected_arcs: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::retained_publication_matches_neutral_pages_and_preserves_both_commit_streams: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::retained_publication_retries_exact_rejected_source_and_honors_zero_fuel: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::retained_publication_cancellation_closes_finished_and_partial_streams_incrementally: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::first_preview_and_continuous_gap_include_bounded_publication: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-renderer-wgpu::renderer_result_lane_vectors_decode_and_reject_unknown_tags: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::retained_window_input_preserves_owner_generation: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::retained_window_input_replacement_rejects_old_authority_and_publication: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::empty_transient_retirement_waits_for_read_release_and_matches_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::full_operation_source_rejects_generic_reducers_and_old_monolithic_shells: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_pages_wait_exact_ack_and_all_three_roots: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_reopened_request_rejects_old_started_cancel: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_partial_admission_retains_successful_roots: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_partial_error_preserves_wrapper_emission_and_retirement_counts: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::sparse_live_instances_receive_successive_round_robin_turns: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::runtime_instance_registry_has_fixed_capacity_collision_and_reuse: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cleanup_queue_saturation_preserves_detached_app_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_final_live_fence_rejects_post_await_revocation: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_is_an_exact_retained_native_close_participant: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::interactive_bridge_coalesces_preview_but_backpressures_lossless_items: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::interactive_bridge_diagnostic_ring_is_item_and_byte_bounded: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::infer_job_checkpoint_restore_matches_an_uninterrupted_run: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::artifact_inference_registry_is_order_independent_and_idempotent: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::artifact_inference_registry_rejects_any_conflicting_duplicate: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::append_chunk_over_cap_faults_instead_of_silently_truncating: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::checkpoint_binary_matches_schema_fixture_and_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::world3d_scene_fields_bind_the_domain_while_the_sun_helper_leaves_it_unset: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::contended_live_cleanup_does_not_consume_structural_stall_credit: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-cad-cad::cad_config_operation_snapshot_round_trips_and_restores_exactly: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-cad-cad::cad_config_set_contributions_round_trips: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-remodel-remodeling::raster_asset_progress_layout_matches_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-remodel-remodeling::maximum_envelope_mesh_chunks_are_bounded_replayable_and_resolve_across_threads: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-remodel-remodeling::shared_durable_chunk_admission_accepts_4k_and_rejects_overflow_and_malformed_rows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::labels_resolve_every_host_locale_and_terminology_axis: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_capacity_plus_one_refusal_preserves_exact_old_state: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_stale_owner_cannot_finish_partial_replacement: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_fixed_collections_use_the_credited_pages_and_return_identical_plus_one_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::overlap_checkpoint_resumes_exact_rng_and_sample_cursor: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::overlap_is_deterministic_across_batch_sizes: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::blocked_vortex_full_ids_and_enumeration_excludes_them: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::weighted_sample_without_replacement_edge_cases: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::same_kind_windows_compose_independently: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::app_pack_and_spr_exclude_window_transient_and_operation_fields: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-draw-drawing::retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::retained_roster_is_exact_and_exhaustive: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::p7c2_restore_stale_step_and_install_preserve_exact_replay_authority: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::sequential_fills_first_unit: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::uniform_splits_proportionally_to_capacity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::surface_incidence_matches_known_surface_normal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-flow-flow::max_semantic_config_publication_cancel_retry_and_close_use_real_grants: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-raster-raster::raster_asset_capacity_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::architect_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::architect_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::architect_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::sample_plugin_round_trips_json: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::composed_register_rows_belong_to_each_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gismap::language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gismap::gis2d_config_operation_lines_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gisterrain::retained_command_factory_matches_the_language_neutral_maximum_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gisterrain::strict_snapshot_and_aggregate_json_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gisterrain::gis3d_config_operation_lines_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_window_actions_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_shot_field_values_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::imperative_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::imperative_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::working_content_is_owned_by_each_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::render_lists_one_row_per_top_level_step: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::render_compiles_the_default_document_into_read_only_text: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::create_step_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::delete_step_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::delete_step_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::reorder_steps_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::reorder_steps_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::edit_step_params_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::edit_step_params_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::create_step_duplicate_id_fatal_never_applies: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::create_step_diff_absorb_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::document_text_round_trip_with_applied_operation: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-imperative::imperative_viewer_never_mutates: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-imperative::imperative_editor_and_viewer_share_dialect: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::presentation_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::presentation_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::presentation_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::title_cards_match_the_neutral_xml_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::from_dwg_builds_single_slide_deck_from_entity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::from_dwg_never_errors_on_empty_drawing: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::wires_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::wires_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::wires_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::renders_canvas_scene_for_the_empty_document: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::renders_canvas_scene_for_the_metabolism_example: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_retained_json_measure_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_carrier_contracts_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::render_produces_a_read_only_scene_for_the_default_document: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::linear_chain_orders_by_dependency_and_depth_by_distance_from_root: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::a_two_step_cycle_is_reported_as_not_cycle_free_but_stays_total: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::a_dangling_edge_is_ignored: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::diamond_depth_takes_the_longest_incoming_path: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::snapshot_materialization_copies_every_nested_owner_and_preserves_typed_text_arc: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::snapshot_materialization_cancellation_during_nested_metadata_reaches_terminal_emptiness: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_pdf14_page_contract_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_ink_canvas_payload_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_document_round_trips_assets_and_grid_settings: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::root_scalar_inverse_and_absorb_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::asset_inverse_law_create_replace_delete: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::block_lifecycle_inverse_law_create_delete_duplicate: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::block_reparent_and_drag_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::block_field_inverse_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::table_row_column_inverse_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::create_block_duplicate_id_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::delete_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::delete_blocks_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::rename_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::change_block_locked_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::move_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::move_block_non_finite_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::resize_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::drag_blocks_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::duplicate_block_missing_source_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::insert_table_row_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::remove_table_row_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::edit_block_text_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::replace_asset_payload_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::create_asset_duplicate_id_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::delete_asset_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_pdf_page_collection_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_inspection_summary_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::background_drawing_and_referenced_model_round_trip_through_text_and_binary: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::absent_composition_slots_round_trip_as_none: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::typed_document_json_matches_serde_and_every_write_is_credit_bounded: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::create_page_obeys_the_inverse_and_absorb_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::move_frame_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::rename_layout_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::delete_page_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::reorder_pages_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::update_page_margins_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::change_frame_fill_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::edit_story_and_create_link_obey_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::create_frame_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::delete_frame_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::move_frame_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::reorder_pages_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::rename_page_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::change_page_height_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::edit_story_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::create_page_duplicate_id_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::admitted_maximum_and_production_grant_make_bounded_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::bounded_mesh_plus_one_fault_retains_the_exact_domain_for_cursor_close: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::vector_layer_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::process_owner_inventory_admits_exact_maximum_and_returns_exact_credit: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::admitted_maximum_and_production_grant_make_bounded_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::procedural_payload_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::procedural_parameter_controls_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::procedural_actor_descriptor_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::module_app_declares_window_kinds: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::module_manifest_contributes_building_component: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::pdf_page_text_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_into_pdf_preserves_text_and_page_size: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_completion_rejection_retires_child_before_command_without_reemission: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::bounded_text_admission_preserves_rejected_job_state_and_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::bounded_open_document_admission_preserves_maximum_plus_one_job_state_and_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::bounded_host_load_and_engagement_admission_reject_plus_one_without_consuming_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_artifact_store_preparation_is_exact_bounded_and_reversible: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::retained_wire_decoder_and_third_party_serde_have_command_parity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_window_state_mutations_are_exact_reversible_and_codec_stable: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::boxed_dsl_fields_match_neutral_values_and_serde: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::boxed_dsl_operation_matches_unboxed_text_and_binary: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::document_codec_of_round_trips_dsl_and_pack_and_edit_text: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::space_history_verbs_match_the_language_neutral_contract: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::str_eq_matches_std_partial_eq: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::retained_group_history_switches_every_direct_reader_at_one_decision: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::derive_mutations_wires_complete_leaf_and_atomic_registration: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_codecs_and_descriptors: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_checked_add_and_structural_diff: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_mixed_inverse_stored_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_exact_i64_codecs: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::ordered_diff_preserves_step_admission_and_associativity: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::ordered_counter_algebra_matches_exact_neutral_boundaries: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::minimum_add_inverse_obeys_store_reverse_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::every_path_mount_in_this_glue_resolves_to_an_existing_file: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::turn_fault_and_cancel_retain_then_close_one_owner_per_grant: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::idle_then_late_send_upgrades_the_host_retained_runner_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::detach_while_pending_retains_future_then_cancel_closes_one_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::registered_rejected_pages_obey_zero_short_and_exact_grants: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::unadmitted_rejected_pages_obey_zero_short_and_exact_grants: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::directory_command_receipt_v1_matches_language_neutral_vectors_and_rejects_hostiles: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::document_descriptor_matches_the_language_neutral_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::canonical_sealer_checkpoint_maximum_accepts_exact_framing_and_identity_overhead_only: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::pack_round_trip_turn_result: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_progress_fixed_capacity_and_aba_admission_fail_closed: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_progress_commit_validates_live_authority_and_rejected_close_is_incremental: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_invokes_exactly_one_step_per_turn: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_preserves_checkpoint_state_and_applied_progress: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_cancellation_is_terminal_and_skips_the_job: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_rejects_stale_commit_before_work_or_publication: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_rejects_replayed_preview_identity_before_work: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_rejects_a_preview_without_exactly_one_sequence_advance: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::chunked_read_write_scan_and_modified_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::resident_memory_observation_does_not_spawn_a_process: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_latest_wins_collapses_older_pending_value: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_coalesced_collapses_same_key_but_queues_distinct_keys: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_ring_overwrites_oldest_by_item_and_byte_bounds: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_payload_bytes_are_enforced_for_every_queueing_policy: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::language_neutral_mutations_match_json_oracle_and_restore_base: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_keeps_an_already_directed_graph_directed::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_keeps_an_already_directed_graph_directed::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_keeps_an_already_directed_graph_directed::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_restates_the_unset_algorithm_and_its_absent_seed::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_restates_the_unset_algorithm_and_its_absent_seed::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_restates_the_unset_algorithm_and_its_absent_seed::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_graph::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_graph::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_graph::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_point_cloud::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_point_cloud::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_point_cloud::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_seeds_the_empty_cloud_with_its_first_point::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_seeds_the_empty_cloud_with_its_first_point::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_seeds_the_empty_cloud_with_its_first_point::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_raises_the_leading_coefficient_to_three_halves::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_raises_the_leading_coefficient_to_three_halves::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_raises_the_leading_coefficient_to_three_halves::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::the_composed_child_triple_is_never_re_minted: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::math_config_dsl_round_trips: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::config_operation_set_camera_diff_writes_the_targeted_field: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::config_operation_set_camera_round_trips: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::retained_semantic_maxima_accept_exact_and_reject_maximum_plus_one: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::retained_interruption_replay_aba_cancel_and_repeated_close_are_exact: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::retained_maximum_microturns_stay_below_eight_milliseconds: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_resumable_matches_neutral_results_and_single_mutation_publication: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_preparation_layout_matches_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_cancelled_preparation_closes_while_source_scene_remains_live: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_preparation_rejects_oversized_node_and_edge_before_clone: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_output_admission_rejects_oversized_table_before_publication: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::editor::jack::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::editor::jack::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-rewriting::editor::rewriting::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-rewriting::editor::rewriting::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-rewriting::trinity_lod_scale_json_lists_all_six_lods: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::board_fixture_json_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::icon_codec_resolves_metabolism_shortcode_to_themed_svg: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::wheel_plan_matches_direct_and_rejects_stale_interaction: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::retained_draw_rebuild_keeps_url_backed_asset_authority: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::mesh_pool_release_clears_at_zero_refcount: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::terrain_writer_matches_legacy_bands_and_closes_interrupted_authority: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_gumball_update_validates_one_selected_aba_token_per_turn: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_authority_rejects_request_and_byte_capacity_plus_one_before_string_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_response_rejects_page_plus_one_and_retires_partial_stream_one_page_per_step: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_decode_resume_and_stale_generation_keep_claimed_pages_until_terminal_return: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_unknown_length_releases_unused_aggregate_credit_at_seal: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_completed_cursor_advances_one_fixed_slot_per_grant_and_hands_back_exact_owner: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-playbook-playbook::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-playbook-playbook::render_builder_emits_playbook_list_component_scene: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::forms_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::vector_replacement_boundaries_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::semantic_question_controls_match_the_language_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::renders_blueprint_builder_cards: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::large_unrelated_config_and_existing_vector_stay_under_one_bounded_slice: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::vector_growth_writes_at_most_sixty_four_components_per_slice: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::missing_non_array_and_malformed_targets_keep_best_effort_semantics: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::scalar_option_and_object_shapes_stay_intact: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::bounded_chunk_values_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::graph_host_sync_from_scene_pack_decodes_pack_shell: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_render_mode_parse: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_vector_style_parse: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::sync_map_json_keeps_position_labels: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::sync_map_json_parses_rich_position_metadata: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::owned_protobuf_decodes_layer_properties_and_point_geometry: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::fixture_linestrings_split_at_moveto: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_polyline_intersects_polygon_detects_crossing_edge: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::pointer_up_emits_camera_after_middle_button_pan: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::interaction_plan_matches_direct_wheel_and_pan_semantics: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::build_vector_scene_respects_render_mode: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::build_vector_scene_respects_vector_style: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::node_record_to_spec_builds_app_instance_kind: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::char_boundary_helpers_handle_multibyte: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::offset_line_col_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::sync_from_scene_json_sets_and_clears_hover_range: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::sync_from_scene_json_applies_all_optional_fields: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-imperative-effect::bundle_contributes_core_module_for_imperative_play: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_matches_serde_and_shares_unchanged_ordered_roots: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_cancellation_and_invalid_projection_preserve_root_until_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_nonterminal_drop_is_guarded_without_destroying_root: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_rejects_root_retirement_overgrant_and_closes_factory_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-infinite-dag::direct_intrinsic_serde_and_selection_are_lossless: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-infinite-dag::dag_document_dsl_round_trips_every_node_kind: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-infinite-dag::dag_document_dsl_round_trips_the_demo_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-playbook-playbook::ordered_document_fixture_matches_serde_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::utility_registry_declares_utilities: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_force_graph_top_level_locked_node_ids_pins: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_force_graph_wraps_flat_options: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::edge_handle_snap_sets_circle_handle_angles_on_center_line: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_force_graph_with_snap_sets_handle_angles: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::force_graph_accepts_logical_nodes_without_xy: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_normal_mode_node_id_edges_stacks_by_depth: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_stacks_by_depth: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_pins_locked_root_coordinates: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_hierarchical_tree_nested_locked_node_ids_pins: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_right_places_children_larger_x_than_root: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_upwards_places_children_smaller_y_than_root: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_rejects_unknown_direction: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_rejects_unknown_mode: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::context_menu_is_grouped_and_keeps_delete_selection_last: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::window_engagements_cover_both_windows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::set_active_utility_emits_no_ops_and_no_history_entry: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::engagements_expose_no_utility_switch_options_for_either_window: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-gis::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-gis::gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-vcs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-vcs::vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind: Error: Expected one completed Cargo executable, got 0

## Combined Runtime Dispatch

Artifact directory: 🗑️generated/native-laws/fleet-WQUChB. Compiling 74 explicit package targets in 2 shared Cargo builds. Features are the union of the selected catalog's declared features; this verifies the combined application configuration. Each native assertion is discovered exactly and bound to its executable hash.

Completed runtime attempt: 2 exact assertions passed; 562 failed or could not run. Overall success: false.

- semio-s-artifact-fem-3d::retained_command_fixture_matches_exact_routes_and_value_codec_boundaries: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::standard_mounts_exactly_one_subset: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::subset_dialect_is_the_canonical_writer_dialect: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::subset_declares_ten_io_entries: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::writer_viewer_never_mutates: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-writer::writer_editor_and_viewer_share_dialect: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::command_ids_are_unique_and_cover_every_row: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::every_command_round_trips_text_and_binary: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::leave_surface_text_and_binary_match_the_command_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::retained_route_dispositions_are_exact_and_exhaustive: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::block3d_world_preview_codecs_and_inverse_match_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-block-3d::preview_partition_matches_language_neutral_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation3d::diff_absorb_prefers_incoming_fixture_and_preserves_generation: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation3d::generation_preview_is_one_app_transient_shared_by_two_generation_windows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation3d::preview_lifecycle_matches_language_neutral_third_party_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation2d::diff_absorb_prefers_incoming_fixture_and_preserves_generation: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation2d::generation_preview_is_one_app_transient_shared_by_two_generation_windows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-generation2d::preview_state_matches_language_neutral_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework::io_compose_via_chains_two_registered_hops: Error: Expected one completed Cargo executable, got 0
- semio-framework::io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry: Error: Expected one completed Cargo executable, got 0
- semio-framework::retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes: Error: Expected one completed Cargo executable, got 0
- semio-framework::production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job: Error: Expected one completed Cargo executable, got 0
- semio-framework::retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation: Error: Expected one completed Cargo executable, got 0
- semio-framework::maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix: Error: Expected one completed Cargo executable, got 0
- semio-framework::return_content_message_all_endpoints_match_independent_bytes_without_payload_parsing: Error: Expected one completed Cargo executable, got 0
- semio-framework::return_content_message_large_payload_and_cancel_keep_original_source_allocation: Error: Expected one completed Cargo executable, got 0
- semio-framework::ui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_toposorts_a_diamond: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_is_deterministic_regardless_of_input_order: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_reports_missing_dependency: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_reports_version_mismatch: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_names_every_member_of_a_cycle: Error: Expected one completed Cargo executable, got 0
- semio-framework::resolve_load_order_accepts_a_self_satisfying_empty_graph: Error: Expected one completed Cargo executable, got 0
- semio-framework::dependents_returns_direct_dependents_sorted: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-contract::standard_base64_matches_the_reference_implementation: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-contract::artifact_assembly_layout_and_identity_match_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-stdio::selected_contribution_identities_are_unique_and_schema_owned: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-stdio::full_catalog_preserves_definition_codec_and_ledger_counts: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::create_binary_editor_builds_a_definition_for_the_editor_role: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::create_binary_viewer_builds_a_definition_for_the_viewer_role: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::parse_hex_dump_round_trips_a_rendered_snapshot: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-binary::parse_hex_dump_rejects_odd_length_hex: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gif::registered_migration_runs_end_to_end_through_the_store_registry: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::mutation_rejection_messages_match_the_language_neutral_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::mutation_restore_preserves_the_language_neutral_wire_and_inverse: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::bind_node_child::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::unbind_node_child::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_node_name::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::create_scene::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-stdio-gltf::standards::v2_0::subsets::any::schema::mutations::delete_scene::contract::canonical_vectors_execute_direct_mutation_and_codec_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-space-space::folds_visibility_and_members_for_this_space_into_config: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-space-space::open_artifact_relays_with_document_and_space_ids: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-space-space::open_artifact_with_relays_the_explicit_choice: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-en1990::qk_working_table_is_owned_by_the_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din18599::climate_working_data_is_owned_by_the_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din4108::set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din4108::selected_check_index_is_a_config_only_edit: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-din4108::the_proof_catalog_covers_exactly_the_shared_retained_tool_ids: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-contract::render_report_falls_back_to_a_placeholder_when_nothing_was_computed: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-norm-contract::render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-runtime::round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-process-process3d::host_contributions_resolve_to_the_event_sourced_config_lane: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-process-process3d::process_machine_contributions_are_configuration_owned: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-process-process3d::registry_enforced_app_accepts_a_declared_operation_action: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::payload_ledger_identity_must_match_the_exact_step_context: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::retained_state_and_output_have_separate_credits_and_close_one_page_per_grant: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::retained_writer_and_reader_advance_exactly_one_page_per_opportunity: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::worker_pool_rejection_returns_exact_job_before_resume: Error: Expected one completed Cargo executable, got 0
- semio-framework-job::worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::canonical_bytes_match_serde_json_for_typical_documents: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::language_neutral_retained_law_ledger_is_complete: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::retained_anchor_rejects_hostile_crc_and_requires_explicit_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::retained_identity_chunk_fragment_parity_exact_boundary_and_interrupted_finish: Error: Expected one completed Cargo executable, got 0
- semio-framework-pack::identity_chunk_cursor_retains_fragment_progress_and_terminal_verification: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_places_exact_pages_and_preserves_wire_and_payload_pointer: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_rejects_duplicate_without_consuming_input_and_cancels_exact_backing: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_and_read_alias_do_not_wait_on_contended_arena: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_assembly_reports_metadata_initialization_separately_from_empty_payload_capacity: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_nine_surfaces_share_one_aggregate: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_last_reader_keeps_credit_and_typed_payload: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_cancel_and_contended_final_return_keep_exact_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_reader_pressure_refuses_then_retries_exact_slot: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_document_root_permit_seal_transfers_output_without_detaching_root_credit: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_fixed_list_pages_counter_refuses_unaddressable_ownership_before_allocation: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui-contract::retained_fixed_list_pages_counter_keeps_actual_failed_allocation_until_release: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::scene_paint_cursor_rejects_stale_node_without_consuming_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-ui::mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_hashes_match_neutral_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_frames_match_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_rejects_malformed_transfers_atomically: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::artifact_bootstrap_cancellation_is_atomic_and_restartable: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::server_frame_welcome_round_trips_for_every_bootstrap_variant: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::causal_insert_rejects_oversized_identity_without_losing_the_envelope_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::insert_already_applied_operation_returns_already_applied_without_erroring: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::drains_applied_envelopes_in_causal_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::duplicate_seed_returns_the_exact_unadopted_identity_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::fault_wire_projection_matches_language_neutral_serde_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::fault_inline_layout_stays_within_the_language_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_matches_neutral_vectors_and_serde_json: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_grants_bound_utf8_progress_and_cancellation_returns_the_exact_source: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_drop_rejects_live_recursive_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-replication::shared_value_clone_rejects_capacity_and_depth_without_losing_the_source: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::gradient_checkerboard_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::scanline_decoder_matches_batch_decode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::oracle_decodes_our_encode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::our_decode_reads_oracle_encode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::our_decode_reads_oracle_palette_encode: Error: Expected one completed Cargo executable, got 0
- semio-framework-pixels::zlib_compress_decompress_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-framework-deflate::reads_miniz_oxide_dynamic_huffman_blocks: Error: Expected one completed Cargo executable, got 0
- semio-framework-deflate::ours_inflates_miniz_oxide_output_and_vice_versa: Error: Expected one completed Cargo executable, got 0
- semio-framework-deflate::stream_produces_the_same_bytes_as_one_shot_inflate: Error: Expected one completed Cargo executable, got 0
- semio-framework-hash::sha256_matches_nist_vectors_and_segmented_input: Error: Expected one completed Cargo executable, got 0
- semio-framework-hash::hash_bytes_agrees_with_the_blake3_oracle_across_lengths: Error: Expected one completed Cargo executable, got 0
- semio-framework-hash::hasher_agrees_with_the_blake3_oracle_for_segmented_updates: Error: Expected one completed Cargo executable, got 0
- semio-framework-trace::clock_is_monotonically_non_decreasing: Error: Expected one completed Cargo executable, got 0
- semio-framework-raster::align_bytes_per_row_pads_to_wgpu_alignment: Error: Expected one completed Cargo executable, got 0
- semio-framework-raster::scene_rasterizer_renders_expected_pixel_count: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::schema_and_language_neutral_fixture_cover_every_operation: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::valid_pack_and_dsl_are_equivalent_deterministic_paged_replies: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::workflow_pack_and_dsl_accept_every_byte_and_field_split: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::codec_abi::tests::deadline_interruption_and_zero_credit_do_not_advance_any_structural_cursor: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping: Error: Expected one completed Cargo executable, got 0
- semio-framework-os::creates_and_lists_space_catalog_entries: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-shell::value_round_trip_matches_serde_shape: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-shell::constructed_cases_match_committed_fixtures: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-shell::fixtures_produce_expected_output: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::exclusive_selection_never_crosses_a_lifecycle_barrier: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::fixed_owner_ring_hands_back_items_and_bytes_at_the_exact_boundary: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::host_error_layout_matches_language_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_register_plugin_rejects_conflicting_io_entry_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::neutral_relay_lifecycle_traces_drive_production_machines: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::mounted_relay_stack_authority_matches_the_neutral_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::retained_pool_future_retries_saturation_once_and_terminalizes_shutdown: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::pending_guest_releases_the_only_worker_and_admits_no_duplicate_step: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::cancellation_race_admits_one_guest_cancel_and_one_terminal_outcome: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::mounted_start_panic_restores_the_instance_and_the_next_route_progresses: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::mounted_step_panic_restores_the_instance_and_terminalizes_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::cancel_panic_quarantines_instance_releases_permit_and_faults_once_on_one_worker: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::context_cancellation_failure_faults_once_quarantines_and_releases_one_worker: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::revoked_capability_cancels_only_its_own_operations_and_actor_survives: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::stale_generation_completion_is_dropped_current_generation_is_delivered: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::park_buffers_completions_and_resume_delivers_them_in_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::completion_burst_while_parked_is_bounded_not_unbounded: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_route_is_deterministic_across_load_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin-host::io_router_route_respects_max_hops: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::id_index_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::id_serde_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::new_full_has_all_patterns_and_correct_sums: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::restrict_reduces_and_updates_caches: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::sum_over_matches_manual: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::assembly_cursor_compiler_matches_canonical_builder: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::assembly_cursor_compiler_matches_canonical_csr_order_and_multiplicity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_resume_preserves_rng_trail_and_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_restore_rejects_foreign_operation_and_topology: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_resume_preserves_preview_sequence: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cancellation_interrupts_checkpoint_and_commit_materialization_without_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::minimum_checkpoint_is_exactly_the_fixed_header_and_restores: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::checkpoint_restore_rejects_size_arithmetic_overflow: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::grid2d::tests::node_at_and_coords_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::grid3d::tests::node_at_and_coords_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::custom_stencil_validation_matches_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::from_coords_dedups_and_assigns_stable_first_seen_ids: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::custom_half_turn_groups_match_neutral_offset_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cube_rotation_group_has_exactly_24_elements: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cube_full_symmetry_group_has_exactly_48_elements: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::budget_exceeded_reports_partial_state: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::cancellation_stops_search_and_reports_partial: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::restart_only_never_proves_unsat_on_unsatisfiable_instance: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::best_of_n_keeps_the_highest_scoring_attempt: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::best_of_n_keeps_the_lowest_scoring_attempt: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::weight_field_identity_is_all_ones: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::solver_grid2d::tests::grid_domain_constraints_enumeration_and_cancellation_match_neutral_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::wfc_engine::solver_grid3d::tests::grid_domain_constraints_enumeration_and_cancellation_match_neutral_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::tagged_and_explicit_selectors_respect_neutral_scoped_cardinality: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::periodic_sample_solves_on_a_same_size_wrapped_grid: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::graph_view_conversion_preserves_neutral_directed_and_undirected_arcs: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::retained_publication_matches_neutral_pages_and_preserves_both_commit_streams: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::retained_publication_retries_exact_rejected_source_and_honors_zero_fuel: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::retained_publication_cancellation_closes_finished_and_partial_streams_incrementally: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-procedural-assembly::first_preview_and_continuous_gap_include_bounded_publication: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-renderer-wgpu::renderer_result_lane_vectors_decode_and_reject_unknown_tags: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::retained_window_input_preserves_owner_generation: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::retained_window_input_replacement_rejects_old_authority_and_publication: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::empty_transient_retirement_waits_for_read_release_and_matches_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::full_operation_source_rejects_generic_reducers_and_old_monolithic_shells: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_pages_wait_exact_ack_and_all_three_roots: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_reopened_request_rejects_old_started_cancel: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_partial_admission_retains_successful_roots: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::local_interaction_live_partial_error_preserves_wrapper_emission_and_retirement_counts: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::sparse_live_instances_receive_successive_round_robin_turns: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::runtime_instance_registry_has_fixed_capacity_collision_and_reuse: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cleanup_queue_saturation_preserves_detached_app_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_final_live_fence_rejects_post_await_revocation: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_ingress_is_an_exact_retained_native_close_participant: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::interactive_bridge_coalesces_preview_but_backpressures_lossless_items: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::interactive_bridge_diagnostic_ring_is_item_and_byte_bounded: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::infer_job_checkpoint_restore_matches_an_uninterrupted_run: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::artifact_inference_registry_is_order_independent_and_idempotent: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::artifact_inference_registry_rejects_any_conflicting_duplicate: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::append_chunk_over_cap_faults_instead_of_silently_truncating: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::checkpoint_binary_matches_schema_fixture_and_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::world3d_scene_fields_bind_the_domain_while_the_sun_helper_leaves_it_unset: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::contended_live_cleanup_does_not_consume_structural_stall_credit: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift: Error: Expected one completed Cargo executable, got 0
- semio-framework-plugin::reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-cad-cad::cad_config_operation_snapshot_round_trips_and_restores_exactly: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-cad-cad::cad_config_set_contributions_round_trips: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-remodel-remodeling::raster_asset_progress_layout_matches_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-remodel-remodeling::maximum_envelope_mesh_chunks_are_bounded_replayable_and_resolve_across_threads: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-remodel-remodeling::shared_durable_chunk_admission_accepts_4k_and_rejects_overflow_and_malformed_rows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::labels_resolve_every_host_locale_and_terminology_axis: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_capacity_plus_one_refusal_preserves_exact_old_state: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_stale_owner_cannot_finish_partial_replacement: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::spatial_fixed_collections_use_the_credited_pages_and_return_identical_plus_one_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::overlap_checkpoint_resumes_exact_rng_and_sample_cursor: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::overlap_is_deterministic_across_batch_sizes: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::blocked_vortex_full_ids_and_enumeration_excludes_them: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::weighted_sample_without_replacement_edge_cases: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::same_kind_windows_compose_independently: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-3d::app_pack_and_spr_exclude_window_transient_and_operation_fields: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-draw-drawing::retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::retained_roster_is_exact_and_exhaustive: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::p7c2_restore_stale_step_and_install_preserve_exact_replay_authority: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::sequential_fills_first_unit: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::uniform_splits_proportionally_to_capacity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-energy-model::surface_incidence_matches_known_surface_normal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-flow-flow::max_semantic_config_publication_cancel_retry_and_close_use_real_grants: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-raster-raster::raster_asset_capacity_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::architect_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::architect_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::architect_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::sample_plugin_round_trips_json: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-architect-program::composed_register_rows_belong_to_each_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gismap::language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gismap::gis2d_config_operation_lines_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gisterrain::retained_command_factory_matches_the_language_neutral_maximum_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gisterrain::strict_snapshot_and_aggregate_json_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-gis-gisterrain::gis3d_config_operation_lines_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_window_actions_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-shooting-shooting::shooting_shot_field_values_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::imperative_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::imperative_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::working_content_is_owned_by_each_exact_child: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::render_lists_one_row_per_top_level_step: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::render_compiles_the_default_document_into_read_only_text: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::create_step_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::delete_step_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::delete_step_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::reorder_steps_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::reorder_steps_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::edit_step_params_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::edit_step_params_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::create_step_duplicate_id_fatal_never_applies: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::create_step_diff_absorb_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-imperative-procedure::document_text_round_trip_with_applied_operation: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-imperative::imperative_viewer_never_mutates: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-imperative::imperative_editor_and_viewer_share_dialect: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::presentation_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::presentation_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::presentation_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::title_cards_match_the_neutral_xml_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::from_dwg_builds_single_slide_deck_from_entity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-animate-presentation::from_dwg_never_errors_on_empty_drawing: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::wires_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::wires_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::wires_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::renders_canvas_scene_for_the_empty_document: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-reasoning-wires::renders_canvas_scene_for_the_metabolism_example: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_retained_json_measure_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_carrier_contracts_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::sequence_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::render_produces_a_read_only_scene_for_the_default_document: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::linear_chain_orders_by_dependency_and_depth_by_distance_from_root: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::a_two_step_cycle_is_reported_as_not_cycle_free_but_stays_total: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::a_dangling_edge_is_ignored: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::diamond_depth_takes_the_longest_incoming_path: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-sequence-sequence::standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::snapshot_materialization_copies_every_nested_owner_and_preserves_typed_text_arc: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::snapshot_materialization_cancellation_during_nested_metadata_reaches_terminal_emptiness: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_pdf14_page_contract_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_semantic_panels_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_ink_canvas_payload_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::note_document_round_trips_assets_and_grid_settings: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::root_scalar_inverse_and_absorb_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::asset_inverse_law_create_replace_delete: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::block_lifecycle_inverse_law_create_delete_duplicate: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::block_reparent_and_drag_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::block_field_inverse_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::table_row_column_inverse_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::create_block_duplicate_id_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::delete_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::delete_blocks_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::rename_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::change_block_locked_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::move_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::move_block_non_finite_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::resize_block_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::drag_blocks_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::duplicate_block_missing_source_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::insert_table_row_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::remove_table_row_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::edit_block_text_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::replace_asset_payload_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::create_asset_duplicate_id_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-note-note::delete_asset_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_pdf_page_collection_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::layout_inspection_summary_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::background_drawing_and_referenced_model_round_trip_through_text_and_binary: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::absent_composition_slots_round_trip_as_none: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::typed_document_json_matches_serde_and_every_write_is_credit_bounded: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::create_page_obeys_the_inverse_and_absorb_laws: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::move_frame_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::rename_layout_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::delete_page_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::reorder_pages_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::update_page_margins_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::change_frame_fill_obeys_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::edit_story_and_create_link_obey_the_inverse_law: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::create_frame_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::delete_frame_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::move_frame_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::reorder_pages_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::rename_page_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::change_page_height_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::edit_story_missing_target_is_error: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::create_page_duplicate_id_is_fatal: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-layout-layout::admitted_maximum_and_production_grant_make_bounded_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::bounded_mesh_plus_one_fault_retains_the_exact_domain_for_cursor_close: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::mesh_mounted_classification_indexes_admit_maximum_reject_plus_one_and_close_exactly: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::vector_layer_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::process_owner_inventory_admits_exact_maximum_and_returns_exact_credit: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-fem-2d::admitted_maximum_and_production_grant_make_bounded_progress: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::procedural_payload_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::procedural_parameter_controls_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::procedural_actor_descriptor_matches_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::module_app_declares_window_kinds: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-playbook-procedural::module_manifest_contributes_building_component: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::pdf_page_text_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_into_pdf_preserves_text_and_page_size: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_completion_rejection_retires_child_before_command_without_reemission: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::bounded_text_admission_preserves_rejected_job_state_and_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::bounded_open_document_admission_preserves_maximum_plus_one_job_state_and_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::bounded_host_load_and_engagement_admission_reject_plus_one_without_consuming_owners: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_artifact_store_preparation_is_exact_bounded_and_reversible: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::retained_wire_decoder_and_third_party_serde_have_command_parity: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-writer-writer::writer_window_state_mutations_are_exact_reversible_and_codec_stable: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::boxed_dsl_fields_match_neutral_values_and_serde: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::boxed_dsl_operation_matches_unboxed_text_and_binary: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::document_codec_of_round_trips_dsl_and_pack_and_edit_text: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::space_history_verbs_match_the_language_neutral_contract: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::str_eq_matches_std_partial_eq: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::retained_group_history_switches_every_direct_reader_at_one_decision: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::derive_mutations_wires_complete_leaf_and_atomic_registration: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_codecs_and_descriptors: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_checked_add_and_structural_diff: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_mixed_inverse_stored_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::counter_fixture_exact_i64_codecs: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::ordered_diff_preserves_step_admission_and_associativity: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::ordered_counter_algebra_matches_exact_neutral_boundaries: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::minimum_add_inverse_obeys_store_reverse_order: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::every_path_mount_in_this_glue_resolves_to_an_existing_file: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::turn_fault_and_cancel_retain_then_close_one_owner_per_grant: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::idle_then_late_send_upgrades_the_host_retained_runner_once: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::detach_while_pending_retains_future_then_cancel_closes_one_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::registered_rejected_pages_obey_zero_short_and_exact_grants: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::unadmitted_rejected_pages_obey_zero_short_and_exact_grants: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::directory_command_receipt_v1_matches_language_neutral_vectors_and_rejects_hostiles: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::document_descriptor_matches_the_language_neutral_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-kernel::canonical_sealer_checkpoint_maximum_accepts_exact_framing_and_identity_overhead_only: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::pack_round_trip_turn_result: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_progress_fixed_capacity_and_aba_admission_fail_closed: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_progress_commit_validates_live_authority_and_rejected_close_is_incremental: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_invokes_exactly_one_step_per_turn: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_preserves_checkpoint_state_and_applied_progress: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_cancellation_is_terminal_and_skips_the_job: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_rejects_stale_commit_before_work_or_publication: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_rejects_replayed_preview_identity_before_work: Error: Expected one completed Cargo executable, got 0
- semio-framework-actor::job_bridge_rejects_a_preview_without_exactly_one_sequence_advance: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::chunked_read_write_scan_and_modified_round_trip: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::resident_memory_observation_does_not_spawn_a_process: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_latest_wins_collapses_older_pending_value: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_coalesced_collapses_same_key_but_queues_distinct_keys: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_ring_overwrites_oldest_by_item_and_byte_bounds: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_payload_bytes_are_enforced_for_every_queueing_policy: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-services::event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::language_neutral_mutations_match_json_oracle_and_restore_base: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_keeps_an_already_directed_graph_directed::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_keeps_an_already_directed_graph_directed::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_keeps_an_already_directed_graph_directed::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_restates_the_unset_algorithm_and_its_absent_seed::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_restates_the_unset_algorithm_and_its_absent_seed::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_restates_the_unset_algorithm_and_its_absent_seed::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_graph::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_graph::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_graph::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_point_cloud::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_point_cloud::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_replays_the_identical_empty_point_cloud::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_seeds_the_empty_cloud_with_its_first_point::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_seeds_the_empty_cloud_with_its_first_point::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_seeds_the_empty_cloud_with_its_first_point::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_raises_the_leading_coefficient_to_three_halves::produces_committed_diff: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_raises_the_leading_coefficient_to_three_halves::committed_diff_is_canonical: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::tests_raises_the_leading_coefficient_to_three_halves::committed_diff_applies_to_after: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::the_composed_child_triple_is_never_re_minted: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::math_config_dsl_round_trips: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::config_operation_set_camera_diff_writes_the_targeted_field: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::config_operation_set_camera_round_trips: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::retained_semantic_maxima_accept_exact_and_reject_maximum_plus_one: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::retained_interruption_replay_aba_cancel_and_repeated_close_are_exact: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-mathematical-equation::retained_maximum_microturns_stay_below_eight_milliseconds: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_resumable_matches_neutral_results_and_single_mutation_publication: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_preparation_layout_matches_neutral_budget: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_cancelled_preparation_closes_while_source_scene_remains_live: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_preparation_rejects_oversized_node_and_edge_before_clone: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::query_ownership_output_admission_rejects_oversized_table_before_publication: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::editor::jack::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-jack::editor::jack::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-rewriting::editor::rewriting::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-rewriting::editor::rewriting::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-trinity-rewriting::trinity_lod_scale_json_lists_all_six_lods: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::board_fixture_json_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::icon_codec_resolves_metabolism_shortcode_to_themed_svg: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::wheel_plan_matches_direct_and_rejects_stale_interaction: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::retained_draw_rebuild_keeps_url_backed_asset_authority: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::mesh_pool_release_clears_at_zero_refcount: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::terrain_writer_matches_legacy_bands_and_closes_interrupted_authority: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_gumball_update_validates_one_selected_aba_token_per_turn: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_authority_rejects_request_and_byte_capacity_plus_one_before_string_ownership: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_response_rejects_page_plus_one_and_retires_partial_stream_one_page_per_step: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_decode_resume_and_stale_generation_keep_claimed_pages_until_terminal_return: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_unknown_length_releases_unused_aggregate_credit_at_seal: Error: Expected one completed Cargo executable, got 0
- semio-framework-os-infinite::asset_completed_cursor_advances_one_fixed_slot_per_grant_and_hands_back_exact_owner: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-playbook-playbook::configuration_and_presence_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-playbook-playbook::render_builder_emits_playbook_list_component_scene: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::forms_configuration_contract_vectors_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::vector_replacement_boundaries_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::semantic_question_controls_match_the_language_neutral_vectors: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::renders_blueprint_builder_cards: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::large_unrelated_config_and_existing_vector_stay_under_one_bounded_slice: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::vector_growth_writes_at_most_sixty_four_components_per_slice: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::missing_non_array_and_malformed_targets_keep_best_effort_semantics: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::scalar_option_and_object_shapes_stay_intact: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-forms-forms::bounded_chunk_values_match_the_json_oracle: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::graph_host_sync_from_scene_pack_decodes_pack_shell: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_render_mode_parse: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_vector_style_parse: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::sync_map_json_keeps_position_labels: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::sync_map_json_parses_rich_position_metadata: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::owned_protobuf_decodes_layer_properties_and_point_geometry: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::fixture_linestrings_split_at_moveto: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::map_polyline_intersects_polygon_detects_crossing_edge: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::pointer_up_emits_camera_after_middle_button_pan: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::interaction_plan_matches_direct_wheel_and_pan_semantics: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::build_vector_scene_respects_render_mode: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::build_vector_scene_respects_vector_style: Error: Expected one completed Cargo executable, got 0
- semio-framework-surface::node_record_to_spec_builds_app_instance_kind: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::char_boundary_helpers_handle_multibyte: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::offset_line_col_roundtrip: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::sync_from_scene_json_sets_and_clears_hover_range: Error: Expected one completed Cargo executable, got 0
- semio-framework-editor::sync_from_scene_json_applies_all_optional_fields: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-imperative-effect::bundle_contributes_core_module_for_imperative_play: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_matches_serde_and_shares_unchanged_ordered_roots: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_cancellation_and_invalid_projection_preserve_root_until_close: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_nonterminal_drop_is_guarded_without_destroying_root: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_rejects_root_retirement_overgrant_and_closes_factory_owner: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-flow-flow::flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-infinite-dag::direct_intrinsic_serde_and_selection_are_lossless: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-infinite-dag::dag_document_dsl_round_trips_every_node_kind: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-infinite-dag::dag_document_dsl_round_trips_the_demo_fixture: Error: Expected one completed Cargo executable, got 0
- semio-framework-artifact-playbook-playbook::ordered_document_fixture_matches_serde_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::utility_registry_declares_utilities: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_force_graph_top_level_locked_node_ids_pins: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_force_graph_wraps_flat_options: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::edge_handle_snap_sets_circle_handle_angles_on_center_line: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_force_graph_with_snap_sets_handle_angles: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::force_graph_accepts_logical_nodes_without_xy: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_normal_mode_node_id_edges_stacks_by_depth: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_stacks_by_depth: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_pins_locked_root_coordinates: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_hierarchical_tree_nested_locked_node_ids_pins: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_right_places_children_larger_x_than_root: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_upwards_places_children_smaller_y_than_root: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::hierarchical_tree_rejects_unknown_direction: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::redraw_rejects_unknown_mode: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-2d::context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::context_menu_is_grouped_and_keeps_delete_selection_last: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::window_engagements_cover_both_windows: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::set_active_utility_emits_no_ops_and_no_history_entry: Error: Expected one completed Cargo executable, got 0
- semio-s-artifact-puzzle-5d::engagements_expose_no_utility_switch_options_for_either_window: Error: Expected one completed Cargo executable, got 0
- semio-s-plugin-gis::gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution: 
running 1 test
test gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution ... FAILED

successes:

successes:

failures:

---- gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution stdout ----

thread 'gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution' (10151771) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2618:9:
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s


- semio-s-plugin-vcs::vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution: 
running 1 test
test vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution ... FAILED

successes:

successes:

failures:

---- vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution stdout ----

thread 'vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution' (10151910) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️.rs:2618:9:
artifact envelope terminal shell reached Drop before its app-owned bounded retirement authority detached every nested owner
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.02s


