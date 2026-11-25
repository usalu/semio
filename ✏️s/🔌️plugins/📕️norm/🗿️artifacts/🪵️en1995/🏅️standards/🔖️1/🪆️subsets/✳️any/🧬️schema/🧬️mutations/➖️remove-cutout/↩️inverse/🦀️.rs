use crate::remove_cutout::RemoveCutout;
use crate::insert_cutout;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &RemoveCutout, base: &En1995Snapshot) -> Option<crate::insert_cutout::InsertCutout> {
    match base.cutouts.get(payload.index) {
        Some(item) => Some(insert_cutout::InsertCutout { index: payload.index, cutout: item.clone() }),
        None => None,
    }
}
