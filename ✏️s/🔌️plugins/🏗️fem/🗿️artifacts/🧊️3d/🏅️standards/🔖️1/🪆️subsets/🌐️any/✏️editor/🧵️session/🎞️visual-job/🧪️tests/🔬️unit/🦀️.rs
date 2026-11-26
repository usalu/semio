use super::*;
use semio_framework_job::StepBudget;
use semio_framework_job::Generation;
use semio_framework_job::OperationId;
use semio_framework_job::RevisionId;

/// 🗑️ Test owner of a sealed snapshot: the last alias retires through the funded protocol instead of a bare drop.
struct Doc(Option<Fem3dSnapshot>);
impl Doc {
    fn new(snapshot: Fem3dSnapshot) -> Self { Self(Some(snapshot)) }
}
impl Default for Doc {
    fn default() -> Self { Self::new(Fem3dSnapshot::default()) }
}
impl std::ops::Deref for Doc {
    type Target = Fem3dSnapshot;
    fn deref(&self) -> &Fem3dSnapshot { self.0.as_ref().expect("live test snapshot") }
}
impl Drop for Doc {
    fn drop(&mut self) { if let Some(snapshot) = self.0.take() { semio_s_fem_receiving::testkit::retire(snapshot); } }
}

fn release_grant(maximum_release_bytes: usize) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_release_bytes, ..PAGE_GRANT }
}

const PAGE_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY, maximum_depth: 1 };

const TEST_GRANT: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 20, maximum_capacity_bytes: 1 << 20, maximum_release_bytes: 2 << 20, maximum_depth: 128 };

fn close_child(child: &mut Fem3dNumericalChild, grant: RetainedCloneGrant) -> (bool, usize, usize) {
    let bank = RetainedCloneGrant { maximum_items: 1 << 12, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 1 << 12 };
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(semio_framework_job::OperationId(17), Generation(1), bank, || Some(1000), 12345, 64 * 32);
    let (done, progress) = caller.turn(grant, |cx| cx.with_original_retirement(|cx| child.close_step(cx, grant))).expect("original close turn");
    caller.retire(bank);
    (done, progress.copied_items, progress.released_bytes)
}

fn close_with(caller: &mut semio_framework_job::OriginalJobFixtureCaller, child: &mut Fem3dNumericalChild, grant: RetainedCloneGrant) -> (bool, usize, usize) {
    let (done, progress) = caller.turn(grant, |cx| cx.with_original_retirement(|cx| child.close_step(cx, grant))).expect("original close turn");
    (done, progress.copied_items, progress.released_bytes)
}

fn freshness(generation: u64) -> Fem3dVisualFreshness {
    Fem3dVisualFreshness { app_instance_id: 7, model_revision: 11, document_generation: generation, operation: 13, numerical_preview_sequence: 17, surface_generation: generation, renderer_scene_generation: generation }
}

#[test]
fn fem3d_numerical_child_absent_lanes_retain_later_close_owners() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/📦️numerical-close/🔣️.json")).unwrap();
    let mut observations = Vec::new();
    for row in corpus["cases"].as_array().unwrap() {
        let lane = row["lane"].as_u64().unwrap() as u8;
        let mut child = Fem3dNumericalChild::new();
        child.close_lane = lane;
        if row["model"].as_bool().unwrap() {
            let model = child.model.as_mut().unwrap();
            while !model.admit_node_one(1, MOUNTED_ANALYSIS_BACKING_BYTES).unwrap().complete {}
            model.push_node(Node { id: "retained-model-node".into(), pos: [0.0; 3] }).unwrap();
        } else { child.model = None; }
        if row["nodeIds"].as_bool().unwrap() {
            while !child.analysis_node_ids.admit_one(1, MOUNTED_ANALYSIS_BACKING_BYTES).unwrap().0 {}
            child.analysis_node_ids.push("retained-analysis-node".into()).unwrap();
        }
        let zero = close_child(&mut child, release_grant(0));
        let held = child.close_lane == lane && child.model.is_some() == row["model"].as_bool().unwrap() && child.analysis_node_ids.len() == usize::from(row["nodeIds"].as_bool().unwrap());
        let first = close_child(&mut child, release_grant(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES));
        let actual = serde_json::json!({ "lane": child.close_lane, "complete": first.0 });
        for _ in 0..1_024 {
            let step = close_child(&mut child, release_grant(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES));
            if step.0 {
                if child.close_lane < 22 { child.close_lane += 1; }
                else { break; }
            }
        }
        let emptied = child.model.is_none() && child.analysis_node_ids.terminal_is_empty();
        observations.push((lane, zero, held, actual, row["expected"].clone(), emptied));
    }
    for (lane, zero, held, actual, expected, emptied) in observations {
        assert!(emptied, "lane {lane} closes retained test owners");
        assert_eq!(zero, (false, 0, 0), "lane {lane} zero grant");
        assert!(held, "lane {lane} zero grant preserves owner");
        assert_eq!(actual, expected, "lane {lane} absence advances instead of completing");
    }
}

#[test]
fn fem3d_production_numerical_child_solid_reaction_modal_and_close_are_cursorized() {

    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::{FemAxis, FemDof};
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::snapshot::Fem3dSnapshotValue;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemLoadCase;
    use semio_s_fem_material::volumetric::FemMaterialValue;
    use semio_s_fem_node::volumetric::FemNodeValue;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSolid;
    use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::structure::FemSupport;
    use semio_framework_trace::InteractiveStage;
    use semio_framework_trace::StepOverrunLedger;
    use semio_framework_trace::Watchdog;
    use semio_framework_trace::INTERACTIVE_STEP_CEILING_US;

    let doc = Doc::new(Fem3dSnapshot::from_reference(Fem3dSnapshotValue {
        nodes: vec![FemNodeValue { id: "n0".into(), x: 0.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n1".into(), x: 1.0, y: 0.0, z: 0.0 }, FemNodeValue { id: "n2".into(), x: 1.0, y: 1.0, z: 0.0 }, FemNodeValue { id: "n3".into(), x: 0.0, y: 1.0, z: 0.0 }],
        materials: vec![FemMaterialValue { id: "m".into(), name: "M".into(), e: 30e9, g: 12.5e9, nu: 0.2, rho: 2400.0 }],
        solids: vec![FemSolid { id: "s".into(), name: "S".into(), outline: vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]], holes: vec![], base_z: 0.0, height: 0.25, layers: 1, mesh_size: 2.0, material_id: "m".into(), axis: FemAxis::Z }],
        supports: (0..4).map(|index| FemSupport { id: format!("f{index}"), node_id: format!("n{index}"), fixed: vec![FemDof::Tx, FemDof::Ty, FemDof::Tz] }).collect(),
        load_cases: vec![FemLoadCase { id: "g".into(), name: "G".into(), loads: vec![], self_weight: true }],
        ..Default::default()
    }).expect("sealed test snapshot"));
    let operation = semio_framework_job::Operation::new(OperationId(13), RevisionId(11), Generation(19), 23);
    let mut child = Fem3dNumericalChild::new();
    child.request = Fem3dResultRequest { mode: Some(0), ..Fem3dResultRequest::default() };
    child.request.mode = Some(0);
    let mut backing = Fem3dBackingCredit::new();
    let mut fields = Fem3dSolverView::new(freshness(19), doc.nodes.len());
    let bank = semio_s_fem_receiving::testkit::numerical_stock();
    let mut caller = semio_framework_job::OriginalJobFixtureCaller::admit(operation.operation, operation.generation, bank, || Some(1000), 1 << 40, 1 << 40);
    let mut idle = 0usize;
    let mut terminal = false;
    let mut last_stage = "initial";
    let mut overruns = StepOverrunLedger::new();
    let modal_columns = 9usize;
    let maximum_modal_order = 40usize;
    let jacobi_sweep_steps = modal_columns * modal_columns + 1 + modal_columns * (modal_columns - 1) / 2 * (12 * modal_columns + 16) + modal_columns;
    let maximum_turns = 200_000 + 30 * (100 * jacobi_sweep_steps + 32 * maximum_modal_order.pow(2) * modal_columns);
    let mut turns = 0;
    let mut failure = None;
    for _ in 0..maximum_turns {
        turns += 1;
        let mut preview_receipt = RetainedCloneProgress::default();
        let before = (child.stage, child.node_cursor, child.scalar_axis, child.reaction_entry);
        let watchdog = Watchdog::start("fem3d.production-numerical-child", operation.operation, operation.generation, InteractiveStage::UserVisibleSimStep);
        let mut stage = "initial";
        let mut spent = 0;
        let mut outcome = None;
        caller.turn(TEST_GRANT, |cx| { let fuel = cx.fuel_remaining(); outcome = Some(child.step(&doc, &mut fields, &mut backing, freshness(19), operation, cx)); stage = cx.stage(); spent = fuel - cx.fuel_remaining(); Ok(()) }).expect("original numerical turn");
        match outcome.expect("the original numerical turn ran") {
            Ok(complete) => terminal = complete,
            Err(fault) => failure = Some(format!("production numerical child at {:?}/{}: {fault:?}", child.stage, stage)),
        }
        let verdict = watchdog.finish();
        let admission = overruns.admit(&verdict);
        let elapsed_us = verdict.elapsed_us();
        let after = (child.stage, child.node_cursor, child.scalar_axis, child.reaction_entry);
        idle = if spent == 0 && before == after && !terminal { idle + 1 } else { 0 };
        if idle > 1_000 {
            failure.get_or_insert_with(|| format!("numerical unconsumed opportunity before={before:?}, after={after:?}, context {stage}, terminal={terminal}, elapsed_us={elapsed_us:?}"));
        }
        let _ = &mut preview_receipt;
        last_stage = stage;
        if verdict.is_fault() {
            eprintln!("numerical before={before:?}, after={after:?}, context {stage}, terminal={terminal}, elapsed_us={elapsed_us:?}, admission={admission:?}");
        }
        if admission.is_terminal() {
            failure.get_or_insert_with(|| format!("numerical watchdog rejected before={before:?}, after={after:?}, admission={admission:?}"));
        }
        if terminal || failure.is_some() {
            break;
        }
    }
    let progress = (child.stage, child.pcg.as_ref().map(PcgJob::visual_progress), child.subspace.as_ref().map(SubspaceIterationJob::visual_progress));
    let ready = fields.ready();
    let has_static = (0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.displacement != [0.0; 3] || scalar.reaction != [0.0; 3]);
    let has_modal = (0..fields.len).filter_map(|index| fields.scalar(index)).any(|scalar| scalar.eigen_estimate > 0.0 && scalar.mode_shape != [0.0; 3]);
    assert_eq!(close_with(&mut caller, &mut child, release_grant(0)), (false, 0, 0));
    let mut closed = false;
    for _ in 0..200_000 {
        if close_with(&mut caller, &mut child, TEST_GRANT).0 {
            closed = true;
            break;
        }
    }
    eprintln!("FEM3D numerical terminal={terminal}, ready={ready}, static={has_static}, modal={has_modal}, closed={closed}, close_lane={}, close_started_jobs={}, turns={turns}/{maximum_turns}, progress={progress:?}, recorded overruns={}, longest run={}, worst elapsed_us={}", child.close_lane, child.close_started_jobs, overruns.total_overruns(), overruns.longest_overrun_run(), overruns.worst_elapsed_us());
    assert!(closed && child.terminal_is_empty(), "numerical child close did not reach exact terminal: {failure:?}");
    assert!(failure.is_none(), "[TRACE] {failure:?}");
    assert!(terminal, "[TRACE] numerical stopped at {progress:?}/{last_stage}");
    assert!(ready);
    assert!(has_static);
    assert!(has_modal);
    caller.retire(bank);
}
#[test]
fn fem3d_numerical_child_classifies_every_lent_outcome_without_retaining_it() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../../../../🧪️tests/🧫️fixtures/🧒️child-outcomes/🔣️.json")).expect("neutral child outcomes");
    let operation = semio_framework_job::Operation::new(OperationId(991), RevisionId(997), Generation(1009), 0);
    for row in corpus["cases"].as_array().expect("cases") {
        let kind = row["kind"].as_str().expect("kind");
        let end = match kind {
            "preview" | "checkpoint" => ChildEnd::Lent,
            "complete" => ChildEnd::Complete,
            "fault" => ChildEnd::Fault,
            _ => unreachable!(),
        };
        let mut child = Box::new(Fem3dNumericalChild::new());
        let result = child.observe_child_end(end);
        assert_eq!(result.is_err(), kind == "fault");
        if let Ok(complete) = result {
            assert_eq!(complete, kind == "complete");
        }
        assert!(child.observe_child_end(ChildEnd::Cancelled).is_err());
        assert_eq!(child.observe_child_end(ChildEnd::Yield), Ok(false));
        let doc = Doc::default();
        let mut solver = Fem3dSolverView::new(freshness(19), 0);
        let mut backing = Fem3dBackingCredit::new();
        let mut sequence = 0;
        let mut receipt = RetainedCloneProgress::default();
        for budget in [StepBudget::new(0, u64::MAX, TEST_GRANT), StepBudget::new(1, 0, TEST_GRANT)] {
            let mut context = StepContext::new(operation.operation, operation.generation, budget, semio_framework_job::root_cancel_token(), || Some(0), &mut sequence, &mut receipt);
            assert!(matches!(child.step(&doc, &mut solver, &mut backing, freshness(19), operation, &mut context), Ok(false)));
        }
        let cancelled = semio_framework_job::root_cancel_token();
        cancelled.cancel_now();
        let mut context = StepContext::new(operation.operation, operation.generation, StepBudget::new(1, u64::MAX, TEST_GRANT), cancelled, || Some(0), &mut sequence, &mut receipt);
        assert!(child.step(&doc, &mut solver, &mut backing, freshness(19), operation, &mut context).is_err());
        let mut complete = false;
        for _ in 0..256 {
            if close_child(&mut child, release_grant(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES)).0 {
                complete = true;
                break;
            }
        }
        assert!(complete, "all numerical owners close");
    }
}


#[path = "../🎵️modal-buckling/🦀️.rs"]
mod modal_buckling;

#[path = "../⏱️settle/🦀️.rs"]
mod settle;
