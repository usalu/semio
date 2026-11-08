use crate::geotechnics::earth_pressure::*;
use crate::geotechnics::slope::{self, Circle, Pseudostatic};
use semio_s_plugin_norm_compliance::python_oracle;
use std::collections::BTreeMap;
use std::sync::OnceLock;

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs().max(1e-300), "{what}: {actual} vs {expected} (rel tol {tol})");
}

fn oracle() -> &'static BTreeMap<String, f64> {
    static CACHE: OnceLock<BTreeMap<String, f64>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/🌍️geotechnics/🐍️.py");
        let out = python_oracle::run_ok(&script, &[], None);
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
    })
}

const GAMMA: f64 = 18_000.0;
const HEIGHT: f64 = 4.0;

#[test]
fn without_acceleration_the_coefficients_are_rankine_and_coulomb() {
    let phi = 30.0_f64.to_radians();
    close(active_coefficient(&Interface::vertical(30.0, 0.0), 0.0).expect("active"), (1.0 - phi.sin()) / (1.0 + phi.sin()), 1e-12, "Rankine Ka");
    close(passive_coefficient(&Interface::vertical(30.0, 0.0), 0.0).expect("passive"), (1.0 + phi.sin()) / (1.0 - phi.sin()), 1e-12, "Rankine Kp");
    let (p, d) = (33.0_f64.to_radians(), 11.0_f64.to_radians());
    let coulomb_a = p.cos().powi(2) / (d.cos() * (1.0 + ((p + d).sin() * p.sin() / d.cos()).sqrt()).powi(2));
    close(active_coefficient(&Interface::vertical(33.0, 11.0), 0.0).expect("active"), coulomb_a, 1e-12, "Coulomb Ka");
    let coulomb_p = p.cos().powi(2) / (d.cos() * (1.0 - ((p + d).sin() * p.sin() / d.cos()).sqrt()).powi(2));
    close(passive_coefficient(&Interface::vertical(33.0, 11.0), 0.0).expect("passive"), coulomb_p, 1e-12, "Coulomb Kp");
}

#[test]
fn a_battered_wall_and_a_sloping_backfill_follow_coulomb() {
    let (phi, delta, alpha, beta) = (32.0_f64, 10.0_f64, 10.0_f64, 10.0_f64);
    let (p, d, a, b) = (phi.to_radians(), delta.to_radians(), alpha.to_radians(), beta.to_radians());
    let root = ((p + d).sin() * (p - b).sin() / ((a + d).cos() * (a - b).cos())).sqrt();
    let coulomb = (p - a).cos().powi(2) / (a.cos().powi(2) * (a + d).cos() * (1.0 + root).powi(2));
    close(active_coefficient(&Interface { phi_deg: phi, delta_deg: delta, wall_deg: 90.0 - alpha, slope_deg: beta }, 0.0).expect("active"), coulomb, 1e-12, "Coulomb with batter and slope");
}

#[test]
fn active_coefficient_thrust_and_critical_plane_match_the_force_polygon() {
    let cases = [("a", 30.0, 0.0, 0.0, 1.0), ("b", 30.0, 0.0, 0.2, 1.0), ("c", 33.0, 11.0, 0.15, 1.075), ("d", 33.0, 11.0, 0.15, 0.925), ("e", 35.0, 17.0, 0.25, 0.9)];
    for (name, phi, delta, kh, vertical) in cases {
        let i = Interface::vertical(phi, delta);
        let t = thrust_for(Side::Active, &i, GAMMA, HEIGHT, Water::NONE, kh, vertical).expect("limit equilibrium");
        close(t.coefficient, oracle()[&format!("mo.active.{name}.k")], 1e-8, name);
        close(t.soil, 0.5 * GAMMA * vertical * HEIGHT * HEIGHT * t.coefficient, 1e-12, "E = ½γ(1±kv)H²K");
        let (rho, force) = critical_wedge(GAMMA, HEIGHT, phi, delta, kh, vertical);
        close(rho, oracle()[&format!("mo.active.{name}.rho")], 1e-4, "critical plane");
        close(force, t.soil, 1e-9, "wedge thrust equals the closed form");
    }
}

#[test]
fn passive_coefficient_matches_the_force_polygon_with_the_inertia_away_from_the_wall() {
    for (name, phi, delta, kh, vertical) in [("a", 30.0, 0.0, 0.0, 1.0), ("b", 30.0, 0.0, 0.1, 1.0), ("c", 33.0, 5.0, 0.1, 1.05)] {
        let t = thrust_for(Side::Passive, &Interface::vertical(phi, delta), GAMMA, HEIGHT, Water::NONE, kh, vertical).expect("passive wedge");
        close(t.coefficient, oracle()[&format!("mo.passive.{name}.k")], 1e-8, name);
    }
    assert!(passive_coefficient(&Interface::vertical(30.0, 0.0), 1.2).is_none(), "θ beyond φ leaves no passive wedge");
}

#[test]
fn the_more_onerous_vertical_sign_governs_and_no_equilibrium_is_none() {
    let i = Interface::vertical(33.0, 11.0);
    let a = Acceleration { kh: 0.15, kv: 0.075 };
    let both = [thrust_for(Side::Active, &i, GAMMA, HEIGHT, Water::NONE, a.kh, 1.0 + a.kv), thrust_for(Side::Active, &i, GAMMA, HEIGHT, Water::NONE, a.kh, 1.0 - a.kv)].map(|t| t.expect("limit equilibrium"));
    let active = thrust(Side::Active, &i, GAMMA, HEIGHT, Water::NONE, a).expect("active");
    close(active.total(), both[0].total().max(both[1].total()), 1e-12, "active takes the larger");
    let passive = thrust(Side::Passive, &i, GAMMA, HEIGHT, Water::NONE, a).expect("passive");
    assert!(passive.total() <= thrust(Side::Passive, &i, GAMMA, HEIGHT, Water::NONE, Acceleration::NONE).expect("static").total());
    assert!(thrust(Side::Active, &Interface::vertical(30.0, 20.0), GAMMA, HEIGHT, Water::NONE, Acceleration { kh: 0.9, kv: 0.0 }).is_none(), "θ beyond φ has no limit equilibrium");
}

#[test]
fn a_permeable_backfill_adds_the_westergaard_thrust_of_its_water() {
    let i = Interface::vertical(32.0, 0.0);
    let (kh, v, wet) = (0.2, 1.0, 3.0);
    let gamma_sat = 20_000.0;
    let water = Water { height_m: wet, saturated_unit_weight: gamma_sat, permeable: true };
    let t = thrust_for(Side::Active, &i, GAMMA, HEIGHT, water, kh, v).expect("thrust");
    close(t.water_dynamic, 7.0 / 12.0 * kh * UNIT_WEIGHT_WATER * wet * wet, 1e-12, "7/12 k_h γ_w H²");
    close(t.water_static, 0.5 * UNIT_WEIGHT_WATER * wet * wet, 1e-12, "hydrostatic");
    let effective = gamma_sat - UNIT_WEIGHT_WATER;
    let k_dry = active_coefficient(&i, kh.atan()).expect("dry");
    let k_wet = active_coefficient(&i, (GAMMA / effective * kh).atan()).expect("wet");
    let dry = HEIGHT - wet;
    close(t.soil, k_dry * 0.5 * GAMMA * dry * dry + k_wet * (GAMMA * dry * wet + 0.5 * effective * wet * wet), 1e-12, "layered soil thrust");
}

#[test]
fn a_backfill_of_low_permeability_moves_with_its_water() {
    let i = Interface::vertical(32.0, 0.0);
    let (kh, gamma_sat) = (0.2, 20_000.0);
    let t = thrust_for(Side::Active, &i, gamma_sat, 3.0, Water { height_m: 3.0, saturated_unit_weight: gamma_sat, permeable: false }, kh, 1.0).expect("thrust");
    let effective = gamma_sat - UNIT_WEIGHT_WATER;
    close(t.soil, 0.5 * effective * 9.0 * active_coefficient(&i, (gamma_sat / effective * kh).atan()).expect("k"), 1e-12, "γ* and θ_w of the saturated inertia");
    assert_eq!(t.water_dynamic, 0.0);
    let dry = thrust_for(Side::Active, &i, GAMMA, 3.0, Water::NONE, kh, 1.0).expect("dry");
    let no_water = thrust_for(Side::Active, &i, GAMMA, 3.0, Water { height_m: 0.0, saturated_unit_weight: 20_000.0, permeable: true }, kh, 1.0).expect("no water");
    assert_eq!(dry, no_water);
    assert!(thrust_for(Side::Active, &i, 9_000.0, 3.0, Water { height_m: 3.0, saturated_unit_weight: 0.0, permeable: false }, kh, 1.0).is_none(), "no effective weight below the water");
}

fn circle() -> Circle {
    Circle { h: 10.0, beta_deg: 30.0, xc: 4.0, yc: 13.0, r: 16.0, gamma: 19_000.0, gamma_sat: 20_000.0, gwl_level: 3.0, ru: 0.0, q: 0.0 }
}

#[test]
fn the_static_factors_match_the_independent_slice_solution() {
    let (tan_phi, c) = (28.0_f64.to_radians().tan() / 1.25, 6_000.0 / 1.25);
    close(slope::bishop(&circle(), tan_phi, c, 120).expect("bishop"), oracle()["slope.bishop.static"], 5e-3, "Bishop");
    close(slope::fellenius(&circle(), tan_phi, c, 120).expect("fellenius"), oracle()["slope.fellenius.static"], 5e-3, "Fellenius");
    let none = Pseudostatic::NONE;
    assert_eq!(slope::bishop_with(&circle(), tan_phi, c, 120, &none), slope::bishop(&circle(), tan_phi, c, 120));
    assert_eq!(slope::janbu_with(&circle(), tan_phi, c, 120, &none), slope::janbu(&circle(), tan_phi, c, 120));
}

#[test]
fn ponded_water_above_the_toe_loads_the_slice_and_the_base_like_the_independent_solution() {
    let ponded = Circle { h: 6.0, beta_deg: 30.0, xc: 6.0, yc: 10.0, r: 11.0, gamma: 18_000.0, gamma_sat: 20_000.0, gwl_level: 2.0, ru: 0.0, q: 0.0 };
    close(slope::janbu(&ponded, 30.0_f64.to_radians().tan(), 5_000.0, 120).expect("janbu"), oracle()["slope.janbu.ponded"], 5e-3, "Janbu ponded");
}

#[test]
fn the_pseudo_static_slice_forces_reduce_the_factors_to_the_worse_vertical_sign() {
    let (tan_phi, c) = (28.0_f64.to_radians().tan() / 1.25, 6_000.0 / 1.25);
    let seismic = Pseudostatic { horizontal: 0.08, vertical: 0.5 };
    let least = |method: &str| oracle()[&format!("slope.{method}.seismic.down")].min(oracle()[&format!("slope.{method}.seismic.up")]);
    close(slope::bishop_with(&circle(), tan_phi, c, 120, &seismic).expect("bishop"), least("bishop"), 5e-3, "Bishop seismic");
    close(slope::fellenius_with(&circle(), tan_phi, c, 120, &seismic).expect("fellenius"), least("fellenius"), 5e-3, "Fellenius seismic");
    let static_f = slope::bishop(&circle(), tan_phi, c, 120).expect("static");
    assert!(slope::bishop_with(&circle(), tan_phi, c, 120, &seismic).expect("seismic") < static_f);
    assert!(slope::janbu_with(&circle(), tan_phi, c, 120, &seismic).expect("janbu") < slope::janbu(&circle(), tan_phi, c, 120).expect("janbu static"));
    let horizontal_only = Pseudostatic { horizontal: 0.08, vertical: 0.0 };
    assert!(slope::bishop_with(&circle(), tan_phi, c, 120, &horizontal_only).expect("horizontal") > slope::bishop_with(&circle(), tan_phi, c, 120, &seismic).expect("both"));
    assert_eq!(seismic.forces(1000.0, -1.0), (80.0, -40.0));
}
