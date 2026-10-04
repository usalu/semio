#!/usr/bin/env python3
"""🧹️ S4-LOAD W2A-6 wave 2: the synchronous whole-document loads leave the `PluginApp` surface in ONE atomic write per file
(fleet rule 39): `load_document_pack/text` (trait + `VcsArtifactApp` impl), `plugin_runtime::plugin_load_document_text`, and the
three PureCommand lane hydrations collapse into the §20.8 head-only `hydrate_pure_head` (driven by `drive_self_waking_ready`, no
`resolve_ready` panic path). Docs that named the removed loads name the document archive load.

Every replacement anchor must occur exactly once; a file whose replacements are already applied is left untouched (idempotent).
Usage: `python3 🧪️s4-load-wave2-plugin-surface.py [--check]`.
"""

import pathlib
import sys

ROOT = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin")

PLUGIN = [
    ("        /// 📜️ Text-DSL counterpart to {@link Self::load_document_pack}.\n        async fn load_document_text(&mut self, files: &store::ArtifactTextFiles) -> Result<(), Fault>;\n", ""),
    ("        /// 📦️ Binary-pack counterpart to {@link Self::load_document_text}.\n        async fn load_document_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault>;\n", ""),
    (
        "        /// 📥 Hydrate document lane from host pack bytes (PureCommand / host authority).\n        async fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault>;\n        /// 📥 Hydrate config lane from host pack bytes.\n        async fn hydrate_config_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault>;\n        /// 📥 Hydrate draft lane from host pack bytes.\n        async fn hydrate_draft_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault>;\n",
        "        /// 🫥️ Hydrates a pure command's head-only document (design §20.8): `head` is the HEAD snapshot pack without history;\n        /// empty evaluates on the live document.\n        async fn hydrate_pure_head(&mut self, head: &[u8]) -> Result<(), Fault>;\n",
    ),
    ("        /// 🧮️ Object-safe counterpart to `load_document_pack`, targeting the config store.\n", "        /// 🧮️ Object-safe load of the config store from its binary pack.\n"),
    ("        /// this, one call per child, after its own `load_document_pack`.\n", "        /// this, one call per child, after its own document archive load.\n"),
    ("        /// (`load_document_text`/`load_document_pack`) edits that never passed through `dispatch_emit`.\n", "        /// (document archive load) edits that never passed through `dispatch_emit`.\n"),
    ("    /// asked about: the pair is the authoritative one, `parse_document_pack` replays it exactly as\n    /// `load_document_pack` does, and `into_snapshot` retires", "    /// asked about: the pair is the authoritative one, `parse_document_pack` replays it exactly as\n    /// a document archive load folds it, and `into_snapshot` retires"),
    (
        "        async fn load_document_text(&mut self, files: &store::ArtifactTextFiles) -> Result<(), Fault> {\n            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_text(&files.dsl, &files.ops).await.map_err(|error| error.into_fault())?;\n            let window_reset = self.prepare_document_window_reset()?;\n            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;\n            self.commit_document_window_reset(window_reset);\n            self.cache = None;\n            self.retire_displaced_document_rows();\n            self.time_travel.set_history_unavailable(false);\n            Ok(())\n        }\n\n",
        "",
    ),
    (
        "        async fn load_document_pack(&mut self, files: &store::ArtifactPackFiles) -> Result<(), Fault> {\n            let parsed: store::ParsedDocumentText<A::Snapshot, A::Mutation> = store::parse_document_pack(&files.pack, &files.spr).await.map_err(|error| error.into_fault())?;\n            let window_reset = self.prepare_document_window_reset()?;\n            self.store.reset(loaded_with_app_dialect::<A>(parsed.into_envelope())).await.map_err(|error| error.into_fault())?;\n            self.commit_document_window_reset(window_reset);\n            self.cache = None;\n            self.retire_displaced_document_rows();\n            self.time_travel.set_history_unavailable(false);\n            Ok(())\n        }\n\n",
        "",
    ),
    (
        "        /// 🫥️ A pure command's document lane is the HEAD snapshot without history (`📓️api-stepped-document-load.md` §4):\n        /// it hydrates in O(snapshot) — no history fold, no suspension — and marks the document head-only, so history verbs\n        /// and queries answer `pure.history-unavailable`; a lane carrying history is refused with that code. Empty lanes keep\n        /// the live document.\n        async fn hydrate_document_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {\n            if pack.is_empty() && spr.is_empty() {\n                return Ok(());\n            }\n            if !spr.is_empty() {\n                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new(PURE_HISTORY_UNAVAILABLE_CODE), \"a pure command carries the head snapshot without history; load a document with history through a live instance\"));\n            }\n            let head = <A::Snapshot as ArtifactPack>::decode_pack(pack).map_err(",
        "        /// 🫥️ A pure command's head is the HEAD snapshot without history (design §20.8, `📓️api-stepped-document-load.md` §4):\n        /// it hydrates in O(snapshot) — no history fold — and marks the document head-only, so history verbs and queries answer\n        /// `pure.history-unavailable`. An empty head keeps the live document.\n        async fn hydrate_pure_head(&mut self, head: &[u8]) -> Result<(), Fault> {\n            if head.is_empty() {\n                return Ok(());\n            }\n            let head = <A::Snapshot as ArtifactPack>::decode_pack(head).map_err(",
    ),
    (
        "        async fn hydrate_config_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {\n            if pack.is_empty() && spr.is_empty() {\n                return Ok(());\n            }\n            self.load_config_pack(&store::ArtifactPackFiles { pack: pack.to_vec(), spr: spr.to_vec(), ops: String::new() }).await\n        }\n\n        async fn hydrate_draft_lane(&mut self, pack: &[u8], spr: &[u8]) -> Result<(), Fault> {\n            if pack.is_empty() && spr.is_empty() {\n                return Ok(());\n            }\n            let parsed: store::ParsedDocumentText<A::Draft, A::DraftMutation> = store::parse_document_pack(pack, spr).await.map_err(|error| error.into_fault())?;\n            self.draft_store.reset(parsed.into_envelope()).await.map_err(|error| error.into_fault())?;\n            Ok(())\n        }\n\n",
        "",
    ),
    (
        "                        ::semio_framework_async::poll::resolve_ready(instance.app.hydrate_document_lane(&head, &[]))?;\n",
        "                        drive_self_waking_ready(instance.app.hydrate_pure_head(&head))?;\n",
    ),
    (
        "    /// 📜️ Replaces the instance's document from its text-DSL files ({@link store::ArtifactTextFiles}) — native tooling's\n    /// in-process load; refused while a document backbone is bound.\n    pub async fn plugin_load_document_text<PA: PluginApp>(runtime: &PluginRuntime<PA>, instance_id: u32, files: &store::ArtifactTextFiles) -> Result<(), Fault> {\n        if runtime.document_backbones.try_borrow().map_err(|_| plugin_internal_fault(\"document backbone binding authority is busy\"))?.get(instance_id).is_some_and(|binding| binding.uri.is_some()) {\n            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new(\"plugin.document-backbone.load-while-bound\"), \"retire the exact document backbone before replacing the document and rebind afterward\"));\n        }\n        with_instances_mut(runtime, |list| {\n            let mut instance = find_instance(list, instance_id)?;\n            ::semio_framework_async::poll::resolve_ready(instance.app.load_document_text(files))\n        })\n        .await\n    }\n\n",
        "",
    ),
]

CONTRACT = [
    ("        PluginApp::load_document_pack(&mut reloaded, &parent_pack).await.expect(\"load parent document pack\");\n", "        artifact_app_laws::load_document(&mut reloaded, &parent_pack).await.expect(\"load parent document pack\");\n"),
]

FILES = [(ROOT / "🦀️.rs", PLUGIN), (ROOT / "🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs", CONTRACT)]


def apply(path, replacements, check):
    text = path.read_text(encoding="utf-8")
    pending = [(old, new) for old, new in replacements if not (new and new in text and old not in text) and not (not new and old not in text)]
    for old, _ in pending:
        if text.count(old) != 1:
            raise SystemExit(f"{path.name}: anchor occurs {text.count(old)}x: {old[:100]!r}")
    if check:
        return len(pending)
    for old, new in pending:
        text = text.replace(old, new)
    if pending:
        path.write_text(text, encoding="utf-8")
    return len(pending)


def main():
    check = "--check" in sys.argv
    total = 0
    for path, replacements in FILES:
        count = apply(path, replacements, check)
        total += count
        print(f"{path.relative_to(ROOT)}: {count} {'pending' if check else 'applied'}")
    return 1 if check and total else 0


if __name__ == "__main__":
    sys.exit(main())
