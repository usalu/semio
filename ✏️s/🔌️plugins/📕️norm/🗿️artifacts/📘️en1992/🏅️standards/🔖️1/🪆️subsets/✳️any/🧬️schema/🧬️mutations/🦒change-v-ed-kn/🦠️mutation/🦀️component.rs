//! 🔧 `change-v-ed-kn` payload — changes the En1992 document's `v_ed_kn` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeVEdKn
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeVEdKn {
    pub new_v_ed_kn: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeVEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-ed-kn", kind: "change-v-ed-kn", record: "ChangedVEdKn" };

    fn diff(&self, base: &En1992Snapshot) -> protocol::MutationOutcome<En1992Diff> {
        crate::artifacts::en1992::mutations::change_v_ed_kn::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_v_ed_kn::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change v ed kn to {:?}", self.new_v_ed_kn)
    }
}
//#endregion 🔖️ChangeVEdKn
