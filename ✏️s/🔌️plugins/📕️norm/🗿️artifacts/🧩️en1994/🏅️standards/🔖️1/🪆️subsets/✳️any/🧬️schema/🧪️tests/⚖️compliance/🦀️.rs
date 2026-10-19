use crate::*;
use semio_s_plugin_norm_compliance::AnnexChoice;
use semio_s_artifact_norm_en1994_model::{CompositeBeam, HeadedStuds, ProfiledSheeting, SteelSection};

fn sample_beam() -> CompositeBeam {
    CompositeBeam::default_placeholder()
}

#[test]
fn effective_width_8m_span() {
    let beff = part_1_1::effective_width_m(8.0, 0.300, 3.0);
    assert!((beff - 2.30).abs() < 0.01);
}

#[test]
fn stud_connector_resistance_worked_example() {
    let mut beam = sample_beam();
    beam.sheeting.ribs_parallel_to_beam = true;
    beam.studs = HeadedStuds { diameter_m: 0.019, height_m: 0.095, f_u_pa: 450e6, count_per_rib: 1, spacing_m: 0.2, total_count: 40 };
    beam.concrete_f_ck_pa = 30e6;
    beam.concrete_e_cm_pa = 33e9;
    let p_rd = part_1_1::connector_resistance_n(&beam, AnnexChoice::En);
    assert!((p_rd / 1000.0 - 81.656).abs() < 0.05, "P_Rd={}", p_rd / 1000.0);
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
    let lim_en = part_2::steel_fatigue_resistance_pa("stud_welded", 2.0e6, AnnexChoice::En);
    let lim_de = part_2::steel_fatigue_resistance_pa("stud_welded", 2.0e6, AnnexChoice::De);
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
    let limit = part_2::steel_fatigue_resistance_pa("stud_welded", 2.0e6, AnnexChoice::En);
    assert!((limit / 1e6 - 80.0 / 1.15).abs() < 0.1);
}
