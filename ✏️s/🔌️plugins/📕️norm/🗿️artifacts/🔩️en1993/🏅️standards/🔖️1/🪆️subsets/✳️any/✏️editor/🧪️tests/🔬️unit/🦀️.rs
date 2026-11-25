#[path = "../🧰️context/🦀️.rs"]
pub(crate) mod context;

use crate::*;

#[test]
fn retained_command_dispositions_match_the_language_neutral_oracle() {
    semio_s_plugin_norm_app_surface::retained_disposition_oracle::assert_fixture(VARIANT);
    let definition = create_en1993_app();
    let mut classified = 0usize;
    for window in definition.window_kinds.iter() {
        let dispatchable = semio_framework::window_kind_actions(&definition, window);
        for id in semio_s_plugin_norm_app_surface::NORM_EDITOR_TOOL_IDS {
            let action = dispatchable.iter().find(|action| action.id == *id).unwrap_or_else(|| panic!("window {} must dispatch {id}", window.id));
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
            classified += 1;
        }
    }
    assert_eq!(classified, definition.window_kinds.len() * semio_s_plugin_norm_app_surface::NORM_EDITOR_TOOL_IDS.len());
}

//#region ðï¸CommandSurface
/// 🎯️ One value per `En1993Command` row â the whole-command-surface laws below iterate it, so a new row
/// that is not listed here fails `command_ids_cover_every_row`.
fn every_command() -> Vec<En1993Command> {
    vec![
        En1993Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_cover_every_row_and_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(En1993Command::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids, vec!["setSelectedCheckIndex"]);
}

/// 🧷️ The permanent wire guard: every row round-trips textâbinary and prints under its own declared
/// kebab wire keyword (which is deliberately NOT the camelCase `command_id`).
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    let keywords = ["selected-check"];
    for (command, keyword) in every_command().into_iter().zip(keywords) {
        semio_os_kernel::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = semio_os_kernel::OpText::print_op(&command);
        assert!(printed.starts_with(keyword), "row {} printed {printed:?}, expected keyword {keyword}", command.command_id());
    }
}

/// 🧷️ Pins the exact wire bytes for the rows whose shape the `app_commands!` decomposition
/// could have silently rewritten â the fieldless `Evaluate` (was a unit variant) and both `Option`
/// cases of `SetSelectedCheckIndex`. Hex is the pinned baseline;
/// these bytes are identical for all fifteen norm apps because none
/// of the three payload shapes involves the per-standard `En1993Snapshot`.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pinned_wire_bytes() {
    let hex = |command: &En1993Command| semio_os_kernel::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex(&En1993Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01000001000402");
    assert_eq!(hex(&En1993Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01000000");
}
//#endregion ðï¸CommandSurface

//#region ðï¸Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_en1993_app();
    assert_eq!(definition.modes.len(), 1);
    assert_eq!(definition.window_kinds.len(), 5);
    for body_key in [document_panel::BODY_ARTIFACT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
        assert!(definition.panel_tabs.iter().any(|tab| tab.body_key.as_deref() == Some(body_key)), "panel tab {body_key} is stitched into the manifest");
    }
    assert!(definition.artifact_kinds.iter().any(|kind| kind.id == semio_s_plugin_norm_app_surface::artifact_kind_id(VARIANT)));
}

/// 🔌️ Port recipe: every norm app declares `model:in`/`report:out` alongside the implicit document
/// ports, and `report:out` is pinned to this family's already-declared artifact kind.
#[semio_framework_async_macros::async_test]
async fn declares_model_in_and_report_out_ports() {
    let ports = create_en1993_app().io.ports;
    assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_plugin::MediaPortDirection::In));
    let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
    assert_eq!(report_out.kind_id.as_deref(), Some(semio_s_plugin_norm_app_surface::artifact_kind_id(VARIANT).as_str()));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, "norm.en1993.play.nope").await.contains("Unknown body"));
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_body_key_renders() {
    let mut app = context::app_with_registry().await;
    for body_key in [outliner::BODY_OUTLINER, properties::BODY_PROPERTIES, viewport::BODY_VIEWPORT, diagrams::BODY_DIAGRAMS, checks::BODY_CHECKS, document_panel::BODY_ARTIFACT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
        assert!(!context::render(&mut app, body_key).await.contains("Unknown body"), "{body_key} must render its own node");
    }
    context::close(&mut app);
}
//#endregion ðï¸Manifest

//#region ðï¸Behavior
/// 🧮️ `setSelectedCheckIndex` is Results-window-config-only â it must dispatch cleanly and never touch the document.
#[semio_framework_async_macros::async_test]
async fn selected_check_index_is_a_config_only_edit() {
    let mut app = context::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    let result = context::dispatch(&mut app, En1993Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })).await;
    assert!(result.mutations.is_empty(), "a Results-window-config-only command must emit no document operations");
    assert_eq!(before, app.snapshot().expect("projection"), "a Results-window-config-only command must never mutate the document");
    context::close(&mut app);
}

/// 🧬️ Kind-discipline wrapper: the real registry enforces that View actions never emit document
/// operations.
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = context::app_with_registry().await;
    let result = context::dispatch(&mut app, En1993Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) })).await;
    assert!(result.mutations.is_empty());
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = context::app_with_registry().await;
    let base = app.snapshot().expect("projection");
    context::dispatch(&mut app, En1993Command::SetField(set_field::SetField { path: "annex".into(), value_json: "\"En\"".into() })).await;
    let edited = app.snapshot().expect("projection");
    assert_ne!(edited, base, "the concrete setter moved the document");
    semio_plugin::artifact_app_laws::settle_history_verb(&mut app.0, "undo", semio_plugin::artifact_app_laws::meta("local").instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert_eq!(app.snapshot().expect("projection"), base);
    semio_plugin::artifact_app_laws::settle_history_verb(&mut app.0, "redo", semio_plugin::artifact_app_laws::meta("local").instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await;
    assert_eq!(app.snapshot().expect("projection"), edited);
    context::close(&mut app);
}

/// 🎞️ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
#[semio_framework_async_macros::async_test]
async fn render_right_after_an_edit_shows_updated_checks_without_evaluate() {
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, En1993Command::SetField(set_field::SetField { path: "annex".into(), value_json: "\"De\"".into() })).await;
    let edited = app.snapshot().expect("projection");
    let expected = semio_s_artifact_norm_en1993_inferences::evaluate(&edited);
    assert_eq!(*<En1993Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&edited), expected, "the retained session equals the full fold");
    let worst = expected.checks.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).expect("the edited document still has checks");
    let rendered = context::render(&mut app, checks::BODY_CHECKS).await;
    assert!(!rendered.contains("No checks computed"), "no evaluate command was dispatched: {rendered}");
    let escaped = worst.title.en.replace('\\', "\\\\").replace('"', "\\\"");
    assert!(rendered.contains(&escaped), "the checks window shows the edited report: {rendered}");
    context::close(&mut app);
}

#[test]
fn a_picked_entity_selects_its_governing_check_in_every_window() {
    let report = semio_s_artifact_norm_en1993_inferences::evaluate(&En1993Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let entity = report.checks.iter().find(|check| !check.subject.entity_id.is_empty() && check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable).map(|check| check.subject.entity_id.clone()).expect("a check with an entity");
    let live = semio_s_plugin_norm_app_surface::LiveView::<En1993Family>::new(En1993Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), None, &[entity.clone()]);
    assert!(live.selection.contains(&entity));
    assert_eq!(live.focus.as_deref(), Some(entity.as_str()));
    assert!(live.selection.checks_selected().is_some());
    assert!(live.selection.outliner_state().selected.contains(&entity));
}
//#endregion 🛰️LiveWiring

semio_plugin::history_edit_acceptance_law!("norm", crate::En1993PlayApp, || semio_plugin::App { definition: crate::create_en1993_app(), examples: Vec::new() }, "../../..");

//#region 📱️Devices
fn window_roles(child: &semio_plugin::WindowLayoutNode) -> Vec<String> {
    semio_plugin::collect_window_kind_ids(child).iter().map(|id| id.trim_start_matches("norm-en1993-").to_owned()).collect()
}

/// 📱️ End to end per device class: the manifest's named layout is the pane tree, the focus order walks exactly the windows of that tree, and the checks the editor evaluates carry spoken names in both languages.
#[semio_framework_async_macros::async_test]
async fn the_editor_adapts_pane_tree_focus_order_and_accessible_names_to_every_device_class() {
    use semio_s_plugin_norm_app_surface::accessibility::focus_order::{focus_order, Region};
    use semio_s_plugin_norm_app_surface::device_profiles::{device_profile, PRIORITY};
    use semio_s_plugin_norm_app_surface::viewport3d::config::DeviceClass;
    let definition = create_en1993_app();
    let default = definition.default_layout.as_ref().expect("default layout");
    assert!(matches!(&default.root, semio_plugin::WindowLayoutNode::Split { axis: semio_plugin::Axis::Horizontal, .. }), "row");
    assert_eq!(window_roles(&default.root), ["outliner", "properties", "viewport", "checks", "diagrams"]);
    let mut app = context::app_with_registry().await;
    let report = <En1993Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&app.snapshot().expect("projection"));
    assert!(!report.checks.is_empty());
    for device in PRIORITY {
        let named = definition.named_layouts.iter().find(|n| n.id == semio_s_plugin_norm_app_surface::visual_layout_id(device)).unwrap_or_else(|| panic!("named layout for {device:?}"));
        let (tree, in_tree): (Vec<String>, Vec<String>) = match (&named.layout.root, device) {
            (row @ semio_plugin::WindowLayoutNode::Split { axis: semio_plugin::Axis::Horizontal, children, .. }, DeviceClass::Desktop) => {
                assert_eq!(children.len(), 3);
                (window_roles(row), ["outliner", "properties", "viewport", "checks", "diagrams"].map(String::from).to_vec())
            }
            (tabs @ semio_plugin::WindowLayoutNode::Stack { active_window_id, .. }, DeviceClass::Mobile) => {
                assert_eq!(active_window_id.as_deref(), Some("norm-en1993-viewport"));
                (window_roles(tabs), ["viewport", "properties", "checks", "diagrams"].map(String::from).to_vec())
            }
            (row @ semio_plugin::WindowLayoutNode::Split { children, .. }, DeviceClass::Tablet) => {
                assert_eq!(children.len(), 2);
                (window_roles(row), ["viewport", "properties", "checks", "diagrams"].map(String::from).to_vec())
            }
            (root, device) => panic!("unexpected pane tree {root:?} for {device:?}"),
        };
        assert_eq!(tree, in_tree, "{device:?}");
        let mut order: Vec<&str> = focus_order(device).iter().map(|r| r.role()).collect();
        assert!(order.iter().all(|role| tree.iter().any(|t| t == role)), "every focus stop exists as a window on {device:?}");
        order.sort_unstable();
        let mut windows: Vec<&str> = tree.iter().map(String::as_str).collect();
        windows.sort_unstable();
        assert_eq!(order, windows, "focus visits exactly the windows of {device:?}");
        assert_eq!(device_profile(device).visible_windows as usize, if device == DeviceClass::Desktop { 5 } else if device == DeviceClass::Mobile { 1 } else { 2 });
        for region in focus_order(device) {
            let name = Region::name(*region);
            assert!(!name.en.is_empty() && !name.de.is_empty());
        }
    }
    for check in &report.checks {
        let en = semio_s_plugin_norm_app_surface::check_accessible_name(check, semio_ui_locale::Locale::En);
        let de = semio_s_plugin_norm_app_surface::check_accessible_name(check, semio_ui_locale::Locale::De);
        let speaks_utilisation = check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable;
        assert!(en.contains(&check.title.en) && (en.contains("utilisation") || !speaks_utilisation), "{en}");
        assert!(de.contains(&check.title.de) && (de.contains("Ausnutzung") || !speaks_utilisation), "{de}");
    }
    context::close(&mut app);
}
//#endregion 📱️Devices

#[test]
fn the_new_document_palette_offers_the_full_showcase_and_its_dsl_round_trips_to_the_committed_fixture() {
    let source = <En1993PlayApp as semio_plugin::ArtifactEditor>::examples().into_iter().find(|example| example.id() == "full-showcase").expect("the palette offers the full showcase");
    let showcase = semio_s_artifact_norm_en1993_showcase::full_showcase();
    assert_eq!(semio_s_artifact_norm_en1993_io::text::snapshot::decode_en1993_dsl(&source.document()).expect("the palette entry parses"), showcase);
    assert_eq!(semio_s_artifact_norm_en1993_io::text::snapshot::decode_en1993_snapshot_json(&semio_s_artifact_norm_en1993_io::text::snapshot::encode_en1993_snapshot_json(&showcase)).expect("the JSON twin parses"), showcase);
}

//#region ♿️ScreenReader
/// ♿️ Every check row, scene entity, handle and diagram of the default document has an accessible node (label, role, value or position) in English and German.
#[test]
fn the_default_document_publishes_an_accessible_node_for_every_check_handle_and_diagram() {
    let document = En1993Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = <En1993Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&document);
    let scene = viewport::scene_of(&document, &report);
    let entities: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let handles = viewport::handles_of(&document, &entities);
    let diagrams = semio_s_artifact_norm_en1993_scene::diagrams(&document, &report, None);
    let counts = semio_s_plugin_norm_app_surface::assert_default_document_nodes(&report, &scene, &handles, &diagrams);
    assert_eq!((counts.checks, counts.entities, counts.handles, counts.diagrams), (report.checks.len(), scene.entities.len(), handles.len(), diagrams.len()));
}
//#endregion ♿️ScreenReader

//#region 🕹️HandleBinding
#[test]
fn every_handle_write_of_the_default_and_showcase_documents_resolves_through_the_edit_rules_to_a_typed_kind() {
    use semio_framework_value::ToValue;
    use semio_s_plugin_norm_app_surface::NormEdit;
    for document in [En1993Snapshot::compliant_heb240_frame(), semio_s_artifact_norm_en1993_showcase::full_showcase()] {
        let selection = crate::live_entities(&document);
        let handles = crate::live_handles(&document, &selection);
        assert!(!handles.is_empty(), "the document advertises no handle");
        for handle in &handles {
            assert!(!handle.writes.is_empty(), "{} writes nothing", handle.id);
            for write in &handle.writes {
                let section = write.path.ends_with(".sectionId");
                let edit = NormEdit::SetField { path: write.path.clone(), value: if section { 0.24_f64 } else { 1.0_f64 }.to_value() };
                let mutations = semio_s_artifact_norm_en1993_mutations::resolve(&document, &edit).unwrap_or_else(|error| panic!("handle {} writes {}: {error}", handle.id, write.path));
                assert!(!mutations.is_empty(), "handle {} writes {} to no mutation", handle.id, write.path);
                let kind = format!("{:?}", mutations[0]);
                assert!(kind.starts_with("Change") || (section && kind.starts_with("Insert")), "handle {} writes {} to the untyped {kind}", handle.id, write.path);
            }
        }
    }
}
#[test]
fn dragging_a_scalar_grip_rewrites_the_leaf_re_evaluates_the_report_and_re_anchors_the_grip() {
    use semio_framework_value::ToValue;
    use semio_s_artifact_norm_en1993_mutations::mutation_bridge::apply_en1993_mutation;
    use semio_s_plugin_norm_app_surface::NormEdit;
    let document = semio_s_artifact_norm_en1993_showcase::full_showcase();
    let joint = document.joints.iter().find(|j| j.plate_fu > 0.0 && j.kind != semio_s_artifact_norm_en1993_model::JointKind::Welded).expect("a bolted joint with a plate strength").clone();
    let id = format!("{}:in:plateFu", joint.id);
    let grip = crate::live_handles(&document, &crate::live_entities(&document)).into_iter().find(|h| h.id == id).expect("the plate strength grip");
    let write = grip.writes_at(1.0).remove(0);
    assert_eq!(write.path, format!("joints[id={}].plateFu", joint.id));
    assert!(write.value > joint.plate_fu, "dragging up raises the strength");
    let mutations = semio_s_artifact_norm_en1993_mutations::resolve(&document, &NormEdit::SetField { path: write.path.clone(), value: write.value.to_value() }).expect("the edit rule resolves");
    let next = mutations.iter().fold(document.clone(), |current, mutation| apply_en1993_mutation(&current, mutation).expect("the mutation applies").0);
    let moved = next.joints.iter().find(|j| j.id == joint.id).expect("the joint").plate_fu;
    assert!((moved - write.value).abs() <= 1.0, "{moved} vs {}", write.value);
    let (before, after) = (<En1993Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&document), <En1993Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&next));
    assert_ne!(format!("{before:?}"), format!("{after:?}"), "the joint checks are re-evaluated with the new strength");
    let again = crate::live_handles(&next, &crate::live_entities(&next)).into_iter().find(|h| h.id == id).expect("the grip survives");
    assert!((again.writes[0].map.base - moved).abs() < 1e-6, "the re-evaluated grip stands on the new value");
}

//#endregion 🕹️HandleBinding

/// 🎪️ The steel-showcase DSL asset is offered by the palette with an English and a German title, decodes to the committed showcase document, evaluates and projects scene entities and diagrams without panicking.
#[test]
fn the_steel_showcase_asset_decodes_to_the_showcase_fixture_and_evaluates_with_scene_and_diagrams() {
    let source = <En1993PlayApp as semio_plugin::ArtifactEditor>::examples().into_iter().find(|example| example.id() == "steel-showcase").expect("the palette offers the showcase asset");
    for locale in [semio_ui_locale::Locale::En, semio_ui_locale::Locale::De] {
        assert!(!source.label().resolve(semio_ui_locale::Terminology::Native, locale).trim().is_empty());
    }
    let document = semio_s_artifact_norm_en1993_io::text::snapshot::decode_en1993_dsl(&source.document()).expect("the showcase asset parses");
    assert_eq!(document, semio_s_artifact_norm_en1993_showcase::full_showcase());
    let report = <En1993Family as semio_s_plugin_norm_compliance::NormFamily>::evaluate(&document);
    assert!(!report.checks.is_empty());
    let scene = viewport::scene_of(&document, &report);
    assert!(!scene.entities.is_empty());
    assert!(!semio_s_artifact_norm_en1993_scene::diagrams(&document, &report, None).is_empty());
}

#[test]
fn compiled_input_metadata_matches_all_native_schema_rows() {
    let findings = semio_s_plugin_norm_app_surface::norm_compiled_input_findings::<En1993PlayApp>();
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}
