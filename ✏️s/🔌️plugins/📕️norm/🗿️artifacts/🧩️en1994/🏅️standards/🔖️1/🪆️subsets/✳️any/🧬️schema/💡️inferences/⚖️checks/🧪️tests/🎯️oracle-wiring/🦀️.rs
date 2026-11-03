//! 🎯️ Typed wiring of the blessed EN 1994 oracle cases to OUR evaluators: one adapter per oracle check, driven by the shared `oracle_driver`.

use crate::catalogue as cat;
use crate::part_1_1;
use semio_s_plugin_norm_compliance::oracle_driver::{self, Adapter, Inputs, KnownDifference, Outputs, Registry};
use semio_s_plugin_norm_mechanics as mech;

const SEGMENTS: usize = 48;
const REACTION: [&str; 4] = ["R_0", "R_1", "R_2", "R_3"];
const SAGGING: [&str; 3] = ["M_sag_0", "M_sag_1", "M_sag_2"];
const HOGGING: [&str; 3] = ["M_hog_0", "M_hog_1", "M_hog_2"];
const DEFLECTION: [&str; 3] = ["delta_0", "delta_1", "delta_2"];
const SHEETING_THICKNESS_M: f64 = 1.0e-3;
const IPE300: cat::ISection = cat::ISection { h: 0.300, b: 0.150, tf: 0.0107, tw: 0.0071, a: 5381e-6, i_y: 83.56e-6 };

/// 📐️ The plate model of a doubly symmetric I-section with web thickness `tw`; area and second moment follow from the three plates.
fn plate_section(h: f64, b: f64, tf: f64, tw: f64) -> cat::ISection {
    let hw = h - 2.0 * tf;
    cat::ISection { h, b, tf, tw, a: 2.0 * b * tf + hw * tw, i_y: (b * h.powi(3) - (b - tw) * hw.powi(3)) / 12.0 }
}

/// 🧱️ The steel section of a case: the rolled IPE300 catalogue row when the case carries root fillets, else the plate model.
fn steel_section(inputs: &Inputs) -> cat::ISection {
    let plates = plate_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"));
    if inputs.opt("r").unwrap_or(0.0) <= 0.0 {
        return plates;
    }
    assert!((plates.h - IPE300.h).abs() + (plates.b - IPE300.b).abs() + (plates.tf - IPE300.tf).abs() + (plates.tw - IPE300.tw).abs() < 1e-9, "root fillets are tabulated for the IPE300 row only");
    IPE300
}

/// 🧩️ Transformed elastic section with the slab on top of the steel: n, A_eff, I_eff and the neutral axis measured from the steel bottom fibre through `cat::transformed` and `cat::modular_ratio`.
fn composite_elastic(inputs: &Inputs) -> Outputs {
    let (section, hc) = (steel_section(inputs), inputs.num("h_c"));
    let n = cat::modular_ratio(inputs.num("E_c"), 0.0, 0.0);
    let elastic = cat::transformed(&section, hc, inputs.num("b_eff"), hc, n);
    Outputs::default().with("n", n).with("A_eff", elastic.area).with("I_eff", elastic.i).with("z_na", hc + section.h - elastic.y_na)
}

/// 🧱️ Sagging plastic moment with full shear connection and the plastic neutral axis measured from the slab top through `cat::plastic_full`.
fn composite_plastic(inputs: &Inputs) -> Outputs {
    let section = plate_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"));
    let hc = inputs.num("h_c");
    let fcc = cat::fcc_beam_pa(inputs.num("f_ck"), inputs.num("gamma_c"));
    let plastic = cat::plastic_full(&section, inputs.num("f_y") / inputs.num("gamma_a"), hc, inputs.num("b_eff"), hc, fcc);
    let axis = if plastic.zone == "slab" { plastic.x_m } else { plastic.y_na_m };
    Outputs::default().with("z_pna", axis).with("M_pl_Rd", plastic.m_nm)
}

/// 📈️ Strain-compatible bending resistance and ultimate curvature through `cat::ultimate_curvature` and `cat::moment_at_curvature`.
fn composite_ultimate(inputs: &Inputs) -> Outputs {
    let section = plate_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw_eff"));
    let (ys, hc, beff) = (inputs.num("y_s"), inputs.num("h_c"), inputs.num("b_eff"));
    let limits = cat::NonlinearInput { fyd: inputs.num("f_yd"), fcd: inputs.num("f_cd"), as_m2: 0.0, y_rebar_m: 0.0, fsd: 0.0, eps_c2: 2.0e-3, eps_cu: inputs.num("eps_cu"), eps_su: inputs.num("eps_su") };
    let kappa = cat::ultimate_curvature(&section, ys, beff, hc, &limits);
    Outputs::default().with("kappa_u", kappa).with("M_Rd", cat::moment_at_curvature(&section, ys, beff, hc, &limits, kappa).m_nm.abs())
}

/// 📉️ Moment at a small curvature in the uncracked elastic range, M = E_a I_eff κ, with I_eff of `cat::transformed`.
fn mkappa_elastic(inputs: &Inputs) -> Outputs {
    let section = plate_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"));
    let n = cat::modular_ratio(inputs.num("E_c"), 0.0, 0.0);
    let elastic = cat::transformed(&section, inputs.num("y_s"), inputs.num("b_eff"), inputs.num("h_c"), n);
    Outputs::default().with("M", cat::constants::E_A * elastic.i * inputs.num("kappa"))
}

/// 🔩️ Headed stud resistance, α and the sheeting reduction through `cat::stud_resistance_n`, `part_1_1::stud_alpha` and `cat::sheeting_reduction` (transverse ribs, sheeting of at most 1 mm).
fn stud_resistance(inputs: &Inputs) -> Outputs {
    let (d, h_sc) = (inputs.num("d"), inputs.num("h_sc"));
    let (steel, concrete, resistance) = cat::stud_resistance_n(d, h_sc, inputs.num("f_u"), inputs.num("f_ck"), inputs.num("E_cm"), inputs.num("gamma_V"));
    let k_t = cat::sheeting_reduction(inputs.num("h_p"), inputs.num("b_0"), h_sc, d, SHEETING_THICKNESS_M, inputs.count("n_r") as u32, false).expect("at most two studs per rib");
    Outputs::default()
        .with("alpha", part_1_1::stud_alpha(h_sc, d))
        .with("P_Rd_steel", steel)
        .with("P_Rd_concrete", concrete)
        .with("P_Rd", resistance)
        .with("k_t", k_t)
        .with("P_Rd_sheeting", k_t * resistance)
}

/// 🦋️ Lateral-torsional buckling of the steel beam: the inverted U-frame `cat::mcr_uframe` when the case restrains the compression flange (k_s > 0), else the bare `cat::mcr_bare`; λ_LT and χ_LT through `cat::chi_lt` (curve of the welded branch for α_LT ≥ 0.49); I_w has no entry point.
fn ltb_restrained(inputs: &Inputs) -> Outputs {
    let section = plate_section(inputs.num("h"), inputs.num("b"), inputs.num("tf"), inputs.num("tw"));
    let (length, spring, f_o) = (inputs.num("L"), inputs.num("k_s"), inputs.num("f_o"));
    let m_cr = if spring > 0.0 { cat::mcr_uframe(&section, length, spring) } else { cat::mcr_bare(&section, length) };
    let modulus = section.w_el();
    let (lambda, chi) = cat::chi_lt(modulus * f_o, m_cr, section.h / section.b, inputs.num("alpha_LT") >= 0.49);
    Outputs::default()
        .with("Iz", section.iz())
        .with("It", section.it())
        .with("M_cr", m_cr)
        .with("lambda_LT", lambda)
        .with("chi_LT", chi)
        .with("M_b_Rd", chi * modulus * f_o / inputs.num("gamma_M1"))
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

/// 🗂️ One adapter per oracle check of EN 1994 that has an evaluator entry point.
static ADAPTERS: [Adapter; 8] = [
    Adapter { check: "en1994.5.4.composite-elastic", evaluator: "crate::catalogue::{transformed, modular_ratio}", run: composite_elastic },
    Adapter { check: "en1994.6.2.composite-plastic", evaluator: "crate::catalogue::{plastic_full, fcc_beam_pa}", run: composite_plastic },
    Adapter { check: "en1994.6.2.1.4.composite-ultimate", evaluator: "crate::catalogue::{ultimate_curvature, moment_at_curvature}", run: composite_ultimate },
    Adapter { check: "en1994.6.2.1.4.mkappa-elastic", evaluator: "crate::catalogue::{transformed, modular_ratio}", run: mkappa_elastic },
    Adapter { check: "en1994.6.6.3.stud-resistance", evaluator: "crate::catalogue::{stud_resistance_n, sheeting_reduction}, crate::part_1_1::stud_alpha", run: stud_resistance },
    Adapter { check: "en1994.6.4.ltb-restrained", evaluator: "crate::catalogue::{mcr_uframe, mcr_bare, chi_lt}", run: ltb_restrained },
    Adapter { check: "mechanics.beam-continuous", evaluator: "semio_s_plugin_norm_mechanics::Analysis::solve_case", run: beam_continuous },
    mechanics_wiring::SPAN_ENVELOPE,
];

/// ⚠️ No documented differences: every deviation of our evaluators from the blessed values stays a visible mismatch.
static KNOWN: [KnownDifference; 0] = [];

/// 🧮️ The EN 1994 adapter registry.
static REGISTRY: Registry = Registry { artifact: "en1994", adapters: &ADAPTERS, known_differences: &KNOWN };

/// 🚦️ Every blessed EN 1994 oracle case is reproduced by our evaluators within its tolerance profile; gaps and mismatches fail the test.
#[test]
fn evaluators_reproduce_the_blessed_oracle_cases() {
    oracle_driver::drive(env!("CARGO_MANIFEST_DIR"), &REGISTRY).assert_clean("en1994");
}

/// 🧭️ The typed adapter registry of this artifact for the shared `oracle-comparison` step layer (compiled for tests and the `oracle-wiring` feature only).
pub fn registry() -> &'static Registry {
    &REGISTRY
}
