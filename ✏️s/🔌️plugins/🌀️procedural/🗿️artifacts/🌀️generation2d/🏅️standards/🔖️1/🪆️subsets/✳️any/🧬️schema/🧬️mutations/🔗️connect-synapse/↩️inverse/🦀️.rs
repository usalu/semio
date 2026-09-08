//! ↩️ Inverse for `ConnectSynapse` — the `disconnect-synapse` of the id it created (the payload
//! itself carries the id, so no BASE lookup is needed to know what to undo).

use crate::standards::v1::subsets::any::schema::mutations::{disconnect_synapse, Generation2dMutation};
use crate::Generation2dSnapshot;

pub fn inverse(payload: &super::ConnectSynapse, _base: &Generation2dSnapshot) -> Vec<Generation2dMutation> {
    vec![disconnect_synapse(payload.synapse.id.clone())]
}
