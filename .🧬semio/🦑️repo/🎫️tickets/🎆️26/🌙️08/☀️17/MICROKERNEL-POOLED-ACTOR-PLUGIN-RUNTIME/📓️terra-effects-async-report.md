# terra-effects-async — Async Effect Execution

## build status — READ THIS FIRST

**The module has never compiled successfully. Not once, ever, this session.** I wrote
`⚡️effects/🦀️component.rs`, mounted it, and edited plugin-host's `Cargo.toml`, then ran the three
acceptance commands. All three were auto-backgrounded by the tool harness at its 120s default
(this machine had ~18 concurrent cargo processes from other packets/sessions), and I made the
mistake of waiting on them across a turn boundary instead of stopping and reporting — that
background state does not survive the turn, so I collected zero real output from any of them: no
compiler errors, no test pass/fail, nothing. I have since killed all three background tasks and am
running NO further builds in this turn, per the coordinator's explicit instruction.

Every command below is marked **UNRUN**. Nothing in this report about "the tests pass" or "it
compiles" should be read as verified — it is my best self-review of the code as written, not a
build result. I did two passes: (1) a manual type/signature check against the actual definitions of
every type I depend on (`OperationContext`, `HostAsyncRuntime`, `CompletionSink`, `HttpPool`,
`StorageScheduler`, `TimerWheel`, `EventRouter`, `ComputePool`, `Effect`, `Event`, `Envelope`,
`Payload`, `ActorId`, etc. — all read directly from their source files, not assumed); (2) a
line-by-line re-read of the whole module after writing it, with no build available, looking
specifically for move/borrow errors and test-logic bugs. That second pass found and fixed THREE
real bugs, which is itself the strongest evidence I have that more may remain:

- `dispatch_http` and `dispatch_router_effect` each referenced `scope` (by `&scope`) inside an
  `async move { ... }` block that also needed the same `scope` value afterward for
  `spawn_scoped(&scope, ..)` — `async move` moves every captured variable, so this would have been
  a hard `use of moved value` compile error. Fixed by cloning into a separate `scope_for_task`
  before building the future.
- The capability-revocation test asserted the NON-revoked operation completes `Ok`, but the
  executor it ran against was built with `UnwiredRouterEffectHandler`, which always returns `Err`
  — the assertion would have failed (or worse, passed for the wrong reason) regardless of whether
  the revocation logic was correct. Fixed by swapping in an always-succeeds handler for that test.
- One backbone test compared a `MutexGuard<Vec<Vec<u8>>>` slice against `&[Vec<u8>; 1]`, an
  array/slice `PartialEq` pairing I was not fully confident resolves unambiguously. Simplified to a
  plain `Vec` comparison to remove the doubt.

I have reasonable but NOT verified confidence the module is close to compiling after these fixes.
There is still a realistic chance of remaining errors I could not catch by reading alone —
`Send`/`'static` bound propagation through nested `Arc<dyn Trait>` compositions and
`ComputePool::run_blocking`/`race_deadline`'s closures is the area I am least certain of. I am
flagging this plainly rather than asserting green.

## commands + exit codes — ALL UNRUN

```
CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-ea cargo check -p semio-framework-plugin-host --all-targets
→ UNRUN (backgrounded, killed before completion, no output collected)

CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-ea cargo test  -p semio-framework-plugin-host --lib -- --skip schema_parity
→ UNRUN (never invoked — blocked behind the check above)

CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-ea cargo test  -p semio-framework-os-services
→ UNRUN (backgrounded, killed before completion, no output collected; this crate's own code was
  not touched by this packet, so a pass here would only confirm the baseline, not my work)
```

No exit code from any of these three commands is known. Whoever runs acceptance next should treat
`semio-framework-plugin-host` as **unverified, possibly red**.

## delivered

- New module `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`:
  `AsyncEffectExecutor`, `EnvelopeCompletionSink` (+ `EnvelopeInjector` seam), `ActorScopeRegistry`,
  `CapabilityRevocationRegistry`, `BackboneRegistry` (+ `BackboneTransport`/`CapabilityChecker`
  seams), `RouterEffectHandler` seam, `StorageBackend` seam, `race_deadline` helper, metrics
  recording seam, and a full unit test suite exercising the six required properties plus
  classification/backbone coverage.
- Mounted from `🖥️host/🦀️component.rs` via a `#[path]` module declaration (see `## line ranges
  edited`).
- `Cargo.toml` (plugin-host's own, not the registrar-only root one): added
  `semio-framework-async`/`semio-framework-os-services` dependencies, mirroring
  `semio-framework-os-services`'s own `testkit` dev-dependency pattern.

## effect → service routing table

| Effect | Route |
|---|---|
| `HttpRequest` | `HttpPool::request` (async, via `spawn_scoped`) |
| `StorageRead`/`StorageWrite`/`StorageDelete` | `StorageScheduler::submit` + own `race_deadline` (StorageScheduler does not race deadlines internally) |
| `SetTimer` | `TimerWheel::arm`/`disarm` for per-plugin QUOTA only; firing/completion is this executor's own `sleep_until` loop — see `## honest gaps` for why `TimerWheel::spawn_driver` was not used |
| `PublishEvent` | `EventRouter::publish` + per-recipient inbox drain |
| `SendMessage` (`MessageEndpoint::Backbone`) | `BackboneRegistry::send` (capability-gated) |
| `SendMessage` (other targets) | documented no-op gap — no id→ActorId directory owned here |
| `Subscribe`/`Unsubscribe` | `EventRouter::subscribe`/`unsubscribe` (direct, synchronous) |
| `BlobWrite`/`BlobLoad`/`DocumentRead`/`DocumentWrite`/`IoCompose`/`CacheDerive`/`CacheRead`/`InvokeExtension`/`DispatchAction` | `RouterEffectHandler::handle` via `ComputePool::run_blocking` |
| `SpawnJob`/`CancelJob` | left untouched — shard loop's own territory |
| everything else (`OpenWindow`, `Navigate`, `Notify`, ...) | shell-owned, passed through untouched |

## OperationContext derivation

Per effect: `actor`/`generation` from `ActorScopeRegistry` (generation snapshotted at dispatch
time, checked again at delivery); `trace` fresh from `TraceIdAllocator`; `lane` from the dispatch
context; `deadline_ms` = the effect's own deadline (if any) clamped to a per-lane host ceiling
(`LANE_DEADLINE_CEILING_MS`), always `Some`; `cancel` = `actor_scope.cancel.child()`; `capability`
= the dispatch batch's capability token, also registered into `CapabilityRevocationRegistry` for
revocation.

## cancellation matrix

| Trigger | Mechanism | Actor survives? |
|---|---|---|
| Suspend | `CancelToken::park()` on the actor's scope; completions keep accumulating in its private mailbox, undelivered until `resume()` calls `EnvelopeCompletionSink::flush` | yes |
| Capability revoked | `CapabilityRevocationRegistry::revoke` cancels only the child tokens registered under that capability id; each in-flight future's own cooperative check emits `Event::Completed{Err(capability-revoked)}` | yes |
| Trap | `cancel_scope(Actor, grace 0)`; `ScopeDrainReport` recorded via `EffectMetricsRecorder`; stale completions dropped by generation gating once the caller re-activates at a bumped generation | no (by design) |
| Quarantine/Disable | `cancel_scope(Package, grace 250ms)`; `ScopeDrainReport` recorded | no (by design, package-wide) |

## EffectBackbone wire shape

See the module doc on `BackboneRegistry`/`BackboneTransport`. For the TypeScript counterpart:
`{ "kind": "send" | "delta", "uri": string, "payload": <base64>, "revision"?: number }` — `send`
mirrors `Effect::SendMessage`'s payload; `delta` carries a monotonic `revision` so the guest can
detect a collapsed delta the same way `UiPatch.base_revision` detects a stale diff. Deltas fan out
through `EventRouter` under `ChannelPolicy::Coalesced{key: uri}` so a burst for the same uri
collapses to the latest.

## PostTurnRelay verdict

**Kept.** Grep evidence (`PluginInstanceHandle`, the actual content of the `//#region
🔀️PostTurnRelay` block at lines 1310–1392): it is a low-level per-plugin cold-job dispatch
mechanism (`start-job`/`step-job` to `Done`/`Failed`), NOT a live per-turn effect dispatcher. It is
actively used today by `IoRouter::run_io`/`io_sniff` (lines ~2101, ~2133) and
`ArtifactInferenceRouter::infer` (line ~2366) to actually execute a guest job — this executor's own
`RouterEffectHandler` seam is expected to call INTO those same routers (which still use
`PluginInstanceHandle` underneath), so retiring it would break the very routing table this packet
implements. Separately confirmed: no code anywhere in this repository outside tests drives a live
`Kernel`/turn loop yet (`grep -rn "Kernel::new("` → zero non-test call sites), and
`🏃️run/🦀️component.rs` explicitly documents its own `run_transaction`/`undo_transaction_group`
gap as "a real post-turn dispatch loop that belongs with the kernel/scheduler (H1-H4/T1)" — i.e.
there was never a live synchronous-after-every-turn effect dispatcher to retire in the first place;
this packet is additive infrastructure for when that loop exists, not a replacement for a mechanism
that was already running.

## line ranges edited in 🖥️host/🦀️component.rs

Lines 3–13 (was 3–8): inserted a 6-line `#[path = "⚡️effects/🦀️component.rs"] pub mod effects;`
module declaration (plus a 3-line doc comment) immediately after the existing
`pub mod process_transport;` declaration. No other line in this 264 KB file was touched.

## unfinished, bluntly

- **Zero verified builds.** See `## build status` at the top — this is the headline fact.
- **No acceptance test has actually run.** The six required properties (revoked-capability,
  quota-denial, generation-gating, park/resume ordering, deadline enforcement, backpressure bound)
  are all WRITTEN as `#[cfg(test)]` cases in `⚡️effects/🦀️component.rs`'s own test module, but I
  have not seen a single one execute.
- **`EnvelopeInjector`, `RouterEffectHandler`, `StorageBackend` are all seams with no real
  implementation** — by design for this packet (see `## honest gaps`), but that means nothing this
  executor does reaches a real kernel, a real router, or real storage yet, even once it compiles.
- **`SetTimer`'s quota-exceeded path and non-`Backbone` `SendMessage` targets are silent no-ops**,
  not because I ran out of time to decide but because the effect schema/actor directory genuinely
  don't carry what's needed yet (see `## honest gaps`) — flagging so this isn't mistaken for
  untested-but-complete.
- **A full line-by-line self-review is done (see `## build status` for the three bugs it caught
  and fixed), but a manual read is not a compiler.** The very next step, once the cargo queue on
  this machine drains, is simply: run the three acceptance commands for real and read their actual
  output. Nothing before that point should be treated as verified.

## lease-requests

None. Touched only: the new `⚡️effects/🦀️component.rs` module, a minimal `#[path]` mount line in
`🖥️host/🦀️component.rs`, and `🖥️host/📦️packages/🦀️rust/Cargo.toml` (not registrar-only — a
per-crate manifest, not the workspace root). `async-worlds`'s `🧬️schema/📜️component.wit` and
`🖥️host/🧪️schema-parity/`, and `kernel-loop`'s wgpu `📦️glue.rs`/`🎠️runtime.rs` and
`🖥️host/🧵️shard/**`, were never opened.

## honest gaps

- **`EnvelopeInjector` has no live implementation.** No code anywhere in this repository drives a
  real `Kernel`/shard loop outside tests today — there is nowhere real yet for a finished envelope
  to go. This packet ships the executor, the sink, and a thoroughly unit-tested contract against
  `RecordingEnvelopeInjector`; wiring a real implementation against `ShardTransport` is
  `kernel-loop`'s or a later packet's job.
- **`RouterEffectHandler` has no concrete wiring to `IoRouter`/`ArtifactInferenceRouter`/
  `ArtifactMutationRouter`/`HostTransactionCoordinator`/`AppRouter`.** No such mapping exists
  anywhere in the codebase today (every router is invoked from application-level orchestration,
  never a per-effect loop) — `UnwiredRouterEffectHandler` fails loudly until a caller wires one.
- **`StorageBackend` has no concrete implementation** — mirrors `HttpTransport`'s own
  `UnwiredHttpTransport` gap in `semio-framework-os-services`.
- **`TimerWheel::spawn_driver` is not used.** `TimerFired`/the wheel's own `TimerId` inner field is
  private outside `semio-framework-os-services` with no accessor, so there is no way to translate
  the wheel's own auto-incrementing id back to the GUEST's chosen `SetTimer.id` through that path.
  This executor instead uses `TimerWheel::arm`/`disarm` for quota admission only and drives its own
  `sleep_until` loop for firing — see the module's own doc on `dispatch_set_timer`. A follow-up
  could add a `TimerId` accessor to `os-services` to unlock the shared driver.
- **`SetTimer`'s quota-exceeded case has nowhere to report to** — the effect carries no `req:
  RequestId` to answer with `Event::Completed`; a quota-denied arm is silently refused today.
- **`SendMessage` to `PluginInstance`/`Shell`/`Extension`/`Topic` targets is a documented no-op** —
  there is no id→`ActorId` directory owned by this module to resolve them; only
  `MessageEndpoint::Backbone` is wired.
- **`Effect::Subscribe` carries no `ChannelPolicy` on the wire** — defaults to `LatestWins` pending
  a schema addition.
- **Per-batch capability is a simplification** — `EffectDispatchContext.capability` is one token
  for the whole batch; a real per-effect capability association (e.g. from `CapabilityRequest`)
  is future wiring.
- **`PublishEvent`/message delivery lane is approximated** as the sender's own lane, not the
  recipient's declared lane (no per-actor lane registry exists in this module).
- **`Effect::Subscribe` bakes the actor's CURRENT generation into the `EventRouter` key.** A
  restarted actor's new generation produces a different `ActorId`, so its old subscription is
  orphaned in `EventRouter` rather than cleaned up — a real leak under restart churn, not caught by
  any test here.
