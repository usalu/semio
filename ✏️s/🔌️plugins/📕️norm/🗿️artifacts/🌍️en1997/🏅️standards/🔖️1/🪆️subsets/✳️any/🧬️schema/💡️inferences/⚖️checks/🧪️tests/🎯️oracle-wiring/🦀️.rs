//! 🎯️ Oracle wiring of EN 1997: every blessed case of `🧪️tests/🧫️fixtures/🔮️oracle-cases` is fed to OUR catalogue evaluators and compared with the blessed value under the case profile. Unfactored inputs use the accidental EN annex (`bsA`: all partial factors 1.0).

use semio_s_artifact_norm_en1997_checks::catalogue::{evaluate_case, evaluate_outcomes, earth_pressure, foundations, piles, settlement, walls_and_anchors, Case};
use semio_s_artifact_norm_en1997_checks::{part_1, sls_pairs};
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};
use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{critical_wedge, thrust, Acceleration, Interface, Side, Water};

/// 🧾️ Catalogue case of the entry `id` with unity partial factors.
fn unfactored(id: &str) -> Case {
    Case::new(id, AnnexChoice::En).with_s("situation", "bsA")
}

/// 🔎️ Value of the outcome `key` of the catalogue entry `entry` (info value, or utilisation of a limit outcome).
fn outcome(entry: &str, case: &Case, key: &str) -> f64 {
    evaluate_outcomes(entry, case)
        .unwrap_or_else(|reason| panic!("{entry}: {reason}"))
        .iter()
        .find(|outcome| outcome.key == key)
        .unwrap_or_else(|| panic!("{entry}: no outcome {key}"))
        .value()
}

/// 📐️ Stress-strain settlement of a rectangle under its centre, SLS-01 over 64 equal layers down to `z_max`.
fn rectangle_settlement(b: f64, l: f64, q: f64, e_m: f64, z_max: f64) -> f64 {
    const LAYERS: usize = 64;
    let h = z_max / LAYERS as f64;
    let case = (1..=LAYERS).fold(unfactored("SLS-01").with_n("sigma0", q).with_s("shape", "rect").with_n("b", b).with_n("l", l), |case, k| case.with_n(&format!("h{k}"), h).with_n(&format!("e{k}"), e_m));
    outcome("SLS-01", &case, "s")
}

/// 📐️ Stress under a point at signed distance `d` from one edge of a strip, half the centre stress of the strip of width `2|d|`.
fn strip_half(q: f64, z: f64, d: f64) -> f64 {
    d.signum() * 0.5 * settlement::sigma_z_strip_centre(q, 2.0 * d.abs(), z)
}

fn boussinesq(i: &Inputs) -> Outputs {
    let (q, z) = (i.num("q"), i.num("z"));
    let sigma = match i.text("shape") {
        "strip" => strip_half(q, z, i.num("width") - i.num("x")) + strip_half(q, z, i.num("x")),
        "rectangle" => q * settlement::newmark_corner(i.num("width") / z, i.num("length") / z),
        _ => settlement::sigma_z_circle(q, i.num("radius"), z),
    };
    Outputs::default().with("sigma_z", sigma)
}

fn bearing_resistance(i: &Inputs) -> Outputs {
    let input = foundations::BearingIn { b: i.num("B"), l: i.num("L"), d: 0.0, alpha_deg: i.num("alpha"), v: i.num("V"), hb: i.num("H"), hl: 0.0, e_b: 0.0, e_l: 0.0, phi_deg: i.num("phi"), c: i.num("c"), gamma_below: i.num("gamma"), q_over: i.num("q"), include_depth: false };
    let out = foundations::bearing_drained(&input).unwrap_or_else(|reason| panic!("{reason}"));
    Outputs::default()
        .with("N_q", out.nq)
        .with("N_c", out.nc)
        .with("N_gamma", out.ng)
        .with("s_q", out.sq)
        .with("s_gamma", out.sg)
        .with("s_c", out.sc)
        .with("i_q", out.iq)
        .with("i_gamma", out.ig)
        .with("i_c", out.ic)
        .with("b_q", out.bq)
        .with("q_ult", out.r_over_a)
        .with("R_k", out.r)
}

fn bearing_factors(i: &Inputs) -> Outputs {
    let phi = i.num("phi");
    Outputs::default().with("Nq", foundations::n_q(phi)).with("Nc", foundations::n_c(phi)).with("Ng_ec7", foundations::n_gamma(phi))
}

fn stress_strain_settlement(i: &Inputs) -> Outputs {
    let (b, l, q, z_max) = (i.num("B"), i.num("L"), i.num("q"), i.num("z_max"));
    Outputs::default().with("s", rectangle_settlement(b, l, q, i.num("E_m"), z_max)).with("sigma_z_mid", settlement::sigma_z_rect_centre(q, b, l, z_max / 2.0))
}

/// 📏️ Angular distortion between the first two footings (catalogue SLS-04) and the gradient of the least-squares settlement plane through all footings.
fn pair_outputs(s: &[f64], xy: &[(f64, f64)]) -> (f64, f64) {
    let pair = unfactored("SLS-04").with_n("s_a", s[0]).with_n("s_b", s[1]).with_n("l", (xy[1].0 - xy[0].0).hypot(xy[1].1 - xy[0].1));
    let points: Vec<(f64, f64, f64)> = xy.iter().zip(s).map(|(p, s)| (p.0, p.1, *s)).collect();
    let (_, gx, gy) = sls_pairs::settlement_plane(&points).expect("settlement plane");
    (outcome("SLS-04", &pair, "tan_omega"), gx.hypot(gy))
}

fn positions(i: &Inputs) -> Vec<(f64, f64)> {
    i.json("xy").as_array().expect("xy list").iter().map(|p| (p[0].as_f64().expect("x"), p[1].as_f64().expect("y"))).collect()
}

fn settlement_pairs(i: &Inputs) -> Outputs {
    let (b, l, e_m, z_max) = (i.num("B"), i.num("L"), i.num("E_m"), i.num("z_max"));
    let s: Vec<f64> = i.nums("q").iter().map(|q| rectangle_settlement(b, l, *q, e_m, z_max)).collect();
    let (beta, tilt) = pair_outputs(&s, &positions(i));
    Outputs::default().with("s_1", s[0]).with("beta_12", beta).with("tilt", tilt)
}

fn settlement_interaction(i: &Inputs) -> Outputs {
    const LAYERS: usize = 64;
    let (b, l, e_m, z_max, q, xy) = (i.num("B"), i.num("L"), i.num("E_m"), i.num("z_max"), i.nums("q"), positions(i));
    let h = z_max / LAYERS as f64;
    let at = |receiver: usize, loaded: usize| -> f64 {
        let (px, py) = (xy[receiver].0 - xy[loaded].0, xy[receiver].1 - xy[loaded].1);
        (0..LAYERS).map(|k| settlement::sigma_z_rect_at(q[loaded], b, l, px, py, (k as f64 + 0.5) * h) / e_m * h).sum()
    };
    let s: Vec<f64> = (0..q.len()).map(|receiver| (0..q.len()).map(|loaded| at(receiver, loaded)).sum()).collect();
    let (beta, tilt) = pair_outputs(&s, &xy);
    Outputs::default().with("s_1", s[0]).with("s_1_alone", at(0, 0)).with("beta_12", beta).with("tilt", tilt)
}

fn oedometer_settlement(i: &Inputs) -> Outputs {
    let case = unfactored("SLS-02").with_n("h", i.num("H0")).with_n("cc", i.num("C_c")).with_n("e0", i.num("e0")).with_n("sigma0", i.num("sigma_v0")).with_n("dsigma", i.num("d_sigma"));
    Outputs::default().with("s", outcome("SLS-02", &case, "s_c"))
}

fn embedded_wall(i: &Inputs) -> Outputs {
    let (phi, delta_a, delta_p) = (i.num("phi"), i.num("delta_a"), i.num("delta_p"));
    let (k_ah, k_ph) = walls_and_anchors::horizontal_coefficients(phi, delta_a, delta_p);
    let case = unfactored("RW-07")
        .with_n("h", i.num("h"))
        .with_n("gamma", i.num("gamma"))
        .with_n("phi_deg", phi)
        .with_n("delta_a_deg", delta_a)
        .with_n("delta_p_deg", delta_p)
        .with_n("q", i.num("q"))
        .with_n("prop_depth", i.opt("prop_depth").unwrap_or(0.0))
        .with_s("kind", i.text("kind"));
    Outputs::default().with("K_ah", k_ah).with("K_ph", k_ph).with("t", outcome("RW-07", &case, "t0"))
}

fn pile_group(i: &Inputs) -> Outputs {
    let case = unfactored("PILE-06").with_n("m", i.num("m")).with_n("n", i.num("n")).with_n("s", i.num("s")).with_n("d", i.num("d"));
    Outputs::default().with("E_g", outcome("PILE-06", &case, "eta"))
}

fn pile_resistance(i: &Inputs) -> Outputs {
    let (d, n) = (i.num("D"), i.count("n_profiles") as u32);
    let shaft: f64 = i.nums("q_s").iter().zip(i.nums("h")).map(|(q_s, h)| part_1::shaft_resistance_n(1.0, d, *q_s, h)).sum();
    let r_c = part_1::base_resistance_n(i.num("q_b"), std::f64::consts::PI * d * d / 4.0) + shaft;
    let (xi_1, xi_2) = part_1::pile_correlation_factors(n);
    Outputs::default().with("R_c", r_c).with("xi_1", xi_1).with("xi_2", xi_2).with("R_ck", part_1::pile_characteristic_resistance_n(r_c, i.num("R_min_ratio") * r_c, n))
}

fn anchor_resistance(i: &Inputs) -> Outputs {
    let case = unfactored("ANC-02").with_n("d", i.num("d")).with_n("l_f", i.num("l_f")).with_n("tau_f", i.num("tau_f")).with_n("a_d", i.num("a_d"));
    Outputs::default().with("R_a", outcome("ANC-02", &case, "r_a_k"))
}

fn anchor_elongation(i: &Inputs) -> Outputs {
    let case = unfactored("ANC-04").with_n("a_d", i.num("a_d")).with_n("p_t_k", 1.0e12).with_n("l_free", i.num("l_free")).with_n("e_t", i.num("E")).with_n("a_t", i.num("A_t"));
    Outputs::default().with("delta", outcome("ANC-04", &case, "elongation")).with("N", i.num("a_d"))
}

fn earth_pressure_coefficients(i: &Inputs) -> Outputs {
    let (phi, delta) = (i.num("phi"), i.num("delta"));
    Outputs::default()
        .with("Ka_rankine", earth_pressure::ka_rankine(phi, 0.0))
        .with("Kp_rankine", earth_pressure::kp_rankine(phi))
        .with("Ka_coulomb", earth_pressure::ka_coulomb(phi, delta, 0.0, 0.0))
        .with("Kp_coulomb", earth_pressure::kp_coulomb(phi, delta, 0.0, 0.0))
}

fn pile_lateral_winkler(i: &Inputs) -> Outputs {
    let (ei, k, d, l, h) = (i.num("EI"), i.num("k_h"), i.num("D"), i.num("L"), i.num("H"));
    let lambda = piles::lateral_lambda(k, d, ei);
    let m_max = (0..20_000).map(|n| piles::lateral_moment(h, lambda, f64::from(n) * 1e-3).abs()).fold(0.0, f64::max);
    Outputs::default().with("y_0", piles::lateral_deflection(h, lambda, k, d, 0.0)).with("M_max", m_max).with("N_cr", piles::winkler_buckling_load(ei, l, k, d))
}

fn mononobe_okabe(i: &Inputs) -> Outputs {
    let (gamma, h, phi, delta, kh, kv) = (i.num("gamma"), i.num("H"), i.num("phi"), i.num("delta"), i.num("k_h"), i.num("k_v"));
    let load = thrust(Side::Active, &Interface::vertical(phi, delta), gamma, h, Water::NONE, Acceleration { kh, kv }).expect("limit equilibrium");
    let (angle, _) = critical_wedge(gamma, h, phi, delta, kh, load.vertical);
    Outputs::default().with("E_ae", load.total()).with("K_ae", load.coefficient).with("rho_deg", angle)
}

fn cpt_friction_angle(i: &Inputs) -> Outputs {
    let case = unfactored("GI-04").with_n("qc", i.num("qc")).with_n("sigma_v0_eff", i.num("sigma_v0_eff"));
    Outputs::default().with("phi_deg", outcome("GI-04", &case, "phi_deg"))
}

fn cpt_undrained_strength(i: &Inputs) -> Outputs {
    const SIGMA_V0: f64 = 100_000.0;
    let case = unfactored("GI-03").with_n("qt", i.num("q_net") + SIGMA_V0).with_n("sigma_v0", SIGMA_V0).with_n("nkt", i.num("N_k"));
    Outputs::default().with("s_u", outcome("GI-03", &case, "cu"))
}

fn cpt_behaviour_index(i: &Inputs) -> Outputs {
    let case = unfactored("GI-02").with_n("qc", i.num("qt")).with_n("fs", i.num("fs")).with_n("sigma_v0", i.num("sigma_v0")).with_n("sigma_v0_eff", i.num("sigma_v0_eff"));
    Outputs::default().with("Ic", outcome("GI-02", &case, "ic"))
}

/// 🔨️ SPT correction GI-06: N60 = N·E_r/60·C_B·C_S·C_R and (N1)60 = C_N·N60 with C_N = min(√(p_a/σ′_v0), 1.7).
fn spt_correction(i: &Inputs) -> Outputs {
    let case = unfactored("GI-06").with_n("n", i.num("n")).with_n("er", i.num("er")).with_n("cb", i.num("cb")).with_n("cs", i.num("cs")).with_n("cr", i.num("cr")).with_n("sigma_v0_eff", i.num("sigma_v0_eff"));
    Outputs::default().with("n60", outcome("GI-06", &case, "n60")).with("n1_60", outcome("GI-06", &case, "n1_60"))
}

/// 🪨️ Bulk and effective unit weights of a soil from the phase relations through `part_1::{bulk_unit_weight, buoyant_unit_weight}`.
fn phase_relations(i: &Inputs) -> Outputs {
    let (s_r, e, g_s, gamma_w) = (i.num("S_r"), i.num("e"), i.num("G_s"), i.num("gamma_w"));
    Outputs::default().with("gamma", part_1::bulk_unit_weight(s_r, e, g_s, gamma_w)).with("gamma_eff", part_1::buoyant_unit_weight(s_r, e, g_s, gamma_w))
}

/// 🧾️ Catalogue case of `entry` with the annex, design approach and situation the case inputs name, so every partial factor is read from the NDP registry rows of that set (never from the inputs).
fn designed(entry: &str, i: &Inputs) -> Case {
    let annex = if i.text("annex") == "de" { AnnexChoice::De } else { AnnexChoice::En };
    Case::new(entry, annex).with_s("approach", i.text("approach")).with_s("situation", i.text("situation"))
}

/// 🏛️ GEO-F01 of a strip footing (length 10^6 m, line load scaled with it) with V_d = gamma_G V_k.
fn bearing_case(i: &Inputs) -> Case {
    const STRIP: f64 = 1.0e6;
    let case = designed("GEO-F01", i);
    let v_d = case.factors().gamma_g * i.num("V_k") * STRIP;
    case.with_n("b", i.num("B")).with_n("l", STRIP).with_n("d", i.num("D")).with_n("v_d", v_d).with_n("phi_k_deg", i.num("phi")).with_n("c_k", i.num("c")).with_n("gamma1", i.num("gamma_below")).with_n("gamma2", i.num("gamma_above"))
}

fn bearing_design(i: &Inputs) -> Outputs {
    let (case, p) = (bearing_case(i), designed("GEO-F01", i).factors());
    Outputs::default()
        .with("gamma_g", p.gamma_g)
        .with("gamma_phi", p.gamma_phi)
        .with("gamma_r_v", p.gamma_r_v)
        .with("r_d", outcome("GEO-F01", &case, "r_over_a") * i.num("B") / p.gamma_r_v)
        .with("utilization", outcome("GEO-F01", &case, "bearing"))
}

/// 🧱️ Design Rankine thrust per metre of a wall of height H with the surcharge q, K_a at the design friction angle atan(tan phi / gamma_phi).
fn rankine_thrust(i: &Inputs, gamma_phi: f64) -> (f64, f64) {
    let k_a = earth_pressure::ka_rankine((i.num("phi").to_radians().tan() / gamma_phi).atan().to_degrees(), 0.0);
    (k_a, k_a * (0.5 * i.num("gamma") * i.num("H").powi(2) + i.num("q") * i.num("H")))
}

/// 🧱️ RW-06 sliding of a gravity wall against the characteristic thrust and weight.
fn wall_case(i: &Inputs) -> Case {
    let case = designed("RW-06", i);
    let (_, thrust) = rankine_thrust(i, case.factors().gamma_phi);
    case.with_n("b", 1.0).with_n("w", i.num("W")).with_n("x_w", 0.5).with_n("eah_g", thrust).with_n("arm_g", i.num("H") / 3.0).with_n("phi_base_deg", i.num("delta_base"))
}

fn wall_sliding_design(i: &Inputs) -> Outputs {
    let (case, p) = (wall_case(i), designed("RW-06", i).factors());
    let (k_a, thrust) = rankine_thrust(i, p.gamma_phi);
    Outputs::default()
        .with("gamma_g", p.gamma_g)
        .with("gamma_phi", p.gamma_phi)
        .with("gamma_r_h", p.gamma_r_h)
        .with("k_a", k_a)
        .with("thrust_k", thrust)
        .with("utilization", outcome("RW-06", &case, "sliding"))
}

/// 📉️ SLS-02 of the layer with the Boussinesq stress increase under the centre of the rectangle at mid-layer depth.
fn settlement_case(i: &Inputs) -> (Case, f64) {
    let d_sigma = settlement::sigma_z_rect_centre(i.num("q"), i.num("B"), i.num("L"), i.num("z_top") + i.num("thickness") / 2.0);
    (unfactored("SLS-02").with_n("h", i.num("thickness")).with_n("cc", i.num("C_c")).with_n("e0", i.num("e0")).with_n("sigma0", i.num("sigma_v0")).with_n("dsigma", d_sigma), d_sigma)
}

fn settlement_primary(i: &Inputs) -> Outputs {
    let (case, d_sigma) = settlement_case(i);
    let s = outcome("SLS-02", &case, "s_c");
    Outputs::default().with("delta_sigma", d_sigma).with("s", s).with("utilization", s / i.num("limit"))
}

/// ⛰️ SLOPE-01 simplified Bishop on the given circle with 400 slices.
fn slope_case(i: &Inputs) -> Case {
    designed("SLOPE-01", i)
        .with_n("h", i.num("H"))
        .with_n("beta_deg", i.num("beta"))
        .with_n("xc", i.num("xc"))
        .with_n("yc", i.num("yc"))
        .with_n("r", i.num("r"))
        .with_n("gamma", i.num("gamma"))
        .with_n("phi_k_deg", i.num("phi"))
        .with_n("c_k", i.num("c"))
        .with_n("slices", 400.0)
}

fn slope_bishop_design(i: &Inputs) -> Outputs {
    let (case, p) = (slope_case(i), designed("SLOPE-01", i).factors());
    Outputs::default().with("gamma_phi", p.gamma_phi).with("gamma_c", p.gamma_c).with("factor", outcome("SLOPE-01", &case, "f")).with("utilization", outcome("SLOPE-01", &case, "bishop"))
}

static ADAPTERS: [Adapter; 19] = [
    Adapter { check: "en1997.2.phase-relations", evaluator: "crate::part_1::{bulk_unit_weight, buoyant_unit_weight}", run: phase_relations },
    Adapter { check: "en1997.F.boussinesq", evaluator: "crate::catalogue::settlement::{sigma_z_strip_centre, newmark_corner, sigma_z_circle}", run: boussinesq },
    Adapter { check: "en1997.D.4.bearing-resistance", evaluator: "crate::catalogue::foundations::bearing_drained", run: bearing_resistance },
    Adapter { check: "en1997.D.4.bearing-factors", evaluator: "crate::catalogue::foundations::{n_q, n_c, n_gamma}", run: bearing_factors },
    Adapter { check: "en1997.F.stress-strain-settlement", evaluator: "catalogue SLS-01 (64 layers), crate::catalogue::settlement::sigma_z_rect_centre", run: stress_strain_settlement },
    Adapter { check: "en1997.H.settlement-pairs", evaluator: "catalogue SLS-01, SLS-04, crate::sls_pairs::settlement_plane", run: settlement_pairs },
    Adapter { check: "en1997.H.settlement-interaction", evaluator: "crate::catalogue::settlement::sigma_z_rect_at, catalogue SLS-04, crate::sls_pairs::settlement_plane", run: settlement_interaction },
    Adapter { check: "en1997.F.oedometer-settlement", evaluator: "catalogue SLS-02", run: oedometer_settlement },
    Adapter { check: "en1997.9.embedded-wall", evaluator: "catalogue RW-07, crate::catalogue::walls_and_anchors::horizontal_coefficients", run: embedded_wall },
    Adapter { check: "en1997.7.pile-group", evaluator: "catalogue PILE-06", run: pile_group },
    Adapter { check: "en1997.7.pile-resistance", evaluator: "crate::part_1::{shaft_resistance_n, base_resistance_n, pile_correlation_factors, pile_characteristic_resistance_n}", run: pile_resistance },
    Adapter { check: "en1997.8.anchor-resistance", evaluator: "catalogue ANC-02", run: anchor_resistance },
    Adapter { check: "en1997.8.anchor-elongation", evaluator: "catalogue ANC-04 elongation", run: anchor_elongation },
    Adapter { check: "en1997.9.earth-pressure", evaluator: "crate::catalogue::earth_pressure::{ka_rankine, kp_rankine, ka_coulomb, kp_coulomb}", run: earth_pressure_coefficients },
    Adapter { check: "en1997.7.pile-lateral-winkler", evaluator: "crate::catalogue::piles::{lateral_lambda, lateral_deflection, lateral_moment, winkler_buckling_load}", run: pile_lateral_winkler },
    Adapter { check: "en1997.9.mononobe-okabe", evaluator: "semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{thrust, critical_wedge}", run: mononobe_okabe },
    Adapter { check: "en1997.2.cpt-friction-angle", evaluator: "catalogue GI-04", run: cpt_friction_angle },
    Adapter { check: "en1997.2.cpt-undrained-strength", evaluator: "catalogue GI-03", run: cpt_undrained_strength },
    Adapter { check: "en1997.2.cpt-behaviour-index", evaluator: "catalogue GI-02", run: cpt_behaviour_index },
    Adapter { check: "en1997.2.spt-correction", evaluator: "catalogue GI-06", run: spt_correction },
];

const CPT_NORMALISATION: &str = "requests.md W3-THIRDPARTY CPT and SPT overburden normalisation → en1997";

/// ⚠️ Quantities where the catalogue differs from the independent oracle for a documented reason.
static KNOWN: [KnownDifference; 2] = [
    KnownDifference { case: "en1997.2.cpt-behaviour-index-4", quantity: "Ic", reason: "GI-02 normalises Q = (q_t - sigma_v0) / sigma'_v0 with n = 1 and no cap; groundhog (Fugro guidance, Robertson and Wride 1998) iterates the stress exponent n = 0.381 I_c + 0.05 sigma'_v0 / p_a - 0.15 (n <= 1) and caps C_n at 1.7, which binds at sigma'_v0 = 30 kPa (I_c 3.52 against 3.27)", request: CPT_NORMALISATION },
    KnownDifference { case: "en1997.2.spt-correction-6", quantity: "n1_60", reason: "GI-06 caps C_N = sqrt(p_a / sigma'_v0) at 1.7 (34 at sigma'_v0 = 25 kPa); groundhog follows Liao and Whitman 1986 without a cap (C_N = 2.0, (N1)60 = 40)", request: CPT_NORMALISATION },
];

static REGISTRY: Registry = Registry { artifact: "en1997", adapters: &ADAPTERS, known_differences: &KNOWN };

#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1997");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}

fn pile_settlement_resistance(i: &Inputs) -> Outputs {
    let ea = i.num("E_c") * std::f64::consts::PI * i.num("D").powi(2) / 4.0;
    let case = unfactored("PILE-09").with_n("n", i.num("N")).with_n("rs_max", i.num("rs_max")).with_n("rb_max", i.num("rb_max")).with_n("d", i.num("D")).with_n("ea", ea).with_n("l", i.num("L")).with_n("s_ref_s", i.num("s_ref_s")).with_n("s_ref_b", i.num("s_ref_b"));
    let total = outcome("PILE-09", &case, "s_total");
    Outputs::default().with("s_mobilised", outcome("PILE-09", &case, "s_mobilised")).with("s_elastic", outcome("PILE-09", &case, "s_elastic")).with("s_total", total).with("utilization", total / i.num("s_lim"))
}

/// 🏛️ GEO-F03 of a rectangular footing under V_d = gamma_G V_k, H_d = gamma_G H_k along B and M_d = gamma_G M_k about the L axis.
fn bearing_din4017(i: &Inputs) -> Outputs {
    let case = designed("GEO-F03", i);
    let p = case.factors();
    let g = p.gamma_g;
    let case = case.with_n("b", i.num("B")).with_n("l", i.num("L")).with_n("d", i.num("D")).with_n("v_d", g * i.num("V_k")).with_n("hb_d", g * i.num("H_k")).with_n("mb_d", g * i.num("M_k")).with_n("phi_k_deg", i.num("phi")).with_n("c_k", i.num("c")).with_n("gamma1", i.num("gamma_below")).with_n("gamma2", i.num("gamma_above"));
    Outputs::default()
        .with("gamma_g", g)
        .with("gamma_phi", p.gamma_phi)
        .with("gamma_r_v", p.gamma_r_v)
        .with("r_d", outcome("GEO-F03", &case, "r_k") / p.gamma_r_v)
        .with("utilization", outcome("GEO-F03", &case, "bearing"))
}

/// 🧱️ RW-03 passive coefficients of the logarithmic-spiral failure surface with the Rankine and plane Coulomb coefficients beside it.
fn passive_curved(i: &Inputs) -> Outputs {
    let case = unfactored("RW-03").with_n("gamma", 18_000.0).with_n("t", 1.0).with_n("phi_deg", i.num("phi")).with_n("delta_deg", i.num("delta"));
    Outputs::default().with("kp_rankine", outcome("RW-03", &case, "kp_rankine")).with("kp_coulomb", outcome("RW-03", &case, "kp_coulomb")).with("kp", outcome("RW-03", &case, "kp"))
}

/// 🧪️ SLS-03 secondary settlement of one layer.
fn secondary_settlement(i: &Inputs) -> Outputs {
    let case = unfactored("SLS-03").with_n("h", i.num("h")).with_n("c_alpha", i.num("c_alpha")).with_n("e0", i.num("e0")).with_n("t1", i.num("t1")).with_n("t2", i.num("t2"));
    Outputs::default().with("s_sec", outcome("SLS-03", &case, "s_sec"))
}

/// 🪵️ PILE-05 drag load of one settling layer with gamma_G of the registry row of the design approach.
fn negative_skin_friction(i: &Inputs) -> Outputs {
    let case = designed("PILE-05", i).with_n("d", i.num("d")).with_n("beta", i.num("beta")).with_n("h1", i.num("h")).with_n("sv1", i.num("sv"));
    let case = match (i.opt("f_cd"), i.opt("r_c_d")) {
        (Some(f_cd), Some(r_c_d)) => case.with_n("f_cd", f_cd).with_n("r_c_d", r_c_d),
        _ => case,
    };
    let out = Outputs::default().with("gamma_g", case.factors().gamma_g).with("f_neg_k", outcome("PILE-05", &case, "f_neg_k")).with("f_neg_d", outcome("PILE-05", &case, "f_neg_d"));
    if i.opt("f_cd").is_some() { out.with("downdrag", outcome("PILE-05", &case, "downdrag")) } else { out }
}

/// 🪵️ PILE-08 dynamic-test evaluation: the series is rebuilt from its count, mean and lowest value, xi5 and xi6 from Table A.11, gamma_t of the registry row, the driving formula beside it.
fn pile_dynamic_tests(i: &Inputs) -> Outputs {
    let (n, mean, lowest) = (i.count("n"), i.num("rm_mean"), i.num("rm_min"));
    let mut case = designed("PILE-08", i).with_n("rm1", lowest);
    for k in 2..=n {
        case = case.with_n(&format!("rm{k}"), (n as f64 * mean - lowest) / (n as f64 - 1.0));
    }
    case = case.with_n("eta_h", i.num("eta")).with_n("w", i.num("W")).with_n("h_drop", i.num("h")).with_n("s_set", i.num("s")).with_n("s_e", i.num("s_e"));
    let case = match i.opt("f_cd") {
        Some(f_cd) => case.with_n("f_cd", f_cd),
        None => case,
    };
    let out = ["xi5", "xi6", "r_c_k", "r_c_d", "r_driving"].into_iter().fold(Outputs::default().with("gamma_t", case.factors().gamma_t), |out, key| out.with(key, outcome("PILE-08", &case, key)));
    if i.opt("f_cd").is_some() { out.with("compression", outcome("PILE-08", &case, "compression")) } else { out }
}

/// 🧱️ RW-09 reinforcement tension and embedment length.
fn reinforced_soil(i: &Inputs) -> Outputs {
    let case = unfactored("RW-09").with_n("k", i.num("k")).with_n("sigma_v", i.num("sigma_v")).with_n("s_v", i.num("s_v")).with_n("f_star", i.num("f_star")).with_n("b", i.num("b"));
    Outputs::default().with("t_max", outcome("RW-09", &case, "t_max")).with("l_e", outcome("RW-09", &case, "l_e"))
}

/// 💧 DEW-01 single-well discharge with the Sichardt radius.
fn single_well(i: &Inputs) -> Outputs {
    let case = unfactored("DEW-01").with_n("k", i.num("k")).with_n("h_aquifer", i.num("h_aquifer")).with_n("h_well", i.num("h_well")).with_n("r_well", i.num("r_well")).with_n("thickness", i.num("thickness")).with_s("aquifer", i.text("aquifer"));
    Outputs::default().with("radius", outcome("DEW-01", &case, "radius")).with("q", outcome("DEW-01", &case, "q"))
}

static GEOTECHNICS_ADAPTERS: [Adapter; 12] = [
    Adapter { check: "en1997.G.secondary-settlement", evaluator: "catalogue SLS-03", run: secondary_settlement },
    Adapter { check: "en1997.G.negative-skin-friction", evaluator: "catalogue PILE-05 with the registry factors", run: negative_skin_friction },
    Adapter { check: "en1997.G.pile-dynamic-tests", evaluator: "catalogue PILE-08 with the registry factors of the design approach", run: pile_dynamic_tests },
    Adapter { check: "en1997.G.reinforced-soil", evaluator: "catalogue RW-09", run: reinforced_soil },
    Adapter { check: "en1997.G.single-well", evaluator: "catalogue DEW-01", run: single_well },
    Adapter { check: "en1997.G.bearing-din4017", evaluator: "catalogue GEO-F03 with the registry factors of the design approach", run: bearing_din4017 },
    Adapter { check: "en1997.G.passive-curved", evaluator: "catalogue RW-03 (kp_curved log-spiral search)", run: passive_curved },
    Adapter { check: "en1997.G.pile-resistance-settlement", evaluator: "catalogue PILE-09 with the pile E_c", run: pile_settlement_resistance },
    Adapter { check: "en1997.G.bearing-design", evaluator: "catalogue GEO-F01 with the registry factors of the design approach", run: bearing_design },
    Adapter { check: "en1997.G.wall-sliding-design", evaluator: "catalogue RW-06 and crate::catalogue::earth_pressure::ka_rankine with the registry factors", run: wall_sliding_design },
    Adapter { check: "en1997.G.settlement-primary", evaluator: "catalogue SLS-02, crate::catalogue::settlement::sigma_z_rect_centre", run: settlement_primary },
    Adapter { check: "en1997.G.slope-bishop", evaluator: "catalogue SLOPE-01 with the registry factors of the design approach", run: slope_bishop_design },
];

static GEOTECHNICS_REGISTRY: Registry = Registry { artifact: "en1997", adapters: &GEOTECHNICS_ADAPTERS, known_differences: &KNOWN };

/// 🌍️ The design-case geotechnics cases: SI inputs naming annex, design approach and situation, quantities blessed by `🔮️oracles/🌍️geotechnics/🐍️.py` (groundhog, scipy) with the partial factors read from the NDP registry rows; they live beside, not inside, `🔮️oracle-cases` because the shared harness has no adapter for them.
const GEOTECHNICS_CASES: &str = include_str!("../../../../../🧪️tests/🧫️fixtures/🌍️geotechnics/🔣️.json");

/// 🧭️ The adapter registry of the geotechnics design cases for the `oracle-comparison` step family of `🧪️tests/🌍️geotechnics-en1997-1`.
pub fn geotechnics_registry() -> &'static Registry {
    &GEOTECHNICS_REGISTRY
}

#[test]
fn evaluators_reproduce_the_blessed_geotechnics_cases_with_registry_factors() {
    let blessed = oracle_driver::load_cases_text(GEOTECHNICS_CASES, "geotechnics");
    let mut failures = Vec::new();
    for (case, ours) in blessed.iter().zip(oracle_driver::evaluate_case_text(GEOTECHNICS_CASES, "geotechnics", &GEOTECHNICS_REGISTRY)) {
        failures.extend(ours.gap.iter().cloned());
        failures.extend(ours.missing.iter().cloned());
        for (name, expectation) in &case.expected {
            match ours.values.get(name) {
                Some(&value) if expectation.accepts(value) => {}
                Some(value) => failures.push(format!("{}.{name}: ours {value}, blessed {}", case.id, expectation.value)),
                None => {}
            }
        }
    }
    assert!(failures.is_empty(), "geotechnics oracle wiring:\n  {}", failures.join("\n  "));
}

#[test]
fn geotechnics_verdicts_rest_on_unverified_registry_rows_and_never_pass() {
    for case in oracle_driver::load_cases_text(GEOTECHNICS_CASES, "geotechnics") {
        let (entry, built) = match case.check.as_str() {
            "en1997.G.bearing-design" => ("GEO-F01", bearing_case(&case.inputs)),
            "en1997.G.wall-sliding-design" => ("RW-06", wall_case(&case.inputs)),
            "en1997.G.slope-bishop" => ("SLOPE-01", slope_case(&case.inputs)),
            _ => continue,
        };
        let results = evaluate_case(entry, &built);
        assert!(!results.is_empty(), "{}", case.id);
        assert!(results.iter().all(|result| result.status != CheckStatus::Pass), "{}: a verdict on unverified NDP rows must not be a Pass", case.id);
    }
}
