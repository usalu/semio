//! 🗑️ Procedural2d mutation — `DeleteWidget`: removes an id-keyed widget (captures the removed
//! widget for its inverse). Wired module name (`remove_widget`) is a leftover of the pre-semantic
//! generic slot this triad was repurposed from — see `sharedFileRequests` in this ticket's wave2
//! report for the glue.rs rename that would align the directory/module with the verb.

use crate::artifacts::procedural2d::diff::Procedural2dDiff;
use crate::artifacts::procedural2d::mutations::Procedural2dMutation;
use crate::artifacts::procedural2d::Procedural2dSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteWidget
/// 🗑️ `delete-widget` payload — removes the widget with `id`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeleteWidget {
    pub id: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub async fn delete_widget(id: String) -> Procedural2dMutation {
    Procedural2dMutation::DeleteWidget(DeleteWidget { id })
}

impl MutationKind<Procedural2dSnapshot, Procedural2dMutation> for DeleteWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    async fn diff(&self, base: &Procedural2dSnapshot) -> protocol::MutationOutcome<Procedural2dDiff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Delete widget \"{}\"", self.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidget
