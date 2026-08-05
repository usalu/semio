//! 🧬️ Flow play app — the `generate` mode: explore parameter generations of the current fixture across
//! three windows (generation list, input form, evaluated preview).

use crate::apps::flow::modes::generate::windows::{form, generations, preview};
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout};

pub const FLOW_PLAY_MODE_GENERATE: &str = "generate";
pub const FLOW_PLAY_LAYOUT_GENERATE: &str = "flow-generate";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: FLOW_PLAY_MODE_GENERATE.into(), label: LocalizedLabel::native("Generate", "Generieren"), icon_id: "sparkles".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The three-window generate layout, registered as an app-level named layout.
pub fn layout() -> NamedLayout {
    create_named_layout(
        FLOW_PLAY_LAYOUT_GENERATE,
        "Generate",
        create_default_layout(
            &[generations::FLOW_PLAY_WINDOW_GENERATIONS.into(), form::FLOW_PLAY_WINDOW_GENERATE_FORM.into(), preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW.into()],
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
    fn the_named_layout_lists_all_three_generate_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        for window in [generations::FLOW_PLAY_WINDOW_GENERATIONS, form::FLOW_PLAY_WINDOW_GENERATE_FORM, preview::FLOW_PLAY_WINDOW_GENERATE_PREVIEW] {
            assert!(json.contains(window), "layout must reference {window}: {json}");
        }
    }
}
//#endregion 🧪️Tests
