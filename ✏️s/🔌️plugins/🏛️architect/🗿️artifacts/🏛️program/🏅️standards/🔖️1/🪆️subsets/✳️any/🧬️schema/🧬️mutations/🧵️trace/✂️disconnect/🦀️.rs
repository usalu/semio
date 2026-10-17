//! 🦠️ ProgramSnapshot mutation — `disconnect-trace` leaf (disconnect). Split from the
//! pre-migration `🧵traces` noun-keyed triad per Wave C's one-triad-dir-per-variant
//! restructuring (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️fanout-brief.md`
//! Phase 2). Behavior unchanged from the wave-2 pass — pure directory/module restructuring.

use crate::standards::v1::subsets::any::schema::kernel::EntityId;
use crate::standards::v1::subsets::any::schema::{diff::ProgramDiff, snapshot::ProgramSnapshot};
use protocol::{MutationKind, SemanticDescriptor};

/// ✂️ Removes one trace edge by id.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct DisconnectTrace {
    pub id: EntityId,
}
impl<Op> MutationKind<ProgramSnapshot, Op> for DisconnectTrace
where
    Op: protocol::Mutation<ProgramSnapshot, Diff = ProgramDiff> + From<DisconnectTrace> + From<crate::standards::v1::subsets::any::schema::mutations::connect_trace::ConnectTrace>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "disconnect", entity: "trace", kind: "disconnect-trace", record: "DisconnectedTrace" };
    fn diff(&self, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Disconnect trace \"{}\"", self.id.0), &format!("Nachverfolgung \"{}\" aufheben", self.id.0))
    }
    fn target(&self) -> Vec<String> {
        vec![self.id.0.clone()]
    }
}
