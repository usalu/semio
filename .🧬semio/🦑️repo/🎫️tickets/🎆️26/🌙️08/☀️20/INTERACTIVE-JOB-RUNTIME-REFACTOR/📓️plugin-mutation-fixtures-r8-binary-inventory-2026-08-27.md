# Plugin Mutation Fixture R8 Binary Inventory

The actual binary list selected exactly 24 requested cases: dummy5 + surface8 + transaction10 + keyed-close1, out of 517 total tests. Original R8 binary SHA-256 was `d823a6688cebb72036ae14c98c235c9cf4e93fae4b4c51ca9bba099371d39c00` before and after all executions. Dag's construction repair had landed in source after the prior hold release, so these invocations intentionally used that already-built binary through `bun x nx exec --projects=@semio-tech/framework-plugin`, with no Cargo/rebuild. This is explicit old-R8 executable attribution, not a claim that current source remained unchanged.

## Outcomes

The combined exact24 invocation reported three passes, then `dummy::assert_two_instances_converge_on_disjoint_edits` overflowed its stack and the process aborted (SIGABRT). There was no complete24 footer. It is preserved separately below.

Every exact case was subsequently run in its own fresh process against the same binary, continuing after failures. Results: 4 process passes, 19 SIGABRT outcomes, and 1 ordinary failed test. One process pass is vacuous: `surface::viewer_never_mutates_the_document_or_draft_store` calls the async `assert_viewer_never_mutates::<SurfaceViewerFixture>()` without awaiting it at the source fixture line195; its viewer-law body did not execute. Only the other three passing cases receive their narrowly named executed-test credit.

First-panic categories for the 20 unsuccessful processes: 10 missing-factory guard/assertion failures, seven artifact Store terminal-witness Drop failures, one Presence terminal-witness Drop failure, one viewer error-code mismatch, and one stack overflow. Secondary unwind/drop aborts remain in full output. A primary guard failure is not credited as execution of the originally intended downstream mutation law.

No `--fail-fast`, serialization flag, stack increase, quota increase, or timing relaxation was used. Fresh-process isolation prevents one abort from hiding the other cases; it is not combined-process or callback timing proof. The existing Plugin runCargo implementation only inherits process environment, and no extra RUST_MIN_STACK/RUST_TEST_THREADS setting was found in its script/runner or Cargo config.

| Case | Exact scope | Actual first outcome | Evidence |
| --- | --- | --- | --- |
| 1 | `dummy::assert_ingest_idempotent_does_not_double_apply` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-01-2026-08-27.txt) |
| 2 | `dummy::assert_two_instances_converge_on_disjoint_edits` | stack overflow; abort | [raw](./🧪️member-plugin-mutation-case-r1-02-2026-08-27.txt) |
| 3 | `dummy::assert_undo_redo_round_trip_passes_for_a_real_operation` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-03-2026-08-27.txt) |
| 4 | `dummy::meta_carries_actor_and_local_instance_id` | PASS | [raw](./🧪️member-plugin-mutation-case-r1-04-2026-08-27.txt) |
| 5 | `dummy::new_app_constructs_a_registry_less_wrapper` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-05-2026-08-27.txt) |
| 6 | `surface::editor_and_viewer_share_one_dialect` | PASS | [raw](./🧪️member-plugin-mutation-case-r1-06-2026-08-27.txt) |
| 7 | `surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-07-2026-08-27.txt) |
| 8 | `surface::editor_fixture_still_mutates_normally` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-08-2026-08-27.txt) |
| 9 | `surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-09-2026-08-27.txt) |
| 10 | `surface::new_viewer_constructs_a_registry_less_wrapper` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-10-2026-08-27.txt) |
| 11 | `surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-11-2026-08-27.txt) |
| 12 | `surface::viewer_never_mutates_the_document_or_draft_store` | process PASS; async viewer body NOT executed | [raw](./🧪️member-plugin-mutation-case-r1-12-2026-08-27.txt) |
| 13 | `surface::viewer_rejects_every_contract_mutating_verb` | unknown-key vs viewer.read-only; then abort | [raw](./🧪️member-plugin-mutation-case-r1-13-2026-08-27.txt) |
| 14 | `transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work` | missing-factory vs transaction.instance-busy; then abort | [raw](./🧪️member-plugin-mutation-case-r1-14-2026-08-27.txt) |
| 15 | `transaction::amended_edit_extends_cached_history_in_place` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-15-2026-08-27.txt) |
| 16 | `transaction::command_cache_inputs_share_immutable_arcs` | strict Presence Drop; ordinary FAIL | [raw](./🧪️member-plugin-mutation-case-r1-16-2026-08-27.txt) |
| 17 | `transaction::commit_produces_exactly_one_edit_with_group_id_and_origin` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-17-2026-08-27.txt) |
| 18 | `transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-18-2026-08-27.txt) |
| 19 | `transaction::generation_mismatch_is_rejected_with_the_frozen_code` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-19-2026-08-27.txt) |
| 20 | `transaction::plain_command_still_applies_normally` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-20-2026-08-27.txt) |
| 21 | `transaction::rollback_leaves_state_untouched` | missing-factory; then abort | [raw](./🧪️member-plugin-mutation-case-r1-21-2026-08-27.txt) |
| 22 | `transaction::second_prepare_while_pending_is_rejected_instance_busy` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-22-2026-08-27.txt) |
| 23 | `transaction::undo_and_redo_by_group` | strict artifact Store Drop; abort | [raw](./🧪️member-plugin-mutation-case-r1-23-2026-08-27.txt) |
| 24 | `keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners` | PASS | [raw](./🧪️member-plugin-mutation-case-r1-24-2026-08-27.txt) |

## Actual List

Command:

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --list 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-roster-r1-2026-08-27.txt'
```

```text
component::app::app_builder_tests::action_args_attaches_declared_arguments: test
component::app::app_builder_tests::build_definition_accepts_and_resolves_mode_tools: test
component::app::app_builder_tests::build_definition_accepts_app_and_mode_scope_commands: test
component::app::app_builder_tests::build_definition_accepts_declared_terminology_document: test
component::app::app_builder_tests::build_definition_accepts_introduction_step_introducing_escape_hatch_element_id: test
component::app::app_builder_tests::build_definition_accepts_introduction_with_declared_window_utility_and_action_targets: test
component::app::app_builder_tests::build_definition_accepts_tutorial_with_declared_action_utility_and_gesture_targets: test
component::app::app_builder_tests::build_definition_auto_injects_clipboard_actions_and_keybindings: test
component::app::app_builder_tests::build_definition_auto_injects_history_actions_and_keybindings: test
component::app::app_builder_tests::build_definition_auto_injects_the_history_panel_tab_and_filter_action: test
component::app::app_builder_tests::build_definition_carries_window_interactions_and_injects_framework_actions: test
component::app::app_builder_tests::build_definition_derives_command_owner_from_structural_containment: test
component::app::app_builder_tests::build_definition_does_not_duplicate_a_manually_declared_history_panel_tab: test
component::app::app_builder_tests::build_definition_does_not_duplicate_manually_declared_history_keybinding: test
component::app::app_builder_tests::build_definition_is_sync_and_accepts_valid_manifest: test
component::app::app_builder_tests::build_definition_rejects_dialog_cancel_action_referencing_undeclared_action: test
component::app::app_builder_tests::build_definition_rejects_dialog_duplicate_arg_ids: test
component::app::app_builder_tests::build_definition_rejects_dialog_submit_action_referencing_undeclared_action: test
component::app::app_builder_tests::build_definition_rejects_duplicate_action_ids: test
component::app::app_builder_tests::build_definition_rejects_duplicate_command_ids: test
component::app::app_builder_tests::build_definition_rejects_duplicate_dialog_ids: test
component::app::app_builder_tests::build_definition_rejects_duplicate_introduction_step_ids: test
component::app::app_builder_tests::build_definition_rejects_duplicate_mode_command_ids: test
component::app::app_builder_tests::build_definition_rejects_duplicate_tutorial_ids: test
component::app::app_builder_tests::build_definition_rejects_empty_mode_command_id: test
component::app::app_builder_tests::build_definition_rejects_introduction_step_interacting_on_undeclared_utility: test
component::app::app_builder_tests::build_definition_rejects_introduction_step_interacting_on_undeclared_window_kind: test
component::app::app_builder_tests::build_definition_rejects_introduction_step_introducing_undeclared_panel_tab: test
component::app::app_builder_tests::build_definition_rejects_introduction_step_introducing_undeclared_window_kind: test
component::app::app_builder_tests::build_definition_rejects_introduction_step_targeting_malformed_element_id: test
component::app::app_builder_tests::build_definition_rejects_introduction_with_no_steps: test
component::app::app_builder_tests::build_definition_rejects_keybinding_for_undeclared_action_once_opted_in: test
component::app::app_builder_tests::build_definition_rejects_layout_with_unknown_window_kind: test
component::app::app_builder_tests::build_definition_rejects_mode_tool_ref_to_undeclared_tool: test
component::app::app_builder_tests::build_definition_rejects_terminology_document_for_undeclared_terminology: test
component::app::app_builder_tests::build_definition_rejects_tool_referenced_by_no_mode: test
component::app::app_builder_tests::build_definition_rejects_tutorial_event_referencing_undeclared_action: test
component::app::app_builder_tests::build_definition_rejects_tutorial_failing_structural_validation: test
component::app::app_builder_tests::build_definition_rejects_tutorial_gesture_targeting_malformed_element_id: test
component::app::app_builder_tests::build_definition_rejects_tutorial_ui_change_referencing_undeclared_utility: test
component::app::app_builder_tests::build_definition_rejects_window_kind_action_referencing_undeclared_action: test
component::app::app_builder_tests::build_definition_rejects_window_kind_utility_referencing_undeclared_utility: test
component::app::app_builder_tests::catalog_chrome_icons_resolve_to_vendored_icon_names: test
component::app::app_builder_tests::declaring_dialog_appends_to_definition: test
component::app::app_builder_tests::declaring_introduction_injects_start_introduction_action: test
component::app::app_builder_tests::declaring_tools_injects_set_active_tool_action_and_keybinding: test
component::app::app_builder_tests::declaring_tutorial_injects_start_tutorial_action: test
component::app::app_builder_tests::declaring_utilities_injects_set_active_utility_action_and_keybinding: test
component::app::app_builder_tests::dialog_submit_action_may_reference_an_injected_history_action: test
component::app::app_builder_tests::no_introduction_means_no_start_introduction_action: test
component::app::app_builder_tests::no_tools_means_no_set_active_tool_action: test
component::app::app_builder_tests::no_tutorial_means_no_start_tutorial_action_but_record_is_always_injected: test
component::app::app_builder_tests::no_utilities_means_no_set_active_utility_action: test
component::app::app_builder_tests::operation_view_and_shell_actions_are_declared_with_their_kind: test
component::app::app_builder_tests::release_catalog_rejects_unclassified_and_retains_explicit_non_ui_dispositions: test
component::app::app_commands_tests::command_id_matches_declared_row: test
component::app::app_commands_tests::ctx_is_threaded_through_dispatch_into_every_handler: test
component::app::app_commands_tests::dispatch_forwards_to_the_payload_modules_own_handle: test
component::app::app_commands_tests::fieldless_payload_matches_a_unit_variants_wire_form: test
component::app::app_commands_tests::generated_tool_job_catalog_is_an_exact_bijection_with_rows: test
component::app::app_commands_tests::keyed_rows_separate_the_command_id_from_the_wire_keyword: test
component::app::app_commands_tests::wire_round_trips_through_dsl_ops_op_text_and_op_binary: test
component::app::artifact_contribution_tests::dependency_gating_rejects_a_contribution_onto_a_non_dependency: test
component::app::artifact_contribution_tests::id_namespacing_rejects_a_collision_with_an_owner_kind: test
component::app::artifact_contribution_tests::mutation_roster_entries_are_deterministic_across_repeated_calls: test
component::app::artifact_definition_contract_tests::identities_and_locales_are_explicit_and_conflicts_do_not_overwrite: test
component::app::artifact_definition_contract_tests::plural_definition_carries_every_artifact_capability_without_a_dispatch_edit: test
component::app::artifact_definition_contract_tests::registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically: test
component::app::artifact_fixed_registry_tests::artifact_envelope_ingress_cancel_and_interrupted_close_release_one_real_page_per_grant: test
component::app::artifact_fixed_registry_tests::artifact_envelope_ingress_saturation_returns_exact_plus_one_owner_and_closes_fifo_slots: test
component::app::artifact_fixed_registry_tests::duplicate_id_never_drops_active_media_snapshot_or_segmented_download_ownership: test
component::app::artifact_fixed_registry_tests::media_construction_failure_at_each_fallible_seam_keeps_exact_close_authority: test
component::app::artifact_fixed_registry_tests::media_registry_detach_preserves_unrelated_live_owner_and_exact_close_handoff: test
component::app::artifact_inference_service_tests::artifact_inference_registry_is_order_independent_and_idempotent: test
component::app::artifact_inference_service_tests::artifact_inference_registry_rejects_any_conflicting_duplicate: test
component::app::artifact_inference_wire_tests::request_rejects_unknown_wire_version_before_registry_lookup: test
component::app::artifact_inference_wire_tests::wire_execution_observes_midflight_cancellation: test
component::app::artifact_inference_wire_tests::wire_execution_preserves_every_echoed_request_fact: test
component::app::artifact_media_export_credit_tests::final_poll_credit_accepts_maximum_and_rejects_plus_one_without_allocating_media: test
component::app::artifact_media_export_credit_tests::segmented_output_accepts_exact_cap_and_rejects_plus_one_or_foreign_authority: test
component::app::artifact_media_export_credit_tests::segmented_output_preallocates_exact_former_growth_boundary_and_drains_terminal_storage_to_zero: test
component::app::artifact_media_export_credit_tests::segmented_output_seal_is_linearly_ordered_with_push: test
component::app::artifact_media_export_credit_tests::snapshot_a_survives_cache_b_and_only_the_bounded_retirement_owner_performs_final_drop: test
component::app::artifact_reserved_tool_job_tests::erased_dispatch_clone_release_preserves_unique_bounded_job_disposal_authority: test
component::app::child_member_registry_tests::fixed_child_member_registry_admits_exact_capacity_rejects_plus_one_and_cursor_detaches_every_owner: test
component::app::child_member_registry_tests::fixed_child_member_registry_resolves_hash_collisions_without_replacement: test
component::app::child_member_registry_tests::incomplete_child_member_registry_drop_faults_in_release_instead_of_destroying_nested_owners: test
component::app::child_member_registry_tests::stale_child_member_admission_cannot_cancel_a_reused_slot_generation: test
component::app::declarations::fixture::a_conflicting_declaration_leaves_zero_rows_behind: test
component::app::declarations::fixture::declaring_registers_schema_io_and_surfaces: test
component::app::declarations::fixture::format_descriptor_identity_is_scoped_to_artifact_and_standard: test
component::app::declarations::fixture::ids_are_derived_from_the_dialect: test
component::app::declarations::fixture::io_route_finds_the_conformance_profile_hop: test
component::app::declarations::fixture::open_mutate_save_round_trips_through_the_generic_snapshot_builder: test
component::app::example_source_tests::example_delegates_to_example_source: test
component::app::example_source_tests::example_source_converts_into_example_definition_and_registers_on_app: test
component::app::form_kit_tests::entity_detail_builds_a_stack_with_header_key_value_and_actions: test
component::app::form_kit_tests::form_panel_builder_from_dictionary_routes_entries_into_field_rows: test
component::app::form_kit_tests::form_panel_builder_wraps_a_field_control_and_submit_button: test
component::app::merge_ui_values_tests::merge_ui_values_falls_back_to_whichever_single_side_is_set: test
component::app::merge_ui_values_tests::merge_ui_values_prefers_input_on_key_collision_between_two_maps: test
component::app::merge_ui_values_tests::merge_ui_values_replaces_a_non_object_args_wholesale_when_input_is_also_set: test
component::app::merge_ui_values_tests::merge_ui_values_returns_none_when_neither_side_is_set: test
component::app::merge_ui_values_tests::retained_ui_value_bridge_advances_every_shape_to_the_matching_json_candidate: test
component::app::merge_ui_values_tests::retained_ui_value_bridge_cancel_and_deadline_preserve_the_original_owner: test
component::app::merge_ui_values_tests::retained_ui_value_bridge_rejects_depth_plus_one_without_consuming_the_original: test
component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply: test
component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits: test
component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation: test
component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id: test
component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper: test
component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect: test
component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id: test
component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally: test
component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id: test
component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper: test
component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id: test
component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store: test
component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb: test
component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work: test
component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place: test
component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs: test
component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin: test
component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying: test
component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code: test
component::app::mutation_fixture::transaction::plain_command_still_applies_normally: test
component::app::mutation_fixture::transaction::rollback_leaves_state_untouched: test
component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy: test
component::app::mutation_fixture::transaction::undo_and_redo_by_group: test
component::app::owned_contribution_error_tests::owned_errors_preserve_their_messages: test
component::app::owned_media_error_tests::owned_errors_preserve_their_messages: test
component::app::panel_kit_tests::panel_tree_builder_produces_a_namespaced_tree_with_placeholder: test
component::app::panel_kit_tests::selection_ids_reads_the_ids_array_arg: test
component::app::panel_kit_tests::tree_item_builds_a_bare_item: test
component::app::panel_kit_tests::tree_item_with_action_draggable_maps_json_object_to_string_drag_data: test
component::app::terminology_tests::resolve_labels_is_exhaustive_over_all_four_cells: test
component::app::tree_convert_tests::dynamic_text_root_reports_fixed_capacity_admission_failure: test
component::app::typed_command_full_operation_tests::document_revision_change_between_ephemeral_preparation_turns_closes_without_publishing_the_lane_root: test
component::app::typed_command_full_operation_tests::every_language_neutral_hostile_row_executes_the_owned_state_machine_and_serde_oracle: test
component::app::typed_command_full_operation_tests::fixture_contract_is_anchored_to_the_production_retained_factory_publisher_and_host_receivers: test
component::app::typed_command_full_operation_tests::full_operation_source_rejects_generic_reducers_and_old_monolithic_shells: test
component::app::typed_command_full_operation_tests::host_configuration_uses_one_bounded_event_sourced_lane_before_the_generic_gate: test
component::app::typed_command_full_operation_tests::language_neutral_empty_single_max_and_plus_one_match_the_test_only_oracle: test
component::app::typed_command_full_operation_tests::language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields: test
component::app::typed_command_full_operation_tests::retained_child_wire_rejection_retires_nested_owners_under_the_production_grant: test
component::app::typed_command_full_operation_tests::retained_latest_wins_contended_finish_is_deferred_and_cannot_release_replacement: test
component::app::typed_command_full_operation_tests::retained_latest_wins_full_domain_exact_keys_match_serde_oracle_and_retire: test
component::app::typed_command_full_operation_tests::retained_latest_wins_full_registry_reclaims_completed_targets_before_admission: test
component::app::typed_command_full_operation_tests::retained_latest_wins_producer_child_cannot_bypass_document_or_app_publication_claim: test
component::app::typed_command_full_operation_tests::retained_latest_wins_rebase_rebinds_exact_registered_cancellation_authority: test
component::app::typed_command_full_operation_tests::typed_child_publication_never_enters_the_monolithic_group_fallback: test
component::app::window_kits_tests::abandoned_table_rows_retire_one_row_action_or_cell_per_opportunity: test
component::app::window_kits_tests::document_kit_renders_one_child_per_page: test
component::app::window_kits_tests::editable_variants_declare_exactly_their_frozen_command_id: test
component::app::window_kits_tests::en_de_labels_differ_except_text: test
component::app::window_kits_tests::image_kit_renders_data_uri_from_base64: test
component::app::window_kits_tests::kind_ids_match_the_frozen_table: test
component::app::window_kits_tests::media_kit_renders_duration_and_position: test
component::app::window_kits_tests::mesh_kit_renders_world3d_component_scene: test
component::app::window_kits_tests::table_kit_render_rows_renders_row_action_buttons_carrying_their_dispatchable_descriptor: test
component::app::window_kits_tests::table_kit_render_rows_stamps_a_stable_row_id_and_omits_the_actions_column_when_no_row_has_one: test
component::app::window_kits_tests::table_kit_renders_columns_and_rows_json: test
component::app::window_kits_tests::table_rows_max_plus_one_returns_the_exact_row_owner: test
component::app::window_kits_tests::text_kit_read_only_stamps_settings_json: test
component::app::window_kits_tests::text_kit_renders_buffer_into_component_scene: test
component::app::window_kits_tests::tree_kit_renders_nested_items: test
component::app::window_kits_tests::window_kind_ids_and_labels_are_non_empty_and_id_matches_definition: test
component::builder::dependency_fixture::mutations::add_value::tests::direct_leaf_contract: test
component::builder::dependency_fixture::tests::actual_descriptor_provenance: test
component::builder::dependency_fixture::tests::contribution_plan_matches_direct_leaf: test
component::builder::dependency_fixture::tests::exact_i32_inverse_and_boundary_laws: test
component::builder::dependency_fixture::tests::keyword_owned_record_codec_is_forwarded_once: test
component::builder::dependency_fixture::tests::ordered_diff_preserves_rejection: test
component::builder::dependency_fixture::tests::strict_payload_and_all_codecs: test
component::builder::plugin_builder_dependency_tests::a_direct_dependency_permits_its_contribution_and_lands_on_the_manifest: test
component::builder::plugin_builder_dependency_tests::dependency_gating_rejects_a_contribution_onto_a_non_dependency: test
component::builder::plugin_builder_dependency_tests::flow_extension_descriptors_are_idempotent_and_conflict_rejecting: test
component::builder::plugin_builder_dependency_tests::host_media_conflicts_reject_the_whole_candidate_before_execution: test
component::builder::plugin_builder_dependency_tests::host_media_contributions_are_idempotent_and_execute_only_at_runtime: test
component::builder::schema_stamping_tests::editor_does_not_overwrite_an_explicitly_set_document_schema: test
component::builder::schema_stamping_tests::editor_stamps_document_schema_from_the_type_when_left_empty: test
component::builder::schema_stamping_tests::routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service: test
component::builder::schema_stamping_tests::viewer_stamps_document_schema_from_the_type_when_left_empty: test
component::contributed_mutation_wire::mutations::add_value::tests::direct_leaf_descriptor_and_inverse_law: test
component::contributed_mutation_wire::tests::descriptor_and_provenance_are_direct: test
component::contributed_mutation_wire::tests::ordered_checked_diff_and_minimum_inverse_are_lawful: test
component::contributed_mutation_wire::tests::serde_binary_and_composite_plan_match_the_leaf: test
component::declaration_fixture_mutations::std1_any::set_value::tests::actual_leaf_descriptor_and_provenance: test
component::declaration_fixture_mutations::std1_any::set_value::tests::assignment_inverse_and_structural_diff: test
component::declaration_fixture_mutations::std1_any::set_value::tests::source_json_codecs_and_i32_boundaries: test
component::declaration_fixture_mutations::std1_strict::set_value::tests::actual_leaf_descriptor_and_provenance: test
component::declaration_fixture_mutations::std1_strict::set_value::tests::assignment_inverse_and_structural_diff: test
component::declaration_fixture_mutations::std1_strict::set_value::tests::source_json_codecs_and_i32_boundaries: test
component::declaration_fixture_mutations::std2_any::set_value::tests::actual_leaf_descriptor_and_provenance: test
component::declaration_fixture_mutations::std2_any::set_value::tests::assignment_inverse_and_structural_diff: test
component::declaration_fixture_mutations::std2_any::set_value::tests::source_json_codecs_and_i32_boundaries: test
component::declaration_fixture_mutations::tests::strict_profile_is_an_io_rule_not_a_mutation_constraint: test
component::derived_artifact_children_tests::derive_artifact_facets_children_arm_wires_the_macro_generated_composer: test
component::derived_artifact_children_tests::derived_composer_compose_routes_matching_sources_through_compose_from_children: test
component::derived_artifact_children_tests::derived_composer_reads_defaults_to_composition_reads_for_a_leaf_with_no_children: test
component::derived_artifact_children_tests::derived_composer_reads_includes_child_slot_dialects: test
component::derived_artifact_children_tests::mutations::tests::empty_and_nonempty_codec_inputs_are_rejected: test
component::derived_artifact_children_tests::mutations::tests::empty_roster_has_no_fabricated_leaf: test
component::derived_artifact_children_tests::mutations::tests::every_neutral_json_value_is_uninhabited: test
component::derived_artifact_children_tests::mutations::tests::existing_children_diff_stays_identity: test
component::describe::tests::package_descriptor_advertises_metadata_only_cold_inference_routes: test
component::engagement::tests::engagement_token_matches_full_token_only: test
component::engagement::tests::strip_engagement_prefix_accepts_normalized_and_raw_forms: test
component::engagement::tests::strip_engagement_prefix_preserves_decimal_points: test
component::engagement::tests::strip_engagement_prefix_rejects_non_matching_commands: test
component::host::body::tests::an_empty_poll_body_yields_no_chunks: test
component::host::body::tests::collect_faults_over_cap_instead_of_truncating: test
component::host::body::tests::collect_reassembles_the_full_poll_buffer: test
component::host::body::tests::poll_backed_reader_yields_the_whole_buffer_then_ends: test
component::local_interaction::authority::tests::local_interaction_topology_close_invalidates_authority_without_wrapping: test
component::local_interaction::authority::tests::local_interaction_topology_input_authority_matches_node_crypto_fixture: test
component::local_interaction::authority::tests::local_interaction_topology_overflow_rejects_before_cache_mutation: test
component::local_interaction::capture::tests::local_interaction_capture_actual_store_matches_canonical_fixture_at_small_grants: test
component::local_interaction::capture::tests::local_interaction_capture_cancel_worker_transfer_and_exact_registry_return: test
component::local_interaction::live::tests::local_interaction_live_pages_wait_exact_ack_and_all_three_roots: test
component::local_interaction::live::tests::local_interaction_live_partial_admission_retains_successful_roots: test
component::local_interaction::live::tests::local_interaction_live_partial_error_preserves_wrapper_emission_and_retirement_counts: test
component::local_interaction::live::tests::local_interaction_live_reopened_request_rejects_old_started_cancel: test
component::local_interaction::live::tests::local_interaction_runtime_query_generation_exhausts_before_slot_admission: test
component::local_interaction::query::tests::local_interaction_query_exact_pages_ack_backpressure_and_terminal_return: test
component::local_interaction::query::tests::local_interaction_query_partial_encoder_failure_keeps_exact_byte_ownership: test
component::local_interaction::query::tests::local_interaction_query_zero_grants_cancel_and_worker_transfer: test
component::local_interaction::retirement::tests::local_interaction_retirement_live_drop_is_rejected: test
component::local_interaction::retirement::tests::local_interaction_retirement_matches_language_neutral_exact_bytes: test
component::local_interaction::retirement::tests::local_interaction_retirement_releases_empty_reserved_allocations: test
component::local_interaction::retirement::tests::local_interaction_retirement_shared_alias_and_final_owner_are_distinct: test
component::local_interaction::set_state::tests::local_interaction_mutation_leaf_descriptor_and_exact_codecs_are_owned: test
component::plugin_runtime::contributed_mutation_wire_tests::artifact_mutation_plan_echoes_identity_and_runs_the_registered_plan: test
component::plugin_runtime::contributed_mutation_wire_tests::artifact_mutation_plan_rejects_a_mismatched_artifact_kind: test
component::plugin_runtime::contributed_mutation_wire_tests::artifact_mutation_plan_rejects_an_unregistered_mutation_id: test
component::plugin_runtime::dff_public_action_admission_tests::action_and_command_specific_dff_wire_caps_are_enforced_before_deserialization: test
component::plugin_runtime::dff_public_action_admission_tests::action_entry_points_await_the_retained_job_instead_of_single_polling_it: test
component::plugin_runtime::dff_public_action_admission_tests::command_classifier_uses_only_the_exact_address_command_id: test
component::plugin_runtime::dff_public_action_admission_tests::malformed_and_hostile_strings_are_rejected_by_predecode_admission: test
component::plugin_runtime::dff_public_action_admission_tests::public_action_and_command_entry_points_require_a_live_instance_before_decode: test
component::plugin_runtime::dff_public_action_admission_tests::same_schema_and_id_cannot_inherit_another_controllers_public_limit: test
component::plugin_runtime::extension_bundle_dependency_tests::extends_matching_the_first_dependency_is_accepted_regardless_of_call_order: test
component::plugin_runtime::extension_bundle_dependency_tests::extends_mismatching_the_first_dependency_panics: test
component::plugin_runtime::extension_bundle_dependency_tests::extends_set_before_a_mismatching_dependency_also_panics: test
component::plugin_runtime::opening_command_relay_tests::default_app_commands_relay_the_validated_wire_coordinates: test
component::plugin_runtime::opening_command_relay_tests::open_artifact_relays_an_exactly_matched_surface: test
component::plugin_runtime::opening_command_relay_tests::opening_relays_reject_invalid_or_inconsistent_addresses: test
component::plugin_runtime::paged_command_ingress_tests::decoded_two_field_cancellation_never_publishes_partial_success: test
component::plugin_runtime::paged_command_ingress_tests::interrupted_two_page_command_closes_one_exact_page_per_step_then_faults: test
component::plugin_runtime::plugin_builder_contract_tests::a_checkpoint_pins_its_children_and_a_checkout_cascades_back_to_them: test
component::plugin_runtime::plugin_builder_contract_tests::a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames: test
component::plugin_runtime::plugin_builder_contract_tests::a_coalesced_gesture_appends_exactly_one_command_log_entry: test
component::plugin_runtime::plugin_builder_contract_tests::a_command_reaches_both_ephemeral_lanes_without_touching_history: test
component::plugin_runtime::plugin_builder_contract_tests::a_command_that_emits_nothing_ephemeral_leaves_both_lanes_untouched: test
component::plugin_runtime::plugin_builder_contract_tests::a_pick_is_never_undoable_the_default_undo_only_ever_walks_the_document_store: test
component::plugin_runtime::plugin_builder_contract_tests::a_spawned_task_awaits_a_real_request_and_its_resume_mutates_the_store_under_the_original_meta: test
component::plugin_runtime::plugin_builder_contract_tests::action_definition_with_category_sets_the_ribbon_taxonomy_field: test
component::plugin_runtime::plugin_builder_contract_tests::action_emit_amend_coalesces_while_commit_does_not: test
component::plugin_runtime::plugin_builder_contract_tests::activate_intent_dispatches_through_the_typed_command_path_same_turn: test
component::plugin_runtime::plugin_builder_contract_tests::activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations: test
component::plugin_runtime::plugin_builder_contract_tests::addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler: test
component::plugin_runtime::plugin_builder_contract_tests::amend_dispatch_reports_only_this_dispatch_new_operations: test
component::plugin_runtime::plugin_builder_contract_tests::an_op_less_view_action_is_logged_with_edit_id_none_and_count_one: test
component::plugin_runtime::plugin_builder_contract_tests::an_operation_action_appends_one_command_log_entry_linked_to_its_edit: test
component::plugin_runtime::plugin_builder_contract_tests::an_operation_kind_action_with_zero_operations_still_logs_one_entry: test
component::plugin_runtime::plugin_builder_contract_tests::app_close_step_drains_at_most_one_segment_and_one_chunk_budget: test
component::plugin_runtime::plugin_builder_contract_tests::app_maintenance_and_close_retain_completed_envelope_results_until_terminal_empty: test
component::plugin_runtime::plugin_builder_contract_tests::app_maintenance_reclaims_late_envelope_field_returns_before_close_terminal: test
component::plugin_runtime::plugin_builder_contract_tests::app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift: test
component::plugin_runtime::plugin_builder_contract_tests::artifact_close_final_destructor_is_constant_after_every_owned_field_is_drained: test
component::plugin_runtime::plugin_builder_contract_tests::attach_detach_reattach_resumes_backbone_convergence: test
component::plugin_runtime::plugin_builder_contract_tests::benign_undo_with_nothing_to_undo_stays_unlogged_with_scope_none: test
component::plugin_runtime::plugin_builder_contract_tests::build_definition_rejects_transitive_flat_interaction: test
component::plugin_runtime::plugin_builder_contract_tests::cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking: test
component::plugin_runtime::plugin_builder_contract_tests::cancellation_supersession_and_saturated_app_close_are_parent_scope_constant_time: test
component::plugin_runtime::plugin_builder_contract_tests::checkpoint_then_restore_requeues_a_restartable_tasks_command_as_a_resume: test
component::plugin_runtime::plugin_builder_contract_tests::child_content_publication_path_copies_fixed_pages_and_command_capture_retains_one_root: test
component::plugin_runtime::plugin_builder_contract_tests::child_root_maintenance_reclaims_completed_owner_for_later_publication: test
component::plugin_runtime::plugin_builder_contract_tests::child_root_maintenance_requires_terminal_empty_before_reclaim: test
component::plugin_runtime::plugin_builder_contract_tests::child_snapshot_retirement_rejection_preserves_exact_erased_owner: test
component::plugin_runtime::plugin_builder_contract_tests::clear_selection_and_select_all_apply_across_every_declared_domain: test
component::plugin_runtime::plugin_builder_contract_tests::coalesced_operations_amend_a_single_edit: test
component::plugin_runtime::plugin_builder_contract_tests::command_from_intent_rejects_a_non_v1_action_version: test
component::plugin_runtime::plugin_builder_contract_tests::command_op_records_history_exactly_like_an_operation_action: test
component::plugin_runtime::plugin_builder_contract_tests::composite_gesture_produces_one_undo_group_spanning_parent_and_child_with_real_handles: test
component::plugin_runtime::plugin_builder_contract_tests::consecutive_identical_view_dispatches_are_distinct_history_entries: test
component::plugin_runtime::plugin_builder_contract_tests::context_menu_funnel_organizes_a_synthetic_apps_flat_overflow_menu: test
component::plugin_runtime::plugin_builder_contract_tests::context_menu_resolves_labels_from_the_registry_and_respects_guards: test
component::plugin_runtime::plugin_builder_contract_tests::context_menu_wire_request_without_view_state_still_parses: test
component::plugin_runtime::plugin_builder_contract_tests::copied_app_type_cannot_inherit_same_controller_schema_and_id_proof: test
component::plugin_runtime::plugin_builder_contract_tests::copy_cut_paste_are_registered_as_clipboard_kind_actions: test
component::plugin_runtime::plugin_builder_contract_tests::copy_emits_clipboard_write_effect_with_no_operations: test
component::plugin_runtime::plugin_builder_contract_tests::copy_on_empty_selection_is_a_benign_no_operation: test
component::plugin_runtime::plugin_builder_contract_tests::created_children_survive_absorb_into_the_child_store_map: test
component::plugin_runtime::plugin_builder_contract_tests::cut_removes_label_and_emits_clipboard_write_as_one_undo_unit: test
component::plugin_runtime::plugin_builder_contract_tests::document_round_trips_through_serialization: test
component::plugin_runtime::plugin_builder_contract_tests::ephemeral_snapshot_carries_encoded_interaction_from_declared_broadcast_specs: test
component::plugin_runtime::plugin_builder_contract_tests::group_undo_skips_a_foreign_tail_child_but_still_undoes_parent_and_touched_child: test
component::plugin_runtime::plugin_builder_contract_tests::history_actions_round_trip_through_the_store: test
component::plugin_runtime::plugin_builder_contract_tests::history_delivery_does_not_widen_a_none_ui_scope: test
component::plugin_runtime::plugin_builder_contract_tests::history_delivery_preserves_partial_ui_scope: test
component::plugin_runtime::plugin_builder_contract_tests::ingest_operations_is_idempotent: test
component::plugin_runtime::plugin_builder_contract_tests::ingested_remote_edits_are_backfilled_into_the_command_log: test
component::plugin_runtime::plugin_builder_contract_tests::instance_close_cancellation_drops_the_instances_tasks_and_leaks_no_registry_slot: test
component::plugin_runtime::plugin_builder_contract_tests::interaction_hover_is_ephemeral_and_never_touches_the_persisted_interaction_store: test
component::plugin_runtime::plugin_builder_contract_tests::interaction_select_replace_persists_through_the_interaction_store: test
component::plugin_runtime::plugin_builder_contract_tests::interaction_verbs_are_recorded_under_the_interaction_action_kind: test
component::plugin_runtime::plugin_builder_contract_tests::interaction_view_peers_selecting_returns_actor_and_color: test
component::plugin_runtime::plugin_builder_contract_tests::key_dedupe_cancels_the_previously_live_task_under_the_same_key: test
component::plugin_runtime::plugin_builder_contract_tests::keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_construction_failure_preserves_original_live_root: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_constructs_worker_shell_before_exact_live_detachment: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_contended_pump_keeps_exact_outcome_source: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_does_not_publish_terminal_before_watchdog: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_fault_outcome_dominates_complete_progress: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_optional_monotonic_clock_rejects_missing_and_backward_authority: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_preflight_and_shared_restore_preserve_exact_owner: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_rejects_foreign_root_and_exhaustion_before_detach: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::instance_lifetime_close_witness_survives_quarantine_removal_and_reused_id: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::local_interaction_cold_transaction_receipts_and_encoded_route_rejection: test
component::plugin_runtime::plugin_builder_contract_tests::local_interaction_dispatch::local_interaction_registered_query_channel_continuation_ack_and_close: test
component::plugin_runtime::plugin_builder_contract_tests::manifest_command_dispatch_validates_structural_app_ownership: test
component::plugin_runtime::plugin_builder_contract_tests::manifest_mode_command_requires_the_active_structural_owner: test
component::plugin_runtime::plugin_builder_contract_tests::maximum_child_public_dispatch_reaches_first_continuation_without_clone_or_encode: test
component::plugin_runtime::plugin_builder_contract_tests::menu_group_produces_a_group_row_keyed_by_category: test
component::plugin_runtime::plugin_builder_contract_tests::merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads: test
component::plugin_runtime::plugin_builder_contract_tests::microsecond_registered_factory_dispatch_preserves_exact_half_ms_fake_clock: test
component::plugin_runtime::plugin_builder_contract_tests::note_shell_command_is_intercepted_before_the_app_and_records_each_repeat: test
component::plugin_runtime::plugin_builder_contract_tests::operation_action_emits_kernel_op_with_true_inverse: test
component::plugin_runtime::plugin_builder_contract_tests::operation_command_emits_kernel_op_with_true_inverse: test
component::plugin_runtime::plugin_builder_contract_tests::paste_materializes_fragment_at_original_anchor: test
component::plugin_runtime::plugin_builder_contract_tests::paste_with_no_fragment_arg_is_a_benign_no_operation: test
component::plugin_runtime::plugin_builder_contract_tests::paste_with_non_original_anchor_reaches_the_app_placement: test
component::plugin_runtime::plugin_builder_contract_tests::peer_presence_capture_is_one_arc_and_retirement_waits_for_then_drains_the_exact_root: test
component::plugin_runtime::plugin_builder_contract_tests::peer_roster_saturation_cancel_stale_and_interrupted_close_preserve_exact_authority: test
component::plugin_runtime::plugin_builder_contract_tests::plugin_builder_builds_bundle_from_fluent_spec: test
component::plugin_runtime::plugin_builder_contract_tests::plugin_builder_wires_app_factory_for_create_app: test
component::plugin_runtime::plugin_builder_contract_tests::plugin_command_handler_is_program_owned_across_app_instances: test
component::plugin_runtime::plugin_builder_contract_tests::poisoned_cancellation_authority_fails_closed_without_recovery_or_waiting: test
component::plugin_runtime::plugin_builder_contract_tests::registry_less_construction_rejects_before_the_reducer: test
component::plugin_runtime::plugin_builder_contract_tests::rendering_the_history_body_reflects_a_log_only_change_with_no_store_generation_bump: test
component::plugin_runtime::plugin_builder_contract_tests::retained_factory_proof_requires_the_exact_registered_runtime_authority: test
component::plugin_runtime::plugin_builder_contract_tests::retained_field_maximum_and_maximum_plus_one_are_language_neutral: test
component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_cancellation_guards_real_store_publication_and_preserves_committed_ack: test
component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_raw_capacity_is_not_initialized_byte_retirement: test
component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_real_document_publication_cancellation_and_exhausted_ack_close: test
component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_registered_dispatch_rebases_worker_and_publishes_real_document: test
component::plugin_runtime::plugin_builder_contract_tests::retained_latest_wins_reserved_slots_and_ready_publisher_are_fair: test
component::plugin_runtime::plugin_builder_contract_tests::retained_operation_continues_after_command_admission_until_publication_and_retirement: test
component::plugin_runtime::plugin_builder_contract_tests::retained_presence_fills_presence_store_and_peer_marks_and_drops_left_peers: test
component::plugin_runtime::plugin_builder_contract_tests::revert_to_command_restores_the_snapshot_and_appends_one_entry: test
component::plugin_runtime::plugin_builder_contract_tests::scope_upgrade_full_stays_full: test
component::plugin_runtime::plugin_builder_contract_tests::segmented_download_remains_addressable_until_terminal_none_is_observed: test
component::plugin_runtime::plugin_builder_contract_tests::selection_count_phrase_formats_mixed_selection: test
component::plugin_runtime::plugin_builder_contract_tests::serializer_entry_of_and_deserializer_entry_of_erase_correctly: test
component::plugin_runtime::plugin_builder_contract_tests::set_active_utility_carries_its_value_directly_and_emits_no_operations: test
component::plugin_runtime::plugin_builder_contract_tests::set_history_command_filter_emits_no_operations_and_updates_the_view: test
component::plugin_runtime::plugin_builder_contract_tests::set_history_command_filter_is_never_logged: test
component::plugin_runtime::plugin_builder_contract_tests::set_selection_mode_and_set_interaction_granularity_persist_immediately: test
component::plugin_runtime::plugin_builder_contract_tests::shared_framework_actions_have_exact_registered_factory_and_joined_bus_identity: test
component::plugin_runtime::plugin_builder_contract_tests::shared_framework_job_is_cancellable_resumable_and_boundedly_closeable: test
component::plugin_runtime::plugin_builder_contract_tests::shared_retained_command_checkpoint_resumes_exact_cursor_and_cancels_with_bounded_close: test
component::plugin_runtime::plugin_builder_contract_tests::shell_action_emits_host_effect_without_operations: test
component::plugin_runtime::plugin_builder_contract_tests::shell_action_with_inverse_bubbles_a_replay_effect_instead_of_replaying_locally: test
component::plugin_runtime::plugin_builder_contract_tests::spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it: test
component::plugin_runtime::plugin_builder_contract_tests::test_app_id_matches_its_own_dialect: test
component::plugin_runtime::plugin_builder_contract_tests::the_child_content_view_never_goes_stale_across_undo_and_redo: test
component::plugin_runtime::plugin_builder_contract_tests::tool_cancellation_isolated_by_live_instance_with_same_controller_and_document: test
component::plugin_runtime::plugin_builder_contract_tests::ui_dispatch_backstop_rejects_every_non_migrated_action_and_command: test
component::plugin_runtime::plugin_builder_contract_tests::ui_history_panel_filters_rows_and_gates_the_backwards_action: test
component::plugin_runtime::plugin_builder_contract_tests::ui_tree_stamping_caches_interaction_topology_from_a_domain_bound_tree: test
component::plugin_runtime::plugin_builder_contract_tests::undo_and_redo_append_entries_and_never_shrink_the_log: test
component::plugin_runtime::plugin_builder_contract_tests::undo_on_empty_history_is_a_benign_no_operation: test
component::plugin_runtime::plugin_builder_contract_tests::unproved_command_fails_before_an_overrun_reducer_can_start: test
component::plugin_runtime::plugin_builder_contract_tests::validate_state_prunes_a_stale_selection_id_after_the_document_deletes_it: test
component::plugin_runtime::plugin_builder_contract_tests::view_action_emits_no_operations: test
component::plugin_runtime::plugin_builder_contract_tests::view_action_emitting_ops_is_rejected: test
component::plugin_runtime::plugin_builder_contract_tests::view_action_with_inverse_is_revertible_and_backwards_restores_app_runtime_state: test
component::plugin_runtime::plugin_builder_contract_tests::view_dispatches_remain_distinct_across_interleaved_entries: test
component::plugin_runtime::plugin_builder_contract_tests::view_kind_intent_returning_operations_hard_faults: test
component::plugin_runtime::runtime_close_budget_tests::contended_live_cleanup_does_not_consume_structural_stall_credit: test
component::plugin_runtime::runtime_close_budget_tests::permanently_blocked_live_cleanup_faults_without_claiming_released_ownership: test
component::plugin_runtime::runtime_close_budget_tests::repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit: test
component::plugin_runtime::runtime_close_budget_tests::structural_zero_progress_exhausts_its_exact_close_credit: test
component::plugin_runtime::runtime_instance_registry_tests::cleanup_queue_saturation_preserves_detached_app_ownership: test
component::plugin_runtime::runtime_instance_registry_tests::exhausted_close_generation_is_rejected_before_exact_owner_detachment: test
component::plugin_runtime::runtime_instance_registry_tests::runtime_instance_close_quarantine_never_implicitly_drops_nested_value: test
component::plugin_runtime::runtime_instance_registry_tests::runtime_instance_registry_has_fixed_capacity_collision_and_reuse: test
component::plugin_runtime::runtime_instance_registry_tests::sparse_live_instances_receive_successive_round_robin_turns: test
component::publication_fixture::tests::no_state_mutations_have_empty_rosters_and_reject_all_codec_input: test
component::publication_fixture::tests::publication_leaf_and_aggregate_codecs_are_exact_and_u64_serde_rejects_invalid_numbers: test
component::publication_fixture::tests::publication_leaves_apply_inverse_preserve_identity_diff_and_expose_full_rosters: test
component::reactor::checkpoint::tests::a_checkpoint_pack_encoded_before_task_restarts_existed_still_decodes: test
component::reactor::checkpoint::tests::checkpoint_of_no_instances_round_trips_through_json: test
component::reactor::checkpoint::tests::task_restarts_round_trip_through_json_and_are_exposed_by_the_accessor: test
component::reactor::command_ingress_terminal_tests::accepted_final_page_completes_without_a_follow_up_turn: test
component::reactor::command_ingress_terminal_tests::accepted_final_page_preserves_a_terminal_fault: test
component::reactor::command_ingress_terminal_tests::async_actor_poll_awaits_exchange_and_render_work: test
component::reactor::executor::tests::a_self_waking_task_is_polled_again_within_the_same_pass: test
component::reactor::executor::tests::a_task_that_never_wakes_stays_pending_until_woken: test
component::reactor::executor::tests::blocked_reactor_task_does_not_starve_ready_peer: test
component::reactor::executor::tests::cancel_before_the_first_run_until_idle_drops_the_future_without_ever_polling_it: test
component::reactor::executor::tests::cancel_is_idempotent_for_an_unknown_or_already_finished_id: test
component::reactor::executor::tests::cancel_of_a_parked_task_drops_it_and_frees_its_slot_for_reuse: test
component::reactor::executor::tests::detach_and_reuse_ten_times_capacity_never_accumulates_stale_ready_authority: test
component::reactor::executor::tests::reactor_executor_shutdown_drains_every_slot_before_terminal_drop: test
component::reactor::executor::tests::reactor_task_close_releases_one_nested_owner_per_step_and_only_then_drops_terminal_shell: test
component::reactor::executor::tests::rejected_reactor_task_is_bounded_disposed_without_drop: test
component::reactor::executor::tests::self_detach_during_poll_cannot_steal_or_drop_the_in_flight_future: test
component::reactor::executor::tests::spawn_runs_a_ready_task_to_completion: test
component::reactor::executor::tests::spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs: test
component::reactor::executor::tests::stale_generation_cannot_commit: test
component::reactor::instance_lifetime::tests::guest_instance_lifecycle_ack_fault_keeps_exact_receipt_and_owner: test
component::reactor::instance_lifetime::tests::guest_instance_lifecycle_partial_close_unwind_never_drops_structural_owner: test
component::reactor::instance_lifetime::tests::guest_instance_lifecycle_same_activation_reopen_rejects_old_authority: test
component::reactor::instance_lifetime::tests::guest_instance_lifecycle_terminal_release_work_is_measured_and_never_repeated_after_late_clock: test
component::reactor::job_render_binding_tests::direct_slots_reject_collisions_and_close_exactly_one_instance: test
component::reactor::job_render_binding_tests::progress_accepts_only_the_current_instance_generation: test
component::reactor::jobs::infer::tests::a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service: test
component::reactor::jobs::infer::tests::infer_job_checkpoint_restore_matches_an_uninterrupted_run: test
component::reactor::jobs::infer::tests::infer_job_reports_a_named_decode_fault_on_garbage_input: test
component::reactor::jobs::infer::tests::interactive_bridge_coalesces_preview_but_backpressures_lossless_items: test
component::reactor::jobs::infer::tests::interactive_bridge_diagnostic_ring_is_item_and_byte_bounded: test
component::reactor::jobs::migrate::tests::a_two_slice_migrate_job_decodes_then_dispatches_to_the_registered_migration: test
component::reactor::jobs::migrate::tests::migrate_job_checkpoint_restore_matches_an_uninterrupted_run: test
component::reactor::jobs::migrate::tests::migrate_job_reports_a_named_decode_fault_on_garbage_input: test
component::reactor::jobs::migrate::tests::migrate_job_reports_a_named_fault_when_no_migration_is_registered: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::mutations::add_value::tests::leaf_descriptor_matches_actual_authored_file: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::mutations::add_value::tests::minimum_inverse_is_stored_as_one_then_maximum: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::mutations::add_value::tests::mixed_inverse_groups_stay_forward_before_store_reversal: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::mutations::add_value::tests::neutral_inverse_vectors_restore_in_store_order: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::mutations::add_value::tests::neutral_payload_and_binary_schema_vectors_match_actual_codecs: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::mutations::add_value::tests::ordinary_contributed_plan_keeps_direct_leaf_and_label: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::absorb_preserves_order_and_intermediate_rejection: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::actual_descriptor_provenance: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::binary_codec_round_trip: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::direct_plan_and_inverse_preserve_job_semantics: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::neutral_checked_diff_boundaries_have_typed_rejections: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::neutral_snapshot_and_diff_schema_vectors_match_serde: test
component::reactor::jobs::mutation_plan::job_test_mutation_fixture::tests::ordered_diff_absorb_is_associative_at_boundaries: test
component::reactor::jobs::mutation_plan::tests::a_two_slice_mutation_plan_job_decodes_then_dispatches_to_the_registered_kind: test
component::reactor::jobs::mutation_plan::tests::mutation_plan_job_checkpoint_restore_matches_an_uninterrupted_run: test
component::reactor::jobs::mutation_plan::tests::mutation_plan_job_reports_a_named_decode_fault_on_garbage_input: test
component::reactor::jobs::tests::a_three_slice_job_returns_running_running_done_with_progress_each_slice: test
component::reactor::jobs::tests::cancel_job_removes_a_pending_record_so_a_later_step_fails: test
component::reactor::jobs::tests::cancelling_a_job_mid_slice_frees_its_slot_for_the_id: test
component::reactor::jobs::tests::checkpoint_restore_resumes_and_matches_an_uninterrupted_run: test
component::reactor::jobs::tests::io_run_dispatches_through_the_registry_and_keeps_its_decode_fault_code: test
component::reactor::jobs::tests::io_sniff_dispatches_through_the_registry_and_keeps_its_decode_fault_code: test
component::reactor::jobs::tests::production_bounded_job_path_advances_one_explicit_state_action_and_retains_admission_fault: test
component::reactor::jobs::tests::step_job_on_an_unknown_id_fails_without_panicking: test
component::reactor::jobs::tests::step_job_on_an_unknown_kind_fails_with_a_named_fault: test
component::reactor::jobs::tests::the_budget_a_tick_observes_is_whatever_step_job_most_recently_passed: test
component::reactor::jobs::tests::the_stall_guard_fires_after_repeated_no_progress_static_budget_slices: test
component::reactor::m1_m2_reactor_tests::a_burst_of_same_key_presence_writes_between_polls_coalesces_to_one_update: test
component::reactor::m1_m2_reactor_tests::a_presence_only_turn_emits_presence_and_zero_patches: test
component::reactor::m1_m2_reactor_tests::revision_guard_never_rejects_an_intent_at_the_never_rendered_default: test
component::reactor::m1_m2_reactor_tests::revision_guard_rejects_an_intent_trailing_by_more_than_the_tolerance: test
component::reactor::m1_m2_reactor_tests::ttl_expiry_drops_a_peer_mark_with_no_goodbye_message: test
component::reactor::patches::tests::actor_close_retires_each_surface_and_old_generation_cannot_resume_reopened_slot: test
component::reactor::patches::tests::cap_plus_one_returns_the_exact_tree_owner: test
component::reactor::patches::tests::close_retires_ready_deferred_unadmitted_active_and_terminal_owners_without_stale_publish: test
component::reactor::patches::tests::effects_publish_in_admission_order_even_when_later_tree_finishes_first: test
component::reactor::patches::tests::generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation: test
component::reactor::patches::tests::mounted_catalogue_publishes_every_section_beyond_thirty_two_nodes: test
component::reactor::patches::tests::mounted_catalogue_reports_producer_failure_once_before_cleanup: test
component::reactor::patches::tests::mounted_catalogue_reports_reconcile_capacity_without_leaking_owners: test
component::reactor::patches::tests::mounted_document_tree_publishes_nested_interactive_rows: test
component::reactor::patches::tests::mounted_output_admission_incomplete_producer_sources_preserve_remaining_owners: test
component::reactor::patches::tests::mounted_output_admission_partial_producer_step_unwind_retains_original_slot_and_box: test
component::reactor::patches::tests::mounted_output_admission_refuses_before_tree_when_shared_output_pool_is_full: test
component::reactor::patches::tests::mounted_path_advances_one_reconcile_opportunity_per_grant: test
component::reactor::patches::tests::mounted_reservation_precedes_tree_and_cap_plus_one_returns_exact_owner: test
component::reactor::patches::tests::mounted_settings_controls_publish_with_authored_fields: test
component::reactor::patches::tests::mounted_sources_publish_every_window_and_panel_tree: test
component::reactor::patches::tests::one_active_surface_does_not_wait_behind_sixty_three_empty_slots_between_steps: test
component::reactor::patches::tests::published_owner_first_ack_rejects_early_stale_duplicate_wrong_instance_and_aba_without_authority_loss: test
component::reactor::patches::tests::resize_storm_coalesces_to_one_deferred_surface_owner: test
component::reactor::patches::tests::stale_generation_fault_is_publicly_retrievable: test
component::reactor::patches::tests::terminal_full_plus_matching_rejected_advances_capacity_before_conversion: test
component::reactor::patches::tests::terminal_full_plus_matching_surface_advances_capacity_before_conversion: test
component::reactor::patches::tests::terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion: test
component::reactor::patches::tests::terminal_saturation_does_not_consume_maximum_generation_before_exact_owner_reservation: test
component::reactor::patches::tests::terminal_saturation_keeps_fault_job_in_its_surface_until_one_slot_is_freed: test
component::reactor::patches::tests::tracker_initialization_fits_the_component_stack_budget: test
component::reactor::pending::instance_lifetime_patch_close_tests::guest_instance_lifecycle_pending_patch_handback_preserves_rejected_owner_and_exact_bytes: test
component::reactor::pending::instance_lifetime_patch_close_tests::guest_instance_lifecycle_pending_patch_unwind_keeps_the_exact_typed_cursor_mounted: test
component::reactor::pending::instance_lifetime_patch_close_tests::instance_lifetime_pending_patch_keeps_scope_after_payload_surface_retires: test
component::reactor::reconcile_budget_tests::patch_frame_limit_does_not_limit_internal_reconciliation_steps: test
component::reactor::reconcile_budget_tests::reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps: test
component::reactor::requests::tests::append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op: test
component::reactor::requests::tests::append_chunk_over_cap_faults_instead_of_silently_truncating: test
component::reactor::requests::tests::append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes: test
component::reactor::requests::tests::cancel_instance_on_an_instance_with_no_pending_requests_is_a_harmless_no_op: test
component::reactor::requests::tests::cancel_instance_removes_only_that_instances_pending_requests: test
component::reactor::requests::tests::for_instance_shares_the_same_id_counter_as_the_registry_it_was_derived_from: test
component::reactor::requests::tests::pending_ids_reports_only_unresolved_requests: test
component::reactor::requests::tests::resolve_before_first_poll_leaves_the_future_immediately_ready: test
component::reactor::shell_fault_frame_tests::shell_fault_frame_round_trips_the_language_neutral_diagnostic: test
component::retained_command::tests::checkpoint_binary_matches_schema_fixture_and_owned_oracle: test
component::retained_command::tests::checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift: test
component::retained_command::tests::owned_little_endian_oracle_preserves_every_hostile_byte_lane: test
component::subset_macro_tests::__subset_registration::conformance::subset_macro_derived_dialect_is_non_any: test
component::subset_macro_tests::__subset_registration::conformance::subset_macro_derived_validator_registers: test
component::subset_macro_tests::subset_macro_derived_kind_and_dialect: test
component::subset_macro_tests::subset_macro_derived_register_is_idempotent: test
component::test_app_mutation_fixture::config::mutations::change_test_config_selection::tests::nullable_selection_serde_text_and_binary_round_trip: test
component::test_app_mutation_fixture::config::mutations::change_test_config_selection::tests::structural_config_diff_serde_preserves_identity_clear_and_set: test
component::test_app_mutation_fixture::document::mutations::set_count::tests::descriptor_has_set_count_identity: test
component::test_app_mutation_fixture::document::mutations::set_label::tests::descriptor_has_set_label_identity: test
component::test_app_mutation_fixture::document::mutations::tests::direct_leaves_preserve_generic_document_codecs_and_laws: test
component::world3d_host::tests::apply_action_switches_kind_and_leaves_other_kinds_untouched_for_later_recall: test
component::world3d_host::tests::isometric_pose_matches_the_classic_35_264_45_direction: test
component::world3d_host::tests::merge_world_selection_ids_supports_add_toggle_invertive_and_remove: test
component::world3d_host::tests::projection_measures_tree_matches_the_requested_taxonomy: test
component::world3d_host::tests::projection_spec_json_projects_only_active_kind_fields: test
component::world3d_host::tests::selection_set_membership_is_constant_time: test

517 tests, 0 benchmarks
```

## Combined Execution

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply' 'component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits' 'component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation' 'component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id' 'component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper' 'component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect' 'component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id' 'component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally' 'component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id' 'component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper' 'component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id' 'component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store' 'component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb' 'component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work' 'component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place' 'component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs' 'component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin' 'component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying' 'component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code' 'component::app::mutation_fixture::transaction::plain_command_still_applies_normally' 'component::app::mutation_fixture::transaction::rollback_leaves_state_untouched' 'component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy' 'component::app::mutation_fixture::transaction::undo_and_redo_by_group' 'component::plugin_runtime::plugin_builder_contract_tests::keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-fixtures-r1-native-2026-08-27.txt'
```

```text

running 24 tests
test component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id ... ok
test component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect ... ok
test component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store ... ok

thread 'component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits' (8015627) has overflowed its stack
fatal runtime error: stack overflow, aborting
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply" "component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits" "component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation" "component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id" "component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper" "component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect" "component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id" "component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally" "component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id" "component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper" "component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id" "component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store" "component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb" "component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work" "component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place" "component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs" "component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin" "component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying" "component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code" "component::app::mutation_fixture::transaction::plain_command_still_applies_normally" "component::app::mutation_fixture::transaction::rollback_leaves_state_untouched" "component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy" "component::app::mutation_fixture::transaction::undo_and_redo_by_group" "component::plugin_runtime::plugin_builder_contract_tests::keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 99990,
  stdout: null,
  stderr: null
}
```

## Isolated Case 1

`component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-01-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply' (8025630) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6757:66:
apply command: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::testkit::assert_ingest_idempotent::<semio_framework_plugin::component::app::mutation_fixture::dummy::DummyApp, i32, semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}::{closure#0}>::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply
   8: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply' (8025630) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106e5af50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106e6e2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106e5f67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106e42054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106e549ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106e54d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106e42108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106e377c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106e42734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106ea3ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x1050d6b14 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummySnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::mutations::DummyMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104fc7ea0 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummySnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::mutations::DummyMutation>>
  12:        0x104fb5248 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp>>
  13:        0x1069d1ca8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::testkit::assert_ingest_idempotent::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp, i32, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}::{closure#0}>::{closure#0}
  14:        0x1069703d4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}
  15:        0x10696f35c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}>
  16:        0x106398e58 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply
  17:        0x10696fac0 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}
  18:        0x1051393e8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  19:        0x1069f3084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  20:        0x1069fe3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  21:        0x1069f9298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  22:        0x106a00880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:        0x106e5a8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  24:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply' (8025630) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106e5af50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106e6e2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106e5f67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106e42054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106e549ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106e54d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106e42108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106e377c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106e42734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106ea3ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x1050d5454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104fc7e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x104fb525c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp>>
  13:        0x1069d1ca8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::testkit::assert_ingest_idempotent::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp, i32, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}::{closure#0}>::{closure#0}
  14:        0x1069703d4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}
  15:        0x10696f35c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}>
  16:        0x106398e58 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply
  17:        0x10696fac0 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0}
  18:        0x1051393e8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  19:        0x1069f3084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  20:        0x1069fe3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  21:        0x1069f9298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  22:        0x106a00880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  23:        0x106e5a8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  24:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply' (8025630) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::dummy::assert_ingest_idempotent_does_not_double_apply" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 1635,
  stdout: null,
  stderr: null
}
```

## Isolated Case 2

`component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-02-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits' (8026451) has overflowed its stack
fatal runtime error: stack overflow, aborting
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 1742,
  stdout: null,
  stderr: null
}
```

## Isolated Case 3

`component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-03-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation' (8026707) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6669:63:
apply command: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::testkit::assert_undo_redo_round_trip::<semio_framework_plugin::component::app::mutation_fixture::dummy::DummyApp, i32, semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}::{closure#0}>::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation
   8: semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation' (8026707) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106776f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x10678a2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x10677b67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x10675e054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1067709ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106770d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x10675e108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1067537c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x10675e734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x1067bfae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x1049f2b14 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummySnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::mutations::DummyMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x1048e3ea0 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummySnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::mutations::DummyMutation>>
  12:        0x1048d1248 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp>>
  13:        0x10628c8ec - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}
  14:        0x10628b63c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}>
  15:        0x105cb4f0c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation
  16:        0x10628bb10 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}
  17:        0x104a55460 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10630f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x10631a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106315298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x10631c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1067768a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation' (8026707) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106776f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x10678a2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x10677b67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x10675e054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1067709ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106770d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x10675e108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1067537c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x10675e734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x1067bfae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x1049f1454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x1048e3e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x1048d125c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp>>
  13:        0x10628c8ec - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}
  14:        0x10628b63c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}>
  15:        0x105cb4f0c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation
  16:        0x10628bb10 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0}
  17:        0x104a55460 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10630f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x10631a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106315298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x10631c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1067768a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation' (8026707) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::dummy::assert_undo_redo_round_trip_passes_for_a_real_operation" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 1774,
  stdout: null,
  stderr: null
}
```

## Isolated Case 4

`component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-04-2026-08-27.txt'
```

```text

running 1 test
test component::app::mutation_fixture::dummy::meta_carries_actor_and_local_instance_id ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 516 filtered out; finished in 0.00s

```

## Isolated Case 5

`component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-05-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper' (8028866) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🎲️dummy/🦀️.rs:154:71:
increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper
   7: semio_framework_plugin::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper' (8028866) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1046c6f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1046da2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1046cb67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1046ae054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1046c09ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1046c0d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1046ae108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1046a37c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1046ae734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x10470fae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102942b14 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummySnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::mutations::DummyMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102833ea0 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummySnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::mutations::DummyMutation>>
  12:        0x102821248 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp>>
  13:        0x1041dbf20 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}
  14:        0x1041db1ec - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}>
  15:        0x103c04dfc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper
  16:        0x1041dba98 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}
  17:        0x1029a53ac - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10425f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x10426a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104265298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x10426c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1046c68a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper' (8028866) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1046c6f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1046da2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1046cb67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1046ae054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1046c09ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1046c0d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1046ae108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1046a37c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1046ae734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x10470fae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102941454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102833e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x10282125c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::DummyApp>>
  13:        0x1041dbf20 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}
  14:        0x1041db1ec - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}>
  15:        0x103c04dfc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper
  16:        0x1041dba98 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0}
  17:        0x1029a53ac - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10425f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x10426a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104265298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x10426c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1046c68a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper' (8028866) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::dummy::new_app_constructs_a_registry_less_wrapper" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 2029,
  stdout: null,
  stderr: null
}
```

## Isolated Case 6

`component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-06-2026-08-27.txt'
```

```text

running 1 test
test component::app::mutation_fixture::surface::editor_and_viewer_share_one_dialect ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 516 filtered out; finished in 0.00s

```

## Isolated Case 7

`component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-07-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id' (8029992) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::EditorApp<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceEditorFixture>>>
   5: semio_framework_plugin::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id
   8: semio_framework_plugin::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id' (8029992) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x107116f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x10712a2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x10711b67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1070fe054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1071109ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x107110d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1070fe108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1070f37c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1070fe734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x10715fae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x105391454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x105283e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x10526e728 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::EditorApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceEditorFixture>>>
  13:        0x106a7529c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
  14:        0x106a72d3c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}>
  15:        0x1066550c4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id
  16:        0x106a7333c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
  17:        0x1053f55c8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x106caf084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x106cba3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106cb5298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x106cbc880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1071168a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id' (8029992) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::surface::editor_app_envelopes_carry_the_real_canonical_surface_app_id" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 2168,
  stdout: null,
  stderr: null
}
```

## Isolated Case 8

`component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-08-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally' (8030781) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🦀️.rs:212:79:
increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally
   7: semio_framework_plugin::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally' (8030781) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106bc2f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106bd62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106bc767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106baa054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106bbc9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106bbcd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106baa108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106b9f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106baa734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106c0bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104e3ee54 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::mutations::SurfaceMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104d2feb4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::mutations::SurfaceMutation>>
  12:        0x104d1a714 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::EditorApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceEditorFixture>>>
  13:        0x10651f7c4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}
  14:        0x10651e7c8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}>
  15:        0x106100f90 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally
  16:        0x10651f29c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}
  17:        0x104ea14d8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10675b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1067663b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106761298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x106768880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106bc28a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally' (8030781) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106bc2f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106bd62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106bc767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106baa054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106bbc9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106bbcd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106baa108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106b9f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106baa734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106c0bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104e3d454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104d2fe14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x104d1a728 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::EditorApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceEditorFixture>>>
  13:        0x10651f7c4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}
  14:        0x10651e7c8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}>
  15:        0x106100f90 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally
  16:        0x10651f29c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0}
  17:        0x104ea14d8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10675b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1067663b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106761298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x106768880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106bc28a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally' (8030781) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::surface::editor_fixture_still_mutates_normally" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 2301,
  stdout: null,
  stderr: null
}
```

## Isolated Case 9

`component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-09-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id' (8031686) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::EditorApp<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceEditorFixture>>>
   5: semio_framework_plugin::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id
   8: semio_framework_plugin::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id' (8031686) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1064cef50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1064e22c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1064d367c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1064b6054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1064c89ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1064c8d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1064b6108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1064ab7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1064b6734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106517ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104749454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x10463be14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x104626728 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::EditorApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceEditorFixture>>>
  13:        0x105e2e600 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0}
  14:        0x105e2b018 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0}>
  15:        0x105a0d174 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id
  16:        0x105e2b38c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0}
  17:        0x1047ad640 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x106067084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1060723b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x10606d298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x106074880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1064ce8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id' (8031686) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::surface::handle_action_invocation_accepts_the_real_canonical_surface_app_id" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 2463,
  stdout: null,
  stderr: null
}
```

## Isolated Case 10

`component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-10-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper' (8032624) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::ViewerApp<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceViewerFixture>>>
   5: semio_framework_plugin::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper
   8: semio_framework_plugin::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper' (8032624) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x104abef50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104ad22c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x104ac367c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x104aa6054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x104ab89ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x104ab8d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x104aa6108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x104a9b7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x104aa6734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104b07ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102d39454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102c2be14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x102c175b4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::ViewerApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceViewerFixture>>>
  13:        0x10441caac - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0}
  14:        0x10441aaa4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0}>
  15:        0x103ffd040 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper
  16:        0x10441b2ec - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0}
  17:        0x102d9d550 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x104657084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1046623b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x10465d298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104664880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x104abe8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper' (8032624) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::surface::new_viewer_constructs_a_registry_less_wrapper" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 2614,
  stdout: null,
  stderr: null
}
```

## Isolated Case 11

`component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-11-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id' (8033881) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin::component::app::mutation_fixture::surface::mutations::SurfaceMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::ViewerApp<semio_framework_plugin::component::app::mutation_fixture::surface::SurfaceViewerFixture>>>
   5: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id
   8: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id' (8033881) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1061eaf50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1061fe2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1061ef67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1061d2054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1061e49ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1061e4d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1061d2108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1061c77c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1061d2734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106233ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104465454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104357e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x1043435b4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::ViewerApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceViewerFixture>>>
  13:        0x105b49994 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
  14:        0x105b46ea8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}>
  15:        0x10572911c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id
  16:        0x105b47364 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0}
  17:        0x1044c9604 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x105d83084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x105d8e3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x105d89298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x105d90880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1061ea8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id' (8033881) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::surface::viewer_app_envelopes_carry_the_real_canonical_surface_app_id" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 2785,
  stdout: null,
  stderr: null
}
```

## Isolated Case 12

`component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-12-2026-08-27.txt'
```

```text

running 1 test
test component::app::mutation_fixture::surface::viewer_never_mutates_the_document_or_draft_store ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 516 filtered out; finished in 0.01s

```

## Isolated Case 13

`component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-13-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb' (8036165) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🪟️surface/🦀️.rs:275:9:
assertion `left == right` failed: 'undo' rejection must carry the frozen viewer.read-only code
  left: "interactive-job.unknown-key"
 right: "viewer.read-only"
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<alloc::string::String, &str>
   4: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb
   7: semio_framework_plugin::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb' (8036165) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x104f92f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104fa62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x104f9767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x104f7a054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x104f8c9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x104f8cd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x104f7a108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x104f6f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x104f7a734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104fdbae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x10320ee54 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::mutations::SurfaceMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x1030ffeb4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::mutations::SurfaceMutation>>
  12:        0x1030eb5a0 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::ViewerApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceViewerFixture>>>
  13:        0x1048efe50 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}
  14:        0x1048ee938 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}>
  15:        0x1044d0fe8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb
  16:        0x1048ef2c4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}
  17:        0x103271514 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x104b2b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x104b363b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104b31298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104b38880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x104f928a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb' (8036165) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x104f92f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104fa62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x104f9767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x104f7a054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x104f8c9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x104f8cd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x104f7a108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x104f6f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x104f7a734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104fdbae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x10320d454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x1030ffe14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x1030eb5b4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::ViewerApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::SurfaceViewerFixture>>>
  13:        0x1048efe50 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}
  14:        0x1048ee938 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}>
  15:        0x1044d0fe8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb
  16:        0x1048ef2c4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0}
  17:        0x103271514 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x104b2b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x104b363b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104b31298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104b38880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x104f928a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb' (8036165) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::surface::viewer_rejects_every_contract_mutating_verb" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 3068,
  stdout: null,
  stderr: null
}
```

## Isolated Case 14

`component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-14-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work' (8037052) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:295:5:
assertion `left == right` failed
  left: "interactive-job.missing-factory"
 right: "transaction.instance-busy"
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::panicking::assert_failed_inner
   3: core::panicking::assert_failed::<alloc::string::String, &str>
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work' (8037052) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106a86f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106a9a2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106a8b67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106a6e054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106a809ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106a80d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106a6e108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106a637c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106a6e734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106acfae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104d027d4 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104bf3e8c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
  12:        0x104be03f4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1064b405c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}
  14:        0x1064addd4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}>
  15:        0x105fc4d20 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work
  16:        0x1064ae2a4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}
  17:        0x104d652f8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10661f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x10662a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106625298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x10662c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106a868a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work' (8037052) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106a86f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106a9a2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106a8b67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106a6e054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106a809ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106a80d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106a6e108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106a637c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106a6e734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106acfae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104d01454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104bf3e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x104be0408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1064b405c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}
  14:        0x1064addd4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}>
  15:        0x105fc4d20 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work
  16:        0x1064ae2a4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0}
  17:        0x104d652f8 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10661f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x10662a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106625298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x10662c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106a868a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work' (8037052) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::a_mutating_command_while_pending_is_rejected_but_reads_still_work" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 3180,
  stdout: null,
  stderr: null
}
```

## Isolated Case 15

`component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-15-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place' (8037449) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:218:78:
first increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place' (8037449) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1049eef50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104a022c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1049f367c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1049d6054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1049e89ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1049e8d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1049d6108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1049cb7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1049d6734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104a37ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102c6a7d4 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102b5be8c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
  12:        0x102b483f4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x10441887c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}
  14:        0x104415820 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}>
  15:        0x103f2cbc0 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place
  16:        0x104416204 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}
  17:        0x102ccd208 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x104587084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1045923b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x10458d298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104594880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1049ee8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place' (8037449) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1049eef50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104a022c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1049f367c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1049d6054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1049e89ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1049e8d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1049d6108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x1049cb7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1049d6734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104a37ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102c69454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102b5be14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x102b48408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x10441887c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}
  14:        0x104415820 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}>
  15:        0x103f2cbc0 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place
  16:        0x104416204 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0}
  17:        0x102ccd208 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x104587084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1045923b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x10458d298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104594880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1049ee8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place' (8037449) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::amended_edit_extends_cached_history_in_place" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 3234,
  stdout: null,
  stderr: null
}
```

## Isolated Case 16

`component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-16-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs' (8038412) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/👥️presence/♻️retirement/🦀️component.rs:200:13:
presence store requires its exact detached terminal-empty owner before Drop
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::PresenceStore<semio_framework_plugin::component::app::NoPresence, semio_framework_plugin::component::app::NoPresenceMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::PresenceStore<semio_framework_plugin::component::app::NoPresence, semio_framework_plugin::component::app::NoPresenceMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnApp>>
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs
   8: semio_framework_plugin::component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
test component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs ... FAILED

failures:

failures:
    component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 516 filtered out; finished in 0.04s

Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::command_cache_inputs_share_immutable_arcs" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: 101,
  signal: null,
  output: [ null, null, null ],
  pid: 3346,
  stdout: null,
  stderr: null
}
```

## Isolated Case 17

`component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-17-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin' (8040945) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnApp>>
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin
   8: semio_framework_plugin::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin' (8040945) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106b2af50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106b3e2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106b2f67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106b12054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106b249ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106b24d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106b12108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106b077c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106b12734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106b73ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x104da5454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x104c97e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x104c84408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x106557d68 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0}
  14:        0x106551c68 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0}>
  15:        0x106068cc8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin
  16:        0x10655227c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0}
  17:        0x104e092bc - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x1066c3084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1066ce3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x1066c9298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x1066d0880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106b2a8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin' (8040945) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::commit_produces_exactly_one_edit_with_group_id_and_origin" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 3745,
  stdout: null,
  stderr: null
}
```

## Isolated Case 18

`component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-18-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying' (8044870) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️component.rs:6781:63:
dispatch a command whose mutations carry foreign steps: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::testkit::assert_proposes_transaction::<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnApp>::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying
   8: semio_framework_plugin::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying' (8044870) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x102a32f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x102a462c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x102a3767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x102a1a054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x102a2c9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x102a2cd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x102a1a108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x102a0f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x102a1a734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x102a7bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x100cae7d4 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x100b9fe8c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
  12:        0x100b8c3f4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x102460aa8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}
  14:        0x102459f40 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}>
  15:        0x101f70d78 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying
  16:        0x10245a2cc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}
  17:        0x100d11334 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x1025cb084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1025d63b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x1025d1298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x1025d8880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x102a328a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying' (8044870) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x102a32f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x102a462c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x102a3767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x102a1a054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x102a2c9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x102a2cd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x102a1a108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x102a0f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x102a1a734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x102a7bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x100cad454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x100b9fe14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x100b8c408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x102460aa8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}
  14:        0x102459f40 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}>
  15:        0x101f70d78 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying
  16:        0x10245a2cc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0}
  17:        0x100d11334 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x1025cb084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1025d63b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x1025d1298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x1025d8880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x102a328a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying' (8044870) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::dispatching_a_mutation_with_foreign_steps_proposes_instead_of_applying" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 4161,
  stdout: null,
  stderr: null
}
```

## Isolated Case 19

`component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-19-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code' (8046443) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:260:73:
the peer edits its own copy: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code' (8046443) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106ec6f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106eda2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106ecb67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106eae054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106ec09ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106ec0d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106eae108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106ea37c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106eae734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106f0fae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x1051427d4 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x105033e8c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
  12:        0x1050203f4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1068f1454 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}
  14:        0x1068ed990 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}>
  15:        0x106404c18 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code
  16:        0x1068ee22c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}
  17:        0x1051a5244 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x106a5f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x106a6a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106a65298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x106a6c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106ec68a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code' (8046443) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x106ec6f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x106eda2c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x106ecb67c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x106eae054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x106ec09ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x106ec0d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x106eae108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x106ea37c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x106eae734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x106f0fae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x105141454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x105033e14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x105020408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1068f1454 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}
  14:        0x1068ed990 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}>
  15:        0x106404c18 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code
  16:        0x1068ee22c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0}
  17:        0x1051a5244 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x106a5f084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x106a6a3b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x106a65298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x106a6c880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x106ec68a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code' (8046443) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::generation_mismatch_is_rejected_with_the_frozen_code" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 4335,
  stdout: null,
  stderr: null
}
```

## Isolated Case 20

`component::app::mutation_fixture::transaction::plain_command_still_applies_normally`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::plain_command_still_applies_normally' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-20-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::plain_command_still_applies_normally' (8047132) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:198:69:
increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::plain_command_still_applies_normally
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::plain_command_still_applies_normally' (8047132) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x104cc2f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104cd62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x104cc767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x104caa054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x104cbc9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x104cbcd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x104caa108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x104c9f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x104caa734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104d0bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102f3e7d4 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102e2fe8c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
  12:        0x102e1c3f4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1046eba7c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}
  14:        0x1046e9548 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}>
  15:        0x104200b10 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally
  16:        0x1046ea1b4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}
  17:        0x102fa1190 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10485b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1048663b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104861298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104868880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x104cc28a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::plain_command_still_applies_normally' (8047132) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x104cc2f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x104cd62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x104cc767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x104caa054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x104cbc9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x104cbcd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x104caa108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x104c9f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x104caa734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x104d0bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x102f3d454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x102e2fe14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x102e1c408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1046eba7c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}
  14:        0x1046e9548 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}>
  15:        0x104200b10 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally
  16:        0x1046ea1b4 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0}
  17:        0x102fa1190 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::plain_command_still_applies_normally::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10485b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1048663b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104861298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104868880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x104cc28a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::plain_command_still_applies_normally' (8047132) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::plain_command_still_applies_normally" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 4448,
  stdout: null,
  stderr: null
}
```

## Isolated Case 21

`component::app::mutation_fixture::transaction::rollback_leaves_state_untouched`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::rollback_leaves_state_untouched' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-21-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::rollback_leaves_state_untouched' (8050605) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../.././🧪️tests/🧬️mutation-fixtures/🔀️transaction/🦀️.rs:243:69:
increment: Fault { origin: Framework, code: FaultCode("interactive-job.missing-factory"), severity: Error, message: "typed command 'typed-command' has no exact controller/owner/factory/tool/schema proof", scope: FaultScope { plugin_id: None, app_id: None, instance_id: None, module: None, body_key: None }, span: None, causes: [], retryable: false }
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   3: <core::result::Result<semio_framework::manifest::kernel::InvocationResult, protocol::diagnostic::Fault>>::expect
   4: semio_framework_plugin::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}>
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}
   8: <semio_framework_plugin::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::rollback_leaves_state_untouched' (8050605) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1044c2f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1044d62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1044c767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1044aa054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1044bc9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1044bcd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1044aa108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x10449f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1044aa734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x10450bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x10273e7d4 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x10262fe8c - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
  12:        0x10261c3f4 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x103eeb208 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}
  14:        0x103ee93dc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}>
  15:        0x103a00ab8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched
  16:        0x103eea18c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}
  17:        0x1027a1154 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10405b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1040663b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104061298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104068880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1044c28a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::rollback_leaves_state_untouched' (8050605) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1044c2f50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1044d62c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1044c767c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x1044aa054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1044bc9ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1044bcd14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x1044aa108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x10449f7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x1044aa734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x10450bae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x10273d454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x10262fe14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x10261c408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x103eeb208 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}
  14:        0x103ee93dc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}>
  15:        0x103a00ab8 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched
  16:        0x103eea18c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0}
  17:        0x1027a1154 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::rollback_leaves_state_untouched::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x10405b084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x1040663b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x104061298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x104068880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1044c28a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::rollback_leaves_state_untouched' (8050605) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::rollback_leaves_state_untouched" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 4930,
  stdout: null,
  stderr: null
}
```

## Isolated Case 22

`component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-22-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy' (8052767) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnApp>>
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy
   8: semio_framework_plugin::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy' (8052767) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x1023aef50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x1023c22c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x1023b367c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x102396054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x1023a89ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x1023a8d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x102396108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x10238b7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x102396734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x1023f7ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x100629454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x10051be14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x100508408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x101ddb21c - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0}
  14:        0x101dd5afc - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0}>
  15:        0x1018ecc70 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy
  16:        0x101dd6254 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0}
  17:        0x10068d280 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x101f47084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x101f523b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x101f4d298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x101f54880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x1023ae8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy' (8052767) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::second_prepare_while_pending_is_rejected_instance_busy" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 5179,
  stdout: null,
  stderr: null
}
```

## Isolated Case 23

`component::app::mutation_fixture::transaction::undo_and_redo_by_group`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::app::mutation_fixture::transaction::undo_and_redo_by_group' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-23-2026-08-27.txt'
```

```text

running 1 test

thread 'component::app::mutation_fixture::transaction::undo_and_redo_by_group' (8054634) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0: __rustc::rust_begin_unwind
   1: core::panicking::panic_fmt
   2: <semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin::component::app::mutation_fixture::transaction::mutations::TxnMutation> as core::ops::drop::Drop>::drop
   3: core::ptr::drop_glue::<semio_framework_os_kernel::os_store::component::ArtifactStore<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnSnapshot, semio_framework_plugin::component::app::mutation_fixture::transaction::mutations::TxnMutation>>
   4: core::ptr::drop_glue::<semio_framework_plugin::component::app::VcsArtifactApp<semio_framework_plugin::component::app::mutation_fixture::transaction::TxnApp>>
   5: semio_framework_plugin::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0}
   6: semio_framework_plugin::component::app::mutation_fixture::transaction::undo_and_redo_by_group::__semio_async_test_block_on::<semio_framework_plugin::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0}>
   7: semio_framework_plugin::component::app::mutation_fixture::transaction::undo_and_redo_by_group
   8: semio_framework_plugin::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0}
   9: <semio_framework_plugin::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0} as core::ops::function::FnOnce<()>>::call_once
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

thread 'component::app::mutation_fixture::transaction::undo_and_redo_by_group' (8054634) panicked at 🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/./../../🔨️modules/🏪️store/🦀️component.rs:16389:9:
artifact store reached Drop without its exact terminal-empty shallow-shell witness
stack backtrace:
   0:        0x102faef50 - <<std[87758e35c17852a5]::sys::backtrace::BacktraceLock>::print::DisplayBacktrace as core[c6c0a6c66382aec3]::fmt::Display>::fmt
   1:        0x102fc22c4 - core[c6c0a6c66382aec3]::fmt::write
   2:        0x102fb367c - <std[87758e35c17852a5]::sys::stdio::unix::Stderr as std[87758e35c17852a5]::io::Write>::write_fmt
   3:        0x102f96054 - std[87758e35c17852a5]::panicking::default_hook::{closure#0}
   4:        0x102fa89ec - std[87758e35c17852a5]::panicking::default_hook
   5:        0x102fa8d14 - std[87758e35c17852a5]::panicking::panic_with_hook
   6:        0x102f96108 - std[87758e35c17852a5]::panicking::panic_handler::{closure#0}
   7:        0x102f8b7c0 - std[87758e35c17852a5]::sys::backtrace::__rust_end_short_backtrace::<std[87758e35c17852a5]::panicking::panic_handler::{closure#0}, !>
   8:        0x102f96734 - __rustc[feecb8598a58626c]::rust_begin_unwind
   9:        0x102ff7ae0 - core[c6c0a6c66382aec3]::panicking::panic_fmt
  10:        0x101229454 - <semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation> as core[c6c0a6c66382aec3]::ops::drop::Drop>::drop
  11:        0x10111be14 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_os_kernel[b836a807ea2fba25]::os_store::component::ArtifactStore<semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfig, semio_framework_plugin[22fb8bccf0b2bad]::component::app::NoConfigMutation>>
  12:        0x101108408 - core[c6c0a6c66382aec3]::ptr::drop_glue::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::VcsArtifactApp<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::TxnApp>>
  13:        0x1029d6f14 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0}
  14:        0x1029d5270 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::undo_and_redo_by_group::__semio_async_test_block_on::<semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0}>
  15:        0x1024eca60 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::undo_and_redo_by_group
  16:        0x1029d6164 - semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0}
  17:        0x10128d118 - <semio_framework_plugin[22fb8bccf0b2bad]::component::app::mutation_fixture::transaction::undo_and_redo_by_group::{closure#0} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once
  18:        0x102b47084 - test[ee52d9429afbedb2]::__rust_begin_short_backtrace::<core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>, fn() -> core[c6c0a6c66382aec3]::result::Result<(), alloc[659a9e145e4cda22]::string::String>>
  19:        0x102b523b8 - test[ee52d9429afbedb2]::run_test::{closure#0}
  20:        0x102b4d298 - std[87758e35c17852a5]::sys::backtrace::__rust_begin_short_backtrace::<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>
  21:        0x102b54880 - <std[87758e35c17852a5]::thread::lifecycle::spawn_unchecked<test[ee52d9429afbedb2]::run_test::{closure#1}, ()>::{closure#1} as core[c6c0a6c66382aec3]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  22:        0x102fae8a4 - <std[87758e35c17852a5]::sys::thread::unix::Thread>::new::thread_start
  23:        0x188071c58 - __pthread_cond_wait

thread 'component::app::mutation_fixture::transaction::undo_and_redo_by_group' (8054634) panicked at /rustc/c4af71034e89a431eeee91125a31ad001379faac/library/core/src/panicking.rs:233:5:
panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
Error: Command failed: "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8" "--exact" "component::app::mutation_fixture::transaction::undo_and_redo_by_group" "--nocapture"
    at genericNodeError (node:internal/errors:985:15)
    at wrappedFn (node:internal/errors:539:14)
    at checkExecSyncError (node:child_process:925:11)
    at execSync (node:child_process:997:15)
    at /Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:60:38
    at Array.forEach (<anonymous>)
    at runScriptAsNxTarget (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:58:19)
    at Object.nxExecCommand (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/exec.js:42:16)
    at async Object.handler (/Users/ueli/Documents/semio/node_modules/nx/src/command-line/exec/command-object.js:11:13) {
  status: null,
  signal: 'SIGABRT',
  output: [ null, null, null ],
  pid: 5376,
  stdout: null,
  stderr: null
}
```

## Isolated Case 24

`component::plugin_runtime::plugin_builder_contract_tests::keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners`

```sh
set -o pipefail
SEMIO_COVERAGE=0 NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 bun x nx exec --projects=@semio-tech/framework-plugin --skip-nx-cache -- '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_plugin-d81b8cb7f98afff8' --exact 'component::plugin_runtime::plugin_builder_contract_tests::keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners' --nocapture 2>&1 | tee '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-plugin-mutation-case-r1-24-2026-08-27.txt'
```

```text

running 1 test
test component::plugin_runtime::plugin_builder_contract_tests::keyed_fixture_no_state_disposers_and_retirement_factories_close_live_owners ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 516 filtered out; finished in 0.01s

```

