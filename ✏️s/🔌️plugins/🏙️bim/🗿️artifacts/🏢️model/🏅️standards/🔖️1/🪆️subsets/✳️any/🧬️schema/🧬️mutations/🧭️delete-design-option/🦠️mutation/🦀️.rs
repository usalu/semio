//! 🧭️ delete-design-option authored mutation.
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteDesignOption {
    pub id: String,
}
impl<Op> MutationKind<ModelSnapshot, Op> for DeleteDesignOption
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteDesignOption> + From<crate::standards::v1::subsets::any::schema::mutations::create_design_option::CreateDesignOption> + From<semio_s_artifact_bim_model_mut_any_50::standards::v1::subsets::any::schema::mutations::set_element_option::SetElementOption>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "design-option", kind: "delete-design-option", record: "DeleteDesignOption" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::delete_design_option::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::delete_design_option::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Delete design option", "Löschen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
