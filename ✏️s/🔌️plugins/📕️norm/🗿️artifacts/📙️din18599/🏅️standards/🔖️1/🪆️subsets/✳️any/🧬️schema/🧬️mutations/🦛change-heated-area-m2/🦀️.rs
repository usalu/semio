//! 🦛 `change-heated-area-m2` payload — changes the Din18599 document's `heated_area_m2` (heated floor area [m2]).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::mutations::change_heated_area_m2::ChangeHeatedAreaM2;

//#region 🔖️ChangeHeatedAreaM2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHeatedAreaM2 {
    pub new_heated_area_m2: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeHeatedAreaM2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "heated-area-m2", kind: "change-heated-area-m2", record: "ChangedHeatedAreaM2" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change heated floor area [m2] to {}", self.new_heated_area_m2)
    }
}
//#endregion 🔖️ChangeHeatedAreaM2
