//! 🌀 `change-cs` — sets the En1991 size factor c_s scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeCS {
    pub new_c_s: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeCS {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cs", kind: "change-cs", record: "ChangedCs" };

    fn diff(&self, base: &En1991Snapshot) -> <En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change size factor c_s to {:?}", self.new_c_s)
    }
}
//#endregion 🔖️Payload
