//! 🏁️ Replaces the app-local solve result every WFC 2D window paints from.

use super::{Wfc2dTransient, Wfc2dTransientMutation};
use crate::editor::wfc2d::transient::Wfc2dAssignment;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
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
    fn inverse(&self, base: &Wfc2dTransient) -> Result<Vec<Wfc2dTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { assignments: base.assignments.clone(), contradiction: base.contradiction }.into()]
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set Solve", "Lösung setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["assignments".into()]
    }
}
