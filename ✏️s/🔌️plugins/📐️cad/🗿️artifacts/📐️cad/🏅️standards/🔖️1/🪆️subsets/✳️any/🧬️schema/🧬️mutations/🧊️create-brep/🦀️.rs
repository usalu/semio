//! 🧊️ Inserts an exact owned topology sibling at its declared document position.
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;
use semio_s_artifact_cad_cad_model_diff::{CadDiff, CadBrepsDelta};
use protocol::MutationKind;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-brep")]
pub struct CreateBrep {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub index: u32,
}

impl<Op> MutationKind<CadSnapshot, Op> for CreateBrep
where
    Op: protocol::Mutation<CadSnapshot, Diff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::delete_brep::DeleteBrep>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "brep", kind: "create-brep", record: "CreatedBrep" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create topology child", "Topologie-Kind erstellen") }
    fn target(&self) -> Vec<String> { vec![self.child_id.clone()] }
}

impl<Op> protocol::ColdMutationKind<CadSnapshot, Op> for CreateBrep
where
    Op: protocol::ColdMutation<CadSnapshot, ReferenceDiff = semio_s_artifact_cad_cad_model_diff::CadDiff> + From<crate::delete_brep::DeleteBrep>,
{
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
        let child = match semio_s_artifact_cad_cad_model::snapshot::cad_brep_child(&self.child_id, &self.target) {
            Ok(child) => child,
            Err(reason) => return protocol::MutationOutcome::fatal("mutation.invariant", reason, [self.child_id.clone()]),
        };
        if self.index as usize > base.breps.len() || base.breps.iter().any(|child| child.child_id == self.child_id) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "topology sibling index or identity is invalid", [self.child_id.clone()]);
        }
        protocol::MutationOutcome::new(CadDiff { breps: Some(CadBrepsDelta::insertion(self.index as usize, child)), ..Default::default() })
    }
    fn inverse(&self, _base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(vec![Op::from(super::delete_brep::DeleteBrep { child_id: self.child_id.clone() })])
    }
}
