# Poll fixture recovery

The first native Poll run after the fixture changes passed 11 of 15 tests. It verified ordinary typed-refusal transfer, the retained failure control owners, and the paid next-round reset. The failure-control and next-round fixtures now settle the real JSON decoder through Source retirement before asserting a terminal Poll phase. Setting the phase alone leaves the decoder open.

The reusable interpreter test fixture now defines every export with the declared Owned Core signature. ChannelVersion uses the i64 packed extent of canonical JSON integer bytes, as the actual startup driver expects. The shared scalar-call integration test now compares the packed i64 with its Wasmtime result. The paid failure-close fixture funds the original Source birth copy quote separately from its business turns, retains its zero-copy refusal case, and has capacity and release stock for diagnostic retirement.

The old Poll Wasmtime output and callback fixtures drive a raw GuestPoll directly. The current Poll freezes each scoped Boot, Input, Poll and CloseOutput response until the issuer records it through GuestIssuedHistoryHome. GuestPollSession performs that authenticated exchange. The old loop ignores Frozen and its synthetic guest has no actor context admission receipt, no bootstrap handback, and one static SRE1 publication serial. Repairing the output and callback scenarios requires a guest and harness that exercise the current escrow protocol.

The second native run passed 12 of 15. The authenticated cancelled-source cleanup now passes, including guest clock reply and fixed close receipt assertions. The close-cause test observed both the unpaid and paid handback states, then exhausted its effective copy allowance before the diagnostic retirement finished. Its final turn copied no items or bytes and left 6,056 copy bytes, 18,296 capacity bytes, 120 items, and 999,997 work units. The fixture now budgets its cleanup turns separately.

A session fixture for the two remaining direct Wasmtime probes must:

- Open an Owned instance through GuestPollSession and the real operation caps.
- Emit a SAB1 response echoing the exact issued 19-field budget. The first response has sequence 1, no prior returned currency, and the fixed two-byte handback amount. Physically return that header through Deallocate.
- Emit an SBT1 transition reimbursing that actual handback and attaching the first SRE1 publication. Return an admitted actor receipt carrying the exact issued input. The static fixture currently traps AdmitActorContext.
- Echo actual input in Input, Poll and CloseOutput responses. Keep the same guest Source identity and emit a contiguous publication serial plus the previous physically returned serial for each SRE1. The static fixture only emits serial 1 with acknowledgment 0.
- Drive frozen receipts and their handbacks through GuestIssuedHistoryHome. Exercise successful capture and retained callback refusal through the actual receiver, and return the final guest context receipt before closing the Source.

## Authenticated session fixture

The two output probes now open a real GuestPollSession through the production origin registry. The guest builder models the first SAB1 header, its matching Deallocate, the SBT1 transition, and five contiguous SRE1 revisions across admission, input, Poll, output close, and context close. Only Deallocate of the exact outstanding range advances the acknowledgment. The callback probe observes the original refusal under its Source, verifies that the captured value remains while retry is denied, retires the failure, and then delivers the same value. Both probes require terminal session and registry closure.

A standalone build with the pinned Rust compiler and Node's WebAssembly runtime verified the packed channel result, exact bootstrap and scoped handbacks, the transition, and the sequence of revisions and acknowledgments. The first host verification attempt failed with ENOSPC while archiving the interpreter runtime, before linking or running the tests.

## Contended native origin retry

The first parallel native run reproduced a production session race. An issued Boot loan was already retained by the driver when the immutable registry temporarily refused the native lookup under contention. The next acknowledgment pass attempted to request a second loan for the unchanged Poll frontier. The driver correctly rejected it because the first loan still owed its exact receipt.

The session now skips fresh subloan requests while it owns an issued guest loan. The original request and budget, including its epoch, remain in place for the next native turn. The Wasmtime result test deterministically takes the real registry lock from another thread at this exact boundary, checks the preserved request and original budget using the neutral JSON fixture, and completes the session after the lock returns. The full fixture runs on a bounded 8 MiB worker stack, after native probes found that the fixture overflowed 2 and 3 MiB stacks and completed on 4 MiB and larger stacks.

The final pinned native Poll test run passed all 15 tests. The linked binary also passed 20 additional parallel repetitions of the output, callback custody, and receiver retirement probes. The separate scalar-call integration test's packed i64 oracle was updated for the same shared guest ABI and was not part of the Poll suite.

## Suspended input cleanup

The real typed Note run exhausted semantic item currency with an Input loan still held. Cleanup attempted to advance the already-started semantic export using retirement authority. The Owned interpreter rejected the attempt before executing another semantic instruction.

Poll now classifies the exact native frontier before checking out cleanup VM fuel. For incomplete ordinary input or a scalar crossing, it enters the paid abort path and returns the input donor through the original instance. A still-held guest loan stays with issuer History until the exact native Core retirement witness permits settlement. If the original VM already returned, retirement funds only the receipt transfer and cleanup exports. The published header and its physical guest return continue through the ordinary frozen journal and deallocation path. This also covers a completed VM whose controller had not yet paid the transition out of the driving phase.

The native session regression drives real Wasm to the original Input call, then closes once with the call suspended and once immediately after the VM returns. The neutral fixture requires the unfinished call to reach terminal custody through full Core retirement, and the returned call to use its authenticated guest context close after receipt handback. Neither scenario delivers Poll output during cleanup.

The final pinned native Poll suite passed all 16 tests. The retirement probe measured positive execution inside the real Input call in both cases using eight-fuel VM slices. The linked binary also passed ten parallel repetitions of the four actual Poll probes. The separate canceled Input continuation after authenticated actor failure remains a distinct Core authorization path.

## Ordinary acknowledgment copy exhaustion

The typed Note diagnostic run stayed at Poll stage 0 with 41,312,713 VM fuel spent across more than one million host steps. The close trace exposed 33,554,490 copy bytes in treasury with two independently held 16,777,216-byte allowances for the guest crossing and cleanup. The ordinary slice could use only the trailing 58 bytes, below the Poll acknowledgment frame's native copy quote.

When the actual Poll callback reaches the ordinary frozen-frontier acknowledgment and cannot pay it while a guest loan is held, the session checks the driver's next physical drive policy after returning the callback receipt. It reports the operation's typed work refusal when that physical copy allowance cannot cover the same `control_bytes()` quote used by the callback. No additional semantic guest budget is issued or taken from cleanup. An unavailable native receiver or temporarily narrow callback remains eligible to retry.

The neutral fixture specifies one byte below the current native quote. The authentic session probe spends retained copy currency through its own Source while an issued Input loan stays in custody. It verifies the typed refusal, the unchanged loan, issuer epoch, request and native owner, and then drives the ordinary Source to terminal cleanup.

A parallel repeat exposed an additional custody edge before native Input had accepted the issued loan. Closing an encoded donor moved the unit into retirement while the outer Poll still treated it as active Input. Poll now selects its persistent retirement phase before closing such an unadmitted donor. The stock regression deliberately holds the actual registry lock at the application boundary so the unadmitted Input case is deterministic.

The final current-source native build passed all 17 Poll tests. The linked binary also passed ten repetitions of the five actual Poll probes in parallel. The suspended-input fixture waits for its native observer when a concurrent registry borrow delays the observation, so it measures the intended Input boundary without advancing past a missing sample.
