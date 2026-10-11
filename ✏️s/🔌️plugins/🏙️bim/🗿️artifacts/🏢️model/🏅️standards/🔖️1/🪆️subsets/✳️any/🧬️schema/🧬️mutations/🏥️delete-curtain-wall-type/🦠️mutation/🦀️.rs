//! 🏥️ `delete-curtain-wall-type` payload. Removes a curtain wall type that no curtain wall uses.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteCurtainWallType {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteCurtainWallType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "curtain-wall-type", kind: "delete-curtain-wall-type", record: "DeleteCurtainWallType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete curtain wall type \"{}\"", self.id), &format!("Fassadentyp \"{}\" löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
