//! 📊️ EN 1999 viewer — the Report window: a read-only table of every computed compliance
//! check, built from the same subset `🧬️schema/💡️inferences::evaluate` pure snapshot→`CheckReport`
//! function the editor's own results window uses — this file imports nothing from the sibling editor
//! surface (`policyViewerPurityBreaches` forbids it outright). Uses the framework `TableWindowKit`
//! (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.6), the right tool for
//! compliance/report data per the contract's own guidance.

use crate::artifacts::en1999::En1999Snapshot;
// 🚧️ SDK GAP: `WindowKit`/`TableWindowKit`/`TableView` are not yet in `semio_framework_plugin`'s
// curated crate-root re-export list — only reachable through `app`, same class of gap as `Dialect`.
use semio_framework_plugin::app::{TableView, TableWindowKit, WindowKit};
use semio_framework_plugin::{UiNode, WindowKindDefinition};

//#region 🔖️Constants
pub const WINDOW_KIND_ID: &str = TableWindowKit::KIND_ID;
pub const BODY_KEY: &str = TableWindowKit::KIND_ID;
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::en1999::create_en1999_viewer`. Read-only variant
/// — a viewer never declares `editable_window_kind()`'s `set-cell` command.
pub fn definition() -> WindowKindDefinition {
    TableWindowKit::window_kind()
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Pure `En1999Snapshot -> UiNode` read: recomputes the compliance report straight off the document
/// (the same pure inference the editor's results window renders through `NormHost`), then tables it.
pub fn render(document: &En1999Snapshot) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let report = crate::artifacts::en1999::standards::v1::subsets::any::schema::inferences::evaluate(document);
    TableWindowKit::render(&TableView { columns: crate::app_surface::report_table_columns(), rows: crate::app_surface::report_table_rows(&report) })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    fn definition_declares_the_shared_table_window_kind() {
        let def = definition();
        assert_eq!(def.id, TableWindowKit::KIND_ID);
    }

    #[semio_framework_async_macros::async_test]
    fn render_produces_a_node_for_the_default_document() {
        let document = En1999Snapshot::default();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
