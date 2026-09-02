//! ⛰️ `change-en-ground-type` payload — changes the En1998 document's `en_ground_type` (EN ground type).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_en_ground_type::ChangeEnGroundType;

//#region 🔖️ChangeEnGroundType
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeEnGroundType {
    pub new_en_ground_type: String,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeEnGroundType {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "en-ground-type", kind: "change-en-ground-type", record: "ChangedEnGroundType" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change EN ground type to \"{}\"", self.new_en_ground_type)
    }
}
//#endregion 🔖️ChangeEnGroundType
