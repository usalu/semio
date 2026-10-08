//! 🧮️ Din4108 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> Din4108Snapshot {
    Din4108Snapshot::default()
}

fn changed() -> Din4108Snapshot {
    let mut other = base();
    other.t_int_c += 1.0;
    other.zones[0].windows.remove(0);
    other.elements[0].layers[1].thickness_m += 0.01;
    other.elements.remove(2);
    other.thermal_bridges.reverse();
    other
}

fn midway() -> Din4108Snapshot {
    let mut other = base();
    other.t_int_c += 1.0;
    other.elements.remove(2);
    other
}

