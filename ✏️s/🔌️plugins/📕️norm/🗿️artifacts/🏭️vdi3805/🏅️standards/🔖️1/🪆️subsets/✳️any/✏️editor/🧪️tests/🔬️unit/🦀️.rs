pub(crate) mod context {
    use crate::*;
    use semio_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};
    
    /// ✏️ Adapts `create_vdi3805_app`'s `AppDefinition` (contract §2.4) into the `App { definition,
    /// examples }` shape `context::new_app_with_registry` still expects (framework test context gap,
    /// see w0-f-report.md gap 3 — swap for the canonical helper once it lands).
    pub fn vdi3805_manifest_for_tests() -> semio_plugin::App {
        semio_plugin::App { definition: create_vdi3805_app(), examples: Vec::new() }
    }
    
    /// 📜️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
    /// scope pays the caller's terminal retirement after the app's paid close.
    pub struct NormApp(pub VcsArtifactApp<EditorApp<Vdi3805PlayApp>>, pub semio_plugin::artifact_app_laws::OriginalFixtureScope);
    
    impl std::ops::Deref for NormApp {
        type Target = VcsArtifactApp<EditorApp<Vdi3805PlayApp>>;
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
        let scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = new_app_with_registry::<EditorApp<Vdi3805PlayApp>>(vdi3805_manifest_for_tests, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(meta("local").instance_id));
        NormApp(app, scope)
    }

    /// 🧹️ Closes every store the wrapper opened. A live `ArtifactStore` asserts in `Drop` unless it
    /// was driven to its terminal-empty shallow shell, so every fixture that mounts an app must end
    /// here.
    pub fn close(app: &mut NormApp) {
        semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    }

    /// 🔁️ Drives one dispatched typed operation to quiescence the way the plugin host does, draining
    /// EVERY result page: a bare `maintenance_step` loop retires nothing and turns a publication
    /// fault into a silent timeout.
    pub async fn settle(app: &mut NormApp) {
        semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle the typed operation");
    }
    
    pub async fn dispatch(app: &mut NormApp, command: Vdi3805Command) -> InvocationResult {
        let mut action_meta = meta("local");
        if command.command_id() == "setSelectedCheckIndex" {
            action_meta.view_state = Some(ViewModel {
                window_id: Some(checks::WINDOW_CHECKS.into()),
                focused_window_id: Some(checks::WINDOW_CHECKS.into()),
                window_instances: vec![ViewWindowInstance { id: checks::WINDOW_CHECKS.into(), window_kind_id: checks::WINDOW_CHECKS.into() }],
                ..ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
            });
        }
        let result = semio_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut app.0, &command, &action_meta, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("dispatch");
        settle(app).await;
        result
    }
    
    pub async fn render(app: &mut NormApp, body_key: &str) -> String {
        semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), app.1.cx()).await.expect("render projection")
    }
}

use crate::*;
use semio_plugin::PluginApp;

#[test]
fn retained_command_dispositions_match_the_language_neutral_oracle() {
    semio_s_plugin_norm_app_surface::retained_disposition_oracle::assert_fixture(VARIANT);
    let definition = create_vdi3805_app();
    let mut classified = 0usize;
    for window in definition.window_kinds.iter() {
        let dispatchable = semio_framework::window_kind_actions(&definition, window);
        for id in semio_s_plugin_norm_app_surface::NORM_RETAINED_TOOL_IDS {
            let action = dispatchable.iter().find(|action| action.id == *id).unwrap_or_else(|| panic!("window {} must dispatch {id}", window.id));
            assert_eq!(action.semantics.execution.interactive_job, InteractiveJobClassification::Migrated);
            classified += 1;
        }
    }
    assert_eq!(classified, definition.window_kinds.len() * semio_s_plugin_norm_app_surface::NORM_RETAINED_TOOL_IDS.len());
}


//#region ðï¸Manifest
#[semio_framework_async_macros::async_test]
async fn the_manifest_stitches_every_taxonomy_node() {
    let definition = create_vdi3805_app();
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
    let ports = create_vdi3805_app().io.ports;
    assert!(ports.iter().any(|port| port.id == "model:in" && port.direction == semio_plugin::MediaPortDirection::In));
    let report_out = ports.iter().find(|port| port.id == "report:out").expect("report:out declared");
    assert_eq!(report_out.kind_id.as_deref(), Some(semio_s_plugin_norm_app_surface::artifact_kind_id(VARIANT).as_str()));
}

#[semio_framework_async_macros::async_test]
async fn an_unknown_body_key_falls_back_to_a_text_node() {
    let mut app = context::app_with_registry().await;
    assert!(context::render(&mut app, "norm.vdi3805.play.nope").await.contains("Unknown body"));
    context::close(&mut app);
}

//#endregion ðï¸Manifest

//#region 🪟️VisualWindows
#[test]
fn the_manifest_declares_the_five_visual_windows_and_no_json_window() {
    let definition = create_vdi3805_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    for expected in [outliner::WINDOW_OUTLINER, properties::WINDOW_PROPERTIES, viewport::WINDOW_VIEWPORT, diagrams::WINDOW_DIAGRAMS, checks::WINDOW_CHECKS] {
        assert!(ids.contains(&expected), "window {expected} missing in {ids:?}");
    }
    assert_eq!(ids.len(), 5);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_body_key_renders_without_json() {
    let mut app = context::app_with_registry().await;
    for body_key in [outliner::BODY_OUTLINER, properties::BODY_PROPERTIES, viewport::BODY_VIEWPORT, diagrams::BODY_DIAGRAMS, checks::BODY_CHECKS, document_panel::BODY_ARTIFACT, catalogue_panel::BODY_CATALOGUE, inspection_panel::BODY_INSPECTION] {
        let rendered = context::render(&mut app, body_key).await;
        assert!(!rendered.contains("Unknown body"), "{body_key} must render its own node");
        assert!(!rendered.contains("\"correctionAsOf\""), "{body_key} must not dump the document as JSON");
    }
    context::close(&mut app);
}
//#endregion 🪟️VisualWindows

//#region 🛰️LiveWiring
#[semio_framework_async_macros::async_test]
async fn render_right_after_an_edit_shows_updated_checks_without_evaluate() {
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, Vdi3805Command::SetField(set_field::SetField { path: "strictMode".into(), value_json: "true".into() })).await;
    let edited = app.snapshot().expect("projection");
    let expected = semio_s_artifact_norm_vdi3805_inferences::evaluate(&edited);
    assert_eq!(*<Vdi3805Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&edited), expected, "the retained session equals the full fold");
    let rendered = context::render(&mut app, checks::BODY_CHECKS).await;
    assert!(!rendered.contains("No checks computed"), "no evaluate command was dispatched: {rendered}");
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn a_geometry_grip_commit_edits_the_bounding_box() {
    let mut app = context::app_with_registry().await;
    let base = app.snapshot().expect("projection");
    context::dispatch(&mut app, Vdi3805Command::SetField(set_field::SetField { path: "geometry[id=geom-valve-50].bbox.maxX".into(), value_json: "0.25".into() })).await;
    let edited = app.snapshot().expect("projection");
    assert_eq!(edited.geometry["geom-valve-50"].bbox.max_x, 0.25);
    assert_ne!(edited, base);
    context::close(&mut app);
}

#[test]
fn a_picked_entity_selects_its_governing_check_in_every_window() {
    let document = Vdi3805Snapshot::default();
    let report = semio_s_artifact_norm_vdi3805_inferences::evaluate(&document);
    let entity = report.checks.iter().find(|check| !check.subject.entity_id.is_empty() && check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable).map(|check| check.subject.entity_id.clone()).expect("a check with an entity");
    let live = semio_s_plugin_norm_app_surface::LiveView::<Vdi3805Family>::new(document, None, &[entity.clone()]);
    assert!(live.selection.contains(&entity));
    assert_eq!(live.focus.as_deref(), Some(entity.as_str()));
    assert!(live.selection.checks_selected().is_some());
    assert!(live.selection.outliner_state().selected.contains(&entity));
}
//#endregion 🛰️LiveWiring

//#region ♿️ScreenReader
#[test]
fn the_default_document_publishes_an_accessible_node_for_every_check_handle_and_diagram() {
    let document = Vdi3805Snapshot::default();
    let report = <Vdi3805Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&document);
    let scene = viewport::scene_of(&document, &report);
    let entities: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let handles = viewport::handles_of(&document, &entities);
    let diagrams = semio_s_artifact_norm_vdi3805_scene::diagrams(&document, &report, None);
    let counts = semio_s_plugin_norm_app_surface::assert_default_document_nodes(&report, &scene, &handles, &diagrams);
    assert_eq!((counts.checks, counts.entities, counts.handles, counts.diagrams), (report.checks.len(), scene.entities.len(), handles.len(), diagrams.len()));
}
//#endregion ♿️ScreenReader

//#region ðï¸Behavior
/// 🧩️ The `NormFamily` binding lives here now (it was in the constitutional `op` crate) it
/// names `evaluate`, so it belongs beside the compute it binds.
#[semio_framework_async_macros::async_test]
async fn norm_family_id() {
    assert_eq!(<Vdi3805Family as semio_s_plugin_norm_compliance::NormFamily>::family_id(), semio_s_plugin_norm_compliance::NormFamilyId::Vdi3805);
    assert_eq!(semio_s_plugin_norm_compliance::NormFamilyId::Vdi3805.label(), "VDI 3805");
}


/// 🧮️ `setSelectedCheckIndex` is Results-window-config-only â it must dispatch cleanly and never touch the document.
#[semio_framework_async_macros::async_test]
async fn selected_check_index_is_a_config_only_edit() {
    let mut app = context::app_with_registry().await;
    let before = app.snapshot().expect("projection");
    let result = context::dispatch(&mut app, Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(2) })).await;
    assert!(result.mutations.is_empty(), "a Results-window-config-only command must emit no document operations");
    assert_eq!(before, app.snapshot().expect("projection"), "a Results-window-config-only command must never mutate the document");
    context::close(&mut app);
}

/// 🧬️ Kind-discipline wrapper: the real registry enforces that View actions never emit document
/// operations.
#[semio_framework_async_macros::async_test]
async fn view_actions_never_emit_artifact_mutations_under_the_real_registry() {
    let mut app = context::app_with_registry().await;
    let result = context::dispatch(&mut app, Vdi3805Command::SetSelectedCheckIndex(selected_check::SetSelectedCheckIndex { index: Some(1) })).await;
    assert!(result.mutations.is_empty());
    context::close(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn undo_redo_round_trips_through_the_wrapper() {
    let mut app = context::app_with_registry().await;
    let base = app.snapshot().expect("projection");
    context::dispatch(&mut app, Vdi3805Command::SetField(set_field::SetField { path: "strictMode".into(), value_json: (!base.strict_mode).to_string() })).await;
    let edited = app.snapshot().expect("projection");
    assert_ne!(edited, base, "the concrete setter moved the document");
    semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "undo", None, &semio_plugin::artifact_app_laws::meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("undo");
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), base);
    semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app.0, "redo", None, &semio_plugin::artifact_app_laws::meta("local"), &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close).await.expect("redo");
    context::settle(&mut app).await;
    assert_eq!(app.snapshot().expect("projection"), edited);
    context::close(&mut app);
}

/// 🎞️ `report:out` dumps the currently computed `CheckReport` as a `Structured` media payload.
#[semio_framework_async_macros::async_test]
async fn report_out_exports_the_computed_check_report() {
    let mut app = context::app_with_registry().await;
    let media = ::semio_framework_async::poll::resolve_ready(semio_plugin::artifact_app_laws::export_fixture_media(&mut app.0, "report:out", app.1.cx(), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close)).expect("export report:out");
    let semio_plugin::MediaPayload::Structured { schema, json } = media.payload else { panic!("expected a structured payload") };
    assert_eq!(schema, semio_s_plugin_norm_app_surface::artifact_kind_id(VARIANT));
    let value: serde_json::Value = serde_json::from_str(&json).expect("report json parses");
    let checks = value.get("checks").and_then(|c| c.as_array()).expect("checks array");
    assert!(!checks.is_empty());
    context::close(&mut app);
}
//#endregion ðï¸Behavior

semio_plugin::history_edit_acceptance_law!("norm", crate::Vdi3805PlayApp, || semio_plugin::App { definition: crate::create_vdi3805_app(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");

#[test]
fn compiled_input_metadata_matches_all_native_schema_rows() {
    let findings = semio_s_plugin_norm_app_surface::norm_compiled_input_findings::<Vdi3805PlayApp>();
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}
