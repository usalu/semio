//! 🛝️ `create-ramp` payload. Brings a new ramp onto a storey; its length, rise, slope, landings, solid and compliance are inferred from the authored path, landings and top constraint.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Ramp};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRamp {
    pub id: String,
    pub ramp: Ramp,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateRamp
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateRamp> + From<semio_s_artifact_bim_model_mut_any_92::standards::v1::subsets::any::schema::mutations::delete_ramp::DeleteRamp>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "ramp", kind: "create-ramp", record: "CreateRamp" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_ramp::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_ramp::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create ramp \"{}\"", self.ramp.name), &format!("Rampe \"{}\" anlegen", self.ramp.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
