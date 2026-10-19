//! 🦴️ Authored set-load semantic leaf.
use semio_s_artifact_bim_model_storey_levels::standards::v1::subsets::any::schema::diff::patches::hand_written::StructuralLoadPatch;
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::StructuralLocation;
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point3;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetLoad { pub id: String, #[value(default, skip_serializing_if = "Option::is_none")] pub name: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub load_case: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub member: Option<String>, #[value(default, skip_serializing_if = "Option::is_none")] pub location: Option<StructuralLocation>, #[value(default, skip_serializing_if = "Option::is_none")] pub force: Option<Point3>, #[value(default, skip_serializing_if = "Option::is_none")] pub moment: Option<Point3>, }
impl SetLoad { pub fn patch(&self) -> StructuralLoadPatch { StructuralLoadPatch { name: self.name.clone(), load_case: self.load_case.clone(), member: self.member.clone(), location: self.location.clone(), force: self.force.clone(), moment: self.moment.clone() } } pub fn from_patch(id: String, patch: StructuralLoadPatch) -> Self { Self { id, name: patch.name, load_case: patch.load_case, member: patch.member, location: patch.location, force: patch.force, moment: patch.moment } } }
impl<Op> MutationKind<ModelSnapshot, Op> for SetLoad
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<SetLoad>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "load", kind: "set-load", record: "SetLoad" };
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::set_load::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::set_load::inverse::inverse(self, base)) }
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("set load", "Ändern Last") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}
