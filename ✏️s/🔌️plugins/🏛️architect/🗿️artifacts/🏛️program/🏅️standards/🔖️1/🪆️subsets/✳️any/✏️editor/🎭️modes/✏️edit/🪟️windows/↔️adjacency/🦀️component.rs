//! ↔️ Architect adjacency window — the signature adjacency matrix surface: a triangle glyph strip
//! beside a lower-triangle pair tree, each pair cycling its `AdjacencyKind` on activation.

use crate::artifacts::program::standards::v1::subsets::any::schema::inferences::{adjacency_matrix, detect_adjacency_conflicts};
use crate::artifacts::program::ProgramSnapshot;
use crate::editor::architect::architect_action;
use crate::editor::architect::chrome::{adjacency_kind_label, element_label, stack_row, tree_item, tree_item_with_action, tree_node, tree_section};
use crate::editor::architect::config::ArchitectConfig;
use semio_framework_plugin::{ui_stack_vertical, ui_text, Label, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};
use serde_json::json;

//#region 🔖️Constants
pub const ARCHITECT_WINDOW_ADJACENCY: &str = "architect-adjacency";
pub const ARCHITECT_BODY_ADJACENCY: &str = "architect.adjacency";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🏛️ Stitched into the app manifest by `crate::editor::architect::create_architect_app`.
pub async fn definition() -> WindowKindDefinition {
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
/// @emoji 🔺️ Signature adjacency matrix — triangle glyph strip plus lower-triangle pair rows.
pub async fn render(program: &ProgramSnapshot, cfg: &ArchitectConfig) -> UiNode {
    let matrix = adjacency_matrix(program);
    let n = matrix.element_ids.len();
    if n == 0 {
        return ui_text(Label::data("Add program elements to edit adjacencies."));
    }

    let mut glyph_rows = Vec::new();
    let mut pair_sections = Vec::new();

    glyph_rows.push(ui_text(Label::data(" ")));
    pair_sections.push(tree_section("architect-adjacency.headers", Some("Columns".into()), matrix.element_ids.iter().enumerate().map(|(index, id)| tree_item(format!("architect-adjacency.col.{index}"), element_label(program, id))).collect()));

    for row in 1..n {
        let row_id = &matrix.element_ids[row];
        let glyph = "▲️".repeat(row);
        glyph_rows.push(ui_text(Label::data(glyph)));

        let mut items = Vec::new();
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
            let label = format!("{} ↔ {} [{kind_label}]", element_label(program, col_id), element_label(program, row_id));
            items.push(tree_item_with_action(
                format!("architect-adjacency.pair.{col_id}-{row_id}"),
                label,
                None,
                architect_action(
                    "setAdjacencyKind",
                    Some(json!({
                        "elementAId": col_id,
                        "elementBId": row_id,
                        "cycle": true
                    })),
                ),
            ));
        }

        pair_sections.push(tree_section(format!("architect-adjacency.row.{row}"), Some(element_label(program, row_id)), items));
    }

    let conflicts = detect_adjacency_conflicts(program);
    if !conflicts.is_empty() {
        pair_sections.push(tree_section(
            "architect-adjacency.conflicts",
            Some(format!("Conflicts ({})", conflicts.len())),
            conflicts.iter().map(|conflict| tree_item(format!("architect-adjacency.conflict.{}", conflict.adjacency_a_id), &conflict.message)).collect(),
        ));
    }

    stack_row("architect-adjacency.matrix", vec![ui_stack_vertical(glyph_rows), tree_node(pair_sections)])
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::program::{empty_plugin, sample_plugin};

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_adjacency_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, ARCHITECT_BODY_ADJACENCY);
        assert!(matches!(definition.surface_kind, SurfaceKind::Canvas2d));
    }

    #[semio_framework_async_macros::async_test]
    async fn the_matrix_renders_a_triangle_strip_with_element_labels() {
        let json = serde_json::to_string(&render(&sample_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains('▲'));
        assert!(json.contains("Reception"));
    }

    #[semio_framework_async_macros::async_test]
    async fn an_empty_program_renders_the_placeholder() {
        let json = serde_json::to_string(&render(&empty_plugin(), &ArchitectConfig::default())).expect("json");
        assert!(json.contains("Add program elements"));
    }
}
//#endregion 🧪️Tests
