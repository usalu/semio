//! 🗑️ `delete-widget` payload — removes an id-keyed [`Widget`] from the fixture.
//!
//! Directory kept at its pre-migration `➖remove-widget` path (glue.rs still path-includes this
//! exact file; renaming the directory needs a glue.rs edit outside this facet's writable
//! boundary — see the migration report's `sharedFileRequests`).

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteWidget
/// 🗑️ Removes the widget with `id`; the diff/inverse leaves capture the full removed payload from
/// `base` so undo is a real `create-widget`, never a sentinel.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteWidget {
    pub id: String,
}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for DeleteWidget {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "widget", kind: "delete-widget", record: "DeletedWidget" };

    async fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::delete_widget::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::delete_widget::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Delete widget \"{}\"", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteWidget
