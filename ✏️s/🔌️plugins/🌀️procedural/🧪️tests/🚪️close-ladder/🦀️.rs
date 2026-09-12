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

/// 🧵️ Every law in this file owns an OS thread with an explicit stack: the guest reactor's registries
/// are `thread_local!`, so one runtime per thread is the only way two laws can run at once, and the
/// composed boot/replay/close future is far larger than a test harness thread's default stack.
fn on_close_ladder_thread<F: std::future::Future<Output = ()>>(label: &'static str, body: impl FnOnce() -> F + Send + 'static) {
    std::thread::Builder::new()
        .name(format!("close-ladder-{label}"))
        .stack_size(CLOSE_LADDER_STACK_BYTES)
        .spawn(move || semio_framework_async::block_on(body()))
        .expect("close ladder thread")
        .join()
        .expect("close ladder thread completes");
}

const CLOSE_LADDER_STACK_BYTES: usize = 256 * 1024 * 1024;

/// 🚪️ LAW: a generation3d instance that loaded a real document, evaluated it and still holds
/// in-flight preview work is driven to `Retired` by the close ladder inside a bounded turn count.
#[test]
fn generation3d_instance_close_reaches_retired() {
    on_close_ladder_thread("generation3d", || async {
        let (runtime, lifetime) = boot(GENERATION3D_EDITOR).await;
        arm_generation3d_preview(&runtime).await;
        close_to_retired(&runtime, lifetime, "generation3d").await;
    });
}

/// 🚪️ LAW: the same close law holds for generation2d — one ladder, two artifacts.
#[test]
fn generation2d_instance_close_reaches_retired() {
    on_close_ladder_thread("generation2d", || async {
        let (runtime, lifetime) = boot(GENERATION2D_EDITOR).await;
        for _ in 0..4 {
            turn(&runtime, Vec::new()).await;
        }
        close_to_retired(&runtime, lifetime, "generation2d").await;
    });
}

/// ⚖️ The language-agnostic close-cost fixture — shared by this law and by the shell-side twin, so
/// the ceiling a close must respect is authored once instead of hard-coded per implementation.
const CLOSE_COST_FIXTURE: &str = include_str!("../../🧫️fixtures/🚪️close-ladder/🔣️.json");

/// 📚️ The bundled generation3d examples, in the order a long editor session picks them — each load
/// retains another document, another history frame and another tessellation generation.
fn example_documents() -> Vec<&'static str> {
    use semio_s_artifact_procedural_generation3d::examples as art;
    vec![
        art::art_generation3d_hexagonal_mushroom_column::PRIMARY_TEXT,
        art::art_generation3d_rectangle_extrude_volume::PRIMARY_TEXT,
        art::art_generation3d_sphere_cut_with_torus::PRIMARY_TEXT,
        art::art_generation3d_box_fillet_preview::PRIMARY_TEXT,
        art::art_generation3d_sphere_box_fuse::PRIMARY_TEXT,
        art::art_generation3d_box_shell_preview::PRIMARY_TEXT,
        art::art_generation3d_face_sweep_extrude::PRIMARY_TEXT,
        art::art_generation3d_rectangle_wire_preview::PRIMARY_TEXT,
    ]
}

/// 🏋️ Replays a realistic editor session: `documents` example picks, each followed by
/// `renders_per_document` preview renders, so the retained window transients, the tessellation table,
/// the neural/eval session and the document history all grow with the session length.
async fn replay_session(runtime: &ProceduralRuntime, documents: usize, renders_per_document: usize) {
    let view_state = preview_view_state();
    for dsl in example_documents().into_iter().take(documents) {
        let booted = settled_document_text(runtime).await;
        plugin_load_document_text(runtime, CLOSE_INSTANCE, &store::ArtifactTextFiles { dsl: dsl.into(), ops: booted.ops }).await.expect("session example loads");
        for _ in 0..renders_per_document {
            let _ = plugin_render(runtime, CLOSE_INSTANCE, GENERATION3D_PREVIEW_BODY, &view_state).await;
            turn(runtime, Vec::new()).await;
        }
    }
}

/// 📄️ Reads the instance's own document text once the live maintenance pump has let go of it — the
/// pump runs on the maintenance worker lane natively, so a read issued in the same breath as the
/// previous load answers `instance busy or poisoned` rather than a document.
async fn settled_document_text(runtime: &ProceduralRuntime) -> store::ArtifactTextFiles {
    for _ in 0..256 {
        turn(runtime, Vec::new()).await;
        if let Ok(text) = plugin_document_text(runtime, CLOSE_INSTANCE).await {
            return text;
        }
    }
    panic!("the booted instance never answered its own document text")
}

/// 🚪️ Drives the exact close and answers how many reactor turns the ladder spent reaching `Retired`.
async fn close_turn_cost(runtime: &ProceduralRuntime, lifetime: ActorInstanceLifetime, label: &str, budget: usize) -> usize {
    let request = ActorInstanceCloseRequest { lifetime, request_sequence: 9 };
    let started = std::time::Instant::now();
    let accepted = turn(runtime, vec![Event::InstanceClose(request)]).await.lifecycle_receipt.expect("close publishes Accepted");
    acknowledge(runtime, accepted).await;
    for spent in 0..budget {
        if let Some(receipt @ ActorInstanceLifecycleReceipt::Retired { .. }) = turn(runtime, Vec::new()).await.lifecycle_receipt {
            acknowledge(runtime, receipt).await;
            for _ in 0..8 {
                if turn(runtime, Vec::new()).await.lifecycle_receipt.is_none() {
                    break;
                }
                acknowledge(runtime, receipt).await;
            }
            eprintln!("[DEBUG] close-cost {label} reached Retired after {spent} close turns in {} ms", started.elapsed().as_millis());
            return spent;
        }
        std::thread::yield_now();
    }
    panic!("{label} close ladder must reach Retired within {budget} reactor turns")
}

/// 🧵️ One session per OS thread — see {@link on_close_ladder_thread}; this twin also carries the
/// session's measured turn count back out of the thread.
fn session_close_cost(app: &str, documents: usize, renders: usize, label: &str, budget: usize) -> usize {
    let app = app.to_string();
    let label = label.to_string();
    std::thread::Builder::new()
        .name(format!("close-cost-{label}"))
        .stack_size(CLOSE_LADDER_STACK_BYTES)
        .spawn(move || {
            semio_framework_async::block_on(async move {
                let (runtime, lifetime) = boot(&app).await;
                replay_session(&runtime, documents, renders).await;
                close_turn_cost(&runtime, lifetime, &label, budget).await
            })
        })
        .expect("close-cost session thread")
        .join()
        .expect("close-cost session thread completes")
}

/// ⚖️ LAW: closing a generation3d editor costs a BOUNDED number of reactor turns whatever session the
/// instance accumulated. Each close turn is one browser worker round trip, so a ladder that pages one
/// retained item per turn is an O(retained) wall-clock stall on the ⌘⌥V role switch — measured on 6018
/// as 62–87 s ending in `plugin-ui.lifecycle-close-budget-exhausted`
/// (`🗑️generated/journey-5/console.txt`).
///
/// Before the fix this law read **2 052 turns for every session** — cold, three documents, eight
/// documents alike: the reactor close walked its fixed slot geometry (1 024 request slots + 1 024 task
/// slots) ONE slot per reactor turn, and the app ladder paged one retained window transient per turn on
/// top of it. Both now drain under the turn's own wall-clock budget, so the geometry costs one turn and
/// the retained families cost a bounded handful.
#[test]
fn generation3d_close_cost_is_independent_of_the_retained_session() {
    let fixture: serde_json::Value = serde_json::from_str(CLOSE_COST_FIXTURE).expect("close-ladder fixture parses");
    let app = fixture["app"].as_str().expect("fixture app");
    let ceiling = fixture["maximumCloseTurns"].as_u64().expect("fixture ceiling") as usize;
    let growth = fixture["maximumCloseTurnGrowth"].as_u64().expect("fixture growth") as usize;
    let budget = fixture["turnBudget"].as_u64().expect("fixture turn budget") as usize;
    let mut costs: Vec<(String, usize)> = Vec::new();
    for session in fixture["sessions"].as_array().expect("fixture sessions") {
        let label = session["label"].as_str().expect("session label").to_string();
        let documents = session["documents"].as_u64().expect("session documents") as usize;
        let renders = session["rendersPerDocument"].as_u64().expect("session renders") as usize;
        let spent = session_close_cost(app, documents, renders, &label, budget);
        costs.push((label, spent));
    }
    eprintln!("[DEBUG] close-cost fixture costs={costs:?} ceiling={ceiling} growth={growth}");
    for (label, spent) in &costs {
        assert!(*spent <= ceiling, "close after the {label} session spent {spent} turns, ceiling {ceiling}");
    }
    let low = costs.iter().map(|(_, spent)| *spent).min().expect("one session").max(1);
    let high = costs.iter().map(|(_, spent)| *spent).max().expect("one session");
    assert!(high <= low.saturating_mul(growth), "close turn count must not scale with the session: {costs:?} grew {}x, ceiling {growth}x", high / low);
}
