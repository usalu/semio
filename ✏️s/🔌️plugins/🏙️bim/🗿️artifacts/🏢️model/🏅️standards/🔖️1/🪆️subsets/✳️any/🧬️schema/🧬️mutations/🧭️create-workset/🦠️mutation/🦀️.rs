//! 🧭️ create-workset authored mutation.
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Workset}};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateWorkset {
    pub id: String,
    pub workset: Workset,
}
impl<Op> MutationKind<ModelSnapshot, Op> for CreateWorkset
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateWorkset> + From<crate::standards::v1::subsets::any::schema::mutations::delete_workset::DeleteWorkset>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "workset", kind: "create-workset", record: "CreateWorkset" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::create_workset::diff::diff(self, base) }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::create_workset::inverse::inverse(self, base)) }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("Create workset", "Anlegen") }
    fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
