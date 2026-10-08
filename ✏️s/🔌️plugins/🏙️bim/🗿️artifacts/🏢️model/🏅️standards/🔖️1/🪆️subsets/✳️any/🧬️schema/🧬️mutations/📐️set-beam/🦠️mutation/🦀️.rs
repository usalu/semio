//! 📐️ `set-beam` payload. Edits a beam sparsely: type, start, end, top offset (signed: positive above, negative below the storey top) and name; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetBeam {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub beam_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetBeam {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "beam", kind: "set-beam", record: "SetBeam" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit beam \"{}\"", self.id), &format!("Träger \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
