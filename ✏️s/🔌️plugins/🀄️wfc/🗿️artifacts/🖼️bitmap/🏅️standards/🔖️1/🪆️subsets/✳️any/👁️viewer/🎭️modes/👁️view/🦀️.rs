//! 👁️ Bitmap viewer — the `view` mode: the same 50/50 row the editor uses, so a document reads the
//! same way whichever surface opened it.

use crate::viewer::bitmap::modes::view::windows::{input, output};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WFC_BITMAP_MODE_VIEW: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::bitmap::create_bitmap_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_BITMAP_MODE_VIEW.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A row split, 50/50.
pub fn layout() -> WindowLayout {
    create_default_layout(&[input::WFC_BITMAP_VIEW_WINDOW_INPUT.into(), output::WFC_BITMAP_VIEW_WINDOW_OUTPUT.into()], "row", Some(&[50.0, 50.0]), Some(&["Input".into(), "Output".into()]))
}
//#endregion 🔖️Definition
