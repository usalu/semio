//! 🌉️ `create-beam-type` payload. Brings a new beam type into the library; it needs a free id, an existing material and a valid profile.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{BeamType, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateBeamType {
    pub id: String,
    pub beam_type: BeamType,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateBeamType
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateBeamType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "beam-type", kind: "create-beam-type", record: "CreateBeamType" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create beam type \"{}\"", self.beam_type.name), &format!("Trägertyp \"{}\" anlegen", self.beam_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateBeamType
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateBeamType> + From<crate::standards::v1::subsets::any::schema::mutations::delete_beam_type::DeleteBeamType>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_beam_type::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_beam_type::inverse::inverse(self, base))
    }
}
