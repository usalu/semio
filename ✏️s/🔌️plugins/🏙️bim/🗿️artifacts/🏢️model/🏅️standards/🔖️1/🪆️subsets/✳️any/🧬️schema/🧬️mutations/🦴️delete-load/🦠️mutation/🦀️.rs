//! 🦴️ Authored delete-load semantic leaf.
use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteLoad { pub id: String, }
impl<Op> MutationKind<ModelSnapshot, Op> for DeleteLoad
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<DeleteLoad> + From<crate::standards::v1::subsets::any::schema::mutations::create_load::CreateLoad>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "load", kind: "delete-load", record: "DeleteLoad" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::delete_load::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::delete_load::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("delete load", "Löschen Last") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
