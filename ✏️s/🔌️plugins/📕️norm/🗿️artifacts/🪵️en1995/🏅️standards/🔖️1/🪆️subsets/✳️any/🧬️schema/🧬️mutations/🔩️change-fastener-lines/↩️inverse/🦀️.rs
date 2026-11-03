use crate::change_fastener_lines::ChangeFastenerLines;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerLines, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_lines::ChangeFastenerLines> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerLines { fastener_id: payload.fastener_id.clone(), new_value: item.lines })],
        None => Vec::new(),
    })
}
