//! 🔧 `change-v-ed-kn` payload — changes the En1995 document's `v_ed_kn` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
//#region 🔖️ChangeVEdKn
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeVEdKn {
    pub new_v_ed_kn: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeVEdKn {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "v-ed-kn", kind: "change-v-ed-kn", record: "ChangedVEdKn" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change v ed kn to {:?}", self.new_v_ed_kn)
    }
}
//#endregion 🔖️ChangeVEdKn
