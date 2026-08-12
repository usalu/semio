//! ⚙️ `change-fy-mpa` — sets the En 1994 steel yield strength f_y [MPa] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeFYMpa {
    pub new_f_y_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeFYMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "fy-mpa", kind: "change-fy-mpa", record: "ChangedFYMpa" };

    fn diff(&self, base: &En1994Snapshot) -> <En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change steel yield strength f_y to {}", self.new_f_y_mpa)
    }
}
//#endregion 🔖️Payload
