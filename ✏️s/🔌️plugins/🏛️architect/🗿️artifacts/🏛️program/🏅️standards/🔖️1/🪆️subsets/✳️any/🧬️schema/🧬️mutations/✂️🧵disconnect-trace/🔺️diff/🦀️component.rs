//! 🔺️ Sparse diff construction for the `disconnect-trace` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🧵traces` per Wave C.

use super::mutation::DisconnectTrace;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::ProgramTracesDelta;

/// ✂️ `removed = [id]`.
pub fn diff(payload: &DisconnectTrace, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { traces: Some(ProgramTracesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
