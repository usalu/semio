//! 🎯️ Oracle wiring of EN 1998: every blessed case of `🧪️tests/🧫️fixtures/🔮️oracle-cases` is fed to OUR clause functions and the mechanics kernel and compared with the blessed value under the case profile.

use crate::part_1;
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_mechanics as mech;
use semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{critical_wedge, thrust, Acceleration, Interface, Side, Water};

const GRAVITY: f64 = 9.81;
static PERIODS: [&str; 5] = ["T_1", "T_2", "T_3", "T_4", "T_5"];
static MODAL_SHEARS: [&str; 5] = ["F_b_1", "F_b_2", "F_b_3", "F_b_4", "F_b_5"];
static DISPLACEMENTS: [&str; 6] = ["u_1", "u_2", "u_3", "u_4", "u_5", "u_6"];
static DRIFTS: [&str; 6] = ["drift_1", "drift_2", "drift_3", "drift_4", "drift_5", "drift_6"];

fn design_spectrum(i: &Inputs) -> Outputs {
    let (a_g, s, t_b, t_c, t_d, t) = (i.num("a_g"), i.num("S"), i.num("T_B"), i.num("T_C"), i.num("T_D"), i.num("T"));
    Outputs::default().with("S_e", part_1::elastic_response_spectrum(a_g, s, t_b, t_c, t_d, t)).with("S_d", part_1::design_spectrum_horizontal(a_g, s, t_b, t_c, t_d, i.num("q"), t))
}

fn shear_building_modal(i: &Inputs) -> Outputs {
    let (masses, stiffness) = (i.nums("m"), i.nums("k"));
    let (periods, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let first = &shapes[0];
    let participation = first.iter().zip(&masses).map(|(phi, m)| phi * m).sum::<f64>() / first.iter().zip(&masses).map(|(phi, m)| phi * phi * m).sum::<f64>();
    let effective = part_1::effective_modal_masses(&masses, &shapes);
    PERIODS.iter().zip(&periods).fold(Outputs::default(), |out, (name, t)| out.with(*name, *t)).with("Gamma_1", participation * first[first.len() - 1]).with("m_eff_1_fraction", effective[0] / masses.iter().sum::<f64>())
}

/// 🌍️ The layers of a profile clipped to the top 30 m that v_s,30 averages over.
fn top_thirty_metres(thickness: &[f64], velocity: &[f64]) -> Vec<(f64, f64)> {
    let mut depth = 0.0;
    let mut layers = Vec::new();
    for (h, v) in thickness.iter().zip(velocity) {
        let taken = h.min(30.0 - depth).max(0.0);
        if taken > 0.0 {
            layers.push((taken, *v));
        }
        depth += taken;
    }
    layers
}

/// 🌋 Modal base shear F_b,j = M_eff,j S_d(T_j) of the shear building (EN 1998-1 4.3.3.3) with its SRSS and CQC combination.
fn modal_base_shear(i: &Inputs) -> Outputs {
    let (masses, stiffness) = (i.nums("m"), i.nums("k"));
    let (periods, shapes) = part_1::shear_building_modes(&masses, &stiffness);
    let modal: Vec<f64> = part_1::effective_modal_masses(&masses, &shapes).iter().zip(&periods).map(|(m, t)| m * part_1::design_spectrum_horizontal(i.num("a_g"), i.num("S"), i.num("T_B"), i.num("T_C"), i.num("T_D"), i.num("q"), *t)).collect();
    MODAL_SHEARS.iter().zip(&modal).fold(Outputs::default(), |out, (name, force)| out.with(*name, *force)).with("V_srss", part_1::srss_combine(&modal)).with("V_cqc", part_1::cqc_combine(&modal, &periods, i.num("xi")))
}

fn vs30(i: &Inputs) -> Outputs {
    let v_s30 = part_1::vs30_from_layers(&top_thirty_metres(&i.nums("h"), &i.nums("v")));
    let class = part_1::ground_class_from_vs30(v_s30).expect("ground class");
    Outputs::default().with("vs30", v_s30).with("class_index", f64::from(class as u8 - b'A'))
}

fn nonstructural(i: &Inputs) -> Outputs {
    let (w_a, gamma_a, q_a) = (i.num("W_a"), i.num("gamma_a"), i.num("q_a"));
    let f_a = part_1::non_structural_force_n(i.num("a_g") / GRAVITY, i.num("S"), i.num("Z"), i.num("H"), i.num("T_a"), i.num("T_1"), w_a, gamma_a, q_a);
    Outputs::default().with("F_a", f_a).with("S_a", f_a * q_a / (w_a * gamma_a))
}

/// 🌋 Mononobe-Okabe thrust of a vertical wall with a horizontal backfill: the shared kernel against the maximised force polygon.
fn mononobe_okabe(i: &Inputs) -> Outputs {
    let (gamma, h, phi, delta) = (i.num("gamma"), i.num("H"), i.num("phi"), i.num("delta"));
    let load = thrust(Side::Active, &Interface::vertical(phi, delta), gamma, h, Water::NONE, Acceleration { kh: i.num("k_h"), kv: i.num("k_v") }).expect("limit equilibrium");
    let (angle, _) = critical_wedge(gamma, h, phi, delta, i.num("k_h"), load.vertical);
    Outputs::default().with("E_ae", load.total()).with("K_ae", load.coefficient).with("rho_deg", angle)
}

/// 🌊 Wall thrust with retained water: the layered Mononobe-Okabe soil thrust, the hydrostatic and the Westergaard thrust of the shared kernel.
fn wall_water(i: &Inputs) -> Outputs {
    let water = Water { height_m: i.num("H_w"), saturated_unit_weight: i.num("gamma_sat"), permeable: i.num("permeable") > 0.5 };
    let load = thrust(Side::Active, &Interface::vertical(i.num("phi"), 0.0), i.num("gamma"), i.num("H"), water, Acceleration { kh: i.num("k_h"), kv: 0.0 }).expect("limit equilibrium");
    Outputs::default().with("H_soil", load.soil).with("H_static", load.water_static).with("H_dynamic", load.water_dynamic).with("H_total", load.total())
}

/// ⛰️ Pseudo-static slope stability: the given circle and the critical circle of the grid search under F_H = 0.5 alpha S S_T W and F_V = ±share F_H with the M2 strengths, against the sliced circle solved by brentq.
fn seismic_slope(i: &Inputs) -> Outputs {
    use crate::part_5::{slope_circle, slope_critical_circle, slope_factor, slope_pseudostatic, Pseudostatic};
    let (height, beta, gamma, phi, cohesion, gamma_phi, gamma_c) = (i.num("H"), i.num("beta"), i.num("gamma"), i.num("phi"), i.num("c"), i.num("gamma_phi"), i.num("gamma_c"));
    let p = slope_pseudostatic(i.num("factor"), i.num("a_g"), i.num("S"), i.num("S_T"), i.num("share"));
    let circle = slope_circle(height, beta, gamma, i.num("xc"), i.num("yc"), i.num("R"));
    let given = slope_factor(&circle, phi, cohesion, gamma_phi, gamma_c, &p).expect("the given circle develops a sliding mass");
    let (_, critical) = slope_critical_circle(height, beta, gamma, phi, cohesion, gamma_phi, gamma_c, &p).expect("a circle develops a sliding mass");
    Outputs::default().with("F_static", slope_factor(&circle, phi, cohesion, gamma_phi, gamma_c, &Pseudostatic::NONE).expect("static factor")).with("F_given", given).with("F_critical", critical).with("F_s", given.min(critical)).with("k_h", p.horizontal)
}

/// 🔗 Tie force of a foundation tie beam with the registry coefficient of the ground type, the steel resistance and their ratio.
fn tie_beam(i: &Inputs) -> Outputs {
    let id = ["ndp.en1998.en5.tie.coefficient.b", "ndp.en1998.en5.tie.coefficient.c", "ndp.en1998.en5.tie.coefficient.d"][i.num("ground_index") as usize];
    let n_tie = crate::part_5::tie_force_n(semio_s_plugin_norm_compliance::ndp_number(id, semio_s_plugin_norm_compliance::AnnexChoice::En), i.num("a_g"), i.num("S"), i.num("N_Ed"));
    let n_rd = crate::part_5::tie_resistance_n(i.num("A_s"), i.num("f_yd"));
    Outputs::default().with("N_tie", n_tie).with("N_Rd", n_rd).with("utilisation", n_tie / n_rd)
}

/// 🏢️ Fixed-base single-bay frame in the X-Z plane under the floor forces at the left joints, solved by the mechanics kernel.
fn lateral_frame(i: &Inputs) -> Outputs {
    let (e, area, span, heights, forces) = (i.num("E"), i.num("A"), i.num("span"), i.nums("h"), i.nums("F"));
    let levels: Vec<f64> = std::iter::once(0.0).chain(heights.iter().scan(0.0, |z, h| { *z += h; Some(*z) })).collect();
    let id = |side: &str, level: usize| format!("{side}{level}");
    let node = |side: &str, x: f64, level: usize| mech::Node { id: id(side, level), x, y: 0.0, z: levels[level] };
    let section = |name: &str, inertia: f64| mech::Section { id: name.into(), a: area, iy: inertia, iz: inertia, j: 1.0e-6, ..Default::default() };
    let bar = |name: String, from: String, to: String, section: &str| mech::Member { id: name, node_i: from, node_j: to, material: "S".into(), section: section.into(), ..Default::default() };
    let storeys = heights.len();
    let mut members = Vec::new();
    for level in 0..storeys {
        for side in ["l", "r"] {
            members.push(bar(format!("c{side}{level}"), id(side, level), id(side, level + 1), "C"));
        }
        members.push(bar(format!("b{}", level + 1), id("l", level + 1), id("r", level + 1), "B"));
    }
    let model = mech::Model {
        nodes: (0..=storeys).flat_map(|level| [node("l", 0.0, level), node("r", span, level)]).collect(),
        materials: vec![mech::Material { id: "S".into(), e, g: e / 2.6, rho: 7850.0 }],
        sections: vec![section("C", i.num("I_col")), section("B", i.num("I_beam"))],
        members,
        supports: ["l0", "r0"].iter().map(|base| mech::Support { node: (*base).into(), fixed: [true; 6], ..Default::default() }).collect(),
        load_cases: vec![mech::LoadCase { id: "F".into(), nodal: forces.iter().enumerate().map(|(level, force)| mech::NodalLoad { node: id("l", level + 1), force: [*force, 0.0, 0.0, 0.0, 0.0, 0.0] }).collect(), ..Default::default() }],
        ..Default::default()
    };
    let control = mech::Control::none();
    let result = mech::Analysis::new(&model, &control).expect("frame analysis").solve_case(&model.load_cases[0], &control).expect("frame load case");
    let sway = |level: usize| if level == 0 { 0.0 } else { result.displacements.iter().find(|displacement| displacement.node == id("l", level)).expect("floor displacement").u[0] };
    let reaction = |base: &str| result.reactions.iter().find(|reaction| reaction.node == base).expect("base reaction").force;
    let mut out = Outputs::default();
    for level in 1..=storeys {
        out = out.with(DISPLACEMENTS[level - 1], sway(level)).with(DRIFTS[level - 1], sway(level) - sway(level - 1));
    }
    out.with("V_base", reaction("l0")[0].abs() + reaction("r0")[0].abs()).with("M_base_left", reaction("l0")[4].abs()).with("M_base_right", reaction("r0")[4].abs())
}

static ADAPTERS: [Adapter; 10] = [
    Adapter { check: "en1998.3.2.2.5.design-spectrum", evaluator: "crate::part_1::{elastic_response_spectrum, design_spectrum_horizontal}", run: design_spectrum },
    Adapter { check: "en1998.4.3.shear-building-modal", evaluator: "crate::part_1::{shear_building_modes, effective_modal_masses}", run: shear_building_modal },
    Adapter { check: "en1998.4.3.3.3.modal-base-shear", evaluator: "crate::part_1::{shear_building_modes, effective_modal_masses, design_spectrum_horizontal, srss_combine, cqc_combine}", run: modal_base_shear },
    Adapter { check: "en1998.3.1.vs30", evaluator: "crate::part_1::{vs30_from_layers, ground_class_from_vs30}", run: vs30 },
    Adapter { check: "en1998.4.3.5.2.nonstructural", evaluator: "crate::part_1::non_structural_force_n", run: nonstructural },
    Adapter { check: "en1998.5.mononobe-okabe", evaluator: "semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::{thrust, critical_wedge}", run: mononobe_okabe },
    Adapter { check: "en1998.5.wall-water", evaluator: "semio_s_plugin_norm_mechanics::geotechnics::earth_pressure::thrust", run: wall_water },
    Adapter { check: "en1998.5.seismic-slope", evaluator: "crate::part_5::{slope_pseudostatic, slope_circle, slope_factor, slope_critical_circle}", run: seismic_slope },
    Adapter { check: "en1998.5.tie-beam", evaluator: "crate::part_5::{tie_force_n, tie_resistance_n}", run: tie_beam },
    Adapter { check: "mechanics.lateral-frame", evaluator: "semio_s_plugin_norm_mechanics::Analysis::solve_case", run: lateral_frame },
];

static KNOWN: [KnownDifference; 0] = [];

static REGISTRY: Registry = Registry { artifact: "en1998", adapters: &ADAPTERS, known_differences: &KNOWN };

#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1998");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
