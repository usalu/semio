//! 🧮️ En1998 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> En1998Snapshot {
    En1998Snapshot::default()
}

fn changed() -> En1998Snapshot {
    let mut other = base();
    other.annex = "en".into();
    other.site.a_gr += 0.1;
    other.buildings[0].plan_regular = !other.buildings[0].plan_regular;
    other.buildings[0].storeys.remove(1);
    let mut extra = other.buildings[0].clone();
    extra.id = "bldg-extra".into();
    other.buildings.push(extra);
    other
}

fn midway() -> En1998Snapshot {
    let mut other = base();
    other.site.a_gr += 0.1;
    other.buildings[0].plan_regular = !other.buildings[0].plan_regular;
    other
}

