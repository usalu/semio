//! 🗼️ `create-column-type` payload. Brings a new column type into the library; it needs a free id, an existing material and a valid profile.

use crate::{ColumnType, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateColumnType {
    pub id: String,
    pub column_type: ColumnType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateColumnType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "column-type", kind: "create-column-type", record: "CreateColumnType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create column type \"{}\"", self.column_type.name), &format!("Stützentyp \"{}\" anlegen", self.column_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
