//! 🏞️ `create-ceiling` payload. Brings a new ceiling onto a storey; its boundary and holes are closed counter-clockwise loops, area, solid and the clear height of the rooms below are inferred.

use crate::{Ceiling, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateCeiling {
    pub id: String,
    pub ceiling: Ceiling,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateCeiling {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "ceiling", kind: "create-ceiling", record: "CreateCeiling" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create ceiling \"{}\"", self.ceiling.name), &format!("Unterdecke \"{}\" anlegen", self.ceiling.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
