# 📓️ terra-host-repair — plugin-host `--lib` residue repair

## Summary

Started at 123 errors (task brief said 124; 123 measured at run start — likely one fixed by a
concurrent session between brief-writing and my first check). Ended at **6 errors, all one
architecturally-blocked cluster confirmed out of `path_scope`** (see `## Not fixed` below).
`semio-s-plugin-note --lib` fan-out went from 123 (all attributed to plugin-host) to **38, all now
in the guest SDK (`🔌️plugin/⚛️reactor`, `🔌️plugin/🌐host`), zero in plugin-host**.

## Acceptance results (all run in foreground, this session, exit codes pasted from real runs)

1. `cargo check -p semio-framework-plugin-host --lib` → **EXIT 101, 6 errors** (down from 123).
   All 6 are one confirmed cluster requiring an out-of-scope fix — see `## Not fixed`.
2. `cargo check -p semio-framework-plugin-host --all-targets` → **EXIT 101, 931 errors**
   (E0599 421, E0308 250, none/other 115, E0609 55, E0277 54, E0369 21, E0053 9, E0614 3, E0608 3).
   Known-and-reported, not silently accepted: this is almost entirely `#[cfg(test)]` residue of the
   same async-codemod shape (`#[test] fn` calling now-async methods without `.await`, e.g.
   `MutationKind::label`/`diff` test call sites in `🎚️config`, `RecordingStorageBackend`/
   `RecordingRouterHandler`/`AlwaysOkRouterHandler` test impls in `⚡️effects`, `#[test] async fn`
   pairs in `🧵️shard/🚚️process-transport`). Not touched this session — `--lib` was the
   contractual gate and the volume here is genuinely a separate packet's worth of work (matches
   this ticket's established pattern of routing `#[cfg(test)]` residue to its own packet, e.g. the
   `sdk-final`/`dispatch-group-split` findings already on record).
3. `cargo check -p semio-s-plugin-note --lib` → **EXIT 101, 38 errors** (down from 123). All 38 are
   in `🔌️plugin/⚛️reactor/**` and `🔌️plugin/🌐host/**` — the guest SDK, explicitly out of my
   `path_scope` per the packet brief. **Zero remaining errors are in plugin-host.**
4. Regression guards, all still green:
   - `cargo check -p semio-framework-plugin --lib` → **EXIT 0** (18 warnings, pre-existing style
     lints, unrelated to this packet).
   - `cargo check -p semio-framework-os-kernel --lib` → **EXIT 0** (57 warnings, all
     `async_fn_in_trait`, R7-sanctioned, unchanged).
   - `cargo test -p semio-framework-os-kernel --lib` → **779 passed / 0 failed / 0 ignored** —
     matches the ticket's recorded baseline exactly.

## Not fixed — confirmed out-of-scope architectural blocker

**6 errors, all `E0277`/Send, in `⚡️effects/🦀️component.rs` at `dispatch_http` (:841),
`dispatch_set_timer`'s spawned loop (:861/:900), `dispatch_router_effect` (:971).**

Root cause: `AsyncEffectExecutor<I, R: HostAsyncRuntime>`'s `dispatch_*` methods are generic over
`R`, and each builds a `HostFuture<()> = Pin<Box<dyn Future<Output=()> + Send + 'static>>` via
`Box::pin(async move { ... R::run_blocking(...).await ... })` (or `R::sleep_until(...).await`).
`HostAsyncRuntime::run_blocking`/`sleep_until` are `async fn` in a trait (AFIT) with no `Send`
bound on their returned futures (correctly, per R7 — "NEVER silence it by writing `-> impl
Future<Output = T> + Send`"). Since `R` is still a type parameter at this point (not
monomorphized), rustc cannot prove the resulting nested future is `Send` for arbitrary `R`, so the
cast to `HostFuture<()>` fails.

Per **R3**: *"Host side: Send-ness is obtained STRUCTURALLY — every former dyn seam becomes a
concrete enum, so at each spawn site the future's concrete type is known and the compiler derives
Send itself... If a generic host path needs to spawn a trait-method future, the fix is route it
through the enum, never add a bound."* — the correct fix is enum-closing `HostAsyncRuntime`
(`dyn_enum_close!`-style) so `run_blocking`/`sleep_until` dispatch through a concrete enum instead
of a generic bound. `HostAsyncRuntime` is declared in
`🧰️framework/🔨️modules/⏳️async/🦀️component.rs` — **outside this packet's
`🔌️plugin/🖥️host/**` path_scope**, so I did not touch it.

Ruled out as unsafe/wrong before stopping:
- **Pinning `R` to a concrete type** (the fix I used for `imports.rs`'s `AsyncActorHostState`,
  which had zero production OR test callers needing genericity) — **not applicable here**:
  `AsyncEffectExecutor`'s own test module (`mod tests`) genuinely instantiates it generically over
  both `TokioHostRuntime` and `semio_framework_async::testkit::ManualRuntime` (the deterministic
  virtual-clock test double), so pinning would break real test infrastructure, not just theoretical
  genericity.
- **Return-type-notation / `+ Send` bound** — explicitly banned by R3 ("Never `+ Send` RPITIT,
  never return-type-notation, never `trait-variant`").
- **`resolve_ready`-style bridge** — wrong for `sleep_until` specifically: it's a genuine timer
  wait, not a pure/instant computation, so forcing it to resolve on first poll would either busy-loop
  or break the actual delay semantics.

**Lease-request**: `🧰️framework/🔨️modules/⏳️async/🦀️component.rs` needs `HostAsyncRuntime`
enum-closed (or an equivalent structural-Send fix) for `AsyncEffectExecutor`'s three `dispatch_*`
spawn sites to compile. Whoever owns `⏳️async` (or a follow-up packet) should pick this up.

## What I fixed (by category)

### Reverted mistaken broad asyncify (self-inflicted, corrected same session)
Ran `asyncify-universal.py --apply` over the whole `🔌️plugin/🖥️host` scope early on (439 fns),
following the tool's own documented workflow for "un-converted or reverted scope." This was too
broad — it broke `dyn`-dispatched traits (`BackboneTransport`, `CapabilityChecker`,
`EffectMetricsRecorder`, `StorageBackend`, `RouterEffectHandler` — all documented as deliberately
`dyn` in this file) and several `wasmtime::component::bindgen!`-generated `Host` trait impls
(E0038/E0053, ~40 errors at peak). Reverted all of them to sync with `// 🚫️async: E1/R9` tags
citing the specific external constraint (dyn-safety or bindgen-fixed signature). Lesson for the
next packet touching a partially-converted file: check for `dyn Trait` and `bindgen!` blocks
*before* running the blind codemod over a scope, not after.

### R9 reversions (pure fns, no I/O, external/dyn/bindgen consumer forced sync)
`TraceIdAllocator::{new,next}`, `DirectAwaitCapabilityRegistry::{new,track,revoke}`,
`CancelOnDrop::{new,disarm}`, `lane_ceiling_ms` (all `⏳️imports.rs`); `StorageOp::{byte_hint,run}`
(R4 clause 3 — `StorageScheduler::submit`'s `work` param is a plain sync closure);
`RuntimeMetricsPublisher::new`, `ActorScopeRegistry::new`, `CapabilityRevocationRegistry::new`
(all consumed by `impl Default`); `hex_encode` (consumed by `impl Debug for CompiledHandle`);
`EpochTicker::start` (spawns a thread but has zero internal `.await`s itself); the topological-sort
`visit` helper in `validate_inference_dependency_graph` (pure recursion, would otherwise ALSO need
`Box::pin` for zero benefit).

### R10 residue shape 1 (await inside a sync closure) — hoisted
`Option::and_then(|bytes| decode_dsl(&bytes))` at 8 sites across `⏳️imports.rs` and
`🦀️component.rs`'s effect-conversion matches; `io_route_rank`/`resolve_io_route`'s `sort_by`
comparators (`IoFidelity::rank`, `ArtifactDialect::to_coordinate`); `ok_or_else`/`map_err` closures
constructing `TransactionError::rejected`/`fault_bytes` in `run_transaction` and the
`HostWithStore` blob/http paths; `Iterator::map` closures building `wit_events::Event`/
`CapabilityGrant` vecs.

### R10 residue shape 2 (future awaited more than once / too late) — awaited once at binding
`begin_call`/`CallSnapshot` split in `⏳️imports.rs` (the big one — `.with()`'s closure cannot
hold a `&mut AsyncActorHostState` across an `.await`, so `begin_call` now takes an owned
`CallSnapshot` extracted synchronously inside `Accessor::with`, and does its real awaiting
outside); `EnvelopeCompletionSink::{flush,complete}` and `derive_ctx` in `⚡️effects`;
`OpeningResolver::resolve`; `RuntimeMetricsPublisher::maybe_sample`.

### R10 residue shape 3 (self-recursive async fn) — `Box::pin`
`ArtifactInferenceRouter::infer_with_visited`'s recursive call, per rustc's own E0733 hint (this
one genuinely awaits real plugin-runtime I/O via `handle.infer`, so R9 did not apply).

### E0107 (wrong generic-arg count)
`AsyncActorHostState`/`CallContext` (`⏳️imports.rs`) referenced `AsyncServices` bare instead of
`AsyncServices<R>`. Pinned to `TokioHostRuntime` (the one production `HostAsyncRuntime` impl) rather
than threading `R` through — this whole async-world state has zero production OR test callers
needing genericity (confirmed via repo-wide grep for `AsyncActorHostState::new(`).

### Cross-crate cascades fixed inside `path_scope`'s `#[path]`-included modules
`🎚️config/🧬️schema/**` (opening-preferences mutations) is `#[path]`-included into
`semio-framework-plugin-host` via `📦️glue.rs` (physically under `💻️os/🎚️config`, outside the
literally-named `🔌️plugin/🖥️host`, but compiled as part of my crate — same precedent as the
`sdk-features` packet's earlier finding). Fixed: `protocol::MutationDiff`/`MutationKind` impls made
async to match their now-async external trait declarations; `apply_opening_config_mutation`/
`inverse_opening_config_mutation` kept sync (real external caller in `🏃️run/🦀️component.rs`, a
different crate) via a `block_on` bridge, tagged E5.

### Silent no-op fixes (bare statement calls to now-async fns, dropped futures)
Added `.await` to ~10 previously-bare `spawn_scoped(...)`/`deliver_message(...)` statement calls in
`⚡️effects/🦀️component.rs` and `⏳️imports.rs`. Verified zero "unused implementer of
`std::future::Future`" warnings remain in `semio-framework-plugin-host --lib`'s own output.

### Struct-literal shorthand corruption (a distinct insert-await bug pattern, 4 instances)
`Envelope { to.await, ... }`, `IoRoute { hops: best, fidelity.await }`,
`Self { ..., engine_config_hash.await }`, `TransactionOutcome { txn_id.await, ... }` — an earlier
insert-await pass had appended `.await` directly onto a struct-literal field-init-shorthand
identifier, turning valid `field,` into invalid `field.await,`. Fixed each by either restoring the
shorthand (once the value was properly awaited at its own binding) or `field: value.await,`.

## Files touched (all inside `🔌️plugin/🖥️host/**` except the two `🎚️config` cascades, justified above)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️imports.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🔺️diff/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/🧹clear-default-app/🦠️mutation/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🔺️diff/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🧬️mutations/📌️set-default-app/🦠️mutation/🦀️component.rs`

## Refused / out of scope

- **`⏳️async` (`HostAsyncRuntime` enum-closing)** — see `## Not fixed` above. Lease-request open.
- **`--all-targets`'s 931 `#[cfg(test)]` errors** — reported, not fixed; matches the ticket's
  established pattern (`sdk-final`'s 1,381-error `--all-targets` residue was routed the same way)
  of treating `--lib` and test-residue as separate packets' work.
