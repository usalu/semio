# terra-async-imports — host-async import layer

## delivered

- **NEW** `🖥️host/⏳️imports.rs` (807 lines): `bindgen!` for `world actor-async` (`mod
  host_async_bindings`), `AsyncActorHostState`, the `pure::Host` impl, the `host_async::Host` impl
  (`emit`/`emit-patch`), and the `host_async::HostWithStore<AsyncActorHostState>` impl covering all
  24 `async func` imports.
- `🖥️host/🦀️component.rs`: 6-line surgical addition mounting the new module (see `## line ranges`).
- Verified (see `## commands + exit codes`) that `wasmtime = "47.0.3"`'s `component-model-async`
  Cargo feature is already pulled in via `default-features` — **no Cargo.toml change needed**, so
  there is no lease-request for that (I initially expected one; wasmtime's own `Cargo.toml` lists
  `component-model-async` inside its `default = [...]` set, and this crate's `wasmtime` dependency
  never sets `default-features = false`).

## the 24 imports

| WIT func | service call | cancellation behaviour |
|---|---|---|
| `storage-read` | `StorageScheduler::submit` + `StorageBackend::read` | checked before submit; `CancelOnDrop` on guest-drop |
| `storage-write` | `StorageScheduler::submit` + `StorageBackend::write` | same |
| `storage-delete` | `StorageScheduler::submit` + `StorageBackend::delete` | same |
| `blob-load` | `ComputePool::run_blocking` + `RouterEffectHandler::handle(BlobLoad)` | same |
| `blob-write` | same, `RouterEffect::BlobWrite` | same |
| `blob-read` | same as `blob-load`, wrapped as a single-chunk `stream<u8>` (no chunked blob backend exists — see `## streams`) | same |
| `http-fetch` | `HttpPool::fetch` (head) + a `spawn_scoped` background task pulling `HttpPoolBody::next_chunk` into a real multi-chunk `stream<u8>` | checked before `fetch`; guard disarmed once the head arrives (streaming task is not interrupted once spawned — matches `⚡️effects`'s own "checked-before-dispatch-only" cancellation model) |
| `document-read` | `ComputePool::run_blocking` + `RouterEffectHandler::handle(DocumentRead)` | checked before dispatch |
| `document-write` | same, `DocumentWrite` | same |
| `link-resolve` | **not wired** — no backing service | checked, then typed `not-wired` fault |
| `registry-query` | **not wired** | same |
| `io-compose` | `RouterEffectHandler::handle(IoCompose)` | checked before dispatch |
| `io-run` | **not wired** — `semio_framework::kernel::Effect` has no `IoRun` variant yet either (same `A3` gap `🦀️component.rs`'s own `wit_effect_to_kernel` already documents) | checked, then typed fault |
| `cache-derive` | `RouterEffectHandler::handle(CacheDerive)` | checked before dispatch |
| `cache-read` | same, `CacheRead` | same |
| `invoke-extension` | same, `InvokeExtension` | same |
| `open-window` | **not wired** — no window-manager service exists in `semio-framework-os-services`/`AsyncServices` | checked, then typed fault |
| `open-dialog` | **not wired** | same |
| `dispatch-action` | `RouterEffectHandler::handle(DispatchAction)` | checked before dispatch |
| `spawn-plugin-instance` | **not wired** | checked, then typed fault |
| `request-file-open` | **not wired** | same |
| `request-media-frames` | **not wired** | same |
| `request-capability` | **not wired** — no capability broker reachable from this crate | same |
| `spawn-job` | **not wired** — by design owned by the shard loop, not this executor (mirrors `AsyncEffectExecutor::execute`'s own `shard_owned` bucket) | same |

14 of 24 route to a real, already-verified service (`HttpPool`/`StorageScheduler`/`ComputePool`+
`RouterEffectHandler`). The other 10 have full `OperationContext`/cancellation/capability-tracking
wiring but end in a typed `not-wired` fault (`fault_bytes("not-wired", ...)`) — same "fails loudly"
idiom `UnwiredRouterEffectHandler`/`UnwiredStorageBackend`/`UnwiredHttpTransport` already establish
in this codebase, not a silent placeholder.

## why no completion round-trip

Every wired import directly `.await`s the real service call inside its own trait-method body
(`ComputePool::run_blocking(...).await`, `StorageTicket::await_result().await`,
`HttpPool::fetch(...).await`) and returns the result as the function's own return value — the
returned Rust future *is* wasmtime's correlation, so there is no `req: request-id`, no
`Event::Completed`, and no `EnvelopeCompletionSink` anywhere in this file. `AsyncEffectExecutor`'s
`dispatch_*` methods (fire far into a `spawn_scoped` task, answered later via the sink) are the
poll-world's own mechanism and are never called from here.

`emit`/`emit-patch` are the deliberate exception: they are plain `func` (not `async func`) — one-way
doors with no future to resolve — so they just convert (`emit`) or pass through (`emit-patch`) and
push onto `AsyncActorHostState::effect_sink` / `patch_sink`. A later packet (the actor task driving
`runner::run`) drains both sinks once per turn via `take_effects`/`take_patches` and hands the batch
to `AsyncEffectExecutor::execute` — the ONE classifier, never reimplemented here.

## AsyncActorHostState

```
AsyncActorHostState {
    services: Arc<crate::effects::AsyncServices>,       // same instance AsyncEffectExecutor uses
    router_handler: Arc<dyn crate::effects::RouterEffectHandler>,
    scope: ScopeHandle, actor: u64, generation: u16, package: PackageId, lane: u8,
    capability: Option<CapabilityTokenId>,               // one ambient capability per actor-store (see honest gaps)
    capability_registry: Arc<DirectAwaitCapabilityRegistry>,  // see honest gaps — NOT the same instance as effects::CapabilityRevocationRegistry
    caps: Vec<BrokerCapabilityGrant>,                     // carried, unused so far — mirrors ActorHostState's own #[allow(dead_code)] caps field
    trace_ids: TraceIdAllocator,                          // local copy of effects' own allocator
    effect_sink: Vec<semio_framework::kernel::Effect>,
    patch_sink: Vec<wit_ui::UiPatch>,                     // kept in WIT shape — kernel::UiPatch conversion is the same open B1 gap 🦀️component.rs's poll loop already documents
    limiter: BudgetLimiter, wasi_ctx: WasiCtx, resource_table: ResourceTable,
}
```

One `Store<AsyncActorHostState>` per actor (S1b's confirmed shape) means `actor`/`generation`/
`package`/`lane` are plain fixed-at-construction fields, not a per-call registry lookup — simpler
than `⚡️effects`'s `ActorScopeRegistry`, which has to serve every actor from one shared executor.

## streams

- `http-fetch`: real, per-chunk streaming. `HttpPool::fetch` returns `(HttpResponseHead,
  HttpPoolBody)`; a `spawn_scoped` background task OWNS the `HttpPoolBody` and loops
  `next_chunk().await`, pushing real chunks into a `Mutex<ChunkShared>` + waking — the exact S5
  shape (park on empty queue, store the waker, wake from elsewhere), just fed by a scoped async task
  instead of an OS thread. This is what actually fixes the poll bridge's documented gap
  (`Event::HttpChunk` only keeping the final `done==true` chunk).
- `blob-read`: single-chunk fallback, per the mission's own explicit allowance. There is no chunked
  blob backend anywhere in this codebase — `RouterEffectHandler::handle(BlobLoad)` is one buffered
  `ComputePool::run_blocking` call — so `blob-read` calls the same buffered path and hands back an
  already-`done`, one-item `ChunkStreamProducer`. Genuinely honest, not a stand-in: real chunked
  blob delivery needs a new service this packet's owned paths cannot add.

## line ranges edited in 🦀️component.rs

- Lines 15-20 (new): 6-line `#[path = "⏳️imports.rs"] pub mod imports;` mount + doc comment,
  inserted immediately after the existing `pub mod effects;` mount (was line 14, unchanged).
  Nothing else in this 4498-line file was touched.

## commands + exit codes

All **UNRUN** — rule 4, coordinator owns every acceptance build:
```
cargo check -p semio-framework-plugin-host --all-targets      # UNRUN
cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity     # UNRUN, baseline 115/0/1
```
Non-cargo checks actually run:
```
rustfmt --check --edition 2021 🖥️host/⏳️imports.rs        # exit 1 — reformatting diffs only
                                                              # (long single-line record literals vs
                                                              # rustfmt's wrapping — same style
                                                              # ⚡️effects/🦀️component.rs already uses
                                                              # unformatted); NOT a parse error —
                                                              # rustfmt only emits a diff when it
                                                              # successfully parsed the file first.
rustfmt --check --edition 2021 🖥️host/🦀️component.rs      # exit 1 — same, pre-existing file style,
                                                              # confirms the surgical edit didn't
                                                              # break parsing.
grep "component-model-async" wasmtime-47.0.3/Cargo.toml     # confirmed inside wasmtime's own
                                                              # `default = [...]` feature list —
                                                              # this crate's `wasmtime` dependency
                                                              # never sets default-features = false.
```

## what is testable now vs blocked on the runtime packet

**Testable now (once the coordinator runs the acceptance commands)**: whether the crate compiles at
all — this is the first real use of wasmtime's `Accessor`/`StreamProducer`/`HasSelf`/`bindgen!`
async-import machinery outside the `🧫️fixtures/🔌️asyncprobe` spike, so `cargo check` is a genuine
first test of every exact API surface guessed at in this file (see `## honest gaps`'s "unverified
against a real compile" list). The existing 115/0/1 test baseline should be unaffected — nothing in
`⚡️effects/🦀️component.rs` or any existing test module was touched.

**Blocked on the runtime packet**: there is no live `Store<AsyncActorHostState>`, no `Linker`
wiring `pure`/`host_async::add_to_linker`, and no actor task driving `runner::run` yet — all of that
is explicitly the next packet's job per the mission. The real run-the-real-thing gate I'd want the
coordinator to run once that exists: **instantiate a real `actor-async` guest, drive one turn that
calls `storage-write` then `storage-read` through the real `StorageScheduler`+a real
`StorageBackend` (not `UnwiredStorageBackend`), then `http-fetch` a small response through a real
`HttpTransport` (not `UnwiredHttpTransport`) and drain its `stream<u8>` body chunk-by-chunk inside
the GUEST, then revoke the actor's capability mid-flight on a second concurrent `storage-read` and
observe the guest receive `capability-revoked` in-band while the actor keeps running** — this
exercises the direct-await path, the real multi-chunk stream, and the cancellation/revocation
mechanics all at once, and needs both a wired `StorageBackend`/`HttpTransport` (today only the
`Unwired*` stand-ins exist) and the runner-driving actor task.

## lease-requests

None outstanding for Cargo.toml (see `## delivered` — verified unnecessary). One soft suggestion,
not a blocking lease-request since I worked around it inside my own owned path: `⚡️effects/
🦀️component.rs`'s `CapabilityRevocationRegistry::track` (line ~136) is private (only `revoke` is
`pub`) — making it `pub` would let `imports.rs`'s `DirectAwaitCapabilityRegistry` be deleted in
favor of the real one, unifying capability-cancel bookkeeping across the poll-batch `emit` path and
the direct-await path. Left as a suggestion for whichever packet next touches that file, since I
must not edit it myself.

## honest gaps

- **`capability: Option<CapabilityTokenId>` is one ambient id per actor-store**, not per-call — WIT's
  `host-async` funcs carry no `capability` parameter (mirrors `semio_framework::kernel::Effect`'s own
  read/write/http variants, which don't either), so there's no wire-level way to know which
  capability grant a given call should be tracked under beyond "the one this actor was given." A
  multi-capability actor would need a real design here; this packet's single-slot simplification is
  a placeholder for that.
- **A second small `DirectAwaitCapabilityRegistry`**, not the one `AsyncEffectExecutor` uses — see
  `## lease-requests`. This does NOT violate "do not build a second classifier": it is bookkeeping
  for cancel-token revocation, not a second effect classifier (only `AsyncEffectExecutor::execute`
  classifies effects, and only `emit`/`emit-patch` feed it).
- **`storage-read`'s `option<pack>` never produces `None`.** `StorageBackend::read` returns
  `Result<Vec<u8>, std::io::Error>` with no structured "not found" signal reachable from this
  module (the scheduler collapses it to `StorageError::Io(String)`), so every read failure surfaces
  as `Err(fault_bytes("storage-error", ...))`, never `Ok(None)`. A real "not found" distinction needs
  either `StorageBackend::read` to return `Result<Option<Vec<u8>>, io::Error>` (a trait change, out
  of my owned paths) or a string-sniffing heuristic I deliberately did not add (too fragile to ship
  silently).
- **`patch_sink` stores WIT-shaped `ui::UiPatch`, not `semio_framework::kernel::UiPatch`** — the
  path/node encoding convention between the two (`patch-op`'s `path: list<u32>` + `node: pack` vs
  kernel `PatchOp`'s `path: String` + `node: UiNode`) is the SAME open gap `🦀️component.rs`'s own
  poll-loop `execute_turn` already documents inline (`ui_patches: Vec::new()`, "not yet implemented").
  I did not invent a lossy conversion; the sink just holds the WIT shape until that gap is resolved.
- **`emit`'s `io-run` case silently drops** (an `eprintln!`, no panic) since `emit: func(value:
  effect);` has no return channel to signal the same `Effect::IoRun`-has-no-kernel-counterpart error
  `io-run` (the async import) returns as a typed fault. Same root cause, two different manifestations
  because one path is fire-and-forget and the other is request/response.
- **`http-fetch`'s WIT `streaming: bool` field on `http-params` goes unused** — host-async always
  streams the body now (that's the whole point of this world vs the poll world's buffered
  `http-request` effect), so the field is vestigial here; harmless, just noted.
- **Nothing in this file has been compiled.** Every wasmtime 47 API signature (`Accessor::with`,
  `Access::get`, `StreamReader::new`, `StreamProducer::poll_produce`, the exact generated
  `HostWithStore<T>`/`Host` trait split for a NAMED interface with both sync and async funcs) is
  drawn from reading `wasmtime-internal-wit-bindgen-47.0.3`'s codegen source directly plus mirroring
  `🧫️fixtures/🔌️asyncprobe/🖥️host/🦀️main.rs`'s proven-building shape as closely as WIT allows — but
  that fixture's world declares its imports directly at world scope, never inside a *named*
  interface with a MIX of sync and async funcs the way `host-async` does, so the sync/async trait
  split (`Host` vs `HostWithStore<T>`) for `emit`/`emit-patch` alongside the 24 async funcs is the
  one part of the wasmtime-internals reading I could not cross-check against a second real example
  in this codebase. `cargo check` is the first real test of that.
