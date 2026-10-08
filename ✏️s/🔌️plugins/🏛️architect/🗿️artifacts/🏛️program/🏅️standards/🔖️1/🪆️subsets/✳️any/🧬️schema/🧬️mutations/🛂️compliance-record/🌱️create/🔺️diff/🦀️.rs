//! 🔺️ Sparse diff construction for the `create-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::CreateComplianceRecord;
use crate::diff::ProgramComplianceRecordsDelta;
use crate::ProgramDiff;
use crate::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists, Error `mutation.index-out-of-range` if `index` lies past the end (both empty diff); else `added = [payload row]`, plus `reordered` (the base order with the row inserted at `index`) unless the row lands last.
pub fn diff(payload: &CreateComplianceRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = &payload.compliance_record.header.id;
    if base.compliance_records.iter().any(|row| row.header.id == *id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A compliance record already exists with this id.", [id.0.clone()]);
    }
    let length = base.compliance_records.len();
    let at = payload.index.unwrap_or(length);
    if at > length {
        return protocol::MutationOutcome::error("mutation.index-out-of-range", "The index lies beyond the end of the compliance record list.", [id.0.clone()]);
    }
    let reordered = (at < length).then(|| {
        let mut order: Vec<String> = base.compliance_records.iter().map(|row| row.header.id.0.clone()).collect();
        order.insert(at, id.0.clone());
        order
    });
    protocol::MutationOutcome::new(ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { added: vec![payload.compliance_record.clone()], reordered, ..Default::default() }), ..Default::default() })
}
