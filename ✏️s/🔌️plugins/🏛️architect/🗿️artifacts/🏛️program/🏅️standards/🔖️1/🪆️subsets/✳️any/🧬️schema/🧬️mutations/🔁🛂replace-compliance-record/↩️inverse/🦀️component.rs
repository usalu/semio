//! ↩️ Inverse (undo) construction for the `replace-compliance-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛂compliance-records` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a replace by restoring the pre-state row content. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::mutation::ReplaceComplianceRecord, base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    match base.compliance_records.iter().find(|row| row.header.id == payload.compliance_record.header.id) {
        Some(existing) => vec![ProgramMutation::ReplaceComplianceRecord(super::mutation::ReplaceComplianceRecord { compliance_record: existing.clone() })],
        None => Vec::new(),
    }
}
