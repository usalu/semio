//! ✏️ WFC 2D editor — the `edit` mode: the slot graph beside its solved preview, split 50/50.
//! Nothing pane-specific lives here; each window binds its own definition and render in its own file.

use crate::editor::wfc2d::modes::edit::tools::fill;
use crate::editor::wfc2d::modes::edit::windows::{graph, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WFC_2D_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::wfc2d::create_wfc2d_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_2D_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: vec![fill::TOOL_ID.into()], layout_id: None, commands: Vec::new() }
}

/// 🪟️ Graph left, preview right, equal halves — the problem and its solution side by side.
pub fn layout() -> WindowLayout {
    create_default_layout(&[graph::WFC_GRAPH_WINDOW.into(), preview::WFC_2D_PREVIEW_WINDOW.into()], "row", Some(&[50.0, 50.0]), Some(&["Graph".into(), "Preview".into()]))
}
//#endregion 🔖️Definition
