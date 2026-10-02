//! 📝️ Design the form beside its interactive preview.

use crate::editor::forms::modes::blueprint::windows::{builder, try_wizard};
use semio_framework_plugin::create_default_layout;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const FORMS_PLAY_MODE_BLUEPRINT: &str = "blueprint";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::forms::create_forms_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: FORMS_PLAY_MODE_BLUEPRINT.into(), label: LocalizedLabel::native("Design", "Entwurf"), icon_id: "cad-shape".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[builder::FORMS_PLAY_WINDOW_BLUEPRINT.into(), try_wizard::FORMS_PLAY_WINDOW_TRY.into()], "row", Some(&[50.0, 50.0]), None)
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
