//! ✏️ WFC 3D editor — the `edit` mode: the slot graph beside its solved 3d preview, split 50/50.
//! Nothing pane-specific lives here; each window binds its own definition and render in its own file.

use crate::editor::wfc3d::modes::edit::windows::{graph, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const WFC_3D_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::wfc3d::create_wfc3d_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: WFC_3D_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Graph left, preview right, equal halves — the problem and its solution side by side.
pub fn layout() -> WindowLayout {
    create_default_layout(&[graph::WFC_GRAPH_WINDOW.into(), preview::WFC_3D_PREVIEW_WINDOW.into()], "row", Some(&[50.0, 50.0]), Some(&["Graph".into(), "Preview".into()]))
}
//#endregion 🔖️Definition
