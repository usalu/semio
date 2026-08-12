//! 🔺️ Sparse diff construction for the `create-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::mutation::CreateComplianceRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramComplianceRecordsDelta};

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.compliance_records` on apply.
pub fn diff(payload: &CreateComplianceRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { added: vec![payload.compliance_record.clone()], ..Default::default() }), ..Default::default() }
}
