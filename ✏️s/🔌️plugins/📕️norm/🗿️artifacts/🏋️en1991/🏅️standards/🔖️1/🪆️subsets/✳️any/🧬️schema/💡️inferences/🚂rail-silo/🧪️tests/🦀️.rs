//! 🧪️ Hand-computed values of the rail and dust formulas against the unverified registry rows.

use super::formulas::*;

fn near(actual: f64, expected: f64, tolerance: f64) { assert!((actual - expected).abs() <= tolerance, "{actual} against {expected}"); }

#[test]
fn derailment_line_loads_follow_the_registry_and_the_lm71_distributed_load() {
    near(derail_line_load_n_m("i", 1.21).expect("situation i"), 45_000.0, 1e-9);
    near(derail_line_load_n_m("ii", 1.0).expect("situation ii"), 1.4 * 80_000.0, 1e-9);
    near(derail_line_load_n_m("ii", 1.21).expect("situation ii"), 1.4 * 1.21 * 80_000.0, 1e-6);
    assert!(derail_line_load_n_m("iii", 1.0).is_none());
    near(derail_length_m(), 20.0, 1e-12);
    near(derail_offset_limit_m(), 1.5 * 1.435, 1e-12);
}

#[test]
fn rail_group_bases_take_the_governing_of_traction_and_braking() {
    near(rail_longitudinal_n(30.0), 990_000.0, 1e-6);
    near(rail_longitudinal_n(100.0), 2_000_000.0, 1e-6);
    near(rail_longitudinal_n(400.0), 6_000_000.0, 1e-6);
    near(rail_nosing_n(1.21), 121_000.0, 1e-6);
    near(footbridge_service_axle_n(), 40_000.0, 1e-9);
}

#[test]
fn the_dust_vent_area_matches_the_independent_python_evaluation() {
    near(dust_vent_area_m2(150.0, 800_000.0, 100.0, 20_000.0, 50_000.0), 3.087294152503173, 1e-9);
    near(dust_vent_area_m2(150.0, 800_000.0, 100.0, 20_000.0, 100_000.0), 2.121524286638777, 1e-9);
    near(dust_vent_area_m2(200.0, 900_000.0, 500.0, 10_000.0, 150_000.0), 5.02517443921499, 1e-9);
}

#[test]
fn dust_inputs_outside_the_validity_range_are_named_by_leaf() {
    assert!(dust_out_of_domain(150.0, 800_000.0, 100.0, 20_000.0, 50_000.0).is_none());
    assert_eq!(dust_out_of_domain(350.0, 800_000.0, 100.0, 20_000.0, 50_000.0).map(|d| d.0), Some("kst"));
    assert_eq!(dust_out_of_domain(150.0, 800_000.0, 20_000.0, 20_000.0, 50_000.0).map(|d| d.0), Some("volume"));
    assert_eq!(dust_out_of_domain(150.0, 800_000.0, 100.0, 60_000.0, 50_000.0).map(|d| d.0), Some("designPressure"));
}
