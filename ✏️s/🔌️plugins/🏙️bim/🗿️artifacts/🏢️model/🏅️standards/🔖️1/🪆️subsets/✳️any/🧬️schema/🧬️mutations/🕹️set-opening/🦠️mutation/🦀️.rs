//! 🕹️ `set-opening` payload. Sparsely changes an opening: kind or type, sill override (an assigned null returns to the sill of the type), width and height overrides (an assigned null clears an override), hand and facing flips, name, reveal (an assigned null clears the authored reveal depth, which centres the frame, or the reveal material, which leaves the jambs in the material of the wall layers).

use crate::{Assigned, ModelDiff, ModelMutation, ModelSnapshot, OpeningKind};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetOpening {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<OpeningKind>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub sill_override: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flip_hand: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flip_facing: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reveal_depth: Option<Assigned<Option<f64>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub reveal_material: Option<Assigned<Option<String>>>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetOpening {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "opening", kind: "set-opening", record: "SetOpening" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change opening \"{}\"", self.id), &format!("Öffnung \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
