//! 🔧 `change-sfp-required-class` payload — changes the Din16798 document's `sfp_required_class` (required SFP class).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_sfp_required_class::ChangeSfpRequiredClass;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSfpRequiredClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
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
