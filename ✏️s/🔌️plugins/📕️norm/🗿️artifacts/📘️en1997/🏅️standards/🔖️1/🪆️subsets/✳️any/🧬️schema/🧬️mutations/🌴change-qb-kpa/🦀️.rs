//! 🌴 `change-qb-kpa` payload — changes the En1997 document's `q_b_kpa` (base resistance q_b [kPa]).


use crate::artifacts::en1997::En1997Snapshot;
use crate::artifacts::en1997::diff::En1997Diff;
use crate::artifacts::en1997::mutations::En1997Mutation;
use crate::artifacts::en1997::mutations::change_q_b_kpa::ChangeQBKpa;

//#region 🔖️ChangeQBKpa
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeQBKpa {
    pub new_q_b_kpa: f64,
}

impl protocol::MutationKind<En1997Snapshot, En1997Mutation> for ChangeQBKpa {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "qb-kpa", kind: "change-qb-kpa", record: "ChangedQBKpa" };

    fn diff(&self, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1997Snapshot) -> Vec<En1997Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change base resistance q_b [kPa] to {}", self.new_q_b_kpa)
    }
}
//#endregion 🔖️ChangeQBKpa
