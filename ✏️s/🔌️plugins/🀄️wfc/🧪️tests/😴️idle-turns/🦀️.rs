//! 😴️ What the REAL wfc editors do when nothing happens.
//!
//! Boot the guest reactor, let the boot document settle — then stop. From that point on the app is
//! idle, and two things must hold:
//!
//! 1. it answers `TurnStatus::Idle`, because every `MoreWork` is another host turn round trip
//!    (CPU, battery, and the 8 ms interactive ceiling) spent producing nothing, and
//! 2. it retains nothing, because the guest's linear memory is a fixed 512 MiB with no process to
//!    restart — a few kilobytes per turn at the browser's cadence exhausts it in minutes.
//!
//! The regression this law was written against is `📓️idle-turns-2026-09-10.md`: `status=MoreWork
//! effects=0 patches=0` on 512 consecutive turns and 4 437 B/turn of guest linear memory.
use semio_framework_plugin::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceOpenRequest, AppInstanceId, Budget, Event, TurnStatus};
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};

/// 🧮️ Weighs every allocation the idle turns make, so the growth law reads bytes rather than pages.
#[global_allocator]
static IDLE_TURN_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

type WfcRuntime = PluginRuntime<semio_s_plugin_wfc::WfcApps>;

/// 🗃️ Every editor this owner ships, by `(label, app id)` — the idle authority holds for all five.
const WFC_EDITORS: [(&str, &str); 5] = [
    ("bitmap", "s.wfc.bitmap@1/*#editor"),
    ("grid2d", "s.wfc.grid2d@1/*#editor"),
    ("wfc2d", "s.wfc.wfc2d@1/*#editor"),
    ("grid3d", "s.wfc.grid3d@1/*#editor"),
    ("wfc3d", "s.wfc.wfc3d@1/*#editor"),
];

/// 🔢️ One instance id PER editor, plus one for the unacknowledged-open law. The fixed instance
/// metadata authority is a process-wide registry and every law here deliberately leaks its runtime
/// (`mem::forget`, because the drop authority aborts on an unwind), so reusing one id across the
/// five boots answers `plugin.instance-metadata-capacity: saturated or collided` on the second.
const IDLE_INSTANCES: [u32; 5] = [3, 4, 5, 6, 7];
const UNACKNOWLEDGED_INSTANCE: u32 = 8;

/// 🔁️ The idle window the laws measure — the count the browser reaches in a few seconds.
const IDLE_TURNS: usize = 512;

/// ⏳️ Turns the app may still answer `MoreWork` after the last input before it must be settled. The
/// solve chain and the surface reconcile all have to reach terminal in here; they are event-driven
/// and bounded, so this is generous, not a budget to grow into.
const SETTLE_TURN_BUDGET: usize = 512;

/// 📐️ One wasm page — the tolerance the guest-growth law admits over the whole idle window.
const IDLE_GROWTH_CEILING_BYTES: isize = 65_536;

fn idle_budget() -> Budget {
    Budget { fuel: 1_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn open_event(app_id: &str, instance_id: u32) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 1, instance_id, request_sequence: 1 },
        app_id: AppInstanceId(app_id.into()),
        actor: format!("wfc#idle{instance_id}"),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }
}

async fn turn(runtime: &WfcRuntime, events: Vec<Event>) -> semio_framework_plugin::kernel::TurnResult {
    semio_framework_plugin::reactor::poll_kernel(runtime, events, None, None, idle_budget()).await.expect("wfc idle turn")
}

async fn acknowledge(runtime: &WfcRuntime, receipt: ActorInstanceLifecycleReceipt) {
    turn(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await;
}

/// 🐣️ Boots one wfc editor and acknowledges its capture, so no lifecycle receipt is owed.
async fn boot(app_id: &str, instance_id: u32) -> WfcRuntime {
    let runtime = WfcRuntime::new();
    install_plugin_bundle_result(&runtime, semio_s_plugin_wfc::plugin());
    let opened = turn(&runtime, vec![open_event(app_id, instance_id)]).await;
    let captured = opened.lifecycle_receipt.expect("open publishes Captured");
    acknowledge(&runtime, captured).await;
    runtime
}

/// 📋️ What an idle window measured, gathered WITHOUT asserting so the runtime is released before any
/// law fails — `NativeLifecycleRegistry`'s drop authority aborts the process on an unwind.
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
async fn measure_idle(runtime: &WfcRuntime, label: &str) -> IdleReading {
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

/// 😴️ LAW: every booted wfc editor answers `Idle` on each of 512 turns once its boot document
/// settles, and grows the heap by at most one page. One idle authority, five artifacts.
#[semio_framework_async_macros::async_test]
async fn every_wfc_editor_idles_settled_and_retains_nothing() {
    for (index, (label, app_id)) in WFC_EDITORS.into_iter().enumerate() {
        let runtime = boot(app_id, IDLE_INSTANCES[index]).await;
        let reading = measure_idle(&runtime, label).await;
        std::mem::forget(runtime);
        assert_idle(label, reading);
    }
}

/// 🚪️ LAW: a host that has not yet acknowledged the open receipt still costs the guest NOTHING per
/// turn. The receipt is retained state the guest re-offers until the ACK arrives; re-offering it must
/// not allocate, because a host that is slow to acknowledge (or an ACK lost to a connection shortage)
/// would otherwise grow a fixed linear memory without bound.
#[semio_framework_async_macros::async_test]
async fn an_unacknowledged_open_retains_nothing_per_turn() {
    let runtime = WfcRuntime::new();
    install_plugin_bundle_result(&runtime, semio_s_plugin_wfc::plugin());
    turn(&runtime, vec![open_event(WFC_EDITORS[1].1, UNACKNOWLEDGED_INSTANCE)]).await.lifecycle_receipt.expect("open publishes Captured");
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
