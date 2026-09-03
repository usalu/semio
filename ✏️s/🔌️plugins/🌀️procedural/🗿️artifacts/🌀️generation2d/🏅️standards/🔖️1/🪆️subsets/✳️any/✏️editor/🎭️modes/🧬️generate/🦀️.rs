//! 🧬️ Generation2d play app — the `generate` mode: generations list + input form + output preview.

use crate::editor::generation2d::modes::generate::windows::{form, generations, preview};
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout};

pub const GENERATION2D_PLAY_MODE_GENERATE: &str = "generate";
pub const GENERATION2D_PLAY_LAYOUT_GENERATE: &str = "generation2d-generate";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GENERATION2D_PLAY_MODE_GENERATE.into(), label: LocalizedLabel::native("Generate", "Generieren"), icon_id: "sparkles".into(), tools: Vec::new(), layout_id: Some(GENERATION2D_PLAY_LAYOUT_GENERATE.into()), commands: Vec::new() }
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
mod tests {
    use super::*;

    #[test]
    fn the_generate_layout_lists_all_three_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(generations::GENERATION2D_PLAY_WINDOW_GENERATIONS));
        assert!(json.contains(form::GENERATION2D_PLAY_WINDOW_GENERATE_FORM));
        assert!(json.contains(preview::GENERATION2D_PLAY_WINDOW_GENERATE_PREVIEW));
    }
}
//#endregion 🧪️Tests
