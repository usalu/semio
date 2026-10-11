//! 🔝️ `set-wall-top` payload. Sets how a wall's top is resolved: free height, the own storey's top, or another storey of the same building.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, TopConstraint};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallTop {
    pub id: String,
    pub top: TopConstraint,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetWallTop {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-top", record: "SetWallTop" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Constrain the top of wall \"{}\"", self.id), &format!("Oberkante von Wand \"{}\" festlegen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
