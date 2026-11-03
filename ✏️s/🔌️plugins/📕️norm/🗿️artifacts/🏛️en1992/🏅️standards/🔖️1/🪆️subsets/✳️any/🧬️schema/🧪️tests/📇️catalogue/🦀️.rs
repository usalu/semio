//! 📇️ Worked examples E-01 … E-18 of the EN 1992 target specification and the fixed defects as catalogue tests; expected values are hand-checked and independent of the implementation.

use semio_s_artifact_norm_en1992_checks::*;
use semio_s_plugin_norm_compliance::{ndp_is_verified, AnnexChoice, CheckStatus};

const DE: AnnexChoice = AnnexChoice::De;
const EN: AnnexChoice = AnnexChoice::En;

fn out(id: &str, case: &Case, key: &str) -> Outcome {
    evaluate_outcomes(id, case).unwrap_or_else(|e| panic!("{id}: {e}")).into_iter().find(|o| o.key == key).unwrap_or_else(|| panic!("{id}: no outcome {key}"))
}

fn v(id: &str, case: &Case, key: &str) -> f64 {
    out(id, case, key).value()
}

fn u(id: &str, case: &Case, key: &str) -> f64 {
    out(id, case, key).utilization()
}

fn near(a: f64, b: f64, rel: f64, what: &str) {
    assert!((a - b).abs() <= rel * b.abs().max(1e-12), "{what}: got {a}, expected {b} (rel {rel})");
}

fn rect_bars(case: Case, ys: &[f64], zs: &[f64], a: f64) -> Case {
    case.with_l("bar_y", ys).with_l("bar_z", zs).with_l("bar_a", &vec![a; ys.len()])
}

fn column_e11(annex: AnnexChoice) -> Case {
    rect_bars(Case::new("col", annex).with_n("fck", 30.0).with_n("b", 400.0).with_n("h", 400.0).with_n("fyk", 500.0), &[-150.0, 150.0, -150.0, 150.0], &[-150.0, -150.0, 150.0, 150.0], 490.87)
}

#[test]
fn e01_rectangular_beam_bending() {
    let c = Case::new("b", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("d", 450.0).with_n("m_ed", 250.0e6).with_n("as_provided", 1571.0);
    near(v("S-03", &c, "as1") * 1e6, 1487.0, 2e-3, "A_s1");
    near(v("S-03", &c, "xi"), 0.3522, 2e-3, "xi");
    near(u("S-03", &c, "util"), 0.947, 2e-3, "utilisation");
}

#[test]
fn e02_parabola_rectangle_and_block() {
    let c = Case::new("m", DE).with_n("fck", 30.0);
    near(v("M-05", &c.clone().with_n("eps", 1.0), "sigma_c") / 1e6, 12.75, 1e-9, "sigma(1.0)");
    near(v("M-05", &c.clone().with_n("eps", 2.0), "sigma_c") / 1e6, 17.0, 1e-9, "sigma(2.0)");
    assert_eq!(v("M-05", &c.clone().with_n("eps", -0.1), "sigma_c"), 0.0);
    near(v("M-05", &c.clone().with_n("eps", 3.5), "alpha_r"), 13.76 / 17.0, 2e-3, "alpha_R");
    near(v("M-05", &c.with_n("eps", 3.5), "delta_r"), 0.416, 5e-3, "delta_R");
}

#[test]
fn e03_shear_with_links_variable_strut_de_and_ec() {
    let c = Case::new("v", DE).with_n("fck", 30.0).with_n("bw", 300.0).with_n("z", 405.0).with_n("v_ed", 200.0e3);
    near(v("V-03", &c, "cot_theta"), 2.194, 2e-3, "cot θ");
    near(v("V-03", &c, "v_rd_max"), 584.6e3, 3e-3, "V_Rd,max");
    near(v("V-03", &c, "asw_req"), 0.5177, 3e-3, "A_sw/s");
    let ec = Case::new("v", EN).with_n("fck", 30.0).with_n("bw", 300.0).with_n("z", 405.0).with_n("v_ed", 200.0e3);
    near(v("V-03", &ec, "cot_theta"), 2.5, 1e-12, "EC cot θ");
    near(v("V-03", &ec, "asw_req"), 0.4543, 2e-3, "EC A_sw/s");
    let big = ec.clone().with_n("v_ed", 500.0e3);
    assert!(v("V-03", &big, "cot_theta") < 2.2 && v("V-03", &big, "cot_theta") > 2.0, "variable strut inclination under high shear");
}

#[test]
fn e04_shear_without_links() {
    let c = Case::new("v", DE).with_n("fck", 30.0).with_n("bw", 300.0).with_n("d", 450.0).with_n("asl", 1257.0);
    near(v("V-02", &c, "v_rd_c"), 68.27e3, 5e-3, "DE V_Rd,c");
    let ec = Case::new("v", EN).with_n("fck", 30.0).with_n("bw", 300.0).with_n("d", 450.0).with_n("asl", 1257.0);
    near(v("V-02", &ec, "v_rd_c"), 81.9e3, 5e-3, "EC V_Rd,c");
    assert!(u("V-02", &c.with_n("v_ed", 80.0e3), "util") > 1.1);
}

#[test]
fn e05_punching_interior() {
    let c = Case::new("p", DE).with_n("fck", 30.0).with_n("d", 210.0).with_n("c1", 400.0).with_n("v_ed", 700.0e3).with_n("rho_ly", 0.01).with_n("rho_lz", 0.01);
    near(v("P-03", &c, "v_ed") / 1e6, 0.8650, 2e-3, "v_Ed");
    near(u("P-03", &c, "util"), 1.174, 3e-3, "util");
    near(v("P-04", &c, "asw_req") * 1e6, 459.7, 5e-3, "A_sw");
    near(v("P-04", &c, "r_out") * 1e3, 537.0, 5e-3, "r_out");
    assert!(u("P-03", &c, "util_max") < 1.0);
}

#[test]
fn na_de_reduces_c_rd_c_of_interior_columns_with_small_u0_over_d_and_the_en_annex_does_not() {
    let (small, floor) = (0.8 / 0.255, 0.5);
    near(c_rd_c_numerator(DE, Position::Interior, small), 0.18 * (0.1 * small + 0.6), 1e-12, "DE reduced numerator");
    near(c_rd_c_numerator(DE, Position::Interior, floor), 0.15, 1e-12, "DE floor of the numerator");
    for u0_over_d in [small, floor, 4.0, 7.0] {
        near(c_rd_c_numerator(EN, Position::Interior, u0_over_d), 0.18, 1e-12, "EN numerator is never reduced");
    }
    near(c_rd_c_numerator(DE, Position::Interior, 4.0), 0.18, 1e-12, "DE at the limit");
    near(c_rd_c_numerator(DE, Position::Edge, small), 0.18, 1e-12, "DE edge column");
    let case = |annex| Case::new("p", annex).with_n("fck", 30.0).with_n("d", 255.0).with_n("c1", 200.0).with_n("v_ed", 500.0e3).with_n("rho_ly", 0.005).with_n("rho_lz", 0.005);
    let v_rd_c = |annex| v("P-03", &case(annex), "v_ed") / u("P-03", &case(annex), "util");
    let k = 1.0 + (200.0_f64 / 255.0).sqrt();
    let main = |numerator: f64| numerator / 1.5 * k * (100.0 * 0.005 * 30.0_f64).powf(1.0 / 3.0) * 1.0e6;
    near(v_rd_c(DE), main(0.18 * (0.1 * small + 0.6)), 1e-9, "DE v_Rd,c of a 200 mm column");
    near(v_rd_c(EN), main(0.18), 1e-9, "EN v_Rd,c of a 200 mm column");
}

#[test]
fn e06_torsion_and_interaction() {
    let c = Case::new("t", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("h", 500.0).with_n("t_ed", 20.0e6).with_n("cot_theta", 1.0);
    near(v("T-01", &c, "tef") * 1e3, 93.75, 1e-9, "t_ef");
    near(out("T-02", &c, "util").capacity * 1.0, 100.2e3, 2e-3, "T_Rd,max");
    near(v("T-02", &c, "asw_s"), 0.2745, 3e-3, "A_sw/s");
    near(v("T-02", &c, "asl") * 1e6, 336.3, 3e-3, "ΣA_sl");
    let i = c.with_n("bw", 300.0).with_n("d", 450.0).with_n("asl", 1257.0).with_n("v_ed", 150.0e3);
    near(u("T-04", &i, "util_max"), 0.393, 5e-3, "interaction");
}

#[test]
fn e07_anchorage_and_lap() {
    let c = Case::new("b", DE).with_n("fck", 30.0).with_n("phi", 20.0).with_n("cd", 45.0);
    near(v("B-02", &c, "lb_rqd"), 0.8525, 2e-3, "l_b,rqd");
    near(v("B-03", &c, "lbd"), 0.6927, 2e-3, "l_bd");
    near(v("B-04", &c.with_n("rho1", 50.0), "l0"), 0.9794, 3e-3, "l_0");
}

#[test]
fn e08_two_pile_cap_strut_and_tie() {
    let c = Case::new("st", DE)
        .with_l("node_x", &[-100.0, 100.0, -600.0, 600.0])
        .with_l("node_y", &[650.0, 650.0, 0.0, 0.0])
        .with_l("member_i", &[0.0, 1.0, 2.0, 0.0, 0.0])
        .with_l("member_j", &[2.0, 3.0, 3.0, 1.0, 3.0])
        .with_l("load_y", &[-1000.0e3, -1000.0e3, 0.0, 0.0])
        .with_l("support_node", &[2.0, 2.0, 3.0])
        .with_l("support_dof", &[0.0, 1.0, 1.0]);
    near(v("ST-04", &c, "force_2"), 769.2e3, 2e-3, "tie force");
    near(-v("ST-04", &c, "force_0"), 1261.6e3, 2e-3, "strut force");
    assert!(v("ST-04", &c, "force_4").abs() < 1.0, "diagonal carries nothing by symmetry");
}

#[test]
fn e09_crack_width_and_span_depth() {
    let c = Case::new("w", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("h", 500.0).with_n("d", 450.0).with_n("as", 1257.0).with_n("c", 40.0).with_n("phi", 20.0).with_n("m_qp", 150.0e6).with_n("alpha_e", 6.06);
    near(v("SL-06", &c, "x"), 0.1279, 3e-3, "x");
    near(v("SL-06", &c, "sr_max"), 0.2366, 3e-3, "s_r,max");
    near(out("SL-06", &c, "util").demand, 0.298e-3, 1e-2, "w_k");
    near(u("SL-06", &c, "util"), 0.99, 1.5e-2, "w_k utilisation");
    let ld = |as_: f64| Case::new("ld", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("d", 450.0).with_n("as", as_).with_n("span", 9000.0);
    near(v("SL-07", &ld(445.5), "ld_limit"), 34.03, 2e-3, "(l/d) rho<rho0");
    near(v("SL-07", &ld(1350.0), "ld_limit"), 15.5, 2e-3, "(l/d) rho>rho0");
}

#[test]
fn e10_creep_and_shrinkage() {
    let c = Case::new("cs", DE).with_n("fck", 30.0).with_n("rh", 50.0).with_n("h0", 187.5).with_n("t0", 28.0);
    near(v("A-01", &c, "phi_inf"), 2.389, 2e-3, "φ(∞,28)");
    near(v("A-02", &c, "eps_cs_inf"), 468.9e-6, 2e-3, "ε_cs,∞");
    near(v("M-11", &c, "eps_cs_inf"), 468.9e-6, 2e-3, "ε_cs = ε_cd + ε_ca of the materials view M-11");
}

#[test]
fn e11_column_interaction_and_slenderness() {
    let c = column_e11(DE);
    near(v("S-02", &c, "n_rd0"), 3574.0e3, 1e-3, "N_Rd,0");
    let s04 = c.clone().with_n("n_ed", 1500.0e3);
    near(v("S-04", &s04, "m_rd"), 235.6e6, 5e-3, "M_Rd");
    near(v("S-04", &s04, "x") * 1e3, 248.8, 5e-3, "x");
    let s05 = Case::new("b", DE).with_n("n_rd", 3574.0e3).with_n("m_rdy", 235.6e6).with_n("m_rdz", 235.6e6).with_n("n_ed", 1500.0e3).with_n("m_edy", 150.0e6).with_n("m_edz", 120.0e6);
    near(v("S-05", &s05, "exponent"), 1.266, 2e-3, "a");
    near(u("S-05", &s05, "util"), 0.990, 3e-3, "Eq. 5.39");
    let sl = Case::new("sl", EN).with_n("fck", 30.0).with_n("b", 400.0).with_n("h", 400.0).with_n("n_ed", 1500.0e3).with_n("l0", 4000.0).with_n("as", 1963.5);
    near(v("A-09", &sl, "lambda"), 34.64, 1e-3, "λ");
    near(v("A-09", &sl, "lambda_lim"), 16.84, 3e-3, "λ_lim");
    let nc = Case::new("nc", DE).with_n("fck", 30.0).with_n("b", 400.0).with_n("h", 400.0).with_n("n_ed", 1500.0e3).with_n("l0", 4000.0).with_n("as", 1963.5).with_n("is", 150.0).with_n("e0", 40.0);
    near(v("A-10", &nc, "e2"), 18.66e-3, 5e-3, "e_2");
    near(v("A-10", &nc, "m_ed"), 103.0e3, 5e-3, "M_Ed");
}

#[test]
fn e12_fire_tables_and_interpolation() {
    let c = Case::new("f", DE).with_s("kind", "beam").with_n("r_class", 90.0).with_n("dim", 300.0).with_n("a", 38.0);
    near(v("R-01", &c, "r_achieved"), 60.0, 1e-12, "R achieved");
    assert!(u("R-01", &c, "axis") > 1.0, "a = 38 < 40 fails R90");
    let slab = Case::new("f", DE).with_s("kind", "slab").with_n("r_class", 105.0).with_n("dim", 150.0).with_n("a", 40.0);
    near(out("R-01", &slab, "axis").capacity, 35.0e-3, 1e-9, "interpolated a for R105");
}

#[test]
fn e13_prestress_losses() {
    let c = Case::new("p", DE).with_n("pmax", 1.0e6).with_n("mu", 0.19).with_n("k", 0.005).with_l("x", &[20.0]).with_l("theta", &[0.15]);
    near(v("PR-04", &c, "p_0"), 0.95361e6, 5e-4, "P(x)");
    near(semio_s_artifact_norm_en1992_checks::relaxation_ratio(2, 0.70, 500000.0, 2.5), 0.0390, 5e-3, "Δσ_pr/σ_pi");
}

#[test]
fn e14_fatigue_of_steel() {
    let c = Case::new("f", DE).with_s("kind", "straight").with_n("delta_sigma_equ", 120.0);
    near(u("F-04", &c, "util_equ"), 0.849, 2e-3, "equivalent range");
    let d = Case::new("f", DE).with_s("kind", "straight").with_l("ranges", &[200.0]).with_l("cycles", &[1.0e5]);
    near(v("F-04", &d, "damage"), 0.568, 3e-3, "damage");
}

#[test]
fn e15_anchor_cone() {
    let c = Case::new("a", DE).with_n("fck", 30.0).with_n("hef", 100.0).with_n("n_ed", 25.0e3).with_s("type", "cast_in");
    near(v("FA-03", &c, "n_rk_c"), 48.75e3, 1e-3, "N_Rk,c");
    near(u("FA-03", &c, "util"), 0.77, 1e-2, "util");
    let e = c.with_n("x_min", -100.0);
    near(v("FA-03", &e, "n_rk_c"), 36.56e3, 2e-3, "edge N_Rk,c");
    near(u("FA-03", &e, "util"), 1.026, 5e-3, "edge util");
}

#[test]
fn e16_partially_loaded_area() {
    let c = Case::new("pa", DE).with_n("fck", 30.0).with_n("ac0", 40000.0).with_n("ac1", 360000.0).with_n("f_ed", 1800.0e3);
    near(v("PA-01", &c, "f_rdu"), 2040.0e3, 1e-9, "F_Rdu");
    near(v("PA-01", &c.clone().with_n("ac1", 640000.0), "f_rdu"), 2040.0e3, 1e-9, "capped F_Rdu");
    near(u("PA-01", &c, "util"), 0.882, 2e-3, "util");
}

#[test]
fn e17_interface_shear() {
    let c = Case::new("i", DE).with_n("fck", 30.0).with_n("bi", 300.0).with_n("z", 405.0).with_n("v_ed", 100.0e3).with_n("rho", 0.002);
    near(v("V-05", &c, "v_rdi") / 1e6, 1.062, 3e-3, "v_Rdi");
    near(u("V-05", &c, "util"), 0.775, 5e-3, "util");
}

#[test]
fn e18_support_reduction() {
    let near_support = Case::new("s", DE).with_n("av", 300.0).with_n("d", 250.0).with_n("v_ed", 100.0e3);
    near(v("V-01", &near_support, "beta"), 0.6, 1e-12, "β");
    assert_eq!(v("V-01", &near_support.with_n("av", 600.0), "beta"), 1.0);
}

#[test]
fn defects_are_fixed() {
    let (e2, ..) = nominal_curvature(30.0, 17.0, 434.78, 200000.0, 400.0, 400.0, 1963.5, 150.0, 1500.0e3, 4000.0, 0.0, std::f64::consts::PI.powi(2));
    near(e2, 18.66, 5e-3, "nominal curvature e_2 (was ~0.45² too small)");
    let ld = ld_limit(EN, 30.0, 0.0033, 0.0, 1.0);
    near(ld, 34.03, 2e-3, "l/d uses √f_ck");
    assert!((semio_s_artifact_norm_en1992_model::ReinforcementGrade::b500c("c").f_yk - 500.0e6).abs() < 1.0, "B500C f_yk");
    near(part_4::cone_resistance_n(AnnexChoice::De, 30.0e6, 0.1, true, 10.0), 32.5e3, 1e-3, "EN 1992-4 cone k1 = 8.9");
    near(part_4::steel_resistance_n(100.0e-6, 800.0e6, 640.0e6), 100.0e-6 * 800.0e6 / 1.5, 1e-12, "γ_Ms");
    let strands = beam_fire_dim_check();
    assert!(strands);
}

fn beam_fire_dim_check() -> bool {
    let slab = Case::new("f", DE).with_s("kind", "slab").with_n("r_class", 90.0).with_n("dim", 100.0).with_n("a", 30.0);
    u("R-01", &slab, "dim") <= 1.0 && u("R-01", &slab, "axis") <= 1.0
}

#[test]
fn unverified_ndps_never_pass_silently() {
    assert!(["ndp.en1992.punch-vrdmax-factor", "ndp.en1992.m-01.alpha-cc-accidental", "ndp.en1992.cmin-dur-table"].iter().all(|id| !ndp_is_verified(id)));
    let c = Case::new("p", DE).with_n("fck", 30.0).with_n("d", 210.0).with_n("c1", 400.0).with_n("v_ed", 300.0e3).with_n("rho_ly", 0.01).with_n("rho_lz", 0.01);
    let results = evaluate_case("P-03", &c);
    assert!(results.iter().all(|r| r.status == CheckStatus::UnverifiedBasis || r.status == CheckStatus::Fail), "unverified NDP basis must never be a compliant verdict");
    assert!(results.iter().any(|r| r.explanation.en.contains("ndp.en1992.p-03")), "registry ids are cited");
    let table = Case::new("m", DE).with_n("fck", 30.0);
    assert!(evaluate_case("M-02", &table).iter().all(|r| r.status == CheckStatus::UnverifiedBasis || r.status == CheckStatus::Fail), "every value read from the registry is unverified until double-keyed");
}

#[test]
fn every_entry_is_registered_with_bilingual_titles() {
    let reg = registry();
    for id in ["M-01", "S-06", "V-08", "T-04", "P-07", "ST-04", "PA-01", "B-09", "DT-14", "SL-09", "F-07", "PR-10", "R-08", "BR-08", "LQ-07", "FA-14", "A-15"] {
        assert!(reg.iter().any(|e| e.id == id), "{id} missing");
    }
    assert!(reg.iter().all(|e| !e.en.is_empty() && !e.de.is_empty() && !e.inputs.is_empty()));
    let mut ids: Vec<&str> = reg.iter().map(|e| e.id).collect();
    ids.sort_unstable();
    let n = ids.len();
    ids.dedup();
    assert_eq!(n, ids.len(), "catalogue ids are unique");
}

fn vectors_path() -> std::path::PathBuf {
    semio_framework_test_fixtures::fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🧫️vectors/catalogue.json")
}

fn oracle_path() -> std::path::PathBuf {
    semio_framework_test_fixtures::fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/⚖️compliance/🐍️.py")
}

#[test]
fn independent_python_oracle_matches_the_rust_catalogue() {
    let text = std::fs::read_to_string(vectors_path()).expect("vectors");
    let doc: serde_json::Value = serde_json::from_str(&text).expect("vector json");
    let cases = doc["cases"].as_array().expect("cases");
    let out = semio_s_plugin_norm_compliance::python_oracle::run_ok(oracle_path(), &[], Some(text.as_bytes()));
    let py: serde_json::Value = serde_json::from_slice(&out.stdout).expect("oracle json");
    let results = py["results"].as_array().expect("results");
    assert_eq!(results.len(), cases.len());
    let mut compared = 0;
    for (cv, rv) in cases.iter().zip(results) {
        let id = cv["id"].as_str().unwrap();
        let annex = if cv["annex"] == "En" { EN } else { DE };
        let mut case = Case::new(id, annex);
        for (k, v) in cv["num"].as_object().unwrap() { case = case.with_n(k, v.as_f64().unwrap()); }
        for (k, v) in cv["text"].as_object().unwrap() { case = case.with_s(k, v.as_str().unwrap()); }
        for (k, v) in cv["list"].as_object().unwrap() { case = case.with_l(k, &v.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect::<Vec<_>>()); }
        let tol = cv["tol"].as_f64().unwrap();
        for (key, pv) in rv["values"].as_object().unwrap() {
            let o = out_of(id, &case, key);
            let rust = if key.starts_with("util") { o.utilization() } else { o.value() };
            let py = pv.as_f64().unwrap();
            assert!((rust - py).abs() <= tol * py.abs().max(1e-9) + 1e-12, "{id}.{key}: rust {rust} python {py} tol {tol}");
            compared += 1;
        }
    }
    assert!(compared >= 40, "compared only {compared} values");
}

fn out_of(id: &str, case: &Case, key: &str) -> Outcome {
    out(id, case, key)
}

#[test]
fn gherkin_examples_are_executable() {
    let path = semio_framework_test_fixtures::fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️en1992-catalogue/🥒️.feature");
    let text = std::fs::read_to_string(path).expect("feature");
    let mut rows = 0;
    let mut header: Vec<String> = Vec::new();
    for line in text.lines().map(str::trim).filter(|l| l.starts_with('|')) {
        let cells: Vec<String> = line.trim_matches('|').split('|').map(|c| c.trim().to_string()).collect();
        if header.is_empty() { header = cells; continue; }
        let col = |name: &str| cells[header.iter().position(|h| h == name).unwrap()].clone();
        let annex = if col("annex") == "En" { EN } else { DE };
        let mut case = Case::new(&col("entry"), annex);
        for kv in col("inputs").split(';') {
            if let Some((k, v)) = kv.split_once('=') {
                case = case.with_n(k, v.parse().unwrap());
            } else {
                let (k, v) = kv.split_once(':').unwrap();
                case = case.with_s(k, v);
            }
        }
        let key = col("key");
        let o = out(&col("entry"), &case, &key);
        let got = if key.starts_with("util") { o.utilization() } else { o.value() };
        near(got, col("expected").parse().unwrap(), col("tolerance").parse().unwrap(), &format!("{} {}", col("entry"), key));
        rows += 1;
    }
    assert!(rows >= 10, "feature rows {rows}");
}

#[test]
fn review_s1_regressions() {
    let high = Case::new("h", DE).with_n("fck", 60.0).with_n("b", 300.0).with_n("d", 450.0).with_n("m_ed", 250.0e6);
    near(v("S-03", &high, "as1") * 1e6, 1371.8, 3e-3, "S-03 uses λ and η above C50/60");
    let (vrd, cot) = part_1_1::shear_capacity_n(0.30, 0.405, 30.0e6, 500.0e6, 0.00067, 0.0, 200.0e3, DE);
    near(cot, 2.194, 2e-3, "member cot θ DE");
    near(vrd, 0.67 * 405.0 * (500.0 / 1.15) * 2.194, 3e-3, "links are not ignored (was 1e6 too small)");
    let (vrd_en, cot_en) = part_1_1::shear_capacity_n(0.30, 0.405, 30.0e6, 500.0e6, 0.00067, 0.0, 200.0e3, EN);
    near(cot_en, 2.5, 1e-9, "member cot θ EC");
    near(vrd_en, 295.0e3, 5e-3, "EC V_Rd,s at cot θ 2.5");
    let ec = Case::new("p", EN).with_n("fck", 30.0).with_n("d", 210.0).with_n("c1", 400.0).with_n("v_ed", 700.0e3).with_n("rho_ly", 0.01).with_n("rho_lz", 0.01).with_n("asw", 5000.0);
    let o = out("P-04", &ec, "util_max");
    near(o.capacity / 1e6, 0.5 * 0.528 * 20.0, 1e-9, "EC crushing 0.5 ν f_cd");
    near(o.demand / 1e6, 1.15 * 700.0e3 / (1600.0 * 210.0), 1e-9, "EC crushing at u0");
}

#[test]
fn combinations_follow_en1990() {
    use semio_s_artifact_norm_en1992_model::{En1992Snapshot, LoadCaseActions, SupportCondition};
    let factors = En1992Snapshot::compliant_office_frame().action_factors();
    let mut m = En1992Snapshot::compliant_office_frame().members[0].clone();
    m.udl = 0.0;
    m.use_fem = false;
    m.span = 6.0;
    m.support = SupportCondition::SimplySupported;
    let act = |id: &str, kind: semio_s_artifact_norm_en1992_model::ActionKind, cat: &str, g: f64, q: f64, mk: f64| LoadCaseActions { id: id.into(), kind, category: cat.into(), source: if kind == semio_s_artifact_norm_en1992_model::ActionKind::Prestress { semio_s_artifact_norm_en1992_model::ActionSource::External } else { semio_s_artifact_norm_en1992_model::ActionSource::Udl }, g_k_line: g, q_k_line: q, point_force: 0.0, m_k: mk, n_k: 0.0, v_k: 0.0, t_k: 0.0, v_k_punch: 0.0 };
    m.actions = vec![act("G", semio_s_artifact_norm_en1992_model::ActionKind::Permanent, "self", 10.0e3, 0.0, 0.0), act("P", semio_s_artifact_norm_en1992_model::ActionKind::Prestress, "self", 0.0, 0.0, -20.0e3)];
    let effects = combine_member_actions(&m, &factors);
    let find = |id: &str| effects.iter().find(|e| e.combination_id == id).unwrap_or_else(|| panic!("{id}")).m_ed;
    near(find("ULS-6.10-G"), 1.35 * 45.0e3 - 20.0e3, 1e-9, "γ_G G + γ_P P with γ_P = 1");
    near(find("ULS-6.10-G-fav"), 45.0e3 - 20.0e3, 1e-9, "favourable γ_G,inf = 1");
    m.udl = 5.0e3;
    m.actions = vec![act("G1", semio_s_artifact_norm_en1992_model::ActionKind::Permanent, "self", 10.0e3, 0.0, 0.0), act("G2", semio_s_artifact_norm_en1992_model::ActionKind::Permanent, "self", 4.0e3, 0.0, 0.0)];
    near(combine_member_actions(&m, &factors).iter().find(|e| e.combination_id == "SLS-char").unwrap().m_ed, 19.0e3 * 36.0 / 8.0, 1e-9, "member udl counted once");
    assert_eq!(psi_factors(&factors, semio_s_artifact_norm_en1992_model::PsiCategory::Storage), (1.0, 0.9, 0.8));
    assert_eq!(psi_factors(&factors, semio_s_artifact_norm_en1992_model::PsiCategory::SnowHigh), (0.7, 0.5, 0.2));
}

fn third_party_cases() -> serde_json::Value {
    let path = semio_framework_test_fixtures::fixture_path!("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🔮️oracle-cases/third-party.json");
    serde_json::from_str(&std::fs::read_to_string(path).expect("third-party cases")).expect("json")
}

#[test]
fn blessed_third_party_cases_are_reproduced_by_the_evaluator() {
    let doc = third_party_cases();
    let mut adapted = 0;
    let mut skipped: Vec<String> = Vec::new();
    for case in doc["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let inp = |k: &str| case["inputs"][k].as_f64();
        let exp = |k: &str| case["expected"][k]["value"].as_f64().unwrap_or_else(|| panic!("{id}: expected {k}"));
        let annex = if case["citation"]["annex"].as_str() == Some("en") || inp("C_Rdc") == Some(0.12) { EN } else { DE };
        let mpa = |k: &str| inp(k).map(|v| v / 1e6);
        if id.contains(".laws-") {
            let c = Case::new("t", DE).with_n("fck", mpa("fck").unwrap());
            for (key, ours, tol) in [("fcm", "fcm", 1e-9), ("fctm", "fctm", 2e-2), ("fctk_5", "fctk05", 2e-2), ("fctk_95", "fctk95", 2e-2), ("Ecm", "ecm", 1e-2)] {
                near(v("M-02", &c, ours), exp(key), tol, &format!("{id} {key}"));
            }
            for (key, ours, scale) in [("eps_c1", "eps_c1", 1e-3), ("eps_c2", "eps_c2", 1e-3), ("eps_cu2", "eps_cu2", 1e-3), ("n", "n", 1.0)] {
                near(v("M-02", &c, ours) * scale, exp(key), 5e-3, &format!("{id} {key}"));
            }
            near(v("M-01", &c, "fcd") / 1e6, exp("fcd") / 1e6, 1e-9, &format!("{id} fcd"));
            adapted += 1;
        } else if id.contains(".fcd-") {
            let c = Case::new("t", DE).with_n("fck", mpa("f_ck").unwrap());
            near(v("M-01", &c, "fcd"), exp("f_cd"), 1e-9, id);
            adapted += 1;
        } else if id.contains(".flexure-") || id.contains(".nm-interaction-") {
            let (b, h, d, a) = (inp("b").unwrap() * 1e3, inp("h").unwrap() * 1e3, inp("d").unwrap() * 1e3, inp("As").unwrap() * 1e6);
            let k = inp("k_hardening").unwrap_or(1.0);
            let base = Case::new("t", DE).with_n("fck", mpa("fck").unwrap()).with_n("b", b).with_n("h", h).with_n("fyk", mpa("fyk").unwrap()).with_n("k", k).with_n("eps_uk", inp("eps_uk").map(|e| e * 1e3).unwrap_or(50.0)).with_l("bar_y", &[0.0]).with_l("bar_z", &[h / 2.0 - d]).with_l("bar_a", &[a]);
            if let Some(list) = case["inputs"]["N_Ed"].as_array() {
                for (i, n) in list.iter().enumerate() {
                    near(v("S-04", &base.clone().with_n("n_ed", n.as_f64().unwrap()), "m_rd"), exp(&format!("M_Rd_{i}")), 5e-3, &format!("{id} M_Rd_{i}"));
                }
            } else {
                let c = base.with_n("n_ed", 0.0);
                near(v("S-04", &c, "m_rd"), exp("M_Rd"), 5e-3, &format!("{id} M_Rd"));
                if case["expected"]["x"].is_object() { near(v("S-04", &c, "x"), exp("x"), 5e-3, &format!("{id} x")); }
            }
            adapted += 1;
        } else if id.contains(".cracked-") {
            let c = Case::new("t", DE).with_n("fck", mpa("fck").unwrap()).with_n("b", inp("b").unwrap() * 1e3).with_n("h", inp("h").unwrap() * 1e3).with_n("d", inp("d").unwrap() * 1e3).with_n("as", inp("As").unwrap() * 1e6).with_n("m_ed", inp("M_Ed").unwrap() * 1e3);
            near(v("SL-03", &c, "x"), exp("x_cr"), 1e-2, &format!("{id} x"));
            near(v("SL-03", &c, "i_ii") * 1e-12, exp("I_cr"), 1.5e-2, &format!("{id} I_cr"));
            near(v("SL-03", &c, "sigma_c"), exp("sigma_c"), 1e-2, &format!("{id} sigma_c"));
            near(v("SL-03", &c, "sigma_s"), exp("sigma_s"), 1e-2, &format!("{id} sigma_s"));
            adapted += 1;
        } else if id.contains(".vrdc-") {
            let c = Case::new("t", annex).with_n("fck", mpa("f_ck").unwrap()).with_n("bw", inp("b_w").unwrap() * 1e3).with_n("d", inp("d").unwrap() * 1e3).with_n("asl", inp("A_sl").unwrap() * 1e6).with_n("sigma_cp", mpa("sigma_cp").unwrap());
            near(v("V-02", &c, "v_rd_c_uncapped"), exp("V_Rd_c"), 1e-4, id);
            adapted += 1;
        } else if id.contains(".links-") {
            let c = Case::new("t", annex).with_n("fck", mpa("f_ck").unwrap()).with_n("bw", inp("b").unwrap() * 1e3).with_n("z", inp("z").unwrap() * 1e3).with_n("v_ed", 100.0e3).with_n("cot_theta", inp("cot_theta").unwrap()).with_n("asw_s", inp("Asw").unwrap() * 1e6 / (inp("s").unwrap() * 1e3));
            near(v("V-03", &c, "v_rd_max"), exp("V_Rd_max"), 1e-4, &format!("{id} V_Rd,max"));
            near(out("V-03", &c, "util_s").capacity, exp("V_Rd_s"), 1e-4, &format!("{id} V_Rd,s"));
            adapted += 1;
        } else if id.contains(".span-depth") {
            let k = inp("capital_k").unwrap();
            near(ld_limit(EN, mpa("f_ck").unwrap(), inp("rho").unwrap(), inp("rho_prime").unwrap(), k), exp("l_over_d"), 2e-3, id);
            adapted += 1;
        } else if id.contains(".anchorage-rqd") {
            near(detailing::l_b_rqd(inp("diameter").unwrap() * 1e3, mpa("sigma_sd").unwrap(), mpa("f_bd").unwrap()) / 1e3, exp("l_b_rqd"), 1e-9, id);
            adapted += 1;
        } else if id.contains(".lambda-lim") {
            let omega = (inp("b").unwrap().powi(2) - 1.0) / 2.0;
            let n = inp("n_ed").unwrap() / (inp("a_c").unwrap() * inp("f_cd").unwrap());
            near(part_1_1::lambda_limit(n, omega, EN), exp("lambda_lim"), 1e-6, id);
            adapted += 1;
        } else if id.contains(".6n.nu") {
            near(shear_torsion::nu1(EN, mpa("f_ck").unwrap()), exp("nu"), 1e-9, id);
            adapted += 1;
        } else {
            skipped.push(id.to_string());
        }
    }
    assert!(adapted >= 25, "adapted {adapted}, skipped {skipped:?}");
}

#[test]
fn annex_f_in_plane_stress_matches_equilibrium() {
    let c = Case::new("f", DE).with_n("sx", 2.0).with_n("sy", 1.0).with_n("txy", 3.0).with_n("t", 200.0).with_n("fck", 30.0);
    near(v("AF-01", &c, "as_x"), 2.3, 1e-9, "A_s,x");
    near(v("AF-01", &c, "as_y"), 1.84, 1e-9, "A_s,y");
    near(v("AF-01", &c, "sigma_c"), 6.0e6, 1e-9, "sigma_c");
    let k = Case::new("f", DE).with_n("sx", -4.0).with_n("sy", 1.0).with_n("txy", 2.0).with_n("t", 200.0).with_n("fck", 30.0);
    assert_eq!(v("AF-01", &k, "as_x"), 0.0);
    near(v("AF-01", &k, "as_y"), 0.92, 1e-9, "clipped A_s,y");
    near(v("AF-01", &k, "sigma_c"), 5.0e6, 1e-9, "clipped sigma_c");
}

#[test]
fn annex_j_corbel_tie_force_and_links() {
    let c = Case::new("j", DE).with_n("f_ed", 300.0e3).with_n("a_c", 150.0).with_n("z0", 300.0).with_n("fck", 30.0);
    near(v("AJ-03", &c, "f_td"), 150.0e3, 1e-9, "F_td");
    near(v("AJ-03", &c, "as_main") * 1e6, 345.0, 1e-9, "A_s,main");
    near(v("AJ-03", &c, "a_links") * 1e6, 86.25, 1e-9, "A_sk");
    near(v("AJ-03", &c, "c_strut"), 335_410.196_624_968_5, 1e-9, "C");
    let s = Case::new("j", DE).with_n("h_ct", 100.0).with_n("as_surf", 1500.0).with_n("s_surf", 200.0);
    near(v("AJ-01", &s, "as_surf_min") * 1e6, 1000.0, 1e-9, "A_s,surf,min");
    assert!(u("AJ-01", &s, "util_s") > 1.0);
}

#[test]
fn warping_torsion_flange_design() {
    let c = Case::new("w", DE).with_n("bimoment", 4.0e9).with_n("h_s", 400.0).with_n("t_f", 150.0).with_n("d_f", 250.0).with_n("fck", 30.0);
    near(v("T-05", &c, "m_f"), 1.0e7 / 1e3, 1e-9, "M_f");
    near(v("T-05", &c, "as_req") * 1e6, 95.083, 1e-4, "A_s");
}

#[test]
fn annex_kk_and_ll() {
    let k = Case::new("k", DE).with_n("fck", 30.0).with_n("phi", 2.5).with_n("m1", 100.0).with_n("m2", 200.0);
    near(v("AK-01", &k, "e_adj") / 1e6, 11000.0, 1e-9, "E_c,adj");
    near(v("AK-01", &k, "xi"), 0.917_915, 1e-6, "xi");
    let l = Case::new("l", DE).with_n("nx", 100.0).with_n("mx", 20000.0).with_n("h", 300.0).with_n("a", 40.0).with_n("fck", 30.0);
    near(v("AL-01", &l, "as_x_bot"), 0.3241, 1e-3, "A_s,x bottom");
    assert_eq!(v("AL-01", &l, "as_x_top"), 0.0);
}

#[test]
fn moment_curvature_rises_monotonically_towards_the_resistance_of_a_non_rectangular_section() {
    let tee_y = [-150.0, 150.0, 150.0, 500.0, 500.0, -500.0, -500.0, -150.0];
    let tee_z = [-383.0, -383.0, 67.0, 67.0, 217.0, 217.0, 67.0, 67.0];
    let base = |id: &str| Case::new(id, EN).with_n("fck", 30.0).with_n("fyk", 500.0).with_n("eps_uk", 75.0).with_n("n_ed", 0.0).with_n("points", 24.0).with_l("poly_y", &tee_y).with_l("poly_z", &tee_z).with_l("bar_y", &[-100.0, 0.0, 100.0]).with_l("bar_z", &[-333.0, -333.0, -333.0]).with_l("bar_a", &[490.87, 490.87, 490.87]);
    let m_rd = v("S-04", &base("S-04"), "m_rd");
    let curve: Vec<f64> = evaluate_outcomes("S-11", &base("S-11")).expect("S-11").into_iter().filter(|o| o.key.starts_with("m_") && o.key != "m_cr").map(|o| o.value()).collect();
    assert!(curve.len() >= 8, "{} points", curve.len());
    assert!(curve.windows(2).all(|w| w[1] >= w[0] * 0.999), "monotone: {curve:?}");
    let last = *curve.last().unwrap();
    assert!(last <= m_rd * 1.02 && last >= 0.85 * m_rd, "{last} vs {m_rd}");
    assert!(v("S-11", &base("S-11"), "m_cr") > 0.0);
}

fn tee_case(id: &str) -> Case {
    let a = std::f64::consts::PI * 25.0 * 25.0 / 4.0;
    Case::new(id, DE)
        .with_n("fck", 30.0)
        .with_n("c", 30.0)
        .with_n("phi_l", 25.0)
        .with_l("poly_y", &[-150.0, 150.0, 150.0, 500.0, 500.0, -500.0, -500.0, -150.0])
        .with_l("poly_z", &[0.0, 0.0, 450.0, 450.0, 600.0, 600.0, 450.0, 450.0])
        .with_l("bar_y", &[-100.0, 0.0, 100.0])
        .with_l("bar_z", &[50.0, 50.0, 50.0])
        .with_l("bar_a", &[a, a, a])
}

#[test]
fn general_section_properties_of_a_tee_match_the_polygon_hand_values() {
    let c = tee_case("t");
    near(v("SG-01", &c, "area") * 1e6, 285_000.0, 1e-9, "A_c");
    near(v("SG-01", &c, "z_bot") * 1e3, 382.894_736_842, 1e-9, "centroid above the bottom");
    near(v("SG-01", &c, "i_y"), 8.954_111_842e9, 1e-8, "I_y");
    near(v("SG-01", &c, "b_w") * 1e3, 300.0, 1e-9, "b_w is the web, not the flange");
    near(v("SG-01", &c, "d") * 1e3, 550.0, 1e-9, "d");
    near(v("SG-01", &c, "t_ef") * 1e3, 89.0625, 1e-9, "t_ef = A/u");
    near(v("SG-01", &c, "a_k") * 1e6, 150_432.128_906, 1e-9, "A_k with the re-entrant corners");
    near(v("SG-01", &c, "u_k") * 1e3, 2_843.75, 1e-9, "u_k");
}

#[test]
fn a_hollow_section_limits_the_torsion_wall_to_its_real_wall() {
    let (outer_y, outer_z) = ([-200.0, 200.0, 200.0, -200.0], [-300.0, -300.0, 300.0, 300.0]);
    let (hole_y, hole_z) = ([-150.0, 150.0, 150.0, -150.0], [-250.0, -250.0, 250.0, 250.0]);
    let c = Case::new("box", DE).with_n("fck", 30.0).with_n("c", 15.0).with_n("phi_l", 10.0).with_l("poly_y", &outer_y).with_l("poly_z", &outer_z).with_l("hole_y", &hole_y).with_l("hole_z", &hole_z);
    near(v("SG-01", &c, "t_ef") * 1e3, 50.0, 1e-9, "t_ef = real wall");
    near(v("SG-01", &c, "a_k") * 1e6, 350.0 * 550.0, 1e-9, "A_k includes the hollow");
    near(v("T-01", &c, "tef") * 1e3, 50.0, 1e-9, "T-01 reads the same wall");
}

#[test]
fn the_elastic_state_of_a_tee_matches_the_concreteproperties_cracked_analysis() {
    let c = tee_case("t").with_n("m_ed", 150.0e6);
    near(v("SG-02", &c, "x") * 1e3, 90.5595, 2e-4, "x");
    near(v("SG-02", &c, "sigma_c") / 1e6, 6.3719, 2e-3, "σ_c");
    near(v("SG-02", &c, "sigma_s") / 1e6, 195.92, 2e-3, "σ_s");
    near(v("SG-02", &c, "i_ii"), 2.1318e9, 2e-3, "I_II");
    let state_one = tee_case("t").with_n("m_ed", 20.0e6).with_s("state", "uncracked");
    assert!(v("SG-02", &state_one, "sigma_c") > 0.0 && v("SG-02", &state_one, "x") > v("SG-02", &c, "x"), "state I keeps a deeper compressed zone");
}

#[test]
fn the_elastic_state_with_normal_force_matches_the_rectangular_closed_form() {
    let (b, h, d, d2, as1, as2, ae, m, n) = (300.0, 500.0, 450.0, 50.0, 1257.0, 402.0, 15.0, 120.0e6, 250.0e3);
    let n1 = |x: f64| b * x * x / 2.0 + ae * as2 * (x - d2) - ae * as1 * (d - x);
    let m1 = |x: f64| b * x * x / 2.0 * (h / 2.0 - x / 3.0) + ae * as2 * (x - d2) * (h / 2.0 - d2) + ae * as1 * (d - x) * (d - h / 2.0);
    let g = |x: f64| n1(x) * m - m1(x) * n;
    let (mut lo, mut hi) = (1e-6, h);
    let up = g(hi) > g(lo);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if (g(mid) > 0.0) == up { hi = mid } else { lo = mid }
    }
    let x = 0.5 * (lo + hi);
    let k = n / n1(x);
    let (gx, i2, sc, ss, ss2) = serviceability::cracked_state(b, h, d, as1, d2, as2, ae, m, n).expect("cracked");
    near(gx, x, 1e-9, "x");
    near(sc, k * x, 1e-9, "σ_c");
    near(ss, ae * k * (d - x), 1e-9, "σ_s");
    near(ss2, ae * k * (x - d2), 1e-9, "σ_s2");
    near(i2, b * x.powi(3) / 3.0 + ae * as1 * (d - x).powi(2) + ae * as2 * (x - d2).powi(2), 1e-6, "I_II");
}

#[test]
fn the_zone_method_follows_the_spec_formula_and_offsets_only_exposed_edges() {
    let c = Case::new("z", DE).with_n("w", 100.0).with_n("theta_m", 300.0).with_l("zone_theta", &[760.0, 640.0, 520.0, 420.0, 340.0]);
    near(v("R-04", &c, "kc_m"), 0.5184, 1e-9, "k_c,m");
    near(v("R-04", &c, "az") * 1e3, 39.011_764_7, 1e-7, "a_z");
    let ring = fire_field::rect_outline(0.4, 0.6);
    let moved = fire_field::offset_polygon(&ring, &[0.02, 0.0, 0.03, 0.0]);
    let area = |p: &[(f64, f64)]| (0..p.len()).map(|i| p[i].0 * p[(i + 1) % p.len()].1 - p[(i + 1) % p.len()].0 * p[i].1).sum::<f64>() / 2.0;
    near(area(&moved), 0.4 * 0.55, 1e-12, "bottom and top moved in, sides kept");
    let field = fire_field::solve_outline(&ring, (false, false, true, false), 60.0, fire_field::MOISTURE_DEFAULT);
    let zone = fire_field::zone_section(&ring, (false, false, true, false), &field, 5);
    assert!(zone.damage[0] > 0.005 && zone.damage[1] == 0.0 && zone.damage[2] == 0.0 && zone.damage[3] == 0.0, "only the heated bottom is damaged: {:?}", zone.damage);
    assert!(area(&zone.contour) < area(&ring) && area(&zone.contour) > 0.0);
}

#[test]
fn the_fire_checks_use_the_reduced_sections_and_the_resistance_falls_with_the_duration() {
    use semio_s_artifact_norm_en1992_model::{En1992Snapshot, FireRating};
    let mut doc = En1992Snapshot::compliant_office_frame();
    let utilisation = |doc: &En1992Snapshot, key: &str| {
        let beam = doc.members.iter().find(|m| m.id == "beam-B1").expect("beam");
        let id = format!("en1992.1-2.{key}.beam-B1");
        evaluate_member(doc, beam).into_iter().find(|r| r.id == id).unwrap_or_else(|| panic!("{id}")).utilization
    };
    let (nm60, zone60) = (utilisation(&doc, "nm"), utilisation(&doc, "zone"));
    doc.members.iter_mut().find(|m| m.id == "beam-B1").expect("beam").fire.as_mut().expect("fire").rating = FireRating::R120;
    let (nm120, zone120) = (utilisation(&doc, "nm"), utilisation(&doc, "zone"));
    assert!(nm60.is_finite() && zone60.is_finite() && nm120 > nm60 && zone120 > zone60, "R60 {nm60} {zone60}, R120 {nm120} {zone120}");
}

#[test]
fn member_geometry_reads_the_outline_not_a_rectangle() {
    use semio_s_artifact_norm_en1992_model::{En1992Snapshot, SectionPoint};
    let mut doc = En1992Snapshot::compliant_office_frame();
    let beam = doc.members.iter().find(|m| m.id == "beam-B1").expect("beam").clone();
    let rect = Geo::of(&member_section_in(&doc, &beam, 30.0e6, 500.0e6, 200.0e9, DE, false), &beam);
    near(rect.area, 0.15, 1e-12, "A_c");
    near(rect.i_y, 0.3 * 0.5_f64.powi(3) / 12.0, 1e-9, "I_y");
    near(rect.b_w, 0.3, 1e-9, "b_w");
    near(rect.d, 0.45, 1e-9, "d");
    let point = |id: &str, y: f64, z: f64| SectionPoint { id: id.into(), y, z };
    let tee = vec![point("a", -0.15, -0.383), point("b", 0.15, -0.383), point("c", 0.15, 0.067), point("d", 0.5, 0.067), point("e", 0.5, 0.217), point("f", -0.5, 0.217), point("g", -0.5, 0.067), point("h", -0.15, 0.067)];
    doc.members.iter_mut().find(|m| m.id == "beam-B1").expect("beam").section_points = tee;
    let beam = doc.members.iter().find(|m| m.id == "beam-B1").expect("beam");
    let t = Geo::of(&member_section_in(&doc, beam, 30.0e6, 500.0e6, 200.0e9, DE, false), beam);
    near(t.b_w, 0.3, 1e-9, "b_w of the web");
    near(t.b_c, 1.0, 1e-6, "flange width at the top fibre");
    near(t.area, 0.285, 1e-9, "A_c of the tee");
    near(t.d, 0.45, 1e-9, "d from the top of the tee to the bars");
}

#[test]
fn transmission_length_follows_the_strength_at_release_and_de_links_use_twelve_diameters() {
    let base = Case::new("p", EN).with_n("fck", 30.0).with_n("phi", 12.5).with_n("sigma_pm0", 1400.0);
    let b3 = (0.25 * (1.0 - (28.0_f64 / 3.0).sqrt())).exp();
    near(v("B-07", &base.clone().with_n("t", 3.0), "lpt"), v("B-07", &base, "lpt") / b3, 1e-9, "l_pt scales with 1/β_cc(t)");
    let column = |annex: AnnexChoice| Case::new("c", annex).with_n("ac", 90_000.0).with_n("phi", 20.0);
    near(v("DT-09", &column(DE), "scl_max") * 1e3, 240.0, 1e-9, "12 φ_min");
    near(v("DT-09", &column(EN), "scl_max") * 1e3, 400.0, 1e-9, "min(20 φ, 400)");
}

#[test]
fn polygon_members_are_centred_on_their_outline_centroid_in_checks_and_bars() {
    use semio_s_artifact_norm_en1992_model::{BarPoint, En1992Snapshot, SectionPoint};
    let base = En1992Snapshot::liquid_retaining_fem_anchor();
    let wall = base.members[0].clone();
    let m_rd = |m: &semio_s_artifact_norm_en1992_model::RcMember| member_m_rd_nm(&member_section_in(&base, m, 35.0e6, 500.0e6, 200.0e9, DE, false), 63.0e3) / 1e3;
    let rect = m_rd(&wall);
    near(rect, 209.358455, 1e-3, "oracle M_Rd of the tank wall, N_Ed = 63 kN tension");
    let corner_frame = |dy: f64, dz: f64| {
        let mut m = wall.clone();
        m.section_points = [(0.0, 0.0), (0.35, 0.0), (0.35, 0.5), (0.0, 0.5)].iter().enumerate().map(|(i, p)| SectionPoint { id: format!("p{i}"), y: p.0 + dy, z: p.1 + dz }).collect();
        m
    };
    for (dy, dz) in [(0.0, 0.0), (0.8, -1.2)] {
        let m = corner_frame(dy, dz);
        near(m_rd(&m), rect, 1e-9, "M_Rd about the outline centroid is independent of the section-point frame");
        let s = member_section_in(&base, &m, 35.0e6, 500.0e6, 200.0e9, DE, false);
        assert!(s.gross_props().zc.abs() < 1e-9 && s.gross_props().yc.abs() < 1e-9, "centroid at the origin");
        let mut one = m.clone();
        one.bars = vec![BarPoint { id: "b".into(), layer_id: "vert".into(), y: 0.175 + dy, z: 0.05 + dz, diameter: 0.016, length: 0.0, bend: semio_s_artifact_norm_en1992_model::BarBend::Auto }];
        let s = member_section_in(&base, &one, 35.0e6, 500.0e6, 200.0e9, DE, false);
        assert!(s.tension_area() > 0.0, "a bar 50 mm above the lower face lies below the centroid");
        near(s.bars[0].z, -200.0, 1e-9, "bar height relative to the centroid in mm");
        near(s.bars[0].y, 0.0, 1e-9, "bar offset relative to the centroid in mm");
    }
}

#[test]
fn e19_torsion_reinforcement_is_compared_with_the_provided_links_and_bars() {
    let c = Case::new("t", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("h", 500.0).with_n("t_ed", 20.0e6).with_n("cot_theta", 1.0);
    let (asw_s, asl_mm2) = (v("T-02", &c, "asw_s"), v("T-02", &c, "asl") * 1e6);
    let provided = c.clone().with_n("asw_s_provided", asw_s * 1.25).with_n("asl_provided", asl_mm2 * 1.25);
    near(u("T-02", &provided, "util_links"), 0.8, 1e-9, "links 1/1.25");
    near(u("T-02", &provided, "util_long"), 0.8, 1e-9, "longitudinal 1/1.25");
    let t_ef = 300.0 * 500.0 / (2.0 * (300.0 + 500.0));
    let (a_k, u_k) = ((300.0 - t_ef) * (500.0 - t_ef), 2.0 * ((300.0 - t_ef) + (500.0 - t_ef)));
    near(v("T-02", &c, "shear_flow") * 1e0, 20.0e6 / (2.0 * a_k), 1e-9, "q = T_Ed/(2 A_k)");
    near(asl_mm2, 20.0e6 * u_k / (2.0 * a_k * 500.0 / 1.15), 1e-9, "Eq. 6.28 by hand");
}

#[test]
fn e20_the_interaction_limits_of_the_scene_diagram_are_the_lines_of_the_check() {
    let c = Case::new("t", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("h", 500.0).with_n("t_ed", 20.0e6).with_n("cot_theta", 1.0).with_n("bw", 300.0).with_n("d", 450.0).with_n("asl", 1257.0).with_n("v_ed", 150.0e3);
    let (t_max, v_max, t_c, v_c) = (v("T-04", &c, "t_rd_max") * 1e3, v("T-04", &c, "v_rd_max"), v("T-04", &c, "t_rd_c") * 1e3, v("T-04", &c, "v_rd_c"));
    near(u("T-04", &c, "util_max"), 20.0e6 / t_max + 150.0e3 / v_max, 1e-12, "T/T_Rd,max + V/V_Rd,max");
    near(u("T-04", &c, "util_min"), 20.0e6 / t_c + 150.0e3 / v_c, 1e-12, "T/T_Rd,c + V/V_Rd,c");
    near(t_max, out("T-02", &c, "util").capacity * 1e3, 1e-12, "T-02 and T-04 share T_Rd,max");
}

#[test]
fn e21_nominal_curvature_follows_the_hand_chain_of_5_8_8() {
    let c = Case::new("col", DE).with_n("fck", 30.0).with_n("b", 400.0).with_n("h", 400.0).with_n("n_ed", 1.2e6).with_n("l0", 6000.0).with_n("as", 1600.0).with_n("is", 160.0).with_n("phi_ef", 1.2);
    let (fcd, fyd, ac) = (0.85 * 30.0 / 1.5, 500.0 / 1.15, 400.0 * 400.0);
    let (omega, n) = (1600.0 * fyd / (ac * fcd), 1.2e6 / (ac * fcd));
    let k_r = (1.0 + omega - n) / (1.0 + omega - 0.4);
    let lambda = 6000.0 / (400.0 / 12.0_f64.sqrt());
    let k_phi = 1.0 + (0.35 + 30.0 / 200.0 - lambda / 150.0) * 1.2;
    let inv_r = k_r * k_phi * (fyd / 200000.0) / (0.45 * 360.0);
    near(v("A-10", &c, "inv_r"), inv_r, 1e-9, "1/r");
    near(v("A-10", &c, "e2") * 1e3, inv_r * 6000.0 * 6000.0 / (std::f64::consts::PI * std::f64::consts::PI), 1e-9, "e_2");
}

#[test]
fn e22_the_crack_width_grows_with_the_moment_and_vanishes_below_cracking() {
    let case = |m_qp: f64| Case::new("sl", DE).with_n("fck", 30.0).with_n("b", 300.0).with_n("h", 500.0).with_n("d", 450.0).with_n("as", 1257.0).with_n("c", 40.0).with_n("phi", 20.0).with_n("m_qp", m_qp).with_n("alpha_e", 6.06);
    let wk = |m: f64| v("SL-06", &case(m), "util");
    let sr = v("SL-06", &case(150.0e6), "sr_max");
    assert!(wk(75.0e6) < wk(100.0e6) && wk(100.0e6) < wk(150.0e6) && wk(150.0e6) < wk(225.0e6), "w_k is monotone in the moment");
    near(wk(150.0e6), sr * v("SL-06", &case(150.0e6), "eps_diff"), 1e-12, "w_k = s_r,max (ε_sm − ε_cm)");
    near(v("SL-06", &case(1.0e6), "wk"), 0.0, 1.0, "no crack below the cracking moment");
}

#[test]
fn e23_torsion_longitudinal_steel_is_added_to_the_bending_steel_of_the_tension_chord() {
    let c = Case::new("t", DE).with_n("fck", 30.0).with_n("b", 400.0).with_n("h", 600.0).with_n("t_ed", 35.0e6).with_n("cot_theta", 1.5).with_n("c", 30.0).with_n("phi_l", 20.0);
    let asl_mm2 = v("T-02", &c, "asl") * 1e6;
    let torsion_only = c.clone().with_n("asl_provided", 1000.0);
    near(u("T-02", &torsion_only, "util_long"), asl_mm2 / 1000.0, 1e-9, "torsion steel against all bars");
    assert!(u("T-02", &torsion_only, "util_long") < 1.0, "the torsion-only check passes");
    let combined = torsion_only.with_n("asl_bending", 700.0).with_n("tension_share", 0.5);
    near(u("T-02", &combined, "util_long"), (700.0 + 0.5 * asl_mm2) / 1000.0, 1e-9, "A_s,bending + share ΣA_sl over provided");
    assert!(u("T-02", &combined, "util_long") > 1.0, "the same bars fail once the bending steel is counted");
    let chords = combined.with_n("asl_compression_provided", 300.0);
    near(u("T-02", &chords, "util_long_comp"), 0.5 * asl_mm2 / 300.0, 1e-9, "compression chord carries the other share");
}

#[test]
fn e24_the_wall_limit_reaches_the_bar_centre_through_the_link() {
    let c = Case::new("slender", DE).with_n("b", 300.0).with_n("h", 550.0).with_n("c", 35.0 + 10.0).with_n("phi_l", 25.0);
    near(v("T-01", &c, "tef") * 1e3, 115.0, 1e-9, "t_ef = 2 (35 + 10 + 12.5) = 115 mm, not 2 (35 + 12.5) = 95 mm");
    near(v("T-01", &c, "ak") * 1e6, 185.0 * 435.0, 1e-9, "A_k = (300 - 115)(550 - 115)");
    near(v("T-01", &c, "uk") * 1e3, 2.0 * (185.0 + 435.0), 1e-9, "u_k");
}
