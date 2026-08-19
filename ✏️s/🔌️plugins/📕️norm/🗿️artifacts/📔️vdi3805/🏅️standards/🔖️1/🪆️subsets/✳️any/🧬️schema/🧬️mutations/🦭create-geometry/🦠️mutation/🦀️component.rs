//! 🧊️ `create-geometry` — brings a new id-keyed parametric geometry definition into existence.

use crate::artifacts::vdi3805::{ParametricGeometry, Vdi3805Mutation, Vdi3805Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateGeometry {
    pub geometry: ParametricGeometry,
}

impl protocol::MutationKind<Vdi3805Snapshot, Vdi3805Mutation> for CreateGeometry {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "geometry", kind: "create-geometry", record: "CreatedGeometry" };

    async fn diff(&self, base: &Vdi3805Snapshot) -> protocol::MutationOutcome<<Vdi3805Mutation as protocol::Mutation<Vdi3805Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &Vdi3805Snapshot) -> Vec<Vdi3805Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Create geometry \"{}\"", self.geometry.id)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.geometry.id.clone()]
    }
}
//#endregion 🔖️Payload
