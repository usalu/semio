//! ⚓ `update-pile-inputs` — atomically updates the pile-inputs facet (pile_sigma_mpa, pile_k_red, pile_n_ed_kn are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdatePileInputs {
    pub new_pile_sigma_mpa: f64,
    pub new_pile_k_red: f64,
    pub new_pile_n_ed_kn: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdatePileInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "pile-inputs", kind: "update-pile-inputs", record: "UpdatedPileInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-5 pile foundation inputs".to_string()
    }
}
//#endregion 🔖️Payload
