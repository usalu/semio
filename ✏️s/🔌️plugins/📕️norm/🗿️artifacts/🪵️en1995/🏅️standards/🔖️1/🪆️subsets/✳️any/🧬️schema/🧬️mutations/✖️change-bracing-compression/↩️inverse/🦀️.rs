use crate::change_bracing_compression::ChangeBracingCompression;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeBracingCompression, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bracing_compression::ChangeBracingCompression> {
    Ok(match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => vec![Op::from(ChangeBracingCompression { bracing_id: payload.bracing_id.clone(), new_value: item.compression_force_n })],
        None => Vec::new(),
    })
}
