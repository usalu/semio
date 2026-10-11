//! 🛷️ `move-opening` payload. Moves an opening along its host axis (centre offset from the host start) and optionally re-hosts it to another wall or curtain wall.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct MoveOpening {
    pub id: String,
    pub offset: f64,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for MoveOpening {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "move", entity: "opening", kind: "move-opening", record: "MoveOpening" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Move opening \"{}\" to {} m", self.id, self.offset), &format!("Öffnung \"{}\" auf {} m verschieben", self.id, self.offset))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
