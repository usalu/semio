//! 🪔️ `set-wall-base-slab` payload. Attaches the base of a wall to the top surface of a slab of its building, or frees it again: an attached base follows the slab (a sloped slab included) plus the base offset; the resolved base height is inferred.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetWallBaseSlab {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub slab: Option<String>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetWallBaseSlab {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "wall", kind: "set-wall-base-slab", record: "SetWallBaseSlab" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Attach the base of wall \"{}\" to {}", self.id, self.slab.as_deref().map_or("the storey".to_string(), |slab| format!("slab \"{slab}\""))), &format!("Basis der Wand \"{}\" an {} anbinden", self.id, self.slab.as_deref().map_or("das Geschoss".to_string(), |slab| format!("Decke \"{slab}\""))))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
