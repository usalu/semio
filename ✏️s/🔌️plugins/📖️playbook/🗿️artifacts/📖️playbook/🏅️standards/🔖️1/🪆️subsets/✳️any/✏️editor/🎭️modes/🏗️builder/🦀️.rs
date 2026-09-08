//! 🏗️ Playbook play app — the `builder` mode: the app's only mode, a single-window Blockly-like builder.

use crate::editor::playbook::modes::builder::windows::builder as builder_window;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PLAYBOOK_PLAY_MODE_BUILDER: &str = "builder";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::playbook::create_playbook_play_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PLAYBOOK_PLAY_MODE_BUILDER.into(), label: LocalizedLabel::native("Builder", "Builder"), icon_id: "clipboard-list".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_default_layout(&[builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER.into()], "row", None, None)
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
