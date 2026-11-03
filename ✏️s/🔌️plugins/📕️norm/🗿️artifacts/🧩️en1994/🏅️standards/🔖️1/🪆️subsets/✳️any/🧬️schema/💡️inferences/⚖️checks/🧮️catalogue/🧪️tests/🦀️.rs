//! 🧪️ Spec worked examples E1-01..E1-12 for the EN 1994 catalogue formulas.

use super::*;

fn ipe300() -> ISection {
    ISection { h: 0.300, b: 0.150, tf: 0.0107, tw: 0.0071, a: 5381e-6, i_y: 83.56e-6 }
}

fn rel(a: f64, b: f64) -> f64 {
    (a - b).abs() / b.abs().max(1e-12)
}

#[test]
fn e1_01_effective_width() {
    assert!(rel(effective_width_m(8.0, 0.0, 1.5, 1.5, false), 2.0) < 1e-12);
    assert!(rel(effective_width_m(8.0, 0.0, 1.5, 1.5, true), 1.5) < 1e-12);
    assert!(rel(effective_width_m(16.0, 0.0, 1.5, 1.5, false), 3.0) < 1e-12);
}

#[test]
fn e1_02_full_plastic_moment() {
    let p = plastic_full(&ipe300(), 355e6, 0.100, 2.0, 0.100, 17e6);
    assert!(rel(p.m_nm, 423.90e3) < 5e-4, "{}", p.m_nm);
    assert_eq!(p.zone, "slab");
    assert!(rel(p.x_m, 0.05618) < 1e-3);
}

#[test]
fn e1_03_partial_connection_invariant() {
    let sec = ipe300();
    let full = plastic_full(&sec, 355e6, 0.100, 2.0, 0.100, 17e6);
    let ma = 628.4e-6 * 355e6;
    let lin = plastic_linear(ma, full.m_nm, 0.5);
    assert!(rel(lin, 323.49e3) < 1e-3, "{lin}");
    let sb = plastic_sagging(&sec, 355e6, 0.100, 2.0, 0.100, 17e6, 0.5 * full.nc_n);
    assert!(rel(sb.m_nm, 364.35e3) < 2e-3, "{}", sb.m_nm);
    assert!(sb.m_nm >= lin);
    assert_eq!(sb.zone, "flange");
    assert!(rel(eta_min(8.0, 355e6, true), 0.49) < 1e-12);
}

#[test]
fn required_force_is_monotone_and_inverts_stress_block() {
    let sec = ipe300();
    let full = plastic_full(&sec, 355e6, 0.100, 2.0, 0.100, 17e6);
    let target = 364.35e3;
    let nc = required_concrete_force(&sec, 355e6, 0.100, 2.0, 0.100, 17e6, target);
    assert!(rel(nc, 0.5 * full.nc_n) < 2e-3);
    assert_eq!(required_concrete_force(&sec, 355e6, 0.100, 2.0, 0.100, 17e6, 1.0), 0.0);
    assert_eq!(required_concrete_force(&sec, 355e6, 0.100, 2.0, 0.100, 17e6, 1e9), full.nc_n);
}

#[test]
fn e1_04_stud_resistance() {
    let (p1, p2, p) = stud_resistance_n(0.019, 0.100, 450e6, 30e6, 33e9, 1.25);
    assert!(rel(p1, 81.656e3) < 1e-4);
    assert!(rel(p2, 83.332e3) < 1e-3);
    assert_eq!(p, p1);
    let kt = sheeting_reduction(0.050, 0.100, 0.095, 0.019, 0.0009, 1, false).unwrap();
    assert!(rel(kt, 0.85) < 1e-12);
    assert!(rel(p * kt, 69.41e3) < 1e-3);
    assert!(sheeting_reduction(0.050, 0.100, 0.095, 0.019, 0.0009, 3, false).is_none());
}

#[test]
fn e1_05_slab_longitudinal_shear() {
    let (v, vmax, asf) = slab_shear(81_656.0 / 0.150, 0.200, 0.100, 30e6, 1.5, 1.15, 1.0);
    assert!(rel(v, 2.722e6) < 1e-3);
    assert!(rel(vmax, 5.28e6) < 1e-3);
    assert!(rel(asf, 626e-6) < 2e-3);
}

#[test]
fn e1_06_shear_interaction() {
    let sec = ipe300();
    let (_, rho) = (0.0, shear_rho(400e3, 526.33e3));
    assert!(rel(rho, 0.2704) < 1e-3);
}

#[test]
fn e1_07_filled_column_buckling() {
    let ia = 121.583e-6;
    let ic = 418.690e-6;
    let ei = ei_eff(ia, 0.0, ic, 33e9, 0.6);
    assert!(rel(ei, 3.3822e7) < 1e-3);
    let ncr = PI * PI * ei / 16.0;
    assert!(rel(ncr, 20863e3) < 1e-3);
    let chi = chi_column((5676.9e3 / ncr).sqrt(), 0.21);
    assert!(rel(chi, 0.9174) < 1e-3);
    let (ea, ec) = confinement(0.52, 0.0);
    assert_eq!((ea, ec), (1.0, 0.0));
    let (ea, ec) = confinement(0.3, 0.0);
    assert!(rel(ea, 0.9) < 1e-12 && ec > 0.0);
}

#[test]
fn e1_08_interaction_polygon() {
    let s = ColSection { b: 0.2, t: 0.01, as_m2: 0.0, yr: 0.0, fyd: 355e6, fcd: 26.667e6, fsd: 0.0 };
    assert!(rel(s.n_pl(), 3562e3) < 1e-3);
    assert!(rel(s.n_pm(), 864e3) < 1e-3);
    let poly = interaction_polygon(&s);
    assert!(rel(poly[2].1, 211.85e3) < 1e-3, "{}", poly[2].1);
    assert!(rel(poly[3].1, 206.94e3) < 2e-3, "{}", poly[3].1);
    assert!(rel(mu_d(&poly, 1700e3), 0.690) < 2e-3);
    assert!(rel(mu_d(&poly, 600e3), 1.0145) < 2e-3);
}

#[test]
fn e1_09_composite_slab() {
    let s = SlabSection { h: 0.150, hp: 0.050, ap: 1100e-6, ep: 0.020, e: 0.025, fyp_d: 350e6, fcd: 20e6 };
    let (m, _, _) = slab_bending(&s, 4.0e3, f64::INFINITY);
    assert!(rel(m, 43.77e3) < 1e-3);
    let (m, _, _) = slab_bending(&s, 4.0e3, 0.6 * 385e3);
    assert!(rel(m, 29.77e3) < 2e-3);
    assert!(rel(slab_mk(0.125, 1100e-6, 1.0, 200e6, 0.04e6, 1.25), 26.0e3) < 1e-9);
    assert!(rel(slab_vertical_shear(0.400, 0.125, 1100e-6, 30e6, 1.5), 46.98e3) < 2e-3);
}

#[test]
fn e1_10_deflection_and_shrinkage() {
    let sec = ipe300();
    let n0 = modular_ratio(33e9, 0.0, 0.0);
    let nl = modular_ratio(33e9, 1.1, 2.5);
    let ns = modular_ratio(33e9, 0.55, 2.5);
    let e0 = transformed(&sec, 0.100, 2.0, 0.100, n0);
    let el = transformed(&sec, 0.100, 2.0, 0.100, nl);
    let es = transformed(&sec, 0.100, 2.0, 0.100, ns);
    assert!(rel(e0.i, 293.53e-6) < 2e-3, "{}", e0.i);
    assert!(rel(el.i, 221.62e-6) < 2e-3, "{}", el.i);
    assert!(rel(deflection_simple(8.0, 6e3, 4e3, el.i, e0.i), 10.34e-3) < 5e-3);
    let zc = es.y_na - 0.050;
    assert!(rel(deflection_shrinkage(8.0, 0.3e-3, 2.0, 0.100, ns, zc, es.i), 7.42e-3) < 1e-2);
}

#[test]
fn e1_11_minimum_reinforcement() {
    assert!(rel(as_min_m2(2.9e6, 2.0 * 0.100, 0.100, 0.250, 500e6), 835.2e-6) < 1e-3);
}

#[test]
fn e1_12_fire_curves() {
    assert!(rel(iso834_c(3600.0), 945.3) < 1e-3);
    assert!(rel(iso834_c(1800.0), 841.8) < 1e-3);
    assert!(rel(iso834_c(5.0), 96.54) < 1e-3);
    let t = protected_steel_temperature_c(3600.0, 150.0, 0.025, BOARD_LAMBDA, BOARD_C, BOARD_RHO);
    assert!(t > 20.0 && t < iso834_c(3600.0));
    assert!(ky_theta(500.0) == 0.78 && ky_theta(20.0) == 1.0);
}

#[test]
fn ltb_uframe_reduces_with_stiffer_restraint() {
    let sec = ipe300();
    let soft = mcr_uframe(&sec, 4.0, 0.0);
    let ks = uframe_ks(1e-6, 3.0, &sec, 2.0);
    let stiff = mcr_uframe(&sec, 4.0, ks);
    assert!(stiff > soft);
    let (lam, chi) = chi_lt(sec.w_pl() * 355e6, soft, sec.h / sec.b, false);
    assert!(lam > 0.0 && chi <= 1.0 && chi > 0.0);
    assert!(mcr_bare(&sec, 4.0) >= soft);
}

#[test]
fn crack_spacing_table_interpolates() {
    assert_eq!(max_bar_spacing_m(200e6, 0.3e-3), Some(0.25));
    assert!(max_bar_spacing_m(500e6, 0.3e-3).is_none());
}

fn oracle_vectors() -> std::collections::BTreeMap<String, f64> {
    let script = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../🔮️oracles/🧮️catalogue/🐍️.py");
    let out = semio_s_plugin_norm_compliance::python_oracle::run_ok(&script, &[], None);
    let text = String::from_utf8(out.stdout).expect("oracle utf8");
    text.trim().trim_start_matches('{').trim_end_matches('}').split(", ").map(|pair| {
        let (k, v) = pair.split_once(": ").expect("pair");
        (k.trim_matches('"').to_string(), v.parse::<f64>().expect("number"))
    }).collect()
}

#[test]
fn rust_catalogue_matches_independent_python_oracle() {
    let o = oracle_vectors();
    let sec = ipe300();
    let check = |name: &str, value: f64, tol: f64| {
        let want = o[name];
        assert!(rel(value, want) < tol, "{name}: rust {value} vs oracle {want}");
    };
    let full = plastic_full(&sec, 355e6, 0.1, 2.0, 0.1, 17e6);
    check("mpl_full", full.m_nm, 1e-3);
    check("x_full", full.x_m, 1e-3);
    check("mpl_partial_sb", plastic_sagging(&sec, 355e6, 0.1, 2.0, 0.1, 17e6, 0.5 * full.nc_n).m_nm, 2e-3);
    check("mpl_linear", plastic_linear(628.4e-6 * 355e6, full.m_nm, 0.5), 1e-3);
    check("beff_simple", effective_width_m(8.0, 0.0, 1.5, 1.5, false), 1e-12);
    check("beff_end", effective_width_m(8.0, 0.0, 1.5, 1.5, true), 1e-12);
    check("eta_min_8m", eta_min(8.0, 355e6, true), 1e-12);
    let (p1, p2, p) = stud_resistance_n(0.019, 0.1, 450e6, 30e6, 33e9, 1.25);
    check("prd1", p1, 1e-6);
    check("prd2", p2, 1e-6);
    check("prd_sheet", p * sheeting_reduction(0.05, 0.1, 0.095, 0.019, 0.0009, 1, false).unwrap(), 1e-6);
    check("asf_req", slab_shear(81_656.0 / 0.150, 0.200, 0.100, 30e6, 1.5, 1.15, 1.0).2, 2e-3);
    let s = ColSection { b: 0.2, t: 0.01, as_m2: 0.0, yr: 0.0, fyd: 355e6, fcd: 26.667e6, fsd: 0.0 };
    let poly = interaction_polygon(&s);
    check("npl_rhs", s.n_pl(), 1e-3);
    check("npm_rhs", s.n_pm(), 1e-3);
    check("mmax_rhs", poly[2].1, 1e-3);
    check("mpl_rhs", poly[3].1, 2e-3);
    check("mu_1700", mu_d(&poly, 1700e3), 2e-3);
    let sl = SlabSection { h: 0.150, hp: 0.050, ap: 1100e-6, ep: 0.020, e: 0.025, fyp_d: 350e6, fcd: 20e6 };
    check("slab_full", slab_bending(&sl, 4.0e3, f64::INFINITY).0, 1e-6);
    check("slab_partial", slab_bending(&sl, 4.0e3, 0.6 * 385e3).0, 1e-6);
    check("slab_vv", slab_vertical_shear(0.4, 0.125, 1100e-6, 30e6, 1.5), 1e-6);
    check("slab_mk", slab_mk(0.125, 1100e-6, 1.0, 200e6, 0.04e6, 1.25), 1e-9);
    let n0 = modular_ratio(33e9, 0.0, 0.0);
    let nl = modular_ratio(33e9, 1.1, 2.5);
    let i0 = transformed(&sec, 0.1, 2.0, 0.1, n0).i;
    let il = transformed(&sec, 0.1, 2.0, 0.1, nl).i;
    check("i_n0", i0, 1e-4);
    check("i_nl", il, 1e-4);
    check("defl", deflection_simple(8.0, 6e3, 4e3, il, i0), 1e-4);
    check("as_min", as_min_m2(2.9e6, 0.2, 0.1, 0.25, 500e6), 1e-9);
    check("iso_60", iso834_c(3600.0), 1e-9);
    check("theta_a_60", protected_steel_temperature_c(3600.0, 150.0, 0.025, BOARD_LAMBDA, BOARD_C, BOARD_RHO), 5e-3);
}

#[test]
fn review_regressions_alpha_lt_stud_rule_and_es() {
    assert_eq!(E_S, E_A);
    let welded = chi_lt(1.0e6, 1.0e6, 1.5, true).1;
    let rolled = chi_lt(1.0e6, 1.0e6, 1.5, false).1;
    assert!(welded < rolled);
    let (u, rule) = stud_detailing(0.019, 0.095, 0.055, 0.085, 0.15, 0.0009);
    assert!(u > 0.9 && rule.starts_with("hsc"), "{u} {rule}");
}

#[test]
fn two_span_cracked_alpha_st_is_the_cracked_ratio() {
    let sec = ipe300();
    let el = transformed(&sec, 0.100, 2.0, 0.100, modular_ratio(33e9, 0.0, 0.0));
    let cr = cracked(&sec, 0.100, 2000e-6, 0.040);
    let a_unc = el.area * el.i / (sec.area() * sec.i_y);
    let a_cr = cr.area * cr.i / (sec.area() * sec.i_y);
    assert!(a_unc > 15.0 && (2.0..3.2).contains(&a_cr), "{a_unc} {a_cr}");
}

#[test]
fn le_follows_fig_5_1_for_every_span_position() {
    assert!(rel(le_sagging_m(SpanPosition::Simple, 8.0), 8.0) < 1e-12);
    assert!(rel(le_sagging_m(SpanPosition::End, 8.0), 6.8) < 1e-12);
    assert!(rel(le_sagging_m(SpanPosition::Interior, 8.0), 5.6) < 1e-12);
    assert!(rel(le_sagging_m(SpanPosition::Cantilever, 3.0), 6.0) < 1e-12);
    assert!(rel(le_hogging_m(8.0, 6.0), 3.5) < 1e-12);
    assert!(rel(adjacent_span_ratio(&[8.0, 5.0, 7.0]), 0.625) < 1e-12);
}

#[test]
fn nonlinear_curve_starts_elastic_and_stays_below_plastic() {
    let sec = ipe300();
    let inp = NonlinearInput::standard(355e6, 17e6, 0.0, 0.0);
    let small = moment_at_curvature(&sec, 0.100, 2.0, 0.100, &inp, 1e-6);
    let n = E_A / (2.0 * inp.fcd / inp.eps_c2);
    let el = transformed(&sec, 0.100, 2.0, 0.100, n);
    assert!(rel(small.m_nm / small.kappa, E_A * el.i) < 0.03, "{}", small.m_nm / small.kappa / (E_A * el.i));
    let m_nl = nonlinear_mrd(&sec, 0.100, 2.0, 0.100, &inp);
    let m_pl = plastic_full(&sec, 355e6, 0.100, 2.0, 0.100, 17e6).m_nm;
    assert!(m_nl <= m_pl * 1.001 && m_nl >= 0.9 * m_pl, "{m_nl} {m_pl}");
    let curve = moment_curvature(&sec, 0.100, 2.0, 0.100, &inp, 30);
    assert!(curve.windows(2).all(|w| w[1].kappa > w[0].kappa));
    assert!(curve.iter().all(|p| p.eps_top <= inp.eps_cu * 1.0001));
}

#[test]
fn nonlinear_steel_only_section_approaches_the_plastic_moment() {
    let sec = ipe300();
    let inp = NonlinearInput::standard(355e6, 17e6, 0.0, 0.0);
    let m = nonlinear_mrd(&sec, 0.0, 1e-9, 1e-9, &inp);
    let m_pl = sec.w_pl() * 355e6;
    assert!(m <= m_pl * 1.001 && m > 0.97 * m_pl, "{m} {m_pl}");
}

#[test]
fn patch_loading_matches_the_hand_calculation() {
    let (f, chi, ly, lam) = patch_loading(0.4, 0.01, 0.3, 0.02, 355e6, 355e6, 0.1, 2.0, 1.0);
    assert!(rel(f, 992_783.694) < 1e-6 && rel(chi, 0.723_420_4) < 1e-6 && rel(ly, 0.386_576_6) < 1e-6 && rel(lam, 0.691_161_1) < 1e-6);
    let (wider, _, _, _) = patch_loading(0.4, 0.01, 0.3, 0.02, 355e6, 355e6, 0.2, 2.0, 1.0);
    assert!(wider > f);
    assert!(rel(patch_interaction(500e3, f, 100.0, 200.0), (0.8 * 500e3 / f + 0.5) / 1.4) < 1e-12);
}

#[test]
fn load_introduction_uses_table_6_6() {
    assert!(rel(interface_tau_rd_pa(InterfaceKind::FilledCircular), 0.55e6) < 1e-12);
    assert!(rel(introduction_length_m(0.4, 6.0), 0.8) < 1e-12);
    assert!(rel(introduction_length_m(0.4, 1.8), 0.6) < 1e-12);
    let (ed, rd, u) = load_introduction(InterfaceKind::ConcreteEncased, 600e3, 1.6, 0.8);
    assert!(rel(ed, 600e3 / 1.28) < 1e-12 && rel(rd, 0.3e6) < 1e-12 && rel(u, ed / rd) < 1e-12);
    assert!(load_introduction(InterfaceKind::PartiallyEncasedWeb, 1.0, 1.0, 1.0).2.is_infinite());
    assert_eq!(introduction_studs(1000e3, 90e3), 12);
}

#[test]
fn lm1_effects_match_brute_force_envelope() {
    assert_eq!(lm1_lane(true, 1), (240e3, 9e3));
    assert_eq!(lm1_lane(false, 2), (200e3, 2.5e3));
    let (axle, _) = lm1_lane(false, 1);
    let (l, d) = (10.0, TANDEM_AXLE_SPACING_M);
    let mut best = 0.0_f64;
    for k in 0..=20000 {
        let x = (l - d) * k as f64 / 20000.0;
        let r = (axle * (l - x) + axle * (l - x - d)) / l;
        for s in [x, x + d] {
            best = best.max(r * s - if s > x { axle * (s - x) } else { 0.0 } - if s > x + d { axle * (s - x - d) } else { 0.0 });
        }
    }
    let (m, v) = lm1_simple_span_effects(l, axle, 0.0);
    assert!(rel(m, best) < 1e-6 && rel(v, axle * (2.0 - d / l)) < 1e-12);
    let (mq, _) = lm1_simple_span_effects(l, 0.0, 9e3);
    assert!(rel(mq, 9e3 * 3.0 * l * l / 8.0) < 1e-12);
}

#[test]
fn e1_13_nonlinear_resistance_reproduces_the_blessed_third_party_value() {
    let sec = ipe300();
    let inp = NonlinearInput::standard(355e6, 17e6, 0.0, 0.0);
    let m = nonlinear_mrd(&sec, 0.100, 2.0, 0.100, &inp);
    assert!(rel(m, 422_131.29) < 3e-3, "{m}");
    assert!(rel(ultimate_curvature(&sec, 0.100, 2.0, 0.100, &inp), 0.050_580_74) < 3e-3);
}

#[test]
fn shear_rho_is_zero_below_half_the_plastic_shear_and_capped_at_one() {
    assert_eq!(shear_rho(0.5, 1.0), 0.0);
    assert!(rel(shear_rho(0.75, 1.0), 0.25) < 1e-12);
    assert_eq!(shear_rho(1.0, 1.0), 1.0);
    assert_eq!(shear_rho(2.63, 1.0), 1.0);
    let sec = ipe300();
    let (m_cap, rho) = shear_reduced_hogging_moment(1.0, &sec, 0.1, 355e6, 1e-3, 435e6, 0.04, 3.0, 1.0);
    let (m_one, _) = shear_reduced_hogging_moment(1.0, &sec, 0.1, 355e6, 1e-3, 435e6, 0.04, 1.0, 1.0);
    assert!(rho == 1.0 && m_cap > 0.0 && rel(m_cap, m_one) < 1e-12);
}

#[test]
fn shear_reduction_re_solves_the_plastic_axis_with_the_reduced_web_strength() {
    let sec = ipe300();
    let full = plastic_full(&sec, 355e6, 0.1, 2.0, 0.1, 17e6);
    let (m_v, rho) = shear_reduced_sagging_moment(&full, &sec, 0.1, 2.0, 0.1, 17e6, 355e6, 0.9, 1.0);
    let bare = plastic_full_k(&sec, 0.0, 355e6, 0.1, 2.0, 0.1, 17e6);
    assert!(rho == 1.0 && rel(m_v, bare.m_nm) < 1e-12 && m_v < full.m_nm && m_v > 0.0);
    let (m_h, _) = plastic_hogging(&sec, 355e6, 0.1, 1e-3, 435e6, 0.04);
    let (m_hv, rho_h) = shear_reduced_hogging_moment(m_h, &sec, 0.1, 355e6, 1e-3, 435e6, 0.04, 0.9, 1.0);
    assert!(rho_h == 1.0 && m_hv < m_h && m_hv > 0.0);
    assert_eq!(shear_reduced_sagging_moment(&full, &sec, 0.1, 2.0, 0.1, 17e6, 355e6, 0.4, 1.0), (full.m_nm, 0.0));
}

#[test]
fn e1_14_cracked_continuous_beams_reproduce_the_blessed_pynite_values() {
    use continuous::{solve, Layout, Stiffness};
    let ei = 210e9 * 6e-4;
    let cracked = Stiffness { ei1: ei, ei2: 0.5 * ei, cracked_fraction: 0.15 };
    let two = Layout::of(SpanPosition::End, 8.0, 0.0);
    let s = solve(&two, &[10e3, 10e3], cracked);
    assert!(rel(-s.supports[1], 64_047.62) < 1e-4 && rel(s.deflection_own(), 2.162_476e-3) < 1e-4);
    let interior = Layout::of(SpanPosition::Interior, 8.0, 6.0);
    assert!(rel(solve(&interior, &[10e3; 3], cracked).deflection_own(), 1.451_09e-3) < 1e-4);
    let tip = Layout::of(SpanPosition::Cantilever, 3.0, 8.0);
    assert!(rel(solve(&tip, &[0.0, 10e3], cracked).deflection_own(), 5.147_31e-3) < 1e-4);
    let unequal = Layout::of(SpanPosition::End, 8.0, 6.0);
    assert!(rel(-solve(&unequal, &[10e3, 10e3], Stiffness::uniform(ei)).supports[1], 65_000.30) < 1e-5);
}

#[test]
fn e1_15_encased_polygon_reproduces_the_exact_integration() {
    use encased::{interaction_polygon, EncasedI, Encasement, Strengths};
    let g = EncasedI { kind: Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.013, as_m2: 4.0 * 314.16e-6, cover: 0.05 };
    let poly = interaction_polygon(&g, &Strengths { fyd: 355e6, fcd: 0.85 * 30e6 / 1.5, fsd: 500e6 / 1.15 });
    assert!(rel(poly[3].1, 342_880.11) < 2e-3 && rel(poly[2].1, 407_639.36) < 2e-3);
}

#[test]
fn e1_16_column_fire_field_reproduces_the_independent_conduction_solution() {
    use column_fire::temperature_field;
    use encased::{EncasedI, Encasement};
    let g = EncasedI { kind: Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.013, as_m2: 4.0 * 314.16e-6, cover: 0.05 };
    let f = temperature_field(&g, 3600.0);
    assert!((f.at(0, 0) - 24.4885).abs() < 0.05 && (f.at(0, 9) - 74.7581).abs() < 0.05 && (f.at(19, 19) - 941.3887).abs() < 0.05);
}

#[test]
fn e1_17_shear_lag_factors_reproduce_the_hand_values() {
    assert!(rel(bridge::beta_sagging(0.1), 0.939_85) < 1e-4 && rel(bridge::beta_hogging(0.1), 0.628_14) < 1e-4);
}

#[test]
fn e1_18_point_forces_and_cracked_stiffness_on_continuous_spans_reproduce_the_pynite_values() {
    use continuous::{action_internals, Layout, Load, Stiffness};
    let ei = 210e9 * 1e-4;
    let uniform = Stiffness::uniform(ei);
    let cracked = Stiffness { ei1: ei, ei2: 0.5 * ei, cracked_fraction: 0.15 };
    let close = |position, span: f64, adjacent: f64, load, permanent, stiffness, want: (f64, f64, f64)| {
        let got = action_internals(position, span, adjacent, load, permanent, stiffness);
        let near = |a: f64, b: f64| (a - b).abs() <= 1e-4 * b.abs().max(1.0);
        assert!(near(got.sagging, want.0) && near(got.hogging, want.1) && near(got.shear, want.2), "{got:?} vs {want:?}");
    };
    close(SpanPosition::End, 10.0, 10.0, Load::Udl(1000.0), true, uniform, (7031.25, 12_500.0, 6250.0));
    close(SpanPosition::End, 10.0, 10.0, Load::Point(100e3, 0.5), true, uniform, (156_250.0, 187_500.0, 68_750.0));
    close(SpanPosition::End, 8.0, 5.0, Load::Point(100e3, 0.375), false, uniform, (157_752.40, 115_948.02, 52_584.13));
    close(SpanPosition::End, 8.0, 5.0, Load::Point(100e3, 0.375), true, uniform, (144_019.49, 115_948.02, 51_993.50));
    close(SpanPosition::Interior, 8.0, 5.0, Load::Point(100e3, 0.25), false, uniform, (88_725.49, 94_500.61, 83_235.68));
    close(SpanPosition::Interior, 8.0, 5.0, Load::Point(100e3, 0.25), true, uniform, (71_300.55, 83_777.57, 77_539.06));
    close(SpanPosition::Cantilever, 2.0, 6.0, Load::Point(100e3, 0.5), false, uniform, (0.0, 100_000.0, 100_000.0));
    close(SpanPosition::End, 8.0, 8.0, Load::Point(100e3, 0.75), false, uniform, (100_781.25, 112_500.0, 89_062.50));
    close(SpanPosition::End, 8.0, 6.0, Load::Point(100e3, 0.375), false, cracked, (166_159.37, 95_764.18, 55_386.46));
    close(SpanPosition::Interior, 8.0, 6.0, Load::Point(100e3, 0.75), true, cracked, (81_669.90, 71_056.67, 76_363.28));
    close(SpanPosition::End, 8.0, 8.0, Load::Udl(10e3), true, cracked, (51_181.04, 64_047.26, 48_005.91));
    let hand_rolled = continuous::solve(&Layout::of(SpanPosition::End, 8.0, 8.0), &[10e3, 10e3], cracked);
    let shared = action_internals(SpanPosition::End, 8.0, 8.0, Load::Udl(10e3), true, cracked);
    assert!(rel(shared.hogging, -hand_rolled.supports[1]) < 1e-3 && rel(shared.sagging, hand_rolled.max_sagging_own()) < 1e-3);
}

#[test]
fn warping_constant_of_the_ipe300_follows_the_closed_form_and_the_section_tables() {
    let s = ipe300();
    assert!(rel(s.iw(), s.iz() * (0.300_f64 - 0.0107).powi(2) / 4.0) < 1e-12);
    assert!(rel(s.iw(), 1.259e-7) < 0.03, "{}", s.iw());
}

#[test]
fn e1_23_natural_frequency_follows_the_closed_form_and_scales_with_the_span() {
    let m = vibration::mass_per_length_kg_m(10e3, 15e3, 0.3);
    assert!(rel(m, 14.5e3 / 9.80665) < 1e-12);
    let f = vibration::natural_frequency_hz(8.0, 6e-4, m);
    assert!(rel(f, 7.164758) < 1e-5, "{f}");
    assert!(rel(vibration::natural_frequency_hz(16.0, 6e-4, m), f / 4.0) < 1e-12);
    assert!(rel(vibration::frequency_utilisation(3.0, f), 0.418718) < 1e-4);
}

#[test]
fn e1_24_uframe_buckling_of_the_ipe300_over_the_support() {
    let sec = ipe300();
    let ks = uframe_ks(4e-5, 3.0, &sec, 2.0);
    assert!(rel(ks, 70476.48) < 1e-6, "{ks}");
    let mcr = mcr_uframe(&sec, 6.0, ks);
    assert!(rel(mcr, 216307.9) < 1e-6, "{mcr}");
    let (lambda, chi) = chi_lt(sec.w_pl() * 355e6, mcr, 2.0, false);
    assert!(rel(chi, 0.66204368) < 1e-6 && rel(lambda, 1.00509) < 1e-5, "{lambda} {chi}");
}

#[test]
fn e1_25_shrinkage_modular_ratio_and_second_moment() {
    let n_s = modular_ratio(ecm_pa(30e6), constants::PSI_L_SHRINKAGE, 2.5);
    assert!(rel(n_s, 15.188859) < 1e-6, "{n_s}");
    assert!(rel(transformed(&ipe300(), 0.100, 2.0, 0.100, n_s).i, 0.00024733105) < 1e-6);
}

#[test]
fn e1_26_critical_temperature_decreases_with_the_degree_of_utilisation() {
    assert!(rel(critical_temperature_c(0.5), 584.66526) < 1e-6);
    assert!(rel(critical_temperature_c(0.3), 663.77669) < 1e-6);
    assert!(critical_temperature_c(0.7) < critical_temperature_c(0.5));
}
