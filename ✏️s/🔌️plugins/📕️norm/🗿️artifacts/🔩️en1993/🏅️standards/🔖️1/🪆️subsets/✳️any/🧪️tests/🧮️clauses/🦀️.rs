//! 🦀️ EN 1993 clause vectors — the Rust SUBJECT adapter. Each worked-example quantity E01 to E13 is evaluated by the clause functions of `🧮️clauses` and compared by the feature's relative tolerance with the independent Python oracle running in the oracle role. Step parsing, tolerances and plumbing are the shared norm step layer (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🥒️steps`).

use semio_repo_test_host::Adapter;

#[allow(dead_code)]
#[path = "../../../../../../../../🔮️oracles/🥒️steps/🦀️.rs"]
mod steps;

#[cfg(feature = "sut")]
mod subject {
    use super::steps::Binding;
    use semio_s_artifact_norm_en1993_checks::clauses::{fatigue_fire, global, joints, mcr_fe, resistance, stability};

    const NCR_PROPS: (f64, f64, f64, f64) = (603.8e4, 20.12e4, 125900.0e6, 5000.0);

    fn flexural() -> (f64, f64) {
        let ncr = stability::n_cr(stability::e_steel() * 3923.0e4, 4000.0);
        let lambda = stability::lambda_bar_from_ncr(10600.0 * 355.0, ncr);
        (stability::chi_flexural(lambda, stability::Curve::C, 1.8e6, ncr), ncr)
    }

    fn mcr_closed() -> f64 {
        let (iz, it, iw, l) = NCR_PROPS;
        stability::mcr_closed_form(stability::e_steel(), stability::g_steel(), iz, it, iw, l, 1.0, 1.0, 1.127, 0.0, 0.0)
    }

    fn lambda_lt() -> f64 {
        stability::lambda_lt_from_mcr(628.4e3, 235.0, mcr_closed())
    }

    fn mcr_fe_lambda(parabolic: bool) -> Result<f64, String> {
        let (iz, it, iw, l) = NCR_PROPS;
        let props = mcr_fe::BeamProps { l, e: stability::e_steel(), g: stability::g_steel(), iz, it, iw };
        let restraints = mcr_fe::Restraints { supports: vec![mcr_fe::Support::fork(0.0), mcr_fe::Support::fork(l)], torsional_spring_per_length: 0.0, lateral_springs: vec![] };
        let uniform = |_x: f64| 1.0;
        let none = |_x: f64| 0.0;
        let parabola = move |x: f64| x * (l - x) / 2.0;
        let load = if parabolic { mcr_fe::Load { moment: &parabola, line_load: &uniform, zg: 0.0 } } else { mcr_fe::Load { moment: &uniform, line_load: &none, zg: 0.0 } };
        let result = mcr_fe::mcr_fe(&props, &load, &restraints, 40).ok_or("the Mcr finite element solve returned no eigenvalue")?;
        Ok(if parabolic { result.lambda * l * l / 8.0 } else { result.lambda })
    }

    fn fire_temperature(seconds: f64) -> Result<f64, String> {
        let iso = |t: f64| fatigue_fire::gas_iso834(t);
        let history = fatigue_fire::unprotected_steel_temperature(&iso, fatigue_fire::k_sh_i_section(167.26 / 215.58), 215.58, 900.0, 5.0, 0.7, 1.0);
        history.iter().find(|(t, _)| (*t - seconds).abs() < 1e-9).map(|point| point.1).ok_or_else(|| format!("no fire sample at {seconds} s"))
    }

    pub fn quantity(name: &str) -> Result<f64, String> {
        Ok(match name {
            "chi_z" => flexural().0,
            "Nb_Rd_z_na_de" => flexural().0 * 10600.0 * 355.0 / 1.1,
            "Mcr_closed_form" => mcr_closed(),
            "chi_LT_6.3.2.2" => stability::chi_lt_general(lambda_lt(), 0.34, 70e6, mcr_closed()),
            "chi_LT_mod_6.3.2.3" => stability::chi_lt_mod(stability::chi_lt_rolled(lambda_lt(), 0.34, 0.4, 0.75, 70e6, mcr_closed()), stability::ltb_f(0.94, lambda_lt())),
            "Mcr_fe_uniform" => mcr_fe_lambda(false)?,
            "Mcr_fe_udl" => mcr_fe_lambda(true)?,
            "b_eff" => semio_s_artifact_norm_en1993_checks::clauses::effective_width::effective_internal(1000.0, 8.0, 355.0, -1.0).b_eff,
            "V_bw_Rd" => {
                let epsilon = semio_s_artifact_norm_en1993_checks::clauses::classification::epsilon(355.0);
                let lambda_w = resistance::lambda_w(1000.0, 8.0, epsilon, resistance::k_tau(2.0));
                resistance::v_bw_rd(resistance::chi_w(lambda_w, 1.2, true), 355.0, 1000.0, 8.0, 1.0) / 1000.0
            }
            "F_b_Rd" => joints::fb_rd(joints::k1_bearing(true, 35.0, 100.0, 22.0), joints::alpha_b(true, 40.0, 100.0, 22.0, 800.0, 490.0), 490.0, 20.0, 12.0, 1.25, 1.0),
            "F_T2_Rd" => semio_s_artifact_norm_en1993_checks::clauses::joint_components::t_stub_modes(210.0, 210.0, 15.0, 355.0, 40.0, 40.0, 0.0, 282240.0, 245.0, 2.0, 60.0, semio_s_artifact_norm_en1993_checks::clauses::joint_components::Mode1Method::Two, 1.0).ft_2,
            "damage" => fatigue_fire::miner_damage(&[(60.0, 1.0e6)], 71.0, 1.15, 1.0),
            "rainflow_cycles" => fatigue_fire::rainflow(&[0.0, 10.0, 2.0, 8.0, 0.0, 9.0, 1.0, 6.0, 0.0]).iter().map(|cycle| cycle.1).sum::<f64>(),
            "rainflow_cycles_detail" => fatigue_fire::rainflow(&[0.0, 80.0, 10.0, 60.0, 20.0, 70.0, 0.0, 90.0, 30.0, 50.0, 0.0]).iter().map(|cycle| cycle.1).sum::<f64>(),
            "theta_a_300s" => fire_temperature(300.0)?,
            "theta_a_600s" => fire_temperature(600.0)?,
            "theta_a_900s" => fire_temperature(900.0)?,
            "phi_sway" => global::sway_phi(6.0, 2.0),
            other => return Err(format!("EN 1993 clause vectors have no quantity {other}")),
        })
    }

    pub fn binding() -> Binding {
        let mut bound = Binding::none();
        bound.vector = Box::new(|_, _vector, name| quantity(name));
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
