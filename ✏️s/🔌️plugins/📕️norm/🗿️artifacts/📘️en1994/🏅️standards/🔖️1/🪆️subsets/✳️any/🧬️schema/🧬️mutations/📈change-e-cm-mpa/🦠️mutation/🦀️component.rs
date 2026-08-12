//! 📈 `change-e-cm-mpa` — sets the En 1994 concrete secant modulus E_cm [MPa] scalar.

use crate::artifacts::en1994::{En1994Mutation, En1994Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeECmMpa {
    pub new_e_cm_mpa: f64,
}

impl protocol::MutationKind<En1994Snapshot, En1994Mutation> for ChangeECmMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "e-cm-mpa", kind: "change-e-cm-mpa", record: "ChangedECmMpa" };

    fn diff(&self, base: &En1994Snapshot) -> <En1994Mutation as protocol::Mutation<En1994Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1994Snapshot) -> Vec<En1994Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change concrete modulus E_cm to {}", self.new_e_cm_mpa)
    }
}
//#endregion 🔖️Payload
