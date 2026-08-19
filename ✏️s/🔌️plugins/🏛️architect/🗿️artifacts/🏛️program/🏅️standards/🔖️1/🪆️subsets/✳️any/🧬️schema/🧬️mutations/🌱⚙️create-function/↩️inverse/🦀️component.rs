//! ↩️ Inverse (undo) construction for the `create-function` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `⚙️functions` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateFunction, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteFunction(super::super::delete_function::mutation::DeleteFunction { id: payload.function.header.id.clone() })]
}
