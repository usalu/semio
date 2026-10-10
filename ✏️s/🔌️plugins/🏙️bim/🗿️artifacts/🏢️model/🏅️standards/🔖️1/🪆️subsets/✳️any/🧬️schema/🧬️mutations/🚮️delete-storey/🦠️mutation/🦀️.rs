//! 🚮️ `delete-storey` payload. Removes a storey together with everything on it (the views and schedules scoped to it, walls, curtain walls, columns, beams, slabs, roofs, stairs, railings, spaces and the openings of its walls) and their properties and classifications; refuses while a surviving element still constrains its top to the storey.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct DeleteStorey {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for DeleteStorey {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "delete", entity: "storey", kind: "delete-storey", record: "DeleteStorey" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Delete storey \"{}\" with everything on it", self.id), &format!("Geschoss \"{}\" samt Inhalt löschen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
