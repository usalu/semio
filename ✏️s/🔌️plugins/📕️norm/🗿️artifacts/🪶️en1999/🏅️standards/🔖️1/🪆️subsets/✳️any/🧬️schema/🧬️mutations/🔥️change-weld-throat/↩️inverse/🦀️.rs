//! ↩️ `change-weld-throat` inverse.

use crate::change_weld_throat::ChangeWeldThroat;

use semio_s_artifact_norm_en1999_model::En1999Snapshot;

pub fn inverse<Op>(payload: &ChangeWeldThroat, base: &En1999Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_weld_throat::ChangeWeldThroat> {
    Ok((|| {
    let t = base.connections.iter().find(|c| c.id == payload.connection_id).map(|c| c.welds.throat).unwrap_or(0.0);
    vec![Op::from(ChangeWeldThroat { connection_id: payload.connection_id.clone(), new_throat: t })]

    })())
}
