# Artifact Runner Terminal Owner And Backend Registration Saturation Audit

Status: read-only source audit on 2026-09-06. No build or product edit was
performed. This packet covers the current dormant writer release repair, the
remaining weak-runner owner loss, and the all-tier registration rollback loss.

## Dormant Writer Release: Current Boundary Is Coherent

The earlier pre-poll unlock was repaired in the current source:

* [`WalWriterPermit::release`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs#L34) now transfers `WalWriterRelease` without requesting its cell.
* The first retained `Future::poll` requests the exact cell/controller, and
  unresolved `Drop` starts nonblocking cleanup
  ([`writer/release/🦀️.rs:147-199`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs#L147)).
* `ArtifactWalAcquiredRejected::into_open_rejected` therefore moves a dormant
  close owner, not a pre-authorized unlock
  ([`wal/🦀️.rs:2336-2338`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L2336)).
* The document mount driver has no ordinary `true` resumes. Its waker and
  creator/joiner scheduling use `request_drive(false)`, and `poll_once`
  consumes a resume only for `Parked` work
  ([`engine/🦀️.rs:7645-7659,7733-7736`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7645)).

I found no further source-level escape in these three seams. Keep one
integration law that converts a writer into an open rejection, asserts a second
acquire is `Conflict` before **any** close poll, drops the unpolled rejection,
then observes exactly one controller retirement and a successful reacquire.
That proves both the dormant handoff and its Drop fallback.

## P0: A Terminal Artifact Job Can Outlive Its Only Runner

`ArtifactRunnerHandoff.close_runner` receives a closure built from
`Weak<ArtifactRunner>` at
[`db/🗿️artifact/🦀️.rs:4301-4316`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4301). A rejected post-ready pool submission stores its exact Job in
`handoff.terminal_job` ([`:3970-3978`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3970)); cancellation while a retry is parked and retry-limit terminalization do
the same at [`:4011-4019`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4011) and [`:4024-4039`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4024).

The fault is possible without a race:

1. An already-ready `ArtifactAuthority` gets a terminal rejected Job and the
   caller takes it with `take_terminal_job` ([`:4362-4364`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4362)).
2. The caller drops the authority. The `cancel` closure was the remaining
   strong runner owner; the handoff only has the weak close closure.
3. `ArtifactRunnerTerminalJob::close` calls it; failed `upgrade()` returns
   `true` ([`:4442-4447`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4442)), so the Job is dropped even though dropping the runner may have destroyed an
   engine, pending builder, turn, history cursor, or WAL close owner without
   a terminal close witness.

### Bounded correction

Do not make the normal `Wake` strong. It correctly remains weak. Instead, when
the runner first places an exact Job in `terminal_job`, promote the handoff's
close continuation to a closure that **strongly captures that `Arc<Runner>`**.
This temporary self-cycle is intentional retained terminal ownership. On the
single `finish` transition, clear that strong continuation after the runner has
retired builder/turn/engine and sent `done`; the currently executing `Arc` keeps
it alive until the call returns.

Both [`ArtifactAuthority::close_step`:4371](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4366) and
[`ArtifactRunnerTerminalJob::close`:4444](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4442) currently invoke the continuation while holding the
`close_runner` mutex. They must clone the `Arc<dyn Fn...>` out, release the
mutex, then invoke it. Otherwise the promoted callback's `finish` cannot clear
its continuation without self-deadlocking. `close` only drops the exact Job
after the runner reports terminal; otherwise it restores that Job. `resume`
and `Drop` keep their current exact handback semantics.

### First laws

1. With a ready runner, inject a hard `try_submit` refusal, take its terminal
   job, drop `ArtifactAuthority`, then close the job. Assert the runner's
   builder/turn/engine close probe reaches its terminal witness once, the
   strong continuation is removed, and the retained pool use can later
   shutdown.
2. Repeat with `resume` refused: the same Job returns to `terminal_job`, its
   runner stays strongly retained, and later `close` works.
3. Drop `ArtifactRunnerTerminalJob` without action, recover it through the
   authority, and assert the exact job reason/owner is returned. This catches
   overwrite of an existing handoff job.

## P0: Registration Discards An Unparkable Backend Owner

`db_io_park_lost_owner` correctly returns `Err(owner)` if primary, overflow,
and quarantine are all full ([`storage/🦀️.rs:3893-3906`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L3893)). Backend registration nevertheless ignores that exact error in five
branches:

| Registration branch | Current line | Owner lost when all tiers are full |
| --- | --- | --- |
| owner-credit reservation fails | [`:2696-2701`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2696) | executor, pool and pool use |
| discovered backing differs | [`:2719-2721`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2719) | executor plus reserved operation/credit/use |
| bind owner-operation fails | [`:2723-2725`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2723) | same |
| backend control capacity/generation refuses | [`:2727-2731`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2727) | same |
| writer-control/controller install fails | [`:2737-2741`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2737) | same, after backend binding |

The current rejected-backend law only saturates the **rejected-backend
registry**, then places an owner successfully in the general lost-owner ring
([`:9825-9861`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L9825)). It does not cover all three lost-owner tiers full, so it cannot prove those
five ignored results safe.

### Smallest coherent registration contract

Add a fixed backend-registration rollback reservation, acquired **before** the
function consumes an executor or allocates pool-use/operation credit. It owns a
specific preallocated backend rollback cell in `Empty | Reserved | Retained`;
it is not another best-effort call to `db_io_park_lost_owner`.

* On a post-consumption failure, commit the exact executor, pool, optional
  pool-use, operation and credit into the reservation. Its existing
  `db_io_park_rejected_backend`/lane-close state machine supplies the terminal
  close. On successful registration, cancel the reservation before publishing
  the registry slot.
* If reservation admission itself is full, the incoming executor has not been
  consumed. Return an owner-bearing rejection, not `DbError` after an implicit
  drop. A minimal greenfield shape is
  `DbIoBackendRegistrationRejected { cause, executor, pool }`, with a retained
  `close_step` that acquires a rollback reservation later or reports the same
  owner unchanged. It must not fabricate success or synchronously destroy an
  unbound executor.
* After the owner-credit reservation, enrich that same rejection with its
  exact `operation`, `credit`, and `WorkerPoolUse`. Its retry/close moves the
  owner only once into the reserved rollback cell and proves the ledger returns
  to its prior witness.

This is better than a generic lost-owner preflight: holding a general ring
placeholder would require maintenance to understand a non-closeable marker,
can block unrelated retirement, and still leaves no owner to return when all
tiers are saturated before registration begins.

The production call sites that must adopt the owner-bearing error are:

* [`FsStorage::open`, storage/🦀️.rs:7766-7773](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L7766)
* [`SqliteStorage::open`, storage/🪶️sqlite/🦀️.rs:721](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🪶️sqlite/🦀️.rs#L721)
* [`PostgresStorage::open`, storage/🐘️postgres/🦀️.rs:843](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🐘️postgres/🦀️.rs#L843)
* [`Neo4jStorage::open`, storage/🌐️neo4j/🦀️.rs:1000](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🌐️neo4j/🦀️.rs#L1000)

The remaining references are in-module test adapters. Do not keep a
`Result<DbIoBackendControl, DbError>` compatibility overload that would make
the unowned-error path callable again.

### First native laws

1. Fill primary, overflow, quarantine, control and rejected-backend capacity;
   attempt registration with a close-probed executor. The returned rejected
   owner still contains that executor and no operation credit/use has leaked.
   Free one rollback cell, call its explicit retry/close, and prove exactly one
   lane retirement and exact prior ledger.
2. For each of the five listed post-acquisition failures, prove the committed
   rollback cell carries the same executor pointer, operation/credit and pool
   use; fault once during close, require a retained retry owner, then terminal
   close. No competing pool shutdown succeeds before that terminal witness.
3. On all four storage constructors, force rollback reservation refusal and
   prove their open future returns the retained rejection rather than dropping
   a backend. Then release capacity and close it without creating a storage
   control.

## Follow-up: Strong Terminal Job Fixes Weak-Runner Escape, But Needs Exclusive State

Current source has replaced the proposed strong `close_runner` cycle with the
better terminal-only anchor. [`retained_terminal_job`
`db/🗿️artifact/🦀️.rs:3943-3947`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3943)
creates a Job which strongly captures the exact `Arc<ArtifactRunner>` and calls
`run_turn(generation)`. Ordinary scheduled work remains weak at
[`3949-3965`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3949).
Every current terminal transition replaces its discarded weak submission with
that strong Job: hard refusal at [`3983-3985`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3983),
cancelled retry at [`4021-4025`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4021),
and retry-authority exhaustion at [`4035-4043`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4035).
`take`, rejected `resume`, and `Drop` retain the exact strong Job
([`4371-4372`, `4442-4467`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4371)).
This removes the prior weak-upgrade data-loss path without introducing a
runner--handoff reference cycle. Both close callers now clone the callback out
of its mutex before invocation ([`4380-4381`, `4454-4455`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4380)),
so the former callback-lock self-deadlock is also gone.

### Remaining P0: No Terminal-Authority Latch

`terminal_job: Some(...)` is not an execution state. In the cancellation and
retry-exhaustion paths, `scheduled` is explicitly reset to `false` immediately
before that Job is stored ([`4021-4024`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4021),
[`4033-4039`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4033)).
The runner's own `terminal` flag remains false until `finish`, and `schedule`
only tests that flag plus `scheduled` ([`3949-3953`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3949)).
Thus a mailbox wake can submit ordinary weak work while a terminal Job is
parked. `take_terminal_job` makes it worse: it empties the only slot
([`4371-4372`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4371)),
but does not mark that its single retained owner is now external.

The concrete loss interleaving is:

1. A retry becomes a parked terminal Job and sets `scheduled = false`.
2. A receiver/waker schedules a normal weak Job. It can enter `run_turn`,
   mutate/finish the same engine, or hard-refuse.
3. On a second hard refusal, `submit_exact` unconditionally assigns
   `terminal_job = Some(...)` at [`3983-3985`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3983),
   replacing and dropping the first terminal continuation. If it was taken,
   the new handoff owner is no longer coordinated with that external one.

The strong capture prevents premature `Arc` destruction, but it does not make
the terminal continuation a sole owner. That remains a close/retry correctness
failure, not merely a test-coverage gap.

### Minimal Correction And Laws

Add a single atomic terminal-authority state to the handoff/runner, with at
least `Runnable | Parked | Resuming | Closing | Terminal` (a compact tagged
atomic is sufficient). It is distinct from the completed `terminal` bit.

* All three terminal-placement paths atomically enter `Parked` before exposing
  the strong Job. `schedule` and wake must return while `Parked`, `Closing`, or
  `Terminal`.
* `take_terminal_job` retains `Parked`; it only moves the owner. `Drop` and a
  refused `resume` restore `Parked` and the same Job. No path may overwrite a
  present/external terminal owner.
* A successful `resume` first changes to `Resuming` and marks the one submitted
  job scheduled; it becomes `Runnable` only when that job starts its retained
  turn. A successful close advances `Closing -> Terminal` through `finish`.
  This prevents a wake between pool admission and `run_turn` from submitting a
  duplicate job.

Add three exact native traces:

1. Park a cancelled/retry-exhausted job, fire the consumer wake and enqueue a
   message, then prove the pool receives no ordinary second job and the same
   terminal reason/continuation remains available.
2. Take the terminal Job, fire the same wake while it is external, then drop
   it. The slot regains exactly that job and no runner turn occurred.
3. Resume into a gated pool; fire a wake before the submitted job enters
   `run_turn`; prove one queued turn only. Refusal restores the same strong Job;
   close then reaches one engine/WAL terminal witness.

Use one packed atomic driver state rather than an independent
`terminal_pending: AtomicBool`: a check of two atomics admits the very wake
racing terminal placement. `schedule` must CAS only `RunnableIdle -> Queued`.
Parking owns `Queued -> Parked`; `resume` writes `Queued` **before**
`try_submit` and restores `Parked` only on refusal, so a fast worker cannot
make a later success store overwrite its `Queued -> RunnableIdle` transition.
`close_step` must treat `Parked` as active even when `take_terminal_job` has
moved its slot externally; shutdown is allowed to win `Parked -> Closing` and
drive the runner, while the external Job later becomes a harmless terminal
continuation. This gives one linear owner without a parallel boolean race.

## Coalescing-Law Fixture: `submit_at(now)` Is The Correct Test Release

The current mount coalescing law releases its intentionally blocked
`mount_catalog_published_hook` with
[`pool.submit_at(pool.now_ms(), Lane::UserVisible, ...)`
`engine/🦀️.rs:12997-13006`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L12997),
rather than a one-shot `try_submit`. That is coherent. Native and cooperative
`submit_at` retain the exact closure in the timer wheel and retry only
transient `Contended | Saturated` admission at a later tick
([`async/🦀️.rs:1866-1887`](../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L1866));
the release callback therefore cannot be misclassified as test starvation just
because the shared worker briefly owns its queue mutex. It introduces neither
inline execution nor a sleep.

The helper deliberately drops only on `Shutdown | Poisoned`. That would be
wrong for a durable production owner, but this closure solely unblocks the
test's own condition variable and shutdown occurs after awaited joins. Keep the
fixture's current ordering; no production scheduling policy should adopt its
fire-and-forget ownership semantics.

## Current Re-audit: Runner Poll Ownership And Mount-15 Shutdown

Status: this addendum is source-only. The supplied native receipt
`exact-cargo-laws-oanQof/00` reached
`database_concurrent_ensure_mounts_one_actor_and_one_writer` and failed only at
[`engine/🦀️.rs:12451`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L12451): after a successful database shutdown, the test pool still reports
`Busy { retained_uses: 1 }`. I did not run a build.

### P0: Closing A Pending History Must Be Waker-Gated

The formerly missing terminal latch is materially improved. The current packed
driver distinguishes ordinary polling, wake-during-poll, externally parked
work, and the closing equivalents
([`db/🗿️artifact/🦀️.rs:3886-3896`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3886)). The normal and direct-close turns CAS into
`Polling` and `ClosingPolling` respectively
([`:4205-4231`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4205)). Thus the earlier concurrent direct-close/queued-job double-poll
interleaving is no longer present in the current design.

There is still one non-negotiable condition for correctness: after a direct
history poll returns `Pending`, a later `shutdown_step` must **not** poll again
until its stored `Waker` has promoted a closing parked state to a closing-ready
state. `ArtifactRunnerClosePoll::drop` is now designed for exactly that
handoff: no wake produces `ClosingParked`, while a wake during the poll produces
`ClosingReady` ([`:3916-3928`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3916)); `schedule` performs the corresponding
`ClosingParked -> ClosingReady` promotion ([`:4032-4056`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4032)).

At this captured source instant, the public close callers still refer to the
predecessor `ClosingIdle` spelling at
[`4440-4447`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4440),
[`4517-4521`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4517), and
[`4539-4543`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4539), while the enum has the newer names. This is an in-flight source
update, not a qualification result. The final wiring must satisfy all of the
following; otherwise it is a P0 retained-future violation:

* `close_runner` may CAS and call `run_turn(..., true)` only from
  `ClosingReady`. It must return incomplete on `ClosingParked` and never turn
  that state into ready itself.
* `close_step`, `terminal_is_empty`, terminal-job `close`, and `Drop` must use
  the complete current closing state set. An external terminal job must not
  overwrite a close-in-progress handoff.
* A `Wake` during `ClosingPolling` sets `ClosingPollingWake`; its drop becomes
  ready. A wake after it has parked promotes just once. Neither path submits an
  ordinary runnable job while closing.

Add one real gated-history native law: make the replay's first close poll save
its Waker and return `Pending`; call database/authority shutdown repeatedly
without firing it and assert the poll count stays one; fire it once and assert
exactly one second poll and one terminal engine/WAL close. Repeat with the wake
arriving while the first poll is still active. A mutex alone is insufficient:
it prevents concurrent polling but still permits an invalid sequential poll
before readiness.

### The Remaining Mount-15 Use Is Not A Reason To Weaken Lifecycle Checks

There are only two *additional* `WorkerPoolUse` acquisitions on this
first-create path beyond `Database::open`'s database lifetime:

1. `DatabaseCreateCatalogFuture::try_prepare` acquires one at
   [`engine/🦀️.rs:7063`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7063). A normal mount reaches
   `publish_mount_catalog`, consumes the result through `into_parts`, and
   calls `release_success` ([`:8118-8124`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8118),
   [`:5460-5476`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L5460)). That release currently refuses to act if
   `roots_are_empty()` is false ([`:7029-7036`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7029)); it has no later success retry.
2. The mount passes a **clone** of `Database::require_open_use()` into
   `ArtifactAuthority::spawn_with_pool_use`; it does not call `acquire_use`
   ([`:8206-8241`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8206),
   [`db/🗿️artifact/🦀️.rs:4387-4403`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4387)). The authority and handoff therefore carry the
   database's one cell, not a second one. Database shutdown drops its own cell
   only after it has removed and terminally driven the ready authority
   ([`engine/🦀️.rs:8328-8389`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8328)).

The supplied `Busy { retained_uses: 1 }` after `database.shutdown()` therefore
must be diagnosed as a retained owner, not bypassed by changing the assertion
or making `WorkerPool::shutdown` ignore uses. The smallest discriminating law
uses a fresh local pool (not the module's static `test_worker_pool`) and makes
three assertions:

1. immediately after `Database::open`, shutdown is `Busy { 1 }`;
2. after both concurrent ensure waiters resolve and their handles drop, but
   before `Database::shutdown`, it is still exactly `Busy { 1 }`; and
3. after terminal database shutdown, it is `Ok(())`.

If (2) is `Busy { 2 }`, retain the exact create-catalog state and expose a
test-only witness for `roots_are_empty`, registry membership, and its
`pool_use.is_some()` at the `into_parts` acknowledgement. If (2) is one but
(3) remains one, the escaped clone is in the ready/closing authority handoff;
assert the ready registry is empty, `closing_authority` is `None`, and the
authority's handoff as well as public authority cell are released before the
final database cell. This localizes the fault without loosening the valid
global-pool lifecycle fence.

### Backend Registration Saturation Remains Independently Blocking

The all-tier owner-loss P0 in the earlier section remains current. In
particular all five registration rollback branches still discard
`db_io_park_lost_owner(...)`'s `Err(owner)` at
[`storage/🦀️.rs:2693-2741`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2693). The current general primary/overflow/quarantine
implementation explicitly returns that exact owner once all three tiers are
full ([`:3893-3906`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L3893)). Therefore the pre-reserved,
owner-bearing registration rollback design above is still required before any
global DB lifecycle claim.

## Final Runner-State Pass: One External-Close P0 Remains

The stabilized state machine fixes the two earlier issues inside an authority:
ordinary and closing polls have distinct packed states, the normal-poll guard
transitions a cancellation to `ClosingReady`
([`db/🗿️artifact/🦀️.rs:3963-3982`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3963)), and a close poll parks unless a real wake
arrived ([`:3916-3928`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3916)). This is sound for its only concrete
history future: `request_close` changes it to fault-retirement, and every
retirement opportunity explicitly wakes the supplied context
([`:3237-3244`, `:3351-3361`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L3237)). I no longer classify
repeated `Database::shutdown_step` itself as an ungated re-poll; it reaches
`ClosingParked`, from which direct `run_turn` refuses until a wake promotes it.

However, `ArtifactRunnerTerminalJob::close` is still a terminal ownership loss
when the caller is the last driver:

1. A hard scheduler rejection supplies an externally checked-out strong
   terminal Job. The caller drops `ArtifactAuthority`; this remains supported
   because the Job owns the runner strongly.
2. The caller invokes `ArtifactRunnerTerminalJob::close`. It changes
   `Parked -> ClosingReady` and runs the close callback
   ([`:4630-4654`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4630)). A pending History replay correctly makes the
   callback return `false`.
3. The method restores the strong job only in private
   `handoff.terminal_job` and returns `()` ([`:4650-4654`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4650)). The authority is gone and
   the external close object was consumed. Its later wake can at most move
   `ClosingPollingWake -> ClosingReady`; `schedule` deliberately does not
   submit a close turn ([`:4032-4056`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4032)). No reachable owner can call
   `close_runner` again. The runner, history cursor and WAL owner remain live.

This is P0 because it leaks an exclusive writer after an ordinary nonblocking
pending close, not merely because a test is absent.

### Minimal Coherent Repair

Do not silently schedule a weak ordinary job: the runner is in closing state,
and that would reintroduce the single-poller violation. Either shape is valid:

* make `ArtifactRunnerTerminalJob::close(self)` return a retained
  `ArtifactRunnerTerminalClose` when incomplete. Its `close_step` owns the
  strong terminal job, calls the close callback only from `ClosingReady`, and
  returns itself unchanged at `ClosingParked`; or
* retain an internal **strong** close-driver job and submit it only after the
  wake's `ClosingParked -> ClosingReady` transition. It must own the same
  terminal continuation, survive submission refusal with an externally
  recoverable owner, and be cleared only by `finish`.

The first is the smaller API-local correction because the existing terminal
job already models caller-owned retry/close authority. It must not restore the
Job behind an unreachable private mutex.

Add a native trace with a real pending `HistoryReplayFuture`: park and check
out the terminal job, drop the authority, call `close`, observe an incomplete
retained close owner, fire the replay wake, repeatedly advance that owner, and
prove one terminal `done`, writer release, and `WorkerPool::shutdown == Ok`.
The existing `artifact_runner_closing_poll_waits_for_retained_wake_before_next_turn`
is useful state coverage but only manipulates a handoff
([`:4742-4788`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4742)); it cannot expose this unreachable-owner path.

## Current Addendum: Preadmitted Authority Retirement And Open Rejections

Status: read-only source review on 2026-09-06. I did not run Cargo or change
product source. The reported Mount-19, prepared-registration, and all-tier
source laws are therefore **not** treated as native-qualified here.

### P0: Every Terminal Authority Retirement Leaks Its Maintenance Ticket

The new preadmission itself is correctly early: an authority reserves one of
the fixed retirement slots and installs its hook before building a mailbox or
runner ([`db/🗿️artifact/🦀️.rs:4096-4115`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4096)). On normal authority `Drop`, the reservation moves the strong close
continuation, handoff, pool-use cell, and ticket into the global cursor before
requesting maintenance ([`:4117-4122`, `:4809-4819`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4117)). The callback correctly removes the global-row mutex before it invokes
the runner ([`:4065-4075`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4065)), so this is not a callback-under-lock issue.

But the terminal branch calls `remove_maintenance_hook(ticket)` **from the
hook that is currently executing** and ignores its result
([`:4084-4092`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗿️artifact/🦀️.rs#L4084)). `WorkerMaintenanceRegistry::remove` sets `closing` and always returns
`Ok(false)` while `running` is true ([`async/🔔️maintenance/🦀️.rs:134-143`](../../../../../../🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs#L134)). Its subsequent `finish` only clears the running bit; it does not erase a
closing entry ([`:197-208`](../../../../../../🧰️framework/🔨️modules/⏳️async/🔔️maintenance/🦀️.rs#L197)). The cursor, the only copy of its ticket, and its pool-use cell are then
dropped. Thus every terminal callback consumes one of the fixed 64 maintenance
entries permanently. It does not keep the pool-use cell (so a shutdown can
appear healthy), but the 65th otherwise-independent authority fails at
retirement preadmission with maintenance-capacity exhaustion.

This is deterministic, not a hypothetical concurrent request: the callback is
marked `running` before its function is called. It also means a stale/terminal
`request_maintenance` error is not the primary flaw.

#### Bounded correction

Do not turn a normal `remove(false)` into a fire-and-forget public contract.
Add a callback-only terminal disposition, e.g.
`WorkerMaintenanceStep::Retire`, handled in `WorkerMaintenanceRegistry::finish`
after it clears `running`: erase the exact entry regardless of a coalesced
request. `artifact_runner_retirement_step` returns that disposition only after
it has removed the cursor/generation and relinquished its retained owners.
The registry continues to execute the callback outside its mutex; no ticket or
cursor must survive past the terminal callback just to call `remove` again.

Required native traces:

1. A direct maintenance law invokes a hook returning `Retire`; its ticket is
   stale after completion, the slot is reusable, and a request racing while it
   was running cannot re-arm it.
2. Drop and terminally retire 65 lightweight authorities serially on one pool;
   all 65 opens must be admitted and the retirement-generation array must be
   empty after every terminal witness.
3. Repeat while a close poll receives a wake. Prove one close callback, no
   callback under the handoff/global-row mutex, no retained pool-use after the
   terminal acknowledgement, and full maintenance capacity reuse.

### Scoped Source Acceptance: Rejection Owners Are Retained Until Their Close Witness

Aside from the ticket leak above, the new storage-open ownership shape is
coherent at source level.

* `DbIoBackendRollbackReservation` reserves a fixed rejected-backend cell
  before a prepared constructor consumes the executor or pool-use, commits the
  full executor/operation/credit/use tuple, and clears an uncommitted
  reservation in `Drop` ([`db/🗄️storage/🦀️.rs:2564-2615`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2564)).
* `register_db_io_backend_prepared_with_use` turns every post-transfer
  admission failure into `DbIoBackendRegistrationRejected::Retiring`, not an
  ignored best-effort lost-owner result ([`:2974-2993`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2974)). The five constructor paths now pre-reserve before
  registration; current Memory and filesystem examples are
  [`:6831-6846`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L6831) and [`:8172-8181`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L8172).
* `DbStorageOpenRejected::retry_close` preserves either the registration
  owner or the registered control on every incomplete/faulted turn
  ([`:2721-2782`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2721)). The rejected-backend poll takes the executor out of the registry before calling
  `close_backend_step`, then puts it back on false/error; the callback is not
  run under that registry mutex ([`:2827-2902`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L2827)).
* `DatabaseOpenAtRejected` retains the filesystem storage and retries its
  exact `close()` rather than exposing the original cause early
  ([`db/⚙️engine/🦀️.rs:7914-7954`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7914)).

The current nine-case neutral backend-pool-use fixture explicitly covers
rollback-before-transfer, all-tier refusal returning an incoming executor,
committed rollback retaining a pool use, and prepared post-transfer submission
refusal. Its source tests exercise the latter through a blocked Io lane at
[`storage/🦀️.rs:10646-10690`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs#L10646)). Those are useful source evidence only until the pending native selector
runs.

I found no additional source-level owner drop in the `Incoming -> Retiring ->
terminal` retry path: reservation failure restores the incoming owner; a
request fault restores the retiring owner; and a terminal generation mismatch
can only follow registry terminal removal. The remaining acceptance condition
is a native all-tier/blocked-lane law that calls `retry_close` through both a
faulted close and a queue refusal, then proves the prior ledger witness and
`WorkerPool::shutdown == Ok(())` only after the exact terminal acknowledgement.
