//! 👁️ WFC 3D viewer — the `view` mode: one full-pane preview of the solved assembly.

use crate::viewer::wfc3d::modes::view::windows::preview;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WFC_3D_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_3D_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ A single full-pane preview — a read-only surface has no second half to show.
pub fn layout() -> WindowLayout {
    create_default_layout(&[preview::WFC_3D_VIEW_WINDOW.into()], "row", Some(&[100.0]), Some(&["Preview".into()]))
}
//#endregion 🔖️Definition
