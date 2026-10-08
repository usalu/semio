//! 🎛️ `set-column` payload. Edits a column sparsely: type, position, rotation, base offset, authored top constraint and name; absent fields stay untouched.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Point2, TopConstraint};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetColumn {
    pub id: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Point2>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_offset: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub top: Option<TopConstraint>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "column", kind: "set-column", record: "SetColumn" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Edit column \"{}\"", self.id), &format!("Stütze \"{}\" ändern", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
