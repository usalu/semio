//! 🎯️ EN 1992 oracle wiring: every blessed case of `🧪️tests/🧫️fixtures/🔮️oracle-cases` is fed to OUR evaluators (clause functions and catalogue entries) and compared under the case profile; checks without an entry point stay unregistered and are reported as gaps by the shared driver.

extern crate semio_s_artifact_norm_en1992_checks as checks;

use checks::catalogue::{Case, Outcome};
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_plugin_norm_mechanics::{solve_continuous_beam, ContinuousBeam};

/// 🧭️ The annex whose registry value of the NDP `id` equals `value`.
fn annex_with(id: &str, value: f64) -> AnnexChoice {
    [AnnexChoice::En, AnnexChoice::De].into_iter().find(|annex| (checks::ndp(*annex, id) - value).abs() < 1e-9).unwrap_or_else(|| panic!("no annex registers {id} = {value}"))
}

/// 🪙️ The annex whose persistent α_cc equals the case input `alpha_cc`.
fn alpha_cc_annex(inputs: &Inputs) -> AnnexChoice {
    annex_with("ndp.en1992.m-01.alpha-cc-persistent", inputs.num("alpha_cc"))
}

/// 🧾️ The outcomes of one catalogue entry; unusable inputs panic and become a gap of the case.
fn outcomes(id: &str, case: &Case) -> Vec<Outcome> {
    checks::catalogue_registry::evaluate_outcomes(id, case).unwrap_or_else(|reason| panic!("catalogue entry {id}: {reason}"))
}

/// 🔎️ The SI value of the outcome `key`.
fn pick(outcomes: &[Outcome], key: &str) -> f64 {
    outcomes.iter().find(|outcome| outcome.key == key).map(Outcome::value).unwrap_or_else(|| panic!("catalogue outcome {key} is missing"))
}

/// 🧪️ Concrete cylinder strength in MPa from a case input in Pa.
fn mpa(inputs: &Inputs, key: &str) -> f64 {
    inputs.num(key) / 1.0e6
}

/// 📏️ A length case input in m as the mm of the catalogue.
fn mm(inputs: &Inputs, key: &str) -> f64 {
    inputs.num(key) * 1.0e3
}

/// 🧵️ Creep φ(t,t0) of Annex B.1 and shrinkage ε_cd, ε_ca, ε_cs of 3.1.4(6) from the notional size h_0 = 2 A_c/u and the cement class 0 = S, 1 = N, 2 = R, with the intermediate terms h_0, φ_RH, φ_0 and ε_cd,0.
fn creep_shrinkage(inputs: &Inputs) -> Outputs {
    let (fck, rh, t0, t, ts) = (mpa(inputs, "fck"), inputs.num("RH"), inputs.num("t0"), inputs.num("t"), inputs.num("t_s"));
    let h0 = checks::materials_and_time::notional_size_ac_u(inputs.num("Ac"), inputs.num("u"));
    let cement = ["S", "N", "R"][inputs.count("cement_class")];
    let (eps_cd, eps_ca, eps_cs) = checks::materials_and_time::shrinkage(fck, rh, h0 * 1.0e3, t, ts, cement);
    let (phi_rh, phi_0, phi) = checks::materials_and_time::creep_terms(fck + 8.0, rh, h0 * 1.0e3, t0, t, cement);
    Outputs::default()
        .with("h_0", h0)
        .with("phi_RH", phi_rh)
        .with("phi_0", phi_0)
        .with("phi", phi)
        .with("eps_cd_0", checks::materials_and_time::eps_cd_0(fck, rh, cement))
        .with("eps_cd", eps_cd)
        .with("eps_ca", eps_ca)
        .with("eps_cs", eps_cs)
}

/// 🔼️ Rectangular stress block factor η of 3.1.7(3).
fn block_eta(inputs: &Inputs) -> Outputs {
    Outputs::default().with("eta", checks::materials_and_time::lambda_eta(mpa(inputs, "f_ck")).1)
}

/// 🔽️ Rectangular stress block factor λ of 3.1.7(3).
fn block_lambda(inputs: &Inputs) -> Outputs {
    Outputs::default().with("lambda", checks::materials_and_time::lambda_eta(mpa(inputs, "f_ck")).0)
}

/// 🏠️ Nominal cover c_nom = c_min + Δc_dev (Eq. 4.1).
fn nominal_cover(inputs: &Inputs) -> Outputs {
    Outputs::default().with("c_nom", checks::part_1_1::c_nom_m(inputs.num("c_min"), 0.0, inputs.num("delta_c_dev")).1)
}

/// 🛡️ Minimum cover c_min = max(c_min,b, c_min,dur + Δc_dur,γ − Δc_dur,st − Δc_dur,add, 10 mm) (Eq. 4.2).
fn minimum_cover(inputs: &Inputs) -> Outputs {
    let durability = inputs.num("c_min_dur") + inputs.num("delta_c_dur_gamma") - inputs.num("delta_c_dur_st") - inputs.num("delta_c_dur_add");
    Outputs::default().with("c_min", checks::part_1_1::c_nom_m(inputs.num("c_min_b"), durability, 0.0).0)
}

/// 🕸️ Minimum crack-control reinforcement A_s,min of catalogue entry SL-04 (the strength input is replaced by the given f_ct,eff).
fn minimum_reinforcement(inputs: &Inputs) -> Outputs {
    let case = Case::new("SL-04", AnnexChoice::En)
        .with_n("fck", 30.0)
        .with_n("b", mm(inputs, "b"))
        .with_n("h", mm(inputs, "h"))
        .with_n("fct_eff", mpa(inputs, "f_ct_eff"))
        .with_n("sigma_s_allowed", mpa(inputs, "sigma_s"))
        .with_n("n_ed", inputs.num("N_Ed"));
    let found = outcomes("SL-04", &case);
    Outputs::default().with("k_c", pick(&found, "k_c")).with("k", pick(&found, "k")).with("A_s_min", pick(&found, "as_min"))
}

/// 🪟️ Rectangular crack-width case of `serviceability::crack_width` with the neutral axis `x` given; cover, bar diameter and steel stress are placeholders that do not enter h_c,eff, ρ_p,eff or the far-bar spacing.
fn crack_case(inputs: &Inputs) -> Case {
    Case::new("SL-06", AnnexChoice::En)
        .with_n("fck", 30.0)
        .with_n("c", 30.0)
        .with_n("phi", 16.0)
        .with_n("sigma_s", 250.0)
        .with_n("b", mm(inputs, "b"))
        .with_n("h", mm(inputs, "h"))
        .with_n("d", mm(inputs, "d"))
        .with_n("as", inputs.num("As") * 1.0e6)
        .with_n("x", mm(inputs, "x"))
}

/// 🌡️ Effective tension height h_c,eff and ratio ρ_p,eff of 7.3.2(3) (Eq. 7.10).
fn effective_tension_area(inputs: &Inputs) -> Outputs {
    let crack = checks::serviceability::crack_width(&crack_case(inputs)).unwrap_or_else(|reason| panic!("crack_width: {reason}"));
    Outputs::default().with("h_c_ef", crack.hc_eff / 1.0e3).with("rho_p_eff", crack.rho_p)
}

/// 🔭️ Maximum crack spacing s_r,max = 1.3 (h − x) for bars farther apart than 5 (c + φ/2) (Eq. 7.14).
fn far_bar_spacing(inputs: &Inputs) -> Outputs {
    let case = crack_case(inputs).with_n("s", 1.0e3);
    let crack = checks::serviceability::crack_width(&case).unwrap_or_else(|reason| panic!("crack_width: {reason}"));
    Outputs::default().with("sr_max", crack.sr_max / 1.0e3)
}

/// ✂️ Shear resistance without links V_Rd,c (Eq. 6.2a/b); the annex is the one whose registry C_Rd,c = 0.18 or 0.15 over γ_c equals the case C_Rd,c.
fn shear_without_links(inputs: &Inputs) -> Outputs {
    let annex = annex_with("ndp.en1992.crd-c-shear", inputs.num("C_Rdc") * inputs.num("gamma_c"));
    let (v_rd_c, ..) = checks::shear_torsion::v_rd_c(annex, mpa(inputs, "f_ck"), mm(inputs, "b_w"), mm(inputs, "d"), inputs.num("A_sl") * 1.0e6, mpa(inputs, "sigma_cp"));
    Outputs::default().with("V_Rd_c", v_rd_c)
}

/// ➰️ Shear resistance with links V_Rd,max and V_Rd,s for the strut inclination cot θ of the case and the limit V_Ed,max of Eq. 6.5 without links.
fn shear_with_links(inputs: &Inputs) -> Outputs {
    let annex = alpha_cc_annex(inputs);
    let (b, z, cot) = (inputs.num("b"), inputs.num("z"), inputs.num("cot_theta"));
    let f_ywd = checks::materials_and_time::f_yd(annex, mpa(inputs, "fyk"), false) * 1.0e6;
    Outputs::default()
        .with("V_Rd_max", checks::part_1_1::shear_v_rd_max_n(b, z, inputs.num("f_ck"), cot, annex))
        .with("V_Rd_s", checks::part_1_1::shear_v_rd_s_n(inputs.num("Asw") / inputs.num("s"), z, f_ywd, cot))
        .with("V_Ed_max_unreinforced", checks::shear_torsion::v_ed_max_unreinforced(b, inputs.num("d"), checks::shear_torsion::nu1(annex, mpa(inputs, "f_ck")), checks::materials_and_time::f_cd(annex, mpa(inputs, "f_ck"), false) * 1.0e6))
}

/// 🔗️ Link resistance V_Rd,s (Eq. 6.8) for the strut angle θ in degrees.
fn link_resistance(inputs: &Inputs) -> Outputs {
    let cot = 1.0 / inputs.num("theta").to_radians().tan();
    Outputs::default().with("V_Rd_s", checks::part_1_1::shear_v_rd_s_n(inputs.num("a_sw") / inputs.num("s"), inputs.num("z"), inputs.num("f_ywd"), cot))
}

/// 🪨️ Strut crushing resistance V_Rd,max (Eq. 6.9) for the strut angle θ in degrees.
fn strut_resistance(inputs: &Inputs) -> Outputs {
    let cot = 1.0 / inputs.num("theta").to_radians().tan();
    Outputs::default().with("V_Rd_max", checks::shear_torsion::v_rd_max(inputs.num("alpha_cw"), inputs.num("b_w"), inputs.num("z"), inputs.num("nu_1"), inputs.num("f_cd"), cot, 0.0))
}

/// 💢️ Strength reduction factor ν of Eq. 6.6N.
fn strength_reduction(inputs: &Inputs) -> Outputs {
    Outputs::default().with("nu", checks::shear_torsion::nu1(AnnexChoice::En, mpa(inputs, "f_ck")))
}

/// 🧱️ Coefficient α_cw of Eq. 6.11a/b/c.
fn compression_chord_factor(inputs: &Inputs) -> Outputs {
    Outputs::default().with("alpha_cw", checks::shear_torsion::alpha_cw(AnnexChoice::En, inputs.num("sigma_cp"), inputs.num("f_cd")))
}

/// 💪️ Catalogue entry S-04 for the case section, the reinforcement at `bar_z` mm above the section base, the axial force `n_ed` (N, compression positive) and the stress-strain line of the case (horizontal branch unless a hardening factor is given).
fn resistance_case(inputs: &Inputs, n_ed: f64) -> Case {
    let case = Case::new("S-04", alpha_cc_annex(inputs))
        .with_n("fck", mpa(inputs, "fck"))
        .with_n("fyk", mpa(inputs, "fyk"))
        .with_n("k", inputs.opt("k_hardening").unwrap_or(1.0))
        .with_n("points", 2.0)
        .with_n("n_ed", n_ed)
        .with_l("bar_y", &[0.0])
        .with_l("bar_a", &[inputs.num("As") * 1.0e6]);
    match inputs.opt("eps_uk") {
        Some(strain) => case.with_n("eps_uk", strain * 1.0e3),
        None => case,
    }
}

/// 🟦️ Rectangular section b × h with the tension steel at the effective depth d.
fn rectangle(inputs: &Inputs, n_ed: f64) -> Case {
    let (h, d) = (mm(inputs, "h"), mm(inputs, "d"));
    resistance_case(inputs, n_ed).with_n("b", mm(inputs, "b")).with_n("h", h).with_l("bar_z", &[h / 2.0 - d])
}

/// 🏛️ T-section with the flange b_f × h_f on top of the web b_w × (h − h_f), tension steel at the effective depth d.
fn tee(inputs: &Inputs, n_ed: f64) -> Case {
    let (bf, hf, bw, h, d) = (mm(inputs, "b_f"), mm(inputs, "h_f"), mm(inputs, "b_w"), mm(inputs, "h"), mm(inputs, "d"));
    let web_top = h - hf;
    resistance_case(inputs, n_ed)
        .with_l("poly_y", &[-bw / 2.0, bw / 2.0, bw / 2.0, bf / 2.0, bf / 2.0, -bf / 2.0, -bf / 2.0, -bw / 2.0])
        .with_l("poly_z", &[0.0, 0.0, web_top, web_top, h, h, web_top, web_top])
        .with_l("bar_z", &[h - d])
}

/// 📐️ Flexural resistance M_Rd and neutral axis depth x of a rectangular section by strain compatibility.
fn rectangular_flexure(inputs: &Inputs) -> Outputs {
    let found = outcomes("S-04", &rectangle(inputs, 0.0));
    Outputs::default().with("M_Rd", pick(&found, "m_rd")).with("x", pick(&found, "x"))
}

/// 🔩️ Flexural resistance M_Rd of a T-section by strain compatibility.
fn tee_flexure(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_Rd", pick(&outcomes("S-04", &tee(inputs, 0.0)), "m_rd"))
}

/// 🏷️ Quantity names of the M-N interaction points.
static INTERACTION_POINTS: [&str; 5] = ["M_Rd_0", "M_Rd_1", "M_Rd_2", "M_Rd_3", "M_Rd_4"];

/// 📊️ Moment resistance M_Rd(N_Ed) of the interaction diagram at the listed axial forces.
fn interaction(inputs: &Inputs) -> Outputs {
    inputs
        .nums("N_Ed")
        .iter()
        .zip(INTERACTION_POINTS.iter())
        .fold(Outputs::default(), |found, (n_ed, name)| found.with(*name, pick(&outcomes("S-04", &rectangle(inputs, *n_ed)), "m_rd")))
}

/// 🔨️ Cracked section of catalogue entry SL-03: x, I_II, σ_c, σ_s under M_Ed; the cracking moment M_cr is the flexural one of M-14.
fn cracked_section(inputs: &Inputs) -> Outputs {
    let annex = alpha_cc_annex(inputs);
    let case = Case::new("SL-03", annex)
        .with_n("fck", mpa(inputs, "fck"))
        .with_n("b", mm(inputs, "b"))
        .with_n("h", mm(inputs, "h"))
        .with_n("d", mm(inputs, "d"))
        .with_n("as", inputs.num("As") * 1.0e6)
        .with_n("m_ed", inputs.num("M_Ed") * 1.0e3);
    let state = outcomes("SL-03", &case);
    let cracking = Case::new("M-14", annex).with_n("fck", mpa(inputs, "fck")).with_n("b", mm(inputs, "b")).with_n("h", mm(inputs, "h"));
    Outputs::default()
        .with("x_cr", pick(&state, "x"))
        .with("I_cr", pick(&state, "i_ii") * 1.0e-12)
        .with("sigma_c", pick(&state, "sigma_c"))
        .with("sigma_s", pick(&state, "sigma_s"))
        .with("M_cr", pick(&outcomes("M-14", &cracking), "mcr"))
}

/// 🏔️ Table 3.1 properties of concrete (strains from permille) and the design strength f_cd = α_cc f_ck/γ_c of the case factors.
fn concrete_laws(inputs: &Inputs) -> Outputs {
    let fck = inputs.num("fck");
    let row = checks::materials_and_time::conc(fck / 1.0e6);
    let factors = checks::na_de::AnnexParams { alpha_cc: inputs.num("alpha_cc"), gamma_c: inputs.num("gamma_c"), ..checks::na_de::AnnexParams::en() };
    Outputs::default()
        .with("fcm", row.fcm * 1.0e6)
        .with("fctm", row.fctm * 1.0e6)
        .with("fctk_5", row.fctk05 * 1.0e6)
        .with("fctk_95", row.fctk95 * 1.0e6)
        .with("Ecm", row.ecm * 1.0e6)
        .with("fcd", factors.f_cd_pa(fck))
        .with("eps_c1", row.eps_c1 / 1.0e3)
        .with("eps_cu1", row.eps_cu1 / 1.0e3)
        .with("eps_c2", row.eps_c2 / 1.0e3)
        .with("eps_cu2", row.eps_cu2 / 1.0e3)
        .with("n", row.n)
}

/// 💎️ Design compressive strength f_cd = α_cc f_ck/γ_c (Eq. 3.15) with the case factors.
fn design_strength(inputs: &Inputs) -> Outputs {
    let factors = checks::na_de::AnnexParams { alpha_cc: inputs.num("alpha_cc"), gamma_c: inputs.num("gamma_c"), ..checks::na_de::AnnexParams::en() };
    Outputs::default().with("f_cd", factors.f_cd_pa(inputs.num("f_ck")))
}

/// 🎚️ Limit slenderness λ_lim = 20 A B C/√n (Eq. 5.13N) with n = N_Ed/(A_c f_cd) and B = √(1 + 2ω) solved for ω; A and C are the 0.7 defaults of the evaluator.
fn limit_slenderness(inputs: &Inputs) -> Outputs {
    let n = inputs.num("n_ed") / (inputs.num("a_c") * inputs.num("f_cd"));
    let omega = (inputs.num("b").powi(2) - 1.0) / 2.0;
    Outputs::default().with("lambda_lim", checks::part_1_1::lambda_limit(n, omega, AnnexChoice::En))
}

/// 🌿️ Slenderness λ = l_0/i (Eq. 5.14).
fn slenderness(inputs: &Inputs) -> Outputs {
    Outputs::default().with("lambda", checks::part_1_1::slenderness(inputs.num("l_0"), inputs.num("i")))
}

/// 📉️ Limiting span-to-depth ratio of Eq. 7.16a/b with the factor K.
fn span_to_depth(inputs: &Inputs) -> Outputs {
    Outputs::default().with("l_over_d", checks::serviceability::ld_limit(AnnexChoice::En, mpa(inputs, "f_ck"), inputs.num("rho"), inputs.num("rho_prime"), inputs.num("capital_k")))
}

/// 🪢️ Basic required anchorage length l_b,rqd = φ/4 σ_sd/f_bd (Eq. 8.3).
fn basic_anchorage(inputs: &Inputs) -> Outputs {
    Outputs::default().with("l_b_rqd", checks::part_1_1::l_b_rqd_m(inputs.num("diameter"), inputs.num("sigma_sd"), inputs.num("f_bd")))
}

/// ⚓️ Design anchorage length l_bd = α_1 α_4 max(α_2 α_3 α_5, 0.7) l_b,rqd ≥ l_b,min (Eq. 8.4) with the minimum length l_b,min of Eq. 8.6.
fn design_anchorage(inputs: &Inputs) -> Outputs {
    let product = checks::part_1_1::alpha_235(inputs.num("alpha_2"), inputs.num("alpha_3"), inputs.num("alpha_5"));
    let alpha = inputs.num("alpha_1") * inputs.num("alpha_4") * product;
    Outputs::default()
        .with("product_alpha_235", product)
        .with("l_b_min", checks::part_1_1::l_b_min_m(inputs.num("l_b_rqd"), inputs.num("diameter")))
        .with("l_bd", checks::part_1_1::l_bd_m(inputs.num("l_b_rqd"), inputs.num("diameter"), alpha))
}

/// 🧶️ Lap length l_0 = α_6 l_bd with l_bd = α_1 α_2 α_3 α_5 l_b,rqd (Eq. 8.10); the minimum l_0,min stands for 15 φ.
fn lap_length(inputs: &Inputs) -> Outputs {
    let anchorage = ["alpha_1", "alpha_2", "alpha_3", "alpha_5"].iter().map(|key| inputs.num(key)).product::<f64>() * inputs.num("l_b_rqd");
    Outputs::default().with("l_0", checks::part_1_1::l_0_m(anchorage, inputs.num("l_0_min") / 15.0, inputs.num("alpha_6")))
}

/// 🎋️ Equivalent bundle diameter φ_n = φ √n_b (Eq. 8.14).
fn bundle_diameter(inputs: &Inputs) -> Outputs {
    Outputs::default().with("diameter_n", checks::part_1_1::c_min_b_m(AnnexChoice::En, inputs.num("diameter"), 0.0, inputs.count("n_b") as u32))
}

/// 🧩️ Reinforcement of a membrane element (Annex F) per unit width at the strut inclination cot θ = 1.
fn membrane_reinforcement(inputs: &Inputs) -> Outputs {
    let (f_x, f_y, _) = checks::annex_parts::plane_stress_design(inputs.num("n_x") / 1.0e3, inputs.num("n_y") / 1.0e3, inputs.num("n_xy") / 1.0e3, 1.0);
    Outputs::default().with("f_x", f_x * 1.0e3).with("f_y", f_y * 1.0e3)
}

/// 🏗️ Corbel truss of catalogue entry AJ-03: tie force F_t and strut force C (the strength input is a placeholder that does not enter the forces).
fn corbel_truss(inputs: &Inputs) -> Outputs {
    let case = Case::new("AJ-03", AnnexChoice::En)
        .with_n("f_ed", inputs.num("F"))
        .with_n("a_c", mm(inputs, "a"))
        .with_n("z0", mm(inputs, "z"))
        .with_n("h_ed", inputs.num("H"))
        .with_n("fck", 30.0);
    let found = outcomes("AJ-03", &case);
    Outputs::default().with("F_t", pick(&found, "f_td")).with("C", pick(&found, "c_strut")).with("theta", pick(&found, "theta"))
}

/// ⏳️ Age-adjusted modulus of EN 1992-2 Annex KK: φ(t,t0) from the given φ_RH and β_H, the registry age-adjusting coefficient χ and E_c,adj = E_c/(1 + χ φ).
fn age_adjusted_modulus(inputs: &Inputs) -> Outputs {
    let fcm = mpa(inputs, "f_cm");
    let (_, phi) = checks::materials_and_time::creep_phi_from(fcm, inputs.num("phi_RH"), inputs.num("beta_H"), inputs.num("t0"), inputs.num("t"));
    let chi = checks::ndp(AnnexChoice::En, "ndp.en1992.kk-ageing-chi");
    Outputs::default().with("phi", phi).with("chi", chi).with("E_adj", checks::materials_and_time::age_adjusted_modulus(inputs.num("E"), chi, phi))
}

/// 🌀️ Creep strain ε_cc = φ σ_c/E_c with the given modulus.
fn creep_deformation(inputs: &Inputs) -> Outputs {
    Outputs::default().with("epsilon_cc", checks::materials_and_time::creep_strain(inputs.num("phi_inf_t0"), inputs.num("sigma_c"), inputs.num("e_c")))
}

/// 🧱️ Upper limit of the link area A_sw,max (Eq. 9.6N) with the annex chosen by α_cc.
fn link_limit(inputs: &Inputs) -> Outputs {
    let annex = alpha_cc_annex(inputs);
    let factors = checks::na_de::AnnexParams { alpha_cc: inputs.num("alpha_cc"), gamma_c: inputs.num("gamma_c"), ..checks::na_de::AnnexParams::en() };
    let f_ck = inputs.num("f_ck");
    Outputs::default().with("A_sw_max", checks::shear_torsion::a_sw_max(annex, f_ck / 1.0e6, factors.f_cd_pa(f_ck), 0.0, inputs.num("b_w"), inputs.num("s"), inputs.num("f_ywd")))
}

/// 💢️ Strut strength reduction factor ν_1 of Eq. 6.10aN/bN.
fn strut_factor(inputs: &Inputs) -> Outputs {
    Outputs::default().with("nu_1", checks::shear_torsion::nu1_strut(mpa(inputs, "f_ck")))
}

/// 🌀️ Shear flow of a torsion wall τ_t,i t_ef,i = T_Ed/(2 A_k) (Eq. 6.26).
fn torsion_wall(inputs: &Inputs) -> Outputs {
    Outputs::default().with("tau_t_t_ef", checks::shear_torsion::torsion_shear_flow(inputs.num("t_ed"), inputs.num("a_k")))
}

/// 🌀️ Longitudinal torsion reinforcement ΣA_sl (Eq. 6.28) for the strut angle θ in degrees.
fn torsion_longitudinal(inputs: &Inputs) -> Outputs {
    let cot = 1.0 / inputs.num("theta").to_radians().tan();
    Outputs::default().with("sum_A_sl", checks::shear_torsion::torsion_longitudinal(inputs.num("t_ed"), inputs.num("u_k"), inputs.num("a_k"), inputs.num("f_yd"), cot))
}

/// 🌀️ Torsion resistance T_Rd,max (Eq. 6.30) for the strut angle θ in degrees.
fn torsion_resistance(inputs: &Inputs) -> Outputs {
    Outputs::default().with("T_Rd_max", checks::shear_torsion::t_rd_max(inputs.num("nu"), inputs.num("alpha_cw"), inputs.num("f_cd"), inputs.num("a_k"), inputs.num("t_ef_i"), inputs.num("theta").to_radians()))
}

/// 🎯️ Punching shear stress v_Ed = β V_Ed/(u_i d) (Eq. 6.38).
fn punching_stress(inputs: &Inputs) -> Outputs {
    Outputs::default().with("v_Ed", checks::punching_and_struts::v_ed_punching(inputs.num("beta"), inputs.num("v_ed"), inputs.num("u_i"), inputs.num("d")))
}

/// 🎯️ Eccentricity factor β = 1 + k M_Ed/V_Ed u_1/W_1 (Eq. 6.39).
fn punching_beta_moment(inputs: &Inputs) -> Outputs {
    Outputs::default().with("beta", checks::punching_and_struts::beta_moment(inputs.num("k"), inputs.num("m_ed"), inputs.num("v_ed"), inputs.num("u_1"), inputs.num("w_1")))
}

/// 🎯️ Eccentricity factor β of a rectangular column (Eq. 6.43).
fn punching_beta_rectangular(inputs: &Inputs) -> Outputs {
    Outputs::default().with("beta", checks::punching_and_struts::beta_rectangular(inputs.num("ey"), inputs.num("ez"), inputs.num("by"), inputs.num("bz")))
}

/// 🎯️ Punching resistance v_Rd,c (Eq. 6.47) with the given C_Rd,c, k and v_min (MPa inside, Pa at the interface).
fn punching_resistance(inputs: &Inputs) -> Outputs {
    let value = checks::punching_and_struts::v_rd_c_from(inputs.num("c_rd_c"), inputs.num("k"), inputs.num("rho_l"), mpa(inputs, "f_ck"), inputs.num("k_1"), inputs.num("sigma_cp") / 1.0e6, inputs.num("v_min") / 1.0e6);
    Outputs::default().with("v_Rd_c", value * 1.0e6)
}

/// 🎯️ Punching stress with a moment transfer v_Ed = V_Ed/(u d) (1 + k M_Ed u/(V_Ed W)) (Eq. 6.51).
fn punching_eccentric(inputs: &Inputs) -> Outputs {
    let (v_ed, u) = (inputs.num("v_ed_red"), inputs.num("u"));
    let beta = checks::punching_and_struts::beta_moment(inputs.num("k"), inputs.num("m_ed"), v_ed, u, inputs.num("w"));
    Outputs::default().with("v_Ed", checks::punching_and_struts::v_ed_punching(beta, v_ed, u, inputs.num("d")))
}

/// 🪟️ Strain difference ε_sm − ε_cm and modular ratio α_e (Eq. 7.9).
fn strain_difference(inputs: &Inputs) -> Outputs {
    let (difference, alpha_e) = checks::serviceability::strain_difference(inputs.num("sigma_s"), inputs.num("e_s"), inputs.num("e_cm"), inputs.num("rho_p_eff"), inputs.num("k_t"), inputs.num("f_ct_eff"));
    Outputs::default().with("epsilon_sm_minus_epsilon_cm", difference).with("alpha_e", alpha_e)
}

/// 📏️ Maximum crack spacing s_r,max (Eq. 7.11).
fn crack_spacing(inputs: &Inputs) -> Outputs {
    Outputs::default().with("sr_max", checks::serviceability::sr_max_closed(inputs.num("c"), inputs.num("diam"), inputs.num("rho_p_eff"), inputs.num("k_1"), inputs.num("k_2"), inputs.num("k_3"), inputs.num("k_4")))
}

/// 📏️ Crack width w_k = s_r,max (ε_sm − ε_cm) (Eq. 7.8).
fn crack_width_of(inputs: &Inputs) -> Outputs {
    Outputs::default().with("w_k", checks::serviceability::crack_width_value(inputs.num("s_r_max"), inputs.num("epsilon_sm_minus_epsilon_cm")))
}

/// 🌿️ Nominal curvature 1/r = K_r K_φ ε_yd/(0.45 d) (Eq. 5.34).
fn nominal_curvature(inputs: &Inputs) -> Outputs {
    Outputs::default().with("curvature", checks::analysis_models::curvature_534(inputs.num("k_r"), inputs.num("k_phi"), inputs.num("f_yd"), inputs.num("e_s"), inputs.num("d")))
}

/// 🪢️ Ultimate bond stress f_bd = 2.25 η_1 η_2 f_ctd (Eq. 8.2).
fn bond_stress(inputs: &Inputs) -> Outputs {
    Outputs::default().with("f_bd", checks::detailing::f_bd_from(AnnexChoice::En, inputs.num("eta_1"), inputs.num("eta_2"), inputs.num("f_ctd")))
}

/// 🏷️ Quantity names of the per-support and per-span results of the continuous beam.
const REACTION: [&str; 4] = ["R_0", "R_1", "R_2", "R_3"];
const SAGGING: [&str; 3] = ["M_sag_0", "M_sag_1", "M_sag_2"];
const HOGGING: [&str; 3] = ["M_hog_0", "M_hog_1", "M_hog_2"];
const DEFLECTION: [&str; 3] = ["delta_0", "delta_1", "delta_2"];

/// 🌉️ Continuous beam (optionally clamped at the start) under span loads and midspan forces through the typed mechanics API `solve_continuous_beam`.
fn continuous_beam(inputs: &Inputs) -> Outputs {
    let spans = inputs.nums("spans");
    let mid_forces = if inputs.0.contains_key("P_mid") { inputs.nums("P_mid") } else { vec![0.0; spans.len()] };
    let fixed_start = inputs.0.get("fixed_start").and_then(|value| value.as_bool()).unwrap_or(false);
    let loads = inputs.nums("w");
    let beam = solve_continuous_beam(&ContinuousBeam { spans: &spans, e: inputs.num("E"), area: inputs.num("A"), inertia: inputs.num("I"), fixed_start, loads: &loads, mid_forces: &mid_forces }).unwrap_or_else(|reason| panic!("continuous beam: {reason:?}"));
    let mut outputs = Outputs::default();
    for (k, reaction) in beam.reactions.iter().enumerate() {
        outputs = outputs.with(REACTION[k], *reaction);
    }
    for k in 0..spans.len() {
        outputs = outputs.with(SAGGING[k], beam.sagging[k]).with(HOGGING[k], beam.hogging[k]).with(DEFLECTION[k], beam.deflection[k]);
    }
    outputs
}

/// 🗂️ One adapter per oracle check that has an evaluator entry point.
static ADAPTERS: [Adapter; 47] = [
    Adapter { check: "en1992.3.1.creep-shrinkage", evaluator: "checks::materials_and_time::{creep_phi, shrinkage}", run: creep_shrinkage },
    Adapter { check: "en1992.3.21.eta", evaluator: "checks::materials_and_time::lambda_eta", run: block_eta },
    Adapter { check: "en1992.3.19.lambda", evaluator: "checks::materials_and_time::lambda_eta", run: block_lambda },
    Adapter { check: "en1992.4.1.c-nom", evaluator: "checks::part_1_1::c_nom_m", run: nominal_cover },
    Adapter { check: "en1992.4.2.c-min", evaluator: "checks::part_1_1::c_nom_m", run: minimum_cover },
    Adapter { check: "en1992.7.3.min-reinforcement", evaluator: "checks::catalogue SL-04 (serviceability::sl04)", run: minimum_reinforcement },
    Adapter { check: "en1992.7.3.effective-tension-area", evaluator: "checks::serviceability::crack_width", run: effective_tension_area },
    Adapter { check: "en1992.7.14.sr-max-far", evaluator: "checks::serviceability::crack_width", run: far_bar_spacing },
    Adapter { check: "en1992.6.2.shear-vrdc", evaluator: "checks::shear_torsion::v_rd_c", run: shear_without_links },
    Adapter { check: "en1992.6.2.shear-links", evaluator: "checks::part_1_1::{shear_v_rd_max_n, shear_v_rd_s_n}", run: shear_with_links },
    Adapter { check: "en1992.6.8.vrds", evaluator: "checks::part_1_1::shear_v_rd_s_n", run: link_resistance },
    Adapter { check: "en1992.6.9.vrdmax", evaluator: "checks::shear_torsion::v_rd_max", run: strut_resistance },
    Adapter { check: "en1992.6.6n.nu", evaluator: "checks::shear_torsion::nu1", run: strength_reduction },
    Adapter { check: "en1992.6.11.alpha-cw", evaluator: "checks::shear_torsion::alpha_cw", run: compression_chord_factor },
    Adapter { check: "en1992.6.1.flexure-rect", evaluator: "checks::catalogue S-04 (section_resistance::s04)", run: rectangular_flexure },
    Adapter { check: "en1992.6.1.flexure-t", evaluator: "checks::catalogue S-04 (section_resistance::s04)", run: tee_flexure },
    Adapter { check: "en1992.6.1.nm-interaction", evaluator: "checks::catalogue S-04 (section_resistance::s04)", run: interaction },
    Adapter { check: "en1992.7.2.cracked-section", evaluator: "checks::catalogue SL-03 (serviceability::sl03), M-14", run: cracked_section },
    Adapter { check: "en1992.3.1.concrete-laws", evaluator: "checks::materials_and_time::conc, checks::na_de::AnnexParams::f_cd_pa", run: concrete_laws },
    Adapter { check: "en1992.3.15.fcd", evaluator: "checks::na_de::AnnexParams::f_cd_pa", run: design_strength },
    Adapter { check: "en1992.5.13n.lambda-lim", evaluator: "checks::part_1_1::lambda_limit", run: limit_slenderness },
    Adapter { check: "en1992.5.14.slenderness", evaluator: "checks::part_1_1::slenderness", run: slenderness },
    Adapter { check: "en1992.7.16.span-depth", evaluator: "checks::serviceability::ld_limit", run: span_to_depth },
    Adapter { check: "en1992.8.3.anchorage-rqd", evaluator: "checks::part_1_1::l_b_rqd_m", run: basic_anchorage },
    Adapter { check: "en1992.8.4.anchorage-design", evaluator: "checks::part_1_1::{alpha_235, l_b_min_m, l_bd_m}", run: design_anchorage },
    Adapter { check: "en1992.8.10.lap-length", evaluator: "checks::part_1_1::l_0_m", run: lap_length },
    Adapter { check: "en1992.8.14.bundle-diameter", evaluator: "checks::part_1_1::c_min_b_m", run: bundle_diameter },
    Adapter { check: "en1992.F.membrane-reinforcement", evaluator: "checks::annex_parts::plane_stress_design", run: membrane_reinforcement },
    Adapter { check: "en1992.J.3.corbel-truss", evaluator: "checks::catalogue AJ-03 (annex_parts::aj03)", run: corbel_truss },
    Adapter { check: "en1992.KK.age-adjusted-modulus", evaluator: "checks::materials_and_time::{creep_phi_from, age_adjusted_modulus}, checks::ndp(ndp.en1992.kk-ageing-chi)", run: age_adjusted_modulus },
    Adapter { check: "en1992.3.6.creep-deformation", evaluator: "checks::materials_and_time::creep_strain", run: creep_deformation },
    Adapter { check: "en1992.6.2.shear-link-limit", evaluator: "checks::shear_torsion::a_sw_max", run: link_limit },
    Adapter { check: "en1992.6.10.nu1", evaluator: "checks::shear_torsion::nu1_strut", run: strut_factor },
    Adapter { check: "en1992.6.26.tau-wall", evaluator: "checks::shear_torsion::torsion_shear_flow", run: torsion_wall },
    Adapter { check: "en1992.6.28.asl-torsion", evaluator: "checks::shear_torsion::torsion_longitudinal", run: torsion_longitudinal },
    Adapter { check: "en1992.6.30.t-rd-max", evaluator: "checks::shear_torsion::t_rd_max", run: torsion_resistance },
    Adapter { check: "en1992.6.38.punching-stress", evaluator: "checks::punching_and_struts::v_ed_punching", run: punching_stress },
    Adapter { check: "en1992.6.39.beta-moment", evaluator: "checks::punching_and_struts::beta_moment", run: punching_beta_moment },
    Adapter { check: "en1992.6.43.beta-rectangular", evaluator: "checks::punching_and_struts::beta_rectangular", run: punching_beta_rectangular },
    Adapter { check: "en1992.6.47.punching-vrdc", evaluator: "checks::punching_and_struts::v_rd_c_from", run: punching_resistance },
    Adapter { check: "en1992.6.51.punching-eccentric", evaluator: "checks::punching_and_struts::{beta_moment, v_ed_punching}", run: punching_eccentric },
    Adapter { check: "en1992.7.9.strain-difference", evaluator: "checks::serviceability::strain_difference", run: strain_difference },
    Adapter { check: "en1992.7.11.sr-max", evaluator: "checks::serviceability::sr_max_closed", run: crack_spacing },
    Adapter { check: "en1992.7.8.crack-width", evaluator: "checks::serviceability::crack_width_value", run: crack_width_of },
    Adapter { check: "en1992.5.34.curvature", evaluator: "checks::analysis_models::curvature_534", run: nominal_curvature },
    Adapter { check: "en1992.8.2.fbd", evaluator: "checks::detailing::f_bd_from", run: bond_stress },
    Adapter { check: "mechanics.beam-continuous", evaluator: "semio_s_plugin_norm_mechanics::solve_continuous_beam", run: continuous_beam },
];

const CHI: &str = "oracle → en1992";
const ORACLE: &str = "oracle-wiring v2 → oracles";
const CREEP_AGE: &str = "requests.md W3-THIRDPARTY creep beta_c loading age → en1992";

/// ⚠️ Quantities where our implementation differs from the independent oracle for a documented reason.
static KNOWN: [KnownDifference; 4] = [
    KnownDifference { case: "en1992.KK.age-adjusted-modulus-c30-rh50", quantity: "chi", reason: "ndp.en1992.kk-ageing-chi defaults to 0.8 (unverified) against 0.864 from the relaxation of the Annex B.1 creep law", request: CHI },
    KnownDifference { case: "en1992.KK.age-adjusted-modulus-c30-rh50", quantity: "E_adj", reason: "E_c,adj = E_c/(1 + chi phi) inherits the registry chi 0.8 against the relaxation chi 0.864 (13.3 against 12.7 GPa)", request: CHI },
    KnownDifference { case: "en1992.3.1.creep-shrinkage-c30-rh60-slow-short-term", quantity: "phi", reason: "we put the Eq. B.9 corrected loading age (cement class S, alpha = -1) into beta_c(t, t0) as well as into beta(t0); structuralcodes applies Eq. B.9 to beta(t0) of Eq. B.5 only and keeps the actual t0 in (t - t0) of Eq. B.7 (3.1 percent apart at t = 60 d after t0 = 28 d, below 2e-5 for t > 1000 d)", request: CREEP_AGE },
    KnownDifference { case: "en1992.7.2.cracked-c30", quantity: "M_cr", reason: "concreteproperties CrackedResults.m_cr gives 40.19 kNm against 39.9 kNm of M-14 (gross section, f_ct,fl = (1.6 - h) f_ctm of Eq. 3.23, table f_ctm 2.9 MPa); 0.8 percent exceeds numerical-v1 (0.5 percent), the definition inside the library (strength model or transformed section) is not isolated", request: ORACLE },
];

/// 🧮️ The EN 1992 adapter registry.
static REGISTRY: Registry = Registry { artifact: "en1992", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1992 oracle case is reproduced by our evaluators within its tolerance profile; gaps, mismatches and stale known differences fail the test.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1992");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
