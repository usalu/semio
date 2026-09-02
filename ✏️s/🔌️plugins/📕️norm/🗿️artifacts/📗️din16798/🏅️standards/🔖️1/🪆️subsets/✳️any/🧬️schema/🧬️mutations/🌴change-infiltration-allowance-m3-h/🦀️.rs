//! 🔧 `change-infiltration-allowance-m3-h` payload — changes the Din16798 document's `infiltration_allowance_m3_h` (infiltration allowance).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_infiltration_allowance_m3_h::ChangeInfiltrationAllowanceM3H;

//#region 🔖️ChangeInfiltrationAllowanceM3H
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeInfiltrationAllowanceM3H {
    pub new_infiltration_allowance_m3_h: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeInfiltrationAllowanceM3H {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "infiltration-allowance-m3-h", kind: "change-infiltration-allowance-m3-h", record: "ChangedInfiltrationAllowanceM3H" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change infiltration allowance to {}", self.new_infiltration_allowance_m3_h)
    }
}
//#endregion 🔖️ChangeInfiltrationAllowanceM3H
