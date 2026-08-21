//! 📐️ Mathematical viewer — the Geometry window: a read-only table of the point cloud's
//! coordinates, built on the framework's `TableWindowKit` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6) — "a table of points/coordinates"
//! fits this pane better than a node-graph kit, per this ticket's own ground truth for this packet.
//! This file itself imports nothing from the sibling editor module (the purity check forbids it
//! outright). No hull/centroid overlay (that is the editor's own `geometry_layers_json` canvas
//! render, a bespoke pure function this window deliberately does not reuse), no selection, no
//! editable `set-cell` action: a viewer has no utilities that edit and emits no mutations by
//! construction (`ViewEmit`).

use crate::artifacts::mathematical::{mathematical_geometry, MathematicalSnapshot};
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::UiNode;

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = <TableWindowKit as WindowKit>::KIND_ID;
pub const BODY_KEY: &str = <TableWindowKit as WindowKit>::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::mathematical::create_mathematical_viewer`.
/// The read-only `window_kind()` variant (never `editable_window_kind()`, which would declare the
/// `set-cell` command a viewer cannot dispatch).
pub async fn definition() -> semio_framework_plugin::WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `MathematicalSnapshot -> UiNode` read: one row per point, in document order — the same
/// artifact-level `mathematical_geometry` helper the editor's own Geometry window reads, since that
/// function lives at the ARTIFACT level (outside both surfaces), not behind the editor module.
pub async fn render(document: &MathematicalSnapshot) -> UiNode {
    let geometry = mathematical_geometry(document);
    let view = TableView { columns: vec!["#".into(), "x".into(), "y".into()], rows: geometry.points.iter().enumerate().map(|(index, point)| vec![index.to_string(), format!("{}", point.x), format!("{}", point.y)]).collect() };
    TableWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_table_kind() {
        let definition = definition();
        assert_eq!(definition.id, WINDOW_KIND_ID);
    }

    #[semio_framework_async_macros::async_test]
    async fn render_produces_a_table_scene_with_one_row_per_point() {
        let document = MathematicalSnapshot::default();
        let points = mathematical_geometry(&document).points;
        assert!(!points.is_empty());
        let json = serde_json::to_string(&render(&document)).unwrap();
        assert!(json.contains("table"));
        for point in &points {
            assert!(json.contains(&format!("{}", point.x)), "row for x={} missing from rendered table: {json}", point.x);
        }
    }
}
//#endregion 🧪️Tests
