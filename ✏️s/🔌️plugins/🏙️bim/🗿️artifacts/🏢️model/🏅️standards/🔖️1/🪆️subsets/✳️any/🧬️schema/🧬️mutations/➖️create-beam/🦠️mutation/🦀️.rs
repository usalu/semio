//! ➖️ `create-beam` payload. Brings a new beam onto a storey; its top lies `top_offset` metres from the storey top (positive above, negative below, zero flush), no elevation is stored.

use crate::{Beam, ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct CreateBeam {
    pub id: String,
    pub beam: Beam,
}

impl MutationKind<ModelSnapshot, ModelMutation> for CreateBeam {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "beam", kind: "create-beam", record: "CreateBeam" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create beam \"{}\"", self.beam.name), &format!("Träger \"{}\" anlegen", self.beam.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
