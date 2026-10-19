//! 🏞️ `create-ceiling` payload. Brings a new ceiling onto a storey; its boundary and holes are closed counter-clockwise loops, area, solid and the clear height of the rooms below are inferred.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Ceiling, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCeiling {
    pub id: String,
    pub ceiling: Ceiling,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateCeiling
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateCeiling> + From<semio_s_artifact_bim_model_mut_any_90::standards::v1::subsets::any::schema::mutations::delete_ceiling::DeleteCeiling>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "ceiling", kind: "create-ceiling", record: "CreateCeiling" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_ceiling::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_ceiling::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create ceiling \"{}\"", self.ceiling.name), &format!("Unterdecke \"{}\" anlegen", self.ceiling.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
