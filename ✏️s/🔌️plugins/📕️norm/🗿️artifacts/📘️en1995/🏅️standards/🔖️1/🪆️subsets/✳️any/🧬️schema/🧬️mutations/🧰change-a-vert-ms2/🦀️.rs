//! 🔧 `change-a-vert-m-s2` payload — changes the En1995 document's `a_vert_m_s2` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::change_a_vert_m_s2::ChangeAVertMS2;

//#region 🔖️ChangeAVertMS2
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeAVertMS2 {
    pub new_a_vert_m_s2: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeAVertMS2 {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "a-vert-ms2", kind: "change-a-vert-ms2", record: "ChangedAVertMS2" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change a vert m s2 to {:?}", self.new_a_vert_m_s2)
    }
}
//#endregion 🔖️ChangeAVertMS2
