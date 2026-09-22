//! ✏️ `s.wfc.grid3d` editor — the `edit` mode: the problem on the left, the inferred solution on the
//! right, split 50/50. Nothing pane-specific lives here; each window binds its own definition and
//! render in its own file.

use crate::editor::grid3d::modes::edit::tools;
use crate::editor::grid3d::modes::edit::windows::{grid, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, WindowLayout};

pub const GRID3D_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::grid3d::create_grid3d_editor`. The mode
/// lists the interactive fill tool whose run the framework drives.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GRID3D_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: vec![semio_framework::io::resolve_ready(semio_framework_plugin::ToolRef::new(tools::fill::TOOL_ID))], layout_id: None, commands: Vec::new() }
}

/// 🪟️ Grid left, preview right, equal halves.
pub fn layout() -> WindowLayout {
    create_default_layout(&[grid::WINDOW_KIND_ID.into(), preview::WINDOW_KIND_ID.into()], "row", Some(&[50.0, 50.0]), Some(&["Grid".into(), "Preview".into()]))
}
//#endregion 🔖️Definition
