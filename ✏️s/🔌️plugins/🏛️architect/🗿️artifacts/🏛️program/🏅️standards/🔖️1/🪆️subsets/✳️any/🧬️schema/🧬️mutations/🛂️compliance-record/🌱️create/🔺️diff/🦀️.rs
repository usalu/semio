//! 🔺️ Sparse diff construction for the `create-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::CreateComplianceRecord;
use crate::diff::ProgramComplianceRecordsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.target-missing` if `index` lies past the end (both empty diff); else `inserted = [{index, payload row}]`, appended when `index` is absent.
pub fn diff(payload: &CreateComplianceRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.compliance_record.header.id;
    if base.compliance_records.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A compliance record already exists with this id.", [id.0.clone()]);
    }
    let at = payload.index.unwrap_or(base.compliance_records.len());
    if at > base.compliance_records.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", "The index lies beyond the end of the compliance record list.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta::insertion(at, payload.compliance_record.clone())), ..Default::default() })
}
