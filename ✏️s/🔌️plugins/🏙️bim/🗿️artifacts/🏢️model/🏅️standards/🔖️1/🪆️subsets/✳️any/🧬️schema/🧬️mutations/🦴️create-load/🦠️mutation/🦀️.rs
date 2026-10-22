//! 🦴️ Authored create-load semantic leaf.
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{StructuralLoad, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateLoad { pub id: String, pub load: StructuralLoad, }
impl<Op> MutationKind<ModelSnapshot, Op> for CreateLoad
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateLoad> + From<crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad>,
{
const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "load", kind: "create-load", record: "CreateLoad" };
fn label(&self) -> semio_framework_ui_locale::LocalizedLabel { semio_framework_ui_locale::LocalizedLabel::native("create load", "Anlegen Last") }
fn target(&self) -> Vec<String> { vec![self.id.clone()] }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateLoad
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateLoad> + From<crate::standards::v1::subsets::any::schema::mutations::delete_load::DeleteLoad>,
{
fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> { crate::standards::v1::subsets::any::schema::mutations::create_load::diff::diff(self, base) }
fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> { Ok(crate::standards::v1::subsets::any::schema::mutations::create_load::inverse::inverse(self, base)) }
}
