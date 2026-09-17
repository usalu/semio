//! 🗂️ Generation2d play app — the generations list window (generate mode).

use crate::editor::generation2d::GENERATION2D_PLAY_APP_ID;
use semio_framework_artifact_playbook_playbook::GenerationPlayState;
use semio_framework_plugin::{BuiltNode, Locale, LocalizedLabel, SurfaceKind, Terminology, TreeWindows, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const GENERATION2D_PLAY_WINDOW_GENERATIONS: &str = "generation2d-generations";
pub const GENERATION2D_PLAY_BODY_GENERATIONS: &str = "generation2d.play.generations";
/// 🌳️ The tree-id namespace this window's roster is keyed under — also the prefix the windowed
/// generations container's node key (`{prefix}.generations`) is built from, which is what a host
/// `TreeWindowRequest` addresses.
pub const GENERATION2D_PLAY_GENERATE_PREFIX: &str = "procedural2d-play-generate";
pub const GENERATION2D_PLAY_GENERATIONS_SECTION: &str = "procedural2d-play-generate.generations";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: GENERATION2D_PLAY_WINDOW_GENERATIONS.into(),
        label: LocalizedLabel::native("Generations", "Generationen"),
        body_key: GENERATION2D_PLAY_BODY_GENERATIONS.into(),
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
pub fn render(generation: &GenerationPlayState, selected_id: Option<&str>, locale: Locale, terminology: Terminology, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    crate::generation_tree(GENERATION2D_PLAY_APP_ID, GENERATION2D_PLAY_GENERATE_PREFIX, generation, selected_id, locale, terminology, windows)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "./🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
