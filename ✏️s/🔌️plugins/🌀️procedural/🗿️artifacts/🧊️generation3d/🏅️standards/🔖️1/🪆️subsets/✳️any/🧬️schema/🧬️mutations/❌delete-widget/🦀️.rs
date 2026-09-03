//! 🗑️ `delete-widget` payload — removes an id-keyed [`Widget`] from the fixture.
//!
//! Directory kept at its pre-migration `➖remove-widget` path (glue.rs still path-includes this
//! exact file; renaming the directory needs a glue.rs edit outside this facet's writable
//! boundary — see the migration report's `sharedFileRequests`).

use crate::artifacts::generation3d::diff::Generation3dDiff;
use crate::artifacts::generation3d::mutations::Generation3dMutation;
use crate::artifacts::generation3d::Generation3dSnapshot;
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteWidget
/// 🗑️ Removes the widget with `id`; the diff/inverse leaves capture the full removed payload from
/// `base` so undo is a real `create-widget`, never a sentinel.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct DeleteWidget {
    pub id: String,
}

impl protocol::MutationKind<Generation3dSnapshot, Generation3dMutation> for DeleteWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    fn diff(&self, base: &Generation3dSnapshot) -> protocol::MutationOutcome<Generation3dDiff> {
        crate::artifacts::generation3d::mutations::delete_widget::diff::diff(self, base)
    }

    fn inverse(&self, base: &Generation3dSnapshot) -> Vec<Generation3dMutation> {
        crate::artifacts::generation3d::mutations::delete_widget::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Delete widget \"{}\"", self.id)
    }

    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidget
