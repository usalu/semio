//! 🐝 `change-retrofit-ed-kn` payload — changes the En1998 document's `retrofit_e_d_kn` (retrofit demand E_d [kN]).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeRetrofitEDKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeRetrofitEDKn {
    pub new_retrofit_e_d_kn: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeRetrofitEDKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "retrofit-ed-kn", kind: "change-retrofit-ed-kn", record: "ChangedRetrofitEDKn" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_retrofit_e_d_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_retrofit_e_d_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change retrofit demand E_d [kN] to {}", self.new_retrofit_e_d_kn)
    }
}
//#endregion 🔖️ChangeRetrofitEDKn
