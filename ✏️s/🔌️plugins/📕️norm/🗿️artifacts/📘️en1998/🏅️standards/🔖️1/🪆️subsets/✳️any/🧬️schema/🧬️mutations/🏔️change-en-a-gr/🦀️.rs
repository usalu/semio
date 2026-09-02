//! 🏔️ `change-en-a-gr` payload — changes the En1998 document's `en_a_gr` (reference ground acceleration a_gr).


use crate::artifacts::en1998::En1998Snapshot;
use crate::artifacts::en1998::diff::En1998Diff;
use crate::artifacts::en1998::mutations::En1998Mutation;
use crate::artifacts::en1998::mutations::change_en_a_gr::ChangeEnAGr;

//#region 🔖️ChangeEnAGr
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeEnAGr {
    pub new_en_a_gr: f64,
}

impl protocol::MutationKind<En1998Snapshot, En1998Mutation> for ChangeEnAGr {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "en-a-gr", kind: "change-en-a-gr", record: "ChangedEnAGr" };

    fn diff(&self, base: &En1998Snapshot) -> protocol::MutationOutcome<En1998Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1998Snapshot) -> Vec<En1998Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change reference ground acceleration a_gr to {}", self.new_en_a_gr)
    }
}
//#endregion 🔖️ChangeEnAGr
