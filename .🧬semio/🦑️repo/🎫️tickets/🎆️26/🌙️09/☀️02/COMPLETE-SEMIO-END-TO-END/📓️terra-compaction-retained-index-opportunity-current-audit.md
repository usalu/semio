# Retained Compaction Index Opportunity Current Audit

## Source result

The current source already retains the *outer* compaction future across ordinary asynchronous I/O: `DatabaseCompactionCore.future` owns one `DatabaseCompactionExecutionFuture` ([`db/🗜️compact/🦀️.rs:1696-1705`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1696)) and `poll_one` takes it only to poll, then puts the same future back on `Pending` ([`~1860-1905`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1860)). `retained_compaction_under_lease` creates the index handle/control once per index kind at [`:1288-1305`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗜️compact/🦀️.rs:1288), not in an explicit retry loop.

The immediate source problem is nonetheless real: the 8 ms/256 boundary is not an executor-visible opportunity. `IndexCursorControl::retained` sets `cooperative`, and `grant` silently calls `std::thread::yield_now()`, resets deadline/fuel, and returns `Ok(())` ([`db/🔢️index/🦀️.rs:123-162`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs:123)). Thus one `handle.compact` future remains correct-owner-preserving, but can monopolize a WorkerPool I/O execution through arbitrarily many nominal 8 ms opportunities. It does not produce a retained scheduler yield.

I cannot substantiate an additional source-level loop that recreates `handle.compact` after each 8 ms deadline in the revision inspected. If the observed native trace has thousands of generation changes, preserve it as an executable diagnosis, but identify the exact changed call site before treating this source as such a loop.

## Required seam

Do not turn `grant` expiry/fuel into an ordinary `DbError` and catch/retry `handle.compact`. The async `IndexHandle::compact` owns live `DbIoU64List`, decoded `RunEntries`, `IndexBytes`, page writers, and the mutation sequence ([`db/🔢️index/🦀️.rs:1030-1065`](../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔢️index/🦀️.rs:1030)). Dropping it loses the exact in-flight merge state. It can also occur after `write_run` but before all `delete_run` calls, so reconstruction is not a proved harmless restart.

The smallest correct first step is an owned async opportunity, not a new retry error:

1. Keep one `IndexCursorControl` and one pinned `handle.compact` future in `DatabaseCompactionCore.future` exactly as now.
2. Make the cooperative grant boundary await an owned one-turn yield (the existing `semio_framework_async::yield_once()` path), then refresh the exact deadline/fuel only after resumption and cancellation recheck. This must cause the pinned outer execution future to return `Pending`; it must not drop/rebuild the index operation.
3. Convert the `IndexCursorControl::grant` uses reached by `IndexHandle::compact` and its nested decode/merge/encode/close helpers to that async opportunity. Leave strict non-retained controls as error-returning, and preserve `Cancelled` as a terminal error rather than a renewable opportunity.

If changing those call sites is unacceptable, the alternative is a real owned `IndexCompactionCursor` whose fixed phases retain `IndexHandle`, run-id list, merge entries, encoded pages, write/delete position, and stats cursor, with `step` returning a non-terminal yield. It must then replace the direct `handle.compact` call. A synchronous error/restart hybrid is not valid.

## Focused executable laws

1. A controlled index operation crosses at least two fuel/deadline opportunities, observes an unrelated queued I/O task between them, and ends with one compaction result (no second `IndexHandle::compact` construction).
2. Force a yield after merge/write but before the second deletion. Resume reaches the saved deletion cursor and leaves exactly one live run with the expected entries and no leaked `DbIoPages`/`DbIoU64List` credits.
3. Cancel while the cursor is parked: it closes all retained owners once, releases the document compaction lease once, and does not publish a report.
4. Inject a storage read/write `Pending` at each phase. The execution future identity remains the same; generation changes only on actual actor scheduling, not on logical restart.

No build/test was run.
