//! 🧹️ `delete-site` payload. Removes a site together with its buildings and everything inside them (storeys, grid lines, walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces, openings) and their properties and classifications; refuses while a surviving element still constrains its top to a removed storey.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteSite {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteSite {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "site", kind: "delete-site", record: "DeleteSite" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete site \"{}\" with everything on it", self.id), &format!("Standort \"{}\" samt Inhalt löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
