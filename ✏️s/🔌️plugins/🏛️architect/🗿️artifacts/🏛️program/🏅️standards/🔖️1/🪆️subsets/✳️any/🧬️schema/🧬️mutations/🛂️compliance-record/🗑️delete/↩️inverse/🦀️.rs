//! ↩️ Inverse (undo) construction for the `delete-compliance-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛂compliance-records` per Wave C.

use crate::ProgramMutation;
use crate::ProgramSnapshot;

/// ↩️ Undo a delete by recreating the captured row. Missing target ⇒ nothing to undo.
pub fn inverse(payload: &super::DeleteComplianceRecord, base: &ProgramSnapshot) -> Result<Vec<ProgramMutation>, semio_framework_value::ValueError> {
    Ok((|| {
    match base.compliance_records.iter().position(|row| row.header.id == payload.id) {
        Some(position) => vec![ProgramMutation::CreateComplianceRecord(super::super::create_compliance_record::CreateComplianceRecord { compliance_record: base.compliance_records[position].clone(), index: Some(position) })],
        None => Vec::new(),
    }

    })())
}
