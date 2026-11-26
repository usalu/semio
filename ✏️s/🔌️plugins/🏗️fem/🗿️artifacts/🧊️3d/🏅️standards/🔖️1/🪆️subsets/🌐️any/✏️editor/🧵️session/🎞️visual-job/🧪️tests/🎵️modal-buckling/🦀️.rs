//! 🎵️ Laws of the mounted modal and buckling numerical child against closed forms and the dense engine.

use super::*;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;

struct Solved {
    ids: Vec<String>,
    scalars: Vec<Fem3dSolverScalar>,
    turns: usize,
    failure: Option<String>,
    ready: bool,
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
    while turns < maximum_turns && !terminal && failure.is_none() {
        turns += 1;
        let mut outcome = None;
        caller.turn(TEST_GRANT, |cx| { outcome = Some(child.step(doc, &mut fields, &mut backing, freshness(19), operation, cx)); Ok(()) }).expect("original numerical turn");
        match outcome.expect("the original numerical turn ran") {
            Ok(complete) => terminal = complete,
            Err(fault) => failure = Some(format!("{:?}: {fault:?}", child.stage)),
        }
    }
    let ready = fields.ready();
    let ids = (0..child.analysis_node_ids.len()).filter_map(|index| child.analysis_node_ids.get(index).cloned()).collect();
    let scalars = (0..fields.len).filter_map(|index| fields.scalar(index)).collect();
    for _ in 0..400_000 {
        if close_with(&mut caller, &mut child, TEST_GRANT).0 { break; }
    }
    caller.retire(bank);
    if !terminal && failure.is_none() { failure = Some(format!("turn budget {maximum_turns} exhausted")); }
    Solved { ids, scalars, turns, failure, ready }
}

fn hertz(eigenvalue: f64) -> f64 { eigenvalue.max(0.0).sqrt() / std::f64::consts::TAU }

#[test]
fn probe_demo_modal() {
    let doc = Doc::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo"));
    eprintln!("[DEBUG] demo nodes={} elements={} solids={}", doc.nodes.len(), doc.elements.len(), doc.solids.len());
    for mode in [None, Some(0), Some(1), Some(3)] {
        let solved = solve_mounted(&doc, Fem3dResultRequest { mode, ..Default::default() }, 30_000_000);
        let eigen = solved.scalars.iter().map(|s| s.eigen_estimate).fold(0.0, f64::max);
        let peak = solved.scalars.iter().map(|s| s.mode_shape.iter().map(|v| v * v).sum::<f64>().sqrt()).fold(0.0, f64::max);
        eprintln!("[DEBUG] mode={mode:?} turns={} failure={:?} ready={} fields={} eigen={eigen} hz={} peak={peak}", solved.turns, solved.failure, solved.ready, solved.scalars.len(), hertz(eigen));
    }
}
