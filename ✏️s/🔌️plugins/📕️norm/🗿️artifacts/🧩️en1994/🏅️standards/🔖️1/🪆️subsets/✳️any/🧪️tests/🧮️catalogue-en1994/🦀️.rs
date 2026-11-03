//! 🦀️ EN 1994 catalogue worked examples — the Rust SUBJECT adapter. Each published quantity of the examples E1-01 to E1-22 is evaluated by the catalogue kernel and must reproduce its stated value within the stated relative tolerance; the independent Python oracle answers the same rows in the oracle role. Step parsing, tolerances and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1994_checks::catalogue::*;
    use std::f64::consts::PI;

    const FY: f64 = 355e6;
    const FCC: f64 = 17e6;
    const Q: f64 = 10e3;
    const EI1: f64 = 210e9 * 6e-4;

    fn ipe300() -> ISection {
        ISection { h: 0.300, b: 0.150, tf: 0.0107, tw: 0.0071, a: 5381e-6, i_y: 83.56e-6 }
    }

    fn shear_plastic_n(sec: &ISection) -> f64 {
        (sec.a - 2.0 * sec.b * sec.tf + (sec.tw + 2.0 * 0.015) * sec.tf) * FY / 3.0_f64.sqrt()
    }

    fn filled_column() -> (f64, f64) {
        let ncr = PI * PI * ei_eff(121.583e-6, 0.0, 418.690e-6, 33e9, 0.6) / 16.0;
        (ncr, chi_column((5676.9e3 / ncr).sqrt(), 0.21))
    }

    fn rhs_polygon() -> (ColSection, [(f64, f64); 4]) {
        let section = ColSection { b: 0.2, t: 0.01, as_m2: 0.0, yr: 0.0, fyd: FY, fcd: 26.667e6, fsd: 0.0 };
        let polygon = interaction_polygon(&section);
        (section, polygon)
    }

    fn slab() -> SlabSection {
        SlabSection { h: 0.150, hp: 0.050, ap: 1100e-6, ep: 0.020, e: 0.025, fyp_d: 350e6, fcd: 20e6 }
    }

    fn composite_inertia(modular: f64) -> f64 {
        transformed(&ipe300(), 0.100, 2.0, 0.100, modular).i
    }

    fn beam_fire() -> beam_fire::Beam {
        beam_fire::Beam { sec: ipe300(), fyd: FY, ys: 0.150, thermal_m: 0.150, beff: 2.4, hc: 0.100, fcc: 0.85 * 30e6, am_v: 150.0, board_m: 0.02, rebar: column_fire::RebarProduct::HotRolled }
    }

    fn column_fire_cell(i: usize, j: usize) -> f64 {
        let column = encased::EncasedI { kind: encased::Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.013, as_m2: 0.0, cover: 0.05 };
        column_fire::temperature_field(&column, 3600.0).at(i, j)
    }

    fn continuous_solution(position: SpanPosition, span_m: f64, adjacent_m: f64, loads: &[f64], stiffness: continuous::Stiffness) -> continuous::Solution {
        continuous::solve(&continuous::Layout::of(position, span_m, adjacent_m), loads, stiffness)
    }

    fn cracked(ei1: f64) -> continuous::Stiffness {
        continuous::Stiffness { ei1, ei2: 0.5 * ei1, cracked_fraction: 0.15 }
    }

    fn encased_polygon() -> [(f64, f64); 4] {
        let section = encased::EncasedI { kind: encased::Encasement::Full, b_c: 0.4, h_c: 0.4, h: 0.2, b: 0.2, tw: 0.0085, tf: 0.013, as_m2: 4.0 * 314.16e-6, cover: 0.05 };
        encased::interaction_polygon(&section, &encased::Strengths { fyd: FY, fcd: 0.85 * 30e6 / 1.5, fsd: 500e6 / 1.15 })
    }

    fn composite_inertia_at(ecm: f64, psi_l: f64) -> f64 {
        transformed(&ipe300(), 0.100, 2.0, 0.100, modular_ratio(ecm, psi_l, 2.5)).i
    }

    fn creep_load_deflection() -> f64 {
        let ecm = ecm_pa(30e6);
        deflection_simple(8.0, 6e3, 4e3, composite_inertia_at(ecm, constants::PSI_L_PERMANENT), composite_inertia_at(ecm, 0.0))
    }

    fn creep_shrinkage_deflection() -> f64 {
        let (ecm, sec) = (ecm_pa(30e6), ipe300());
        let n_s = modular_ratio(ecm, constants::PSI_L_SHRINKAGE, 2.5);
        let el = transformed(&sec, 0.100, 2.0, 0.100, n_s);
        deflection_shrinkage(8.0, 0.3e-3, 2.0, 0.100, n_s, el.y_na - 0.050, el.i)
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        let sec = ipe300();
        let full = plastic_full(&sec, FY, 0.100, 2.0, 0.100, FCC);
        let (n0, nl) = (modular_ratio(33e9, 0.0, 0.0), modular_ratio(33e9, 1.1, 2.5));
        Ok(match name {
            "beff_simple" => effective_width_m(8.0, 0.0, 1.5, 1.5, false),
            "beff_end" => effective_width_m(8.0, 0.0, 1.5, 1.5, true),
            "mpl_full" => full.m_nm,
            "mpl_linear" => plastic_linear(628.4e-6 * FY, full.m_nm, 0.5),
            "mpl_partial_sb" => plastic_sagging(&sec, FY, 0.100, 2.0, 0.100, FCC, 0.5 * full.nc_n).m_nm,
            "eta_min_8m" => eta_min(8.0, FY, true),
            "prd1" => stud_resistance_n(0.019, 0.100, 450e6, 30e6, 33e9, 1.25).0,
            "prd2" => stud_resistance_n(0.019, 0.100, 450e6, 30e6, 33e9, 1.25).1,
            "prd_sheet" => stud_resistance_n(0.019, 0.100, 450e6, 30e6, 33e9, 1.25).2 * sheeting_reduction(0.050, 0.100, 0.095, 0.019, 0.0009, 1, false).ok_or("sheeting reduction undefined")?,
            "prd_two_per_rib" => stud_resistance_n(0.019, 0.100, 450e6, 30e6, 33e9, 1.25).2 * sheeting_reduction(0.050, 0.100, 0.095, 0.019, 0.0009, 2, false).ok_or("sheeting reduction undefined")?,
            "stud_gap" => stud_spacing_rule(0.019, 0.085, 0.60).0,
            "asf_req" => slab_shear(81_656.0 / 0.150, 0.200, 0.100, 30e6, 1.5, 1.15, 1.0).2,
            "vpl" => shear_plastic_n(&sec),
            "rho_shear" => shear_rho(400e3, shear_plastic_n(&sec)),
            "ncr_col" => filled_column().0,
            "chi_col" => filled_column().1,
            "mmax_rhs" => rhs_polygon().1[2].1,
            "mpl_rhs" => rhs_polygon().1[3].1,
            "mu_1700" => {
                let (_, polygon) = rhs_polygon();
                mu_d(&polygon, 1700e3)
            }
            "slab_full" => slab_bending(&slab(), 4.0e3, f64::INFINITY).0,
            "slab_partial" => slab_bending(&slab(), 4.0e3, 0.6 * 385e3).0,
            "slab_mk" => slab_mk(0.125, 1100e-6, 1.0, 200e6, 0.04e6, 1.25),
            "slab_vv" => slab_vertical_shear(0.400, 0.125, 1100e-6, 30e6, 1.5),
            "i_n0" => composite_inertia(n0),
            "i_nl" => composite_inertia(nl),
            "defl" => deflection_simple(8.0, 6e3, 4e3, composite_inertia(nl), composite_inertia(n0)),
            "as_min" => as_min_m2(2.9e6, 2.0 * 0.100, 0.100, 0.250, 500e6),
            "iso_60" => iso834_c(3600.0),
            "iso_30" => iso834_c(1800.0),
            "mnl_ipe300" => nonlinear_mrd(&sec, 0.100, 2.0, 0.100, &NonlinearInput::standard(FY, FCC, 0.0, 0.0)),
            "kappa_u_ipe300" => ultimate_curvature(&sec, 0.100, 2.0, 0.100, &NonlinearInput::standard(FY, FCC, 0.0, 0.0)),
            "cont_end_mb_cracked" => -continuous_solution(SpanPosition::End, 8.0, 8.0, &[Q, Q], cracked(EI1)).supports[1],
            "cont_end_defl_cracked" => continuous_solution(SpanPosition::End, 8.0, 8.0, &[Q, Q], cracked(EI1)).deflection_own(),
            "cont_interior_defl_cracked" => continuous_solution(SpanPosition::Interior, 8.0, 6.0, &[Q, Q, Q], cracked(EI1)).deflection_own(),
            "cont_cantilever_tip_cracked" => continuous_solution(SpanPosition::Cantilever, 3.0, 8.0, &[0.0, Q], cracked(EI1)).deflection_own(),
            "cont_end_unequal_mb" => -continuous_solution(SpanPosition::End, 8.0, 6.0, &[Q, Q], continuous::Stiffness::uniform(EI1)).supports[1],
            "enc_mb" => encased_polygon()[3].1,
            "enc_md" => encased_polygon()[2].1,
            "crack_band_k6" => continuous::hogging_band_profile(&continuous::Layout::of(SpanPosition::End, 8.0, 8.0), cracked(EI1), 25)[0][6].1,
            "fire_core_60" => column_fire_cell(0, 0),
            "fire_flange_60" => column_fire_cell(0, 9),
            "fire_corner_60" => column_fire_cell(19, 19),
            "beta1_k01" => bridge::beta_sagging(0.1),
            "beta2_k01" => bridge::beta_hogging(0.1),
            "girder_cross_ribbed" => bridge::stud_cross_spacing_min_m(0.022, false),
            "girder_cross_solid" => bridge::stud_cross_spacing_min_m(0.022, true),
            "delta_sigma_ts_example" => delta_sigma_ts(fctm_pa(30e6), 0.02, 8e-4, 0.01, 2e-4, 0.015),
            "girder_as_min" => bridge::min_transverse_reinforcement_m2_per_m(0.25),
            "end_plate_open" => bridge::end_plate_bearing_n(0.85 * 30e6 / 1.5, 0.2, 0.1, 1.0, 0.25),
            "end_plate_capped" => bridge::end_plate_bearing_n(0.85 * 30e6 / 1.5, 0.1, 0.1, 5.0, 5.0),
            "beam_fire_steel_60" => beam_fire().evolve(&[60.0]).points[0].steel_c,
            "beam_fire_soffit_60" => beam_fire().evolve(&[60.0]).fields[0].at(47, 0),
            "beam_fire_mid_60" => beam_fire().evolve(&[60.0]).fields[0].at(47, 7),
            "beam_fire_mfi_120" => beam_fire().evolve(&[120.0]).points[0].reduced.steel_only_nm,
            "fatigue_stud_endurance" => fatigue::Curve { delta_c_pa: 90e6, n_ref: 2e6, m: 8.0, gamma: 1.15 }.endurance(40e6),
            "fatigue_stud_damage" => fatigue::Curve { delta_c_pa: 90e6, n_ref: 2e6, m: 8.0, gamma: 1.15 }.damage(40e6, 5e6),
            "delta_sigma_ts_fatigue_example" => delta_sigma_ts_fatigue(fctm_pa(30e6), 0.02, 8e-4, 0.01, 2e-4, 0.015),
            "fatigue_rebar_sigma_max" => bridge::rebar_fatigue(300e3, 100e3, 0.34, 0.04, 8e-4, delta_sigma_ts_fatigue(fctm_pa(30e6), 0.02, 8e-4, 0.01, 2e-4, 0.015)).sigma_max_f_pa,
            "fatigue_rebar_util_pass" => fatigue::rebar_curve().utilisation(bridge::rebar_fatigue(300e3, 100e3, 0.34, 0.04, 8e-4, delta_sigma_ts_fatigue(fctm_pa(30e6), 0.02, 8e-4, 0.01, 2e-4, 0.015)).delta_equ_pa, 2e6),
            "fatigue_rebar_util_fail" => fatigue::rebar_curve().utilisation(bridge::rebar_fatigue(400e3, 400e3, 0.34, 0.04, 8e-4, delta_sigma_ts_fatigue(fctm_pa(30e6), 0.02, 8e-4, 0.01, 2e-4, 0.015)).delta_equ_pa, 2e6),
            "fatigue_rebar_damage_ts" => fatigue::rebar_curve().damage(bridge::rebar_fatigue(300e3, 100e3, 0.34, 0.04, 8e-4, delta_sigma_ts_fatigue(fctm_pa(30e6), 0.02, 8e-4, 0.01, 2e-4, 0.015)).delta_equ_pa, 2e6),
            "chi_lt_rolled_lambda" => chi_lt(500e3, 1500e3, 1.8, false).0,
            "chi_lt_rolled" => chi_lt(500e3, 1500e3, 1.8, false).1,
            "chi_lt_welded" => chi_lt(500e3, 1500e3, 3.0, true).1,
            "patch_f_rd" => patch_loading(0.6, 0.008, 0.2, 0.014, 355e6, 355e6, 0.1, 1.0, 1.0).0,
            "patch_chi" => patch_loading(0.6, 0.008, 0.2, 0.014, 355e6, 355e6, 0.1, 1.0, 1.0).1,
            "patch_ly" => patch_loading(0.6, 0.008, 0.2, 0.014, 355e6, 355e6, 0.1, 1.0, 1.0).2,
            "patch_interaction" => patch_interaction(150e3, patch_loading(0.6, 0.008, 0.2, 0.014, 355e6, 355e6, 0.1, 1.0, 1.0).0, 300e3, 450e3),
            "fatigue_rebar_damage" => fatigue::rebar_curve().damage(100e6, 2e6),
            "fatigue_steel_util" => fatigue::Curve { delta_c_pa: 80e6, n_ref: 2e6, m: 3.0, gamma: 1.15 }.utilisation(50e6, 1e7),
            "vibration_mass" => vibration::mass_per_length_kg_m(10e3, 15e3, 0.3),
            "vibration_f1" => vibration::natural_frequency_hz(8.0, 6e-4, vibration::mass_per_length_kg_m(10e3, 15e3, 0.3)),
            "vibration_utilisation" => vibration::frequency_utilisation(3.0, vibration::natural_frequency_hz(8.0, 6e-4, vibration::mass_per_length_kg_m(10e3, 15e3, 0.3))),
            "ltb_ks" => uframe_ks(4e-5, 3.0, &sec, 2.0),
            "ltb_mcr" => mcr_uframe(&sec, 6.0, uframe_ks(4e-5, 3.0, &sec, 2.0)),
            "ltb_lambda" => chi_lt(sec.w_pl() * FY, mcr_uframe(&sec, 6.0, uframe_ks(4e-5, 3.0, &sec, 2.0)), 2.0, false).0,
            "ltb_chi" => chi_lt(sec.w_pl() * FY, mcr_uframe(&sec, 6.0, uframe_ks(4e-5, 3.0, &sec, 2.0)), 2.0, false).1,
            "ltb_mbrd" => chi_lt(sec.w_pl() * FY, mcr_uframe(&sec, 6.0, uframe_ks(4e-5, 3.0, &sec, 2.0)), 2.0, false).1 * sec.w_pl() * FY,
            "creep_ns" => modular_ratio(ecm_pa(30e6), constants::PSI_L_SHRINKAGE, 2.5),
            "creep_i_ns" => composite_inertia_at(ecm_pa(30e6), constants::PSI_L_SHRINKAGE),
            "creep_defl_load" => creep_load_deflection(),
            "creep_defl_shrinkage" => creep_shrinkage_deflection(),
            "creep_defl_total" => creep_load_deflection() + creep_shrinkage_deflection(),
            "fire_crit_mu05" => critical_temperature_c(0.5),
            "fire_crit_mu03" => critical_temperature_c(0.3),
            "fire_protected_90" => protected_steel_temperature_c(5400.0, 150.0, 0.02, 0.12, 1000.0, 800.0),
            other => return Err(format!("EN 1994 catalogue has no quantity {other}")),
        })
    }

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.vector = Box::new(|_, _example, name| quantity(name));
        bound
    }
}

/// 🧭️ Registration is by scenario family; the oracle role is the Python adapter beside this file.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = steps::norm_adapter(subject::binding());
    }
    built
}
