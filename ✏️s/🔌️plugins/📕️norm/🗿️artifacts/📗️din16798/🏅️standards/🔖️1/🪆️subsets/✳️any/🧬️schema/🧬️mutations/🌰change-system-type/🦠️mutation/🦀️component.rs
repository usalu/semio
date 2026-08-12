//! 🔧 `change-system-type` payload — changes the Din16798 document's `system_type` (ventilation system type).

use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::Din16798Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeSystemType
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSystemType {
    pub new_system_type: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeSystemType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "system-type", kind: "change-system-type", record: "ChangedSystemType" };

    fn diff(&self, base: &Din16798Snapshot) -> Din16798Diff {
        crate::artifacts::din16798::mutations::change_system_type::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        crate::artifacts::din16798::mutations::change_system_type::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ventilation system type to \"{}\"", self.new_system_type)
    }
}
//#endregion 🔖️ChangeSystemType
