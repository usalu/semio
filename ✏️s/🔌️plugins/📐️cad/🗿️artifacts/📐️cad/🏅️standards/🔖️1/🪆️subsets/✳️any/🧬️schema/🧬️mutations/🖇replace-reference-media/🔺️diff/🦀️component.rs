//! 🔺️ Sparse diff builder for `ReplaceReferenceMedia`.
use super::mutation::ReplaceReferenceMedia;
use crate::artifacts::cad::diff::CadDiff;
use crate::artifacts::cad::diff::apply_reference_patch;
use crate::artifacts::cad::mutations::CadReferencePatch;
use crate::artifacts::cad::CadSnapshot;
use std::collections::BTreeMap;

//#region 🔖️Diff
pub fn diff(payload: &ReplaceReferenceMedia, base: &CadSnapshot) -> CadDiff {
    let references = base.references_by_model_definition_id.get(&payload.model_definition_id).cloned().unwrap_or_default();
    let patch = CadReferencePatch {
        source_url: Some(payload.new_source_url.clone()),
        media_kind: Some(payload.new_media_kind.clone()),
        orientation: payload.new_orientation,
        scale: payload.new_scale,
        opacity: payload.new_opacity,
        ..Default::default()
    };
    let next = references
        .into_iter()
        .map(|mut reference| {
            if reference.id == payload.reference_id {
                apply_reference_patch(&mut reference, &patch);
            }
            reference
        })
        .collect();
    CadDiff { references_by_model_definition_id: Some(BTreeMap::from([(payload.model_definition_id.clone(), next)])), ..Default::default() }
}
//#endregion 🔖️Diff
