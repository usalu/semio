//! 🔨 `change-v-ed-per-stud-kn` — sets the En 1994 design shear force per stud V_Ed [kN] scalar.


use crate::artifacts::en1994::{En1994Diff, En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeVEdPerStudKn {
    pub new_v_ed_per_stud_kn: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeVEdPerStudKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-ed-per-stud-kn", kind: "change-v-ed-per-stud-kn", record: "ChangedVEdPerStudKn" };

    fn diff(&self, base: &En1994Snapshot) -> protocol::MutationOutcome<<En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change shear per stud V_Ed to {}", self.new_v_ed_per_stud_kn)
    }
}
//#endregion 🔖️Payload
