//! 🎨️ Lowpoly play app — the `paint` mode: a two-window layout (Model + UV) for paint-focused editing.
//! Reuses the edit mode's `model` window kind alongside its own `uv` window (sibling `🪟️windows/🖼️uv/`).

use crate::apps::lowpoly::modes::edit::windows::model;
use crate::apps::lowpoly::modes::paint::windows::uv;
use semio_framework_plugin::{create_default_layout, create_named_layout, LocalizedLabel, ModeDefinition, NamedLayout};

pub const LOWPOLY_PLAY_MODE_PAINT: &str = "paint";
pub const LOWPOLY_PLAY_LAYOUT_PAINT: &str = "lowpoly-paint";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::lowpoly::create_lowpoly_app`. `layout_id` binds
/// this mode directly to its named layout (equivalent to the pre-taxonomy builder's
/// `.mode_layout("paint", "lowpoly-paint")` post-hoc call — `ModeDefinition::layout_id` already carries
/// the same information through `mode_definition_to_spec`, so setting it here needs no separate
/// passthrough).
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: LOWPOLY_PLAY_MODE_PAINT.into(), label: LocalizedLabel::native("Paint", "Malen"), icon_id: "paintbrush".into(), tools: Vec::new(), layout_id: Some(LOWPOLY_PLAY_LAYOUT_PAINT.into()), commands: Vec::new() }
}

/// 🪟️ The two-window paint layout (Model 60% / UV 40%), registered as an app-level named layout.
pub fn layout() -> NamedLayout {
    create_named_layout(
        LOWPOLY_PLAY_LAYOUT_PAINT,
        "Paint",
        create_default_layout(&[model::LOWPOLY_PLAY_WINDOW_MAIN.into(), uv::LOWPOLY_PLAY_WINDOW_UV.into()], "row", Some(&[60.0, 40.0]), Some(&["Model".into(), "UV".into()])),
        "builtin",
        Some("paintbrush".into()),
        None,
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_named_layout_lists_both_paint_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(model::LOWPOLY_PLAY_WINDOW_MAIN) && json.contains(uv::LOWPOLY_PLAY_WINDOW_UV), "layout must reference both window kinds: {json}");
    }

    #[test]
    fn the_mode_binds_directly_to_its_named_layout() {
        assert_eq!(definition().layout_id.as_deref(), Some(LOWPOLY_PLAY_LAYOUT_PAINT));
    }
}
//#endregion 🧪️Tests
