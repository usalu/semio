//! 🔲️ `create-grid-line` payload. Brings a new labelled grid line into a building; its label is unique within the building and it has length.

use crate::{GridLine, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateGridLine {
    pub id: String,
    pub grid_line: GridLine,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateGridLine {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "grid-line", kind: "create-grid-line", record: "CreateGridLine" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create grid line \"{}\"", self.grid_line.label), &format!("Rasterlinie \"{}\" anlegen", self.grid_line.label))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
