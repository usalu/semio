//! 🔧 `change-m-crit-knm` payload — changes the En1995 document's `m_crit_knm` (EN 1995 input).


use crate::artifacts::en1995::En1995Snapshot;
use crate::artifacts::en1995::diff::En1995Diff;
use crate::artifacts::en1995::mutations::En1995Mutation;
use crate::artifacts::en1995::mutations::change_m_crit_knm::ChangeMCritKnm;

//#region 🔖️ChangeMCritKnm
#[derive(Clone, Debug, PartialEq, dsl::MutationLeaf, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ChangeMCritKnm {
    pub new_m_crit_knm: f64,
}

impl protocol::MutationKind<En1995Snapshot, En1995Mutation> for ChangeMCritKnm {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "m-crit-knm", kind: "change-m-crit-knm", record: "ChangedMCritKnm" };

    fn diff(&self, base: &En1995Snapshot) -> protocol::MutationOutcome<En1995Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &En1995Snapshot) -> Vec<En1995Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change m crit knm to {:?}", self.new_m_crit_knm)
    }
}
//#endregion 🔖️ChangeMCritKnm
