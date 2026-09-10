//! 😴️ What the REAL `generation3d` editor does when nothing happens.
//!
//! Boot the guest reactor, load the bundled `hexagonal-mushroom-column` document, render its
//! preview once — then stop. From that point on the app is idle, and two things must hold:
//!
//! 1. it answers `TurnStatus::Idle`, because every `MoreWork` is another host turn round trip
//!    (CPU, battery, and the 8 ms interactive ceiling) spent producing nothing, and
//! 2. it retains nothing, because the guest's linear memory is a fixed 512 MiB with no process to
//!    restart — 4 437 B per turn at the browser's cadence exhausts it in minutes.
//!
//! Measured 2026-09-10 before the fix: `status=MoreWork effects=0 patches=0` on 512 consecutive
//! turns and 4 437 B/turn of guest linear memory. See `📓️idle-turns-2026-09-10.md`.
use semio_framework_os_kernel as store;
use semio_framework_plugin::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceOpenRequest, AppInstanceId, Budget, Event, TurnStatus};
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, plugin_document_text, plugin_load_document_text, plugin_render, PluginRuntime};

/// 🧮️ Weighs every allocation the idle turns make, so the growth law reads bytes rather than pages.
#[global_allocator]
static IDLE_TURN_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

type ProceduralRuntime = PluginRuntime<semio_s_plugin_procedural::ProceduralApps>;

const GENERATION3D_EDITOR: &str = "s.procedural.generation3d@1/*#editor";
const GENERATION2D_EDITOR: &str = "s.procedural.generation2d@1/*#editor";
const GENERATION3D_PREVIEW_WINDOW: &str = "procedural-preview";
const GENERATION3D_PREVIEW_BODY: &str = "procedural.play.preview";
const IDLE_INSTANCE: u32 = 3;

/// 🔁️ The idle window the laws measure — the count the browser reaches in a few seconds.
const IDLE_TURNS: usize = 512;

/// ⏳️ Turns the app may still answer `MoreWork` after the last input before it must be settled.
/// The eval chain, the tessellation jobs and the surface reconcile all have to reach terminal in
/// here; they are event-driven and bounded, so this is generous, not a budget to grow into.
const SETTLE_TURN_BUDGET: usize = 512;

/// 📐️ One wasm page — the tolerance the guest-growth law admits over the whole idle window.
const IDLE_GROWTH_CEILING_BYTES: isize = 65_536;

fn idle_budget() -> Budget {
    Budget { fuel: 1_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn open_event(app_id: &str) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 1, instance_id: IDLE_INSTANCE, request_sequence: 1 },
        app_id: AppInstanceId(app_id.into()),
        actor: "procedural#idle".into(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }
}

async fn turn(runtime: &ProceduralRuntime, events: Vec<Event>) -> semio_framework_plugin::kernel::TurnResult {
    semio_framework_plugin::reactor::poll_kernel(runtime, events, None, None, idle_budget()).await.expect("procedural idle turn")
}

async fn acknowledge(runtime: &ProceduralRuntime, receipt: ActorInstanceLifecycleReceipt) {
    turn(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await;
}

/// 🐣️ Boots one procedural editor and acknowledges its capture, so no lifecycle receipt is owed.
async fn boot(app_id: &str) -> ProceduralRuntime {
    let runtime = ProceduralRuntime::new();
    install_plugin_bundle_result(&runtime, semio_s_plugin_procedural::plugin());
    let opened = turn(&runtime, vec![open_event(app_id)]).await;
    let captured = opened.lifecycle_receipt.expect("open publishes Captured");
    acknowledge(&runtime, captured).await;
    runtime
}

fn preview_view_state() -> String {
    let view = semio_framework_plugin::ViewModel {
        window_id: Some("procedural-idle-preview".into()),
        window_instances: vec![semio_framework_plugin::ViewWindowInstance { id: "procedural-idle-preview".into(), window_kind_id: GENERATION3D_PREVIEW_WINDOW.into() }],
        ..Default::default()
    };
    serde_json::to_string(&view).expect("view model json")
}

/// 🍄️ Loads the bundled example and renders its preview once, arming the retained evaluation
/// session, the window-transient partitions and the tessellation request table.
async fn arm_example(runtime: &ProceduralRuntime, dsl: &str) {
    let booted = plugin_document_text(runtime, IDLE_INSTANCE).await.expect("booted document text");
    plugin_load_document_text(runtime, IDLE_INSTANCE, &store::ArtifactTextFiles { dsl: dsl.into(), ops: booted.ops }).await.expect("the bundled example loads into the booted instance");
    let view_state = preview_view_state();
    for _ in 0..4 {
        let _ = plugin_render(runtime, IDLE_INSTANCE, GENERATION3D_PREVIEW_BODY, &view_state).await;
        turn(runtime, Vec::new()).await;
    }
}

/// 📋️ What an idle window measured, gathered WITHOUT asserting so the runtime is released before
/// any law fails — `NativeLifecycleRegistry`'s drop authority aborts the process on an unwind.
#[derive(Default)]
struct IdleReading {
    settle_turns: usize,
    settled: bool,
    hot_turns: usize,
    retained: isize,
    produced: usize,
    sources: std::collections::BTreeMap<String, usize>,
}

fn record(sources: &mut std::collections::BTreeMap<String, usize>) -> bool {
    let armed = semio_framework_plugin::reactor::last_turn_more_work_sources();
    let key = match (armed.any(), armed.typed_operation_contended) {
        (true, _) => armed.names().join("+"),
        (false, true) => "settled(contended)".into(),
        (false, false) => "settled".into(),
    };
    *sources.entry(key).or_default() += 1;
    armed.any()
}

/// 😴️ Drives empty turns until the app answers `Idle` `SETTLED_STREAK` times in a row, then measures
/// `IDLE_TURNS` further turns.
async fn measure_idle(runtime: &ProceduralRuntime, label: &str) -> IdleReading {
    const SETTLED_STREAK: usize = 8;
    let mut reading = IdleReading::default();
    let mut streak = 0;
    let mut settle_sources: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for spent in 0..SETTLE_TURN_BUDGET {
        let idle = matches!(turn(runtime, Vec::new()).await.status, TurnStatus::Idle);
        record(&mut settle_sources);
        streak = if idle { streak + 1 } else { 0 };
        if streak == SETTLED_STREAK {
            reading.settle_turns = spent + 1 - SETTLED_STREAK;
            reading.settled = true;
            break;
        }
    }
    eprintln!("[DEBUG] {label} settle: settled={} after {} turns; sources {settle_sources:?}", reading.settled, reading.settle_turns);
    let before = semio_framework_trace::retained_heap_bytes();
    let mut hot_at = Vec::new();
    for index in 0..IDLE_TURNS {
        let result = turn(runtime, Vec::new()).await;
        if record(&mut reading.sources) {
            reading.hot_turns += 1;
            hot_at.push(index);
        }
        reading.produced += result.effects.len() + result.ui_patches.iter().count() + result.presence.len();
    }
    reading.retained = semio_framework_trace::retained_heap_bytes() - before;
    eprintln!("[DEBUG] {label} idle {IDLE_TURNS} turns: hot={} at {hot_at:?} produced={} retained={} B ({} B/turn); sources {:?}", reading.hot_turns, reading.produced, reading.retained, reading.retained / IDLE_TURNS as isize, reading.sources);
    reading
}

fn assert_idle(label: &str, reading: IdleReading) {
    assert!(reading.settled, "{label} never answered Idle in {SETTLE_TURN_BUDGET} turns — MoreWork sources: {:?}", reading.sources);
    assert_eq!(reading.hot_turns, 0, "{label} answered MoreWork on {} of {IDLE_TURNS} idle turns — sources: {:?}; every one is a host turn round trip that produced nothing", reading.hot_turns, reading.sources);
    assert_eq!(reading.produced, 0, "{label} produced {} effects/patches/presence updates on idle turns", reading.produced);
    assert!(reading.retained <= IDLE_GROWTH_CEILING_BYTES, "{label} retained {} B over {IDLE_TURNS} idle turns ({} B/turn) — the guest's linear memory is fixed", reading.retained, reading.retained / IDLE_TURNS as isize);
}

/// 😴️ LAW: a booted generation3d editor that loaded and evaluated a real document answers `Idle`
/// on every one of 512 turns once its eval chain settles, and grows the heap by at most one page.
#[semio_framework_async_macros::async_test]
async fn generation3d_idle_turns_are_settled_and_retain_nothing() {
    let runtime = boot(GENERATION3D_EDITOR).await;
    arm_example(&runtime, semio_s_artifact_procedural_generation3d::examples::art_generation3d_hexagonal_mushroom_column::PRIMARY_TEXT).await;
    let reading = measure_idle(&runtime, "generation3d").await;
    std::mem::forget(runtime);
    assert_idle("generation3d", reading);
}

/// 🚪️ LAW: a host that has not yet acknowledged the open receipt still costs the guest NOTHING per
/// turn. The receipt is retained state the guest re-offers until the ACK arrives; re-offering it
/// must not allocate, because a host that is slow to acknowledge (or an ACK lost to a connection
/// shortage) would otherwise grow a fixed linear memory without bound. This is the exact shape
/// `🖥️host/🧪️tests/🔬️poll-turn-memory` drives against the real `wasm32-wasip2` component.
#[semio_framework_async_macros::async_test]
async fn an_unacknowledged_open_retains_nothing_per_turn() {
    let runtime = ProceduralRuntime::new();
    install_plugin_bundle_result(&runtime, semio_s_plugin_procedural::plugin());
    turn(&runtime, vec![open_event(GENERATION3D_EDITOR)]).await.lifecycle_receipt.expect("open publishes Captured");
    for _ in 0..64 {
        turn(&runtime, Vec::new()).await;
    }
    let before = semio_framework_trace::retained_heap_bytes();
    let mut sources: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for _ in 0..IDLE_TURNS {
        turn(&runtime, Vec::new()).await;
        record(&mut sources);
    }
    let retained = semio_framework_trace::retained_heap_bytes() - before;
    eprintln!("[DEBUG] unacknowledged open, {IDLE_TURNS} turns: retained={retained} B ({} B/turn); sources {sources:?}", retained / IDLE_TURNS as isize);
    std::mem::forget(runtime);
    assert!(retained <= IDLE_GROWTH_CEILING_BYTES, "an unacknowledged open retained {retained} B over {IDLE_TURNS} turns ({} B/turn)", retained / IDLE_TURNS as isize);
}

/// 😴️ LAW: the same holds for generation2d — one idle authority, two artifacts.
#[semio_framework_async_macros::async_test]
async fn generation2d_idle_turns_are_settled_and_retain_nothing() {
    let runtime = boot(GENERATION2D_EDITOR).await;
    let reading = measure_idle(&runtime, "generation2d").await;
    std::mem::forget(runtime);
    assert_idle("generation2d", reading);
}
