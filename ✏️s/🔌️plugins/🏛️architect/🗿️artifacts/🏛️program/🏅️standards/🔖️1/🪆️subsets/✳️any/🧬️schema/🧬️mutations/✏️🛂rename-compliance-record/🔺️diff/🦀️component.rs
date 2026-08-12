//! 🔺️ Sparse diff construction for the `rename-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::mutation::RenameComplianceRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramComplianceRecordsDelta, ProgramComplianceRecordsPatchEntry};
use crate::artifacts::program::registers::ComplianceRecordPatch;

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff(payload: &RenameComplianceRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ComplianceRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { patched: vec![ProgramComplianceRecordsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
