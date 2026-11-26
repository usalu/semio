//! 🎵️ Laws of the mounted modal and buckling numerical child against closed forms and the dense engine.

use super::*;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemDof, FemLoad, FemSection, FemSupport};

struct Solved {
    scalars: Vec<Fem3dSolverScalar>,
    turns: usize,
    failure: Option<String>,
    ready: bool,
    stages: std::collections::BTreeMap<String, usize>,
    progress: Option<(usize, usize, f64, bool)>,
}

fn solve_mounted(doc: &Fem3dSnapshot, request: Fem3dResultRequest, maximum_turns: usize) -> Solved {
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    child.request = request;
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let bank = semio_s_fem_receiving::testkit::numerical_stock();
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
    let mut turns = 0;
    let mut failure = None;
    let mut terminal = false;
    let mut stages = std::collections::BTreeMap::new();
    while turns < maximum_turns && !terminal && failure.is_none() {
        turns += 1;
        let mut outcome = None;
        caller.turn(TEST_GRANT, |cx| { outcome = Some(child.step(doc, &mut fields, &mut backing, freshness(19), operation, cx)); Ok(()) }).expect("original numerical turn");
        *stages.entry(format!("{:?}", child.stage)).or_insert(0) += 1;
        match outcome.expect("the original numerical turn ran") {
            Ok(complete) => terminal = complete,
            Err(fault) => failure = Some(format!("{:?}: {fault:?}", child.stage)),
        }
    }
    let ready = fields.ready();
    let progress = child.subspace.as_ref().map(SubspaceIterationJob::visual_progress);

    let scalars = (0..fields.len).filter_map(|index| fields.scalar(index)).collect();
    for _ in 0..400_000 {
        if close_with(&mut caller, &mut child, TEST_GRANT).0 { break; }
    }
    caller.retire(bank);
    if !terminal && failure.is_none() { failure = Some(format!("turn budget {maximum_turns} exhausted at {:?}, stages {stages:?}", child.stage)); }
    Solved { scalars, turns, failure, ready, stages, progress }
}

fn hertz(eigenvalue: f64) -> f64 { eigenvalue.max(0.0).sqrt() / std::f64::consts::TAU }

fn eigenvalue_of(solved: &Solved) -> f64 { solved.scalars.iter().map(|scalar| scalar.eigen_estimate).fold(0.0, f64::max) }

fn shape_of(solved: &Solved) -> Vec<[f64; 3]> { solved.scalars.iter().map(|scalar| scalar.mode_shape).collect() }

fn peak_of(shape: &[[f64; 3]]) -> f64 { shape.iter().map(|value| value.iter().map(|component| component * component).sum::<f64>().sqrt()).fold(0.0, f64::max) }

fn norm_of(shape: &[[f64; 3]]) -> f64 { shape.iter().map(|value| value.iter().map(|component| component * component).sum::<f64>()).sum::<f64>().sqrt() }

const HEA200: (f64, f64, f64, f64) = (0.00538, 3.692e-5, 1.336e-5, 2.098e-7);
const SHS150: (f64, f64, f64, f64) = (0.004416, 1.4595e-5, 9.7301e-6, 2.2646e-5);
const ALL: [FemDof; 6] = [FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rx, FemDof::Ry, FemDof::Rz];

fn fixed(id: &str, node: &str, dofs: &[FemDof]) -> FemSupport { FemSupport { id: id.into(), node_id: node.into(), fixed: dofs.to_vec() } }

fn line(direction: [f64; 3], length: f64, elements: usize, (area, iy, iz, j): (f64, f64, f64, f64), supports: Vec<FemSupport>, loads: Vec<FemLoad>) -> Doc {
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemElement, FemLoadCase};
    use semio_s_fem_material::volumetric::FemMaterialValue;
    use semio_s_fem_node::volumetric::FemNodeValue;
    let step = length / elements as f64;
    let mut value = Fem3dSnapshotValue {
        nodes: (0..=elements).map(|index| FemNodeValue { id: format!("n{index}"), x: direction[0] * step * index as f64, y: direction[1] * step * index as f64, z: direction[2] * step * index as f64 }).collect(),
        elements: (0..elements).map(|index| FemElement::Frame { id: format!("e{index}"), start: format!("n{index}"), end: format!("n{}", index + 1), material_id: "steel".into(), section_id: "section".into(), roll: 0.0 }).collect(),
        materials: vec![FemMaterialValue { id: "steel".into(), name: "Steel".into(), e: 210e9, g: 80.769230769e9, nu: 0.3, rho: 7850.0 }],
        sections: vec![FemSection { id: "section".into(), name: "Section".into(), area, iy, iz, j }],
        supports,
        load_cases: vec![FemLoadCase { id: "axial".into(), name: "Axial".into(), loads, self_weight: false }],
        ..Default::default()
    };
    value.analysis.modal_count = 4;
    value.analysis.buckling_count = 2;
    value.analysis.deformation_scale = 1.0;
    Doc::new(Fem3dSnapshot::from_reference(value).expect("sealed line model"))
}

fn tip_load(node: &str, dof: FemDof, value: f64) -> Vec<FemLoad> { vec![FemLoad::Nodal { id: "p".into(), node_id: node.into(), dof, value }] }

fn cantilever(elements: usize) -> Doc { line([1.0, 0.0, 0.0], 4.0, elements, SHS150, vec![fixed("fixity", "n0", &ALL)], vec![]) }

fn closed_cantilever_hz(second_moment: f64) -> f64 {
    let beta = 1.8751040687_f64;
    beta * beta / (std::f64::consts::TAU * 16.0) * (210e9 * second_moment / (7850.0 * SHS150.0)).sqrt()
}

fn solved_modes(doc: &Fem3dSnapshot, count: usize) -> Vec<Solved> {
    (0..count).map(|mode| solve_mounted(doc, Fem3dResultRequest { mode: Some(mode), ..Default::default() }, 60_000_000)).collect()
}

/// 🧪️ LAW: the mounted modal child lands on the Euler-Bernoulli cantilever closed form (weak axis first, strong axis second) and on the dense engine's frequencies, ascending, finite and positive.
#[test]
fn the_mounted_cantilever_modes_match_the_closed_form_the_dense_engine_and_ascend() {
    let doc = cantilever(3);
    let expected = [closed_cantilever_hz(SHS150.2), closed_cantilever_hz(SHS150.1)];
    let dense = semio_s_artifact_fem_3d_engine::fem3d_engine::modal_buckling::fem3d_modal(&doc).expect("dense modal");
    let mut previous = 0.0;
    for (index, solved) in solved_modes(&doc, 2).iter().enumerate() {
        assert!(solved.failure.is_none() && solved.ready, "mode {index} settles: {:?}", solved.failure);
        let frequency = hertz(eigenvalue_of(solved));
        assert!(frequency.is_finite() && frequency > previous, "mode {index} frequency {frequency} is finite, positive and ascends past {previous}");
        assert!((frequency - expected[index]).abs() / expected[index] < 0.08, "mode {index}: {frequency} Hz against the closed form {} Hz", expected[index]);
        assert!((frequency - dense.frequencies_hz[index]).abs() / dense.frequencies_hz[index] < 0.08, "mode {index}: {frequency} Hz against the dense engine {} Hz", dense.frequencies_hz[index]);
        previous = frequency;
    }
}

/// 🧪️ LAW: two mode indices select two different, non-trivial shapes of the same structure.
#[test]
fn each_mode_index_selects_its_own_non_trivial_shape() {
    let doc = cantilever(3);
    let shapes: Vec<Vec<[f64; 3]>> = solved_modes(&doc, 2).iter().map(shape_of).collect();
    assert!(shapes.iter().all(|shape| peak_of(shape) > 0.0), "every mode deforms");
    let dot: f64 = shapes[0].iter().zip(&shapes[1]).map(|(a, b)| a[0] * b[0] + a[1] * b[1] + a[2] * b[2]).sum();
    assert!(dot.abs() / (norm_of(&shapes[0]) * norm_of(&shapes[1])) < 0.5, "modes 0 and 1 are different shapes");
}

/// 🧪️ LAW: the mounted linear buckling child lands on the Euler load of the fixed-free and the pinned-pinned column.
#[test]
fn the_mounted_buckling_factor_matches_the_euler_load() {
    let cases: [(&str, Vec<FemSupport>, f64); 2] = [
        ("fixed-free", vec![fixed("base", "n0", &ALL)], 0.25),
        ("pinned-pinned", vec![fixed("base", "n0", &[FemDof::Tx, FemDof::Ty, FemDof::Tz, FemDof::Rz]), fixed("head", "n2", &[FemDof::Tx, FemDof::Ty])], 1.0),
    ];
    for (name, supports, factor) in cases {
        let doc = line([0.0, 0.0, 1.0], 5.0, 2, SHS150, supports, tip_load("n2", FemDof::Tz, -1.0));
        let euler = std::f64::consts::PI.powi(2) * 210e9 * SHS150.2 * factor / 25.0;
        let solved = solve_mounted(&doc, Fem3dResultRequest { buckling: Some(0), ..Default::default() }, 60_000_000);
        assert!(solved.failure.is_none() && solved.ready, "{name} buckling settles: {:?}", solved.failure);
        let found = eigenvalue_of(&solved);
        assert!(found.is_finite() && found > 0.0, "{name} factor {found} is finite and positive");
        assert!((found - euler).abs() / euler < 0.03, "{name}: factor {found} against the Euler load {euler}");
        assert!(peak_of(&shape_of(&solved)) > 0.0, "{name} buckles into a shape");
    }
}

/// 🧪️ LAW: a load that never compresses the structure has no buckling factor, and the child says so with a finished, shapeless result instead of a fault or a hang.
#[test]
fn a_load_without_compression_ends_in_a_shapeless_result_not_a_fault() {
    let doc = line([1.0, 0.0, 0.0], 4.0, 1, HEA200, vec![fixed("fixity", "n0", &ALL)], tip_load("n1", FemDof::Ty, 1000.0));
    let solved = solve_mounted(&doc, Fem3dResultRequest { buckling: Some(0), ..Default::default() }, 60_000_000);
    assert!(solved.failure.is_none() && solved.ready, "a transverse load settles: {:?}", solved.failure);
    assert_eq!(eigenvalue_of(&solved), 0.0, "no buckling factor");
    assert_eq!(peak_of(&shape_of(&solved)), 0.0, "no shape");
}

