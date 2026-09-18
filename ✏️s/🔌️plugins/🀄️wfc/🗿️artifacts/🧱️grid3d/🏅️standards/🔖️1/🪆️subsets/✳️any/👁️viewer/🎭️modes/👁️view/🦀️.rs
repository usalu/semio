//! 👁️ `s.wfc.grid3d` viewer — the `view` mode: one full-pane preview of the solved grid.

use crate::viewer::grid3d::modes::view::windows::preview;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const GRID3D_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::grid3d::create_grid3d_viewer`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GRID3D_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A viewer has nothing to author, so the solved grid takes the whole pane.
pub fn layout() -> WindowLayout {
    create_default_layout(&[preview::WINDOW_KIND_ID.into()], "row", None, Some(&["Preview".into()]))
}
//#endregion 🔖️Definition
