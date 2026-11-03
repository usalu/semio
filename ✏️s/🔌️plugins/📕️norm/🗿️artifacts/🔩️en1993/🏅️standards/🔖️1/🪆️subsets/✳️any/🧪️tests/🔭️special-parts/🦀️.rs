//! 🦀️ EN 1993 special-part clause vectors and wired checks — the Rust SUBJECT adapter. Every quantity is evaluated by the clause functions of `🧮️clauses` (or by the compliance report of the special-parts showcase subject) and compared by the feature's relative tolerance with the independent Python oracle running in the oracle role. Step parsing, tolerances and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1993_checks::clauses::{
        bridge_fatigue::*, cold_formed::*, containment::*, crane_runway::*, joint_components::*, misc::cold_forming_shift, plated_ext::*, sheet_piling::*, shell_stability::*, stainless::*, tension_components::*, toughness::*, towers_masts::*,
    };
    use semio_s_artifact_norm_en1993_checks::clauses::{classification::{angle_class, chs_class}, parametric_sections::{angle, chs, chs_torsion}};
    use semio_s_artifact_norm_en1993_checks::check_full_steel_structure;
    use semio_s_artifact_norm_en1993_model::En1993Snapshot;
    use std::f64::consts::PI;

    fn edge_stiffened() -> EdgeStiffened {
        EdgeStiffened { bp: 60.0, cp: 20.0, t: 1.5, fyb: 350.0, hw: 150.0, b2: 60.0, kf: 1.0, e: 210000.0 }
    }

    fn fbv_cold() -> f64 {
        f_bv(lambda_w_cold(150.0, 1.5, 350.0, 210000.0), 350.0, false)
    }

    fn t_stub(method: Mode1Method, lever: f64) -> TStubModes {
        t_stub_modes(280.0, 300.0, 15.0, 355.0, 40.0, 50.0, 8.0, 282240.0, 245.0, 2.0, lever, method, 1.0)
    }

    fn c_strip(plate_width: f64) -> CStrip {
        base_plate_c_strip(17.0, 240.0, 30.0, 355.0, 1.0, 350.0, plate_width, 2.0 / 3.0, 20.0, &|b, l| (b * l, 9.0 * b * l))
    }

    fn patch(kind: PatchType) -> f64 {
        patch_resistance(kind, 1000.0, 8.0, 2000.0, 20.0, 300.0, 355.0, 355.0, 100.0, 150.0, 1.0).f_rd
    }

    fn row_forces() -> Vec<f64> {
        let group = |i: usize, j: usize| match (i, j) {
            (0, 1) => 420.0e3,
            (0, 2) => 560.0e3,
            _ => 380.0e3,
        };
        distribute_rows(&[300.0e3, 250.0e3, 200.0e3], &group, 650.0e3)
    }

    fn crane_web() -> (f64, f64) {
        let le = l_eff(i_rf(1.2e6, 200.0, 20.0, true), 12.0);
        (le, sigma_oz(180.0e3, le, 12.0))
    }

    fn silo() -> (f64, f64, f64, f64, f64, f64) {
        (9.0e-6, 0.5, 0.4, PI * 9.0e6, 2.0 * PI * 3000.0, 12000.0)
    }

    fn cold_forming_t_ed() -> Result<f64, String> {
        Ok(brittle_fracture_utilisation("S355", "J2", 0.75, 1.0, 10.0, 0.0, 0.0, 0.0, 0.0, cold_forming_shift(20.0, 100.0)).ok_or("brittle fracture inputs outside the table")?.0)
    }

    fn tmax(temperature: f64) -> Result<f64, String> {
        max_thickness("S355", "J2", 0.75, temperature).ok_or_else(|| format!("no S355 J2 thickness limit at {temperature} °C"))
    }

    fn cell_count(unset_only: bool) -> f64 {
        GRADES.iter().flat_map(|(g, subs)| subs.iter().map(move |s| (*g, *s))).flat_map(|(g, s)| LEVELS.iter().flat_map(move |(_, key)| TEMPERATURES.iter().map(move |&t| cell_id(g, s, key, t)))).filter(|id| !unset_only || cell(id).is_none()).count() as f64
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        let (gamma, kk, mu, area, circumference, depth) = silo();
        let shell = axial_design(3000.0, 1000.0, 5.0, 235.0, 'B', 6.0, 1.1);
        let hoop = hoop_design(3000.0, 1000.0, 5.0, 235.0, 'B', 1.5, 1.1);
        let edge = distortional_edge_stiffener(&edge_stiffened(), 1.0);
        let stainless = stainless_fire_factors("1.4301", 650.0).ok_or("no stainless fire factors at 650 °C")?;
        let (z_eq, k_eq) = z_eq_and_k_eq(&[12.0, 8.0], &[300.0, 220.0]);
        let (le, soz) = crane_web();
        Ok(match name {
            "cf_chi_d" => edge.chi_d,
            "cf_lambda_d" => edge.lambda_d,
            "cf_sigma_cr_s" => edge.sigma_cr_s,
            "cf_a_s" => edge.a_s,
            "cf_i_s" => edge.i_s,
            "cf_fbv" => fbv_cold(),
            "cf_vb_rd" => v_b_rd_cold(150.0, 90.0, 1.5, fbv_cold(), 1.0),
            "cf_rw_rd" => web_crippling_rd(0.075, 1.5, 350.0, 210000.0, 3.0, 10.0, 90.0, 1.0),
            "st_rho_int_cold" => rho_internal_stainless(0.9, false),
            "st_rho_int_weld" => rho_internal_stainless(0.9, true),
            "st_rho_out_cold" => rho_outstand_stainless(0.9, false),
            "st_rho_out_weld" => rho_outstand_stainless(0.9, true),
            "st_es" => secant_modulus(200.0, 200000.0, 220.0, 7.0),
            "st_k02_650" => stainless.0,
            "st_ke_650" => stainless.1,
            "st_chi_flex" => chi_flexural_stainless(1.0, StainlessMember::ColdFormedOpen),
            "st_chi_lt_welded" => chi_lt_stainless(0.8, StainlessMember::WeldedOpen),
            "st_k_int" => k_interaction(0.9, 0.3),
            "pl_rho_c" => rho_c_interpolated(0.8, 0.6, 300.0, 200.0),
            "pl_beta_hog" => shear_lag_beta_hogging(0.4),
            "pl_beta_hog_hi" => shear_lag_beta_hogging(1.2),
            "pl_patch_a" => patch(PatchType::Through),
            "pl_patch_b" => patch(PatchType::OneFlangeNearEnd),
            "pl_patch_c" => patch(PatchType::EndFlange),
            "pl_k_tau" => k_tau_stiffened(2000.0, 1000.0, 8.0, 2.0e6),
            "pl_k_tau_low" => k_tau_stiffened(500.0, 1000.0, 8.0, 2.0e6),
            "pl_k_tau_floor" => k_tau_stiffened(2000.0, 1000.0, 8.0, 1.0e3),
            "pl_mv" => mv_interaction(0.8 * 400.0, 400.0, 100.0, 0.8, 1.0),
            "pl_ist" => i_st_min(2000.0, 1000.0, 8.0),
            "pl_alpha_ult" => alpha_ult_k(100.0, 50.0, 30.0, 355.0),
            "sh_omega" => omega(3000.0, 1000.0, 5.0),
            "sh_sigma_cr" => shell.sigma_cr,
            "sh_lambda" => shell.lambda,
            "sh_alpha" => alpha_x(1000.0, 5.0, 25.0),
            "sh_chi" => shell.chi,
            "sh_sigma_rd" => shell.sigma_rd,
            "sh_theta_cr" => hoop.sigma_cr,
            "sh_theta_chi" => hoop.chi,
            "sh_interaction" => interaction_ls3(0.5, 1.0, shell.chi, 0.4, 1.0, hoop.chi, 0.2, 1.0, 0.7),
            "sh_omega_short_cx" => c_x(1.0, 1000.0, 5.0, 6.0),
            "tf_t_ed" => cold_forming_t_ed()?,
            "tf_tmax" => tmax(-5.0)?,
            "tf_tmax_s355j2_m25" => tmax(-25.0)?,
            "tf_tmax_s355k2_m15" => max_thickness("S355", "K2", 0.75, -15.0).ok_or("no S355 K2 thickness limit at -15 °C")?,
            "tf_tmax_s355k2_r50_m10_fallback" => max_thickness("S355", "K2", 0.5, -10.0).ok_or("no S355 K2 fallback limit at -10 °C")?,
            "tf_cells_total" => cell_count(false),
            "tf_cells_unset" => cell_count(true),
            "tf_blend_r60" => select(0.6, &|level| Ok([80.0, 100.0, 120.0][level])).map_err(|g| format!("{g:?}"))?.t_max,
            "tf_blend_below_r25_conservative" => select(0.1, &|level| Ok([80.0, 100.0, 120.0][level])).map_err(|g| format!("{g:?}"))?.t_max,
            "tc_ernst" => ernst_modulus(210000.0, 7.85e-5, 100.0e3, 400.0),
            "tc_sag" => cable_sag(0.5, 50000.0, 250.0e3),
            "tc_pin_m" => pin_moment_ed(120.0e3, 10.0, 30.0, 5.0),
            "tc_pin_comb" => pin_combined(40.0e6, 80.0e6, 60.0e3, 100.0e3),
            "tc_rope" => rope_min_breaking(0.6, 20.0, 1770.0),
            "jc_lb_star" => t_stub(Mode1Method::One, 100.0).l_b_star,
            "jc_f1_m1" => t_stub(Mode1Method::One, 100.0).ft_1,
            "jc_f2" => t_stub(Mode1Method::One, 100.0).ft_2,
            "jc_ft_rd_m1" => t_stub(Mode1Method::One, 100.0).ft_rd,
            "jc_f1_m2" => t_stub(Mode1Method::Two, 100.0).ft_1,
            "jc_ft_noprying" => t_stub(Mode1Method::One, 300.0).ft_rd,
            "jc_z_eq" => z_eq,
            "jc_k_eq" => k_eq,
            "jc_omega" => omega_table_6_3(0.75, 200.0, 9.0, 3000.0),
            "jc_omega_15" => omega_table_6_3(1.5, 200.0, 9.0, 3000.0),
            "jc_mj" => m_j_rd_rows(&[300.0, 220.0, 140.0], &row_forces()),
            "jc_mu" => mu_stiffness(0.9e6, 1.0e6, 2.7),
            "jc_sj_ini" => s_j_ini_joint(210000.0, 300.0, 4000.0, 9000.0, 6000.0),
            "jc_bp_a" => base_plate_m_rd(100.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 300.0, 300.0).ok_or("base plate case a has no solution")?,
            "jc_bp_b" => base_plate_m_rd(-200.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 300.0, 300.0).ok_or("base plate case b has no solution")?,
            "jc_bp_c" => base_plate_m_rd(100.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 200.0, 400.0).ok_or("base plate case c has no solution")?,
            "jc_bp_d" => base_plate_m_rd(-300.0e3, 150.0e3, 400.0e3, 150.0e3, 400.0e3, 200.0, 400.0).ok_or("base plate case d has no solution")?,
            "jc_cs_leff_free" => c_strip(450.0).l_eff,
            "jc_cs_fc_free" => c_strip(450.0).fc_rd,
            "jc_cs_leff_plate" => c_strip(300.0).l_eff,
            "jc_cs_fc_plate" => c_strip(300.0).fc_rd,
            "jc_cr_free" => base_plate_c_required(17.0, 240.0, 450.0, 350.0e3 / 40.0),
            "jc_cr_plate" => base_plate_c_required(17.0, 240.0, 250.0, 350.0e3 / 40.0),
            "jc_fjd" => f_jd_bearing(2.0 / 3.0, 300.0 * 300.0, 300.0 * 300.0 * 1.5, 20.0, 150.0, 250.0),
            "br_l2" => lambda_2(0.5, 0.5, 1.2e6, 0.5e6),
            "br_l3" => lambda_3(80.0),
            "br_l4" => lambda_4(&[(0.4, 0.8), (0.2, 0.6)], 1.0, 1.0),
            "br_l1" => lambda_1_interpolated(&[(2.0, 2.5), (10.0, 2.0), (50.0, 1.2)], 30.0),
            "br_ds" => delta_sigma_e2_bridge(1.1, 1.0, 0.9, 1.05, 2.0, 1.0, 40.0),
            "br_cat" => category_at_or_below(103.0).ok_or("no detail category at or below 103")?,
            "br_ks" => size_effect(40.0),
            "tw_vcrit" => v_crit(2000.0, 1.2, 0.2),
            "tw_sc" => scruton(0.05, 80.0, 1.25, 2.0),
            "tw_freq" => cantilever_frequency(30000.0, 2.1e5 * 5.0e9, 100.0),
            "tw_cycles" => vortex_cycles(50.0, 1.0e6, 5.0, 8.0),
            "tw_ovalling" => ovalling_frequency(3.0, 500.0, 6.0, 210000.0, 7.85e-9),
            "ct_ph" => janssen_ph(gamma, kk, mu, area, circumference, depth),
            "ct_nx" => janssen_nx(gamma, kk, mu, area, circumference, depth),
            "ct_pv" => janssen_pv(gamma, kk, mu, area, circumference, depth),
            "ct_tank_t" => tank_course_thickness(1000.0, 15000.0, 3000.0, 10000.0, 235.0, 1.0),
            "ct_pipe_t" => pipe_wall_thickness(8.0, 508.0, 355.0, 1.1),
            "ct_pipe_ax" => restrained_axial_stress(0.3, hoop_membrane(8.0, 500.0, 10.0), 210000.0, 1.2e-5, 40.0),
            "ct_pcr" => pipe_p_cr(210000.0, 0.3, 10.0, 500.0),
            "pi_loss_30" => corrosion_loss(Environment::UndisturbedSoil, 30.0),
            "pi_loss_3" => corrosion_loss(Environment::PollutedSoil, 3.0),
            "pi_fy_red" => fy_reduced_by_shear(0.8, 1.0, 355.0),
            "pi_vpl" => v_pl_rd_pile(3000.0, 355.0, 1.0),
            "cr_leff" => le,
            "cr_soz" => soz,
            "cr_tau" => tau_oxz(soz),
            "cr_vm" => web_von_mises(120.0, soz, tau_oxz(soz), 355.0, 1.0),
            "cr_hertz" => hertz_line_contact(180.0e3, 210000.0, 50.0, 300.0),
            "cr_lambda" => lambda_crane(0.5, 0.25, 3.0),
            "cr_phi2" => phi_2(1.1, 0.3, 0.5),
            "sec_ang_a" => angle(100.0, 65.0, 8.0).area,
            "sec_ang_iu" => angle(100.0, 65.0, 8.0).iu,
            "sec_ang_iv" => angle(100.0, 65.0, 8.0).iv,
            "sec_ang_alpha" => angle(100.0, 65.0, 8.0).alpha.to_degrees().rem_euclid(180.0),
            "sec_ang_wel_u" => angle(100.0, 65.0, 8.0).w_el_u,
            "sec_ang_wel_v" => angle(100.0, 65.0, 8.0).w_el_v,
            "sec_ang_wpl_u" => angle(100.0, 65.0, 8.0).w_pl_u,
            "sec_ang_wpl_v" => angle(100.0, 65.0, 8.0).w_pl_v,
            "sec_ang_it" => angle(100.0, 65.0, 8.0).it,
            "sec_chs_a" => chs(168.3, 6.3).area,
            "sec_chs_i" => chs(168.3, 6.3).iy,
            "sec_chs_wel" => chs(168.3, 6.3).w_el_y,
            "sec_chs_wpl" => chs(168.3, 6.3).w_pl_y,
            "sec_chs_it" => chs_torsion(168.3, 6.3),
            "sec_ang_class" => f64::from(angle_class(100.0, 65.0, 8.0, (235.0_f64 / 355.0).sqrt())),
            "sec_chs_class" => f64::from(chs_class(168.3 / 6.3, (235.0_f64 / 355.0).sqrt())),
            other => return Err(format!("EN 1993 special parts have no quantity {other}")),
        })
    }

    pub fn utilisation(check: &str) -> Result<f64, String> {
        let report = check_full_steel_structure(&En1993Snapshot::special_parts_showcase());
        report.checks.iter().find(|candidate| candidate.id == check).map(|candidate| candidate.utilization).ok_or_else(|| format!("the report holds no check {check}"))
    }

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.vector = Box::new(|_, _vector, name| quantity(name));
        bound.utilisation = Box::new(|_, _subject, _annex, check, _quantity| utilisation(check));
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
