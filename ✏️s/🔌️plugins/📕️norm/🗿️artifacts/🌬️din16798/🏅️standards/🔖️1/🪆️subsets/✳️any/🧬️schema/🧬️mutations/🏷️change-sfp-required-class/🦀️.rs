//! 🔧 `change-sfp-required-class` payload — changes the Din16798 document's `sfp_required_class` (required SFP class).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
//#region 🔖️ChangeSfpRequiredClass
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeSfpRequiredClass {
    pub new_sfp_required_class: u8,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeSfpRequiredClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "sfp-required-class", kind: "change-sfp-required-class", record: "ChangedSfpRequiredClass" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change required SFP class to {}", self.new_sfp_required_class)
    }
}
//#endregion 🔖️ChangeSfpRequiredClass
