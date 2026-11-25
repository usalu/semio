use crate::change_fastener_columns::ChangeFastenerColumns;

use semio_s_artifact_norm_en1995_model::En1995Snapshot;
pub fn inverse(payload: &ChangeFastenerColumns, base: &En1995Snapshot) -> Option<crate::change_fastener_columns::ChangeFastenerColumns> {
    match base.fasteners.iter().find(|item| item.id == payload.fastener_id) {
        Some(item) => Some(ChangeFastenerColumns { fastener_id: payload.fastener_id.clone(), new_value: item.columns }),
        None => None,
    }
}
