//! 🔧 `change-humidification-provided-kg-h` payload — changes the Din16798 document's `humidification_provided_kg_h` (provided humidification rate).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHumidificationProvidedKgH
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHumidificationProvidedKgH {
    pub new_humidification_provided_kg_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHumidificationProvidedKgH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidification-provided-kg-h", kind: "change-humidification-provided-kg-h", record: "ChangedHumidificationProvidedKgH" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_humidification_provided_kg_h::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_humidification_provided_kg_h::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change provided humidification rate to {}", self.new_humidification_provided_kg_h)
    }
}
//#endregion 🔖️ChangeHumidificationProvidedKgH
