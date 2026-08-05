//! 📝️ Procedural3d play app — the generation input-form window (generate mode).

use crate::apps::procedural3d::terminology::Procedural3dLabels;
use crate::apps::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use flow_core::forms_bridge::flow_fixture_to_form_spec;
use flow_core::FlowFixture;
use playbook::{selected_generation, GenerationPlayState};
use semio_framework_plugin::{render_generation_form_body, ui_text, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural3d-generate-form";
pub const PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM: &str = "procedural.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL_3D_PLAY_WINDOW_GENERATE_FORM.into(),
        label: LocalizedLabel::native("Form", "Formular"),
        body_key: PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "clipboard-list".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_projection_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(fixture: &FlowFixture, generation: &GenerationPlayState, labels: &Procedural3dLabels) -> UiNode {
    let spec = flow_fixture_to_form_spec(fixture);
    let Some(current) = selected_generation(generation) else {
        return ui_text(labels.generate_hint);
    };
    render_generation_form_body(&spec, &current.values, PROCEDURAL_3D_PLAY_APP_ID, "updateGenerationValues", &current.id)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn generate_form_hints_without_a_selected_generation() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_GENERATE_FORM).contains("Add a generation"));
    }
}
//#endregion 🧪️Tests
