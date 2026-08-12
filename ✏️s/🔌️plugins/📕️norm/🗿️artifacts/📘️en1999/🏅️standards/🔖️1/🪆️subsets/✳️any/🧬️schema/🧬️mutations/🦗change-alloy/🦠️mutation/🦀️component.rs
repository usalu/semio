//! 🦗 `change-alloy` payload — changes the En1999 document's `alloy` (aluminium alloy designation).

use crate::artifacts::en1999::diff::En1999Diff;
use crate::artifacts::en1999::mutations::En1999Mutation;
use crate::artifacts::en1999::En1999Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeAlloy
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeAlloy {
    pub new_alloy: String,
}

impl protocol::MutationKind<En1999Snapshot, En1999Mutation> for ChangeAlloy {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "alloy", kind: "change-alloy", record: "ChangedAlloy" };

    fn diff(&self, base: &En1999Snapshot) -> En1999Diff {
        crate::artifacts::en1999::mutations::change_alloy::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1999Snapshot) -> Vec<En1999Mutation> {
        crate::artifacts::en1999::mutations::change_alloy::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change aluminium alloy designation to \"{}\"", self.new_alloy)
    }
}
//#endregion 🔖️ChangeAlloy
