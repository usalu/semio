//! 🧱️ Structural system checks: the per system checks of every building of the full seismic check. See [`super`].

use super::*;

#[path = "🧭️direction-and-base-shear/🦀️.rs"]
mod direction_and_base_shear;
use direction_and_base_shear::{check_system_direction, check_system_base_shear};

#[path = "🏬️storey-forces-and-drift/🦀️.rs"]
mod storey_forces_and_drift;
use storey_forces_and_drift::{check_storey_forces, check_drift_and_second_order};

#[path = "🔗️ductility-and-masonry/🦀️.rs"]
mod ductility_and_masonry;
use ductility_and_masonry::{check_member_ductility, check_simple_masonry};

/// 🧱 Checks every structural system of one building.
pub(super) fn check_building_systems(
    document: &En1998Snapshot,
    annex: AnnexChoice,
    report: &mut CheckReport,
    gamma_i: f64,
    a_g: f64,
    s: f64,
    tb: f64,
    tc: f64,
    td: f64,
    building: &semio_s_artifact_norm_en1998_model::En1998Building,
    bid: &str,
    bpath: &str,
    blabel: &LocalizedCopy,
    masses: &[f64],
    heights: &[f64],
    total_mass: f64,
    t1: f64,
    systems: &[semio_s_artifact_norm_en1998_model::En1998System],
) {
    for system in systems {
        check_system_direction(annex, report, building, bid, bpath, system);
        let (q, f_b) = check_system_base_shear(annex, report, gamma_i, a_g, s, tb, tc, td, building, bid, bpath, total_mass, t1, system);
        let forces = check_storey_forces(annex, report, building, bid, bpath, blabel, masses, heights, system, f_b);
        check_drift_and_second_order(annex, report, building, bid, bpath, system, f_b, &forces);
        check_member_ductility(annex, report, building, bid, bpath, system, q);
        check_simple_masonry(document, annex, report, building, bid, bpath, blabel, system);
    }
}
