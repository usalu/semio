//! 🔬️ Editor unit-test harness for DIN 4108.

pub(crate) mod context {
    use crate::*;
    use semio_plugin::artifact_app_laws::{meta, new_app_with_registry};
    use semio_plugin::{EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel, ViewWindowInstance};

    pub fn din4108_manifest_for_tests() -> semio_plugin::App {
        semio_plugin::App { definition: create_din4108_app(), examples: Vec::new() }
    }

    /// 📜️ Field `1` is the fixture's ONE original caller scope: every call that needs a `cx` threads `app.1.cx()`, and the
    /// scope pays the caller's terminal retirement after the app's paid close.
    pub struct NormApp(pub VcsArtifactApp<EditorApp<Din4108PlayApp>>, pub semio_plugin::artifact_app_laws::OriginalFixtureScope);
    
    impl std::ops::Deref for NormApp {
        type Target = VcsArtifactApp<EditorApp<Din4108PlayApp>>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl std::ops::DerefMut for NormApp {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    pub async fn app_with_registry() -> NormApp {
        let scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
        let mut app = new_app_with_registry::<EditorApp<Din4108PlayApp>>(din4108_manifest_for_tests, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity()).await;
        ::semio_framework_async::poll::resolve_ready(app.bind_instance_id(meta("local").instance_id));
        NormApp(app, scope)
    }

    pub fn close(app: &mut NormApp) {
        semio_plugin::artifact_app_laws::close_registered_fixture_app(&mut app.0, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, app.1.cx()).expect("original caller paid app close");
    }

    pub async fn dispatch(app: &mut NormApp, command: Din4108Command) -> InvocationResult {
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

    pub async fn settle(app: &mut NormApp) {
        semio_plugin::artifact_app_laws::settle_registered_typed_operation(&mut app.0, meta("local").instance_id, semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, &mut semio_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("settle");
    }

    pub async fn render(app: &mut NormApp, body_key: &str) -> String {
        semio_plugin::artifact_app_laws::render_registered_fixture_app(&mut app.0, body_key, &ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native), app.1.cx()).await.expect("render projection")
    }
}

use crate::*;

#[test]
fn editor_module_links() {
    let _ = create_din4108_app();
}

#[test]
fn the_manifest_declares_the_four_visual_windows_and_no_json_window() {
    let definition = create_din4108_app();
    let ids: Vec<&str> = definition.window_kinds.iter().map(|window| window.id.as_str()).collect();
    for expected in [properties::WINDOW_PROPERTIES, viewport::WINDOW_VIEWPORT, diagrams::WINDOW_DIAGRAMS, checks::WINDOW_CHECKS] {
        assert!(ids.contains(&expected), "window {expected} missing in {ids:?}");
    }
    assert_eq!(ids.len(), 4);
}

#[semio_framework_async_macros::async_test]
async fn every_declared_body_key_renders_without_json() {
    let mut app = context::app_with_registry().await;
    for body_key in [outliner::BODY_OUTLINER, properties::BODY_PROPERTIES, viewport::BODY_VIEWPORT, diagrams::BODY_DIAGRAMS, checks::BODY_CHECKS] {
        let rendered = context::render(&mut app, body_key).await;
        assert!(!rendered.contains("Unknown body"), "{body_key} must render its own node");
        assert!(!rendered.contains("\"tIntC\""), "{body_key} must not dump the document as JSON");
    }
    context::close(&mut app);
}

//#region 🛰️LiveWiring
#[semio_framework_async_macros::async_test]
async fn render_right_after_an_edit_shows_updated_checks_without_evaluate() {
    let mut app = context::app_with_registry().await;
    context::dispatch(&mut app, Din4108Command::SetField(set_field::SetField { path: "tIntC".into(), value_json: "22.0".into() })).await;
    let edited = app.snapshot().expect("projection");
    let expected = semio_s_artifact_norm_din4108_inferences::evaluate(&edited);
    assert_eq!(*<Din4108Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&edited), expected, "the retained session equals the full fold");
    let worst = expected.checks.iter().max_by(|a, b| a.utilization.total_cmp(&b.utilization)).expect("the edited document still has checks");
    let rendered = context::render(&mut app, checks::BODY_CHECKS).await;
    assert!(!rendered.contains("No checks computed"), "no evaluate command was dispatched: {rendered}");
    let escaped = worst.title.en.replace('\\', "\\\\").replace('"', "\\\"");
    assert!(rendered.contains(&escaped), "the checks window shows the edited report: {rendered}");
    context::close(&mut app);
}

#[test]
fn a_picked_entity_selects_its_governing_check_in_every_window() {
    let report = semio_s_artifact_norm_din4108_inferences::evaluate(&Din4108Snapshot::default());
    let entity = report.checks.iter().find(|check| !check.subject.entity_id.is_empty() && check.status != semio_s_plugin_norm_compliance::CheckStatus::NotApplicable).map(|check| check.subject.entity_id.clone()).expect("a check with an entity");
    let live = semio_s_plugin_norm_app_surface::LiveView::<Din4108Family>::new(Din4108Snapshot::default(), None, &[entity.clone()]);
    assert!(live.selection.contains(&entity));
    assert_eq!(live.focus.as_deref(), Some(entity.as_str()));
    assert!(live.selection.checks_selected().is_some());
    assert!(live.selection.outliner_state().selected.contains(&entity));
}
//#endregion 🛰️LiveWiring

semio_plugin::history_edit_acceptance_law!("norm", crate::Din4108PlayApp, || semio_plugin::App { definition: crate::create_din4108_app(), examples: Vec::new() }, "../../🏅️standards/🔖️1/🪆️subsets/✳️any");

//#region ♿️ScreenReader
/// ♿️ Every check row, scene entity, handle and diagram of the default document has an accessible node (label, role, value or position) in English and German.
#[test]
fn the_default_document_publishes_an_accessible_node_for_every_check_handle_and_diagram() {
    let document = Din4108Snapshot::default();
    let report = <Din4108Family as semio_s_plugin_norm_compliance::NormFamily>::live_report(&document);
    let scene = viewport::scene_of(&document, &report);
    let entities: Vec<_> = scene.entities.iter().map(|e| e.entity.clone()).collect();
    let handles = viewport::handles_of(&document, &entities);
    let diagrams = semio_s_artifact_norm_din4108_scene::diagrams(&document, &report, None);
    let counts = semio_s_plugin_norm_app_surface::assert_default_document_nodes(&report, &scene, &handles, &diagrams);
    assert_eq!((counts.checks, counts.entities, counts.handles, counts.diagrams), (report.checks.len(), scene.entities.len(), handles.len(), diagrams.len()));
}
//#endregion ♿️ScreenReader

#[test]
fn compiled_input_metadata_matches_all_native_schema_rows() {
    let findings = semio_s_plugin_norm_app_surface::norm_compiled_input_findings::<Din4108PlayApp>();
    assert!(findings.is_empty(), "{}", findings.join("\n"));
}
