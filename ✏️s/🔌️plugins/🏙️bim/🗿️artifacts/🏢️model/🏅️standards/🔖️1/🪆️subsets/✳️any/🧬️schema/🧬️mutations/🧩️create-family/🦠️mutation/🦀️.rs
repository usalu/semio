//! 🧩️ `create-family` payload. Brings a new, empty parametric family into the project: a name and a category; parameters and solids are added afterwards.

use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{Family, ModelSnapshot};
use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateFamily {
    pub id: String,
    pub family: Family,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateFamily
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateFamily> + From<crate::standards::v1::subsets::any::schema::mutations::delete_family::DeleteFamily>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "family", kind: "create-family", record: "CreateFamily" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_family::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_family::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create family \"{}\"", self.family.name), &format!("Familie \"{}\" anlegen", self.family.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
