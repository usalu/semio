//! ↩️ `change-connections` inverse.

use crate::mutations::change_connections::ChangeConnections;
use crate::mutations::En1999Mutation;
use crate::En1999Snapshot;

pub fn inverse(_payload: &ChangeConnections, base: &En1999Snapshot) -> Vec<En1999Mutation> {
    vec![En1999Mutation::ChangeConnections(ChangeConnections { connections: base.connections.clone() })]
}
