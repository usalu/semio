//! 🔧 `change-f-yk` payload — changes the En1992 document's `f_yk` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeFYk
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeFYk {
    pub new_f_yk: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeFYk {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "f-yk", kind: "change-f-yk", record: "ChangedFYk" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_f_yk::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_f_yk::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change f yk to {:?}", self.new_f_yk)
    }
}
//#endregion 🔖️ChangeFYk
