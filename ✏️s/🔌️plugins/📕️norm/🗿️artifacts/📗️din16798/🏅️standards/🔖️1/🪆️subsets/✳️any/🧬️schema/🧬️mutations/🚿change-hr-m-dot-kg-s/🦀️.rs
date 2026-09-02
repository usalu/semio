//! 🔧 `change-hr-m-dot-kg-s` payload — changes the Din16798 document's `hr_m_dot_kg_s` (heat recovery mass flow rate).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_hr_m_dot_kg_s::ChangeHrMDotKgS;

//#region 🔖️ChangeHrMDotKgS
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeHrMDotKgS {
    pub new_hr_m_dot_kg_s: f64,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeHrMDotKgS {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hr-m-dot-kg-s", kind: "change-hr-m-dot-kg-s", record: "ChangedHrMDotKgS" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change heat recovery mass flow rate to {}", self.new_hr_m_dot_kg_s)
    }
}
//#endregion 🔖️ChangeHrMDotKgS
