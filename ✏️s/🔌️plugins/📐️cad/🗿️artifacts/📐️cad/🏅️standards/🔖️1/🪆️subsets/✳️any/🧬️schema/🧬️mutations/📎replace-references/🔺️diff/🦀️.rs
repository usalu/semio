//! 🔺️ Sparse diff builder for `ReplaceReferences` — the model's list becomes the payload list: base-only ids are removed,
//! payload-only rows are appended, rows present in both are patched field by field, and the payload order is carried only when it
//! differs from "survivors in base order, then the appended rows".
use super::ReplaceReferences;
use crate::diff::{CadDiff, CadReferencePatchEntry, CadReferencesDelta};
use crate::mutations::{CadOpacitySet, CadOrientationSet, CadReferencePatch, CadScaleSet};
use crate::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceReferences, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let existing = base.references_by_model_definition_id.get(&payload.model_definition_id).map(Vec::as_slice).unwrap_or_default();
    if payload.references.iter().enumerate().any(|(index, reference)| payload.references[..index].iter().any(|prior| prior.id == reference.id)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", format!("References for \"{}\" must have unique ids.", payload.model_definition_id), [payload.model_definition_id.clone()]);
    }
    if existing == payload.references.as_slice() {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("References for \"{}\" are already up to date.", payload.model_definition_id));
    }
    let removed: Vec<String> = existing.iter().filter(|reference| !payload.references.iter().any(|next| next.id == reference.id)).map(|reference| reference.id.clone()).collect();
    let added: Vec<_> = payload.references.iter().filter(|next| !existing.iter().any(|reference| reference.id == next.id)).cloned().collect();
    let patched: Vec<CadReferencePatchEntry> = payload
        .references
        .iter()
        .filter_map(|next| {
            let reference = existing.iter().find(|reference| reference.id == next.id)?;
            let patch = CadReferencePatch {
                source_url: (reference.source_url != next.source_url).then(|| next.source_url.clone()),
                media_kind: (reference.media_kind != next.media_kind).then(|| next.media_kind.clone()),
                origin: (reference.origin != next.origin).then_some(next.origin),
                orientation: (reference.orientation != next.orientation).then_some(CadOrientationSet { value: next.orientation }),
                scale: (reference.scale != next.scale).then_some(CadScaleSet { value: next.scale }),
                width_world: (reference.width_world != next.width_world).then_some(next.width_world),
                hidden: (reference.hidden != next.hidden).then_some(next.hidden),
                locked: (reference.locked != next.locked).then_some(next.locked),
                opacity: (reference.opacity != next.opacity).then_some(CadOpacitySet { value: next.opacity }),
            };
            (patch != CadReferencePatch::default()).then(|| CadReferencePatchEntry { id: next.id.clone(), patch })
        })
        .collect();
    let natural: Vec<&str> = existing.iter().filter(|reference| !removed.contains(&reference.id)).chain(added.iter()).map(|reference| reference.id.as_str()).collect();
    let wanted: Vec<&str> = payload.references.iter().map(|reference| reference.id.as_str()).collect();
    let reordered = (natural != wanted).then(|| payload.references.iter().map(|reference| reference.id.clone()).collect());
    let mut removed = removed;
    removed.sort();
    let mut patched = patched;
    patched.sort_by(|left, right| left.id.cmp(&right.id));
    protocol::MutationOutcome::new(CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), CadReferencesDelta { added, removed, patched, reordered })])), ..Default::default() })
}
//#endregion 🔖️Diff
