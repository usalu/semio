//! 📝️ Generation3d play app — the generation input-form window (generate mode).

use crate::editor::generation3d::terminology::Generation3dLabels;
use crate::editor::generation3d::GENERATION_3D_PLAY_APP_ID;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{selected_generation, GenerationPlayState};
use flow::FlowFixture;
use semio_framework_plugin::{built_text_node, BuiltNode, LocalizedLabel, SurfaceKind, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const GENERATION_3D_PLAY_WINDOW_GENERATE_FORM: &str = "generation3d-generate-form";
pub const GENERATION_3D_PLAY_BODY_GENERATE_FORM: &str = "procedural.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION_3D_PLAY_WINDOW_GENERATE_FORM.into(),
        label: LocalizedLabel::native("Form", "Formular"),
        body_key: GENERATION_3D_PLAY_BODY_GENERATE_FORM.into(),
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
pub fn render(fixture: &FlowFixture, generation: &GenerationPlayState, labels: &Generation3dLabels) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let spec = flow_fixture_to_form_spec(fixture);
    let Some(current) = selected_generation(generation) else {
        return built_text_node(semio_framework_plugin::Label::data(labels.generate_hint.as_str())).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.generate-form.hint", "fixed UI hint admission failed"));
    };
    crate::generation_form(&spec, &current.values, GENERATION_3D_PLAY_APP_ID, "updateGenerationValues", &current.id)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::generation3d::testkit::{app, render as render_body};

    #[test]
    fn generate_form_hints_without_a_selected_generation() {
        let mut app = app();
        assert!(render_body(&mut app, GENERATION_3D_PLAY_BODY_GENERATE_FORM).contains("Add a generation"));
    }
}
//#endregion 🧪️Tests
