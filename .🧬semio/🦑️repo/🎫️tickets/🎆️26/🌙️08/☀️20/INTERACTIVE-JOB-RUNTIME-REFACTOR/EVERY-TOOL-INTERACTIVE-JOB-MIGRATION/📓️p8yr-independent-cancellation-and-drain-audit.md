# P8yr Independent Cancellation and Segmented-Drain Audit

Date: 2026-08-22  
Scope: cancellation ownership and Rust/WIT segmented-drain foundation only  
Verdict: **REJECT — the claimed O(1), hard-bounded cancellation/close foundation is not accepted.**

Phase 8 and the typed full-operation pipeline remain **RED / REJECTED**. This is a source/static audit only: no Cargo, native, Wasm, browser, or runtime claim is made. The concurrently changing TypeScript browser drain is outside this audit and is not claimed complete.

## Method

Read in full:

- repository `AGENTS.md`;
- attached master plan;
- `p8yh`, `p8yj`, `p8yk`, and `p8yp` reports.

Those reports were treated as hypotheses, then rechecked in the current source. I did not run Cargo, make a modifying Git operation, use ticket lifecycle tooling, or edit production source.

## A. Cancellation ownership and hard-boundedness — REJECT

### What is structurally present

`ToolCancellationHandle` builds the intended fixed-depth token ancestry:

```text
app_scope CancelToken -> per-document CancelToken -> operation CancelToken
```

`begin` hands the operation token to `BatchJobParams.cancel`; old same-document scopes are cancelled before the replacement is installed. `ToolCancellationLease::release_current` compares both document generation and full operation key, so a superseded lease cannot remove the replacement. `CancelToken::cancel_now` is an atomic store; descendants discover a cancelled ancestor by walking the fixed three-node chain, not by cancellation fan-out.

The static Rust tests are attached to live `ToolCancellationHandle`/`VcsArtifactApp` source, rather than being text-only fixtures: one covers live-instance document isolation, and one exercises same-document supersession plus 1,024 descendant tokens and `cancel_scope_generation`. They were not executed because Cargo was prohibited. The source does **not** contain a production call to `cancel_document`; the only call-site found is that test. Thus it does not prove document-close integration.

### Blocking defects

1. The document index is `HashMap<ToolDocumentCancellationKey, ...>` with `parent_document_id: String` and starts as `HashMap::new` (`plugin component.rs:12107-12135`). `begin`, `cancel`, and `cancel_document` clone or allocate that unbounded string, hash it, and mutate the ordinary map (`:12142-12180`). There is no document-ID cap/interned numeric authority, capacity admission/preallocation, or incremental rehash. `insert` can therefore allocate and synchronously resize/rehash the entire table. Average-case hash lookup is not the plan's hard bounded callback guarantee.

2. `begin` holds `state: Mutex<ToolCancellationState>` while it calls `scope.token.cancel_now()` and then inserts the replacement (`:12143-12152`). `cancel` likewise holds the mutex while removing and cancelling; `cancel_document`'s `if let` scrutinee retains its temporary mutex guard through the body, then calls `cancel_now` (`:12156-12180`). Lock contention is unbounded, poison recovery silently takes the poisoned guard, and cancellation occurs lock-held. The verifier only rejects textual scans/collects/drains (`📜️script.ts:1327-1340`), not this violation.

3. `VcsArtifactApp::drop` calls only `cancel_scope_generation` (`:15008-15012`), but Rust then destructs every field. In declaration order the app owns `media_exports: HashMap`, `media_export_documents: HashMap`, and `segmented_downloads: HashMap` (`:12462-12503`). Dropping these maps walks/deallocates every entry. Dropping an `ActiveMediaExport` drops its `ToolCancellationLease`, whose `Drop` calls `release_current`, takes the cancellation mutex, and can remove an entry (`:12222-12229`). A small custom `Drop` body is therefore insufficient: app close is still O(N), may perform lock-held work, and discards queued segmented chunks synchronously.

4. Media-export submission has a second unbounded document-key map, `media_export_documents`, with the same `String` key; its `insert` can rehash, and supersession removes/drops an active export synchronously (`:14303-14306`). `cancel_owned_media_export` also drops the active operation synchronously (`:14413-14426`). Neither a bounded worker cleanup hand-off nor a fixed-capacity admission rule exists.

5. No executable test drops a saturated `VcsArtifactApp` and observes that destruction itself does not scan/deallocate operation collections. Nor is there a test for mutex contention/poison, arbitrary-length document IDs, forced map growth/rehash, a real document-close call path, or close cleanup ownership. The named saturation test only calls `cancel_scope_generation` on a stand-alone handle and retains its leases; it cannot exercise implicit app-field destruction.

These defects disprove the required O(1) supersession/document close/app drop claim even though parent-token cancellation itself does not fan out.

## B. Rust/WIT segmented-drain semantics — source foundation present, but close cleanup is not accepted

The semantic path is present end to end in Rust/WIT:

```text
ArtifactOutputChunks -> segmented_downloads[operation_id]
-> PluginApp::take_segmented_download_chunk
-> plugin_take_segmented_download_chunk(instance_id, operation_id)
-> jobs.take-segmented-download-chunk WIT
-> component Guest Result<Option<Vec<u8>>, PluginError>
```

Source findings:

- `ArtifactOutputChunks::push` rejects empty chunks and chunks over 4,096 bytes, uses `checked_add`, and rejects totals over its exact maximum; it seals before consumption (`plugin component.rs:11369-11415`).
- `ArtifactMediaExportResult` validates exact operation authority through `Arc::ptr_eq` (`same_operation`), rather than only matching bytes (`:11428-11467`).
- `take_chunk` is FIFO `VecDeque::pop_front`, one chunk per call. The app retains the operation after the last `Some`; it removes `segmented_downloads[operation_id]` only after the next `None`, and subsequent calls return `interactive-job.unknown-segmented-download` (`:15597-15604`). The live-source test at `:21863-21874` checks this exact sequence, but was not run.
- The WIT declaration is exactly `result<option<list<u8>>, plugin-error>` (`schema component.wit:1066-1069`). The component guest delegates to `plugin_take_segmented_download_chunk(...).map_err(component::plugin_error)` (`plugin component.rs:20522-20526`); no `.ok().flatten()` occurrence was found in the audited route. Schema parity's exported jobs inventory includes `jobs.take-segmented-download-chunk` (`schema-parity component.rs:243`).

Limits and rejection relevance:

- The generic runtime lookup awaits the instance map and instance mutex (`plugin component.rs:19262-19268`); source alone gives no hard latency proof under lock contention.
- Browser draining is deliberately out of scope and not certified.
- More importantly, application close implicitly drops `segmented_downloads`, which can synchronously walk any number of incomplete downloads and their chunk queues. There is no explicit close/cancellation cleanup transfer to a bounded worker. Therefore the segmented protocol's FIFO/authority/WIT semantics are accepted as a **source seam**, but its required close-cleanup authority cannot be accepted independently of defect A3.

## C. Typed route and fail-closed production state — remains rejected

`dispatch_typed_command_inner` calls `require_complete_tool_operation_pipeline(&admission)?` before `refresh_cache`, snapshots, child construction, serialization, or the old dispatch sequence (`plugin component.rs:14754-14770`). The authority method always returns `interactive-job.full-operation-pending`; there is no `TypedCommandFullOperationJob` implementation. This is correctly fail-closed before preparation. The nine proof rows are not execution authority.

The verifier reports zero admitted complete operations and zero production activations. This static failure closure is retained; it does not turn the old preparation/reducer/commit code into a bounded operation.

## Commands reproduced

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts verify interactivity tool-jobs --self-test` | Exit 0: `self-tests=46 clean`. This exercises verifier fixtures, not Rust execution. |
| `bun ./📜️script.ts verify interactivity` | Exit 0: DENY clean; one recorded allowlisted test-only blocking bridge. |
| `bun ./📜️script.ts verify interactivity tool-jobs --format json --output .../📊️p8yr-ledger-a.json` | Expected fail-closed exit 1. |
| Same command to `📊️p8yr-ledger-b.json` | Expected fail-closed exit 1. |
| `cmp -s` the two P8yr ledgers | Exit 0, byte-identical. |
| `shasum -a 256` both ledgers | Both `1844ed06f3f4840f16b7cf33f79b35fa10a3a2ab0f02c8227014edc718a115a3`. |
| `git diff --check --` root script, plugin Rust, WIT, Scale fixture, and schema-parity source | Exit 0. |

Fresh ledger counts: 50 macro hosts/invocations; 775 rows (773 unique); 656 literal registrations; 0 admitted complete operations; 11 production factories; 0 production registrations; 3 typed dispatches; 4 aliases; 884 remaining command rows; 8 framework-reserved routes; 35 pending importer owners; 34 global payload-store candidates; 46 self-tests; and 7 fail-closed failure classes.

## Required repair before reconsideration

Replace string-keyed/unbounded maps in every UI/close-reachable cancellation and media path with bounded, pre-admitted numeric/interned authority; make lookup/admission and storage growth hard-bounded. Never cancel while holding a mutex. On app/document close, atomically revoke a parent scope only, then transfer all operation-owned maps/chunks to a bounded worker cleanup mechanism; ensure implicit field destruction cannot walk the active collection. Wire a real document-close path, and add executable saturation, contention/poison, forced-growth, stale-generation, close/destructor, and incomplete-segmented-download cleanup tests. Only then can the cancellation and close foundations be reconsidered; typed full-operation and Phase 8 still require their separate complete-operation work.
