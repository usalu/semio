//! 🔧 `change-chiller-type` payload — changes the Din16798 document's `chiller_type` (chiller type).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_chiller_type::ChangeChillerType;

//#region 🔖️ChangeChillerType
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeChillerType {
    pub new_chiller_type: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeChillerType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "chiller-type", kind: "change-chiller-type", record: "ChangedChillerType" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change chiller type to \"{}\"", self.new_chiller_type)
    }
}
//#endregion 🔖️ChangeChillerType
