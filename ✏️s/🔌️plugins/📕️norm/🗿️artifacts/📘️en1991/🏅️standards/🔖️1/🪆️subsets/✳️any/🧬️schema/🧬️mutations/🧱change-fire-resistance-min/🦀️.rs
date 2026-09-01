//! ⏱️ `change-fire-resistance-min` — sets the En1991 fire resistance scalar.


use crate::artifacts::en1991::{En1991Diff, En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeFireResistanceMin {
    pub new_fire_resistance_min: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeFireResistanceMin {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fire-resistance-min", kind: "change-fire-resistance-min", record: "ChangedFireResistanceMin" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change fire resistance to {:?}", self.new_fire_resistance_min)
    }
}
//#endregion 🔖️Payload
