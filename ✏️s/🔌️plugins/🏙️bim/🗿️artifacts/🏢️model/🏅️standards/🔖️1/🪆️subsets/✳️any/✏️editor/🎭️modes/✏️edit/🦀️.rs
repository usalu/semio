//! ✏️ BIM model editor: the `edit` mode and its default window layout: the plan beside the 3D world, the section and the schedule alongside.

use crate::editor::bim::modes::edit::windows::{plan, schedule, section, world};
use crate::editor::bim::terminology::BimLabels;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_ui_locale::LocalizedLabel;

pub const BIM_EDIT_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: BIM_EDIT_MODE_EDIT.into(),
        label: LocalizedLabel::native(BimLabels::NATIVE_EN.mode_edit.as_str(), BimLabels::NATIVE_DE.mode_edit.as_str()),
        icon_id: "pencil".into(),
        tools: Vec::new(),
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ The app's default window layout: this mode is the app's `default_mode_id`, so its layout is the app-level `default_layout`. The windows carry their own localized labels.
pub fn layout() -> WindowLayout {
    create_default_layout(&[plan::WINDOW_KIND_ID.into(), world::WINDOW_KIND_ID.into(), section::WINDOW_KIND_ID.into(), schedule::WINDOW_KIND_ID.into()], "row", Some(&[28.0, 38.0, 18.0, 16.0]), None)
}
//#endregion 🔖️Definition

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
