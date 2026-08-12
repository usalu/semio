//! 🔧 `change-cellar-ventilation-m3-h` payload — changes the Din16798 document's `cellar_ventilation_m3_h` (cellar ventilation air flow).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeCellarVentilationM3H
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeCellarVentilationM3H {
    pub new_cellar_ventilation_m3_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeCellarVentilationM3H {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "cellar-ventilation-m3-h", kind: "change-cellar-ventilation-m3-h", record: "ChangedCellarVentilationM3H" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_cellar_ventilation_m3_h::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_cellar_ventilation_m3_h::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change cellar ventilation air flow to {}", self.new_cellar_ventilation_m3_h)
    }
}
//#endregion 🔖️ChangeCellarVentilationM3H
