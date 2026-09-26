use crate::mutations::change_delta_c_dev::ChangeDeltaCDev;
use crate::mutations::En1992Mutation;
use crate::En1992Snapshot;

pub fn inverse(_payload: &ChangeDeltaCDev, base: &En1992Snapshot) -> Vec<En1992Mutation> {
    vec![En1992Mutation::ChangeDeltaCDev(ChangeDeltaCDev { new_delta_c_dev: base.delta_c_dev })]
}
