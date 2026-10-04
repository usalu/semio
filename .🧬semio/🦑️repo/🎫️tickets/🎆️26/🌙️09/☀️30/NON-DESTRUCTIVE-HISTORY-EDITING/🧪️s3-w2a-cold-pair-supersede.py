"""🛑️ S3-W2A: a cold-pair page of another transfer cancels the stale in-flight cold-pair document load (W1G review)."""
import pathlib

PLUGIN = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")
ROOT = PLUGIN / "🦀️.rs"
TURN = PLUGIN / "⚛️reactor/🔄️turn/🦀️.rs"

plan = [
    (ROOT,
     "    /// 📨️ Releases one terminal whole-document archive load — the in-guest twin of `AppCommand::AcknowledgeDocumentArchiveLoad`.\n",
     "    /// 🛑️ Requests the cancellation of one running whole-document archive load — the in-guest twin of\n    /// `AppCommand::CancelDocumentArchiveLoad`; the load reaches `Cancelled` over later polls.\n    pub async fn plugin_cancel_document_archive_load<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, operation: u64) -> Result<(), Fault> {\n        with_instances_mut(runtime, |list| {\n            let mut instance = find_instance(list, instance_id)?;\n            instance.app.cancel_document_archive_load(operation)\n        })\n        .await\n    }\n\n    /// 📨️ Releases one terminal whole-document archive load — the in-guest twin of `AppCommand::AcknowledgeDocumentArchiveLoad`.\n"),
    (TURN,
     "    if let Some(page) = cold_pair_page {\n        let lifetime = page.header.lifetime;\n        let transfer_generation = page.header.transfer_generation;\n",
     "    if let Some(page) = cold_pair_page {\n        let lifetime = page.header.lifetime;\n        let transfer_generation = page.header.transfer_generation;\n        supersede_cold_pair_document_load(runtime, lifetime.instance_id, transfer_generation).await;\n"),
    (TURN,
     "async fn step_cold_pair_document_load<PA: crate::app::PluginApp>(",
     "/// 🛑️ A cold-pair page of another transfer for an instance whose earlier pair still loads means the host abandoned that\n/// transfer: its archive load is cancelled so the stale fold never publishes, and the turn's step drives it to\n/// `cold-pair.load-cancelled`. A load that already reached a terminal state refuses the cancel and finishes as it ended.\nasync fn supersede_cold_pair_document_load<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, instance_id: u32, transfer_generation: u64) {\n    let stale = COLD_PAIR_DOCUMENT_LOAD\n        .with(|slot| slot.borrow().as_ref().filter(|pending| pending.cursor.lifetime.instance_id == instance_id && pending.cursor.transfer_generation != transfer_generation).map(|pending| pending.operation));\n    if let Some(operation) = stale {\n        let _ = crate::plugin_runtime::plugin_cancel_document_archive_load(runtime, instance_id, operation).await;\n    }\n}\n\nasync fn step_cold_pair_document_load<PA: crate::app::PluginApp>("),
]

for path, old, new in plan:
    if path.read_text(encoding="utf-8").count(old) != 1:
        raise SystemExit(f"{path.name}: anchor count != 1: {old[:90]!r}")
for path, old, new in plan:
    text = path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        raise SystemExit(f"{path.name}: anchor drifted: {old[:90]!r}")
    path.write_text(text.replace(old, new), encoding="utf-8")
print(f"cold-pair supersede: {len(plan)} edits over 2 files")
