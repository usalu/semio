//! 🦴️ Authored delete-support semantic leaf.
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSupport { pub id: String, }
impl<Op> MutationKind<ModelSnapshot, Op> for DeleteSupport
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteSupport> + From<crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "support", kind: "delete-support", record: "DeleteSupport" };
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("delete support", "Löschen Auflager") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for DeleteSupport
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<DeleteSupport> + From<crate::standards::v1::subsets::any::schema::mutations::create_support::CreateSupport>,
{
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::delete_support::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::delete_support::inverse::inverse(self, base)) }
}
