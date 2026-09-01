//! 🧮️ `change-layer-lambda` — sets one construction layer's thermal conductivity
//! `lambda_w_mk`, addressed by BASE-state index.


use crate::artifacts::din4108::{Din4108Diff, Din4108Mutation, Din4108Snapshot};
use crate::artifacts::din4108::diff::Din4108LayerList;
use serde::{Deserialize, Serialize};

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct ChangeLayerLambda {
    pub index: usize,
    pub new_lambda_w_mk: f64,
}

impl protocol::MutationKind<Din4108Snapshot, Din4108Mutation> for ChangeLayerLambda {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "layer-lambda", kind: "change-layer-lambda", record: "ChangedLayerLambda" };

    fn diff(&self, base: &Din4108Snapshot) -> protocol::MutationOutcome<<Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::Diff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Din4108Snapshot) -> Vec<Din4108Mutation> {
        super::inverse::inverse(self, base)
    }
    fn label(&self) -> String {
        format!("Change layer #{} lambda to {}", self.index, self.new_lambda_w_mk)
    }
    fn target(&self) -> Vec<String> {
        vec![self.index.to_string()]
    }
}
//#endregion 🔖️Payload
