//! 🔧 `change-chiller-type` payload — changes the Din16798 document's `chiller_type` (chiller type).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeChillerType
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeChillerType {
    pub new_chiller_type: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeChillerType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "chiller-type", kind: "change-chiller-type", record: "ChangedChillerType" };

    async fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        crate::artifacts::din16798::mutations::change_chiller_type::diff::diff(self, base)
    }

    async fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_chiller_type::inverse::inverse(self, base)
    }

    async fn label(&self) -> String {
        format!("Change chiller type to \"{}\"", self.new_chiller_type)
    }
}
//#endregion 🔖️ChangeChillerType
