//! 🥖️ `split-beam` payload. Splits a beam at the fraction `t` of its length into the original beam, which keeps the first part, and a new beam that continues it with the same type, top offset, phase and name.

use crate::{ModelDiff, ModelMutation, ModelSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SplitBeam {
    pub id: String,
    pub t: f64,
    pub new_id: String,
}

impl MutationKind<ModelSnapshot, ModelMutation> for SplitBeam {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "split", entity: "beam", kind: "split-beam", record: "SplitBeam" };
    fn diff(&self, base: &ModelSnapshot) -> protocol::MutationOutcome<ModelDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ModelSnapshot) -> Result<Vec<ModelMutation>, semio_framework_value::ValueError> {
        Ok(super::inverse::inverse(self, base))
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Split beam \"{}\" at {}", self.id, self.t), &format!("Träger \"{}\" bei {} teilen", self.id, self.t))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.clone()]
    }
}
