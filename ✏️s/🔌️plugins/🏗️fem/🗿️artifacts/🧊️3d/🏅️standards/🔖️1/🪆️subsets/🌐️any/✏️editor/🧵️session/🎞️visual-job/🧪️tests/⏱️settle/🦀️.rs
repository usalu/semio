use super::*;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

/// ⏱️ Drives the production numerical child over `doc` until it is terminal, returning the turns spent, the turns per stage and any fault.
fn settle(doc: Doc, request: Fem3dResultRequest, budget: usize) -> (bool, usize, std::collections::BTreeMap<String, usize>, Option<String>) {
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    child.request = request;
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let bank = semio_s_fem_receiving::testkit::numerical_stock();
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
    let mut stages = std::collections::BTreeMap::new();
    let (mut terminal, mut failure, mut turns) = (false, None, 0);
    while turns < budget && !terminal && failure.is_none() {
        turns += 1;
        let mut outcome = None;
        caller.turn(TEST_GRANT, |cx| { outcome = Some(child.step(&doc, &mut fields, &mut backing, freshness(19), operation, cx)); Ok(()) }).expect("original numerical turn");
        *stages.entry(format!("{:?}", child.stage)).or_insert(0) += 1;
        match outcome.expect("the numerical turn ran") { Ok(done) => terminal = done, Err(fault) => failure = Some(format!("{:?}: {fault:?}", child.stage)) }
    }
    for _ in 0..200_000 { if close_with(&mut caller, &mut child, TEST_GRANT).0 { break; } }
    caller.retire(bank);
    (terminal, turns, stages, failure)
}

/// ⏱️ LAW: the browser's default Add Solid slab (4 × 2 × 0.5, mesh 0.5, one layer) and the demo document each reach a terminal static and modal solve within an interactive turn budget, so the results caption never sticks at "Preparing analysis" or "Computing modes".
#[test]
fn a_default_slab_and_the_demo_settle_within_an_interactive_turn_budget() {
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemAxis, FemDof, FemLoadCase, FemSolid, FemSupport};
    use semio_s_fem_material::volumetric::FemMaterialValue;
    use semio_s_fem_node::volumetric::FemNodeValue;
    let slab = Fem3dSnapshotValue {
        nodes: (0..4).map(|index| FemNodeValue { id: format!("n{index}"), x: [0.0, 4.0, 4.0, 0.0][index], y: [0.0, 0.0, 2.0, 2.0][index], z: 0.0 }).collect(),
        materials: vec![FemMaterialValue { id: "concrete".into(), name: "Concrete".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
        solids: vec![FemSolid { id: "s".into(), name: "S".into(), outline: vec![[0.0, 0.0], [4.0, 0.0], [4.0, 2.0], [0.0, 2.0]], holes: vec![], base_z: 0.0, height: 0.5, layers: 1, mesh_size: 0.5, material_id: "concrete".into(), axis: FemAxis::Z }],
        supports: (0..4).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] }).collect(),
        load_cases: vec![FemLoadCase { id: "dead".into(), name: "Dead".into(), loads: vec![], self_weight: true }],
        ..Default::default()
    };
    let slab = Doc::new(Fem3dSnapshot::from_reference(slab).expect("sealed slab"));
    let demo = || Doc::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::fem3d_demo_snapshot().expect("demo"));
    for (name, doc, request) in [("slab", slab, Fem3dResultRequest::default()), ("demo", demo(), Fem3dResultRequest::default()), ("demo-modal", demo(), Fem3dResultRequest { mode: Some(0), ..Fem3dResultRequest::default() })] {
        let (terminal, turns, stages, failure) = settle(doc, request, 60_000);
        let busiest = stages.iter().max_by_key(|(_, count)| **count).map(|(stage, count)| format!("{stage}×{count}"));
        assert!(failure.is_none() && terminal, "{name} did not settle in {turns} turns, busiest stage {busiest:?}, stages {stages:?}: {failure:?}");
    }
}
