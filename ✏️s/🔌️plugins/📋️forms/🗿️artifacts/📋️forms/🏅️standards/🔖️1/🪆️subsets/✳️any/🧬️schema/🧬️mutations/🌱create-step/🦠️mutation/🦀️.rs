//! 🌱️ Forms mutation payload — `create-step`, the `steps` id-keyed collection's `create` verb.
//! Physical dir name (`➕add-step`, wired by `🦀️.rs`, out of this facet's edit boundary) predates
//! the semantic rename; the Rust module is still `add_step`, the type/variant/kind are `create-step`.

use crate::{FormMutation, FormStep, FormsDiff, FormsSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

//#region 🌱️CreateStep
/// 🌱️ Brings a new [`FormStep`] into existence at an optional FINAL-state `index` (`None` appends).
/// A duplicate `step.id` is Fatal `mutation.duplicate-id` (an id-keyed entity that already exists
/// cannot be re-created).
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct CreateStep {
    pub step: FormStep,
    pub index: Option<usize>,
}

impl MutationKind<FormsSnapshot, FormMutation> for CreateStep {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "step", kind: "create-step", record: "CreatedStep" };

    fn diff(&self, base: &FormsSnapshot) -> protocol::MutationOutcome<FormsDiff> {
        super::diff::diff_create_step(self, base)
    }
    fn inverse(&self, base: &FormsSnapshot) -> Result<Vec<FormMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse_create_step(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create step \"{}\"", self.step.title), &format!("Schritt \"{}\" erstellen", self.step.title))
    }
    fn target(&self) -> Vec<String> {
        vec![self.step.id.clone()]
    }
}
//#endregion 🌱️CreateStep
