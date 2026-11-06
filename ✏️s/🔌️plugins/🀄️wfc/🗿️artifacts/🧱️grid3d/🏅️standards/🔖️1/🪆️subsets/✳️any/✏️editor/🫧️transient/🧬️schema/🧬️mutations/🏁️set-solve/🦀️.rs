//! 🏁️ Replaces the app-local solve result every WFC 3D window paints from.

use super::{Grid3dTransient, Grid3dTransientDiff};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "set-solve")]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSolve {
    pub payload_json: String,
}

impl<Op> protocol::MutationKind<Grid3dTransient, Op> for SetSolve
where
    Op: protocol::Mutation<Grid3dTransient, Diff = Grid3dTransientDiff> + From<SetSolve>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solve", kind: "set-solve", record: "SetSolve" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set Solve", "Lösung setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["payloadJson".into()]
    }
}

impl<Op> protocol::ColdMutationKind<Grid3dTransient, Op> for SetSolve
where
    Op: protocol::ColdMutation<Grid3dTransient, ReferenceDiff = Grid3dTransientDiff> + From<SetSolve>,
{
    fn diff(&self, base: &Grid3dTransient) -> protocol::MutationOutcome<Grid3dTransientDiff> {
        let diff = Grid3dTransientDiff { payload_json: (base.payload_json != self.payload_json).then(||self.payload_json.clone()) };
        if protocol::DiffAlgebra::<Grid3dTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The solve already holds that result.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &Grid3dTransient) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Self { payload_json: base.payload_json.clone() }.into()]

    })())
}
}

protocol::retained_set_record!(SetSolve => Grid3dTransient, Grid3dTransientDiff, Grid3dTransientMutation { authored: SetSolve = |leaf| leaf, patch: |value: SetSolve| (Grid3dTransientDiff { payload_json: Some(value.payload_json) },1), patch_work: |_: &SetSolve| Ok(1), inverse: |root: Grid3dTransient| (SetSolve { payload_json: root.payload_json }.into(),2), inverse_work: |_: &Grid3dTransient| Ok(2) });
