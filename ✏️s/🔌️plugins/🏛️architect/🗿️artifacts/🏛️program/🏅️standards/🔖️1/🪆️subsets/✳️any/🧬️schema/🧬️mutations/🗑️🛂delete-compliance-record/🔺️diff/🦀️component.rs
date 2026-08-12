//! 🔺️ Sparse diff construction for the `delete-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::mutation::DeleteComplianceRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramComplianceRecordsDelta};

/// 🗑️ `removed = [id]`.
pub fn diff(payload: &DeleteComplianceRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}
