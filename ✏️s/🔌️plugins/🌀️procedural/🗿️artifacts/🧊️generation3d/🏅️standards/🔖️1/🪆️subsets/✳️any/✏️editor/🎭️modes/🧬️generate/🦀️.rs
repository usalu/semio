//! 🧬️ Generation3d play app — the `generate` mode: generations list + input form + output preview.

use crate::editor::generation3d::modes::generate::windows::{form, generations, preview};
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout, ToolRef};

pub const GENERATION_3D_PLAY_MODE_GENERATE: &str = "generate";
pub const GENERATION_3D_PLAY_LAYOUT_GENERATE: &str = "generation3d-generate";

//#region 🔖️Definition
/// ⏯️ This mode mounts the generate preview, which starts the same `previewEval` run the edit preview
/// does — so it references the tool for the same reason `🎭️modes/✏️edit` does.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: GENERATION_3D_PLAY_MODE_GENERATE.into(),
        label: LocalizedLabel::native("Generate", "Generieren"),
        icon_id: "sparkles".into(),
        tools: vec![semio_framework::io::resolve_ready(ToolRef::new(crate::preview_eval::PREVIEW_EVAL_TOOL_ID))],
        layout_id: Some(GENERATION_3D_PLAY_LAYOUT_GENERATE.into()),
        commands: Vec::new(),
    }
}

pub fn layout() -> NamedLayout {
    create_named_layout(
        GENERATION_3D_PLAY_LAYOUT_GENERATE,
        "Generate",
        create_default_layout(
            &[generations::GENERATION_3D_PLAY_WINDOW_GENERATIONS.into(), form::GENERATION_3D_PLAY_WINDOW_GENERATE_FORM.into(), preview::GENERATION_3D_PLAY_WINDOW_GENERATE_PREVIEW.into()],
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
