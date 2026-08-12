//! 📉 `change-silo-mu` — sets the En1991 silo friction coefficient scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSiloMu {
    pub new_silo_mu: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSiloMu {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-mu", kind: "change-silo-mu", record: "ChangedSiloMu" };

    fn diff(&self, base: &En1991Snapshot) -> <En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change silo friction coefficient to {:?}", self.new_silo_mu)
    }
}
//#endregion 🔖️Payload
