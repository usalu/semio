//! 🪝 `change-crane-class` — sets the En1991 crane class scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeCraneClass {
    pub new_crane_class: String,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeCraneClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "crane-class", kind: "change-crane-class", record: "ChangedCraneClass" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change crane class to \"{}\"", self.new_crane_class)
    }
}
//#endregion 🔖️Payload
