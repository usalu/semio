//! ⏱️ Boots `generation3d` through the real guest reactor lifecycle and holds its first step to the
//! strict time authority (`semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US`). The browser boot
//! of 2026-09-10 died on exactly this turn (`plugin.reactor-turn-deadline` on `[handler/first-step]`),
//! so the whole bootstrap cost is measured here, phase by phase, instead of inferred from a console.
use semio_framework_plugin::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceOpenRequest, AppInstanceId, Budget, Event, TurnStatus};
use semio_framework_plugin::plugin_runtime::{install_plugin_bundle_result, PluginRuntime};

type ProceduralRuntime = PluginRuntime<semio_s_plugin_procedural::ProceduralApps>;

const GENERATION3D_EDITOR: &str = "s.procedural.generation3d@1/*#editor";
const BOOT_INSTANCE: u32 = 1;

fn boot_budget() -> Budget {
    Budget { fuel: 1_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn boot_open() -> Event {
    Event::InstanceOpen {
        request: ActorInstanceOpenRequest { activation_generation: 1, instance_id: BOOT_INSTANCE, request_sequence: 1 },
        app_id: AppInstanceId(GENERATION3D_EDITOR.into()),
        actor: "procedural#1".into(),
        config: Vec::new(),
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: Default::default(),
    }
}

fn now_us() -> u64 {
    semio_framework_job::default_now_us().expect("native microsecond clock")
}

async fn turn(runtime: &ProceduralRuntime, events: Vec<Event>) -> (semio_framework_plugin::kernel::TurnResult, u64) {
    let started_us = now_us();
    let result = semio_framework_plugin::reactor::poll_kernel(runtime, events, None, None, boot_budget()).await.expect("generation3d lifecycle turn");
    (result, now_us() - started_us)
}

async fn acknowledge(runtime: &ProceduralRuntime, receipt: ActorInstanceLifecycleReceipt) -> u64 {
    turn(runtime, vec![Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })]).await.1
}

/// 🚪️ Releases the booted runtime WITHOUT opening a close. This law owns the OPEN budget; terminal
/// retirement is a different authority with its own law
/// (`⚛️reactor/🚪️lifetime`'s `reactor_native_lifecycle_retains_exact_close_until_ack`), and
/// generation3d does not reach `Retired` today — measured 2026-09-10: `InstanceClose` publishes
/// `Accepted` (generation 1, ACK in 5 902 us) but no `Retired` receipt arrives inside 16 384 reactor
/// turns (~5 min), after which `NativeLifecycleRegistry`'s and `ColdDocumentPairIngressRegistry`'s
/// drop authorities abort the process. Closing here would make this deadline law red for a close-path
/// defect it does not own; see `📓️first-step-deadline-2026-09-10.md` §7.
fn release_without_close(runtime: ProceduralRuntime) {
    std::mem::forget(runtime);
}

/// ⏱️ LAW: every generation3d lifecycle turn of a cold boot fits the strict time authority with a
/// margin, and the descriptor/manifest bootstrap that a host pays once stays out of that budget.
#[semio_framework_async_macros::async_test]
async fn generation3d_boot_fits_the_strict_lifecycle_turn_authority() {
    let manifest_started_us = now_us();
    let bundle = semio_s_plugin_procedural::plugin().expect("procedural plugin bundle");
    let manifest_us = now_us() - manifest_started_us;

    let runtime = ProceduralRuntime::new();
    let install_started_us = now_us();
    install_plugin_bundle_result(&runtime, Ok(bundle));
    let install_us = now_us() - install_started_us;

    let describe_started_us = now_us();
    let descriptor_bytes = semio_framework_plugin::app::resolve_ready(semio_framework_plugin::describe::describe_plugin(&runtime));
    let describe_us = now_us() - describe_started_us;

    let (opened, open_us) = turn(&runtime, vec![boot_open()]).await;
    let captured = opened.lifecycle_receipt.expect("open publishes Captured");
    let ActorInstanceLifecycleReceipt::Captured { lifetime, request_sequence } = captured else { panic!("open must publish Captured") };
    assert_eq!(request_sequence, 1);
    assert_eq!(lifetime.instance_id, BOOT_INSTANCE);
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
    eprintln!("[DEBUG] generation3d first step manifest_us={manifest_us} install_us={install_us} describe_us={describe_us} descriptor_bytes={} open_us={open_us} ack_us={ack_us} settle_turns={} settle_total_us={settle_total_us} settle_max_us={settle_max_us}", descriptor_bytes.len(), settle_us.len());

    let ceiling_us = semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US;
    for (phase, spent_us) in [("open", open_us), ("ack", ack_us), ("settle-max", settle_max_us)] {
        assert!(spent_us * 4 < ceiling_us, "generation3d {phase} turn spent {spent_us} us and must keep a 4x margin under the {ceiling_us} us strict lifecycle authority");
    }
    release_without_close(runtime);
}
