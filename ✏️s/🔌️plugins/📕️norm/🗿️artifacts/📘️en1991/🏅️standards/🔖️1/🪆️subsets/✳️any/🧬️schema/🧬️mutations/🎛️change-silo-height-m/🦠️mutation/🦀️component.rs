//! 🗼 `change-silo-height-m` — sets the En1991 silo height scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSiloHeightM {
    pub new_silo_height_m: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSiloHeightM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-height-m", kind: "change-silo-height-m", record: "ChangedSiloHeightM" };

    fn diff(&self, base: &En1991Snapshot) -> <En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change silo height to {:?}", self.new_silo_height_m)
    }
}
//#endregion 🔖️Payload
