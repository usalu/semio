//! 🏔️ `change-en-a-gr` payload — changes the En1998 document's `en_a_gr` (reference ground acceleration a_gr).

use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::En1998Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeEnAGr
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeEnAGr {
    pub new_en_a_gr: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeEnAGr {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "en-a-gr", kind: "change-en-a-gr", record: "ChangedEnAGr" };

    fn diff(&self, base: &En1998Snapshot) -> En1998Diff {
        crate::artifacts::en1998::mutations::change_en_a_gr::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        crate::artifacts::en1998::mutations::change_en_a_gr::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change reference ground acceleration a_gr to {}", self.new_en_a_gr)
    }
}
//#endregion 🔖️ChangeEnAGr
