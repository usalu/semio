//! ✏️ Generation2d play app — the `edit` mode: the default two-window authoring layout (flow graph +
//! preview).
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as store;

use crate::windows::flow;
use crate::windows::preview;
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::ToolRef;
use semio_framework_plugin::WindowLayout;

pub const GENERATION2D_PLAY_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::generation2d::create_generation2d_app`. References
/// the `previewEval` run because this mode mounts the edit preview that starts it — a declared tool no
/// mode references is refused by `build_definition`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: GENERATION2D_PLAY_MODE_EDIT.into(),
        label: LocalizedLabel::native("Edit", "Bearbeiten"),
        icon_id: "pencil".into(),
        tools: vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(semio_s_artifact_procedural_2d_preview_eval::PREVIEW_EVAL_TOOL_ID))],
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[flow::GENERATION2D_PLAY_WINDOW_MAIN.into(), preview::GENERATION2D_PLAY_WINDOW_PREVIEW.into()], "row", Some(&[55.0, 45.0]), Some(&["Main".into(), "Preview".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod windows {
    #[path = "🪟️windows/🕸️flow/🦀️.rs"]
    pub mod flow;
    #[path = "🪟️windows/👁️preview/🦀️.rs"]
    pub mod preview;
}
