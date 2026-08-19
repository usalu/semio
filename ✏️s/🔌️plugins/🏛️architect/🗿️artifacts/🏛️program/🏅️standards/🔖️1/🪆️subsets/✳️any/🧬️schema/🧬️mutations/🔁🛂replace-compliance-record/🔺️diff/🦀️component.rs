//! 🔺️ Sparse diff construction for the `replace-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::mutation::ReplaceComplianceRecord;
use crate::artifacts::program::diff::{ProgramComplianceRecordsDelta, ProgramComplianceRecordsPatchEntry};
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the value is unchanged (both empty diff), else `patched = [{id, full patch}]` via `Patchable::diff_patch`.
pub async fn diff(payload: &ReplaceComplianceRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let Some(existing) = base.compliance_records.iter().find(|row| row.header.id == payload.compliance_record.header.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No compliance record exists with this id.", [payload.compliance_record.header.id.0.clone()]);
    };
    if existing == &payload.compliance_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warn("mutation.no-op", "This compliance record already matches the requested value.").at([existing.header.id.0.clone()])]);
    }
    let patch = existing.diff_patch(&payload.compliance_record).expect("diff_patch always produces a full patch");
    protocol::MutationOutcome::new(ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { patched: vec![ProgramComplianceRecordsPatchEntry { id: payload.compliance_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() })
}
