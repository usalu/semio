//! 🔧 `change-humidification-required-kg-h` payload — changes the Din16798 document's `humidification_required_kg_h` (required humidification rate).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_humidification_required_kg_h::ChangeHumidificationRequiredKgH;

//#region 🔖️ChangeHumidificationRequiredKgH
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHumidificationRequiredKgH {
    pub new_humidification_required_kg_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHumidificationRequiredKgH {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "humidification-required-kg-h", kind: "change-humidification-required-kg-h", record: "ChangedHumidificationRequiredKgH" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change required humidification rate to {}", self.new_humidification_required_kg_h)
    }
}
//#endregion 🔖️ChangeHumidificationRequiredKgH
