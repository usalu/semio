//! 🏁️ Replaces the app-local solve result every WFC 2D window paints from.

use super::{Wfc2dTransient, Wfc2dTransientMutation};
use crate::editor::wfc2d::transient::Wfc2dAssignment;

#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, dsl::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-solve")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSolve {
    #[dsl(table)]
    pub assignments: Vec<Wfc2dAssignment>,
    pub contradiction: bool,
}

impl protocol::MutationKind<Wfc2dTransient, Wfc2dTransientMutation> for SetSolve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solve", kind: "set-solve", record: "SetSolve" };
    fn diff(&self, _base: &Wfc2dTransient) -> protocol::MutationOutcome<Wfc2dTransient> {
        protocol::MutationOutcome::new(Wfc2dTransient { assignments: self.assignments.clone(), contradiction: self.contradiction })
    }
    fn inverse(&self, base: &Wfc2dTransient) -> Vec<Wfc2dTransientMutation> {
        vec![Self { assignments: base.assignments.clone(), contradiction: base.contradiction }.into()]
    }
    fn label(&self) -> String {
        "Set Solve".into()
    }
    fn target(&self) -> Vec<String> {
        vec!["assignments".into()]
    }
}
