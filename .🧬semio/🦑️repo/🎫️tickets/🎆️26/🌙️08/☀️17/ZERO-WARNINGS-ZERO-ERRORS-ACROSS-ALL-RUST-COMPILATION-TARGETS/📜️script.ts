import { readFileSync, writeFileSync, appendFileSync, existsSync, readdirSync } from "node:fs";
import { join, resolve, relative, dirname, basename } from "node:path";

const [command, name, filter] = process.argv.slice(2);
if (command === "laws") {
  const { runExactCargoLaws } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts"));
  const generated = resolve(import.meta.dir, "🗑️generated");
  const report = join(import.meta.dir, "📓️2026-09-07-verification.md");
  const groups = [
    {"package":"semio-framework","target":{"kind":"lib" as const},"laws":["io_compose_via_chains_two_registered_hops","io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry"]},
    {"package":"semio-s-plugin-stdio","target":{"kind":"lib" as const},"laws":["artifacts::binary::component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer","registered_migration_runs_end_to_end_through_the_store_registry","mutation_rejection_messages_match_the_language_neutral_json_oracle","mutation_restore_preserves_the_language_neutral_wire_and_inverse"]},
    {"package":"semio-s-plugin-gis","target":{"kind":"test" as const,"name":"native_codecs"},"laws":["gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution","gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace"]},
    {"package":"semio-s-plugin-vcs","target":{"kind":"test" as const,"name":"native_codecs"},"laws":["vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution","vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind"]},
    { package: "semio-s-plugin-space", laws: ["folds_visibility_and_members_for_this_space_into_config", "open_artifact_relays_with_document_and_space_ids", "open_artifact_with_relays_the_explicit_choice"] },
    { package: "semio-s-plugin-norm", laws: ["qk_working_table_is_owned_by_the_exact_child", "climate_working_data_is_owned_by_the_exact_child", "render_report_falls_back_to_a_placeholder_when_nothing_was_computed", "render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index"] },
    {"package":"semio-framework-ui-runtime","laws":["mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate","deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close","resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics","persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement","round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot"]},
    {"package":"semio-s-plugin-process","laws":["host_contributions_resolve_to_the_event_sourced_config_lane","process_machine_contributions_are_configuration_owned","registry_enforced_app_accepts_a_declared_operation_action"]},
    {"package":"semio-framework-job","laws":["payload_ledger_identity_must_match_the_exact_step_context","retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned","retained_state_and_output_have_separate_credits_and_close_one_page_per_grant","retained_writer_and_reader_advance_exactly_one_page_per_opportunity","worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact","worker_pool_rejection_returns_exact_job_before_resume","worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned"]},
    {"package":"semio-framework-pack","laws":["canonical_bytes_match_serde_json_for_typical_documents","language_neutral_retained_law_ledger_is_complete","retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable","retained_anchor_rejects_hostile_crc_and_requires_explicit_close","retained_identity_chunk_fragment_parity_exact_boundary_and_interrupted_finish","identity_chunk_cursor_retains_fragment_progress_and_terminal_verification"]},
    {"package":"semio-framework-ui-contract","laws":["retained_document_assembly_places_exact_pages_and_preserves_wire_and_payload_pointer","retained_document_assembly_rejects_duplicate_without_consuming_input_and_cancels_exact_backing","retained_document_assembly_and_read_alias_do_not_wait_on_contended_arena","retained_document_assembly_reports_metadata_initialization_separately_from_empty_payload_capacity","retained_document_root_permit_nine_surfaces_share_one_aggregate","retained_document_root_permit_last_reader_keeps_credit_and_typed_payload","retained_document_root_permit_cancel_and_contended_final_return_keep_exact_owner","retained_document_root_permit_reader_pressure_refuses_then_retries_exact_slot","retained_document_root_permit_seal_transfers_output_without_detaching_root_credit"]},
    { package: "semio-framework-replication", laws: ["artifact_bootstrap_hashes_match_neutral_fixture", "artifact_bootstrap_frames_match_neutral_vectors", "artifact_bootstrap_rejects_malformed_transfers_atomically", "artifact_bootstrap_cancellation_is_atomic_and_restartable", "server_frame_welcome_round_trips_for_every_bootstrap_variant", "fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time", "causal_insert_rejects_oversized_identity_without_losing_the_envelope_owner"] },
    { package: "semio-framework-pixels", laws: ["gradient_checkerboard_round_trip", "scanline_decoder_matches_batch_decode", "oracle_decodes_our_encode", "our_decode_reads_oracle_encode", "our_decode_reads_oracle_palette_encode", "zlib_compress_decompress_round_trip"] },
    { package: "semio-framework-deflate", laws: ["reads_miniz_oxide_dynamic_huffman_blocks", "ours_inflates_miniz_oxide_output_and_vice_versa", "stream_produces_the_same_bytes_as_one_shot_inflate"] },
    { package: "semio-framework-hash", laws: ["sha256_matches_nist_vectors_and_segmented_input", "hash_bytes_agrees_with_the_blake3_oracle_across_lengths", "hasher_agrees_with_the_blake3_oracle_for_segmented_updates"] },
    { package: "semio-framework-trace", laws: ["clock_is_monotonically_non_decreasing"] },
    { package: "semio-framework-raster", laws: ["align_bytes_per_row_pads_to_wgpu_alignment", "scene_rasterizer_renders_expected_pixel_count"] },
    { package: "semio-framework-os", laws: ["codec_abi::tests::schema_and_language_neutral_fixture_cover_every_operation", "codec_abi::tests::valid_pack_and_dsl_are_equivalent_deterministic_paged_replies", "codec_abi::tests::workflow_pack_and_dsl_accept_every_byte_and_field_split", "codec_abi::tests::deadline_interruption_and_zero_credit_do_not_advance_any_structural_cursor", "demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping", "creates_and_lists_space_catalog_entries"] },
    { package: "semio-framework-plugin-host", laws: ["exclusive_selection_never_crosses_a_lifecycle_barrier", "fixed_owner_ring_hands_back_items_and_bytes_at_the_exact_boundary", "replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting"] },
    {"package":"semio-s-plugin-procedural","laws":["id_index_roundtrip", "id_serde_roundtrip", "new_full_has_all_patterns_and_correct_sums", "restrict_reduces_and_updates_caches", "sum_over_matches_manual", "assembly_cursor_compiler_matches_canonical_builder","assembly_cursor_compiler_matches_canonical_csr_order_and_multiplicity","checkpoint_resume_preserves_rng_trail_and_progress","checkpoint_restore_rejects_foreign_operation_and_topology","checkpoint_resume_preserves_preview_sequence","cancellation_interrupts_checkpoint_and_commit_materialization_without_progress","minimum_checkpoint_is_exactly_the_fixed_header_and_restores","checkpoint_restore_rejects_size_arithmetic_overflow"]},
    { package: "semio-framework-plugin", laws: ["full_operation_source_rejects_generic_reducers_and_old_monolithic_shells","spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it","local_interaction_live_pages_wait_exact_ack_and_all_three_roots","local_interaction_live_reopened_request_rejects_old_started_cancel","local_interaction_live_partial_admission_retains_successful_roots","local_interaction_live_partial_error_preserves_wrapper_emission_and_retirement_counts","sparse_live_instances_receive_successive_round_robin_turns","runtime_instance_registry_has_fixed_capacity_collision_and_reuse","cleanup_queue_saturation_preserves_detached_app_ownership","cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once","cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement","cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close","cold_pair_ingress_final_live_fence_rejects_post_await_revocation","cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close","cold_pair_ingress_is_an_exact_retained_native_close_participant","cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes","interactive_bridge_coalesces_preview_but_backpressures_lossless_items","interactive_bridge_diagnostic_ring_is_item_and_byte_bounded","spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs","a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service","infer_job_checkpoint_restore_matches_an_uninterrupted_run","artifact_inference_registry_is_order_independent_and_idempotent","artifact_inference_registry_rejects_any_conflicting_duplicate","append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes","append_chunk_over_cap_faults_instead_of_silently_truncating","append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op","routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service","checkpoint_binary_matches_schema_fixture_and_owned_oracle","checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift","world3d_scene_fields_bind_the_domain_while_the_sun_helper_leaves_it_unset","repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit","contended_live_cleanup_does_not_consume_structural_stall_credit","cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking"] },
    { package: "semio-s-plugin-puzzle", laws: ["spatial_capacity_plus_one_refusal_preserves_exact_old_state", "spatial_stale_owner_cannot_finish_partial_replacement", "spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress", "spatial_fixed_collections_use_the_credited_pages_and_return_identical_plus_one_owners", "overlap_checkpoint_resumes_exact_rng_and_sample_cursor", "overlap_is_deterministic_across_batch_sizes", "blocked_vortex_full_ids_and_enumeration_excludes_them", "weighted_sample_without_replacement_edge_cases"] },
    { package: "semio-s-plugin-draw", laws: ["retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback"] },
    { package: "semio-s-plugin-energy", laws: ["retained_roster_is_exact_and_exhaustive", "p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner", "p7c2_restore_stale_step_and_install_preserve_exact_replay_authority", "sequential_fills_first_unit", "uniform_splits_proportionally_to_capacity", "surface_incidence_matches_known_surface_normal"] },
    { package: "semio-s-plugin-raster", laws: ["raster_asset_capacity_matches_the_json_oracle"] },
    { package: "semio-s-plugin-architect", laws: ["architect_configuration_contract_vectors_match_the_json_oracle", "architect_presence_contract_vectors_match_the_json_oracle", "architect_semantic_panels_match_the_json_oracle", "sample_plugin_round_trips_json", "composed_register_rows_belong_to_each_exact_child"] },
    { package: "semio-s-plugin-gis", laws: ["language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload", "retained_command_factory_matches_the_language_neutral_maximum_oracle", "strict_snapshot_and_aggregate_json_vectors", "gis2d_config_operation_lines_round_trip", "gis3d_config_operation_lines_round_trip"] },
    { package: "semio-s-plugin-shooting", laws: ["shooting_configuration_contract_vectors_match_the_json_oracle", "shooting_presence_contract_vectors_match_the_json_oracle", "shooting_window_actions_match_the_json_oracle", "shooting_semantic_panels_match_the_json_oracle", "shooting_shot_field_values_match_the_json_oracle"] },
    { package: "semio-s-plugin-imperative", laws: ["imperative_configuration_contract_vectors_match_the_json_oracle", "imperative_semantic_panels_match_the_json_oracle", "working_content_is_owned_by_each_exact_child", "render_lists_one_row_per_top_level_step", "render_compiles_the_default_document_into_read_only_text", "imperative_viewer_never_mutates", "imperative_editor_and_viewer_share_dialect", "create_step_inverse_law", "delete_step_inverse_law", "delete_step_missing_target_is_error", "reorder_steps_inverse_law", "reorder_steps_missing_target_is_error", "edit_step_params_inverse_law", "edit_step_params_missing_target_is_error", "create_step_duplicate_id_fatal_never_applies", "create_step_diff_absorb_law", "document_text_round_trip_with_applied_operation"] },
    { package: "semio-s-plugin-animate", laws: ["presentation_configuration_contract_vectors_match_the_json_oracle", "presentation_presence_contract_vectors_match_the_json_oracle", "presentation_semantic_panels_match_the_json_oracle", "title_cards_match_the_neutral_xml_oracle", "from_dwg_builds_single_slide_deck_from_entity", "from_dwg_never_errors_on_empty_drawing"] },
    { package: "semio-s-plugin-reasoning-mindmap", laws: ["wires_configuration_contract_vectors_match_the_json_oracle", "wires_presence_contract_vectors_match_the_json_oracle", "wires_semantic_panels_match_the_json_oracle", "renders_canvas_scene_for_the_empty_document", "renders_canvas_scene_for_the_metabolism_example"] },
    { package: "semio-s-plugin-sequence", laws: ["sequence_semantic_panels_match_the_json_oracle", "sequence_retained_json_measure_matches_the_json_oracle", "sequence_carrier_contracts_match_the_json_oracle", "sequence_configuration_contract_vectors_match_the_json_oracle", "sequence_presence_contract_vectors_match_the_json_oracle", "render_produces_a_read_only_scene_for_the_default_document", "artifacts::sequence::standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law", "artifacts::sequence::standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law", "linear_chain_orders_by_dependency_and_depth_by_distance_from_root", "a_two_step_cycle_is_reported_as_not_cycle_free_but_stays_total", "a_dangling_edge_is_ignored", "diamond_depth_takes_the_longest_incoming_path"] },
    { package: "semio-s-plugin-note", laws: ["note_configuration_contract_vectors_match_the_json_oracle", "note_presence_contract_vectors_match_the_json_oracle", "note_pdf14_page_contract_matches_the_json_oracle", "note_semantic_panels_match_the_json_oracle", "note_ink_canvas_payload_matches_the_json_oracle", "note_document_round_trips_assets_and_grid_settings", "root_scalar_inverse_and_absorb_laws", "asset_inverse_law_create_replace_delete", "block_lifecycle_inverse_law_create_delete_duplicate", "block_reparent_and_drag_inverse_law", "block_field_inverse_laws", "table_row_column_inverse_laws", "create_block_duplicate_id_is_fatal", "delete_block_missing_target_is_error", "delete_blocks_missing_target_is_error", "rename_block_missing_target_is_error", "change_block_locked_missing_target_is_error", "move_block_missing_target_is_error", "move_block_non_finite_is_fatal", "resize_block_missing_target_is_error", "drag_blocks_missing_target_is_error", "duplicate_block_missing_source_is_error", "insert_table_row_missing_target_is_error", "remove_table_row_missing_target_is_error", "edit_block_text_missing_target_is_error", "replace_asset_payload_missing_target_is_error", "create_asset_duplicate_id_is_fatal", "delete_asset_missing_target_is_error"] },
    { package: "semio-s-plugin-layout", laws: ["layout_configuration_contract_vectors_match_the_json_oracle", "layout_presence_contract_vectors_match_the_json_oracle", "layout_pdf_page_collection_matches_the_json_oracle", "layout_inspection_summary_matches_the_json_oracle", "background_drawing_and_referenced_model_round_trip_through_text_and_binary", "absent_composition_slots_round_trip_as_none", "typed_document_json_matches_serde_and_every_write_is_credit_bounded", "create_page_obeys_the_inverse_and_absorb_laws", "move_frame_obeys_the_inverse_law", "rename_layout_obeys_the_inverse_law", "delete_page_obeys_the_inverse_law", "reorder_pages_obeys_the_inverse_law", "update_page_margins_obeys_the_inverse_law", "change_frame_fill_obeys_the_inverse_law", "edit_story_and_create_link_obey_the_inverse_law", "create_frame_missing_target_is_error", "delete_frame_missing_target_is_error", "move_frame_missing_target_is_error", "reorder_pages_missing_target_is_error", "rename_page_missing_target_is_error", "change_page_height_missing_target_is_error", "edit_story_missing_target_is_error", "create_page_duplicate_id_is_fatal"] },
    { package: "semio-s-plugin-fem", laws: ["vector_layer_vectors_match_the_json_oracle", "process_owner_inventory_admits_exact_maximum_and_returns_exact_credit"] },
    { package: "semio-s-plugin-playbook-procedural", laws: ["procedural_payload_vectors_match_the_json_oracle", "procedural_parameter_controls_match_the_json_oracle", "procedural_actor_descriptor_matches_the_json_oracle", "module_app_declares_window_kinds", "module_manifest_contributes_building_component"] },
    { package: "semio-s-plugin-writer", laws: ["writer_configuration_contract_vectors_match_the_json_oracle", "writer_presence_contract_vectors_match_the_json_oracle", "pdf_page_text_vectors_match_the_json_oracle", "writer_into_pdf_preserves_text_and_page_size"] },
    { package: "semio-framework-os-kernel", laws: ["document_codec_of_round_trips_dsl_and_pack_and_edit_text", "register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first", "dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop", "space_history_verbs_match_the_language_neutral_contract", "str_eq_matches_std_partial_eq", "retained_group_history_switches_every_direct_reader_at_one_decision", "fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba"] },
    { package: "semio-framework-actor", laws: ["pack_round_trip_turn_result", "mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner", "mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification", "mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged", "mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay", "job_progress_fixed_capacity_and_aba_admission_fail_closed", "job_progress_commit_validates_live_authority_and_rejected_close_is_incremental"] },
    { package: "semio-s-plugin-mathematical", laws: ["language_neutral_mutations_match_json_oracle_and_restore_base"] },
    { package: "semio-s-plugin-trinity", laws: ["editor::jack::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle", "editor::rewriting::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle", "editor::jack::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle", "editor::rewriting::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle"] },
    { package: "semio-framework-os-infinite", laws: ["board_fixture_json_vectors_match_the_json_oracle", "icon_codec_resolves_metabolism_shortcode_to_themed_svg", "wheel_plan_matches_direct_and_rejects_stale_interaction", "typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing", "retained_draw_rebuild_keeps_url_backed_asset_authority", "retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle", "retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically"] },
    { package: "semio-s-plugin-playbook", laws: ["configuration_and_presence_contract_vectors_match_the_json_oracle", "render_builder_emits_playbook_list_component_scene"] },
    { package: "semio-s-plugin-forms", laws: ["forms_configuration_contract_vectors_match_the_json_oracle", "vector_replacement_boundaries_match_the_json_oracle", "semantic_question_controls_match_the_language_neutral_vectors", "renders_blueprint_builder_cards", "large_unrelated_config_and_existing_vector_stay_under_one_bounded_slice", "vector_growth_writes_at_most_sixty_four_components_per_slice", "missing_non_array_and_malformed_targets_keep_best_effort_semantics", "scalar_option_and_object_shapes_stay_intact", "bounded_chunk_values_match_the_json_oracle"] },
    { package: "semio-framework-surface", laws: ["graph_host_sync_from_scene_pack_decodes_pack_shell", "map_render_mode_parse","map_vector_style_parse","sync_map_json_keeps_position_labels","sync_map_json_parses_rich_position_metadata","owned_protobuf_decodes_layer_properties_and_point_geometry","fixture_linestrings_split_at_moveto","map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside","map_polyline_intersects_polygon_detects_crossing_edge","pointer_up_emits_camera_after_middle_button_pan","interaction_plan_matches_direct_wheel_and_pan_semantics","build_vector_scene_respects_render_mode","build_vector_scene_respects_vector_style"] },
    { package: "semio-framework-editor", laws: ["char_boundary_helpers_handle_multibyte","offset_line_col_roundtrip","sync_from_scene_json_sets_and_clears_hover_range","sync_from_scene_json_applies_all_optional_fields"] },
    { package: "semio-s-plugin-imperative-effect", laws: ["bundle_contributes_core_module_for_imperative_play"] },
  ].filter(group => !name || name.split(",").includes(group.package));
  if (!groups.length) throw new Error("No matching native verification group");
  let failures = 0;
  for (const group of groups) {
    let artifactDir = "";
    try {
      const receipts = await runExactCargoLaws({ cwd: process.cwd(), groups: [{ ...group, target: "target" in group ? group.target : { kind: "lib" } }], artifactDir: join(generated, "native-laws"), cargoArgs: group.package === "semio-framework-os" ? ["-j2", "--features", "semio-framework-os/os-host-full"] : ["-j2"], env: { ...process.env, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", CARGO_TARGET_DIR: join(generated, "target"), TMPDIR: generated }, buildBudgetMs: 0, progress(event) { artifactDir = event.artifactDir; console.log(`[DEBUG] ${event.package} ${event.stage} ${event.law ?? ""}`); } });
      appendFileSync(report, `\n### Native Verification: ${group.package}\n\n${receipts.map(receipt => `${receipt.assertions} exact native assertions passed; executable SHA-256 ${receipt.sha256}.\n\n${receipt.laws.map(law => "- " + law).join("\n")}`).join("\n")}\n`);
    } catch (error) {
      failures += 1;
      appendFileSync(report, `\n### Native Verification Failure: ${group.package}\n\n${String(error)}\n`);
      console.error(String(error));
    } finally {
      const build = join(artifactDir, "build.stdout");
      if (existsSync(build)) {
        const messages = readFileSync(build, "utf8").split("\n").flatMap(line => { try { const row = JSON.parse(line); return row.reason === "compiler-message" ? [row] : []; } catch { return []; } });
        const errors = messages.filter(row => row.message.level === "error"), warnings = messages.filter(row => row.message.level === "warning");
        appendFileSync(report, `\nCompiler diagnostics: ${errors.length} errors, ${warnings.length} warnings.\n\n${errors.map(row => row.message.rendered).join("\n")}\n`);
      }
    }
  }
  process.exit(failures ? 1 : 0);
}
if (command === "scope-packages") {
  const { rustWarningTargetScope } = await import(resolve(process.cwd(), "📜️script.ts"));
  console.log(JSON.stringify(rustWarningTargetScope(process.cwd(), "wasm32-wasip2").packages));
  process.exit(0);
}
if (command === "scope-test") {
  const { rustWarningTargetScope } = await import(resolve(process.cwd(), "📜️script.ts"));
  const fixture = await Bun.file("🧪️tests/🦀️rust-warnings/🔣️.json").json();
  for (const row of fixture.cases) {
    const scope = rustWarningTargetScope(process.cwd(), row.target);
    for (const pkg of row.requiredPackages) if (!scope.packages.includes(pkg)) throw new Error(row.target + " misses " + pkg);
  }
  console.log("Warning scope fixtures passed");
  process.exit(0);
}

if (["paths", "apply-paths"].includes(command)) {
  const files = Bun.spawnSync(["rg", "--files", "✏️s/🔌️plugins", "-g", "*.rs"]).stdout.toString().trim().split("\n").filter(file => file.endsWith("📦️packages/🦀️rust/🦀️.rs"));
  const changed: string[] = [];
  for (const file of files) {
    const source = readFileSync(file, "utf8");
    let next = source;
    for (const match of source.matchAll(/#\[path = "([^"]+)"\]/g)) {
      const oldPath = match[1];
      if (!oldPath.includes("🧪️tests/") || existsSync(resolve(dirname(file), oldPath))) continue;
      const missing = resolve(dirname(file), oldPath);
      const testDirectory = dirname(missing);
      const parent = dirname(testDirectory);
      if (!existsSync(parent)) { console.log(`UNRESOLVED ${missing}`); continue; }
      const stem = basename(testDirectory);
      const candidates = readdirSync(parent).filter(candidate => {
        const target = join(parent, candidate, basename(missing));
        return existsSync(target) && readFileSync(target, "utf8").slice(0, 1_000).includes(`\`${stem}\``);
      });
      if (candidates.length !== 1) { console.log(`UNRESOLVED ${missing} candidates=${candidates.join(",")}`); continue; }
      const target = join(parent, candidates[0], basename(missing));
      const newPath = relative(dirname(file), target);
      console.log(`${file}: ${oldPath} → ${newPath}`);
      next = next.replaceAll(`#[path = "${oldPath}"]`, `#[path = "${newPath}"]`);
    }
    if (command === "apply-paths" && next !== source) { writeFileSync(file, next); changed.push(file); }
  }
  if (changed.length) appendFileSync(join(import.meta.dir, "📓️2026-09-07-verification.md"), `\n### Test Module Paths Repaired\n\n${changed.map(file => `- \`${file}\``).join("\n")}\n`);
  process.exit(0);
}
if (!["diagnostics", "unique", "suggestions", "apply"].includes(command) || !name || name.includes("/") || name.includes("\\")) throw new Error("Expected diagnostics|unique|suggestions|apply <log-name> [filter]");
const messages = readFileSync(join(import.meta.dir, "🗑️generated", `${name}.jsonl`), "utf8").split("\n").flatMap(line => {
  try {
    const row = JSON.parse(line);
    return row.reason === "compiler-message" && ["warning", "error"].includes(row.message.level) ? [row] : [];
  } catch { return []; }
});
if (["suggestions", "apply"].includes(command)) {
  const byFile = new Map<string, Map<string, { start: number; end: number; replacement: string; original: string }>>();
  const allowed = new Set(["unused_qualifications", "non_shorthand_field_patterns", "deprecated", "unused_imports", "unused_braces", "unused_mut"]);
  for (const row of messages) {
    if (!allowed.has(row.message.code?.code) || (filter && !JSON.stringify(row).includes(filter))) continue;
    for (const child of row.message.children) {
      if (!child.spans.length || child.spans.some(span => span.suggestion_applicability !== "MachineApplicable" || span.suggested_replacement === null)) continue;
      for (const span of child.spans) {
        if (row.message.code.code === "deprecated" && span.suggested_replacement !== "try_update") continue;
        const file = resolve(span.file_name);
        if (!file.startsWith(process.cwd() + "/") || !file.endsWith(".rs") || !existsSync(file)) continue;
        const content = readFileSync(file);
        if ((file.includes("generated") || /@generated|GENERATED-BY/.test(content.toString().slice(0, 1_000))) && !content.toString().slice(0, 100).includes("Hand-written")) continue;
        const lines = content.toString().split("\n");
        if (span.text.some((line, index) => lines[span.line_start - 1 + index] !== line.text)) continue;
        if (row.message.code.code === "unused_imports") {
          if (span.text.some(line => line.text.includes("pub use"))) continue;
          if (/#\[cfg\(test\)\]/.test(content.toString())) continue;
          const names = [...row.message.message.matchAll(/`([^`]+)`/g)].map(match => match[1].split("::").at(-1));
          const rest = lines.filter((_, index) => index < span.line_start - 1 || index >= span.line_end).join("\n");
          const unconditionalPluginLeaf = file.includes("✏️s/🔌️plugins/") && !/#\!?\[cfg|\bmod\s+[a-zA-Z_]\w*\s*[;{]/.test(content.toString());
          if (!unconditionalPluginLeaf && (!names.length || names.some(name => !/^[A-Za-z_][A-Za-z_0-9]*$/.test(name) || new RegExp(`\\b${name}\\b`).test(rest)))) continue;
        }
        const original = span.text.map(line => Array.from(line.text).slice(line.highlight_start - 1, line.highlight_end - 1).join("")).join("\n");
        if (content.subarray(span.byte_start, span.byte_end).toString() !== original) continue;
        const changes = byFile.get(file) ?? new Map();
        changes.set(`${span.byte_start}:${span.byte_end}`, { start: span.byte_start, end: span.byte_end, replacement: span.suggested_replacement, original });
        byFile.set(file, changes);
      }
    }
  }
  const changed: string[] = [];
  for (const [file, changes] of byFile) {
    let content = readFileSync(file);
    let boundary = content.length;
    for (const edit of [...changes.values()].sort((a, b) => b.start - a.start)) {
      if (edit.end > boundary) continue;
      console.log(`${relative(process.cwd(), file)}: ${edit.original} → ${edit.replacement}`);
      content = Buffer.concat([content.subarray(0, edit.start), Buffer.from(edit.replacement), content.subarray(edit.end)]);
      boundary = edit.start;
    }
    if (command === "apply") { writeFileSync(file, content); changed.push(relative(process.cwd(), file)); }
  }
  if (changed.length) appendFileSync(join(import.meta.dir, "📓️2026-09-07-verification.md"), `\n### Compiler Suggestions Applied from ${name}\n\n${changed.map(file => `- \`${file}\``).join("\n")}\n`);
  process.exit(0);
}
if (command === "unique") {
  const seen = new Set<string>();
  for (const row of messages) {
    if (filter && !JSON.stringify(row).includes(filter)) continue;
    const span = row.message.spans.find(span => span.is_primary);
    const key = `${row.message.code?.code}: ${row.message.message} ${span?.file_name}:${span?.line_start}`;
    if (!seen.has(key)) console.log(key);
    seen.add(key);
  }
  process.exit(0);
}
const counts = new Map<string, number>();
for (const row of messages) {
  const key = `${row.message.level} ${row.package_id.split("#").at(-1)} ${row.message.code?.code ?? ""}`;
  counts.set(key, (counts.get(key) ?? 0) + 1);
}
console.log([...counts].sort((a, b) => b[1] - a[1]).map(([key, count]) => `${count} ${key}`).join("\n"));
if (filter) for (const row of messages.filter(row => JSON.stringify(row).includes(filter))) console.log(row.message.rendered);
console.log(`[DEBUG] ${messages.filter(row => row.message.level === "warning").length} warnings; ${messages.filter(row => row.message.level === "error").length} errors`);
