//! ↩️ Inverse for `ConnectSynapse` — the `disconnect-synapse` of the id it created (the payload
//! itself carries the id, so no BASE lookup is needed to know what to undo).

use crate::artifacts::generation2d::mutations::{disconnect_synapse, Generation2dMutation};
use crate::artifacts::generation2d::Generation2dSnapshot;

pub fn inverse(payload: &super::ConnectSynapse, _base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    vec![disconnect_synapse(payload.synapse.id.clone())]
}
