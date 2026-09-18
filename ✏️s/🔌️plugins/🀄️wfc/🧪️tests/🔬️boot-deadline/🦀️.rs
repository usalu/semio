//! ⏱️ Boots every wfc editor through the real guest reactor lifecycle and holds its first step to the
//! strict time authority (`semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US`). The browser boot
//! that first motivated this law died on exactly this turn (`plugin.reactor-turn-deadline` on
//! `[handler/first-step]`), so the whole bootstrap cost is measured here, phase by phase, instead of
//! inferred from a console.
use semio_framework_plugin::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceOpenRequest, AppInstanceId, Budget, Event, TurnStatus};
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};

type WfcRuntime = PluginRuntime<semio_s_plugin_wfc::WfcApps>;

/// 🗃️ Every editor this owner ships, by `(label, app id)` — the deadline holds for all five.
const WFC_EDITORS: [(&str, &str); 5] = [
    ("bitmap", "s.wfc.bitmap@1/*#editor"),
    ("grid2d", "s.wfc.grid2d@1/*#editor"),
    ("wfc2d", "s.wfc.wfc2d@1/*#editor"),
    ("grid3d", "s.wfc.grid3d@1/*#editor"),
    ("wfc3d", "s.wfc.wfc3d@1/*#editor"),
];

/// 🔢️ One instance id PER editor. The fixed instance-metadata authority is a process-wide registry
/// and `release_without_close` deliberately leaks each runtime, so reusing one id across the five
/// boots answers `plugin.instance-metadata-capacity: saturated or collided` on the second editor.
const BOOT_INSTANCES: [u32; 5] = [1, 2, 3, 4, 5];

fn boot_budget() -> Budget {
    Budget { fuel: 1_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn boot_open(app_id: &str, instance_id: u32) -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 1, instance_id, request_sequence: 1 },
        app_id: AppInstanceId(app_id.into()),
        actor: format!("wfc#{instance_id}"),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }
}

fn now_us() -> u64 {
    semio_framework_job::default_now_us().expect("native microsecond clock")
}

async fn turn(runtime: &WfcRuntime, events: Vec<Event>) -> (semio_framework_plugin::kernel::TurnResult, u64) {
    let started_us = now_us();
    let result = semio_framework_plugin::reactor::poll_kernel(runtime, events, None, None, boot_budget()).await.expect("wfc lifecycle turn");
    (result, now_us() - started_us)
}

async fn acknowledge(runtime: &WfcRuntime, receipt: ActorInstanceLifecycleReceipt) -> u64 {
    turn(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await.1
}

/// 🚪️ Releases the booted runtime WITHOUT opening a close. This law owns the OPEN budget; terminal
/// retirement is a different authority with its own law (`🧪️tests/🚪️close-ladder`), and closing here
/// would make this deadline law red for a close-path defect it does not own.
fn release_without_close(runtime: WfcRuntime) {
    std::mem::forget(runtime);
}

/// ⏱️ LAW: every wfc lifecycle turn of a cold boot fits the strict time authority with a margin, and
/// the descriptor/manifest bootstrap that a host pays once stays out of that budget.
#[semio_framework_async_macros::async_test]
async fn every_wfc_editor_boot_fits_the_strict_lifecycle_turn_authority() {
    let ceiling_us = semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US;
    for (index, (label, app_id)) in WFC_EDITORS.into_iter().enumerate() {
        let boot_instance = BOOT_INSTANCES[index];
        let manifest_started_us = now_us();
        let bundle = semio_s_plugin_wfc::plugin().expect("wfc plugin bundle");
        let manifest_us = now_us() - manifest_started_us;

        let runtime = WfcRuntime::new();
        let install_started_us = now_us();
        install_plugin_bundle_result(&runtime, Ok(bundle));
        let install_us = now_us() - install_started_us;

        let describe_started_us = now_us();
        let descriptor_bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));
        let describe_us = now_us() - describe_started_us;

        let (opened, open_us) = turn(&runtime, vec![boot_open(app_id, boot_instance)]).await;
        let captured = opened.lifecycle_receipt.expect("open publishes Captured");
        let ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } = captured else { panic!("open must publish Captured") };
        assert_eq!(request_sequence, 1);
        assert_eq!(lifetime.instance_id, boot_instance);
        let ack_us = acknowledge(&runtime, captured).await;

        let mut settle_us = Vec::new();
        for _ in 0..64 {
            let (result, spent_us) = turn(&runtime, Vec::new()).await;
            settle_us.push(spent_us);
            if matches!(result.status, TurnStatus::Idle) {
                break;
            }
        }
        let settle_total_us: u64 = settle_us.iter().sum();
        let settle_max_us = settle_us.iter().copied().max().unwrap_or(0);
        eprintln!("[DEBUG] {label} first step manifest_us={manifest_us} install_us={install_us} describe_us={describe_us} descriptor_bytes={} open_us={open_us} ack_us={ack_us} settle_turns={} settle_total_us={settle_total_us} settle_max_us={settle_max_us}", descriptor_bytes.len(), settle_us.len());

        for (phase, spent_us) in [("open", open_us), ("ack", ack_us), ("settle-max", settle_max_us)] {
            assert!(spent_us * 4 < ceiling_us, "{label} {phase} turn spent {spent_us} us and must keep a 4x margin under the {ceiling_us} us strict lifecycle authority");
        }
        release_without_close(runtime);
    }
}
