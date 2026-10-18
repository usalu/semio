//! 🦴️ Authored create-support semantic leaf.
use crate::standards::v1::subsets::any::schema::{snapshot::{StructuralSupport, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSupport { pub id: String, pub support: StructuralSupport, }
impl<Op> MutationKind<ModelSnapshot, Op> for CreateSupport
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSupport> + From<crate::standards::v1::subsets::any::schema::mutations::delete_support::DeleteSupport>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "support", kind: "create-support", record: "CreateSupport" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::create_support::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::create_support::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("create support", "Anlegen Auflager") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
