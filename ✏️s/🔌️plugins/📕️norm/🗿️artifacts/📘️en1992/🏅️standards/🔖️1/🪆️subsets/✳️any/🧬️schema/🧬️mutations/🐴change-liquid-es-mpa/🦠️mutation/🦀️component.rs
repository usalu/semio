//! 🔧 `change-liquid-es-mpa` payload — changes the En1992 document's `liquid_e_s_mpa` (EN 1992 input).

use crate::artifacts::en1992::diff::En1992Diff;
use crate::artifacts::en1992::mutations::En1992Mutation;
use crate::artifacts::en1992::En1992Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeLiquidESMpa
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeLiquidESMpa {
    pub new_liquid_e_s_mpa: f64,
}

impl protocol::MutationKind<En1992Snapshot, En1992Mutation> for ChangeLiquidESMpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "liquid-es-mpa", kind: "change-liquid-es-mpa", record: "ChangedLiquidESMpa" };

    fn diff(&self, base: &En1992Snapshot) -> En1992Diff {
        crate::artifacts::en1992::mutations::change_liquid_e_s_mpa::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1992Snapshot) -> Vec<En1992Mutation> {
        crate::artifacts::en1992::mutations::change_liquid_e_s_mpa::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change liquid e s mpa to {:?}", self.new_liquid_e_s_mpa)
    }
}
//#endregion 🔖️ChangeLiquidESMpa
