//! 🚪️ Drives the REAL guest-reactor instance close of `generation3d` and `generation2d`: boot, load
//! the bundled `hexagonal-mushroom-column` document, arm the preview evaluation (which mounts the
//! retained `FlowEvalSession`, the per-window transient partitions and the in-flight tessellation
//! table), then `InstanceClose` and hold the close ladder to a bounded `Retired` receipt.
//!
//! Measured 2026-09-10 before the fix: `InstanceClose` published `Accepted` but no `Retired` ever
//! arrived, because `⚛️reactor`'s `step_reactor_close` discarded `cancel_instance_tasks_step`'s own
//! completion witness and looped on a cursor the production executor stops advancing — see
//! `📓️close-ladder-2026-09-10.md`.
use semio_framework_os_kernel as store;
use semio_framework_plugin::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, AppInstanceId, Budget, Event};
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, plugin_document_text, plugin_load_document_text, plugin_render, PluginRuntime};

type ProceduralRuntime = PluginRuntime<semio_s_plugin_procedural::ProceduralApps>;

const GENERATION3D_EDITOR: &str = "s.procedural.generation3d@1/*#editor";
const GENERATION2D_EDITOR: &str = "s.procedural.generation2d@1/*#editor";
const GENERATION3D_PREVIEW_WINDOW: &str = "procedural-preview";
const GENERATION3D_PREVIEW_BODY: &str = "procedural.play.preview";
const CLOSE_INSTANCE: u32 = 7;
const CLOSE_TURN_BUDGET: usize = 16_384;

fn close_budget() -> Budget {
    Budget { fuel: 1_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn open_event(app_id: &str) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 1, instance_id: CLOSE_INSTANCE, request_sequence: 1 },
        app_id: AppInstanceId(app_id.into()),
        actor: "procedural#close".into(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }
}

async fn turn(runtime: &ProceduralRuntime, events: Vec<Event>) -> semio_framework_plugin::kernel::TurnResult {
    semio_framework_plugin::reactor::poll_kernel(runtime, events, None, None, close_budget()).await.expect("procedural close-ladder turn")
}

async fn acknowledge(runtime: &ProceduralRuntime, receipt: ActorInstanceLifecycleReceipt) {
    turn(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await;
}

/// 🐣️ Boots one procedural editor and returns its acknowledged lifetime.
async fn boot(app_id: &str) -> (ProceduralRuntime, ActorInstanceLifetime) {
    let runtime = ProceduralRuntime::new();
    install_plugin_bundle_result(&runtime, semio_s_plugin_procedural::plugin());
    let opened = turn(&runtime, vec![open_event(app_id)]).await;
    let captured = opened.lifecycle_receipt.expect("open publishes Captured");
    let ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must publish Captured") };
    acknowledge(&runtime, captured).await;
    (runtime, lifetime)
}

fn preview_view_state() -> String {
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("procedural-close-preview".into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "procedural-close-preview".into(), window_kind_id: GENERATION3D_PREVIEW_WINDOW.into() }],
        ..Default::default()
    };
    serde_json::to_string(&view).expect("view model json")
}

/// 🍄️ Loads the bundled example and renders its preview so the retained evaluation session, the
/// window-transient partitions and the tessellation request table are all live at close time.
async fn arm_generation3d_preview(runtime: &ProceduralRuntime) {
    let dsl = semio_s_artifact_procedural_generation3d::examples::art_generation3d_hexagonal_mushroom_column::PRIMARY_TEXT;
    let booted = plugin_document_text(runtime, CLOSE_INSTANCE).await.expect("booted generation3d document text");
    plugin_load_document_text(runtime, CLOSE_INSTANCE, &store::ArtifactTextFiles { dsl: dsl.into(), ops: booted.ops }).await.expect("hexagonal-mushroom-column loads into the booted instance");
    let view_state = preview_view_state();
    for _ in 0..4 {
        let _ = plugin_render(runtime, CLOSE_INSTANCE, GENERATION3D_PREVIEW_BODY, &view_state).await;
        turn(runtime, Vec::new()).await;
    }
}

/// 🚪️ Opens the exact close and drives empty turns until the terminal receipt arrives.
async fn close_to_retired(runtime: &ProceduralRuntime, lifetime: ActorInstanceLifetime, label: &str) {
    let request = ActorInstanceCloseRequest { lifetime, request_sequence: 9 };
    let accepted = turn(runtime, vec![Event::InstanceClose(request)]).await.lifecycle_receipt.expect("close publishes Accepted");
    let ActorInstanceLifecycleReceipt::Accepted { close_generation, .. } = accepted else { panic!("{label} close must publish Accepted first") };
    assert!(close_generation > 0);
    acknowledge(runtime, accepted).await;
    let mut retired = None;
    for spent in 0..CLOSE_TURN_BUDGET {
        if let Some(receipt @ ActorInstanceLifecycleReceipt::Retired { .. }) = turn(runtime, Vec::new()).await.lifecycle_receipt {
            eprintln!("[DEBUG] {label} reached Retired after {spent} close turns");
            retired = Some(receipt);
            break;
        }
        std::thread::yield_now();
    }
    let retired = retired.unwrap_or_else(|| panic!("{label} close ladder must reach Retired within {CLOSE_TURN_BUDGET} reactor turns"));
    let ActorInstanceLifecycleReceipt::Retired { close_generation: retired_generation, .. } = retired else { unreachable!() };
    assert_eq!(retired_generation, close_generation, "{label} terminal receipt keeps the admitted close generation");
    acknowledge(runtime, retired).await;
    for _ in 0..8 {
        if turn(runtime, Vec::new()).await.lifecycle_receipt.is_none() {
            return;
        }
        acknowledge(runtime, retired).await;
    }
    panic!("{label} terminal ACK must release the structural owner");
}

/// 🚪️ LAW: a generation3d instance that loaded a real document, evaluated it and still holds
/// in-flight preview work is driven to `Retired` by the close ladder inside a bounded turn count.
#[semio_framework_async_macros::async_test]
async fn generation3d_instance_close_reaches_retired() {
    let (runtime, lifetime) = boot(GENERATION3D_EDITOR).await;
    arm_generation3d_preview(&runtime).await;
    close_to_retired(&runtime, lifetime, "generation3d").await;
}

/// 🚪️ LAW: the same close law holds for generation2d — one ladder, two artifacts.
#[semio_framework_async_macros::async_test]
async fn generation2d_instance_close_reaches_retired() {
    let (runtime, lifetime) = boot(GENERATION2D_EDITOR).await;
    for _ in 0..4 {
        turn(&runtime, Vec::new()).await;
    }
    close_to_retired(&runtime, lifetime, "generation2d").await;
}
