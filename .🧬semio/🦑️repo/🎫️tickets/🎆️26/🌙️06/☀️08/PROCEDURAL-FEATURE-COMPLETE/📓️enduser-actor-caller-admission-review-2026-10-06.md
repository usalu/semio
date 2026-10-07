# Generation3d Actor Caller Admission Review, October 6

Parent authorized only canonical contract caller reconciliation. The two remaining actual app suite mismatches each contain exactly one constructor argument addition. Current original artifact_app_laws helper at plugin7855 explicitly requires protocol::ActorId and forwards it through original registry/member helper7861 into VcsArtifactApp::with_registry. The existing standalone fixture uses canonical local actor text and its original actions already use meta(local); supplying its named canonical ActorId is fixture opening authority, not deriving an actor from historical edits or adding a production default. There are no feature expectation, mutation payload, runtime gate or schema changes.

Current shared release readback confirms persisted hydrator from_decoded_pack retains original decoded state plus original Pack/digest/opened actor; at Finish318 it constructs ArtifactGenesis::from_verified_pack before original envelope. Config hydrator from_snapshots explicitly accepts ArtifactGenesis<P>. Original Window retained loader1254 constructs it from already verified original Pack/digest, then forwards same construction-time opened actor1261–1273 into hydrator. Owner release reports accepted; this source readback is not a native compile or runtime claim.

## ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs

Exact admitted hash `e9c01f045c373bd44d1d1a5999f7b163b447cf05ad4d8b1cbe6ba827265b4bca` recovered from authoritative5c7 blob. Current `076b79a4d5721e8699650c95dc65541b58559636d0367d1946fc84f2d0d682d9`. Reversing only the one explicit fixture ActorId constructor argument recovers every admitted byte exactly. All function names/order, every assertion/history block and complete retained roster remain unchanged.

```diff
--- admitted
+++ current
@@ -56,5 +56,5 @@
     
     pub async fn app() -> Generation3dViewerFixture {
-        let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_tests).await;
+        let mut app = new_app_with_registry::<ViewerApp<Generation3dViewer>>(generation3d_viewer_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
         app.bind_instance_id(1).await;
         Generation3dViewerFixture(app)
```

Preserved law roster:

- `viewer_domain::viewer_laws::create_generation3d_viewer_builds_a_definition_for_the_viewer_role`
- `viewer_domain::viewer_laws::viewer_dialect_matches_the_artifact_coordinate`
- `viewer_domain::viewer_laws::the_viewer_offers_export_in_both_languages_and_never_offers_import`
- `viewer_domain::viewer_laws::the_preview_window_is_bound_to_the_graph_interaction_domain`
- `viewer_domain::viewer_laws::the_interaction_topology_declares_node_handle_and_edge_targets`
- `viewer_domain::viewer_laws::the_viewer_renders_a_world3d_preview_for_the_default_document`
- `viewer_domain::viewer_laws::every_viewer_action_dispatches_live_and_never_mutates_the_document`
- `viewer_domain::viewer_laws::every_emitted_action_is_declared_on_the_preview_window_kind`

## ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs

Exact admitted hash `229d9110fffb3feefe84cff79ff3db3f59c1f89ff9cf3d6762524a0db935973a` recovered from authoritative5c7 blob plus three previously admitted IO snapshot-helper path replacements. Current `fabb3ac1ca2bbf173a73a594d3b9cf22bd153232e40d0852affbb98a93b15ba6`. Reversing only the one explicit fixture ActorId constructor argument recovers every admitted byte exactly. All function names/order, every assertion/history block and complete retained roster remain unchanged.

```diff
--- admitted
+++ current
@@ -79,5 +79,5 @@
     
     pub async fn app_with_registry() -> Generation3dAppFixture {
-        let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_tests).await;
+        let mut app = new_app_with_registry::<EditorApp<Generation3dPlayApp>>(generation3d_app_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into())).await;
         app.bind_instance_id(1).await;
         Generation3dAppFixture(app)
```

Preserved law roster:

- `editor_domain::editor_laws::preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app`
- `editor_domain::editor_laws::generation_preview_is_one_evaluation_shared_by_two_generation_windows`
- `editor_domain::editor_laws::every_command_round_trips_through_text_and_binary`
- `editor_domain::editor_laws::every_printed_op_line_starts_with_the_rows_wire_keyword`
- `editor_domain::editor_laws::the_editor_declares_every_io_action_and_chord_the_fixture_names`
- `editor_domain::editor_laws::the_editor_binds_every_keyboard_verb_the_fixture_names`
- `editor_domain::editor_laws::every_window_scoped_chord_names_its_owning_window_kind_and_the_modes_that_mount_it`
- `editor_domain::editor_laws::the_example_picker_offers_the_flow_examples_and_never_the_command_session`
- `editor_domain::editor_laws::the_export_action_offers_every_declared_format_in_both_languages`
- `editor_domain::editor_laws::declared_actions_bridge_to_commands`
- `editor_domain::editor_laws::node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one`
- `editor_domain::editor_laws::registry_backed_editor_installs_every_declared_bounded_command_proof`
- `editor_domain::editor_laws::the_manifest_stitches_every_taxonomy_node`
- `editor_domain::editor_laws::each_example_loads_distinct_fixture_and_preview_geometry`
- `editor_domain::editor_laws::refresh_pending_effects_starts_the_preview_eval_run`
- `editor_domain::editor_laws::a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias`
- `editor_domain::editor_laws::undo_redo_round_trips_flow_graph_edits`
- `editor_domain::editor_laws::two_instances_converge_disjoint_widget_moves`
- `editor_domain::editor_laws::generation3d_labels_translate_catalogue_and_inspector_in_german`
- `editor_domain::editor_laws::generation3d_interaction_selection_owns_its_persisted_history`
- `editor_domain::editor_laws::context_menu_grouped_disclosure_stays_within_budget`
- `editor_domain::editor_laws::context_menu_reads_the_framework_owned_graph_selection`
- `editor_domain::editor_laws::graph_selection_splits_into_node_and_edge_domains`
- `editor_domain::editor_laws::context_menu_groups_are_taxonomy_categories`
- `editor_domain::editor_laws::window_measure_labels_are_localized`
- `editor_domain::editor_laws::sun_measures_are_exposed_on_preview_windows`
- `editor_domain::editor_laws::preview_payload_has_meshes_and_instances`
- `editor_domain::editor_laws::document_from_mesh_rejects_empty_or_invalid_geometry`
- `editor_domain::editor_laws::mesh_import_preserves_fixture_coordinates_in_an_editable_graph`
- `editor_domain::editor_laws::generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs`
- `editor_domain::editor_laws::rectangle_wire_preview_emits_edge_only_mesh`
- `editor_domain::editor_laws::all_bundled_examples_emit_preview_meshes`
- `editor_domain::editor_laws::preview_tolerance_follows_lod_mode`
- `editor_domain::editor_laws::wireframe_show_mode_strips_shaded_triangles`
- `editor_domain::editor_laws::generation3d_io_declares_the_params_and_geometry_ports`
- `editor_domain::editor_laws::mesh_preview_renders_without_a_brep_conversion_in_editor_and_viewer`
- `editor_domain::editor_laws::preview_payload_channel_qualifies_ids_across_two_output_channels`
- `editor_domain::editor_laws::preview_payload_flattens_a_list_channel_into_indexed_instances`
- `editor_domain::editor_laws::preview_payload_emits_no_instance_for_a_pure_data_channel`
- `editor_domain::editor_laws::preview_marks_resolve_node_channel_and_instance_ids`
- `editor_domain::editor_laws::preview_payload_marks_every_channel_of_a_hovered_node`
- `editor_domain::editor_laws::preview_payload_marks_only_the_hovered_channel`
- `editor_domain::editor_laws::graph_marks_project_instance_hover_back_onto_its_node_and_port`
- `editor_domain::editor_laws::interaction_topology_ports_match_the_node_graph_port_ids`
- `editor_domain::editor_laws::widget_preview_eligibility_covers_neurons_output_previews_and_clusters`
- `editor_domain::editor_laws::examples_match_set_active_example_select_options`
- `editor_domain::editor_laws::every_window_and_panel_surface_fits_the_resident_surface_bound`
- `editor_domain::editor_laws::the_first_turn_sequence_retires_every_flow_host_it_builds`
- `editor_domain::editor_laws::hex_column_evaluates_end_to_end_through_the_extension_round_trip`
- `editor_domain::editor_laws::host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`
- `editor_domain::editor_laws::the_shell_boot_surface_burst_renders_inside_one_turn`
- `editor_domain::editor_laws::the_served_one_page_contributions_crossing_installs_inside_the_watchdog_budget`
- `editor_domain::editor_laws::the_settled_evaluation_tick_cycle_retains_nothing_that_would_exhaust_the_guest`
- `editor_domain::editor_laws::extension_invocations_address_the_contributing_plugin_and_a_missing_contribution_faults_the_preview`
- `editor_domain::editor_laws::a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
- `editor_domain::editor_laws::re_pushing_an_unchanged_closure_owes_the_settled_run_nothing`
- `editor_domain::editor_laws::hex_column_boot_stays_inside_the_interactive_turn_budget`
- `editor_domain::editor_laws::every_emitted_action_is_declared_on_its_window_kind`
- `editor_domain::editor_laws::an_evaluate_fault_outranks_the_addressing_miss_and_a_contribution_install_clears_it`
- `editor_domain::editor_laws::staged_argument_actions_declare_no_trailing_ellipsis`
- `editor_domain::editor_laws::mesh_gumball_splices_typed_transforms_and_preserves_analysis_consumers`
- `editor_domain::editor_laws::mesh_component_edits_insert_typed_widgets_and_update_downstream_analysis`
- `editor_domain::editor_laws::mesh_component_gumball_pins_the_complete_component_set`
- `editor_domain::editor_laws::mesh_component_gumball_reuses_only_the_same_selection_and_operation`
- `editor_domain::editor_laws::mesh_component_gumball_projects_a_topology_pivot_and_live_dispatch`
- `editor_domain::editor_laws::mesh_component_edit_undo_redo_restores_geometry_and_analysis_connections`
- `editor_domain::editor_laws::mesh_component_commands_reject_stale_topology_before_publication`
- `editor_domain::editor_laws::mesh_component_gumball_rejects_changed_selection_before_and_during_drag`
- `editor_domain::editor_laws::mesh_component_gumball_retains_selection_coalesces_drags_and_round_trips_history`
- `editor_domain::editor_laws::a_document_archive_loads_into_a_fresh_instance_through_the_import_door`
- `editor_domain::editor_laws::a_hub_genesis_pair_is_produced_and_parses_back_without_trapping`
- `editor_domain::editor_laws::geometry_media_export_uses_the_supplied_instance_owner_and_refuses_closed_authority`
- `editor_domain::editor_laws::mesh_quick_actions_execute_through_the_registered_preview_app`
- `editor_domain::editor_laws::mesh_brep_measurement_outputs_are_selected_read_only_in_every_locale`
- `editor_domain::editor_laws::mesh_brep_live_scoped_selection_edits_the_evaluated_solid_and_restores_history`
