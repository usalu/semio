use crate::catalogue::*;
use crate::*;

fn out(id: &str, case: &Case) -> Vec<Outcome> { evaluate_outcomes(id, case).unwrap_or_else(|e| panic!("{id}: {e}")) }
fn v(o: &[Outcome], key: &str) -> f64 { o.iter().find(|x| x.key == key).unwrap_or_else(|| panic!("missing outcome {key}")).value }
fn near(actual: f64, expected: f64, relative: f64) { assert!((actual - expected).abs() <= relative * expected.abs().max(1e-9), "{actual} differs from {expected} by more than {relative}"); }
fn de(id: &str) -> Case { Case::new(id, AnnexChoice::De) }
fn en(id: &str) -> Case { Case::new(id, AnnexChoice::En) }

#[semio_framework_async_macros::async_test]
async fn registry_has_unique_ids_documented_inputs_and_basis_ids() {
    let all = registry();
    assert!(all.len() >= 90, "{}", all.len());
    let mut ids: Vec<&str> = all.iter().map(|e| e.id).collect();
    ids.sort_unstable();
    let before = ids.len();
    ids.dedup();
    assert_eq!(before, ids.len(), "duplicate catalogue ids");
    for e in &all { assert!(!e.en.is_empty() && !e.de.is_empty(), "{} lacks a label", e.id); }
    assert!(["ndp.en1991.e11.density", "ndp.en1991.e14.cpe-pitched", "ndp.en1991.e13.na-zones", "ndp.en1991.density.steel", "ndp.en1991.vb0.zone1"].iter().all(|id| semio_s_plugin_norm_compliance::ndp(id).is_some()), "basis ids live in the shared registry");
}

#[semio_framework_async_macros::async_test]
async fn missing_and_invalid_inputs_become_not_applicable_results() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    for e in registry() {
        if e.inputs.is_empty() || e.id == "E2.03" || e.id == "E2.14" { continue; }
        let results = evaluate_case(e.id, &de("empty"));
        assert!(!results.is_empty(), "{}", e.id);
        assert!(results.iter().all(|r| r.status == CheckStatus::NotApplicable), "{} must reject an empty case", e.id);
    }
    assert!(evaluate_case("E99.99", &de("x"))[0].status == CheckStatus::NotApplicable);
}

#[semio_framework_async_macros::async_test]
async fn e11_worked_examples() {
    let o = out("E11.01", &en("1").with_s("material", "reinforced_concrete").with_n("thickness", 0.2).with_s("material2", "cement_screed").with_n("thickness2", 0.06));
    near(v(&o, "g"), 5000.0, 1e-9);
    near(v(&o, "g2"), 1320.0, 1e-9);
    near(v(&o, "total"), 6320.0, 1e-9);
    near(v(&out("E11.01", &en("2").with_s("material", "steel").with_n("thickness", 0.01)), "g"), 770.0, 1e-9);
    near(v(&out("E11.01", &de("2").with_s("material", "steel").with_n("thickness", 0.01)), "g"), 785.0, 1e-9);
    near(v(&out("E11.01", &de("3").with_s("material", "soil_fill").with_n("thickness", 0.5)), "g"), 10_000.0, 1e-9);
    let o = out("E11.04", &en("4").with_s("category", "C1").with_n("area", 40.0).with_n("storeys", 1.0));
    near(v(&o, "alpha_a"), 0.75, 1e-9);
    near(v(&out("E11.04", &en("4b").with_s("category", "C1").with_n("area", 10.0).with_n("storeys", 1.0)), "alpha_a"), 1.0, 1e-9);
    near(v(&out("E11.04", &en("4c").with_s("category", "C1").with_n("area", 200.0).with_n("storeys", 1.0)), "alpha_a"), 0.6, 1e-9);
    near(v(&out("E11.04", &en("5").with_s("category", "B").with_n("area", 20.0).with_n("storeys", 5.0)), "alpha_n"), 0.82, 1e-9);
    let o = out("E11.02", &de("6").with_n("wall_gamma", 14_000.0).with_n("wall_thickness", 0.115).with_n("wall_height", 2.6).with_n("finish", 400.0));
    near(v(&o, "line"), 4586.0, 1e-9);
    near(v(&o, "delta_q"), 1200.0, 1e-9);
    near(v(&out("E11.09", &de("7").with_n("load", 10_000.0).with_n("a", 0.2).with_n("b", 0.2)), "pressure"), 250_000.0, 1e-9);
    near(v(&out("E11.03", &de("8").with_s("category", "Z").with_n("depth", 3.0)), "line"), 12_000.0, 1e-9);
    for (g, band) in [(3000.0, 800.0), (5000.0, 1200.0), (6000.0, 1600.0)] { near(v(&out("E11.02", &de("9").with_n("wall_load", g)), "delta_q"), band, 1e-12); }
    assert!(evaluate_outcomes("E11.02", &de("9b").with_n("wall_load", 7000.0)).is_err());
    assert!(evaluate_outcomes("E11.03", &de("x").with_s("category", "Q9")).is_err());
    assert!(evaluate_outcomes("E11.01", &de("x").with_s("material", "unobtainium").with_n("thickness", 0.1)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn e12_worked_examples() {
    for (t, e) in [(15.0, 738.6), (30.0, 841.8), (60.0, 945.3), (90.0, 1006.0), (120.0, 1049.0)] {
        let o = out("E12.01", &de("1").with_s("curve", "standard").with_n("t_min", t));
        assert!((v(&o, "theta_g") - 273.15 - e).abs() < 0.6, "{t}");
    }
    for (t, e) in [(5.0, 947.7), (30.0, 1097.7), (60.0, 1100.0)] { assert!((v(&out("E12.01", &de("2").with_s("curve", "hydrocarbon").with_n("t_min", t)), "theta_g") - 273.15 - e).abs() < 0.1); }
    for (t, e) in [(5.0, 588.5), (30.0, 680.0), (60.0, 680.0)] { assert!((v(&out("E12.01", &de("3").with_s("curve", "external").with_n("t_min", t)), "theta_g") - 273.15 - e).abs() < 0.1); }
    let o = out("E12.02", &de("4").with_n("a_f", 50.0).with_n("a_t", 184.84).with_n("a_v", 5.0).with_n("h_eq", 2.0).with_n("b", 1160.0).with_n("q_fd_mj", 336.0));
    near(v(&o, "O"), 0.0382551, 1e-5);
    near(v(&o, "gamma"), 0.914657, 1e-5);
    near(v(&o, "q_td"), 90.8894, 1e-5);
    near(v(&o, "t_max") / 3600.0, 0.475176, 1e-5);
    near(v(&o, "theta_max") - 273.15, 822.168, 1e-5);
    near(v(&out("E12.03", &de("5").with_n("area", 250.0).with_n("q_fk_mj", 420.0).with_n("delta_q2", 1.0)), "q_fd"), 504e6, 1e-9);
    near(v(&out("E12.06", &de("6").with_n("theta_g_c", 841.8).with_n("theta_m_c", 500.0)), "h_net"), 62_407.0, 2e-3);
    near(v(&out("E12.07", &de("7").with_n("g_k", 100.0).with_n("q_k", 50.0).with_n("psi", 0.5)), "eta_fi"), 0.595238, 1e-6);
    let o = out("E12.04", &de("8").with_n("q_fd_mj", 504.0).with_n("height", 3.0).with_n("alpha_v", 0.1).with_n("k_b", 0.07));
    near(v(&o, "t_ed") / 60.0, 58.5935, 1e-5);
    let o = out("E12.05", &de("9").with_n("d", 1.0).with_n("q_w", 2.0e6).with_n("z", 3.0));
    near(v(&o, "L_f"), 3.88539, 1e-5);
    near(v(&o, "z_0"), 0.716774, 1e-5);
    near(v(&o, "theta") - 273.15, 883.85, 2e-3);
    near(v(&out("E12.09", &de("10").with_n("width", 1.0).with_n("height", 1.0).with_n("distance", 1.0)), "phi"), 0.1385316, 1e-6);
    assert!(evaluate_outcomes("E12.01", &de("x").with_s("curve", "campfire").with_n("t_min", 5.0)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn e13_worked_examples() {
    let sk = |zone: &str, alt: f64| v(&out("E13.02", &de("sk").with_s("zone", zone).with_n("altitude", alt)), "s_k");
    near(sk("2", 300.0), 890.194, 1e-6);
    near(sk("3", 800.0), 4761.655, 1e-6);
    near(sk("1a", 50.0), 812.5, 1e-9);
    near(sk("1", 900.0), 1894.04, 1e-5);
    near(sk("2a", 100.0), 1062.5, 1e-9);
    near(v(&out("E13.02", &de("g").with_s("zone", "1").with_n("altitude", 50.0).with_n("north_german_lowland", 1.0)), "s_Ad"), 1495.0, 1e-9);
    near(v(&out("E13.01", &de("s").with_n("mu", 0.8).with_s("zone", "2").with_n("altitude", 300.0)), "s"), 712.155, 1e-6);
    near(v(&out("E13.03", &de("m").with_n("alpha", 40.0)), "mu1"), 0.533333, 1e-6);
    near(v(&out("E13.04", &de("m").with_n("alpha_1", 30.0).with_n("alpha_2", 30.0)), "mu2"), 1.6, 1e-12);
    let o = out("E13.06", &de("o").with_n("h", 1.0).with_n("s_k", 850.0));
    near(v(&o, "mu2_raw"), 2.35294, 1e-5);
    near(v(&o, "mu2"), 2.0, 1e-12);
    near(v(&o, "l_s"), 5.0, 1e-12);
    near(v(&out("E13.07", &de("e").with_n("s", 900.0)), "s_e"), 243.0, 1e-9);
    near(v(&out("E13.05", &de("c").with_n("b", 12.0).with_n("h", 2.0)), "mu3"), 1.86667, 1e-5);
    near(v(&out("E13.10", &de("r").with_n("v", 0.4).with_n("n_years", 100.0)), "factor"), 1.10768, 1e-5);
    near(v(&out("E13.08", &de("f").with_n("s", 1000.0).with_n("b", 4.0).with_n("alpha", 30.0)), "F_s"), 2000.0, 1e-9);
    near(v(&out("E13.12", &de("w").with_n("c_e", 0.8).with_n("c_t", 1.0).with_n("mu", 0.8).with_n("s_k", 850.0)), "s"), 544.0, 1e-9);
    assert!(evaluate_outcomes("E13.02", &de("x").with_s("zone", "7").with_n("altitude", 0.0)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn e14_worked_examples() {
    for (zone, q) in [(1.0, 316.4), (2.0, 390.6), (3.0, 472.7), (4.0, 562.5)] { near(v(&out("E14.01", &de("b").with_n("zone", zone)), "q_b"), q, 2e-4); }
    for (z, q) in [(5.0, 585.9), (10.0, 664.1), (30.0, 997.1), (100.0, 1425.5)] { near(v(&out("E14.02", &de("p").with_n("zone", 2.0).with_n("z", z)), "q_p"), q, 2e-4); }
    let o = out("E14.05", &en("3").with_n("z", 10.0).with_n("terrain", 3.0).with_n("v_b", 25.0));
    near(v(&o, "k_r"), 0.215389, 1e-5);
    near(v(&o, "c_r"), 0.755275, 1e-5);
    near(v(&o, "v_m"), 18.8819, 1e-5);
    near(v(&o, "I_v"), 0.28518, 1e-4);
    near(v(&o, "q_p"), 667.652, 1e-5);
    let o = out("E14.05", &en("4").with_n("z", 10.0).with_n("terrain", 2.0).with_n("v_b", 25.0));
    near(v(&o, "c_r"), 1.00668, 1e-5);
    near(v(&o, "q_p"), 918.863, 1e-5);
    for (t, q) in [(1.0, 1081.45), (0.0, 1165.83), (4.0, 459.442)] { near(v(&out("E14.05", &en("5").with_n("z", 10.0).with_n("terrain", t).with_n("v_b", 25.0)), "q_p"), q, 1e-5); }
    near(v(&out("E14.03", &en("6").with_n("phi", 0.15).with_n("s", 1.0)), "c_o"), 1.3, 1e-12);
    near(v(&out("E14.05", &en("6b").with_n("z", 10.0).with_n("terrain", 2.0).with_n("v_b", 25.0).with_n("c_o", 1.3)), "q_p"), 1348.91, 1e-5);
    let o = out("E14.07", &en("7").with_n("b", 30.0).with_n("h", 12.0).with_n("d", 20.0));
    near(v(&o, "e"), 24.0, 1e-12);
    near(v(&o, "A"), 4.8, 1e-12);
    near(v(&o, "B"), 15.2, 1e-12);
    let wall = |zone: &str| out("E14.08", &en("8").with_s("zone", zone).with_n("h_over_d", 0.6));
    near(v(&wall("D"), "c_pe10_0"), 0.746667, 1e-5);
    near(v(&wall("E"), "c_pe10_0"), -0.393333, 1e-5);
    let qp12 = v(&out("E14.05", &en("8b").with_n("z", 12.0).with_n("terrain", 2.0).with_n("v_b", 25.0)), "q_p");
    near(qp12, 964.573, 1e-5);
    near(qp12 * v(&wall("D"), "c_pe10_0"), 720.215, 1e-5);
    let flat = out("E14.08", &en("9").with_s("zone", "F").with_s("roof_type", "flat").with_n("area", 3.0).with_n("e", 24.0).with_n("b", 30.0).with_n("d", 20.0));
    near(v(&flat, "area_f"), 14.4, 1e-9);
    near(v(&flat, "area_g"), 43.2, 1e-9);
    near(v(&flat, "area_h"), 288.0, 1e-9);
    near(v(&flat, "area_i"), 240.0, 1e-9);
    near(v(&flat, "c_pe_area_0"), -2.16602, 1e-5);
    near(v(&out("E14.09", &en("10").with_n("q_p", 1000.0).with_n("c_pe", -1.2).with_n("dominant_ratio", 2.0)), "c_pi_dominant"), -0.9, 1e-12);
    near(v(&out("E14.09", &en("10b").with_n("q_p", 1000.0).with_n("c_pe", -1.2).with_n("dominant_ratio", 3.0)), "c_pi_dominant"), -1.08, 1e-12);
    near(v(&out("E14.09", &en("10c").with_n("q_p", 1000.0).with_n("c_pe", -1.2).with_n("dominant_ratio", 2.5)), "c_pi_dominant"), -0.99, 1e-12);
    let o = out("E14.12", &en("12").with_n("h", 40.0).with_n("b", 30.0).with_n("terrain", 3.0).with_n("v_b", 25.0).with_n("delta", 0.1));
    near(v(&o, "z_s"), 24.0, 1e-12);
    near(v(&o, "L"), 82.3388, 1e-5);
    near(v(&o, "B2"), 0.551724, 1e-5);
    near(v(&o, "n_1"), 1.15, 1e-12);
    near(v(&o, "S_L"), 0.0539174, 1e-5);
    near(v(&o, "R2"), 0.0385584, 1e-4);
    near(v(&o, "nu"), 0.293919, 1e-4);
    near(v(&o, "k_p"), 3.40290, 1e-4);
    near(v(&o, "cscd"), 0.844394, 1e-4);
    for (p, f) in [(0.2, 0.854501), (0.1, 0.90248), (0.5, 0.776433)] { near(v(&out("E14.06", &en("13").with_n("p", p)), "c_prob"), f, 1e-5); }
    let o = out("E14.14", &en("14").with_n("b", 0.5).with_n("n", 1.5).with_n("delta_s", 0.05).with_n("m_e", 300.0));
    near(v(&o, "v_crit"), 4.16667, 1e-5);
    near(v(&o, "Sc"), 96.0, 1e-9);
    near(v(&out("E14.13", &en("15").with_n("v_b", 25.0).with_n("c_e", 2.0).with_n("c_fx", 1.3).with_n("a_ref", 120.0)), "F_w"), 121_875.0, 1e-9);
    near(v(&out("E14.16", &en("16").with_n("q_p", qp12).with_n("b", 0.5)), "Re"), 1.3095e6, 1e-4);
    assert!(evaluate_outcomes("E14.01", &de("x").with_n("zone", 0.0)).is_err());
    assert!(evaluate_outcomes("E14.05", &en("x").with_n("z", 10.0).with_n("terrain", 7.0).with_n("v_b", 25.0)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn e15_e16_e17_worked_examples() {
    for (ty, tmin, tmax, con, exp, dl_con, dl_exp) in [(1.0, -27.0, 53.0, 37.0, 43.0, 0.0444, 0.0516), (2.0, -20.0, 41.0, 30.0, 31.0, 0.036, 0.0372), (3.0, -16.0, 39.0, 26.0, 29.0, 0.026, 0.029)] {
        let o = out("E15.01", &de("t").with_n("deck_type", ty).with_n("t_min_c", -24.0).with_n("t_max_c", 37.0));
        near(v(&o, "T_e_min") - 273.15, tmin, 1e-9);
        near(v(&o, "T_e_max") - 273.15, tmax, 1e-9);
        near(v(&o, "dT_con"), con, 1e-9);
        near(v(&o, "dT_exp"), exp, 1e-9);
        near(v(&o, "dL_con"), dl_con, 1e-9);
        near(v(&o, "dL_exp"), dl_exp, 1e-9);
    }
    let o = out("E15.02", &de("g").with_s("deck", "steel").with_n("k_sur_heat", 0.7).with_n("k_sur_cool", 1.1));
    near(v(&o, "dT_M_heat"), 12.6, 1e-9);
    near(v(&o, "dT_M_cool"), 14.3, 1e-9);
    near(v(&out("E15.03", &de("s").with_n("e", 210e9).with_n("alpha", 12e-6).with_n("dt", 40.0)), "sigma"), 100.8e6, 1e-9);
    near(v(&out("E15.03", &de("s2").with_n("e", 33e9).with_n("alpha", 10e-6).with_n("dt", 30.0)), "sigma"), 9.9e6, 1e-9);
    let o = out("E15.04", &de("c").with_n("dt_m", 18.0).with_n("dt_n", 43.0));
    near(v(&o, "m_leading"), 33.05, 1e-9);
    near(v(&o, "n_leading"), 56.5, 1e-9);
    near(v(&out("E15.05", &de("b").with_n("e", 210e9).with_n("i", 1e-3).with_n("alpha", 12e-6).with_n("dt_m", 18.0).with_n("h", 1.0)), "M"), 45_360.0, 1e-9);
    near(v(&out("E15.06", &de("j").with_n("alpha", 12e-6).with_n("dt", 43.0).with_n("length", 100.0)), "movement"), 0.0516, 1e-9);
    near(v(&out("E15.07", &de("a").with_n("t_min_c", -24.0).with_n("t_max_c", 37.0).with_n("altitude", 800.0)), "t_min") - 273.15, -28.0, 1e-9);
    near(v(&out("E15.08", &de("k").with_n("t_min_c", -24.0).with_n("t_max_c", 37.0)), "t_0"), 283.15, 1e-12);

    near(v(&out("E16.02", &de("q").with_n("area", 9.0)), "Q_ca"), 9000.0, 1e-9);
    let o = out("E16.02", &de("f").with_n("area", 9.0).with_n("gamma", 25_000.0).with_n("h", 0.6));
    near(v(&o, "p_fresh"), 15_000.0, 1e-9);
    near(v(&o, "p_total"), 15_750.0, 1e-9);
    let o = out("E16.03", &de("w").with_n("p", 0.2).with_n("v_b", 25.0));
    near(v(&o, "v_b"), 21.3625, 1e-4);
    near(v(&o, "q_b"), 285.223, 1e-4);
    near(v(&out("E16.06", &de("s").with_v("q_ci", &[1000.0, 200.0])), "Q_c_sum"), 1200.0, 1e-12);
    near(v(&out("E16.01", &de("r").with_n("duration_days", 2.0)), "snow_factor"), 0.458916, 1e-5);

    let o = out("E17.01", &de("i").with_s("road", "urban"));
    near(v(&o, "F_dx"), 750_000.0, 1e-12);
    near(v(&o, "F_dy"), 375_000.0, 1e-12);
    let o = out("E17.06", &de("t").with_n("g_k", 5000.0).with_n("q_k", 3000.0).with_n("psi", 0.3).with_n("s", 6.0).with_n("l", 7.2).with_n("floor_area", 400.0).with_n("key_area", 18.0));
    near(v(&o, "T_p"), 101_952.0, 1e-6);
    near(v(&o, "T_i"), 203_904.0, 1e-6);
    near(v(&o, "T_p_cap"), 75_000.0, 1e-12);
    near(v(&o, "removal_area"), 60.0, 1e-12);
    near(v(&o, "key_element"), 612_000.0, 1e-12);
    let o = out("E17.08", &de("d").with_n("mass", 1500.0).with_n("speed", 15.0));
    near(v(&o, "F_el"), 318_198.0, 1e-5);
    near(v(&o, "energy"), 168_750.0, 1e-9);
    let o = out("E17.09", &de("x").with_n("p_v", 5.0).with_n("av_over_v", 0.04));
    near(v(&o, "p_d1"), 8000.0, 1e-12);
    near(v(&o, "p_d2"), 30_500.0, 1e-12);
    near(v(&out("E2.11", &de("c").with_s("member", "pier")), "F_dx"), 1_000_000.0, 1e-12);
}

#[semio_framework_async_macros::async_test]
async fn e2_e3_e4_worked_examples() {
    for (w, n, wl, rem) in [(5.0, 1.0, 3.0, 2.0), (5.5, 2.0, 2.75, 0.0), (6.0, 2.0, 3.0, 0.0), (10.5, 3.0, 3.0, 1.5)] {
        let o = out("E2.01", &de("l").with_n("width", w));
        near(v(&o, "n_lanes"), n, 1e-12);
        near(v(&o, "w_lane"), wl, 1e-12);
        assert!((v(&o, "w_rem") - rem).abs() < 1e-12);
    }
    let o = out("E2.02", &de("m").with_n("width", 10.5));
    near(v(&o, "tandems"), 1_200_000.0, 1e-12);
    near(v(&o, "udl_1"), 27_000.0, 1e-12);
    near(v(&o, "udl_2"), 7500.0, 1e-12);
    near(v(&o, "udl_rest"), 3750.0, 1e-12);
    near(v(&out("E2.07", &de("b").with_n("loaded_length", 40.0)), "Q_lk"), 468_000.0, 1e-9);
    near(v(&out("E2.08", &de("c").with_n("q_v", 1_200_000.0).with_n("radius", 300.0)), "Q_tk"), 160_000.0, 1e-9);
    near(v(&out("E2.08", &de("c2").with_n("q_v", 1_200_000.0).with_n("radius", 150.0)), "Q_tk"), 240_000.0, 1e-9);
    for (l, q) in [(50.0, 3500.0), (5.0, 5000.0), (300.0, 2500.0)] { near(v(&out("E2.12", &de("f").with_n("span", l).with_n("width", 1.0)), "q_fk"), q, 1e-6); }
    let o = out("E2.16", &de("d").with_n("l_phi", 20.0));
    near(v(&o, "phi_2"), 1.15707, 1e-5);
    near(v(&o, "phi_3"), 1.2356, 1e-4);
    let o = out("E2.17", &de("r").with_n("v_kmh", 160.0).with_n("radius", 1000.0).with_n("l_f", 30.0).with_n("q_vk", 1_000_000.0));
    near(v(&o, "f"), 0.811241, 1e-5);
    near(v(&o, "Q_tk"), 163_526.0, 1e-4);
    let o = out("E2.17", &de("a").with_n("v_kmh", 160.0).with_n("radius", 1000.0).with_n("l_f", 30.0).with_n("q_vk", 1_000_000.0).with_n("loaded_length", 30.0));
    near(v(&o, "traction"), 990_000.0, 1e-12);
    near(v(&o, "braking"), 600_000.0, 1e-12);
    near(v(&out("E2.15", &de("s").with_s("model", "sw0")), "total"), 1_995_000.0, 1e-12);
    near(v(&out("E2.14", &de("n")), "udl"), 80_000.0, 1e-12);
    near(v(&out("E2.03", &de("w")), "wheel_pressure"), 952_381.0, 1e-6);
    near(v(&out("E2.06", &de("w2").with_n("a", 0.4).with_n("b", 0.4).with_n("load", 150_000.0)), "pressure"), 937_500.0, 1e-12);

    let o = out("E3.01", &de("p").with_s("hoist_class", "HC2").with_n("v_h", 0.25).with_n("dm_over_m", 0.5).with_n("beta_3", 0.5));
    near(v(&o, "phi_2"), 1.185, 1e-12);
    near(v(&o, "phi_3"), 0.25, 1e-12);
    for (class, phi) in [("HC1", 1.0925), ("HC3", 1.2775), ("HC4", 1.37)] { near(v(&out("E3.01", &de("p").with_s("hoist_class", class).with_n("v_h", 0.25)), "phi_2"), phi, 1e-12); }
    near(v(&out("E3.02", &de("h").with_n("q_c", 0.0).with_n("q_h", 100_000.0).with_n("wheels", 1.0).with_s("hoist_class", "HC2").with_n("v_h", 0.25)), "Q_r_max"), 118_500.0, 1e-12);
    near(v(&out("E3.02", &de("g1").with_n("q_c", 300_000.0).with_n("q_h", 100_000.0).with_n("wheels", 2.0).with_s("hoist_class", "HC2").with_n("v_h", 0.25)), "Q_r_max"), 224_250.0, 1e-9);
    let o = out("E3.03", &de("k").with_n("sum_q_r", 400_000.0).with_n("runway_beams", 2.0).with_n("skew_rad", 0.015).with_n("phi_7", 1.25).with_n("v_1", 0.7).with_n("m_c", 20_000.0).with_n("s_b", 2.0e6).with_n("xi", 0.1).with_n("m_drive", 500_000.0).with_n("a", 2.0));
    near(v(&o, "K"), 80_000.0, 1e-12);
    near(v(&o, "H_L"), 60_000.0, 1e-12);
    near(v(&o, "f_skew"), 0.292945, 1e-5);
    near(v(&o, "H_B"), 175_000.0, 1e-9);
    near(v(&o, "H_T"), 25_000.0, 1e-12);

    for (z, y, phf, pwf, pvf) in [(2.0, 0.254226, 8580.14, 3432.06, 15600.3), (10.0, 0.769307, 25_964.1, 10_385.6, 47_207.5), (20.0, 0.946781, 31_953.8, 12_781.5, 58_097.9)] {
        let o = out("E4.03", &en("j").with_n("gamma", 9000.0).with_n("phi", 30.0).with_n("mu", 0.4).with_n("dc", 6.0).with_n("z", z));
        near(v(&o, "K"), 0.55, 1e-9);
        near(v(&o, "z_o"), 6.81818, 1e-5);
        near(v(&o, "p_ho"), 33_750.0, 1e-9);
        near(v(&o, "Y_J"), y, 1e-5);
        near(v(&o, "p_hf"), phf, 1e-5);
        near(v(&o, "p_wf"), pwf, 1e-5);
        near(v(&o, "p_vf"), pvf, 1e-5);
    }
    let o = out("E4.03", &en("e").with_n("gamma", 9000.0).with_n("phi", 30.0).with_n("mu", 0.4).with_n("dc", 6.0).with_n("z", 10.0));
    let a_over_u = 1.5;
    near(v(&o, "N_wf"), 64_188.8, 1e-5);
    near(v(&o, "N_wf"), 9000.0 * a_over_u * 10.0 - a_over_u * v(&o, "p_vf"), 1e-5);
    let o = out("E4.01", &en("c").with_n("capacity_t", 100.0).with_n("hc", 12.0).with_n("dc", 6.0).with_n("gamma", 9000.0));
    near(v(&o, "aac"), 2.0, 1e-12);
    near(v(&o, "slenderness"), 2.0, 1e-12);
    near(v(&o, "mass"), 311_277.0, 1e-4);
    near(v(&out("E4.01", &en("c2").with_n("capacity_t", 10_001.0)), "aac"), 3.0, 1e-12);
    near(v(&out("E4.01", &en("c3").with_n("capacity_t", 10_000.0)), "aac"), 2.0, 1e-12);
    near(v(&out("E4.01", &en("c4").with_n("capacity_t", 99.9)), "aac"), 1.0, 1e-12);
    let o = out("E4.02", &en("b").with_n("phi", 30.0));
    near(v(&o, "K_a"), 1.0 / 3.0, 1e-9);
    near(v(&o, "mu"), 0.414214, 1e-5);
    near(v(&out("E4.08", &en("t").with_n("gamma", 10_000.0).with_n("z", 5.0)), "p"), 50_000.0, 1e-12);
    assert!(evaluate_outcomes("E4.03", &en("x").with_n("gamma", 9000.0).with_n("z", 1.0)).is_err());
}

#[semio_framework_async_macros::async_test]
async fn validity_ranges_warn_and_limits_fail_with_remedies() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    let results = evaluate_case("E12.02", &de("v").with_n("a_f", 50.0).with_n("a_t", 184.84).with_n("a_v", 0.5).with_n("h_eq", 2.0).with_n("b", 1160.0).with_n("q_fd_mj", 336.0));
    assert!(results.iter().any(|r| r.status == CheckStatus::Warning), "tiny opening must leave the validity range");
    assert!(results.iter().all(|r| r.status != CheckStatus::Fail));
    let results = evaluate_case("E12.04", &de("r").with_n("q_fd_mj", 504.0).with_n("height", 3.0).with_n("alpha_v", 0.1).with_n("k_b", 0.07).with_n("resistance_min", 30.0));
    let fail = results.iter().find(|r| r.status == CheckStatus::Fail).expect("58.6 min exceeds R30");
    assert!(!fail.remedies.is_empty());
    assert!(fail.utilization > 1.0);
}

#[semio_framework_async_macros::async_test]
async fn informative_rows_pass_with_zero_utilisation_and_keep_their_value() {
    use semio_s_plugin_norm_compliance::CheckStatus;
    let results = evaluate_case("E11.01", &en("i").with_s("material", "reinforced_concrete").with_n("thickness", 0.2));
    assert!(!results.is_empty());
    for r in &results { assert_eq!(r.status, CheckStatus::Pass, "{}", r.id); assert!(r.utilization.abs() < 1e-12, "{}", r.id); }
    assert!((results[0].computed.value - 5000.0).abs() < 1e-9);
    let negative = evaluate_case("E14.08", &en("n").with_s("zone", "D").with_n("h_over_d", 0.6));
    assert!(negative.iter().all(|r| r.status == CheckStatus::Pass));
    let suction = evaluate_case("E14.08", &en("s").with_s("zone", "A").with_n("h_over_d", 1.0));
    assert!(suction.iter().all(|r| r.status == CheckStatus::Pass && r.utilization.abs() < 1e-12), "negative information must not fail");
}

#[semio_framework_async_macros::async_test]
async fn lm1_lanes_beyond_three_carry_udl_and_test_loads_follow_practice() {
    let o = out("E2.02", &de("w").with_n("width", 14.0));
    near(v(&o, "udl_4"), 7500.0, 1e-12);
    near(v(&o, "udl_rest"), 5000.0, 1e-12);
    let t = out("E3.04", &de("t").with_n("q_h", 100_000.0));
    near(v(&t, "static"), 125_000.0, 1e-12);
    near(v(&t, "dynamic"), 110_000.0, 1e-12);
    let e = out("E4.05", &en("p").with_n("gamma", 9000.0).with_n("phi", 30.0).with_n("mu", 0.4).with_n("dc", 6.0).with_n("z", 20.0));
    near(v(&e, "p_pf"), 0.2 * 36_994.25, 1e-5);
}
