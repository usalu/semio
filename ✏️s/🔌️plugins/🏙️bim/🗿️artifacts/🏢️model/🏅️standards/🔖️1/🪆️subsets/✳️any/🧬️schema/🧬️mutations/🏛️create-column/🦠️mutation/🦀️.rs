//! 🏛️ `create-column` payload. Brings a new column onto a storey; its height is never stored, it is inferred from the authored top constraint.

use crate::{Column, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateColumn {
    pub id: String,
    pub column: Column,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateColumn {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "column", kind: "create-column", record: "CreateColumn" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create column \"{}\"", self.column.name), &format!("Stütze \"{}\" anlegen", self.column.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
