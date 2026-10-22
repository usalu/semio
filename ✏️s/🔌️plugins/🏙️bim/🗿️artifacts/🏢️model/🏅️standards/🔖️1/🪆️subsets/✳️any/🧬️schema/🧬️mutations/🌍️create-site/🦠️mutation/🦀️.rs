//! 🌍️ `create-site` payload. Brings a new site into existence.

use semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::diff::ModelDiff;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot, Site};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSite {
    pub id: String,
    pub site: Site,
}

impl<Op> MutationKind<ModelSnapshot, Op> for CreateSite
where
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSite> + From<semio_s_artifact_bim_model_mut_any_58::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "site", kind: "create-site", record: "CreateSite" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create site \"{}\"", self.site.name), &format!("Standort \"{}\" anlegen", self.site.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ModelSnapshot, Op> for CreateSite
where
    Op: protocol::ColdMutation<ModelSnapshot, ReferenceDiff = ModelDiff> + From<CreateSite> + From<semio_s_artifact_bim_model_mut_any_58::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite>,
{
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_site::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_site::inverse::inverse(self, base))
    }
}
