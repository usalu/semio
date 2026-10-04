//! 🏁️ Replaces the app-local solve result every WFC 3D window paints from.

use super::{Wfc3dTransient, Wfc3dTransientMutation};
use crate::editor::wfc3d::transient::Wfc3dAssignment;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-solve")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSolve {
    #[dsl(table)]
    pub assignments: Vec<Wfc3dAssignment>,
    pub contradiction: bool,
}

impl protocol::MutationKind<Wfc3dTransient, Wfc3dTransientMutation> for SetSolve {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solve", kind: "set-solve", record: "SetSolve" };
    fn diff(&self, _base: &Wfc3dTransient) -> protocol::MutationOutcome<Wfc3dTransient> {
        protocol::MutationOutcome::new(Wfc3dTransient { assignments: self.assignments.clone(), contradiction: self.contradiction })
    }
    fn inverse(&self, base: &Wfc3dTransient) -> Result<Vec<Wfc3dTransientMutation>, semio_framework_value::ValueError> {
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
