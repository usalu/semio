use crate::insert_cutout::InsertCutout;
use crate::remove_cutout;
use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &InsertCutout, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::remove_cutout::RemoveCutout> {
    let at = payload.index.min(base.cutouts.len());
    Ok(vec![Op::from(remove_cutout::RemoveCutout { index: at })])
}
