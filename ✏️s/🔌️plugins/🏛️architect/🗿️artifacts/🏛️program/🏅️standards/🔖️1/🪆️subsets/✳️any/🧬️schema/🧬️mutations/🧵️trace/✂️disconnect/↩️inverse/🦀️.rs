//! ↩️ Inverse (undo) construction for the `disconnect-trace` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧵traces` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo by reconnecting the captured edge. Missing target ⇒ nothing to undo.
pub fn inverse<Op>(payload: &super::DisconnectTrace, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_trace::ConnectTrace>,
{
    Ok((|| {
    match base.traces.iter().position(|row| row.id == payload.id) {
        Some(position) => vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::connect_trace::ConnectTrace { trace: base.traces[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
