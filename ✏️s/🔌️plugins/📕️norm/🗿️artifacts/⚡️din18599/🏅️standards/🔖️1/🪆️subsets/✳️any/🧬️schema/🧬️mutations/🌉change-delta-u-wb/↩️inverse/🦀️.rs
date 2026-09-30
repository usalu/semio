//! ↩️ `change-delta-u-wb` inverse.

use crate::mutations::change_delta_u_wb::ChangeDeltaUWb;
use crate::mutations::Din18599Mutation;
use crate::Din18599Snapshot;

pub fn inverse(payload: &ChangeDeltaUWb, base: &Din18599Snapshot) -> Vec<Din18599Mutation> {
    vec![Din18599Mutation::ChangeDeltaUWb(ChangeDeltaUWb { new_delta_u_wb_w_m2k: base.delta_u_wb_w_m2k })]
}
