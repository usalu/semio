//! 💧 `change-silo-hydraulic-radius-m` — sets the En1991 silo hydraulic radius scalar.

use crate::artifacts::en1991::{En1991Mutation, En1991Snapshot};
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChangeSiloHydraulicRadiusM {
    pub new_silo_hydraulic_radius_m: f64,
}

impl protocol::MutationKind<En1991Snapshot, En1991Mutation> for ChangeSiloHydraulicRadiusM {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "silo-hydraulic-radius-m", kind: "change-silo-hydraulic-radius-m", record: "ChangedSiloHydraulicRadiusM" };

    fn diff(&self, base: &En1991Snapshot) -> protocol::MutationOutcome<<En1991Mutation as protocol::Mutation<En1991Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &En1991Snapshot) -> Vec<En1991Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change silo hydraulic radius to {:?}", self.new_silo_hydraulic_radius_m)
    }
}
//#endregion 🔖️Payload
