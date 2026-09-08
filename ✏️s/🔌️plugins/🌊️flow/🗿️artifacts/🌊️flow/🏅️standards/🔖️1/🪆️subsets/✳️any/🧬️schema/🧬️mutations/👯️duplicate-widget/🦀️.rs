//! 👯️ Duplicates a widget and wires the copy to its source — the repo's pilot COMPOSITE mutation
//! (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS): its plan
//! calls the EXISTING leaf kinds `create-widget` (mints the copy) then `connect-widgets` (wires
//! source → copy) through a shared `protocol::Planner`, proving a mutation can call other mutations.
//! A composite owns no `🔺️diff`/`↩️inverse` of its own — both fold from `plan` via
//! `protocol::fold_plan_diff`/`fold_plan_inverse`, wired in by `#[derive(dsl_derive::CompositeMutation)]`.

use crate::FlowSnapshot;
use crate::schema::mutations::FlowMutation;
use protocol::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};

//#region 👯️DuplicateWidget
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl_derive::CompositeMutation, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[mutation_leaf(contract = ::protocol)]
#[composite(snapshot = FlowSnapshot, op = FlowMutation)]
pub struct DuplicateWidget {
    pub source_id: String,
    pub new_id: String,
    pub synapse_id: String,
    pub from_port: String,
    pub to_port: String,
}

impl CompositeMutationKind<FlowSnapshot, FlowMutation> for DuplicateWidget {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "duplicate", entity: "widget", kind: "duplicate-widget", record: "DuplicatedWidget" };

    fn plan(&self, base: &FlowSnapshot, planner: &mut Planner<FlowSnapshot, FlowMutation>) -> Result<(), PlanError> {
        super::plan::plan(self, base, planner)
    }
    fn label(&self) -> String {
        format!("Duplicate widget \"{}\"", self.source_id)
    }
    fn target(&self) -> Vec<String> {
        vec![self.source_id.clone(), self.new_id.clone()]
    }
}
//#endregion 👯️DuplicateWidget

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
