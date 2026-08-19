# lease-request: Shell/🧊️component.rs (registrar-only)

File: 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs

Reason: DirectoryClient's public API now requires an `&OperationContext` on every
request-issuing call, and NativeDirectoryTransport::new() now requires
runtime/scope/http_pool/package/actor (no longer a zero-arg constructor) — both
breaking, intentional changes per ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME.
Shell is registrar-only; these edits cannot be applied directly.

## 1. New imports (near existing `use semio_framework_os_kernel::os_directory::...`, ~L35)
Add:
```rust
use semio_framework_async::{HostAsyncRuntime, OperationContext, CancelToken, TraceId, ScopeHandle, ScopeOwner, ThreadPlan, ThreadBudget, thread_plan};
use semio_framework_os_services::{TokioHostRuntime, ComputePool};
use semio_framework_actor::{PackageId, ActorId as DirectoryClientActorId}; // renamed to avoid clashing with any existing ActorId alias in this file
```

## 2. One shared runtime/scope/compute pool, minted once (wherever Shell does its other
one-time host-service wiring — `ShellState::new` or `boot()`)
```rust
let directory_thread_plan = thread_plan(std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4));
let directory_thread_budget = ThreadBudget::from_plan(directory_thread_plan);
let directory_runtime: std::sync::Arc<dyn HostAsyncRuntime> =
    std::sync::Arc::new(TokioHostRuntime::new(directory_thread_plan, &directory_thread_budget).expect("directory TokioHostRuntime builds"));
let directory_scope: ScopeHandle = directory_runtime.open_scope(ScopeOwner::Service("directory_client"), None);
let directory_compute = std::sync::Arc::new(ComputePool::new(directory_thread_plan.compute));
```
NOTE: if Shell (or the wider os-host product) already computes a `ThreadPlan`/`ThreadBudget`
ELSEWHERE for its own purposes, reuse THAT one instead of minting a second — the whole point
of `ThreadPlan` (see its own doc in `semio-framework-async`) is "the ONE place any component
reads to learn how many OS threads of each role exist." I could not find one in Shell's own
file within my owned-path search; if one exists in a sibling registrar file, wire to it
instead of the snippet above. This is exactly the kind of judgment call rule 3 asks me to
surface rather than guess at, since I cannot search the whole tree with authority here.

Store `directory_runtime`/`directory_scope`/`directory_compute` as new `ShellState` fields
(`#[cfg(not(target_arch = "wasm32"))]`, alongside `directory_client`/`directory_events_rx`).

## 3. Replace the 3 `NativeDirectoryTransport::new()` call sites
All three become:
```rust
NativeDirectoryTransport::with_new_http_pool(
    self.directory_runtime.clone(),
    self.directory_scope.clone(),
    self.directory_compute.clone(),
    10_000_000,  // bytes_per_minute_cap — tune; not load-bearing today, no byte budget was enforced before this change either
    8,           // outstanding_cap — tune
    PackageId("os.directory-client".to_string()),
    DirectoryClientActorId(0), // or a per-session id if Shell wants separate quota buckets per shell session
)
```
at:
- `bootstrap_identity` (~L3196-3197, inside the `std::thread::spawn` closure)
- `poll_identity_bootstrap` (~L3213)
- `open_directory_stream` (~L3256, inside the closure — see §5 below, this whole closure changes shape)

## 4. `client.command(&command)` → `client.command(&ctx, &command)` (2 call sites, ~L3148, ~L3173)
`client.me()`/`client.mint_session(...)` calls are NOT in Shell directly — they are inside
`mint_or_restore`, whose signature I already updated (see the packet report's
`## OperationContext propagation` — `mint_or_restore(ctx, client, env)`, now takes ctx first).
Shell's `bootstrap_identity` call site (~L3198) becomes:
```rust
let outcome = pollster::block_on(mint_or_restore(&ctx, &client, &env)).map_err(|error| error.to_string());
```
where `ctx` is a fresh `OperationContext` this closure builds (e.g. `CancelToken::root()`,
`deadline_ms: None`) — a real, live context, not a "fake" one; see §6 for a note on whether
Shell wants a longer-lived cancel handle instead (recommended).

For `dispatch_directory_command`/`flush_pending_directory_commands`, build `ctx` the same way
(or thread one down from a Shell-level field — see §6).

## 5. `stream.recv().await` → `stream.recv(&ctx).await` (~L3268)

## 6. `open_directory_stream`'s whole body (~L3240-3275) — retire the private tokio runtime
CURRENT (what to remove):
```rust
std::thread::spawn(move || {
    let runtime = match tokio::runtime::Builder::new_current_thread().enable_all().build() { ... };
    runtime.block_on(async move {
        let client = DirectoryClient::new(NativeDirectoryTransport::new(), base_url);
        ...
        loop { match stream.recv().await { ... tokio::time::sleep(...).await ... } }
    });
});
```
REPLACEMENT — run the loop as a scoped task on the ALREADY-SHARED `directory_runtime`
instead of a private one-off reactor:
```rust
let ctx = OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root(), capability: None };
self.directory_runtime.spawn_scoped(&self.directory_scope, ctx.clone(), Box::pin(async move {
    let client = DirectoryClient::new(
        NativeDirectoryTransport::with_new_http_pool(runtime_for_task, scope_for_task, compute_for_task, 10_000_000, 8, PackageId("os.directory-client".into()), DirectoryClientActorId(0)),
        base_url,
    );
    client.set_token(Some(token));
    let mut stream = client.stream(0);
    loop {
        match stream.recv(&ctx).await {
            Some(DirectoryStreamEvent::Message(message)) => { if tx.send(message).is_err() { break; } }
            Some(DirectoryStreamEvent::Reconnecting { after_ms }) => {
                // 🕐️ tokio::time::sleep still works here because this closure now runs AS A TASK
                // spawned onto directory_runtime's own tokio::runtime::Runtime (via spawn_scoped's
                // internal ScopeTable::spawn_scoped -> record.tasks.spawn_on(wrapped, &self.0.handle)) —
                // it has real ambient tokio context, unlike a bare std::thread + pollster::block_on.
                tokio::time::sleep(std::time::Duration::from_millis(after_ms)).await;
            }
            None => break,
        }
    }
})); // spawn_scoped is fire-and-forget from the caller's perspective — same "detached from boot()" shape the old dedicated thread had
```
This removes BOTH the dedicated OS thread AND the `tokio::runtime::Builder::new_current_thread()`
call — `tokio-tungstenite`'s WS transport (inside `NativeDirectoryTransport::open_ws`) gets a
real reactor because it is now polled as a task on `directory_runtime`'s own multi-thread
runtime, not because it builds its own.

`std::sync::mpsc::channel` (`tx`/`rx`) stays exactly as-is — `pump_directory_events` still
drains it via `try_recv()` every frame; this channel has nothing to do with which executor
produces into it.

## Important technical caveat (verified by reading `semio-framework-os-services`'s
`TokioHostRuntime::sleep_until`): its impl is `Box::pin(async move { tokio::time::sleep_until(target).await })`
— this future must be POLLED from inside `directory_runtime`'s own tokio runtime (its time
driver), or tokio panics ("no reactor running"). That only matters when `ctx.deadline_ms` is
`Some(...)`: `ComputePool::run_blocking`'s `tokio::select!` only calls `runtime.sleep_until(...)`
on the deadline branch. Every `ctx` this packet builds defaults `deadline_ms: None`, so
`bootstrap_identity`'s bare `std::thread::spawn` + `pollster::block_on(mint_or_restore(...))`
is SAFE to leave as a bare OS thread (no deadline ever exercises the tokio-time path there) —
but if Shell ever wants a deadline on identity bootstrap, that call must move onto
`directory_runtime.block_on(...)` (or a `spawn_scoped` task) instead of a bare thread. Flagging
this now so it isn't rediscovered as a mysterious panic later.

## Open question for sol
Should Shell keep ONE long-lived `CancelToken` (e.g. `self.directory_cancel`, cancelled once on
app shutdown) rather than each call site minting `CancelToken::root()` fresh? I recommend yes —
it is the natural hook for "stop every in-flight directory request when the shell tears down" —
but did not add the field myself since `ShellState`'s shutdown path is outside my owned paths
and I do not know its shape.
