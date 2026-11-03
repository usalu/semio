use crate::diagram_kit::{curves::*, interaction_polygon::*, sn_curves::*, spectra::*, stress_blocks::*};
use serde_json::{json, Value};
use std::io::Write;
use std::process::Stdio;

const POINTS: [[f64; 2]; 9] = [[0.0, 3000.0], [150.0, 2600.0], [320.0, 1500.0], [400.0, 700.0], [350.0, 0.0], [200.0, -600.0], [0.0, -900.0], [100.0, 800.0], [250.0, 1200.0]];
const PARAMS: SpectrumParameters = SpectrumParameters { ag: 1.5, s: 1.2, tb: 0.15, tc: 0.5, td: 2.0, eta: 1.0, q: 3.0, beta: 0.2 };

fn oracle(case: &Value) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../📈️diagram-kit/🧪️tests/🔮️oracle/🐍️.py");
    let mut child = semio_s_plugin_norm_compliance::python_oracle::command().arg(path).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().expect("python oracle");
    child.stdin.take().expect("stdin").write_all(case.to_string().as_bytes()).expect("write");
    let out = child.wait_with_output().expect("oracle");
    assert!(out.status.success(), "oracle failed");
    serde_json::from_slice(&out.stdout).expect("json")
}

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-9 * b.abs().max(1.0), "{a} != {b}");
}

fn f(v: &Value) -> f64 {
    v.as_f64().expect("number")
}

fn series(v: &Value) -> Vec<f64> {
    v.as_array().expect("array").iter().map(f).collect()
}

#[test]
fn builders_match_the_numpy_shapely_oracle() {
    let lambdas: Vec<f64> = (0..=30).map(|i| i as f64 * 0.1).collect();
    let periods: Vec<f64> = vec![0.0, 0.05, 0.15, 0.3, 0.5, 1.0, 2.0, 3.0, 4.0];
    let cycles = [1e4, 2e5, 2e6, 3e6, 5e6, 2e7, 1e8, 5e8];
    let minutes = [0.0, 5.0, 30.0, 60.0, 120.0];
    let case = json!({
        "points": POINTS, "origin": [0.0, 0.0], "demand": [120.0, 900.0],
        "lambdas": lambdas, "alpha": 0.34,
        "spectrum": {"ag": PARAMS.ag, "s": PARAMS.s, "tb": PARAMS.tb, "tc": PARAMS.tc, "td": PARAMS.td, "eta": PARAMS.eta, "q": PARAMS.q, "beta": PARAMS.beta},
        "periods": periods, "dc": 80.0, "cycles": cycles, "block": [300.0, 450.0, 150.0, 60.0, 34.0], "minutes": minutes,
    });
    let o = oracle(&case);
    let ring = interaction_polygon(&POINTS);
    near(signed_area(&ring), f(&o["hull"][0]));
    assert_eq!(ring.len() as u64, o["hull"][1].as_u64().expect("count"));
    let hit = utilisation_ray(&ring, [0.0, 0.0], [120.0, 900.0]).expect("hit");
    near(hit.boundary[0], f(&o["ray"][0]));
    near(hit.boundary[1], f(&o["ray"][1]));
    near(hit.utilisation, f(&o["ray"][2]));
    assert_eq!(contains(&ring, [120.0, 900.0]), o["ray"][3].as_bool().expect("bool"));
    for (l, e) in lambdas.iter().zip(series(&o["chi"])) {
        near(buckling_chi(*l, 0.34), e);
    }
    for ((t, e), d) in periods.iter().zip(series(&o["elastic"])).zip(series(&o["design"])) {
        near(elastic(&PARAMS, *t), e);
        near(design(&PARAMS, *t), d);
    }
    for (n, e) in cycles.iter().zip(series(&o["sn"])) {
        near(strength(80.0, *n), e);
    }
    let b = rectangular_block(300.0, 450.0, 150.0, 60.0, 34.0);
    near(b.compression_force, f(&o["block"][0]));
    near(b.lever_arm, f(&o["block"][1]));
    for (kind, name) in [(FireCurve::Iso834, "iso834"), (FireCurve::External, "external"), (FireCurve::Hydrocarbon, "hydrocarbon")] {
        for (m, e) in minutes.iter().zip(series(&o["fire"][name])) {
            near(fire_temperature(kind, *m), e);
        }
    }
}

#[test]
fn hull_is_counter_clockwise_and_contains_inputs() {
    let ring = interaction_polygon(&POINTS);
    assert!(signed_area(&ring) > 0.0);
    assert!(POINTS.iter().all(|p| contains(&ring, *p)));
}

#[test]
fn utilisation_is_one_on_the_boundary_and_half_at_half_distance() {
    let ring = interaction_polygon(&POINTS);
    let hit = utilisation_ray(&ring, [0.0, 0.0], [120.0, 900.0]).expect("hit");
    let half = [hit.boundary[0] / 2.0, hit.boundary[1] / 2.0];
    near(utilisation_ray(&ring, [0.0, 0.0], half).expect("half").utilisation, 0.5);
    near(utilisation_ray(&ring, [0.0, 0.0], hit.boundary).expect("edge").utilisation, 1.0);
    assert!(utilisation_ray(&ring, [0.0, 0.0], [0.0, 0.0]).is_none());
    assert!(utilisation_ray(&ring, [9000.0, 0.0], [1.0, 1.0]).is_none());
}

#[test]
fn spectrum_samples_contain_exact_corner_periods() {
    let curve = spectrum_curve(&PARAMS, SpectrumKind::Elastic, 4.0, 40);
    for t in [0.15, 0.5, 2.0] {
        assert!(curve.iter().any(|p| (p[0] - t).abs() < 1e-12));
    }
    let markers = period_markers(&PARAMS, SpectrumKind::Design);
    assert_eq!(markers.iter().map(|m| m.0).collect::<Vec<_>>(), ["TB", "TC", "TD"]);
}

#[test]
fn sn_curve_vertices_follow_the_detail_category() {
    let curve = sn_curve(71.0, 1e4, 1e9);
    near(strength(71.0, 2.0e6), 71.0);
    near(curve[1][1], 71.0);
    assert!((fatigue_limit(71.0) - 52.4).abs() < 0.1);
}

#[test]
fn stress_block_factors_follow_the_strength_class() {
    assert_eq!(block_factors(30.0), (0.8, 1.0));
    near(block_factors(90.0).0, 0.7);
    near(block_factors(90.0).1, 0.8);
    near(ultimate_strain_permille(90.0), 2.6);
    near(parabola_rectangle_stress(30.0, 17.0, 2.0), 17.0);
}

#[test]
fn buckling_curve_starts_on_the_plateau() {
    let curve = buckling_curve(0.21, 3.0, 30);
    assert_eq!(curve[0], [0.0, 1.0]);
    assert!(curve.iter().any(|p| (p[0] - 0.2).abs() < 1e-12 && p[1] == 1.0));
    assert!(log_space(1.0, 1000.0, 3).iter().zip([1.0, 10.0, 100.0, 1000.0]).all(|(a, b)| (a - b).abs() < 1e-9));
}
