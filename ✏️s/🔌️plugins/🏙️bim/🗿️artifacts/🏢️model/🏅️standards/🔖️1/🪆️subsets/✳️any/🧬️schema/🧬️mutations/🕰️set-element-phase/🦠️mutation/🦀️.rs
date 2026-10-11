//! 🕰️ `set-element-phase` payload. Puts a phasable element (wall, curtain wall, column, beam, slab, roof, stair, railing, space) into a construction phase: existing, new, demolished or temporary. An opening takes the phase of its host, so only the host is set.

use crate::{ModelDiff, ModelMutation, ModelSnapshot, Phase};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetElementPhase {
    pub id: String,
    pub phase: Phase,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SetElementPhase {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "element", kind: "set-element-phase", record: "SetElementPhase" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set element \"{}\" to phase {:?}", self.id, self.phase), &format!("Element \"{}\" in Phase {:?} setzen", self.id, self.phase))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
