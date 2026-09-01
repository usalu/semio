//! 🗼 `update-tower-inputs` — atomically updates the tower-inputs facet (tower_wind_factor, tower_n_ed_kn are validated together for one EN 1993 check, never one-field-at-a-time).



use crate::artifacts::en1993::{En1993Diff, En1993Mutation, En1993Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct UpdateTowerInputs {
    pub new_tower_wind_factor: f64,
    pub new_tower_n_ed_kn: f64,
}

impl protocol::MutationKind<En1993Snapshot, En1993Mutation> for UpdateTowerInputs {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "update", entity: "tower-inputs", kind: "update-tower-inputs", record: "UpdatedTowerInputs" };

    fn diff(&self, base: &En1993Snapshot) -> protocol::MutationOutcome<<En1993Mutation as protocol::Mutation<En1993Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1993Snapshot) -> Vec<En1993Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        "Update EN 1993-3-1 tower buckling inputs".to_string()
    }
}
//#endregion 🔖️Payload
