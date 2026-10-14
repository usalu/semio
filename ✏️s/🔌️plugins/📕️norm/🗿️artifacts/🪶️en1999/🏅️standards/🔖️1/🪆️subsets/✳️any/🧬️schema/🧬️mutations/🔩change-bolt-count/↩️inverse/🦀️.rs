//! ↩️ `change-bolt-count` inverse.

use crate::change_bolt_count::ChangeBoltCount;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeBoltCount, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_bolt_count::ChangeBoltCount> {
    Ok((|| {
    let (r, b) = base.connections.iter().find(|c| c.id == payload.connection_id).map(|c| (c.bolts.rows, c.bolts.bolts_per_row)).unwrap_or((1,1));
    vec![Op::from(ChangeBoltCount { connection_id: payload.connection_id.clone(), new_rows: r, new_bolts_per_row: b })]

    })())
}
