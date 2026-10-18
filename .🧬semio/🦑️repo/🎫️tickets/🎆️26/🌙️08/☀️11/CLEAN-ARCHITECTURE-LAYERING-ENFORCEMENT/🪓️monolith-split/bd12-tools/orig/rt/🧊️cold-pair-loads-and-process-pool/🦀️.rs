//! 🧊️ Cold pair loads and process pool of the reactor turn: the cold pair and restored document load steps, the patch page filler and the process worker pool pump. See [`super`].

use super::*;

/// 🧾️ The page plus the ONE receipt that authorizes it — minted for the instance the batch belongs
/// to, which is single by construction (the pending authority only adds to a batch that already
/// names an instance).
#[expect(clippy::result_large_err, reason = "A refused publication returns its exact patch owner to the reserved pending slot without allocating an error wrapper.")]
/// 🛬️ One turn of a cold pair's stepped document load (`📓️api-stepped-document-load.md` §8): polls its archive operation —
/// `Loading` with the last page's cursor while it runs, the pair `Applied` once it is `Ready`, its fault when it is
/// cancelled or refused (the previous document stays: the archive load leaves zero trace) — and releases the terminal
/// operation. An instance that closed meanwhile ends the pair `not-live`; the closing app retires its load with it.
/// 🛑️ A cold-pair page of a newer transfer for the lifetime whose earlier pair still loads means the host abandoned that
/// transfer: its archive load is cancelled so the stale fold never publishes, and it is marked superseded so its terminal
/// answer is never sent to the newer transfer's turns. The newer pages answer `Backpressure` with the stale cursor until the
/// registry has retired the stale owner (`accept_page`, one bounded step per attempt); the host resends them fresh. A load
/// that already reached a terminal state refuses the cancel and finishes as it ended.
pub(super) async fn supersede_cold_pair_document_load<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, lifetime: semio_framework::kernel::ActorInstanceLifetime, transfer_generation: u64) {
    let stale = COLD_PAIR_DOCUMENT_LOAD.with(|slot| {
        slot.borrow_mut().as_mut().filter(|pending| pending.cursor.lifetime == lifetime && pending.cursor.transfer_generation < transfer_generation && !pending.superseded).map(|pending| {
            pending.superseded = true;
            pending.operation
        })
    });
    if let Some(operation) = stale {
        let _ = semio_framework_plugin_runtime_ops::ops_document::plugin_cancel_document_archive_load(runtime, lifetime.instance_id, operation).await;
    }
}

pub(super) async fn step_cold_pair_document_load<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, pending: cold_pair::ColdPairDocumentLoad, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> semio_framework::kernel::ColdPairIngressStatus {
    use protocol::DocumentArchiveLoadState;
    use semio_framework::kernel::ColdPairIngressStatus;
    let lifetime = pending.cursor.lifetime;
    let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
    let finish = |load, result: Result<(), Vec<u8>>| COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(load, result, live));
    if live.is_none() {
        return finish(pending.load, Err(b"cold-pair.not-live".to_vec()));
    }
    let status = match semio_framework_plugin_runtime_ops::ops_document::plugin_poll_document_archive_load(runtime, lifetime.instance_id, pending.operation, identity, cx).await {
        Ok(status) => status,
        Err(fault) => return finish(pending.load, Err(semio_framework_diagnostic::encode_fault_bytes(&fault))),
    };
    if matches!(status.state, DocumentArchiveLoadState::Pending | DocumentArchiveLoadState::Running) {
        let cursor = pending.cursor;
        COLD_PAIR_DOCUMENT_LOAD.with(|slot| *slot.borrow_mut() = Some(pending));
        return ColdPairIngressStatus::Loading(cursor);
    }
    let acknowledged = semio_framework_plugin_runtime_ops::ops_document::plugin_acknowledge_document_archive_load(runtime, lifetime.instance_id, pending.operation).await;
    let result = match (status.state, acknowledged) {
        (_, Err(fault)) => Err(semio_framework_diagnostic::encode_fault_bytes(&fault)),
        (DocumentArchiveLoadState::Ready, Ok(())) => Ok(()),
        (DocumentArchiveLoadState::Cancelled, Ok(())) => Err(b"cold-pair.load-cancelled".to_vec()),
        (_, Ok(())) => Err(status.fault),
    };
    finish(pending.load, result)
}

/// 📸️ Drives every checkpoint-restored document load by one poll (`📓️api-stepped-document-load.md` §4). A terminal load
/// is acknowledged; a faulted one reaches the instance's shell as a fault, a cancelled one keeps the initial document.
/// A closed instance's load is dropped with it. Answers whether any load still runs.
pub(super) async fn step_restored_document_loads<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
    use protocol::DocumentArchiveLoadState;
    let operation = checkpoint::RESTORE_DOCUMENT_LOAD_OPERATION;
    let loads = RESTORED_DOCUMENT_LOADS.with(|loads| std::mem::take(&mut *loads.borrow_mut()));
    let mut running = Vec::with_capacity(loads.len());
    for instance in loads {
        if INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).is_none()) {
            continue;
        }
        let status = match semio_framework_plugin_runtime_ops::ops_document::plugin_poll_document_archive_load(runtime, instance, operation, identity, cx).await {
            Ok(status) => status,
            Err(fault) => {
                effects.push(shell_fault_effect(instance, &fault));
                continue;
            }
        };
        if matches!(status.state, DocumentArchiveLoadState::Pending | DocumentArchiveLoadState::Running) {
            running.push(instance);
            continue;
        }
        match semio_framework_plugin_runtime_ops::ops_document::plugin_acknowledge_document_archive_load(runtime, instance, operation).await {
            Err(fault) => effects.push(shell_fault_effect(instance, &fault)),
            Ok(()) if status.state == DocumentArchiveLoadState::Fault => effects.push(shell_fault_effect(instance, &semio_framework_diagnostic::decode_fault_bytes(&status.fault))),
            Ok(()) => {}
        }
    }
    let pending = !running.is_empty();
    RESTORED_DOCUMENT_LOADS.with(|loads| loads.borrow_mut().extend(running));
    pending
}

pub(super) fn fill_turn_patch_page<PA: semio_framework_plugin_reactor_app::PluginApp>(runtime: &semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime<PA>, budget_bytes: usize) -> Result<(semio_framework::kernel::UiTurnPatches, Option<ActorUiPatchReceipt>), semio_framework::Fault> {
    let page = take_turn_patch_page(budget_bytes)?;
    let instance = page.iter().next().and_then(|patch| parse_surface_instance(&patch.surface.0));
    let receipt = instance.and_then(|instance| runtime.guest_lifetimes.borrow_mut().next_patch_receipt(instance));
    Ok((page, receipt))
}

#[cfg(target_arch = "wasm32")]
const PROCESS_POOL_PUMPS_PER_TURN: usize = 64;

/// ⏱️ Wall-clock bound on process-pool pumping per reactor turn on wasm.
#[cfg(target_arch = "wasm32")]
const PROCESS_POOL_WALL_MS: u64 = 2;

/// 🏃️ Runs queued process-pool job steps inside this turn on wasm, where the pool has no threads and
/// a step submitted by a mounted worker session (retained commands, framework reserved routes) only
/// executes when the pool is pumped. Before this the only pump was the cooperative-maintenance cadence,
/// so every submitted step waited ~170 reactor turns (measured 2026-09-09). Bounded by
/// [`PROCESS_POOL_PUMPS_PER_TURN`] and [`PROCESS_POOL_WALL_MS`]; returns whether pool work remains.
pub(super) fn pump_process_worker_pool() -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        PROCESS_POOL.with(|pool| {
            if !pool.has_pending_work() {
                return false;
            }
            let deadline = std::time::Instant::now() + std::time::Duration::from_millis(PROCESS_POOL_WALL_MS);
            let mut pumps = 0usize;
            while pumps < PROCESS_POOL_PUMPS_PER_TURN && pool.has_pending_work() && std::time::Instant::now() < deadline {
                pool.pump(semio_framework_job::default_now_ms().unwrap_or(1_000_000 + pumps as u64));
                pumps += 1;
            }
            pool.has_pending_work()
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }
}
