//! 🐬 `change-silo-n-rd-kn` payload — changes the En1998 document's `silo_n_rd_kn` (silo axial resistance N_Rd [kN]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSiloNRdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSiloNRdKn {
    pub new_silo_n_rd_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeSiloNRdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-n-rd-kn", kind: "change-silo-n-rd-kn", record: "ChangedSiloNRdKn" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_silo_n_rd_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_silo_n_rd_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change silo axial resistance N_Rd [kN] to {}", self.new_silo_n_rd_kn)
    }
}
//#endregion 🔖️ChangeSiloNRdKn
