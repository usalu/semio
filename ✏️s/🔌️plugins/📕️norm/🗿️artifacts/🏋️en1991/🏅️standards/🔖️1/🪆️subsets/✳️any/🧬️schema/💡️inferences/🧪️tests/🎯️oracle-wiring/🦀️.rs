//! 🎯️ Typed wiring of the blessed EN 1991 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use semio_s_artifact_norm_en1991_checks::{part_1_2, part_1_3, part_1_4, part_1_5, part_1_7, part_2, part_4};
use semio_s_artifact_norm_en1991_checks::part_1_4::TerrainCategory;
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_compliance::AnnexChoice;

extern crate semio_s_artifact_norm_en1991_inferences_rail_silo as rail_silo_crate;
use rail_silo_crate::formulas as rail_silo;

/// 🏞️ The terrain category whose roughness length z₀ equals the oracle input; the oracle carries z₀ and z_min, our evaluator the category.
fn terrain_of(roughness_length_m: f64) -> TerrainCategory {
    [TerrainCategory::Zero, TerrainCategory::I, TerrainCategory::II, TerrainCategory::III, TerrainCategory::IV]
        .into_iter()
        .find(|terrain| (terrain.z_0_m() - roughness_length_m).abs() < 1e-12)
        .expect("a terrain category for the oracle roughness length")
}

/// 🫙 Janssen silo pressures p_hf, p_vf, p_wf, p_ho and z_o of EN 1991-4 Eq. (5.1)-(5.3).
fn janssen_pressure(inputs: &Inputs) -> Outputs {
    let (gamma, a, mu, k, depth) = (inputs.num("gamma"), inputs.num("a_m"), inputs.num("mu"), inputs.num("K"), inputs.num("depth"));
    Outputs::default()
        .with("p_hf", part_4::janssen_horizontal_pa(gamma, a, mu, k, depth))
        .with("p_vf", part_4::janssen_vertical_pa(gamma, a, mu, k, depth))
        .with("p_wf", part_4::wall_friction_pa(gamma, a, mu, k, depth))
        .with("p_ho", part_4::janssen_p_ho_pa(gamma, a, mu))
        .with("z_o", part_4::janssen_z_o_m(a, mu, k))
}

/// 🌉 LM1 tandem design moment and maximum support reaction of a simply supported span.
fn lm1_tandem(inputs: &Inputs) -> Outputs {
    Outputs::default()
        .with("M_max", part_2::tandem_moment_nm(inputs.num("Q"), inputs.num("span")))
        .with("R_max", part_2::tandem_reaction_n(inputs.num("Q"), inputs.num("span")))
}

/// 🚆 LM71 mid-span moment of a simply supported span: the axle group centred at mid-span plus the distributed load outside the group.
fn rail_lm71_mid_moment(inputs: &Inputs) -> Outputs {
    Outputs::default().with("M_mid", part_2::rail_lm71_mid_moment_nm(inputs.num("Q"), inputs.num("q"), inputs.num("n") as u32, inputs.num("s"), inputs.num("lead"), inputs.num("span")))
}

/// 🌬️ Wind turbulence intensity, roughness factor and peak velocity pressure of EN 1991-1-4 4.3-4.5; the mean velocity v_m through `part_1_4::mean_wind_velocity`.
fn wind_peak_pressure(inputs: &Inputs) -> Outputs {
    let (terrain, z, c_o) = (terrain_of(inputs.num("z0")), inputs.num("z"), inputs.num("c_o"));
    Outputs::default()
        .with("I_v", part_1_4::turbulence_intensity(z, terrain, c_o))
        .with("c_r", part_1_4::c_r(z, terrain))
        .with("v_m", part_1_4::mean_wind_velocity(z, terrain, inputs.num("v_b"), c_o))
        .with("q_p", part_1_4::en_peak_pressure_pa(z, terrain, inputs.num("v_b"), inputs.num("rho"), c_o))
}

/// 🧱 External pressure coefficients c_pe,10, c_pe,1 and the area-interpolated c_pe of the wall zones of EN 1991-1-4 Table 7.1.
fn wind_cpe(inputs: &Inputs) -> Outputs {
    let (c_pe_10, c_pe_1) = part_1_4::wall_cpe(inputs.text("zone"), inputs.num("h_d")).expect("a wall zone A-E");
    Outputs::default()
        .with("c_pe_10", c_pe_10)
        .with("c_pe_1", c_pe_1)
        .with("c_pe", part_1_4::area_cpe(c_pe_1, c_pe_10, inputs.num("area")))
}

/// ❄️ Mono-pitch snow shape coefficients μ₁, μ₂ and roof snow load s of EN 1991-1-3 5.3.
fn snow_mono_pitch(inputs: &Inputs) -> Outputs {
    let (alpha, s_k) = (inputs.num("alpha"), inputs.num("s_k"));
    let mu_1 = part_1_3::mu_1(alpha);
    let mu_2 = part_1_3::shape_mu("multi-span", alpha, false, 0.0, 0.0, true, s_k, AnnexChoice::En);
    Outputs::default().with("mu_1", mu_1).with("mu_2", mu_2).with("s", part_1_3::roof_snow_pa(s_k, mu_1, inputs.num("C_e"), inputs.num("C_t")))
}

/// 🧱 Snow drift behind an obstruction: shape coefficient μ_w, drift length l_s and peak load s of EN 1991-1-3 Annex B.
fn snow_drift(inputs: &Inputs) -> Outputs {
    let (h, s_k) = (inputs.num("h"), inputs.num("s_k"));
    let (_, mu_w, l_s) = part_1_3::obstruction_drift(h, s_k, AnnexChoice::En);
    Outputs::default().with("mu_w", mu_w).with("l_s", l_s).with("s_peak", part_1_3::roof_snow_pa(s_k, mu_w, 1.0, 1.0))
}

/// 🔥 Annex A parametric fire of a unit compartment (A_f = A_t = 1 m², h_eq = 1 m, so O = A_v) in °C values: Γ, t_max, θ_max and θ_g(t).
fn parametric_fire(inputs: &Inputs) -> Outputs {
    let fire = part_1_2::parametric_fire(1.0, 1.0, inputs.num("O"), 1.0, inputs.num("b"), inputs.num("q_td") / 1.0e6, inputs.num("t_lim") / 3600.0);
    Outputs::default()
        .with("Gamma", fire.gamma_heating)
        .with("t_max", fire.t_max_h * 3600.0)
        .with("theta_max", fire.theta_max_c)
        .with("theta_g", part_1_2::parametric_theta_c(&fire, inputs.num("t") / 3600.0))
}

/// 🔥 ISO 834 standard fire curve θ_g(t) in °C values.
fn iso834(inputs: &Inputs) -> Outputs {
    Outputs::default().with("theta_g", part_1_2::standard_gas_temp_c(inputs.num("t_min")))
}

/// 🌉 Bridge deck wind force per metre of EN 1991-1-4 8.3: d_tot, c_f,x0 and the horizontal, vertical and longitudinal forces.
fn bridge_wind_force(inputs: &Inputs) -> Outputs {
    let d_tot = part_1_4::bridge_depth_m(inputs.num("d"), inputs.num("parapet_kind") as u8, inputs.num("d1"), inputs.num("traffic_kind") as u8).expect("admissible parapet and traffic kinds");
    let c_fx0 = part_1_4::bridge_c_fx0(inputs.num("b"), d_tot);
    let f_x = part_1_4::bridge_force_x_n_m(inputs.num("q_p"), c_fx0, d_tot);
    Outputs::default().with("d_tot", d_tot).with("c_fx0", c_fx0).with("F_x", f_x).with("F_z", part_1_4::bridge_force_z_n_m(inputs.num("q_p"), inputs.num("b"))).with("F_y", part_1_4::bridge_force_y_n_m(f_x))
}

/// 🌀 Structural factor c_s c_d of EN 1991-1-4 Annex B with its intermediate B², S_L, R², ν and k_p.
fn structural_factor(inputs: &Inputs) -> Outputs {
    let r = part_1_4::structural_factor(inputs.num("h"), inputs.num("b"), terrain_of(inputs.num("z0")), inputs.num("v_b"), inputs.num("delta"), inputs.num("n1"));
    Outputs::default().with("z_s", r[0]).with("L", r[1]).with("B2", r[2]).with("S_L", r[4]).with("R2", r[5]).with("nu", r[6]).with("k_p", r[7]).with("c_s_c_d", r[8])
}

/// 🌪️ Vortex shedding critical velocity, Scruton number, onset velocity and Reynolds number of EN 1991-1-4 Annex E.
fn vortex_shedding(inputs: &Inputs) -> Outputs {
    let v_crit = part_1_4::vortex_critical_velocity(inputs.num("b"), inputs.num("n1"), inputs.num("St"));
    Outputs::default().with("v_crit", v_crit).with("Sc", part_1_4::scruton(inputs.num("delta_s"), inputs.num("m_e"), inputs.num("rho"), inputs.num("b"))).with("v_onset", part_1_4::vortex_onset_velocity(inputs.num("v_m"))).with("Re", part_1_4::reynolds(inputs.num("b"), v_crit))
}

/// 💥 Equivalent static impact force F = v √(k m) and the kinetic energy of EN 1991-1-7 Annex C.
fn impact_force(inputs: &Inputs) -> Outputs {
    Outputs::default().with("F", part_1_7::dynamic_impact_force_n(inputs.num("m"), inputs.num("v"), inputs.num("k"))).with("E", part_1_7::impact_energy_j(inputs.num("m"), inputs.num("v")))
}

/// 🏠 Internal explosion design pressures D.1, D.2 and the governing value of EN 1991-1-7 Annex D in kN/m².
fn gas_explosion(inputs: &Inputs) -> Outputs {
    let ratio = inputs.num("a_v") / inputs.num("V");
    Outputs::default().with("p_d1", part_1_7::gas_explosion_pd_static_kn_m2(inputs.num("p_v"))).with("p_d2", part_1_7::gas_explosion_pd_kn_m2(inputs.num("p_v"), ratio)).with("p_d", part_1_7::gas_explosion_governing_kn_m2(inputs.num("p_v"), ratio)).with("ratio", ratio)
}

/// 🌡️ Bridge temperature actions of EN 1991-1-5 6.1: ranges, curvature, end rotation, movement and combination; the deck type follows from the shade shifts of the case.
fn bridge_temperature(inputs: &Inputs) -> Outputs {
    let class = (1..=3u8).find(|c| part_1_5::bridge_shift(*c).is_some_and(|(lo, hi)| (lo - inputs.num("shift_min")).abs() < 1e-12 && (hi - inputs.num("shift_max")).abs() < 1e-12)).expect("a deck type for the shade shifts");
    let (con, exp) = part_1_5::bridge_delta_t_n(class, inputs.num("t_min"), inputs.num("t_max"), inputs.num("t_0")).expect("known deck type");
    let (heat, cool) = part_1_5::bridge_delta_t_m(class, if class == 1 { 2 } else { 0 }).expect("known deck type and surfacing");
    let m = heat.max(cool);
    Outputs::default()
        .with("dt_con", con)
        .with("dt_exp", exp)
        .with("kappa", part_1_5::curvature_per_m(inputs.num("alpha"), m, inputs.num("h")))
        .with("theta", part_1_5::end_rotation_rad(inputs.num("alpha"), m, inputs.num("L"), inputs.num("h")))
        .with("movement", part_1_5::expansion_m(inputs.num("alpha"), con + exp, inputs.num("L")))
        .with("combined", part_1_5::combined_delta_t(con.max(exp), m))
}

/// 🚛 LM3 special vehicle of the designation total/axle in kN: weights, axle lines and length.
fn special_vehicle(inputs: &Inputs) -> Outputs {
    let (total, axle, lines) = part_2::special_vehicle(inputs.text("designation")).expect("an admissible designation");
    Outputs::default().with("total", total).with("axle", axle).with("lines", lines as f64).with("length", part_2::special_vehicle_length_m(lines)).with("sum", lines as f64 * axle)
}

/// 🪜 Snow drift at a roof step: μ_w, μ_s and the drift length of EN 1991-1-3 5.3.6.
fn roof_step_drift(inputs: &Inputs) -> Outputs {
    let (h, s_k) = (inputs.num("h"), inputs.num("s_k"));
    Outputs::default().with("mu_w", part_1_3::drift_mu_w(h, inputs.num("b1"), inputs.num("b2"), s_k, AnnexChoice::En)).with("mu_s", part_1_3::sliding_mu_s(inputs.num("pitch_upper"))).with("l_s", part_1_3::obstruction_drift(h, s_k, AnnexChoice::En).2)
}

/// 🔀 Snow drift with γh/s_k capping μ_w below 0.8 (reading 1 of the NDP `drift-mu-w-reading`).
fn drift_gamma_limited(inputs: &Inputs) -> Outputs {
    let (h, s_k) = (inputs.num("h"), inputs.num("s_k"));
    let mu_w = part_1_3::drift_mu_w_reading(h, inputs.num("b1"), inputs.num("b2"), s_k, 4.0, 1.0);
    Outputs::default().with("mu_w", mu_w).with("l_s", part_1_3::obstruction_drift(h, s_k, AnnexChoice::En).2).with("s_peak", part_1_3::roof_snow_pa(s_k, mu_w, 1.0, 1.0))
}

/// 🌉 Tandem travel over a simple span: envelope of the mid-span moment and the left reaction (EN 1991-2, 4.3.2).
fn tandem_travel_envelope(inputs: &Inputs) -> Outputs {
    let (moment, reaction) = part_2::tandem_envelope(inputs.num("axle"), inputs.num("span"), inputs.num("spacing"));
    Outputs::default().with("m_env", moment).with("r_env", reaction)
}

/// 🌈️ Envelope of the LM1 tandem at one section of a simply supported span: largest bending moment and largest and smallest shear force.
fn tandem_section_envelope(inputs: &Inputs) -> Outputs {
    let (moment, shear_max, shear_min) = part_2::tandem_section_envelope(inputs.num("Q"), inputs.num("span"), inputs.num("s"), inputs.num("x"));
    Outputs::default().with("M_env", moment).with("V_max", shear_max).with("V_min", shear_min)
}

/// 🛑 Braking and acceleration force Q_lk of lane 1 (EN 1991-2, 4.4.1) with its bounds.
fn braking_force(inputs: &Inputs) -> Outputs {
    Outputs::default().with("Q_lk", part_2::braking_force_kn(AnnexChoice::De, inputs.num("L"), inputs.num("w1")) * 1000.0)
}

/// 🌀 Centrifugal force Q_tk of Table 4.3 (EN 1991-2, 4.4.2).
fn centrifugal_force(inputs: &Inputs) -> Outputs {
    Outputs::default().with("Q_tk", part_2::centrifugal_force_kn(AnnexChoice::De, inputs.num("Q_v") / 1000.0, inputs.num("r")) * 1000.0)
}

/// ❄️ Shape coefficient along a drift triangle of a roof section (EN 1991-1-3, 5.3.6).
fn snow_profile_mu(inputs: &Inputs) -> Outputs {
    Outputs::default().with("mu", part_1_3::drift_mu_at(inputs.num("base"), inputs.num("start"), inputs.num("length"), inputs.num("peak"), inputs.num("rising") > 0.5, inputs.num("x")))
}

/// 🌬️ Net wind pressure of a zone (EN 1991-1-4, 7.2.9).
fn wind_net_pressure(inputs: &Inputs) -> Outputs {
    Outputs::default().with("w", part_1_4::wind_pressure_pa(inputs.num("qp"), inputs.num("cpe"), inputs.num("cpi"), inputs.num("factor"), 1.0))
}

/// 🚶 Footbridge uniform load q_fk and horizontal force Q_flk (EN 1991-2, 5.3.2.1 and 5.4).
fn footbridge_loads(inputs: &Inputs) -> Outputs {
    let (length, width) = (inputs.num("L"), inputs.num("b"));
    Outputs::default().with("q_fk", part_2::footbridge_q_fk_pa(length)).with("Q_flk", part_2::footbridge_horizontal_n(length, width))
}

/// 🚆 Dynamic factors Phi_2 and Phi_3 of the determinant length (EN 1991-2, 6.4.5.2).
fn rail_dynamic(inputs: &Inputs) -> Outputs {
    let (phi_2, phi_3) = part_2::rail_dynamic_factors(inputs.num("L_phi"));
    Outputs::default().with("Phi_2", phi_2).with("Phi_3", phi_3)
}

/// 🌾 Mean lateral pressure ratio K_m of a bulk solid (EN 1991-4, Annex E).
fn bulk_k_mean(inputs: &Inputs) -> Outputs {
    Outputs::default().with("K_m", part_4::k_mean(inputs.num("phi_i_deg").to_radians()))
}

/// ⚙️ Equivalent static force of a rotating unbalance (EN 1991-3, 5).
fn machine_force(inputs: &Inputs) -> Outputs {
    Outputs::default().with("F_dyn", semio_s_artifact_norm_en1991_checks::part_3::machine_dynamic_force_n(inputs.num("m"), inputs.num("e"), inputs.num("f")))
}

/// 📐 Dispersed wheel footprint area (EN 1991-2, 4.3.6).
fn dispersal_footprint(inputs: &Inputs) -> Outputs {
    let (t_p, t_s) = (inputs.num("t_p"), inputs.num("t_s"));
    Outputs::default().with("A_foot", part_2::dispersed_side_m(inputs.num("a"), t_p, t_s) * part_2::dispersed_side_m(inputs.num("b"), t_p, t_s))
}

/// 🔁 Road fatigue load models of a lane (EN 1991-2, 4.6): FLM1 axle load and distributed load, FLM3 axle load and vehicle length.
fn fatigue_load_models(inputs: &Inputs) -> Outputs {
    let lane = inputs.num("lane") as u8;
    Outputs::default()
        .with("axle_flm1", part_2::fatigue_flm1_axle_n(AnnexChoice::De, lane))
        .with("udl_flm1", part_2::fatigue_flm1_udl_pa(AnnexChoice::De, lane))
        .with("axle_flm3", part_2::fatigue_flm3_axle_n())
        .with("length_flm3", part_2::fatigue_flm3_length_m())
}

/// 🚛 Collision forces on a support at deck level or on the soffit (EN 1991-2, 4.7.3.1); the soffit has no transverse force.
fn support_collision(inputs: &Inputs) -> Outputs {
    let soffit = inputs.num("soffit") > 0.5;
    let forces = Outputs::default().with("F_dx", part_2::collision_force_x_n(soffit));
    if soffit { forces } else { forces.with("F_dy", part_2::collision_force_y_n()) }
}

/// 🔥 Localised fire (EN 1991-1-2, Annex C): flame length, horizontal flame length, virtual source height and Hasemi heat flux.
fn local_fire_hasemi(inputs: &Inputs) -> Outputs {
    let (diameter, power, ceiling, distance) = (inputs.num("D"), inputs.num("Q"), inputs.num("H"), inputs.num("r"));
    let z_prime = part_1_2::local_fire_virtual_source_m(power, diameter);
    let l_h = part_1_2::local_fire_horizontal_flame_m(power, ceiling);
    Outputs::default()
        .with("L_f", part_1_2::local_fire(diameter, power, 1.0).0)
        .with("L_h", l_h)
        .with("z_prime", z_prime)
        .with("flux", part_1_2::hasemi_flux_w_m2(part_1_2::hasemi_y(distance, ceiling, z_prime, l_h)))
}

/// 🛡️ Snow guard line force F_s = s b sin α (EN 1991-1-3, 6.3).
fn snow_guard(inputs: &Inputs) -> Outputs {
    Outputs::default().with("F_s", part_1_3::snow_guard_force_n_m(inputs.num("s"), inputs.num("b"), inputs.num("alpha_deg")))
}

/// 🪝 Dynamic factor of the sudden release of a payload fraction φ₃ = 1 − (Δm/m)(1 + β₃) (EN 1991-3, 2.5).
fn hoist_release(inputs: &Inputs) -> Outputs {
    Outputs::default().with("phi_3", semio_s_artifact_norm_en1991_checks::part_3::phi_3(inputs.num("dm_over_m"), inputs.num("beta_3")))
}

/// 🚆 Derailment line loads of design situations I and II, the loaded length and the offset limit (EN 1991-2, 6.7.1).
fn rail_derailment_load(inputs: &Inputs) -> Outputs {
    let alpha = inputs.num("alpha");
    Outputs::default()
        .with("q_i", rail_silo::derail_line_load_n_m("i", alpha).expect("situation i"))
        .with("q_ii", rail_silo::derail_line_load_n_m("ii", alpha).expect("situation ii"))
        .with("length", rail_silo::derail_length_m())
        .with("offset_limit", rail_silo::derail_offset_limit_m())
}

/// 🛤️ Characteristic bases of the Table 6.11 groups: the governing longitudinal force of the span and the nosing force.
fn rail_group_bases(inputs: &Inputs) -> Outputs {
    Outputs::default().with("F_long", rail_silo::rail_longitudinal_n(inputs.num("span"))).with("F_nosing", rail_silo::rail_nosing_n(inputs.num("alpha")))
}

/// 💥 Vent area of a dust explosion after Bartknecht (EN 14491); the oracle carries the pressures in bar.
fn dust_vent_area(inputs: &Inputs) -> Outputs {
    Outputs::default().with("A_v", rail_silo::dust_vent_area_m2(inputs.num("K_St"), inputs.num("p_max_bar") * 1e5, inputs.num("V"), inputs.num("p_stat_bar") * 1e5, inputs.num("p_red_bar") * 1e5))
}

/// 🗂️ One adapter per oracle check of EN 1991.
static ADAPTERS: [Adapter; 37] = [
    Adapter { check: "en1991.4.janssen-pressure", evaluator: "semio_s_artifact_norm_en1991_checks::part_4::{janssen_horizontal_pa, janssen_vertical_pa, wall_friction_pa, janssen_p_ho_pa, janssen_z_o_m}", run: janssen_pressure },
    Adapter { check: "mechanics.lm1-tandem", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::{tandem_moment_nm, tandem_reaction_n}", run: lm1_tandem },
    Adapter { check: "mechanics.rail-lm71-mid-moment", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::rail_lm71_mid_moment_nm", run: rail_lm71_mid_moment },
    Adapter { check: "en1991.4.wind-peak-pressure", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_4::{turbulence_intensity, c_r, en_peak_pressure_pa}", run: wind_peak_pressure },
    Adapter { check: "en1991.7.wind-cpe", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_4::{wall_cpe, area_cpe}", run: wind_cpe },
    Adapter { check: "en1991.snow.shape-mono-pitch", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_3::{mu_1, shape_mu, roof_snow_pa}", run: snow_mono_pitch },
    Adapter { check: "en1991.snow.drift", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_3::{obstruction_drift, roof_snow_pa}", run: snow_drift },
    Adapter { check: "en1991.fire.parametric-curve", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_2::{parametric_fire, parametric_theta_c}", run: parametric_fire },
    Adapter { check: "en1991.fire.iso834", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_2::standard_gas_temp_c", run: iso834 },
    Adapter { check: "en1991.8.bridge-wind-force", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_4::{bridge_depth_m, bridge_c_fx0, bridge_force_x_n_m, bridge_force_z_n_m, bridge_force_y_n_m}", run: bridge_wind_force },
    Adapter { check: "en1991.6.structural-factor", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_4::structural_factor", run: structural_factor },
    Adapter { check: "en1991.6.vortex-shedding", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_4::{vortex_critical_velocity, scruton, vortex_onset_velocity, reynolds}", run: vortex_shedding },
    Adapter { check: "en1991.7.impact-force", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_7::{dynamic_impact_force_n, impact_energy_j}", run: impact_force },
    Adapter { check: "en1991.7.gas-explosion", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_7::{gas_explosion_pd_static_kn_m2, gas_explosion_pd_kn_m2, gas_explosion_governing_kn_m2}", run: gas_explosion },
    Adapter { check: "en1991.5.bridge-temperature", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_5::{bridge_delta_t_n, bridge_delta_t_m, curvature_per_m, end_rotation_rad, expansion_m, combined_delta_t}", run: bridge_temperature },
    Adapter { check: "en1991.2.special-vehicle", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::{special_vehicle, special_vehicle_length_m}", run: special_vehicle },
    Adapter { check: "en1991.snow.drift-gamma-limited", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_3::{drift_mu_w_reading, obstruction_drift, roof_snow_pa}", run: drift_gamma_limited },
    Adapter { check: "en1991.2.tandem-travel-envelope", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::{tandem_envelope, tandem_effects}", run: tandem_travel_envelope },
    Adapter { check: "mechanics.tandem-section-envelope", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::tandem_section_envelope", run: tandem_section_envelope },
    Adapter { check: "en1991.2.braking", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::braking_force_kn", run: braking_force },
    Adapter { check: "en1991.2.centrifugal", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::centrifugal_force_kn", run: centrifugal_force },
    Adapter { check: "en1991.snow.profile-mu", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_3::drift_mu_at", run: snow_profile_mu },
    Adapter { check: "en1991.wind.net-pressure", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_4::wind_pressure_pa", run: wind_net_pressure },
    Adapter { check: "en1991.snow.roof-step-drift", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_3::{drift_mu_w, sliding_mu_s, obstruction_drift}", run: roof_step_drift },
    Adapter { check: "en1991.2.footbridge-load", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::{footbridge_q_fk_pa, footbridge_horizontal_n}", run: footbridge_loads },
    Adapter { check: "en1991.2.rail-dynamic", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::rail_dynamic_factors", run: rail_dynamic },
    Adapter { check: "en1991.4.bulk-k-mean", evaluator: "semio_s_artifact_norm_en1991_checks::part_4::k_mean", run: bulk_k_mean },
    Adapter { check: "en1991.3.machine-force", evaluator: "semio_s_artifact_norm_en1991_checks::part_3::machine_dynamic_force_n", run: machine_force },
    Adapter { check: "en1991.2.dispersal-footprint", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::dispersed_side_m", run: dispersal_footprint },
    Adapter { check: "en1991.2.fatigue-load-model", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::{fatigue_flm1_axle_n, fatigue_flm1_udl_pa, fatigue_flm3_axle_n, fatigue_flm3_length_m}", run: fatigue_load_models },
    Adapter { check: "en1991.2.support-collision", evaluator: "semio_s_artifact_norm_en1991_checks::part_2::{collision_force_x_n, collision_force_y_n}", run: support_collision },
    Adapter { check: "en1991.1-2.local-fire-hasemi", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_2::{local_fire, local_fire_horizontal_flame_m, local_fire_virtual_source_m, hasemi_y, hasemi_flux_w_m2}", run: local_fire_hasemi },
    Adapter { check: "en1991.1-3.snow-guard", evaluator: "semio_s_artifact_norm_en1991_checks::part_1_3::snow_guard_force_n_m", run: snow_guard },
    Adapter { check: "en1991.3.hoist-release", evaluator: "semio_s_artifact_norm_en1991_checks::part_3::phi_3", run: hoist_release },
    Adapter { check: "en1991.2.rail-derailment-load", evaluator: "semio_s_artifact_norm_en1991_inferences_rail_silo::formulas::{derail_line_load_n_m, derail_length_m, derail_offset_limit_m}", run: rail_derailment_load },
    Adapter { check: "en1991.2.rail-group-bases", evaluator: "semio_s_artifact_norm_en1991_inferences_rail_silo::formulas::{rail_longitudinal_n, rail_nosing_n}", run: rail_group_bases },
    Adapter { check: "en1991.4.dust-vent-area", evaluator: "semio_s_artifact_norm_en1991_inferences_rail_silo::formulas::dust_vent_area_m2", run: dust_vent_area },
];

/// ⚠️ No documented OUR-vs-blessed differences are registered for EN 1991: the LM1 tandem moment now follows both axles and agrees with the blessed value.
static KNOWN: [KnownDifference; 0] = [];

/// 🧭️ The EN 1991 registry.
static REGISTRY: Registry = Registry { artifact: "en1991", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1991 oracle case is reproduced by our evaluators; unwired checks and quantities are reported as gaps.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1991");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}

/// 🏛️ The closed forms behind the new entry points reproduce the blessed PyNite and anastruct values (R_max 540 kN on 6 m, 562.5 kN on 9.6 m; LM71 mid-span 8 297 600 N·m on 24 m).
#[test]
fn tandem_reaction_and_rail_mid_moment_match_the_blessed_values() {
    assert!((part_2::tandem_reaction_n(300_000.0, 6.0) - 540_000.0).abs() < 1e-6);
    assert!((part_2::tandem_reaction_n(300_000.0, 9.6) - 562_500.0).abs() < 1e-6);
    assert!((part_2::tandem_reaction_n(300_000.0, 1.0) - 300_000.0).abs() < 1e-9);
    assert!((part_2::rail_lm71_mid_moment_nm(250_000.0, 80_000.0, 4, 1.6, 0.8, 24.0) - 8_297_600.0).abs() < 1e-3);
}
