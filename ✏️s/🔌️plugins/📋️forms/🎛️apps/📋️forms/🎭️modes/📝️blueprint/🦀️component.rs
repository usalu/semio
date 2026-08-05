//! 📝️ Forms play app — the `blueprint` mode: the app's only mode, a two-window authoring layout (the
//! playbook builder + the Try wizard preview).

use crate::apps::forms::modes::blueprint::windows::{builder, try_wizard};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const FORMS_PLAY_MODE_BLUEPRINT: &str = "blueprint";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::apps::forms::create_forms_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: FORMS_PLAY_MODE_BLUEPRINT.into(), label: LocalizedLabel::native("Blueprint", "Entwurf"), icon_id: "cad-shape".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[builder::FORMS_PLAY_WINDOW_BLUEPRINT.into(), try_wizard::FORMS_PLAY_WINDOW_TRY.into()], "row", Some(&[50.0, 50.0]), Some(&["Blueprint".into(), "Try".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_layout_lists_both_blueprint_windows() {
        let json = serde_json::to_string(&layout()).expect("layout json");
        assert!(json.contains(builder::FORMS_PLAY_WINDOW_BLUEPRINT) && json.contains(try_wizard::FORMS_PLAY_WINDOW_TRY), "layout must reference both window kinds: {json}");
    }
}
//#endregion 🧪️Tests
