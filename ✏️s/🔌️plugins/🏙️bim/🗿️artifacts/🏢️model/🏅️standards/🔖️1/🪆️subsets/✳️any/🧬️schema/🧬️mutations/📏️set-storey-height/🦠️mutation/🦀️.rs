//! 📏️ `set-storey-height` payload. Sets a storey's floor-to-floor height in metres; every elevation above and every wall resolved by it follows by inference.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetStoreyHeight {
    pub id: String,
    pub height: f64,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetStoreyHeight {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "storey", kind: "set-storey-height", record: "SetStoreyHeight" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set storey height to {} m", self.height), &format!("Geschosshöhe auf {} m setzen", self.height))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
