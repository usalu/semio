"""🚦️ S3-W2A W2A-4: one cold-pair document load per actor at a time, answered to the page that arrived; a newer transfer of
the same lifetime cancels and silences the stale load and retires its owner in bounded steps before taking page 0."""
import pathlib

RX = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor")
TURN = RX / "🔄️turn/🦀️.rs"
PAIR = RX / "📥️cold-pair/🦀️.rs"

plan = {
    PAIR: [
        ("pub(crate) struct ColdPairDocumentLoad {\n    pub(crate) load: ColdDocumentPairLoad,\n    pub(crate) operation: u64,\n    pub(crate) cursor: ColdDocumentPairCursor,\n}\n",
         "pub(crate) struct ColdPairDocumentLoad {\n    pub(crate) load: ColdDocumentPairLoad,\n    pub(crate) operation: u64,\n    pub(crate) cursor: ColdDocumentPairCursor,\n    pub(crate) superseded: bool,\n}\n"),
        ("        Self { load, operation: COLD_PAIR_DOCUMENT_LOAD_OPERATION | transfer_generation, cursor }\n",
         "        Self { load, operation: COLD_PAIR_DOCUMENT_LOAD_OPERATION | transfer_generation, cursor, superseded: false }\n"),
        ("        let index = Self::slot_index(page.header.lifetime);\n        if self.closes[index].is_some() {\n            return Self::fault(cursor, \"cold-pair.not-live\");\n        }\n",
         "        let index = Self::slot_index(page.header.lifetime);\n        if self.closes[index].is_some() {\n            return Self::fault(cursor, \"cold-pair.not-live\");\n        }\n        let superseded = self.slots[index].as_ref().map(|owner| owner.borrow()).filter(|owner| owner.header.lifetime == page.header.lifetime && owner.header.transfer_generation < page.header.transfer_generation && !matches!(owner.phase, ColdDocumentPairPhase::Loading)).map(|owner| owner.cursor());\n        if let Some(stale) = superseded {\n            if !self.close_step(page.header.lifetime).closed {\n                return ColdPairIngressStatus::Backpressure(stale);\n            }\n        }\n"),
    ],
    TURN: [
        ("        let lifetime = page.header.lifetime;\n        let transfer_generation = page.header.transfer_generation;\n        supersede_cold_pair_document_load(runtime, lifetime.instance_id, transfer_generation).await;\n        let terminal_cursor = page.header.cursor(page.header.page_count.saturating_sub(1));\n        let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());\n        cold_pair_ingress = COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().accept_page(&page, live));\n",
         "        let lifetime = page.header.lifetime;\n        let transfer_generation = page.header.transfer_generation;\n        supersede_cold_pair_document_load(runtime, lifetime, transfer_generation).await;\n        let terminal_cursor = page.header.cursor(page.header.page_count.saturating_sub(1));\n        let live = native_close_key(runtime, lifetime.instance_id).ok().filter(|key| key.lifetime() == lifetime).map(|key| key.lifetime());\n        let another_loads = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().as_ref().is_some_and(|pending| pending.cursor.lifetime != lifetime));\n        cold_pair_ingress = if another_loads && page.page_index.saturating_add(1) == page.header.page_count {\n            semio_framework::kernel::ColdPairIngressStatus::Backpressure(page.header.cursor(page.page_index))\n        } else {\n            COLD_PAIR_INGRESS.with(|ingress| ingress.borrow_mut().accept_page(&page, live))\n        };\n"),
        ("    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {\n        cold_pair_ingress = step_cold_pair_document_load(runtime, pending).await;\n    }\n    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));\n    let document_load_work = step_restored_document_loads(runtime, &mut effects).await || cold_pair_loading;\n",
         "    if let Some(pending) = COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow_mut().take()) {\n        let answers_this_turn = !pending.superseded && cold_pair_page_lifetime.is_none_or(|lifetime| lifetime == pending.cursor.lifetime);\n        let status = step_cold_pair_document_load(runtime, pending).await;\n        if answers_this_turn {\n            cold_pair_ingress = status;\n        }\n    }\n    let cold_pair_loading = matches!(cold_pair_ingress, semio_framework::kernel::ColdPairIngressStatus::Loading(_));\n    let document_load_work = step_restored_document_loads(runtime, &mut effects).await || COLD_PAIR_DOCUMENT_LOAD.with(|slot| slot.borrow().is_some());\n"),
        ("    let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;\n    if let Some(page) = cold_pair_page {\n",
         "    let mut cold_pair_ingress = semio_framework::kernel::ColdPairIngressStatus::Idle;\n    let cold_pair_page_lifetime = cold_pair_page.as_ref().map(|page| page.header.lifetime);\n    if let Some(page) = cold_pair_page {\n"),
        ("/// 🛑️ A cold-pair page of another transfer for an instance whose earlier pair still loads means the host abandoned that\n/// transfer: its archive load is cancelled so the stale fold never publishes, and the turn's step drives it to\n/// `cold-pair.load-cancelled`. A load that already reached a terminal state refuses the cancel and finishes as it ended.\nasync fn supersede_cold_pair_document_load<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, instance_id: u32, transfer_generation: u64) {\n    let stale = COLD_PAIR_DOCUMENT_LOAD\n        .with(|slot| slot.borrow().as_ref().filter(|pending| pending.cursor.lifetime.instance_id == instance_id && pending.cursor.transfer_generation != transfer_generation).map(|pending| pending.operation));\n    if let Some(operation) = stale {\n        let _ = crate::plugin_runtime::plugin_cancel_document_archive_load(runtime, instance_id, operation).await;\n    }\n}\n",
         "/// 🛑️ A cold-pair page of a newer transfer for the lifetime whose earlier pair still loads means the host abandoned that\n/// transfer: its archive load is cancelled so the stale fold never publishes, and it is marked superseded so its terminal\n/// answer is never sent to the newer transfer's turns. The newer pages answer `Backpressure` with the stale cursor until the\n/// registry has retired the stale owner (`accept_page`, one bounded step per attempt); the host resends them fresh. A load\n/// that already reached a terminal state refuses the cancel and finishes as it ended.\nasync fn supersede_cold_pair_document_load<PA: crate::app::PluginApp>(runtime: &crate::plugin_runtime::PluginRuntime<PA>, lifetime: semio_framework::kernel::ActorInstanceLifetime, transfer_generation: u64) {\n    let stale = COLD_PAIR_DOCUMENT_LOAD.with(|slot| {\n        slot.borrow_mut().as_mut().filter(|pending| pending.cursor.lifetime == lifetime && pending.cursor.transfer_generation < transfer_generation && !pending.superseded).map(|pending| {\n            pending.superseded = true;\n            pending.operation\n        })\n    });\n    if let Some(operation) = stale {\n        let _ = crate::plugin_runtime::plugin_cancel_document_archive_load(runtime, lifetime.instance_id, operation).await;\n    }\n}\n"),
    ],
}

for path, edits in plan.items():
    text = path.read_text(encoding="utf-8")
    for old, new in edits:
        if text.count(old) != 1:
            raise SystemExit(f"{path.name}: anchor count {text.count(old)}: {old[:110]!r}")
        text = text.replace(old, new)
    plan[path] = text
for path, text in plan.items():
    path.write_text(text, encoding="utf-8")
print("cold-pair serialized: 7 edits over 2 files")
