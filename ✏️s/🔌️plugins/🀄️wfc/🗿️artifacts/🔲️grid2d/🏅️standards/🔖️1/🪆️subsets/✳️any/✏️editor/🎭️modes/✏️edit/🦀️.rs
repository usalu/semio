//! ✏️ 2D-grid editor — the `edit` mode: the grid board beside the solved preview, 50/50, exactly
//! the two-pane shape every sibling `wfc` artifact uses. Nothing pane-specific lives here; each
//! window binds its own definition and render in its own file.

use crate::editor::grid2d::modes::edit::windows::{grid, preview};
use semio_framework_plugin::{create_default_layout, LocalizedLabel, ModeDefinition, UtilityCategory, UtilityDefinition, WindowLayout};

pub const GRID2D_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::grid2d::create_grid2d_editor`.
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: GRID2D_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ The grid board beside the inferred preview.
pub fn layout() -> WindowLayout {
    create_default_layout(&[grid::WINDOW_KIND_ID.into(), preview::WINDOW_KIND_ID.into()], "row", Some(&[50.0, 50.0]), Some(&["Grid".into(), "Preview".into()]))
}

/// 🧰️ The three pointer utilities the grid pane arms. A declared utility no window references is
/// refused by the plugin builder, so the grid window's own `utilities` list names all three.
pub fn utilities() -> Vec<UtilityDefinition> {
    vec![
        UtilityDefinition { category: Some(UtilityCategory::Selection), ..UtilityDefinition::new(grid::UTILITY_SELECT, LocalizedLabel::native("Select", "Auswählen"), "mouse-pointer") },
        UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(grid::UTILITY_PIN, LocalizedLabel::native("Pin", "Fixieren"), "lock") },
        UtilityDefinition { category: Some(UtilityCategory::Utilities), ..UtilityDefinition::new(grid::UTILITY_MASK, LocalizedLabel::native("Mask", "Ausblenden"), "eraser") },
    ]
}
//#endregion 🔖️Definition

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
