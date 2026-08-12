//! 🪑 `change-es-mpa` payload — changes the En1997 document's `e_s_mpa` (soil modulus E_s [MPa]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeESMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeESMpa {
    pub new_e_s_mpa: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeESMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "es-mpa", kind: "change-es-mpa", record: "ChangedESMpa" };

    fn diff(&self, base: &En1997Snapshot) -> En1997Diff {
        crate::artifacts::en1997::mutations::change_e_s_mpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_e_s_mpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change soil modulus E_s [MPa] to {}", self.new_e_s_mpa)
    }
}
//#endregion 🔖️ChangeESMpa
