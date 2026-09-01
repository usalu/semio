//! 🔧 `change-ida-class` payload — changes the Din16798 document's `ida_class` (indoor air quality class).


use crate::artifacts::din16798::Din16798Snapshot;
use crate::artifacts::din16798::diff::Din16798Diff;
use crate::artifacts::din16798::mutations::Din16798Mutation;
use crate::artifacts::din16798::mutations::change_ida_class::ChangeIdaClass;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeIdaClass
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeIdaClass {
    pub new_ida_class: String,
}

impl protocol::MutationKind<Din16798Snapshot, Din16798Mutation> for ChangeIdaClass {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "ida-class", kind: "change-ida-class", record: "ChangedIdaClass" };

    fn diff(&self, base: &Din16798Snapshot) -> protocol::MutationOutcome<Din16798Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din16798Snapshot) -> Vec<Din16798Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change indoor air quality class to \"{}\"", self.new_ida_class)
    }
}
//#endregion 🔖️ChangeIdaClass
