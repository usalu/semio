//! 🔺️ Sparse diff builder for `ReplaceReferenceMedia`.
use super::ReplaceReferenceMedia;
use crate::diff::{CadDiff, CadReferencePatchEntry, CadReferencesDelta};
use crate::mutations::{CadOpacitySet, CadOrientationSet, CadReferencePatch, CadScaleSet};
use crate::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceReferenceMedia, base: &CadSnapshot) -> protocol::MutationOutcome<CadDiff> {
    let Some(existing) = base.references_by_model_definition_id.get(&payload.model_definition_id).and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id)) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Reference \"{}\" does not exist.", payload.reference_id), [payload.model_definition_id.clone(), payload.reference_id.clone()]);
    };
    if existing.source_url == payload.new_source_url && existing.media_kind == payload.new_media_kind && existing.orientation == payload.new_orientation && existing.scale == payload.new_scale && existing.opacity == payload.new_opacity {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", format!("Reference \"{}\" media is already up to date.", payload.reference_id));
    }
    let patch = CadReferencePatch { source_url: Some(payload.new_source_url.clone()), media_kind: Some(payload.new_media_kind.clone()), orientation: Some(CadOrientationSet { value: payload.new_orientation }), scale: Some(CadScaleSet { value: payload.new_scale }), opacity: Some(CadOpacitySet { value: payload.new_opacity }), ..Default::default() };
    protocol::MutationOutcome::new(CadDiff {
        references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), CadReferencesDelta { patched: vec![CadReferencePatchEntry { id: payload.reference_id.clone(), patch }], ..Default::default() })])),
        ..Default::default()
    })
}
//#endregion 🔖️Diff
