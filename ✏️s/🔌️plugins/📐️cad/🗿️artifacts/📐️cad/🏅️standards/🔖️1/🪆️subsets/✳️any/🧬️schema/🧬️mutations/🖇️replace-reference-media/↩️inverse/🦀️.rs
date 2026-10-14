//! ↩️ Inverse for `ReplaceReferenceMedia` — recovers the pre-mutation media bundle from `base`.
use super::ReplaceReferenceMedia;
use semio_s_artifact_cad_cad_model::snapshot::CadSnapshot;

//#region 🔖️Inverse
pub fn inverse<Op: From<crate::replace_reference_media::ReplaceReferenceMedia>>(payload: &ReplaceReferenceMedia, base: &CadSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok(base.references_by_model_definition_id
        .get(&payload.model_definition_id)
        .and_then(|references| references.iter().find(|reference| reference.id == payload.reference_id))
        .map(|reference| {
            vec![Op::from(ReplaceReferenceMedia {
                model_definition_id: payload.model_definition_id.clone(),
                reference_id: payload.reference_id.clone(),
                new_source_url: reference.source_url.clone(),
                new_media_kind: reference.media_kind.clone(),
                new_orientation: reference.orientation,
                new_scale: reference.scale,
                new_opacity: reference.opacity,
            })]
        })
        .unwrap_or_default())
}
//#endregion 🔖️Inverse
