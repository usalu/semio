//! ✏️ EN 1997 play app — the `edit` mode: the inputs/results authoring layout.

use crate::editor::en1997::modes::edit::windows::{inputs, results};
use semio_framework_plugin::{create_default_layout, ModeDefinition, WindowLayout};

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::en1997::create_en1997_app`. Every norm app declares
/// the same single `edit` mode, so its shape is constructed once in `crate::document::app`.
pub fn definition() -> ModeDefinition {
    crate::app_surface::edit_mode_definition()
}

/// 🪟️ The app's default window layout — this mode is the app's `default_mode_id`, so its layout IS the
/// app-level `default_layout`: the inputs surface beside the results surface, 42/58.
pub fn layout() -> WindowLayout {
    create_default_layout(&[inputs::WINDOW_INPUTS.into(), results::WINDOW_RESULTS.into()], "row", Some(&[42.0, 58.0]), Some(&["Inputs".into(), "Results".into()]))
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
