//! 🦴️ Authored set-support semantic leaf.
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::diff::patches::hand_written::StructuralSupportPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{StructuralLocation, Restraints};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point3;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSupport { pub id: String, #[value(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub member: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub location: Option<StructuralLocation>, #[value(default, skip_serializing_if = "Option::is_none")] pub offset: Option<Point3>, #[value(default, skip_serializing_if = "Option::is_none")] pub restraints: Option<Restraints>, }
impl SetSupport { pub fn patch(&self) -> StructuralSupportPatch { StructuralSupportPatch { name: self.name.clone(), member: self.member.clone(), location: self.location.clone(), offset: self.offset.clone(), restraints: self.restraints.clone() } } pub fn from_patch(id: String, patch: StructuralSupportPatch) -> Self { Self { id, name: patch.name, member: patch.member, location: patch.location, offset: patch.offset, restraints: patch.restraints } } }
impl<Op> MutationKind<ModelSnapshot, Op> for SetSupport
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetSupport>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "support", kind: "set-support", record: "SetSupport" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::set_support::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::set_support::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("set support", "Ändern Auflager") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
