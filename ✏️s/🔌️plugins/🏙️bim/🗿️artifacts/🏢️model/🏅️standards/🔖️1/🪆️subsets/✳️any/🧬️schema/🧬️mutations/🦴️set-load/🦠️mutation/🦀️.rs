//! 🦴️ Authored set-load semantic leaf.
use crate::{StructuralLoad, StructuralLoadPatch, ModelDiff, ModelMutation, ModelSnapshot};
use crate::{StructuralLocation, Restraints, Point3};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLoad { pub id: String, #[value(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub load_case: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub member: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub location: Option<StructuralLocation>, #[value(default, skip_serializing_if = "Option::is_none")] pub force: Option<Point3>, #[value(default, skip_serializing_if = "Option::is_none")] pub moment: Option<Point3>, }
impl SetLoad { pub fn patch(&self) -> StructuralLoadPatch { StructuralLoadPatch { name: self.name.clone(), load_case: self.load_case.clone(), member: self.member.clone(), location: self.location.clone(), force: self.force.clone(), moment: self.moment.clone() } } pub fn from_patch(id: String, patch: StructuralLoadPatch) -> Self { Self { id, name: patch.name, load_case: patch.load_case, member: patch.member, location: patch.location, force: patch.force, moment: patch.moment } } }
impl MutationKind<ModelSnapshot, ModelMutation> for SetLoad {
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "load", kind: "set-load", record: "SetLoad" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("set load", "Ändern Last") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
