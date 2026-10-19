//! 🕳️ `create-opening` payload. Brings a window, door or void into a wall or curtain wall; only its centre offset along the host axis and its optional sill override are stored, the frame follows the host by inference.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Opening};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateOpening {
    pub id: String,
    pub opening: Opening,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateOpening
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateOpening> + From<semio_s_artifact_bim_model_mut_any_78::standards::v1::subsets::any::schema::mutations::delete_opening::DeleteOpening>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "opening", kind: "create-opening", record: "CreateOpening" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_opening::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_opening::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create opening \"{}\"", self.opening.name), &format!("Öffnung \"{}\" anlegen", self.opening.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
