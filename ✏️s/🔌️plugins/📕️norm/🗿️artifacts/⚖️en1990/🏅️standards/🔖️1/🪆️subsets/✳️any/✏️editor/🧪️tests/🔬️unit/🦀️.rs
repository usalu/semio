pub(crate) mod context {
    use crate::*;
    use semio_framework_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_framework_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    /// ✏️ Adapts `create_en1990_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry` still expects (framework test context gap,
    /// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
    pub fn en1990_manifest_for_tests() -> semio_framework_plugin::App {
        semio_framework_plugin::App { definition: create_en1990_app(), examples: Vec::new() }
    }

    semio_framework_plugin::history_edit_acceptance_law!("norm", En1990PlayApp, en1990_manifest_for_tests, "../..");
    
    /// 📜️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
    /// scope pays the caller's terminal retirement after the app's paid close.
    pub struct NormApp(pub VcsArtifactApp<EditorApp<En1990PlayApp>>, pub semio_framework_plugin::artifact_app_laws::OriginalFixtureScope);
    
    impl std::ops::Deref for NormApp {
        type Target = VcsArtifactApp<EditorApp<En1990PlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl std::ops::DerefMut for NormApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    
    /// 🧬️ A wrapper carrying the real registry so kind discipline (View-emits-operations rejection) runs.
    pub async fn app_with_registry() -> NormApp {
        let scope = semio_framework_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = new_app_with_registry::<EditorApp<En1990PlayApp>>(en1990_manifest_for_tests, semio_framework_os_kernel::ActorId(semio_framework_os_kernel::LOCAL_ACTOR_ID.into()), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity()).await;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(meta("local").instance_id));
        NormApp(app, scope)
    }

    /// 🧹️ Closes every store the wrapper opened. A live `ArtifactStore` asserts in `Drop` unless it was
    /// driven to its terminal-empty shallow shell, so every fixture that mounts an app must end here.
    pub fn close(app: &mut NormApp) {
        semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    }
    
    pub async fn dispatch(app: &mut NormApp, command: En1990Command) -> InvocationResult {
        let mut action_meta = meta("local");
        if command.command_id() == "setSelectedCheckIndex" {
            action_meta.view_state = Some(ViewModel {
                window_id: Some(checks::WINDOW_CHECKS.into()),
                focused_window_id: Some(checks::WINDOW_CHECKS.into()),
                window_instances: vec![ViewWindowInstance { id: checks::WINDOW_CHECKS.into(), window_kind_id: checks::WINDOW_CHECKS.into() }],
                ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native)
            });
        }
        let result = semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &action_meta, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        settle(app).await;
        result
    }

    /// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does, draining
    /// EVERY result page: a bare `maintenance_step` loop retires nothing and turns a publication fault
    /// into a silent timeout, which is exactly how ticket 26/09/18 slice B2b lost the real cause.
    pub async fn settle(app: &mut NormApp) {
        semio_framework_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
    }
    
    pub async fn render(app: &mut NormApp, body_key: &str) -> String {
        semio_framework_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native), app.1.cx()).await.expect("render projection")
    }
}

use crate::*;
use semio_framework_plugin::PluginApp;

#[test]
fn retained_command_dispositions_match_the_language_neutral_oracle() {
    semio_s_plugin_norm_app_surface::retained_disposition_oracle::assert_fixture(VARIANT);
    let definition = create_en1990_app();
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
/// 🎯️ One value per `En1990Command` row â the whole-command-surface laws below iterate it, so a new row
/// that is not listed here fails `command_ids_cover_every_row`.
fn every_command() -> Vec<En1990Command> {
    vec![
        En1990Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) }),
        En1990Command::SetField(set_field::SetField { path: "consequenceClass".into(), value_json: "2".into() }),
        En1990Command::InsertItem(insert_item::InsertItem { path: "variables".into(), index: 0, value_json: None }),
        En1990Command::RemoveItem(remove_item::RemoveItem { path: "variables".into(), index: 0 }),
        En1990Command::ApplyRemedy(apply_remedy::ApplyRemedy { check_id: String::new(), remedy_index: 0 }),
    ]
}

#[semio_framework_async_macros::async_test]
async fn command_ids_cover_every_row_and_are_unique() {
    let commands = every_command();
    let ids: Vec<&str> = commands.iter().map(En1990Command::command_id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "duplicate command ids in {ids:?}");
    assert_eq!(ids, vec!["setSelectedCheckIndex", "setField", "insertItem", "removeItem", "applyRemedy"]);
}

/// 🧷️ The permanent wire guard: every row round-trips textâbinary and prints under its own declared
/// kebab wire keyword (which is deliberately NOT the camelCase `command_id`).
#[semio_framework_async_macros::async_test]
async fn every_command_round_trips_text_and_binary_under_its_declared_wire_keyword() {
    let keywords = ["selected-check", "set-field", "insert-item", "remove-item", "apply-remedy"];
    for (command, keyword) in every_command().into_iter().zip(keywords) {
        store::os_store::test_support::assert_op_text_binary_equivalence(&command);
        let printed = protocol::OpText::print_op(&command);
        assert!(printed.starts_with(keyword), "row {} printed {printed:?}, expected keyword {keyword}", command.command_id());
    }
}

/// 🧷️ Pins the exact wire bytes for the rows whose shape the `app_commands!` decomposition
/// could have silently rewritten â the fieldless `Evaluate` (was a unit variant) and both `Option`
/// cases of `SetSelectedCheckIndex`. Hex is the pinned baseline;
/// these bytes are identical for all fifteen norm apps because none
/// of the three payload shapes involves the per-standard `En1990Snapshot`.
#[semio_framework_async_macros::async_test]
async fn optional_field_rows_keep_their_pinned_wire_bytes() {
    let hex = |command: &En1990Command| protocol::OpBinary::encode_op(command).expect("encode").iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    assert_eq!(hex(&En1990Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })), "01010001000402");
    assert_eq!(hex(&En1990Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: None })), "01010000");
}
//#endregion ðï¸CommandSurface

//#region ðï¸Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_en1990_app();
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
    let ports = create_en1990_app().io.ports;
    assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_framework_plugin::MediaPortDirection::In));
    let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
    assert_eq!(report_out.kind_id.as_deref(), Some(semio_s_plugin_norm_app_surface::artifact_kind_id(VARIANT).as_str()));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, "norm.en1990.play.nope").await.contains("Unknown body"));
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
#[semio_framework_async_macros::async_test]
async fn evaluate_commits_a_host_backed_report() {
    let mut app = context::app_with_registry().await;
    let mut host = NormHost::<En1990Family>::live(app.snapshot().expect("projection"));
    host.evaluate();
    assert!(!host.report().checks.is_empty());
    context::close(&mut app);
}

/// 🧮️ `setSelectedCheckIndex` is Results-window-config-only â it must dispatch cleanly and never touch the document.
#[semio_framework_async_macros::async_test]
async fn selected_check_index_is_a_config_only_edit() {
    let mut app = context::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    let result = context::dispatch(&mut app, En1990Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })).await;
    assert!(result.mutations.is_empty(), "a Results-window-config-only command must emit no document operations");
    assert_eq!(before, app.snapshot().expect("projection"), "a Results-window-config-only command must never mutate the document");
    context::close(&mut app);
}

/// 🧬️ Kind-discipline wrapper: the real registry enforces that View actions never emit document
/// operations.
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = context::app_with_registry().await;
    let result = context::dispatch(&mut app, En1990Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) })).await;
    assert!(result.mutations.is_empty());
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = context::app_with_registry().await;
    let base = app.snapshot().expect("projection");
    context::dispatch(&mut app, En1990Command::SetField(set_field::SetField { path: "annex".into(), value_json: "\"En\"".into() })).await;
    let edited = app.snapshot().expect("projection");
    assert_ne!(edited, base, "the concrete setter moved the document");
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "undo", None, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("undo");
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), base);
    semio_framework_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "redo", None, &semio_framework_plugin::artifact_app_laws::meta("local"), &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("redo");
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), edited);
    context::close(&mut app);
}

/// 🎞️ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
#[semio_framework_async_macros::async_test]
async fn report_out_exports_the_computed_check_report() {
    let mut app = context::app_with_registry().await;
    let mut host = NormHost::<En1990Family>::live(app.snapshot().expect("projection"));
    host.evaluate();
    let media = ::semio_framework_async::poll::resolve_ready(semio_framework_plugin::artifact_app_laws::export_fixture_media(&mut app.0, "report:out", app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).expect("export report:out");
    let semio_framework_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
    assert_eq!(schema, semio_s_plugin_norm_app_surface::artifact_kind_id(VARIANT));
    let value: serde_json::Value = serde_json::from_str(&json).expect("report json parses");
    let checks = value.get("checks").and_then(|c| c.as_array()).expect("checks array");
    assert!(!checks.is_empty());
    context::close(&mut app);
}

/// 🏷️ Every document leaf a drag handle, a dimension or a diagram marker writes has unit-aware field metadata, so inspector, handles and diagrams share units and labels.
#[test]
fn every_scene_binding_has_field_meta() {
    use semio_s_artifact_norm_en1990_scene::{diagrams, handles, project, SceneInput, COMBINATION_PATH};
    let generic = |path: &str| {
        let mut out = String::new();
        let mut rest = path;
        while let Some(at) = rest.find("[id=") {
            let end = rest[at..].find(']').map_or(rest.len(), |e| at + e);
            out.push_str(&rest[..at]);
            out.push_str("[]");
            rest = &rest[(end + 1).min(rest.len())..];
        }
        out.push_str(rest);
        out
    };
    for document in [En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), semio_s_artifact_norm_en1990_examples::road_bridge_compliant::reference_snapshot()] {
        let report = semio_s_artifact_norm_en1990_inferences::evaluate(&document);
        let scene = project(&SceneInput { document: &document, report: &report, placements: &[], revision: semio_s_plugin_norm_structural_scene::SceneRevision { document_seq: 0, report_seq: 0 }, combination: None, traffic_group: None });
        let selection: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
        let mut paths: Vec<String> = handles(&document, &[], &selection).iter().flat_map(|h| h.writes.iter().map(|w| w.path.clone())).filter(|p| p != COMBINATION_PATH && p != semio_s_artifact_norm_en1990_scene::TRAFFIC_GROUP_PATH).collect();
        paths.extend(scene.annotations.iter().filter_map(|a| match a {
            semio_s_plugin_norm_structural_scene::Annotation::Dimension { binding: Some(b), .. } => Some(b.path.clone()),
            _ => None,
        }));
        paths.extend(diagrams(&document, &report, None, None).iter().flat_map(|d| d.markers.iter().filter_map(|m| m.binding.as_ref().map(|b| b.path.clone()))));
        assert!(!paths.is_empty());
        for path in paths {
            assert!(field_meta::en1990_field_meta(&generic(&path)).is_some(), "no field meta for {path}");
        }
    }
}

//#region 🛰️LiveWiring
#[semio_framework_async_macros::async_test]
async fn render_right_after_an_edit_shows_updated_checks_without_evaluate() {
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, En1990Command::SetField(set_field::SetField { path: "consequenceClass".into(), value_json: "2".into() })).await;
    let edited = app.snapshot().expect("projection");
    let expected = semio_s_artifact_norm_en1990_inferences::evaluate(&edited);
    assert_eq!(*<En1990Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&edited), expected, "the retained session equals the full fold");
    let worst = expected.checks.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).expect("the edited document still has checks");
    let rendered = context::render(&mut app, checks::BODY_CHECKS).await;
    assert!(!rendered.contains("No checks computed"), "no evaluate command was dispatched: {rendered}");
    let escaped = worst.title.en.replace('\\', "\\\\").replace('"', "\\\"");
    assert!(rendered.contains(&escaped), "the checks window shows the edited report: {rendered}");
    context::close(&mut app);
}

#[test]
fn a_picked_entity_selects_its_governing_check_in_every_window() {
    let report = semio_s_artifact_norm_en1990_inferences::evaluate(&En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De));
    let entity = report.checks.iter().find(|check| !check.subject.entity_id.is_empty() && check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable).map(|check| check.subject.entity_id.clone()).expect("a check with an entity");
    let live = semio_s_plugin_norm_app_surface::LiveView::<En1990Family>::new(En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De), None, &[entity.clone()]);
    assert!(live.selection.contains(&entity));
    assert_eq!(live.focus.as_deref(), Some(entity.as_str()));
    assert!(live.selection.checks_selected().is_some());
    assert!(live.selection.outliner_state().selected.contains(&entity));
}
//#endregion 🛰️LiveWiring

//#region ♿️ScreenReader
/// ♿️ Every check row, scene entity, handle and diagram of the default document has an accessible node (label, role, value or position) in English and German.
#[test]
fn the_default_document_publishes_an_accessible_node_for_every_check_handle_and_diagram() {
    let document = En1990Snapshot::example(semio_s_plugin_norm_compliance::AnnexChoice::De);
    let report = <En1990Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&document);
    let scene = viewport::scene_of(&document, &report, semio_s_plugin_norm_structural_scene::projection::ViewOptions::none());
    let entities: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let handles = viewport::handles_of(&document, &entities);
    let diagrams = semio_s_artifact_norm_en1990_scene::diagrams(&document, &report, None, None);
    let counts = semio_s_plugin_norm_app_surface::assert_default_document_nodes(&report, &scene, &handles, &diagrams);
    assert_eq!((counts.checks, counts.entities, counts.handles, counts.diagrams), (report.checks.len(), scene.entities.len(), handles.len(), diagrams.len()));
}
//#endregion ♿️ScreenReader

#[test]
fn compiled_input_metadata_matches_all_native_schema_rows() {
    let findings = semio_s_plugin_norm_app_surface::norm_compiled_input_findings::<En1990PlayApp>();
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}
