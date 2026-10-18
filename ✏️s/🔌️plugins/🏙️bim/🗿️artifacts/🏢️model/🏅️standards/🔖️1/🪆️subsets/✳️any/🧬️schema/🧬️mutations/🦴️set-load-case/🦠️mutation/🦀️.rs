//! 🦴️ Authored set-load-case semantic leaf.
use crate::standards::v1::subsets::any::schema::{diff::{patches::LoadCasePatch, ModelDiff}, snapshot::ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLoadCase { pub id: String, #[value(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub category: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub factor: Option<f64>, }
impl SetLoadCase { pub fn patch(&self) -> LoadCasePatch { LoadCasePatch { name: self.name.clone(), category: self.category.clone(), factor: self.factor.clone() } } pub fn from_patch(id: String, patch: LoadCasePatch) -> Self { Self { id, name: patch.name, category: patch.category, factor: patch.factor } } }
impl<Op> MutationKind<ModelSnapshot, Op> for SetLoadCase
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetLoadCase>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "load-case", kind: "set-load-case", record: "SetLoadCase" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::set_load_case::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::set_load_case::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("set load-case", "Ändern Lastfall") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
