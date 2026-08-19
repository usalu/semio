//! ↩️ Inverse (undo) construction for the `create-compliance-record` mutation leaf — computed from
//! captured pre-state (`base`), never by structurally inverting the diff. Split from
//! `🛂compliance-records` per Wave C.

use crate::artifacts::program::ProgramMutation;
use crate::artifacts::program::ProgramSnapshot;

/// ↩️ Undo a create by deleting the row it added.
pub async fn inverse(payload: &super::mutation::CreateComplianceRecord, _base: &ProgramSnapshot) -> Vec<ProgramMutation> {
    vec![ProgramMutation::DeleteComplianceRecord(super::super::delete_compliance_record::mutation::DeleteComplianceRecord { id: payload.compliance_record.header.id.clone() })]
}
