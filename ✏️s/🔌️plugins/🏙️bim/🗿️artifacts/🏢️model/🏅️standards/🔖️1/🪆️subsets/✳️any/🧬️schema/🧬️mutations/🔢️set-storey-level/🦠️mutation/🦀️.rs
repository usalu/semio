//! 🔢️ `set-storey-level` payload. Sets a storey's level index; level 0 is the building datum, positive levels stack upward, negative levels downward.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStoreyLevel {
    pub id: String,
    pub level: i32,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetStoreyLevel {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "storey", kind: "set-storey-level", record: "SetStoreyLevel" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set storey level to {}", self.level), &format!("Geschossebene auf {} setzen", self.level))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
