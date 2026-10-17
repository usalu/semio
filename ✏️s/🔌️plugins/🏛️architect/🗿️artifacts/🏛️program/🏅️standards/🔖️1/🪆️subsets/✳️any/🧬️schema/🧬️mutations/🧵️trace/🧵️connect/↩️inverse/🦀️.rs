//! ↩️ Inverse (undo) construction for the `connect-trace` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🧵traces` per Wave C.

use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ If the id already existed, undo restores its prior full value; otherwise undo disconnects
/// the newly-added edge.
pub fn inverse<Op>(payload: &super::ConnectTrace, base: &ProgramSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::connect_trace::ConnectTrace> + From<crate::standards::v1::subsets::any::schema::mutations::disconnect_trace::DisconnectTrace>,
{
    Ok((|| {
    match base.traces.iter().find(|row| row.id == payload.trace.id) {
        Some(existing) => vec![Op::from(super::ConnectTrace { trace: existing.clone(), index: None })],
        None => vec![Op::from(crate::standards::v1::subsets::any::schema::mutations::disconnect_trace::DisconnectTrace { id: payload.trace.id.clone() })],
    }

    })())
}
