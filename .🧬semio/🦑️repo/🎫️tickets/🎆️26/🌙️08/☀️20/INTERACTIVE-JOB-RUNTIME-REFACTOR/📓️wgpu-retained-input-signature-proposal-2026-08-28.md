# Retained Input Candidate and Caller Signatures

## Status

Proposed signatures for review, not production APIs. The schema has22 cases, including metrics event-receiver-full and metrics input-generation exhaustion. Separate physical-retirement data requires64 bytes of source capacity for an eight-byte UTF-8 payload, zero/one/seven logical-byte frontiers, and an explicit empty-backing release. Canonical sourceR1 executed22cases/7hostiles/3frontiers and exited0; this remains source-model proof only.

The five unchanged baseline tests actually executed in nativeR3:0passed/5failed/63skipped,.070s. Failures:14336B constructor backing; refusal generation257vs256; event and metrics MAX→0; terminaltrue with14336B queue backing. Exact String pointer/capacity assertions passed before cleanup (logical8/capacity64). This followed R2's compile-only browser test cfg-import failure and seven test-only cfg joins; no queue production change. The zero-short candidate retirement and Watchdog tests still require the reviewed new API and are not counted among these five baselines. Full output: [nativeR3](./📓️ui-host-baseline-five-r3-native-red-2026-08-28.md).

## Unwind Constraint

AbortDrop may record only fixed scalar aborted/verdict state in the still-installed candidate. It must not allocate, retire a whole payload, invoke a callback, release live source backing or publish while unwinding. The narrow borrowed writer and private affine commit-turn design have parent approval; production remains gated on the actual native REDs.

## Canonical Source Owner

The normalized source must live in a persistent host-owned field before any producer allocation or copy, not in the callback's returned `Result<DispatchEvent, DispatchEvent>`. Use one shared `RetainedInputCandidate` representation, owned by the delegate/OsHost and reused by native, CanvasHost and worker callers. It contains:

- A checked candidate key/epoch and original queue/frame generation snapshots.
- One structurally installed `Option<DispatchEvent>`, initially the exact empty variant plus scalar descriptor.
- Building/ready/closing phase, initialized-byte/copy/close cursors, actual owned String capacity and fixed owner metadata accounting.
- The exact existing EventQueue reservation plus optional metrics surface-lane/mailbox receiver reservation identities; these are installed before normalization. These are stable checked slot/epoch keys, never stored MutexGuards.
- The callback-owned fault verdict, if any; optional global trace storage is never authority.

There is no second queue, hidden spill allocation or cloned event payload. Reservation consumes existing receiver capacity. The browser transport event remains in its existing structural pending owner until accepted; simultaneous transport and normalized allocations must both remain in the census.

## Shared Delegate Boundary

Proposed owned-code types, with no platform SDK in the public trait:

```rust
fn reserve_input(
    &mut self,
    descriptor: InputDescriptor,
    grant: InputGrant,
) -> Result<InputAdmission, InputFault>;

fn input_writer(
    &mut self,
    admission: InputAdmission,
    grant: InputGrant,
) -> Result<InputWriter<'_>, InputFault>;

fn drive_input(
    &mut self,
    admission: InputAdmission,
    grant: InputGrant,
    cancel: bool,
) -> InputCallbackResult;
```

`InputAdmission` is an opaque non-forgeable candidate key, not an owner transfer or an increment derived by the caller. All live ownership stays in the delegate's field. `InputCallbackResult` distinguishes accepted, retained/blocked, cancelled and faulted; dropping the result cannot drop a live event. A stale key cannot borrow or commit a replacement candidate.

The producer uses only a narrow borrowed `InputWriter` facade: `reserve_payload`, `write_utf8` and `finish_payload` validate the original admission key and debit the facade's remaining grant. It has no DerefMut, whole-owner reference, raw mutable String/Vec/payload accessor, source extraction or replacement method. The candidate remains private in the delegate; even swapping two public facades cannot swap installed roots. Writing with a stale or other-root key is rejected before changing bytes.

The empty variant is installed first; its actual String backing is reserved in that owner under the physical grant; only the granted UTF-8 prefix is copied. Unexpected allocator over-capacity returns a typed error reporting actual retained bytes and leaves that exact backing mounted. Dropping the writer or unwinding inside a write ends only a borrow; the exact partial candidate remains reachable through its original key. There is no arbitrary producer closure that takes the entire candidate by value.

The consuming `handle_event` and void `handle_metrics` APIs are removed together with all authored native/browser/worker callers in the eventual coherent cutover. No compatibility default, old enqueue wrapper, or ignored overflow result remains.

## Queue Candidate Boundary

Proposed private/in-crate methods on the actual EventQueue:

```rust
fn reserve_input_into(
    &mut self,
    source: &mut RetainedInputCandidate,
    descriptor: InputDescriptor,
    grant: InputGrant,
) -> Result<InputAdmission, InputFault>;

fn prepare_input_into(
    &mut self,
    source: &mut RetainedInputCandidate,
    grant: InputGrant,
) -> Result<InputPreparation, InputFault>;

impl InputCommitTurn<'_> {
    fn finish(self) -> InputCallbackResult;
}
```

These names represent a single ownership seam, not additional public fallback routes. The concrete implementation may combine borrow guards, but it may not detach the source into a return value. The prepared token captures actual receiver identity and checked generations; it cannot be reconstructed from equal numeric counters.

`InputCommitTurn` is private and affine. It is constructed only inside the mounted `drive_input` turn, owns that turn's actual Watchdog, and borrows the exact private candidate and freshly reacquired receivers. `finish(self)` calls that owned guard's `finish()` itself, then gates its own prepared receivers. There is no API accepting a caller-supplied CallbackVerdict or successful prior callback receipt. The original proposal's verdict-accepting commit signature is rejected: current CallbackVerdict is Clone+Copy and binds only operation/generation/site/elapsed, not a unique callback instance.

This structural boundary avoids adding a second watchdog or an unrelated global nonce counter. A copied old verdict remains diagnostic data only. It cannot construct an InputCommitTurn, replace its guard, change its captured candidate key, or authorize another receiver. The private turn cannot survive a yield; on abort/unwind it retains the candidate, records the exact final guard result plus aborted status, and publishes nothing. Tests must retain a previous accepted result while a new same-operation/generation callback reaches 8000us and prove the latter still refuses; cross-candidate and duplicate-finish attempts must likewise have no ownership transition.

Zero physical/work grants refuse before allocation, copy or source detachment. Full queue/source-reservation saturation and stale/occupied targets preserve the source pointer, capacity, initialized prefix and exact reservations. At commit, the source-to-target move is a pre-admitted fixed descriptor transfer; no String clone, reserve, whole generic owner drop, waker, user callback or unchecked increment is allowed.

## Metrics Three-Receiver Boundary

The mounted metrics helper obtains the EventQueue candidate, existing surface token and existing RuntimeCompletionQueue guard/reservation before preparing any mutation. Its signature should consume only borrows of those installed owners, and return a typed `InputCallbackResult`; `handle_metrics` must not enqueue anything afterward.

Mailbox preflight uses real try_lock, preserving busy versus poison. The ready queue's capacity and its existing interaction reserve remain unchanged. It reserves an exact slot/epoch identity and then releases the guard. No MutexGuard is stored inside a candidate, borrowed self-referentially, or held across resumed source-copy turns. The prepared-commit turn reacquires every actual guard nonblockingly, revalidates all reservation keys and original generations, and only then stages the checked revision. Busy/poison/stale refusal preserves the persistent reservations and all source owners. Event-receiver-full means the actual input admission receiver is unavailable; it must not be modeled by an unrelated boolean. It does not authorize arbitrarily changing the semantics of the fixed coalesced metrics slot.

A surface request retains the original lane token and previous child. Preparation never calls `session.begin_close()`. Only accepted scalar publication marks later bounded driver work; refusal preserves active child identity and cancellation state. The metrics candidate must refuse before any sink changes when any one of the three receivers is unavailable.

A final accepted verdict authorizes only the already-prepared fixed receiver writes while the guards acquired in this same commit turn remain held. Those guards are released before the turn returns; only checked reservation identities persist. All allocation, copying, lock acquisition, coalescing inspection and prepared-owner work stays within the measured body. No runtime callback/waker runs after finish. Waking must be represented by already-owned bounded notification state consumed by the normal driver, not moved outside the measured callback to hide its cost.

## Close and Physical Accounting

The close API must distinguish logical work from physical release, following the existing retained-wire/typed-list separation:

```rust
fn close_step(
    &mut self,
    maximum_items: usize,
    maximum_bytes: usize,
) -> Result<InputCloseProgress, InputFault>;

fn allocated_bytes(&self) -> usize;
fn terminal_is_empty(&self) -> bool;
```

The byte grant bounds actual initialized-payload inspection/retirement work. A zero-byte grant cannot advance live text. A short grant may inspect a prefix without violating String UTF-8 validity; the root remains unreadable once closing starts. The String capacity and resident credit remain unchanged until the entire initialized payload is retired and its separate empty-allocation release occurs. Do not count prefix inspection as physical memory release. Report the actual released allocation delta separately from logical bytes; empty queue backing likewise requires its own bounded item step. Terminal requires no source, reservations, pending fault-owned payload, or retained backing.

Native tests must inspect actual allocator capacity and exact pointers, not assume `try_reserve_exact(64)` yields exactly 64. The independent Buffer oracle models a retained backing/view; it is not proof of Rust allocator behavior. New tests will cover zero/one/seven close, allocator over-capacity retention, every interrupted producer/close frontier and full exact credit restoration.

## Platform Caller Behavior

CanvasHost keeps its original transport `pending_dispatch/latest_pointer/latest_metrics` owner until the delegate reports accepted. It does not ACK on blocked/faulted. Clipboard text is built in the installed candidate, not cloned into a temporary event.

NativeHost and WinitApp preflight the candidate before normalization. A previously generated refused event remains in the candidate. A continuously full native external event stream cannot be buffered losslessly by a bounded system without upstream flow control; the bridge must expose a terminal admission/backpressure fault and stop producing normalized payloads, never silently log Overflow and continue as accepted or allocate an unbounded spill queue. Actual fault notification/closure must preserve the already-owned candidate.

Browser worker messages require the same retained source/reply behavior. No browser completion may claim accepted merely because a void delegate returned. All caller migration tests must retain original transport and payload allocation identity across failure/unwind.

## Remaining Gate Order

1. Canonical UI-host source/native target and launch registration; run the actual 22-case/separate-backing source oracle.
2. Execute five mounted real-source baseline REDs in the existing sole target after compiler coordination.
3. Review these concrete signatures and native REDs before production mounting.
4. Add missing-API candidate/close/Watchdog/mutex/three-sink tests; preserve their actual RED.
5. One coherent owned API/caller cutover, then focused/full UI-host and mounted WGPU tests. GPU/redraw publication remains separate.
