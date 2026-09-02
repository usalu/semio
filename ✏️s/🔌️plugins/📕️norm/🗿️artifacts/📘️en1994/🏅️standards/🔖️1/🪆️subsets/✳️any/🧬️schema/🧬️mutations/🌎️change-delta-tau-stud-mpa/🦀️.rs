//! 📉 `change-delta-tau-stud-mpa` — sets the En 1994 stud fatigue shear stress range Δτ [MPa] scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeDeltaTauStudMpa {
    pub new_delta_tau_stud_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeDeltaTauStudMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "delta-tau-stud-mpa", kind: "change-delta-tau-stud-mpa", record: "ChangedDeltaTauStudMpa" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change stud fatigue stress range Δτ to {}", self.new_delta_tau_stud_mpa)
    }
}
//#endregion 🔖️Payload
