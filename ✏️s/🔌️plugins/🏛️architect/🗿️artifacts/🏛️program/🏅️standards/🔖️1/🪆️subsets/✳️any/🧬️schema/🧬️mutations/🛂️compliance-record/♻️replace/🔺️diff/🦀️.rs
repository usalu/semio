//! 🔺️ Sparse diff construction for the `replace-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::ReplaceComplianceRecord;
use crate::diff::ProgramComplianceRecordsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🔁️ Error `mutation.target-missing` if absent, Warning `mutation.no-op` if the row is unchanged (both empty diff), else the replacement the kind owns, in place:
/// `removed = [{id, index}]` and `inserted = [{index, payload row}]` at the same coordinate, so the new row keeps its position.
pub fn diff(payload: &ReplaceComplianceRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.compliance_record.header.id;
    let Some(position) = base.compliance_records.iter().position(|row| row.header.id == *id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", "No compliance record exists with this id.", [id.0.clone()]);
    };
    if base.compliance_records[position] == payload.compliance_record {
        return protocol::MutationOutcome::empty().absorb_messages([protocol::MutationMessage::warning("mutation.no-op", "This compliance record already matches the requested value.").at([id.0.clone()])]);
    }
    let mut delta = ProgramComplianceRecordsDelta::removal(&base.compliance_records, position);
    delta.absorb(ProgramComplianceRecordsDelta::insertion(position, payload.compliance_record.clone()));
    protocol::MutationOutcome::new(ProgramDiff { compliance_records: Some(delta), ..Default::default() })
}
