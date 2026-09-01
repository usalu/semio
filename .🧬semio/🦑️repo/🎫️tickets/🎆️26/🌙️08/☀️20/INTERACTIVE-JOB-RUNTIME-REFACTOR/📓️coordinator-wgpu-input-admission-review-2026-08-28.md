# WGPU Input Admission Coordinator Review

## Exact Root and Callback Authority Refinement

After reading the complete executor identity review, the coordinator approved schema-first private root identity through one checked, non-reused crate-local AtomicU64 sequence. Each mint performs one load, checked successor and strong compare-exchange; contention returns Busy without a retry loop, MAX exhaustion is permanent, and cancelled identities are never recycled. Actual root metadata must already be admitted and structurally retained before mint/install or payload allocation. Static storage and concrete queue/key/padding sizes are explicitly counted; this creates no second ledger or capacity allowance.

Private keys never cross serialized process/Wasm transport. Equal-counter live queues, move/address reuse, refused grants, MAX boundary, concurrent mint and post-install failure require actual tests. The sequence is not callback authority: the private affine InputCommitTurn still owns the actual Watchdog and exact installed candidate. No public mutable candidate, copied success verdict, caller nonce, address-based identity or unmetered Arc is accepted. The original baseline five tests remain unchanged and must run before implementation.

## Decision

The coordinator read the complete declaration, its original twenty neutral cases/schema, oracle source, and current EventQueue implementation through close_step. The test-first event/metrics packet is approved; production remains unmounted. The executor is extending two missing metrics laws (event receiver full and input-generation exhaustion) and separate logical/physical backing retirement cases before native RED. Taxonomy owns canonical UI-host package routing/launch metadata; no direct Cargo bypass or generated WGPU publication is authorized.

The canonical boundary must include WindowDelegate and actual native/browser callers. Queue-only acceptance cannot fix void callback ACK/source loss. Exact original event ownership must precede normalization and survive full/cancel/clock/unwind refusal. A result-only returned owned event is insufficient across an unwind boundary. Discrete capacity256, per-event4096 logical bytes, mailbox128, and exclusive8000 microseconds remain unchanged. String capacity and queue backing are separate physical charges. No unbounded native spill queue or implied lossless arbitrary device stream is accepted.

Metrics must reserve all three real receivers before publication. Poison/busy/full, generation overflow, and refused clock verdict commit none; refusal cannot cancel an existing surface child. No arbitrary runtime callback, generic replaced payload drop, wake fan-out, or expensive work may be moved after the watchdog finish call. Fixed prepared visibility is not a mathematical proof of every unmeasured post-finish instruction; final end-to-end timing remains open. Native tests must use the actual Watchdog verdict, not model booleans.

The initial scripted oracle remains declaration evidence until executed through its registered target. Requested native REDs target current real defects first: generation increments on refusal, wrapping overflow, constructor preadmission allocation, and terminal emptiness while backing remains. No eager constructor or legacy accepting fallback is approved.

## Native Verification and Scheduling

The coordinator inspected the common-framework R2 report summary and actual footer: 266 passed, zero skipped,4.898s,Nx0. This supersedes263/2 after the action-bus short-grant conservation repair and canonical document_dsl join. It is not full Plugin/guest/WGPU acceptance. The executor retained complete raw output and a selected131-file capture; the coordinator did not rerun all266 or claim an atomic full closure.

A bounded TestApp<true> missing-feature inventory has a fresh Mutation/Dag source hold. Repeated unrelated stdio builds in workspace target are not grounds for an indefinite global compiler hold. Root read-only resource check found32GiB RAM,10CPUs,51% memory free,295GiB disk free; observed previous stdio PIDs had ended. The executor may run one compile-only inventory in the disjoint retained target with unchanged jobs2, remaining the sole compiler in this fleet. Same-target conflict/incompatible source mutation or actual resource refusal requires renewed coordination. No foreign process is terminated, no outputs are shared, and no timing certificate follows from a concurrent compile.

## Read-Only Resource Output

```text
The system has 34359738368 (2097152 pages with a page size of 16384).
System-wide memory free percentage: 51%
hw.memsize: 34359738368
hw.ncpu: 10
Filesystem      Size    Used   Avail Capacity iused ifree %iused  Mounted on
/dev/disk3s5   926Gi   593Gi   295Gi    67%    4.9M  3.1G    0%   /System/Volumes/Data
  PID  PPID  %CPU %MEM    RSS ELAPSED COMM

```
# Actual Baseline R3 Review

The coordinator read the complete retained R3 report and raw output, plus R2's compile-only diagnosis. After seven test cfg joins, all five unchanged native EventQueue laws actually execute and fail: constructor allocation14336 bytes, generation257 after full refusal from256, event and metrics u64MAX wrapping to0, and terminal=true with14336 bytes of queue backing retained. The final test also observed the original eight-byte String with capacity64. There was no secondary abort;0PASS/5FAIL/63unselected,.070s,Nx1.

The R2 unresolved browser test import was a distinct compiler failure, not five semantic failures. Its existing adapter method bodies were exposed under native cfg(test) without changing non-test native behavior; the browser envelope law itself is queued for real execution. The source oracle's22neutral/seven hostile/three frontier results are separate from these native assertions.

The next approved owned packet retains the actual root/writer/backing and fresh callback authority. No production queue repair or WGPU success follows merely from having this RED. Exact source/report: `📓️ui-host-baseline-five-r3-native-red-2026-08-28.md` and `🧪️member-ui-host-baseline-five-r3-2026-08-28.md`.
