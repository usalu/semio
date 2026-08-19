//! 📝️ Procedural2d play app — the generation input-form window (generate mode).

use crate::editor::procedural2d::terminology::Procedural2dLabels;
use crate::editor::procedural2d::PROCEDURAL2D_PLAY_APP_ID;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use flow::forms_bridge::flow_fixture_to_form_spec;
use flow::playbook::{render_generation_form_body, selected_generation, GenerationPlayState};
use semio_framework_plugin::{ui_text, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM: &str = "procedural2d-generate-form";
pub const PROCEDURAL2D_PLAY_BODY_GENERATE_FORM: &str = "procedural2d.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL2D_PLAY_WINDOW_GENERATE_FORM.into(),
        label: LocalizedLabel::native("Form", "Formular"),
        body_key: PROCEDURAL2D_PLAY_BODY_GENERATE_FORM.into(),
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
        capabilities: Vec::new()}
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(document: &Procedural2dSnapshot, generation: &GenerationPlayState, labels: &Procedural2dLabels) -> UiNode {
    let spec = flow_fixture_to_form_spec(&document.fixture);
    let Some(current) = selected_generation(generation) else {
        return ui_text(labels.generate_hint);
    };
    render_generation_form_body(&spec, &current.values, PROCEDURAL2D_PLAY_APP_ID, "updateGenerationValues", &current.id)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, render as render_body};

    #[test]
    async fn generate_form_hints_without_a_selected_generation() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_GENERATE_FORM).contains("Add a generation"));
    }
}
//#endregion 🧪️Tests
