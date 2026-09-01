//! ↩️ Inverse (undo) construction for the `delete-compliance-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛂compliance-records` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub async fn inverse(payload: &super::DeleteComplianceRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.compliance_records.iter().find(|row| row.header.id == payload.id) {
        Some(existing) => vec![ProgramMutation::CreateComplianceRecord(super::super::create_compliance_record::CreateComplianceRecord { compliance_record: existing.clone() })],
        None => Vec::new(),
    }
}
