//! 🔍️ Architect play app — the `review` mode: validation, trace and impact review over the same
//! window kinds the `✏️edit` mode lays out (window kinds are app-scoped in the manifest, so this mode
//! declares no windows and no layout of its own).

use semio_framework_plugin::{LocalizedLabel, ModeDefinition};

pub const ARCHITECT_MODE_REVIEW: &str = "review";

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub async fn definition() -> ModeDefinition {
    ModeDefinition { id: ARCHITECT_MODE_REVIEW.into(), label: LocalizedLabel::native("Review", "Überprüfen"), icon_id: "search".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn the_mode_declares_no_layout_of_its_own() {
        let definition = definition();
        assert_eq!(definition.id, ARCHITECT_MODE_REVIEW);
        assert!(definition.layout_id.is_none());
    }
}
//#endregion 🧪️Tests
