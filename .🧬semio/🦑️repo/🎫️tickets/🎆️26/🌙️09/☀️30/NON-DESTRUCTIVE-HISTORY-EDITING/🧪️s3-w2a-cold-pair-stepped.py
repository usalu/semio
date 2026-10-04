"""🧪️ S3-W2A, `📓️api-stepped-document-load.md` §8 (1) guest step 2: a verified cold document pair loads through the stepped,
ACK-owned document archive load (`members: []`) over reactor turns instead of one synchronous `plugin_load_document_pack`
fold. The guest answers `Loading` with the last page's cursor on every turn until the load is terminal, holds its UI patches
back meanwhile, then answers `Applied` (or the fault). One pass over the cold-pair module, the reactor thread-locals,
the plugin_runtime helpers and the turn; nothing is written unless every anchor matches."""

import pathlib
import sys

P = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")

EDITS = {
    P / "⚛️reactor/📥️cold-pair/🦀️.rs": [
        ("""impl Drop for ColdDocumentPairLoad {""",
         """/// 🛬️ The archive operation id a cold pair's stepped document load runs under: the transfer generation with the high
/// bit set, so it never collides with a host-chosen archive load sequence.
pub(crate) const COLD_PAIR_DOCUMENT_LOAD_OPERATION: u64 = 1 << 63;

/// 🛬️ A verified cold pair whose document loads through the stepped document archive load (`📓️api-stepped-document-load.md`
/// §8): the claimed load, the archive operation and the last page's cursor every `Loading` answer carries.
pub(crate) struct ColdPairDocumentLoad {
    pub(crate) load: ColdDocumentPairLoad,
    pub(crate) operation: u64,
    pub(crate) cursor: ColdDocumentPairCursor,
}

impl ColdPairDocumentLoad {
    pub(crate) fn new(load: ColdDocumentPairLoad, transfer_generation: u64, cursor: ColdDocumentPairCursor) -> Self {
        Self { load, operation: COLD_PAIR_DOCUMENT_LOAD_OPERATION | transfer_generation, cursor }
    }

    /// 🧬️ The whole-document archive of the verified pair: its pack and spr, no members.
    pub(crate) fn archive(&self) -> protocol::DocumentArchivePack {
        let files = self.load.files();
        protocol::DocumentArchivePack { parent_pack: files.pack.clone(), parent_spr: files.spr.clone(), members: Vec::new() }
    }
}

impl Drop for ColdDocumentPairLoad {"""),
    ],
    P / "⚛️reactor/🦀️.rs": [
        ("""    static COLD_PAIR_INGRESS: RefCell<cold_pair::ColdDocumentPairIngressRegistry<PLUGIN_REACTOR_INSTANCE_SLOTS>> = RefCell::new(cold_pair::ColdDocumentPairIngressRegistry::new());
}""",
         """    static COLD_PAIR_INGRESS: RefCell<cold_pair::ColdDocumentPairIngressRegistry<PLUGIN_REACTOR_INSTANCE_SLOTS>> = RefCell::new(cold_pair::ColdDocumentPairIngressRegistry::new());
    /// 🛬️ The cold pair whose document still loads through the stepped archive load, stepped once per turn.
    static COLD_PAIR_DOCUMENT_LOAD: RefCell<Option<cold_pair::ColdPairDocumentLoad>> = const { RefCell::new(None) };
}"""),
    ],
    P / "🦀️.rs": [
        ("""    /// 📡️ One inbound backbone delivery's turn output. `document_changed` is true when the delivery""",
         """    /// 🛬️ Admits one whole-document archive load under `operation` on instance `instance_id` (`📓️api-stepped-document-load.md`)
    /// — the in-guest twin of `AppCommand::LoadDocumentArchive`.
    pub async fn plugin_begin_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64, archive: protocol::DocumentArchivePack) -> Result<(), Fault> {
        with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            instance.app.begin_document_archive_load(operation, archive)
        })
        .await
    }

    /// 📊️ Advances one whole-document archive load for at most one poll wall and answers its status — the in-guest twin of
    /// `AppCommand::PollDocumentArchiveLoad`.
    pub async fn plugin_poll_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<protocol::DocumentArchiveLoadStatus, Fault> {
        with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            drive_self_waking_ready(instance.app.poll_document_archive_load(operation))
        })
        .await
    }

    /// 📨️ Releases one terminal whole-document archive load — the in-guest twin of `AppCommand::AcknowledgeDocumentArchiveLoad`.
    pub async fn plugin_acknowledge_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<(), Fault> {
        with_instances_mut(runtime, |list| {
            let mut instance = find_instance(list, instance_id)?;
            instance.app.acknowledge_document_archive_load(operation)
        })
        .await
    }

    /// 📡️ One inbound backbone delivery's turn output. `document_changed` is true when the delivery"""),
    ],
    P / "⚛️reactor/🔄️turn/🦀️.rs": [
        ("""        if matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_)) {
            let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
            let load = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().begin_load(lifetime, transfer_generation, live));
            if let Some(load) = load {
                let result = crate::plugin_runtime::plugin_load_document_pack(runtime, lifetime.instance_id, load.files()).await.map_err(|fault| semio_framework_diagnostic::encode_fault_bytes(&fault));
                let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
                cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(load, result, live));
            } else {
                cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Fault { cursor: terminal_cursor, fault: b"cold-pair.load-admission".to_vec() };
            }
        }
    }""",
         """        if matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_)) {
            let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
            let load = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().begin_load(lifetime, transfer_generation, live));
            match load.map(|load| cold_pair::ColdPairDocumentLoad::new(load, transfer_generation, terminal_cursor)) {
                Some(pending) => match crate::plugin_runtime::plugin_begin_document_archive_load(runtime, lifetime.instance_id, pending.operation, pending.archive()).await {
                    Ok(()) => COLD_PAIR_DOCUMENT_LOAD.with(|slot| *slot.borrow_mut() = Some(pending)),
                    Err(fault) => cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(pending.load, Err(semio_framework_diagnostic::encode_fault_bytes(&fault)), live)),
                },
                None => cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Fault { cursor: terminal_cursor, fault: b"cold-pair.load-admission".to_vec() },
            }
        }
    }
    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {
        cold_pair_ingress = step_cold_pair_document_load(runtime, pending).await;
    }"""),
        ("""    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance)).transpose()?.flatten();
    let (ui_patches, ui_patch_receipt) = fill_turn_patch_page(runtime, turn_patch_budget_bytes(budget))?;""",
         """    let lifecycle_receipt = focus.map(|instance| runtime.guest_lifetimes.borrow_mut().prepare_turn(instance)).transpose()?.flatten();
    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));
    let (ui_patches, ui_patch_receipt) = if cold_pair_loading { (semio_framework::kernel::UiTurnPatches::default(), None) } else { fill_turn_patch_page(runtime, turn_patch_budget_bytes(budget))? };"""),
        ("""    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };""",
         """    let status = if more_work || cold_pair_loading { TurnStatus::MoreWork } else { TurnStatus::Idle };"""),
        ("""fn fill_turn_patch_page<PA: crate::app::PluginApp>(""",
         """/// 🛬️ One turn of a cold pair's stepped document load (`📓️api-stepped-document-load.md` §8): polls its archive operation —
/// `Loading` with the last page's cursor while it runs, the pair `Applied` once it is `Ready`, its fault when it is
/// cancelled or refused (the previous document stays: the archive load leaves zero trace) — and releases the terminal
/// operation. An instance that closed meanwhile ends the pair `not-live`; the closing app retires its load with it.
async fn step_cold_pair_document_load<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, pending: cold_pair::ColdPairDocumentLoad) -> semio_framework::kernel::ColdPairIngressStatus {
    use protocol::DocumentArchiveLoadState;
    use semio_framework::kernel::ColdPairIngressStatus;
    let lifetime = pending.cursor.lifetime;
    let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());
    let finish = |load, result: Result<(), Vec<u8>>| COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().finish_load(load, result, live));
    if live.is_none() {
        return finish(pending.load, Err(b"cold-pair.not-live".to_vec()));
    }
    let status = match crate::plugin_runtime::plugin_poll_document_archive_load(runtime, lifetime.instance_id, pending.operation).await {
        Ok(status) => status,
        Err(fault) => return finish(pending.load, Err(semio_framework_diagnostic::encode_fault_bytes(&fault))),
    };
    if matches!(status.state, DocumentArchiveLoadState::Pending | DocumentArchiveLoadState::Running) {
        let cursor = pending.cursor;
        COLD_PAIR_DOCUMENT_LOAD.with(|slot| *slot.borrow_mut() = Some(pending));
        return ColdPairIngressStatus::Loading(cursor);
    }
    let acknowledged = crate::plugin_runtime::plugin_acknowledge_document_archive_load(runtime, lifetime.instance_id, pending.operation).await;
    let result = match (status.state, acknowledged) {
        (_, Err(fault)) => Err(semio_framework_diagnostic::encode_fault_bytes(&fault)),
        (DocumentArchiveLoadState::Ready, Ok(())) => Ok(()),
        (DocumentArchiveLoadState::Cancelled, Ok(())) => Err(b"cold-pair.load-cancelled".to_vec()),
        (_, Ok(())) => Err(status.fault),
    };
    finish(pending.load, result)
}

fn fill_turn_patch_page<PA: crate::app::PluginApp>("""),
    ],
}


def main():
    staged = {}
    for path, edits in EDITS.items():
        text = path.read_text(encoding="utf-8")
        for old, new in edits:
            if text.count(old) != 1:
                sys.exit(f"{path.name} ({path.parent.name}): anchor count {text.count(old)}: {old[:100]!r}")
            text = text.replace(old, new)
        staged[path] = text
    for path, text in staged.items():
        path.write_text(text, encoding="utf-8")
    print(f"cold-pair stepped load: {sum(len(edits) for edits in EDITS.values())} edits over {len(EDITS)} files")


if __name__ == "__main__":
    main()
