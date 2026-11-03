//! ↩️ Inverse for `change-site-liquefiable-layer`.
use crate::change_site_liquefiable_layer::ChangeSiteLiquefiableLayer;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteLiquefiableLayer, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_liquefiable_layer::ChangeSiteLiquefiableLayer> {
    Ok(vec![Op::from(ChangeSiteLiquefiableLayer { new_liquefiable_layer: base.site.liquefiable_layer })])
}
