//! 🚪️ Drives the REAL guest-reactor instance close of every wfc editor: boot, accumulate a session
//! by round-tripping the instance's own document text through the history ledger, then `InstanceClose`
//! and hold the close ladder to a bounded `Retired` receipt.
//!
//! Each close turn is one browser worker round trip, so a ladder whose turn count grows with the
//! retained session is a role switch whose wall time grows with it — the exact regression
//! `📓️close-ladder-2026-09-10.md` measured on procedural.
use semio_framework_os_kernel as store;
use semio_framework_plugin::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, AppInstanceId, Budget, Event};
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, plugin_document_text, plugin_load_document_text, PluginRuntime};

type WfcRuntime = PluginRuntime<semio_s_plugin_wfc::WfcApps>;

/// 🗃️ Every editor this owner ships, by `(label, app id)` — the close law holds for all five.
const WFC_EDITORS: [(&str, &str); 5] = [
    ("bitmap", "s.wfc.bitmap@1/*#editor"),
    ("grid2d", "s.wfc.grid2d@1/*#editor"),
    ("wfc2d", "s.wfc.wfc2d@1/*#editor"),
    ("grid3d", "s.wfc.grid3d@1/*#editor"),
    ("wfc3d", "s.wfc.wfc3d@1/*#editor"),
];

const CLOSE_INSTANCE: u32 = 7;
const CLOSE_TURN_BUDGET: usize = 16_384;

/// 🧵️ Every law in this file owns an OS thread with an explicit stack: the guest reactor's registries
/// are `thread_local!`, so one runtime per thread is the only way two laws can run at once, and the
/// composed boot/replay/close future is far larger than a test harness thread's default stack.
const CLOSE_LADDER_STACK_BYTES: usize = 256 * 1024 * 1024;

fn close_budget() -> Budget {
    Budget { fuel: 1_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn open_event(app_id: &str) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 1, instance_id: CLOSE_INSTANCE, request_sequence: 1 },
        app_id: AppInstanceId(app_id.into()),
        actor: "wfc#close".into(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }
}

async fn turn(runtime: &WfcRuntime, events: Vec<Event>) -> semio_framework_plugin::kernel::TurnResult {
    semio_framework_plugin::reactor::poll_kernel(runtime, events, None, None, close_budget()).await.expect("wfc close-ladder turn")
}

async fn acknowledge(runtime: &WfcRuntime, receipt: ActorInstanceLifecycleReceipt) {
    turn(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await;
}

/// 🐣️ Boots one wfc editor and returns its acknowledged lifetime.
async fn boot(app_id: &str) -> (WfcRuntime, ActorInstanceLifetime) {
    let runtime = WfcRuntime::new();
    install_plugin_bundle_result(&runtime, semio_s_plugin_wfc::plugin());
    let opened = turn(&runtime, vec![open_event(app_id)]).await;
    let captured = opened.lifecycle_receipt.expect("open publishes Captured");
    let ActorInstanceLifecycleReceipt::Captured { lifetime, .. } = captured else { panic!("open must publish Captured") };
    acknowledge(&runtime, captured).await;
    (runtime, lifetime)
}

/// 📄️ Reads the instance's own document text once the live maintenance pump has let go of it — the
/// pump runs on the maintenance worker lane natively, so a read issued in the same breath as the
/// previous load answers `instance busy or poisoned` rather than a document.
async fn settled_document_text(runtime: &WfcRuntime) -> store::ArtifactTextFiles {
    for _ in 0..256 {
        turn(runtime, Vec::new()).await;
        if let Ok(text) = plugin_document_text(runtime, CLOSE_INSTANCE).await {
            return text;
        }
    }
    panic!("the booted instance never answered its own document text")
}

/// 🏋️ Replays a realistic editor session: `loads` document loads, each followed by `turns_per_load`
/// reactor turns, so the retained window transients, the history ledger and the engagement frames all
/// grow with the session length.
async fn replay_session(runtime: &WfcRuntime, loads: usize, turns_per_load: usize) {
    for _ in 0..loads {
        let text = settled_document_text(runtime).await;
        plugin_load_document_text(runtime, CLOSE_INSTANCE, &text).await.expect("the instance reloads its own document");
        for _ in 0..turns_per_load {
            turn(runtime, Vec::new()).await;
        }
    }
}

/// 🚪️ Opens the exact close and drives empty turns until the terminal receipt arrives.
async fn close_to_retired(runtime: &WfcRuntime, lifetime: ActorInstanceLifetime, label: &str) {
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

/// 🚪️ Drives the exact close and answers how many reactor turns the ladder spent reaching `Retired`.
async fn close_turn_cost(runtime: &WfcRuntime, lifetime: ActorInstanceLifetime, label: &str, budget: usize) -> usize {
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

/// 🧵️ One runtime per OS thread — see [`CLOSE_LADDER_STACK_BYTES`].
fn on_close_ladder_thread<T: Send + 'static, F: std::future::Future<Output = T>>(label: &str, body: impl FnOnce() -> F + Send + 'static) -> T {
    std::thread::Builder::new()
        .name(format!("close-ladder-{label}"))
        .stack_size(CLOSE_LADDER_STACK_BYTES)
        .spawn(move || semio_framework_async::block_on(body()))
        .expect("close ladder thread")
        .join()
        .expect("close ladder thread completes")
}

/// 🚪️ Drives one editor's whole ladder: boot, four idle turns, close to a bounded `Retired` receipt.
fn editor_close_reaches_retired(index: usize) {
    let (label, app_id) = WFC_EDITORS[index];
    let app_id = app_id.to_string();
    let owned_label = label.to_string();
    on_close_ladder_thread(label, move || async move {
        let (runtime, lifetime) = boot(&app_id).await;
        for _ in 0..4 {
            turn(&runtime, Vec::new()).await;
        }
        close_to_retired(&runtime, lifetime, &owned_label).await;
    });
}

/// 🚪️ LAW: every wfc editor instance is driven to `Retired` by the close ladder inside a bounded turn
/// count. One law PER ARTIFACT, not one loop over five: a cleanup fault raised inside the guest step
/// aborts the whole test binary (`panic in a destructor during cleanup`), so a single looping law
/// reports only whichever artifact happens to run first and hides the other four.
macro_rules! wfc_close_ladder_law {
    ($name:ident, $index:expr) => {
        #[test]
        fn $name() {
            editor_close_reaches_retired($index);
        }
    };
}

wfc_close_ladder_law!(bitmap_editor_instance_close_reaches_retired, 0);
wfc_close_ladder_law!(grid2d_editor_instance_close_reaches_retired, 1);
wfc_close_ladder_law!(wfc2d_editor_instance_close_reaches_retired, 2);
wfc_close_ladder_law!(grid3d_editor_instance_close_reaches_retired, 3);
wfc_close_ladder_law!(wfc3d_editor_instance_close_reaches_retired, 4);

/// 👁️ Every viewer this owner ships, by `(label, app id)`. A viewer owns the very same document,
/// config, presence and transient stores its editor does and releases them through the same ladder,
/// so the owned-store declaration that made the editors close is a law for the read-only surfaces too.
const WFC_VIEWERS: [(&str, &str); 5] = [
    ("bitmap-viewer", "s.wfc.bitmap@1/*#viewer"),
    ("grid2d-viewer", "s.wfc.grid2d@1/*#viewer"),
    ("wfc2d-viewer", "s.wfc.wfc2d@1/*#viewer"),
    ("grid3d-viewer", "s.wfc.grid3d@1/*#viewer"),
    ("wfc3d-viewer", "s.wfc.wfc3d@1/*#viewer"),
];

/// 🚪️ Drives one viewer's whole ladder: boot, four idle turns, close to a bounded `Retired` receipt.
fn viewer_close_reaches_retired(index: usize) {
    let (label, app_id) = WFC_VIEWERS[index];
    let app_id = app_id.to_string();
    let owned_label = label.to_string();
    on_close_ladder_thread(label, move || async move {
        let (runtime, lifetime) = boot(&app_id).await;
        for _ in 0..4 {
            turn(&runtime, Vec::new()).await;
        }
        close_to_retired(&runtime, lifetime, &owned_label).await;
    });
}

/// 🚪️ LAW: every wfc viewer instance is driven to `Retired` by the close ladder inside a bounded turn
/// count. One law PER ARTIFACT for the same reason the editor laws are split.
macro_rules! wfc_viewer_close_ladder_law {
    ($name:ident, $index:expr) => {
        #[test]
        fn $name() {
            viewer_close_reaches_retired($index);
        }
    };
}

wfc_viewer_close_ladder_law!(bitmap_viewer_instance_close_reaches_retired, 0);
wfc_viewer_close_ladder_law!(grid2d_viewer_instance_close_reaches_retired, 1);
wfc_viewer_close_ladder_law!(wfc2d_viewer_instance_close_reaches_retired, 2);
wfc_viewer_close_ladder_law!(grid3d_viewer_instance_close_reaches_retired, 3);
wfc_viewer_close_ladder_law!(wfc3d_viewer_instance_close_reaches_retired, 4);

/// ⚖️ The language-agnostic close-cost fixture — shared by this law and by the shell-side twin, so
/// the ceiling a close must respect is authored once instead of hard-coded per implementation.
const CLOSE_COST_FIXTURE: &str = include_str!("../../🧫️fixtures/🚪️close-ladder/🔣️.json");

/// 🧵️ One session per OS thread; this twin also carries the session's measured turn count back out.
fn session_close_cost(app: &str, loads: usize, turns_per_load: usize, label: &str, budget: usize) -> usize {
    let app = app.to_string();
    let owned_label = label.to_string();
    on_close_ladder_thread(label, move || async move {
        let (runtime, lifetime) = boot(&app).await;
        replay_session(&runtime, loads, turns_per_load).await;
        close_turn_cost(&runtime, lifetime, &owned_label, budget).await
    })
}

/// ⚖️ LAW: closing a wfc editor costs a BOUNDED number of reactor turns whatever session the instance
/// accumulated. `maximumCloseTurns` is the browser's own close budget and EVERY session must fit the
/// same one; `minimumRetainedWorkDilutionPercent` is the load-INVARIANT half — the dearest session's
/// turns per retained unit must be at least that much cheaper than the cheapest non-cold session's,
/// which is what "does not scale with the session" means once the fixed floor is divided out. A
/// ladder that pages one retained item per turn scores a flat 100%.
#[test]
fn wfc_close_cost_is_independent_of_the_retained_session() {
    let fixture: serde_json::Value = serde_json::from_str(CLOSE_COST_FIXTURE).expect("close-ladder fixture parses");
    let app = fixture["app"].as_str().expect("fixture app");
    let ceiling = fixture["maximumCloseTurns"].as_u64().expect("fixture ceiling") as usize;
    let dilution_percent = fixture["minimumRetainedWorkDilutionPercent"].as_u64().expect("fixture dilution") as u128;
    let budget = fixture["turnBudget"].as_u64().expect("fixture turn budget") as usize;
    let mut costs: Vec<(String, usize, usize)> = Vec::new();
    for session in fixture["sessions"].as_array().expect("fixture sessions") {
        let label = session["label"].as_str().expect("session label").to_string();
        let loads = session["documentLoads"].as_u64().expect("session loads") as usize;
        let turns_per_load = session["turnsPerLoad"].as_u64().expect("session turns") as usize;
        let spent = session_close_cost(app, loads, turns_per_load, &label, budget);
        costs.push((label, spent, loads * turns_per_load));
    }
    eprintln!("[DEBUG] close-cost fixture costs={costs:?} ceiling={ceiling} dilution-percent={dilution_percent}");
    for (label, spent, _) in &costs {
        assert!(*spent <= ceiling, "close after the {label} session spent {spent} turns, ceiling {ceiling}");
    }
    let mut retaining: Vec<&(String, usize, usize)> = costs.iter().filter(|(_, _, work)| *work > 0).collect();
    retaining.sort_by_key(|(_, _, work)| *work);
    let (lean_label, lean_cost, lean_work) = retaining.first().expect("one session retains work");
    let (full_label, full_cost, full_work) = retaining.last().expect("one session retains work");
    assert!(full_work > lean_work, "the fixture must declare one session that retains strictly more than another");
    let measured_percent = (*lean_cost as u128) * (*full_work as u128) * 100 / ((*full_cost as u128) * (*lean_work as u128)).max(1);
    assert!(
        measured_percent >= dilution_percent,
        "close turn count must not scale with the session: {lean_label} spent {lean_cost} turns on {lean_work} retained units, {full_label} spent {full_cost} turns on {full_work} — a dilution of {measured_percent}%, under the required {dilution_percent}%"
    );
}
