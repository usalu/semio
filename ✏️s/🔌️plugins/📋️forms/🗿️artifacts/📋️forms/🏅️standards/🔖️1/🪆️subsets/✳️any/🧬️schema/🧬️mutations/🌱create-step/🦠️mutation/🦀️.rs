//! 🌱️ Forms mutation payload — `create-step`, the `steps` id-keyed collection's `create` verb.
//! Physical dir name (`➕add-step`, wired by `🦀️.rs`, out of this facet's edit boundary) predates
//! the semantic rename; the Rust module is still `add_step`, the type/variant/kind are `create-step`.

use crate::artifacts::forms::{FormMutation, FormStep, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};
use serde::{Deserialize, Serialize};

//#region 🌱️CreateStep
/// 🌱️ Brings a new [`FormStep`] into existence at an optional FINAL-state `index` (`None` appends).
/// A duplicate `step.id` is Fatal `mutation.duplicate-id` (an id-keyed entity that already exists
/// cannot be re-created).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateStep {
    pub step: FormStep,
    pub index: Option<usize>,
}

impl MutationKind<FormsSnapshot, FormMutation> for CreateStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    async fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_create_step(self, base)
    }
    async fn inverse(&self, base: &FormsSnapshot) -> Vec<FormMutation> {
        super::inverse::inverse_create_step(self, base)
    }
    async fn label(&self) -> String {
        format!("Create step \"{}\"", self.step.title)
    }
    async fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
    }
}
//#endregion 🌱️CreateStep
