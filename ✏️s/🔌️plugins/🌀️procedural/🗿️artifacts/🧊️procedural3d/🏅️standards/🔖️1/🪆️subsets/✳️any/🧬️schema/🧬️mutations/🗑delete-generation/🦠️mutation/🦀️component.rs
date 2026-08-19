//! 🗑️ `delete-generation` payload — removes an id-keyed [`FormGeneration`] entry.

use crate::artifacts::procedural3d::diff::Procedural3dDiff;
use crate::artifacts::procedural3d::mutations::Procedural3dMutation;
use crate::artifacts::procedural3d::Procedural3dSnapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️DeleteGeneration
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteGeneration {
    pub id: String}

impl protocol::MutationKind<Procedural3dSnapshot, Procedural3dMutation> for DeleteGeneration {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "generation", kind: "delete-generation", record: "DeletedGeneration" };

    async fn diff(&self, base: &Procedural3dSnapshot) -> protocol::MutationOutcome<Procedural3dDiff> {
        crate::artifacts::procedural3d::mutations::delete_generation::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Procedural3dSnapshot) -> Vec<Procedural3dMutation> {
        crate::artifacts::procedural3d::mutations::delete_generation::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Delete generation \"{}\"", self.id)
    }

    async fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
//#endregion 🔖️DeleteGeneration
