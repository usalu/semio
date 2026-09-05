//! 📝️ Generation2d play app — the generation input-form window (generate mode).

use crate::artifacts::generation2d::Generation2dSnapshot;
use crate::editor::generation2d::terminology::Generation2dLabels;
use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{selected_generation, GenerationPlayState};
use semio_framework_plugin::{built_text_node, BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_WINDOW_GENERATE_FORM: &str = "generation2d-generate-form";
pub const GENERATION2D_PLAY_BODY_GENERATE_FORM: &str = "generation2d.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
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
pub fn render(document: &Generation2dSnapshot, generation: &GenerationPlayState, labels: &Generation2dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let spec = flow_fixture_to_form_spec(&document.fixture);
    let Some(current) = selected_generation(generation) else {
        return built_text_node(semio_framework_plugin::Label::data(labels.generate_hint.as_str())).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.generate-form.hint", "fixed UI hint admission failed"));
    };
    crate::generation_form(&spec, &current.values, GENERATION2D_PLAY_APP_ID, "updateGenerationValues", &current.id)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn generate_form_hints_without_a_selected_generation() {
        let mut app = app().await;
        assert!(render_body(&mut app, GENERATION2D_PLAY_BODY_GENERATE_FORM).await.contains("Add a generation"));
    }
}
//#endregion 🧪️Tests
