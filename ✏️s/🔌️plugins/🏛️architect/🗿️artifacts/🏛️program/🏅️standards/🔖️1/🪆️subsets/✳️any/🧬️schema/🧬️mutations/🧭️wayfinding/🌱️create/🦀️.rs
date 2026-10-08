//! 🦠️ ProgramSnapshot mutation — `create-wayfinding-requirement` leaf (create). Split from the
//! pre-migration `🧭wayfinding` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::registers::WayfindingRequirement;
use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🌱️ Brings a new wayfinding requirement row into existence in `program.wayfinding`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct CreateWayfindingRequirement {
    pub wayfinding_requirement: WayfindingRequirement,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub index: Option<usize>,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for CreateWayfindingRequirement {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "create", entity: "wayfinding-requirement", kind: "create-wayfinding-requirement", record: "CreatedWayfindingRequirement" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Create wayfinding requirement \"{}\"", self.wayfinding_requirement.header.name), &format!("Wegeführungsanforderung \"{}\" erstellen", self.wayfinding_requirement.header.name))
    }
    fn target(&self) -> Vec<String> {
        vec![self.wayfinding_requirement.header.id.0.clone()]
    }
}
