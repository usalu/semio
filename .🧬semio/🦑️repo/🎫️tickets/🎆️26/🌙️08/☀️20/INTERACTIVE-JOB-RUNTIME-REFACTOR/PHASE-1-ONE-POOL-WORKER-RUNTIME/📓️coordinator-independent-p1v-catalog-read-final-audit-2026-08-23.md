# Coordinator Independent P1v Catalog Read Final Audit — 2026-08-23

## Verdict

**REJECT — two source ownership/liveness blockers remain.**

The packet correctly removes exactly one production `db_actor::block_on` group, mounts the retained
I/O-lane catalog read in `Database::open_with`, returns the storage/key/root owners by value, and
provides fixed admission, coalesced backend wakes, saturation retry, panic/cancel/stale handling,
and public terminal recovery. The future's consumer wake registration nevertheless has a lost-wake
race, and the public rejected-owner close releases two owners in one advertised one-owner grant.

No production or verifier source was edited by this audit.

## Blocking findings

### 1. Completion can race between the consumer check and waker installation

`DatabaseCatalogReadFuture::poll` first takes/checks `state.completion`, then later locks
`state.waker`, installs `context.waker().clone()`, and returns `Pending`. `publish_staged` separately
writes `state.completion` and then takes/wakes the current `state.waker`.

The following legal interleaving loses the only wake:

1. consumer poll observes no completion;
2. worker publishes the completion;
3. worker observes no registered waker and returns;
4. consumer installs its waker and returns `Pending`.

No later catalog event is required, so `Database::open_with` can remain pending forever despite a
published result. The controlled backend-wake fixture exercises backend Pending/Ready successors,
but it does not place publication precisely between the public future's completion check and waker
registration. The permanent verifier therefore accepts the race.

Use one atomic/mutex-protected completion+waker protocol, or install the waker and recheck
completion before returning Pending. Add a controlled public-future fixture for completion in that
exact window plus a mutation that restores check-before-register behavior.

### 2. Rejected close bulk-releases storage and root key

`DatabaseCatalogReadRejected::close_step` calls `storage.take()` and, in the same branch,
`key.take()`. That contradicts the packet's public one-owner-close contract and its own caller census
requirement to release at most one exact storage/key owner per grant. `close_and_take_error` also
invokes only this bulk step before returning the error.

Retire storage and key in separate observable grants. The mounted admission-rejection path must
retain unfinished close ownership after returning its error rather than relying on an ordinary
drop, and the verifier needs a mutation/fixture that distinguishes the current two-owner branch.

## Properties that pass

- Production DB-engine wait census is exactly four in order: initial catalog CAS, create-document
  CAS, compaction, and sync hello. The removed direct catalog-read bridge has zero matches.
- Admission is a fixed 64-slot, generation-keyed registry with 8 items/64 KiB per operation and
  512 items/4 MiB aggregate checked credits.
- `DatabaseCatalogRootKey` is non-Clone and enters the backend future with the exact `Arc<DbBackend>`.
- The backend future is polled only through the supplied process `WorkerPool` I/O lane and returns
  storage/key/root together; oversized output remains retained for terminal cleanup.
- Handoff, poll, work retention/drain/release, result retention, publish, and terminal phases are
  explicit. Backend wakes and pool saturation retain exact successor jobs.
- Public terminal handles expose work/result checkout, take/resume/close, generation protection,
  Drop handback, and terminal emptiness.

## Gates rerun

| Gate | Result |
|---|---|
| Rust-2021 scoped `rustfmt --check` | PASS |
| `bun ./📜️script.ts verify interactivity --self-test --format json` | PASS, DENY clean; the two false-negative classes above are not represented |
| exact production wait census | PASS, four |
| scoped working diff check | PASS |
| Cargo/Nx/native/Wasm/browser/runtime | Not run while overlapping Rust source packets are active |

P1v remains source-rejected until both repairs receive a fresh independent audit. Phase 1 remains
RED regardless, with four wait groups and the Phase 9 backend-poll latency residual outstanding.
