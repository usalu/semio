//! ✏️ BIM model editor: the `edit` mode and its default window layout: the plan beside the 3D world, the section and the schedule alongside.

use semio_s_artifact_bim_model_tests::editor::bim::terminology::BimLabels;
use semio_framework_plugin::create_default_layout;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;

pub const BIM_EDIT_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::bim::create_bim_app`.
pub fn definition() -> ModeDefinition {
    ModeDefinition {
        id: BIM_EDIT_MODE_EDIT.into(),
        label: BimLabels::localized(|labels| labels.mode_edit),
        icon_id: "pencil".into(),
        tools: Vec::new(),
        layout_id: None,
        commands: Vec::new(),
    }
}

/// 🪟️ The app's default window layout: this mode is the app's `default_mode_id`, so its layout is the app-level `default_layout`. The windows carry their own localized labels.
pub fn layout() -> WindowLayout {
    create_default_layout(&[semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::plan::WINDOW_KIND_ID.into(), semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::world::WINDOW_KIND_ID.into(), semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::section::WINDOW_KIND_ID.into(), semio_s_artifact_bim_model_kit::editor::bim::modes::edit::windows::schedule::WINDOW_KIND_ID.into()], "row", Some(&[28.0, 38.0, 18.0, 16.0]), None)
}
//#endregion 🔖️Definition

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

