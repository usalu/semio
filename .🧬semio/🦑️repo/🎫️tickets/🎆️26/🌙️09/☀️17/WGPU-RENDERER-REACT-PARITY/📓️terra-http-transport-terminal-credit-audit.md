# HTTP Transport Terminal Credit Audit

Read-only source audit on 2026-09-21. No build or test was run. This reviews the current terminal-handle implementation and the four current `AsyncHttpTransport` implementations.

## Contract that must hold

An actor's `outstanding_requests` credit may return only after its physical request work has reached a terminal state. A dropped awaiting future may release its *logical* owner, but it must transfer the exact credit to a physical-work terminal owner. A rejected request that never submitted physical work must release immediately. There must be no cycle in which the terminal owner needs the credit it is retaining in order to terminate.

## Confirmed head-start transfer

`HttpTransportTerminalHandle` owns only a terminal flag plus one release closure; its worker guard owns a cloned handle. `retain_release` either installs the closure before terminal or executes it after terminal, and guard drop takes the closure under the mutex then runs it outside the mutex. The state therefore has no `Handle -> state -> Handle` cycle.

- [services](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1027-L1104) defines the paired handle/guard, exact once-only transfer, and `HttpTransportStart`.
- [services](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1595-L1621) binds one credit and transfers its decrement to the terminal handle.
- [services](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1737-L1792) increments before byte admission, releases an unbound credit on admission refusal, binds before publishing `on_started`, and moves the credit into `HttpPoolBody` only after a returned head.

All current head implementations capture their guard in the physical `run_io` closure:

| Implementation | Source | Head owner result |
| --- | --- | --- |
| Legacy blocking wrapper | [services:1192-1218](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1192-L1218) | Correct. The closure owns the guard until `transport.call` returns or the closure is dropped before running. |
| Socket transport | [services:1323-1349](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1323-L1349) | Correct for connection/head work. |
| Ureq streaming transport | [directory client:1603-1633](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L1603-L1633) | Correct for connection/head work. The explicit pre-cancel branch drops the guard before returning the immediate error. |
| Local socket test transport | [services test:1070-1101](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs#L1070-L1101) | Mirrors the socket route correctly. |

There is no normal synchronous `begin` refusal: the trait returns `HttpTransportStart`, not `Result`. The listed implementations defer physical work to the response future. A budget refusal occurs before `begin`; the credit is still unbound and its `Drop` decrements it immediately. An immediate transport failure also drops the response future's guard. The current head test correctly covers the started-worker case, but it is source present rather than executed evidence in this audit: [services test:902-931](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs#L902-L931).

## Defect: worker-pool shutdown can retain a queued guard forever

This is a source-confirmed terminal-liveness defect.

Native shutdown sets `shutdown` and joins workers without draining lane queues. Once shutdown is observed, `select_and_pop` selects deferred wakes only, never a queued job: [async:1722-1726](../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L1722-L1726), [async:1794-1810](../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L1794-L1810), [async:2080-2095](../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L2080-L2095). Wasm shutdown leaves scheduler queues intact and `pump` immediately returns false when shutdown is set: [async:2395-2412](../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L2395-L2412), [async:2420-2423](../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L2420-L2423).

A queued `ComputePool::run_io` closure retains both its admission permit and its `HttpTransportTerminalGuard`. Since `ComputePool` itself does not register a `WorkerPoolUse`, the pool's retained-use check does not account for that queued closure. The credit's release closure stays installed but cannot run.

There is also a native race: `WorkerPool::submit` checks `is_shutdown` before taking the lane queue lock, then enqueues without a second check ([async:1902-1912](../../../../../../../🧰️framework/🔨️modules/⏳️async/🦀️.rs#L1902-L1912)). A submit that passed the first check can enqueue after shutdown begins; the new job is then neither run nor discarded.

**Required bounded repair:** at generic `WorkerPool` shutdown, fence submissions under the same admission boundary, re-check shutdown after queue acquisition, and take/drop every queued `Job` after in-flight workers have joined. On wasm, take/drop every queue before `Stopped`. Dropping the queued closure drops its terminal guard and admission permit without making an actor credit an execution prerequisite. This makes the terminal guard's documented “discarded before admission” path real on both targets.

**Fail-first law:** occupy the only worker, submit an HTTP head so its `run_io` closure is queued, drop the head future, then shut the pool down. The exact actor count must become zero after the queued closure is discarded; `HttpTransport::call` must not run. A second variant must interleave shutdown after native `submit`'s first check and before its queue push.

## Defect: the head terminal does not cover abandoned in-flight body reads

This is independently source-confirmed for streaming transports.

The start guard completes when the head/connection closure returns. The actor credit then resides in `HttpPoolBody`, whose `finish` only drops the credit; it does not invoke its cancellation handle: [services:1828-1887](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1828-L1887). Socket and Ureq body `next_chunk` methods create independent static `run_io` futures holding an `Arc` to the reader/state, but no terminal guard:

- [Socket body:1250-1269](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️.rs#L1250-L1269)
- [Ureq body:1585-1601](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs#L1585-L1601)
- [Local socket test body:1028-1053](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs#L1028-L1053)

Reachable sequence:

1. A streaming head succeeds, so the head guard completes.
2. `HttpPoolBody::next_chunk` is polled far enough to submit its reader work.
3. The caller drops that pending future, releasing its `&mut HttpPoolBody` borrow, then drops the body.
4. `HttpPoolBody::Drop` releases the credit immediately because the *head* terminal is already complete.
5. The submitted reader closure still owns the `Arc<Mutex<Option<reader>>>` and can remain queued or blocked. No body-drop path invokes the socket interrupt; the existing explicit-cancellation test shows that interrupt is what ends the blocked socket read: [services test:1244-1314](../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🧪️tests/🔬️component-unit/🦀️.rs#L1244-L1314).

That admits a same-actor sibling while the first stream still owns physical I/O. The new head guard correctly protects an abandoned response-head future, but cannot establish the stronger request-wide credit law on its own.

**Required boundary:** do not solve this by calling `cancel_in_flight` then immediately dropping the credit. `ReadDeadline` is not terminal, and a socket close is asynchronous with respect to an already-submitted worker closure. The request needs one terminal group that tracks both the persistent body owner and every submitted body-read lease. `HttpPoolBody` must mark the group closing on EOF/error/drop and request cancellation; the actor decrement runs only after closing and the last head/body worker lease has dropped. The group must be owned by the transport/body contract so Socket and Ureq body-read closures can take an exact lease.

**Fail-first law:** adapt the existing stalled-socket fixture. Poll the first body read until its worker is blocked, drop that pending read future and its `HttpPoolBody`, then attempt a same-actor second fetch with cap one. It must remain `OutstandingCapReached` until the first reader observes cancellation/terminal. After that terminal signal, the second fetch must admit. This law must be separate from the existing explicit-`cancel_in_flight` test.

## Recommendation

Keep the new head terminal transfer. It eliminates the original abandoned-head early release without an ownership cycle. Before declaring per-actor transport credit coherent, repair generic queued-job disposal and extend the terminal group through streaming body work. Neither repair should release an actor slot merely because an awaiting future, response owner, or logical body wrapper has been dropped.
