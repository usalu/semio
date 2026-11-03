//! 🧪 Worked examples of the critical region kernel, recomputed with numpy in `🔮️oracles/🚦️check-outcome/🐍️.py` (`region_utilisations`) and by hand.

use super::*;

fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1.0e-9 * b.abs().max(1.0), "{a} vs {b}");
}

fn region() -> En1998CriticalRegion {
    En1998CriticalRegion {
        id: "cr-1".into(),
        building_id: "bldg-office".into(),
        member_id: "col-c1".into(),
        section_width_m: 0.4,
        section_depth_m: 0.5,
        clear_length_m: 3.0,
        hooped_length_m: 0.75,
        core_width_m: 0.32,
        core_depth_m: 0.42,
        axial_force_n: 800_000.0,
        concrete_design_strength_mpa: 20.0,
        steel_design_yield_mpa: 434.8,
        hoop_design_yield_mpa: 434.8,
        longitudinal_steel_area_m2: 0.0048,
        longitudinal_bar_diameter_m: 0.016,
        longitudinal_bar_max_diameter_m: 0.02,
        hoop_diameter_m: 0.01,
        hoop_spacing_m: 0.08,
        hoop_legs_x: 3.0,
        hoop_legs_y: 3.0,
        restrained_bar_count: 8.0,
        restrained_bar_spacing_m: 0.11,
    }
}

fn column(class: DuctilityClass) -> Detailing {
    detail(&region(), MemberKind::Column, class, 4.5, 0.6, 0.5, AnnexChoice::En).expect("column detailing")
}

#[test]
fn the_confinement_of_a_dch_column_matches_the_hand_calculation() {
    let d = column(DuctilityClass::Dch);
    near(d.alpha_n, 0.8799603174603174);
    near(d.alpha_s, 0.7916666666666666);
    near(d.alpha, 0.6966352513227513);
    near(d.omega_wd, 0.35254384747412915);
    near(d.nu_d, 0.2);
    near(d.eps_sy_d, 0.002174);
    near(d.mu_phi, 8.0);
    near(d.confinement_required, 0.09544);
    near(d.omega_wd_min, 0.12);
    near(d.rho_tot, 0.024);
    near(d.nu_d_max, 0.55);
    assert!(d.alpha * d.omega_wd > d.confinement_required);
}

#[test]
fn dch_tightens_the_limits_of_dcm() {
    let (dcm, dch) = (column(DuctilityClass::Dcm), column(DuctilityClass::Dch));
    near(dcm.hoop_spacing_max_m, 0.128);
    near(dch.hoop_spacing_max_m, 0.096);
    near(dcm.hoop_diameter_min_m, 0.006);
    near(dch.hoop_diameter_min_m, 0.008);
    near(dcm.bar_spacing_max_m, 0.2);
    near(dch.bar_spacing_max_m, 0.15);
    near(dcm.omega_wd_min, 0.08);
    near(dcm.nu_d_max, 0.65);
    near(dcm.critical_length_m, 0.5);
}

#[test]
fn a_stocky_column_is_a_critical_region_over_its_whole_clear_length() {
    let stocky = En1998CriticalRegion { clear_length_m: 1.2, ..region() };
    let d = detail(&stocky, MemberKind::Column, DuctilityClass::Dcm, 4.5, 0.6, 0.5, AnnexChoice::En).unwrap();
    assert!(d.whole_height);
    near(d.critical_length_m, 1.2);
    let slender = En1998CriticalRegion { clear_length_m: 6.0, ..region() };
    near(detail(&slender, MemberKind::Column, DuctilityClass::Dcm, 4.5, 0.6, 0.5, AnnexChoice::En).unwrap().critical_length_m, 1.0);
}

#[test]
fn a_beam_takes_the_depth_based_region_and_spacing() {
    let d = detail(&region(), MemberKind::Beam, DuctilityClass::Dch, 4.5, 0.6, 0.5, AnnexChoice::En).unwrap();
    assert!(!d.column);
    near(d.critical_length_m, 0.75);
    near(d.hoop_spacing_max_m, 0.096);
    assert!(d.omega_wd.is_nan() && d.rho_tot.is_nan());
    let dcm = detail(&region(), MemberKind::Beam, DuctilityClass::Dcm, 4.5, 0.6, 0.5, AnnexChoice::En).unwrap();
    near(dcm.critical_length_m, 0.5);
    near(dcm.hoop_spacing_max_m, 0.125);
}

#[test]
fn the_curvature_ductility_follows_the_period_branch() {
    near(curvature_ductility(4.5, 0.6, 0.5), 8.0);
    near(curvature_ductility(3.0, 0.3, 0.5), 1.0 + 2.0 * 2.0 * 0.5 / 0.3);
    assert!(curvature_ductility(3.0, 0.0, 0.5).is_nan());
}

#[test]
fn low_ductility_walls_and_braces_have_no_critical_region_rules() {
    assert!(detail(&region(), MemberKind::Column, DuctilityClass::Dcl, 4.5, 0.6, 0.5, AnnexChoice::En).is_none());
    assert!(detail(&region(), MemberKind::Wall, DuctilityClass::Dch, 4.5, 0.6, 0.5, AnnexChoice::En).is_none());
    assert!(detail(&region(), MemberKind::Brace, DuctilityClass::Dch, 4.5, 0.6, 0.5, AnnexChoice::En).is_none());
    assert!(detail(&En1998CriticalRegion { core_width_m: 0.0, ..region() }, MemberKind::Column, DuctilityClass::Dch, 4.5, 0.6, 0.5, AnnexChoice::En).is_none());
}

#[test]
fn the_governing_ratio_is_the_largest_of_required_over_provided() {
    let r = region();
    let d = column(DuctilityClass::Dch);
    near(governing_ratio(&r, &d), 0.8333333333333334);
    let wide = En1998CriticalRegion { hoop_spacing_m: 0.12, ..region() };
    let dw = detail(&wide, MemberKind::Column, DuctilityClass::Dch, 4.5, 0.6, 0.5, AnnexChoice::En).unwrap();
    near(governing_ratio(&wide, &dw), 0.12 / 0.096);
}

#[test]
fn the_confinement_falls_with_the_hoop_spacing() {
    let reach = |s: f64| {
        let probe = En1998CriticalRegion { hoop_spacing_m: s, ..region() };
        alpha_n(8.0, 0.11, 0.32, 0.42) * alpha_s(s, 0.32, 0.42) * omega_wd(&probe)
    };
    assert!(reach(0.05) > reach(0.08) && reach(0.08) > reach(0.12));
}

#[test]
fn a_heavily_loaded_column_is_governed_by_the_normalised_axial_force() {
    let loaded = En1998CriticalRegion { axial_force_n: 3_000_000.0, ..region() };
    let d = detail(&loaded, MemberKind::Column, DuctilityClass::Dch, 4.5, 0.6, 0.5, AnnexChoice::En).unwrap();
    near(d.nu_d, 0.75);
    near(governing_ratio(&loaded, &d), 0.75 / 0.55);
}
