//! 🧹️ Removes an exact topology sibling and preserves its ordered inverse authority.
use crate::{CadSnapshot, mutations::CadMutation, diff::{CadDiff, CadBrepChildList}};
use protocol::MutationKind;

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value_derive::RetireOwned)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "delete-brep")]
pub struct DeleteBrep {
    pub child_id: String,
}

impl MutationKind<CadSnapshot, CadMutation> for DeleteBrep {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "brep", kind: "delete-brep", record: "DeletedBrep" };
    fn diff(&self, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
        if !base.breps.iter().any(|child| child.child_id == self.child_id) {
            return protocol::MutationOutcome::fatal("mutation.missing-id", "topology sibling is absent", [self.child_id.clone()]);
        }
        let values = base.breps.iter().filter(|child| child.child_id != self.child_id).cloned().collect();
        protocol::MutationOutcome::new(CadDiff { breps: Some(CadBrepChildList { values }), ..Default::default() })
    }
    fn inverse(&self, base: &CadSnapshot) -> Result<Vec<CadMutation>, semio_framework_value::ValueError> {
        let Some((index, child)) = base.breps.iter().enumerate().find(|(_, child)| child.child_id == self.child_id) else { return Ok(Vec::new()) };
        let index = u32::try_from(index).map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "topology sibling position exceeds the portable u32 domain"))?;
        Ok(vec![CadMutation::CreateBrep(super::create_brep::CreateBrep { child_id: child.child_id.clone(), target: child.target.clone(), index })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Delete topology child", "Topologie-Kind löschen") }
    fn target(&self) -> Vec<String> { vec![self.child_id.clone()] }
}
