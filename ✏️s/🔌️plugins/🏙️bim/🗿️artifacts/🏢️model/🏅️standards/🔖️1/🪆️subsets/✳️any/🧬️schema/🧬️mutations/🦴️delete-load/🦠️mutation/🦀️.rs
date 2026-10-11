//! 🦴️ Authored delete-load semantic leaf.
use crate::{StructuralLoad, StructuralLoadPatch, ModelDiff, ModelMutation, ModelSnapshot};
use crate::{StructuralLocation, Restraints, Point3};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteLoad { pub id: String, }
impl MutationKind<ModelSnapshot, ModelMutation> for DeleteLoad {
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "load", kind: "delete-load", record: "DeleteLoad" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("delete load", "Löschen Last") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
