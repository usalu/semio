use crate::clauses::bridge_fatigue::*;
use crate::clauses::cold_formed::*;
use crate::clauses::containment::*;
use crate::clauses::crane_runway::*;
use crate::clauses::joint_components::*;
use crate::clauses::plated_ext::*;
use crate::clauses::sheet_piling::*;
use crate::clauses::shell_stability::*;
use crate::clauses::stainless::*;
use crate::clauses::tension_components::*;
use crate::clauses::toughness::*;
use crate::clauses::towers_masts::*;

fn close(a: f64, b: f64, rel: f64, what: &str) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: rust {a} vs reference {b}");
}

fn oracle() -> serde_json::Value {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let script = loop {
        let candidate = dir.join("🔮️oracles/🧮️special-parts/🐍️.py");
        if candidate.is_file() {
            break candidate;
        }
        assert!(dir.pop(), "oracle script not found above the manifest dir");
    };
    let out = semio_s_plugin_norm_compliance::python_oracle::command().arg(&script).output().expect("spawn oracle");
    assert!(out.status.success(), "oracle failed: {}", String::from_utf8_lossy(&out.stderr));
    let text = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(&text[text.find('{').expect("json")..=text.rfind('}').expect("json end")]).expect("parse oracle")
}

#[test]
fn sp01_cold_formed_distortional_shear_crippling() {
    let g = EdgeStiffened { bp: 60.0, cp: 20.0, t: 1.5, fyb: 350.0, hw: 150.0, b2: 60.0, kf: 1.0, e: 210000.0 };
    let d = distortional_edge_stiffener(&g, 1.0);
    assert!(d.chi_d > 0.0 && d.chi_d < 1.0 && d.iterations > 1 && d.iterations < 30);
    let lw = lambda_w_cold(150.0, 1.5, 350.0, 210000.0);
    let fbv = f_bv(lw, 350.0, false);
    assert!(lw > 1.4);
    close(fbv, 0.67 * 350.0 / (lw * lw), 1e-12, "fbv");
    close(f_bv(lw, 350.0, true), 0.48 * 350.0 / lw, 1e-12, "fbv stiffened support");
    close(f_bv(1.1, 350.0, false), 0.48 * 350.0 / 1.1, 1e-12, "fbv mid");
    close(f_bv(0.5, 350.0, false), 0.58 * 350.0, 1e-12, "fbv plateau");
    close(f_bv(1.6, 350.0, false), 0.67 * 350.0 / 2.56, 1e-12, "fbv slender");
    close(k_sigma_edge_fold(0.2), 0.5, 1e-12, "k edge fold");
    close(k_sigma_edge_fold(0.6), 0.5 + 0.83 * 0.25_f64.powf(2.0 / 3.0), 1e-12, "k edge fold max");
    let (a, la) = web_crippling_alpha(1, 40.0);
    assert!((a, la) == (0.075, 10.0));
    close(bending_crippling_utilisation(1.0, 2.0, 1.0, 2.0), 0.8, 1e-12, "M and F");
}

#[test]
fn sp02_stainless_thresholds_and_tables() {
    close(rho_internal_stainless(0.4, false), 1.0, 1e-12, "rho 1");
    close(rho_outstand_stainless(1.0, true), 1.0 - 0.242, 1e-12, "rho out welded");
    close(secant_modulus(220.0, 200000.0, 220.0, 7.0), 220.0 / (220.0 / 200000.0 + 0.002), 1e-12, "secant at f0.2");
    assert!(stainless_fire_factors("1.4462", 500.0).is_none());
    let (k, ke, _) = stainless_fire_factors("1.4301", 500.0).expect("table");
    assert!((k, ke) == (0.54, 0.80));
    close(k_interaction(0.3, 0.5), 1.2, 1e-12, "k lower bound");
    close(k_interaction(3.0, 0.5), 2.2, 1e-12, "k upper bound");
}

#[test]
fn v255_to_v257_and_v260_cold_formed_shear_crippling_and_restraint_follow_the_hand_formulas() {
    let (t, fyb, e, hw) = (1.5, 350.0, 210000.0, 150.0);
    let lw = 0.346 * hw / t * (fyb / e).sqrt();
    close(lambda_w_cold(hw, t, fyb, e), lw, 1e-12, "lambda_w");
    close(v_b_rd_cold(hw, 90.0, t, f_bv(lw, fyb, false), 1.1), hw * t * 0.67 * fyb / (lw * lw) / 1.1, 1e-12, "V_b,Rd");
    let (alpha, la) = web_crippling_alpha(1, 40.0);
    let rw = alpha * t * t * (fyb * e).sqrt() * (1.0 - 0.1 * (3.0_f64 / t).sqrt()) * (0.5 + (0.02 * la / t).sqrt()) * (2.4 + 1.0) / 1.0;
    close(web_crippling_rd(alpha, t, fyb, e, 3.0, la, 90.0, 1.0), rw, 1e-12, "R_w,Rd");
    close(bending_crippling_utilisation(0.5, 1.0, 0.5, 1.0), 1.0 / 1.25, 1e-12, "M + F over 1.25");
    let c_b = e * t.powi(3) / (4.0 * (1.0 - NU_STEEL * NU_STEEL) * hw);
    close(c_d_web_bending(e, t, hw), c_b, 1e-12, "C_D,B");
    close(c_d_series(2000.0, c_b), 1.0 / (1.0 / 2000.0 + 1.0 / c_b), 1e-12, "C_D series");
}

#[test]
fn v258_cold_formed_axial_bending_adds_the_two_utilisations() {
    close(axial_bending_utilisation(30.0, 100.0, 2.0, 5.0), 0.3 + 0.4, 1e-12, "N + M");
    close(axial_bending_utilisation(0.0, 100.0, 5.0, 5.0), 1.0, 1e-12, "pure bending");
}

#[test]
fn v301_to_v305_stainless_effective_width_resistance_buckling_fire_and_ramberg_osgood() {
    let v = oracle();
    let f = |k: &str| v[k].as_f64().unwrap_or_else(|| panic!("missing {k}"));
    close(rho_internal_stainless(0.9, false) * 100.0, 100.0 * f("st_rho_int_cold"), 1e-12, "V-301 b_eff internal");
    close(rho_outstand_stainless(0.9, true) * 100.0, 100.0 * f("st_rho_out_weld"), 1e-12, "V-301 b_eff outstand welded");
    close(section_rd(2000.0, 220.0, 1.1), 2000.0 * 220.0 / 1.1, 1e-12, "V-302 N_c,Rd");
    close(chi_flexural_stainless(1.0, StainlessMember::ColdFormedOpen), f("st_chi_flex"), 1e-9, "V-303 chi");
    close(chi_lt_stainless(0.8, StainlessMember::WeldedOpen), f("st_chi_lt_welded"), 1e-9, "V-303 chi_LT");
    let (k, ke, _) = stainless_fire_factors("1.4301", 650.0).expect("V-304");
    close(k, f("st_k02_650"), 1e-12, "V-304 k0.2p");
    close(ke, f("st_ke_650"), 1e-12, "V-304 kE");
    close(crate::clauses::cold_stainless::ramberg_osgood_strain(200.0, 200000.0, 220.0, 7.0), f("st_ro_strain"), 1e-12, "V-305 strain");
    close(crate::clauses::cold_stainless::ramberg_osgood_tangent(200.0, 200000.0, 220.0, 7.0), f("st_ro_tangent"), 1e-5, "V-305 tangent");
}

#[test]
fn v305_stainless_grades_are_a_typed_enum_over_the_registry_rows() {
    use crate::clauses::cold_stainless::{stainless_grade, StainlessGradeId};
    for id in StainlessGradeId::ALL {
        assert_eq!(StainlessGradeId::from_name(id.name()), Some(id));
        let g = stainless_grade(id.name()).expect("registry row");
        assert!(g.fu > g.fy && g.n > 1.0, "{}", id.name());
    }
    assert!(StainlessGradeId::from_name("1.9999").is_none() && stainless_grade("S355").is_none());
}

#[test]
fn sp03_plated_patch_types_and_interactions() {
    close(sigma_e(210000.0, 10.0, 500.0), 210000.0 * std::f64::consts::PI.powi(2) / (12.0 * 0.91) * 4.0e-4, 1e-12, "sigma_E");
    close(mv_interaction(100.0, 400.0, 100.0, 0.3 * 100.0, 100.0).min(1.0), 0.25f64.max(0.3), 1e-12, "low shear = max(eta)");
    close(i_st_min(2000.0, 1000.0, 8.0), 0.75 * 1000.0 * 512.0, 1e-12, "I_st");
    close(shear_lag_beta_hogging(0.01), 1.0, 1e-12, "beta hogging 1");
    close(shear_lag_beta_end(0.02, 0.8), 0.8, 1e-12, "beta0 capped");
    close(shear_lag_beta_end(0.5, 0.8), 0.6 * 0.8, 1e-12, "beta0");
    close(patch_bending_utilisation(1.0, 1.0, 0.5, 1.0), 1.4 / 1.4, 1e-12, "F+M");
    let a = patch_resistance(PatchType::Through, 1000.0, 8.0, 2000.0, 20.0, 300.0, 355.0, 355.0, 100.0, 150.0, 1.0);
    let c = patch_resistance(PatchType::EndFlange, 1000.0, 8.0, 2000.0, 20.0, 300.0, 355.0, 355.0, 100.0, 150.0, 1.0);
    assert!(a.f_rd > c.f_rd && c.l_y <= 250.0);
}

#[test]
fn sp04_shell_classes_and_interaction() {
    assert!(fabrication_q('A') > fabrication_q('B') && fabrication_q('B') > fabrication_q('C'));
    close(c_x(1.0, 1000.0, 5.0, 6.0), 1.36 - 1.83 + 2.07, 1e-12, "Cx short");
    close(c_x(40.0, 1000.0, 5.0, 6.0), 1.0, 1e-12, "Cx medium");
    close(chi_shell(0.1, 0.5, 0.6, 1.0, 0.2), 1.0, 1e-12, "chi plateau");
    close(interaction_ls3(0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0), 0.0, 1e-12, "no stress");
    close(interaction_ls3(1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0), 1.0, 1e-12, "pure axial at resistance");
    assert!(axial_design(3000.0, 1000.0, 5.0, 235.0, 'A', 6.0, 1.1).sigma_rd > axial_design(3000.0, 1000.0, 5.0, 235.0, 'C', 6.0, 1.1).sigma_rd);
}

#[test]
fn sp05_toughness_table() {
    assert!(max_thickness("S355", "J2", 0.75, -60.0).is_none() && max_thickness("S460", "N", 0.75, 0.0).is_none() && max_thickness("S355", "J2", 0.9, 0.0).is_none());
    close(max_thickness("S355", "K2", 0.5, -10.0).expect("K2"), 90.0, 1e-12, "S355 K2 -10");
    assert_eq!(limit("S355", "K2", 0.5, -10.0).map(|l| l.conservative), Ok(true));
    assert_eq!(limit("S355", "J2", 0.9, 0.0), Err(Gap::StressAboveTable));
    assert_eq!(limit("S355", "J2", 0.75, -60.0), Err(Gap::TemperatureBelowTable));
    assert_eq!(limit("S690", "Q", 0.75, 0.0), Err(Gap::UnknownColumn));
    assert_eq!(limit("S460", "QL1", 0.75, 0.0), Err(Gap::CellNotTranscribed(cell_id("S460", "QL1", "r75", 10.0))));
    assert_eq!(sub_grades("S460"), ["N", "NL", "M", "ML", "Q", "QL", "QL1"]);
    let stress = |level: usize| -> Result<f64, Gap> { if level == 0 { Err(Gap::CellNotTranscribed("r25".into())) } else { Ok([0.0, 100.0, 120.0][level]) } };
    close(select(0.6, &stress).expect("interpolated").t_max, 108.0, 1e-12, "blend 0.6");
    assert!(select(0.3, &stress).expect("fallback").conservative);
    assert_eq!(select(0.5, &stress).map(|l| (l.t_max, l.conservative, l.levels)), Ok((100.0, false, vec![0.5])));
    let (t_ed, u) = brittle_fracture_utilisation("S355", "J2", 0.75, 45.0, -20.0, 0.0, 0.0, 0.0, 0.0, 0.0).expect("util");
    close(t_ed, -20.0, 1e-12, "T_Ed");
    close(u, 45.0 / 55.0, 1e-12, "utilisation");
}

#[test]
fn sp06_joint_components_equilibrium_and_classification() {
    let l = l_eff_column_flange_single(40.0, 30.0);
    close(l.l1(), 197.5, 1e-12, "leff1");
    close(l.cp, 2.0 * std::f64::consts::PI * 40.0, 1e-12, "leff cp");
    close(l.l2(), 4.0 * 40.0 + 1.25 * 30.0, 1e-12, "leff2");
    let t = t_stub_modes(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 8.0, 282240.0, 245.0, 2.0, 100.0, Mode1Method::One, 1.0);
    assert!(t.prying && t.mode == 3);
    let t0 = t_stub_modes(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 8.0, 282240.0, 245.0, 2.0, 300.0, Mode1Method::One, 1.0);
    assert!(!t0.prying && t0.ft_2.is_infinite());
    let e10 = t_stub_modes(210.0, 210.0, 15.0, 355.0, 40.0, 40.0, 0.0, 282240.0, 245.0, 2.0, 60.0, Mode1Method::Two, 1.0);
    assert!(e10.prying && e10.mode == 2);
    close(e10.ft_rd, 245_955.9375, 1e-9, "E10 F_T,Rd");
    assert_eq!(classify_stiffness_frame(1.0e12, 210000.0, 1.0e8, 5000.0, false, 0.05), "semi-rigid");
    assert_eq!(classify_strength_full(50.0, 100.0, 100.0, true), "partial");
    assert!(base_plate_m_rd(1.0e7, 1.0, 1.0, 1.0, 1.0, 300.0, 300.0).is_none());
    let curve = m_phi_curve(1.0e9, 1.0e6, 2.7, 30);
    assert!(curve.windows(2).all(|w| w[1].0 > w[0].0 && w[1].1 > w[0].1));
}

#[test]
fn sp07_special_parts_match_independent_oracle() {
    let v = oracle();
    let f = |k: &str| v[k].as_f64().unwrap_or_else(|| panic!("missing {k}"));
    let g = EdgeStiffened { bp: 60.0, cp: 20.0, t: 1.5, fyb: 350.0, hw: 150.0, b2: 60.0, kf: 1.0, e: 210000.0 };
    let d = distortional_edge_stiffener(&g, 1.0);
    close(d.chi_d, f("cf_chi_d"), 1e-6, "chi_d");
    close(d.lambda_d, f("cf_lambda_d"), 1e-6, "lambda_d");
    close(d.sigma_cr_s, f("cf_sigma_cr_s"), 1e-6, "sigma_cr_s");
    close(d.a_s, f("cf_a_s"), 1e-6, "A_s");
    close(d.i_s, f("cf_i_s"), 1e-6, "I_s");
    let fbv = f_bv(lambda_w_cold(150.0, 1.5, 350.0, 210000.0), 350.0, false);
    close(fbv, f("cf_fbv"), 1e-9, "fbv");
    close(v_b_rd_cold(150.0, 90.0, 1.5, fbv, 1.0), f("cf_vb_rd"), 1e-9, "Vb,Rd");
    close(web_crippling_rd(0.075, 1.5, 350.0, 210000.0, 3.0, 10.0, 90.0, 1.0), f("cf_rw_rd"), 1e-9, "Rw,Rd");

    close(rho_internal_stainless(0.9, false), f("st_rho_int_cold"), 1e-12, "rho int cold");
    close(rho_internal_stainless(0.9, true), f("st_rho_int_weld"), 1e-12, "rho int welded");
    close(rho_outstand_stainless(0.9, false), f("st_rho_out_cold"), 1e-12, "rho out cold");
    close(rho_outstand_stainless(0.9, true), f("st_rho_out_weld"), 1e-12, "rho out welded");
    close(secant_modulus(200.0, 200000.0, 220.0, 7.0), f("st_es"), 1e-12, "Es");
    let (k, ke, _) = stainless_fire_factors("1.4301", 650.0).expect("fire");
    close(k, f("st_k02_650"), 1e-12, "k0.2p");
    close(ke, f("st_ke_650"), 1e-12, "kE");
    close(chi_flexural_stainless(1.0, StainlessMember::ColdFormedOpen), f("st_chi_flex"), 1e-9, "chi stainless");
    close(chi_lt_stainless(0.8, StainlessMember::WeldedOpen), f("st_chi_lt_welded"), 1e-9, "chi_LT stainless");
    close(k_interaction(0.9, 0.3), f("st_k_int"), 1e-12, "k_y");

    close(sigma_e(210000.0, 10.0, 500.0), f("pl_sigma_e") * 0.91 / 0.91, 1e-12, "sigma_E");
    close(rho_c_interpolated(0.8, 0.6, 300.0, 200.0), f("pl_rho_c"), 1e-12, "rho_c");
    close(shear_lag_beta_hogging(0.4), f("pl_beta_hog"), 1e-12, "beta hogging");
    close(shear_lag_beta_hogging(1.2), f("pl_beta_hog_hi"), 1e-12, "beta hogging high");
    let p = |kind| patch_resistance(kind, 1000.0, 8.0, 2000.0, 20.0, 300.0, 355.0, 355.0, 100.0, 150.0, 1.0).f_rd;
    close(p(PatchType::Through), f("pl_patch_a"), 1e-9, "patch a");
    close(p(PatchType::OneFlangeNearEnd), f("pl_patch_b"), 1e-9, "patch b");
    close(p(PatchType::EndFlange), f("pl_patch_c"), 1e-9, "patch c");
    close(k_tau_stiffened(2000.0, 1000.0, 8.0, 2.0e6), f("pl_k_tau"), 1e-9, "k_tau");
    close(k_tau_stiffened(500.0, 1000.0, 8.0, 2.0e6), f("pl_k_tau_low"), 1e-9, "k_tau a/hw < 1");
    close(k_tau_stiffened(500.0, 1000.0, 8.0, 2.0e6), 125.3882168146894, 1e-9, "k_tau a/hw < 1 hand value 4 + 5.34 * 4 + 100.0282");
    close(k_tau_stiffened(2000.0, 1000.0, 8.0, 1.0e3), f("pl_k_tau_floor"), 1e-9, "k_tau floor");
    close(k_tau_stiffened(2000.0, 1000.0, 8.0, 1.0e3), 6.6025, 1e-9, "k_tau floor hand value 5.34 + 1 + 0.2625");
    close(k_tau_stiffened(1000.0, 1000.0, 8.0, 0.0), 9.34, 1e-9, "k_tau is continuous at a/hw = 1");
    close(k_tau_stiffened(999.999, 1000.0, 8.0, 0.0), 9.34, 1e-4, "k_tau is continuous at a/hw = 1 from below");
    close(mv_interaction(0.8 * 400.0, 400.0, 100.0, 0.8, 1.0), f("pl_mv"), 1e-12, "M-V");
    close(i_st_min(2000.0, 1000.0, 8.0), f("pl_ist"), 1e-12, "I_st");
    close(alpha_ult_k(100.0, 50.0, 30.0, 355.0), f("pl_alpha_ult"), 1e-12, "alpha_ult");

    let ds = axial_design(3000.0, 1000.0, 5.0, 235.0, 'B', 6.0, 1.1);
    close(omega(3000.0, 1000.0, 5.0), f("sh_omega"), 1e-12, "omega");
    close(ds.sigma_cr, f("sh_sigma_cr"), 1e-12, "sigma_x,Rcr");
    close(ds.lambda, f("sh_lambda"), 1e-12, "lambda shell");
    close(alpha_x(1000.0, 5.0, 25.0), f("sh_alpha"), 1e-12, "alpha_x");
    close(ds.chi, f("sh_chi"), 1e-12, "chi_x");
    close(ds.sigma_rd, f("sh_sigma_rd"), 1e-12, "sigma_x,Rd");
    let hs = hoop_design(3000.0, 1000.0, 5.0, 235.0, 'B', 1.5, 1.1);
    close(hs.sigma_cr, f("sh_theta_cr"), 1e-12, "sigma_theta,Rcr");
    close(hs.chi, f("sh_theta_chi"), 1e-12, "chi_theta");
    close(interaction_ls3(0.5, 1.0, ds.chi, 0.4, 1.0, hs.chi, 0.2, 1.0, 0.7), f("sh_interaction"), 1e-12, "LS3 interaction");
    close(c_x(1.0, 1000.0, 5.0, 6.0), f("sh_omega_short_cx"), 1e-12, "Cx short");

    close(brittle_fracture_utilisation("S355", "J2", 0.75, 1.0, 10.0, 0.0, 0.0, 0.0, 0.0, crate_cold_shift(20.0, 100.0)).expect("t").0, f("tf_t_ed"), 1e-12, "T_Ed");
    close(max_thickness("S355", "J2", 0.75, -5.0).expect("tmax"), f("tf_tmax"), 1e-12, "tmax");
    close(max_thickness("S355", "J2", 0.75, -25.0).expect("tmax"), f("tf_tmax_s355j2_m25"), 1e-12, "tmax -25");
    close(max_thickness("S355", "K2", 0.75, -15.0).expect("tmax"), f("tf_tmax_s355k2_m15"), 1e-12, "K2 -15");
    let fallback = limit("S355", "K2", 0.5, -10.0).expect("fallback");
    close(fallback.t_max, f("tf_tmax_s355k2_r50_m10_fallback"), 1e-12, "K2 r50 fallback");
    assert!(fallback.conservative && fallback.levels == vec![0.75]);
    let ids = GRADES.iter().flat_map(|(g, subs)| subs.iter().map(move |s| (*g, *s))).flat_map(|(g, s)| LEVELS.iter().flat_map(move |(_, key)| TEMPERATURES.iter().map(move |&t| cell_id(g, s, key, t))));
    let (total, unset) = ids.fold((0.0, 0.0), |(n, u), id| (n + 1.0, u + f64::from(u8::from(cell(&id).is_none()))));
    close(total, f("tf_cells_total"), 1e-12, "Table 2.1 cells");
    close(unset, f("tf_cells_unset"), 1e-12, "untranscribed cells");
    let columns = |level: usize| -> Result<f64, Gap> { Ok([80.0, 100.0, 120.0][level]) };
    close(select(0.6, &columns).expect("blend").t_max, f("tf_blend_r60"), 1e-12, "blend 0.6");
    close(select(0.1, &columns).expect("below").t_max, f("tf_blend_below_r25_conservative"), 1e-12, "below 0.25");

    close(ernst_modulus(210000.0, 7.85e-5, 100.0e3, 400.0), f("tc_ernst"), 1e-12, "Ernst");
    close(cable_sag(0.5, 50000.0, 250.0e3), f("tc_sag"), 1e-12, "sag");
    close(pin_moment_ed(120.0e3, 10.0, 30.0, 5.0), f("tc_pin_m"), 1e-12, "pin moment");
    close(pin_combined(40.0e6, 80.0e6, 60.0e3, 100.0e3), f("tc_pin_comb"), 1e-12, "pin combined");
    close(rope_min_breaking(0.6, 20.0, 1770.0), f("tc_rope"), 1e-12, "rope");

    let t = t_stub_modes(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 8.0, 282240.0, 245.0, 2.0, 100.0, Mode1Method::One, 1.0);
    assert_eq!(t.prying, f("jc_prying_at_100") > 0.5);
    close(t.l_b_star, f("jc_lb_star"), 1e-12, "Lb*");
    close(t.ft_1, f("jc_f1_m1"), 1e-12, "F_T1 method 1");
    close(t.ft_2, f("jc_f2"), 1e-12, "F_T2");
    close(t.ft_rd, f("jc_ft_rd_m1"), 1e-12, "F_T,Rd");
    let t2 = t_stub_modes(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 8.0, 282240.0, 245.0, 2.0, 100.0, Mode1Method::Two, 1.0);
    close(t2.ft_1, f("jc_f1_m2"), 1e-12, "F_T1 method 2");
    let t0 = t_stub_modes(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 8.0, 282240.0, 245.0, 2.0, 300.0, Mode1Method::One, 1.0);
    assert_eq!(t0.prying, f("jc_prying_at_300") > 0.5);
    close(t0.ft_rd, f("jc_ft_noprying"), 1e-12, "no prying");
    let (z, keq) = z_eq_and_k_eq(&[12.0, 8.0], &[300.0, 220.0]);
    close(z, f("jc_z_eq"), 1e-12, "z");
    close(keq, f("jc_k_eq"), 1e-12, "k_eq");
    close(omega_table_6_3(0.75, 200.0, 9.0, 3000.0), f("jc_omega"), 1e-12, "omega 0.75");
    close(omega_table_6_3(1.5, 200.0, 9.0, 3000.0), f("jc_omega_15"), 1e-12, "omega 1.5");
    let grp = |i: usize, j: usize| match (i, j) {
        (0, 1) => 420.0e3,
        (0, 2) => 560.0e3,
        _ => 380.0e3,
    };
    let rows = distribute_rows(&[300.0e3, 250.0e3, 200.0e3], &grp, 650.0e3);
    let expected: Vec<f64> = v["jc_rows"].as_array().expect("rows").iter().map(|x| x.as_f64().expect("num")).collect();
    for (a, b) in rows.iter().zip(&expected) {
        close(*a, *b, 1e-12, "row force");
    }
    close(m_j_rd_rows(&[300.0, 220.0, 140.0], &rows), f("jc_mj"), 1e-12, "Mj,Rd");
    close(mu_stiffness(0.9e6, 1.0e6, 2.7), f("jc_mu"), 1e-12, "mu");
    close(s_j_ini_joint(210000.0, 300.0, 4000.0, 9000.0, 6000.0), f("jc_sj_ini"), 1e-12, "Sj,ini");
    close(base_plate_m_rd(100.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 300.0, 300.0).expect("a"), f("jc_bp_a"), 1e-9, "base plate a");
    close(base_plate_m_rd(-200.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 300.0, 300.0).expect("b"), f("jc_bp_b"), 1e-9, "base plate b");
    close(base_plate_m_rd(100.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 200.0, 400.0).expect("c"), f("jc_bp_c"), 1e-9, "base plate c, tension side lever 200 mm, compression side lever 400 mm");
    close(base_plate_m_rd(-300.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 200.0, 400.0).expect("d"), f("jc_bp_d"), 1e-9, "base plate d");
    close(base_plate_m_rd(100.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 200.0, 400.0).expect("c"), 50.0e6, 1e-12, "hand value: T = 150 kN at 200 mm plus C = 50 kN at 400 mm");
    close(base_plate_m_rd(-300.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 200.0, 400.0).expect("d"), 180.0e6, 1e-12, "hand value: T = 100 kN at 200 mm plus C = 400 kN at 400 mm");
    close(f_jd_bearing(2.0 / 3.0, 300.0 * 300.0, 300.0 * 300.0 * 1.5, 20.0, 150.0, 250.0), f("jc_fjd"), 1e-12, "fjd");
    let spread = |b: f64, l: f64| (b * l, 9.0 * b * l);
    let free = base_plate_c_strip(17.0, 240.0, 30.0, 355.0, 1.0, 350.0, 450.0, 2.0 / 3.0, 20.0, &spread);
    let held = base_plate_c_strip(17.0, 240.0, 30.0, 355.0, 1.0, 350.0, 300.0, 2.0 / 3.0, 20.0, &spread);
    close(free.f_jd, 40.0, 1e-12, "f_jd = beta_j 3 f_cd");
    close(free.c, 51.59941860137573, 1e-12, "hand value: c = 30 sqrt(355/120)");
    close(free.b_eff, 120.19883720275146, 1e-12, "hand value: b_eff = 17 + 2c");
    close(free.l_eff, 343.19883720275146, 1e-12, "hand value: l_eff = b_fc + 2c = 240 + 103.2 on a 450 mm plate, not the plate width");
    close(free.fc_rd, 1650084.0464442852, 1e-12, "hand value: F_c,Rd = 40 N/mm2 x 120.2 x 343.2 mm");
    close(free.l_eff, f("jc_cs_leff_free"), 1e-12, "l_eff free");
    close(free.fc_rd, f("jc_cs_fc_free"), 1e-12, "F_c,Rd free");
    close(held.l_eff, 300.0, 1e-12, "hand value: the plate edge limits l_eff to 300 mm");
    close(held.fc_rd, 1442386.0464330176, 1e-12, "hand value: F_c,Rd = 40 x 120.2 x 300");
    close(held.l_eff, f("jc_cs_leff_plate"), 1e-12, "l_eff plate");
    close(held.fc_rd, f("jc_cs_fc_plate"), 1e-12, "F_c,Rd plate");
    close(base_plate_c_required(17.0, 240.0, 450.0, 8750.0), 8.520615635708353, 1e-12, "hand value: (17 + 2c)(240 + 2c) = 8750");
    close(base_plate_c_required(17.0, 240.0, 450.0, 8750.0), f("jc_cr_free"), 1e-9, "required c free");
    close(base_plate_c_required(17.0, 240.0, 250.0, 8750.0), 9.0, 1e-12, "hand value: plate limited, (8750 / 250 - 17) / 2");
    close(base_plate_c_required(17.0, 240.0, 250.0, 8750.0), f("jc_cr_plate"), 1e-12, "required c plate");
    close(base_plate_c_required(17.0, 240.0, 450.0, 1000.0), 0.0, 1e-12, "the bare flange already bears the area");

    close(lambda_2(0.5, 0.5, 1.2e6, 0.5e6), f("br_l2"), 1e-12, "lambda2");
    close(lambda_3(80.0), f("br_l3"), 1e-12, "lambda3");
    close(lambda_4(&[(0.4, 0.8), (0.2, 0.6)], 1.0, 1.0), f("br_l4"), 1e-12, "lambda4");
    close(lambda_1_interpolated(&[(2.0, 2.5), (10.0, 2.0), (50.0, 1.2)], 30.0), f("br_l1"), 1e-12, "lambda1");
    close(delta_sigma_e2_bridge(1.1, 1.0, 0.9, 1.05, 2.0, 1.0, 40.0), f("br_ds"), 1e-12, "dsigma E2");
    close(category_at_or_below(103.0).expect("cat"), f("br_cat"), 1e-12, "category");
    close(size_effect(40.0), f("br_ks"), 1e-12, "size effect");

    close(v_crit(2000.0, 1.2, 0.2), f("tw_vcrit"), 1e-12, "v_crit");
    close(scruton(0.05, 80.0, 1.25, 2.0), f("tw_sc"), 1e-12, "Scruton");
    close(cantilever_frequency(30000.0, 2.1e5 * 5.0e9, 100.0), f("tw_freq"), 1e-12, "cantilever");
    close(vortex_cycles(50.0, 1.0e6, 5.0, 8.0), f("tw_cycles"), 1e-12, "vortex cycles");
    close(ovalling_frequency(3.0, 500.0, 6.0, 210000.0, 7.85e-9), f("tw_ovalling"), 1e-12, "ovalling");

    let (gamma, kk, mu, a, u, z) = (9.0e-6, 0.5, 0.4, std::f64::consts::PI * 9.0e6, 2.0 * std::f64::consts::PI * 3000.0, 12000.0);
    close(janssen_ph(gamma, kk, mu, a, u, z), f("ct_ph"), 1e-12, "ph");
    close(janssen_nx(gamma, kk, mu, a, u, z), f("ct_nx"), 1e-9, "nx");
    close(janssen_pv(gamma, kk, mu, a, u, z), f("ct_pv"), 1e-12, "pv");
    close(tank_course_thickness(1000.0, 15000.0, 3000.0, 10000.0, 235.0, 1.0), f("ct_tank_t"), 1e-12, "tank");
    close(pipe_wall_thickness(8.0, 508.0, 355.0, 1.1), f("ct_pipe_t"), 1e-12, "pipe");
    close(restrained_axial_stress(0.3, hoop_membrane(8.0, 500.0, 10.0), 210000.0, 1.2e-5, 40.0), f("ct_pipe_ax"), 1e-12, "restrained");
    close(pipe_p_cr(210000.0, 0.3, 10.0, 500.0), f("ct_pcr"), 1e-12, "p_cr");

    close(corrosion_loss(Environment::UndisturbedSoil, 30.0), f("pi_loss_30"), 1e-12, "loss 30");
    close(corrosion_loss(Environment::PollutedSoil, 3.0), f("pi_loss_3"), 1e-12, "loss 3");
    close(fy_reduced_by_shear(0.8, 1.0, 355.0), f("pi_fy_red"), 1e-12, "fy red");
    close(v_pl_rd_pile(3000.0, 355.0, 1.0), f("pi_vpl"), 1e-12, "Vpl");

    let irf = i_rf(1.2e6, 200.0, 20.0, true);
    let le = l_eff(irf, 12.0);
    close(le, f("cr_leff"), 1e-12, "leff");
    let soz = sigma_oz(180.0e3, le, 12.0);
    close(soz, f("cr_soz"), 1e-12, "sigma_oz");
    close(tau_oxz(soz), f("cr_tau"), 1e-12, "tau_oxz");
    close(web_von_mises(120.0, soz, tau_oxz(soz), 355.0, 1.0), f("cr_vm"), 1e-12, "von Mises");
    close(hertz_line_contact(180.0e3, 210000.0, 50.0, 300.0), f("cr_hertz"), 1e-12, "Hertz");
    close(lambda_crane(0.5, 0.25, 3.0), f("cr_lambda"), 1e-12, "lambda crane");
    close(phi_2(1.1, 0.3, 0.5), f("cr_phi2"), 1e-12, "phi2");
}

#[test]
fn sp09_angle_and_chs_sections_match_the_sectionproperties_oracle() {
    use crate::clauses::classification::{angle_class, chs_class};
    use crate::clauses::parametric_sections::{angle, chs};
    let v = oracle();
    let f = |key: &str| v[key].as_f64().unwrap_or_else(|| panic!("missing {key}"));
    let a = angle(100.0, 65.0, 8.0);
    close(a.area, f("sec_ang_a"), 1e-9, "angle A");
    close(a.iu, f("sec_ang_iu"), 1e-6, "angle I_u");
    close(a.iv, f("sec_ang_iv"), 1e-6, "angle I_v");
    close(a.alpha.to_degrees().rem_euclid(180.0), f("sec_ang_alpha"), 1e-6, "angle alpha");
    close(a.w_el_u, f("sec_ang_wel_u"), 1e-6, "angle W_el,u");
    close(a.w_el_v, f("sec_ang_wel_v"), 1e-6, "angle W_el,v");
    close(a.w_pl_u, f("sec_ang_wpl_u"), 1e-5, "angle W_pl,u");
    close(a.w_pl_v, f("sec_ang_wpl_v"), 1e-5, "angle W_pl,v");
    close(a.it, f("sec_ang_it"), 4e-2, "angle I_t");
    let c = chs(168.3, 6.3);
    close(c.area, f("sec_chs_a"), 2e-3, "CHS A");
    close(c.iy, f("sec_chs_i"), 3e-3, "CHS I");
    close(c.w_el_y, f("sec_chs_wel"), 3e-3, "CHS W_el");
    close(c.w_pl_y, f("sec_chs_wpl"), 3e-3, "CHS W_pl");
    close(crate::clauses::parametric_sections::chs_torsion(168.3, 6.3), f("sec_chs_it"), 3e-3, "CHS I_t");
    let eps = (235.0_f64 / 355.0).sqrt();
    close(f64::from(angle_class(100.0, 65.0, 8.0, eps)), f("sec_ang_class"), 1e-12, "angle class");
    close(f64::from(chs_class(168.3 / 6.3, eps)), f("sec_chs_class"), 1e-12, "CHS class");
}

fn crate_cold_shift(t: f64, r: f64) -> f64 {
    crate::clauses::misc::cold_forming_shift(t, r)
}

#[test]
fn sp08_wired_checks_match_independent_oracle() {
    use crate::check_full_steel_structure;
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;
    let v = oracle();
    let report = check_full_steel_structure(&En1993Snapshot::special_parts_showcase());
    for (id, key) in [("en1993.4-2.6.silo-1", "w_tank"), ("en1993.1-10.t.member-b1", "w_brittle"), ("en1993.5.5.2.il.pile-1", "w_pile_il"), ("en1993.1-3.6.1.5.cf-1", "w_cf_shear"), ("en1993.4-1.5.2.ph.silo-1", "w_silo_ph"), ("en1993.6.5.7.vm.crane-1", "w_crane_vm")] {
        let check = report.checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}"));
        close(check.utilization, v[key].as_f64().unwrap_or_else(|| panic!("missing {key}")), 1e-9, id);
    }
}

#[test]
fn the_pin_moment_counts_the_gap_four_times_the_outer_plates_twice_and_the_middle_plate_once() {
    close(pin_moment_of_plates(100.0e3, 20.0, 25.0, 2.0), 912_500.0, 1e-12, "100 kN (25 + 4 * 2 + 2 * 20) / 8");
}
