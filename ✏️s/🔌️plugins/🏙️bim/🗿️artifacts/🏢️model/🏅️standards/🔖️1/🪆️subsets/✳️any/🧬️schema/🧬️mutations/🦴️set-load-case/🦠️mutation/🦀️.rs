//! 🦴️ Authored set-load-case semantic leaf.
use crate::{LoadCase, LoadCasePatch, ModelDiff, ModelMutation, ModelSnapshot};
use crate::{StructuralLocation, Restraints, Point3};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLoadCase { pub id: String, #[value(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub category: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub factor: Option<f64>, }
impl SetLoadCase { pub fn patch(&self) -> LoadCasePatch { LoadCasePatch { name: self.name.clone(), category: self.category.clone(), factor: self.factor.clone() } } pub fn from_patch(id: String, patch: LoadCasePatch) -> Self { Self { id, name: patch.name, category: patch.category, factor: patch.factor } } }
impl MutationKind<ModelSnapshot, ModelMutation> for SetLoadCase {
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "load-case", kind: "set-load-case", record: "SetLoadCase" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("set load-case", "Ändern Lastfall") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
