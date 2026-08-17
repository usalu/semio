//! 🧱 `change-self-weight-material` — sets the En1991 self-weight material scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSelfWeightMaterial {
    pub new_self_weight_material: String,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSelfWeightMaterial {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "self-weight-material", kind: "change-self-weight-material", record: "ChangedSelfWeightMaterial" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change self-weight material to \"{}\"", self.new_self_weight_material)
    }
}
//#endregion 🔖️Payload
