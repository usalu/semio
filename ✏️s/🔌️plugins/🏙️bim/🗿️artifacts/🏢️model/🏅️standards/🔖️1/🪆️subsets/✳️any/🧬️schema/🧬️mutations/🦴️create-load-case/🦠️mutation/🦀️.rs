//! 🦴️ Authored create-load-case semantic leaf.
use crate::standards::v1::subsets::any::schema::{snapshot::{LoadCase, ModelSnapshot}, diff::ModelDiff};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLoadCase { pub id: String, pub load_case: LoadCase, }
impl<Op> MutationKind<ModelSnapshot, Op> for CreateLoadCase
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateLoadCase> + From<crate::standards::v1::subsets::any::schema::mutations::delete_load_case::DeleteLoadCase>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "load-case", kind: "create-load-case", record: "CreateLoadCase" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::create_load_case::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::create_load_case::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("create load-case", "Anlegen Lastfall") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
