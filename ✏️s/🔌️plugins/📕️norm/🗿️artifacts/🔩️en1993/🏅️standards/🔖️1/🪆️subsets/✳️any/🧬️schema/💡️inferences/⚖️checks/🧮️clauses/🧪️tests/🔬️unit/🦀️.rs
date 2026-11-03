use super::joint_components;
use super::classification::*;
use super::effective_width::*;
use super::fatigue_fire::*;
use super::global::*;
use super::joints::*;
use super::mcr_fe::*;
use super::resistance::*;
use super::stability::*;

fn close(a: f64, b: f64, rel: f64) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{a} vs {b}");
}

const IPE300: Geometry = Geometry { h: 300.0, b: 150.0, tw: 7.1, tf: 10.7, r: 15.0, weld_throat: 0.0 };

#[test]
fn e01_flexural_buckling_heb240() {
    let (a, iz, iy, l, fy) = (10600.0, 3923.0e4, 11260.0e4, 4000.0, 355.0);
    let ncr_z = n_cr(e_steel() * iz, l);
    close(ncr_z, 5081.8e3, 1e-4);
    let lam_z = lambda_bar_from_ncr(a * fy, ncr_z);
    close(lam_z, 0.8605, 1e-3);
    let cz = curve_table_6_2(BucklingSectionKind::RolledI, false, 240.0, 240.0, 17.0, fy);
    assert_eq!(cz, Curve::C);
    let chi_z = chi_flexural(lam_z, cz, 1800e3, ncr_z);
    close(chi_z, 0.6243, 1e-3);
    close(chi_z * a * fy, 2349.1e3, 1e-3);
    close(chi_z * a * fy / 1.1, 2135.6e3, 1e-3);
    let ncr_y = n_cr(e_steel() * iy, l);
    let lam_y = lambda_bar_from_ncr(a * fy, ncr_y);
    let cy = curve_table_6_2(BucklingSectionKind::RolledI, true, 240.0, 240.0, 17.0, fy);
    assert_eq!(cy, Curve::B);
    close(chi_flexural(lam_y, cy, 1800e3, ncr_y), 0.8807, 1e-3);
    close(1800e3 / (chi_z * a * fy), 0.766, 2e-3);
}

#[test]
fn e02_ltb_ipe300() {
    let (iz, it, iw, l) = (603.8e4, 20.12e4, 125900.0e6, 5000.0);
    let mcr = mcr_closed_form(e_steel(), g_steel(), iz, it, iw, l, 1.0, 1.0, 1.127, 0.0, 0.0);
    close(mcr, 130.38e6, 1e-3);
    let lam = lambda_lt_from_mcr(628.4e3, 235.0, mcr);
    close(lam, 1.0643, 1e-3);
    let chi = chi_lt_general(lam, 0.34, 70e6, mcr);
    close(chi, 0.5569, 1e-3);
    close(chi * 628.4e3 * 235.0, 82.23e6, 1e-3);
    close(chi * 628.4e3 * 235.0 / 1.1, 74.76e6, 1e-3);
    let chi2 = chi_lt_rolled(lam, 0.34, 0.4, 0.75, 70e6, mcr);
    close(chi2, 0.6603, 1e-3);
    let f = ltb_f(0.94, lam);
    close(f, 0.9742, 1e-3);
    let cm = chi_lt_mod(chi2, f);
    close(cm, 0.6778, 1e-3);
    close(cm * 628.4e3 * 235.0, 100.10e6, 1e-3);
    assert!(chi_lt_mod(chi_lt_rolled(0.2, 0.34, 0.4, 0.75, 70e6, mcr), 1.0) <= 1.0);
    let short = mcr_closed_form(e_steel(), g_steel(), iz, it, iw, 2500.0, 1.0, 1.0, 1.127, 0.0, 0.0);
    assert!(chi_lt_mod(chi_lt_rolled(lambda_lt_from_mcr(628.4e3, 235.0, short), 0.34, 0.4, 0.75, 70e6, short), ltb_f(0.94, lambda_lt_from_mcr(628.4e3, 235.0, short))) > cm);
}

#[test]
fn e02_mcr_fe_matches_closed_form() {
    let p = BeamProps { l: 5000.0, e: e_steel(), g: g_steel(), iz: 603.8e4, it: 20.12e4, iw: 125900.0e6 };
    let uniform = |_x: f64| 1.0;
    let none = |_x: f64| 0.0;
    let r = Restraints { supports: vec![Support::fork(0.0), Support::fork(5000.0)], torsional_spring_per_length: 0.0, lateral_springs: vec![] };
    let res = mcr_fe(&p, &Load { moment: &uniform, line_load: &none, zg: 0.0 }, &r, 40).expect("fe");
    close(res.lambda, mcr_closed_form(e_steel(), g_steel(), p.iz, p.it, p.iw, p.l, 1.0, 1.0, 1.0, 0.0, 0.0), 5e-3);
    let l = p.l;
    let parabola = move |x: f64| x * (l - x) / 2.0;
    let q = |_x: f64| 1.0;
    let res = mcr_fe(&p, &Load { moment: &parabola, line_load: &q, zg: 0.0 }, &r, 40).expect("fe");
    close(res.lambda * l * l / 8.0, 130.38e6, 1.5e-2);
}

#[test]
fn e03_classification() {
    let eps = epsilon(235.0);
    let b = classify_i_section(&IPE300, 235.0, 0.0, -1.0, true);
    close(b.c_web / IPE300.tw, 35.01, 1e-3);
    close(b.c_flange / IPE300.tf, 5.28, 1e-2);
    assert_eq!((b.web, b.flange, b.section), (1, 1, 1));
    let c = classify_i_section(&IPE300, 235.0, 1.0e9, 1.0, true);
    assert_eq!((c.web, c.flange, c.section), (2, 1, 2));
    let heb = Geometry { h: 240.0, b: 240.0, tw: 10.0, tf: 17.0, r: 21.0, weld_throat: 0.0 };
    let d = classify_i_section(&heb, 355.0, 1.0e9, 1.0, true);
    assert_eq!((d.web, d.flange), (1, 1));
    assert_eq!(internal_part_class(125.0, epsilon(355.0), -1.0, 0.5), 4);
    close(62.0 * epsilon(355.0) * 2.0, 100.89, 1e-3);
    let alpha = alpha_internal_web(200e3, 248.6, 7.1, 235.0);
    close(alpha, 0.741, 1e-3);
    close(396.0 * eps / (13.0 * alpha - 1.0), 45.9, 2e-3);
    assert_eq!(chs_class(40.0, 1.0), 1);
    assert_eq!(chs_class(95.0, 1.0), 4);
    assert_eq!(angle_class(100.0, 100.0, 10.0, 1.0), 3);
}

#[test]
fn e04_effective_width() {
    let e = effective_internal(1000.0, 8.0, 355.0, -1.0);
    close(e.k_sigma, 23.9, 1e-9);
    close(e.lambda_p, 1.1066, 1e-3);
    close(e.rho, 0.8139, 1e-3);
    close(e.b_eff, 406.9, 1e-3);
    close(e.b_e1, 162.8, 2e-3);
    close(e.b_e2, 244.2, 2e-3);
    let f = effective_internal(100.0, 2.0, 350.0, 1.0);
    close(f.lambda_p, 1.0743, 1e-3);
    close(f.rho, 0.7402, 1e-3);
    close(f.b_eff, 74.02, 1e-3);
    let i = i_section_bending(1040.0, 300.0, 8.0, 20.0, 355.0);
    close(i.w_eff_y, 7007.0e3, 5e-3);
    assert!(i.e_n < 0.0 && (i.e_n + 11.2).abs() < 0.5);
}

#[test]
fn e05_axial_biaxial_class1() {
    let (a, b, tf, tw, fy) = (5381.0, 150.0, 10.7, 7.1, 235.0);
    let npl = n_pl_rd(a, fy, 1.0);
    close(npl, 1264.5e3, 1e-3);
    let mpl_y = 628.4e3 * fy;
    let mpl_z = 125.2e3 * fy;
    let n = 300e3 / npl;
    let aw = (a - 2.0 * b * tf) / a;
    close(n, 0.2372, 1e-3);
    close(aw, 0.4035, 1e-3);
    let mny = m_n_y_rd_i(mpl_y, n, aw);
    close(mny, 141.10e6, 1e-3);
    let mnz = m_n_z_rd_i(mpl_z, n, aw);
    close(mnz, 29.42e6, 1e-3);
    let (al, be) = biaxial_exponents(0, n);
    close(be, 1.1862, 1e-3);
    close(biaxial_utilisation(80e6, 10e6, mny, mnz, al, be), 0.5994, 1e-3);
    let hw = 300.0 - 2.0 * tf;
    assert!(!neglect_axial_effect_y(300e3, npl, hw, tw, fy, 1.0));
    assert!(neglect_axial_effect_y(100e3, npl, hw, tw, fy, 1.0));
}

#[test]
fn e06_beam_column_annex_b() {
    let n_rk = 3763.0e3;
    let i = InteractionInput {
        n_ed: 800e3,
        my_ed: 100e6,
        mz_ed: 0.0,
        n_rk,
        my_rk: 373.8e6,
        mz_rk: 1.0,
        chi_y: 0.8807,
        chi_z: 0.6243,
        chi_lt: 1.0,
        lambda_y: 0.5079,
        lambda_z: 0.8605,
        cmy: 0.6,
        cmz: 0.6,
        cm_lt: 0.6,
        class: 1,
        susceptible_torsion: true,
        rhs: false,
        gm1: 1.0,
        delta_my: 0.0,
        delta_mz: 0.0,
    };
    let r = interaction_annex_b(&i);
    close(r.kyy, 0.6446, 1e-3);
    close(r.kzy, 0.9163, 1e-3);
    close(r.eta_61, 0.4138, 2e-3);
    close(r.eta_62, 0.5857, 2e-3);
    let de = interaction_annex_b(&InteractionInput { gm1: 1.1, ..i });
    close(de.eta_61, 0.4565, 2e-3);
    close(de.eta_62, 0.6418, 2e-3);
    close(c1_end_moments(0.0), 1.879, 1e-9);
    close(kc_psi(0.0), 0.7519, 1e-3);
}

#[test]
fn e07_shear_buckling_web() {
    let eps = epsilon(355.0);
    let k = k_tau(2.0);
    close(k, 6.34, 1e-3);
    assert!(shear_buckling_check_required(1000.0, 8.0, eps, 1.2, Some(k)));
    let lw = lambda_w(1000.0, 8.0, eps, k);
    close(lw, 1.6315, 1e-3);
    let chi = chi_w(lw, 1.2, true);
    close(chi, 0.5876, 1e-3);
    close(v_bw_rd(chi, 355.0, 1000.0, 8.0, 1.0), 963.5e3, 1e-3);
    close(v_bw_rd(chi, 355.0, 1000.0, 8.0, 1.1), 875.9e3, 1e-3);
    close(v_b_rd_cap(1.2, 355.0, 1000.0, 8.0, 1.0), 1967e3, 2e-3);
    let eta = bending_shear_buckling_utilisation(100.0, 400.0, 200.0, 100.0, 963.5);
    close(eta, 0.25, 1e-9);
}

#[test]
fn e08_bolts() {
    let fv = fv_rd("8.8", 20.0, true, 1.25);
    close(fv, 94.08e3, 1e-3);
    let ft = ft_rd("8.8", 20.0, false, 1.25);
    close(ft, 141.12e3, 1e-3);
    let ab = alpha_b(true, 40.0, 100.0, 22.0, 800.0, 490.0);
    close(ab, 0.6061, 1e-3);
    let k1 = k1_bearing(true, 35.0, 100.0, 22.0);
    close(k1, 2.5, 1e-9);
    close(fb_rd(k1, ab, 490.0, 20.0, 12.0, 1.25, 1.0), 142.55e3, 1e-3);
    close(shear_tension_utilisation(50e3, fv, 70e3, ft), 0.8858, 1e-3);
    close(beta_lf(300.0, 20.0), 1.0, 1e-9);
    close(beta_lf(1000.0, 20.0), 0.825, 1e-9);
    close(fp_c("8.8", 20.0), 137.2e3, 1e-3);
    assert!(spacing_checks(40.0, 35.0, 60.0, 60.0, 22.0, 12.0).iter().all(|s| s.ok));
}

#[test]
fn e09_fillet_weld() {
    let f = fvw_d(490.0, 0.9, 1.25);
    close(f, 251.47, 1e-4);
    close(f * 5.0, 1257.3, 1e-4);
    close(weld_directional(0.0, 0.0, f, 490.0, 0.9, 1.25), 1.0, 1e-4);
    assert!((beta_w("S355") - 0.9).abs() < 1e-12 && (beta_w("S460N") - 1.0).abs() < 1e-12);
}

#[test]
fn directional_weld_method_resolves_the_throat_stresses_and_differs_from_the_simplified_method() {
    let (a, l, fu, beta, gm2) = (5.0, 200.0, 490.0, 0.9, 1.25);
    let quarter = std::f64::consts::FRAC_PI_4;
    let (sigma, tau_perp, tau_par) = weld_throat_stresses(0.0, 250e3, quarter, a, l);
    close(sigma, 176.77669529663686, 1e-12);
    close(tau_perp, 176.77669529663686, 1e-12);
    assert_eq!(tau_par, 0.0);
    let (c1, c2) = weld_directional_criteria(sigma, tau_perp, tau_par, fu, beta, gm2);
    close(c1, 0.8117297233008836, 1e-12);
    close(c2, 0.5010677304326442, 1e-12);
    close(weld_directional(sigma, tau_perp, tau_par, fu, beta, gm2), 0.8117297233008836, 1e-12);
    close(250.0 / fvw_d(fu, beta, gm2), 0.994161815568871, 1e-12);
    let (sigma, tau_perp, tau_par) = weld_throat_stresses(100e3, 200e3, quarter, a, l);
    close(weld_directional(sigma, tau_perp, tau_par, fu, beta, gm2), 0.7614699773775152, 1e-12);
    close(100.0_f64.hypot(200.0) / fvw_d(fu, beta, gm2), 0.8892053600986417, 1e-12);
}

#[test]
fn directional_weld_method_normal_stress_criterion_governs_for_a_force_along_the_throat_normal() {
    let (sigma, tau_perp, tau_par) = weld_throat_stresses(0.0, 300e3, 0.0, 5.0, 200.0);
    assert_eq!((sigma, tau_perp, tau_par), (300.0, 0.0, 0.0));
    let (c1, c2) = weld_directional_criteria(sigma, tau_perp, tau_par, 490.0, 0.9, 1.25);
    close(c1, 0.6887755102040817, 1e-12);
    close(c2, 0.8503401360544217, 1e-12);
    close(weld_directional(sigma, tau_perp, tau_par, 490.0, 0.9, 1.25), c2, 1e-15);
    close(weld_directional(-sigma, tau_perp, tau_par, 490.0, 0.9, 1.25), c2, 1e-15);
}

#[test]
fn directional_weld_method_equals_the_simplified_method_for_a_longitudinal_force() {
    let (sigma, tau_perp, tau_par) = weld_throat_stresses(100e3, 0.0, std::f64::consts::FRAC_PI_4, 5.0, 200.0);
    close(weld_directional(sigma, tau_perp, tau_par, 490.0, 0.9, 1.25), 100.0 / fvw_d(490.0, 0.9, 1.25), 1e-12);
    close(weld_directional(sigma, tau_perp, tau_par, 490.0, 0.9, 1.25), 0.39766472622754834, 1e-12);
}

#[test]
fn e10_end_plate_t_stub_and_stiffness() {
    close(l_eff_circular(40.0), 251.3, 1e-3);
    let leff = l_eff_noncircular_end_plate_row(40.0, 40.0);
    close(leff, 210.0, 1e-9);
    let t = crate::joint_components::t_stub_modes(leff, leff, 15.0, 355.0, 40.0, 40.0, 0.0, 282.24e3, 245.0, 2.0, 60.0, crate::joint_components::Mode1Method::Two, 1.0);
    assert!(t.prying);
    close(t.m_pl_1, 4193.4e3, 1e-3);
    close(t.ft_1, 419.3e3, 1e-3);
    close(t.ft_2, 245.96e3, 1e-3);
    close(t.ft_3, 282.24e3, 1e-9);
    assert_eq!(t.mode, 2);
    let k5 = crate::joint_components::k5_end_plate_bending(leff, 15.0, 40.0);
    close(k5, 9.9668, 1e-4);
    let k10 = k10_bolts(245.0, 60.0, 1.0);
    close(k10, 6.533, 1e-3);
    let keq = k_eq(&[k5, k10]);
    close(keq, 3.9464, 1e-4);
    let sj = s_j_ini(210000.0, 250.0, keq);
    close(sj, 51.80e9, 1e-3);
    assert_eq!(classify_stiffness(sj, 210000.0, 8356.0e4, 6000.0, 8.0), JointStiffnessClass::Rigid);
    close(210000.0 * 8356.0e4 / 6000.0 * 8.0, 23.40e9, 1e-3);
}

#[test]
fn e11_fatigue() {
    close(delta_sigma_d(71.0), 52.31, 1e-3);
    close(delta_sigma_l(71.0), 28.73, 1e-3);
    close(endurance(60.0, 71.0).unwrap(), 3.314e6, 1e-3);
    let d = miner_damage(&[(60.0, 1.0e6)], 71.0, 1.15, 1.0);
    close(d, 0.4589, 1e-3);
    let d2 = miner_damage(&[(60.0, 1.0e6), (40.0, 2.0e6)], 71.0, 1.15, 1.0);
    close(d2 - d, 2.0e6 / (5.0e6 * (52.31 / 1.15 / 40.0_f64).powi(5)), 1e-3);
    assert!(endurance(20.0, 71.0).is_none());
    let rf = rainflow(&[0.0, 10.0, 2.0, 8.0, 0.0]);
    let total: f64 = rf.iter().map(|(_, n)| n).sum();
    close(total, 2.0, 1e-9);
}

#[test]
fn e12_fire() {
    let iso = |t: f64| gas_iso834(t);
    close(gas_iso834(5.0), 576.4, 1e-3);
    let ksh = k_sh_i_section(167.26 / 215.58);
    close(ksh, 0.6982, 1e-3);
    let h = unprotected_steel_temperature(&iso, ksh, 215.58, 900.0, 5.0, 0.7, 1.0);
    let at = |s: f64| h.iter().find(|(t, _)| (*t - s).abs() < 1e-9).unwrap().1;
    assert!((at(300.0) - 237.9).abs() <= 3.0 && (at(600.0) - 491.5).abs() <= 3.0 && (at(900.0) - 646.3).abs() <= 3.0);
    let cr = theta_critical(0.6);
    close(cr, 554.3, 2e-3);
    close(k_y_theta(cr), 0.612, 2e-3);
    let t_cr = time_to_temperature(&h, cr).unwrap();
    assert!(t_cr / 60.0 > 10.0 && t_cr / 60.0 < 15.0);
    let chi = chi_fi(0.8, cr, 235.0);
    assert!(chi > 0.0 && chi < 1.0);
}

#[test]
fn e13_global_stability() {
    close(alpha_cr_storey(50.0, 1200.0, 6000.0, 12.0).unwrap(), 20.83, 1e-3);
    assert_eq!(analysis_mode(20.83, false), AnalysisMode::FirstOrder);
    let a = alpha_cr_storey(50.0, 1200.0, 6000.0, 40.0).unwrap();
    close(a, 6.25, 1e-3);
    assert_eq!(analysis_mode(a, false), AnalysisMode::AmplifiedFirstOrder);
    close(amplification(a).unwrap(), 1.1905, 1e-3);
    let phi = sway_phi(6.0, 2.0);
    close(phi, 3.5355e-3, 1e-4);
    close(sway_equivalent_force(phi, 500.0), 1.768, 1e-3);
    close(bow_ratio(2, true), 1.0 / 200.0, 1e-12);
    close(bow_ratio(2, false), 1.0 / 250.0, 1e-12);
}

#[test]
fn e14_tension_net_section() {
    let a_net = a_net_staggered(200.0, 12.0, 2.0, 22.0, &[]);
    close(a_net, 1872.0, 1e-9);
    close(n_pl_rd(200.0 * 12.0, 235.0, 1.0), 564.0e3, 1e-3);
    close(n_u_rd(a_net, 360.0, 1.25), 485.2e3, 1e-3);
    close(n_t_rd(2400.0, a_net, 235.0, 360.0, 1.0, 1.25), 485.2e3, 1e-3);
}

#[test]
fn e15_bending_and_shear() {
    let av = av_rolled_i(5381.0, 150.0, 10.7, 7.1, 15.0, 278.6, 1.2);
    close(av, 2567.97, 1e-4);
    let vpl = v_pl_rd(av, 235.0, 1.0);
    close(vpl, 348.42e3, 1e-3);
    let rho = shear_rho(200e3, vpl);
    close(rho, 0.02192, 2e-3);
    let aw = 278.6 * 7.1;
    close(m_y_v_rd_i(628.4e3, rho, aw, 7.1, 235.0, 1.0), 146.96e6, 1e-3);
    assert_eq!(shear_rho(100e3, vpl), 0.0);
}

#[test]
fn nomogram_and_misc() {
    close(k_nomogram(1.0, 1.0, true), 1.32, 5e-3);
    close(k_nomogram(1.0, 1.0, false), 0.77, 1e-2);
    let lm = plastic_hinge_stable_length(30.0, 0.0, 5000.0, 1.0, 1.0e6, 1.0e5, 235.0);
    assert!(lm > 0.0);
    let g = general_method_6_3_4(2.0, 1.5, 0.34, 1.0);
    assert!(g.utilisation > 0.0 && g.chi_op <= 1.0);
    let t = warping_torsion_fork(5000.0, e_steel() * 125900.0e6, g_steel() * 20.12e4, 1.0e6, 2500.0, false);
    assert!(t.theta_max > 0.0 && t.b_max > 0.0 && t.t_t_max + t.t_w_max >= 0.5e6 * 0.99);
}

#[test]
fn plates_shells_cold_stainless_and_parts_2_to_6() {
    use super::{cold_stainless::*, parts_2_6::*, plates_shells::*};
    close(shear_lag_beta(0.1), 1.0 / 1.064, 1e-12);
    close(shear_lag_beta(1.0), 1.0 / 5.9, 1e-12);
    let alpha = cylinder_alpha_x(3000.0, 10.0, 25.0);
    close(alpha, 0.3943, 2e-3);
    let lam = (235.0 / cylinder_sigma_x_rcr(210000.0, 10.0, 3000.0)).sqrt();
    close(lam, 0.745, 2e-3);
    close(cylinder_chi_x(lam, alpha), 0.5876, 3e-3);
    close(cylinder_sigma_x_rd(210000.0, 235.0, 10.0, 3000.0, 25.0, 1.1), cylinder_chi_x(lam, alpha) * 235.0 / 1.1, 1e-12);
    let p = crate::plated_ext::patch_resistance(crate::plated_ext::PatchType::Through, 1000.0, 8.0, 2000.0, 20.0, 300.0, 355.0, 355.0, 100.0, 0.0, 1.0);
    assert!(p.f_rd > 0.0 && p.chi_f <= 1.0 && p.l_y <= 2000.0);
    close(reduced_stress_utilisation(100.0, 0.0, 0.0, 1.0, 1.0, 1.0, 355.0, 1.0), (100.0f64 / 355.0).powi(2), 1e-12);
    close(f_ya(300.0, 400.0, 7.0, 4.0, 2.0, 400.0), (300.0_f64 + 100.0 * 7.0 * 4.0 * 4.0 / 400.0).min(350.0), 1e-12);
    close(chi_distortional(0.5), 1.0, 1e-12);
    close(chi_distortional(1.0), 1.47 - 0.723, 1e-12);
    close(ramberg_osgood_tangent(0.0, 200000.0, 220.0, 7.0), 200000.0, 1e-9);
    close(ramberg_osgood_strain(220.0, 200000.0, 220.0, 7.0), 220.0 / 200000.0 + 0.002, 1e-12);
    assert!(stainless_grade("1.4462").is_some() && stainless_grade("S355").is_none());
    close(stainless_eps(220.0, 200000.0), (235.0f64 / 220.0 * 200000.0 / 210000.0).sqrt(), 1e-12);
    assert!(stainless_chi(0.3, false) == 1.0 && stainless_chi(1.0, true) < 1.0);
    close(lambda_equivalence(1.0, 1.0, 1.0, 1.2, 2.0), 1.2, 1e-12);
    close(bridge_fatigue_utilisation(40.0, 71.0, 1.0, 1.15), 40.0 * 1.15 / 71.0, 1e-12);
    close(cable_resistance(1770.0, 1000.0, 1.5), 1000.0, 1e-12);
    close(crane_l_eff(1.0e6, 8.0), 3.25 * (1.0e6f64 / 8.0).cbrt(), 1e-12);
    close(base_plate_c(20.0, 235.0, 20.0, 1.0), 20.0 * (235.0f64 / 60.0).sqrt(), 1e-12);
}

#[test]
fn e02_mcr_fe_load_height_and_restraint() {
    let p = BeamProps { l: 5000.0, e: e_steel(), g: g_steel(), iz: 603.8e4, it: 20.12e4, iw: 125900.0e6 };
    let l = p.l;
    let parabola = move |x: f64| x * (l - x) / 2.0;
    let q = |_x: f64| 1.0;
    let r = Restraints { supports: vec![Support::fork(0.0), Support::fork(l)], torsional_spring_per_length: 0.0, lateral_springs: vec![] };
    let top = mcr_fe(&p, &Load { moment: &parabola, line_load: &q, zg: 150.0 }, &r, 40).expect("top").lambda * l * l / 8.0;
    close(top, 97.6253e6, 5e-3);
    let bottom = mcr_fe(&p, &Load { moment: &parabola, line_load: &q, zg: -150.0 }, &r, 40).expect("bottom").lambda * l * l / 8.0;
    assert!(bottom > 130.0e6 && top < 130.0e6);
    let mid = Restraints { supports: vec![Support::fork(0.0), Support::fork(l), Support { x: l / 2.0, lateral: true, twist: false, warping: false, lateral_rotation: false }], torsional_spring_per_length: 0.0, lateral_springs: vec![] };
    let braced = mcr_fe(&p, &Load { moment: &parabola, line_load: &q, zg: 0.0 }, &mid, 40).expect("braced").lambda * l * l / 8.0;
    assert!(braced > 130.0e6);
}

#[test]
fn review_regressions_shear_moment_curves_and_fire_limits() {
    let rho = shear_rho(0.9 * 348.42e3, 348.42e3);
    close(rho, 0.64, 1e-9);
    close(m_y_v_rd_i(628.4e3, rho, 278.6 * 7.1, 7.1, 235.0, 1.0), 126.95e6, 1e-3);
    assert_eq!(shear_rho(-0.9 * 348.42e3, 348.42e3), 0.0, "signed shear must be passed as magnitude by callers");
    assert_eq!(curve_table_6_2(BucklingSectionKind::WeldedI, true, 400.0, 200.0, 20.0, 355.0), Curve::B);
    assert_eq!(curve_table_6_2(BucklingSectionKind::WeldedI, false, 400.0, 200.0, 20.0, 355.0), Curve::C);
    assert_eq!(curve_table_6_2(BucklingSectionKind::ColdHollow, true, 200.0, 200.0, 8.0, 355.0), Curve::C);
    assert_eq!(curve_table_6_2(BucklingSectionKind::RolledI, true, 300.0, 150.0, 10.0, 460.0), Curve::A0);
    assert_eq!(classify_rhs(200.0, 200.0, 5.0, 355.0, 1.0, 1.0), 4);
    assert_eq!(classify_rhs(100.0, 100.0, 8.0, 355.0, 1.0, 1.0), 1);
    assert_eq!(theta_critical(1.2), 20.0);
    assert_eq!(kappa_1(BeamExposure::AllSides), 1.0);
    assert_eq!(kappa_1(BeamExposure::ProtectedThreeSidesWithSlab), 0.85);
}

#[test]
fn weld_group_elastic_vector_method_matches_hand_values() {
    use super::weld_group::*;
    let line = group_properties(WeldPattern::Line, 0.0, 200.0);
    close(line.i_polar, 200.0_f64.powi(3) / 12.0, 1e-12);
    let two = group_properties(WeldPattern::TwoLines, 100.0, 200.0);
    close(two.length, 400.0, 1e-12);
    close(two.i_polar, 2.0 * 200.0_f64.powi(3) / 12.0 + 2.0 * 200.0 * 50.0 * 50.0, 1e-12);
    let q = max_force_per_length(&two, 100.0e3, 150.0);
    let t = 100.0e3 * 150.0;
    let expected = (100.0e3 / 400.0 + t * 50.0 / two.i_polar).hypot(t * 100.0 / two.i_polar);
    close(q, expected, 1e-12);
    close(resistance_per_length(5.0, 510.0, 0.9, 1.25), 510.0 / 3.0_f64.sqrt() / (0.9 * 1.25) * 5.0, 1e-12);
    let boxed = group_properties(WeldPattern::Box, 100.0, 200.0);
    close(boxed.length, 600.0, 1e-12);
}

#[test]
fn parametric_sections_match_closed_forms() {
    use super::parametric_sections::*;
    let i = welded_i(240.0, 240.0, 10.0, 17.0);
    close(i.area, 2.0 * 240.0 * 17.0 + 206.0 * 10.0, 1e-12);
    close(i.w_pl_y, 240.0 * 17.0 * 223.0 + 10.0 * 206.0 * 206.0 / 4.0, 1e-12);
    let square = rhs(100.0, 100.0, 5.0);
    close(square.iy, square.iz, 1e-12);
    close(square.area, 100.0 * 100.0 - 90.0 * 90.0, 1e-12);
    let tube = chs(168.3, 8.0);
    close(tube.w_el_y, 2.0 * tube.iy / 168.3, 1e-12);
    assert!(max_deviation(&i, &i) == 0.0);
}

#[test]
fn class4_compression_shift_vanishes_for_symmetric_sections_and_matches_the_strip_formula() {
    close(compression_centroid_shift(1000.0, &[(100.0, 50.0)]), -100.0 * 50.0 / 900.0, 1e-12);
    assert!(i_section_compression_shift(1000.0, 300.0, 300.0, 8.0, 20.0, 0.0, 355.0).abs() < 1e-9);
    let mono = i_section_compression_shift(1000.0, 300.0, 500.0, 8.0, 20.0, 0.0, 355.0);
    assert!(mono.abs() > 1e-3, "unequal flanges lose unequal strips and move the effective centroid: {mono}");
}

#[test]
fn weld_group_vector_field_peaks_at_the_corner_and_sums_to_the_applied_shear() {
    use super::weld_group::*;
    let two = group_properties(WeldPattern::TwoLines, 100.0, 200.0);
    let (qx, qy) = vector_at(&two, 100.0e3, 150.0, 50.0, 100.0);
    close(qx.hypot(qy), max_force_per_length(&two, 100.0e3, 150.0), 1e-12);
    let points = sample_points(WeldPattern::TwoLines, 100.0, 200.0, 41);
    let trapezoid = |line: &[(f64, f64)]| line.iter().enumerate().map(|(i, (x, y))| (if i == 0 || i == 40 { 0.5 } else { 1.0 }) * vector_at(&two, 100.0e3, 150.0, *x, *y).1).sum::<f64>() * 5.0;
    let sum_y: f64 = points.chunks(41).map(trapezoid).sum();
    close(sum_y, 100.0e3, 1e-12);
}

fn showcase_end_plate() -> (joint_components::ColumnSide, joint_components::PlateSide, Vec<joint_components::RowGeom>) {
    use joint_components::*;
    let column = ColumnSide { tfc: 17.0, twc: 10.0, dc: 164.0, avc: 2400.0, rc: 21.0, fy: 355.0, w: 110.0, e: 40.0, sigma_com: 50.0, beta: 1.0, tfb: 10.7, ab: 6.0 };
    let plate = PlateSide { tp: 20.0, bp: 200.0, fy: 355.0, twb: 0.0, fy_wb: 0.0 };
    let row = |kind, lever, mx, ex, pitch| RowGeom { kind, lever, m: 40.0, e: 40.0, mx, ex, alpha: 6.5, pitch, bolts: 2.0, d: 20.0, a_s: 245.0, lb: 50.0, ft_rd_bolt: 0.9 * 1000.0 * 245.0 / 1.25, group: false, column_alpha: 0.0 };
    (column, plate, vec![row(RowKind::Extended, 270.0, 40.0, 40.0, 0.0), row(RowKind::FirstBelow, 180.0, 0.0, 0.0, 90.0)])
}

#[test]
fn end_plate_component_rows_match_the_independent_oracle() {
    let (column, plate, rows) = showcase_end_plate();
    let f_beam = 110000.0e3 / 270.0;
    let r = joint_components::component_rows(&column, &plate, &rows, f_beam, 210000.0, 1.0, 1.1);
    close(r.rows[0].f_iso, 265150.0, 1e-9);
    close(r.rows[1].f_iso, 320890.47814207646, 1e-9);
    close(r.rows[0].f_rd, 265150.0, 1e-9);
    close(r.rows[1].f_rd, 142257.40740740742, 1e-9);
    close(r.m_j_rd, 97196833.33333333, 1e-9);
    close(r.z, 230.0677123269161, 1e-9);
    close(r.s_ini, 21605237362.556316, 1e-9);
    assert_eq!((r.rows[0].mode_plate, r.rows[1].mode_plate, r.rows[0].mode_column), (2, 3, 2));
}

#[test]
fn grouped_rows_never_resist_more_than_the_same_rows_individually() {
    let (column, plate, mut rows) = showcase_end_plate();
    let single = joint_components::component_rows(&column, &plate, &rows, 1.0e9, 210000.0, 1.0, 1.1);
    rows[1].group = true;
    let grouped = joint_components::component_rows(&column, &plate, &rows, 1.0e9, 210000.0, 1.0, 1.1);
    assert!(grouped.m_j_rd <= single.m_j_rd + 1e-6, "{} vs {}", grouped.m_j_rd, single.m_j_rd);
    assert!(grouped.rows[0].f_rd + grouped.rows[1].f_rd <= single.rows[0].f_rd + single.rows[1].f_rd + 1e-6);
}

fn three_grouped_rows(pitch_below: f64, pitch_inner: f64) -> Vec<joint_components::RowGeom> {
    let (_, _, mut rows) = showcase_end_plate();
    rows[1].pitch = pitch_below;
    rows.push(joint_components::RowGeom { kind: joint_components::RowKind::Inner, lever: 90.0, pitch: pitch_inner, ..rows[1] });
    rows.iter_mut().skip(1).for_each(|r| r.group = true);
    rows
}

#[test]
fn row_group_lengths_follow_tables_6_4_and_6_6_with_the_inner_row_at_2p_and_p() {
    use std::f64::consts::PI;
    let (column, _, _) = showcase_end_plate();
    let m_c = joint_components::m_rolled(column.w, column.twc, column.rc);
    close(m_c, 33.2, 1e-12);
    let (col, plate) = joint_components::group_lengths(&column, &three_grouped_rows(90.0, 90.0), 0, 2);
    close(col.cp, 2.0 * (PI * 33.2 + 90.0) + 180.0, 1e-12);
    close(col.nc, 2.0 * (2.0 * 33.2 + 0.625 * 40.0 + 45.0) + 90.0, 1e-12);
    close(plate.cp, 2.0 * (PI * 40.0 + 90.0) + 180.0, 1e-12);
    close(plate.nc, 2.0 * (80.0 + 25.0 + 45.0) + 90.0, 1e-12);
    close(plate.cp, 611.3274122871835, 1e-12);
    close(plate.nc, 390.0, 1e-12);
    assert!(plate.nc < 2.0 * (80.0 + 25.0 + 45.0) + 6.5 * 40.0, "the individual inner-row length alpha m = 260 mm must not enter a group");
}

#[test]
fn row_group_lengths_depend_on_the_span_of_the_group_not_on_the_split_of_its_pitches() {
    let (column, _, _) = showcase_end_plate();
    let (even_col, even_plate) = joint_components::group_lengths(&column, &three_grouped_rows(90.0, 90.0), 0, 2);
    let (odd_col, odd_plate) = joint_components::group_lengths(&column, &three_grouped_rows(70.0, 110.0), 0, 2);
    close(odd_col.cp, even_col.cp, 1e-12);
    close(odd_col.nc, even_col.nc, 1e-12);
    close(odd_plate.cp, even_plate.cp, 1e-12);
    close(odd_plate.nc, even_plate.nc, 1e-12);
}

#[test]
fn a_row_adjacent_to_a_column_stiffener_takes_the_table_6_5_group_values() {
    let (column, _, mut rows) = showcase_end_plate();
    rows[1].group = true;
    rows[1].kind = joint_components::RowKind::Inner;
    rows[0].kind = joint_components::RowKind::FirstBelow;
    rows[0].column_alpha = 6.0;
    let (col, plate) = joint_components::group_lengths(&column, &rows, 0, 1);
    close(col.cp, 2.0 * (std::f64::consts::PI * 33.2 + 90.0), 1e-12);
    close(col.nc, (45.0 + 6.0 * 33.2 - (2.0 * 33.2 + 25.0)) + (2.0 * 33.2 + 25.0 + 45.0), 1e-12);
    close(col.nc, 289.2, 1e-12);
    close(plate.cp, 431.32741228718345, 1e-12);
    close(plate.nc, 350.0, 1e-12);
    let single = joint_components::l_eff_column_flange_stiffened_adjacent(33.2, 6.0);
    close(single.cp, 2.0 * std::f64::consts::PI * 33.2, 1e-12);
    close(single.nc, 6.0 * 33.2, 1e-12);
}

#[test]
fn the_group_resistance_uses_the_group_lengths_and_is_checked_alongside_the_single_rows() {
    let (column, mut plate, _) = showcase_end_plate();
    let rows = three_grouped_rows(90.0, 90.0);
    close(joint_components::group_resistance(&column, &plate, &rows, 0, 2, 1.0), 646343.4534320034, 1e-9);
    close(joint_components::group_resistance(&column, &plate, &rows, 1, 2, 1.0), 576748.0601092896, 1e-9);
    plate.tp = 10.0;
    close(joint_components::group_resistance(&column, &plate, &rows, 0, 2, 1.0), 346125.0, 1e-9);
    let result = joint_components::component_rows(&column, &plate, &rows, 1.0e9, 210000.0, 1.0, 1.1);
    let forces: Vec<f64> = result.rows.iter().map(|r| r.f_rd).collect();
    for (row, force) in result.rows.iter().zip(&forces) {
        assert!(*force <= row.f_iso + 1e-6, "a row never exceeds its individual resistance");
    }
    for first in 0..rows.len() {
        for last in first + 1..rows.len() {
            let share: f64 = forces[first..=last].iter().sum();
            let limit = joint_components::group_resistance(&column, &plate, &rows, first, last, 1.0);
            assert!(share <= limit + 1e-6, "rows {first}..={last}: {share} > group {limit}");
        }
    }
    assert!(forces[0] + forces[1] + forces[2] <= 346125.0 + 1e-6);
}

#[test]
fn flange_curling_is_dimensionally_a_length_and_scales_with_the_fourth_power_of_the_width() {
    let u = super::cold_formed::flange_curling(150.0, 60.0, 210000.0, 2.0, 75.0);
    close(u, 0.044082, 1e-4);
    close(super::cold_formed::flange_curling(150.0, 120.0, 210000.0, 2.0, 75.0), 16.0 * u, 1e-12);
}

#[test]
fn block_tearing_areas_of_a_two_by_two_group() {
    let (a_nt, a_nv) = block_tearing_areas(2, 2, 60.0, 60.0, 40.0, 40.0, 22.0, 10.0);
    close(a_nt, 380.0, 1e-12);
    close(a_nv, 1340.0, 1e-12);
    let (single_nt, single_nv) = block_tearing_areas(2, 1, 60.0, 0.0, 40.0, 40.0, 22.0, 10.0);
    close(single_nt, 290.0, 1e-12);
    close(single_nv, 670.0, 1e-12);
    close(block_tearing(510.0, 355.0, a_nt, a_nv, true, 1.0, 1.25), 510.0 * 380.0 / 1.25 + 355.0 * 1340.0 / 3.0_f64.sqrt(), 1e-12);
}

#[test]
fn punching_mean_diameter_is_the_mean_of_across_flats_and_across_points() {
    close(punching_mean_diameter(29.16, 32.95), 31.055, 1e-12);
    close(bp_rd(31.055, 12.0, 510.0, 1.25), 286598.60079660185, 1e-12);
    close(31.055 / punching_mean_diameter(30.0, 30.0 / 30.0_f64.to_radians().cos()), 0.9608450438735313, 1e-12);
}

#[test]
fn bolt_size_designation_names_whole_millimetre_diameters_only() {
    assert_eq!(bolt_size_designation(0.020), "m20");
    assert_eq!(bolt_size_designation(0.024), "m24");
    assert_eq!(bolt_size_designation(0.0205), "m20.5");
}

#[test]
fn weld_flange_effective_width_and_its_inverse() {
    let s = 2.0_f64.sqrt() * 5.0;
    let beff = weld_flange_beff(10.0, s, 17.0, 10.0, 355.0, 355.0, 240.0);
    close(beff, 10.0 + 2.0 * s + 7.0 * 17.0, 1e-12);
    close(weld_flange_thickness_for(beff, 10.0, s, 10.0, 355.0, 355.0), 17.0, 1e-12);
    let thin = weld_flange_beff(10.0, s, 5.0, 10.0, 355.0, 355.0, 240.0);
    close(thin, 10.0 + 2.0 * s + 7.0 * 0.5 * 5.0, 1e-12);
    close(weld_flange_thickness_for(thin, 10.0, s, 10.0, 355.0, 355.0), 5.0, 1e-9);
}

#[test]
fn fire_interaction_helpers_follow_the_bounds_of_4_2_3_5() {
    close(mu_lt_fire(1.0, 1.0), 0.0, 1e-12);
    close(mu_lt_fire(10.0, 2.0), 0.9, 1e-12);
    close(mu_y_fire(1.3, 0.8), (1.2 * 1.3 - 3.0) * 0.8 + 0.44 * 1.3 + 0.29, 1e-12);
    close(mu_y_fire(3.0, 2.0), 0.8, 1e-12);
    close(k_fire(0.0, 1.0e5, 0.5, 1.0e-2, 0.5, 355.0e6, 1.0, 3.0), 1.0, 1e-12);
    close(k_fire(-5.0, 1.0e6, 0.5, 1.0e-2, 0.5, 355.0e6, 1.0, 3.0), 3.0, 1e-12);
    close(m_b_fi_t_rd(0.5, 1.0e-3, 0.5, 355.0e6, 1.0), 88750.0, 1e-12);
    let (a, b) = bending_axial_fire(100.0, 1000.0, 500.0, 50.0, 200.0, 1.2, 0.8);
    close(a, 0.1 + 0.3, 1e-12);
    close(b, 0.2 + 0.2, 1e-12);
}

#[test]
fn e10b_beam_compression_and_web_tension() {
    close(crate::joint_components::fc_fb_rd(110.0e6, 270.0), 110.0e6 / 270.0, 1e-12);
    close(crate::joint_components::fc_fb_rd(110.0e6, 270.0), 407_407.407, 1e-6);
    close(crate::joint_components::ft_wb_rd(210.0, 7.1, 355.0, 1.0), 529_305.0, 1e-12);
    close(crate::joint_components::ft_wb_rd(210.0, 7.1, 355.0, 1.1), 529_305.0 / 1.1, 1e-12);
}

#[test]
fn w3_panel_shear_buckling_terms_of_5_2() {
    let web = crate::plated_ext::panel_shear(2000.0, 1000.0, 8.0, 355.0, 0.0, None, 1.0);
    close(web.k_tau, 6.34, 1e-12);
    close(web.lambda_w, 1.6314509729733295, 1e-12);
    close(web.chi_w, 0.5087495816606243, 1e-12);
    close(web.v_bw_rd, 834_183.850_498_1, 1e-9);
    assert_eq!(web.v_bf_rd, 0.0);
    close(web.v_cap, 1_967_609.719_77, 1e-9);
    let m_f = 300.0 * 20.0 * 1020.0 * 355.0;
    close(crate::plated_ext::flange_hinge_distance(2000.0, 300.0, 20.0, 355.0, 8.0, 1000.0, 355.0), 548.0, 1e-12);
    let with = crate::plated_ext::panel_shear(2000.0, 1000.0, 8.0, 355.0, 0.0, Some((300.0, 20.0, 355.0, m_f, 0.5 * m_f)), 1.0);
    close(with.v_bf_rd, 58_302.919_708_03, 1e-9);
    close(with.v_b_rd, 892_486.770_206_2, 1e-9);
    close(crate::plated_ext::panel_shear(2000.0, 1000.0, 8.0, 355.0, 2.0e7, None, 1.0).k_tau, 41.496_25, 1e-9);
}

#[test]
fn w3_the_beam_web_in_tension_is_a_fourth_row_component_only_when_given() {
    use joint_components::*;
    let (column, mut plate, rows) = showcase_end_plate();
    let without = component_rows(&column, &plate, &rows, 1.0e9, 210000.0, 1.0, 1.1);
    assert!(without.rows.iter().all(|r| r.f_beam_web.is_infinite() && r.governing != RowComponent::BeamWebTension));
    plate.twb = 1.0;
    plate.fy_wb = 355.0;
    let thin = component_rows(&column, &plate, &rows, 1.0e9, 210000.0, 1.0, 1.1);
    plate.twb = 2.0;
    let double = component_rows(&column, &plate, &rows, 1.0e9, 210000.0, 1.0, 1.1);
    for (a, b) in thin.rows.iter().zip(&double.rows) {
        assert_eq!(a.governing, RowComponent::BeamWebTension);
        close(a.f_iso, a.f_beam_web, 1e-12);
        close(b.f_beam_web, 2.0 * a.f_beam_web, 1e-12);
        assert!(a.f_beam_web < without.rows[0].f_iso);
    }
    close(ft_wb_rd(210.0, 7.1, 355.0, 1.0), beam_web_tension(&PlateSide { twb: 7.1, fy_wb: 355.0, ..plate }, 210.0, 1.0), 1e-12);
    assert!(beam_web_tension(&PlateSide { twb: 0.0, ..plate }, 210.0, 1.0).is_infinite());
}

#[test]
fn w3_fire_utilisation_matches_the_scipy_oracle() {
    let history = unprotected_steel_temperature(&gas_iso834, 1.0, 200.0, 1800.0, 10.0, 0.7, 1.0);
    let at = |minutes: f64| history.iter().find(|(t, _)| (*t - minutes * 60.0).abs() < 1e-9).map(|(_, theta)| *theta).expect("sampled");
    for (minutes, theta) in [(5.0, 290.9477752347565), (10.0, 552.4103696512241), (15.0, 681.7811228181926), (30.0, 828.1725515208364)] {
        assert!((at(minutes) - theta).abs() <= 0.03 * theta, "{minutes} min: {} vs {theta}", at(minutes));
    }
    let mu = |minutes: f64| 0.5 / k_y_theta(at(minutes)).max(1e-3);
    close(mu(5.0), 0.5, 1e-12);
    assert!(mu(10.0) > 0.5 && mu(15.0) > mu(10.0) && mu(30.0) > mu(15.0));
    let critical = time_to_temperature(&history, theta_critical(0.5)).expect("reached");
    assert!(mu(critical / 60.0) > 0.9 && mu(critical / 60.0) < 1.1);
}
