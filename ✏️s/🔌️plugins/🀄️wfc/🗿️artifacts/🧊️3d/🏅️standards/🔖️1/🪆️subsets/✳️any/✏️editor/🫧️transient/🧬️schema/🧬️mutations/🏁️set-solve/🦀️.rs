//! 🏁️ Replaces the app-local solve result every WFC 3D window paints from.

use super::{Wfc3dTransient, Wfc3dTransientDiff};
use crate::editor::wfc3d::transient::Wfc3dAssignment;

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-solve")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSolve {
    #[dsl(table)]
    pub assignments: Vec<Wfc3dAssignment>,
    pub contradiction: bool,
    pub source_revision: Vec<u8>,
}

impl<Op> protocol::MutationKind<Wfc3dTransient, Op> for SetSolve
where
    Op: protocol::Mutation<Wfc3dTransient, Diff = Wfc3dTransientDiff> + From<SetSolve>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solve", kind: "set-solve", record: "SetSolve" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set Solve", "Lösung setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["assignments".into()]
    }
}

impl<Op> protocol::ColdMutationKind<Wfc3dTransient, Op> for SetSolve
where
    Op: protocol::ColdMutation<Wfc3dTransient, ReferenceDiff = Wfc3dTransientDiff> + From<SetSolve>,
{
    fn diff(&self, base: &Wfc3dTransient) -> protocol::MutationOutcome<Wfc3dTransientDiff> {
        let diff = Wfc3dTransientDiff { assignments: (base.assignments != self.assignments).then(|| self.assignments.clone()), contradiction: (base.contradiction != self.contradiction).then_some(self.contradiction), source_revision: (base.source_revision != self.source_revision).then(|| self.source_revision.clone()) };
        if protocol::DiffAlgebra::<Wfc3dTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The solve already holds that result.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Wfc3dTransient) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { assignments: base.assignments.clone(), contradiction: base.contradiction, source_revision: base.source_revision.clone() }.into()]
    
    })())
}
}

protocol::retained_set_record!(SetSolve => Wfc3dTransient, Wfc3dTransientDiff, Wfc3dTransientMutation { authored: SetSolve = |leaf| leaf, patch: |value: SetSolve| Wfc3dTransientDiff { assignments: Some(value.assignments), contradiction: Some(value.contradiction), source_revision: Some(value.source_revision) }, inverse: |root: Wfc3dTransient| SetSolve { assignments: root.assignments, contradiction: root.contradiction, source_revision: root.source_revision }.into() });
