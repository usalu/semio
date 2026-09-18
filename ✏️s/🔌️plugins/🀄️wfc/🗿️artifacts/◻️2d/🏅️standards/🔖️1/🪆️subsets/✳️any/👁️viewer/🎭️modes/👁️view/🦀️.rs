//! 👁️ WFC 2D viewer — the `view` mode: one full-pane board preview.

use crate::viewer::wfc2d::modes::view::windows::preview;
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WFC_2D_VIEW_MODE_ID: &str = "view";

//#region 🔖️Definition
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_2D_VIEW_MODE_ID.into(), label: LocalizedLabel::native("View", "Ansicht"), icon_id: "eye".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane board window — a read-only surface has nothing to put beside it.
pub fn layout() -> WindowLayout {
    create_default_layout(&[preview::WFC_2D_VIEW_WINDOW.into()], "row", None, Some(&["Board".into()]))
}
//#endregion 🔖️Definition
