# Staged patch (NOT applied): reserved permit lease must retire through its paid cursor

Plugin tree P = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Unproven until `semio-plugin-job`, `semio-plugin-instance-*` compile; stage because it spans 6 files in shared crates.

## Fault
`FrameworkReservedCommitPermit::finish(mut self)` (P/👷️job/🏛️reserved/🦀️.rs:~736) calls `self.lease.finish()` and then lets `self` drop. `lease: ToolCancellationLease` has `Drop` that asserts `thread::panicking()` (P/♻️cancellation/🦀️.rs:284-286). Every non-panicking `finish` therefore aborts with "original cancellation lease requires its paid terminal cursor". Undo/redo/clipboard/checkpoint are framework-reserved routes, so they reach it; the typed-command (apply) route retires its lease through `original_lease_cursor_admit` and survives. Matches W-aggregator-s65: apply passes, panic directly after `U parent_before_undo`.

## Call sites that drop a permit (all must change in ONE edit)
- P/🧵️instance/🧷️command-routes/📥️staging/🧊️reserved-routes/🦀️.rs:98,105,135 (`finish_reserved_emit_producer`, `commit.permit.finish()` in `step_framework_reserved_commit`)
- P/🧵️instance/👷️jobs/📥️staging/🧷️command-routes/🦀️.rs:41,54,58 (latest-wins retire, admit_framework_reserved_commit fault + capacity)
- P/🧵️instance/👷️jobs/📥️staging/🚪️ingress/🦀️.rs:49-52 `finish_reserved_emit_producer`
- P/🧵️instance/👷️jobs/📥️staging/🧾️typed-operations/🦀️.rs:270, P/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs:333,387, P/🧵️instance/📨️dispatch/📥️staging/🎟️producers/🦀️.rs:187,208
- P/🧵️instance/📥️staging/🔒️close-ladder/🦀️.rs:112 (PendingReserved rung), :119 (ReservedCommit rung)
- P/🧵️instance/🦀️.rs:149,155 (`Drop for VcsArtifactApp`: must not drop a live permit; assert empty instead)

## Shape (mirrors the existing `lease` + `lease_close` pair of AgentLanePreview, P/👷️job/🦀️.rs:741 and P/🧵️instance/👷️jobs/📥️staging/🧷️command-routes/🦀️.rs:125-128)
1. reserved/🦀️.rs: `finish(self) -> ReservedLeaseRetirement` (no Drop on the permit, so `let Self{lease,..}=self` destructures). New type next to the permit:
   - `ReservedLeaseRetirement{lease:Option<ToolCancellationLease>,close:Option<Box<ToolCancellationLeaseRetirement>>}`
   - `is_empty()`, `demands(&self,canonical,body)->Result<RetirementDemand,ValueError>` (lease: `copy_bytes:size_of::<ToolCancellationLease>(),capacity_bytes:size_of::<ToolCancellationLeaseRetirement>(),depth:1`; close non-empty: `close.retirement_demands(canonical,body)`; close empty: `release_bytes:size_of::<ToolCancellationLeaseRetirement>()`), 
   - `step(&mut self,canonical,grant)->Result<RetainedCloneProgress,ValueError>`: lease -> `original_lease_cursor_admit(&mut self.lease,grant)`; close non-empty -> `close.close_step(canonical,grant)`; close empty -> `original_lease_cursor_release(&mut self.close,grant)`; always through `cx.consume_retained(progress)` + work from `retirement_work_demand`.
   - `Drop`: `assert!(std::thread::panicking()||self.is_empty())`.
2. Home for the retirements: `pending` (the struct owning `pending_reserved`/`reserved_commits`) gets `reserved_leases: ManuallyDrop<VecDeque>` bounded by `ARTIFACT_LIVE_OUTPUT_SLOTS` (same bound as the commit queue); `ToolOperations` gets one `Option<ReservedLeaseRetirement>` for the producer sites that only hold `self.tools`.
3. Drive: `step_framework_reserved_commit` pops/advances one lease retirement per turn before the next commit (FIFO, like the outcome wait); the PendingReserved/ReservedCommit close rungs and `close-terminal` (P/🧵️instance/🔌️plugin-app-impl/🔒️close-terminal/🦀️.rs:39,119) gain `reserved_leases.is_empty()`; the close-ladder rung quotes `ReservedLeaseRetirement::demands`.
4. `Drop for VcsArtifactApp`: remove the two `finish()` loops; `assert!(thread::panicking()||pending empty)`.

## Law (language-agnostic)
`aggregator_reserved_undo_admits_under_the_original_caller` (already in the tree) is the proof; add a unit law in reserved/ `finish_returns_a_retirement_that_must_be_paid`: `finish` then `step` until `is_empty()`, heap witness receipts equal. Third-party oracle: none feasible (framework-internal ownership); state it.

## Unproven
Whether the undo lease is the only dropper (the `[DEBUG] ledger-lease-drop` backtrace never printed in any log; grep over 6249 logs: 0 hits). Static reading shows `finish` drops it on every reserved path, which is sufficient for the panic.
