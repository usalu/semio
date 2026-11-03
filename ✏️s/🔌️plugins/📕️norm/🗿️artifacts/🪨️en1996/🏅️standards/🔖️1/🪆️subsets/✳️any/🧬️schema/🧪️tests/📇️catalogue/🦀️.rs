//! 📇️ Catalogue tests: the worked examples X1 … X12 of the target specification, remedy and basis laws, and the superseded-form scan.

use crate::checks::masonry::{evaluate_case, evaluate_outcomes, registry, Case, Mode};
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckStatus};

pub(super) fn value(entry: &str, case: &Case, key: &str) -> f64 {
    let outcomes = evaluate_outcomes(entry, case).unwrap_or_else(|e| panic!("{entry}: {e}"));
    let o = outcomes.iter().find(|o| o.key == key).unwrap_or_else(|| panic!("{entry} has no outcome {key}"));
    o.capacity
}

fn close(got: f64, want: f64, rel: f64, what: &str) {
    assert!((got - want).abs() <= rel * want.abs().max(1e-12), "{what}: got {got}, want {want}");
}

fn clay_g1(annex: AnnexChoice) -> Case {
    Case::new("x1", annex).with_n("fb_pa", 20e6).with_n("fm_pa", 10e6).with_s("material", "clay").with_n("gamma_m", 1.5)
}

#[test]
fn x1_fk_clay_group1_gp_m10() {
    let c = clay_g1(AnnexChoice::En);
    close(value("M-02", &c, "fk"), 8.935e6, 1e-3, "f_k");
    close(value("M-02", &c, "fd"), 5.957e6, 1e-3, "f_d");
    let m06 = Case::new("x1e", AnnexChoice::En).with_n("fk_pa", 8.935e6).with_s("material", "clay");
    close(value("M-06", &m06, "e"), 8935e6, 1e-9, "E");
    close(value("M-06", &m06, "e-lt"), 3574e6, 1e-3, "E_lt");
}

#[test]
fn x2_effective_height_and_restraint_factor() {
    let base = Case::new("x2", AnnexChoice::En).with_n("h_m", 2.75).with_n("t_m", 0.365).with_n("rho2", 0.75);
    close(value("M-10", &base.clone().with_n("sides", 2.0), "hef"), 2.0625, 1e-9, "h_ef two sides");
    close(value("M-10", &base.clone().with_n("sides", 3.0).with_n("l_m", 3.0), "rho-n"), 0.7126, 1e-3, "rho_3");
    close(value("M-10", &base.with_n("sides", 4.0).with_n("l_m", 3.0), "rho-n"), 0.5093, 1e-3, "rho_4");
}

#[test]
fn x3_x4_phi_at_top_and_mid_height() {
    let top = Case::new("x3", AnnexChoice::En)
        .with_n("t_m", 0.365).with_n("hef_m", 2.0625).with_n("fd_pa", 5.957e6)
        .with_n("n_top_n", 200e3).with_n("m_top_nm", 0.02 * 200e3).with_n("n_bot_n", 200e3).with_n("n_mid_n", 200e3);
    close(value("M-12", &top, "n-top"), 1881e3, 2e-3, "N_Rd,top per metre");
    close(value("M-13", &top, "phi-top"), 0.8653, 1e-3, "Phi_top");
    let mid = Case::new("x4", AnnexChoice::En)
        .with_n("t_m", 0.365).with_n("hef_m", 2.0625).with_n("fd_pa", 5.957e6)
        .with_n("n_mid_n", 200e3).with_n("m_mid_nm", 0.0125 * 200e3).with_n("phi_inf", 1.5).with_n("k_e", 1000.0);
    close(value("M-13", &mid, "phi-m"), 0.88579, 2e-4, "Phi_m, e_mk = e_m + e_k = 0.018422 (the spec writes 0.05 t)");
}

#[test]
fn x5_concentrated_load() {
    let c = Case::new("x5", AnnexChoice::En).with_n("ab_m2", 0.073).with_n("a1_m", 0.5).with_n("hc_m", 2.75).with_n("t_m", 0.365).with_n("lefm_m", 1.0).with_n("fd_pa", 5.957e6).with_n("ned_c_n", 500e3);
    close(value("M-14", &c, "beta"), 1.3409, 1e-3, "beta");
    close(value("M-14", &c, "n-rdc"), 583.1e3, 2e-3, "N_Rdc");
}

#[test]
fn x6_shear_wall() {
    let c = Case::new("x6", AnnexChoice::En)
        .with_n("l_m", 2.0).with_n("t_m", 0.365).with_n("n_ed_n", 0.8e6 * 2.0 * 0.365).with_n("v_ed_n", 200e3)
        .with_n("fb_pa", 20e6).with_n("fm_pa", 10e6).with_n("gamma_m", 1.5);
    close(value("M-20", &c, "v-rd"), 301.7e3, 2e-3, "V_Rd");
}

#[test]
fn x7_one_way_wall_with_and_without_apparent_strength() {
    let c = Case::new("x7", AnnexChoice::En).with_n("t_m", 0.175).with_n("h_m", 2.75).with_n("w_ed_pa", 960.0).with_n("fxk1_pa", 0.2e6).with_n("gamma_m", 1.5);
    close(value("M-30", &c, "m-rd"), 680.6, 2e-3, "M_Rd");
    close(value("M-30", &c.with_n("sigma_d_pa", 0.2e6), "m-rd"), 1700.0, 2e-2, "M_Rd with sigma_d");
}

#[test]
fn x8_isotropic_square_panel_has_alpha_one_twentyfourth() {
    let c = Case::new("x8", AnnexChoice::En).with_n("t_m", 0.1).with_n("h_m", 1.0).with_n("l_m", 1.0).with_n("w_ed_pa", 1.0).with_n("fxk1_pa", 1.5e6).with_n("fxk2_pa", 1.5e6).with_n("gamma_m", 1.0);
    close(value("M-31", &c, "alpha1"), 1.0 / 24.0, 1e-4, "alpha");
}

#[test]
fn x9_reinforced_flexure() {
    let c = Case::new("x9", AnnexChoice::En).with_n("b_m", 1.0).with_n("d_m", 0.3).with_n("as_m2", 600e-6).with_n("fyd_pa", 435e6).with_n("fd_pa", 5.957e6).with_n("m_ed_nm", 50e3);
    close(value("M-40", &c, "z"), 0.2781, 1e-3, "z");
    close(value("M-40", &c, "m-rd"), 72.58e3, 1e-3, "M_Rd");
}

#[test]
fn x10_simplified_vertical_resistance() {
    let c = Case::new("x10", AnnexChoice::En).with_n("t_m", 0.24).with_n("hef_m", 2.0625).with_n("fk_pa", 5.654e6).with_n("gamma_m", 1.5).with_n("n_ed_n", 500e3);
    close(value("M-81", &c, "phi-s"), 0.7688, 1e-3, "Phi_s");
    close(value("M-81", &c, "n-rd"), 695.4e3, 2e-3, "N_Rd");
}

#[test]
fn x11_basement_minimum_normal_force() {
    let c = Case::new("x11", AnnexChoice::De).with_n("h_m", 2.5).with_n("t_m", 0.365).with_n("he_m", 2.0).with_n("rho_e_n_m3", 18e3).with_n("n_min_n", 30e3).with_n("n_max_n", 200e3).with_n("fd_pa", 3e6);
    let out = evaluate_outcomes("M-34", &c).expect("outcomes");
    let n_min = out.iter().find(|o| o.key == "n-min").expect("n-min");
    close(n_min.demand, 24.66e3, 1e-3, "N_min,req");
}

#[test]
fn x12_tabulated_fire_resistance() {
    let c = Case::new("x12", AnnexChoice::De).with_n("t_m", 0.365).with_n("rei", 90.0).with_s("material", "clay").with_n("mu_fi", 0.7);
    close(value("M-90", &c, "t-min"), 0.150, 1e-9, "t_min, base row of clay R90");
    assert!(evaluate_case("M-90", &c).iter().all(|r| r.status == CheckStatus::Pass));
    close(value("M-90", &c, "achievable-rei"), 240.0, 1e-9, "achievable REI");
}

fn parse_inputs(annex: AnnexChoice, inputs: &str) -> Case {
    let mut case = Case::new("feature", annex);
    for part in inputs.split(';').filter(|p| !p.trim().is_empty()) {
        if let Some((k, v)) = part.split_once('=') {
            case = case.with_n(k.trim(), v.trim().parse::<f64>().unwrap_or_else(|e| panic!("{part}: {e}")));
        } else if let Some((k, v)) = part.split_once(':') {
            case = case.with_s(k.trim(), v.trim());
        }
    }
    case
}

#[test]
fn feature_file_examples_hold() {
    let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let feature = loop {
        let candidate = dir.join("🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/📇️en1996-catalogue/🥒️.feature");
        if candidate.exists() {
            break candidate;
        }
        assert!(dir.pop(), "feature file not found");
    };
    let text = std::fs::read_to_string(feature).expect("feature");
    let mut rows = 0;
    for line in text.lines().filter(|l| l.trim_start().starts_with('|')) {
        let cells: Vec<&str> = line.trim().trim_matches('|').split('|').map(str::trim).collect();
        if cells.len() != 7 || cells[0] == "id" {
            continue;
        }
        let annex = if cells[2] == "de" { AnnexChoice::De } else { AnnexChoice::En };
        let case = parse_inputs(annex, cells[3]);
        let want: f64 = cells[5].parse().expect("expected");
        let tol: f64 = cells[6].parse().expect("tolerance");
        close(value(cells[1], &case, cells[4]), want, tol, cells[0]);
        rows += 1;
    }
    assert!(rows >= 15, "only {rows} feature rows evaluated");
}

#[test]
fn the_annex_e_coefficient_of_m_31_rests_on_the_unverified_row_of_its_class() {
    let case = Case::new("annex-e", AnnexChoice::En).with_n("t_m", 0.1).with_n("h_m", 1.0).with_n("l_m", 1.0).with_n("w_ed_pa", 1000.0).with_n("fxk1_pa", 1.5e6).with_n("fxk2_pa", 1.5e6).with_n("gamma_m", 1.0).with_s("edges", "XSSS");
    let results = evaluate_case("M-31", &case);
    let cell = results.iter().find(|c| c.id.contains("annex-e") && !c.id.contains("m-ed")).expect("the Annex E coefficient is reported");
    assert!(cell.ndps.iter().any(|id| id == "ndp.en1996.annex-e.sxss"), "{:?}", cell.ndps);
}

#[test]
fn review_regressions_thin_layer_k_f_vk0_f_vlt_and_shear_length() {
    use crate::checks::masonry::{fk_factors, f_vk0_mpa, f_vlt_mpa, psi_imposed, rho_n};
    use semio_s_artifact_norm_en1996_model::{MortarClass, MortarType, UnitGroup, UnitMaterial};
    close(fk_factors(AnnexChoice::En, UnitMaterial::CalciumSilicate, UnitGroup::Group2, MortarType::ThinLayer).k, 0.65, 1e-12, "K calcium silicate G2 thin layer");
    close(f_vk0_mpa(AnnexChoice::En, UnitMaterial::Clay, MortarType::GeneralPurpose, MortarClass::M10), 0.30, 1e-12, "f_vk0 clay M10");
    close(f_vk0_mpa(AnnexChoice::En, UnitMaterial::CalciumSilicate, MortarType::GeneralPurpose, MortarClass::M10), 0.20, 1e-12, "f_vk0 calcium silicate M10");
    close(f_vk0_mpa(AnnexChoice::En, UnitMaterial::CalciumSilicate, MortarType::GeneralPurpose, MortarClass::M5), 0.15, 1e-12, "f_vk0 calcium silicate M5");
    close(f_vk0_mpa(AnnexChoice::En, UnitMaterial::CalciumSilicate, MortarType::ThinLayer, MortarClass::M10), 0.40, 1e-12, "f_vk0 calcium silicate thin layer");
    let solid = f_vlt_mpa(AnnexChoice::De, UnitGroup::Group1, 20.0, 0.8);
    let hollow = f_vlt_mpa(AnnexChoice::De, UnitGroup::Group4, 20.0, 0.8);
    close(hollow, 0.45 * 0.4 * (1.0 + 0.8 / 0.4_f64).sqrt(), 1e-9, "hollow f_vlt");
    assert!(solid > hollow, "solid units carry the larger tensile cap");
    close(rho_n(1, 0.75, 2.75, 5.0), 2.0, 1e-12, "free-standing wall h_ef = 2 h");
    close(psi_imposed(AnnexChoice::De, "H").expect("category H").1, 0.0, 1e-12, "psi_1 of category H");
}

#[test]
fn the_flange_overhang_is_capped_by_the_flange_length_of_every_flange() {
    let c = Case::new("flange", AnnexChoice::En)
        .with_n("l_m", 2.0).with_n("t_m", 0.24).with_n("n_ed_n", 584e3).with_n("v_ed_n", 200e3).with_n("fb_pa", 20e6).with_n("fm_pa", 10e6).with_s("material", "clay").with_n("gamma_m", 1.5)
        .with_n("flange_t_m", 0.24).with_n("web_spacing_m", 4.0).with_n("height_total_m", 8.0).with_n("flanges", 2.0).with_v("flange_len_m", &[0.5, 2.0]);
    close(value("M-20", &c, "flange-overhang"), 0.97, 1e-9, "mean overhang of 0.5 m and min(1.44, 2.0) m");
    close(value("M-20", &c, "flanged-area"), 0.9456, 1e-9, "flanged area");
    close(value("M-20", &c, "flanged-i"), 0.3452605856, 1e-8, "flanged second moment");
}

#[test]
fn review_regression_shear_wall_uses_the_compressed_length() {
    let c = Case::new("b6", AnnexChoice::En)
        .with_n("l_m", 3.0).with_n("t_m", 0.365).with_n("n_ed_n", 300e3).with_n("m_ed_nm", 275e3).with_n("v_ed_n", 100e3)
        .with_n("fb_pa", 20e6).with_n("fm_pa", 10e6).with_s("material", "calcium_silicate").with_n("gamma_m", 1.5);
    close(value("M-20", &c, "v-rd"), 165.2e3, 3e-3, "V_Rd with l_c = 1.75 m");
    close(value("M-20", &c, "lc"), 1.75, 1e-9, "compressed length");
}

#[test]
fn na_de_rules_do_not_leak_into_the_en_annex() {
    use crate::checks::masonry::{phi_2_slab_span, eccentricity_from_slab_rotation_m};
    use semio_s_artifact_norm_en1996_model::En1996Snapshot;
    let doc = En1996Snapshot::compliant_clay_wall();
    close(phi_2_slab_span(AnnexChoice::En, 5.5), 1.0, 1e-12, "Phi_2 in EN");
    assert!(phi_2_slab_span(AnnexChoice::De, 5.5) < 1.0);
    close(eccentricity_from_slab_rotation_m(AnnexChoice::En, &doc.walls[0], &doc.walls[0].load_cases[0], 1e5), 0.0, 1e-12, "e_theta in EN");
}

#[test]
fn every_target_catalogue_id_is_registered() {
    let ids: Vec<&str> = registry().iter().map(|e| e.id).collect();
    for want in ["M-01", "M-02", "M-03", "M-04", "M-05", "M-06", "M-07", "M-10", "M-11", "M-12", "M-13", "M-14", "M-15", "M-16", "M-20", "M-21", "M-30", "M-31", "M-32", "M-33", "M-34", "M-40", "M-41", "M-42", "M-43", "M-44", "M-50", "M-51", "M-52", "M-60", "M-61", "M-70", "M-80", "M-81", "M-82", "M-83", "M-90", "M-91", "M-95"] {
        assert!(ids.contains(&want), "missing catalogue entry {want}");
    }
}

#[test]
fn every_basis_id_is_on_the_unverified_list() {
    for (id, case) in fixture_cases() {
        for o in evaluate_outcomes(id, &case).unwrap_or_else(|e| panic!("{id}/{}: {e}", case.id)) {
            for b in &o.basis {
                assert!(semio_s_plugin_norm_compliance::ndp(b).is_some(), "{id}: basis {b} is not registered");
            }
        }
    }
}

#[test]
fn failing_outcomes_carry_a_remedy_that_flips_them() {
    let mut flipped = 0;
    for (id, case) in fixture_cases() {
        for o in evaluate_outcomes(id, &case).unwrap() {
            let Some(fix) = &o.fix else { continue };
            if o.mode != Mode::Limit || o.utilization() <= 1.0 {
                continue;
            }
            let mut fixed = case.clone();
            fixed.num.insert(fix.key.into(), fix.required);
            let after = evaluate_outcomes(id, &fixed).unwrap();
            let again = after.iter().find(|x| x.key == o.key).expect("same outcome");
            assert!(again.utilization() <= 1.0 + 1e-9, "{id}/{}: remedy {}={} leaves utilisation {}", o.key, fix.key, fix.required, again.utilization());
            flipped += 1;
        }
    }
    assert!(flipped >= 6, "expected failing fixture outcomes with remedies, flipped {flipped}");
}

#[test]
fn every_case_evaluates_to_check_results_without_panicking() {
    for (id, case) in fixture_cases() {
        let results = evaluate_case(id, &case);
        assert!(!results.is_empty(), "{id}");
        for r in &results {
            assert!(r.id.starts_with("en1996."), "{}", r.id);
            if r.status == CheckStatus::Fail {
                assert!(!r.remedies.is_empty(), "{} fails without remedy", r.id);
            }
        }
    }
}

#[test]
fn missing_input_is_not_applicable_not_a_placeholder() {
    let results = evaluate_case("M-14", &Case::new("empty", AnnexChoice::De));
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, CheckStatus::NotApplicable);
}

#[test]
fn wall_checks_report_computed_ratios_not_placeholders() {
    use crate::checks::masonry::evaluate_building;
    use semio_s_artifact_norm_en1996_model::En1996Snapshot;
    let doc = En1996Snapshot::compliant_clay_wall();
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
    let find = |needle: &str| report.checks.iter().find(|c| c.id.contains(needle)).unwrap_or_else(|| panic!("missing {needle}"));
    close(find("2.bedjoint").utilization, 0.012 / 0.015, 1e-9, "bed joint ratio");
    close(find("2.durability").utilization, 1.0 / 10.0, 1e-9, "durability ratio");
    assert_eq!(find("6.6.reinforced").status, CheckStatus::NotApplicable);
    let fire = find("1-2.fire.wall");
    assert!(fire.utilization > 0.0 && fire.utilization < 1.0, "fire utilisation {}", fire.utilization);
}

#[test]
fn document_cases_run_from_the_wall_and_resolve_in_the_document() {
    use crate::checks::masonry::evaluate_building;
    use semio_s_artifact_norm_en1996_model::En1996Snapshot;
    let doc = En1996Snapshot::reinforced_wall_example();
    let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
    let ids: Vec<&str> = report.checks.iter().map(|c| c.id.as_str()).collect();
    for needle in ["en1996.m-02.fk.", "en1996.m-10.rho-n.", "en1996.m-31.w-rd.", "en1996.m-32.q-lat.", "en1996.m-40.m-rd.", "en1996.m-52.spacing.", "en1996.m-70.min-thickness."] {
        assert!(ids.iter().any(|id| id.starts_with(needle)), "missing document case {needle}: {ids:?}");
    }
    let two_way = report.checks.iter().find(|c| c.id.starts_with("en1996.m-31.w-rd.")).expect("two-way check");
    assert_ne!(two_way.status, CheckStatus::Fail, "alternative resistance models only warn");
    assert!(two_way.subject.path.starts_with("walls[id="));
    let mut wider = doc.clone();
    wider.walls[0].thickness_m *= 2.0;
    let after = evaluate_building(wider.annex, wider.masonry_class, wider.design_situation, wider.reliability_class, wider.storeys, &wider.walls);
    let before_u = two_way.utilization;
    let after_u = after.checks.iter().find(|c| c.id.starts_with("en1996.m-31.w-rd.")).expect("two-way check").utilization;
    assert!(after_u < before_u, "a thicker wall must lower the two-way utilisation: {before_u} -> {after_u}");
}

#[test]
fn yield_line_fixed_edge_cases_match_the_closed_forms() {
    use crate::checks::masonry::numerical_models::{yield_line_pressure, Edges};
    let w = |code: &str, l: f64, h: f64| yield_line_pressure(Edges::parse(code).expect("edges"), l, h, 1.0, 1.0);
    close(w("SSSS", 1.0, 1.0), 24.0, 1e-3, "simply supported square");
    close(w("XXXX", 1.0, 1.0), 48.0, 1e-3, "fixed square");
    close(w("SSFF", 1.0, 1.0), 8.0, 1e-3, "one-way simply supported");
    close(w("XXFF", 1.0, 1.0), 16.0, 1e-3, "one-way fixed");
    close(w("FXFF", 1.0, 1.0), 2.0, 1e-3, "cantilever");
}

#[test]
fn superseded_forms_are_gone_from_the_masonry_sources() {
    use crate::checks::masonry::{CATALOGUE_SOURCE_ROOT, FORMULAS_SOURCE_ROOT, SOURCE_ROOT};
    let mut stack: Vec<std::path::PathBuf> = [SOURCE_ROOT, CATALOGUE_SOURCE_ROOT, FORMULAS_SOURCE_ROOT].into_iter().map(std::path::PathBuf::from).collect();
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("read dir") {
            let path = entry.expect("entry").path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "rs") && !path.to_string_lossy().contains("tests") {
                let text = std::fs::read_to_string(&path).expect("read");
                for needle in ["Compatibility", "older durability", "pub fn compressive_strength_mpa"] {
                    assert!(!text.contains(needle), "{} contains {needle}", path.display());
                }
            }
        }
    }
}

/// 🧾 The shared fixture cases: valid inputs for every registered entry, some of them failing.
pub(super) fn fixture_cases() -> Vec<(&'static str, Case)> {
    let en = AnnexChoice::En;
    let de = AnnexChoice::De;
    let masonry = |c: Case| c.with_n("fb_pa", 20e6).with_n("fm_pa", 10e6).with_s("material", "clay").with_n("gamma_m", 1.5);
    vec![
        ("M-01", Case::new("a", de).with_n("fb_decl_pa", 12e6).with_n("unit_h_m", 0.238).with_n("unit_w_m", 0.24).with_n("hole_pct", 40.0)),
        ("M-02", masonry(Case::new("a", en))),
        ("M-02", masonry(Case::new("tl", de)).with_s("mortar", "tl").with_n("group", 2.0)),
        ("M-03", masonry(Case::new("a", en)).with_n("sigma_pa", 0.8e6)),
        ("M-03", masonry(Case::new("de", de)).with_n("sigma_pa", 0.2e6).with_s("head_joint", "unfilled")),
        ("M-04", masonry(Case::new("a", en)).with_n("sigma_pa", 0.1e6)),
        ("M-05", Case::new("a", en).with_n("diameter_m", 0.008).with_n("fyd_pa", 435e6).with_n("lb_avail_m", 0.3).with_n("gamma_m", 1.5)),
        ("M-06", Case::new("a", de).with_n("fk_pa", 6e6).with_s("material", "calcium_silicate")),
        ("M-07", Case::new("a", de).with_n("class", 3.0).with_s("situation", "accidental")),
        ("M-10", Case::new("a", en).with_n("h_m", 2.75).with_n("t_m", 0.365).with_n("sides", 3.0).with_n("l_m", 3.0).with_n("rho2", 0.75).with_n("stiff_len_m", 0.4).with_n("stiff_t_m", 0.1)),
        ("M-11", Case::new("a", en).with_n("hef_m", 4.5).with_n("t_m", 0.175)),
        ("M-12", Case::new("a", en).with_n("t_m", 0.1).with_n("hef_m", 2.0).with_n("fd_pa", 4e6).with_n("n_top_n", 300e3).with_n("n_bot_n", 300e3).with_n("m_top_nm", 6000.0).with_n("m_bot_nm", 3000.0)),
        ("M-13", Case::new("a", de).with_n("t_m", 0.175).with_n("hef_m", 2.5).with_n("fd_pa", 3e6).with_n("n_mid_n", 400e3).with_n("m_mid_nm", 2000.0).with_n("m_hm_nm", 500.0).with_n("phi_inf", 1.5).with_n("n_top_n", 400e3).with_n("n_bot_n", 420e3)),
        ("M-14", Case::new("a", de).with_n("ab_m2", 0.02).with_n("a1_m", 0.2).with_n("hc_m", 2.5).with_n("t_m", 0.2).with_n("lefm_m", 1.0).with_n("fd_pa", 3e6).with_n("ned_c_n", 200e3)),
        ("M-15", Case::new("a", en).with_n("pier_l_m", 0.4).with_n("t_m", 0.365).with_n("n_pier_n", 300e3).with_n("fd_pa", 4e6).with_n("opening_w_m", 1.2).with_n("density", 1800.0)),
        ("M-16", Case::new("a", en).with_n("t_m", 0.175).with_n("hef_m", 2.0).with_n("fd_pa", 3e6).with_n("n_mid_n", 120e3).with_n("w_ed_pa", 960.0).with_n("h_m", 2.75)),
        ("M-20", masonry(Case::new("a", en)).with_n("l_m", 2.0).with_n("t_m", 0.365).with_n("n_ed_n", 300e3).with_n("m_ed_nm", 90e3).with_n("v_ed_n", 900e3).with_n("flange_t_m", 0.365).with_n("web_spacing_m", 4.0).with_n("height_total_m", 8.0)),
        ("M-21", Case::new("a", en).with_v("l_m", &[2.0, 3.0, 2.5]).with_v("t_m", &[0.24, 0.24, 0.24]).with_v("x_m", &[0.0, 5.0, 10.0]).with_v("vrd_n", &[100e3, 150e3, 120e3]).with_n("h_m", 2.75).with_n("e_pa", 5e9).with_n("v_ed_n", 280e3).with_n("b_m", 10.0).with_n("e0_m", 0.3)),
        ("M-21", Case::new("flexible", de).with_v("l_m", &[2.0, 3.0, 3.0]).with_v("t_m", &[0.365, 0.365, 0.365]).with_v("x_m", &[0.0, 2.0, 6.0]).with_v("vrd_n", &[30e3, 120e3, 70e3]).with_v("trib", &[1.0 / 6.0, 0.5, 1.0 / 3.0]).with_s("diaphragm", "flexible").with_n("edge_lo_m", 0.0).with_n("edge_hi_m", 6.0).with_n("h_m", 2.75).with_n("e_pa", 6e9).with_n("v_ed_n", 200e3).with_n("b_m", 1.0).with_n("e0_m", 0.01)),
        ("M-30", Case::new("a", en).with_n("t_m", 0.1).with_n("h_m", 2.75).with_n("w_ed_pa", 960.0).with_n("fxk1_pa", 0.1e6).with_n("gamma_m", 1.5)),
        ("M-31", Case::new("a", en).with_n("t_m", 0.1).with_n("h_m", 2.75).with_n("l_m", 3.5).with_n("w_ed_pa", 2500.0).with_n("fxk1_pa", 0.1e6).with_n("fxk2_pa", 0.4e6).with_n("gamma_m", 1.5)),
        ("M-31", Case::new("free", en).with_n("t_m", 0.175).with_n("h_m", 2.75).with_n("l_m", 3.5).with_n("w_ed_pa", 960.0).with_n("fxk1_pa", 0.2e6).with_n("fxk2_pa", 0.4e6).with_n("gamma_m", 1.5).with_s("edges", "SSSF")),
        ("M-31", Case::new("fixed", en).with_n("t_m", 0.175).with_n("h_m", 2.75).with_n("l_m", 3.5).with_n("w_ed_pa", 1500.0).with_n("fxk1_pa", 0.2e6).with_n("fxk2_pa", 0.4e6).with_n("gamma_m", 1.5).with_s("edges", "XXXX")),
        ("M-31", Case::new("mixed", en).with_n("t_m", 0.1).with_n("h_m", 2.75).with_n("l_m", 3.5).with_n("w_ed_pa", 2500.0).with_n("fxk1_pa", 0.1e6).with_n("fxk2_pa", 0.4e6).with_n("gamma_m", 1.5).with_s("edges", "SXSS")),
        ("M-31", Case::new("cantilever", en).with_n("t_m", 0.2).with_n("h_m", 2.0).with_n("l_m", 3.0).with_n("w_ed_pa", 1500.0).with_n("fxk1_pa", 0.2e6).with_n("fxk2_pa", 0.4e6).with_n("gamma_m", 1.5).with_s("edges", "FXFF")),
        ("M-32", Case::new("a", en).with_n("t_m", 0.175).with_n("la_m", 3.0).with_n("fd_pa", 3e6).with_n("w_ed_pa", 2000.0)),
        ("M-33", Case::new("a", de).with_n("qp_pa", 800.0).with_n("cpe", 0.8).with_n("k0", 0.5).with_n("gamma_soil_n_m3", 18e3).with_n("he_m", 2.0)),
        ("M-34", Case::new("a", de).with_n("h_m", 2.5).with_n("t_m", 0.3).with_n("he_m", 2.0).with_n("rho_e_n_m3", 18e3).with_n("qk_pa", 10e3).with_n("n_min_n", 20e3).with_n("n_max_n", 300e3).with_n("fd_pa", 3e6)),
        ("M-40", Case::new("a", en).with_n("b_m", 1.0).with_n("d_m", 0.3).with_n("as_m2", 200e-6).with_n("fd_pa", 5.957e6).with_n("m_ed_nm", 60e3)),
        ("M-41", Case::new("a", en).with_n("b_m", 1.0).with_n("d_m", 0.3).with_n("fvk_pa", 0.3e6).with_n("fd_pa", 5e6).with_n("asw_m2", 100e-6).with_n("s_m", 0.2).with_n("v_ed_n", 400e3).with_n("av_m", 0.4).with_n("gamma_m", 1.5)),
        ("M-42", Case::new("a", de).with_n("b_m", 0.365).with_n("l_m", 2.0).with_n("h_m", 1.5).with_n("fd_pa", 5e6).with_n("n_ed_n", 300e3).with_n("m_ed_nm", 900e3).with_v("bar_y_m", &[0.1, 1.9]).with_v("bar_a_m2", &[6e-4, 6e-4])),
        ("M-43", Case::new("a", en).with_n("col_a_m2", 0.015).with_n("col_min_m", 0.12).with_n("col_as_m2", 150e-6).with_n("col_spacing_m", 5.0).with_n("v_ed_n", 80e3).with_n("l_m", 3.0).with_n("t_m", 0.24).with_n("fvk_pa", 0.3e6).with_n("gamma_m", 1.5)),
        ("M-44", Case::new("a", en).with_n("p0_n", 120e3).with_n("ap_m2", 150e-6).with_n("fpk_pa", 1770e6).with_n("ep_pa", 195e9).with_n("e_pa", 8e9).with_n("phi_inf", 1.5).with_n("eps_cs", 0.0003).with_n("rho_relax", 0.05).with_n("a_m2", 0.24).with_n("n_ed_n", 200e3).with_n("fd_pa", 4e6).with_n("fxk1_pa", 0.1e6).with_n("z_m3", 0.0096).with_n("gamma_m", 1.5)),
        ("M-45", Case::new("a", de).with_n("diameter_m", 0.012).with_n("provided_m", 1.0).with_n("fyd_pa", 435e6).with_n("gamma_m", 1.5)),
        ("M-50", Case::new("a", de).with_n("n_per_m2", 4.0).with_n("cavity_m", 0.12).with_n("h_wall_m", 8.0).with_n("w_ed_pa", 1200.0).with_n("t1_m", 0.115).with_n("t2_m", 0.175).with_n("tie_rd_n", 400.0)),
        ("M-51", Case::new("a", en).with_n("w_ed_pa", 1000.0).with_n("h_trib_m", 2.75).with_n("s_anchor_m", 2.0).with_n("anchor_rk_n", 6000.0).with_n("ring_tie_n", 20e3)),
        ("M-52", Case::new("a", en).with_n("spacing_m", 14.0).with_s("material", "clay")),
        ("M-60", Case::new("a", de).with_s("exposure", "mx3").with_s("material", "clay").with_n("fm_pa", 2.5e6).with_n("frost_class", 1.0).with_n("cover_m", 0.02)),
        ("M-61", Case::new("a", en).with_n("span_m", 3.0).with_n("d_m", 0.1).with_n("deflection_m", 0.01)),
        ("M-70", Case::new("a", de).with_n("bed_joint_m", 0.02).with_n("overlap_m", 0.03).with_n("unit_h_m", 0.113).with_n("t_m", 0.1).with_n("pier_area_m2", 0.05).with_n("chase_depth_m", 0.04)),
        ("M-80", Case::new("a", de).with_n("height_total_m", 22.0).with_n("span_m", 6.5).with_n("h_m", 3.0).with_n("t_m", 0.175).with_n("qk_pa", 6e3).with_n("l_m", 0.4).with_n("storeys", 3.0)),
        ("M-81", Case::new("a", de).with_n("t_m", 0.175).with_n("hef_m", 2.0).with_n("span_m", 5.0).with_n("fk_pa", 4e6).with_n("gamma_m", 1.5).with_n("n_ed_n", 300e3).with_n("ab_m2", 0.02).with_n("a_m2", 0.1).with_n("ned_c_n", 80e3)),
        ("M-82", Case::new("a", de).with_n("lc_m", 0.25).with_n("t_m", 0.175).with_n("sigma_d_pa", 0.3e6).with_n("fvk0_pa", 0.2e6).with_n("gamma_m", 1.5).with_n("v_ed_n", 60e3)),
        ("M-83", Case::new("a", de).with_n("t_m", 0.115).with_n("h_m", 2.75).with_n("w_ed_pa", 1200.0).with_n("fxk1_pa", 0.1e6).with_n("gamma_m", 1.5).with_n("basement", 1.0).with_n("he_m", 2.0).with_n("rho_e_n_m3", 18e3).with_n("n_min_n", 10e3).with_n("n_max_n", 100e3).with_n("fd_pa", 3e6)),
        ("M-90", Case::new("a", de).with_n("t_m", 0.1).with_n("rei", 120.0).with_s("material", "aerated").with_n("group", 2.0).with_s("mortar", "tl").with_n("mu_fi", 0.9)),
        ("M-91", Case::new("a", en).with_n("t_m", 0.24).with_n("l_m", 1.0).with_n("a_theta_m", 0.05).with_n("k_theta", 0.5).with_n("fd_pa", 4e6).with_n("n_ed_fi_n", 400e3)),
        ("M-95", Case::new("a", en).with_s("system", "confined").with_n("q", 3.5)),
    ]
}

mod wave2 {
    use crate::checks::masonry::{evaluate_building, evaluate_outcomes, Case};
    use semio_s_artifact_norm_en1996_model::En1996Snapshot;
    use semio_s_plugin_norm_compliance::AnnexChoice;

    #[test]
    fn anchorage_length_follows_phi_f_yd_over_four_f_bd() {
        let c = Case::new("a", AnnexChoice::De).with_n("diameter_m", 0.012).with_n("provided_m", 1.0).with_n("fyd_pa", 435e6).with_n("gamma_m", 1.5);
        let out = evaluate_outcomes("M-45", &c).unwrap();
        let expected = 0.012 * 435e6 / (4.0 * 2.0e6 / 1.5);
        let lb = out.iter().find(|o| o.key == "lb").unwrap().capacity;
        assert!((lb - expected).abs() < 1e-9, "l_b {lb} against {expected}");
        assert!(out.iter().find(|o| o.key == "anchorage").unwrap().utilization() < 1.0);
    }

    #[test]
    fn lintel_deflection_limit_follows_the_document_ratio() {
        let c = |ratio: f64| Case::new("l", AnnexChoice::De).with_n("span_m", 2.0).with_n("d_m", 0.25).with_n("deflection_m", 0.005).with_n("limit_ratio", ratio);
        let util = |ratio: f64| evaluate_outcomes("M-61", &c(ratio)).unwrap().iter().find(|o| o.key == "lintel-deflection").unwrap().utilization();
        assert!((util(500.0) - 0.005 / (2.0 / 500.0)).abs() < 1e-12);
        assert!(util(250.0) < util(500.0));
    }

    #[test]
    fn distance_a1_never_lowers_the_concentrated_resistance() {
        let utilisation = |a1: f64| {
            let mut doc = En1996Snapshot::concentrated_load_example();
            doc.walls[0].load_cases[0].concentrated[0].distance_a1_m = a1;
            let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
            report.checks.iter().find(|c| c.id.starts_with("en1996.6.1.3.concentrated.")).unwrap().utilization
        };
        assert!(utilisation(1.0) <= utilisation(0.0) + 1e-12);
    }

    #[test]
    fn the_members_example_runs_every_member_case_from_the_document() {
        let doc = En1996Snapshot::members_wall_example();
        let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
        let ids: Vec<&str> = report.checks.iter().map(|c| c.id.as_str()).collect();
        for needle in ["en1996.m-15.", "en1996.m-61.", "en1996.m-50.", "en1996.m-51.", "en1996.m-44.", "en1996.m-43.", "en1996.m-20.", "en1996.m-41.", "en1996.m-45.", "en1996.m-60.", "en1996.m-91.", "en1996.5.4.extent."] {
            assert!(ids.iter().any(|id| id.starts_with(needle)), "missing member case {needle}: {ids:?}");
        }
    }

    #[test]
    fn member_rows_outside_the_wall_fail_and_offer_a_remedy_on_their_position() {
        let mut doc = En1996Snapshot::members_wall_example();
        doc.walls[0].piers[0].position_m = doc.walls[0].length_m;
        let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
        let c = report.checks.iter().find(|c| c.id.starts_with("en1996.5.4.extent.") && c.id.contains(".piers.")).unwrap();
        assert!(c.utilization > 1.0 && !c.remedies.is_empty());
    }

    #[test]
    fn bracing_distribution_carries_the_storey_shear_and_the_torsion_adds_to_the_stiff_walls() {
        let case = Case::new("b", AnnexChoice::En).with_v("l_m", &[2.0, 3.0]).with_v("t_m", &[0.24, 0.24]).with_v("x_m", &[0.0, 6.0]).with_v("vrd_n", &[1e6, 1e6]).with_n("h_m", 2.75).with_n("e_pa", 5e9).with_n("v_ed_n", 100e3).with_n("b_m", 6.0);
        let b = crate::checks::masonry::bracing_distribution(&case).unwrap();
        assert!(b.centre > 3.0 && b.centre < 6.0, "the longer wall is stiffer: {}", b.centre);
        assert!((b.direct.iter().sum::<f64>() - 100e3).abs() < 1e-6);
        assert!(b.shares.iter().zip(&b.direct).all(|(s, d)| s >= d));
        assert!((b.torsion - 100e3 * 0.05 * 6.0).abs() < 1e-9);
    }

    #[test]
    fn a_flexible_diaphragm_distributes_the_storey_shear_by_tributary_width_without_torsion() {
        let case = Case::new("b", AnnexChoice::En).with_v("l_m", &[2.0, 3.0]).with_v("t_m", &[0.24, 0.24]).with_v("x_m", &[0.0, 6.0]).with_v("vrd_n", &[1e6, 1e6]).with_n("h_m", 2.75).with_n("e_pa", 5e9).with_n("v_ed_n", 100e3).with_n("b_m", 6.0).with_s("diaphragm", "flexible").with_v("trib", &[0.5, 0.5]);
        let b = crate::checks::masonry::bracing_distribution(&case).unwrap();
        assert!((b.shares[0] - 50e3).abs() < 1e-6 && (b.shares[1] - 50e3).abs() < 1e-6 && b.torsion == 0.0, "{:?}", b.shares);
        let rigid = crate::checks::masonry::bracing_distribution(&case.clone().with_s("diaphragm", "rigid")).unwrap();
        assert!(rigid.shares[1] > rigid.shares[0] && rigid.torsion > 0.0, "the stiffer wall takes more under the rigid rule");
    }

    #[test]
    fn the_bracing_case_of_a_document_with_a_soft_diaphragm_carries_the_tributary_rule() {
        let mut doc = En1996Snapshot::compliant_clay_wall();
        let mut second = doc.walls[0].clone();
        second.id = "wall-shear-2".into();
        for (wall, y) in doc.walls.iter_mut().chain(std::iter::once(&mut second)).zip([0.0, 6.0]) {
            wall.wall_type = semio_s_artifact_norm_en1996_model::WallType::Shear;
            wall.plan_x_m = 0.0;
            wall.plan_y_m = y;
            wall.in_plane_shear_n = 50_000.0;
            wall.diaphragm_stiffness_n_per_m = 1.0e5;
        }
        doc.walls.push(second);
        let case = crate::checks::masonry::bracing_case(doc.annex, &doc.walls).unwrap();
        assert_eq!(case.so("diaphragm", "rigid"), "flexible");
        assert_eq!(case.v("trib").unwrap().len(), 2);
        doc.walls.iter_mut().for_each(|w| w.diaphragm_stiffness_n_per_m = 1.0e12);
        assert_eq!(crate::checks::masonry::bracing_case(doc.annex, &doc.walls).unwrap().so("diaphragm", "rigid"), "rigid");
    }

    #[test]
    fn the_bracing_example_reaches_the_evaluator_as_vector_inputs_and_the_anchorage_bar_as_a_bond_case() {
        let doc = En1996Snapshot::bracing_walls_example();
        let case = crate::checks::masonry::bracing_case(doc.annex, &doc.walls).unwrap();
        assert_eq!(case.v("l_m").unwrap().len(), 2);
        assert_eq!(case.v("x_m").unwrap().len(), 2);
        let members = En1996Snapshot::members_wall_example();
        let cases = crate::checks::masonry::document_cases(members.annex, members.masonry_class, members.design_situation, members.reliability_class, members.storeys, &members.walls[0]);
        let bond = cases.iter().find(|(id, _)| *id == "M-05").expect("the anchorage bar yields M-05");
        let twin = cases.iter().find(|(id, _)| *id == "M-45").expect("and M-45");
        let (bond, twin) = (crate::checks::masonry::evaluate_case("M-05", &bond.1), crate::checks::masonry::evaluate_case("M-45", &twin.1));
        assert!(bond.iter().any(|c| c.id.contains("m-05.")) && twin.iter().any(|c| c.id.contains("m-45.")));
    }

    #[test]
    fn an_overstressed_bracing_wall_offers_a_longer_wall_as_remedy() {
        let mut doc = En1996Snapshot::compliant_clay_wall();
        let mut second = doc.walls[0].clone();
        second.id = "wall-shear-2".into();
        for (wall, x) in doc.walls.iter_mut().chain(std::iter::once(&mut second)).zip([0.0, 6.0]) {
            wall.wall_type = semio_s_artifact_norm_en1996_model::WallType::Shear;
            wall.plan_x_m = x;
            wall.in_plane_shear_n = 400_000.0;
        }
        doc.walls.push(second);
        let case = crate::checks::masonry::bracing_case(doc.annex, &doc.walls).unwrap();
        let results = crate::checks::masonry::evaluate_case("M-21", &case);
        let governing = results.iter().find(|c| c.id.contains("governing-wall")).unwrap();
        assert!(governing.utilization > 1.0 && !governing.remedies.is_empty(), "{:?}", governing.remedies);
    }

    #[test]
    fn simplified_cases_run_for_shear_and_non_load_bearing_walls() {
        let mut doc = En1996Snapshot::compliant_clay_wall();
        doc.walls[0].wall_type = semio_s_artifact_norm_en1996_model::WallType::Shear;
        doc.walls[0].in_plane_shear_n = 20_000.0;
        let ids = |doc: &En1996Snapshot| evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls).checks.into_iter().map(|c| c.id).collect::<Vec<_>>();
        let shear = ids(&doc);
        for needle in ["en1996.m-80.", "en1996.m-81.", "en1996.m-82."] {
            assert!(shear.iter().any(|id| id.starts_with(needle)), "shear wall misses {needle}");
        }
        doc.walls[0].wall_type = semio_s_artifact_norm_en1996_model::WallType::NonLoadBearing;
        assert!(ids(&doc).iter().any(|id| id.starts_with("en1996.m-80.")));
    }

    #[test]
    fn bed_joint_reinforcement_needs_five_millimetres_more_than_the_bar() {
        let c = |joint: f64| Case::new("j", AnnexChoice::En).with_n("bed_joint_m", joint).with_n("diameter_m", 0.008).with_s("mortar", "gp");
        let ratio = |joint: f64| evaluate_outcomes("M-70", &c(joint)).unwrap().iter().find(|o| o.key == "bed-joint-bar").unwrap().utilization();
        assert!(ratio(0.014) <= 1.0 && ratio(0.014) > 0.0);
        assert!(ratio(0.010) > 1.0);
    }

    use crate::checks::masonry::{evaluate_case, ROW_KEYS};
    use semio_s_plugin_norm_compliance::{CheckResult, LocalizedCopy};

    fn bound(path: &str) -> Case {
        Case::new("t", AnnexChoice::En).bound("w", path, LocalizedCopy::new("W", "W"))
    }

    fn rows(case: Case, entity: &str, positions: &[f64]) -> Case {
        let paths: Vec<(&str, String)> = positions.iter().enumerate().map(|(i, _)| (ROW_KEYS[i], format!("walls[id=w].{entity}[id=r{i}].positionM"))).collect();
        case.with_v("pos_m", positions).with_paths(paths)
    }

    fn first<'a>(results: &'a [CheckResult], needle: &str) -> &'a CheckResult {
        results.iter().find(|c| c.id.contains(needle)).unwrap_or_else(|| panic!("{needle} missing in {:?}", results.iter().map(|c| &c.id).collect::<Vec<_>>()))
    }

    #[test]
    fn a_confined_masonry_spacing_remedy_moves_one_column_to_the_computed_position() {
        let case = rows(bound("walls[id=w].confined[id=r0].asM2"), "confined", &[0.5, 6.0])
            .with_n("col_a_m2", 0.09)
            .with_n("col_min_m", 0.3)
            .with_n("col_as_m2", 1e-3)
            .with_n("col_spacing_m", 5.5)
            .with_n("l_m", 9.0);
        let results = evaluate_case("M-43", &case);
        let spacing = first(&results, "col-spacing");
        let remedy = spacing.remedies.first().expect("remedy");
        assert!(remedy.target.path.ends_with("confined[id=r0].positionM"), "{}", remedy.target.path);
        assert!((remedy.required.value - 2.0).abs() < 1e-9, "{}", remedy.required.value);
        let moved = rows(case.clone(), "confined", &[2.0, 6.0]).with_n("col_spacing_m", 4.0);
        assert!(first(&evaluate_case("M-43", &moved), "col-spacing").utilization <= 1.0 + 1e-9);
    }

    #[test]
    fn a_confined_masonry_area_remedy_widens_the_smallest_column_to_the_required_area() {
        let case = bound("walls[id=w].confined[id=r0].asM2")
            .with_path("col_a_side_m", "walls[id=w].confined[id=r0].depthM")
            .with_n("col_a_side_m", 0.12)
            .with_n("col_a_other_m", 0.15)
            .with_n("col_a_m2", 0.018)
            .with_n("col_min_m", 0.12)
            .with_n("col_as_m2", 1e-3)
            .with_n("col_spacing_m", 3.0);
        let results = evaluate_case("M-43", &case);
        let remedy = first(&results, "col-area").remedies.first().expect("remedy").clone();
        assert!(remedy.target.path.ends_with("depthM"));
        assert!((remedy.required.value - 0.134).abs() < 1e-9, "0.02 / 0.15 rounded up to the millimetre: {}", remedy.required.value);
        assert!(0.134 * 0.15 >= 0.02);
    }

    #[test]
    fn an_anchor_spacing_remedy_moves_the_anchor_that_closes_the_widest_interval() {
        let case = rows(bound("walls[id=w].ties[id=r0].capacityN"), "ties", &[1.0, 1.8, 5.4])
            .with_n("w_ed_pa", 1000.0)
            .with_n("h_trib_m", 1.0)
            .with_n("s_anchor_m", 3.6)
            .with_n("anchor_rk_n", 4500.0)
            .with_n("l_m", 6.0);
        let results = evaluate_case("M-51", &case);
        let remedy = first(&results, "anchor").remedies.first().expect("remedy").clone();
        assert!(remedy.target.path.ends_with("ties[id=r1].positionM"), "{}", remedy.target.path);
        assert!((remedy.required.value - 2.4).abs() < 1e-9, "{}", remedy.required.value);
        let moved = rows(case.clone(), "ties", &[1.0, 2.4, 5.4]).with_n("s_anchor_m", 3.0);
        assert!(first(&evaluate_case("M-51", &moved), "anchor").utilization <= 1.0 + 1e-9);
    }

    #[test]
    fn a_prestress_bearing_remedy_thickens_the_wall_to_the_required_net_area() {
        let case = bound("walls[id=w].prestress[id=r0].forceN")
            .with_path("t_m", "walls[id=w].thicknessM")
            .with_n("t_m", 0.2)
            .with_n("p0_n", 500e3)
            .with_n("ap_m2", 1e-3)
            .with_n("fpk_pa", 1.86e9)
            .with_n("ep_pa", 195e9)
            .with_n("e_pa", 5e9)
            .with_n("a_m2", 0.2)
            .with_n("n_ed_n", 100e3)
            .with_n("fd_pa", 1.5e6);
        let results = evaluate_case("M-44", &case);
        let remedy = first(&results, "compression").remedies.first().expect("remedy").clone();
        assert!(remedy.target.path.ends_with("walls[id=w].thicknessM"));
        assert!((remedy.required.value - 0.4).abs() < 1e-9, "(N + P0)/f_d = 0.4 m² on 0.2 m² at 0.2 m: {}", remedy.required.value);
        assert!(remedy.action.en.contains("0.4000 m²"), "{}", remedy.action.en);
    }

    #[test]
    fn a_cavity_tie_remedy_names_the_resistance_or_the_count_that_the_check_needs() {
        let base = |n: f64| {
            bound("walls[id=w].ties[id=r0].capacityN").with_path("tie_rd_n", "walls[id=w].ties[id=r0].capacityN").with_n("n_per_m2", n).with_n("cavity_m", 0.1).with_n("w_ed_pa", 900.0).with_n("t1_m", 0.1).with_n("t2_m", 0.1).with_n("tie_rd_n", 100.0).with_n("face_m2", 10.0)
        };
        let force = evaluate_case("M-50", &base(3.0));
        let remedy = first(&force, "tie-force").remedies.first().expect("remedy").clone();
        assert!(remedy.target.path.ends_with("capacityN") && (remedy.required.value - 150.0).abs() < 1e-9, "{}", remedy.required.value);
        let density = evaluate_case("M-50", &base(2.0));
        let text = &first(&density, "density").remedies.first().expect("remedy").action.en;
        assert!(text.contains("at least 30 wall ties"), "{text}");
    }

    #[test]
    fn the_catalogue_derivations_that_restate_a_wall_check_run_from_the_wall_and_only_warn() {
        let mut wind = En1996Snapshot::reinforced_wall_example();
        wind.walls[0].fire_rei_min = 90;
        let load = En1996Snapshot::concentrated_load_example();
        for (doc, needles) in [(wind, vec!["m-03.", "m-07.", "m-11.", "m-12.", "m-13.", "m-16.", "m-30.", "m-90."]), (load, vec!["m-14."])] {
            let report = evaluate_building(doc.annex, doc.masonry_class, doc.design_situation, doc.reliability_class, doc.storeys, &doc.walls);
            for needle in needles {
                let hits: Vec<_> = report.checks.iter().filter(|c| c.id.starts_with(&format!("en1996.{needle}"))).collect();
                assert!(!hits.is_empty(), "missing cross-check {needle}");
                assert!(hits.iter().all(|c| c.status != semio_s_plugin_norm_compliance::CheckStatus::Fail), "{needle} is advisory");
            }
        }
    }
}
