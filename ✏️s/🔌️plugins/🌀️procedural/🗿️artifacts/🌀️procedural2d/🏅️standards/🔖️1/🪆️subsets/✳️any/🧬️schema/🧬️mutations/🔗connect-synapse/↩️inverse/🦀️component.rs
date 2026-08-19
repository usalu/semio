//! ↩️ Inverse for `ConnectSynapse` — the `disconnect-synapse` of the id it created (the payload
//! itself carries the id, so no BASE lookup is needed to know what to undo).

use crate::artifacts::procedural2d::mutations::{disconnect_synapse, Procedural2dMutation};
use crate::artifacts::procedural2d::Procedural2dSnapshot;

pub async fn inverse(payload: &super::mutation::ConnectSynapse, _base: &Procedural2dSnapshot) -> Vec<Procedural2dMutation> {
    vec![disconnect_synapse(payload.synapse.id.clone())]
}
