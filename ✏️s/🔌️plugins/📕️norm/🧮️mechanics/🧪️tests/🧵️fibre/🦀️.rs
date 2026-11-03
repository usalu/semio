use crate::section::fibre::*;
use semio_s_plugin_norm_compliance::python_oracle;
use std::collections::BTreeMap;
use std::sync::OnceLock;

const LIMITS: Limits = Limits { concrete_yield: 2.0, concrete_ultimate: 3.5, steel_yield: 2.5, steel_ultimate: 45.0 };
const CURVATURES: [f64; 4] = [0.004, 0.008, 0.016, 0.024];

fn close(actual: f64, expected: f64, tol: f64, what: &str) {
    assert!((actual - expected).abs() <= tol * expected.abs().max(1e-300), "{what}: {actual} vs {expected} (rel tol {tol})");
}

fn rc_section(b: f64, h: f64, d: f64, a_s1: f64, a_s2: f64, strength: f64, f_y: f64, displaces: bool) -> FibreSection<'static> {
    let bar = move |area: f64, z: f64| Reinforcement { fibre: Fibre::lumped(z, area), law: Box::new(move |e: f64| elastic_plastic(f_y, 200000.0, e)), displaces };
    let mut reinforcement = vec![bar(a_s1, 0.5 * h - d)];
    if a_s2 > 0.0 {
        reinforcement.push(bar(a_s2, d - 0.5 * h));
    }
    FibreSection { concrete: rectangle_fibres(b, h, 400), concrete_law: Box::new(move |e: f64| parabola_rectangle(strength, 2.0, 2.0, e)), reinforcement, top: 0.5 * h, cache: Default::default() }
}

fn en_member(n_ed: f64) -> (FibreSection<'static>, f64) {
    (rc_section(300.0, 500.0, 450.0, 1800.0, 900.0, 30.0, 500.0, true), n_ed)
}

#[test]
fn rectangle_fibres_integrate_a_linear_law_to_the_closed_form() {
    let (b, h, modulus, kappa, eps_m) = (300.0, 500.0, 30000.0, 0.01, 0.5);
    let section = FibreSection { concrete: rectangle_fibres(b, h, 50), concrete_law: Box::new(move |e: f64| modulus * e / 1000.0), reinforcement: Vec::new(), top: 0.5 * h, cache: Default::default() };
    let (n, m) = section.integrate(eps_m, kappa);
    close(n, modulus * eps_m / 1000.0 * b * h, 1e-12, "N = E eps_m A");
    close(m, modulus * kappa / 1000.0 * b * h.powi(3) / 12.0, 1e-9, "M = E kappa I");
}

#[test]
fn a_linear_cracked_section_yields_at_the_closed_form_curvature() {
    let (b, h, d, area, f_y, alpha) = (300.0, 500.0, 450.0, 1800.0, 500.0, 200000.0 / 30000.0);
    let section = FibreSection {
        concrete: rectangle_fibres(b, h, 4000),
        concrete_law: Box::new(|e: f64| if e > 0.0 { 30000.0 * e / 1000.0 } else { 0.0 }),
        reinforcement: vec![Reinforcement { fibre: Fibre::lumped(0.5 * h - d, area), law: Box::new(move |e: f64| elastic_plastic(f_y, 200000.0, e)), displaces: false }],
        top: 0.5 * h,
        cache: Default::default(),
    };
    let rho = area / (b * d);
    let xi = (alpha * rho * (alpha * rho + 2.0)).sqrt() - alpha * rho;
    let limits = Limits { concrete_yield: 1.0e6, concrete_ultimate: 1.0e6, steel_yield: f_y / 200000.0 * 1000.0, steel_ultimate: 45.0 };
    let c = section.characteristics(0.0, &limits).expect("characteristics");
    close(c.kappa_y, f_y / 200000.0 * 1000.0 / ((1.0 - xi) * d), 2e-3, "phi_y = eps_y / ((1 - xi) d)");
    close(c.m_y, area * f_y * (d - xi * d / 3.0), 3e-3, "M_y = A_s f_y (d - x/3)");
}

#[test]
fn the_yield_and_ultimate_points_sit_on_their_strain_limits() {
    for n_ed in [0.0, 300.0e3, -200.0e3] {
        let (section, n_ed) = en_member(n_ed);
        let c = section.characteristics(n_ed, &LIMITS).expect("characteristics");
        assert!(c.kappa_u > c.kappa_y && c.m_u >= c.m_y && c.m_y > 0.0, "{c:?}");
        let strains = |kappa: f64| {
            let eps_m = section.balance(kappa, n_ed, 35.0).expect("balance");
            (eps_m + kappa * section.top, -(eps_m + kappa * (0.5 * 500.0 - 450.0)))
        };
        let (top_y, steel_y) = strains(c.kappa_y);
        assert!((top_y - 2.0).abs() < 1e-6 || (steel_y - 2.5).abs() < 1e-6, "yield strains {top_y} {steel_y} at N {n_ed}");
        let (top_u, steel_u) = strains(c.kappa_u);
        assert!((top_u - 3.5).abs() < 1e-6 || (steel_u - 45.0).abs() < 1e-6, "ultimate strains {top_u} {steel_u} at N {n_ed}");
        let (_, m_at_yield) = section.integrate(section.balance(c.kappa_y, n_ed, 35.0).expect("balance"), c.kappa_y);
        close(m_at_yield, c.m_y, 1e-12, "M_y is the moment at phi_y");
    }
}

#[test]
fn the_curve_rises_from_the_origin_to_the_ultimate_moment() {
    let (section, n_ed) = en_member(300.0e3);
    let c = section.characteristics(n_ed, &LIMITS).expect("characteristics");
    let curve = section.curve(n_ed, c.kappa_u, 48, 35.0);
    assert_eq!(curve[0], (0.0, 0.0));
    assert!(curve.windows(2).all(|w| w[1].0 > w[0].0 && w[1].1 >= w[0].1 - 1e-3 * c.m_u), "monotone");
    close(curve.last().expect("last").1, c.m_u, 1e-9, "last point is the ultimate moment");
}

#[test]
fn an_axial_force_beyond_the_section_range_has_no_characteristics() {
    let (section, _) = en_member(0.0);
    assert!(section.characteristics(5.0e7, &LIMITS).is_none());
    assert!(section.moment_at(0.01, 5.0e7, 35.0).is_none());
}

fn oracle() -> &'static BTreeMap<String, f64> {
    static CACHE: OnceLock<BTreeMap<String, f64>> = OnceLock::new();
    CACHE.get_or_init(|| {
        let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔮️oracles/🧵️fibre/🐍️.py");
        let out = python_oracle::run_ok(&script, &[], None);
        String::from_utf8_lossy(&out.stdout).lines().filter_map(|l| l.split_once(' ')).map(|(k, v)| (k.to_string(), v.parse().expect("number"))).collect()
    })
}

#[test]
fn moment_curvature_matches_concreteproperties() {
    let oracle = oracle();
    for (name, n_ed) in [("n0", 0.0), ("n300", 300.0e3)] {
        let (section, n_ed) = en_member(n_ed);
        for (i, kappa) in CURVATURES.iter().enumerate() {
            let m = section.moment_at(*kappa, n_ed, 35.0).expect("moment") * 1.0e-6;
            close(m, oracle[&format!("{name}.m_k{i}")], 5e-3, &format!("{name} M(kappa {kappa})"));
        }
        let ultimate_limits = Limits { steel_ultimate: 1.0e6, ..LIMITS };
        let c = section.characteristics(n_ed, &ultimate_limits).expect("characteristics");
        close(c.m_u * 1.0e-6, oracle[&format!("{name}.m_u")], 5e-3, &format!("{name} M_u"));
        close(c.kappa_u, oracle[&format!("{name}.phi_u")], 5e-3, &format!("{name} phi_u"));
    }
}

fn reference_bisection(section: &FibreSection, kappa: f64, n_ed: f64, eps_top_max: f64) -> Option<f64> {
    let (mut lo, mut hi) = (-200.0, eps_top_max - kappa * section.top);
    if hi <= lo || section.integrate(lo, kappa).0 > n_ed || section.integrate(hi, kappa).0 < n_ed {
        return None;
    }
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if section.integrate(mid, kappa).0 < n_ed {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(0.5 * (lo + hi))
}

#[test]
fn the_bracketed_root_finder_matches_the_reference_bisection_on_random_sections() {
    let mut seed = 0x9E37_79B9_7F4A_7C15u64;
    let mut next = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        (seed >> 11) as f64 / (1u64 << 53) as f64
    };
    for _ in 0..40 {
        let (b, h) = (200.0 + 400.0 * next(), 300.0 + 700.0 * next());
        let section = rc_section(b, h, h * (0.85 + 0.1 * next()), 300.0 + 3000.0 * next(), 3000.0 * next(), 20.0 + 40.0 * next(), 400.0 + 200.0 * next(), next() > 0.5);
        for _ in 0..6 {
            let (kappa, n_ed) = (0.002 + 0.03 * next(), (next() - 0.4) * 1.0e6);
            let (fast, slow) = (section.balance(kappa, n_ed, 3.5), reference_bisection(&section, kappa, n_ed, 3.5));
            assert_eq!(fast.is_some(), slow.is_some(), "bracket agreement at kappa {kappa}, N {n_ed}");
            if let (Some(fast), Some(slow)) = (fast, slow) {
                let (a, b) = (section.integrate(fast, kappa).1, section.integrate(slow, kappa).1);
                close(a, b, 1e-9, "moment at the balanced strain");
                assert!((fast - slow).abs() <= 1e-9 * (1.0 + slow.abs()), "{fast} vs {slow}");
            }
        }
    }
}

#[test]
fn characteristics_are_memoised_per_normal_force() {
    let (section, n_ed) = en_member(100.0e3);
    let first = section.characteristics(n_ed, &LIMITS);
    assert_eq!(first, section.characteristics(n_ed, &LIMITS));
}
