import { readFileSync, writeFileSync, appendFileSync, existsSync, readdirSync } from "node:fs";
import { join, resolve, relative, dirname, basename } from "node:path";

const [command, name, filter] = process.argv.slice(2);
if (["native-start", "native-worker", "native-cancel", "native-protect", "native-lease"].includes(command)) {
  if (!/^native\d+$/.test(name ?? "")) throw new Error("Expected a unique native build name");
  const cargoCommand = filter ?? "check";
  if (!["check", "clippy"].includes(cargoCommand)) throw new Error("Expected check or clippy");
  const { spawn } = await import("node:child_process");
  const { mkdirSync, openSync, closeSync, mkdtempSync, rmSync } = await import("node:fs");
  const generated = resolve(import.meta.dir, "🗑️generated");
  mkdirSync(generated, { recursive: true });
  const prefix = join(generated, name);
  const receiptPath = prefix + "-receipt.json";
  const cancelPath = prefix + "-cancel.json";
  const reportPath = join(import.meta.dir, "📓️2026-09-08-checks.md");
  if (command === "native-cancel") {
    writeFileSync(cancelPath, JSON.stringify({ requestedAt: new Date().toISOString() }));
    console.log("[DEBUG] Cancellation requested for " + name);
    process.exit(0);
  }
  if (command === "native-start" || command === "native-protect") {
    const protection = command === "native-protect";
    if (protection) {
      if (existsSync(receiptPath)) process.exit(0);
      process.kill(JSON.parse(readFileSync(prefix + "-process.json", "utf8")).pid, 0);
    } else if (existsSync(prefix + "-process.json") || existsSync(receiptPath)) throw new Error("Build name already exists: " + name);
    const suffix = protection ? "lease" : "worker";
    const stdout = openSync(prefix + "-" + suffix + ".log", "a");
    const stderr = openSync(prefix + "-" + suffix + ".stderr", "a");
    const worker = spawn(process.execPath, [import.meta.path, protection ? "native-lease" : "native-worker", name, cargoCommand], { cwd: process.cwd(), env: process.env, detached: true, stdio: ["ignore", stdout, stderr] });
    closeSync(stdout);
    closeSync(stderr);
    if (!worker.pid) throw new Error("Native worker did not start");
    writeFileSync(prefix + (protection ? "-lease-process.json" : "-process.json"), JSON.stringify({ pid: worker.pid, script: import.meta.path, cwd: process.cwd(), startedAt: new Date().toISOString() }, null, 2));
    worker.unref();
    console.log("[DEBUG] " + name + " " + suffix + " started: " + worker.pid);
    process.exit(0);
  }
  const { EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX, EXACT_CARGO_ACTIVE_LEASE_MANIFEST, exactCargoGeneratedOutputHasLiveLease } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts"));
  const ownerPid = command === "native-lease" ? JSON.parse(readFileSync(prefix + "-process.json", "utf8")).pid : process.pid;
  if (existsSync(receiptPath)) process.exit(0);
  process.kill(ownerPid, 0);
  const leaseRoot = mkdtempSync(join(generated, EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX));
  const leaseManifest = join(leaseRoot, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
  writeFileSync(leaseManifest, JSON.stringify({ version: 1, pid: process.pid }), { mode: 0o600 });
  const leaseTimer = setInterval(() => {
    try {
      process.kill(ownerPid, 0);
      if (existsSync(receiptPath)) { endLease(); return; }
      writeFileSync(leaseManifest, JSON.stringify({ version: 1, pid: process.pid }), { mode: 0o600 });
    } catch { endLease(); }
  }, 10_000);
  const endLease = () => { clearInterval(leaseTimer); rmSync(leaseRoot, { recursive: true, force: true }); };
  process.once("exit", endLease);
  console.log("[DEBUG] " + name + " cleanup lease recognized: " + exactCargoGeneratedOutputHasLiveLease(generated));
  if (command === "native-lease") await new Promise<void>(resolve => {
    const terminal = setInterval(() => {
      if (existsSync(leaseManifest)) return;
      clearInterval(terminal);
      resolve();
    }, 1_000);
  });
  if (command === "native-lease") process.exit(0);
  leaseTimer.unref();
  const startedAt = new Date().toISOString();
  const env = { ...process.env, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", CARGO_TARGET_DIR: join(generated, "derive-target"), TMPDIR: generated };
  const run = async (args: string[], stdout: string, stderr: string) => {
    if (existsSync(cancelPath)) throw new Error("Native build cancelled");
    const child = Bun.spawn(["cargo", ...args], { env, stdout: Bun.file(stdout), stderr: Bun.file(stderr) });
    const cancellation = setInterval(() => { if (existsSync(cancelPath)) child.kill("SIGTERM"); }, 1_000);
    try { return await child.exited; } finally { clearInterval(cancellation); }
  };
  writeFileSync(prefix + "-started.json", JSON.stringify({ pid: process.pid, startedAt }, null, 2));
  try {
    const metadataPath = prefix + "-metadata.json";
    const metadataExit = await run(["metadata", "--no-deps", "--format-version=1"], metadataPath, prefix + "-metadata.log");
    if (metadataExit !== 0) throw new Error("Cargo metadata failed: " + metadataExit);
    const metadata = JSON.parse(readFileSync(metadataPath, "utf8"));
    const base = ["semio-framework-actor", "semio-framework", "semio-framework-os-kernel", "semio-framework-os-renderer-wgpu"];
    const packages: string[] = metadata.packages.filter(p => base.includes(p.name) || p.name.startsWith("semio-s-plugin-") || p.name.startsWith("semio-s-artifact-") || p.manifest_path.includes("/🗿️artifacts/") || p.manifest_path.includes("\\🗿️artifacts\\")).map(p => p.name).sort();
    const args = [cargoCommand, "--all-targets", "--features", "semio-framework-os-renderer-wgpu/native-bin", "--message-format=json", "--keep-going", "-j2", ...packages.flatMap(pkg => ["-p", pkg]), ...(cargoCommand === "clippy" ? ["--", "-D", "warnings"] : [])];
    writeFileSync(prefix + "-scope.json", JSON.stringify({ cargoCommand, packages, args }, null, 2));
    appendFileSync(reportPath, "\n\n## " + name + " — Retained Native Fleet Check\n\nStarted a detached, cancellable worker under Nx for " + packages.length + " current packages, including plugin and artifact test targets and the native renderer entry point. The worker records its PID, scope, output and final receipt inside this ticket so a task continuation does not terminate the compiler run. Only one compiler validation for this ticket was active at dispatch.\n");
    console.log("[DEBUG] " + name + " " + cargoCommand + " checking " + packages.length + " packages");
    const exitCode = await run(args, prefix + ".jsonl", prefix + ".log");
    const diagnostics: unknown[] = [];
    const counts: Record<string, number> = {};
    for (const line of readFileSync(prefix + ".jsonl", "utf8").split("\n")) {
      try {
        const row = JSON.parse(line);
        if (row.reason === "compiler-message") {
          diagnostics.push(row);
          const code = row.message.code?.code ?? row.message.level;
          counts[code] = (counts[code] ?? 0) + 1;
        }
      } catch {}
    }
    const cargoWarnings = [...new Set([prefix + "-metadata.log", prefix + ".log"].flatMap(file => readFileSync(file, "utf8").split("\n").filter(line => /^warning:/.test(line))))];
    const receipt = { status: existsSync(cancelPath) ? "cancelled" : "complete", exitCode, startedAt, finishedAt: new Date().toISOString(), packages, args, counts, cargoWarnings };
    writeFileSync(prefix + "-diagnostics.json", JSON.stringify(diagnostics));
    writeFileSync(receiptPath, JSON.stringify(receipt, null, 2));
    appendFileSync(reportPath, "\n" + name + " ended with exit code " + exitCode + " and diagnostic counts " + JSON.stringify(counts) + ", plus " + cargoWarnings.length + " distinct Cargo warning lines. A check result does not prove actual links or runtime behavior.\n");
    console.log("[DEBUG] " + name + " " + JSON.stringify({ exitCode, counts, cargoWarnings }));
    process.exit(exitCode);
  } catch (error) {
    const receipt = { status: existsSync(cancelPath) ? "cancelled" : "failed", startedAt, finishedAt: new Date().toISOString(), error: String(error) };
    writeFileSync(receiptPath, JSON.stringify(receipt, null, 2));
    appendFileSync(reportPath, "\n" + name + " failed before completing compiler validation: " + String(error) + ".\n");
    console.error("[DEBUG] " + name + " " + String(error));
    process.exit(1);
  }
}

if (["laws", "laws-fleet"].includes(command)) {
  const { runExactCargoLaws } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts"));
  const generated = resolve(import.meta.dir, "🗑️generated");
  const report = join(import.meta.dir, "📓️2026-09-07-verification.md");
  const groups = [
    {"package":"semio-s-artifact-fem-3d","laws":["retained_command_fixture_matches_exact_routes_and_value_codec_boundaries"],"cargoArgs":["--features","semio-s-artifact-fem-3d/component-app-assembly"]},
    {"package":"semio-s-plugin-writer","laws":["standard_mounts_exactly_one_subset","subset_dialect_is_the_canonical_writer_dialect","subset_declares_ten_io_entries","writer_viewer_never_mutates","writer_editor_and_viewer_share_dialect"]},
    {"package":"semio-s-artifact-block-3d","laws":["command_ids_are_unique_and_cover_every_row","every_command_round_trips_text_and_binary","leave_surface_text_and_binary_match_the_command_oracle","retained_route_dispositions_are_exact_and_exhaustive","block3d_world_preview_codecs_and_inverse_match_neutral_vectors","preview_partition_matches_language_neutral_json_oracle"],"cargoArgs":["--features","semio-s-artifact-block-3d/component-app-assembly"]},
    {"package":"semio-s-artifact-procedural-generation3d","laws":["diff_absorb_prefers_incoming_fixture_and_preserves_generation","generation_preview_is_one_app_transient_shared_by_two_generation_windows","preview_lifecycle_matches_language_neutral_third_party_oracle"],"cargoArgs":["--features","semio-s-artifact-procedural-generation3d/component-app-assembly"]},
    {"package":"semio-s-artifact-procedural-generation2d","laws":["diff_absorb_prefers_incoming_fixture_and_preserves_generation","generation_preview_is_one_app_transient_shared_by_two_generation_windows","preview_state_matches_language_neutral_json_oracle"],"cargoArgs":["--features","semio-s-artifact-procedural-generation2d/component-app-assembly"]},
    {"package":"semio-framework","target":{"kind":"lib" as const},"laws":["io_compose_via_chains_two_registered_hops","io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry","retained_wire_pages_are_admitted_sealed_transferred_and_closed_by_logical_bytes","production_typed_payload_and_retained_pages_enter_the_same_registered_factory_job","retained_wire_admission_rejects_plus_one_and_returns_the_page_owner_on_saturation","maximum_extent_owner_exists_before_incremental_encoding_and_seals_to_its_exact_prefix","return_content_message_all_endpoints_match_independent_bytes_without_payload_parsing","return_content_message_large_payload_and_cancel_keep_original_source_allocation","ui_turn_patch_owner_typed_descendants_preserve_exact_one_byte_grants","resolve_load_order_toposorts_a_diamond","resolve_load_order_is_deterministic_regardless_of_input_order","resolve_load_order_reports_missing_dependency","resolve_load_order_reports_version_mismatch","resolve_load_order_names_every_member_of_a_cycle","resolve_load_order_accepts_a_self_satisfying_empty_graph","dependents_returns_direct_dependents_sorted"]},
    {"package":"semio-s-artifact-stdio-contract","laws":["standard_base64_matches_the_reference_implementation","artifact_assembly_layout_and_identity_match_neutral_budget"]},
    {"package":"semio-s-plugin-stdio","laws":["selected_contribution_identities_are_unique_and_schema_owned","full_catalog_preserves_definition_codec_and_ledger_counts"],"cargoArgs":["--features","semio-s-plugin-stdio/component-app-assembly"]},
    {"package":"semio-s-artifact-stdio-binary","laws":["component::io_registry::tests::register_then_resolve_through_the_typed_registry_finds_this_composer","create_binary_editor_builds_a_definition_for_the_editor_role","create_binary_viewer_builds_a_definition_for_the_viewer_role","parse_hex_dump_round_trips_a_rendered_snapshot","parse_hex_dump_rejects_odd_length_hex"],"cargoArgs":["--features","semio-s-artifact-stdio-binary/component-app-assembly"]},
    {"package":"semio-s-artifact-stdio-gif","laws":["registered_migration_runs_end_to_end_through_the_store_registry"],"cargoArgs":["--features","semio-s-artifact-stdio-gif/component-app-assembly"]},
    {"package":"semio-s-artifact-stdio-gltf","laws":["mutation_rejection_messages_match_the_language_neutral_json_oracle","mutation_restore_preserves_the_language_neutral_wire_and_inverse","standards::v2_0::subsets::any::schema::mutations::bind_node_child::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::unbind_node_child::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::bind_scene_root_node::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::unbind_scene_root_node::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::change_node_name::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::change_node_extra_data::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::change_material_alpha_mode::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::change_material_double_sided::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::create_scene::contract::canonical_vectors_execute_direct_mutation_and_codec_laws","standards::v2_0::subsets::any::schema::mutations::delete_scene::contract::canonical_vectors_execute_direct_mutation_and_codec_laws"],"cargoArgs":["--features","semio-s-artifact-stdio-gltf/component-app-assembly"]},
    {"package":"semio-s-plugin-gis","target":{"kind":"test","name":"native_codecs"},"laws":["gis_native_receipts_bind_literal_two_codec_closure_without_identity_or_factory_substitution","gis_native_controlled_inference_executes_literal_progress_cancel_and_deadline_trace"]},
    {"package":"semio-s-plugin-vcs","target":{"kind":"test","name":"native_codecs"},"laws":["vcs_native_receipts_bind_literal_one_codec_closure_without_identity_or_factory_substitution","vcs_native_receipt_closure_denies_every_hostile_row_including_the_retired_document_kind"]},
    {"package":"semio-s-artifact-space-space","laws":["folds_visibility_and_members_for_this_space_into_config","open_artifact_relays_with_document_and_space_ids","open_artifact_with_relays_the_explicit_choice"],"cargoArgs":["--features","semio-s-artifact-space-space/component-app-assembly"]},
    {"package":"semio-s-artifact-norm-en1990","laws":["qk_working_table_is_owned_by_the_exact_child"]},
    {"package":"semio-s-artifact-norm-din18599","laws":["climate_working_data_is_owned_by_the_exact_child"]},
    {"package":"semio-s-artifact-norm-din4108","laws":["set_snapshot_dispatches_through_the_tool_job_path_and_publishes_the_payload_document","selected_check_index_is_a_config_only_edit","the_proof_catalog_covers_exactly_the_shared_retained_tool_ids"]},
    {"package":"semio-s-artifact-norm-contract","laws":["render_report_falls_back_to_a_placeholder_when_nothing_was_computed","render_inspection_falls_back_to_the_first_check_for_an_out_of_range_index"]},
    {"package":"semio-framework-ui-runtime","laws":["mounted_producer_advances_one_opportunity_and_publishes_only_complete_candidate","deep_tree_maximum_and_plus_one_preserve_exact_fault_owner_for_incremental_close","resumable_cursor_matches_the_existing_keyed_diff_and_revision_semantics","persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement","round_trip_property_every_emitted_patch_applies_cleanly_and_reproduces_the_snapshot"]},
    {"package":"semio-s-artifact-process-process3d","laws":["host_contributions_resolve_to_the_event_sourced_config_lane","process_machine_contributions_are_configuration_owned","registry_enforced_app_accepts_a_declared_operation_action"]},
    {"package":"semio-framework-job","laws":["payload_ledger_identity_must_match_the_exact_step_context","retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned","retained_state_and_output_have_separate_credits_and_close_one_page_per_grant","retained_writer_and_reader_advance_exactly_one_page_per_opportunity","worker_session_contention_rejection_take_resume_terminal_drop_and_close_are_exact","worker_pool_rejection_returns_exact_job_before_resume","worker_session_slots_max_plus_one_exact_rejection_and_drop_pump_are_owned"]},
    {"package":"semio-framework-pack","laws":["canonical_bytes_match_serde_json_for_typical_documents","language_neutral_retained_law_ledger_is_complete","retained_anchors_segments_catalog_and_deflate_are_wire_identical_and_resumable","retained_anchor_rejects_hostile_crc_and_requires_explicit_close","retained_identity_chunk_fragment_parity_exact_boundary_and_interrupted_finish","identity_chunk_cursor_retains_fragment_progress_and_terminal_verification"]},
    {"package":"semio-framework-ui-contract","laws":["retained_document_assembly_places_exact_pages_and_preserves_wire_and_payload_pointer","retained_document_assembly_rejects_duplicate_without_consuming_input_and_cancels_exact_backing","retained_document_assembly_and_read_alias_do_not_wait_on_contended_arena","retained_document_assembly_reports_metadata_initialization_separately_from_empty_payload_capacity","retained_document_root_permit_nine_surfaces_share_one_aggregate","retained_document_root_permit_last_reader_keeps_credit_and_typed_payload","retained_document_root_permit_cancel_and_contended_final_return_keep_exact_owner","retained_document_root_permit_reader_pressure_refuses_then_retries_exact_slot","retained_document_root_permit_seal_transfers_output_without_detaching_root_credit","retained_fixed_list_pages_counter_refuses_unaddressable_ownership_before_allocation","retained_fixed_list_pages_counter_keeps_actual_failed_allocation_until_release"]},
    {"package":"semio-framework-ui","laws":["scene_paint_cursor_rejects_stale_node_without_consuming_owner","scene_paint_cursor_advances_one_scalar_and_closes_one_bound_owner","mounted_layout_multi_page_unicode_uses_one_glyph_or_atlas_boundary_per_turn","mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained","mounted_layout_deadline_and_partial_close_each_advance_at_most_one_owner","mounted_layout_publication_rechecks_full_identity_and_repeat_ready_swaps_once"],"cargoArgs":["--features","semio-framework-ui/wgpu-engine"]},
    {"package":"semio-framework-replication","laws":["artifact_bootstrap_hashes_match_neutral_fixture","artifact_bootstrap_frames_match_neutral_vectors","artifact_bootstrap_rejects_malformed_transfers_atomically","artifact_bootstrap_cancellation_is_atomic_and_restartable","server_frame_welcome_round_trips_for_every_bootstrap_variant","fixed_causal_authority_rejects_capacity_plus_one_with_exact_identity_and_closes_one_owner_at_a_time","causal_insert_rejects_oversized_identity_without_losing_the_envelope_owner","insert_already_applied_operation_returns_already_applied_without_erroring","drains_applied_envelopes_in_causal_order","duplicate_seed_returns_the_exact_unadopted_identity_owner","fault_wire_projection_matches_language_neutral_serde_oracle","fault_inline_layout_stays_within_the_language_neutral_budget","shared_value_clone_matches_neutral_vectors_and_serde_json","shared_value_clone_grants_bound_utf8_progress_and_cancellation_returns_the_exact_source","shared_value_clone_drop_rejects_live_recursive_ownership","shared_value_clone_rejects_capacity_and_depth_without_losing_the_source"]},
    {"package":"semio-framework-pixels","laws":["gradient_checkerboard_round_trip","scanline_decoder_matches_batch_decode","oracle_decodes_our_encode","our_decode_reads_oracle_encode","our_decode_reads_oracle_palette_encode","zlib_compress_decompress_round_trip"]},
    {"package":"semio-framework-deflate","laws":["reads_miniz_oxide_dynamic_huffman_blocks","ours_inflates_miniz_oxide_output_and_vice_versa","stream_produces_the_same_bytes_as_one_shot_inflate"]},
    {"package":"semio-framework-hash","laws":["sha256_matches_nist_vectors_and_segmented_input","hash_bytes_agrees_with_the_blake3_oracle_across_lengths","hasher_agrees_with_the_blake3_oracle_for_segmented_updates"]},
    {"package":"semio-framework-trace","laws":["clock_is_monotonically_non_decreasing"]},
    {"package":"semio-framework-raster","laws":["align_bytes_per_row_pads_to_wgpu_alignment","scene_rasterizer_renders_expected_pixel_count"]},
    {"package":"semio-framework-os","laws":["codec_abi::tests::schema_and_language_neutral_fixture_cover_every_operation","codec_abi::tests::valid_pack_and_dsl_are_equivalent_deterministic_paged_replies","codec_abi::tests::workflow_pack_and_dsl_accept_every_byte_and_field_split","codec_abi::tests::deadline_interruption_and_zero_credit_do_not_advance_any_structural_cursor","demote_asset_moves_bytes_back_and_reregisters_draft_bookkeeping","creates_and_lists_space_catalog_entries"]},
    {"package":"semio-framework-os-shell","laws":["value_round_trip_matches_serde_shape","constructed_cases_match_committed_fixtures","fixtures_produce_expected_output"]},
    {"package":"semio-framework-plugin-host","laws":["exclusive_selection_never_crosses_a_lifecycle_barrier","fixed_owner_ring_hands_back_items_and_bytes_at_the_exact_boundary","replay_owners_drop_safely_from_every_owned_frontier_and_balance_accounting","host_error_layout_matches_language_neutral_budget","io_router_register_plugin_rejects_conflicting_io_entry_ownership","neutral_relay_lifecycle_traces_drive_production_machines","mounted_relay_stack_authority_matches_the_neutral_fixture","retained_pool_future_retries_saturation_once_and_terminalizes_shutdown","dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll","pending_guest_releases_the_only_worker_and_admits_no_duplicate_step","cancellation_race_admits_one_guest_cancel_and_one_terminal_outcome","mounted_start_panic_restores_the_instance_and_the_next_route_progresses","mounted_step_panic_restores_the_instance_and_terminalizes_once","cancel_panic_quarantines_instance_releases_permit_and_faults_once_on_one_worker","context_cancellation_failure_faults_once_quarantines_and_releases_one_worker","revoked_capability_cancels_only_its_own_operations_and_actor_survives","stale_generation_completion_is_dropped_current_generation_is_delivered","park_buffers_completions_and_resume_delivers_them_in_order","completion_burst_while_parked_is_bounded_not_unbounded","io_router_route_is_deterministic_across_load_order","io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops","io_router_route_respects_max_hops"]},
    {"package":"semio-s-artifact-procedural-assembly","laws":["id_index_roundtrip","id_serde_roundtrip","new_full_has_all_patterns_and_correct_sums","restrict_reduces_and_updates_caches","sum_over_matches_manual","assembly_cursor_compiler_matches_canonical_builder","assembly_cursor_compiler_matches_canonical_csr_order_and_multiplicity","checkpoint_resume_preserves_rng_trail_and_progress","checkpoint_restore_rejects_foreign_operation_and_topology","checkpoint_resume_preserves_preview_sequence","cancellation_interrupts_checkpoint_and_commit_materialization_without_progress","minimum_checkpoint_is_exactly_the_fixed_header_and_restores","checkpoint_restore_rejects_size_arithmetic_overflow","wfc_engine::grid2d::tests::node_at_and_coords_roundtrip","wfc_engine::grid3d::tests::node_at_and_coords_roundtrip","custom_stencil_validation_matches_neutral_vectors","from_coords_dedups_and_assigns_stable_first_seen_ids","custom_half_turn_groups_match_neutral_offset_vectors","cube_rotation_group_has_exactly_24_elements","cube_full_symmetry_group_has_exactly_48_elements","budget_exceeded_reports_partial_state","cancellation_stops_search_and_reports_partial","restart_only_never_proves_unsat_on_unsatisfiable_instance","best_of_n_keeps_the_highest_scoring_attempt","best_of_n_keeps_the_lowest_scoring_attempt","weight_field_identity_is_all_ones","wfc_engine::solver_grid2d::tests::grid_domain_constraints_enumeration_and_cancellation_match_neutral_oracle","wfc_engine::solver_grid3d::tests::grid_domain_constraints_enumeration_and_cancellation_match_neutral_oracle","tagged_and_explicit_selectors_respect_neutral_scoped_cardinality","periodic_sample_solves_on_a_same_size_wrapped_grid","graph_view_conversion_preserves_neutral_directed_and_undirected_arcs","retained_publication_matches_neutral_pages_and_preserves_both_commit_streams","retained_publication_retries_exact_rejected_source_and_honors_zero_fuel","retained_publication_cancellation_closes_finished_and_partial_streams_incrementally","first_preview_and_continuous_gap_include_bounded_publication"],"cargoArgs":["--features","semio-s-artifact-procedural-assembly/component-app-assembly"]},
    {"package":"semio-framework-os-renderer-wgpu","laws":["renderer_result_lane_vectors_decode_and_reject_unknown_tags"],"cargoArgs":["--features","semio-framework-os-renderer-wgpu/native-bin"]},
    {"package":"semio-framework-plugin","laws":["language_neutral_renderer_page_and_exact_ack_have_bounded_stable_wire_fields","full_operation_source_rejects_generic_reducers_and_old_monolithic_shells","spawn_task_quota_gate_faults_the_n_plus_1th_task_and_never_silently_drops_it","local_interaction_live_pages_wait_exact_ack_and_all_three_roots","local_interaction_live_reopened_request_rejects_old_started_cancel","local_interaction_live_partial_admission_retains_successful_roots","local_interaction_live_partial_error_preserves_wrapper_emission_and_retirement_counts","sparse_live_instances_receive_successive_round_robin_turns","runtime_instance_registry_has_fixed_capacity_collision_and_reuse","cleanup_queue_saturation_preserves_detached_app_ownership","cold_pair_ingress_streams_the_exact_four_mibibyte_pair_and_loads_once","cold_pair_ingress_rechecks_live_and_rejects_hostile_pages_without_displacement","cold_pair_ingress_keeps_the_structural_owner_across_load_cancel_and_bounded_close","cold_pair_ingress_final_live_fence_rejects_post_await_revocation","cold_pair_ingress_charges_aggregate_reserved_capacity_until_final_close","cold_pair_ingress_is_an_exact_retained_native_close_participant","cold_pair_header_requires_an_active_checkpoint_frontier_and_exact_hashes","interactive_bridge_coalesces_preview_but_backpressures_lossless_items","interactive_bridge_diagnostic_ring_is_item_and_byte_bounded","spawn_with_id_hands_the_reserved_id_to_the_future_builder_before_it_ever_runs","a_two_slice_infer_job_decodes_then_dispatches_to_the_registered_service","infer_job_checkpoint_restore_matches_an_uninterrupted_run","artifact_inference_registry_is_order_independent_and_idempotent","artifact_inference_registry_rejects_any_conflicting_duplicate","append_chunk_reassembles_a_multi_chunk_body_to_the_exact_original_bytes","append_chunk_over_cap_faults_instead_of_silently_truncating","append_chunk_on_an_unknown_or_already_resolved_id_is_a_harmless_no_op","routed_inference_is_frozen_into_the_plugin_roster_without_a_sync_service","checkpoint_binary_matches_schema_fixture_and_owned_oracle","checkpoint_decode_rejects_context_workspace_and_reserved_byte_drift","world3d_scene_fields_bind_the_domain_while_the_sun_helper_leaves_it_unset","repeated_transient_close_lock_contention_never_consumes_structural_livelock_credit","contended_live_cleanup_does_not_consume_structural_stall_credit","cancellation_numeric_authority_rejects_collision_capacity_and_contention_without_blocking","app_owned_request_context_identity_matches_language_neutral_oracle_and_rejects_every_root_drift","reactor_close_drains_requests_resumes_tasks_timers_and_metadata_in_bounded_steps"]},
    {"package":"semio-s-artifact-cad-cad","laws":["cad_config_operation_snapshot_round_trips_and_restores_exactly","cad_config_set_contributions_round_trips"]},
    {"package":"semio-s-artifact-remodel-remodeling","laws":["raster_asset_progress_layout_matches_neutral_budget","maximum_envelope_mesh_chunks_are_bounded_replayable_and_resolve_across_threads","shared_durable_chunk_admission_accepts_4k_and_rejects_overflow_and_malformed_rows"]},
    {"package":"semio-s-artifact-puzzle-3d","laws":["labels_resolve_every_host_locale_and_terminology_axis","spatial_capacity_plus_one_refusal_preserves_exact_old_state","spatial_stale_owner_cannot_finish_partial_replacement","spatial_multi_cell_oversized_replacement_and_removal_make_bounded_progress","spatial_fixed_collections_use_the_credited_pages_and_return_identical_plus_one_owners","overlap_checkpoint_resumes_exact_rng_and_sample_cursor","overlap_is_deterministic_across_batch_sizes","blocked_vortex_full_ids_and_enumeration_excludes_them","weighted_sample_without_replacement_edge_cases","language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle","same_kind_windows_compose_independently","app_pack_and_spr_exclude_window_transient_and_operation_fields"],"cargoArgs":["--features","semio-s-artifact-puzzle-3d/component-app-assembly"]},
    {"package":"semio-s-artifact-draw-drawing","laws":["retained_drawing_aggregate_credit_admits_exact_4096_rejects_plus_one_with_owner_handback"]},
    {"package":"semio-s-artifact-energy-model","laws":["retained_roster_is_exact_and_exhaustive","p7c2_wire_schema_maximum_plus_one_and_exact_restore_owner","p7c2_restore_stale_step_and_install_preserve_exact_replay_authority","sequential_fills_first_unit","uniform_splits_proportionally_to_capacity","surface_incidence_matches_known_surface_normal"]},
    {"package":"semio-s-artifact-flow-flow","laws":["max_semantic_config_publication_cancel_retry_and_close_use_real_grants"]},
    {"package":"semio-s-artifact-raster-raster","laws":["raster_asset_capacity_matches_the_json_oracle"]},
    {"package":"semio-s-artifact-architect-program","laws":["architect_configuration_contract_vectors_match_the_json_oracle","architect_presence_contract_vectors_match_the_json_oracle","architect_semantic_panels_match_the_json_oracle","sample_plugin_round_trips_json","composed_register_rows_belong_to_each_exact_child"]},
    {"package":"semio-s-artifact-gis-gismap","laws":["language_neutral_vectors_match_geo_bounding_rect_oracle_and_stable_payload","gis2d_config_operation_lines_round_trip"],"cargoArgs":["--features","semio-s-artifact-gis-gismap/component-app-assembly"]},
    {"package":"semio-s-artifact-gis-gisterrain","laws":["retained_command_factory_matches_the_language_neutral_maximum_oracle","strict_snapshot_and_aggregate_json_vectors","gis3d_config_operation_lines_round_trip"],"cargoArgs":["--features","semio-s-artifact-gis-gisterrain/component-app-assembly"]},
    {"package":"semio-s-artifact-shooting-shooting","laws":["shooting_configuration_contract_vectors_match_the_json_oracle","shooting_presence_contract_vectors_match_the_json_oracle","shooting_window_actions_match_the_json_oracle","shooting_semantic_panels_match_the_json_oracle","shooting_shot_field_values_match_the_json_oracle"]},
    {"package":"semio-s-artifact-imperative-procedure","laws":["imperative_configuration_contract_vectors_match_the_json_oracle","imperative_semantic_panels_match_the_json_oracle","working_content_is_owned_by_each_exact_child","render_lists_one_row_per_top_level_step","render_compiles_the_default_document_into_read_only_text","create_step_inverse_law","delete_step_inverse_law","delete_step_missing_target_is_error","reorder_steps_inverse_law","reorder_steps_missing_target_is_error","edit_step_params_inverse_law","edit_step_params_missing_target_is_error","create_step_duplicate_id_fatal_never_applies","create_step_diff_absorb_law","document_text_round_trip_with_applied_operation"]},
    {"package":"semio-s-plugin-imperative","laws":["imperative_viewer_never_mutates","imperative_editor_and_viewer_share_dialect"]},
    {"package":"semio-s-artifact-animate-presentation","laws":["presentation_configuration_contract_vectors_match_the_json_oracle","presentation_presence_contract_vectors_match_the_json_oracle","presentation_semantic_panels_match_the_json_oracle","title_cards_match_the_neutral_xml_oracle","from_dwg_builds_single_slide_deck_from_entity","from_dwg_never_errors_on_empty_drawing"]},
    {"package":"semio-s-artifact-reasoning-wires","laws":["wires_configuration_contract_vectors_match_the_json_oracle","wires_presence_contract_vectors_match_the_json_oracle","wires_semantic_panels_match_the_json_oracle","renders_canvas_scene_for_the_empty_document","renders_canvas_scene_for_the_metabolism_example"]},
    {"package":"semio-s-artifact-sequence-sequence","laws":["sequence_semantic_panels_match_the_json_oracle","sequence_retained_json_measure_matches_the_json_oracle","sequence_carrier_contracts_match_the_json_oracle","sequence_configuration_contract_vectors_match_the_json_oracle","sequence_presence_contract_vectors_match_the_json_oracle","render_produces_a_read_only_scene_for_the_default_document","linear_chain_orders_by_dependency_and_depth_by_distance_from_root","a_two_step_cycle_is_reported_as_not_cycle_free_but_stays_total","a_dangling_edge_is_ignored","diamond_depth_takes_the_longest_incoming_path","standards::v1::subsets::any::schema::inferences::component::tests::inference_determinism_law","standards::v1::subsets::any::schema::inferences::component::tests::inference_default_law"]},
    {"package":"semio-s-artifact-note-note","laws":["note_configuration_contract_vectors_match_the_json_oracle","note_presence_contract_vectors_match_the_json_oracle","note_pdf14_page_contract_matches_the_json_oracle","note_semantic_panels_match_the_json_oracle","note_ink_canvas_payload_matches_the_json_oracle","note_document_round_trips_assets_and_grid_settings","root_scalar_inverse_and_absorb_laws","asset_inverse_law_create_replace_delete","block_lifecycle_inverse_law_create_delete_duplicate","block_reparent_and_drag_inverse_law","block_field_inverse_laws","table_row_column_inverse_laws","create_block_duplicate_id_is_fatal","delete_block_missing_target_is_error","delete_blocks_missing_target_is_error","rename_block_missing_target_is_error","change_block_locked_missing_target_is_error","move_block_missing_target_is_error","move_block_non_finite_is_fatal","resize_block_missing_target_is_error","drag_blocks_missing_target_is_error","duplicate_block_missing_source_is_error","insert_table_row_missing_target_is_error","remove_table_row_missing_target_is_error","edit_block_text_missing_target_is_error","replace_asset_payload_missing_target_is_error","create_asset_duplicate_id_is_fatal","delete_asset_missing_target_is_error"]},
    {"package":"semio-s-artifact-layout-layout","laws":["layout_configuration_contract_vectors_match_the_json_oracle","layout_presence_contract_vectors_match_the_json_oracle","layout_pdf_page_collection_matches_the_json_oracle","layout_inspection_summary_matches_the_json_oracle","background_drawing_and_referenced_model_round_trip_through_text_and_binary","absent_composition_slots_round_trip_as_none","typed_document_json_matches_serde_and_every_write_is_credit_bounded","create_page_obeys_the_inverse_and_absorb_laws","move_frame_obeys_the_inverse_law","rename_layout_obeys_the_inverse_law","delete_page_obeys_the_inverse_law","reorder_pages_obeys_the_inverse_law","update_page_margins_obeys_the_inverse_law","change_frame_fill_obeys_the_inverse_law","edit_story_and_create_link_obey_the_inverse_law","create_frame_missing_target_is_error","delete_frame_missing_target_is_error","move_frame_missing_target_is_error","reorder_pages_missing_target_is_error","rename_page_missing_target_is_error","change_page_height_missing_target_is_error","edit_story_missing_target_is_error","create_page_duplicate_id_is_fatal","admitted_maximum_and_production_grant_make_bounded_progress"]},
    {"package":"semio-s-artifact-fem-2d","laws":["vector_layer_vectors_match_the_json_oracle","process_owner_inventory_admits_exact_maximum_and_returns_exact_credit","admitted_maximum_and_production_grant_make_bounded_progress"],"cargoArgs":["--features","semio-s-artifact-fem-2d/component-app-assembly"]},
    {"package":"semio-s-plugin-playbook-procedural","laws":["procedural_payload_vectors_match_the_json_oracle","procedural_parameter_controls_match_the_json_oracle","procedural_actor_descriptor_matches_the_json_oracle","module_app_declares_window_kinds","module_manifest_contributes_building_component"]},
    {"package":"semio-s-artifact-writer-writer","laws":["pdf_page_text_vectors_match_the_json_oracle","writer_into_pdf_preserves_text_and_page_size","writer_completion_rejection_retires_child_before_command_without_reemission","bounded_text_admission_preserves_rejected_job_state_and_owners","bounded_open_document_admission_preserves_maximum_plus_one_job_state_and_owners","bounded_host_load_and_engagement_admission_reject_plus_one_without_consuming_owners","writer_artifact_store_preparation_is_exact_bounded_and_reversible","retained_wire_decoder_and_third_party_serde_have_command_parity","writer_window_state_mutations_are_exact_reversible_and_codec_stable"]},
    {"package":"semio-framework-os-kernel","laws":["document_codec_of_round_trips_dsl_and_pack_and_edit_text","register_document_codec_rejects_a_duplicate_schema_without_replacing_the_first","dialect_migration_preflight_and_batch_commit_are_conflict_free_or_noop","space_history_verbs_match_the_language_neutral_contract","str_eq_matches_std_partial_eq","retained_group_history_switches_every_direct_reader_at_one_decision","fixed_history_reservation_returns_exact_rejected_owner_and_blocks_aba","derive_mutations_wires_complete_leaf_and_atomic_registration","counter_fixture_codecs_and_descriptors","counter_fixture_checked_add_and_structural_diff","counter_fixture_mixed_inverse_stored_order","counter_fixture_exact_i64_codecs","ordered_diff_preserves_step_admission_and_associativity","ordered_counter_algebra_matches_exact_neutral_boundaries","minimum_add_inverse_obeys_store_reverse_order","every_path_mount_in_this_glue_resolves_to_an_existing_file","turn_fault_and_cancel_retain_then_close_one_owner_per_grant","quiet_pool_saturation_retains_exact_successor_for_timer_wheel_retry","idle_then_late_send_upgrades_the_host_retained_runner_once","detach_while_pending_retains_future_then_cancel_closes_one_owner","registered_rejected_pages_obey_zero_short_and_exact_grants","unadmitted_rejected_pages_obey_zero_short_and_exact_grants","directory_command_receipt_v1_matches_language_neutral_vectors_and_rejects_hostiles","document_descriptor_matches_the_language_neutral_fixture","canonical_sealer_checkpoint_maximum_accepts_exact_framing_and_identity_overhead_only"],"cargoArgs":["--features","semio-framework-os-kernel/sync"]},
    {"package":"semio-framework-actor","laws":["pack_round_trip_turn_result","mounted_fixed_replay_capture_is_deterministic_and_returns_the_exact_live_owner","mounted_replay_records_and_replays_the_exact_cancelled_terminal_classification","mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged","mounted_replay_preserves_the_exact_fault_payload_and_prefix_across_replay","job_progress_fixed_capacity_and_aba_admission_fail_closed","job_progress_commit_validates_live_authority_and_rejected_close_is_incremental","job_bridge_invokes_exactly_one_step_per_turn","job_bridge_preserves_checkpoint_state_and_applied_progress","job_bridge_cancellation_is_terminal_and_skips_the_job","job_bridge_rejects_stale_commit_before_work_or_publication","job_bridge_rejects_replayed_preview_identity_before_work","job_bridge_rejects_a_preview_without_exactly_one_sequence_advance"]},
    {"package":"semio-framework-os-services","laws":["path_set_max_plus_one_identity_zero_grant_and_job_close_are_exact","chunked_read_write_scan_and_modified_round_trip","resident_memory_observation_does_not_spawn_a_process","event_router_latest_wins_collapses_older_pending_value","event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth","event_router_coalesced_collapses_same_key_but_queues_distinct_keys","event_router_ring_overwrites_oldest_by_item_and_byte_bounds","event_router_payload_bytes_are_enforced_for_every_queueing_policy","event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket","event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber"]},
    {"package":"semio-s-artifact-mathematical-equation","laws":["language_neutral_mutations_match_json_oracle_and_restore_base","tests_keeps_an_already_directed_graph_directed::produces_committed_diff","tests_keeps_an_already_directed_graph_directed::committed_diff_is_canonical","tests_keeps_an_already_directed_graph_directed::committed_diff_applies_to_after","tests_restates_the_unset_algorithm_and_its_absent_seed::produces_committed_diff","tests_restates_the_unset_algorithm_and_its_absent_seed::committed_diff_is_canonical","tests_restates_the_unset_algorithm_and_its_absent_seed::committed_diff_applies_to_after","tests_replays_the_identical_empty_graph::produces_committed_diff","tests_replays_the_identical_empty_graph::committed_diff_is_canonical","tests_replays_the_identical_empty_graph::committed_diff_applies_to_after","tests_replays_the_identical_empty_point_cloud::produces_committed_diff","tests_replays_the_identical_empty_point_cloud::committed_diff_is_canonical","tests_replays_the_identical_empty_point_cloud::committed_diff_applies_to_after","tests_seeds_the_empty_cloud_with_its_first_point::produces_committed_diff","tests_seeds_the_empty_cloud_with_its_first_point::committed_diff_is_canonical","tests_seeds_the_empty_cloud_with_its_first_point::committed_diff_applies_to_after","tests_raises_the_leading_coefficient_to_three_halves::produces_committed_diff","tests_raises_the_leading_coefficient_to_three_halves::committed_diff_is_canonical","tests_raises_the_leading_coefficient_to_three_halves::committed_diff_applies_to_after","the_composed_child_triple_is_never_re_minted","math_config_dsl_round_trips","config_operation_set_camera_diff_writes_the_targeted_field","config_operation_set_camera_round_trips","retained_semantic_maxima_accept_exact_and_reject_maximum_plus_one","retained_interruption_replay_aba_cancel_and_repeated_close_are_exact","retained_maximum_microturns_stay_below_eight_milliseconds"]},
    {"package":"semio-s-artifact-trinity-jack","laws":["editor::jack::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle","editor::jack::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle"],"cargoArgs":["--features","semio-s-artifact-trinity-jack/component-app-assembly"]},
    {"package":"semio-s-artifact-trinity-rewriting","laws":["editor::rewriting::config::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle","editor::rewriting::presence::component::contract_vectors::configuration_and_presence_contract_vectors_match_the_json_oracle","trinity_lod_scale_json_lists_all_six_lods"],"cargoArgs":["--features","semio-s-artifact-trinity-rewriting/component-app-assembly"]},
    {"package":"semio-framework-os-infinite","laws":["board_fixture_json_vectors_match_the_json_oracle","icon_codec_resolves_metabolism_shortcode_to_themed_svg","wheel_plan_matches_direct_and_rejects_stale_interaction","typed_camera_snapshot_matches_current_camera_fixture_without_production_parsing","retained_draw_rebuild_keeps_url_backed_asset_authority","retained_draw_rebuild_preserves_prepared_material_colors_from_the_json_oracle","retained_draw_rebuild_preserves_mixed_group_and_instance_fifo_then_swaps_atomically","mesh_pool_release_clears_at_zero_refcount","terrain_writer_matches_legacy_bands_and_closes_interrupted_authority","world_marquee_mesh_cursor_matches_legacy_window_crossing_disjoint_and_degenerate_cases","world_marquee_lasso_edge_cursor_matches_legacy_and_rejects_object_aba","world_marquee_page_claim_saturation_preserves_all_results_for_exact_retry","world_gumball_update_validates_one_selected_aba_token_per_turn","world_gumball_commit_saturation_aba_and_interrupted_close_retain_claim_authority","asset_authority_rejects_request_and_byte_capacity_plus_one_before_string_ownership","asset_response_rejects_page_plus_one_and_retires_partial_stream_one_page_per_step","asset_decode_resume_and_stale_generation_keep_claimed_pages_until_terminal_return","asset_unknown_length_releases_unused_aggregate_credit_at_seal","asset_completed_cursor_advances_one_fixed_slot_per_grant_and_hands_back_exact_owner"]},
    {"package":"semio-s-artifact-playbook-playbook","laws":["configuration_and_presence_contract_vectors_match_the_json_oracle","render_builder_emits_playbook_list_component_scene"]},
    {"package":"semio-s-artifact-forms-forms","laws":["forms_configuration_contract_vectors_match_the_json_oracle","vector_replacement_boundaries_match_the_json_oracle","semantic_question_controls_match_the_language_neutral_vectors","renders_blueprint_builder_cards","large_unrelated_config_and_existing_vector_stay_under_one_bounded_slice","vector_growth_writes_at_most_sixty_four_components_per_slice","missing_non_array_and_malformed_targets_keep_best_effort_semantics","scalar_option_and_object_shapes_stay_intact","bounded_chunk_values_match_the_json_oracle"]},
    {"package":"semio-framework-surface","laws":["graph_host_sync_from_scene_pack_decodes_pack_shell","map_render_mode_parse","map_vector_style_parse","sync_map_json_keeps_position_labels","sync_map_json_parses_rich_position_metadata","owned_protobuf_decodes_layer_properties_and_point_geometry","fixture_linestrings_split_at_moveto","map_polyline_intersects_rect_detects_edge_crossing_without_endpoints_inside","map_polyline_intersects_polygon_detects_crossing_edge","pointer_up_emits_camera_after_middle_button_pan","interaction_plan_matches_direct_wheel_and_pan_semantics","build_vector_scene_respects_render_mode","build_vector_scene_respects_vector_style","node_record_to_spec_builds_app_instance_kind"]},
    {"package":"semio-framework-editor","laws":["char_boundary_helpers_handle_multibyte","offset_line_col_roundtrip","sync_from_scene_json_sets_and_clears_hover_range","sync_from_scene_json_applies_all_optional_fields"]},
    {"package":"semio-s-plugin-imperative-effect","laws":["bundle_contributes_core_module_for_imperative_play"]},
    {"package":"semio-framework-artifact-flow-flow","laws":["flow_selected_copy_matches_serde_and_shares_unchanged_ordered_roots","flow_selected_copy_cancellation_and_invalid_projection_preserve_root_until_close","flow_selected_copy_nonterminal_drop_is_guarded_without_destroying_root","flow_selected_copy_rejects_root_retirement_overgrant_and_closes_factory_owner","flow_selected_copy_allocation_admission_is_separate_and_never_reallocates_payload_pages"]},
    {"package":"semio-framework-artifact-infinite-dag","laws":["direct_intrinsic_serde_and_selection_are_lossless","dag_document_dsl_round_trips_every_node_kind","dag_document_dsl_round_trips_the_demo_fixture"]},
    {"package":"semio-framework-artifact-playbook-playbook","laws":["ordered_document_fixture_matches_serde_oracle"]},
    {"package":"semio-s-artifact-puzzle-2d","laws":["language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle","utility_registry_declares_utilities","redraw_force_graph_top_level_locked_node_ids_pins","redraw_force_graph_wraps_flat_options","edge_handle_snap_sets_circle_handle_angles_on_center_line","redraw_force_graph_with_snap_sets_handle_angles","force_graph_accepts_logical_nodes_without_xy","hierarchical_tree_normal_mode_node_id_edges_stacks_by_depth","hierarchical_tree_stacks_by_depth","hierarchical_tree_pins_locked_root_coordinates","redraw_hierarchical_tree_nested_locked_node_ids_pins","hierarchical_tree_right_places_children_larger_x_than_root","hierarchical_tree_upwards_places_children_smaller_y_than_root","hierarchical_tree_rejects_unknown_direction","redraw_rejects_unknown_mode","context_menu_grouped_disclosure_stays_within_budget_and_keeps_destructive_last"],"cargoArgs":["--features","semio-s-artifact-puzzle-2d/component-app-assembly"]},
    {"package":"semio-s-artifact-puzzle-5d","laws":["language_neutral_fixtures_match_production_catalogs_through_the_owned_oracle","context_menu_is_grouped_and_keeps_delete_selection_last","window_engagements_cover_both_windows","set_active_utility_emits_no_ops_and_no_history_entry","engagements_expose_no_utility_switch_options_for_either_window"],"cargoArgs":["--features","semio-s-artifact-puzzle-5d/component-app-assembly"]},
  ].filter(group => !name || name.split(",").includes(group.package));
  if (!groups.length) throw new Error("No matching native verification group");

  if (command === "laws-fleet") {
    const { mkdirSync, mkdtempSync, rmSync } = await import("node:fs");
    const { runExactCargoLawProcess, exactExecutableFingerprint, EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX, EXACT_CARGO_ACTIVE_LEASE_MANIFEST } = await import(resolve(process.cwd(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts"));
    const batches = [...new Set(groups.map(group => JSON.stringify("target" in group ? group.target : { kind: "lib" })))].map(key => {
      const target = JSON.parse(key);
      const selected = groups.filter(group => JSON.stringify("target" in group ? group.target : { kind: "lib" }) === key);
      const features = selected.flatMap(group => {
        const args = "cargoArgs" in group ? group.cargoArgs ?? [] : [];
        if (args.length && (args.length !== 2 || args[0] !== "--features")) throw new Error("Unexpected shared-build arguments for " + group.package);
        return [...(args[1]?.split(",") ?? []), ...(group.package === "semio-framework-os" ? ["semio-framework-os/os-host-full"] : [])];
      });
      return { target, groups: selected, args: ["test", ...(target.kind === "lib" ? ["--lib"] : ["--" + target.kind, target.name]), ...selected.flatMap(group => ["-p", group.package]), ...(features.length ? ["--features", [...new Set(features)].sort().join(",")] : []), "--no-run", "--message-format=json", "--keep-going", "-j2"] };
    });
    if (filter === "--scope-only") {
      console.log(JSON.stringify(batches, null, 2));
      process.exit(0);
    }
    for (const file of readdirSync(generated).filter(file => /^native\d+-process\.json$/.test(file))) {
      if (existsSync(join(generated, file.replace("-process.json", "-receipt.json")))) continue;
      let live = false;
      try { process.kill(JSON.parse(readFileSync(join(generated, file), "utf8")).pid, 0); live = true; } catch {}
      if (live) throw new Error("The active native fleet check must finish before test compilation");
    }
    mkdirSync(join(generated, "native-laws"), { recursive: true });
    const runRoot = mkdtempSync(join(generated, "native-laws", "fleet-"));
    const leaseRoot = mkdtempSync(join(generated, EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX));
    const heartbeat = () => writeFileSync(join(leaseRoot, EXACT_CARGO_ACTIVE_LEASE_MANIFEST), JSON.stringify({ version: 1, pid: process.pid }), { mode: 0o600 });
    heartbeat();
    const timer = setInterval(heartbeat, 10_000);
    timer.unref();
    let interrupted = false;
    process.once("SIGINT", () => { interrupted = true; });
    process.once("SIGTERM", () => { interrupted = true; });
    const cancelled = () => interrupted || existsSync(join(runRoot, "cancel.json"));
    const env = { ...process.env, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", CARGO_TARGET_DIR: join(generated, "derive-target"), TMPDIR: generated };
    const results: { package: string; selector: string; law?: string; passed: boolean; error?: string }[] = [];
    const builds: { status: number | null; errors: number; warnings: number; cargoWarnings: string[]; directory: string }[] = [];
    let failed = false;
    const reportPath = join(import.meta.dir, "📓️2026-09-09-combined-runtime.md");
    const capture = async (directory: string, label: string, executable: string, args: string[], budgetMs: number) => {
      if (cancelled()) throw new Error("Combined test run cancelled");
      const result = await runExactCargoLawProcess(executable, args, { cwd: process.cwd(), env, budgetMs, maxOutputBytes: budgetMs === 0 ? 256 * 1024 * 1024 : 8 * 1024 * 1024, stdoutPath: join(directory, label + ".stdout"), stderrPath: join(directory, label + ".stderr"), cancelled });
      writeFileSync(join(directory, label + ".json"), JSON.stringify({ executable, args, status: result.status, signal: result.signal, reason: result.reason }, null, 2));
      return result;
    };
    writeFileSync(join(runRoot, "scope.json"), JSON.stringify(batches, null, 2));
    writeFileSync(join(runRoot, "process.json"), JSON.stringify({ pid: process.pid, startedAt: new Date().toISOString() }));
    console.log("[DEBUG] Combined exact runtime artifacts: " + runRoot);
    appendFileSync(reportPath, "\n## Combined Runtime Dispatch\n\nArtifact directory: " + relative(import.meta.dir, runRoot) + ". Compiling " + groups.length + " explicit package targets in " + batches.length + " shared Cargo builds. Features are the union of the selected catalog's declared features; this verifies the combined application configuration. Each native assertion is discovered exactly and bound to its executable hash.\n");
    try {
      for (const [batchIndex, batch] of batches.entries()) {
        const directory = join(runRoot, "build-" + batchIndex);
        mkdirSync(directory);
        console.log("[DEBUG] Compiling " + batch.groups.length + " " + batch.target.kind + " targets");
        const built = await capture(directory, "build", "cargo", batch.args, 0);
        const messages = built.stdout.split("\n").flatMap(line => { try { return [JSON.parse(line)]; } catch { return []; } });
        const diagnostics = messages.filter(row => row.reason === "compiler-message");
        const counts = { status: built.status, errors: diagnostics.filter(row => row.message.level === "error").length, warnings: diagnostics.filter(row => row.message.level === "warning").length, cargoWarnings: [...new Set(built.stderr.split("\n").filter(line => /^warning:/.test(line)))], directory };
        builds.push(counts);
        writeFileSync(join(directory, "diagnostics.json"), JSON.stringify(diagnostics, null, 2));
        if (built.status !== 0 || built.signal !== null || built.reason !== "exit" || counts.errors || counts.warnings || counts.cargoWarnings.length) failed = true;
        console.log("[DEBUG] Shared build " + JSON.stringify(counts));
        for (const [groupIndex, group] of batch.groups.entries()) {
          const groupRoot = join(directory, "group-" + groupIndex);
          mkdirSync(groupRoot);
          try {
            const artifacts = messages.filter(row => {
              if (row.reason !== "compiler-artifact" || row.profile?.test !== true || typeof row.executable !== "string") return false;
              const packageId = String(row.package_id);
              const packageName = packageId.includes("#") ? packageId.slice(packageId.lastIndexOf("#") + 1).split("@")[0] : packageId.split(" ")[0];
              return packageName === group.package && row.target.kind.some(kind => batch.target.kind === "lib" ? ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"].includes(kind) : kind === batch.target.kind) && (!batch.target.name || row.target.name === batch.target.name);
            });
            if (artifacts.length !== 1) throw new Error("Expected one completed Cargo executable, got " + artifacts.length);
            const executable = exactExecutableFingerprint(artifacts[0].executable, { cancelled });
            const verify = () => { if (exactExecutableFingerprint(executable.path, { cancelled }).sha256 !== executable.sha256) throw new Error("Executable changed after shared build"); };
            writeFileSync(join(groupRoot, "executable.json"), JSON.stringify({ package: group.package, target: batch.target, buildDirectory: directory, ...executable }, null, 2));
            const listed = await capture(groupRoot, "list", executable.path, ["--list"], 60_000);
            verify();
            if (listed.status !== 0 || listed.signal !== null || listed.reason !== "exit") throw new Error("Native discovery failed");
            const discovered = listed.stdout.split(/\r?\n/u).filter(line => line.endsWith(": test")).map(line => line.slice(0, -6));
            for (const [lawIndex, selector] of group.laws.entries()) {
              try {
                const matches = discovered.filter(law => law === selector || law.endsWith("::" + selector));
                if (matches.length !== 1) throw new Error("Exact discovery selected " + matches.length + " laws");
                const law = matches[0];
                verify();
                const result = await capture(groupRoot, "law-" + lawIndex, executable.path, [law, "--exact", "--test-threads=1", "--show-output"], 60_000);
                verify();
                const terminals = [...result.stdout.matchAll(/^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/gm)];
                const passed = result.status === 0 && result.signal === null && result.reason === "exit" && terminals.length === 1 && terminals[0][1] === "1" && terminals[0][2] === "0" && terminals[0][3] === "0" && result.stdout.split(/\r?\n/u).includes("test " + law + " ... ok");
                results.push({ package: group.package, selector, law, passed, ...(!passed ? { error: (result.stdout + result.stderr).slice(-6000) } : {}) });
                if (!passed) failed = true;
                console.log("[DEBUG] " + (passed ? "PASS " : "FAIL ") + group.package + " " + selector);
              } catch (error) {
                failed = true;
                results.push({ package: group.package, selector, passed: false, error: String(error) });
                console.log("[DEBUG] FAIL " + group.package + " " + selector + ": " + String(error));
              }
              writeFileSync(join(runRoot, "progress.json"), JSON.stringify({ builds, results }, null, 2));
              if (cancelled()) throw new Error("Combined test run cancelled");
            }
          } catch (error) {
            failed = true;
            for (const selector of group.laws) if (!results.some(row => row.package === group.package && row.selector === selector)) results.push({ package: group.package, selector, passed: false, error: String(error) });
            console.log("[DEBUG] " + group.package + ": " + String(error));
          }
          if (cancelled()) throw new Error("Combined test run cancelled");
        }
      }
    } catch (error) {
      failed = true;
      appendFileSync(reportPath, "\nCombined run interruption: " + String(error) + ".\n");
      console.error(String(error));
    } finally {
      writeFileSync(join(runRoot, "receipt.json"), JSON.stringify({ finishedAt: new Date().toISOString(), failed, cancelled: cancelled(), builds, results }, null, 2));
      appendFileSync(reportPath, "\nCompleted runtime attempt: " + results.filter(row => row.passed).length + " exact assertions passed; " + results.filter(row => !row.passed).length + " failed or could not run. Overall success: " + !failed + ".\n\n" + results.filter(row => !row.passed).map(row => "- " + row.package + "::" + row.selector + ": " + row.error).join("\n") + "\n");
      clearInterval(timer);
      rmSync(leaseRoot, { recursive: true, force: true });
    }
    process.exit(failed ? 1 : 0);
  }

  let failures = 0;
  for (const group of groups) {
    let artifactDir = "";
    try {
      const receipts = await runExactCargoLaws({ cwd: process.cwd(), groups: [{ ...group, target: "target" in group ? group.target : { kind: "lib" } }], artifactDir: join(generated, "native-laws"), cargoArgs: group.package === "semio-framework-os" ? ["-j2", "--features", "semio-framework-os/os-host-full"] : ["-j2"], env: { ...process.env, CARGO_INCREMENTAL: "0", RUSTC_WRAPPER: "", CARGO_TARGET_DIR: join(generated, "derive-target"), TMPDIR: generated }, buildBudgetMs: 0, progress(event) { artifactDir = event.artifactDir; console.log(`[DEBUG] ${event.package} ${event.stage} ${event.law ?? ""}`); } });
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
  const snapshots = new Map<string, Buffer>();
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
        if (!snapshots.has(file)) snapshots.set(file, content);
        changes.set(`${span.byte_start}:${span.byte_end}`, { start: span.byte_start, end: span.byte_end, replacement: span.suggested_replacement, original });
        byFile.set(file, changes);
      }
    }
  }
  const changed: string[] = [];
  for (const [file, changes] of byFile) {
    let content = readFileSync(file);
    const snapshot = snapshots.get(file)!;
    if (!content.equals(snapshot)) { console.log("[DEBUG] Skipped concurrently changed file: " + file); continue; }
    let boundary = content.length;
    for (const edit of [...changes.values()].sort((a, b) => b.start - a.start)) {
      if (edit.end > boundary || content.subarray(edit.start, edit.end).toString() !== edit.original) continue;
      console.log(`${relative(process.cwd(), file)}: ${edit.original} → ${edit.replacement}`);
      content = Buffer.concat([content.subarray(0, edit.start), Buffer.from(edit.replacement), content.subarray(edit.end)]);
      boundary = edit.start;
    }
    if (command === "apply") {
      if (!readFileSync(file).equals(snapshot)) { console.log("[DEBUG] Skipped concurrently changed file: " + file); continue; }
      writeFileSync(file, content);
      changed.push(relative(process.cwd(), file));
    }
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
