//! 🌍️ `create-site` payload. Brings a new site into existence.

use crate::standards::v1::subsets::any::schema::{diff::ModelDiff, snapshot::{ModelSnapshot, Site}};
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
    Op: protocol::Mutation<ModelSnapshot, Diff = ModelDiff> + From<CreateSite> + From<crate::standards::v1::subsets::any::schema::mutations::delete_site::DeleteSite>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "site", kind: "create-site", record: "CreateSite" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        crate::standards::v1::subsets::any::schema::mutations::create_site::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(crate::standards::v1::subsets::any::schema::mutations::create_site::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create site \"{}\"", self.site.name), &format!("Standort \"{}\" anlegen", self.site.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
