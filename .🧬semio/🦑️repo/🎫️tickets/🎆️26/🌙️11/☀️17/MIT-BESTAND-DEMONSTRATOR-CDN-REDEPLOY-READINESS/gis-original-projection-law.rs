use semio_framework_job::{CancelToken, Generation, JobPayloadAuthority, OperationId, OriginalCallerSourceAdmission, OriginalCallerSourceParent, OriginalTurnFailure, StepBudget, StepContext};
use semio_framework_pack_json::{JsonBorrowedWriteCursor, JsonWriteSource};
use semio_framework_value::{DslValue, NativeEncodeControl, Number, RetainedCloneGrant, RetainedCloneProgress, RetirementPollExit, ToValue, ValueError, ValueRefusalKind, retirement::controlled::ControlledRetirement};

const JSON_ACTIONS_PER_TURN: usize = 16;
const STEM: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 2_000_000, maximum_copy_bytes: 256 * 1024 * 1024,
    maximum_capacity_bytes: 8 * 1024 * 1024, maximum_release_bytes: 8 * 1024 * 1024, maximum_depth: 256,
};
const TURN: RetainedCloneGrant = RetainedCloneGrant {
    maximum_items: 16, maximum_copy_bytes: 4096, maximum_capacity_bytes: 1024 * 1024,
    maximum_release_bytes: 1024 * 1024, maximum_depth: 256,
};
fn clock() -> Option<u64> { Some(1000) }
fn stock() -> StepBudget { StepBudget::new(2_000_000, 20_000, STEM) }

fn native_parent() -> OriginalCallerSourceParent {
    let (cancel, _) = CancelToken::admit_root(STEM).result.unwrap().unwrap();
    let (authority, _) = JobPayloadAuthority::admit(OperationId(222), Generation(3), STEM, &mut None).result.unwrap().unwrap();
    let (mut cancel, mut authority) = (Some(cancel), Some(authority));
    let (mut sequence, mut progress) = (0, RetainedCloneProgress::default());
    let mut cx = StepContext::new(OperationId(739), Generation(3), stock(), CancelToken::root_now(), clock, &mut sequence, &mut progress);
    let mut admission = OriginalCallerSourceAdmission::new();
    let mut parent = None;
    cx.with_original_effect(STEM, |_, effect| admission.prepare(&mut authority, &mut cancel, effect)).unwrap().unwrap().0.result.unwrap();
    cx.with_original_effect(STEM, |_, effect| admission.admit_backing(effect)).unwrap().unwrap().0.result.unwrap();
    cx.with_original_effect(STEM, |_, effect| admission.publish(&mut parent, effect)).unwrap().unwrap().0.result.unwrap();
    parent.unwrap()
}

fn run<V: JsonWriteSource>(make: impl Fn() -> V, cancel_at: Option<usize>, maximum: usize) -> (Option<Vec<u8>>, Option<&'static str>) {
    let mut parent = native_parent();
    let mut cursor = JsonBorrowedWriteCursor::new();
    cursor.set_output_ceiling(maximum);
    let mut output = None;
    let mut failure = None;
    let (mut sequence, mut progress) = (0, RetainedCloneProgress::default());
    let mut writer_turns = 0;
    for turn in 0..10000 {
        let before = progress;
        let mut cx = parent.borrow_context(stock(), clock, &mut sequence, &mut progress).unwrap();
        cx.bind_original_clock_binding(19).unwrap();
        cx.with_retained_loan(TURN, |cx| write_borrowed(|| Ok(make()), &mut cursor, &mut output, &mut failure, "oversize", cx)).unwrap();
        let consumed_work = cx.original_work_units_spent();
        drop(cx);
        assert!(consumed_work <= 16);
        assert!(progress.copied_bytes - before.copied_bytes <= 4096);
        writer_turns = turn + 1;
        if output.is_some() || failure.is_some() || cancel_at == Some(writer_turns) { break; }
    }
    assert!(writer_turns < 10000);
    let result = output.clone();
    if cancel_at.is_some() {
        assert!(result.is_none() && failure.is_none());
        assert!(cursor.retained_capacity_bytes().unwrap() != 0);
        parent.cancellation().unwrap().cancel_now();
    }
    let retained = cursor.retained_capacity_bytes().unwrap() + output.as_ref().map_or(0, Vec::capacity);
    let mut close = ControlledRetirement::retain((cursor, output));
    let released_before = progress.released_bytes;
    let reserved_before = progress.retained_capacity_bytes;
    for _ in 0..10000 {
        if close.terminal_is_empty() { break; }
        let mut cx = parent.borrow_context(stock(), clock, &mut sequence, &mut progress).unwrap();
        cx.bind_original_clock_binding(19).unwrap();
        cx.with_original_retirement(|cx| {
            let Some((poll, _)) = cx.with_original_effect(TURN, |_, effect| close.step(TURN.maximum_copy_bytes, effect))? else { return Ok(()) };
            let turn = RetirementPollExit::settled(poll).ok().unwrap();
            semio_framework_job::receive_original_retained_work(cx, turn)?;
            Ok(())
        }).unwrap();
    }
    assert!(close.terminal_is_empty());
    assert_eq!(progress.released_bytes - released_before, retained + progress.retained_capacity_bytes - reserved_before);
    let (mut sequence, mut progress) = (0, RetainedCloneProgress::default());
    let mut cx = StepContext::new(OperationId(745), Generation(3), stock(), CancelToken::root_now(), clock, &mut sequence, &mut progress);
    for _ in 0..10000 {
        if parent.terminal_is_empty() { break; }
        let run = cx.with_original_effect(STEM, |_, effect| parent.close_step(effect)).unwrap().unwrap().0;
        assert!(matches!(run, semio_framework_value::RetainedWorkPoll::Complete(turn) if turn.result.is_ok()));
    }
    assert!(parent.terminal_is_empty());
    (result, failure)
}

fn main() {
    let sample = DslValue::object([("name".into(), DslValue::String("é".repeat(2000))), ("levels".into(), vec![1u64, 2, 30000].to_value())]);
    let (result, failure) = run(|| DslBorrowed(&sample), None, 1_048_576);
    assert_eq!(failure, None);
    assert_eq!(result.unwrap(), semio_framework_pack_json::to_json_string(&sample).as_bytes());
    assert_eq!(run(|| DslBorrowed(&sample), Some(3), 1_048_576), (None, None));
    assert_eq!(run(|| DslBorrowed(&sample), None, 128), (None, Some("oversize")));

    let rows = (0..33).map(|row| (0..33).map(|col| (row * 11 + col - 80) as i16).collect::<Vec<_>>()).collect::<Vec<_>>();
    let bounds: [f64; 4] = [5.49, 50.60, 5.63, 50.68];
    let origin = [bounds[0].midpoint(bounds[2]), bounds[1].midpoint(bounds[3])];
    let projection = grid::TerrainGridMeshProjection::from_bounds(bounds, origin, 2.75);
    let mut range = scene::TerrainSceneRange::default();
    for _ in 0..33 { range.step(&rows).unwrap(); }
    let heights: Vec<f32> = rows.iter().rev().flatten().copied().map(f32::from).collect();
    let expected = baseline::build_terrain_grid_mesh(&heights, bounds, origin[0], origin[1], 2.75);
    let scene = || scene::TerrainSceneView { rows: &rows, projection, range };
    let (result, failure) = run(scene, None, 1_048_576);
    assert_eq!(failure, None);
    let output: serde_json::Value = serde_json::from_slice(&result.unwrap()).unwrap();
    assert_eq!(output["schema"], "mesh.document");
    assert_eq!(output["mesh"]["colors"], serde_json::json!([]));
    for (name, expected) in [("positions", expected.positions), ("normals", expected.normals), ("uvs", expected.uvs)] {
        let independent: serde_json::Value = serde_json::from_slice(&serde_json::to_vec(&expected).unwrap()).unwrap();
        assert_eq!(output["mesh"][name], independent, "{name}");
    }
    assert_eq!(output["mesh"]["indices"], serde_json::json!(expected.indices));

    let map = map_view::GisMapSnapshot {
        positions: vec![map_view::MapFeature { data: DslValue::object([("sample".into(), DslValue::Number(Number::Int(-2))), ("text".into(), sample.clone())]) }],
        routes: vec![], regions: vec![],
    };
    let view = || map_view::MapJsonView(&map);
    let mut validation = map_validation::MapJsonValidation::default();
    while !validation.step(&view()).unwrap() {}
    let (result, failure) = run(view, None, 1_048_576);
    assert_eq!(failure, None);
    let expected = DslValue::object([
        ("positions".into(), DslValue::Array(vec![map.positions[0].data.clone()])),
        ("routes".into(), DslValue::Array(vec![])), ("regions".into(), DslValue::Array(vec![])),
    ]);
    assert_eq!(result.unwrap(), semio_framework_pack_json::to_json_string(&expected).as_bytes());

    let elevation = elevation_view::ElevationSource { bounds, rows };
    let (result, failure) = run(|| elevation_view::ElevationJsonView(&elevation), None, 1_048_576);
    assert_eq!(failure, None);
    let output: serde_json::Value = serde_json::from_slice(&result.unwrap()).unwrap();
    assert_eq!(output, serde_json::json!({"bounds": elevation.bounds, "rows": &elevation.rows}));
    println!("four original-Sourced projections, overflow, cancellation and exact paid close passed");
}

struct DslBorrowed<'a>(&'a DslValue);
impl JsonWriteSource for DslBorrowed<'_> {
    fn node_at_path(&self,path:&[usize])->Result<semio_framework_pack_json::JsonWriteNode<'_>,ValueError>{self.0.node_at_path(path)}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.0.object_key_at_path(path,index)}
}
