//! 🦴️ Authored set-support semantic leaf.
use crate::{StructuralSupport, StructuralSupportPatch, ModelDiff, ModelMutation, ModelSnapshot};
use crate::{StructuralLocation, Restraints, Point3};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSupport { pub id: String, #[value(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub member: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub location: Option<StructuralLocation>, #[value(default, skip_serializing_if = "Option::is_none")] pub offset: Option<Point3>, #[value(default, skip_serializing_if = "Option::is_none")] pub restraints: Option<Restraints>, }
impl SetSupport { pub fn patch(&self) -> StructuralSupportPatch { StructuralSupportPatch { name: self.name.clone(), member: self.member.clone(), location: self.location.clone(), offset: self.offset.clone(), restraints: self.restraints.clone() } } pub fn from_patch(id: String, patch: StructuralSupportPatch) -> Self { Self { id, name: patch.name, member: patch.member, location: patch.location, offset: patch.offset, restraints: patch.restraints } } }
impl MutationKind<ModelSnapshot, ModelMutation> for SetSupport {
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "support", kind: "set-support", record: "SetSupport" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("set support", "Ändern Auflager") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
