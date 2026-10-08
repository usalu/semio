//! 🧮️ En1993 diff algebra: `between`, `inverse` and `absorb` hold their laws over a document changed in every collection.

use super::*;
use protocol::os_spr::protocol_laws::{assert_diff_algebra_inverse_law, assert_mutation_diff_absorb_law};
use protocol::DiffAlgebra;

fn base() -> En1993Snapshot {
    En1993Snapshot::default()
}

fn changed() -> En1993Snapshot {
    let mut other = base();
    other.annex = crate::document::AnnexChoice::En;
    other.materials[0].fy += 1.0;
    let mut extra = other.members[0].clone();
    extra.id = "member-extra".into();
    other.members.insert(0, extra);
    other.load_cases.pop();
    other.member_actions.reverse();
    other.joints.clear();
    other
}

fn midway() -> En1993Snapshot {
    let mut other = base();
    other.materials[0].fy += 1.0;
    other.load_cases.pop();
    other
}

