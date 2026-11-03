use crate::linalg::*;
use crate::ltb::*;
use crate::*;
use semio_s_plugin_norm_compliance::python_oracle;
use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::sync::atomic::AtomicBool;

const E: f64 = 210e9;
const G: f64 = 81e9;
const RHO: f64 = 7850.0;
const A: f64 = 0.01;
const I: f64 = 8e-5;
const J: f64 = 1e-4;
const FEATURE: &str = include_str!("../🧮️mechanics/🥒️.feature");

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs().max(1e-300), "{what}: {actual} vs {expected} (rel tol {tol})");
}

fn node(id: &str, x: f64, y: f64, z: f64) -> Node {
    Node { id: id.into(), x, y, z }
}

fn member(id: &str, a: &str, b: &str, section: &str) -> Member {
    Member { id: id.into(), node_i: a.into(), node_j: b.into(), material: "S".into(), section: section.into(), ..Default::default() }
}

fn support(node: &str, fixed: [bool; 6]) -> Support {
    Support { node: node.into(), fixed, ..Default::default() }
}

fn base() -> Model {
    let section = |id: &str, scale: f64| Section { id: id.into(), a: A * scale, iy: I * scale, iz: I * scale, j: J * scale, iw: 0.0, asy: 0.0033, asz: 0.0033 };
    Model { materials: vec![Material { id: "S".into(), e: E, g: G, rho: RHO }], sections: vec![section("X", 1.0), section("R", 1e4), Section { a: A * 1e4, ..section("C", 1.0) }], ..Default::default() }
}

fn beam(length: f64) -> Model {
    let mut m = base();
    m.nodes = vec![node("a", 0.0, 0.0, 0.0), node("b", length, 0.0, 0.0)];
    m.members = vec![member("m", "a", "b", "X")];
    m
}

fn force(node: &str, f: Dof6) -> NodalLoad {
    NodalLoad { node: node.into(), force: f }
}

fn case(id: &str) -> LoadCase {
    LoadCase { id: id.into(), ..Default::default() }
}

fn analyse(model: &Model) -> Analysis<'_> {
    Analysis::new(model, &Control::none()).expect("analysis")
}

fn disp<'a>(r: &'a StaticResult, node: &str) -> &'a Dof6 {
    &r.displacements.iter().find(|d| d.node == node).expect("node").u
}

fn reaction<'a>(r: &'a StaticResult, node: &str) -> &'a Dof6 {
    &r.reactions.iter().find(|d| d.node == node).expect("reaction").force
}

const T: bool = true;
const F: bool = false;

fn rows(title: &str) -> Vec<Vec<String>> {
    let start = FEATURE.find(title).expect("outline");
    FEATURE[start..]
        .lines()
        .skip_while(|l| !l.trim().starts_with("Examples:"))
        .skip(2)
        .take_while(|l| l.trim().starts_with('|'))
        .map(|l| l.trim().trim_matches('|').split('|').map(|c| c.trim().to_string()).collect())
        .collect()
}

fn oracle() -> BTreeMap<String, f64> {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/🐍️.py");
    let out = python_oracle::run_ok(&script, &[], None);
    String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
}

fn spd(n: usize) -> Dense {
    let mut seed = 12345_u64;
    let mut next = move || {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((seed >> 33) as f64) / (1u64 << 31) as f64 - 0.5
    };
    let mut b = Dense::zeros(n);
    b.a.iter_mut().for_each(|v| *v = next());
    let mut a = Dense::zeros(n);
    for i in 0..n {
        for j in 0..n {
            a.a[i * n + j] = (0..n).map(|k| b.at(k, i) * b.at(k, j)).sum::<f64>() + if i == j { 1.0 } else { 0.0 };
        }
    }
    a
}

#[test]
fn dense_and_skyline_solvers_agree() {
    let n = 30;
    let a = spd(n);
    let b: Vec<f64> = (0..n).map(|i| (i as f64).sin()).collect();
    let mut x = b.clone();
    a.cholesky(&Control::none()).unwrap().cholesky_solve(&mut x);
    let r = a.mul_vec(&x);
    r.iter().zip(&b).for_each(|(p, q)| assert!((p - q).abs() < 1e-10));
    let mut sky = Skyline::new(vec![0; n]);
    for j in 0..n {
        for i in 0..=j {
            sky.add(i, j, a.at(i, j));
        }
    }
    let original = sky.clone();
    sky.cholesky(&Control::none()).unwrap();
    let mut y = b.clone();
    sky.solve(&mut y);
    x.iter().zip(&y).for_each(|(p, q)| assert!((p - q).abs() < 1e-10));
    original.mul_vec(&y).iter().zip(&b).for_each(|(p, q)| assert!((p - q).abs() < 1e-10));
}

#[test]
fn banded_skyline_solves_tridiagonal() {
    let n = 50;
    let mut sky = Skyline::new((0..n).map(|j: usize| j.saturating_sub(1)).collect());
    for j in 0..n {
        sky.add(j, j, 2.0);
        if j > 0 {
            sky.add(j - 1, j, -1.0);
        }
    }
    let mut x = vec![1.0; n];
    let mut f = sky.clone();
    f.cholesky(&Control::none()).unwrap();
    f.solve(&mut x);
    let (i, expect) = (10, 0.5 * 11.0 * (n as f64 + 1.0 - 11.0));
    close(x[i], expect, 1e-10, "tridiagonal solution");
}

#[test]
fn symmetric_eigen_has_small_residuals() {
    let n = 25;
    let a = spd(n);
    let (values, z) = symmetric_eigen(&a, &Control::none()).unwrap();
    for c in 0..n {
        let v: Vec<f64> = (0..n).map(|r| z.at(r, c)).collect();
        let av = a.mul_vec(&v);
        av.iter().zip(&v).for_each(|(p, q)| assert!((p - values[c] * q).abs() < 1e-9));
    }
    values.windows(2).for_each(|w| assert!(w[0] <= w[1]));
}

#[test]
fn cantilever_tip_load_matches_beam_theory() {
    let l = 3.0;
    let mut m = beam(l);
    m.supports = vec![support("a", [T; 6])];
    m.load_cases = vec![LoadCase { nodal: vec![force("b", [0.0, 0.0, -10e3, 0.0, 0.0, 0.0])], ..case("p") }];
    let r = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
    close(disp(&r, "b")[2], -10e3 * l.powi(3) / (3.0 * E * I), 1e-9, "tip deflection");
    close(reaction(&r, "a")[2], 10e3, 1e-9, "shear reaction");
    close(reaction(&r, "a")[4].abs(), 10e3 * l, 1e-9, "root moment");
    close(r.members[0].diagram[0].my.abs(), 10e3 * l, 1e-9, "diagram root moment");
    assert!(r.members[0].diagram.last().unwrap().my.abs() < 1e-6);
}

#[test]
fn timoshenko_adds_shear_deflection() {
    let l = 2.0;
    let mut m = beam(l);
    m.members[0].shear_deformable = true;
    m.supports = vec![support("a", [T; 6])];
    m.load_cases = vec![LoadCase { nodal: vec![force("b", [0.0, 0.0, -5e3, 0.0, 0.0, 0.0])], ..case("p") }];
    let r = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
    close(disp(&r, "b")[2], -(5e3 * l.powi(3) / (3.0 * E * I) + 5e3 * l / (G * 0.0033)), 1e-9, "Timoshenko tip");
}

#[test]
fn simply_supported_udl_matches_closed_form() {
    let (l, w) = (6.0, 12e3);
    let mut m = beam(l);
    m.supports = vec![support("a", [T, T, T, T, F, F]), support("b", [F, T, T, F, F, F])];
    m.load_cases = vec![LoadCase { distributed: vec![DistributedLoad { member: "m".into(), frame: LoadFrame::Global, start: [0.0, 0.0, -w], end: [0.0, 0.0, -w] }], ..case("q") }];
    let r = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
    close(reaction(&r, "a")[2], w * l / 2.0, 1e-10, "reaction");
    let mid = r.members[0].diagram.iter().map(|d| d.my.abs()).fold(0.0, f64::max);
    close(mid, w * l * l / 8.0, 1e-10, "mid moment");
    assert!(r.members[0].diagram[10].vz.abs() < 1e-6);
    let fine = m.refine(2);
    let f = analyse(&fine).solve_case(&fine.load_cases[0], &Control::none()).unwrap();
    close(disp(&f, "m@1")[2].abs(), 5.0 * w * l.powi(4) / (384.0 * E * I), 1e-9, "midspan deflection");
}

#[test]
fn settlement_and_thermal_loads_match_restrained_beam_theory() {
    let l = 4.0;
    let mut m = beam(l);
    m.supports = vec![support("a", [T; 6]), support("b", [T; 6])];
    m.load_cases = vec![LoadCase { temperature: vec![TemperatureLoad { member: "m".into(), alpha: 1.2e-5, uniform: 30.0, gradient_y: 0.0, gradient_z: 50.0 }], ..case("thermal") }];
    let t = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
    close(reaction(&t, "a")[0].abs(), E * A * 1.2e-5 * 30.0, 1e-9, "restrained expansion");
    close(reaction(&t, "a")[4].abs(), E * I * 1.2e-5 * 50.0, 1e-9, "restrained curvature");
    let mut sink = support("b", [T; 6]);
    sink.prescribed[2] = -0.01;
    m.supports = vec![support("a", [T; 6]), sink];
    m.load_cases = vec![case("settlement")];
    let s = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
    close(reaction(&s, "a")[2].abs(), 12.0 * E * I * 0.01 / l.powi(3), 1e-9, "settlement shear");
    close(reaction(&s, "a")[4].abs(), 6.0 * E * I * 0.01 / l.powi(2), 1e-9, "settlement moment");
}

fn portal(base_fixed: [bool; 6], rigid_beam: bool) -> Model {
    let col = if rigid_beam { "C" } else { "X" };
    let mut m = base();
    m.nodes = vec![node("a", 0.0, 0.0, 0.0), node("b", 0.0, 0.0, 4.0), node("c", 6.0, 0.0, 4.0), node("d", 6.0, 0.0, 0.0)];
    m.members = vec![member("l", "a", "b", col), member("r", "b", "c", if rigid_beam { "R" } else { "X" }), member("s", "c", "d", col)];
    m.supports = vec![support("a", base_fixed), support("d", base_fixed)];
    m.load_cases = vec![LoadCase { nodal: vec![force("b", [20e3, 0.0, 0.0, 0.0, 0.0, 0.0])], ..case("h") }];
    m
}

#[test]
fn portal_frame_sway_matches_closed_forms() {
    for (fixed, factor, what) in [([T; 6], 24.0, "fixed bases"), ([T, T, T, T, F, T], 6.0, "pinned bases")] {
        let m = portal(fixed, true);
        let r = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
        close(disp(&r, "b")[0], 20e3 * 64.0 / (factor * E * I), 2e-3, what);
        close(reaction(&r, "a")[0] + reaction(&r, "d")[0], -20e3, 1e-9, "horizontal equilibrium");
    }
}

#[test]
fn releases_make_a_propped_cantilever() {
    let mut m = beam(2.0);
    m.members[0].release_j = [F, F, F, F, T, T];
    m.supports = vec![support("a", [T; 6]), support("b", [F, T, T, T, T, T])];
    m.load_cases = vec![LoadCase { distributed: vec![DistributedLoad { member: "m".into(), frame: LoadFrame::Global, start: [0.0, 0.0, -1e3], end: [0.0, 0.0, -1e3] }], ..case("q") }];
    let r = analyse(&m).solve_case(&m.load_cases[0], &Control::none()).unwrap();
    close(reaction(&r, "a")[2], 5.0 * 1e3 * 2.0 / 8.0, 1e-9, "propped root shear");
    close(reaction(&r, "b")[2], 3.0 * 1e3 * 2.0 / 8.0, 1e-9, "prop reaction");
    close(reaction(&r, "a")[4].abs(), 1e3 * 4.0 / 8.0, 1e-9, "propped root moment");
    assert!(r.members[0].end_j[4].abs() < 1e-6 && r.members[0].end_j[5].abs() < 1e-6);
}

#[test]
fn mechanism_is_reported_with_its_node() {
    let mut m = beam(2.0);
    m.supports = vec![support("a", [T, T, T, T, T, F])];
    let error = Analysis::new(&m, &Control::none()).err().expect("unstable");
    assert!(matches!(error, MechanicsError::Unstable { .. }), "{error:?}");
}

fn column(support_name: &str) -> Model {
    let mut m = beam(5.0);
    let (a, b) = match support_name {
        "pinned-pinned" => ([T, T, T, T, F, F], [F, T, T, F, F, F]),
        "fixed-free" => ([T; 6], [F; 6]),
        "fixed-pinned" => ([T; 6], [F, T, T, F, F, F]),
        _ => ([T; 6], [F, T, T, T, T, T]),
    };
    m.supports = vec![support("a", a), support("b", b)];
    m.load_cases = vec![LoadCase { nodal: vec![force("b", [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0])], ..case("n") }];
    m.refine(16)
}

#[test]
fn euler_columns_follow_the_feature_table() {
    for row in rows("Scenario Outline: Euler column") {
        let m = column(&row[0]);
        let r = analyse(&m).buckling(&m.load_cases[0], 2, &Control::none()).unwrap();
        let expected = row[1].parse::<f64>().unwrap() * PI * PI * E * I / 25.0;
        close(r.modes[0].alpha_cr, expected, 1e-3, &row[0]);
    }
}

#[test]
fn buckling_mode_shape_is_a_half_sine() {
    let m = column("pinned-pinned");
    let a = analyse(&m);
    let r = a.buckling(&m.load_cases[0], 1, &Control::none()).unwrap();
    let curve = a.member_curve(&r.modes[0].shape, "m#8", 5).unwrap();
    let peak = curve.iter().map(|p| p[1].abs().max(p[2].abs())).fold(0.0, f64::max);
    assert!(peak > 0.9, "mid-span mode shape peaks near 1, got {peak}");
}

#[test]
fn simply_supported_beam_frequencies_follow_the_feature_table() {
    let mut m = beam(6.0);
    m.self_mass = true;
    m.supports = vec![support("a", [T, T, T, T, F, F]), support("b", [F, T, T, F, F, F])];
    let m = m.refine(16);
    let r = analyse(&m).modal(8, MassType::Consistent, &Control::none()).unwrap();
    let base = PI / (2.0 * 36.0) * (E * I / (RHO * A)).sqrt();
    let bending: Vec<&ModalMode> = r.modes.iter().filter(|x| x.shape.iter().any(|s| s.u[1].abs().max(s.u[2].abs()) > 0.5)).collect();
    for row in rows("Scenario Outline: Simply supported beam mode") {
        let mode: usize = row[0].parse().unwrap();
        close(bending[2 * (mode - 1)].frequency_hz, row[1].parse::<f64>().unwrap() * base, 1e-3, &format!("mode {mode}"));
    }
    let first = &bending[0];
    close(first.effective_mass[1].max(first.effective_mass[2]) / r.total_mass[1], 8.0 / (PI * PI), 5e-3, "first effective mass ratio");
    let lumped = analyse(&m).modal(2, MassType::Lumped, &Control::none()).unwrap();
    close(lumped.modes[0].frequency_hz, base, 1e-2, "lumped first frequency");
}

#[test]
fn cantilever_first_frequency_matches_beam_theory() {
    let mut m = beam(3.0);
    m.self_mass = true;
    m.supports = vec![support("a", [T; 6])];
    let m = m.refine(12);
    let r = analyse(&m).modal(1, MassType::Consistent, &Control::none()).unwrap();
    close(r.modes[0].omega, 1.875_104_07_f64.powi(2) * (E * I / (RHO * A)).sqrt() / 9.0, 1e-4, "cantilever omega");
}

fn ltb_member() -> LtbMember {
    let l = 5.0;
    LtbMember { length: l, e: E, g: G, iz: 6.04e-6, iw: 1.26e-7, j: 2.01e-7, end_i: fork_end(), end_j: fork_end(), elements: 32, ..Default::default() }
}

#[test]
fn uniform_moment_ltb_equals_the_closed_form() {
    let mut m = ltb_member();
    m.moment = uniform_moment(1.0, m.length);
    let r = ltb_analyze(&m, &Control::none()).unwrap();
    let mcr = PI / m.length * (E * m.iz * G * m.j).sqrt() * (1.0 + PI * PI * E * m.iw / (m.length * m.length * G * m.j)).sqrt();
    close(r.mcr, mcr, 1e-4, "Mcr");
    let mid = r.shape.iter().find(|p| (p.x - 2.5).abs() < 1e-9).unwrap();
    close(mid.lateral.abs(), 1.0, 1e-3, "mode peak at midspan");
}

#[test]
fn ltb_axial_compression_recovers_euler_and_end_conditions_stiffen() {
    let mut m = ltb_member();
    m.axial = -1.0;
    m.polar_radius_sq = 0.0169;
    let r = ltb_analyze(&m, &Control::none()).unwrap();
    close(r.alpha_cr, PI * PI * E * m.iz / 25.0, 1e-3, "weak axis Euler");
    let mut free = ltb_member();
    free.moment = uniform_moment(1.0, 5.0);
    let pinned = ltb_analyze(&free, &Control::none()).unwrap().alpha_cr;
    free.end_i = fixed_end();
    free.end_j = fixed_end();
    assert!(ltb_analyze(&free, &Control::none()).unwrap().alpha_cr > 1.5 * pinned);
}

#[test]
fn ltb_load_height_is_destabilising_above_and_stabilising_below() {
    let mut m = ltb_member();
    m.moment = uniform_load_moment(1.0, m.length, 64);
    m.line_load = vec![(0.0, 1.0), (m.length, 1.0)];
    let centre = ltb_analyze(&m, &Control::none()).unwrap().alpha_cr;
    m.load_height = 0.15;
    let top = ltb_analyze(&m, &Control::none()).unwrap().alpha_cr;
    m.load_height = -0.15;
    let bottom = ltb_analyze(&m, &Control::none()).unwrap().alpha_cr;
    assert!(top < centre && centre < bottom, "{top} {centre} {bottom}");
}

#[test]
fn kernel_matches_pynite_anastruct_and_scipy() {
    let o = oracle();
    let get = |k: &str| *o.get(k).unwrap_or_else(|| panic!("oracle key {k}"));

    let mut m = beam(6.0);
    m.supports = vec![support("a", [T, T, T, T, F, F]), support("b", [F, T, T, F, F, F])];
    m.load_cases = vec![LoadCase { distributed: vec![DistributedLoad { member: "m".into(), frame: LoadFrame::Global, start: [0.0, 0.0, -12e3], end: [0.0, 0.0, -12e3] }], ..case("q") }];
    let fine = m.refine(8);
    let r = analyse(&fine).solve_case(&fine.load_cases[0], &Control::none()).unwrap();
    close(disp(&r, "m@4")[2].abs(), get("pynite_ss_udl_mid"), 1e-6, "Pynite SS mid");
    close(disp(&r, "m@4")[2].abs(), get("anastruct_ss_udl_mid"), 1e-6, "anastruct SS mid");
    close(reaction(&r, "a")[2], get("pynite_ss_udl_reaction"), 1e-9, "Pynite reaction");

    let p = portal([T; 6], false);
    let r = analyse(&p).solve_case(&p.load_cases[0], &Control::none()).unwrap();
    close(disp(&r, "b")[0], get("pynite_portal_sway"), 1e-6, "Pynite portal sway");
    close(disp(&r, "b")[0], get("anastruct_portal_sway"), 1e-6, "anastruct portal sway");
    close(reaction(&r, "a")[4].abs(), get("pynite_portal_base_moment"), 1e-6, "Pynite base moment");

    let mut l = base();
    l.nodes = vec![node("a", 0.0, 0.0, 0.0), node("b", 3.0, 0.0, 0.0), node("c", 3.0, 2.0, 0.0)];
    l.members = vec![member("m1", "a", "b", "X"), member("m2", "b", "c", "X")];
    l.supports = vec![support("a", [T; 6])];
    l.load_cases = vec![LoadCase { nodal: vec![force("c", [4e3, 0.0, -10e3, 0.0, 0.0, 0.0])], ..case("t") }];
    let r = analyse(&l).solve_case(&l.load_cases[0], &Control::none()).unwrap();
    let c = disp(&r, "c");
    close(c[0], get("pynite_lframe_ux"), 1e-6, "L-frame ux");
    close(c[1], get("pynite_lframe_uy"), 1e-6, "L-frame uy");
    close(c[2], get("pynite_lframe_uz"), 1e-6, "L-frame uz");

    let mut col = beam(5.0);
    col.supports = vec![support("a", [T, T, T, T, F, F]), support("b", [F, T, T, F, F, F])];
    col.load_cases = vec![LoadCase { nodal: vec![force("b", [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0])], ..case("n") }];
    let col = col.refine(4);
    let r = analyse(&col).buckling(&col.load_cases[0], 1, &Control::none()).unwrap();
    close(r.modes[0].alpha_cr, get("pynite_pinned_column_alpha_4el"), 1e-6, "Pynite element matrices");

    let mut beam16 = beam(6.0);
    beam16.self_mass = true;
    beam16.supports = vec![support("a", [T, T, T, T, F, F]), support("b", [F, T, T, F, F, F])];
    let beam16 = beam16.refine(16);
    let r = analyse(&beam16).modal(6, MassType::Consistent, &Control::none()).unwrap();
    close(r.modes[0].frequency_hz, get("pynite_k_scipy_ss_modal_f1"), 1e-6, "modal f1");
    close(r.modes[2].frequency_hz, get("pynite_k_scipy_ss_modal_f2"), 1e-6, "modal f2");
    close(r.modes[0].frequency_hz, get("closed_form_ss_modal_f1"), 1e-3, "closed-form f1");

    let mut m = ltb_member();
    m.moment = uniform_moment(1.0, m.length);
    close(ltb_analyze(&m, &Control::none()).unwrap().mcr, get("ltb_uniform_closed_form"), 1e-4, "ltb uniform");
    for (name, height, point) in [("udl_shear_centre", 0.0, false), ("udl_top_flange", 0.15, false), ("point_shear_centre", 0.0, true), ("point_top_flange", 0.15, true)] {
        let mut m = ltb_member();
        m.load_height = height;
        if point {
            m.moment = central_point_moment(1.0, m.length);
            m.point_loads = vec![(m.length / 2.0, 1.0)];
        } else {
            m.moment = uniform_load_moment(1.0, m.length, 64);
            m.line_load = vec![(0.0, 1.0), (m.length, 1.0)];
        }
        close(ltb_analyze(&m, &Control::none()).unwrap().alpha_cr, get(&format!("ltb_{name}_alpha")), 2e-3, name);
    }
}

#[test]
fn progress_cancellation_and_determinism() {
    let m = column("pinned-pinned");
    let cancel = AtomicBool::new(true);
    let control = Control::none().with_cancel(&cancel);
    assert_eq!(Analysis::new(&m, &control).err(), Some(MechanicsError::Cancelled));
    let seen = std::cell::RefCell::new(Vec::new());
    let sink = |p: &Progress| seen.borrow_mut().push(p.stage.clone());
    let control = Control::none().with_progress(&sink);
    let a = Analysis::new(&m, &control).unwrap().buckling(&m.load_cases[0], 3, &control).unwrap();
    assert!(seen.borrow().iter().any(|s| s == "assemble") && seen.borrow().iter().any(|s| s == "geometric"));
    let b = Analysis::new(&m, &Control::none()).unwrap().buckling(&m.load_cases[0], 3, &Control::none()).unwrap();
    assert_eq!(a, b);
}

#[test]
fn large_models_use_the_skyline_subspace_path() {
    let mut m = beam(6.0);
    m.self_mass = true;
    m.supports = vec![support("a", [T, T, T, T, F, F]), support("b", [F, T, T, F, F, F])];
    let m = m.refine(120);
    let a = analyse(&m);
    assert!(a.system.n_free > DENSE_LIMIT);
    let r = a.modal(2, MassType::Consistent, &Control::none()).unwrap();
    close(r.modes[0].frequency_hz, PI / 72.0 * (E * I / (RHO * A)).sqrt(), 1e-4, "subspace first frequency");
}

#[test]
fn every_schema_type_is_declared_in_every_twin() {
    let rust = include_str!("../../🧬️schema/🦀️.rs");
    let twins = [include_str!("../../🧬️schema/🛰️.proto"), include_str!("../../🧬️schema/🔗️.graphql"), include_str!("../../🧬️schema/🟦️.ts")];
    let names: Vec<&str> = rust.lines().filter_map(|l| l.strip_prefix("pub struct ").or_else(|| l.strip_prefix("pub enum "))).filter_map(|l| l.split([' ', '{']).next()).collect();
    assert!(names.len() > 25);
    for twin in twins {
        for name in &names {
            assert!(twin.contains(&format!(" {name} ")), "{name} missing in a twin");
        }
    }
}

fn strut() -> member_modes::Props {
    member_modes::Props { length: 5.0, e: E, g: G, a: A, iy: 2.0 * I, iz: I, j: J }
}

#[test]
fn member_modes_reproduce_euler_loads_for_the_three_end_conditions() {
    use member_modes::*;
    let euler = PI * PI * E * I / 25.0;
    for (ends, factor) in [(Ends::Pinned, 1.0), (Ends::Cantilever, 0.25), (Ends::Fixed, 4.0)] {
        let (ncr, shape) = buckling(&strut(), ends).unwrap();
        close(ncr, euler * factor, 2e-3, "Ncr");
        let weak = shape.iter().map(|(_, u)| u[1].abs()).fold(0.0, f64::max);
        let strong = shape.iter().map(|(_, u)| u[2].abs()).fold(0.0, f64::max);
        assert!(weak > 0.99 && strong < 1e-6, "{ends:?}: {weak} {strong}");
    }
}

#[test]
fn member_modes_deflection_matches_the_beam_formulas() {
    use member_modes::*;
    let p = strut();
    let mid = |c: &Curve| c.iter().min_by(|a, b| (a.0 - 0.5).abs().total_cmp(&(b.0 - 0.5).abs())).unwrap().1[2];
    close(mid(&deflection(&p, Ends::Pinned, &Loads { udl: 1000.0, ..Default::default() }).unwrap()), -5.0 * 1000.0 * 625.0 / (384.0 * E * 2.0 * I), 1e-3, "udl");
    close(mid(&deflection(&p, Ends::Pinned, &Loads { point: (2.5, 4000.0), ..Default::default() }).unwrap()), -4000.0 * 125.0 / (48.0 * E * 2.0 * I), 1e-3, "point");
    close(mid(&deflection(&p, Ends::Pinned, &Loads { couples: (3000.0, 3000.0), ..Default::default() }).unwrap()), -3000.0 * 25.0 / (8.0 * E * 2.0 * I), 1e-3, "couples");
    close(deflection(&p, Ends::Cantilever, &Loads { point: (5.0, 1000.0), ..Default::default() }).unwrap().last().unwrap().1[2], -1000.0 * 125.0 / (3.0 * E * 2.0 * I), 1e-3, "tip");
}
