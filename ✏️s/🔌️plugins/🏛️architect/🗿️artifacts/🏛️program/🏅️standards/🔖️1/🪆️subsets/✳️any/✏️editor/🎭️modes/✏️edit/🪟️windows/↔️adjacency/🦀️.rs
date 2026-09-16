//! ↔️ Architect adjacency window — the signature adjacency matrix surface: a triangle glyph strip
//! beside a lower-triangle pair tree, each pair cycling its `AdjacencyKind` on activation.

use crate::editor::architect::chrome::{adjacency_kind_label, element_label};
use crate::editor::architect::ui_label;
use crate::editor::architect::{architect_action, ui_value_bool, ui_value_map, ui_value_text};
use crate::standards::v1::subsets::any::schema::inferences::{adjacency_matrix, detect_adjacency_conflicts};
use crate::ProgramSnapshot;
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, tree_window_item, BuiltNode, LocalizedLabel, PanelTreeBuilder, PluginAssemblyError, SurfaceKind, TreeWindows, UiAssemblyResult, WindowKindDefinition, WindowOptions};
use semio_framework_ui_contract::HasBase;

#[path = "🎚️config/🦀️.rs"]
pub mod config;

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_ADJACENCY: &str = "architect-adjacency";
pub const ARCHITECT_BODY_ADJACENCY: &str = "architect.adjacency";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: ARCHITECT_WINDOW_ADJACENCY.into(),
        label: LocalizedLabel::native("Adjacency", "Adjazenz"),
        body_key: ARCHITECT_BODY_ADJACENCY.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "grid-3x3".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        // 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the matrix's cells cycle
        // their `AdjacencyKind` directly on click (`setAdjacencyKind`) — there is no genuine
        // select-then-act step here to model as an interaction domain (see the crate's migration
        // notes), so this window declares none.
        interactions: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🔺️ One cell of a matrix row: the leading triangle glyph, or one lower-triangle pair whose click
/// cycles its `AdjacencyKind`. Pre-resolved so the row's children are an entry slice a window indexes.
enum MatrixCell {
    Glyph(String),
    Pair { col_id: String, row_id: String, label: String },
}

fn matrix_cell_row(row: usize, cell: &MatrixCell) -> UiAssemblyResult<BuiltNode> {
    match cell {
        MatrixCell::Glyph(glyph) => tree_item_desc(format!("architect-adjacency.row.{row}.glyph"), ui_label(glyph)?, None),
        MatrixCell::Pair { col_id, row_id, label } => {
            let args = ui_value_map([("cycle", ui_value_bool(true)), ("elementAId", ui_value_text(col_id)?), ("elementBId", ui_value_text(row_id)?)])?;
            tree_item_with_action(format!("architect-adjacency.pair.{col_id}-{row_id}"), ui_label(label)?, None, architect_action("setAdjacencyKind", Some(args))?)
        }
    }
}

/// @emoji 🔺️ Signature adjacency matrix — triangle glyph strip plus lower-triangle pair rows.
///
/// 🪟️ One windowed `rows` section spans the whole lower triangle and each matrix row is a windowed
/// group row over its own cells, so a program with hundreds of elements streams instead of fanning a
/// section per row straight into the fixed root child list.
pub fn render(program: &ProgramSnapshot, cfg: &config::ArchitectAdjacencyWindowConfig, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let matrix = adjacency_matrix(program);
    let n = matrix.element_ids.len();
    if n == 0 {
        return crate::editor::architect::ui_node(semio_framework_ui_contract::text(ui_label("Add program elements to edit adjacencies.")?), "architect-adjacency.empty");
    }
    let headers: Vec<_> = matrix.element_ids.iter().enumerate().collect();
    let rows: Vec<(usize, Vec<MatrixCell>)> = (1..n)
        .map(|row| {
            let row_id = &matrix.element_ids[row];
            let mut cells = vec![MatrixCell::Glyph("▲️".repeat(row))];
            for col in 0..row {
                let col_id = &matrix.element_ids[col];
                let cell = &matrix.cells[row][col];
                if let Some(filter) = &cfg.adjacency_kind_filter {
                    match cell {
                        Some(existing) if &existing.kind != filter => continue,
                        None => continue,
                        _ => {}
                    }
                }
                let kind_label = cell.as_ref().map_or_else(|| "—".into(), |existing| adjacency_kind_label(&existing.kind).to_string());
                cells.push(MatrixCell::Pair {
                    col_id: col_id.to_string(),
                    row_id: row_id.to_string(),
                    label: format!("{} ↔ {} [{kind_label}]", element_label(program, col_id), element_label(program, row_id)),
                });
            }
            (row, cells)
        })
        .collect();
    let conflicts = detect_adjacency_conflicts(program);
    let mut tree = PanelTreeBuilder::new("architect-adjacency")?
        .window_section(windows, "architect-adjacency.headers", Some(ui_label("Columns")?), true, &headers, |(index, id)| {
            tree_item_desc(format!("architect-adjacency.col.{index}"), ui_label(element_label(program, id))?, None)
        })?
        .window_section(windows, "architect-adjacency.rows", Some(ui_label("Pairs")?), true, &rows, |(row, cells)| {
            let id = format!("architect-adjacency.row.{row}");
            let group = semio_framework_ui_contract::tree_item(ui_label(element_label(program, &matrix.element_ids[*row]))?)
                .try_id(&id)
                .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "architect adjacency row admission failed"))?;
            tree_window_item(windows, group, &id, true, cells, |cell| matrix_cell_row(*row, cell))
        })?;
    if !conflicts.is_empty() {
        tree = tree.window_section(windows, "architect-adjacency.conflicts", Some(ui_label(format!("Conflicts ({})", conflicts.len()))?), true, &conflicts, |conflict| {
            tree_item_desc(format!("architect-adjacency.conflict.{}", conflict.adjacency_a_id), ui_label(&conflict.message)?, None)
        })?;
    }
    tree.build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
