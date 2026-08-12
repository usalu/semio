//! 🔧 `change-f-v-k` payload — changes the En1995 document's `f_v_k` (EN 1995 input).

use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::En1995Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFVK
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFVK {
    pub new_f_v_k: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeFVK {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fvk", kind: "change-fvk", record: "ChangedFVK" };

    fn diff(&self, base: &En1995Snapshot) -> En1995Diff {
        crate::artifacts::en1995::mutations::change_f_v_k::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        crate::artifacts::en1995::mutations::change_f_v_k::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change f v k to {:?}", self.new_f_v_k)
    }
}
//#endregion 🔖️ChangeFVK
