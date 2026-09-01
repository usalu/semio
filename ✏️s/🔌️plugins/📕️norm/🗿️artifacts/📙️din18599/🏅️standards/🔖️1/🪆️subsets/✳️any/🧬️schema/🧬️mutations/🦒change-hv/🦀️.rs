//! 🦒 `change-hv` payload — changes the Din18599 document's `h_v` (ventilation heat transfer coefficient H_V [W/K]).


use crate::artifacts::din18599::Din18599Snapshot;
use crate::artifacts::din18599::diff::Din18599Diff;
use crate::artifacts::din18599::mutations::Din18599Mutation;
use crate::artifacts::din18599::mutations::change_h_v::ChangeHV;
use serde::{Deserialize, Serialize};

//#region 🔖️ChangeHV
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[serde(rename_all = "camelCase")]
pub struct ChangeHV {
    pub new_h_v: f64,
}

impl protocol::MutationKind<Din18599Snapshot, Din18599Mutation> for ChangeHV {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "hv", kind: "change-hv", record: "ChangedHV" };

    fn diff(&self, base: &Din18599Snapshot) -> protocol::MutationOutcome<Din18599Diff> {
        super::diff::diff(self, base)
    }

    fn inverse(&self, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
        super::inverse::inverse(self, base)
    }

    fn label(&self) -> String {
        format!("Change ventilation heat transfer coefficient H_V [W/K] to {}", self.new_h_v)
    }
}
//#endregion 🔖️ChangeHV
