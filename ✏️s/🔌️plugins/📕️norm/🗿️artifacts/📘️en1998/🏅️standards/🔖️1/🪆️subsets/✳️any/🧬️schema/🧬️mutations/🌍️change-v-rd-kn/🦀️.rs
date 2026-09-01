//! 🌍️ `change-v-rd-kn` payload — changes the En1998 document's `v_rd_kn` (design shear resistance V_Rd [kN]).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_v_rd_kn::ChangeVRdKn;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeVRdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeVRdKn {
    pub new_v_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeVRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-rd-kn", kind: "change-v-rd-kn", record: "ChangedVRdKn" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change design shear resistance V_Rd [kN] to {}", self.new_v_rd_kn)
    }
}
//#endregion 🔖️ChangeVRdKn
