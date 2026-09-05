//! ↩️ Inverse for `ReplaceReferenceMedia` — recovers the pre-mutation media bundle from `base`.
use super::ReplaceReferenceMedia;
use crate::artifacts::cad::mutations::CadMutation;
use crate::artifacts::cad::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse(payload: &ReplaceReferenceMedia, base: &CadSnapshot) -> Vec<CadMutation> {
    base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| {
            vec![CadMutation::ReplaceReferenceMedia(ReplaceReferenceMedia {
                model_definition_id: payload.model_definition_id.clone(),
                reference_id: payload.reference_id.clone(),
                new_source_url: reference.source_url.clone(),
                new_media_kind: reference.media_kind.clone(),
                new_orientation: reference.orientation,
                new_scale: reference.scale,
                new_opacity: reference.opacity,
            })]
        })
        .unwrap_or_default()
}
//#endregion 🔖️Inverse
