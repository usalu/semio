//! 🗂️ Procedural3d play app — the generations list window (generate mode).

use crate::editor::procedural3d::PROCEDURAL_3D_PLAY_APP_ID;
use flow::playbook::GenerationPlayState;
use semio_framework_plugin::{BuiltNode, Locale, LocalizedLabel, SurfaceKind, Terminology, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS: &str = "procedural3d-generations";
pub const PROCEDURAL_3D_PLAY_BODY_GENERATIONS: &str = "procedural.play.generations";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: PROCEDURAL_3D_PLAY_WINDOW_GENERATIONS.into(),
        label: LocalizedLabel::native("Generations", "Generationen"),
        body_key: PROCEDURAL_3D_PLAY_BODY_GENERATIONS.into(),
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
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(generation: &GenerationPlayState, locale: Locale, terminology: Terminology) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    crate::generation_tree(PROCEDURAL_3D_PLAY_APP_ID, "procedural3d-play-generate", generation, locale, terminology)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::procedural3d::testkit::{app, render as render_body};

    #[test]
    fn generate_mode_renders_surfaces() {
        let mut app = app();
        assert!(render_body(&mut app, PROCEDURAL_3D_PLAY_BODY_GENERATIONS).contains("addGeneration"));
    }
}
//#endregion 🧪️Tests
