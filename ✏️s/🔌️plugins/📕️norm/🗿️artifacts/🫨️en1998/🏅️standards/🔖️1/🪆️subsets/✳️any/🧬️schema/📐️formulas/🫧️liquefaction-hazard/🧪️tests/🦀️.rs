//! 🧪 Worked examples of the liquefaction hazard kernel; the numbers come from `🧰️scripts/en1998-wave3d/1-hand-values.py` (r_u from the arcsine law, Euler load against the finite-difference eigenvalue of a pinned column) and hand calculation.

use super::*;
use crate::part_5::{ProfileLayer, SoilProfile};

fn near(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() <= tolerance * b.abs().max(1.0e-12), "{a} vs {b}");
}

#[test]
fn the_excess_pore_pressure_ratio_is_one_at_and_below_unity_and_falls_with_the_factor_of_safety() {
    assert_eq!(excess_pore_pressure_ratio(0.8, 0.7, 0.34), 1.0);
    assert_eq!(excess_pore_pressure_ratio(1.0, 0.7, 0.34), 1.0);
    near(excess_pore_pressure_ratio(1.25, 0.7, 0.34), 0.4304218152411323, 1e-12);
    near(excess_pore_pressure_ratio(1.5, 0.7, 0.34), 0.2806048911602874, 1e-12);
    near(excess_pore_pressure_ratio(2.0, 0.7, 0.34), 0.14978857401200624, 1e-12);
    near(excess_pore_pressure_ratio(3.0, 0.7, 0.34), 0.06342247293372394, 1e-12);
    assert!(excess_pore_pressure_ratio(f64::NAN, 0.7, 0.34).is_nan());
}

#[test]
fn the_ground_slope_classifies_lateral_spreading() {
    near(slope_percent(1.0), 1.7455064928217585, 1e-12);
    assert_eq!(spreading_class(0.05, 0.1, 6.0), Spreading::Level);
    assert_eq!(spreading_class(0.1, 0.1, 6.0), Spreading::Gentle);
    assert_eq!(spreading_class(6.0, 0.1, 6.0), Spreading::Gentle);
    assert_eq!(spreading_class(6.1, 0.1, 6.0), Spreading::Steep);
}

fn profile() -> SoilProfile {
    let layer = |t: f64, n: f64| ProfileLayer { thickness_m: t, unit_weight_n_m3: 18000.0, shear_wave_velocity_m_s: 150.0, n1_60: n, fines_percent: 5.0 };
    SoilProfile { layers: vec![layer(2.0, 0.0), layer(3.0, 6.0), layer(4.0, 40.0), layer(5.0, 8.0)], water_depth_m: 1.0, magnitude: 7.0, a_g_s_ms2: 3.0, csr_factor: 0.65 }
}

#[test]
fn the_liquefied_layers_are_the_assessed_layers_below_the_required_factor() {
    let layers = liquefied_layers(&profile(), 1.25);
    let rows: Vec<(usize, f64, f64)> = layers.iter().map(|l| (l.index, l.top_m, l.bottom_m)).collect();
    assert_eq!(rows, vec![(1, 2.0, 5.0), (3, 9.0, 14.0)]);
    near(layers[0].safety_factor, 0.2697040547704221, 1e-12);
    near(layers[1].safety_factor, 0.28428918878203546, 1e-12);
    near(liquefied_thickness_m(&layers), 8.0, 1e-12);
    assert!(liquefied_layers(&profile(), 0.2).is_empty());
}

#[test]
fn the_pile_in_liquefied_soil_buckles_over_the_liquefied_length_plus_the_depth_to_fixity() {
    let ei = 30.0e9 * std::f64::consts::PI * 0.8_f64.powi(4) / 64.0;
    near(ei, 603185789.4892403, 1e-12);
    let beta = crate::part_5::pile_beta_per_m(30.0e9, 0.8, 25.0e6);
    near(beta, 0.3017378043092047, 1e-12);
    let l_eff = pile_effective_length_m(4.0, 1.0, beta, 1.0);
    near(l_eff, 7.314135602893344, 1e-12);
    near(pile_buckling_load_n(ei, l_eff), 111281976.62694676, 1e-12);
    near(pile_buckling_load_n(ei, l_eff), 111281404.5929185, 1e-5);
    near(pile_unsupported_moment_nm(100.0e3, l_eff, false), 731413.5602893344, 1e-12);
    near(pile_unsupported_moment_nm(100.0e3, l_eff, true), 365706.7801446672, 1e-12);
}
