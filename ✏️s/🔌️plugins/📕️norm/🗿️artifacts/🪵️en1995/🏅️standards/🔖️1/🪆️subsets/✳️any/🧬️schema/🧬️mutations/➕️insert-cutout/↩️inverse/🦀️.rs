use crate::insert_cutout::InsertCutout;
use crate::remove_cutout;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &InsertCutout, base: &En1995Snapshot) -> Option<crate::remove_cutout::RemoveCutout> {
    let at = payload.index.min(base.cutouts.len());
    Some(remove_cutout::RemoveCutout { index: at })
}
