# Archive save and reload persistence (journey step 17, defect D)

## Root cause
Persistence already exists and works; the sourcing host never met its precondition.
- Save control: the sync card (`s-sync-status` > group "sync" > `framework.sync.folder`, en "Folder sync" / de "Ordnersynchronisierung", path field, Attach / Verbinden). Attaching binds the document to a `persistedLocalOnly` folder; the store worker writes the document archive (`.semio/events.semio`, `documents.db`) on every published batch (`FolderArchivePersistenceV1`) and remembers the binding in the device-local event-sourced facet `os.config.local-folders` (localStorage `semio.os.config`). Progress and cancellation already exist (stepped archive load, reconnect band busy state, close-guard save wait).
- Restore control: after a reload the shell offers "Reconnect folder" / "Ordner wieder verbinden" (`#s-folder-reconnect`, role=status band); the person's gesture reloads the folder archive like a fresh load.
- Why sourcing lost the state: the offer was keyed by the booted program's document id (`localFolderReconnectOfferV1`). The puzzle plugin has a stable id; every sourcing boot mints a fresh uuid-v7 store id (`time_ordered_id()` in `🔌️plugin/🧵️instance/📥️staging/🚪️ingress/🦀️.rs`). Verified live: two boots stored ids `01a20b72-...` and `01a20b73-...`; the remembered binding never matched, so no band appeared. Earlier the journey also never attached a folder at all, so nothing was written.

So the four hypotheses: log written (yes, once attached), replayed on boot (no: identity never matches), save command wired (yes, sync card), journey triggered it (no).

## Design
Generic, TS-only, no parallel mechanism; reuse of the sync card and the reconnect band.
1. `localFolderReconnectOfferV1` (`🏛️ShellHost/📎️local-folders/🟦️.tsx`): exact identity match still wins; otherwise the program's most recent binding (bindings are sorted by time-ordered document id, so the last of the program is the newest) is offered. A binding whose document a session already holds is never offered.
2. `SyncAttachTargetV1` folder target gets an optional `documentId` (`🔄️ShellSync/🟦️.tsx`) and `syncAttachDocumentIdV1` honours it for every target kind (`🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts`): the reconnect opens the folder lane under the remembered document, whose archive loads into the fresh program. Edits after the reconnect keep persisting under the same lane (verified through a second edit, reload, reconnect cycle).
3. `reconnectLocalFolder` (`🏛️ShellHost/🗄️shell-uri-documents/🟠️close-document/🟦️.ts`) passes `binding.documentId`.

Event-driven and local-first: the binding facet stays an event log, the archive stays the folder lane's snapshot, no CRUD, nothing shared.

## Tests
- `🏛️ShellHost/📎️local-folders/🧪️tests/🔁️fresh-identity/reconnect-offer.test.ts` with language-agnostic corpus `🔣️.json` (5 offer cases, 3 attach-target cases): `bun test ./...` 9 pass.
- Existing shared corpus `🧫️local-folder-bindings` offers still hold (8 of its 11 component tests pass under bun; the 3 failing are DOM tests that need jsdom/vitest and fail without my change as well).
- Browser: `sourcing-journey.ts` step 17 below; scratch `archive-probe.ts` ran the two-cycle persistence.

## Journey step 17 (rewritten)
"folder save, reload and reconnect keep the curated state": sync card > Folder > path under `.cache/b/p/m/sourcing-data/journey-<lang>-<ts>` > Attach, curate Steel IPE 200, wait for `.semio/events.semio`, reload, assert the reconnect band is offered and the curated state absent, click Reconnect, assert the curated row is back and the band is gone. Result: en pass, de pass.

## Limits and follow-ups
- The wgpu shell has a Rust twin of the offer (`🧊️wgpu/📥️staging/📄️docs/📎️local-folders/🦀️.rs`, `local_folder_reconnect_offer`); it still matches exact ids only and its reconnect does not pass the remembered document. Not changed (corpus parity: the shared corpus is a subset of the new behaviour); add the fresh-identity corpus there if wgpu programs mint fresh ids.
- The web host does not auto-reconnect; by the documented design the person's gesture restores the folder. A silent auto-restore would need a policy decision.
- A path must be typed (absolute); the dev host's `S_DATA_DIR` is not used as a default folder.
- No Rust or wasm change: no rebuild needed. TS is live after a page reload.

## wgpu parity
The wgpu shell's Rust twin now behaves like the TS offer, driven by the same corpus (`🏛️ShellHost/📎️local-folders/🧪️tests/🔁️fresh-identity/🔣️.json`).
- `local_folder_reconnect_offer` (`🧊️wgpu/📥️staging/📄️docs/📎️local-folders/🦀️.rs`): exact identity wins, else the program's newest binding (last of its program); nothing is offered when the booted or the candidate document is attached. `attached` is now a slice of ids (the wgpu shell holds one attached document, passed as `Option::as_slice`).
- `sync_attach_document_id` (same file) is the Rust twin of `syncAttachDocumentIdV1`: the remembered document id, else the program's own.
- `reattach_local_folder` (`🔀️actions/📎️local-folders/🦀️.rs`) calls the new `attach_sync_backbone_as(uri, Some(binding.document_id))` (`🔀️actions/🎬️shell-actions/📎️sync-attach-and-open/🦀️.rs`), which adopts the remembered id for both the native `open_document`/archive restore and the browser host-route attach, so later edits persist in the same folder lane. `attach_sync_backbone(uri)` stays the plain attach.
- Test: `the_fresh_identity_corpus_holds_on_wgpu` in `🐚️Shell/🧪️tests/🧪️wgpu-local-folders/🦀️.rs` (crate `semio-renderer-wgpu-shell-tests-docs-2`); the older shared-corpus test adapted to the slice signature.
- Result (`cargo test --lib -- fresh_identity local_folder`, crate `semio-renderer-wgpu-shell-tests-docs-2`): `the_fresh_identity_corpus_holds_on_wgpu`, `the_shared_local_folder_corpus_holds_on_wgpu`, `the_shared_backbone_folder_door_corpus_holds_on_wgpu` and the band tests pass (10 pass, 2 fail). The two failures are outside this change: `a_remembered_folder_comes_back_after_a_restart_and_a_detach_forgets_it` fails in `restore_folder_archive` ("folder archive source requires a bounded document and schema", store-sync `archive-source.rs`), and `a_browser_folder_keeps_its_archive_through_the_host_route_and_comes_back_after_a_restart` fails in the values fixture ("UI arena descendants remain borrowed at lifetime closure"). Neither calls the changed offer or `attach_sync_backbone_as`.
