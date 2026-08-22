# Renderer Kernel Bridge

## Outcome

The renderer now consumes the canonical semantic UI contract. Its production crate check reaches a clean typed build. A patch is validated and applied to an owned `UiSnapshotState` transaction before any retained state or legacy presentation node is committed. Rejected patches preserve the previous revision and presentation and produce a typed `PatchRejected` event.

## Implemented bridge

- Replaced the removed legacy `PatchOp::Replace` path with `SurfaceId`, `UiRevision`, `UiPatchOp`, `UiSnapshotState`, and `UiDocumentLimits`.
- Retained semantic state and its presented node together per `(instance, surface)`.
- Applied contract validation transactionally and queued rejection detail without mutating the retained revision.
- Added an explicit owned semantic-state presenter. Every contract component is exhaustively converted to the renderer's current `UiNode` presentation vocabulary.
- Decoded all 15 typed scene-surface schemas through the UI scene codec. Malformed or unknown scene payloads remain valid semantic documents and produce a visible diagnostic presentation node.
- Added bridge coverage for transactional rejection and typed Canvas2d scene decoding.
- Repaired the `ShardLoop::register` future's `Send` drift by making its map-only insertion synchronous. `ShardExecutor::register` retains its asynchronous public boundary without carrying the shard mutex guard across a suspension point.
- Repaired the renderer kernel-seam `UiIntent` fixture to supply its typed sequence.

## Verification

- `cargo check -p semio-framework-os-renderer-wgpu --message-format=short`: passed before the subsequent native-task-pool diagnostic probe.
- `cargo test -p semio-framework-os-renderer-wgpu --lib --no-run --message-format=short`: renderer production code compiled; test compilation reported 34 fixture/seam diagnostics. The exact list and ownership grouping are in `📝️renderer-lib-test-diagnostics.md`; the sole renderer seam diagnostic is repaired.
- `cargo test -p semio-framework-plugin-host --lib --no-run --message-format=short`: passed after the final stale test-only `.await` was removed.

## Native task-pool audit

The native `spawn_app_task` path is not a worker-pool executor. It appends a non-`Send` future to a thread-local `TASK_POOL`, while `winit_app::about_to_wait` polls only the winit thread's instance of that pool.

This has two distinct consequences:

1. Tasks spawned by winit callbacks are resumed on the UI thread. The boot continuation, drained-input reduction, and kernel-seam continuation therefore remain UI-thread reachable.
2. `AppRuntime::frame` is now invoked inside `FrameBuildHandle`'s worker-pool closure. Every `spawn_app_task` issued by that method appends to that worker's own thread-local pool. The winit thread polls a different thread-local instance, so those continuations are not guaranteed to be polled at all.

There are 16 static call sites:

- `🦀️kernel_seam.rs:153`: captures `Rc<RefCell<VecDeque<KernelOutcome>>>`, `Rc<RefCell<Option<HostWaker>>>`, and a boxed future without `Send`.
- `🦀️winit_app.rs:144`: carries `MutexGuard<AppRuntime>` across `dispatch_drained_events(...).await`.
- `🦀️winit_app.rs:513`: runtime boot; structurally eligible for worker execution if its complete future proves `Send`.
- `📦️glue.rs:2568`: carries `MutexGuard<AppRuntime>` across `shell.boot().await`.
- `📦️glue.rs:2607`: carries the guard across `pump_sync_events().await`.
- `📦️glue.rs:2646`, `2658`, `2697`, `2741`, `2757`, `2773`: carry the guard across `dispatch_actions(...).await`.
- `📦️glue.rs:2707`: carries the guard across `tutorial_flush_pending_document_ops().await`.
- `📦️glue.rs:2827`, `2838`: carry the guard across `poll_world3d_assets().await`; the latter correctly performs byte fetches before taking the guard, but its final continuation is still non-`Send`.
- `📦️glue.rs:2911`, `2939`: carry the guard across `handle_keyboard_async(...).await`.

Changing native `spawn_app_task` to `KernelPoolFuture::spawn(renderer_worker_pool(), Lane::Interactive, future)` requires `Future + Send`. It cannot be a local signature-only change: 14 AppRuntime continuations retain `std::sync::MutexGuard` across suspension, while the seam uses `Rc` and a non-`Send` boxed exchange future. Blocking a worker on those futures or introducing a dedicated local executor would violate the one-pool/resumable-job requirement.

The clean conversion boundary is an owned mailbox/job protocol: commands enter an AppRuntime owner without holding a mutex guard; awaited kernel/I/O work carries owned `Send` inputs; completions re-enter as bounded messages and short synchronous state transitions. The kernel seam separately needs `Arc<Mutex<...>>`, an `Arc<dyn Fn() + Send + Sync>` host waker, `KernelOutcome.detail: Box<dyn Any + Send>`, and a `Send` exchange future. Until those changes land, Phase 3 cannot be claimed complete and the static deny audit must not exempt this runtime root.

## Native mailbox implementation completed

The conversion boundary described above is now the implementation:

- `KernelPoolFuture` polls native `Send` futures only when woken and submits every turn to the
  process `WorkerPool` interactive lane. The thread-local `TASK_POOL` and winit-side polling hooks
  no longer exist.
- `RuntimeMailbox` owns the AppRuntime slot and a fixed-capacity completion queue. Capacity accounts
  for ready plus in-flight work; one of 128 slots is dedicated to the single interaction-state
  return, while ordinary work is limited to 127.
- `AppInteractionState` is moved, not borrowed, through each awaited Shell/Input mutation and is
  restored by a reserved serial completion. No `MutexGuard` or AppRuntime is held across suspension.
- Keyless command/checkpoint completions are lossless and reject on capacity. Only matching keyed
  replaceable completions may coalesce, and monotonic revisions reject stale keyed results.
- Native frame work uses the same process pool, one in-flight generation, and a non-blocking result
  receiver. Winit callbacks enqueue/invalidate/present only; no callback polls a future.

The kernel seam now uses `Arc<Mutex<OutcomeMailbox>>`, `HostWaker(Arc<dyn Fn + Send + Sync>)`,
`Box<dyn Any + Send>`, and a native `IntentExchange` future with `Send`. Ready plus in-flight outcomes
share a capacity of 64. `submit_intents` returns overflow intents to the caller for retry, preserving
lossless command semantics without eviction or coalescing.

## Final gates

- Native production check: passed.
- Native release check: passed.
- Renderer lib focused mailbox/executor tests: 4 passed.
- Kernel seam continuation/backpressure tests: 3 passed.
- Frame job/generation tests: 6 passed.
- Mounted input callback and stalled-I/O mailbox p99 assertions: both passed below 2 ms.
- Interactivity audit: DENY mode clean.
- Owned source scan: zero native UI executor/polling symbols and zero `Arc<Mutex<AppRuntime>>`.

The browser-Wasm branch retains bounded cooperative `spawn_local` continuation driving and inline
frame-job driving. Platform-specific wake storage is explicit: native uses `Arc + Send + Sync`, while
wasm uses `Rc` for winit-web's non-`Send` event-loop proxy. The prior four upstream missing-await
errors are repaired. The final `wasm32-unknown-unknown` renderer check passed in 21.91 s.

WASI no longer attempts to compile winit. The crate root selects only the same target-neutral
bounded mailbox core for `wasm32-wasip2`, and target-scoped dependencies exclude presentation and
window-system code there. The final Wasip2 check passed in 0.46 s with only three mailbox-core
dead-code warnings. There is no remaining renderer native/browser/WASI compiler-evidence gap.
