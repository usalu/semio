//! 📝️ Generation2d play app — the generation input-form window (generate mode).

use semio_framework_artifact_playbook_playbook::flow_forms::PlaybookFlowFormProducer;
use semio_s_artifact_procedural_2d_terminology::Generation2dLabels;
use semio_s_artifact_procedural_2d_model::GENERATION2D_PLAY_APP_ID;
use semio_s_artifact_procedural_2d_model::snapshot::Generation2dSnapshot;
use semio_framework_artifact_playbook_playbook::{selected_generation, GenerationPlayState};
use semio_os_flow::forms_bridge::flow_host_snapshot_to_form_spec;
use semio_plugin::built_text_node;
use semio_plugin::BuiltNode;
use semio_ui_locale::LocalizedLabel;
use semio_plugin::SurfaceKind;
use semio_plugin::WindowKindDefinition;
use semio_plugin::WindowOptions;

//#region 🔖️Constants
pub const GENERATION2D_PLAY_WINDOW_GENERATE_FORM: &str = "generation2d-generate-form";
pub const GENERATION2D_PLAY_BODY_GENERATE_FORM: &str = "generation2d.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: GENERATION2D_PLAY_WINDOW_GENERATE_FORM.into(),
        label: LocalizedLabel::native("Form", "Formular"),
        body_key: GENERATION2D_PLAY_BODY_GENERATE_FORM.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "clipboard-list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &Generation2dSnapshot, generation: &GenerationPlayState, view: &semio_plugin::ViewModel, labels: &Generation2dLabels) -> semio_plugin::UiAssemblyResult<BuiltNode> {
    let Some(current) = selected_generation(generation) else {
        return built_text_node(semio_ui_locale::Label::data(labels.generate_hint.as_str())).map_err(|_| semio_plugin::PluginAssemblyError::new("ui.generate-form.hint", "fixed UI hint admission failed"));
    };
    let spec = flow_host_snapshot_to_form_spec(&document.host_snapshot, &PlaybookFlowFormProducer);
    let window = view.window_id.as_deref().unwrap_or(GENERATION2D_PLAY_WINDOW_GENERATE_FORM);
    let page = semio_s_procedural_core_semantic_ui::GenerationFormPage::requested(view, GENERATION2D_PLAY_BODY_GENERATE_FORM, window);
    let resources = semio_plugin::WindowResourceRequests { window_id: window, body_key: GENERATION2D_PLAY_BODY_GENERATE_FORM, requests: &view.tree_windows };
    semio_s_procedural_core_semantic_ui::generation_form(&spec, &current.values, GENERATION2D_PLAY_APP_ID, "updateGenerationValues", &current.id, &semio_s_procedural_core_semantic_ui::GenerationFormLabels { title: labels.window_generate_form.as_str(), no_inputs: labels.form_no_inputs.as_str(), field: labels.form_field.as_str(), no_image: labels.form_no_image.as_str() }, page, &resources)?.present(window, view.locale)
}
//#endregion 🔖️Render

//#region 🧪️Tests
//#endregion 🧪️Tests
