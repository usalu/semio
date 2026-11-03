use crate::change_fastener_columns::ChangeFastenerColumns;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse<Op>(payload: &ChangeFastenerColumns, base: &En1995Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_fastener_columns::ChangeFastenerColumns> {
    Ok(match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => vec![Op::from(ChangeFastenerColumns { fastener_id: payload.fastener_id.clone(), new_value: item.columns })],
        None => Vec::new(),
    })
}
