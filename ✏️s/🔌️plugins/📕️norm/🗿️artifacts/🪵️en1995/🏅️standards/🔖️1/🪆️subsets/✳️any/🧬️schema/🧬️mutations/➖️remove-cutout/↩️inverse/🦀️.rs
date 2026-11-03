use crate::remove_cutout::RemoveCutout;
use crate::insert_cutout;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &RemoveCutout, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::insert_cutout::InsertCutout> {
    Ok(match base.cutouts.get(payload.index) {
        Some(item) => vec![Op::from(insert_cutout::InsertCutout { index: payload.index, cutout: item.clone() })],
        None => Vec::new(),
    })
}
