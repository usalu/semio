//! 🔧 `change-bridge-sigma-c-mpa` payload — changes the En1992 document's `bridge_sigma_c_mpa` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeBridgeSigmaCMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeBridgeSigmaCMpa {
    pub new_bridge_sigma_c_mpa: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeBridgeSigmaCMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "bridge-sigma-c-mpa", kind: "change-bridge-sigma-c-mpa", record: "ChangedBridgeSigmaCMpa" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_bridge_sigma_c_mpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_bridge_sigma_c_mpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change bridge sigma c mpa to {:?}", self.new_bridge_sigma_c_mpa)
    }
}
//#endregion 🔖️ChangeBridgeSigmaCMpa
