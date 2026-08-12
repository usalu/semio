//! 🔧 `change-dwelling-ventilation-m3-h` payload — changes the Din16798 document's `dwelling_ventilation_m3_h` (dwelling ventilation air flow).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeDwellingVentilationM3H
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeDwellingVentilationM3H {
    pub new_dwelling_ventilation_m3_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeDwellingVentilationM3H {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "dwelling-ventilation-m3-h", kind: "change-dwelling-ventilation-m3-h", record: "ChangedDwellingVentilationM3H" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_dwelling_ventilation_m3_h::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_dwelling_ventilation_m3_h::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change dwelling ventilation air flow to {}", self.new_dwelling_ventilation_m3_h)
    }
}
//#endregion 🔖️ChangeDwellingVentilationM3H
