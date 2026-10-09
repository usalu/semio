# Finished Replay Custody Audit

Read-only current Source observation; no compiler or native execution performed.

The actual failure is a defining production transfer defect. Store `EditReplay::finish` (26635) transfers `order` with `mem::take`. VCS `HistoryPageStack::default` (414–417) calls `new` (243), whose `try_new` opens one original resident page (247–249). Thus the residual retained at Store26651 owns a newly allocated empty page even though the original order moved into the result. Logical emptiness does not imply zero physical capacity.

The result retirement guard at replay/cancel169 requires every residual output slot physically empty before moving the result outputs back. It correctly catches this nonzero order capacity. Its own handback uses `mem::replace(..., HistoryPageStack::empty())` (171), which is allocation-free.

The smallest defining repair is the same allocation-free empty replacement in `finish` for the order transfer. Preserve the original result page, residual schema pointer, all empty spare Vec/String capacities, and existing full-grant retirement. Do not discard the newly resident page after construction or weaken the guard. Other output fields should retain their existing original handoff; this report establishes the order default defect specifically.

The original planning test (107–117) constructs real original schema/draft spare capacity and snapshot ownership, then calls genuine `finish`; it is not a malformed parser specimen. It checks original schema pointer and observes complete construction-to-terminal allocator closure, per-axis denial, grant-fitting receipts, and allocation-free terminal Drop. Retain these assertions and the fixed caller policy. No test success is inferred here.

Exact observed Source hashes are in the companion input; all are observations with ownership false. Source may change concurrently.
