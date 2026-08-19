//! ↩️ Inverse (undo) construction for the `create-decision` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `✅decisions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateDecision, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteDecision(super::super::delete_decision::mutation::DeleteDecision { id: payload.decision.header.id.clone() })]
}
