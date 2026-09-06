# DB Worker-Pool Liveness Current Frontier

## Finding

There is no scoped `WorkerPool` use/lifetime guard today. Native and cooperative `WorkerPool` are both clonable public handles, and either exposes a public zero-argument `shutdown` ([async native:1724](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1724), [async native shutdown:1910](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1910), [cooperative shutdown:2396](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:2396)). Its only ownership statement says that dropping a cloned handle must not tear down siblings; it does not reserve `shutdown` to an owner or require retained work to be terminal.

Therefore an early shutdown by another holder is representable by the current public contract, not merely external misuse. No inspected production Hub/DB route currently calls `WorkerPool::shutdown`; the observed calls are tests and explicit process-owner fixtures. That lowers current exposure, but does not make the mount owner's hard-scheduler path safe.

Shutdown is terminal: it sets `shutdown` before firing timer callbacks and workers thereafter select only deferred wakes ([async:1685](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1685), [async:1910](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1910)). A callback which retries a rejected mount can never regain ordinary lane execution. `Poisoned` is separately terminal for an owner: generic timer submission deliberately drops jobs on `Shutdown|Poisoned` ([async:1844](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs:1844)).

## Existing retained-owner pattern — useful but insufficient

`DatabaseCapabilityOpenState` already distinguishes transient queue failures from a nonrunnable terminal submission: it places the exact closure in `terminal_job`, retains work in `terminal_work`, and returns a `DatabaseCapabilityOpenTerminalHandle` ([engine:465](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:465), [engine:753](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:753), [engine:1007](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:1007)). Catalog-read, catalog-bootstrap, create-catalog, artifact-submit, and the artifact runner have analogous terminal-job/work ownership. In particular, `ArtifactRunnerTerminalJob` never discards a failed runner job; it can hand it back or make the runner's explicit close step own it ([artifact:4385](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4385)).

Those are **owner preservation**, not executor liveness:

- each resume calls the same `state.pool.try_submit`, so it cannot revive a shut pool;
- the capability close step may drop its pure backend future, but a mount's `ArtifactEngineOpenRejected::retry_close` can retain a WAL writer and needs bounded close opportunities ([artifact:1202](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1202), [artifact:1243](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:1243));
- no state holds a pool capability which prevents the separate public `shutdown` call.

Do not copy a terminal-handle API alone into document mount and call that recovery. It would preserve the writer but strand it behind the same dead executor.

## Smallest coherent coordinated correction

### 1. Make pool shutdown an exclusive lifecycle transition

Add a repo-owned `WorkerPoolUse` to both target implementations in [async](../../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs), backed by a tiny `Mutex`-protected state in each `PoolInner`:

```text
Open { retained_uses }
Closing
Stopped
```

`WorkerPool::acquire_use() -> Result<Arc<WorkerPoolUse>, WorkerPoolUseError>` increments only while `Open`; final `Arc` drop decrements. `WorkerPool::shutdown() -> Result<(), WorkerPoolShutdownError>` must linearize `Open -> Closing` under that same mutex before it checks `retained_uses`:

- a nonzero count returns `Busy { retained_uses }` and restores `Open`; the pool stays executable;
- zero enters `Closing`, rejects new use acquisition, performs the existing timer/worker shutdown, then records `Stopped`;
- `Stopped` is idempotently successful; a concurrently observed `Closing` is an explicit non-success, never a second join.

This is one cold lifecycle mutex, not a per-job scheduler lock or a new worker/thread. It prevents the acquire-vs-shutdown TOCTOU that a counter checked outside a closing transition would leave. `WorkerPoolUse` is clone-cheap through its `Arc`; copies keep one retained-use cell alive until the final owner leaves.

The current public zero-argument shutdown must not remain as a bypass. Change its return type and migrate current callers to observe the result (test-only owners normally use `expect` after their terminal close). A second `try_shutdown` while keeping the old unconditional `shutdown` would leave the hard-scheduler P0 public.

### 2. Root the lease at the Database lifetime and pass it to escapees

Add `pool_use: Option<Arc<WorkerPoolUse>>` to `Database` in [engine](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7796). `open_with` must acquire it before the first retained capability probe ([engine:7858](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7858)); failure releases it normally. The database retains it until its existing `shutdown_step` has closed every `Opening`/`Ready` authority, graph, and emitter, then takes/drops it immediately before returning `Complete` ([engine:8227](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8227)). This preserves the existing successful order `database.shutdown(...); pool.shutdown()` while refusing the inverse order.

The liveness reference must be cloned into any owner that can outlive the `Database` stack frame:

- `DatabaseDocumentMountOwner` at installation ([engine:7552](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:7552), [engine:8108](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs:8108)); this is the direct repair for an `Opening` with a retained WAL-close owner.
- `ArtifactAuthority::spawn`/`ArtifactRunnerHandoff` for a ready actor ([artifact:4230](../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs:4230)). A direct authority spawn must acquire its own pool use, rather than depending on callers to remember one; a Database-created authority may carry a clone of the database use.
- any public retained root that can escape a `Database` borrow: capability-open, catalog-read, catalog-bootstrap, and create-catalog state. These already have explicit terminal-root accounting in this file; give each its own `Arc<WorkerPoolUse>` at admission and release it only once `terminal_is_empty`. The three `Database::open_*_retained` helpers have no non-test in-repository callers; make them private if no deliberate public retained-open surface is required, rather than exposing an unleased root.

`ArtifactSubmitFuture` need not acquire another use if its `Arc<ArtifactAuthority>` owns the runner use. The `db_io` task/controller and `db_sync` roots have no use guard either; they must acquire one at their own public retained-root admission before the API can claim **global** DB pool-shutdown safety. They are outside the minimal document-mount patch, so do not describe the database guard as covering raw storage/sync consumers.

### 3. Preserve a real executor fault; do not secretly retry it

The mount owner should treat only `Contended|Saturated` as coalesced timer retry. On `Shutdown|Poisoned`, it must discard only its weak driver closure, retain `DatabaseDocumentMountWork` and the exact rejection, record a scheduler-fault witness, and make `Database::shutdown_step` return a new explicit executor-blocked result instead of false `Progress`. The first correction prevents `Shutdown` during a valid DB lifetime; this second correction is still necessary for a poisoned scheduler or a violated/lost process-lifetime invariant.

Do not fan out a normal terminal mount error while a retained WAL close owner remains. Callers need a truthful blocked/nonrunnable result and the database must continue to own the exact work until a supervisor has a valid executor-recovery protocol. The existing capability terminal-handle shape is the right ownership model, but its "resume on the same pool" method is not valid recovery for this mount.

## Required laws

1. Native and cooperative pool laws: a held `WorkerPoolUse` makes shutdown return `Busy`, leaves `is_shutdown == false`, and after final guard drop exactly one shutdown succeeds. A use acquisition racing the successful close must fail, never increment after `Closing`.
2. Database mount law: hold a published/opening mount with its initiating waiter dropped; shutdown of the shared pool must be `Busy`, then a join and release must reach exactly one ready authority. After `Database::shutdown` completes and all handles are gone, the same pool shutdown succeeds.
3. Retained-root law: an escaped create-catalog/capability-open operation keeps pool shutdown busy until its existing terminal owner is drained; no storage/page/job identity changes.
4. Poisoned-admission law (test-only controlled submit seam): document mount records the executor witness, retains the same `ArtifactEngineOpenRejected`/writer, and `Database::shutdown_step` reports executor-blocked rather than looping or releasing it.

## Scope decision

The immediate mount repair should take the database/authority lease path plus the nonrunnable state. Do not attempt a broad WorkerPool scheduler rewrite or pretend the existing terminal handles solve shutdown. After that narrow repair, stamp the same lease on raw DB I/O and sync retained roots before advertising process-wide shutdown safety.
