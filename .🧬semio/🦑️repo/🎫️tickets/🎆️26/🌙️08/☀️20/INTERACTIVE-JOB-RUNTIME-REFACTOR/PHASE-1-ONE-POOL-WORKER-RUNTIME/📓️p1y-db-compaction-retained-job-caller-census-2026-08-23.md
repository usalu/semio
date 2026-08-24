# P1y DB Compaction Retained Job Caller Census

Date: 2026-08-23  
Status: **PRE-EDIT SOURCE CENSUS.** P1y is not implemented or accepted by this report.

## Selected Production Wait

The P1y facade cut is `Database::compact_document`, which currently synchronously drives one complete `Compactor::run_from_latest_snapshot` pass. That future is not a single bounded I/O wait. It performs a multi-subsystem operation:

1. load the latest snapshot and derive the WAL floor;
2. acquire the per-document compaction lease;
3. replay the complete document WAL;
4. build segment groups/horizons and retention selection;
5. delete selected WAL segments;
6. build payload candidate/live hash sets and delete orphaned payloads;
7. compact every index kind;
8. optionally materialize the snapshot chain, deduplicate and read pages, publish a full baseline, enumerate generations, retain the new floor, and enumerate again;
9. release the compaction lease and publish the report or precedence-correct error.

The existing numeric `CompactionBudget` caps total counts for some collections, but most helpers still execute their entire allowed count within one async poll/grant, allocate dynamic maps/vectors eagerly, and await many backend operations without an explicit resumable state owner.

## Caller Reachability

The direct live facade is `Database::compact_document`. Repository tests also invoke `Compactor::run`, `run_from_latest_snapshot`, `SnapshotConsolidator`, and the lower-level helpers directly. P1y must preserve those batch/test surfaces as adapters over the same job rather than leave a second run-to-completion implementation beside the retained route.

## Required Job Graph

The production compaction owner must be one admitted, cancellable state machine with explicit phases and persistent cursors for every list, record, segment, hash, index kind, generation, descriptor, page, and backend request. Its retained boundary includes:

- exact document and holder `String` capacities;
- the storage owner, lease resource/holder/fence, and release obligation;
- budget/timestamp/consolidation flags;
- replay records and every nested record payload backing;
- segment groups/horizons/selection and their allocated capacities;
- candidate/live hash-table buckets and entries;
- index reports and any retained index merge owners;
- snapshot descriptors, combined archives, page vectors/page byte backings, dedupe tables, generation lists, and publication pages;
- report/error/completion owners and retry intent.

The state machine must prioritize lease release after every partial failure, cancellation, stale generation, or panic. A failed release must not overwrite an earlier run error, matching the existing error-precedence contract; a release error becomes terminal only after successful work.

## Boundedness Gaps In Current Helpers

The following current helpers are whole-operation or eager-allocation boundaries and therefore cannot remain on the live path unchanged: `replay_document`, `group_records_by_segment`, `segment_horizons`, `plan_wal_retention`, `apply_wal_retention`, `sweep_payloads`, `compact_all_indexes`, `materialize_chain`, `collect_chain_pages`, `SnapshotConsolidator::consolidate`, and the generation-list/retain/list sequence. Batch adapters may drive their resumable replacements to completion at entry/test boundaries.

## Verification Obligations

Permanent fixtures and the verifier must prove:

- after P1w, P1x, and P1y the only remaining selected production wait is sync hello;
- low fuel/deadline forces repeated yields through every phase, including map construction, sorting/selection, each delete, each index kind, page union/publication, and close;
- cancellation/staleness is observed within one grant and still releases an acquired lease;
- lease contention, renewal/release faults, storage faults, worker panic, queue/admission saturation, and delayed callback retry preserve exact owners and deterministic error precedence;
- allocated capacity/backing, not logical length or semantic estimates, feeds every item/byte ledger;
- deep terminal cleanup is iterative and releases at most one dynamic backing or fixed bounded unit per governed grant;
- deterministic ordering is stable across worker counts, especially hash-derived payload candidates and page unions;
- no nested executor, recursive dynamic drop, unbounded loop, or full helper invocation remains reachable from the live facade;
- public completion uses check-register-recheck and generation-tagged slots reject ABA-stale callbacks.

Each backend operation may remain one explicit Phase 9 indivisible-latency residual poll. Native, Wasm, browser, stress, and timing validation remains deferred to the serialized build matrix after overlapping Rust source packets are quiescent.

## 2026-08-24 Independent RED Remediation Clarification

The retained production graph owns one shared cancellation atomic and passes that exact identity into every retained index-compaction child together with an eight-millisecond deadline and bounded fuel. A private child token, thirty-second deadline, or 65,536-fuel control is outside the P1y contract.

Snapshot consolidation must publish through one generation-qualified authority. The admitted latest generation is the expected generation; a fixed per-document publication claim serializes observation and write, refuses a mismatched generation before descriptor/page construction or storage write, and returns the exact snapshot-body owner for incremental close. A post-write generation comparison is not revalidation and cannot satisfy P1y.

Lease ownership becomes recoverable immediately after acquire. A panic publishes both the unwound execution owner and the typed lease-release future before releasing the driver claim. Public fault completion is forbidden until the release witness, execution/release quarantine retirement, admission release, and generation-registry drain have all completed exactly once on the shared I/O lane.

Only `Ok(())` from the backend release may consume the retained fence or set the released witness. Every release `Err` keeps fence, resource, holder, storage, admission and registry discoverable, retains the first exact fault, and backs off through a real `WorkerPool::callback_at` before resubmitting the typed release future to `Lane::Io`. A persistent release error blocks public terminal completion; later success incrementally retires the fault before completion.

Backing credit is cumulative across every simultaneously live descriptor, document clone, page owner, snapshot body, list, and publication owner. Credit uses observed `Vec`, `String`, `Arc`, page, and backing capacity after allocation, is returned only after incremental owner retirement, and every refusal/cancel/deadline/stale/fault branch mounts the same retained cleanup. The expected-generation publication path cannot construct a hidden page-hash `Vec`.
