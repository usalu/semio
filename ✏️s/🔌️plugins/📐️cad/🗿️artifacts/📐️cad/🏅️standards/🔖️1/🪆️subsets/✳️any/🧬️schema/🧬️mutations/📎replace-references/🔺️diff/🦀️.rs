//! 🔺️ Sparse diff builder for `ReplaceReferences` — the model's list becomes the payload list: base-only ids are removed,
//! payload-only rows are appended, rows present in both are patched field by field, and the payload order is carried only when it
//! differs from "survivors in base order, then the appended rows".
use super::ReplaceReferences;
use crate::diff::{CadDiff, CadReferencesDelta};
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
    let removed: Vec<crate::diff::CadReferenceRemoval> = existing.iter().enumerate().filter(|(_, reference)| !payload.references.iter().any(|next| next.id == reference.id)).map(|(index, reference)| crate::diff::CadReferenceRemoval { id: reference.id.clone(), index }).collect();
    let inserted: Vec<crate::diff::CadReferenceInsertion> = payload.references.iter().enumerate().filter(|(_, next)| !existing.iter().any(|reference| reference.id == next.id)).map(|(index, next)| crate::diff::CadReferenceInsertion { index, row: next.clone() }).collect();
    let modified: Vec<crate::diff::CadReferenceModification> = payload
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
            (patch != CadReferencePatch::default()).then(|| crate::diff::CadReferenceModification { id: next.id.clone(), patch })
        })
        .collect();
    let survivors: Vec<(usize, usize)> = payload.references.iter().enumerate().filter_map(|(after, next)| existing.iter().position(|reference| reference.id == next.id).map(|base| (base, after))).collect();
    let stay = longest_increasing_base_run(&survivors);
    let moved: Vec<crate::diff::CadReferenceRelocation> = survivors.iter().filter(|(base, _)| !stay.contains(base)).map(|(base, after)| crate::diff::CadReferenceRelocation { id: existing[*base].id.clone(), from: *base, to: *after }).collect();
    let mut modified = modified;
    modified.sort_by(|left, right| left.id.cmp(&right.id));
    protocol::MutationOutcome::new(CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), CadReferencesDelta { removed, inserted, moved, modified })])), ..Default::default() })
}

/// 🧮️ The base indices of the longest run of survivors whose base order already matches their after order — the rows that need no move row.
fn longest_increasing_base_run(survivors: &[(usize, usize)]) -> Vec<usize> {
    let mut best: Vec<Vec<usize>> = Vec::with_capacity(survivors.len());
    for (at, (base, _)) in survivors.iter().enumerate() {
        let tail = (0..at).filter(|before| survivors[*before].0 < *base).max_by_key(|before| best[*before].len()).map(|before| best[before].clone()).unwrap_or_default();
        best.push(tail.into_iter().chain(std::iter::once(*base)).collect());
    }
    best.into_iter().max_by_key(|run| run.len()).unwrap_or_default()
}
//#endregion 🔖️Diff
