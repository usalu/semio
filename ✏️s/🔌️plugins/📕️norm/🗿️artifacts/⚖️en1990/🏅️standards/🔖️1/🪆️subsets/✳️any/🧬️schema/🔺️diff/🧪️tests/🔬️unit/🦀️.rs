//! 🧮️ En1990 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> En1990Snapshot {
    En1990Snapshot::default()
}

fn changed() -> En1990Snapshot {
    let mut other = base();
    other.annex = crate::document::AnnexChoice::En;
    other.consequence_class = 3;
    other.permanents.remove(0);
    other.permanents[0].gk += 1.0;
    other.variables.reverse();
    other.effects.truncate(2);
    other.members[0].span = 7.0;
    other
}

fn midway() -> En1990Snapshot {
    let mut other = base();
    other.consequence_class = 3;
    other.variables.reverse();
    other
}

