//! 🗂️ Procedural2d play app — the generations list window (generate mode).

use crate::editor::procedural2d::PROCEDURAL2D_PLAY_APP_ID;
use flow::playbook::{render_generations_tree, GenerationPlayState};
use semio_framework_plugin::{Locale, LocalizedLabel, SurfaceKind, Terminology, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL2D_PLAY_WINDOW_GENERATIONS: &str = "procedural2d-generations";
pub const PROCEDURAL2D_PLAY_BODY_GENERATIONS: &str = "procedural2d.play.generations";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL2D_PLAY_WINDOW_GENERATIONS.into(),
        label: LocalizedLabel::native("Generations", "Generationen"),
        body_key: PROCEDURAL2D_PLAY_BODY_GENERATIONS.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "sparkles".into(),
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
pub async fn render(generation: &GenerationPlayState, locale: Locale, terminology: Terminology) -> UiNode {
    render_generations_tree(PROCEDURAL2D_PLAY_APP_ID, "procedural2d-play-generate", &generation.generations, generation.selected_generation_id.as_deref(), locale, terminology)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural2d::testkit::{app, render as render_body};

    #[semio_framework_async_macros::async_test]
    async fn generate_mode_renders_surfaces() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL2D_PLAY_BODY_GENERATIONS).contains("addGeneration"));
    }
}
//#endregion 🧪️Tests
