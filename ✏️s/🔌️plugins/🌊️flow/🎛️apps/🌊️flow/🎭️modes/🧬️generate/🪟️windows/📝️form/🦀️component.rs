//! 📝️ Generate-mode window — the input form for the active generation.

use crate::apps::flow::config::FlowConfig;
use crate::apps::flow::terminology::flow_play_labels;
use crate::apps::flow::FLOW_PLAY_APP_ID;
use crate::artifacts::flow::FlowFixture;
use flow::forms_bridge::flow_fixture_to_form_spec;
use crate::playbook::{render_generation_form_body, selected_generation};
use semio_framework_plugin::{ui_text, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_GENERATE_FORM: &str = "flow-generate-form";
pub const FLOW_PLAY_BODY_GENERATE_FORM: &str = "flow.play.generate-form";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_GENERATE_FORM.into(),
        label: LocalizedLabel::native("Form", "Formular"),
        body_key: FLOW_PLAY_BODY_GENERATE_FORM.into(),
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
pub fn render(fixture: &FlowFixture, config: &FlowConfig) -> UiNode {
    let spec = flow_fixture_to_form_spec(fixture);
    let generation = config.generation();
    let Some(active) = selected_generation(&generation) else {
        return ui_text(flow_play_labels(config).generation_needed);
    };
    render_generation_form_body(&spec, &active.values, FLOW_PLAY_APP_ID, "updateGenerationValues", &active.id)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{flow_app, render as render_body};

    #[test]
    fn without_a_generation_the_form_shows_the_placeholder_copy() {
        let mut app = flow_app();
        assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATE_FORM).contains("Add a generation"));
    }
}
//#endregion 🧪️Tests
