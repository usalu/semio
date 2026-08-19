//! 🧺 `change-dfm` payload — changes the En1997 document's `d_f_m` (founding depth D_f [m]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDFM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDFM {
    pub new_d_f_m: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeDFM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "dfm", kind: "change-dfm", record: "ChangedDFM" };

    async fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        crate::artifacts::en1997::mutations::change_d_f_m::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_d_f_m::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change founding depth D_f [m] to {}", self.new_d_f_m)
    }
}
//#endregion 🔖️ChangeDFM
