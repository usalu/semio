//! 🦏 `change-use-class` payload — changes the Din18599 document's `use_class` (building use class).

use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::Din18599Snapshot;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeUseClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUseClass {
    pub new_use_class: crate::artifacts::din18599::UseClass,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeUseClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "use-class", kind: "change-use-class", record: "ChangedUseClass" };

    fn diff(&self, base: &Din18599Snapshot) -> Din18599Diff {
        crate::artifacts::din18599::mutations::change_use_class::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        crate::artifacts::din18599::mutations::change_use_class::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change building use class to {:?}", self.new_use_class)
    }
}
//#endregion 🔖️ChangeUseClass
