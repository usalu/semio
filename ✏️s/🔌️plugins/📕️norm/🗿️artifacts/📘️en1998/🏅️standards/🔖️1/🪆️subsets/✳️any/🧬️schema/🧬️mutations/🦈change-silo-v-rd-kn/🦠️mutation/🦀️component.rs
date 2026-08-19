//! 🦈 `change-silo-v-rd-kn` payload — changes the En1998 document's `silo_v_rd_kn` (silo shear resistance V_Rd [kN]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSiloVRdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSiloVRdKn {
    pub new_silo_v_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSiloVRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-v-rd-kn", kind: "change-silo-v-rd-kn", record: "ChangedSiloVRdKn" };

    async fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        crate::artifacts::en1998::mutations::change_silo_v_rd_kn::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_silo_v_rd_kn::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change silo shear resistance V_Rd [kN] to {}", self.new_silo_v_rd_kn)
    }
}
//#endregion 🔖️ChangeSiloVRdKn
