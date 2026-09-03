//! ➕ `create-widget` payload — brings a new id-keyed [`Widget`] into existence at an insertion
//! index (FINAL-state, per `📓️derivation-rules.md` rule 3's addressing convention).

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use flow::Widget;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️CreateWidget
/// ➕ Full initial payload for a new widget, placed at `index` if no widget with the same id
/// already exists (upsert-by-id, matching `apply_widgets_diff`'s own dedupe rule).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateWidget {
    pub index: usize,
    pub widget: Widget,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for CreateWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "widget", kind: "create-widget", record: "CreatedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::create_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::create_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Create widget \"{}\"", crate::artifacts::generation3d::widget_id(&self.widget))
    }

    fn target(&self) -> Vec<String> {
        vec![crate::artifacts::generation3d::widget_id(&self.widget).to_string()]
    }
}
//#endregion 🔖️CreateWidget
