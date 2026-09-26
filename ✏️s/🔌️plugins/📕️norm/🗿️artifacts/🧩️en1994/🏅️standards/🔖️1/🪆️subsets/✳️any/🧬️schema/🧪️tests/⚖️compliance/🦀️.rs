use super::*;
use crate::document::AnnexChoice;
use crate::{CompositeBeam, HeadedStuds, ProfiledSheeting, SteelSection};

fn sample_beam() -> CompositeBeam {
    CompositeBeam::default_placeholder()
}

#[test]
fn effective_width_8m_span() {
    let beff = part_1_1::effective_width_m(8.0, 0.300, 3.0);
    // be1 = 8/8 + 0.15 = 1.15; 2*be1=2.30; be2=1.5; 2*be2=3.0 → min = 2.30 m
    assert!((beff - 2.30).abs() < 0.01);
}

#[test]
fn stud_connector_resistance_worked_example() {
    let mut beam = sample_beam();
    beam.sheeting.ribs_parallel_to_beam = true; // k_t = 1
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
    // EN 1994-2 §6.8.2 → EN 1993-1-9 Table 3.1; DIN EN 1993-1-9/NA safe-life γ_Mf = 1,35 vs EN damage-tolerant 1,15.
    let en = AnnexParams::en().gamma_mf;
    let de = AnnexParams::de().gamma_mf;
    assert!((en - 1.15).abs() < 1e-9);
    assert!((de - 1.35).abs() < 1e-9);
    let lim_en = part_2::steel_fatigue_resistance_pa("stud_welded", 2.0e6, AnnexChoice::En);
    let lim_de = part_2::steel_fatigue_resistance_pa("stud_welded", 2.0e6, AnnexChoice::De);
    assert!(lim_de < lim_en);
}

#[test]
fn bridge_fatigue_uses_detail_as_strength_not_as_mpa_limit() {
    let limit = part_2::steel_fatigue_resistance_pa("stud_welded", 2.0e6, AnnexChoice::En);
    // Δσ_c = 80 MPa / γ_Mf(1.15) at N_ref (EN 1993-1-9 detail category via EN 1994-2 §6.8.2)
    assert!((limit / 1e6 - 80.0 / 1.15).abs() < 0.1);
}
