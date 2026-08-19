//! 🔧 `change-n-ed-kn` payload — changes the En1992 document's `n_ed_kn` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeNEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeNEdKn {
    pub new_n_ed_kn: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeNEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "n-ed-kn", kind: "change-n-ed-kn", record: "ChangedNEdKn" };

    async fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::change_n_ed_kn::diff::diff(self, base)
    }

    async fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_n_ed_kn::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change n ed kn to {:?}", self.new_n_ed_kn)
    }
}
//#endregion 🔖️ChangeNEdKn
