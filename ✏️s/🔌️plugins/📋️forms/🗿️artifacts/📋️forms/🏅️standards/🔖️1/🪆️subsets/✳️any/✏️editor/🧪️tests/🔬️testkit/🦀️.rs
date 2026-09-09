use super::*;
use semio_framework_plugin::testkit::{meta, new_app_with_registry};
use semio_framework_plugin::{App, EditorApp, InvocationResult, PluginApp, VcsArtifactApp, ViewModel};

pub type FormsApp = VcsArtifactApp<EditorApp<FormsPlayApp>>;

/// 🧪️ An app instance with its concrete command registry and retained job proofs.
pub async fn forms_app() -> FormsApp {
    new_app_with_registry::<EditorApp<FormsPlayApp>>(forms_manifest_for_testkit).await
}

/// 🚧️ SDK GAP (w0-f-report Gap 3): `new_app_with_registry`/`assert_declared_actions_bridge_to_commands`
/// still take `fn() -> App` (the pre-migration manifest wrapper), unchanged for this ticket —
/// `create_forms_app` now returns `AppDefinition`, so wrap it in a throwaway `App` (empty examples)
/// rather than widen the framework testkit signature.
fn forms_manifest_for_testkit() -> App {
    App { definition: create_forms_app(), examples: Vec::new() }
}

/// 🧪️ An app wired to the real manifest registry — enforces View/Shell kind discipline, and the
/// `kind` default declared on `addQuestion` materializes host-side.
pub async fn forms_app_with_registry() -> FormsApp {
    new_app_with_registry::<EditorApp<FormsPlayApp>>(forms_manifest_for_testkit).await
}

pub async fn config(app: &FormsApp) -> FormsConfig {
    let files = app.config_pack().await.expect("config pack");
    store::parse_document_pack::<FormsConfig, FormsConfigMutation>(&files.pack, &files.spr).await.expect("config projection").snapshot
}

pub fn action_args(value: &serde_json::Value) -> dsl::DslValue {
    dsl::os_pack::json_to_dsl_value(&dsl::os_pack::json::parse(&value.to_string()).expect("fixture JSON"))
}

pub async fn dispatch(app: &mut FormsApp, command: FormsCommand) -> InvocationResult {
    app.dispatch_typed(command, &meta("local")).await.expect("dispatch")
}

pub async fn render(app: &mut FormsApp, body_key: &str) -> String {
    serde_json::to_string(&app.render(body_key, None, &ViewModel::default()).await.expect("render").root).expect("rendered component JSON")
}

/// 🧩️ A host contribution registering `"buildingComponent"` as an extension question kind rendered
/// by `forms-module-procedural` — shared by every test exercising the extension-question path.
pub fn building_component_contributions() -> Vec<ProgramContributionEntry> {
    vec![ProgramContributionEntry {
        plugin_id: "forms-module-procedural".into(),
        topic_contribution: Some(semio_framework_plugin::TopicContribution::new(
            "forms.questionKind",
            semio_framework_os_kernel::DslValue::object([
                ("appId".to_string(), semio_framework_os_kernel::DslValue::String("forms-module-procedural".to_string())),
                ("questionKind".to_string(), semio_framework_os_kernel::DslValue::String("buildingComponent".to_string())),
                ("label".to_string(), semio_framework_os_kernel::DslValue::String("Building Component".to_string())),
                ("iconId".to_string(), semio_framework_os_kernel::DslValue::String("building".to_string())),
                ("paramsBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("params".to_string())),
                ("previewBodyKey".to_string(), semio_framework_os_kernel::DslValue::String("preview".to_string())),
            ]),
        )),
    }]
}

/// 🧩️ A standalone `buildingComponent` question, for tests that exercise `render_extension_question`
/// directly without going through a full document.
pub fn building_component_question() -> FormQuestion {
    let mut question = add_question::question_shell("geometry".into(), "Geometry".into(), "buildingComponent".into());
    question.fixture_slug = Some("hexagonal-mushroom-column".into());
    question.params = Some(crate::schema::value_to_dsl(&dsl::json!({ "height": 6.0, "radius": 0.5, "sides": 6.0 })));
    question
}
