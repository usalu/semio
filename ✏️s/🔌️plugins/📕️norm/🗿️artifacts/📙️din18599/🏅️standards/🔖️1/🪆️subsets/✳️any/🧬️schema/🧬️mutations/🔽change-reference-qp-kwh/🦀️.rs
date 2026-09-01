//! 🔽 `change-reference-qp-kwh` payload — changes the Din18599 document's `reference_q_p_kwh` (reference primary energy demand Q_p [kWh]).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::mutations::change_reference_q_p_kwh::ChangeReferenceQPKwh;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeReferenceQPKwh
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeReferenceQPKwh {
    pub new_reference_q_p_kwh: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeReferenceQPKwh {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "reference-qp-kwh", kind: "change-reference-qp-kwh", record: "ChangedReferenceQPKwh" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change reference primary energy demand Q_p [kWh] to {}", self.new_reference_q_p_kwh)
    }
}
//#endregion 🔖️ChangeReferenceQPKwh
