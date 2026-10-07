# Exact Enduser Admission Source Changes

## ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs
Exact retained SHA baseline in read-only commit 202c4b7b5b174ff40a814834615481f60272a32a
```diff
--- retained
+++ current
@@ -32,8 +32,8 @@
         let Effect::DispatchAction { action: dispatched, args, .. } = effect else { panic!("a hop must be a dispatch effect") };
         assert_eq!(dispatched, text(action));
         let args = args.expect("an addressed hop carries args");
-        assert_eq!(args.get("windowId").and_then(dsl::DslValue::as_str), Some("preview-1"));
-        assert_eq!(args.get("windowKindId").and_then(dsl::DslValue::as_str), Some("procedural-view-preview"));
+        assert_eq!(args.get("windowId").and_then(semio_framework_value::DslValue::as_str), Some("preview-1"));
+        assert_eq!(args.get("windowKindId").and_then(semio_framework_value::DslValue::as_str), Some("procedural-view-preview"));
     }
 }
 
```

## ✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs
Exact retained SHA baseline in read-only commit 25bb77059d671749edbde5c9034f9ef78588f8d6
```diff
--- retained
+++ current
@@ -8,7 +8,7 @@
     assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_MAIN).await.contains("node-graph"));
 }
 
-/// 🛍️ The scene names its operators by KIND ID (inside `fixtureJson`) and carries only the document's
+/// 🛍️ The scene names its operators by KIND ID (inside `snapshotJson`) and carries only the document's
 /// own neuron kinds as operator records — never the registered catalogue (~100 KB, three times the
 /// fixed 32 KiB per-surface admission; ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1). Those records
 /// are how the canvas instantiates the graph instead of `FlowHostSnapshot::default()`'s placeholder slider.
```

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️fold-contract/🦀️.rs
Exact retained SHA baseline in read-only commit 5c7f51ee6430f0ca3e517b29c29043d83ffd3b3b
```diff
--- retained
+++ current
@@ -2,7 +2,8 @@
 use crate::editor::generation3d::config::SetSnapshot;
 use crate::standards::v1::subsets::any::schema::mutations::generation3d_host_snapshot_operations;
 use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
-use crate::standards::v1::subsets::any::schema::{default_snapshot, example_snapshot, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS};
+use crate::standards::v1::subsets::any::schema::{PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_BOX_SHELL, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_SPHERE_TORUS};
+use crate::standards::v1::subsets::any::io::text::snapshot::{default_snapshot, example_snapshot};
 
 const BUNDLED_EXAMPLES: [&str; 8] = [PROCEDURAL_EXAMPLE_HEX_COLUMN, PROCEDURAL_EXAMPLE_RECT_EXTRUDE, PROCEDURAL_EXAMPLE_SPHERE_TORUS, PROCEDURAL_EXAMPLE_BOX_FILLET, PROCEDURAL_EXAMPLE_SPHERE_BOX_FUSE, PROCEDURAL_EXAMPLE_FACE_SWEEP_EXTRUDE, PROCEDURAL_EXAMPLE_RECTANGLE_WIRE, PROCEDURAL_EXAMPLE_BOX_SHELL];
 
```

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️flow/🧪️tests/🔬️unit/🦀️.rs
Exact retained SHA baseline in read-only commit 202c4b7b5b174ff40a814834615481f60272a32a
```diff
--- retained
+++ current
@@ -19,7 +19,7 @@
 }
 
 #[semio_framework_async_macros::async_test]
-async fn the_node_graph_canvas_declares_the_activate_binding_the_keyboard_fixture_states() {
+async fn the_node_graph_canvas_declares_the_activate_binding_the_keyboard_snapshot_states() {
     let _serial = crate::test_serial::lock();
     let fixture: serde_json::Value = serde_json::from_str(KEYBOARD_REACHABILITY_FIXTURE_JSON).expect("keyboard fixture");
     let row = fixture["surfaceBindings"].as_array().expect("surfaceBindings").iter().find(|row| row["surface"].as_str() == Some(GENERATION_3D_PLAY_SURFACE_MAIN)).expect("the node-graph canvas has a surface-binding row");
```

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔄️rotate-selection/🧪️tests/🔬️unit/🦀️.rs
Exact retained SHA baseline in read-only commit 202c4b7b5b174ff40a814834615481f60272a32a
```diff
--- retained
+++ current
@@ -5,7 +5,7 @@
 #[test]
 fn rotate_selection_refuses_invalid_motions_and_targets_atomically() {
     let _serial = crate::test_serial::lock();
-    let snapshot = crate::standards::v1::subsets::any::schema::example_snapshot(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
+    let snapshot = crate::standards::v1::subsets::any::io::text::snapshot::example_snapshot(crate::standards::v1::subsets::any::schema::PROCEDURAL_EXAMPLE_MESH_WORKBENCH).unwrap();
     let count = snapshot.host_snapshot.widgets.len();
     let still = RotateSelection { node_ids: vec!["extrude".into()], ax: 0.0, ay: 0.0, az: 0.0, angle: 1.0, phase: None, reason: None, window_id: None };
     assert!(!still.motion().moves(), "a zero axis is no motion");
```

## ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🧭️navigate-graph/🧪️tests/🔬️unit/🦀️.rs
Exact retained SHA baseline in read-only commit 48d881aa7ab522321a135c3e5dbc469c369138ea
```diff
--- retained
+++ current
@@ -1,6 +1,7 @@
 use super::*;
 use crate::standards::v1::subsets::any::schema::snapshot::Generation3dSnapshotRead;
-use crate::standards::v1::subsets::any::schema::{example_snapshot, PROCEDURAL_EXAMPLE_HEX_COLUMN};
+use crate::standards::v1::subsets::any::schema::{PROCEDURAL_EXAMPLE_HEX_COLUMN};
+use crate::standards::v1::subsets::any::io::text::snapshot::{example_snapshot};
 
 const GRAPH_KEYBOARD_FIXTURE_JSON: &str = include_str!("../../../../../🧫️fixtures/🧭️graph-keyboard-navigation.json");
 
```

Six exact SHA baselines independently recovered from read-only Git history and compared byte-for-byte. Refreshed only their six current admission hash strings. Changes are canonical Value caller2, doc terminology1, original snapshot-helper imports/caller3, original keyboard test naming1. Every source assertion and original law roster remains unchanged. Three remaining transient baseline rows (native unit/full-editor/translate) remain untouched pending precise attribution.

Prepared original live shell law existed in full-editor source and both launch rows, but current composition law roster did not select it. Appended only mesh_brep_live_scoped_selection_edits_the_evaluated_solid_and_restores_history to that original suite, preserving every previous law name/order and pending full-editor hash. No native execution result claimed.

## Full-Editor Semantic Source Receipt Reconciliation

Parent authorized closest authoritative baseline review after bounded transient-source recovery. The retained `53ddc589b548e2f7108d766866b1db7425c8f47756e45cfac7348e78307f89b5` blob could not be recovered from four repository path revisions or ticket source-check captures (the latter are process logs, not source snapshots). No exact reversal to that transient hash is claimed.

Read-only repository blob `5c7f51ee643` is the closest authoritative baseline. Applying exactly one rectangle-wire constant path replacement and two `ensure_gumball_node` path replacements from `schema` to existing `io::text::snapshot` reproduces every current byte. Therefore every original assertion, history block, fixture/context/driver, guard and all declared laws already occur identically in that baseline. The prepared live selected-solid law is also already present in that baseline; its existing original source remained untouched.

```diff
--- 5c7f51ee643
+++ current
@@ -1431,5 +1431,5 @@
 fn rectangle_wire_preview_emits_edge_only_mesh() {
     let _serial = test_serial();
-    let projection = <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::snapshot::text::GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
+    let projection = <Generation3dSnapshot as store::ArtifactDsl>::parse_dsl(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::text::snapshot::GENERATION3D_EXAMPLE_RECTANGLE_WIRE_TEXT).expect("rectangle wire example");
     let config = Generation3dConfig::default();
     let (meshes_json, instances_json) = preview_payload_from_evaluated_fixture(&projection.host_snapshot, &config);
@@ -2701,10 +2701,10 @@
         semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::with_host(&snapshot.host_snapshot, |host| {
             let operation = case["operation"].as_str().unwrap();
-            let id = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::ensure_gumball_node(host, "extrude@meshOut#0", operation).unwrap();
+            let id = semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::text::snapshot::ensure_gumball_node(host, "extrude@meshOut#0", operation).unwrap();
             let transform = host.host_snapshot.widgets.iter().find(|widget| semio_s_artifact_procedural_generation3d::widget_id(widget) == id).unwrap();
             assert!(matches!(transform, semio_framework_artifact_flow_flow::Widget::Neuron { neuron_kind, preview: true, .. } if neuron_kind == case["operator"].as_str().unwrap()));
             assert!(host.host_snapshot.synapses.iter().any(|wire| wire.from == "extrude" && wire.from_port == "meshOut" && wire.to == id && wire.to_port == case["input"].as_str().unwrap()));
             assert!(host.host_snapshot.synapses.iter().any(|wire| wire.from == id && wire.from_port == case["output"].as_str().unwrap() && wire.to == "analysis"));
-            assert_eq!(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::ensure_gumball_node(host, &id, operation).unwrap(), id);
+            assert_eq!(semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::io::text::snapshot::ensure_gumball_node(host, &id, operation).unwrap(), id);
         });
         snapshot.retire_cold();
```

Preserved original 74 law names in their unchanged order, followed only by the existing prepared live-shell law previously omitted from this suite:

1. `editor_domain::editor_laws::preview_eval_exact_window_transient_isolates_and_resets_in_the_registered_app`
2. `editor_domain::editor_laws::generation_preview_is_one_evaluation_shared_by_two_generation_windows`
3. `editor_domain::editor_laws::every_command_round_trips_through_text_and_binary`
4. `editor_domain::editor_laws::every_printed_op_line_starts_with_the_rows_wire_keyword`
5. `editor_domain::editor_laws::the_editor_declares_every_io_action_and_chord_the_fixture_names`
6. `editor_domain::editor_laws::the_editor_binds_every_keyboard_verb_the_fixture_names`
7. `editor_domain::editor_laws::every_window_scoped_chord_names_its_owning_window_kind_and_the_modes_that_mount_it`
8. `editor_domain::editor_laws::the_example_picker_offers_the_flow_examples_and_never_the_command_session`
9. `editor_domain::editor_laws::the_export_action_offers_every_declared_format_in_both_languages`
10. `editor_domain::editor_laws::declared_actions_bridge_to_commands`
11. `editor_domain::editor_laws::node_graph_viewport_decodes_an_absent_viewport_as_identity_and_refuses_a_malformed_one`
12. `editor_domain::editor_laws::registry_backed_editor_installs_every_declared_bounded_command_proof`
13. `editor_domain::editor_laws::the_manifest_stitches_every_taxonomy_node`
14. `editor_domain::editor_laws::each_example_loads_distinct_fixture_and_preview_geometry`
15. `editor_domain::editor_laws::refresh_pending_effects_starts_the_preview_eval_run`
16. `editor_domain::editor_laws::a_replacing_preview_run_start_retires_the_previous_runs_last_snapshot_alias`
17. `editor_domain::editor_laws::undo_redo_round_trips_flow_graph_edits`
18. `editor_domain::editor_laws::two_instances_converge_disjoint_widget_moves`
19. `editor_domain::editor_laws::generation3d_labels_translate_catalogue_and_inspector_in_german`
20. `editor_domain::editor_laws::generation3d_interaction_selection_owns_its_persisted_history`
21. `editor_domain::editor_laws::context_menu_grouped_disclosure_stays_within_budget`
22. `editor_domain::editor_laws::context_menu_reads_the_framework_owned_graph_selection`
23. `editor_domain::editor_laws::graph_selection_splits_into_node_and_edge_domains`
24. `editor_domain::editor_laws::context_menu_groups_are_taxonomy_categories`
25. `editor_domain::editor_laws::window_measure_labels_are_localized`
26. `editor_domain::editor_laws::sun_measures_are_exposed_on_preview_windows`
27. `editor_domain::editor_laws::preview_payload_has_meshes_and_instances`
28. `editor_domain::editor_laws::document_from_mesh_rejects_empty_or_invalid_geometry`
29. `editor_domain::editor_laws::mesh_import_preserves_fixture_coordinates_in_an_editable_graph`
30. `editor_domain::editor_laws::generation3d_mesh_bridges_round_trip_through_obj_glb_stl_codecs`
31. `editor_domain::editor_laws::rectangle_wire_preview_emits_edge_only_mesh`
32. `editor_domain::editor_laws::all_bundled_examples_emit_preview_meshes`
33. `editor_domain::editor_laws::preview_tolerance_follows_lod_mode`
34. `editor_domain::editor_laws::wireframe_show_mode_strips_shaded_triangles`
35. `editor_domain::editor_laws::generation3d_io_declares_the_params_and_geometry_ports`
36. `editor_domain::editor_laws::mesh_preview_renders_without_a_brep_conversion_in_editor_and_viewer`
37. `editor_domain::editor_laws::preview_payload_channel_qualifies_ids_across_two_output_channels`
38. `editor_domain::editor_laws::preview_payload_flattens_a_list_channel_into_indexed_instances`
39. `editor_domain::editor_laws::preview_payload_emits_no_instance_for_a_pure_data_channel`
40. `editor_domain::editor_laws::preview_marks_resolve_node_channel_and_instance_ids`
41. `editor_domain::editor_laws::preview_payload_marks_every_channel_of_a_hovered_node`
42. `editor_domain::editor_laws::preview_payload_marks_only_the_hovered_channel`
43. `editor_domain::editor_laws::graph_marks_project_instance_hover_back_onto_its_node_and_port`
44. `editor_domain::editor_laws::interaction_topology_ports_match_the_node_graph_port_ids`
45. `editor_domain::editor_laws::widget_preview_eligibility_covers_neurons_output_previews_and_clusters`
46. `editor_domain::editor_laws::examples_match_set_active_example_select_options`
47. `editor_domain::editor_laws::every_window_and_panel_surface_fits_the_resident_surface_bound`
48. `editor_domain::editor_laws::the_first_turn_sequence_retires_every_flow_host_it_builds`
49. `editor_domain::editor_laws::hex_column_evaluates_end_to_end_through_the_extension_round_trip`
50. `editor_domain::editor_laws::host_pushed_contribution_pages_install_the_registry_the_served_chain_needs`
51. `editor_domain::editor_laws::the_shell_boot_surface_burst_renders_inside_one_turn`
52. `editor_domain::editor_laws::the_served_one_page_contributions_crossing_installs_inside_the_watchdog_budget`
53. `editor_domain::editor_laws::the_settled_evaluation_tick_cycle_retains_nothing_that_would_exhaust_the_guest`
54. `editor_domain::editor_laws::extension_invocations_address_the_contributing_plugin_and_a_missing_contribution_faults_the_preview`
55. `editor_domain::editor_laws::a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted`
56. `editor_domain::editor_laws::re_pushing_an_unchanged_closure_owes_the_settled_run_nothing`
57. `editor_domain::editor_laws::hex_column_boot_stays_inside_the_interactive_turn_budget`
58. `editor_domain::editor_laws::every_emitted_action_is_declared_on_its_window_kind`
59. `editor_domain::editor_laws::an_evaluate_fault_outranks_the_addressing_miss_and_a_contribution_install_clears_it`
60. `editor_domain::editor_laws::staged_argument_actions_declare_no_trailing_ellipsis`
61. `editor_domain::editor_laws::mesh_gumball_splices_typed_transforms_and_preserves_analysis_consumers`
62. `editor_domain::editor_laws::mesh_component_edits_insert_typed_widgets_and_update_downstream_analysis`
63. `editor_domain::editor_laws::mesh_component_gumball_pins_the_complete_component_set`
64. `editor_domain::editor_laws::mesh_component_gumball_reuses_only_the_same_selection_and_operation`
65. `editor_domain::editor_laws::mesh_component_gumball_projects_a_topology_pivot_and_live_dispatch`
66. `editor_domain::editor_laws::mesh_component_edit_undo_redo_restores_geometry_and_analysis_connections`
67. `editor_domain::editor_laws::mesh_component_commands_reject_stale_topology_before_publication`
68. `editor_domain::editor_laws::mesh_component_gumball_rejects_changed_selection_before_and_during_drag`
69. `editor_domain::editor_laws::mesh_component_gumball_retains_selection_coalesces_drags_and_round_trips_history`
70. `editor_domain::editor_laws::a_document_archive_loads_into_a_fresh_instance_through_the_import_door`
71. `editor_domain::editor_laws::a_hub_genesis_pair_is_produced_and_parses_back_without_trapping`
72. `editor_domain::editor_laws::geometry_media_export_uses_the_supplied_instance_owner_and_refuses_closed_authority`
73. `editor_domain::editor_laws::mesh_quick_actions_execute_through_the_registered_preview_app`
74. `editor_domain::editor_laws::mesh_brep_measurement_outputs_are_selected_read_only_in_every_locale`
75. `editor_domain::editor_laws::mesh_brep_live_scoped_selection_edits_the_evaluated_solid_and_restores_history`

The single source receipt SHA is reconciled from `53ddc589b548e2f7108d766866b1db7425c8f47756e45cfac7348e78307f89b5` to `229d9110fffb3feefe84cff79ff3db3f59c1f89ff9cf3d6762524a0db935973a`; no feature expectations or source bytes are changed. The original runtime gate remains required.

## Translate-Selection Semantic Source Receipt Reconciliation

The retained `352b5686c3dec5f5b543bda25fba1ab654dfa0e085f2e0ca7917ae1abdd26f5e` transient blob could not be recovered from the three recorded path revisions. No exact reversal to that hash is claimed. Current source is byte-identical to authoritative `5c7f51ee643`. Compared against prior authoritative `202c4b7b5b1`, removing exactly the appended 74-line `streamed` helper plus two gumball laws and reversing the original canonical Value/Locale/Terminology paths reproduces every old source byte. Existing history transaction counts, exact translated labels, operation vectors, abort and undo/redo assertions are preserved unchanged.

Original source law names preserved:

- `translate_selection_persists_transform_into_flow_graph`
- `a_gumball_drag_is_one_transaction_of_the_relative_leaf`
- `a_streamed_gumball_drag_is_one_edit_and_an_abort_is_zero_trace`

Additional source laws preserved:

- `an_open_gesture_previews_exactly_what_its_release_commits`
- `the_world3d_gumball_live_protocol_lands_as_its_guest_edits`

The 57-suite law roster remains unchanged; its selected names are `editor_generation3d_commands_translate_selection_tests_laws::laws::translate_selection_persists_transform_into_flow_graph`. Source-only receipt reconciled to `9b72b4b89db203bd5de720b5e63d94495e1d40921e8f5ef7a3c9068e6cf1f629`.

```diff
--- 202c4b7b5b1
+++ current
@@ -38,5 +38,5 @@
 async fn tick(app: &mut context::Generation3dApp, args: serde_json::Value) {
     let action_meta = semio_framework_plugin::artifact_app_laws::meta("local");
-    let args: dsl::DslValue = args.into();
+    let args: semio_framework_value::DslValue = args.into();
     app.handle_action("translateSelection", Some(&args), &action_meta).await.expect("translateSelection admitted");
     context::settle(app).await;
@@ -60,8 +60,8 @@
     assert_ne!(refs[0].id, refs[1].id, "two drags are two transactions");
     assert!(rows[0].op_lines.last().is_some_and(|line| line.starts_with("drag-transforms")), "the first drag ends on its relative leaf: {:?}", rows[0].op_lines);
-    assert!(rows[0].label.resolve(protocol::Terminology::Native, protocol::Locale::En).starts_with("Drag 1 shape(s) by (1, 2, 3)"), "the declared intent leaf labels a first grab, never its splice (design §19.1)");
+    assert!(rows[0].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En).starts_with("Drag 1 shape(s) by (1, 2, 3)"), "the declared intent leaf labels a first grab, never its splice (design §19.1)");
     assert!(rows[1].op_lines.iter().all(|line| line.starts_with("drag-transforms")), "a re-grab is the relative leaf alone: {:?}", rows[1].op_lines);
-    assert_eq!(rows[1].label.resolve(protocol::Terminology::Native, protocol::Locale::En), "Drag 1 shape(s) by (0.5, 0, 0)");
-    assert_eq!(rows[1].label.resolve(protocol::Terminology::Native, protocol::Locale::De), "1 Form(en) um (0,5; 0; 0) ziehen");
+    assert_eq!(rows[1].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::En), "Drag 1 shape(s) by (0.5, 0, 0)");
+    assert_eq!(rows[1].label.resolve(semio_framework_ui_locale::Terminology::Native, semio_framework_ui_locale::Locale::De), "1 Form(en) um (0,5; 0; 0) ziehen");
     assert_eq!(crate::gumball_param_vector(&context::snapshot(&app).host_snapshot, "extrude__gumball_translate", "offset", [0.0; 3]), [1.5, 2.0, 3.0]);
 }
@@ -92,3 +92,77 @@
     assert_eq!(rows_after - rows_before, 1, "a tick or an abort logs no session row of its own either");
 }
+/// 🎚️ One streamed (or released) gumball tick of `extrude` in window `preview-1`, as the live `World3dHost` sends it.
+fn streamed(phase: &str, offset: [f64; 3]) -> semio_s_artifact_procedural_generation3d::editor::generation3d::transform_commands::GumballDispatch<'static> {
+    use semio_s_artifact_procedural_generation3d::editor::generation3d::transform_commands::{GesturePhase, GumballDispatch, GumballMotion};
+    let phase = match phase { "abort" => GesturePhase::parse(Some("abort"), Some("blur")), other => GesturePhase::parse(Some(other), None) }.expect("a host phase");
+    GumballDispatch { verb: "translateSelection", window: "preview-1", ids: vec!["extrude".into()], motion: GumballMotion::Translate(offset), phase, authoring_seed: "seed", base_revision: [0; 32] }
+}
+
+/// ⚖️ LAW (live consumer): while the first grab of a shape streams, the previews paint exactly what its release commits —
+/// the overlay splices the transform operator (with its identity input, design §19.4) in place of the shape with the NET
+/// offset, the marks follow the selection onto that operator, the committed snapshot stays untouched — and a host abort
+/// leaves nothing to paint.
+#[test]
+fn an_open_gesture_previews_exactly_what_its_release_commits() {
+    use semio_s_artifact_procedural_generation3d::editor::generation3d::{generation3d_gumball_preview, transform_commands::GumballGestures, PreviewInteractionMarks};
+    use semio_s_artifact_procedural_generation3d::standards::v1::subsets::any::schema::{example_snapshot, mutations::apply_generation3d_mutation, PROCEDURAL_EXAMPLE_HEX_COLUMN};
+    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
+    let committed = example_snapshot(PROCEDURAL_EXAMPLE_HEX_COLUMN).expect("the hexagonal column example");
+    let operator = "extrude__gumball_translate";
+    let marks = PreviewInteractionMarks { selected: ["extrude".to_string()].into(), ..Default::default() };
+    let mut gestures = GumballGestures::default();
+    for offset in [[1.0, 0.0, 0.0], [0.5, 2.0, 0.0]] {
+        assert!(gestures.dispatch(streamed("stream", offset), &committed.host_snapshot).expect("a tick streams").artifact_mutations.is_empty(), "a tick is provisional");
+    }
+    let (overlay, following) = generation3d_gumball_preview(&committed, &marks, &gestures).expect("an open gesture paints");
+    let previews = |snapshot: &semio_s_artifact_procedural_generation3d::Generation3dSnapshot, id: &str| snapshot.host_snapshot.widgets.iter().find(|widget| widget_id(widget) == id).map(|widget| matches!(widget, Widget::Neuron { preview: true, .. }));
+    assert_eq!((previews(&overlay, operator), previews(&overlay, "extrude")), (Some(true), Some(false)), "the operator is painted in place of the shape");
+    assert_eq!(previews(&committed, operator), None, "the committed snapshot never sees the open gesture");
+    assert_eq!(following.selected, [operator.to_string()].into(), "the marks follow the selection onto the operator");
+    assert_eq!(crate::gumball_param_vector(&overlay.host_snapshot, operator, "offset", [f64::NAN; 3]), [1.5, 2.0, 0.0], "the preview composes the net offset from the identity");
+    let released = gestures.dispatch(streamed("commit", [0.0; 3]), &committed.host_snapshot).expect("the release commits");
+    assert!(released.transaction.is_some(), "the release is ONE tool transaction");
+    let mut landed = committed.clone();
+    for row in released.artifact_mutations {
+        apply_generation3d_mutation(&mut landed, &row).expect("the committed rows apply");
+        row.retire_cold();
+    }
+    assert_eq!(landed, overlay, "the preview painted exactly what the release committed");
+    assert!(generation3d_gumball_preview(&committed, &marks, &gestures).is_none(), "a released gesture paints nothing more");
+    gestures.dispatch(streamed("stream", [1.0, 0.0, 0.0]), &committed.host_snapshot).expect("a second gesture opens");
+    gestures.dispatch(streamed("abort", [0.0; 3]), &committed.host_snapshot).expect("the host aborts");
+    assert!(generation3d_gumball_preview(&committed, &marks, &gestures).is_none(), "an aborted gesture leaves nothing to paint");
+    for snapshot in [landed, overlay, committed] {
+        snapshot.retire_cold();
+    }
+}
+/// 🛠️ LAW — the World3d live-consumer API end to end (`🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json`, audit S2):
+/// every verb dispatch the host owes for a scripted gesture, sent with its wire args exactly as the host sends them (the
+/// pinned targets swapped for generation3d's `extrude` shape), publishes exactly the fixture's guest edits and moves the shape
+/// by its offset — a streamed gesture is ONE edit, an aborted one leaves zero trace, one that never moved publishes nothing.
+#[semio_framework_async_macros::async_test]
+async fn the_world3d_gumball_live_protocol_lands_as_its_guest_edits() {
+    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
+    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧫️fixtures/🛠️gumball-live-protocol.json"))).expect("the protocol fixture parses");
+    let offset_of = |app: &context::Generation3dApp| crate::gumball_param_vector(&context::snapshot(app).host_snapshot, "extrude__gumball_translate", "offset", [0.0; 3]);
+    let mut consumed = 0;
+    for case in fixture["cases"].as_array().expect("cases").iter().filter(|case| case.get("guest").is_some()) {
+        let name = case["name"].as_str().expect("name");
+        let mut app = app().await;
+        let before = edit_rows(&mut app).await.len();
+        for dispatch in case["steps"].as_array().expect("steps").iter().map(|step| &step["dispatch"]).filter(|dispatch| !dispatch.is_null()) {
+            let mut args = dispatch["args"].clone();
+            args["ids"] = serde_json::json!(["extrude"]);
+            let args: semio_framework_value::DslValue = args.into();
+            let action = dispatch["action"].as_str().expect("action");
+            app.handle_action(action, Some(&args), &semio_framework_plugin::artifact_app_laws::meta("local")).await.unwrap_or_else(|fault| panic!("{name}: {action} is admitted from the host's wire args: {fault:?}"));
+            context::settle(&mut app).await;
+        }
+        let offset: Vec<f64> = case["guest"]["offset"].as_array().expect("offset").iter().map(|value| value.as_f64().expect("number")).collect();
+        assert_eq!((edit_rows(&mut app).await.len() - before) as u64, case["guest"]["edits"].as_u64().expect("edits"), "{name}: the published edits");
+        assert_eq!(offset_of(&app), [offset[0], offset[1], offset[2]], "{name}: the shape's offset");
+        consumed += 1;
+    }
+    assert!(consumed >= 8, "every guest case of the protocol fixture is consumed: {consumed}");
+}
 //#endregion 🛠️GumballTool
```

## Native Editor Semantic Source Receipt Reconciliation

Document owner supplied [the complete baseline/current review](📓️document-native-unit-source-admission-review-2026-10-06.md). Independent exact in-memory normalization of authoritative `5c7f51ee643` (`9407f54861682ce90c5a6000c11e49e54324860f37cd8a1fd241dd7567e7c9e1`) reproduces every current byte using only those recorded current IO/host helper path moves and matching documentation path. Every original function, assertion/literal/loop/retirement, scoped field and history acceptance macro remains intact; no added or removed function. A broad lexical scan retains all 23 function/method declarations in identical order. Surface strengthened TXT, typed-refusal, incoming-group and source-lease laws are already present in this authoritative baseline and remain untouched.

Retained `ce14732b5dd8166aea3f0d88150ca2ef8e319157d745a7b271a10d2160528444` transient blob remains unrecovered; no exact old-hash reversal is claimed. Parent authorization applies to this explicit semantic review. The original retained law roster remains byte-for-byte unchanged:

- `editor::generation3d::component::unit_tests::command_ids_are_unique_and_cover_every_row`
- `editor::generation3d::component::unit_tests::contributions_route_declares_a_reachable_wire_ceiling`
- `editor::generation3d::component::unit_tests::document_io_route_declares_a_reachable_wire_ceiling`
- `editor::generation3d::component::unit_tests::mesh_component_action_is_scoped_and_publishes_history_and_selection`
- `editor::generation3d::component::unit_tests::no_pointer_down_route_survives_the_framework_owned_selection_domain`
- `editor::generation3d::component::unit_tests::retained_route_dispositions_are_exact_and_exhaustive`
- `editor::generation3d::component::unit_tests::tessellate_transfer_unit_fits_the_declared_response_wire_bound`
- `editor::generation3d::component::unit_tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed`

Only this source receipt SHA is reconciled to `55fefa69fc3b49222c4ab57bc34abea321d5fad890ee46c0c61690e53ba057af`; pending native behavior is not accepted by a SHA receipt.

## Native Editor Import Law Receipt, October 6

Document author retained exact55fefa baseline47154 bytes before appending two neutral strict-import/value-oracle laws. Independent current readbackbc33e33560d5cec23e446c4d66e655917442351420dcf999173fe985e0446ac1 differs from delivered49dede by one externally authored original mounted-VCS fixture key initialSnapshot→initialPack. Document reviewed the current canonical VCS FromValue1462 requires initialPack and ToValue1488 emits it; same encoded mounted Pack travels through canonical Genesis codec. This changes an obsolete caller key, with no assertion or history behavior change. Independent reversal removes only appended two-law suffix and reverses only that key; full old55fefa bytes reproduce exactly.

Reconciled only this retainedArtifactUnits source SHA and appended the two authored law names. Every old byte apart from that canonical key, every old assertion/history block and old law name is preserved. Fixture remains17 retained sources and now63 composition suites due concurrent owner appends; all prior57 were preserved. Both launch sources register original source-check target with fresh three Nx flags adjacent to widget verifier. Current guard runtime pending; native import law expectations remain authored/runtime pending behind external core prerequisite.

## Final Pending Gate Handoff

Original metadata11 remains last actual metadata outcome: PREASSERT exit1/Nx4m50, Store3 missing explicit ActorId constructor callers. No metadata12 launched. Current external Actor/window changes are coherent per owners but shared immutable Genesis floor still blocks root World/Document native gates; parent will resume High metadata and selected-shell lane after that release. Actual metadata failure payload, finalized operation relay result and native live selected shell consumer/geometry/history expectations remain unproven. No ContributionRelay diagnosis was inferred.

New original composition source-check50718 terminal PREASSERT exit1/Nx10.2: unrelated generation2d-example-export source changed from retainedf4262f925d713ade7b13a36bf2f727d4eb135dbd955043b11b0a4668be2c9f7a to currentf3ece8e14cf9875752e4832e1ff2e43b7c442f0c3795e597cd26b7170be2538a. No unrelated source receipt refreshed.

Final read-only targeted Generation3d admission hash check: 74 rows; mismatches2. This is a source hash readback, not a runtime gate receipt.

- 🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs
  retained `e9c01f045c373bd44d1d1a5999f7b163b447cf05ad4d8b1cbe6ba827265b4bca`; current `076b79a4d5721e8699650c95dc65541b58559636d0367d1946fc84f2d0d682d9`.

- 🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs
  retained `229d9110fffb3feefe84cff79ff3db3f59c1f89ff9cf3d6762524a0db935973a`; current `fabb3ac1ca2bbf173a73a594d3b9cf22bd153232e40d0852affbb98a93b15ba6`.
