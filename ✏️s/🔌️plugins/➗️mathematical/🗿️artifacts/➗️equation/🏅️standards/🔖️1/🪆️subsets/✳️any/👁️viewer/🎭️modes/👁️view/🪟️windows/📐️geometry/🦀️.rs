//! 📐️ Equation viewer — the Geometry window: a read-only table of the point cloud's
//! coordinates, built on the framework's `TableWindowKit` (ticket
//! 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6) — "a table of points/coordinates"
//! fits this pane better than a node-graph kit, per this ticket's own ground truth for this packet.
//! This file itself imports nothing from the sibling editor module (the purity check forbids it
//! outright). No hull/centroid overlay (that is the editor's own `geometry_layers_json` canvas
//! render, a bespoke pure function this window deliberately does not reuse), no selection, no
//! editable `set-cell` action: a viewer has no utilities that edit and emits no mutations by
//! construction (`ViewEmit`).

use crate::{equation_geometry, EquationSnapshot};
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{BuiltNode, UiAssemblyResult};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = <TableWindowKit as WindowKit>::KIND_ID;
pub const BODY_KEY: &str = <TableWindowKit as WindowKit>::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::equation::create_equation_viewer`.
/// The read-only `window_kind()` variant (never `editable_window_kind()`, which would declare the
/// `set-cell` command a viewer cannot dispatch).
pub fn definition() -> semio_framework_plugin::WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `EquationSnapshot -> UiNode` read: one row per point, in document order — the same
/// artifact-level `equation_geometry` helper the editor's own Geometry window reads, since that
/// function lives at the ARTIFACT level (outside both surfaces), not behind the editor module.
pub fn render(document: &EquationSnapshot) -> UiAssemblyResult<BuiltNode> {
    let geometry = equation_geometry(document);
    let view = TableView { columns: vec!["#".into(), "x".into(), "y".into()], rows: geometry.points.iter().enumerate().map(|(index, point)| vec![index.to_string(), format!("{}", point.x), format!("{}", point.y)]).collect() };
    TableWindowKit::render(&view)
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
