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
