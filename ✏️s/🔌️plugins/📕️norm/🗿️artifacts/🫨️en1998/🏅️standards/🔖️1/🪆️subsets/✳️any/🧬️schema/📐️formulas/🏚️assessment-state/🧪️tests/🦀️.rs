//! 🧪 Worked examples of the assessment state kernel, recomputed by hand.

use super::*;
use semio_s_plugin_norm_compliance::AnnexChoice;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1.0e-6 * b.abs().max(1.0), "{a} vs {b}");
}

fn member() -> RcMember {
    RcMember { h: 0.5, b: 0.3, d: 0.46, l_v: 1.75, n: 3.0e5, a_s1: 0.0012, a_s2: 0.0008, a_si: 0.0, d_b: 0.02, f_y: 500.0, f_c: 30.0, a_sx: 1.0e-4, s_h: 0.1, f_yw: 500.0, b_0: 0.22, h_0: 0.42, restrained: 6.0, b_i: 0.15, rho_d: 0.0, shear_cracking: false }
}

#[test]
fn knowledge_level_gates_the_analysis_methods() {
    assert_eq!(AnalysisMethod::permitted_methods(KnowledgeLevel::Kl1), vec![AnalysisMethod::LateralForce, AnalysisMethod::ModalResponseSpectrum]);
    assert_eq!(AnalysisMethod::permitted_methods(KnowledgeLevel::Kl2).len(), 4);
    assert_eq!(AnalysisMethod::permitted_methods(KnowledgeLevel::Kl3).len(), 4);
    assert!(!AnalysisMethod::NonlinearStatic.permitted_at(KnowledgeLevel::Kl1));
    assert_eq!(AnalysisMethod::NonlinearTimeHistory.token(), "nlth");
}

#[test]
fn ductile_strengths_divide_by_cf_and_brittle_ones_also_by_the_partial_factor() {
    near(Mechanism::Ductile.design_strength(30.0, 1.2, 1.5), 25.0);
    near(Mechanism::Brittle.design_strength(30.0, 1.2, 1.5), 30.0 / 1.8);
    assert_eq!(Mechanism::Brittle.design_strength(30.0, 0.0, 1.5), 0.0);
}

#[test]
fn the_verification_rule_is_a_ratio_of_at_most_one() {
    assert!(verified(demand_capacity_ratio(0.9, 1.0)));
    assert!(verified(demand_capacity_ratio(1.0, 1.0)));
    assert!(!verified(demand_capacity_ratio(1.1, 1.0)));
    assert!(demand_capacity_ratio(1.0, 0.0).is_infinite());
    assert_eq!(demand_capacity_ratio(0.0, 0.0), 0.0);
}

#[test]
fn plastic_ductility_is_zero_below_yield() {
    near(plastic_ductility(0.015, 0.005), 2.0);
    assert_eq!(plastic_ductility(0.004, 0.005), 0.0);
    assert_eq!(plastic_ductility(0.01, 0.0), 0.0);
}

#[test]
fn cyclic_shear_capacity_matches_the_hand_calculation() {
    let k = ShearCoefficients { axial_limit: 0.55, ductility_cap: 5.0, ductility_slope: 0.05, base: 0.16, rho_min: 0.5, slenderness_slope: 0.16, slenderness_cap: 5.0 };
    let brittle = RcMember { f_c: Mechanism::Brittle.design_strength(30.0, 1.2, 1.5), f_yw: Mechanism::Brittle.design_strength(500.0, 1.2, 1.15), ..member() };
    let v = cyclic_shear_capacity(&brittle, 0.1, 2.0, 1.0, &k);
    near(v.axial_n, 34285.714285714);
    near(v.concrete_n, 57481.3592979);
    near(v.stirrup_n, 152173.913043);
    near(v.degradation, 0.9);
    near(v.resistance_n, 222975.459392);
}

#[test]
fn ductility_degrades_the_shear_capacity_up_to_the_cap() {
    let k = ShearCoefficients { axial_limit: 0.55, ductility_cap: 5.0, ductility_slope: 0.05, base: 0.16, rho_min: 0.5, slenderness_slope: 0.16, slenderness_cap: 5.0 };
    let elastic = cyclic_shear_capacity(&member(), 0.1, 0.0, 1.0, &k).resistance_n;
    let capped = cyclic_shear_capacity(&member(), 0.1, 5.0, 1.0, &k).resistance_n;
    let beyond = cyclic_shear_capacity(&member(), 0.1, 9.0, 1.0, &k).resistance_n;
    assert!(elastic > capped);
    near(capped, beyond);
    assert_eq!(cyclic_shear_capacity(&member(), 0.1, 0.0, 0.0, &k).resistance_n, 0.0);
}

#[test]
fn tension_gives_no_axial_term() {
    let k = ShearCoefficients::from_registry(AnnexChoice::De);
    assert_eq!(cyclic_shear_capacity(&RcMember { n: -1.0e5, ..member() }, 0.1, 0.0, 1.0, &k).axial_n, 0.0);
}

#[test]
fn the_neutral_axis_depth_comes_from_the_fibre_section_at_the_yield_curvature() {
    let k = AnnexA::from_registry(AnnexChoice::De);
    let m = RcMember { h: 0.5, b: 0.3, d: 0.45, l_v: 2.0, n: 1.0e5, a_s1: 0.0018, a_s2: 0.0009, a_si: 0.0, d_b: 0.02, f_y: 500.0 / 1.2, f_c: 30.0 / 1.2, a_sx: 0.0, s_h: 0.0, f_yw: 0.0, b_0: 0.0, h_0: 0.0, restrained: 0.0, b_i: 0.0, rho_d: 0.0, shear_cracking: false };
    let x = m.yield_neutral_axis_depth_m(&k).expect("fibre section");
    near(x, 0.1767513282874565);
    assert!(x > 0.0 && x < m.h);
    assert!(RcMember { f_c: 80.0, ..m }.yield_neutral_axis_depth_m(&k).is_none(), "outside the strength range of the registry rows");
}

#[test]
fn the_basis_reads_cf_and_the_return_period_from_the_registry() {
    let mut a = semio_s_artifact_norm_en1998_model::En1998Snapshot::full_showcase().assessments[0].clone();
    (a.knowledge_level, a.limit_state) = (KnowledgeLevel::Kl1, semio_s_artifact_norm_en1998_model::LimitState::Nc);
    let nc = basis(&a, AnnexChoice::En);
    near(nc.cf, 1.35);
    near(nc.return_period_yr, 2475.0);
    near(nc.ag_factor, (2475.0_f64 / 475.0).powf(1.0 / 3.0));
    a.limit_state = semio_s_artifact_norm_en1998_model::LimitState::Dl;
    near(basis(&a, AnnexChoice::En).cf, 1.0);
    near(basis(&a, AnnexChoice::En).cf_table, 1.35);
    near(basis(&a, AnnexChoice::En).return_period_yr, 225.0);
}

#[test]
fn the_german_annex_fixes_the_return_period_of_every_limit_state() {
    let mut a = semio_s_artifact_norm_en1998_model::En1998Snapshot::full_showcase().assessments[0].clone();
    for state in [semio_s_artifact_norm_en1998_model::LimitState::Nc, semio_s_artifact_norm_en1998_model::LimitState::Sd, semio_s_artifact_norm_en1998_model::LimitState::Dl] {
        a.limit_state = state;
        let de = basis(&a, AnnexChoice::De);
        near(de.return_period_yr, 475.0);
        near(de.ag_factor, 1.0);
    }
    a.limit_state = semio_s_artifact_norm_en1998_model::LimitState::Nc;
    near(basis(&a, AnnexChoice::De).cf_table, basis(&a, AnnexChoice::En).cf_table);
    assert!(basis(&a, AnnexChoice::En).ag_factor > 1.7);
}

#[test]
fn the_member_ratios_follow_the_demands_of_both_mechanisms() {
    let mut a = semio_s_artifact_norm_en1998_model::En1998Snapshot::full_showcase().assessments.iter().find(|a| a.id == "as-2").cloned().expect("as-2");
    a.shear_force_demand_n = 0.0;
    let ductile = member_ratios(&a, AnnexChoice::De);
    assert!(ductile.chord.is_some() && ductile.shear.is_none());
    a.shear_force_demand_n = 1.0e5;
    let both = member_ratios(&a, AnnexChoice::De);
    let shear = both.shear.expect("brittle ratio");
    near(shear, 1.0e5 / both.shear_capacity_n);
    near(both.governing().expect("governing"), both.chord.expect("ductile ratio").max(shear));
}

#[test]
fn the_total_longitudinal_ratio_counts_the_intermediate_bars_with_both_end_layers() {
    let k = ShearCoefficients { axial_limit: 0.55, ductility_cap: 5.0, ductility_slope: 0.05, base: 0.16, rho_min: 0.5, slenderness_slope: 0.16, slenderness_cap: 5.0 };
    let ends = cyclic_shear_capacity(&member(), 0.1, 0.0, 1.0, &k);
    let all = cyclic_shear_capacity(&RcMember { a_si: 0.0004, ..member() }, 0.1, 0.0, 1.0, &k);
    near(all.concrete_n, 1.2 * ends.concrete_n);
    near(all.stirrup_n, ends.stirrup_n);
    let thin = cyclic_shear_capacity(&RcMember { a_s1: 0.0001, a_s2: 0.0, a_si: 0.0, ..member() }, 0.1, 0.0, 1.0, &k);
    let floor = cyclic_shear_capacity(&RcMember { a_s1: 0.0, a_s2: 0.0, a_si: 0.0001, ..member() }, 0.1, 0.0, 1.0, &k);
    near(thin.concrete_n, floor.concrete_n);
}
