//! 🧊️ Inserts an exact owned topology sibling at its declared document position.
use crate::{CadSnapshot, mutations::CadMutation, diff::{CadDiff, CadBrepsDelta}};
use protocol::MutationKind;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "create-brep")]
pub struct CreateBrep {
    pub child_id: String,
    pub target: semio_framework_artifact_reference::ArtifactRef,
    pub index: u32,
}

impl MutationKind<CadSnapshot, CadMutation> for CreateBrep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "brep", kind: "create-brep", record: "CreatedBrep" };
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
        let child = match crate::cad_brep_child(&self.child_id, &self.target) {
            Ok(child) => child,
            Err(reason) => return protocol::MutationOutcome::fatal("mutation.invariant", reason, [self.child_id.clone()]),
        };
        if self.index as usize > base.breps.len() || base.breps.iter().any(|child| child.child_id == self.child_id) {
            return protocol::MutationOutcome::fatal("mutation.invariant", "topology sibling index or identity is invalid", [self.child_id.clone()]);
        }
        protocol::MutationOutcome::new(CadDiff { breps: Some(CadBrepsDelta::insertion(self.index as usize, child)), ..Default::default() })
    }
    fn inverse(&self, _base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        Ok(vec![CadMutation::DeleteBrep(super::delete_brep::DeleteBrep { child_id: self.child_id.clone() })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create topology child", "Topologie-Kind erstellen") }
    fn target(&self) -> Vec<String> { vec![self.child_id.clone()] }
}
