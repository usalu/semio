//! 🔥 `change-fire-curve` — sets the En1991 fire curve scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFireCurve {
    pub new_fire_curve: crate::artifacts::en1991::part_1_2::FireCurve,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeFireCurve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-curve", kind: "change-fire-curve", record: "ChangedFireCurve" };

    async fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    async fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    async fn label(&self) -> String {
        format!("Change fire curve to {:?}", self.new_fire_curve)
    }
}
//#endregion 🔖️Payload
