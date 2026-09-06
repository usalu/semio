# DB Task-Slot and Result-Lease Decoupling Audit

## Current verdict

The current source has the right ownership split and I found no concrete ABA, double-release, or
page-credit P0 in the new path.  A task slot can retire after its backend/task cleanup while its
operation ledger row remains live for the caller-owned result.  That is the required repair for
the observed compaction exhaustion; keeping the task slot Closing until `DbIoPages` retires would
again cap retained results at the 64 task slots.

This is source review only.  WG reported the source oracle green; I did not run it or any native
law.

## Exact current ownership proof

`DbIoTaskOperation::take` first reserves one result lease against the operation, removes the
terminal result, and marks the task Closing at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4654).
For an owned result, `DbIoResultLease::into_result` installs the one handback cursor in the moved
`DbIoPages`, `DbIoU64List`, or `DbIoLeaseResult`; scalars return the lease immediately at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4343).

The close cursor independently finishes backend cleanup, task input/output cleanup, queue credit,
backend admission, and then `db_io_operation_detach_task` before it frees the task arena slot at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4800).
`db_io_operation_try_release_locked` refuses to recycle the operation row while either its live
page/list/result credit or `result_leases` is nonzero at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:309).

The result owner's final `close_step` returns its resource pages/list/text first, then invokes
`db_io_result_handback`; that returns exactly one operation result credit and only re-enqueues a
close when the full old `(slot, generation, operation)` still matches at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4368).
An already-freed/reused task slot therefore cannot be touched by a late old result.  The operation
id is monotonic and allocation stops before `u64::MAX`, so it is not recycled as a second ABA key.

The same one-shot cursor is retained on direct result drop through `DbIoLostOwner::ResultLease` at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:4327).
That path drains the moved result before the operation result credit.  It does not turn a dropped
result into an unaccounted page owner.

## Required strengthening of the 128-task law

The current memory law is valuable but incomplete for the incident.  It retains one
`DbIoU64List` and performs 128 later `segment_len` calls at
[storage.rs](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs:10493),
with the schema-owned `sequentialTasks: 128` in
[memory-backing fixture](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧪️fixtures/🧮️memory-backing/🔣️.json:7).
It demonstrates task-slot reuse, but does not hold the `DbIoPages` backing which consumed the
reported compaction capacity.

Add one neutral row and one native case which:

1. Creates known one-page data through the actual `MemoryStorage` task route, obtains a `DbIoPages`
   read result, and retains it without closing.
2. Repeats 128 scalar operations.  After every operation, assert the held result's old full task
   handle no longer matches its slot, its old operation ledger row still exists, and that row
   contains both the result-lease credit and the one-page/shell credit.
3. Force reuse of the old numeric task slot before closing the held page result.  Snapshot the new
   owner generation/operation/phase; close the old result; require that snapshot unchanged.  This
   tests result-handback ABA directly; the existing delayed job ABA law does not.
4. Drain page(s), shell, then handback; repeat close/drop maintenance opportunities.  Require exact
   pre-law ledger/page-arena witness only after the final owner is terminal.  A second close must
   be inert, never decrement `result_leases` twice.

For parity with the observed 44 retained results, retain 44 one-page results in a second fixture
row before the 128 scalar operations.  It should remain bounded by the existing page and operation
limits, then show that new task admission is limited by genuine page/operation credit—not stale
Closing task slots.  Include one dropped page-result variant and drain the existing lost-owner
cursor so the result-handback credit is proven under both explicit and Drop retirement.

## Defensive boundary to preserve

`db_io_result_handback` currently returns operation credit before its advisory old-task enqueue.
The normal `take` path has already enqueued that exact task, so the subsequent enqueue is idempotent
and this is not a demonstrated failure.  Preserve that invariant: no code may manufacture a
`DbIoResultLease` without `take` having successfully installed the close cursor.  If future code
makes an old-task enqueue fallible after the credit return, it must either make that enqueue
non-failing/advisory or record successful credit return before retry; otherwise an owner retaining
its handback cursor could retry and double-return the lease.  The direct handback/reused-slot law
above guards the present intended contract.

No product edits or builds were performed.
