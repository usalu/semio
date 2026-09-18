//! ✏️ Bitmap editor — the `edit` mode: the authored sample on the left, the inferred output on the
//! right, split evenly. Nothing pane-specific lives here; each window binds its own definition and
//! render in its own file.

use crate::editor::bitmap::modes::edit::windows::{input, output};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WFC_BITMAP_MODE_EDIT: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::bitmap::create_bitmap_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_BITMAP_MODE_EDIT.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A row split, 50/50: the problem and its answer at the same scale.
pub fn layout() -> WindowLayout {
    create_default_layout(&[input::WFC_BITMAP_WINDOW_INPUT.into(), output::WFC_BITMAP_WINDOW_OUTPUT.into()], "row", Some(&[50.0, 50.0]), Some(&["Input".into(), "Output".into()]))
}
//#endregion 🔖️Definition
