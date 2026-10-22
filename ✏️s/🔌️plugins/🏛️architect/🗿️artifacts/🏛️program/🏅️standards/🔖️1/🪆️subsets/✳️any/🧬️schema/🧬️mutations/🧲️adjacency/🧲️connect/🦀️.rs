//! 🦠️ ProgramSnapshot mutation — `connect-adjacency` leaf (connect). Split from the
//! pre-migration `🗺️set-adjacency` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::standards::v1::subsets::any::schema::registers::Adjacency;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// 🔌️ Upserts an adjacency edge between two elements: normalizes the endpoint pair, replaces the
/// existing edge for that pair if present (keeping its id), otherwise adds a new edge.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
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
impl<Op> MutationKind<ProgramSnapshot, Op> for ConnectAdjacency
where
    Op: protocol::Mutation<ProgramSnapshot, Diff = ProgramDiff> + From<ConnectAdjacency> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_adjacency::DisconnectAdjacency>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "connect", entity: "adjacency", kind: "connect-adjacency", record: "ConnectedAdjacency" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect adjacency between \"{}\" and \"{}\"", self.adjacency.element_a_id.0, self.adjacency.element_b_id.0), &format!("Nachbarschaft zwischen \"{}\" und \"{}\" herstellen", self.adjacency.element_a_id.0, self.adjacency.element_b_id.0))
    }
    fn target(&self) -> Vec<String> {
        vec![self.adjacency.header.id.0.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<ProgramSnapshot, Op> for ConnectAdjacency
where
    Op: protocol::ColdMutation<ProgramSnapshot, ReferenceDiff = ProgramDiff> + From<ConnectAdjacency> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_adjacency::DisconnectAdjacency>,
{
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
