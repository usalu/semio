use semio_plugin::{PluginApp, WindowConfigOwner, WindowConfigOwnerRegistry};
use semio_plugin::artifact_app_laws::{OriginalFixtureCaller, OriginalFixtureScope, fixture_renew_turn};
use semio_plugin_window_config::WindowConfigCommandLoad;
use semio_s_artifact_puzzle_5d_v1::editor::puzzle5d::config::Puzzle5dCamera2d;
use semio_s_artifact_puzzle_5d_tests::editor::puzzle5d::window::{Puzzle5dBoardWindowConfigOwner, Puzzle5dWorldWindowConfigOwner};
use semio_s_artifact_puzzle_5d_testkit::{app_with_registry, close_app, dispatch};
use semio_os_kernel::{ActorId, WindowConfigPackEntry};

#[semio_framework_async_macros::async_test]
async fn restore_exact_5d_camera_in_native_window_registry() {
    let stock = OriginalFixtureCaller::event_stock();
    let fuel = OriginalFixtureCaller::turn_fuel();
    let actor = semio_os_kernel::LOCAL_ACTOR_ID;
    let cameras = [
        ("puzzle5d-board-a", Puzzle5dCamera2d { x: 12.5, y: -6.5, zoom: 3.5 }),
        ("puzzle5d-board-b", Puzzle5dCamera2d { x: -42.5, y: 7.5, zoom: 1.5 }),
    ];
    let world = "puzzle5d-world-a";
    let mut source = Box::new(app_with_registry());
    for (id, camera) in &cameras {
        let args = semio_framework_pack_json::json!({"camera":{"x":camera.x,"y":camera.y,"zoom":camera.zoom}});
        dispatch(&mut source, "setCamera2d", Some(&args), Some(id)).unwrap();
    }
    let args = semio_framework_pack_json::json!({"value":0.42});
    dispatch(&mut source, "setSunIntensity", Some(&args), Some(world)).unwrap();
    let packs = source.window_config_packs().await.unwrap();
    assert_eq!(packs.len(), 3);
    let mut entries = Vec::new();
    for pack in packs {
        let (document, history) = pack.source.pair().unwrap();
        let envelope_pack = semio_os_kernel::os_store::serialization::encode_document_pack_bytes(document, history).await;
        entries.push(WindowConfigPackEntry { window_id: pack.window_id, window_kind_id: pack.window_kind_id, envelope_pack });
    }
    close_app(&mut source);

    let mut scope = OriginalFixtureScope::admit();
    let cx = scope.cx();
    let mut reopened = WindowConfigOwnerRegistry::new(ActorId(actor.into()));
    reopened.register::<Puzzle5dBoardWindowConfigOwner>().unwrap();
    reopened.register::<Puzzle5dWorldWindowConfigOwner>().unwrap();
    let reuse_event = std::env::var_os("SEMIO_UI_WINDOW_REUSE_EVENT").is_some();
    let maximum_turns = std::env::var("SEMIO_UI_WINDOW_MAXIMUM_TURNS").ok().map_or_else(OriginalFixtureCaller::maximum_window_restore_turns, |value| value.parse().unwrap());
    for mut entry in entries {
        let id = entry.window_id.clone();
        let mut load = WindowConfigCommandLoad::default();
        if !reuse_event { semio_framework_job::testkit::issue_initiating_event(cx, stock).unwrap(); }
        let mut last_phase = None;
        for turn in 0..maximum_turns {
            fixture_renew_turn(cx, fuel, stock).unwrap();
            let done = load.advance(&mut reopened, &mut entry, cx).unwrap();
            let phase = load.load.as_ref().map(|p| p.phase());
            if phase != last_phase || turn.is_power_of_two() || done {
                eprintln!("[DEBUG] {id} turn={turn} admitted={} closing={} done={} phase={phase:?} total={:?} failure={:?} close_failure={:?}", load.admitted, load.closing, load.done, cx.retained_progress(), load.failure, load.close_failure);
            }
            last_phase = phase;
            if done { break; }
        }
        assert!(load.terminal_is_empty(), "unfinished restore: {id}");
    }
    for (id, camera) in &cameras {
        let restored = reopened.borrow(Puzzle5dBoardWindowConfigOwner::WINDOW_KIND_ID, id).expect("restored board");
        assert_eq!(&restored.get::<Puzzle5dBoardWindowConfigOwner>().unwrap().camera2d, camera);
    }
    let restored = reopened.borrow(Puzzle5dWorldWindowConfigOwner::WINDOW_KIND_ID, world).expect("restored world");
    assert_eq!(restored.get::<Puzzle5dWorldWindowConfigOwner>().unwrap().sun.intensity, 0.42);
    semio_framework_job::testkit::issue_initiating_event(cx, stock).unwrap();
    for _ in 0..OriginalFixtureCaller::maximum_close_turns() {
        if reopened.terminal_is_empty() { break; }
        fixture_renew_turn(cx, fuel, stock).unwrap();
        reopened.close_step_context(stock, cx).unwrap();
    }
    assert!(reopened.terminal_is_empty());
    scope.retire();
}
