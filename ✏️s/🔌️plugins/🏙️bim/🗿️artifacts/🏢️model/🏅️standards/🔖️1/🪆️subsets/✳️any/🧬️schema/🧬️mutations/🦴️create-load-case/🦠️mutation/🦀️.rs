//! 🦴️ Authored create-load-case semantic leaf.
use crate::{LoadCase, LoadCasePatch, ModelDiff, ModelMutation, ModelSnapshot};
use crate::{StructuralLocation, Restraints, Point3};
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLoadCase { pub id: String, pub load_case: LoadCase, }
impl MutationKind<ModelSnapshot, ModelMutation> for CreateLoadCase {
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "load-case", kind: "create-load-case", record: "CreateLoadCase" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { super::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> { Ok(super::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("create load-case", "Anlegen Lastfall") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
