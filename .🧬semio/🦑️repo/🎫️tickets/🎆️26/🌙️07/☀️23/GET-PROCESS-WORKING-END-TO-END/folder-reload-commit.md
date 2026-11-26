# Folder reload: commit publishes 0 batches, replacement initializer fault

## Root causes
1. **Commit published 0 batches.** The original replay publication (history-edit finalize, undo, redo) appends its batch message to the Store's `original_outbox` (phase 8/10), but nothing drained that outbox for time travel. Only ToolRun called `advance_original_outbound`, and that sent to `Backbones::Channel` only (`_ => Ok(None)`), so a bound `Backbones::Memory` document port never received it.
2. **Replacement-initializer fault** (`installed operation wire has no original receiving capability`). `AppDocumentStoreOwnerProducer::admit_sources` (artifact-app-trait) built the native replay initializer's catalog with `build_document_store_owners`, whose wire is the canonical JSON wire. The persisted wire was only installed on the live Store and the maintenance-ladder replacements, not on the folder-load initializer Store.
3. **Fixture turn bound on large archives.** No longer reproduces: with 2 fixed, the 360 KB archive load completes inside the turn bound (it was the initializer retry loop, not a bound).
4. (found while verifying 1) `ArtifactEnvelope::retire_unadopted` never popped `transitions`, so the first merge of a history-edited archive aborted with "artifact history ledger reached Drop before every exact entry owner was retired".

## Fixes
- `store/backbone`: `MemoryBackbone::send_original` (same custody handoff as `ChannelBackbone::send_original`); `ArtifactStore::advance_original_outbound` routes `Backbones::Memory` to it.
- `plugin/time-travel/store-state/finalize`: new commit stage 3 drains the outbox after the visibility decision and before the receipt (`take_receipt` -> 4, `acknowledge` -> 5, done -> 6).
- `plugin/declaration/app/artifact-app-trait`: `AppDocumentStoreOwnerProducer` uses `A::persisted_document_store_owners`.
- `store/envelope`: `retire_unadopted` pops `transitions`.

## Results (`semio-plugin-tests-07-3`, `cargo test -j 2 folder_reload_route`): 9 of 9
- Preview: not a product fault. The draft preview is a paid job; the law now pumps `has_pending_work` before each preview read.
- Document id: a fresh program births a new time-ordered id; loading the folder's archive replaces the envelope and restores it. The law asserted it before the load, which no seam can satisfy; the assertion now holds after the load (decided: the archive owns identity).
- Alternatives (decided: the trunk is implicit): `envelope.vcs.alternatives` holds named alternatives only. The trunk row was inserted by two ingest paths, never by the local branch commit, so live authors and ingesting/restored programs disagreed. Both ingest paths now skip it: persisted-document hydration (plain and original steps), the text parse of a document, and the store's batch fold (`store-owners-batch-fold-and-history`). Presentations prepend the trunk (`trunk_alternative_id()`, empty name): the history alternatives list (`history_alternatives`) always lists it first; the history panel hides the section while only the trunk is listed.
- Deviation from the decision: `fold_history` (replication causal transition fold) still lists the trunk row with its checkpoint chain in `HistoryFold.alternatives`. It is the fold's derived result, not a stored row, and its consumers (frontier heads, CLI, its own unit laws) use the chain; hydration and ingest drop it at the store boundary instead. Removing it from the fold needs a separate trunk-chain accessor for those consumers.
- Laws and twins updated: viewer-head corpus (`alternatives: []` is now `[""]` everywhere, TS `listed` prepends `""`, Rust `listed` prepends `""`, the registration law lists `["red","blue"]` stored), supersede-replay trunk law (the envelope lists `["edited"]`, the trunk is never a row), folder-reload-route `route_view` (prepends `""` once a named alternative exists; TS twin unchanged because it already listed `["", ...]`).
- Not run: store test crates `semio-os-store-tests-08` (viewer-head) and the supersede-replay host do not compile at baseline (`LawOp: OpBinaryReceiving`, `RetainedAllocationEffect` not found in `rejected-page-close`), so the updated viewer-head and supersede-replay laws are unverified. The plugin-side host `semio-plugin-tests-07-2` (time-travel-part-1, history alternatives) does not compile at baseline either (`semio_framework_async` and `Value` unresolved in the unit-3 mount), so the history-panel and alternatives presentation laws there are unverified too.

## Changed files
- `framework/products/os/modules/store/document/history/hydration/.../change-hydration/🦀️.rs`, `store/io/text/text-files-parsed-document-and-ops-log/🦀️.rs`, `store/engine/store-components/lifted/store/store-owners-batch-fold-and-history/🦀️.rs`, `store-log-operations/🦀️.rs` (doc): trunk never stored
- `plugin/instance/stores/staging/dispatch/🦀️.rs`, `plugin/time-travel/panel/lifted/command/🦀️.rs`: presentation prepends the trunk
- `store/tests/viewer-head/{🦀️.rs,🟦️.ts}`, `store/tests/fixtures/viewer-head/🔣️.json`, `store/tests/supersede-replay/trunk-history-shape-and-command-decode-laws/🦀️.rs`
- `framework/products/os/modules/store/backbone/🦀️.rs`
- `framework/products/os/modules/store/engine/lifted/store/delegating-operations-and-publication/🦀️.rs`
- `framework/products/os/modules/plugin/time-travel/store-state/finalize/🦀️.rs`
- `framework/products/os/modules/plugin/declaration/app/artifact-app-trait/🦀️.rs`
- `framework/products/os/modules/store/envelope/🦀️.rs`
- `framework/products/os/modules/plugin/tests/folder-reload-route/🦀️.rs` (preview pumps, identity asserted after the load)
No law added yet for the Memory outbox send (the folder-reload laws cover it end to end).
