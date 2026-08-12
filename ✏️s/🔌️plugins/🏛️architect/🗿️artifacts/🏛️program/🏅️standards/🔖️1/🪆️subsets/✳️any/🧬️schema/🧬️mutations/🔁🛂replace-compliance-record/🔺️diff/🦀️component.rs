//! 🔺️ Sparse diff construction for the `replace-compliance-record` mutation leaf — real handcrafted
//! `ProgramDiff` builder, never apply-then-capture. Split from `🛂compliance-records` per Wave C.

use super::mutation::ReplaceComplianceRecord;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use crate::artifacts::program::diff::{ProgramComplianceRecordsDelta, ProgramComplianceRecordsPatchEntry};

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff(payload: &ReplaceComplianceRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.compliance_records.iter().find(|row| row.header.id == payload.compliance_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.compliance_record).expect("diff_patch always produces a full patch");
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { patched: vec![ProgramComplianceRecordsPatchEntry { id: payload.compliance_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
