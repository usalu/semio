//! ↩️ `change-weld-throat` inverse.

use crate::mutations::change_weld_throat::ChangeWeldThroat;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(payload: &ChangeWeldThroat, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    let t = base.connections.iter().find(|c| c.id == payload.connection_id).map(|c| c.welds.throat).unwrap_or(0.0);
    vec![En1999Mutation::ChangeWeldThroat(ChangeWeldThroat { connection_id: payload.connection_id.clone(), new_throat: t })]
}
