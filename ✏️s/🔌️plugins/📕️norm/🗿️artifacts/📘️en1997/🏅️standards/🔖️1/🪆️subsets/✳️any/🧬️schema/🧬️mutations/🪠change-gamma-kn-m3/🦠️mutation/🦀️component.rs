//! 🪠 `change-gamma-kn-m3` payload — changes the En1997 document's `gamma_kn_m3` (soil unit weight [kN/m3]).

use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::En1997Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeGammaKnM3
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeGammaKnM3 {
    pub new_gamma_kn_m3: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeGammaKnM3 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "gamma-kn-m3", kind: "change-gamma-kn-m3", record: "ChangedGammaKnM3" };

    fn diff(&self, base: &En1997Snapshot) -> En1997Diff {
        crate::artifacts::en1997::mutations::change_gamma_kn_m3::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        crate::artifacts::en1997::mutations::change_gamma_kn_m3::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change soil unit weight [kN/m3] to {}", self.new_gamma_kn_m3)
    }
}
//#endregion 🔖️ChangeGammaKnM3
