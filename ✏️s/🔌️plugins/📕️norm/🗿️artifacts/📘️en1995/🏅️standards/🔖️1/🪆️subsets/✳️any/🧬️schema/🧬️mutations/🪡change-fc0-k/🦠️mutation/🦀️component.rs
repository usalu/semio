//! 🔧 `change-f-c-0-k` payload — changes the En1995 document's `f_c_0_k` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFC0K
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFC0K {
    pub new_f_c_0_k: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeFC0K {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fc0-k", kind: "change-fc0-k", record: "ChangedFC0K" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        crate::artifacts::en1995::mutations::change_f_c_0_k::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_f_c_0_k::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change f c 0 k to {:?}", self.new_f_c_0_k)
    }
}
//#endregion 🔖️ChangeFC0K
