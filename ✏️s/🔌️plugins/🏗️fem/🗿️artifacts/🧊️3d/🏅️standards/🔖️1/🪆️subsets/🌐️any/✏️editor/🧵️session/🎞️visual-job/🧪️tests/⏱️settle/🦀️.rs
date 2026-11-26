use super::*;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;

/// ⏱️ Drives the production numerical child over `value` until it is terminal, returning the turns spent, the turns per stage and any fault.
fn settle(value: Fem3dSnapshotValue, request: Fem3dResultRequest, budget: usize) -> (bool, usize, std::collections::BTreeMap<String, usize>, Option<String>) {
    let doc = Doc::new(Fem3dSnapshot::from_reference(value).expect("sealed settle snapshot"));
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

/// ⏱️ LAW: the browser's default Add Solid slab (4 × 2 × 0.5, mesh 0.5, one layer) and the bundled House example each reach a terminal static solve within an interactive turn budget, so the results caption never sticks at "Preparing analysis".
#[test]
fn a_default_slab_and_the_house_settle_within_an_interactive_turn_budget() {
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
    for (name, value) in [("slab", slab), ("house", semio_s_artifact_fem_3d_io::examples::house::build_rows())] {
        let (terminal, turns, stages, failure) = settle(value, Fem3dResultRequest::default(), 400_000);
        let busiest = stages.iter().max_by_key(|(_, count)| **count).map(|(stage, count)| format!("{stage}×{count}"));
        eprintln!("[DEBUG] settle {name}: terminal={terminal} turns={turns} busiest={busiest:?} stages={stages:?} failure={failure:?}");
        assert!(failure.is_none() && terminal, "{name} did not settle in {turns} turns, busiest stage {busiest:?}: {failure:?}");
    }
}
