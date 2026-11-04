# 💾️ Persistence Contract for Well-Known Artifact Editors (T03)

Status: published early; defect list at the bottom is updated as fixes land.

## Canonical path (edit to durable reopen)

1. Editor window action (e.g. `set-node`, `textEdit`) -> editor command -> `ArtifactEditor` reduces to `Emit { artifact_mutations, .. }` (typed mutation with sparse diff + inverse; this is the only write path, no CRUD and no whole-document replace by an editor).
2. The shell turns mutations into `MutationEnvelope`s (`documentBackbone` / `localMutations` message to the artifact actor). Identity (`document_id`, `mutation_id`) is minted before send and is the idempotency key.
3. Artifact actor, one per open document, two twins that MUST agree:
   - TypeScript worker: `🏪️store/👷️worker/♻️artifact-lifecycle` (`admitLocalMutations`, `handleLocalMsg`), `📁️folder/✍️write` (`writeFolder`), status via `setStatus`.
   - Rust native actor: `🏪️store/🔄️sync/🖥️native/🎭️actor` (`persist_operations` appends to the spr log, `persist_archive`/`persist_write_archive` write the recursive archive through `FolderEndpoint`, `status()` and `emit_status_if_changed`).
4. Durable form: document archive = parent pack (genesis snapshot) + parent spr (append-only history log) + members. `decode_document_archive_bytes` is the reopen reader. Undo/redo are history transitions in the same log (`is_history_transition`), never destructive rewrites. Unknown/retained fields live in the pack/retained state and must survive splices (zip `🧵️retained`, draw `🖊️host-state` are the reference shapes).
5. Reopen = folder read -> `DocumentArchiveReplaced` / bootstrap -> fold of pack + spr. After restart the same archive bytes are the source of truth.

## Host contract: events and verbs (published for the shell)

Status event (`ArtifactEvent::Status` / TS `{kind:"status"}`): `{ persisted, pendingMutations, remote, writeFailure? }`. `persisted` is derived once per host and is true only when the folder acknowledged the newest archive covering every admitted edit. `writeFailure = { code, message? }` is present while unsaved because of a failure; emitted by the Rust actor (`SyncWriteFailure`) and the TS worker. Codes emitted by T03: `folder-write-failed` (refused write, retried with backoff), `folder-unreadable` (folder or archive unreadable/corrupt, all writes blocked, re-read with backoff), `external-divergence` (folder changed under unsaved edits, nothing overwritten, retry stopped until the folder matches again). `hub-rejected` and `quota-exceeded` are shell-defined codes T03 does not emit.

Conflict events (`ArtifactEvent::Conflict` / TS `{kind:"conflict", code, message, ...}`), one per episode: `folderReadFailed`, `folderArchiveCorrupt`, `externalDivergence`, `closeWithUnsavedState` (TS adds `unsavedMutations`).

Verbs (worker request `kind:"close"` and actor `Detach`):
- Close request = flush then close. TS: `close` runs `closeArtifactFlushed`: waits for the write queue (queued, in-flight, retry waiting) up to 10 s while status events report progress (`pendingMutations`, `persisted`, `writeFailure`); then closes. If anything is still unacknowledged it emits the `closeWithUnsavedState` conflict first and then closes (nothing is dropped silently; the outcome is that conflict). Native: the closing actor retries `unsaved` until acknowledged or the close deadline, else emits `closeWithUnsavedState`, then terminates.
- Cancellation: TS `closeArtifactFlushed(documentId, spaceId, clientInstanceId, signal)`; aborting the signal stops waiting and falls into the `closeWithUnsavedState` path. For a prompt-before-close UX the host should read the last status (`persisted=false` or `writeFailure`) before sending `close`, and send `close` only after the user chose Discard or after `persisted` turned true.
- Retry verb: a new `localDocumentArchive` send retries immediately (the newest archive supersedes the queued one); `externalChanged` re-probes a diverged/unreadable folder. A dedicated `retry` request is not added: the archive send is the retry and is idempotent.

## Rules every batch editor and adapter must keep

- R1 Saved only after acknowledgment: `ArtifactSyncStatus.persisted == true` only when the bound folder holds an archive covering every locally admitted mutation. A first successful write must not mark later edits saved.
- R2 Dirty: any admitted mutation not yet covered by an acknowledged write sets `persisted=false` (TS: `admitLocalMutations`; native: derived from held hash vs current archive and write failure flag).
- R3 Failure is visible and retried: a failed folder write keeps `persisted=false`, emits a status event, keeps the edit in memory/pending and retries with backoff; it never silently drops the edit and never records `last_written_hash` on failure.
- R4 Serialized writes: archive writes of one document are applied in submission order, one in flight; a stale or slower write must never overwrite a newer one; an acknowledgment clears only the mutations that archive covered.
- R5 Stale async work: results of reads/writes that began before a newer write/close/replace are discarded (`folderArchive` / `folder_held_hash` echo guard, `docAbort` on close).
- R6 Conflicts: external divergence with pending local ops emits `Conflict` (`externalDivergence`), never silently replaces; editor-level drafts use a text/content revision token and refuse with a `*.draft-conflict` fault (zip `text_revision`).
- R7 Metadata preservation: an editor mutation touches only the addressed field; identity, retained, unknown-but-retained fields round-trip byte-stable. Verify with a no-op edit -> identical archive.
- R8 Language-agnostic fixtures: persistence laws live as JSON fixtures consumed by both the Rust and TS tests (see fixtures listed below).

## Findings



### Fixed (code complete; verification status in the last section)

Native actor (`🏪️store/🔄️sync/🖥️native/🎭️actor/*`, `📥️lifted/🔄️sync/🦀️.rs`):
- F1 `FolderEventLogStorage::append` truncates an interrupted final record (header walk, last-record checksum) before appending, `events()` stops at a torn final record, `torn_tail_bytes()` reports the damage, the directory is fsynced when `.semio` is created. Fixture `🏪️store/🧪️tests/🧫️fixtures/🩹️folder-event-log-torn-tail`.
- F2 Write failures are no longer swallowed. `persist_write_archive` returns whether the folder acknowledged; `commit_archive` advances `current_pack/spr/archive` only on acknowledgment, otherwise the newest archive is held in `unsaved`; later operations, retractions and genesis build on `unsaved` (`working_pack_spr`), a backoff retry (`retry_folder_write`, driven from the `Status` drive phase and the idle deadline) rewrites it, a failed `append_history_events_to_spr` un-registers the ids it just inserted, `install_hub_seed` goes through the same commit. `handle_external_change` returns early while `unsaved` is held (no rollback of unsaved edits) and reports `externalDivergence` if the folder really changed. `known_op_ids` deliberately still covers ids held in `unsaved` (they are part of the working state and are the dedup key); the guard above is what keeps them from being treated as lost.
- F3 `ArtifactSyncStatus.persisted` is no longer sticky: it is true only while the folder acknowledged the newest archive (folder documents) or the hub acknowledged everything with nothing pending/queued (hub documents, native and wasm). Status is emitted on every write outcome.
- F4 The watcher watches the canonical `FolderEventLogStorage::event_path()` (`.semio/events.semio`) instead of `.semio/documents.db`.
- F5 `FolderTextStorage` writes are atomic (unique staged name per process and write, fsync of file and directory, rename) and serialized with the same per-folder writer lock as the event log. Deviation: the primitive in `🛎️services/📄️fixed-file-storage` is a token/page session API and its crate is not a dependency of `os-store-sync-native`, so the equivalent minimal `atomic_replace` lives next to the storage; consolidating both is a follow-up.
- F6 A setup read error is a recoverable `folderReadFailed` conflict: `folder_unread` blocks every write (an unreadable archive is never replaced by a genesis), status stays unsaved, and the retry re-reads; if the folder then holds an archive the unsaved edits were not built on, `externalDivergence` is reported instead of writing.

TypeScript worker (`🏪️store/👷️worker`):
- F7 `📁️folder/✍️write` `persistFolderArchive`: one write in flight per document, submission order, newest archive supersedes a queued one, only the mutations an archive covered leave `pendingMutations`, `persisted` true only when none remain, refusal keeps the edit pending/unsaved and retries with backoff. Used by `localDocumentArchive` and the config facet lane. `writeFolder` no longer sets `persisted` itself.

### Fixtures (language-agnostic, shared by Rust and TS)
- `🏪️store/🧪️tests/🧫️fixtures/💾️folder-write-acknowledgment/🔣️.json` (Rust corpus test `the_folder_write_acknowledgment_corpus_matches_the_native_actor`)
- `🏪️store/🧪️tests/🧫️fixtures/🩹️folder-event-log-torn-tail/🔣️.json`
- `💻️os/🧪️tests/🧫️fixtures/💾️folder-write-sequencing/🔣️.json` (vitest `💻️os/🧪️tests/💾️folder-write-acknowledgment/🟦️.ts`, registered in the worker test block)

### Second pass (audit follow-ups)

- G1 Corrupt or undecodable archive is never treated as absent: `setup` and `handle_external_change` call `block_folder` (`folderArchiveCorrupt` / `folderReadFailed`), which sets `folder_unread`, blocks every write (no genesis over a corrupt archive), reports one conflict per episode and retries the read with backoff.
- G2 `retry_folder_write` re-reads before writing (`probe_folder`): unreadable -> blocked; a folder that no longer holds what this actor last read or wrote -> `diverge_folder` (`externalDivergence`, `folder_diverged`, retry stops, nothing overwritten); resumes automatically when `handle_external_change` sees the folder equal to the held archive again.
- G3 The external-change rollback now also counts queued hub operations (`outbox`) as pending and the whole handler is skipped while `unsaved` is held. Test: corpus case `external-change-after-refused-write-never-rolls-back`.
- G4 Typed failure on the status: Rust `ArtifactSyncStatus.write_failure: Option<SyncWriteFailure { code, message }>` with codes `folder-write-failed`, `folder-unreadable`, `external-divergence`; TS `ArtifactSyncStatus.writeFailure` (field and codes agreed with the shell's saved-state derivation). The TS worker sets it on a refused write and clears it on acknowledgment.
- G5 `append_ops` truncates a torn trailing line, appends under the folder writer lock and fsyncs. The stale comment on `persist_write_archive` is rewritten. Config schema description and the mcp comment now name `.semio/events.semio`. NOT unified: the dev vite plugins (`🧑‍💻dev/🔌️vite-plugins/🗄️backbone-storage`, `📇️bootstrap-folder-mirror`) read a sqlite `documents.db`, a different format from the SEMIOEL1 event log, so renaming alone would be wrong; they need a SEMIOEL1 reader (T05 owns that file).
- G6 One corpus: `🏪️store/🧪️tests/🧫️fixtures/💾️folder-write-acknowledgment/🔣️.json` (v2) with a `native` section (Rust actor) and a `worker` section (TS worker); the former second fixture is deleted. The two hosts cannot share steps yet because the Rust actor is serial and needs a failing folder binding while the TS worker injects fetch responses.
- G7 TS `persisted` is derived in one place (`setStatus` in `🎭️actor-lifecycle`): for a folder document it is `pendingMutations == 0 && no writeFailure && folderArchive held`; `pollFolderOnce`, admission and the write queue no longer set it. Rust derives it in `status()` from the acknowledged write state; the wasm actor from pending/outbox (it has no folder).
- G8 Close never discards silently. Native: the closing branch flushes `unsaved` (retry until acknowledged or the close deadline), otherwise emits the `closeWithUnsavedState` conflict, then terminates. TS: `closeArtifactFlushed` (used by the `close` request) waits for the write queue (`settleFolderArchive`, cancellable via AbortSignal, status events show progress) and `closeArtifact` emits `closeWithUnsavedState` (`unsavedMutations`) if work remains. The shell prompts on that conflict code (host owner).
  Limit: the worker can only flush archives the guest already sent; edits admitted but never followed by a `localDocumentArchive` cannot be saved by the worker and are reported, not dropped.
- G9 Shell closure for folder persistence (`🏛️ShellHost/🗄️shell-uri-documents/🔸️open-document`) reads through `readOwnedDocumentArchiveV1`: fail closed unless the entry is current, no other open document shares the app instance, and the instance identity (read before and after the archive) does not name another document. A null `parentDocumentId` is accepted (no foreign identity known). Test: `🎨engine/🧪️tests/🔏️folder-archive-owner`.

### Plan for event-log authority (CQRS/ES, contract section 4) — not implemented
Today the folder holds whole-archive snapshots (`DOCUMENT_ARCHIVE_PUT_EVENT` carries the complete archive) and `FolderTextStorage::write`/`write_pack` overwrite files; the spr log inside the archive is the only history. Target:
1. Add event kinds to `FolderEventLogStorage`: `OPERATION_APPENDED` (mutation envelope bytes + id, authoritative), `HISTORY_TRANSITION_APPENDED` (undo, redo, checkout, alternative), keep `DOCUMENT_ARCHIVE_PUT` only as a `SNAPSHOT` projection with the log position it covers.
2. `persist_operations` appends one event per envelope (idempotent by mutation id) instead of re-encoding the archive; `commit_archive` writes snapshots only every N operations or on close/idle, never on the edit path; a torn snapshot is irrelevant because the log is replayed from the last intact snapshot.
3. Reopen = last intact snapshot + fold of later events (reuse `spr_events`); `handle_external_change` compares log positions, not archive hashes, so append-only external edits merge and only a missing id is a divergence.
4. `FolderTextStorage::write`/`write_pack` become export projections written from the fold (derived, atomic), no longer the undo/redo path; undo/redo are appended transitions.
5. Fixtures: replay equivalence (snapshot + tail == full replay), torn tail across a snapshot boundary, idempotent re-append, divergence by missing id; run in Rust and TS.
This changes the on-disk format and the TS dev plugins together, so it needs its own ticket and T05/T06 coordination.

### Not done / follow-ups
- Not changed (host part, sonnet-host): publish the session after `openDocument` succeeds; shell prompt for `closeWithUnsavedState`.
- `watch_external` defaults to false in the config; the shell's open path sets `watchExternal: true`.
- Stdio `Pack` bindings need a registered document codec per schema (write fails with "no document codec resolves schema"); registration belongs to T01/T02.

### Verification status
- Rust: `repo cargo --store=shared --configuration=prototype -- test -p semio-framework-os-store-sync-native --lib -- folder_ the_folder_` queued/compiling at the time of writing (host heavily loaded); result recorded in the final message.
- TypeScript tests (vitest) not run: `bun` is not on PATH in this session (T05/T06 own the toolchain).
