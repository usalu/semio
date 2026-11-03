//! 🔬️ Worked resistance examples of the EN 1999 compliance tests: the spec examples E2-02 to E2-12, the alloy rows, the fire and the thin-wall values. See [`super`].

use super::*;
use semio_s_artifact_norm_en1999_checks::{joints, part_1_4_1_5, web_stiffener};
use semio_s_artifact_norm_en1999_model::{AluminiumSection, PlateElement, TransverseStiffener};

fn el(id: &str, width: f64, t: f64, outstand: bool) -> PlateElement {
    PlateElement { id: id.into(), width, thickness: t, outstand, welded: false, weld_position: 0.0, weld_process: semio_s_artifact_norm_en1999_model::WeldProcess::Mig, weld_t_junction: false, weld_direction: semio_s_artifact_norm_en1999_model::WeldDirection::Longitudinal, edge_stiffener_depth: 0.0, edge_stiffener_thickness: 0.0 }
}

fn rhs(h: f64, t: f64) -> AluminiumSection {
    AluminiumSection {
        id: "rhs".into(), kind: semio_s_artifact_norm_en1999_model::SectionKind::Rhs, height: h, width: h, flange_thickness: t, web_thickness: t, outer_diameter: 0.0,
        elements: vec![el("flange-top", h - 2.0 * t, t, false), el("flange-bot", h - 2.0 * t, t, false), el("web-l", h - 2.0 * t, t, false), el("web-r", h - 2.0 * t, t, false)],
    }
}

fn i_section() -> AluminiumSection {
    AluminiumSection {
        id: "i".into(), kind: semio_s_artifact_norm_en1999_model::SectionKind::ExtrudedI, height: 0.120, width: 0.080, flange_thickness: 0.010, web_thickness: 0.006, outer_diameter: 0.0, elements: vec![],
    }
}

fn close(a: f64, b: f64, rel: f64) -> bool { (a - b).abs() <= rel * b.abs().max(1e-12) }

#[test]
fn e2_01_alloy_row_6082_t6_bands() {
    let thick = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.010).expect("band");
    assert!(close(thick.f_o_pa, 260.0e6, 1e-12) && close(thick.f_u_pa, 310.0e6, 1e-12));
    assert!(close(thick.rho_o_haz * thick.f_o_pa, 124.8e6, 1e-9) && close(thick.rho_u_haz * thick.f_u_pa, 186.0e6, 1e-9));
    let thin = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.005).expect("band");
    assert!(close(thin.f_o_pa, 250.0e6, 1e-12), "t exactly on the limit belongs to the lower band");
    let cat = part_1_1::resolve_alloy("aw6082-t6").expect("catalogue alloy");
    assert_eq!(cat, thick);
    assert!(cat.buckling_class_a);
}

#[test]
fn e2_02_classification_and_local_buckling() {
    let rho = part_1_1::rho_c(28.0, true, false, false);
    assert!(close(rho, 32.0 / 28.0 - 220.0 / 784.0, 1e-12) && close(rho, 0.86224, 1e-4));
    let alloy = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.005).unwrap();
    let sec = rhs(0.150, 0.005);
    assert_eq!(part_1_1::section_class(&sec, alloy.f_o_pa, true, 1.0), part_1_1::SectionClass::Class4);
    let ctx = part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: true, eta_web: 1.0, local: true, haz_rho: None, haz_extents: &[] };
    let p = part_1_1::props(&sec, &ctx);
    assert!(close(p.a, 2514.3e-6, 1e-3), "A_eff={}", p.a * 1e6);
    assert!(close(p.a * alloy.f_o_pa / 1.1, 571.4e3, 1e-3));
    let rhs100 = rhs(0.100, 0.005);
    assert_eq!(part_1_1::section_class(&rhs100, alloy.f_o_pa, true, 1.0), part_1_1::SectionClass::Class3);
    let i = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.010).unwrap();
    let eps = part_1_1::epsilon(i.f_o_pa);
    assert!(close(eps, 0.98058, 1e-4));
    let flange = el("f", 0.037, 0.010, true);
    assert_eq!(part_1_1::classify_element(&flange, i.f_o_pa, true, 1.0), part_1_1::SectionClass::Class2);
    let web = el("w", 0.100, 0.006, false);
    let eta = part_1_1::stress_gradient_eta(-1.0);
    assert!(close(eta, 0.4, 1e-12));
    assert_eq!(part_1_1::classify_element(&web, i.f_o_pa, true, eta), part_1_1::SectionClass::Class1);
}

#[test]
fn e2_03_tension_with_haz() {
    let a = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.010).unwrap();
    assert!(close(1000e-6 * a.rho_o_haz * a.f_o_pa / 1.1, 113.45e3, 1e-4));
    assert!(close(1000e-6 * a.rho_u_haz * a.f_u_pa / 1.25, 148.80e3, 1e-4));
    assert!(close(1000e-6 * a.f_o_pa / 1.1, 236.36e3, 1e-4));
    assert!(close(0.9 * 1000e-6 * a.f_u_pa / 1.25, 223.2e3, 1e-4));
}

#[test]
fn e2_04_and_e2_05_bending_and_shear_of_the_i_section() {
    let sec = i_section();
    let (_, iy, _, wel, _, _, _) = part_1_1::section_geometry(&sec);
    let (wpl, _) = part_1_1::plastic_moduli(&sec);
    assert!(close(iy, 5.3533e-6, 1e-4) && close(wel, 89222e-9, 1e-4) && close(wpl, 103000e-9, 1e-9));
    let a = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.010).unwrap();
    let alpha = part_1_1::shape_factor(part_1_1::SectionClass::Class2, wpl, wel, 3.7, [3.0, 4.5, 6.0]);
    assert!(close(alpha, 1.1544, 1e-3));
    assert!(close(alpha * wel * a.f_o_pa / 1.1, 24.345e3, 1e-3));
    let p = part_1_1::props(&sec, &part_1_1::EffCtx::gross(a.f_o_pa, true));
    assert!(close(p.a_v, 600e-6, 1e-9));
    assert!(close(p.a_v * a.f_o_pa / (3.0f64.sqrt() * 1.1), 81.88e3, 1e-3));
}

#[test]
fn e2_06_flexural_buckling_of_the_rhs() {
    let sec = rhs(0.100, 0.005);
    let (a, iy, _, _, _, _, _) = part_1_1::section_geometry(&sec);
    assert!(close(a, 1900e-6, 1e-9));
    let ncr = part_1_1::n_cr(na_de::E_PA, iy, 2.5);
    assert!(close(ncr, 316.79e3, 1e-3), "N_cr={ncr}");
    let lam = part_1_1::lambda_bar(2.5, iy, a, 250.0e6);
    assert!(close(lam, 1.2245, 1e-3));
    let chi = part_1_1::chi_from_lambda(lam, 0.20, 0.10);
    assert!(close(chi, 0.5105, 1e-3));
    assert!(close(chi * a * 250.0e6 / 1.1, 220.44e3, 1e-3));
    assert!(close(part_1_1::chi_from_lambda(1.0, 0.20, 0.10), 0.656, 0.02));
}

#[test]
fn e2_07_lateral_torsional_buckling_of_the_i_section() {
    let sec = i_section();
    let (_, _, iz, _, _, _, _) = part_1_1::section_geometry(&sec);
    let (it, iw) = part_1_1::thin_walled_idealisation(&sec);
    assert!(close(iz, 855_133e-12, 1e-6) && close(it, 60_533e-12, 1e-4) && close(iw, 2.5813e-3 * 1e-6, 1e-3), "It={it} Iw={iw}");
    let mcr = part_1_1::m_cr_ltb(1.0, na_de::E_PA, iz, iw, it, 2.0);
    assert!(close(mcr, 17.53e3, 2e-3), "M_cr={mcr}");
    let (wpl, _) = part_1_1::plastic_moduli(&sec);
    let lam = (wpl * 260.0e6 / mcr).sqrt();
    assert!(close(lam, 1.2360, 1e-3));
    let (alpha, l0) = part_1_1::ltb_curve(part_1_1::SectionClass::Class2);
    let chi = part_1_1::chi_from_lambda(lam, alpha, l0);
    assert!(close(chi, 0.5937, 1e-3));
    assert!(close(chi * wpl * 260.0e6 / 1.1, 14.45e3, 2e-3));
}

#[test]
fn the_exact_torsion_of_an_extrusion_is_below_the_thin_walled_idealisation() {
    let sec = i_section();
    let (it_fe, iw_fe) = part_1_1::torsion_constants(&sec);
    let (it_tw, iw_tw) = part_1_1::thin_walled_idealisation(&sec);
    assert!(it_fe < it_tw && it_tw / it_fe < 1.05 && close(iw_fe, iw_tw, 0.02), "FE It={it_fe} Iw={iw_fe}, thin It={it_tw} Iw={iw_tw}");
    assert!(close(it_fe, 5.8215761708542667e-08, 5e-3) && close(iw_fe, 2.575608989487931e-09, 5e-3), "It={it_fe} Iw={iw_fe}");
    let big = open_section("extrudedI", 0.2, 0.1, 0.01, 0.006);
    let (it, iw) = part_1_1::torsion_constants(&big);
    assert!(close(it, 7.754696340343729e-08, 5e-3) && close(iw, 1.50224726622881e-08, 5e-3), "It={it} Iw={iw}");
    let mcr = part_1_1::m_cr_of(&sec, 1.0, 2.0);
    assert!(close(mcr, 17258.6, 3e-3), "exact M_cr={mcr}");
}

#[test]
fn the_channel_critical_moment_follows_the_general_formula_with_the_monosymmetry_term() {
    let channel = open_section("channel", 0.1, 0.05, 0.007, 0.005);
    let (ey, _) = part_1_1::shear_centre_offset(&channel);
    assert!(close(ey, -0.03286766245220711, 1.5e-2), "e_y={ey}");
    assert!(part_1_1::section_zj(&channel).abs() < 1e-6, "a channel bent about its axis of symmetry has z_j = 0");
    assert!(close(part_1_1::m_cr_of(&channel, 1.0, 3.0), 3015.33, 3e-3));
    let outer = [[0.02, 0.0], [0.08, 0.0], [0.08, 0.008], [0.053, 0.008], [0.053, 0.188], [0.1, 0.188], [0.1, 0.2], [0.0, 0.2], [0.0, 0.188], [0.047, 0.188], [0.047, 0.008], [0.02, 0.008]];
    let (mcr, zj) = part_1_1::m_cr_of_outline(&outer, &[], 1.0, 4.0).expect("mono-symmetric I outline");
    assert!(close(zj, 0.06400390761890133, 2e-3), "z_j={zj}");
    assert!(close(mcr, 14283.5, 6e-3), "M_cr={mcr}");
    let flipped: Vec<[f64; 2]> = outer.iter().map(|p| [p[0], 0.2 - p[1]]).collect();
    let (down, zj_down) = part_1_1::m_cr_of_outline(&flipped, &[], 1.0, 4.0).expect("flipped outline");
    assert!(zj_down < 0.0 && down < 0.8 * mcr, "the smaller compression flange lowers M_cr: {down}");
}

#[test]
fn e2_08_bolted_joint() {
    assert!(close(joints::shear_resistance("8.8", 0.016, 1.25), 60.29e3, 1e-3));
    assert!(close(joints::bearing_resistance(0.040, 0.060, 0.030, 0.016, 0.008, 310.0e6, 800.0e6, 1.25), 58.79e3, 1e-3));
    assert!(close(joints::tension_resistance("8.8", 0.016, 1.25), 90.4e3, 1e-3));
    assert!(close(joints::long_joint_factor(0.5, 0.016), 1.0 - (0.5 - 15.0 * 0.016) / (200.0 * 0.016), 1e-12));
}

#[test]
fn e2_09_fatigue_corner_stresses_and_damage() {
    let dc = 71.0e6;
    assert!(close(part_1_3::delta_sigma_d(dc, 3.4), 54.23e6, 1e-3));
    assert!(close(part_1_3::delta_sigma_l(dc, 3.4, 5.4), 31.14e6, 2e-3));
    assert!(close(part_1_3::cycles_to_failure(60.0e6, dc, 3.4, 5.4), 3.545e6, 1e-3));
    let d1 = part_1_3::damage(60.0e6, 1.0e6, dc, 3.4, 5.4, 1.0, 1.0);
    let d2 = part_1_3::damage(40.0e6, 2.0e6, dc, 3.4, 5.4, 1.0, 1.0);
    assert!(close(d1, 0.2821, 2e-3) && close(d2, 0.0773, 3e-3) && close(d1 + d2, 0.3594, 2e-3));
    assert!(part_1_3::damage(20.0e6, 1.0e9, dc, 3.4, 5.4, 1.0, 1.0) == 0.0, "below the cut-off no damage accumulates");
    let rd = part_1_3::fatigue_strength_pa(71.0e6, 4.3, 6.3, 500_000.0);
    assert!(close(rd, 98.01e6, 5e-3), "rd_mpa={}", rd / 1e6);
    assert!(part_1_3::detail_category_delta_sigma_c("73").is_none());
    assert!(part_1_3::detail_category_delta_sigma_c("40-weld").is_some());
}

#[test]
fn e2_10_fire_first_step_and_critical_temperature() {
    let first = part_1_2::unprotected_temperature_c(20.0, 200.0, 5.0);
    assert!(close(first - 20.0, 0.856, 5e-3), "step={}", first - 20.0);
    assert!(close(part_1_2::gas_temperature_c(300.0), 20.0 + 345.0 * 41.0f64.log10(), 1e-12));
    assert!(part_1_2::k_theta(20.0) == 1.0 && part_1_2::k_theta(600.0) == 0.0);
    let k = part_1_2::k_theta(175.0);
    assert!(k < 0.75 && k > 0.5);
    let cr = part_1_2::critical_temperature_c(0.5);
    assert!(close(part_1_2::k_theta(cr), 0.5, 1e-6));
    assert!(part_1_2::k_e_theta(300.0) > part_1_2::k_theta(300.0));
}

#[test]
fn e2_11_shell_critical_stress_and_reduction() {
    let s = part_1_4_1_5::sigma_x_rcr(0.5, 0.003, 0.5);
    assert!(close(s, 254.1e6, 1e-3), "σ={}", s / 1e6);
    assert!(close((200.0e6 / s).sqrt(), 0.8872, 1e-3));
    assert!(part_1_4_1_5::shell_chi(0.15, 0.5, 0.6, 1.0, 0.2) == 1.0);
    let chi = part_1_4_1_5::shell_chi(0.9, 0.5, 0.6, 1.0, 0.2);
    assert!(chi < 1.0 && chi > 0.2);
    assert!(part_1_4_1_5::alpha_x(0.5, 0.003, part_1_4_1_5::shell_quality_q("B")) < 0.62);
}

#[test]
fn e2_12_haz_extent_and_effective_area() {
    let a = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.008).unwrap();
    assert!(close(part_1_1::haz_width(0.008), 0.030, 1e-12));
    let a_haz = 2.0 * 0.030 * 0.008;
    let a_eff = 120e-3 * 8e-3 - a_haz * (1.0 - a.rho_o_haz);
    assert!(close(a_eff, 710.4e-6, 1e-4));
    assert!(close(a_eff * a.f_o_pa / 1.1, 167.91e3, 1e-3));
    let mut plate = el("p", 0.120, 0.008, true);
    plate.welded = true;
    plate.weld_position = 0.060;
    let sec = AluminiumSection { id: "p".into(), kind: semio_s_artifact_norm_en1999_model::SectionKind::Angle, height: 0.120, width: 0.120, flange_thickness: 0.008, web_thickness: 0.008, outer_diameter: 0.0, elements: vec![plate.clone(), el("q", 0.112, 0.008, true)] };
    let base = part_1_1::EffCtx { f_o_pa: a.f_o_pa, class_a: true, eta_web: 1.0, local: false, haz_rho: Some(a.rho_o_haz), haz_extents: &[] };
    let gross = part_1_1::props(&sec, &part_1_1::EffCtx::gross(a.f_o_pa, true));
    let reduced = part_1_1::props(&sec, &base);
    assert!(reduced.a < gross.a, "HAZ strip must reduce the area");
}

#[test]
fn alloy_5083_class_b() {
    let a = part_1_1::resolve_alloy("aw5083-o").expect("catalogue alloy");
    assert!(!a.buckling_class_a);
    assert!((a.rho_o_haz - 1.0).abs() < 1e-12);
    assert_eq!(part_1_1::buckling_curve(false), (0.32, 0.0));
    assert_eq!(part_1_1::rho_constants(false, false, true), (25.0, 150.0));
}

#[test]
fn rho_c_is_continuous_at_every_class_3_limit() {
    for class_a in [true, false] {
        for outstand in [false, true] {
            for welded in [false, true] {
                let lim = part_1_1::class_limits(class_a, outstand, welded);
                let at = part_1_1::rho_c(lim[2] * 1.0000001, class_a, outstand, welded);
                assert!((at - 1.0).abs() < 1e-5, "ρ_c jumps at β3 for {class_a} {outstand} {welded}: {at}");
            }
        }
    }
}

#[test]
fn interaction_exponents_are_data_driven_and_never_below_one() {
    let (eta_c, gamma_c, xi_c) = part_1_1::member_interaction_exponents(part_1_1::SectionClass::Class2, 0.6);
    assert!(eta_c >= 1.0 && gamma_c >= 1.0 && xi_c >= 1.0);
    let (psi, eta, xi) = part_1_1::section_interaction_exponents("extrudedI");
    assert!(psi >= 1.0 && eta >= 1.0 && xi >= 1.0);
    assert!(part_1_1::kappa(0.5, 0.0) == 0.5 && part_1_1::kappa(0.5, 2.0) == 1.0);
    assert!((part_1_1::omega_x(0.9, 0.48, 0.0, 1.0, 1.0) - 1.0).abs() < 1e-12, "a weld at the support has no effect");
    assert!((part_1_1::omega_x(1.0, 0.48, 0.5, 1.0, 1.0) - 0.48).abs() < 1e-12, "a weld at mid-span of a stocky member keeps rho_haz");
    assert!((part_1_1::omega_x(0.3, 0.48, 0.5, 1.0, 1.0) - 1.0).abs() < 1e-12, "a slender member is not weakened");
}

#[test]
fn unknown_alloy_emits_fail_with_oneof_catalogue() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.materials[0].designation = "aw9999-t6".into();
    let report = evaluate_structure(&doc);
    let fail = report.failing().find(|c| c.id.contains("alloy")).expect("unknown alloy must Fail");
    assert!(
        fail.remedies.iter().any(|r| r.target.path.contains("designation") && r.options.iter().any(|o| o.starts_with("aw"))),
        "expected OneOf catalogue remedy, got {:?}",
        fail.remedies
    );
}

#[test]
fn compliant_member_resistance_is_the_catalogue_value() {
    let snap = En1999Snapshot::compliant_roof_purlin();
    let report = evaluate_structure(&snap);
    let n = report.checks.iter().find(|c| c.id == "en1999.6.2.3.n.purlin-1").expect("N_Rd check");
    let section = snap.sections.iter().find(|s| s.id == snap.members[0].section_id).unwrap();
    let alloy = part_1_1::resolve_alloy_for_thickness("aw6082-t6", part_1_1::max_thickness(section)).unwrap();
    let ctx = part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: true, eta_web: 1.0, local: false, haz_rho: Some(alloy.rho_o_haz), haz_extents: &[] };
    let expected = part_1_1::props(section, &ctx).a * alloy.f_o_pa / na_de::AnnexParams::de().gamma_m1;
    let ult_ctx = part_1_1::EffCtx { haz_rho: Some(alloy.rho_u_haz), ..ctx };
    let ult = part_1_1::props(section, &ult_ctx).a * alloy.f_u_pa / na_de::AnnexParams::de().gamma_m2;
    assert!(close(n.limit.value, expected.min(ult), 1e-9), "N_t,Rd {} vs {}", n.limit.value, expected.min(ult));
}

#[test]
fn haz_reduces_the_effective_modulus() {
    let doc = En1999Snapshot::noncompliant_multi_fail();
    let sec = &doc.sections[0];
    let alloy = part_1_1::resolve_alloy("aw6060-t6").expect("alloy");
    let haz = part_1_1::props(sec, &part_1_1::EffCtx { f_o_pa: alloy.f_o_pa, class_a: true, eta_web: 1.0, local: true, haz_rho: Some(alloy.rho_o_haz), haz_extents: &[] });
    let (_, _, _, wel_gross, _, _, _) = part_1_1::section_geometry(sec);
    assert!(haz.wy < wel_gross);
}

#[test]
fn per_element_reduction_is_not_a_global_minimum() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let sec = &doc.sections[0];
    let a = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.010).unwrap();
    let only_flange = {
        let mut s = sec.clone();
        for e in &mut s.elements { if !e.outstand { e.welded = false; e.weld_position = 0.0; } }
        s
    };
    let ctx = part_1_1::EffCtx { f_o_pa: a.f_o_pa, class_a: true, eta_web: 1.0, local: true, haz_rho: Some(a.rho_o_haz), haz_extents: &[] };
    let all = part_1_1::props(sec, &ctx);
    let flanges = part_1_1::props(&only_flange, &ctx);
    assert!(flanges.wy > all.wy, "a web without weld must keep more modulus than a welded web");
    let global = part_1_1::props(sec, &part_1_1::EffCtx::gross(a.f_o_pa, true)).wy * a.rho_o_haz;
    assert!(all.wy > global, "the strip model keeps the unaffected zones, a global ρ_min would not");
}

#[test]
fn fire_utilization_applies_k_theta_to_member_resistance() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let report = evaluate_structure(&doc);
    let check = report.checks.iter().find(|c| c.id == "en1999.1-2.fire.fire-1").expect("fire check");
    assert!(check.explanation.en.contains("N_fi,Rd") && check.explanation.en.contains("k_θ"));
    assert!(check.explanation.en.contains("γ_M,fi"));
    for id in ["buckling", "ltb", "theta-cr"] {
        assert!(report.checks.iter().any(|c| c.id == format!("en1999.1-2.fire.{id}.fire-1")), "missing fire.{id}");
    }
}

#[test]
fn fatigue_verdict_follows_the_miner_sum() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.fatigue_details[0].delta_sigma_ed = 60.0e6;
    doc.fatigue_details[0].n_cycles = 6.0e6;
    let report = evaluate_structure(&doc);
    let d = report.checks.iter().find(|c| c.id == "en1999.1-3.fat.fat-1").expect("damage check");
    assert!(d.utilization > 1.0 && d.status == CheckStatus::Fail, "D={}", d.utilization);
    assert!(d.remedies.iter().any(|r| r.applicable));
    let mut unknown = En1999Snapshot::compliant_roof_purlin();
    unknown.fatigue_details[0].detail_category = "73".into();
    let report = evaluate_structure(&unknown);
    assert!(report.failing().any(|c| c.id == "en1999.1-3.fat.fat-1"), "an unknown category must not fall back silently");
}

#[test]
fn pin_joint_resistances_follow_the_closed_forms() {
    assert!(close(joints::pin_shear_resistance("8.8", 0.020, 1.25), 120.637e3, 1e-3));
    assert!(close(joints::pin_bending_resistance("8.8", 0.020, 1.25), 603.19, 1e-3));
    assert!(close(joints::pin_bearing_resistance(0.010, 0.020, 260.0e6, 1.1), 70.909e3, 1e-3));
    assert!(close(joints::pin_bending_moment(100.0e3, 0.010), 125.0, 1e-12));
}

#[test]
fn a_pinned_connection_emits_the_pin_checks() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.connections[0].kind = semio_s_artifact_norm_en1999_model::ConnectionKind::Pinned;
    let report = evaluate_structure(&doc);
    for suffix in ["shear", "bearing", "bend", "int"] {
        let id = format!("en1999.8.5.pin.{suffix}.support-weld");
        assert!(report.checks.iter().any(|c| c.id == id), "{id}");
    }
    assert!(!report.checks.iter().any(|c| c.id == "en1999.8.kind.support-weld"));
}

#[test]
fn cold_formed_and_shells_na_when_absent() {
    let report = evaluate_structure(&En1999Snapshot::empty());
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-4.empty" && c.status == CheckStatus::NotApplicable));
    assert!(report.checks.iter().any(|c| c.id == "en1999.1-5.empty" && c.status == CheckStatus::NotApplicable));
}

#[test]
fn cold_formed_and_shells_evaluate_when_present() {
    let pass = evaluate_structure(&En1999Snapshot::compliant_roof_purlin());
    assert!(pass.checks.iter().any(|c| c.id.contains("1-4.") && c.status != CheckStatus::NotApplicable));
    assert!(pass.checks.iter().any(|c| c.id.contains("1-5.") && c.status != CheckStatus::NotApplicable));
    assert!(!pass.checks.iter().any(|c| c.id == "en1999.1-4.empty"));
    assert!(!pass.checks.iter().any(|c| c.id == "en1999.1-5.empty"));
    let fail = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    let cold_fail = fail.checks.iter().filter(|c| c.id.contains("1-4.") && c.status == CheckStatus::Fail).collect::<Vec<_>>();
    let shell_fail = fail.checks.iter().filter(|c| c.id.contains("1-5.") && c.status == CheckStatus::Fail).collect::<Vec<_>>();
    assert!(!cold_fail.is_empty(), "expected EN 1999-1-4 Fail");
    assert!(!shell_fail.is_empty(), "expected EN 1999-1-5 Fail");
    assert!(cold_fail.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "1-4 remedies");
    assert!(shell_fail.iter().all(|c| c.remedies.iter().any(|r| r.applicable)), "1-5 remedies");
}

#[test]
fn every_catalogue_family_is_reported() {
    let report = evaluate_structure(&En1999Snapshot::noncompliant_multi_fail());
    for family in [
        "en1999.6.1.4.class.", "en1999.6.2.3.n.", "en1999.6.2.5.m.", "en1999.6.2.5.mz.", "en1999.6.2.6.v.", "en1999.6.2.6.vy.", "en1999.6.2.8.mv.", "en1999.6.2.9.nm.",
        "en1999.6.3.1.fb.", "en1999.6.3.1.tb.", "en1999.6.3.2.ltb.", "en1999.6.3.3.int.", "en1999.6.7.4.vb.", "en1999.7.2.defl.", "en1999.7.2.stress.", "en1999.7.2.sls-freq.",
        "en1999.8.5.bolt.", "en1999.8.5.tension.", "en1999.8.5.vt.", "en1999.8.5.punch.", "en1999.8.5.pitch.", "en1999.8.5.edge.", "en1999.8.5.gauge.", "en1999.8.5.emax.", "en1999.8.5.block.",
        "en1999.8.6.weld.", "en1999.8.6.length.", "en1999.8.6.haz.", "en1999.8.sls-freq.",
        "en1999.1-2.fire.", "en1999.1-2.fire.buckling.", "en1999.1-2.fire.ltb.", "en1999.1-2.fire.theta-cr.", "en1999.1-3.fat.", "en1999.1-3.fat.range.",
        "en1999.1-4.local.", "en1999.1-4.axial.", "en1999.1-4.bend.", "en1999.1-4.shear.", "en1999.1-4.nm.", "en1999.1-5.buckle.", "en1999.1-5.ring.", "en1999.1-5.shear.", "en1999.1-5.int.",
    ] {
        assert!(report.checks.iter().any(|c| c.id.starts_with(family)), "missing check family {family}");
    }
    for removed in ["en1999.6.3.1.lcr.", "en1999.6.3.1.lambda.", "en1999.6.2.8.nm."] {
        assert!(!report.checks.iter().any(|c| c.id.starts_with(removed)), "{removed} must not exist any more");
    }
}

#[test]
fn annex_parameters_come_from_the_shared_registry_and_switch_exactly_where_expected() {
    use semio_s_plugin_norm_compliance::{ndp_is_verified, ndp_number};
    let (en, de) = (na_de::AnnexParams::en(), na_de::AnnexParams::de());
    for id in ["ndp.en1999.gamma-m1", "ndp.en1999.gamma-m2", "ndp.en1999.gamma-mw", "ndp.en1999.gamma-m-fi", "ndp.en1999.gamma-g", "ndp.en1999.gamma-q", "ndp.en1999.xi", "ndp.en1999.gamma-mf", "ndp.en1999.gamma-ff", "ndp.en1999.deflection-ratio", "ndp.en1999.stress-ratio"] {
        assert!(!ndp_is_verified(id), "{id} stays unverified until double-keyed");
    }
    assert_eq!(en.gamma_mf, ndp_number("ndp.en1999.gamma-mf", semio_s_plugin_norm_compliance::AnnexChoice::En));
    assert_eq!(de.gamma_mf, ndp_number("ndp.en1999.gamma-mf", semio_s_plugin_norm_compliance::AnnexChoice::De));
    assert!(de.gamma_mf > en.gamma_mf);
    let (e, d) = (en, de);
    let differing = [e.gamma_m1 != d.gamma_m1, e.gamma_m2 != d.gamma_m2, e.gamma_mw != d.gamma_mw, e.gamma_m_fi != d.gamma_m_fi, e.gamma_g != d.gamma_g, e.gamma_q != d.gamma_q, e.xi != d.xi, e.gamma_mf != d.gamma_mf, e.gamma_ff != d.gamma_ff, e.deflection_ratio != d.deflection_ratio, e.stress_ratio != d.stress_ratio];
    assert_eq!(differing, [false, false, false, false, false, false, false, true, false, false, false], "only gamma_Mf differs between EN and DIN EN 1999-1-1/NA");
}

fn limit_of(doc: &En1999Snapshot, id: &str) -> f64 {
    evaluate_structure(doc).checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}")).limit.value
}

fn util_of(doc: &En1999Snapshot, id: &str) -> f64 {
    evaluate_structure(doc).checks.iter().find(|c| c.id == id).unwrap_or_else(|| panic!("missing {id}")).utilization
}

#[test]
fn torsion_and_patch_loads_come_from_the_document() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    assert_eq!(util_of(&doc, "en1999.6.2.7.t.purlin-1"), 0.0);
    assert_eq!(util_of(&doc, "en1999.6.7.5.patch.purlin-1"), 0.0);
    doc.members[0].actions[0].torsion_k = 400.0;
    doc.members[0].actions[0].local_load_k = 8_000.0;
    assert!(util_of(&doc, "en1999.6.2.7.t.purlin-1") > 0.0);
    assert!(util_of(&doc, "en1999.6.2.7.vt.purlin-1") >= util_of(&En1999Snapshot::compliant_roof_purlin(), "en1999.6.2.7.vt.purlin-1"));
    let short = limit_of(&doc, "en1999.6.7.5.patch.purlin-1");
    doc.members[0].bearing_length = 0.100;
    assert!(limit_of(&doc, "en1999.6.7.5.patch.purlin-1") > short, "a longer bearing length raises the patch resistance");
}

#[test]
fn transverse_weld_and_tig_reduce_the_resistances() {
    let base = En1999Snapshot::compliant_roof_purlin();
    let mut weld = base.clone();
    weld.members[0].transverse_weld = true;
    weld.members[0].transverse_weld_offset = 0.75;
    assert!(limit_of(&weld, "en1999.6.3.1.fb.z.purlin-1") <= limit_of(&base, "en1999.6.3.1.fb.z.purlin-1"));
    weld.members[0].transverse_weld_offset = 0.0;
    assert!((limit_of(&weld, "en1999.6.3.1.fb.z.purlin-1") - limit_of(&base, "en1999.6.3.1.fb.z.purlin-1")).abs() < 1e-6, "a weld at the end has no effect");
    let mut tig = base.clone();
    for e in &mut tig.sections[0].elements { e.weld_process = semio_s_artifact_norm_en1999_model::WeldProcess::Tig; }
    assert!(limit_of(&tig, "en1999.6.2.5.m.purlin-1") < limit_of(&base, "en1999.6.2.5.m.purlin-1"), "TIG widens the HAZ");
    let mut transverse = base.clone();
    for e in &mut transverse.sections[0].elements { e.weld_direction = semio_s_artifact_norm_en1999_model::WeldDirection::Transverse; }
    assert!(limit_of(&transverse, "en1999.6.2.5.m.purlin-1") < limit_of(&base, "en1999.6.2.5.m.purlin-1"), "a transverse weld puts the whole element in the HAZ");
}

#[test]
fn shell_weld_and_fabrication_class_change_the_axial_resistance() {
    let base = En1999Snapshot::compliant_roof_purlin();
    let id = "en1999.1-5.buckle.shell-pass";
    let mut a = base.clone();
    a.shells[0].fabrication_class = semio_s_artifact_norm_en1999_model::FabricationClass::A;
    let mut c = base.clone();
    c.shells[0].fabrication_class = semio_s_artifact_norm_en1999_model::FabricationClass::C;
    assert!(limit_of(&a, id) > limit_of(&base, id) && limit_of(&base, id) > limit_of(&c, id));
    let mut welded = base.clone();
    welded.shells[0].welded = true;
    assert!(limit_of(&welded, id) < limit_of(&base, id));
}

#[test]
fn edge_stiffener_and_unknown_detail_values_are_checked() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.sections[0].elements[0].edge_stiffener_depth = 0.010;
    let report = evaluate_structure(&doc);
    assert!(report.failing().any(|c| c.id.starts_with("en1999.ref.element.stiffener.")), "a depth without thickness is incomplete");
    doc.sections[0].elements[0].edge_stiffener_thickness = 0.010;
    let report = evaluate_structure(&doc);
    assert!(!report.failing().any(|c| c.id.starts_with("en1999.ref.element.stiffener.")));
}

#[test]
fn rhs_flange_in_bending_keeps_its_own_class() {
    let alloy = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.005).unwrap();
    let sec = rhs(0.150, 0.005);
    let eta = part_1_1::stress_gradient_eta(-1.0);
    assert_eq!(part_1_1::section_class(&sec, alloy.f_o_pa, true, eta), part_1_1::SectionClass::Class4, "the web gradient must not relax the flange");
}

#[test]
fn check_units_group_a_report_by_entity() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let report = evaluate_structure(&doc);
    let units = semio_s_artifact_norm_en1999_checks::check_units(&doc, &report);
    assert!(units.iter().any(|u| u.entity_kind == "member" && u.entity_id == "purlin-1" && !u.check_ids.is_empty()));
    assert!(units.iter().any(|u| u.entity_kind == "shell"));
}

#[test]
fn a_narrower_panel_raises_the_shear_buckling_factor() {
    let fo = 260.0e6;
    let none = part_1_1::shear_buckling_rho(120.0, fo, 0.6, 0.0);
    let spaced = part_1_1::shear_buckling_rho(120.0, fo, 0.6, 0.6);
    assert!(spaced > none && none < 1.0);
    assert_eq!(part_1_1::shear_buckling_rho(20.0, fo, 0.6, 0.0), 1.0);
}

#[test]
fn the_web_panels_follow_the_stiffener_rows_and_shorten_the_shear_buckling_panel() {
    let near = |a: f64, b: f64| (a - b).abs() < 1e-12;
    assert!(web_stiffener::web_panels(6.0, &[]).is_empty() && web_stiffener::web_panels(6.0, &[0.0, 6.0, 7.0]).is_empty());
    let panels = web_stiffener::web_panels(6.0, &[4.0, 1.5, 4.0, 9.0]);
    assert!(panels.len() == 3 && near(panels[0], 1.5) && near(panels[1], 2.5) && near(panels[2], 2.0));
    assert!(near(web_stiffener::widest(&panels), 2.5) && web_stiffener::spacings(&[]) == vec![0.0] && web_stiffener::spacings(&panels) == panels);
    let mut base = En1999Snapshot::compliant_roof_purlin();
    let section_id = base.members.iter().find(|m| m.id == "purlin-1").map(|m| m.section_id.clone()).unwrap_or_default();
    for s in base.sections.iter_mut().filter(|s| s.id == section_id) { s.web_thickness = 0.001; }
    for m in base.members.iter_mut().filter(|m| m.id == "purlin-1") { m.length = 0.2; }
    let unstiffened = util_of(&base, "en1999.6.7.4.vb.purlin-1");
    let mut stiffened = base.clone();
    stiffened.transverse_stiffeners.push(TransverseStiffener { id: "st-1".into(), girder_id: "purlin-1".into(), position: 0.1, width: 0.03, thickness: 0.005, sides: semio_s_artifact_norm_en1999_model::StiffenerSides::Two });
    let centred = util_of(&stiffened, "en1999.6.7.4.vb.purlin-1");
    stiffened.transverse_stiffeners[0].position = 0.19;
    let off_centre = util_of(&stiffened, "en1999.6.7.4.vb.purlin-1");
    assert!(centred < off_centre && off_centre < unstiffened);
}

#[test]
fn the_shear_remedy_moves_the_stiffener_with_the_smallest_displacement_or_finds_none() {
    let rows = vec![("a".to_string(), 0.5), ("b".to_string(), 5.0)];
    let moved = web_stiffener::nearest_move(8.0, &rows, |panels| web_stiffener::widest(panels) <= 3.0 + 1e-9);
    assert_eq!(moved, Some(("a".to_string(), 0.5, 2.0)));
    assert_eq!(web_stiffener::nearest_move(8.0, &rows, |panels| web_stiffener::widest(panels) <= 1.0), None, "two stiffeners cannot cut 8 m into panels of 1 m: one must be inserted");
    assert_eq!(web_stiffener::rows_of(&[TransverseStiffener { id: "s".into(), girder_id: "g".into(), position: 1.0, width: 0.05, thickness: 0.006, sides: semio_s_artifact_norm_en1999_model::StiffenerSides::One }], "g"), vec![("s".to_string(), 1.0)]);
}

#[test]
fn the_shear_remedy_inserts_the_minimal_stiffener_nearest_the_widest_panel_or_finds_none() {
    let rows = vec![("a".to_string(), 0.5), ("b".to_string(), 5.0)];
    let at = web_stiffener::minimal_insert(8.0, &rows, |panels| web_stiffener::widest(panels) <= 3.0 + 1e-9).expect("one insert cuts the widest 4.5 m panel");
    assert!((at - 2.75).abs() < 8.0 / 400.0 + 1e-9, "{at}");
    assert_eq!(web_stiffener::minimal_insert(8.0, &[], |panels| web_stiffener::widest(panels) <= 4.0 + 1e-9), Some(4.0));
    assert_eq!(web_stiffener::minimal_insert(8.0, &rows, |panels| web_stiffener::widest(panels) <= 1.0), None);
    assert_eq!(web_stiffener::fresh_id("g", &[("stiffener-g-2".to_string(), 1.0), ("x".to_string(), 2.0)]), "stiffener-g-3");
    assert_eq!(web_stiffener::fresh_id("g", &[("stiffener-g-1".to_string(), 1.0)]), "stiffener-g-2");
}

fn open_section(kind: &str, height: f64, width: f64, flange: f64, web: f64) -> AluminiumSection {
    AluminiumSection { id: kind.into(), kind: match kind { "tube" | "chs" => semio_s_artifact_norm_en1999_model::SectionKind::Chs, "rhs" => semio_s_artifact_norm_en1999_model::SectionKind::Rhs, "box" => semio_s_artifact_norm_en1999_model::SectionKind::Box, "angle" => semio_s_artifact_norm_en1999_model::SectionKind::Angle, "channel" => semio_s_artifact_norm_en1999_model::SectionKind::Channel, "free" => semio_s_artifact_norm_en1999_model::SectionKind::Free, _ => semio_s_artifact_norm_en1999_model::SectionKind::ExtrudedI }, height, width, flange_thickness: flange, web_thickness: web, outer_diameter: 0.0, elements: vec![] }
}

#[test]
fn thin_walled_constants_reproduce_sectionproperties() {
    let i = part_1_1::thin_walled_constants(&open_section("extrudedI", 0.2, 0.1, 0.01, 0.006)).expect("I walls");
    assert!(close(i.j, 7.728382252651452e-08, 5e-2) && close(i.iw, 1.502671224291989e-08, 5e-3), "{} {}", i.j, i.iw);
    assert!((i.shear_centre_y - i.centroid_y).abs() < 1e-6 && (i.shear_centre_z - i.centroid_z).abs() < 1e-6);
    let c = part_1_1::thin_walled_constants(&open_section("channel", 0.1, 0.05, 0.007, 0.005)).expect("channel walls");
    assert!(close(c.j, 1.4343270282331156e-08, 5e-2) && close(c.iw, 4.3340193261414e-10, 5e-2), "{} {}", c.j, c.iw);
    let (ey, ez) = part_1_1::shear_centre_offset(&open_section("channel", 0.1, 0.05, 0.007, 0.005));
    assert!((ey - -0.03286886344335031).abs() < 1.5e-3 && ez.abs() < 1e-5, "{ey} {ez}");
    let a = part_1_1::thin_walled_constants(&open_section("angle", 0.06, 0.06, 0.006, 0.006)).expect("angle walls");
    assert!(close(a.j, 8.031940710442781e-09, 5e-2) && a.iw.abs() < 1e-11, "{} {}", a.j, a.iw);
    let (ay, az) = part_1_1::shear_centre_offset(&open_section("angle", 0.06, 0.06, 0.006, 0.006));
    assert!((ay - -0.014031872310364076).abs() < 3e-3 && (az - -0.01403186915022582).abs() < 3e-3, "{ay} {az}");
}

#[test]
fn closed_and_round_sections_have_no_thin_walls() {
    assert!(part_1_1::thin_walls(&open_section("rhs", 0.1, 0.1, 0.005, 0.005)).is_none());
    assert!(part_1_1::thin_walls(&open_section("chs", 0.1, 0.1, 0.005, 0.005)).is_none());
}

#[test]
fn the_channel_torsional_buckling_load_includes_the_shear_centre_offset() {
    let s = open_section("channel", 0.1, 0.05, 0.007, 0.005);
    let (a, iy, iz, _, _, it, iw) = part_1_1::section_geometry(&s);
    let centred = part_1_1::n_cr_torsional(a, it, iw, iy, iz, (0.0, 0.0), 2.0);
    let offset = part_1_1::n_cr_torsional(a, it, iw, iy, iz, part_1_1::shear_centre_offset(&s), 2.0);
    assert!(offset < centred && it > 0.0 && iw > 0.0, "{offset} {centred}");
}

#[test]
fn stress_gradient_eta_scales_internal_webs_and_never_internal_flanges() {
    let eta = part_1_1::stress_gradient_eta(-1.0);
    let f_o = part_1_1::resolve_alloy_for_thickness("aw6082-t6", 0.010).unwrap().f_o_pa;
    let (web, flange) = (el("web-l", 0.100, 0.006, false), el("flange-top", 0.100, 0.006, false));
    assert!(close(part_1_1::element_beta_over_eps(&web, f_o, eta), eta * part_1_1::element_beta_over_eps(&web, f_o, 1.0), 1e-12));
    assert!(close(part_1_1::element_beta_over_eps(&flange, f_o, eta), part_1_1::element_beta_over_eps(&flange, f_o, 1.0), 1e-12));
}

#[test]
fn e2_08b_the_elastic_group_distribution_balances_the_load_and_the_torque() {
    for (rows, per_row, p1, p2, shear, e) in [(2, 2, 0.036, 0.040, 30.0e3, 0.080), (3, 2, 0.050, 0.045, 12.0e3, 0.120), (1, 3, 0.030, 0.050, 9.0e3, 0.060), (2, 3, 0.040, 0.040, 21.0e3, 0.0)] {
        let forces = joints::group_forces(rows, per_row, p1, p2, shear, e);
        assert_eq!(forces.len(), (rows * per_row) as usize);
        assert!(close(forces.iter().map(|f| f.fx).sum::<f64>(), shear, 1e-12) && forces.iter().map(|f| f.fy).sum::<f64>().abs() < 1e-9 * shear, "force balance");
        let torque: f64 = forces.iter().map(|f| f.x * f.fy - f.y * f.fx).sum();
        assert!((torque + shear * e).abs() < 1e-9 * (shear * e).max(1.0), "moment balance {torque} vs {}", -shear * e);
        let top = forces.iter().map(|f| f.resultant()).fold(0.0, f64::max);
        assert!(close(forces[joints::governing_fastener(&forces)].resultant(), top, 1e-12));
        if e == 0.0 {
            assert!(forces.iter().all(|f| close(f.resultant(), shear / f64::from(rows * per_row), 1e-12)), "a concentric load is shared equally");
        } else {
            assert!(top > shear / f64::from(rows * per_row), "the torque loads the outer fasteners above V/n");
        }
    }
}

#[test]
fn e2_08c_the_bolt_checks_read_e2_from_the_edge_distance_and_p2_from_the_gauge() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let id = doc.connections[0].id.clone();
    let with = |edit: &dyn Fn(&mut semio_s_artifact_norm_en1999_model::AluminiumConnection)| {
        let mut d = doc.clone();
        edit(&mut d.connections[0]);
        evaluate_structure(&d)
    };
    let util = |r: &semio_s_plugin_norm_compliance::CheckReport, prefix: &str| r.checks.iter().find(|c| c.id == format!("en1999.8.5.{prefix}.{id}")).unwrap_or_else(|| panic!("missing {prefix}")).utilization;
    let base = with(&|_| {});
    let wide = with(&|c| c.bolts.gauge *= 2.0);
    for prefix in ["bolt", "block", "tension", "vt", "edge", "emax"] {
        assert!(close(util(&wide, prefix), util(&base, prefix), 1e-12), "{prefix} does not depend on the spacing p2");
    }
    let d0 = joints::hole_diameter(doc.connections[0].bolts.diameter);
    assert!(close(util(&base, "gauge"), joints::min_spacing_d0() * d0 / doc.connections[0].bolts.gauge, 1e-12), "p2 ≥ 2.4·d0 of Table 8.2");
    assert_eq!(joints::min_spacing_d0(), 2.4, "registry row {}", joints::MIN_SPACING_ROW);
    let gauge = base.checks.iter().find(|c| c.id == format!("en1999.8.5.gauge.{id}")).expect("gauge check");
    assert!(gauge.ndps.iter().any(|n| n == joints::MIN_SPACING_ROW), "the spacing check cites the registry row");
    assert!(close(util(&wide, "gauge"), 0.5 * util(&base, "gauge"), 1e-12));
    let close_edge = with(&|c| c.bolts.edge_distance *= 0.75);
    assert!(util(&close_edge, "bolt") >= util(&base, "bolt") && util(&close_edge, "block") > util(&base, "block"), "e2 is the edge distance");
    let single = with(&|c| c.bolts.bolts_per_row = 1);
    assert_eq!(util(&single, "gauge"), 0.0, "a single fastener per row has no spacing p2");
}

#[test]
fn e2_08d_an_eccentric_shear_loads_the_governing_fastener_by_the_elastic_resultant() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let c0 = &doc.connections[0];
    let id = c0.id.clone();
    let util = |d: &En1999Snapshot| evaluate_structure(d).checks.iter().find(|c| c.id == format!("en1999.8.5.bolt.{id}")).expect("bolt check").utilization;
    let mut eccentric = doc.clone();
    eccentric.connections[0].load_eccentricity = 0.060;
    let b = &c0.bolts;
    let forces = joints::group_forces(b.rows, b.bolts_per_row, b.pitch, b.gauge, 1.0, 0.060);
    let mean = forces.iter().map(|f| f.fx).sum::<f64>() / forces.len() as f64;
    let factor = forces[joints::governing_fastener(&forces)].resultant() / mean;
    assert!(factor > 1.0);
    assert!(close(util(&eccentric), util(&doc) * factor, 1e-9), "{} vs {}", util(&eccentric), util(&doc) * factor);
}

#[test]
fn e2_08e_the_instantaneous_centre_method_matches_scipy_and_balances_the_load_and_the_torque() {
    use semio_s_artifact_norm_en1999_model::FastenerForceMethod;
    let reference: [(u32, u32, f64, f64, f64, &[(f64, f64)]); 2] = [
        (2, 2, 0.05, 0.06, 0.08, &[(-0.080957000468, 0.547752476746), (0.580957000468, 0.257950722131), (-0.080957000468, -0.547752476746), (0.580957000468, -0.257950722131)]),
        (3, 2, 0.05, 0.04, -0.045, &[(0.1942258633, -0.165066480635), (0.086642118174, -0.230032062308), (0.248801916044, 0.0), (0.189462121008, 0.0), (0.1942258633, 0.165066480635), (0.086642118174, 0.230032062308)]),
    ];
    for (rows, per_row, p1, p2, e, expected) in reference {
        let forces = joints::instantaneous_centre_forces(rows, per_row, p1, p2, 1.0, e);
        assert_eq!(forces.len(), expected.len());
        for (f, (fx, fy)) in forces.iter().zip(expected) {
            assert!((f.fx - fx).abs() < 1e-9 && (f.fy - fy).abs() < 1e-9, "scipy {fx} {fy} against {} {}", f.fx, f.fy);
        }
        assert!(close(forces.iter().map(|f| f.fx).sum::<f64>(), 1.0, 1e-9) && forces.iter().map(|f| f.fy).sum::<f64>().abs() < 1e-9);
        assert!((forces.iter().map(|f| f.x * f.fy - f.y * f.fx).sum::<f64>() + e).abs() < 1e-9);
        let elastic = joints::group_forces(rows, per_row, p1, p2, 1.0, e);
        assert!(forces[joints::governing_fastener(&forces)].resultant() < elastic[joints::governing_fastener(&elastic)].resultant(), "ductile redistribution unloads the governing fastener");
    }
    assert_eq!(joints::instantaneous_centre_forces(2, 2, 0.05, 0.06, 1.0, 0.0), joints::group_forces(2, 2, 0.05, 0.06, 1.0, 0.0), "a concentric shear is shared equally");
    assert_eq!(joints::fastener_forces(FastenerForceMethod::Elastic, 2, 2, 0.05, 0.06, 2.0, 0.08), joints::group_forces(2, 2, 0.05, 0.06, 2.0, 0.08));
    assert_eq!(joints::fastener_forces(FastenerForceMethod::InstantaneousCentre, 2, 2, 0.05, 0.06, 2.0, 0.08), joints::instantaneous_centre_forces(2, 2, 0.05, 0.06, 2.0, 0.08));
}

#[test]
fn e2_08f_the_selected_force_method_scales_the_bolt_check() {
    use semio_s_artifact_norm_en1999_model::FastenerForceMethod;
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    doc.connections[0].load_eccentricity = 0.060;
    let id = doc.connections[0].id.clone();
    let b = doc.connections[0].bolts.clone();
    let util = |d: &En1999Snapshot| evaluate_structure(d).checks.iter().find(|c| c.id == format!("en1999.8.5.bolt.{id}")).expect("bolt check").utilization;
    let elastic = util(&doc);
    doc.connections[0].force_method = FastenerForceMethod::InstantaneousCentre;
    let ductile = util(&doc);
    let top = |method| { let f = joints::fastener_forces(method, b.rows, b.bolts_per_row, b.pitch, b.gauge, 1.0, 0.060); f[joints::governing_fastener(&f)].resultant() };
    assert!(ductile < elastic && close(ductile / elastic, top(FastenerForceMethod::InstantaneousCentre) / top(FastenerForceMethod::Elastic), 1e-9));
}

#[test]
fn e2_12b_net_width_of_aligned_and_staggered_hole_lines() {
    assert!(close(joints::net_width(0.140, 0.018, 3, 0.040, 0.0), 0.086, 1e-12));
    assert!(close(joints::net_width(0.140, 0.018, 3, 0.040, 0.030), 0.09725, 1e-12));
    assert!(close(joints::net_width(0.140, 0.018, 2, 0.040, 0.030), 0.140 - 0.036 + 0.030f64.powi(2) / (4.0 * 0.040), 1e-12));
    assert!(close(joints::net_width(0.140, 0.018, 1, 0.040, 0.030), 0.122, 1e-12));
    assert_eq!(joints::net_width(0.140, 0.018, 0, 0.040, 0.030), 0.140);
    assert!(close(joints::net_width(0.140, 0.018, 3, 0.040, 0.100), 0.104, 1e-12), "a large offset leaves the straight path through the two aligned holes");
    assert_eq!(joints::net_path(0.140, 0.018, 3, 0.040, 0.030), vec![0, 1, 2]);
    assert_eq!(joints::net_path(0.140, 0.018, 3, 0.040, 0.0), vec![0, 1, 2]);
    assert_eq!(joints::net_path(0.140, 0.018, 3, 0.040, 0.100), vec![0, 2]);
    assert!(joints::net_path(0.140, 0.018, 0, 0.040, 0.0).is_empty());
    assert!(close(joints::plate_width(2, 0.060, 0.040), 0.140, 1e-12) && close(joints::plate_width(1, 0.060, 0.040), 0.080, 1e-12));
    let (n_o, n_u) = joints::plate_tension_resistances(0.140, 0.104, 0.008, 260.0e6, 310.0e6, 1.1, 1.25);
    assert!(close(n_o, 264.727e3, 1e-6) && close(n_u, 185.702e3, 1e-6), "N_o={n_o} N_u={n_u}");
}

#[test]
fn e2_12c_the_net_section_check_follows_the_hole_pattern() {
    let doc = En1999Snapshot::compliant_roof_purlin();
    let c0 = &doc.connections[0];
    let id = c0.id.clone();
    let net = |d: &En1999Snapshot| evaluate_structure(d).checks.into_iter().find(|c| c.id == format!("en1999.8.5.net.{id}")).expect("net-section check");
    let base = net(&doc);
    let b = &c0.bolts;
    let alloy = part_1_1::resolve_alloy_for_thickness(&doc.materials.iter().find(|m| m.id == c0.material_id).expect("material").designation, b.plate_thickness).expect("alloy");
    let p = na_de::AnnexParams::for_choice(doc.annex);
    let width = joints::plate_width(b.bolts_per_row, b.gauge, b.edge_distance);
    let d0 = joints::hole_diameter(b.diameter);
    let (n_o, n_u) = joints::plate_tension_resistances(width, joints::net_width(width, d0, b.bolts_per_row, b.gauge, 0.0), b.plate_thickness, alloy.f_o_pa, alloy.f_u_pa, p.gamma_m1, p.gamma_m2);
    assert!(close(base.limit.value, n_o.min(n_u), 1e-9), "limit {} vs {}", base.limit.value, n_o.min(n_u));
    assert!(close(base.utilization, base.computed.value / base.limit.value, 1e-9));
    let mut staggered = doc.clone();
    staggered.connections[0].bolts.bolts_per_row = 4;
    staggered.connections[0].hole_stagger = 0.5 * b.pitch;
    let aligned = { let mut d = staggered.clone(); d.connections[0].hole_stagger = 0.0; d };
    assert!(net(&staggered).limit.value >= net(&aligned).limit.value, "staggering never lowers the net-section resistance");
    let mut thin = doc.clone();
    thin.connections[0].bolts.plate_thickness *= 0.25;
    let failed = net(&thin);
    assert!(failed.utilization > base.utilization * 3.9, "A_net scales with the plate thickness");
    if failed.utilization > 1.0 {
        assert!(failed.remedies.iter().any(|r| r.target.path.ends_with("bolts.plateThickness")) && failed.remedies.iter().any(|r| r.target.path.ends_with("bolts.edgeDistance")));
    }
}

fn rib() -> part_1_4_1_5::Rib { part_1_4_1_5::Rib { pitch: 0.200, top: 0.060, bottom: 0.060, height: 0.040 } }

#[test]
fn e2_13_trapezoidal_rib_gross_section_by_hand() {
    let r = rib();
    assert!(r.is_valid() && close(r.projection(), 0.040, 1e-12) && close(r.web_length(), 0.040 * 2.0f64.sqrt(), 1e-12) && close(r.web_angle(), std::f64::consts::FRAC_PI_4, 1e-12));
    let p = part_1_4_1_5::rib_props(&r, 0.001, [1.0; 3]);
    assert!(close(p.z_c, 0.020, 1e-12) && close(p.a, 0.001 * (0.120 + 0.080 * 2.0f64.sqrt()), 1e-12));
    assert!(close(p.inertia, 6.3099659e-8, 1e-7), "I={}", p.inertia);
    assert!(close(p.w_top, 3.0780321e-6, 1e-7) && close(p.w_bottom, p.w_top, 1e-12), "symmetric rib: equal moduli");
    assert!(!part_1_4_1_5::Rib { pitch: 0.100, ..r }.is_valid() && !part_1_4_1_5::Rib { height: 0.0, ..r }.is_valid());
    assert!(close(r.mirrored().top, r.bottom, 1e-15));
}

#[test]
fn e2_13b_effective_rib_bending_axial_and_shear_by_hand() {
    let (r, t, fo) = (rib(), 0.001, 250.0e6);
    let (k, p) = part_1_4_1_5::rib_effective(&r, t, fo, true, false);
    assert!(close(k[0], 32.0 / 60.0 - 220.0 / 3600.0, 1e-12) && k[2] == 1.0, "compressed flange β/ε = 60, tension flange gross");
    let psi = -p.z_c / (r.height - p.z_c);
    let eta = 0.70 + 0.30 * psi;
    let web = eta * r.web_length() / t;
    assert!(close(k[1], 32.0 / web - 220.0 / (web * web), 1e-9), "the web factor is the fixed point of the stress ratio");
    assert!(close(k[1], 0.8718097, 1e-6) && close(p.z_c, 0.01661260, 1e-6), "k_web={} z_c={}", k[1], p.z_c);
    let (m_sag, _) = part_1_4_1_5::rib_bending_resistance(&r, t, fo, true, false, false, 1.1);
    assert!(close(m_sag, 440.96877, 1e-6), "M_Rd,sag={m_sag}");
    let (m_hog, _) = part_1_4_1_5::rib_bending_resistance(&r, t, fo, true, false, true, 1.1);
    assert!(close(m_hog, m_sag, 1e-9), "a symmetric rib resists both senses alike");
    let skew = part_1_4_1_5::Rib { top: 0.100, bottom: 0.040, ..r };
    let (sag, hog) = (part_1_4_1_5::rib_bending_resistance(&skew, t, fo, true, false, false, 1.1).0, part_1_4_1_5::rib_bending_resistance(&skew, t, fo, true, false, true, 1.1).0);
    assert!(sag < hog, "the wide top flange buckles first under a sagging moment: {sag} {hog}");
    let (n_rd, a_eff) = part_1_4_1_5::rib_axial_resistance(&r, t, fo, true, false, 1.1);
    assert!(close(a_eff, 112.8885e-6, 1e-6) && close(n_rd, 25656.5, 1e-5), "A_eff={a_eff} N_Rd={n_rd}");
    let rho_v = part_1_4_1_5::sheet_web_rho_v(r.web_length() / t, fo, 0.35, 0.83);
    assert!(close(rho_v, 0.7014780, 1e-6) && part_1_4_1_5::sheet_web_rho_v(10.0, fo, 0.35, 0.83) == 1.0);
    assert!(close(part_1_4_1_5::rib_shear_resistance(&r, t, fo, rho_v, 1.1), 7363.61, 1e-6));
}

#[test]
fn e2_13c_a_profile_replaces_the_flat_strip_in_the_sheet_checks() {
    let mut doc = En1999Snapshot::compliant_roof_purlin();
    let sheet = doc.cold_formed[0].clone();
    let flat = evaluate_structure(&doc);
    let find = |r: &semio_s_plugin_norm_compliance::CheckReport, k: &str| r.checks.iter().find(|c| c.id == format!("en1999.1-4.{k}.{}", sheet.id)).unwrap_or_else(|| panic!("missing {k}")).clone();
    doc.sheet_profiles.push(semio_s_artifact_norm_en1999_model::SheetProfile { id: "prof-1".into(), sheet_id: sheet.id.clone(), pitch: 0.200, top_flange: 0.060, bottom_flange: 0.060, web_height: 0.040 });
    let profiled = evaluate_structure(&doc);
    assert!(find(&profiled, "bend").limit.value > 0.0 && find(&profiled, "bend").limit.value != find(&flat, "bend").limit.value, "the rib resistance replaces b·t²/6");
    let (r, scale) = (rib(), sheet.width / 0.200);
    let alloy = part_1_1::resolve_alloy_for_thickness(&doc.materials.iter().find(|m| m.id == sheet.material_id).expect("material").designation, sheet.thickness).expect("alloy");
    let p = na_de::AnnexParams::for_choice(doc.annex);
    let rho_haz = if sheet.welded { alloy.rho_o_haz } else { 1.0 };
    let sag = part_1_4_1_5::rib_bending_resistance(&r, sheet.thickness, alloy.f_o_pa, alloy.buckling_class_a, sheet.welded, false, p.gamma_m1).0;
    let hog = part_1_4_1_5::rib_bending_resistance(&r, sheet.thickness, alloy.f_o_pa, alloy.buckling_class_a, sheet.welded, true, p.gamma_m1).0;
    assert!(close(find(&profiled, "bend").limit.value, sag.min(hog) * scale * rho_haz, 1e-9));
    let (n_rd, _) = part_1_4_1_5::rib_axial_resistance(&r, sheet.thickness, alloy.f_o_pa, alloy.buckling_class_a, sheet.welded, p.gamma_m1);
    assert!(close(find(&profiled, "axial").limit.value, n_rd * scale * rho_haz, 1e-9));
    let shear = find(&profiled, "shear");
    assert!(shear.ndps.iter().any(|n| n.as_str() == "ndp.en1999.sheet-web-shear-factor"), "the sheeting web rule rests on unverified rows: {:?}", shear.ndps);
    let ok = evaluate_structure(&doc).checks.into_iter().find(|c| c.id == "en1999.1-4.profile.prof-1").expect("geometry check");
    assert!(close(ok.utilization, 0.6, 1e-12));
    doc.sheet_profiles[0].pitch = 0.100;
    let bad = evaluate_structure(&doc).checks.into_iter().find(|c| c.id == "en1999.1-4.profile.prof-1").expect("geometry check");
    assert_eq!(bad.status, CheckStatus::Fail);
    assert!(bad.remedies.iter().any(|r| r.target.path.ends_with("sheetProfiles[id=prof-1].pitch")));
    doc.sheet_profiles[0].sheet_id = "nobody".into();
    assert!(evaluate_structure(&doc).checks.iter().any(|c| c.id == "en1999.1-4.profile-ref.prof-1" && c.status == CheckStatus::Fail));
}

#[test]
fn e2_14_protected_member_temperature_by_the_insulation_formula() {
    let ins = part_1_2::Insulation { thickness: 0.020, conductivity: 0.20, specific_heat: 1200.0, density: 800.0 };
    assert_eq!(part_1_2::protected_temperature_c(20.0, 200.0, 5.0, &ins), 20.0, "the exponential term holds the member back in the first steps");
    assert!(close(part_1_2::protected_temperature_c(20.0, 200.0, 300.0, &ins), 45.195371, 1e-6));
    assert!(close(part_1_2::protected_temperature_c(20.0, 200.0, 900.0, &ins), 192.428219, 1e-6));
    assert!(close(part_1_2::protected_temperature_c(20.0, 200.0, 1800.0, &ins), 398.450595, 1e-6));
    let (mut previous, mut thicker_cooler) = (20.0, true);
    for t in (1..=36).map(|n| f64::from(n) * 50.0) {
        let now = part_1_2::protected_temperature_c(20.0, 200.0, t, &ins);
        assert!(now >= previous - 1e-9 && now <= part_1_2::unprotected_temperature_c(20.0, 200.0, t) + 1e-9, "monotone and below the bare member at {t} s");
        thicker_cooler &= now >= part_1_2::protected_temperature_c(20.0, 200.0, t, &part_1_2::Insulation { thickness: 0.030, ..ins }) - 1e-9;
        previous = now;
    }
    assert!(thicker_cooler, "a thicker insulation never heats the member faster");
    let bare = part_1_2::protected_temperature_c(20.0, 200.0, 900.0, &part_1_2::Insulation { thickness: 0.0, ..ins });
    assert_eq!(bare, part_1_2::unprotected_temperature_c(20.0, 200.0, 900.0), "no thickness is the unprotected member");
}
