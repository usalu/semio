use crate::*;
use semio_s_plugin_norm_compliance::{AnnexChoice, ReliabilityClass};
use semio_s_artifact_norm_en1994_model::{CompositeBeam, ProfiledSheeting, SteelSection, Stud};

fn sample_beam() -> CompositeBeam {
    let mut beam = CompositeBeam::default_example();
    beam.id = "beam-B1".into();
    beam
}

fn uniform_studs(beam: &CompositeBeam, count: u32) -> Vec<Stud> {
    let pitch = beam.span_m / f64::from(count.max(1));
    Stud::typical_row(&beam.id, beam.span_m / 2.0, pitch, count)
}

#[test]
fn effective_width_8m_span() {
    let beff = part_1_1::effective_width_m(8.0, 0.300, 3.0);
    assert!((beff - 2.30).abs() < 0.01);
}

#[test]
fn stud_connector_resistance_worked_example() {
    let mut beam = sample_beam();
    beam.sheeting.height_m = 0.0;
    beam.concrete_f_ck_pa = 30e6;
    beam.concrete_e_cm_pa = 33e9;
    let studs = part_studs::of(&beam, &uniform_studs(&beam, 40), AnnexChoice::En);
    let p_rd = part_1_1::connector_resistance_n(&studs);
    assert!((p_rd / 1000.0 - 81.656).abs() < 0.05, "P_Rd={}", p_rd / 1000.0);
    assert!(studs.rows.iter().all(|s| (s.reduction - 1.0).abs() < 1e-12 && (s.p_rd_n - p_rd).abs() < 1e-9));
}

#[test]
fn min_shear_connection_degree_span_8m() {
    let eta_min = part_1_1::min_shear_connection_degree(8.0, 355e6);
    assert!((eta_min - 0.49).abs() < 1e-6);
}

#[test]
fn plastic_moment_partial_equation() {
    let m = part_1_1::plastic_moment_partial_nm(80e3, 250e3, 0.75);
    assert!((m - 207.5e3).abs() < 1.0);
}

#[test]
fn de_bridge_gamma_mf_stricter_than_en() {
    let en = AnnexParams::en().gamma_mf;
    let de = AnnexParams::de().gamma_mf;
    assert!((en - 1.15).abs() < 1e-9);
    assert!((de - 1.35).abs() < 1e-9);
    let lim_en = part_2::steel_fatigue_resistance_pa(semio_s_artifact_norm_en1994_model::FatigueDetail::StudWelded, 2.0e6, AnnexChoice::En);
    let lim_de = part_2::steel_fatigue_resistance_pa(semio_s_artifact_norm_en1994_model::FatigueDetail::StudWelded, 2.0e6, AnnexChoice::De);
    assert!(lim_de < lim_en);
}

#[test]
fn de_gamma_m1_stricter_than_en() {
    assert!((AnnexParams::en().gamma_m1 - 1.0).abs() < 1e-9);
    assert!((AnnexParams::de().gamma_m1 - 1.1).abs() < 1e-9);
    let beam = sample_beam();
    let m_en = part_1_1::ltb_moment_resistance_nm(&beam, 355e6, AnnexChoice::En);
    let m_de = part_1_1::ltb_moment_resistance_nm(&beam, 355e6, AnnexChoice::De);
    assert!(m_de < m_en);
    assert!((m_en / m_de - 1.1).abs() < 1e-9);
}

#[test]
fn de_deflection_limit_tighter_than_en() {
    let lim_en = part_1_1::deflection_limit_m(8.0, AnnexChoice::En);
    let lim_de = part_1_1::deflection_limit_m(8.0, AnnexChoice::De);
    assert!((lim_en - 8.0 / 250.0).abs() < 1e-12);
    assert!((lim_de - 8.0 / 300.0).abs() < 1e-12);
    assert!(lim_de < lim_en);
}

#[test]
fn bridge_fatigue_uses_detail_as_strength_not_as_mpa_limit() {
    let limit = part_2::steel_fatigue_resistance_pa(semio_s_artifact_norm_en1994_model::FatigueDetail::StudWelded, 2.0e6, AnnexChoice::En);
    assert!((limit / 1e6 - 80.0 / 1.15).abs() < 0.1);
}

#[test]
fn plastic_moment_has_no_empirical_factor_and_matches_spec_e1_02() {
    let mut beam = sample_beam();
    beam.steel = SteelSection { designation: "IPE300".into(), height_m: 0.300, width_m: 0.150, tw_m: 0.0071, tf_m: 0.0107, a_m2: 5381e-6, w_pl_y_m3: 628.4e-6, i_y_m4: 83.56e-6, a_v_m2: 2568e-6, r_m: 0.015 };
    beam.sheeting.height_m = 0.0;
    beam.slab_thickness_m = 0.100;
    beam.concrete_f_ck_pa = 30e6;
    let m = part_1_1::full_plastic_moment_nm(&beam, 2.0, 355e6, AnnexChoice::En);
    assert!((m / 1e3 - 423.9).abs() < 0.3, "M_pl,Rd={}", m / 1e3);
}

#[test]
fn stud_demand_follows_the_applied_moment() {
    use crate::catalogue as cat;
    let sec = cat::ISection { h: 0.300, b: 0.150, tf: 0.0107, tw: 0.0071, a: 5381e-6, i_y: 83.56e-6 };
    let low = cat::required_concrete_force(&sec, 355e6, 0.1, 2.0, 0.1, 17e6, 250e3);
    let mid = cat::required_concrete_force(&sec, 355e6, 0.1, 2.0, 0.1, 17e6, 330e3);
    let high = cat::required_concrete_force(&sec, 355e6, 0.1, 2.0, 0.1, 17e6, 420e3);
    assert!(low < mid && mid < high && low > 0.0);
}

#[test]
fn ltb_uses_the_elastic_critical_moment() {
    use crate::catalogue as cat;
    let beam = sample_beam();
    let sec = part_1_1::isection(&SteelSection::resolve(&beam.steel));
    let mcr = cat::mcr_bare(&sec, beam.ltb_length_m);
    let (lam, chi) = cat::chi_lt(beam.steel.w_pl_y_m3 * 355e6, mcr, sec.h / sec.b, false);
    let expected = chi * beam.steel.w_pl_y_m3 * 355e6 / AnnexParams::de().gamma_m1;
    assert!(lam > 0.0);
    assert!((part_1_1::ltb_moment_resistance_nm(&beam, 355e6, AnnexChoice::De) - expected).abs() < 1.0);
}

#[test]
fn composite_slab_mk_without_max_of_methods() {
    let slab = semio_s_artifact_norm_en1994_model::CompositeSlab::default_example();
    let v = part_slab::longitudinal_shear_mk_n_per_m(&slab);
    let s = part_slab::section(&slab, AnnexChoice::En);
    let expected = s.dp() / 1.25 * (slab.m_factor * 1e6 * s.ap / (slab.span_m / 4.0) + slab.k_factor * 1e6);
    assert!((v - expected).abs() < 1e-6 * expected);
}

#[test]
fn two_span_statics_follow_the_exact_coefficients() {
    use semio_s_artifact_norm_en1994_model::CharacteristicAction;
    let l = 10.0;
    let g = CharacteristicAction::permanent_area("G", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "self", 1000.0);
    let q = CharacteristicAction::imposed_area("Q", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 1000.0);
    let w = 1000.0;
    let near = |a: f64, b: f64| (a - b).abs() <= 1e-9 * b.abs().max(1.0);
    let (mg, hg, vg, _) = part_en1990::action_internals(&g, l, l, semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span, 1.0);
    assert!(near(mg, 9.0 / 128.0 * w * l * l) && near(hg, w * l * l / 8.0) && near(vg, 5.0 * w * l / 8.0), "{mg} {hg} {vg}");
    let (mq, _, _, _) = part_en1990::action_internals(&q, l, l, semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span, 1.0);
    assert!(near(mq, 49.0 / 512.0 * w * l * l), "{mq}");
    let p = CharacteristicAction::point_force("P", semio_s_artifact_norm_en1994_model::ActionStage::Composite, semio_s_artifact_norm_en1994_model::ActionKind::Permanent, "self", 100e3);
    let (mp, hp, vp, _) = part_en1990::action_internals(&p, l, l, semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span, 1.0);
    assert!(near(mp, 5.0 / 32.0 * 100e3 * l) && near(hp, 3.0 / 16.0 * 100e3 * l) && near(vp, 11.0 / 16.0 * 100e3), "{mp} {hp} {vp}");
}

#[test]
fn a_point_force_off_mid_span_on_continuous_spans_follows_the_pynite_solution() {
    use semio_s_artifact_norm_en1994_model::CharacteristicAction;
    let force = |x_m: f64, kind: semio_s_artifact_norm_en1994_model::ActionKind| CharacteristicAction { x_m, ..CharacteristicAction::point_force("P", semio_s_artifact_norm_en1994_model::ActionStage::Composite, kind, "B", 100e3) };
    let close = |got: (f64, f64, f64, f64), want: (f64, f64, f64)| (got.0 - want.0).abs() <= 1e-4 * want.0.abs().max(1.0) && (got.1 - want.1).abs() <= 1e-4 * want.1.abs().max(1.0) && (got.2 - want.2).abs() <= 1e-4 * want.2.abs().max(1.0);
    let end = part_en1990::action_internals(&force(3.0, semio_s_artifact_norm_en1994_model::ActionKind::Imposed), 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan, 1.0);
    assert!(close(end, (157_752.40, 115_948.02, 52_584.13)), "{end:?}");
    let mid = part_en1990::action_internals(&force(0.0, semio_s_artifact_norm_en1994_model::ActionKind::Imposed), 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan, 1.0);
    assert!((mid.0 - end.0).abs() > 1e3 && (mid.1 - end.1).abs() > 1e3, "{mid:?} {end:?}");
    let interior = part_en1990::action_internals(&force(2.0, semio_s_artifact_norm_en1994_model::ActionKind::Permanent), 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::InteriorSpan, 1.0);
    assert!(close(interior, (71_300.55, 83_777.57, 77_539.06)), "{interior:?}");
    let tip = part_en1990::action_internals(&force(1.5, semio_s_artifact_norm_en1994_model::ActionKind::Permanent), 2.0, 6.0, semio_s_artifact_norm_en1994_model::SupportCondition::Cantilever, 1.0);
    assert!(close(tip, (0.0, 150_000.0, 100_000.0)), "{tip:?}");
}

#[test]
fn unequal_interior_and_cantilever_statics_match_the_finite_element_solution() {
    use semio_s_artifact_norm_en1994_model::CharacteristicAction;
    let g = CharacteristicAction::permanent_area("G", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "self", 1000.0);
    let q = CharacteristicAction::imposed_area("Q", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 1000.0);
    let close = |a: f64, b: f64| (a - b).abs() < 2e-3 * b.abs().max(1.0);
    let (mg, hg, vg, _) = part_en1990::action_internals(&g, 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan, 1.0);
    assert!(close(mg, 5230.5) && close(hg, 6125.0) && close(vg, 8e3 - 3234.375), "{mg} {hg} {vg}");
    let (mq, _, _, _) = part_en1990::action_internals(&q, 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan, 1.0);
    assert!(close(mq, 5727.7));
    let (mi, hi, _, _) = part_en1990::action_internals(&g, 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::InteriorSpan, 1.0);
    assert!(close(mi, 3316.2) && close(hi, 4684.0), "{mi} {hi}");
    let (mm, hm, _, _) = part_en1990::action_internals(&q, 8.0, 5.0, semio_s_artifact_norm_en1994_model::SupportCondition::InteriorSpan, 1.0);
    assert!(close(mm, 4235.3) && close(hm, 5092.4), "{mm} {hm}");
    let (mc, hc, vc, _) = part_en1990::action_internals(&g, 2.0, 6.0, semio_s_artifact_norm_en1994_model::SupportCondition::Cantilever, 1.0);
    assert!(mc == 0.0 && close(hc, 2000.0) && close(vc, 2000.0));
    let (m0, h0, _, _) = part_en1990::action_internals(&g, 8.0, 0.0, semio_s_artifact_norm_en1994_model::SupportCondition::Continuous2Span, 1.0);
    assert!(close(m0, 9.0 / 128.0 * 1000.0 * 64.0) && close(h0, 8000.0));
}

/// 🪜️ EN 1990 A1.3.1 on a two-span beam 6 m + 6 m (G 20, Q 15 kN/m, 1.35 / 1.0 / 1.5): the permanent load of the neighbouring span relieves the sagging moment and takes γG,inf, 151 423.65 N·m (PyNite, anastruct and the three-moment closed form) instead of the 145 000.36 N·m with γG,sup on both spans; the hogging moment at the support keeps γG,sup on both spans.
#[test]
fn the_permanent_load_of_the_neighbouring_span_takes_the_favourable_factor_in_the_sagging_combination() {
    use semio_s_artifact_norm_en1994_model::CharacteristicAction;
    let actions = [CharacteristicAction::permanent_area("G", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "self", 20_000.0), CharacteristicAction::imposed_area("Q", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 15_000.0)];
    let effects = part_en1990::uls_composite(&actions, 6.0, 6.0, semio_s_artifact_norm_en1994_model::SupportCondition::EndSpan, 1.0, AnnexChoice::En, semio_s_plugin_norm_compliance::ReliabilityClass::RC2);
    assert!((effects.m_nm - 151_423.65).abs() < 1e-3 * 151_423.65, "{}", effects.m_nm);
    assert!(effects.m_nm > 1.04 * 145_000.36);
    assert!((effects.m_hog_nm - 222_750.0).abs() < 1e-3 * 222_750.0, "{}", effects.m_hog_nm);
}

#[test]
fn annex_switch_reads_the_registry_and_changes_only_gamma_m1_and_gamma_mf() {
    let (en, de) = (AnnexParams::en(), AnnexParams::de());
    assert_eq!((en.gamma_m1, de.gamma_m1, en.gamma_mf, de.gamma_mf), (1.0, 1.1, 1.15, 1.35));
    assert_eq!((en.gamma_v, en.gamma_c, en.gamma_s, en.gamma_m0), (de.gamma_v, de.gamma_c, de.gamma_s, de.gamma_m0));
}

#[test]
fn section_class_includes_the_root_radius_in_the_outstand_ratios() {
    let heb300 = SteelSection::heb300();
    let r = heb300.root_radius_m();
    assert_eq!(r, 0.027, "the HEB300 catalogue row stores 27 mm");
    assert!(["HEB320", "HEB400", "HEB500", "HEB700"].iter().all(|d| SteelSection::from_catalogue(d).is_some_and(|s| s.r_m == 0.027)));
    assert_eq!(SteelSection { r_m: 0.02, ..SteelSection::heb300() }.root_radius_m(), 0.02, "the stored leaf wins over the catalogue-implied radius");
    let implied = SteelSection { designation: "CUSTOM".into(), r_m: 0.0, ..SteelSection::heb300() }.root_radius_m();
    assert!((implied - r).abs() < r * 0.02, "radius {implied} m implied by the catalogue area against the tabulated {r} m");
    assert!(["HEB300", "HEB450", "HEB700"].iter().all(|d| SteelSection::from_catalogue(d).is_some_and(|s| ((SteelSection { designation: "X".into(), r_m: 0.0, ..s }).root_radius_m() - 0.027).abs() < 0.027 * 0.06)), "the tabulated 27 mm agrees with the area-implied radius over the catalogue");
    let c_over_t = ((heb300.width_m - heb300.tw_m) / 2.0 - r) / heb300.tf_m;
    assert!((c_over_t - 6.184).abs() < 6.184 * 1e-2, "c/t_f={c_over_t}, oracle (144.5 - 27) / 19 = 6.184");
    assert_eq!(part_1_1::section_class(&heb300, 355.0e6), 1, "6.18 <= 9 eps = 7.33; without r the ratio 7.61 gave class 2");
    let welded = SteelSection { designation: "CUSTOM-PLATE".into(), r_m: 0.0, a_m2: 2.0 * 0.3 * 0.019 + 0.262 * 0.011, ..SteelSection::heb300() };
    assert!(welded.root_radius_m() < 1e-6);
    assert_eq!(part_1_1::section_class(&welded, 355.0e6), 2, "a welded plate section keeps c = (b - t_w) / 2: 7.61 in (7.33, 8.14]");
}

#[test]
fn uls_governing_combination_is_chosen_by_unit_free_demand() {
    use semio_s_artifact_norm_en1994_model::CharacteristicAction;
    let actions = [
        CharacteristicAction::permanent_area("G", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "self", 4000.0),
        CharacteristicAction::imposed_area("Q1", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 5000.0),
        CharacteristicAction::imposed_area("Q2", semio_s_artifact_norm_en1994_model::ActionStage::Composite, "B", 2000.0),
    ];
    for annex in [AnnexChoice::En, AnnexChoice::De] {
        let e = part_en1990::uls_composite(&actions, 8.0, 8.0, semio_s_artifact_norm_en1994_model::SupportCondition::SimplySupported, 3.0, annex, semio_s_plugin_norm_compliance::ReliabilityClass::RC2);
        assert!((e.m_nm - 360_000.0).abs() < 1e-6 * 360_000.0, "M_Ed = 1.35 * 96 + 1.5 * (120 + 0.7 * 48) kNm = 360 kNm, got {}", e.m_nm);
        assert!((e.v_n - 180_000.0).abs() < 1e-6 * 180_000.0, "V_Ed = 1.35 * 48 + 1.5 * (60 + 0.7 * 24) kN = 180 kN, got {}", e.v_n);
        assert!(e.label_en.contains("leading Q1"));
    }
}

#[test]
fn shear_connection_degree_follows_the_resistance_of_the_weaker_half_span() {
    let beam = sample_beam();
    let (b_eff, f_y) = (part_1_1::effective_width_m(beam.span_m, beam.steel.width_m, beam.spacing_m), 355e6);
    let n_cf = part_1_1::n_cf_n(&beam, b_eff, f_y, AnnexChoice::En);
    let degree = |count: u32| part_1_1::declared_shear_connection_degree(&part_studs::of(&beam, &uniform_studs(&beam, count), AnnexChoice::En), n_cf);
    let (full, partial, single) = (degree(80), degree(40), degree(2));
    assert!(partial <= full, "{partial} {full}");
    assert!(single < partial, "{single} {partial}");
    let one = part_studs::of(&beam, &uniform_studs(&beam, 2), AnnexChoice::En);
    assert!((single - one.rows[0].p_rd_n / n_cf).abs() < 1e-12, "one stud per half span carries one P_Rd");
    let lopsided: Vec<Stud> = Stud::typical_row(&beam.id, beam.span_m / 4.0, 0.1, 20);
    assert!(part_studs::of(&beam, &lopsided, AnnexChoice::En).capacity_n() == 0.0, "no stud in the right half: the weaker half carries nothing");
}

#[test]
fn every_stud_gets_its_own_resistance_with_the_sheeting_reduction_of_its_station() {
    let mut beam = sample_beam();
    beam.sheeting.ribs_parallel_to_beam = false;
    let mut studs = uniform_studs(&beam, 4);
    studs.push(Stud { id: "twin".into(), x_m: studs[1].x_m, ..studs[1].clone() });
    studs[3].diameter_m = 0.022;
    let set = part_studs::of(&beam, &studs, AnnexChoice::En);
    let by = |id: &str| set.rows.iter().find(|r| r.id == id).expect("stud");
    let (single, paired, thick) = (by(&studs[0].id), by(&studs[1].id), by(&studs[3].id));
    assert_eq!((single.n_r, paired.n_r, by("twin").n_r), (1, 2, 2));
    assert!(paired.reduction < single.reduction + 1e-12, "k_t falls with the studs per rib: {} {}", paired.reduction, single.reduction);
    assert!((paired.reduction - by("twin").reduction).abs() < 1e-12);
    assert!(thick.p_rd_n > single.p_rd_n * 0.99 || thick.reduction < single.reduction, "the larger stud carries more unless k_t,max bites");
    let expected = |r: &part_studs::StudResistance| r.p_rd1_n.min(r.p_rd2_n) * r.reduction;
    assert!(set.rows.iter().all(|r| (r.p_rd_n - expected(r)).abs() <= 1e-9 * expected(r).max(1.0)));
}

#[test]
fn the_spacing_check_reads_the_gaps_between_the_stud_stations() {
    let beam = sample_beam();
    let origin = part_studs::of(&beam, &[], AnnexChoice::En).origin_m;
    let studs: Vec<Stud> = [0.5, 1.0, 1.05].iter().enumerate().map(|(k, x)| Stud::typical(&format!("s{k}"), &beam.id, origin + x)).collect();
    let set = part_studs::of(&beam, &studs, AnnexChoice::En);
    let gaps = set.gaps();
    assert_eq!(gaps.len(), 2);
    assert!((gaps[0].gap_m - 0.5).abs() < 1e-12 && (gaps[1].gap_m - 0.05).abs() < 1e-9);
    let (s_min, s_max) = part_1_1::stud_spacing_limits_m(&beam, gaps[1].diameter_m);
    assert!(gaps[1].gap_m < s_min && gaps[0].gap_m < s_max, "5d = {s_min}, s_max = {s_max}");
}

#[test]
fn a_point_force_off_mid_span_follows_f_a_b_over_l_on_a_simple_span() {
    let force = semio_s_artifact_norm_en1994_model::CharacteristicAction { x_m: 2.0, ..semio_s_artifact_norm_en1994_model::CharacteristicAction::point_force("P", semio_s_artifact_norm_en1994_model::ActionStage::Composite, semio_s_artifact_norm_en1994_model::ActionKind::Imposed, "B", 100e3) };
    let (m, hogging, v, _) = part_en1990::action_internals(&force, 8.0, 0.0, semio_s_artifact_norm_en1994_model::SupportCondition::SimplySupported, 3.0);
    assert!((m - 100e3 * 2.0 * 6.0 / 8.0).abs() < 1e-6, "{m}");
    assert!((v - 100e3 * 6.0 / 8.0).abs() < 1e-6, "{v}");
    assert_eq!(hogging, 0.0);
    let centred = semio_s_artifact_norm_en1994_model::CharacteristicAction { x_m: 0.0, ..force };
    let (mid, _, shear, _) = part_en1990::action_internals(&centred, 8.0, 0.0, semio_s_artifact_norm_en1994_model::SupportCondition::SimplySupported, 3.0);
    assert!((mid - 100e3 * 8.0 / 4.0).abs() < 1e-6 && (shear - 50e3).abs() < 1e-6);
}

#[test]
fn the_column_rebar_product_leaf_selects_the_fire_strength_reduction() {
    use semio_s_artifact_norm_en1994_model::CompositeColumn;
    let column = CompositeColumn::default_example();
    let hot = catalogue::column_fire::RebarProduct::from(column.reinforcement_product);
    let cold = catalogue::column_fire::RebarProduct::from(semio_s_artifact_norm_en1994_model::ReinforcementProduct::ColdWorked);
    assert_eq!(hot, catalogue::column_fire::RebarProduct::HotRolled);
    assert!(cold.reduction().at(500.0) < hot.reduction().at(500.0), "cold-worked bars lose more strength above 300 °C");
}

#[test]
fn the_beam_rebar_product_leaf_reaches_the_beam_fire_model_and_its_reinforcement_reduction() {
    let mut beam = sample_beam();
    assert_eq!(beam.reinforcement_product, semio_s_artifact_norm_en1994_model::ReinforcementProduct::HotRolled);
    let hot = part_1_2::beam_fire(&beam, 355e6, 0.02);
    beam.reinforcement_product = semio_s_artifact_norm_en1994_model::ReinforcementProduct::ColdWorked;
    let cold = part_1_2::beam_fire(&beam, 355e6, 0.02);
    assert_eq!((hot.rebar, cold.rebar), (catalogue::column_fire::RebarProduct::HotRolled, catalogue::column_fire::RebarProduct::ColdWorked));
    let (a, b) = (hot.evolve(&[90.0]).points[0], cold.evolve(&[90.0]).points[0]);
    assert_eq!(a.rebar_ks, hot.rebar.reduction().at(a.rebar_c));
    assert_eq!(b.rebar_ks, cold.rebar.reduction().at(b.rebar_c));
}

#[test]
fn reliability_class_rc3_raises_the_governing_design_effect_by_k_fi() {
    for annex in [AnnexChoice::De, AnnexChoice::En] {
        let beam = sample_beam();
        let governing = |class| {
            let e = part_en1990::uls_composite(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, annex, class);
            e.m_nm.abs() + e.v_n.abs()
        };
        let k_fi = ReliabilityClass::RC3.k_fi(annex);
        assert!((k_fi - 1.1).abs() < 1e-9, "registry K_FI of RC3 (unverified) is {k_fi}");
        let ratio = governing(ReliabilityClass::RC3) / governing(ReliabilityClass::RC2);
        assert!((ratio - k_fi / ReliabilityClass::RC2.k_fi(annex)).abs() < 1e-9, "governing design effect ratio {ratio}");
    }
}
