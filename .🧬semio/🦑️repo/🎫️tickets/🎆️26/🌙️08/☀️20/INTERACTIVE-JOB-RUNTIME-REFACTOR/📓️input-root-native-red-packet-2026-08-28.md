# Input Root Native RED Packet

## Current Boundary

Canonical source R5 executed original22 cases and new6 root arithmetic vectors with strict Ajv/Node Buffer/BigInt. Native root5 is mounted at `ui/host/📥️input/🎟️admission/🪪️root/🧪️tests/🦀️.rs`. Actual R6 reached31 missing-module/method/field compiler diagnostics and executed zero tests; full output is preserved in [R6](./📓️ui-host-input-root-r6-compile-red-2026-08-28.md). No production identity or queue change. Original five semantic queue REDs remain unchanged.

## Concrete Private Seam

The numeric `granted_physical_bytes` argument below is only a primitive work/preflight bound. It is **not actual resident funding**, a composition permit, or authority to create a live queue/candidate allowance. The live caller still needs its original funded composition/owner binding and simultaneous metadata/backing accounting. No standalone static reservation or second budget is introduced by this primitive test.

The intended private `input_root` module contains `InputRootSequence { last: AtomicU64 }` and `InputRootFault::{Busy,Exhausted}`. The actual `EventQueue` gains private `root: Option<NonZeroU64>`; its private `try_admit_root_with(&InputRootSequence, granted_physical_bytes) -> Result<bool, InputRootFault>` is the shared implementation behind the eventual single process-sequence admission caller. It does not return a root owner. The injected sequence is visible only inside the owning enqueue module/tests; no public caller may supply a counter or root.

Before minting it must preflight the complete actual queue metadata `size_of::<EventQueue>()`. New() is still an unadmitted empty shell and must allocate no discrete backing. One load/checked successor/strong CAS installs the nonzero identity directly into the still-owned queue. The static counter and root field are measured separately, not an independent budget. Existing admitted root retry preserves its identity. Later actual candidate/receiver admission must debit that original root only once and separately admit backing and initialized work; this root primitive is not that entire funding join.

Controlled competition uses a cfg(test)-only thread-local one-shot atomic store immediately after the actual load, before the actual CAS. It deliberately changes7→8 and expects Busy with no installed root. A retry loop would incorrectly mint9 and fail. No production callback, extra counter field, mutex, or public observed-value API is added. That interference hook is test instrumentation, not allocator-failure or scheduler timing proof.

## Five Native Laws

1. All six neutral arithmetic rows through actual EventQueue admission, including zero/short physical grant, exact u64 little-endian bytes, MAX and controlled CAS contention. A test-only System allocator observer covers construction and root admission and requires zero calls/backing.
2. Two equal-generation queues have distinct private roots; moving a queue preserves its root; replacing another at exactly the same stack address cannot reuse its old identity.
3. Actual root installation occurs inside a caught panic while the queue stays outside the closure; retry and later construction prove no root return/remint. This is root-install unwind, not payload allocation failure or callback-tail quiescence.
4. Eight real threads perform four barrier-coordinated single attempts each. Successful roots must be distinct and contiguous; Busy retains an empty queue. Assertions occur after every thread finishes all barriers, preventing assertion-induced test deadlock. No success retry loop exists in the tested callback.
5. MAX-1→MAX succeeds exactly once and three subsequent attempts refuse permanently. Actual atomic/queue/root-field layout is recorded and queue backing remains absent.

Cross-root **admission-key rejection**, stale candidate epochs, writer containment, partial String/unexpected allocator capacity and actual Watchdog replay remain the next candidate packet; distinct numeric root tests do not claim those behaviors. Physical backing release, zero/short close, all-three metrics receiver refusal/poison and active child preservation also remain required.

## Production Hold

The missing-module/native RED is next after the independent resident missing-API capture. Root implementation remains unmounted until review. Changing new() to empty backing cannot be released alone while old consuming enqueue assumes preallocation: the final production queue/candidate/caller cutover removes that old route coherently. No compatibility wrapper will be introduced to make a partial representation green.
