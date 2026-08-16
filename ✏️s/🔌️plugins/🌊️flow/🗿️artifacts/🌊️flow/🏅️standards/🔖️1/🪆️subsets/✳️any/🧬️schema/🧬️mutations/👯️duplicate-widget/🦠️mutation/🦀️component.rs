//! 👯️ Duplicates a widget and wires the copy to its source — the repo's pilot COMPOSITE mutation
//! (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS): its plan
//! calls the EXISTING leaf kinds `create-widget` (mints the copy) then `connect-widgets` (wires
//! source → copy) through a shared `protocol::Planner`, proving a mutation can call other mutations.
//! A composite owns no `🔺️diff`/`↩️inverse` of its own — both fold from `plan` via
//! `protocol::fold_plan_diff`/`fold_plan_inverse`, wired in by `#[derive(dsl_derive::CompositeMutation)]`.
use crate::artifacts::flow::schema::mutations::FlowMutation;
use crate::artifacts::flow::FlowSnapshot;
use protocol::{CompositeMutationKind, PlanError, Planner, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 👯️DuplicateWidget
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl_derive::CompositeMutation)]
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
    fn validate(&self, base: &FlowSnapshot) -> Result<(), String> {
        super::plan::precondition(self, base)
    }
}
//#endregion 👯️DuplicateWidget

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> DuplicateWidget {
        DuplicateWidget { source_id: "note-1".into(), new_id: "note-2".into(), synapse_id: "note-1-to-note-2".into(), from_port: "out".into(), to_port: "in".into() }
    }

    #[test]
    fn label_and_target_are_sensible() {
        let payload = sample();
        assert_eq!(CompositeMutationKind::label(&payload), "Duplicate widget \"note-1\"");
        assert_eq!(CompositeMutationKind::target(&payload), vec!["note-1".to_string(), "note-2".to_string()]);
    }

    #[test]
    fn semantics_kind_and_verb_match_the_directory() {
        let semantics = <DuplicateWidget as CompositeMutationKind<FlowSnapshot, FlowMutation>>::SEMANTICS;
        assert_eq!(semantics.kind, "duplicate-widget");
        assert_eq!(semantics.verb, "duplicate");
    }
}
//#endregion 🧪️Tests
