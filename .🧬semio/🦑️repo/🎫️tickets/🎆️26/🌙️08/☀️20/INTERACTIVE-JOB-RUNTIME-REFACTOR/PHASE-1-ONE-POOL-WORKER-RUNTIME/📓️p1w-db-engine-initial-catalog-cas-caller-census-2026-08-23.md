# P1w DB Engine Initial Catalog CAS Caller Census

Date: 2026-08-23  
Status: **PRE-EDIT SOURCE CENSUS.** P1w is not implemented or accepted by this report.

## Selected Production Wait

The exact P1w cut is the fresh-storage `None` arm in `Database::open_with`:

- encode the empty catalog;
- transfer the resulting allocation into `DbIoPages`;
- synchronously bridge `storage.catalog().await.cas_root(EpochFence::INITIAL, pages).await`;
- continue construction with the returned epoch and an empty in-memory entry list.

The backend contract makes this a compare-and-swap against the absent-root state: `EpochFence::INITIAL` is legal only when no catalog root has yet been written, success installs the pages at `expected.next()`, and a mismatch returns `DbError::Fenced` rather than overwriting a concurrent root.

## Caller Reachability

All reachability converges on the one private `Database::open_with` implementation:

1. `Database::open` for an arbitrary backend;
2. `Database::open_at` through filesystem storage, then `open`;
3. `Database::open_with_emit` for a supplied event sink;
4. `Database::open_with_authz` for a supplied authorization hook.

Observed downstream reachability includes the DB CLI, the DB facade, DB-engine tests, and the DB testkit/replay harness. `open_with_emit` is a documented public seam with no current repository caller; it remains part of the same live constructor surface.

## Required Retained Boundary

The P1w packet must admit before submission and retain until explicit handback or governed retirement:

- the exact `Arc<DbBackend>` storage owner;
- the exact `DbIoPages` owner, including its allocated byte backing rather than only logical length;
- `EpochFence::INITIAL` and fixed state-machine metadata;
- the backend future/result owner while its one Phase 9 residual poll is mounted;
- the returned storage plus successful epoch, or the storage plus exact error on failure.

Queue/admission failure must return the original storage and pages without silent drop. Cancellation, stale generation, panic, delayed callback, queue saturation, and public-future publication must each have a retained retry/close route. Rejection and terminal cleanup must release at most one dynamic owner/backing per governed grant and reach an observable terminal-empty state.

## Concurrency Semantics To Preserve

P1w must not turn a lost bootstrap CAS into a blind retry with `EpochFence::INITIAL`; that could never legally overwrite the winning root. The existing public behavior is an explicit `DbError::Fenced`. Any proposed reconcile-on-conflict behavior would require a second bounded catalog read plus decoding the winning root and is outside this narrowly selected CAS cut unless its complete ownership and wake protocol is implemented and verified in the same packet.

## Verification Obligations

The permanent verifier/fixtures must prove:

- production `db_actor::block_on` wait census drops from four to exactly three in order: create-document catalog CAS, compaction, sync hello;
- success returns the identical retained storage owner and the backend epoch;
- injected fenced/backend errors return the identical retained storage owner and exact error;
- admission and callback-retry saturation do not lose storage/pages or work intent;
- cancellation/staleness and worker panic publish exactly one terminal result and retire every owner incrementally;
- check-register-recheck prevents public-future lost wakeups;
- slot reuse cannot accept an ABA-stale generation;
- malformed/invalid construction input cannot fault before its owner guard is installed;
- no nested executor, unbounded loop, or caller-thread backend poll is introduced.

Native, Wasm, browser, stress, and timing validation remains deferred to the serialized build matrix after overlapping Rust source packets are quiescent.
