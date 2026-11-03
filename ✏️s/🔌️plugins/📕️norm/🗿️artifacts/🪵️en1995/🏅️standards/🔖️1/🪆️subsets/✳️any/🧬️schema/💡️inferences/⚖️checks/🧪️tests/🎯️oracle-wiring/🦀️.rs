//! 🎯️ Typed wiring of the blessed EN 1995 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use semio_s_artifact_norm_en1995_schema::checks::timber::{floor_f1, strength_row};
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_mechanics as mech;

const SEGMENTS: usize = 48;
const REACTION: [&str; 4] = ["R_0", "R_1", "R_2", "R_3"];
const SAGGING: [&str; 3] = ["M_sag_0", "M_sag_1", "M_sag_2"];
const HOGGING: [&str; 3] = ["M_hog_0", "M_hog_1", "M_hog_2"];
const DEFLECTION: [&str; 3] = ["delta_0", "delta_1", "delta_2"];

/// 🪵️ The tabulated strength class of the case in Pa and kg/m³ through `strength_row`; a class without a row (the T classes of EN 338) panics and becomes a gap of the case.
fn strength_class(inputs: &Inputs) -> Outputs {
    let class = inputs.text("strength_class");
    let row = strength_row(class).unwrap_or_else(|| panic!("no EN 338 / EN 14080 strength row for {class}"));
    let pascal = |mpa: f64| (mpa * 1.0e6).round();
    Outputs::default()
        .with("f_m_k", pascal(row.f_m_k))
        .with("f_t_0_k", pascal(row.f_t_0_k))
        .with("f_t_90_k", pascal(row.f_t_90_k))
        .with("f_c_0_k", pascal(row.f_c_0_k))
        .with("f_c_90_k", pascal(row.f_c_90_k))
        .with("f_v_k", pascal(row.f_v_k))
        .with("e_m_0_mean", pascal(row.e_0_mean))
        .with("e_m_0_k", pascal(row.e_0_05))
        .with("e_m_90_mean", pascal(row.e_90_mean))
        .with("g_mean", pascal(row.g_mean))
        .with("rho_k", row.rho_k)
        .with("rho_mean", row.rho_mean)
}

/// 🌊️ Fundamental floor frequency f₁ = π/(2 ℓ²)·√((EI)_ℓ/m) through `floor_f1`.
fn natural_frequency(inputs: &Inputs) -> Outputs {
    Outputs::default().with("f_1", floor_f1(inputs.num("length"), inputs.num("ei_l"), inputs.num("m")))
}

/// 🌉️ The solved beam: support reactions and per-span extremes (sagging and hogging moments, downward deflection).
struct Beam {
    reactions: Vec<f64>,
    sagging: Vec<f64>,
    hogging: Vec<f64>,
    deflection: Vec<f64>,
}

/// 🧮️ A continuous beam on pinned supports solved by the mechanics kernel: `SEGMENTS` elements per span, uniform downward line load `loads[k]` per span and the optional downward midspan forces `P_mid`.
fn solve_beam(inputs: &Inputs, loads: &[f64]) -> Beam {
    let spans = inputs.nums("spans");
    assert!(spans.len() <= 3 && loads.len() == spans.len(), "the oracle wiring solves up to three spans");
    let mid_forces = if inputs.0.contains_key("P_mid") { inputs.nums("P_mid") } else { vec![0.0; spans.len()] };
    let e = inputs.num("E");
    let mut nodes = Vec::new();
    let mut start = 0.0;
    for length in &spans {
        for step in 0..SEGMENTS {
            nodes.push(mech::Node { id: format!("n{}", nodes.len()), x: start + length * step as f64 / SEGMENTS as f64, y: 0.0, z: 0.0 });
        }
        start += length;
    }
    nodes.push(mech::Node { id: format!("n{}", nodes.len()), x: start, y: 0.0, z: 0.0 });
    let members: Vec<mech::Member> = (0..spans.len() * SEGMENTS).map(|j| mech::Member { id: format!("m{j}"), node_i: format!("n{j}"), node_j: format!("n{}", j + 1), material: "S".into(), section: "X".into(), ..Default::default() }).collect();
    let supports: Vec<mech::Support> = (0..=spans.len())
        .map(|k| mech::Support { node: format!("n{}", k * SEGMENTS), fixed: if k == 0 { [true, true, true, true, false, false] } else { [false, true, true, false, false, false] }, ..Default::default() })
        .collect();
    let distributed = members.iter().enumerate().map(|(j, member)| mech::DistributedLoad { member: member.id.clone(), frame: mech::LoadFrame::Global, start: [0.0, 0.0, -loads[j / SEGMENTS]], end: [0.0, 0.0, -loads[j / SEGMENTS]] }).collect();
    let nodal = mid_forces.iter().enumerate().map(|(k, force)| mech::NodalLoad { node: format!("n{}", k * SEGMENTS + SEGMENTS / 2), force: [0.0, 0.0, -force, 0.0, 0.0, 0.0] }).collect();
    let model = mech::Model {
        nodes,
        materials: vec![mech::Material { id: "S".into(), e, g: e / 2.6, rho: 7850.0 }],
        sections: vec![mech::Section { id: "X".into(), a: inputs.num("A"), iy: inputs.num("I"), iz: inputs.num("I"), j: 1.0e-6, ..Default::default() }],
        members,
        supports,
        load_cases: vec![mech::LoadCase { id: "q".into(), nodal, distributed, ..Default::default() }],
        ..Default::default()
    };
    let control = mech::Control::none();
    let result = mech::Analysis::new(&model, &control).expect("beam analysis").solve_case(&model.load_cases[0], &control).expect("beam load case");
    let reactions = (0..=spans.len()).map(|k| result.reactions.iter().find(|reaction| reaction.node == format!("n{}", k * SEGMENTS)).expect("support reaction").force[2]).collect();
    let moments = |k: usize| result.members.iter().skip(k * SEGMENTS).take(SEGMENTS).flat_map(|member| member.diagram.iter().map(|point| point.my)).collect::<Vec<f64>>();
    let sagging = (0..spans.len()).map(|k| moments(k).into_iter().fold(0.0, |peak, my| f64::max(peak, -my))).collect();
    let hogging = (0..spans.len()).map(|k| moments(k).into_iter().fold(0.0, f64::max)).collect();
    let deflection = (0..spans.len())
        .map(|k| result.displacements.iter().filter(|displacement| (k * SEGMENTS..=(k + 1) * SEGMENTS).any(|j| displacement.node == format!("n{j}"))).fold(0.0, |peak, displacement| f64::max(peak, -displacement.u[2])))
        .collect();
    Beam { reactions, sagging, hogging, deflection }
}

/// 🌉️ Continuous beam under span loads and midspan forces through `Analysis::solve_case`: support reactions R_k, extreme moments and deflections per span.
fn beam_continuous(inputs: &Inputs) -> Outputs {
    let beam = solve_beam(inputs, &inputs.nums("w"));
    let mut outputs = Outputs::default();
    for (k, reaction) in beam.reactions.iter().enumerate() {
        outputs = outputs.with(REACTION[k], *reaction);
    }
    for k in 0..beam.sagging.len() {
        outputs = outputs.with(SAGGING[k], beam.sagging[k]).with(HOGGING[k], beam.hogging[k]).with(DEFLECTION[k], beam.deflection[k]);
    }
    outputs
}

/// 🪜️ The shared load arrangement adapter `mechanics.beam-span-envelope` (mounted from the mechanics tree).
#[path = "../../../../../../../../../../../🧮️mechanics/🎯️oracle-wiring/🦀️.rs"]
#[allow(dead_code)]
mod mechanics_wiring;

/// 🗂️ One adapter per oracle check of EN 1995 that has an evaluator entry point.
static ADAPTERS: [Adapter; 4] = [
    Adapter { check: "en1995.338.strength-class", evaluator: "semio_s_artifact_norm_en1995_kernel::strength_row", run: strength_class },
    Adapter { check: "en1995.7.5.natural-frequency", evaluator: "semio_s_artifact_norm_en1995_kernel::floor_f1", run: natural_frequency },
    Adapter { check: "mechanics.beam-continuous", evaluator: "semio_s_plugin_norm_mechanics::Analysis::solve_case", run: beam_continuous },
    mechanics_wiring::SPAN_ENVELOPE,
];

/// ⚠️ No known differences: the registry strength rows are the EN 338:2016 values of the blessed cases.
static KNOWN: [KnownDifference; 0] = [];

/// 🧮️ The EN 1995 adapter registry.
static REGISTRY: Registry = Registry { artifact: "en1995", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1995 oracle case is reproduced by our evaluators within its tolerance profile; gaps, mismatches and stale known differences fail the test.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1995");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
