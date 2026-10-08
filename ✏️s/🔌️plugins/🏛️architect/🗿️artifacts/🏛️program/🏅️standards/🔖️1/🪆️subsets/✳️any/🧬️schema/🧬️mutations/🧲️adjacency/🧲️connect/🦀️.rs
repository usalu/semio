//! 🦠️ ProgramSnapshot mutation — `connect-adjacency` leaf (connect). Split from the
//! pre-migration `🗺️set-adjacency` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::registers::Adjacency;
use crate::{ProgramDiff, ProgramMutation, ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔌️ Upserts an adjacency edge between two elements: normalizes the endpoint pair, replaces the
/// existing edge for that pair if present (keeping its id), otherwise adds a new edge.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct ConnectAdjacency {
    pub adjacency: Adjacency,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub index: Option<usize>,
}
impl MutationKind<ProgramSnapshot, ProgramMutation> for ConnectAdjacency {
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "adjacency", kind: "connect-adjacency", record: "ConnectedAdjacency" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect adjacency between \"{}\" and \"{}\"", self.adjacency.element_a_id.0, self.adjacency.element_b_id.0), &format!("Nachbarschaft zwischen \"{}\" und \"{}\" herstellen", self.adjacency.element_a_id.0, self.adjacency.element_b_id.0))
    }
    fn target(&self) -> Vec<String> {
        vec![self.adjacency.header.id.0.clone()]
    }
}
