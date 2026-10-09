//! 🛝️ `create-ramp` payload. Brings a new ramp onto a storey; its length, rise, slope, landings, solid and compliance are inferred from the authored path, landings and top constraint.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Ramp};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateRamp {
    pub id: String,
    pub ramp: Ramp,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateRamp {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "ramp", kind: "create-ramp", record: "CreateRamp" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create ramp \"{}\"", self.ramp.name), &format!("Rampe \"{}\" anlegen", self.ramp.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
