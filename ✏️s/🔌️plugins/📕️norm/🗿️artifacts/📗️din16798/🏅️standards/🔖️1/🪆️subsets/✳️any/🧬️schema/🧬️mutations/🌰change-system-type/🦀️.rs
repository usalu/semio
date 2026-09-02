//! 🔧 `change-system-type` payload — changes the Din16798 document's `system_type` (ventilation system type).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_system_type::ChangeSystemType;

//#region 🔖️ChangeSystemType
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSystemType {
    pub new_system_type: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeSystemType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "system-type", kind: "change-system-type", record: "ChangedSystemType" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ventilation system type to \"{}\"", self.new_system_type)
    }
}
//#endregion 🔖️ChangeSystemType
