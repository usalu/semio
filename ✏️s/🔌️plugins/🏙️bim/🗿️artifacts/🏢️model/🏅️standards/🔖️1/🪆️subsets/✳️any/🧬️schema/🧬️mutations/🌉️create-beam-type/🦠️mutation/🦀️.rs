//! 🌉️ `create-beam-type` payload. Brings a new beam type into the library; it needs a free id, an existing material and a valid profile.

use crate::{BeamType, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateBeamType {
    pub id: String,
    pub beam_type: BeamType,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateBeamType {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "beam-type", kind: "create-beam-type", record: "CreateBeamType" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create beam type \"{}\"", self.beam_type.name), &format!("Trägertyp \"{}\" anlegen", self.beam_type.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
