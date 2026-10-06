#!/usr/bin/env python3
"""🫙️ S5-STORE wave AA (design §22.22, coordinator decision 2026-10-05): a document actor bound to a local folder announces
`documentArchiveAbsent` once per open — when the first answered folder read finds no archive and nothing was read or written
before — and writes an archive only when its bytes differ from the one the folder is known to hold. Both actor twins: the
TypeScript worker and the native Rust actor (the wasm Rust actor has no folder).

Lands, in one compile-atomic write per file:
- schema + corpus `🏪️store/🔄️sync/🧬️schema/🔣️folder-archive-presence`, `🏪️store/🧫️fixtures/🧫️folder-archive-presence` (staged copies);
- TS: `💻️os/🟦️.ts` (`ArtifactEvent` union + strict parse), `🏪️store/👷️worker/🟦️.ts` (`pollFolderOnce`, `writeFolder`, state);
- Rust: `🏪️store/🔄️sync/🦀️.rs` (`ArtifactEvent::DocumentArchiveAbsent`, native actor `setup` / `handle_external_change` /
  `persist_write_archive`, two test seams), the wgpu shell's exhaustive event match (one no-op arm), and the two test
  name maps that match `ArtifactEvent` exhaustively.

Every edit is keyed on an anchor that must occur exactly once in its file; nothing is written unless every anchor of every
file resolved. Idempotent. `--check` prints what is pending; `--emit <dir>` writes the edited files into `<dir>`.

    python3 🧪️s5-store-archive-absent.py [--check | --emit <dir>]
"""
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = TICKET.parents[6]
OS = ROOT / "🧰️framework/🛍️products/💻️os"
STORE = OS / "🔨️modules/🏪️store"
STAGED = TICKET / "🗑️generated/s5-store/stage-aa/🏪️store"
NEW_FILES = ["🔄️sync/🧬️schema/🔣️folder-archive-presence/🔣️.json", "🧫️fixtures/🧫️folder-archive-presence/🔣️.json"]

OS_TS = OS / "🟦️.ts"
WORKER = STORE / "👷️worker/🟦️.ts"
SYNC = STORE / "🔄️sync/🦀️.rs"
WGPU = OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"
PARITY_TEST = STORE / "🔄️sync/🧪️tests/🔬️backbone-parity/🦀️.rs"
UNIT_TEST = STORE / "🔄️sync/🧪️tests/🔬️unit/🦀️.rs"
RESTORE_TEST = OS / "🧪️tests/🧪️folder-archive-restore/🟦️.ts"
TS_LAW = (TICKET / "🗑️generated/s5-store/stage-aa/laws/folder-archive-presence.ts.txt").read_text(encoding="utf-8")
RUST_LAW = (TICKET / "🗑️generated/s5-store/stage-aa/laws/folder-archive-presence.rs.txt").read_text(encoding="utf-8")

EDITS = {
    OS_TS: [
        (
            "union",
            '  | { readonly kind: "documentArchiveReplaced"; readonly archive: readonly number[] }\n',
            '  | { readonly kind: "documentArchiveReplaced"; readonly archive: readonly number[] }\n'
            "  /** 🫙️ The bound folder holds no archive of this document: once per open, from the bootstrap folder read alone, never\n"
            "   * after an archive was read or written — mirrors Rust `ArtifactEvent::DocumentArchiveAbsent`. */\n"
            '  | { readonly kind: "documentArchiveAbsent" }\n',
            '  | { readonly kind: "documentArchiveAbsent" }\n',
        ),
        (
            "parse",
            '  if (event.kind === "commandOutcome") {\n    const batchId = commandBatchIdOfValueV1(event.batchId);\n',
            '  if (event.kind === "documentArchiveAbsent") {\n'
            '    if (Object.keys(event).join(",") !== "kind") throw new Error("backbone worker response: invalid document archive absence");\n'
            '    return { kind: "documentArchiveAbsent" };\n'
            "  }\n"
            '  if (event.kind === "commandOutcome") {\n    const batchId = commandBatchIdOfValueV1(event.batchId);\n',
            '"backbone worker response: invalid document archive absence"',
        ),
    ],
    WORKER: [
        (
            "state-type",
            "  folderArchive: Uint8Array | null;\n  reconnectDelayMs: number;\n",
            "  folderArchive: Uint8Array | null;\n"
            "  /** 🪹️ Whether this open's first folder read was answered, or an archive was read or written: no later read announces\n"
            "   * an absent archive (`documentArchiveAbsent`). */\n"
            "  folderBootstrapped: boolean;\n"
            "  reconnectDelayMs: number;\n",
            "  folderBootstrapped: boolean;\n",
        ),
        (
            "state-init",
            "    folderArchive: null,\n    reconnectDelayMs: HUB_RECONNECT_MIN_MS,\n",
            "    folderArchive: null,\n    folderBootstrapped: false,\n    reconnectDelayMs: HUB_RECONNECT_MIN_MS,\n",
            "    folderBootstrapped: false,\n",
        ),
        (
            "read-held",
            "  try {\n    const response = (await fetchWithTimeout(folderEnvelopeUrl(binding, state.config.documentId), undefined, {\n",
            "  try {\n    const held = state.folderArchive;\n    const response = (await fetchWithTimeout(folderEnvelopeUrl(binding, state.config.documentId), undefined, {\n",
            "    const held = state.folderArchive;\n",
        ),
        (
            "read-absent",
            "    if (response.status === 204 || response.status === 404) return;\n"
            "    if (!response.ok) throw new Error(`folder backbone read failed (${response.status})`);\n"
            "    const archive = new Uint8Array(await response.arrayBuffer());\n"
            "    decodeDocumentArchiveBytes(archive);\n"
            "    const echo = state.folderArchive !== null && equalByteArrays(state.folderArchive, archive);\n"
            "    state.folderArchive = archive;\n",
            "    if (response.status === 204 || response.status === 404) {\n"
            "      if (state.folderArchive !== held) return;\n"
            '      if (!state.folderBootstrapped) emitEvent(state, { kind: "documentArchiveAbsent" });\n'
            "      state.folderBootstrapped = true;\n"
            "      if (held !== null) {\n"
            "        state.folderArchive = null;\n"
            "        setStatus(state, { persisted: false });\n"
            "      }\n"
            "      return;\n"
            "    }\n"
            "    if (!response.ok) throw new Error(`folder backbone read failed (${response.status})`);\n"
            "    const archive = new Uint8Array(await response.arrayBuffer());\n"
            "    decodeDocumentArchiveBytes(archive);\n"
            "    const echo = state.folderArchive !== null && equalByteArrays(state.folderArchive, archive);\n"
            "    state.folderArchive = archive;\n"
            "    state.folderBootstrapped = true;\n",
            '      if (!state.folderBootstrapped) emitEvent(state, { kind: "documentArchiveAbsent" });\n',
        ),
        (
            "read-doc",
            " * ({@link ArtifactState.docAbort}, finding 3); an abort is a clean shutdown, not a failure, so it\n * is swallowed without logging. */\nasync function pollFolderOnce(",
            " * ({@link ArtifactState.docAbort}, finding 3); an abort is a clean shutdown, not a failure, so it\n"
            " * is swallowed without logging. The first answered read of an open that finds no archive, with none read or written\n"
            " * before, announces `documentArchiveAbsent` once; a later empty answer only forgets the archive the folder held, unless a\n"
            " * write landed while that read was in flight. */\nasync function pollFolderOnce(",
            " * before, announces `documentArchiveAbsent` once; a later empty answer only forgets the archive the folder held, unless a\n",
        ),
        (
            "write-skip",
            '  if (bytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");\n'
            "  decodeDocumentArchiveBytes(bytes);\n"
            "  const response = await fetchWithTimeout(\n"
            "    folderEnvelopeUrl(binding, state.config.documentId),\n"
            '    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: bytes },\n'
            "    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },\n"
            "  );\n"
            "  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);\n"
            "  state.folderArchive = bytes;\n",
            '  if (bytes.length > DOCUMENT_ARCHIVE_MAXIMUM_BYTES) throw new Error("document archive exceeds its fixed byte authority");\n'
            "  decodeDocumentArchiveBytes(bytes);\n"
            "  if (state.folderArchive !== null && equalByteArrays(state.folderArchive, bytes)) return;\n"
            "  const response = await fetchWithTimeout(\n"
            "    folderEnvelopeUrl(binding, state.config.documentId),\n"
            '    { method: "PUT", headers: { "content-type": "application/octet-stream" }, body: bytes },\n'
            "    { timeoutMs: FOLDER_FETCH_TIMEOUT_MS, signal: state.docAbort.signal },\n"
            "  );\n"
            "  if (!response.ok) throw new Error(`folder backbone write failed (${response.status})`);\n"
            "  state.folderArchive = bytes;\n"
            "  state.folderBootstrapped = true;\n",
            "  if (state.folderArchive !== null && equalByteArrays(state.folderArchive, bytes)) return;\n",
        ),
    ],
    SYNC: [
        (
            "variant",
            "    /// 🗃️ The whole root plus recursive owned-member closure was replaced atomically.\n    DocumentArchiveReplaced { archive: Vec<u8> },\n",
            "    /// 🗃️ The whole root plus recursive owned-member closure was replaced atomically.\n    DocumentArchiveReplaced { archive: Vec<u8> },\n"
            "    /// 🫙️ The bound folder holds no archive of this document: answered once per open, by the first answered folder read alone,\n"
            "    /// and never after an archive was read or written.\n"
            "    DocumentArchiveAbsent,\n",
            "    DocumentArchiveAbsent,\n",
        ),
        (
            "field",
            "        last_written_hash: Option<String>,\n        remote_state: RemoteState,\n",
            "        last_written_hash: Option<String>,\n"
            "        /// 🪞️ The hash of the archive the folder is known to hold — the last one read or written, none after a read that found\n"
            "        /// the folder empty: an archive with these bytes is not written again.\n"
            "        folder_held_hash: Option<String>,\n"
            "        /// 🪹️ Whether this open's first folder read was answered, or an archive was read or written: no later read announces an\n"
            "        /// absent archive ([`ArtifactEvent::DocumentArchiveAbsent`]).\n"
            "        folder_bootstrapped: bool,\n"
            "        #[cfg(test)]\n"
            "        folder_writes: usize,\n"
            "        remote_state: RemoteState,\n",
            "        folder_held_hash: Option<String>,\n",
        ),
        (
            "init",
            "                last_written_hash: None,\n                remote_state: RemoteState::Detached,\n",
            "                last_written_hash: None,\n"
            "                folder_held_hash: None,\n"
            "                folder_bootstrapped: false,\n"
            "                #[cfg(test)]\n"
            "                folder_writes: 0,\n"
            "                remote_state: RemoteState::Detached,\n",
            "                folder_held_hash: None,\n",
        ),
        (
            "setup",
            "        /// 🌱️ Seeds persistence state from any already-stored recursive archive and installs the file watcher.\n"
            "        async fn setup(&mut self) {\n"
            "            let seeded = match self.folder.as_ref().filter(|_| self.current_pack.is_none()) {\n"
            "                Some(folder) => folder.read_archive().await.ok().flatten(),\n"
            "                None => None,\n"
            "            };\n"
            "            if let Some(bytes) = seeded {\n",
            "        /// 🌱️ Seeds persistence state from any already-stored recursive archive and installs the file watcher. A bound folder\n"
            "        /// whose bootstrap read finds no archive is announced once ([`ArtifactEvent::DocumentArchiveAbsent`]).\n"
            "        async fn setup(&mut self) {\n"
            "            let read = match self.folder.as_ref().filter(|_| self.current_pack.is_none()) {\n"
            "                Some(folder) => Some(folder.read_archive().await),\n"
            "                None => None,\n"
            "            };\n"
            "            if matches!(read, Some(Ok(None))) && !self.folder_bootstrapped {\n"
            "                self.emit(ArtifactEvent::DocumentArchiveAbsent);\n"
            "            }\n"
            "            self.folder_bootstrapped |= matches!(read, Some(Ok(_)));\n"
            "            let seeded = read.and_then(|read| read.ok().flatten());\n"
            "            self.folder_held_hash = seeded.as_deref().map(document_archive_hash).or(self.folder_held_hash.take());\n"
            "            if let Some(bytes) = seeded {\n",
            "            if matches!(read, Some(Ok(None))) && !self.folder_bootstrapped {\n",
        ),
        (
            "write",
            "        async fn persist_write_archive(&mut self, archive: &[u8]) {\n"
            "            let Some(folder) = self.folder.as_ref() else { return };\n"
            "            if folder.write_archive(archive).await.is_ok() {\n"
            "                self.last_written_hash = Some(document_archive_hash(archive));\n"
            "            }\n"
            "        }\n",
            "        ///\n"
            "        /// Bytes the folder is known to hold already — the last archive read or written — are not written again.\n"
            "        async fn persist_write_archive(&mut self, archive: &[u8]) {\n"
            "            let Some(folder) = self.folder.as_ref() else { return };\n"
            "            let hash = document_archive_hash(archive);\n"
            "            if self.folder_held_hash.as_deref() == Some(hash.as_str()) {\n"
            "                return;\n"
            "            }\n"
            "            if folder.write_archive(archive).await.is_ok() {\n"
            "                self.last_written_hash = Some(hash.clone());\n"
            "                self.folder_held_hash = Some(hash);\n"
            "                self.folder_bootstrapped = true;\n"
            "                #[cfg(test)]\n"
            "                {\n"
            "                    self.folder_writes += 1;\n"
            "                }\n"
            "            }\n"
            "        }\n",
            "            if self.folder_held_hash.as_deref() == Some(hash.as_str()) {\n                return;\n",
        ),
        (
            "external",
            "        async fn handle_external_change(&mut self) {\n"
            "            let seeded = match self.folder.as_ref() {\n"
            "                Some(folder) => folder.read_archive().await.ok().flatten(),\n"
            "                None => None,\n"
            "            };\n"
            "            let Some(bytes) = seeded else { return };\n"
            "            let Ok(archive) = crate::os_spr::decode_document_archive_bytes(&bytes).await else { return };\n"
            "            let pack = archive.parent_pack.clone();\n"
            "            let spr = archive.parent_spr.clone();\n"
            "            let hash = document_archive_hash(&bytes);\n",
            "        ///\n"
            "        /// The first answered read of an open that finds no archive, with none read or written before, announces\n"
            "        /// [`ArtifactEvent::DocumentArchiveAbsent`] once; a later empty answer only forgets the archive the folder held.\n"
            "        async fn handle_external_change(&mut self) {\n"
            "            let read = match self.folder.as_ref() {\n"
            "                Some(folder) => folder.read_archive().await,\n"
            "                None => return,\n"
            "            };\n"
            "            let bytes = match read {\n"
            "                Ok(Some(bytes)) => bytes,\n"
            "                Ok(None) => {\n"
            "                    if !self.folder_bootstrapped {\n"
            "                        self.emit(ArtifactEvent::DocumentArchiveAbsent);\n"
            "                    }\n"
            "                    self.folder_bootstrapped = true;\n"
            "                    self.folder_held_hash = None;\n"
            "                    return;\n"
            "                }\n"
            "                Err(_) => return,\n"
            "            };\n"
            "            self.folder_bootstrapped = true;\n"
            "            let Ok(archive) = crate::os_spr::decode_document_archive_bytes(&bytes).await else { return };\n"
            "            let pack = archive.parent_pack.clone();\n"
            "            let spr = archive.parent_spr.clone();\n"
            "            let hash = document_archive_hash(&bytes);\n"
            "            self.folder_held_hash = Some(hash.clone());\n",
            "                    self.folder_held_hash = None;\n                    return;\n",
        ),
        (
            "seams",
            "        #[cfg(test)]\n        pub(super) fn current_archive_test(&self) -> Option<Vec<u8>> {\n            self.current_archive.clone()\n        }\n",
            "        #[cfg(test)]\n        pub(super) fn current_archive_test(&self) -> Option<Vec<u8>> {\n            self.current_archive.clone()\n        }\n"
            "\n"
            "        #[cfg(test)]\n        pub(super) async fn setup_test(&mut self) {\n            self.setup().await;\n        }\n"
            "\n"
            "        #[cfg(test)]\n        pub(super) fn folder_writes_test(&self) -> usize {\n            self.folder_writes\n        }\n",
            "        pub(super) fn folder_writes_test(&self) -> usize {\n",
        ),
    ],
    WGPU: [
        (
            "arm",
            "                ArtifactEvent::CommandOutcome { .. } => {}\n",
            "                ArtifactEvent::CommandOutcome { .. } => {}\n                ArtifactEvent::DocumentArchiveAbsent => {}\n",
            "                ArtifactEvent::DocumentArchiveAbsent => {}\n",
        ),
    ],
}

EDITS[RESTORE_TEST] = [
    (
        "imports",
        "import { decodeDocumentArchiveBytes, encodeBackboneMessage, encodeDocumentArchiveBytes, encodePackValue, type ArtifactActorConfig,",
        "import { decodeBackboneWorkerResponse, decodeDocumentArchiveBytes, encodeBackboneMessage, encodeBackboneWorkerResponse, encodeDocumentArchiveBytes, encodePackValue, type ArtifactActorConfig,",
        "import { decodeBackboneWorkerResponse, decodeDocumentArchiveBytes,",
    ),
    (
        "law",
        '      expect(calls).toEqual(["history", "refresh"]);\n    });\n  });\n}\n',
        '      expect(calls).toEqual(["history", "refresh"]);\n    });\n' + TS_LAW + "  });\n}\n",
        "the folder-archive-presence corpus: an absent archive is announced once",
    ),
]

NAME_MAPS = [PARITY_TEST, UNIT_TEST]
NAME_ARM = 'ArtifactEvent::DocumentArchiveReplaced { .. } => "documentArchiveReplaced",\n'


def plan_file(path, edits):
    """🧮️ The edited text of `path` and the names of its pending edits."""
    text = path.read_text(encoding="utf-8")
    pending = []
    for name, old, new, marker in edits:
        if marker in text:
            continue
        if text.count(old) != 1:
            raise SystemExit(f"{path.name} of {path.parent.name}: anchor `{name}` occurs {text.count(old)} times (expected 1): re-derive the wave")
        text = text.replace(old, new)
        pending.append(name)
    return text, pending


def plan_name_map(path):
    """🏷️ The test name map of `path` with the new event's arm after the replaced archive's."""
    text = path.read_text(encoding="utf-8")
    if 'ArtifactEvent::DocumentArchiveAbsent => "documentArchiveAbsent",' in text:
        return text, []
    lines = text.split("\n")
    hits = [index for index, line in enumerate(lines) if line.strip() == NAME_ARM.strip()]
    if len(hits) != 1:
        raise SystemExit(f"{path.name} of {path.parent.name}: the name-map arm occurs {len(hits)} times (expected 1): re-derive the wave")
    indent = lines[hits[0]][: len(lines[hits[0]]) - len(lines[hits[0]].lstrip())]
    lines.insert(hits[0] + 1, indent + 'ArtifactEvent::DocumentArchiveAbsent => "documentArchiveAbsent",')
    text = "\n".join(lines)
    if path == UNIT_TEST and "--rust-law" in sys.argv[1:]:
        text = text.rstrip("\n") + "\n" + RUST_LAW
        return text, ["name-map", "law"]
    return text, ["name-map"]


def main():
    writes, pending = [], []
    for relative in NEW_FILES:
        source = (STAGED / relative).read_bytes()
        target = STORE / relative
        if not target.exists() or target.read_bytes() != source:
            writes.append((target, source))
            pending.append(f"new:{relative.split('/')[-2]}")
    for path, edits in EDITS.items():
        text, names = plan_file(path, edits)
        if names:
            writes.append((path, text.encode("utf-8")))
            pending += [f"{path.parent.name}:{name}" for name in names]
    for path in NAME_MAPS:
        text, names = plan_name_map(path)
        if names:
            writes.append((path, text.encode("utf-8")))
            pending += [f"{path.parent.name}:{name}" for name in names]
    print("pending: " + (", ".join(pending) if pending else "none"))
    if "--emit" in sys.argv[1:]:
        target = Path(sys.argv[sys.argv.index("--emit") + 1])
        target.mkdir(parents=True, exist_ok=True)
        for index, (path, content) in enumerate(writes):
            (target / f"{index}-{path.parent.name}{path.suffix}").write_bytes(content)
        print(f"emitted to {target}")
        return
    if "--check" in sys.argv[1:] or not writes:
        return
    for path, content in writes:
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(content)
    print("applied")


main()
