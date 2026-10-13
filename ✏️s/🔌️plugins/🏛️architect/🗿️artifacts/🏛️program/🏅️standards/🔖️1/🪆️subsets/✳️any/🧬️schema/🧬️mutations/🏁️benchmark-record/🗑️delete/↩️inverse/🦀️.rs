//! ↩️ Inverse (undo) construction for the `delete-benchmark-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🏁benchmarks` per Wave C.

use crate::standards::v1::subsets::any::schema::mutations::ProgramMutation;
use crate::standards::v1::subsets::any::schema::snapshot::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteBenchmarkRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.benchmarks_payload.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateBenchmarkRecord(super::super::create_benchmark_record::CreateBenchmarkRecord { benchmark_record: base.benchmarks_payload[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
