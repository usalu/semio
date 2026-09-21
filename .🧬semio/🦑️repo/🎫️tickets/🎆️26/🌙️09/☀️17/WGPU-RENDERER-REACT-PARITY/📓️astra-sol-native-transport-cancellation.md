# Native Transport Cancellation

## Fail-first boundary

`a_stalled_native_component_body_is_aborted_before_its_sibling_enters_the_single_fetch_lane` drives the real renderer `RuntimeMailbox`, actual component-owned World A and World B requests, the native HTTP pool, and a local TCP server. The server publishes World A's response head and retains its body. The test repeatedly advances component close and the native asset lane while that body remains withheld.

The required observation is that exact World A cancellation interrupts the blocked body and allows World B to enter the pool's single outstanding fetch slot before the server releases A. The fixture releases A after the observation window and continues pumping every production owner before asserting, so the current expected failure cannot strand a socket, worker, fetch response, or server thread.

After cleanup the test also requires World A component close to be terminal, World B to have started, no frame fault, and no retained fetch/handoff owner. Once production introduces the exact `NativeAssetTransportLease`, this terminal census must include that slot directly.

Focused filter:

```text
test(a_stalled_native_component_body_is_aborted_before_its_sibling_enters_the_single_fetch_lane)
```

## Neutral contract

`native-asset-response` schema v3 adds `transportCancellation`. It fixes distinct A/B host identities, one withheld response byte, a two-second observation boundary, a five-second cleanup boundary, zero A publications, zero terminal transport owners, and zero frame faults. The fixture and schema are shared by the native law and the existing browser-worker third-party oracle.

The focused Ajv oracle passed through the canonical Nx browser-worker target: one selected test passed and 152 were filtered. Receipt: `🗑️generated/astra-runtime/native-transport-oracle/run.log`.

The complete worker-step-budget file was also attempted during concurrent browser-worker source changes. Its new schema/oracle assertion passed, while a separately owned source-shape assertion at line 352 rejected a concurrent `next_deadline` change. That run does not provide a complete-suite green claim.

## Production seam after RED

The renderer needs an exact per-request transport lease keyed by both `AdmittedSurfaceToken` and `WorldAssetRequestToken`. It must retain the child cancellation token and an object-safe body abort handle while a native response body is checked out. Component close marks the exact request cancelled, signals the child, aborts the matching body, and remains pending until the existing fetch handoff returns the response owner. It must never clear `native_asset_fetching`, take the fetch response, or affect a same-host successor or World B.

The services-side socket implementation should clone the connected `TcpStream` for abort control and call `shutdown(Shutdown::Both)` without acquiring the reader-state mutex. HTTPS requires a separately proven terminal policy; a no-op abort cannot satisfy this contract.

The body API should expose a cloneable, object-safe cancellation handle before the first `next_chunk` future is created. A closed result distinguishes `Interrupted`, `AwaitingReadDeadline { maximum_ms }`, and `Idle`; it must never label a no-op as an interrupt. `SocketHttpBody` returns a handle around a cloned `TcpStream`, while a buffered body reports `Idle` because it has no platform read in flight.

`UreqStreamingHttpBody` cannot recover the TLS socket from its opaque `Box<dyn Read>`. Its supported policy must therefore configure a real `ureq::AgentBuilder::timeout_read` bound and return `AwaitingReadDeadline` with that same bound after the exact request child token is cancelled. The renderer retains its transport lease, response owner, and single-fetch credit until the read returns. An I/O error after exact cancellation maps to cancellation; an error for a live request remains a transport fault. This is a bounded wait with typed evidence, not a claimed abort.

The renderer lease belongs in one fixed mailbox slot because the native lane admits one fetch. It contains `{ surface_token, request_token, child_cancel, body_cancellation }` and no response/body/payload owner. A small guard installs it after the response head is available and removes only the same token pair when the stream future terminates. Component close may signal only a matching admitted surface and request. It remains pending until the guard clears and the existing handoff returns the fetch owner; it cannot clear `native_asset_fetching` itself.

Native 125 recorded the intended renderer failure in 2.07 seconds: World B did not enter the one-fetch lane while World A's response body remained withheld. The fixture then released A and completed its cleanup, so the RED left no blocked server or worker. Receipt: `🗑️generated/astra-runtime/renderer-native125-retained-owner-red/run.log`.

The service API and renderer lease are implemented. `HttpBodyCancellationHandle` has exact `Idle`, interrupt, and read-deadline policies; invoking it returns `HttpBodyCancellationStep::{Idle, Interrupted, AwaitingReadDeadline, Fault}`. Socket bodies own a cloned `TcpStream` shutdown handle. Ureq bodies use a real 15-second `timeout_read` and report the same bound. `HttpPoolBody` exposes the cloneable handle without sharing or moving its body owner.

`RuntimeMailbox` now owns one `NativeAssetTransportLease` with the admitted surface token, request token, child cancel token, body handle, and first cancellation result. `NativeAssetTransportGuard` removes only the same token pair on terminal future handback and wakes the runtime. `cancel_native_asset_transport_step` supports exact component and whole-runtime scopes and reports `Busy`, typed `Pending`, or `Terminal`. The stream/handoff owner will bind and advance these APIs in its integration slice.

## Integrated body review and exact outcomes

The native HTTP stream now creates one request child token before constructing its `OperationContext`; that same child is retained in the transport lease after the response head. The guard spans status validation and every body read. The stream checks both the child token and exact World request before each `next_chunk().await` and again immediately after it, before interpreting EOF, an I/O error, or response bytes. Consequently a socket shutdown error after exact close is classified as cancellation, while the same error for a live request remains a transport fault. Component close scopes cancellation by `AdmittedSurfaceToken`; whole-runtime close signals the one admitted lease. Neither path clears the fetch boolean or handoff owner.

All four `HttpBody` implementations now expose an explicit policy: buffered bodies return `Idle`, production socket bodies return an interrupt handle around a cloned `TcpStream`, the services test socket uses the same real shutdown behavior, and Ureq returns the exact configured 15-second read deadline. The services unit laws cover every closed outcome (`Idle`, `Interrupted`, `AwaitingReadDeadline`, `Fault`) and use the production socket transport to require a checked-out body read to finish before server cleanup releases it, after which the same actor's single pool slot admits its sibling request.

The neutral fixture is schema v4. Its `serviceOutcomes` row pins the four exact outcomes and the 15,000 ms HTTPS read bound. The Ajv/TypeScript oracle passed through the canonical browser-worker Nx target: one selected test passed and 153 were filtered, in 9.2 seconds on the recorded run. Receipt: `🗑️generated/astra-runtime/native-transport-oracle-v4/run.log`. Both changed Rust test files parse and match `rustfmt --edition 2021 --check`; their native execution remains root-owned.

## Withheld response head remains fail-first

`a_withheld_native_component_head_is_cancelled_before_its_sibling_enters_the_single_fetch_lane` uses the same real local TCP and actual World A/B runtime as the body law, but signals request receipt and withholds the entire response head. It repeatedly closes exact World A and pumps the one-fetch lane, requiring World B to enter before cleanup releases A's head. The server release, native handoff drain, transport census, World retirement, and thread join all happen before assertions, so the expected RED cannot retain a socket or worker.

Focused filter:

```text
test(a_withheld_native_component_head_is_cancelled_before_its_sibling_enters_the_single_fetch_lane)
```

Production remains held for this law. The current lease cannot exist before `HttpPool::fetch` returns the head. Ureq's 15-second value is a per-read timeout and does not bound a slow-drip head's total duration; the `OperationContext` deadline currently does not interrupt a running blocking I/O closure. A separate exact pre-head transport owner is therefore required before claiming bounded head cancellation.

Local native-file `ReadPage` is also outside this acceptance boundary. `stream_native_renderer_asset` checks exact World cancellation before `run_renderer_io(ReadPage).await`, then pushes the returned retained payload without a second cancellation check. Dropping the awaiting future only drops `RendererIoHandle`, whose `Drop` sets `cancel_requested`; it cannot interrupt the `NativeIoJob::step` blocking `file.read`. If cancellation arrives during that await and the read returns a retained page, the page needs a bounded handback/close owner before the stream may classify the request as cancelled. The HTTP body result does not prove this path.

Native 126 executed the real stalled-body law against the integrated stream/lease implementation. It passed: exact World A cancellation woke its body read, World B entered the single-fetch lane before the server released A, and cleanup reached zero transport owners without a frame fault. The phase-one reference worker law also passed. The selected run recorded 67 passes and one independently owned Map cleanup failure. Receipt: `🗑️generated/astra-runtime/renderer-native126-integrated-owner-green/run.log`.

## Local retained-page fail-first boundary

Schema v5 adds `localPageCancellation`: one 16 KiB page, zero bytes appended to the cancelled response, the full 16 KiB retained in handoff before close, one page-retirement turn, zero terminal native owners, and zero frame faults. The independent Ajv/TypeScript derivation passed through the canonical browser-worker Nx target with one selected pass and 153 filtered tests. Receipt: `🗑️generated/astra-runtime/native-local-page-oracle/run.log`.

`a_cancelled_local_component_page_is_retained_for_bounded_handback_without_response_publication` writes one exact local page and drives the real RuntimeMailbox, RendererIo mounted session, World A asset owner, and component close. A test-only exact-surface barrier pauses after `ReadPage.await` returns its retained payload and before the stream interprets it. The law begins World A close, releases the returned page, and requires the stream to put that page into the existing `NativeAssetHandoff.payload` with `seal = false`, leaving `received_bytes = 0`. Cleanup advances that payload through one existing 16 KiB handoff grant, returns the request, completes World close, drains the mounted I/O session, retires both fixture Worlds, and deletes the local file before asserting.

Focused filter:

```text
test(a_cancelled_local_component_page_is_retained_for_bounded_handback_without_response_publication)
```

Production remains unchanged pending the native RED. The bounded repair should add a cancellation variant that owns the returned `RetainedJobPayload`, check exact cancellation immediately after the await, and pass that payload to the existing handoff without recording a frame fault. It must not push the page, seal the response, drop the payload locally, or synchronously loop its retirement.

## Pre-head owner API design

The start owner must exist before its I/O future is first polled. A closed services API can provide this without exposing renderer types:

```rust
pub struct HttpTransportStart {
    cancellation: HttpBodyCancellationHandle,
    response: HostFuture<StartedTransport>,
}

pub trait AsyncHttpTransport: Send + Sync {
    fn begin(&self, ctx: &OperationContext, request: HttpRequest) -> HttpTransportStart;
}
```

`HttpPool::begin_fetch` performs outstanding-slot and byte-budget admission synchronously, constructs that transport start, and returns an `HttpPoolFetch` future plus the cloneable cancellation handle. `HttpPoolFetch::Drop` releases the exact actor slot if the start is abandoned; successful polling transfers that credit into `HttpPoolBody`. Existing `fetch` can remain the convenience operation implemented by `begin_fetch(...).await`, rather than retaining a second transport path.

The renderer installs `NativeAssetTransportLease` from `HttpPoolFetch::cancellation_handle()` before awaiting the response head. Thus component close can cancel the exact surface/request while the start future is checked out, and still remains pending until that future returns, the guard clears, and the fetch owner enters normal handoff. Cancellation never clears `native_asset_fetching` and never returns a World request while its I/O owner is live.

Socket start and body should share one immutable cancellation authority. It contains an atomic requested bit and a `OnceLock<TcpStream>` clone. Cancellation first sets the bit, then shuts down the clone if connected. Connect binds the clone and immediately shuts it down if the bit was already set. No callback acquires the reader-state mutex or runs while the renderer lease mutex is held. The body receives a clone of the same handle, so the head-to-body transition cannot lose a cancellation admitted just before head publication.

The current socket response-line reader already reads one byte per call. Its pre-head variant can carry an absolute `Instant` deadline, set the socket read timeout to the remaining duration before each byte, and check the cancellation authority between reads. That bounds a slow-drip status/header sequence rather than resetting one full timeout for every byte. Connect uses a bounded connect operation for resolved addresses. System DNS remains an honest platform constraint unless moved behind a separately cancellable resolver owner.

Ureq cannot expose its TLS stream. Its start owner therefore carries the request child token and typed deadline evidence; configure Ureq's overall request timeout as well as the body read timeout, keep component close pending, and classify the eventual return as cancellation when the exact child is set. DNS may exceed Ureq's documented overall timeout because the system resolver cannot be interrupted, so the HTTPS outcome must remain `AwaitingReadDeadline`/pending until the actual closure returns. No HTTPS test may claim immediate abort from the HTTP socket law.

The renderer's cancellation method must copy the chosen handle out of its lease, mark the exact child cancelled, release the lease mutex, and only then invoke the platform callback. It may reacquire the slot afterward to store typed evidence if the same surface/request pair remains. This prevents a platform shutdown callback from blocking component-close inspection or guard terminal handback.

## Native 128 RED and coherent repairs

Native 128 executed 70 laws: 67 passed and three failed. The local-page law reached its intended assertion with 16,384 bytes published for cancelled World A instead of zero. The withheld-head law also reached its intended assertion after about 2.08 seconds: World B had not entered the one-fetch lane before the server released A. The third failure was the independently owned Map retirement law. Receipt: `🗑️generated/astra-runtime/renderer-native128-map-head-page-red/run.log`.

The local-file stream now rechecks exact World cancellation after `ReadPage.await` and before interpreting or publishing the returned payload. `NativeAssetStreamFailure::CancelledPage` carries that `RetainedJobPayload` into the existing native handoff with `seal = false`; the handoff retires one page per established grant and records no frame fault.

The pre-head transport API is now coherent across the services trait, blocking transport, production socket transport, services local-socket test transport, directory Ureq transport, pool, and renderer. `AsyncHttpTransport::begin` returns `HttpTransportStart`, whose cloneable cancellation owner exists alongside an unpolled response future. `HttpPool::fetch_started` performs the same admission/accounting as `fetch`, invokes its one-shot owner callback before polling the start future, and transfers successful request credit into the ordinary `HttpPoolBody`. `fetch` delegates to this implementation with a no-op callback, so there remains one admission and response path.

The renderer callback installs `NativeAssetTransportLease` before awaiting the response head. Socket head and body share one `SocketHttpCancellation`: cancellation sets an atomic requested bit and shuts down a `OnceLock<TcpStream>` clone if connected; binding after prior cancellation immediately shuts down the new connection. Ureq publishes its typed 15-second deadline before work starts and configures both overall and read timeouts. The renderer invokes the platform cancellation callback outside its lease mutex, then stores typed evidence only if the exact token pair remains.

This repair does not claim interruptible system DNS or immediate opaque TLS abort. Those paths keep typed deadline evidence and exact ownership until their actual worker returns. All `AsyncHttpTransport` implementations and call sites were enumerated with `rg`; the changed services, services tests, and directory files parse and pass `rustfmt --edition 2021 --check`. Native and services execution is root-owned and remains pending Native 129.

## Services gate and abandoned-start fail-first law

The full services native gate passed 47/47 laws in 0.268 seconds, with no skipped laws; Nx completed in 21.4 seconds. This includes the closed cancellation-outcome law and the real socket blocked-read/pool-slot law. Receipt: `🗑️generated/astra-runtime/services-native-request-cancellation/run.log`.

The new pre-head callback makes a previously existing pool-accounting risk reachable by an exact cancellation consumer: `HttpPool::fetch_started` increments the per-actor outstanding credit before awaiting the head, but dropping that future bypasses both the explicit error release and `HttpPoolBody::drop`. This does not invalidate the green body test, whose successful head transfers credit to a body owner. It affects only abandonment before that transfer.

`http_pool_abandoned_response_head_retires_its_exact_outstanding_credit_after_worker_terminal` is the fail-first law. It polls a real blocking-transport/ComputePool fetch to pending, observes the exact cancellation owner before the wait, drops the response-head future, then explicitly releases and observes the underlying worker terminal. Only after worker terminal does it ask the same actor to use the one-credit pool. The current expected failure is `OutstandingCapReached`; the law does not cancel, clear, or fabricate completion for the compute owner.

Schema v6 adds `transportCancellation.abandonedStart` with one outstanding credit, required worker terminal before sibling admission, and required sibling admission. Its Ajv derivation passed in the canonical browser-worker full run: 11 files and 155 tests passed, tests took 3.10 seconds and Nx took 9.2 seconds. Command: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker -- '🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts'`.

Focused native filter:

```text
test(http_pool_abandoned_response_head_retires_its_exact_outstanding_credit_after_worker_terminal)
```

The production repair remains held pending this native RED. The intended repair is a fixed RAII outstanding-credit owner moved into `HttpPoolBody` only after successful head publication. Dropping the pending fetch or returning an error retires the same credit exactly once; it does not claim or destroy the independently bounded ComputePool job.

The services fail-first gate executed all 48 laws: 47 passed and only `http_pool_abandoned_response_head_retires_its_exact_outstanding_credit_after_worker_terminal` failed at its sibling-admission assertion, after the fixture observed the first worker terminal. Tests took 0.257 seconds and Nx took 14.6 seconds. Receipt: `🗑️generated/astra-runtime/services-native-abandoned-head-red/run.log`.

Production now represents the admitted actor slot as `HttpOutstandingCredit`. `fetch_started` creates it immediately after incrementing the admitted count and retains it across byte-budget and response-head waits. Successful head publication moves the same owner into `HttpPoolBody`; body EOF, body error, explicit finish, and body drop all retire it by taking that one option. Budget refusal, transport error, or abandonment before head publication drops the still-local owner. No path performs a second decrement, and this owner does not cancel, clear, or take the separate ComputePool job. Services GREEN is pending the root-owned rerun.

## Physical head-work terminal credit

The strengthened services law added the missing pre-terminal observation. After dropping the response-head future while the first blocking worker remains held, a same-actor sibling must receive `OutstandingCapReached`; after the fixture releases and observes the first transport terminal, the sibling must be admitted. The focused run reached the intended RED at the pre-terminal assertion: one selected law failed, 47 were filtered, the test took 0.012 seconds, and Nx took 2 minutes 15 seconds. Receipt: `🗑️generated/astra-runtime/services-native-abandoned-head-terminal-red/run.log`.

The simple local RAII owner was insufficient because `ComputePool` is a global execution bound, while the HTTP outstanding count is an independent per-actor transport bound. Production now pairs each response-head operation with `HttpTransportTerminalHandle` and a worker-owned `HttpTransportTerminalGuard`. Every implementation places that guard in the real physical closure: direct blocking pool work, the Blocking wrapper, production Socket, the services LocalSocket transport, and directory Ureq. Dropping a work future before admission drops the guarded closure and completes the same witness; dropping it after submission leaves the guard with the worker until the closure returns.

`HttpOutstandingCredit` binds that terminal. If the response-head future is abandoned or faults while physical work remains live, its Drop parks the exact release operation on the terminal witness. The guard invokes it after returning. A successful head arrives only after the guard completes and transfers the same credit into `HttpPoolBody`; body terminal/drop then releases immediately against the completed witness. Terminal callbacks run outside the witness mutex. The exact cancellation handle remains a separate owner and is not treated as terminal evidence. Services GREEN is pending the root-owned rerun.

The v6 Ajv/TypeScript contract now includes `siblingRefusedBeforeWorkerTerminal: true`. The canonical browser-worker full run passed 155/155 laws across 11 files in 4.33 seconds; Nx took 11.8 seconds.

The strengthened service contract is now GREEN in the complete service census: 48/48 laws passed in 0.240 seconds and Nx completed in 14.6 seconds. This verifies that the first actor credit remains unavailable while the abandoned physical response-head worker is live, returns only after that worker's terminal witness, and then admits the sibling. Receipt: `🗑️generated/astra-runtime/services-native-terminal-credit-green/run.log`.

## Native 130/131 local-page cleanup evidence

Native 130 ran 1,359 renderer laws: 1,306 passed and 53 failed. The local cancelled-page law no longer failed its zero-publication assertion; its cleanup instead exposed a `WorldAssetFetchOwner` reaching Drop before terminal handback, followed by nonterminal whole-World retirement. Receipt: `🗑️generated/astra-runtime/renderer-native130-full/run.log`.

The fixture now drives the real component Asset-before-World close bridge for each exact A/B host before taking either World into dynamic retirement, with a strict per-host asset-terminal assertion. Native 131 selected 83 laws: 82 passed, while the local law still failed under the full selected runner after about 11.3 seconds because component-world-a had not reached asset terminal and the worker later hit the same Drop guard. Receipt: `🗑️generated/astra-runtime/renderer-native131-owner-integration/run.log`.

Replaying the exact Native 131 binary for the local law with backtraces passed 1/1 in 0.07 seconds and produced no panic. Receipt: `🗑️generated/astra-runtime/native131-local-page-backtrace/run.log`. This is evidence of a scheduling/terminal race or runner-load dependence, not a full GREEN claim. The original failure remains authoritative until the same official nextest environment is replayed and the checked-out owner path is accounted for.

Native 132 reproduced the local failure in the full 1,360-law runner and supplied its owner trace. The `WorldAssetFetchOwner` was dropped from the pending `RuntimeMailbox::pump_native_asset` future when `KernelPoolFuture::run_turn` released its final `Arc`; the component cleanup assertion was secondary. Receipt: `🗑️generated/astra-runtime/renderer-native132-full/run.log`.

The exact lost-wake seam was `RendererIoHandle::poll`. When `pump_renderer_io_sessions` transiently owned the mounted slot as `RENDERER_IO_CHECKED_OUT`, both attempts to register the current task waker could fail while `renderer_io_generation_live` still correctly reported the generation live. The method then returned `Pending` without any retained or newly notified waker. Because `spawn_app_task` intentionally discards its initial `KernelPoolFuture` handle, the active pool turn could release the last strong owner and drop the still-live asset future.

The repair keeps the existing exact-generation check and wakes the current task only in that live-but-contended branch. A deterministic retained-I/O law checks out the precise slot before the handle's first poll and requires one self-wake; it then restores the slot and drains normal mounted-session cancellation. No timeout or retirement grant changed. Neutral schema v7 records the checked-out registration, live exact generation, pending result, one wake, and zero terminal owners. Its Ajv/Vitest oracle drives a real `MessageChannel` turn and passed in the canonical browser-worker census: 11 files and 156 laws passed, tests took 3.16 seconds and Nx took 10.9 seconds. Receipt: `🗑️generated/astra-runtime/native-io-contention-oracle/run.log`. Both edited Rust files pass `rustfmt --edition 2021 --check` (the renderer root used `skip_children=true` because unrelated included modules currently have independent formatting diffs). Native execution is root-owned and pending Native 133.

## Pending body-read actor-credit RED

Schema v8 adds `transportCancellation.abandonedBodyRead`: a one-credit actor, an observed pending physical reader before body drop, sibling refusal until reader terminal, reader terminal before later admission, and successful post-terminal sibling admission. The Ajv/Vitest derivation passed in the canonical browser-worker target: 11 files and 156 laws passed, tests took 2.85 seconds and Nx took 7.8 seconds. Receipt: `🗑️generated/astra-runtime/http-body-credit-oracle/run.log`.

`http_pool_abandoned_pending_body_read_retires_its_actor_credit_after_reader_terminal` uses a real local TCP stream and the actual `HttpPool`, `ComputePool`, `HttpPoolBody`, and asynchronous transport boundary. A probe inside the physical read closure establishes that the body read has started, then holds the closure after its socket read returns but before its terminal handback. The law drops the pending read future and its body, releases the socket only for deterministic cleanup, and attempts a same-actor sibling while the reader closure is still held. It records the result, releases and observes the reader terminal, drains any incorrectly admitted head owner, and verifies the subsequent sibling can read. All cleanup completes before the four contract assertions, so the intended RED cannot leak a socket or worker.

Focused filter:

```text
test(http_pool_abandoned_pending_body_read_retires_its_actor_credit_after_reader_terminal)
```

Production remains held pending the root-owned RED. The required repair is request-wide terminal ownership across the persistent body and every submitted body-read worker lease. Invoking cancellation on body drop does not itself return the actor credit; a socket interrupt or typed read deadline must still reach physical worker terminal.

The full 49-law services gate measured the intended failure: 48 laws passed and the pending-body law failed at its pre-terminal sibling-refusal assertion (`false` observed versus `true` required). Tests took 0.266 seconds. Receipt: `🗑️generated/astra-runtime/services-native-pending-body-red2/run.log`.

Production now makes `HttpTransportTerminalHandle` a counted terminal group. Its response-head guard is the initial owner; each `HttpPoolBody::next_chunk` takes another exact reader guard before calling the transport body. `BufferedHttpBody` keeps that guard in its returned future, while Socket, the real LocalSocket test transport, and directory Ureq move it into their physical `ComputePool::run_io` closures. If the awaiting future is abandoned, the physical closure continues to own the lease. Dropping or aborting `HttpPoolBody` invokes its exact cancellation owner and parks `HttpOutstandingCredit` on the terminal group. The actor slot returns only after every live reader guard reaches terminal; cancellation is wake evidence, not terminal evidence.

Successful response-head publication transfers the same terminal handle into `HttpPoolBody`; there is no second admission path or separate numeric cap. EOF releases without an artificial interrupt. Byte-budget refusal, transport error, and body drop interrupt the body and retain the credit behind any physical reader. All four `HttpBody` implementations and the pool construction path were updated together. The services, services test, and directory source files pass `rustfmt --edition 2021 --check`. Native GREEN execution remains root-owned.

The complete services gate is GREEN: 49/49 laws passed in 0.242 seconds and Nx completed in 16.6 seconds. This includes the real pending body-read law: the same actor remains refused while the abandoned physical reader is held, then regains its one credit only after that reader's terminal handback. Receipt: `🗑️generated/astra-runtime/services-native-pending-body-green/run.log`.
