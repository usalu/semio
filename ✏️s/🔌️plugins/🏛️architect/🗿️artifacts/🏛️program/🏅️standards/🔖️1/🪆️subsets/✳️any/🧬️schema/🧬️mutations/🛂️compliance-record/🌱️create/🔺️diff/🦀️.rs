//! 🔺️ Sparse diff construction for the `create-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::CreateComplianceRecord;
use crate::artifacts::program::diff::ProgramComplianceRecordsDelta;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;

/// 🌱️ Fatal `mutation.duplicate-id` if the id already exists (empty diff), else `added = [payload row]`.
pub async fn diff(payload: &CreateComplianceRecord, base: &ProgramSnapshot) -> protocol::MutationOutcome<ProgramDiff> {
    let id = payload.compliance_record.header.id.clone();
    if base.compliance_records.iter().any(|row| row.header.id == id) {
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", "A compliance record already exists with this id.", [id.0.clone()]);
    }
    protocol::MutationOutcome::new(ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { added: vec![payload.compliance_record.clone()], ..Default::default() }), ..Default::default() })
}
