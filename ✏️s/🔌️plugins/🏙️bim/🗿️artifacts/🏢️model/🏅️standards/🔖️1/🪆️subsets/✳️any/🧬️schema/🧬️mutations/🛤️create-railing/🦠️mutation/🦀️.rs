//! 🛤️ `create-railing` payload. Brings a new railing onto a storey; its posts, rails and extent are inferred from the authored path, height and post spacing.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Railing};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRailing {
    pub id: String,
    pub railing: Railing,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateRailing
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateRailing> + From<semio_s_artifact_bim_model_mut_any_62::standards::v1::subsets::any::schema::mutations::delete_railing::DeleteRailing>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "railing", kind: "create-railing", record: "CreateRailing" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_railing::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_railing::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create railing \"{}\"", self.railing.name), &format!("Geländer \"{}\" anlegen", self.railing.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
