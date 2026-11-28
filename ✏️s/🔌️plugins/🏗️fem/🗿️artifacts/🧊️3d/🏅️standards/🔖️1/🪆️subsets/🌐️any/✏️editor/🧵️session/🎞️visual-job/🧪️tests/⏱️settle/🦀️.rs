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

/// ⏱️ LAW: the browser's default Add Solid slab (4 × 2 × 0.5, mesh 0.5, one layer) and the demo document each reach a terminal static and modal solve within an interactive turn budget, so the results caption never sticks at "Preparing analysis" or "Computing modes". Measured at 232, 43 and 900 turns; the budgets leave headroom but fail on any stage that falls back to one bounded unit, one payload scalar or one retained item per turn.
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
    for (name, doc, request, budget) in [("slab", slab, Fem3dResultRequest::default(), 400), ("demo", demo(), Fem3dResultRequest::default(), 100), ("demo-modal", demo(), Fem3dResultRequest { mode: Some(0), ..Fem3dResultRequest::default() }, 1_500)] {
        let (terminal, turns, stages, failure) = settle(doc, request, budget);
        let busiest = stages.iter().max_by_key(|(_, count)| **count).map(|(stage, count)| format!("{stage}×{count}"));
        assert!(failure.is_none() && terminal, "{name} did not settle within {budget} turns (spent {turns}), busiest stage {busiest:?}, stages {stages:?}: {failure:?}");
    }
}

/// [DEBUG] 🔬️ Profiles the House solve: turns, wall time and peak heap per stage, real clock sliced like a host frame.
#[test]
fn zz_debug_house_profile() {
    let text = include_str!("../../../../../🖼️assets/🏠️house/🗣️.dsl.semio");
    let doc = Doc::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).unwrap());
    let sliced = std::env::var("PROFILE_SLICE").map(|value| value == "1").unwrap_or(true);
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let bank = semio_s_fem_receiving::testkit::numerical_stock();
    static EPOCH: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let clock: fn() -> Option<u64> = if sliced { || Some(1000 + EPOCH.get_or_init(std::time::Instant::now).elapsed().as_micros() as u64) } else { || Some(1000) };
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, clock, 1 << 40, 1 << 40);
    if sliced { caller.slice_each_turn(semio_framework_trace::INTERACTIVE_STEP_CEILING_US / 2); }
    let mut stages: std::collections::BTreeMap<String, (usize, u128, u64)> = Default::default();
    let mut heap_by_stage: std::collections::BTreeMap<String, isize> = Default::default();
    let mut pcg_seen = None;
    let started = std::time::Instant::now();
    semio_framework_trace::reset_heap_peak_on_this_thread();
    let base = semio_framework_trace::retained_heap_bytes_on_this_thread();
    let (mut terminal, mut failure, mut turns, mut peak_live) = (false, None, 0usize, 0usize);
    while turns < 20_000_000 && !terminal && failure.is_none() {
        turns += 1;
        let before = format!("{:?}", child.stage);
        let tick = std::time::Instant::now();
        let mut outcome = None;
        let mut fuel_spent = 0u64;
        caller.turn(TEST_GRANT, |cx| { let fuel = cx.fuel_remaining(); outcome = Some(child.step(&doc, &mut fields, &mut backing, freshness(19), operation, cx)); fuel_spent = fuel - cx.fuel_remaining(); Ok(()) }).expect("turn");
        if let Some(progress) = child.pcg.as_ref().map(PcgJob::visual_progress) { pcg_seen = Some(progress); }
        let entry = stages.entry(before).or_insert((0, 0, 0));
        entry.0 += 1;
        entry.1 += tick.elapsed().as_micros();
        entry.2 += fuel_spent;
        let here = semio_framework_trace::retained_heap_bytes_on_this_thread() - base;
        let slot = heap_by_stage.entry(format!("{:?}", child.stage)).or_insert(0);
        *slot = (*slot).max(here);
        peak_live = peak_live.max(backing.live_bytes);
        match outcome.unwrap() { Ok(done) => terminal = done, Err(fault) => failure = Some(format!("{:?}: {fault:?}", child.stage)) }
    }
    let total = started.elapsed();
    let peak = semio_framework_trace::peak_heap_bytes_on_this_thread() - base;
    eprintln!("[DEBUG] house sliced={sliced} terminal={terminal} turns={turns} wall={total:?} peak_heap={peak} peak_backing_live={peak_live} failure={failure:?}");
    let mut rows: Vec<_> = stages.iter().collect();
    rows.sort_by_key(|(_, (_, micros, _))| std::cmp::Reverse(*micros));
    eprintln!("[DEBUG] pcg last progress (iteration,max,residual,tol,converged)={pcg_seen:?} free_order_nodes={}", doc.nodes.len());
    for (stage, bytes) in &heap_by_stage { if *bytes > 2_000_000 { eprintln!("[DEBUG] heap after stage {stage}: {bytes}"); } }
    for (stage, (count, micros, fuel)) in rows { eprintln!("[DEBUG] stage {stage}: turns={count} us={micros} fuel={fuel}"); }
    let closing = std::time::Instant::now();
    let mut close_turns = 0usize;
    for _ in 0..30_000_000 { close_turns += 1; if close_with(&mut caller, &mut child, TEST_GRANT).0 { break; } }
    eprintln!("[DEBUG] full-solve close turns={close_turns} wall={:?}", closing.elapsed());
    caller.retire(bank);
}

/// [DEBUG] 🔬️ Closes a House solve stopped at an env-chosen stage and reports the close turns per lane.
#[test]
fn zz_debug_house_close_profile() {
    let text = include_str!("../../../../../🖼️assets/🏠️house/🗣️.dsl.semio");
    let stop = std::env::var("STOP_STAGE").unwrap_or_default();
    let doc = Doc::new(semio_s_artifact_fem_3d_model::standards::v1::subsets::any::io::text::snapshot::parse_dsl(text).unwrap());
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let bank = semio_s_fem_receiving::testkit::numerical_stock();
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
    let mut terminal = false;
    let mut turns = 0usize;
    while !terminal && format!("{:?}", child.stage) != stop && turns < 5_000_000 {
        turns += 1;
        let mut outcome = None;
        caller.turn(TEST_GRANT, |cx| { outcome = Some(child.step(&doc, &mut fields, &mut backing, freshness(19), operation, cx)); Ok(()) }).expect("turn");
        terminal = matches!(outcome.unwrap(), Ok(true));
    }
    let live_before = backing.live_bytes;
    let heap_before = semio_framework_trace::retained_heap_bytes_on_this_thread();
    let mut lanes: std::collections::BTreeMap<u8, (usize, u128)> = Default::default();
    let started = std::time::Instant::now();
    let mut close_turns = 0usize;
    for _ in 0..20_000_000 {
        close_turns += 1;
        let lane = child.close_lane;
        let tick = std::time::Instant::now();
        let done = close_with(&mut caller, &mut child, TEST_GRANT).0;
        let entry = lanes.entry(lane).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += tick.elapsed().as_micros();
        if done { break; }
    }
    eprintln!("[DEBUG] stop={stop:?} solve_turns={turns} terminal={terminal} backing_live_before_close={live_before} heap_before_close={heap_before} close_turns={close_turns} close_wall={:?}", started.elapsed());
    for (lane, (count, micros)) in lanes { eprintln!("[DEBUG] close lane {lane}: turns={count} us={micros}"); }
    caller.retire(bank);
}
