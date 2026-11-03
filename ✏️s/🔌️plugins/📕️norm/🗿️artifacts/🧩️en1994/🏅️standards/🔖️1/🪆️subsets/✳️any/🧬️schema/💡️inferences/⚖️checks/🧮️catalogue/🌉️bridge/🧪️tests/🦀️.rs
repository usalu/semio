//! 🧪️ Shear lag and reinforcement range against hand calculations (independent Python derivation in `🧰️scripts/en1994-bridge-oracle.py`).

use super::*;
use super::super::{cracked, delta_sigma_ts, delta_sigma_ts_fatigue};

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / b.abs().max(1e-12)
}

#[test]
fn beta_factors_follow_the_three_branches_of_table_3_1() {
    assert_eq!(beta_sagging(0.01), 1.0);
    assert!(rel(beta_sagging(0.1), 1.0 / 1.064) < 1e-12);
    assert!(rel(beta_sagging(1.0), 1.0 / 5.9) < 1e-12);
    assert!(rel(beta_hogging(0.1), 1.0 / 1.592) < 1e-12);
    assert!(rel(beta_hogging(1.0), 1.0 / 8.6) < 1e-12);
    assert!(rel(beta_end_support(0.1), 0.8 * beta_sagging(0.1)) < 1e-12);
    assert!(beta_end_support(0.7) <= beta_sagging(0.7));
}

#[test]
fn beta_is_continuous_at_the_branch_boundaries() {
    assert!(rel(beta_sagging(0.7), 1.0 / 5.9 / 0.7) < 5e-3);
    assert!(rel(beta_hogging(0.7), 1.0 / 8.6 / 0.7) < 1e-2);
    assert!(rel(beta_sagging(0.02), 1.0) < 5e-3);
}

#[test]
fn reduced_flanges_remove_area_and_stiffness() {
    let sec = ISection { h: 0.5, b: 0.3, tf: 0.02, tw: 0.01, a: 2.0 * 0.3 * 0.02 + 0.46 * 0.01, i_y: 2.0 * (0.3 * 0.02 * 0.24_f64.powi(2) + 0.3 * 0.02_f64.powi(3) / 12.0) + 0.01 * 0.46_f64.powi(3) / 12.0 };
    assert_eq!(shear_lag_section(&sec, 1.0).b, sec.b);
    let reduced = shear_lag_section(&sec, 0.5);
    assert!(rel(reduced.b, 0.01 + 0.145) < 1e-12);
    assert!(rel(reduced.a, 2.0 * 0.155 * 0.02 + 0.46 * 0.01) < 1e-12);
    assert!(reduced.i_y < sec.i_y);
}

#[test]
fn rebar_range_is_the_cracked_section_stress_plus_the_tension_stiffening_increment() {
    let sec = ISection { h: 0.3, b: 0.15, tf: 0.0107, tw: 0.0071, a: 5381e-6, i_y: 83.56e-6 };
    let cr = cracked(&sec, 0.1, 15e-4, 0.04);
    let plain = rebar_fatigue(50e3, 50e3, cr.y_na, 0.04, cr.i, 0.0);
    assert!(rel(plain.delta_equ_pa, 50e3 * (cr.y_na - 0.04) / cr.i) < 1e-12);
    assert!(rel(plain.sigma_max_f_pa, plain.sigma_0_pa) < 1e-12);
    let ts = delta_sigma_ts_fatigue(3.2e6, cr.area, cr.i, sec.area(), sec.i_y, 0.01);
    let with = rebar_fatigue(80e3, 50e3, cr.y_na, 0.04, cr.i, ts);
    assert!(rel(with.delta_equ_pa, plain.delta_equ_pa + ts) < 1e-12);
    assert!(rel(with.sigma_max_f_pa, 80e3 * (cr.y_na - 0.04) / cr.i + ts) < 1e-12);
    assert!(rel(ts * 2.0, delta_sigma_ts(3.2e6, cr.area, cr.i, sec.area(), sec.i_y, 0.01)) < 1e-12);
}

#[test]
fn girder_detailing_limits_follow_the_catalogue_rules() {
    assert!(rel(stud_cross_spacing_min_m(0.022, false), 0.055) < 1e-12);
    assert!(rel(stud_cross_spacing_min_m(0.022, true), 0.088) < 1e-12);
    assert!(rel(min_transverse_reinforcement_m2_per_m(0.25), 5.0e-4) < 1e-12);
}

#[test]
fn end_plate_bearing_is_the_partially_loaded_area_resistance_capped_at_three_times_fcd() {
    let fcd = 0.85 * 30e6 / 1.5;
    let open = end_plate_bearing_n(fcd, 0.2, 0.1, 1.0, 0.25);
    assert!(rel(open, 0.02 * fcd * (0.6_f64 * 0.25 / 0.02).sqrt()) < 1e-12);
    let capped = end_plate_bearing_n(fcd, 0.1, 0.1, 5.0, 5.0);
    assert!(rel(capped, 3.0 * fcd * 0.01) < 1e-12);
    assert!(end_plate_bearing_n(fcd, 0.1, 0.1, 0.1, 0.1) > 0.99 * fcd * 0.01);
}
