"""📸️ S3-W2A: checkpoint restore rides the stepped document archive load (📓️api-stepped-document-load.md §4)."""
import pathlib

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor")
CHECKPOINT = ROOT / "📸️checkpoint/🦀️.rs"
CHECKPOINT_TEST = ROOT / "📸️checkpoint/🧪️tests/🔬️unit/🦀️.rs"
REACTOR = ROOT / "🦀️.rs"
TURN = ROOT / "🔄️turn/🦀️.rs"


def edit(path, old, new, count=1):
    text = path.read_text(encoding="utf-8")
    found = text.count(old)
    if found != count:
        raise SystemExit(f"{path.name}: expected {count} anchor(s), found {found}: {old[:80]!r}")
    path.write_text(text.replace(old, new), encoding="utf-8")


plan = [
    (CHECKPOINT,
     "//! ⚠️ Scope note (reported honestly): this wave ships the pack ENVELOPE — `instances` (id +\n//! app_id + document/config/draft packs via the SAME `plugin_document_pack`/`plugin_load_document_\n//! pack` round trip `AppCommand::LoadDocument`/`ReadDocument` already use), `timers` (id list from",
     "//! ⚠️ Scope note (reported honestly): this wave ships the pack ENVELOPE — `instances` (id +\n//! app_id + the document pack `plugin_document_pack` reads, restored through the stepped document\n//! archive load of `📓️api-stepped-document-load.md` §4), `timers` (id list from"),
    (CHECKPOINT,
     "/// 📸️ Builds the checkpoint pack for every currently-open instance in this actor. `document_pack`\n/// is `store::encode_document_pack_bytes(files.pack, files.spr)` — the SAME wire codec\n/// `AppCommand::LoadDocument`/`ReadDocument` already use for a whole document as one binary blob;\n/// `files.ops` (a derived text mirror, never authoritative) is not carried.",
     "/// 📸️ Builds the checkpoint pack for every currently-open instance in this actor. `document_pack`\n/// is `store::encode_document_pack_bytes(files.pack, files.spr)` — a whole document as one binary\n/// blob; `files.ops` (a derived text mirror, never authoritative) is not carried."),
    (CHECKPOINT,
     "/// 📸️ Restores every instance recorded in `state`, re-creating each and reloading its document\n/// pack — `⚛️reactor::poll`'s caller is responsible for re-arming `timers`/treating\n/// `pending_requests` as stale (design-abi.md §4).\npub async fn restore<PA: crate::app::PluginApp>(runtime: &plugin_runtime::PluginRuntime<PA>, state: &[u8]) -> Result<CheckpointPack, Fault> {",
     "/// 🛬️ The archive operation every restored instance's document load runs under: bit 62 alone, below the cold-pair\n/// namespace (bit 63) and above any host-chosen archive load sequence. One id suffices because each restored instance is\n/// fresh and owns at most one load.\npub(crate) const RESTORE_DOCUMENT_LOAD_OPERATION: u64 = 1 << 62;\n\n/// 📸️ A decoded checkpoint and the instances whose document load it admitted under [`RESTORE_DOCUMENT_LOAD_OPERATION`].\npub struct RestoredCheckpoint {\n    pub pack: CheckpointPack,\n    pub document_loads: Vec<u32>,\n}\n\n/// 📸️ Restores every instance recorded in `state` under its checkpointed id and admits its document pack as a stepped\n/// whole-document archive load (`📓️api-stepped-document-load.md` §4), which `⚛️reactor`'s turn drives to `Ready` while\n/// the instance answers `document.loading`. `⚛️reactor::poll`'s caller is responsible for re-arming\n/// `timers`/treating `pending_requests` as stale (design-abi.md §4).\npub async fn restore<PA: crate::app::PluginApp>(runtime: &plugin_runtime::PluginRuntime<PA>, state: &[u8]) -> Result<RestoredCheckpoint, Fault> {"),
    (CHECKPOINT,
     "    for instance in &pack.instances {\n        let new_id = plugin_runtime::plugin_create_app(runtime, &instance.app_id).await?;\n        if !instance.document_pack.is_empty() {\n            let (doc_pack, spr) =\n                store::decode_document_pack_bytes(&instance.document_pack).await.map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new(\"plugin.checkpoint.decode-document\"), format!(\"{error:?}\")))?;\n            let files = store::ArtifactPackFiles { pack: doc_pack, spr, ops: String::new() };\n            plugin_runtime::plugin_load_document_pack(runtime, new_id, &files).await?;\n        }\n    }\n    Ok(pack)\n}",
     "    let mut document_loads = Vec::with_capacity(pack.instances.len());\n    for instance in &pack.instances {\n        let id = plugin_runtime::plugin_create_app_with_id(runtime, instance.id, &instance.app_id).await?;\n        if !instance.document_pack.is_empty() {\n            let (parent_pack, parent_spr) =\n                store::decode_document_pack_bytes(&instance.document_pack).await.map_err(|error| Fault::new(semio_framework::FaultOrigin::Plugin, semio_framework::FaultCode::new(\"plugin.checkpoint.decode-document\"), format!(\"{error:?}\")))?;\n            plugin_runtime::plugin_begin_document_archive_load(runtime, id, RESTORE_DOCUMENT_LOAD_OPERATION, store::DocumentArchivePack { parent_pack, parent_spr, members: Vec::new() }).await?;\n            document_loads.push(id);\n        }\n    }\n    Ok(RestoredCheckpoint { pack, document_loads })\n}"),
    (CHECKPOINT_TEST,
     "    let pack = restore(&runtime, &bytes).await.expect(\"must decode back\");",
     "    let pack = restore(&runtime, &bytes).await.expect(\"must decode back\").pack;"),
    (REACTOR,
     "    static COLD_PAIR_DOCUMENT_LOAD: RefCell<Option<cold_pair::ColdPairDocumentLoad>> = const { RefCell::new(None) };\n}",
     "    static COLD_PAIR_DOCUMENT_LOAD: RefCell<Option<cold_pair::ColdPairDocumentLoad>> = const { RefCell::new(None) };\n    /// 📸️ Instances whose checkpoint-restored document still loads under `checkpoint::RESTORE_DOCUMENT_LOAD_OPERATION`,\n    /// each stepped once per turn.\n    static RESTORED_DOCUMENT_LOADS: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };\n}"),
    (REACTOR,
     "/// 📸️ `checkpoint::restore` body — re-arms the timer list from the restored pack;",
     "/// 📸️ `checkpoint::restore` body — re-arms the timer list from the restored pack and hands every admitted document load\n/// to the turn, which drives it to `Ready` (`📓️api-stepped-document-load.md` §4);"),
    (REACTOR,
     "    let pack = checkpoint::restore(runtime, state).await?;\n    let instances = pack.instances().await;",
     "    let checkpoint::RestoredCheckpoint { pack, document_loads } = checkpoint::restore(runtime, state).await?;\n    RESTORED_DOCUMENT_LOADS.with(|loads| loads.borrow_mut().extend(document_loads));\n    let instances = pack.instances().await;"),
    (TURN,
     "    /// ♻️ UI owners released during this turn still owe bounded retirement work.\n    pub ui_retirement: bool,\n}",
     "    /// ♻️ UI owners released during this turn still owe bounded retirement work.\n    pub ui_retirement: bool,\n    /// 🛬️ A cold-pair or checkpoint-restored document still loads through the stepped archive load.\n    pub document_load: bool,\n}"),
    (TURN,
     "        lifecycle: false,\n        ui_retirement: false,\n    };",
     "        lifecycle: false,\n        ui_retirement: false,\n        document_load: false,\n    };"),
    (TURN,
     "            (self.ui_retirement, \"ui_retirement\"),\n        ]",
     "            (self.ui_retirement, \"ui_retirement\"),\n            (self.document_load, \"document_load\"),\n        ]"),
    (TURN,
     "    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {\n        cold_pair_ingress = step_cold_pair_document_load(runtime, pending).await;\n    }\n",
     "    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {\n        cold_pair_ingress = step_cold_pair_document_load(runtime, pending).await;\n    }\n    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));\n    let document_load_work = step_restored_document_loads(runtime, &mut effects).await || cold_pair_loading;\n"),
    (TURN,
     "    let more_work = more_work || close_cleanup_work || typed_operation_scan.runnable || reconcile_work || resumes_remain || executor_pending || command_ingress_pending || lifecycle_work || ui_retirement_work;",
     "    let more_work = more_work || close_cleanup_work || typed_operation_scan.runnable || reconcile_work || resumes_remain || executor_pending || command_ingress_pending || lifecycle_work || ui_retirement_work || document_load_work;"),
    (TURN,
     "        lifecycle: lifecycle_work,\n        ui_retirement: ui_retirement_work,\n    });",
     "        lifecycle: lifecycle_work,\n        ui_retirement: ui_retirement_work,\n        document_load: document_load_work,\n    });"),
    (TURN,
     "    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));\n    let (ui_patches, ui_patch_receipt) = if cold_pair_loading",
     "    let (ui_patches, ui_patch_receipt) = if cold_pair_loading"),
    (TURN,
     "    let status = if more_work || cold_pair_loading { TurnStatus::MoreWork } else { TurnStatus::Idle };",
     "    let status = if more_work { TurnStatus::MoreWork } else { TurnStatus::Idle };"),
    (TURN,
     "fn fill_turn_patch_page<PA: crate::app::PluginApp>(",
     "/// 📸️ Drives every checkpoint-restored document load by one poll (`📓️api-stepped-document-load.md` §4). A terminal load\n/// is acknowledged; a faulted one reaches the instance's shell as a fault, a cancelled one keeps the initial document.\n/// A closed instance's load is dropped with it. Answers whether any load still runs.\nasync fn step_restored_document_loads<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, effects: &mut Vec<Effect>) -> bool {\n    use protocol::DocumentArchiveLoadState;\n    let operation = checkpoint::RESTORE_DOCUMENT_LOAD_OPERATION;\n    let loads = RESTORED_DOCUMENT_LOADS.with(|loads| std::mem::take(&mut *loads.borrow_mut()));\n    let mut running = Vec::with_capacity(loads.len());\n    for instance in loads {\n        if INSTANCE_METADATA.with(|metadata| metadata.borrow().get(instance).is_none()) {\n            continue;\n        }\n        let status = match crate::plugin_runtime::plugin_poll_document_archive_load(runtime, instance, operation).await {\n            Ok(status) => status,\n            Err(fault) => {\n                effects.push(shell_fault_effect(instance, &fault));\n                continue;\n            }\n        };\n        if matches!(status.state, DocumentArchiveLoadState::Pending | DocumentArchiveLoadState::Running) {\n            running.push(instance);\n            continue;\n        }\n        match crate::plugin_runtime::plugin_acknowledge_document_archive_load(runtime, instance, operation).await {\n            Err(fault) => effects.push(shell_fault_effect(instance, &fault)),\n            Ok(()) if status.state == DocumentArchiveLoadState::Fault => effects.push(shell_fault_effect(instance, &semio_framework_diagnostic::decode_fault_bytes(&status.fault))),\n            Ok(()) => {}\n        }\n    }\n    let pending = !running.is_empty();\n    RESTORED_DOCUMENT_LOADS.with(|loads| loads.borrow_mut().extend(running));\n    pending\n}\n\nfn fill_turn_patch_page<PA: crate::app::PluginApp>("),
]

for path, old, new in plan:
    text = path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        raise SystemExit(f"{path.name}: anchor count {text.count(old)}: {old[:90]!r}")
for path, old, new in plan:
    edit(path, old, new)
print(f"checkpoint restore stepped: {len(plan)} edits over 4 files")
