use crate::catalogue::*;
use crate::*;

fn value(case: &Case, id: &str, key: &str) -> f64 {
    let outcome = evaluate_outcomes(id, case).unwrap_or_else(|reason| panic!("{id}: {reason}")).into_iter().find(|o| o.key == key).unwrap_or_else(|| panic!("{id}: no outcome {key}"));
    if outcome.mode == Mode::Limit { outcome.demand } else { outcome.capacity }
}

fn close(left: f64, right: f64) {
    assert!((left - right).abs() <= 1e-9 * right.abs().max(1.0), "{left} != {right}");
}

const WORKED: &str = "G|G|50;Q1|Q|20|office;W|Q|10|wind;S|Q|8|snow";

fn worked(annex: AnnexChoice) -> Case {
    Case::new("worked", annex).with_s("actions", WORKED)
}

#[test]
fn ndp_registry_resolves_every_symbol_used_by_the_factor_functions() {
    for annex in [AnnexChoice::En, AnnexChoice::De] {
        for domain in [Domain::Building, Domain::Bridge] {
            for set in [FactorSet::EquA, FactorSet::StrB, FactorSet::GeoC] {
                let (sup, inf) = gamma_g(annex, domain, set);
                assert!(sup.is_finite() && inf.is_finite(), "{annex:?} {domain:?} {set:?}");
                for category in ["office", "road_traffic", "rail_traffic", "wind_bridge"] {
                    assert!(gamma_q(annex, domain, set, category).is_finite(), "{annex:?} {domain:?} {set:?} {category}");
                }
            }
            assert!(xi(annex, domain).is_finite());
        }
        assert!(gamma_p(annex, false).is_finite() && gamma_p(annex, true).is_finite());
    }
    assert!(semio_s_plugin_norm_compliance::NDP_TABLE.iter().filter(|row| row.id.starts_with("ndp.en1990.")).all(|row| [AnnexChoice::En, AnnexChoice::De].into_iter().all(|annex| semio_s_plugin_norm_compliance::ndp_number_opt(row.id, annex).is_none_or(f64::is_finite))));
}

#[test]
fn psi_table_is_ordered_and_covers_every_annex_a_category() {
    for entry in PSI_TABLE {
        assert!(psi_is_ordered(entry.row(AnnexChoice::En)) && psi_is_ordered(entry.row(AnnexChoice::De)), "{}", entry.key);
    }
    assert!(PSI_TABLE.iter().any(|entry| entry.table == "A2.3") && PSI_TABLE.iter().any(|entry| entry.table == "A1.1"));
    assert_eq!(psi_lookup(AnnexChoice::De, "B"), psi_lookup(AnnexChoice::De, "office"));
    assert!(psi_lookup(AnnexChoice::En, "unknown").is_none());
}

#[test]
fn snow_switches_to_the_high_altitude_row_in_both_annexes() {
    for annex in [AnnexChoice::De, AnnexChoice::En] {
        assert_eq!(resolve_psi_category(annex, "snow", 1200.0), "snow_high");
        assert_eq!(resolve_psi_category(annex, "snow", 800.0), "snow");
        assert_eq!(resolve_psi_category(annex, "snow_high", 800.0), "snow");
    }
    assert!((NaDe.psi_0("snow_high") - 0.7).abs() < 1e-12 && (NaEn.psi_0("snow") - 0.5).abs() < 1e-12);
}

#[test]
fn spec_example_e90_01_to_03_fundamental_6_10_lead_enumeration() {
    let case = worked(AnnexChoice::De);
    close(value(&case, "E90.16", "e610"), 1.35 * 50.0 + 1.5 * 20.0 + 1.5 * 0.6 * 10.0 + 1.5 * 0.5 * 8.0);
    let ctx = Context::new(AnnexChoice::De, Domain::Building, FactorSet::StrB, 1.0);
    let actions = Action::parse_list(WORKED).expect("actions");
    let all = combinations(&ctx, &actions, Equation::E610).expect("combinations");
    assert_eq!(all.len(), 3);
    let by = |lead: &str| all.iter().find(|c| c.leading == lead).expect("lead").total();
    close(by("Q1"), 112.5);
    close(by("W"), 1.35 * 50.0 + 1.5 * 10.0 + 1.5 * 0.7 * 20.0 + 1.5 * 0.5 * 8.0);
    close(by("S"), 1.35 * 50.0 + 1.5 * 8.0 + 1.5 * 0.7 * 20.0 + 1.5 * 0.6 * 10.0);
}

#[test]
fn spec_example_e90_04_05_en_offers_610a_and_610b_and_de_does_not() {
    let en = worked(AnnexChoice::En);
    close(value(&en, "E90.16", "e610a"), 1.35 * 50.0 + 1.5 * 0.7 * 20.0 + 1.5 * 0.6 * 10.0 + 1.5 * 0.5 * 8.0);
    close(value(&en, "E90.16", "e610b"), 0.85 * 1.35 * 50.0 + 1.5 * 20.0 + 1.5 * 0.6 * 10.0 + 1.5 * 0.5 * 8.0);
    close(value(&en, "E90.16", "e610ab"), 0.85 * 1.35 * 50.0 + 1.5 * 20.0 + 1.5 * 0.6 * 10.0 + 1.5 * 0.5 * 8.0);
    let de = evaluate_outcomes("E90.16", &worked(AnnexChoice::De)).expect("de");
    assert!(de.iter().all(|o| o.key != "e610a" && o.key != "e610b"), "NA-DE offers 6.10 only");
    assert!(de.iter().any(|o| o.key == "simplified"));
}

#[test]
fn spec_example_e90_06_equ_stabilising_and_destabilising() {
    let case = Case::new("equ", AnnexChoice::De).with_s("actions", "Gs|G|-400;Gd|G|120;Qd|Q|60|office");
    close(value(&case, "E90.15", "e-stb"), 0.9 * 400.0);
    close(value(&case, "E90.15", "e-dst"), 1.1 * 120.0 + 1.5 * 60.0);
    let with_anchor = case.clone().with_n("t_d", 200.0);
    assert!(value(&with_anchor, "E90.15", "equ") <= 1.1 * 120.0 + 1.5 * 60.0 + 1e-9);
}

#[test]
fn spec_example_e90_07_09_serviceability_accidental_and_seismic() {
    let sls = worked(AnnexChoice::De);
    close(value(&sls, "E90.19", "6.14b"), 50.0 + 20.0 + 0.6 * 10.0 + 0.5 * 8.0);
    close(value(&sls, "E90.19", "6.15b"), 50.0 + 0.5 * 20.0);
    close(value(&sls, "E90.19", "6.16b"), 50.0 + 0.3 * 20.0);
    let accidental = Case::new("a", AnnexChoice::De).with_s("actions", &format!("{WORKED};Ad|A|100"));
    close(value(&accidental, "E90.17", "6.11b"), 50.0 + 100.0 + 0.5 * 20.0);
    let seismic = Case::new("s", AnnexChoice::De).with_s("actions", &format!("{WORKED};E|E|40"));
    close(value(&seismic, "E90.18", "6.12b"), 50.0 + 40.0 + 0.3 * 20.0);
}

#[test]
fn spec_example_e90_15_bridge_traffic_group_is_one_leading_unit() {
    let actions = "G|G|1000;TS|Q|300|road_traffic|gr1a;UDL|Q|150|road_udl|gr1a;W|Q|80|wind_bridge";
    let case = Case::new("bridge", AnnexChoice::De).with_s("actions", actions);
    close(value(&case, "E90.26", "str-e610"), 1.35 * 1000.0 + 1.35 * (300.0 + 150.0) + 1.5 * 0.6 * 80.0);
    close(value(&case, "E90.26", "exclusive-groups"), 1.0);
}

#[test]
fn exclusive_groups_enter_at_most_once_per_combination() {
    let actions = Action::parse_list("G|G|100;A1|Q|10|road_traffic|gr1a;B1|Q|20|road_single_axle|gr1b;W|Q|5|wind_bridge").expect("actions");
    let ctx = Context::new(AnnexChoice::En, Domain::Bridge, FactorSet::StrB, 1.0);
    for combination in combinations(&ctx, &actions, Equation::E610).expect("combinations") {
        let traffic: Vec<&str> = combination.terms.iter().filter(|t| t.action == "A1" || t.action == "B1").map(|t| t.action.as_str()).collect();
        assert_eq!(traffic.len(), 1, "{}", combination.describe());
    }
}

#[test]
fn favourable_variable_actions_are_never_counted() {
    let actions = Action::parse_list("G|G|100;Q|Q|-30|office").expect("actions");
    let ctx = Context::new(AnnexChoice::De, Domain::Building, FactorSet::StrB, 1.0);
    close(governing(&combinations(&ctx, &actions, Equation::E610).expect("combinations"), false).expect("governing").total(), 135.0);
}

#[test]
fn k_fi_scales_unfavourable_factors_only() {
    let actions = Action::parse_list("Gs|G|100;Gi|G|-20").expect("actions");
    let ctx = Context::new(AnnexChoice::De, Domain::Building, FactorSet::StrB, 1.1);
    let total = governing(&combinations(&ctx, &actions, Equation::E610).expect("combinations"), false).expect("governing").total();
    close(total, 1.1 * 1.35 * 100.0 - 1.0 * 20.0);
    let rc3 = Case::new("k", AnnexChoice::De).with_n("cc", 3.0).with_n("gamma_f", 1.5);
    close(value(&rc3, "E90.02", "gamma-f-eff"), 1.1 * 1.5);
}

#[test]
fn reliability_tables_and_probabilities_match_table_b2() {
    for (rc, beta50, beta1, pf50) in [(1u8, 3.3, 4.2, 4.83e-4), (2, 3.8, 4.7, 7.23e-5), (3, 4.3, 5.2, 8.54e-6)] {
        close(target_reliability_index_for(rc, 50.0).expect("50 a"), beta50);
        close(target_reliability_index_for(rc, 1.0).expect("1 a"), beta1);
        assert!((phi_cdf(-beta50) / pf50 - 1.0).abs() < 0.01, "RC{rc}");
    }
    assert!(target_reliability_index_for(2, 100.0).expect("100 a") < 3.8);
    assert!(target_reliability_index_for(2, 10.0).expect("10 a") > 3.8);
    assert!(target_reliability_index_for(9, 50.0).is_none());
}

#[test]
fn normal_distribution_round_trips_and_matches_reference_points() {
    close(phi_cdf(0.0), 0.5);
    assert!((phi_cdf(1.959_963_984_540_054) - 0.975).abs() < 1e-12);
    for p in [1e-9, 1e-6, 0.01, 0.2, 0.5, 0.9, 0.999_999] {
        assert!((phi_cdf(phi_inv(p)) / p - 1.0).abs() < 1e-9, "{p}");
    }
    assert!((phi_cdf(-5.2) / 9.96e-8 - 1.0).abs() < 0.01);
}

#[test]
fn form_matches_the_closed_form_for_normal_variables() {
    let (r, e) = (Dist::Normal { mean: 100.0, sd: 10.0 }, Dist::Normal { mean: 60.0, sd: 12.0 });
    let result = form_r_minus_e(&r, &e);
    close(result.beta, 40.0 / (10.0_f64.powi(2) + 12.0_f64.powi(2)).sqrt());
    assert!(result.converged && result.alpha_r > 0.0 && result.alpha_e < 0.0);
    close(result.alpha_r.powi(2) + result.alpha_e.powi(2), 1.0);
}

#[test]
fn annex_c_alpha_regimes() {
    assert_eq!(annex_c_alphas(AnnexChoice::En, 1.0, 1.0), (0.8, -0.7));
    assert_eq!(annex_c_alphas(AnnexChoice::En, 10.0, 1.0), (0.4, -1.0));
    assert_eq!(annex_c_alphas(AnnexChoice::En, 1.0, 10.0), (1.0, -0.4));
}

#[test]
fn annex_d_k_n_table_and_characteristic_value() {
    close(k_n(6.0, false).expect("kn"), 2.18);
    close(k_n(10.0, true).expect("kn"), 1.72);
    assert!(k_n(2.0, false).is_none());
    let between = k_n(15.0, true).expect("kn");
    assert!(between < 1.72 && between > 1.68);
    let case = Case::new("d", AnnexChoice::De).with_n("n", 6.0).with_n("mean", 100.0).with_n("v", 0.08);
    close(value(&case, "E90.30", "x-k"), 100.0 * (1.0 - 2.18 * 0.08));
}

#[test]
fn palmgren_miner_follows_the_sn_curve() {
    close(en1993_endurance_cycles(100.0, 100.0).expect("N"), 2.0e6);
    assert!(en1993_endurance_cycles(1.0, 100.0).is_none());
    let damage = miner_damage(&[(100.0, 1.0e6), (100.0, 1.0e6)], 100.0);
    close(damage, 1.0);
    let case = Case::new("f", AnnexChoice::De).with_s("spectrum", "100:1000000;100:1000000").with_n("delta_sigma_c", 100.0);
    close(value(&case, "E90.21", "miner"), 1.0);
}

#[test]
fn serviceability_deflection_components_follow_figure_a1_1() {
    let case = Case::new("d", AnnexChoice::De).with_n("span", 6.0).with_n("w_c", 5.0).with_n("w1", 6.0).with_n("w2", 3.0).with_n("w3", 11.25);
    close(value(&case, "E90.24", "w-net-fin"), 15.25);
    let results = evaluate_case("E90.24", &case);
    assert!(results.iter().all(|c| matches!(c.status, CheckStatus::Pass | CheckStatus::UnverifiedBasis)), "{results:?}");
}

#[test]
fn every_registry_entry_reports_missing_inputs_instead_of_panicking() {
    for entry in registry() {
        let results = evaluate_case(entry.id, &Case::new("empty", AnnexChoice::De));
        assert!(!results.is_empty(), "{}", entry.id);
        assert!(results.iter().all(|c| c.status == CheckStatus::NotApplicable || entry.inputs.is_empty()), "{}", entry.id);
    }
    let ids: Vec<&str> = registry().iter().map(|e| e.id).collect();
    for n in 1..=30 {
        assert!(ids.contains(&format!("E90.{n:02}").as_str()), "E90.{n:02} missing");
    }
}

#[test]
fn reliability_index_check_fails_below_target_with_remedy() {
    let check = check_reliability_index_for(3.0, 2, 2, 50.0, AnnexChoice::De);
    assert_eq!(check.status, CheckStatus::Fail);
    assert!(!check.remedies.is_empty());
    assert_eq!(check_reliability_index_for(3.85, 2, 2, 50.0, AnnexChoice::De).status, CheckStatus::UnverifiedBasis);
    assert_eq!(check_reliability_index_for(3.85, 2, 7, 50.0, AnnexChoice::De).status, CheckStatus::NotApplicable);
}


#[test]
fn review_demo_a_gamma_follows_the_sign_of_the_effect() {
    let case = Case::new("a", AnnexChoice::De).with_s("actions", "G1|G|100;G2|G|-40;Q|Q|20|office");
    close(value(&case, "E90.16", "e610"), 1.35 * 100.0 + 1.0 * -40.0 + 1.5 * 20.0);
}

#[test]
fn review_demo_b_reversal_envelope_uses_the_minimum_direction() {
    let ctx = Context::new(AnnexChoice::De, Domain::Building, FactorSet::StrB, 1.0);
    let actions = Action::parse_list("G|G|30;W|Q|-80|wind").expect("actions");
    close(governing(&combinations(&ctx, &actions, Equation::E610).expect("max"), false).expect("max").total(), 1.35 * 30.0);
    let low = combinations_dir(&ctx, &actions, Equation::E610, -1.0).expect("min");
    close(governing_min(&low).expect("min").total(), 1.0 * 30.0 + 1.5 * -80.0);
}

#[test]
fn gamma_sign_regression_adverse_takes_sup_relieving_takes_inf_in_both_directions() {
    let ctx = Context::new(AnnexChoice::De, Domain::Building, FactorSet::StrB, 1.0);
    let actions = Action::parse_list("G1|G|100;G2|G|-40").expect("actions");
    let (g_sup, g_inf) = gamma_g(AnnexChoice::De, Domain::Building, FactorSet::StrB);
    close(governing(&combinations(&ctx, &actions, Equation::E610).expect("max"), false).expect("max").total(), g_sup * 100.0 + g_inf * -40.0);
    close(governing_min(&combinations_dir(&ctx, &actions, Equation::E610, -1.0).expect("min")).expect("min").total(), g_inf * 100.0 + g_sup * -40.0);
    let max = combinations(&ctx, &actions, Equation::E610).expect("max");
    let factors: Vec<f64> = max[0].terms.iter().map(|term| term.factor).collect();
    assert_eq!(factors, vec![g_sup, g_inf]);
    let min = combinations_dir(&ctx, &actions, Equation::E610, -1.0).expect("min");
    let factors: Vec<f64> = min[0].terms.iter().map(|term| term.factor).collect();
    assert_eq!(factors, vec![g_inf, g_sup]);
}

#[test]
fn quadrature_failure_probability_matches_the_closed_form_of_two_normals_and_the_cut_off_ranges_the_s_n_curve() {
    let (resistance, action) = (Dist::Normal { mean: 300.0, sd: 30.0 }, Dist::Normal { mean: 150.0, sd: 30.0 });
    let beta = -phi_inv(exact_failure_probability(&resistance, &action));
    assert!((beta - 150.0 / (2.0_f64 * 900.0).sqrt()).abs() < 1e-5, "{beta}");
    let (limit, cut_off) = en1993_cut_off_ranges(71.0e6);
    assert!((limit - 52_313_247.28069349).abs() < 1.0 && (cut_off - 28_734_634.67739296).abs() < 1.0);
    assert!(en1993_endurance_cycles(limit * 1.0000001, 71.0e6).is_some() && en1993_endurance_cycles(cut_off * 0.999, 71.0e6).is_none());
}

#[test]
fn one_permanent_action_is_unfavourable_in_one_span_and_favourable_in_the_other() {
    let (g, q) = (20000.0, 15000.0);
    let beam = SpanBeam { spans: &[6.0, 6.0], e: 33.0e9, area: 0.15, inertia: 0.003125 };
    let factors = SpanFactors { gamma_g: 1.35, gamma_g_inf: 1.0, gamma_q: 1.5 };
    let envelope = continuous_beam_envelope(&beam, g, q, &factors).expect("envelope");
    let three_moment = |w0: f64, w1: f64| {
        let hogging = (w0 + w1) * 6.0_f64.powi(3) / (8.0 * 12.0);
        let reaction = w0 * 3.0 - hogging / 6.0;
        (hogging, reaction * reaction / (2.0 * w0), w0 * 3.0 + w1 * 3.0 + 2.0 * hogging / 6.0)
    };
    let (_, sagging, _) = three_moment(1.35 * g + 1.5 * q, 1.0 * g);
    let (_, same_factor, _) = three_moment(1.35 * g + 1.5 * q, 1.35 * g);
    assert!(sagging > same_factor * 1.04);
    assert!((envelope.sagging[0] - sagging).abs() <= 1e-3 * sagging && (envelope.sagging[1] - sagging).abs() <= 1e-3 * sagging, "{:?} vs {sagging}", envelope.sagging);
    let (hogging, _, support) = three_moment(1.35 * g + 1.5 * q, 1.35 * g + 1.5 * q);
    assert!((envelope.hogging - hogging).abs() <= 1e-3 * hogging && (envelope.reaction_max[1] - support).abs() <= 1e-3 * support);
}

#[test]
fn unknown_material_has_no_partial_factor() {
    assert!(NaDe.gamma_m("masonry").is_nan() && NaEn.gamma_m("glass").is_nan());
    close(NaDe.gamma_m("concrete"), 1.5);
}

#[test]
fn national_annex_imposed_categories_map_to_the_table_rows() {
    for (code, row) in [("A1", "residential"), ("B2", "office"), ("C3", "congregation"), ("D1", "retail"), ("E1", "storage"), ("T1", "residential"), ("Z", "residential")] {
        assert_eq!(psi_lookup(AnnexChoice::De, code), psi_lookup(AnnexChoice::De, row), "{code}");
    }
    assert!(psi_lookup(AnnexChoice::De, "Q9").is_none());
}

#[test]
fn form_without_convergence_is_indeterminate() {
    let case = Case::new("f", AnnexChoice::De).with_n("r_mean", 100.0).with_n("r_sd", 10.0).with_n("e_mean", 60.0).with_n("e_sd", 12.0).with_s("r_dist", "normal").with_s("e_dist", "normal");
    assert!(evaluate_outcomes("E90.29", &case).is_ok());
    let zero = Case::new("z", AnnexChoice::De).with_n("span", 0.0);
    assert!(evaluate_outcomes("E90.24", &zero).is_err());
}

#[test]
fn annex_switch_reads_the_registry_and_changes_only_the_expected_values() {
    let bridge = psi_entry_of("temperature_bridge").expect("row");
    assert_eq!((bridge.row(AnnexChoice::En).psi_0, bridge.row(AnnexChoice::De).psi_0), (0.6, 0.8));
    assert_eq!(psi_lookup(AnnexChoice::En, "office"), psi_lookup(AnnexChoice::De, "office"));
    assert_eq!(gamma_g(AnnexChoice::En, Domain::Building, FactorSet::StrB), gamma_g(AnnexChoice::De, Domain::Building, FactorSet::StrB));
    assert!(!bridge.verified(), "registry values stay unverified until double-keyed");
}

fn within(left: f64, right: f64, tolerance: f64) {
    assert!((left - right).abs() <= tolerance * right.abs().max(1.0), "{left} != {right}");
}

#[test]
fn form_of_two_normal_actions_is_the_closed_form_and_reports_the_implied_partial_factors() {
    let (r, g, q) = (Dist::Normal { mean: 500.0, sd: 50.0 }, Dist::Normal { mean: 200.0, sd: 10.0 }, Dist::Normal { mean: 100.0, sd: 40.0 });
    let form = form_multi(&r, &[g, q]);
    let norm = (50.0_f64.powi(2) + 10.0_f64.powi(2) + 40.0_f64.powi(2)).sqrt();
    assert!(form.converged);
    within(form.beta, 200.0 / norm, 1e-12);
    within(form.alpha_r, 50.0 / norm, 1e-12);
    within(form.alpha_e[1], -40.0 / norm, 1e-12);
    within(form.design_e[1], 100.0 + 40.0 / norm * form.beta * 40.0, 1e-12);
    let two = form_r_minus_e(&r, &Dist::Normal { mean: 300.0, sd: (10.0_f64.powi(2) + 40.0_f64.powi(2)).sqrt() });
    within(form_multi(&r, &[Dist::Normal { mean: 300.0, sd: (10.0_f64.powi(2) + 40.0_f64.powi(2)).sqrt() }]).beta, two.beta, 1e-12);
    let case = Case::new("two", AnnexChoice::De).with_s("r_dist", "normal").with_s("e_dist", "normal").with_s("e2_dist", "normal").with_n("r_mean", 500.0).with_n("r_sd", 50.0).with_n("e_mean", 200.0).with_n("e_sd", 10.0).with_n("e2_mean", 100.0).with_n("e2_sd", 40.0).with_n("r_k", 420.0).with_n("e_k", 200.0).with_n("e2_k", 160.0);
    within(value(&case, "E90.29", "beta"), 3.0860669992418384, 1e-9);
    within(value(&case, "E90.29", "gamma-r"), 1.1024999968008158, 1e-7);
    within(value(&case, "E90.29", "gamma-e2"), 1.1011904840590572, 1e-7);
}

#[test]
fn form_of_a_lognormal_resistance_against_normal_and_gumbel_actions_matches_scipy() {
    let case = Case::new("gumbel", AnnexChoice::En).with_s("r_dist", "lognormal").with_s("e_dist", "normal").with_s("e2_dist", "gumbel").with_n("r_mean", 300.0).with_n("r_sd", 30.0).with_n("e_mean", 100.0).with_n("e_sd", 10.0).with_n("e2_mean", 60.0).with_n("e2_sd", 18.0).with_n("e2_k", 110.0);
    within(value(&case, "E90.29", "beta"), 3.5236976164761438, 2e-6);
    within(value(&case, "E90.29", "alpha-e2"), -0.864350127572305, 2e-6);
    within(value(&case, "E90.29", "gamma-e2"), 1.3340640106563628, 2e-6);
}

#[test]
fn annex_d_standard_evaluation_reproduces_the_numpy_fit() {
    let case = Case::new("d72", AnnexChoice::De).with_s("pairs", "412:400;388:380;455:430;352:360;431:410;398:390;376:370;423:405").with_n("v_rt", 0.09).with_n("g_rt_mean", 400.0).with_n("eta_d", 0.9).with_n("gamma_m", 1.25);
    within(value(&case, "E90.30", "b"), 1.029797786334953, 1e-12);
    within(value(&case, "E90.30", "v-delta"), 0.024861024717758494, 1e-9);
    within(value(&case, "E90.30", "v-r"), 0.09339741389070758, 1e-9);
    within(value(&case, "E90.30", "r-k"), 351.1657343811248, 1e-9);
    within(value(&case, "E90.30", "r-d"), 252.83932875440988, 1e-9);
    assert!(standard_evaluation(&[(1.0, 1.0), (2.0, 2.0)], 0.1, 1.0).is_err());
    assert!(evaluate_outcomes("E90.30", &case.clone().with_s("pairs", "1:0;2:1;3:1")).is_err());
}

#[test]
fn table_d1_nodes_follow_the_prediction_fractile_closed_form_within_the_print_rounding() {
    for n in [3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 20.0, 30.0] {
        let known = phi_inv(0.95) * (1.0 + 1.0 / n).sqrt();
        assert!((k_n(n, true).expect("kn") - known).abs() < 0.02, "n = {n}");
    }
}

#[test]
fn annex_c7_fixed_alphas_give_the_accompanying_action_forty_percent_of_the_leading_sensitivity() {
    let case = Case::new("c7", AnnexChoice::De).with_s("r_dist", "normal").with_s("e_dist", "normal").with_s("e2_dist", "normal").with_n("r_mean", 500.0).with_n("r_sd", 50.0).with_n("e_mean", 200.0).with_n("e_sd", 10.0).with_n("e2_mean", 100.0).with_n("e2_sd", 40.0);
    within(value(&case, "E90.29", "r-d-fixed"), 500.0 - 0.8 * 3.8 * 50.0, 1e-12);
    within(value(&case, "E90.29", "e-lead-fixed"), 100.0 + 0.7 * 3.8 * 40.0, 1e-12);
    within(value(&case, "E90.29", "e-second-fixed"), 200.0 + 0.28 * 3.8 * 10.0, 1e-12);
    within(annex_c_secondary_alpha(AnnexChoice::En, -0.7), -0.28, 1e-12);
}

#[test]
fn table_d2_nodes_bound_the_closed_form_fractile_factor_and_feed_the_direct_design_value() {
    for n in [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 20.0, 30.0, f64::INFINITY] {
        let closed = 0.8 * 3.8 * (1.0 + if n.is_finite() { 1.0 / n } else { 0.0 }).sqrt();
        let printed = k_d_n(n, true).expect("kd");
        assert!(printed >= closed - 1e-12 && printed - closed < 0.07, "n = {n}: {printed} vs {closed}");
    }
    assert!(k_d_n(5.0, false).is_none() && k_d_n(0.5, true).is_none());
    let between = k_d_n(7.0, true).expect("kd");
    assert!(between < k_d_n(6.0, true).expect("kd") && between > k_d_n(8.0, true).expect("kd"));
    let case = Case::new("d2", AnnexChoice::De).with_n("n", 10.0).with_n("mean", 100.0).with_n("v", 0.1).with_n("v_known", 1.0).with_n("eta_d", 0.9);
    within(value(&case, "E90.30", "k-d-n"), 3.23, 1e-12);
    within(value(&case, "E90.30", "x-d-direct"), 0.9 * 100.0 * (1.0 - 3.23 * 0.1), 1e-12);
}
