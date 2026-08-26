use semio_framework::io::resolve_ready;
use semio_framework_plugin::plugin_app_close_prelude::TypedOperationResultLane;
use semio_framework_plugin::{ActionMeta, App, EditorApp, PluginApp, PluginCloseStep, VcsArtifactApp};
use semio_s_plugin_puzzle::editor::{
    puzzle2d::{Puzzle2dCommand, Puzzle2dPlayApp, create_puzzle2d_app},
    puzzle3d::{Puzzle3dCommand, Puzzle3dPlayApp, create_puzzle3d_app},
    puzzle5d::{Puzzle5dCommand, Puzzle5dPlayApp, create_puzzle5d_app},
};
use std::time::{Duration, Instant};

const MAXIMUM_PUMP_TURNS: usize = 100_000;
const PAGE_BYTES: usize = 4_096;

fn puzzle2d_manifest() -> App {
    App { definition: create_puzzle2d_app(), examples: Vec::new() }
}

fn puzzle3d_manifest() -> App {
    App { definition: create_puzzle3d_app(), examples: Vec::new() }
}

fn puzzle5d_manifest() -> App {
    App { definition: create_puzzle5d_app(), examples: Vec::new() }
}

fn meta() -> ActionMeta {
    ActionMeta { actor: "runtime-harness".into(), instance_id: 1 }
}

fn bind<A: PluginApp>(app: &mut A) {
    resolve_ready(app.bind_instance_id(1));
}

fn drive_to_terminal<A: PluginApp>(app: &mut A) -> (Vec<TypedOperationResultLane>, Duration, usize) {
    let mut lanes = Vec::new();
    let mut maximum_turn = Duration::ZERO;
    for turn in 1..=MAXIMUM_PUMP_TURNS {
        std::thread::yield_now();
        let started = Instant::now();
        let _ = app.maintenance_step(1, PAGE_BYTES).expect("production maintenance step");
        resolve_ready(app.advance_typed_operation_publication()).expect("production publication step");
        maximum_turn = maximum_turn.max(started.elapsed());
        if let Some(page) = app.take_typed_operation_result_page(1) {
            let lane = page.lane;
            let mut wrong = page.token;
            wrong.generation = wrong.generation.wrapping_add(1);
            assert!(!app.acknowledge_typed_operation_result(wrong).expect("wrong-generation ACK is handled"));
            assert!(app.acknowledge_typed_operation_result(page.token).expect("exact ACK is handled"));
            lanes.push(lane);
            if matches!(lane, TypedOperationResultLane::Terminal | TypedOperationResultLane::Fault) {
                return (lanes, maximum_turn, turn);
            }
        }
    }
    panic!("retained operation did not reach a terminal result page");
}

fn close_cancelled<A: PluginApp>(app: &mut A) -> (Duration, usize) {
    let mut maximum_turn = Duration::ZERO;
    for turn in 1..=MAXIMUM_PUMP_TURNS {
        std::thread::yield_now();
        let started = Instant::now();
        let step = app.close_step(1, PAGE_BYTES).expect("production close step");
        maximum_turn = maximum_turn.max(started.elapsed());
        if step == PluginCloseStep::Complete {
            assert!(app.close_terminal_is_empty());
            return (maximum_turn, turn);
        }
    }
    panic!("cancelled retained operation did not close to terminal-empty");
}

fn dispatch_2d(app: &mut VcsArtifactApp<EditorApp<Puzzle2dPlayApp>>) {
    let result = resolve_ready(app.dispatch_typed(Puzzle2dCommand::ForceLayout { window_id: None, args: None }, &meta())).expect("Puzzle2d exact production dispatch");
    assert!(format!("{:?}", result.output).contains("operationId"));
}

fn dispatch_3d(app: &mut VcsArtifactApp<EditorApp<Puzzle3dPlayApp>>) {
    let result = resolve_ready(app.dispatch_typed(
        Puzzle3dCommand::SetGridVisible { window_id: Some("main".into()), args: Some(serde_json::json!({ "pressed": true })) },
        &meta(),
    ))
    .expect("Puzzle3d exact production dispatch");
    assert!(format!("{:?}", result.output).contains("operationId"));
}

fn dispatch_5d(app: &mut VcsArtifactApp<EditorApp<Puzzle5dPlayApp>>) {
    let result = resolve_ready(app.dispatch_typed(
        Puzzle5dCommand::SetGridSnapEnabled { window_id: Some("board2d".into()), args: Some(serde_json::json!({ "enabled": true })) },
        &meta(),
    ))
    .expect("Puzzle5d exact production dispatch");
    assert!(format!("{:?}", result.output).contains("operationId"));
}

fn main() {
    let mut complete_2d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest));
    bind(&mut complete_2d);
    dispatch_2d(&mut complete_2d);
    let complete_2d_result = drive_to_terminal(&mut complete_2d);

    let mut replay_2d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest));
    bind(&mut replay_2d);
    dispatch_2d(&mut replay_2d);
    let replay_2d_result = drive_to_terminal(&mut replay_2d);
    assert_eq!(complete_2d_result.0, replay_2d_result.0, "Puzzle2d replay lane sequence");

    let mut cancel_2d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle2dPlayApp>>(puzzle2d_manifest));
    bind(&mut cancel_2d);
    dispatch_2d(&mut cancel_2d);
    let cancel_2d_result = close_cancelled(&mut cancel_2d);

    let mut complete_3d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest));
    bind(&mut complete_3d);
    dispatch_3d(&mut complete_3d);
    let complete_3d_result = drive_to_terminal(&mut complete_3d);

    let mut cancel_3d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle3dPlayApp>>(puzzle3d_manifest));
    bind(&mut cancel_3d);
    dispatch_3d(&mut cancel_3d);
    let cancel_3d_result = close_cancelled(&mut cancel_3d);

    let mut complete_5d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle5dPlayApp>>(puzzle5d_manifest));
    bind(&mut complete_5d);
    dispatch_5d(&mut complete_5d);
    let complete_5d_result = drive_to_terminal(&mut complete_5d);

    let mut cancel_5d = resolve_ready(semio_framework_plugin::testkit::new_app_with_registry::<EditorApp<Puzzle5dPlayApp>>(puzzle5d_manifest));
    bind(&mut cancel_5d);
    dispatch_5d(&mut cancel_5d);
    let cancel_5d_result = close_cancelled(&mut cancel_5d);

    println!("[DEBUG] puzzle2d-complete lanes={:?} max_host_turn_us={} turns={}", complete_2d_result.0, complete_2d_result.1.as_micros(), complete_2d_result.2);
    println!("[DEBUG] puzzle2d-replay lanes={:?} max_host_turn_us={} turns={}", replay_2d_result.0, replay_2d_result.1.as_micros(), replay_2d_result.2);
    println!("[DEBUG] puzzle2d-cancel-close max_close_turn_us={} turns={}", cancel_2d_result.0.as_micros(), cancel_2d_result.1);
    println!("[DEBUG] puzzle3d-complete lanes={:?} max_host_turn_us={} turns={}", complete_3d_result.0, complete_3d_result.1.as_micros(), complete_3d_result.2);
    println!("[DEBUG] puzzle3d-cancel-close max_close_turn_us={} turns={}", cancel_3d_result.0.as_micros(), cancel_3d_result.1);
    println!("[DEBUG] puzzle5d-complete lanes={:?} max_host_turn_us={} turns={}", complete_5d_result.0, complete_5d_result.1.as_micros(), complete_5d_result.2);
    println!("[DEBUG] puzzle5d-cancel-close max_close_turn_us={} turns={}", cancel_5d_result.0.as_micros(), cancel_5d_result.1);
}
