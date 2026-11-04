# 🔍️ T17 Data Audit: Persistence Path (haiku-data-audit, code inspection only)

Prefixes: P1–P5 = `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🖥️native/🎭️actor/{📁️actor-folder-persistence-and-watcher, 🎬️actor-construction-drive-and-commands, 📨️actor-hub-frames-acks-and-delivery, 🔗️actor-hub-link-and-bootstrap, 🔌️actor-socket-types-bootstrap-control-and-folder-endpoint}/🦀️.rs`; FS = `…/🖥️native/📥️lifted/🔄️sync/🦀️.rs`; MSG = `…/🔄️sync/📡️actor-message-bytes-status-and-events/🦀️.rs`; CFG = `…/🔄️sync/🗄️persistence-binding-actor-config-and-messages/🦀️.rs`; IO = `🏪️store/🚪️io/🦀️.rs`; FAC = `🏪️store/📦️codec/🏭️factory/📥️lifted/🏪️store/🦀️.rs`; SEM = `🔨️modules/🧬️semio/🦀️.rs`; HOST-* = `💻️os/🖥️host/{🔌️plugin-host/🗃️backbone-document, 🔌️plugin-host/📚️space-catalog, 🗃️document-backbone, 🔀️workflow, 🖼️media-export-raster}/🦀️.rs`; ZIP-* = `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/…`.

## Top Risks

1. Torn tail in `FolderEventLogStorage::append`: there is no truncation of a partial trailing record, and `events()` silently breaks on `UnexpectedEof` (FS:155-215). The directory is not fsynced after `.semio` is created.
2. Write failures are swallowed: `persist_write_archive`/`persist_archive` return `()` and the caller advances `current_*` anyway (P1:16-41,104-117; P2:382-387). `persist_operations` inserts `known_op_ids` before the write (P1:75,85). `handle_external_change` can then roll back unsaved edits for folder-only documents (P1:173-189).
3. There is no per-edit save acknowledgment: `persisted = last_written_hash.is_some() || server_frontier.is_some()` is sticky and true for hub-only documents (P3:291-312). `CommandOutcome` exists only on hub acks (P3:140-181).
4. Export names use the kind id as the file stem (`2d.note.svg`) (HOST-WF:657,664-666; HOST-RASTER:158-180). Pack/DSL exports drop `BackboneDocument.name` (HOST-BB:267-296).
5. The envelope registry ignores version (SEM:393,411-427). `pack_schema_hash` comes from three different sources (FAC:50-54,233-236; TIFF:39). DSL `wrap_text` trims leading whitespace and the BOM (SEM:233,239,333-335).

## Other Findings

- The folder text storage `write_archive`/`write_pack`/`write` are non-atomic, have no fsync and no lock, and use a fixed staged name (FS:260-381). An atomic primitive exists in `🛎️services/📄️fixed-file-storage` (:200-208) but is unused.
- The watcher watches `.semio/documents.db` while storage writes `.semio/events.semio` (P1:211-217 vs FS:102-104). `watch_external` defaults to false (CFG:101-103).
- A setup read error is treated as "no archive" (P2:323).
- Import overwrites by id without a check (HOST-CAT:119-126). The file-space port writes `<stem>.<ext>.*.semio` beside the chosen file instead of the file itself (HOST-FILE:45-114), contradicting CFG:70-71.
- The zip editor has a revision-digest draft-conflict guard (ZIP-ED:102-108; ZIP-RET:120), reversible `ZipMutation`, and legacy header retention (ZIP-ENC:15-19). History is persisted as `HistoryEdit`/`HistoryTransitionRecord` events (P1:68-101). The ops-text fallback loses undo history (P5:130-139).

## Requested Runtime Repros

These cover: the torn tail, a read-only folder mid-edit, a stale rollback after a failed write, the watch path, the export file name, the pack name round trip, a DSL with BOM/leading whitespace, PNG tEXt/eXIf/iCCP retention, v2 envelope dispatch, double import overwrite, concurrent `write_archive`, and a corrupt archive followed by genesis.

## Per-Codec Preservation (T17b, static)

- **Preservation-oriented:** PNG (ancillary chunks), WAV (other chunks), BCF (unmodeled parts), OBJ (unknown statements), LAS (raw IEEE + VLRs), EPW, TSV (byte-exact), TIFF, GIF, XML (CDATA, comments, PI), JSON (number lexeme), docx (raw rPr/pPr), xlsx/pptx (XML parts), IDML/ZIP archive, binary.
- **Lossy or regenerated:** JPG (re-encoded scan and tables; APPn segments kept raw), PDF (page size + text only; fonts, images, annotations and metadata dropped), DWG (rebuilt from the logical model), DXF (leading spaces and CR trimmed), deflate (recompressed), HTML (root whitespace), MP4/AVI (boxes and index rebuilt), CSV (line endings normalized), WAV (odd trailing data byte dropped; PCM16 only).
- **`wrap_text` (BOM and leading-whitespace trim in DSL print)** is used by about 20 codecs: csv, tsv, idml, gif, pdf, stl, las, bcf, epw, mp3, avi, deflate, binary, semio, md, html, xml, json, and others.
- **Priority preservation tests:** cells marked unverified, plus the PDF/JPG/DWG losses. These must be documented as declared limits or fixed.

## T03 Verdict (T17d, static)

| Risk | Verdict |
| --- | --- |
| Torn tail | fixed |
| Sticky `persisted` | fixed |
| Watcher path (Rust) | fixed |
| Atomic archive and mirror writes | fixed |
| Write failures | partial (reason discarded; no error field) |
| Setup read error | fixed for read errors; decode failure still treated as absent, so a corrupt archive can be overwritten |
| Stale rollback | partial (retry clobbers the external divergence; rollback still runs when `unsaved` is none) |
| `append_ops` | not atomic |
| TS dev plugin and admit schema | still use `documents.db` |
| Law corpora | two separate corpora (Rust and TS) |
| CQRS/ES | not met: whole-archive overwrite, and undo goes through `FolderTextStorage::write` |

All open items were routed to T03.

## Save Status to UI (T17e, static): All Four Points Not Met

- **(a) "Saved" before acknowledgement.** `pollFolderOnce` sets `persisted: true` on any successful read (`🏪️store/👷️worker/📁️folder/🟦️.ts:103`), racing the admission (`♻️artifact-lifecycle/🟦️.ts:131`). The check-in pill ignores `persisted` (`transfer-checkin-tutorial/🟦️.ts:397-401`). The status carries no error payload.
- **(b) Dirty marker.** The only marker is the text-draft one (`✏️TextEditor/🟦️.tsx:546`). It clears when the local command is admitted, not on acknowledgement. There is no window or tab marker.
- **(c) Dirty navigation.** There is no guard. `closeDocument` clears pending (`close-document/🟦️.ts:20-94,148-157`), and the worker teardown discards `pendingMutations` and the unsaved archive (`🛑️session-teardown/🟦️.ts:110-128`). The Rust close never flushes `unsaved` (`🎬️…/🦀️.rs:181-208`). There is no beforeunload handler.
- **(d) In-flight saves after close.** In TS, an in-flight PUT is aborted by `docAbort` (`✍️write/🟦️.ts:29-33,65`). In Rust, the write completes, but `unsaved` is dropped.

Routed to T03 (worker/actor) and T02 (shell UI).

## Stale-Async Protection (T17f, static)

- **Worker events.** Met: events are keyed by runtime key and `clientInstanceId`; entries are deleted on close.
- **Projections.** Met per instance. Gap: `currentDocumentRuntimeKey` takes the first plugin and instance match (`🏛️ShellHost/📤️document-transfer-tabs/🔸️document-transfer-ref/🟦️.tsx:156-161`).
- **Archive closure.** Risk: the folder persistence closure reads the archive by instance (`🔸️open-document/🟦️.ts:46-53`), so if the instance switched to document B, A's persistence writes B's archive under A's id.
- **Natural-file import.** Cancel is safe. There is no single-flight guard, and the panel is rebuilt from stale render-time state (`🔶️scope/🟦️.tsx:141-187`, `update-space-panel:63-65`).
- **Undo.** Met per app instance.

Routed: archive closure to T03; runtime key and single-flight import to T02.
