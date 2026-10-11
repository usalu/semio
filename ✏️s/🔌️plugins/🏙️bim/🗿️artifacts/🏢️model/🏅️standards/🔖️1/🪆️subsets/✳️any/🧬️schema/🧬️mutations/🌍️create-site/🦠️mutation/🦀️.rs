//! 🌍️ `create-site` payload. Brings a new site into existence.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Site};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateSite {
    pub id: String,
    pub site: Site,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateSite {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "site", kind: "create-site", record: "CreateSite" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create site \"{}\"", self.site.name), &format!("Standort \"{}\" anlegen", self.site.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
