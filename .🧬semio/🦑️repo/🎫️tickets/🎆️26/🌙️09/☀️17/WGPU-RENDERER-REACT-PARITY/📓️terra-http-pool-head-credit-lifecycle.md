# HTTP Pool Response-Head Credit Lifecycle Audit

Read-only source audit on 2026-09-21. No build or test was run.

## Verdict

The newly added local `HttpOutstandingCredit` in `HttpPool::fetch_started` fixes a leak after an abandoned head future, but it releases the per-actor HTTP credit **too early**. It is not sound to substitute the global `ComputePool` admission permit for the pool's per-actor `outstanding_requests` authority.

The response-head credit must remain owned until the exact start operation is terminal, then either transfer to the returned `HttpPoolBody` or be retired. A caller dropping its future is a cancellation request or loss of interest; it is not proof that queued or in-flight I/O stopped.

## Direct source evidence

- `services/🦀️.rs:1545-1549` defines a **per-actor** `outstanding_requests` cap at the HTTP-pool boundary.
- `services/🦀️.rs:1655-1663` increments that actor's count before outbound-budget and head work.
- Current `fetch_started` creates a local RAII credit at line 1663. Dropping the response-head future drops that local immediately.
- A blocking head uses `ComputePool::run_io` at `1676-1689`.
- `ComputePool::run_in_lane` at `754-784` moves the I/O closure and its semaphore permit into a WorkerPool job, then waits on a oneshot. If the awaiting future is dropped or its deadline wins, the receiver is dropped while the queued/running closure can continue.
- Compute admission is process-wide (`services/🦀️.rs:666-685`), so it cannot limit a single actor's concurrent HTTP work. A pool with compute capacity 2 and actor HTTP cap 1 demonstrates the difference.
- The new test at `services/🧪️tests/🔬️component-unit/🦀️.rs:902-930` waits for its blocked worker's explicit terminal signal before it attempts the sibling. It therefore does not observe the early-release violation.

The current shape can admit a second request for actor 19 after `drop(abandoned)` and before `release_tx.send(())`: the first blocking transport worker is still live, but the local credit has already decremented the actor map.

## Exact invariant

For each admitted actor credit, exactly one of these owners must retain it:

| State | Credit owner | Release condition |
|---|---|---|
| Awaiting compute admission | response-start owner | Future dropped before worker submission, budget rejection, or cancellation before admission |
| WorkerPool queued or in-flight head | exact queued/in-flight start work | Worker closure terminates, including a failed send because its consumer vanished |
| Head completed successfully | `HttpPoolBody` | EOF, body error/budget rejection, or body drop |
| Head completed with error | start result owner | Error is returned after the head work is terminal |
| Deadline/cancel reported while I/O remains live | detached head-retirement owner | Cancellation acknowledgement and worker terminal, not the caller's early error return |

The body half already follows this rule: `HttpPoolBody` owns `Option<HttpOutstandingCredit>` at `1737-1796` and releases once on EOF, read error/budget rejection, or drop. Preserve that behavior.

## Narrow repair shape

For the shipped blocking path, move a clone of the credit into the exact `run_io` closure before it is submitted. Keep the caller's handle until a successful response constructs `HttpPoolBody`.

Conceptually:

```rust
let credit = Arc::new(HttpOutstandingCredit::new(...));
let worker_credit = credit.clone();
let result = compute.run_io(..., move || {
    let _credit_until_terminal = worker_credit;
    transport.call(request)
}).await;
// success moves caller's Arc into HttpPoolBody
```

If the response future disappears after worker submission, the worker's clone remains until the blocking closure returns. A failed oneshot send then drops the result and the final clone. If the future is dropped before the closure is submitted, no I/O owner exists and the caller's credit can retire immediately. This is the smallest first-party proof for `HttpPoolTransport::Blocking`.

Do not implement the same rule for arbitrary `AsyncHttpTransport` by retaining the credit only in `fetch_started`'s local future. `HttpTransportStart` currently contains an opaque `HostFuture` and only a cancellation handle (`1027-1045`); it provides no terminal/close owner. Its contract must be extended so the start operation itself retains or returns the credit through its terminal close path. Immediate `cancel_in_flight()` is insufficient: `ReadDeadline` explicitly reports an outstanding wait (`944-982`), and even an `Interrupted` signal does not establish that the underlying response task has stopped.

## Required test-first cases

Extend the new abandoned-head test using its existing blocked transport:

1. Poll the first `fetch_started` once so its worker starts, then drop it.
2. **Before** `release_tx.send(())`, issue a same-actor request under `outstanding_cap: 1`.
3. Require `Err(HttpPoolError::OutstandingCapReached { actor, limit: 1 })`.
4. Release the first worker and observe `terminal_rx`.
5. Require the sibling to succeed.

Add two adjacent cases:

- **Queued admission:** occupy a compute slot; drop a head that is still awaiting the semaphore; same-actor retry may succeed because no worker start exists.
- **Deadline/cancel after worker submission:** force the head await to return while its blocking closure remains held; same-actor retry remains cap-rejected until the closure signals terminal. Repeat with a published `HttpBodyCancellationHandle::interrupt`; signal delivery alone is not terminal proof.

The existing normal body-drop test at `1155-1158+` remains the takeover oracle: after a body actually exists, early body drop releases the actor slot and disposes the body connection.

## Impact on renderer transport

The renderer calls `fetch_started` at `renderer/🦀️.rs:15591-15597` and records a cancellation owner before awaiting the head. That provides cancellation signalling, but it does not alter the HTTP-pool accounting invariant. The transport's credit must still be retained through terminal I/O after a renderer close or a caller-side deadline.

## Confidence

High. The early-release sequence follows the current local RAII scope and `run_in_lane` ownership directly. The safe blocking repair is a narrow ownership transfer. Cross-transport terminal retention needs an explicit `HttpTransportStart` lifecycle contract; current code does not establish one.

