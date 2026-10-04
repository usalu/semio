# 📓️ API: stepped whole-document load (design §16.6, gap N17 — reload)

**Status:** binding proposal (S3-W2A, 2026-10-02 19:50), written before the code (coordinator decision 19:40).
**Owners:**
- Guest runtime, channel, run module and MCP sender: S3-W2A.
- React host: S3-W2B.
- wgpu host: S3-W2C.

**Store contract:** `📓️api-deferred-history-replays.md` §1 (bounded retained initializer, one operation per job step).

## 1. Goal

Loading a whole document folds its history. Over a long history (the paged ledger admits thousands of edits) that fold
must not run inside one call or one reactor turn. AGENTS requires progress and cancellation for every expensive operation.
Today three entry points fold synchronously behind `resolve_ready(..)`, which panics on any suspension:
`AppCommand::LoadDocument`, the cold-pair ingress and checkpoint restore. A fourth, `PureCommand` lane hydration, does the
same.

Every whole-document load becomes ONE resumable operation with a handle, progress `{done, total}`, cancellation that
restores the previous document with zero trace, and a single completion that publishes the ready document. No
`resolve_ready` remains on a load path.

## 2. One protocol: the document archive load

The guest already owns a stepped, ACK-owned whole-document load: the **document archive load**. It runs decode, then the
stepped `PersistedDocumentHydration`, then `begin_persisted_document_store_replacement` (the bounded retained initializer),
then members, then one atomic commit. A plain document is an archive with no members:

```
DocumentArchivePack { parent_pack: <pack>, parent_spr: <spr>, members: [] }
```

The protocol is therefore unified on it. **`AppCommand::LoadDocument { seq, pack, spr }` is deleted**, together with its
reply `Done`, its TS twin (`💻️os/🟦️.ts` `LoadDocument` / `AppChannelClient.loadDocument`) and its decoder arms. No
compatibility path remains.

### 2.1 Commands (unchanged wire, existing since CHANNEL_VERSION 16)

| Command | Meaning |
|---|---|
| `LoadDocumentArchive { seq, archive }` | Admits the load under a host-chosen operation id. It is refused with a typed fault when the archive is malformed, when a load is already live, or when the slot is occupied. |
| `PollDocumentArchiveLoad { seq, operation }` | Advances the load for up to `DOCUMENT_ARCHIVE_POLL_WALL_US` and answers its status. |
| `CancelDocumentArchiveLoad { seq, operation }` | Requests cancellation. Cleanup is owned by the guest. |
| `AcknowledgeDocumentArchiveLoad { seq, operation }` | Releases a terminal operation after the host has read its terminal status. |

### 2.2 Status (`AppFrame::DocumentArchiveLoad { in_reply_to, status }`)

`DocumentArchiveLoadStatus { operation, state: Pending|Running|Ready|Cancelled|Fault, completed, total, fault }`

The counting changes (S3-W2A) so that progress is real for long histories:

- `total` is the operations the parent's history folds, plus 1 per member, plus 1 for the commit.
- `completed` is the operations folded by the retained initializer so far, plus the members admitted, plus the commit.
- `completed ≤ total` always holds. Both are monotonic within one operation.
- Hosts show `completed/total` as `done/total`.

### 2.3 Lifecycle

1. **Admit.** The host sends `LoadDocumentArchive`. The reply carries no frame beyond `Done`. The instance is now
   **loading** (§3).
2. **Drive.** The host polls once per animation frame or turn. The guest maintenance pump also advances the load in the
   background within its per-turn budget. Neither the poll nor the pump folds more than one retained-initializer budget
   in a turn. The initializer folds one operation per job step, and the poll's wall bound is `DOCUMENT_ARCHIVE_POLL_WALL_US`.
3. **Terminal.**
   - **`Ready`:** the new store, windows and caches are swapped in atomically. The guest publishes `DocumentChanged` and a
     history patch with every row (backfilled from the loaded log). Interaction resumes.
   - **`Cancelled`:** the previous document, history, windows, caches and selection are exactly as before the admit. That
     is zero trace: nothing of the candidate was published.
   - **`Fault`:** same restore as `Cancelled`. `fault` carries the encoded `Fault`, and the host shows it.
4. **Acknowledge.** The host sends `AcknowledgeDocumentArchiveLoad`. Until then the terminal status stays readable, and a
   new load for the same instance is refused.

## 3. The "loading" document state

While a load is live (admitted, not yet acknowledged and not yet terminal), the instance refuses everything that would read
or mutate the document being replaced:
- app commands and typed operations;
- history verbs, including `historyEdit*`;
- clipboard;
- child commands;
- interaction verbs.

The refusal is a typed fault with code **`document.loading`**. Its notice joins the kernel `HISTORY_NOTICE_LABELS` table
(`historyNotice()` twin):
- en "The document is still loading — wait for it or cancel it first."
- de "Das Dokument wird noch geladen — abwarten oder zuerst abbrechen."

Reads (`ReadDocument`, `ReadHistory`, render) answer the previous document until `Ready`. View-only verbs keep working
(camera, panels).

The history wire and body show the load as a reprojection of kind `load`:
- `HistoryPatch.reprojection = { done, total, kind: "load" }`, with `done/total` taken from the operation's status;
- the history body's `framework.history.reprojection` section reads "Document load / Loading document: {done} of
  {total}" (de "Dokument laden / Dokument wird geladen: {done} von {total}");
- its Cancel button dispatches the existing `historyEditCancelReplay`. While no session is open, that verb cancels, in
  order of precedence: a live document load (previous document kept, zero trace), else a deferred local history step
  (zero trace), else pauses a remote change.

No new verb is introduced, so no descriptor regeneration is needed for this. Hosts may also show their own progress
chrome from the status frames.

## 4. Entry points (all move to the one protocol)

| Today | After |
|---|---|
| `AppCommand::LoadDocument` (MCP workspace `exchange_one_real` ≈1477/2657, `🏃️run` ≈1129, TS `AppChannelClient.loadDocument`, wgpu `loadAppDocumentPack`, React `PluginRuntime.loadAppDocumentPack`) | `LoadDocumentArchive` with `members: []`, then Poll until terminal, then Ack. TS helper: `AppChannelClient.loadDocumentArchive(archive, signal, progress)` already exists, so `loadAppDocumentPack(instanceId, pack, spr, signal?, progress?)` becomes a thin call of it. |
| Cold-pair ingress (`⚛️reactor/🔄️turn/🦀️.rs` ≈1038, `ColdPairIngressStatus::Loading`) | `begin_load` admits an archive load (operation id derived from the transfer generation). `ColdPairIngressStatus::Loading` persists across turns while the reactor pump drives it. `finish_load` runs at the terminal state. The host's cold-pair poll already observes `Loading`. |
| Checkpoint restore (`⚛️reactor/📸️checkpoint/🦀️.rs` ≈99) | Each restored instance is re-created under its checkpointed id and admits an archive load under `RESTORE_DOCUMENT_LOAD_OPERATION` (bit 62). `restore` answers the instances with a load, and the reactor turn polls each until terminal. The instance is **loading** until its load is `Ready`. |
| `PureCommand` lane hydration (`🔌️plugin/🦀️.rs` ≈44056) | **Open, needs a decision.** A pure command is a stateless evaluation that the MCP workspace sends with all three lanes on every call. Its `.pack` is the document's INITIAL snapshot (`print_document_pack` encodes `vcs.initial_snapshot`), so the head can only be reached by folding the `.spr` history, and a head-only hydration is impossible on this wire. Options: (a) the sender keeps a live instance (stateful MCP) and stops sending pure commands for documents with history; (b) the pure command's document lane carries the HEAD snapshot pack with an empty history (a sender-side change: the MCP workspace prints `head + []`). (b) is cheapest and keeps the evaluation O(snapshot). The fold there never suspends, so `resolve_ready` cannot panic today, but the cost is O(history) per command. |
| `PluginApp::load_document_pack/text`, `hydrate_document_lane` (trait) | `load_document_pack` / `load_document_text` leave the host-facing `PluginApp` surface. The guest's only document load is `begin_document_archive_load`. Native tooling and tests use `artifact_app_laws::load_document(app, files)`, which admits the same archive load and drives it to completion with real polls, so tests prove the stepped path. Text files convert to pack+spr through the existing printers before admission. `hydrate_document_lane` becomes the head-only hydration above. |

## 5. Channel and component interface

The `AppCommand::LoadDocument` command tag and its decoder are deleted. That is a frame-layout change, so it **rides the
final channel-bump wave (design §20.7)**. Hosts move to `LoadDocumentArchive` now, since it exists since
CHANNEL_VERSION 16. The deletion of `LoadDocument` lands with the bump wave.

The progress counting change (§2.2) and the new fault code `document.loading` are additive.

## 6. Laws (S3-W2A, toy app and puzzle 2d with ≥ 200 mutations)

1. **Ready.** A 240-operation document loads through the archive protocol. No poll or pump turn folds more than one
   initializer budget, `completed/total` is monotonic, and `Ready` holds exactly the source head, history rows,
   supersessions and outcomes.
2. **Cancel.** Cancelling at every phase restores the previous document with zero trace: head, revision, history rows,
   windows and selection are unchanged.
3. **Loading.** While loading, an app command, `undo` and `historyEditBegin` answer `document.loading` (notice en/de), a
   view verb succeeds, and `historyEditCancelReplay` cancels the load.
4. **Hosts.** Cold-pair load, checkpoint restore and the MCP/run senders reach `Ready` through polls.
5. **Pure.** Once §4 is decided: a pure command over a long-history document never replays the history.

## 7. Host work

- **S3-W2B (React, `🔌️PluginRuntime`, `🏛️ShellHost`):** route `loadAppDocumentPack` through `loadDocumentArchive(archive,
  signal, progress)`; show `document.loading` notices via `historyNotice(code)`; the guest's Rust-built
  `framework.history.reprojection` section (kind `load`) needs no host code.
- **S3-W2C (wgpu, `🐚️plugin-bridge`):** the same change to `loadAppDocumentPack`; apply the notice mapping; the progress chip
  reads the status frames.
- **MCP / `🏃️run` senders (S3-W2A):** replace `AppCommand::LoadDocument` with the archive admit + poll loop.


## 8. Coordination findings (S3-W2A, 2026-10-03 06:40)

- **Cold-pair ingress needs a host change first.** The browser host (`🏪️store/👷️worker/🟦️.ts` `transferColdPair`) asserts that
  the turn of the LAST page answers `Applied` (`owner.assertApplied`), and the wgpu renderer and the gis cold-pair tests rely
  on the same thing. Order of the switch:
  1. Hosts accept `loading` on the last page and keep invoking empty poll turns (`invokePoll(child, [], null)`) until
     `applied` or `fault`.
  2. Only then does the guest return `ColdPairIngressStatus::Loading` across turns while its archive load runs.
  
  Owners: the actor/host cold-pair owners (store worker and wgpu renderer) for step 1, then S3-W2A for step 2.
- **MCP and `🏃️run` senders are not S3-W2A files.** `🌉️mcp/🏠️workspace/🦀️.rs` (`load_session_document`, `ExportMedia`) and
  `🏃️run/🦀️.rs` (the command batch) are other WPs' regions, and peers are active in them. The change for their owners:
  1. Replace `AppCommand::LoadDocument { pack, spr }` with `LoadDocumentArchive { archive: { parent_pack: pack, parent_spr:
     spr, members: [] } }`.
  2. Loop `PollDocumentArchiveLoad { operation }` until the state is `Ready`, `Cancelled` or `Fault`, then send
     `AcknowledgeDocumentArchiveLoad`.
  
  The guest refuses an empty `parent_spr`, so a document with no history sends `store::empty_document_spr(id, schema)`.
- **Status of the guest side.** Done:
  - the `document.loading` refusal;
  - the `kind: load` reprojection status;
  - cancel through `historyEditCancelReplay`;
  - op-level `completed/total` through S3-W1G's `progress()` hook.
  
  Still open: `load_document_pack/text` stay on the `PluginApp` surface until the bump wave. Their 84 test call sites in 55
  files belong to many owners, and the component host's `LoadDocument` arm goes away with the command itself.

## 9. Session 4 completion (S4-LOAD, 2026-10-04, coordinator-approved 02:20)

- **`MediaIn` of a whole document is a stepped archive load (P5 closed).** `PluginApp::consume_media` answers `MediaConsumption::Applied`
  or `MediaConsumption::DocumentLoad(archive)`; for a `Document{schema}` wire of the app's own schema the data is the one whole-document
  carrier `produce_media("artifact:out")` answers — `pk:` base64 text of `encode_document_archive_bytes(document_archive())` (parent pack +
  `.spr` + owned members; the `Structured{schema, json}` convention of `export_media`, carried losslessly by text-only host edges such as
  `🏃️run`'s `MediaPayload::Structured`) — and the archive loads verbatim (source document and members), nothing folds inside the call. A
  foreign schema is refused under the framework code `plugin.media.schema-mismatch` (`{found}`, `{expected}`, en/de notice). The runtime admits it under the
  `MediaIn`'s own `seq` and answers `AppFrame::DocumentArchiveLoad { in_reply_to: seq, status: Pending }` (no frame-layout change). The host
  drives it with `DocumentArchiveLoadHost::admitted(seq)` (Rust + TS twin, corpus field `admittedByGuest`) — poll / cancel / acknowledge
  exactly like an admission it sent. `🏃️run` closes an import batch at every structured input and drives the load before any later import,
  export or read.
- **The sync surface is gone (P4, T1–T3 closed).** `PluginApp::load_document_pack/text` and `plugin_runtime::plugin_load_document_text`
  are deleted; `AppCommand::LoadDocument` + `plugin_load_document_pack` went with S4-BUMP's channel bump (v21). Tests load through
  `artifact_app_laws::load_document[_text]` (app level) or `artifact_app_laws::plugin_load_document[_text]` (runtime level, the checkpoint
  path).
- **Pure commands carry one `head` (P6 resolved, §20.8).** `AppCommand::PureCommand { seq, command, head }` (bump); the guest's
  `PluginApp::hydrate_pure_head(head)` replaces the three lane hydrations (head-only, O(snapshot), driven by `drive_self_waking_ready`);
  an empty head evaluates on the live document.
