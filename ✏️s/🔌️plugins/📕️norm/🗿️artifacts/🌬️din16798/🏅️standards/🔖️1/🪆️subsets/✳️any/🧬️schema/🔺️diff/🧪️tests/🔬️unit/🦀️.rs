//! 🧮️ Din16798 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> Din16798Snapshot {
    Din16798Snapshot::default()
}

fn changed() -> Din16798Snapshot {
    let mut other = base();
    other.theta_rm_c += 1.0;
    let mut extra = other.zones[0].clone();
    extra.id = "zone-extra".into();
    other.zones.insert(0, extra);
    other.vent_systems[0].heat_recovery_eta += 0.01;
    other
}

fn midway() -> Din16798Snapshot {
    let mut other = base();
    other.theta_rm_c += 1.0;
    other.vent_systems[0].heat_recovery_eta += 0.01;
    other
}

