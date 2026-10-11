//! 🏚️ `delete-building` payload. Removes a building together with its storeys, grid lines and everything on them (walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces, openings) and their properties and classifications; refuses while a surviving element still constrains its top to a removed storey.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteBuilding {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteBuilding {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "building", kind: "delete-building", record: "DeleteBuilding" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete building \"{}\" with everything in it", self.id), &format!("Gebäude \"{}\" samt Inhalt löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
