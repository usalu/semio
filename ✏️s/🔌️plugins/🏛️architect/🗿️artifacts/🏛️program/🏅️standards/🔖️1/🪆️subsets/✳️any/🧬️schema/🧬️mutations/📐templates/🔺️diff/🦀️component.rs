//! 🔺️ Sparse diff construction for the `templates` mutation leaf — real handcrafted
//! `ProgramDiff` builders, never apply-then-capture.

use super::mutation::{CreateTemplateRecord, DeleteTemplateRecord, RenameTemplateRecord, ReplaceTemplateRecord};
use crate::artifacts::program::diff::{ProgramTemplatesDelta, ProgramTemplatesPatchEntry};
use crate::artifacts::program::registers::TemplateRecordPatch;
use crate::artifacts::program::ProgramDiff;
use crate::artifacts::program::ProgramSnapshot;
use protocol::Patchable;

/// 🌱️ `added = [payload row]` — the row lands at the end of `program.templates` on apply.
pub fn diff_create(payload: &CreateTemplateRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { templates: Some(ProgramTemplatesDelta { added: vec![payload.template_record.clone()], ..Default::default() }), ..Default::default() }
}

/// 🗑️ `removed = [id]`.
pub fn diff_delete(payload: &DeleteTemplateRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    ProgramDiff { templates: Some(ProgramTemplatesDelta { removed: vec![payload.id.0.clone()], ..Default::default() }), ..Default::default() }
}

/// ✏️ `patched = [{id, name: Some(new_name)}]`.
pub fn diff_rename(payload: &RenameTemplateRecord, _base: &ProgramSnapshot) -> ProgramDiff {
    let patch = TemplateRecordPatch { name: Some(payload.new_name.clone()), ..Default::default() };
    ProgramDiff { templates: Some(ProgramTemplatesDelta { patched: vec![ProgramTemplatesPatchEntry { id: payload.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}

/// 🔁️ `patched = [{id, full patch}]` via `Patchable::diff_patch` — every field of the payload
/// row becomes the patch, so applying it fully overwrites the target's non-identity content.
/// Target absent from `base` ⇒ empty diff (nothing to change).
pub fn diff_replace(payload: &ReplaceTemplateRecord, base: &ProgramSnapshot) -> ProgramDiff {
    let Some(existing) = base.templates.iter().find(|row| row.header.id == payload.template_record.header.id) else {
        return ProgramDiff::default();
    };
    let patch = existing.diff_patch(&payload.template_record).expect("diff_patch always produces a full patch");
    ProgramDiff { templates: Some(ProgramTemplatesDelta { patched: vec![ProgramTemplatesPatchEntry { id: payload.template_record.header.id.0.clone(), patch }], ..Default::default() }), ..Default::default() }
}
