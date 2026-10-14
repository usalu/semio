//! 🧬️ Generation2d play app — the `generate` mode: generations list + input form + output preview.
#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as store;

use crate::windows::form;
use crate::windows::generations;
use crate::windows::preview;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::create_named_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::NamedLayout;
use semio_framework_plugin::ToolRef;

pub const GENERATION2D_PLAY_MODE_GENERATE: &str = "generate";
pub const GENERATION2D_PLAY_LAYOUT_GENERATE: &str = "generation2d-generate";

//#region 🔖️Definition
/// ⏯️ This mode mounts the generate preview, which starts the same `previewEval` run the edit preview
/// does — so it references the tool for the same reason `🎭️modes/✏️edit` does.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: GENERATION2D_PLAY_MODE_GENERATE.into(),
        label: LocalizedLabel::native("Generate", "Generieren"),
        icon_id: "sparkles".into(),
        tools: vec![::semio_framework_async::poll::resolve_ready(ToolRef::new(semio_s_artifact_procedural_2d_preview_eval::PREVIEW_EVAL_TOOL_ID))],
        layout_id: Some(GENERATION2D_PLAY_LAYOUT_GENERATE.into()),
        commands: Vec::new(),
    }
}

pub fn layout() -> NamedLayout {
    create_named_layout(
        GENERATION2D_PLAY_LAYOUT_GENERATE,
        "Generate",
        create_default_layout(
            &[generations::GENERATION2D_PLAY_WINDOW_GENERATIONS.into(), form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM.into(), preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW.into()],
            "row",
            Some(&[22.0, 43.0, 35.0]),
            Some(&["Generations".into(), "Form".into(), "Preview".into()]),
        ),
        "builtin",
        Some("sparkles".into()),
        None,
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod windows {
    #[path = "🪟️windows/📝️form/🦀️.rs"]
    pub mod form;
    #[path = "🪟️windows/🗂️generations/🦀️.rs"]
    pub mod generations;
    #[path = "🪟️windows/👁️preview/🦀️.rs"]
    pub mod preview;
}
