//! 🥶️ `remove-space-conditions` payload. Removes the thermal conditions of a space; the space itself stays, and an unheated space is a thermal boundary of its heated neighbours again.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct RemoveSpaceConditions {
    pub id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for RemoveSpaceConditions {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "space-conditions", kind: "remove-space-conditions", record: "RemoveSpaceConditions" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove conditions of space \"{}\"", self.id), &format!("Bedingungen des Raums \"{}\" entfernen", self.id))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
