import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/testing/cargo/exact";
import { runAdmittedExactCargoLawsV1 as runExactCargoLaws } from "@semio-tech/process/cargo-test";
import { readCargoTestPolicyV1 } from "@semio-tech/process/cargo-test";
import { scriptInvocationBudget } from "@semio-tech/process/routing";
import { runOwnedCommand } from "@semio-tech/process/owned-execution";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const FEM3D_VERIFICATION_LAWS = [
  "editor::fem3d::interaction::gumball::tests::wide_selection_resolves_each_geometry_once_in_document_order",
  "editor::fem3d::commands::patch_solid::tests::layer_count_text_admission_is_exact_and_sparse",
  "standards::v1::subsets::any::schema::move_selection::component::tests::transform_authors_only_changed_geometry_and_preserves_peer_fields",
  "standards::v1::subsets::any::schema::move_selection::component::tests::finite_transform_overflow_is_refused_atomically_without_an_inverse",
  "standards::v1::subsets::any::schema::move_selection::component::tests::rotation_and_scaling_inverse_leave_unmoved_peer_coordinates_intact",
  "editor::fem3d::commands::remove_selection::tests::every_selected_entity_is_removed_after_its_selected_referrers_and_restored",
  "editor::fem3d::commands::remove_selection::tests::selected_loads_remove_once_without_removing_their_case_or_geometry",
  "editor::fem3d::interaction::gumball::tests::streamed_ticks_commit_the_net_leaf_once",
  "editor::fem3d::interaction::gumball::tests::host_aborts_leave_zero_trace_and_keep_sibling_gestures",
  "editor::fem3d::interaction::gumball::tests::a_gumball_move_edited_in_history_replays_its_downstream",
  "editor::fem3d::interaction::scoped_tests::scoped_targets_separate_all_entity_kinds_and_repeated_load_ids",
  "editor::fem3d::interaction::scoped_tests::scoped_load_commands_modify_only_the_addressed_case_and_reject_raw_ids",
  "editor::fem3d::interaction::scoped_tests::viewport_identity_collisions_are_distinct_and_unencodable_ids_refuse_scene",
  "editor::fem3d::interaction::scoped_tests::snapshot_identity_admission_preserves_namespaces_and_refuses_invalid_owners",
  "editor::fem3d::interaction::scoped_tests::public_json_identity_admission_refuses_invalid_owners_and_controlled_scratch_is_metered",
  "editor::fem3d::panels::inspection::tests::a_selected_load_names_its_owning_case_3d",
  "editor::fem3d::panels::inspection::tests::a_multi_selection_headers_the_count_and_inspects_the_first_3d",
  "editor::fem3d::panels::inspection::tests::a_selected_load_case_lists_its_loads_as_picks_3d",
  "editor::fem3d::panels::artifact::tests::rows_are_keyed_by_scoped_entity_identity_and_bound_to_the_fem3d_domain",
  "editor::fem3d::panels::artifact::tests::load_case_rows_nest_their_loads_and_combination_rows_nest_their_terms",
  "editor::fem3d::unit_tests::every_command_keeps_its_current_bytes",
  "editor::fem3d::unit_tests::context::history_edits_end_to_end",
  "editor::fem3d::unit_tests::context::history_edit_inputs_resolve",
  "editor::fem3d::unit_tests::context::conflict_history_edits_end_to_end",
  "editor::fem3d::unit_tests::undo_restores_document_after_add_node",
  "editor::fem3d::unit_tests::undo_restores_document_after_add_support",
  "editor::fem3d::unit_tests::add_support_without_a_node_does_not_journal_a_phantom_edit",
  "editor::fem3d::commands::gumball::tests::one_drag_is_one_edit_one_row_and_one_transaction",
  "editor::fem3d::commands::gumball::tests::an_abort_leaves_zero_trace_and_two_drags_are_two_transactions",
  "editor::fem3d::commands::gumball::tests::the_world3d_gumball_live_protocol_lands_as_its_guest_edits",
  "live_visual::tests::fem3d_visual_stale_cancel_fault_deadline_and_interrupted_device_close_preserve_last_valid",
  "editor::fem3d::unit_tests::mounted_peer_catch_up_preserves_independent_fields_and_deduplicates_events",
  "editor::fem3d::commands::patch_node::tests::patch_node_emits_only_the_authored_ordinate_3d",
] as const;

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  if (segments[0] === "fem3d-results") {
    assert(segments.length === 2 && ["source", "native"].includes(segments[1]!), "results verification requires source or native");
    const source = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/🎞️results/🟦️.ts");
    const shapeSource = resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎼️shape/🟦️.ts");
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", context.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", source, shapeSource], context.repoRoot, "fem3d:results-source-typecheck", scriptInvocationBudget(context.invocation, 120000), { signal: context.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
    const { testSubspaceShapeOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎼️shape/🟦️.ts");
    testSubspaceShapeOracle();
    const { testFem3dRetainedResultsOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/🎞️results/🟦️.ts");
    testFem3dRetainedResultsOracle();
    if (segments[1] === "source") return;
    const policy = readCargoTestPolicyV1(process.env), manifest = resolve(context.root, "Cargo.toml");
    assert.equal(resolve(policy.manifestPath), manifest, "results laws require the original FEM3D owner manifest");
    const laws = ["live_visual::tests::fem3d_wide_physical_request_and_window_text_use_shared_granted_custody", "live_visual::tests::fem3d_original_fault_and_cancel_terminal_receipts_prevent_implicit_resubmit", "live_visual::tests::fem3d_request_families_share_identical_panes_and_preserve_physical_sources", "live_visual::tests::fem3d_exact_app_leases_keep_colliding_jobs_and_close_isolated", "editor::fem3d::interaction::gumball::tests::wide_selection_resolves_each_geometry_once_in_document_order", "live_visual::tests::fem3d_results_retained_pages_route_only_matching_completed_sources", "live_visual::tests::fem3d_retained_results_close_uses_original_retirement_and_preserves_foreign_apps", "live_visual::tests::fem3d_mounted_selected_sources_match_pynite_and_case_order_is_irrelevant",
          "live_visual::tests::fem3d_source_preflight_rejects_ambiguous_missing_nonfinite_and_excess_sources", "live_visual::tests::fem3d_production_numerical_child_solves_the_demo_with_paged_owner_backing", "editor::fem3d::modes::edit::windows::results::tests::mounted_results_frames_never_invoke_synchronous_solvers", "editor::fem3d::component::unit_tests::mounted_camera_and_interaction_presence_reach_peers_without_document_edits"];
    const fem2Manifest = resolve(context.root, "../../../◻️2d/📦️packages/🦀️rust/Cargo.toml");
    const fem2Laws = ["sparse::tests::subspace_shape_admission_matches_exact_paged_layout_arithmetic", "editor::fem2d::session::tests::mounted_app_admission_isolates_colliding_ids_until_original_children_return", "editor::fem2d::session::tests::mounted_app_close_keeps_borrowed_original_child_and_full_digest_authority"];
    const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation, policy: configuredExactCargoLawPolicyV1(), manifestPaths: { "semio-s-artifact-fem-2d": fem2Manifest, "semio-s-artifact-fem-3d": manifest }, cwd: context.repoRoot, groups: [{ package: "semio-s-artifact-fem-3d", target: { kind: "lib", name: "semio_s_artifact_fem_3d" }, cargoArgs: ["--locked", "--features", "component-app-assembly"], laws }, { package: "semio-s-artifact-fem-2d", target: { kind: "lib", name: "semio_s_artifact_fem_2d" }, cargoArgs: ["--locked", "--features", "component-app-assembly"], laws: fem2Laws }] });
    assert.equal(receipts[0]!.assertions, laws.length); assert.equal(receipts[1]!.assertions, fem2Laws.length);
    return;
  }
  if (segments[0] === "fem3d-transform-intent") {
    if (segments.length !== 2 || !["source", "native"].includes(segments[1]!)) throw Error("fem3d-transform-intent requires source or native");
    const sources = ["../../🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🔬️unit/🟦️.ts", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🟦️.ts", "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts"].map(path => resolve(context.root, path));
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", context.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...sources], context.repoRoot, "fem3d:source-typecheck", scriptInvocationBudget(context.invocation, 120000), { signal: context.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
    const { testFem3dTransformIntentOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/🧭️move-selection/🧪️tests/🔬️unit/🟦️.ts");
    testFem3dTransformIntentOracle();
    const { testFem3dSelectionProjectionOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧭️gumball/🧪️tests/🎯️projection/🟦️.ts");
    testFem3dSelectionProjectionOracle();
    const { testFem3dRemovalOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/🗂️remove-selection/🧪️tests/🔬️unit/🟦️.ts");
    testFem3dRemovalOracle();
    const { testFem3dIdentityOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🕹️interaction/🧪️tests/🔬️unit/🟦️.ts");
    testFem3dIdentityOracle();
    if (segments[1] === "source") return;
    const policy = readCargoTestPolicyV1(process.env), manifest = resolve(context.root, "Cargo.toml");
    assert.equal(resolve(policy.manifestPath), manifest, "FEM3d exact laws require the original selected manifest");
    const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation, policy: configuredExactCargoLawPolicyV1(), manifestPaths: { "semio-framework-dsl-record": resolve(context.repoRoot, "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust/Cargo.toml"), "semio-framework-replication": resolve(context.repoRoot, "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml"), "semio-framework-os-kernel": resolve(context.repoRoot, "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml"), "semio-s-artifact-fem-3d": manifest }, cwd: context.repoRoot, groups: [{ package: "semio-framework-dsl-record", target: { kind: "lib", name: "semio_framework_dsl_record" }, cargoArgs: ["--locked"], laws: ["tests::borrowed_object_tests::declared_variant_field_has_one_named_record_and_preserves_control"] }, { package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, cargoArgs: ["--locked"], laws: ["mutation::map::tests::shared_map_controlled_codec_and_dsl"] }, { package: "semio-framework-os-kernel", target: { kind: "lib", name: "semio_framework_os_kernel" }, cargoArgs: ["--locked"], laws: ["os_store::component::paged_one_item::mutation_apply::tests::mutation_apply_inverse_cancellation_uses_original_factory", "os_store::component::paged_one_item::mutation_apply::tests::source_custody::paged_original_mutation_sources_keep_genuine_refs_and_exact_issuer_through_all_outcomes", "os_store::component::paged_one_item::mutation_apply::tests::mutation_apply_diagnostics_gate_atomic_publication", "os_store::component::paged_one_item::mutation_apply::tests::mutation_apply_cancellation_after_decision_preserves_base"] }, { package: "semio-s-artifact-fem-3d", target: { kind: "lib", name: "semio_s_artifact_fem_3d" }, cargoArgs: ["--locked", "--features", "component-app-assembly"], laws: FEM3D_VERIFICATION_LAWS }] });
    assert.equal(receipts.length, 4);
    assert.equal(receipts[0]!.assertions, 1);
    assert.equal(receipts[1]!.assertions, 1);
    assert.equal(receipts[2]!.assertions, 4);
    assert.equal(receipts[3]!.assertions, FEM3D_VERIFICATION_LAWS.length);
    return;
  }
  if (["fem3d-numerical-child-native", "fem3d-numerical-child-source"].includes(segments[0]!)) {
    assert(segments.length === 1 || (segments.length === 2 && segments[1] === "source"), "numerical verification accepts only an explicit source selector");
    const closeRoot = resolve(context.root, "../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session");
    const stiffnessRoot = resolve(context.root, "../../../../../../🔨️modules/🏗️fem/⚙️engine/🧱️elements3d");
    const closeCorpus = JSON.parse(readFileSync(resolve(closeRoot, "🧪️tests/🧫️fixtures/📦️numerical-close/🔣️.json"), "utf8")) as { cases: unknown[] };
    const stiffnessCorpus = JSON.parse(readFileSync(resolve(stiffnessRoot, "🧪️tests/🧫️fixtures/🧱️mounted-stiffness/🔣️.json"), "utf8")) as { cases: { kind: string; matrix: number[] }[] };
    assert.equal(closeCorpus.cases.length, 4);
    assert.deepEqual(stiffnessCorpus.cases.map(row => [row.kind, row.matrix.length]), [["bar3", 36], ["frame3", 144], ["tet4", 144]]);
    const sources = [resolve(closeRoot, "🧪️tests/📦️numerical-close/🟦️.ts"), resolve(closeRoot, "🧪️tests/🏷️sources/🟦️.ts"), resolve(stiffnessRoot, "🧪️tests/🧱️mounted-stiffness/🟦️.ts"), resolve(stiffnessRoot, "🧪️tests/🌀️geometric-cells/🟦️.ts"), resolve(context.repoRoot, "🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎯️positive-spectrum/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🧪️tests/❄️cold/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧮️triplet-csr/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📚️paged-ldlt/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎼️pencil/🟦️.ts"), resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️restore-funding/🟦️.ts")];
    await runOwnedCommand(process.execPath, [Bun.resolveSync("typescript/bin/tsc", context.root), "--noEmit", "--strict", "--skipLibCheck", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--types", "bun", ...sources], context.repoRoot, "fem3d:numerical-source-typecheck", scriptInvocationBudget(context.invocation, 120000), { signal: context.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
    context.invocation.control.signal?.throwIfAborted();
    const { testFem3dNumericalCloseOwners } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/📦️numerical-close/🟦️.ts");
    await testFem3dNumericalCloseOwners();
    scriptInvocationBudget(context.invocation, 120000);
    context.invocation.control.signal?.throwIfAborted();
    const { testFem3dMountedStiffnessOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/📦️packages/🟦️typescript/🧱️elements3d/🧪️testing/🧱️mounted-stiffness/🟦️.ts");
    await testFem3dMountedStiffnessOracle();
    const { testFem3dMountedSourceOracle } = await import("../../🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🧪️tests/🏷️sources/🟦️.ts");
    testFem3dMountedSourceOracle();
    const { verifyGeometricCellOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🌀️geometric-cells/🟦️.ts");
    await verifyGeometricCellOracle();
    const { verifyRetainedIndexSortOracle } = await import("../../../../../../../🧰️framework/🔨️modules/🌱️value/📶️sort/🧪️tests/🟦️.ts");
    await verifyRetainedIndexSortOracle();
    const { verifyPositiveSpectrumOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎯️positive-spectrum/🟦️.ts");
    verifyPositiveSpectrumOracle();
    await runOwnedCommand(process.execPath, ["test", resolve(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🔁️pcg/🧪️tests/❄️cold/🟦️.ts")], context.repoRoot, "fem3d:numerical-mathematical-source", scriptInvocationBudget(context.invocation, 120000), { signal: context.invocation.control.signal, output: { maximumBytes: 8388608, maximumLines: 65536 } });
    const { verifyTripletCsrOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/🧮️triplet-csr/🟦️.ts");
    verifyTripletCsrOracle();
    const { verifyPagedLdltOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📚️paged-ldlt/🟦️.ts");
    verifyPagedLdltOracle();
    const { verifySymmetricPencilOracle } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🎼️pencil/🟦️.ts");
    verifySymmetricPencilOracle();
    const { testNumericalRestoreFundingWire } = await import("../../../../../../🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️restore-funding/🟦️.ts");
    testNumericalRestoreFundingWire();
    if (segments[0] === "fem3d-numerical-child-source" || segments[1] === "source") return;
    const policy = readCargoTestPolicyV1(process.env), manifest = resolve(context.root, "Cargo.toml");
    assert.equal(resolve(policy.manifestPath), manifest, "numerical laws require the original FEM3d owner manifest");
    const groups = [
      { package: "semio-s-fem-sparse-ldlt", target: { kind: "lib", name: "semio_s_fem_sparse_ldlt" }, cargoArgs: ["--locked"], laws: ["paged_ldlt_tests::paged_ldlt_crosses_original_owner_and_checkpoint_boundaries"] },
      { package: "semio-s-fem-sparse-pcg", target: { kind: "lib", name: "semio_s_fem_sparse_pcg" }, cargoArgs: ["--locked"], laws: ["tests::mathematical_pcg_and_original_native_job_share_neutral_outcomes"] },
      { package: "semio-s-fem-sparse-subspace", target: { kind: "lib", name: "semio_s_fem_sparse_subspace" }, cargoArgs: ["--locked"], laws: ["positive_spectrum_tests::generalized_subspace_targets_positive_spectrum_and_certifies_original_residual", "positive_spectrum_tests::mathematical_subspace_certifies_the_same_neutral_positive_spectrum"] },
      { package: "semio-framework-value", target: { kind: "lib", name: "semio_framework_value" }, cargoArgs: ["--locked"], laws: ["list::counter_tests::retained_fixed_list_reservation_copy_quotes_actual_original_metadata_and_heap", "list::counter_tests::retained_fixed_list_capacity_funding_preserves_original_pages_before_items", "sort::tests::retained_index_sort_matches_portable_order_and_bounds_every_action", "sort::tests::retained_index_sort_refuses_wrong_observation_without_changing_cursor"] },
      { package: "semio-s-artifact-fem-2d", target: { kind: "lib", name: "semio_s_artifact_fem_2d" }, cargoArgs: ["--locked", "--features", "component-app-assembly"], laws: ["elements3d::tests::mounted_3d_element_interfaces_match_numpy_fixture", "elements3d::tests::mounted_tetrahedral_stress_matches_numpy_energy_without_scratch_owners", "elements3d::tests::geometric_cells::mounted_geometric_cells_match_independent_quadrature_and_tensor_contraction", "analyses::tests::assembly_order_pages_admit_exact_grants_and_retire_every_original_backing", "analyses::triplet_csr_tests::original_triplet_csr_matches_portable_sparse_matrix_and_closes_every_page", "sparse::tests::symmetric_pencil::original_symmetric_pencil_matches_independent_sparse_storage_and_refuses_owner_transfer", "sparse::tests::numerical_page_ldlt_checkpoint_restores_matrix_across_original_pages", "sparse::tests::numerical_restore_funds_each_original_contributor_frontier_separately", "sparse::tests::subspace_factor_cursor_matches_numpy_for_three_nondiagonal_right_hand_sides", "sparse::tests::subspace_publication_restarts_at_zero_and_commits_convergence_after_the_last_scalar", "sparse::tests::numerical_page_subspace_restore_close_preserves_every_ungranted_backing", "sparse::tests::numerical_page_subspace_work_persists_solver_state_without_physical_retirement_cursor", "sparse::tests::long::p6h_subspace_cancellation_is_observed_at_every_nested_stage_and_worker_replay_is_exact", "analyses::tests::numerical_owner_page_admission_refuses_before_allocating_and_is_not_singularity", "app_surface::result_projection_tests::result_projection_uses_eigenvectors_and_correct_units_for_both_eigenmodes"] },
      { package: "semio-s-artifact-fem-3d", target: { kind: "lib", name: "semio_s_artifact_fem_3d" }, cargoArgs: ["--locked", "--features", "component-app-assembly"], laws: [
          "live_visual::tests::fem3d_numerical_child_classifies_every_lent_outcome_without_retaining_it",
          "live_visual::tests::fem3d_visual_maximum_plus_one_rejects_before_owner_transfer",
          "live_visual::tests::fem3d_snapshot_preflight_page_maximum_plus_one_returns_exact_producer",
          "live_visual::tests::fem3d_solver_nonzero_generation_corresponds_to_every_published_field_page",
          "live_visual::tests::fem3d_visual_stale_cancel_fault_deadline_and_interrupted_device_close_preserve_last_valid",
          "live_visual::tests::fem3d_visual_replay_accessibility_fixed_page_close_and_each_step_are_bounded",
          "live_visual::tests::fem3d_numerical_fixed_owner_maximum_plus_one_refuses_unchanged_and_closes_one_slot",
          "live_visual::tests::fem3d_numerical_child_absent_lanes_retain_later_close_owners",
          "live_visual::tests::fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized",
          "live_visual::tests::fem3d_production_numerical_child_solves_the_demo_with_paged_owner_backing",
          "live_visual::tests::fem3d_mounted_selected_sources_match_pynite_and_case_order_is_irrelevant",
          "live_visual::tests::fem3d_source_preflight_rejects_ambiguous_missing_nonfinite_and_excess_sources",
          "editor::fem3d::modes::edit::windows::results::tests::mounted_results_render_borrows_jobs_without_synchronous_solves",
          "live_visual::tests::render_revision_tests::fem3d_scalar_publication_refuses_reused_counters_from_another_document",
          "live_visual::tests::fem3d_production_field_correspondence_rejects_sparse_and_zero_aliases",
          "live_visual::tests::fem3d_process_permits_precede_solver_order_allocation_and_drop_handoff_closes_one_backing",
          "live_visual::tests::fem3d_queued_running_and_state_drop_publish_exact_identity_and_drain_one_owner",
          "live_visual::tests::fem3d_window_config_mounted_close_preserves_foreign_instance_in_same_slot",
          "live_visual::tests::fem3d_visual_close_grants_retain_exact_backing_until_admitted"
        ] },
    ] as const;
    const receipts = await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation: context.invocation, policy: configuredExactCargoLawPolicyV1(), manifestPaths: { "semio-framework-value": resolve(context.repoRoot, "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/Cargo.toml"), "semio-framework-plugin": resolve(context.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml"), "semio-s-artifact-fem-2d": resolve(context.root, "../../../◻️2d/📦️packages/🦀️rust/Cargo.toml"), "semio-s-artifact-fem-3d": manifest }, cwd: context.repoRoot, groups });
    assert.equal(receipts.length, groups.length);
    for (const [index, group] of groups.entries()) {
      assert(group.laws.length > 0, "numerical law coverage must be positive");
      assert.equal(receipts[index]!.assertions, group.laws.length);
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}

/** 🏃️ Runs the `MaterialNativeScript` command under the caller's roots and invocation. */
export async function runMaterialNativeScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args:string[]): Promise<void> {
assert(args.length===1&&["source","native"].includes(args[0]!),"test-material requires source or native");
const oracle=await import("../../../../⚙️engine/🧱️material/📋️table/🧪️tests/🟦️.ts");oracle.materialTableOracle();
if(args[0]==="native"){const p=readCargoTestPolicyV1(process.env),pkg="semio-s-fem-material";await runExactCargoLaws({control:exactCargoCallerControlV1(context.invocation,"exact-cargo:caller",0),invocation:context.invocation,policy:{...configuredExactCargoLawPolicyV1(),buildMilliseconds:scriptInvocationBudget(context.invocation,p.buildBudgetMs>0?p.buildBudgetMs:600000),lawMilliseconds:scriptInvocationBudget(context.invocation,p.assertionBudgets[p.level]>0?p.assertionBudgets[p.level]:300000)},cwd:context.repoRoot,nativeEnv:{RUST_MIN_STACK:p.rustMinStack},manifestPaths:{[pkg]:resolve(context.root,"../../../../⚙️engine/🧱️material/📦️packages/🦀️rust/Cargo.toml")},groups:[{package:pkg,target:{kind:"lib",name:"semio_s_fem_material"},cargoArgs:["--offline"],laws:["volumetric::native_material_tests::native_material_receives_canonical_and_authored_original_values","volumetric::native_material_tests::native_material_receives_actual_declared_text_ordinals","volumetric::native_material_tests::native_material_refuses_invalid_parts_without_dropping_partial_fields","volumetric::native_material_tests::table::native_material_table_receives_original_order_and_closes_cancelled_frontiers"]}]});}
 }
