use crate::change_bracing_brace_width::ChangeBracingBraceWidth;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeBracingBraceWidth, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bracing_brace_width::ChangeBracingBraceWidth> {
    Ok(match base.bracings.iter().find(|item| item.id == payload.bracing_id) {
        Some(item) => vec![Op::from(ChangeBracingBraceWidth { bracing_id: payload.bracing_id.clone(), new_value: item.brace_width_m })],
        None => Vec::new(),
    })
}
