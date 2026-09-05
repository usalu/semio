//! 🔧 `change-years-since-inspection` payload — changes the Din16798 document's `years_since_inspection` (years since last inspection).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeYearsSinceInspection
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeYearsSinceInspection {
    pub new_years_since_inspection: u32,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeYearsSinceInspection {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "years-since-inspection", kind: "change-years-since-inspection", record: "ChangedYearsSinceInspection" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change years since last inspection to {}", self.new_years_since_inspection)
    }
}
//#endregion 🔖️ChangeYearsSinceInspection
