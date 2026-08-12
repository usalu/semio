//! 🔺️ Sparse diff construction for the `compliance_records` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateComplianceRecord, DeleteComplianceRecord, RenameComplianceRecord, ReplaceComplianceRecord};
use crate::artifacts::program::diff::{ProgramComplianceRecordsDelta, ProgramComplianceRecordsPatchEntry};
use crate::artifacts::program::registers::ComplianceRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.compliance_records` on apply.
pub fn diff_create(payload: &CreateComplianceRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { added: vec![payload.compliance_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteComplianceRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameComplianceRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = ComplianceRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { patched: vec![ProgramComplianceRecordsPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceComplianceRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.compliance_records.iter().find(|row| row.header.id == payload.compliance_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.compliance_record).expect("diff_patch always produces a full patch");
    ProgramDiff { compliance_records: Some(ProgramComplianceRecordsDelta { patched: vec![ProgramComplianceRecordsPatchEntry { id: payload.compliance_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
