# Database Mount Ninth-Law Resume Ordering Review

## Finding

The failed ninth native law does not presently show the retained owner being dropped. Its observed failure is an unintended **joiner-controlled resume**: the second `ensure_document` wakes the shared owner with `resume = true`, so a one-shot writer-unlock fault can be retried and completed before the law observes `Parked`.

This is a source-and-captured-receipt review only. I did not rerun Cargo or a native law. The captured failure is [`law-8.stdout`](🗑️generated/exact-cargo-laws-RIqL0s/00/law-8.stdout): the exact test panics at engine line 13074, `temporary unlock fault parks the retained mount owner`, after 0.09 seconds.

## Exact Interleaving

`mount_document` treats both a new `Opening` owner and a joining waiter identically after it drops the registry lock: both return `Some(owner)` and call `owner.drive()` ([`⚙️engine/🦀️.rs:8290-8375`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8290)). `drive` is not a mere wake: it calls `request_drive(true)` ([`:7696-7712`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7696)), setting both `wake_requested` and `resume_requested`.

The law first creates the owner and then immediately polls a second `ensure_document` ([`:13039-13059`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L13039)). The second caller is only a waiter, but it therefore carries an unintended retry authority.

The sole poller then performs this valid race:

1. The mount future returns a retained rejection; `poll_once` installs `Cleanup { rejected.retry_close(), … }` and continues ([`:7814-7816`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7814)).
2. The one injected `fail_next_writer_release` makes that cleanup return `Err(rejected)`. The owner stores `Parked { rejected, … }` ([`:7819-7837`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7819)).
3. A joiner's prior `wake_requested` keeps the polling loop running (`:7848-7866`). Its prior `resume_requested` is consumed at the next loop head **before work is inspected** (`:7801-7806`), changing `Parked` straight back to `Cleanup` through `resume_parked` (`:7738-7743`).
4. The injected release fault has now been consumed; cleanup reaches terminal completion and removes `Opening` before the 100,000-yield observation loop can find it.

The existing racing-resume law deliberately exercises this same sequence: its cleanup-fault hook blocks while the work mutex is held, calls `owner.drive()`, then releases the hook and expects exactly one resume ([`:13162-13217`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L13162)). That validates an explicit controller resume, but proves a join must not use that signal.

## Minimal Coherent Correction

Separate **wake** from **controlled resume** at the call site and preserve a controlled resume request until a parked owner can consume it.

1. In `mount_document`, carry whether the caller installed the `Opening` slot versus joined it. For a new owner and a joiner, call `owner.request_drive(false)` (or rename it to `request_wake`) after releasing the registry. Initial work is `Mount`, so it needs scheduling but no retry permission. A joiner can wake a pending mount; it cannot convert a retained cleanup into a retry.
2. Reserve `request_drive(true)` behind an explicitly named `request_controlled_resume` path. `Database::shutdown_step` is the current real owner of that authority ([`:8453-8459`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8453)); the direct test-only `owner.drive()` in the racing-resume law is its controlled test seam. The Waker remains `request_drive(false)` as it is at `:7659-7664`.
3. In `poll_once`, do **not** unconditionally `swap(false)` on `resume_requested` before matching `work`. Under the sole `work` mutex, consume it only when `work` is `Parked`; if work is `Mount` or `Cleanup`, leave the bit set. This prevents a genuine shutdown resume from being lost merely because it arrived one poll before cleanup created `Parked`.

The critical shape is:

```rust
let mut work = self.work.lock()...;
if matches!(&*work, DatabaseDocumentMountWork::Parked { .. })
    && self.resume_requested.swap(false, Ordering::AcqRel)
{
    Self::resume_parked(&mut work);
}
```

`request_controlled_resume` also sets `wake_requested` and queues an idle owner, so a retained bit is eventually seen even if the mount is currently pending. Coalescing is acceptable: multiple explicit controllers need one retry, while a later concurrent request after the `swap` remains set for the next parked epoch.

## First Laws After The Fix

1. The current `database_document_mount_unlock_fault_parks_exact_owner_until_controlled_shutdown_resume` must observe the exact `(generation, Arc::as_ptr(owner))` in `Parked` after two joins, then observe that same pair after cancelled shutdown, and only controlled non-cancelled shutdown may consume the retained writer.
2. Add a focused joiner race: arrange a cleanup fault hook, have a second `ensure_document` join while the hook blocks, release it without calling controlled resume, and assert `Parked`, `resume_requested == false`, no terminal waiter reply, and unchanged owner pointer. Then call the explicit controlled-resume seam and assert one terminal cleanup.
3. Add the complementary pre-park controller race: issue `request_controlled_resume` while work is `Cleanup` but before its `Err(rejected)` transition. The bit must still be set when `Parked` is installed and exactly one later retry occurs. This is the missing case hidden by the old unconditional `swap(false)`.

No change should use an automatic retry timer or make `Parked` self-resume. The retained rejection carries a real writer owner; only the shutdown/controller path may retry or retire it.

## 2026-09-06 Follow-up — Fresh `uv5IaM` Failure Is Not the Driver

The new failed receipt is
[`exact-cargo-laws-uv5IaM/00/law-8.stdout`](🗑️generated/exact-cargo-laws-uv5IaM/00/law-8.stdout).
It fails the same assertion at engine line 13074. This is **not** a stale test
binary: the engine source timestamp is `03:27:28`, its selected test binary in
`space-public-boundary-sol-target` is `03:29:48`, and the law ran at `03:29:53`.
The post-fix source was therefore compiled into that executable.

Current source also leaves no uncontrolled resume before that assertion:

| Source | Call | Authority |
| --- | --- | --- |
| [`⚙️engine/🦀️.rs:7661-7664`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7661) | wake callback → `request_drive(false)` | ordinary future wake |
| [`⚙️engine/🦀️.rs:8371-8374`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8371) | creator and joiner → `request_drive(false)` | admission wake only |
| [`⚙️engine/🦀️.rs:8455-8459`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L8455) | `shutdown_step` → `drive()` | the real controlled resume |
| [`⚙️engine/🦀️.rs:13197-13200`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L13197) | dedicated racing-resume law → `drive()` | test-only controller |

The ninth law neither calls shutdown nor the separate test seam before it seeks
`Parked`. The `poll_once` guard consumes `resume_requested` only for actual
`Parked` work ([`⚙️engine/🦀️.rs:7801-7805`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7801)). Thus another `true` resume is not the remaining explanation.

### P0: Rejection conversion releases the writer before retained cleanup owns it

The correlated current native receipt
[`exact-cargo-laws-yDnbws/00/law-4.stdout`](🗑️generated/exact-cargo-laws-yDnbws/00/law-4.stdout)
fails `artifact_wal_open_rejection_retains_exact_writer_for_close_or_same_owner_retry`
at [`📝️wal/🦀️.rs:4046`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L4046): immediately after `ArtifactWal::create` returns an
`ArtifactWalOpenRejected`, another `acquire_writer(document)` succeeds instead
of returning `Conflict`. The law has not yet called `rejected_open_error`, so
it proves that the rejection itself failed to retain exclusivity.

The ownership break is exact:

1. A failed `ArtifactWal::create` turns the acquired permit into an open
   rejection at [`📝️wal/🦀️.rs:2412-2417`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L2412).
2. `ArtifactWalAcquiredRejected::into_open_rejected` invokes
   `self.writer.release()` before returning the rejection
   ([`📝️wal/🦀️.rs:2336-2338`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L2336)).
3. `WalWriterPermit::release` immediately marks the fixed release cell
   requested and asks its maintenance controller to run
   ([`🔐️writer/🦀️.rs:25-31`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs#L25)).
4. The controller may retire the guard before any caller has polled
   `ArtifactWalOpenRejected::retry_close`; `retry_close` is the only intended
   cleanup handoff ([`📝️wal/🦀️.rs:2381-2393`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L2381)).

That violates the `#[must_use]` retained-owner contract. It also explains why
the ninth law cannot reliably observe a rejection with an exact retained writer:
it receives a syntactically retained release future, but the fixed
table/controller has already been authorized to retire the backing guard. This
is independent of mount wake ordering.

### Minimal coherent repair

Keep the normal public `WalWriterPermit::release()` eager for ordinary terminal
callers. Add a crate-private **dormant transfer** solely for retained
rejections, for example `into_dormant_release(self) -> WalWriterRelease`.
It moves the same key/epoch release owner without calling `release::request` or
`request_controller`; use it only at
`ArtifactWalAcquiredRejected::into_open_rejected`.

`WalWriterRelease` then needs an internal `started` bit:

1. its first `poll` atomically marks the exact cell requested and asks the
   controller before reading terminal/fault state;
2. `retry` clears only its exact fault, remains/returns started, and requests
   that same controller;
3. `Drop` of an unresolved dormant owner starts nonblocking release as a
   fallback. This is required so an abandoned `#[must_use]` rejection does
   not strand a fixed writer slot. It performs no I/O/allocation and never
   treats completion as acknowledged.

The signal cell's active key, terminal epoch, and ABA fences remain unchanged.
The fix does **not** put I/O in `WalWriterPermit::Drop`, weaken cross-process
exclusion, or auto-retry a `Parked` mount.

Required native regressions, before re-running the mount group:

1. Repair the current WAL law at `📝️wal/🦀️.rs:4032-4067`: re-acquire remains
   `Conflict` until `retry_close` reaches terminal; the acquired same-owner
   `retry_open` branch remains exclusive too.
2. Add a dormant-release/drop law: convert a permit into a retained rejection,
   prove conflict before first poll, drop it, then prove exactly one eventual
   controller release and reacquisition.
3. Re-run the ninth mount law. It must observe the same generation and owner
   pointer in `Parked`, retain it through cancelled shutdown, and allow only
   the uncancelled controlled shutdown to begin the second release attempt.
4. Instrument that mount law's existing test-only cleanup hook to prove the
   one injected release failure was observed; the existing loop only reports
   a missing `Parked`, which otherwise conflates a missed injector with an
   ownership regression.

## 2026-09-06 Follow-up — Dormant Retained Release Repair Review

WGPU's current source repairs the P0 identified above. This is a read-only
review; I did not rerun its source/native gates.

* `WalWriterPermit::release` now only moves the retained release owner
  ([`🔐️writer/🦀️.rs:34-36`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🦀️.rs#L34)). It does not request the fixed cell or the controller.
* `WalWriterRelease::poll` starts the exact request before it observes its
  terminal/fault witness ([`🔔️release/🦀️.rs:147-190`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs#L147)); its unresolved `Drop` starts
  nonblocking cleanup ([`:171-176`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🔐️writer/🔔️release/🦀️.rs#L171)).
* `ArtifactWalAcquiredRejected::into_open_rejected` still performs the sole
  conversion at [`📝️wal/🦀️.rs:2336-2338`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📝️wal/🦀️.rs#L2336), but that conversion is now dormant, so it cannot release the
  guard before `retry_close` owns the first poll.
* The mount driver still has only false ordinary wakes; its ninth law now
  blocks on the cleanup-fault hook and explicitly injects the sole controlled
  resume. `poll_once` only consumes `resume_requested` for `Parked` work
  ([`⚙️engine/🦀️.rs:7733-7736`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️.rs#L7733)).

I found no remaining source-level writer-owner escape in those three seams.
Retain one end-to-end regression in addition to the existing table laws:
convert an acquired writer to an `ArtifactWalOpenRejected`, assert a competing
acquire is `Conflict` **before its first close poll**, drop that unpolled
rejection, then observe exactly one controller-driven unlock and successful
reacquire. This proves the dormant transfer and Drop fallback together rather
than only their independent unit behavior.
