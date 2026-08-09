//! 🗂️ Generate-mode window — the generation list.

use crate::apps::flow::config::FlowConfig;
use crate::apps::flow::FLOW_PLAY_APP_ID;
use crate::playbook::render_generations_tree;
use semio_framework_plugin::{Locale, LocalizedLabel, SurfaceKind, Terminology, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const FLOW_PLAY_WINDOW_GENERATIONS: &str = "flow-generations";
pub const FLOW_PLAY_BODY_GENERATIONS: &str = "flow.play.generations";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: FLOW_PLAY_WINDOW_GENERATIONS.into(),
        label: LocalizedLabel::native("Generations", "Generationen"),
        body_key: FLOW_PLAY_BODY_GENERATIONS.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "sparkles".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        document_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(config: &FlowConfig, locale: Locale, terminology: Terminology) -> UiNode {
    let generation = config.generation();
    render_generations_tree(FLOW_PLAY_APP_ID, "flow-play-generate", &generation.generations, generation.selected_generation_id.as_deref(), locale, terminology)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::flow::testkit::{flow_app, render as render_body};

    #[test]
    fn the_empty_generation_list_still_offers_the_add_action() {
        let mut app = flow_app();
        assert!(render_body(&mut app, FLOW_PLAY_BODY_GENERATIONS).contains("addGeneration"));
    }
}
//#endregion 🧪️Tests
