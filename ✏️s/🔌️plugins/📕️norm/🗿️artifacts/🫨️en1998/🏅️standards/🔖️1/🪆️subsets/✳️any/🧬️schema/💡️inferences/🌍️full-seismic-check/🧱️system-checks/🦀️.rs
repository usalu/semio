//! 🧱️ Structural system checks: the per system checks of every building of the full seismic check. See [`super`].

use semio_s_artifact_norm_en1998_model::En1998Snapshot;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckReport, CheckResult, ClauseId, LocalizedCopy, Quantity, Remedy, SubjectRef};
use semio_s_artifact_norm_en1998_schema::{na_de, part_1, wall_plan};
use semio_s_artifact_norm_en1998_inferences_details::seismic_helpers::*;

#[path = "🧭️direction-and-base-shear/🦀️.rs"]
mod direction_and_base_shear;
use direction_and_base_shear::{check_system_direction, check_system_base_shear};

#[path = "🏬️storey-forces-and-drift/🦀️.rs"]
mod storey_forces_and_drift;
use storey_forces_and_drift::{check_storey_forces, check_drift_and_second_order};

#[path = "🔗️ductility-and-masonry/🦀️.rs"]
mod ductility_and_masonry;
use ductility_and_masonry::{check_member_ductility, check_simple_masonry, check_global_ductility_and_diaphragm};

#[path = "🔩️critical-regions/🦀️.rs"]
mod critical_regions;
use critical_regions::check_critical_regions;

#[path = "🪧️horizontal-elements/🦀️.rs"]
pub(crate) mod horizontal_elements;
use horizontal_elements::{check_diaphragms, check_overturning, check_time_history};

/// 🧱 Checks every structural system of one building.
pub fn check_building_systems(
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
    let mut directional: Vec<(String, String, f64, f64)> = Vec::new();
    check_time_history(document, annex, report, gamma_i, a_g, s, tb, tc, td, building, bid, t1);
    let has_diaphragms = document.diaphragms.iter().any(|d| d.building_id == building.id);
    let spectrum_ndps = resolve_annex(document).map(|params| params.ndp_ids()).unwrap_or_default();
    for system in systems {
        check_system_direction(annex, report, building, bid, bpath, system);
        let (q, f_b) = check_system_base_shear(annex, report, gamma_i, a_g, s, tb, tc, td, building, bid, bpath, total_mass, t1, system, &spectrum_ndps);
        directional.push((system.id.clone(), system.direction.token().to_string(), f_b, system.base_shear_resistance_n));
        let forces = check_storey_forces(annex, report, building, bid, bpath, blabel, masses, heights, system, f_b);
        check_drift_and_second_order(annex, report, building, bid, bpath, system, f_b, &forces);
        check_member_ductility(annex, report, building, bid, bpath, system, q);
        check_overturning(annex, report, building, bid, bpath, blabel, system, &forces, heights);
        check_diaphragms(document, annex, report, building, bid, system, &forces);
        check_global_ductility_and_diaphragm(annex, report, bid, bpath, blabel, system, q, t1, tc, &forces, has_diaphragms);
        check_simple_masonry(document, annex, report, building, bid, bpath, blabel, system);
    }
    check_critical_regions(document, annex, report, building, bid, systems, t1, tc);
    let first = |d: &str| directional.iter().find(|e| e.1 == d);
    if let (Some(x), Some(y)) = (first("x"), first("y")) {
        for (own, other) in [(x, y), (y, x)] {
            let e_comb = part_1::horizontal_component_envelope(own.2, other.2);
            let mut c = CheckResult::assess(
                format!("en1998.1.{bid}.{}.componentCombination", own.0),
                "DIN EN 1998-1",
                ClauseId::new("EN 1998-1", "1", "4.3.3.5.1"),
                SubjectRef::new(bid, format!("{bpath}.systems[id={}].baseShearResistanceN", own.0), blabel.clone()),
                lc("Combined horizontal components E + 0.30·E⊥ ≤ V_Rd", "Kombination der Horizontalkomponenten E + 0,30·E⊥ ≤ V_Rd"),
            )
            .utilization(q_force(e_comb), floor_limit(q_force(own.3)))
            .annex(annex)
            .explanation(lc(
                &format!("E={:.0} N, E⊥={:.0} N (system {}), envelope={e_comb:.0} N, V_Rd={:.0} N", own.2, other.2, other.0, own.3),
                &format!("E={:.0} N, E⊥={:.0} N (System {}), Einhüllende={e_comb:.0} N, V_Rd={:.0} N", own.2, other.2, other.0, own.3),
            ));
            if e_comb > own.3 {
                c = c.remedy(Remedy::at_least(
                    SubjectRef::new(bid, format!("{bpath}.systems[id={}].baseShearResistanceN", own.0), lc("Base shear resistance", "Fundamentschubwiderstand")),
                    q_force(own.3),
                    q_force(e_comb),
                    lc(&format!("Increase the base shear resistance to at least {e_comb:.0} N."), &format!("Fundamentschubwiderstand auf mindestens {e_comb:.0} N erhöhen.")),
                ));
            }
            report.push(c.build());
        }
    }
}
