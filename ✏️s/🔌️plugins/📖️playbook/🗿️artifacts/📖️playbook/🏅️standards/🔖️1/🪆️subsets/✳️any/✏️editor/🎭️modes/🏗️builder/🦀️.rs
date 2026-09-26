//! 🏗️ Playbook play app — the `builder` mode: the app's only mode, a single-window Blockly-like builder.

use crate::editor::playbook::modes::builder::windows::activity as activity_window;
use crate::editor::playbook::modes::builder::windows::builder as builder_window;
use crate::editor::playbook::modes::builder::windows::changes as changes_window;
use crate::editor::playbook::modes::builder::windows::source as source_window;
use crate::editor::playbook::modes::builder::windows::steps as steps_window;
use semio_framework_plugin::{create_tab_stack_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const PLAYBOOK_PLAY_MODE_BUILDER: &str = "builder";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::playbook::create_playbook_play_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: PLAYBOOK_PLAY_MODE_BUILDER.into(), label: LocalizedLabel::native("Builder", "Builder"), icon_id: "clipboard-list".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`.
pub fn layout() -> WindowLayout {
    create_tab_stack_layout(
        &[
            builder_window::PLAYBOOK_PLAY_WINDOW_BUILDER.into(),
            steps_window::PLAYBOOK_PLAY_WINDOW_STEPS.into(),
            changes_window::PLAYBOOK_PLAY_WINDOW_CHANGES.into(),
            activity_window::PLAYBOOK_PLAY_WINDOW_ACTIVITY.into(),
            source_window::PLAYBOOK_PLAY_WINDOW_SOURCE.into(),
        ],
        None,
    )
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
