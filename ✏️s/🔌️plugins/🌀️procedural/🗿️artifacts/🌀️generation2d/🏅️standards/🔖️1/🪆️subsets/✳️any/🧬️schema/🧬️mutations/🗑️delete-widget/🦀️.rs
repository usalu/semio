//! 🗑️ Generation2d mutation — `DeleteWidget`: removes an id-keyed widget (captures the removed
//! widget for its inverse). Wired module name (`remove_widget`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::generation2d::diff::Generation2dDiff;
use crate::artifacts::generation2d::mutations::Generation2dMutation;
use crate::artifacts::generation2d::Generation2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use semio_framework_value_derive::{FromValue, ToValue};
//#region 🔖️DeleteWidget
/// 🗑️ `delete-widget` payload — removes the widget with `id`.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteWidget {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn delete_widget(id: String) -> Generation2dMutation {
    Generation2dMutation::DeleteWidget(DeleteWidget { id })
}

impl MutationKind<Generation2dSnapshot, Generation2dMutation> for DeleteWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    fn diff(&self, base: &Generation2dSnapshot) -> protocol::MutationOutcome<Generation2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Delete widget \"{}\"", self.id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidget
