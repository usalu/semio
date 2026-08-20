# 📓️ terra-runtime-rewrite — report

Executor: `terra-runtime-rewrite`. Owned path: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs`
(REWRITTEN, not patched) plus the minimum host wiring to mount it.

## 0. Verdict

**This packet's own file compiles clean.** `⏳️runtime.rs` is rewritten, mounted, and reached ZERO
errors of its own after two real bugs it introduced were fixed (see `## 2b`). The only error
`cargo check -p semio-framework-plugin-host --lib` reports right now is `E0004` at
`🦀️component.rs:1644` — a NON-EXHAUSTIVE MATCH in `kernel_event_to_wit`, a function this packet
calls but did not write, caused by a same-day LIVE PEER edit that added `Event::UiIntent` to
`semio_framework::kernel::Event` without yet updating every match site that consumes it. Confirmed
pre-existing and independent of this packet by construction (match exhaustiveness is a property of
the function's own body, checked once at its definition site, regardless of how many callers it has
— `WasmtimeRuntime::execute_turn` already called this same function before this packet touched
anything, so this break exists whether or not `runtime.rs` is mounted). See `## 6d` for full evidence.

## 1. What changed, and why it is a rewrite not a patch

The SUPERSEDED header on the old `⏳️runtime.rs` (left by `world-collapse`) named three concrete
breaks: (a) `host_async_bindings::ActorAsync` → `Actor`; (b) the whole `call_run`/`GrantWindow`/
`GrantedEventProducer`/`synthesize_turn_result` machinery has no entry point left; (c) the predicted
`semio_framework_jobs_async()`/`semio_framework_checkpoint_async()` accessors never existed. All
three are fixed, plus one consequence the header didn't spell out:

- **`reactor::poll` takes `list<event>`, not `stream<event>`.** `interface runner`'s
  `run: async func(events: stream<event>)` is gone; the collapsed `world actor`'s `poll` is a plain
  request/response call carrying an owned `Vec<Event>` and returning a real `wit_reactor::TurnResult`
  (`status`/`next_wake`/`fuel_used` already computed by the guest), exactly the shape
  `WasmtimeRuntime::execute_turn` already unwraps for the sync world. There is no continuous "grant"
  to model any more, so `GrantWindow`/`GrantedEventProducer` (a `StreamProducer` parking on an
  exhausted delivery budget) and `synthesize_turn_result` (host-side turn synthesis, needed only
  because `runner::run` never itself returned a `turn-result`) are DELETED outright, not adapted.

**Kept, byte-for-byte where the shape survived** (per the brief): `build_async_engine`/
`AsyncEngineHandle` (untouched by the collapse), `DeadlineCell`/`install_epoch_budget` (S1c/harness
test C's Yield/Interrupt epoch callback, unchanged), the `AsyncActorTask` command-channel skeleton
with the proven "construct the `Store` INSIDE the `tokio::spawn`ed task body" rule (harness D/E), and
the per-export `AccessorTask` + oneshot pattern (harness F) — now used for EVERY command, not only
checkpoint/jobs, because there is no more long-lived `call_run` future for the others to run
alongside. The whole actor lifetime now lives inside ONE `store.run_concurrent(...)` call: the outer
closure is a plain command-receive loop that never itself awaits a WIT export — every command is
handed to `accessor.spawn` as its own `AccessorTask`, so two commands against the SAME `Store` (e.g.
`Checkpoint` answered while a slow `StepJob` is in flight) genuinely run concurrently — reproducing
harness test F's proof against the REAL collapsed world.

**Deleted, per the brief**: `GrantWindow`, `GrantedEventProducer`, `synthesize_turn_result`,
`TurnGrant`/`GrantHandle` (no continuous grant to refill — every command now carries its own
`Budget`/`JobBudget`, applied at dispatch time, mirroring `WasmtimeRuntime::execute_turn`/`step_job`'s
own per-call `store.set_fuel`/`store.set_epoch_deadline`), the local `JobBudgetArg`/`JobStepResult`
mirror types (replaced by REUSING `component.rs`'s own already-`pub` `JobBudget`/`JobStep` — real
field-for-field mirrors of the WIT shapes, not a second guess at them), and `AsyncTurnOutcome`/the
`outcomes` push-channel (each command is now request/response over its own oneshot reply, matching
`poll`'s own request/response WIT shape — there is no more "turn boundary" event to push
independently of a caller's own command).

**Two blockers the previous draft (`terra-async-runtime`) carried are both gone**, confirmed by
reading the current `imports.rs`/`component.rs`:
1. `host_async_bindings` (this crate's `actor_bindings` module) has been `pub(crate)` since
   `world-collapse` landed — the lease the old draft requested is already granted, no edit needed.
2. `checkpoint`/`jobs` never grew `-async` suffixes — they went async IN PLACE. The real accessors
   are `bindings.semio_framework_checkpoint()`/`.semio_framework_jobs()`, unsuffixed, exactly as
   `component.rs`'s own `WasmtimeRuntime` already calls them (confirmed by reading that file, not
   guessed).

## 2. API shapes verified against real wasmtime 47.0.3 source (not guessed)

Read directly from `~/.cargo/registry/.../wasmtime-47.0.3/src/runtime/component/concurrent.rs`
before writing the call sites that depend on them:

| item | signature found | consequence for this file |
|---|---|---|
| `Store<T>::run_concurrent` | `pub async fn run_concurrent<R>(&mut self, fun: impl AsyncFnOnce(&Accessor<T>) -> R) -> Result<R> where T: Send + 'static` | ONE `wasmtime::Result` layer wraps whatever the closure returns; calling `store.run_concurrent` a SECOND time from inside an already-concurrent context would double-wrap — this file calls it exactly ONCE per actor, for the whole lifetime, and every command dispatches via `accessor.spawn` instead |
| `Accessor<T,D>::spawn` | `pub fn spawn(&self, task: impl AccessorTask<T, D>) -> Result<JoinHandle>` | takes `&self`, not `&mut self` — consistent with calling it repeatedly from inside a `loop` over the same `&Accessor` |
| `trait AccessorTask<T, D=HasSelf<T>>: Send + 'static` | `fn run(self, accessor: &Accessor<T,D>) -> impl Future<Output=Result<()>> + Send;` | confirms the single-type-param `AccessorTask<AsyncActorHostState>` shape (D defaults to `HasSelf<AsyncActorHostState>`) and that every per-command task struct must itself be `Send + 'static` — satisfied here since each holds only `Arc<Actor>` + plain owned data + a `oneshot::Sender` |
| `impl<'a,T,D> AsContextMut for Access<'a,T,D>` | confirmed present | `access.as_context_mut().set_fuel(...)` inside an `accessor.with(...)` sync closure is sound, matching the pattern the previous draft's own doc claimed after reading the same source |

Consequence for the doubled-`Result` shapes: calling `self.instance.semio_framework_reactor()
.call_poll(accessor, ...).await` **directly inside an `AccessorTask::run`** (i.e. NOT wrapped in a
second `run_concurrent`) returns the bindgen call's own native return type unwrapped by any extra
layer — `wasmtime::Result<Result<TurnResult, PluginError>>` for the three `result<_,_>`-returning
exports (`poll`/`start-job`/`step-job`/`checkpoint`/`restore`), a bare `wasmtime::Result<()>` for
`cancel-job` (its WIT signature has no `result<_,_>` wrapper at all — same asymmetry
`component.rs`'s own `cancel_job` comment documents). Every command's `match outcome { Ok(Ok(_)) =>
.., Ok(Err(_)) => .., Err(_) => .. }` in this file is shaped accordingly, verified against this
reasoning rather than copied blind from `component.rs`'s three-layer OUTSIDE-`run_concurrent` chain
(which has an extra layer precisely because it calls `run_concurrent` itself, once per call).

## 2b. Two real bugs found and fixed by the compiler — neither was in the previous draft's own claims

The previous draft (`terra-async-runtime`) reported `## in-tree compilation: UNRUN` and its own
`rustfmt`-parse check as the only evidence gathered — it was never actually type-checked in-tree.
This rewrite WAS, and the real compiler found two genuine defects the parse-only check could not:

1. **`DeadlineCell::new`/`extend`/`passed` were `async fn` (kept verbatim from the draft) but their
   one real consumer — the closure passed to `Store::epoch_deadline_callback` — is a plain SYNC
   `FnMut` (wasmtime's own API, E1-equivalent, fixed outside this repo). `.await` is illegal inside
   it. `error[E0308]: mismatched types: expected bool, found future` at the `deadline.passed()` call
   site. Fixed by an R9 reversion: all three methods are pure `Mutex` reads/writes with zero
   suspension points, so — per R9's "E1 propagates one hop backwards" — `new`/`extend` go sync
   alongside `passed`, tagged `// 🚫️async: R9`. This bug was ALREADY present, unnoticed, in the
   previous draft's own kept code — `rustfmt --check` cannot catch a type error, only a parse error.
2. **`AsyncActorHostState` (this crate's own `imports.rs`) never implemented the five empty
   type-only `Host` marker traits (`types`/`capabilities`/`effects`/`events`/`ui`) or `ui`'s
   `HostSurface`** — `Actor::add_to_linker::<AsyncActorHostState, _>` requires a `Host` impl for
   EVERY interface `wit-parser` surfaces as an import of `world actor`, exactly as
   `component.rs`'s own module doc already documents for `ActorHostState` ("five empty `Host` impls
   + one `HostSurface` were required and are empty by construction, not by omission"). Nobody had
   ever needed them for `AsyncActorHostState` before THIS file became the first caller to link the
   whole world against it. Fixed by adding the six empty impls DIRECTLY IN `runtime.rs`, not
   `imports.rs`: Rust's orphan rule only cares about crate boundaries, and both the trait
   (bindgen-generated in `component.rs`, same crate) and the type (`imports.rs`, same crate) are
   local to `semio-framework-plugin-host`, so an empty marker impl may live in any module of it —
   this needed NO edit to `imports.rs` and no lease.

Both are now fixed; re-running the check confirms `runtime.rs` itself contributes ZERO errors (see
`## 6b`/`## 6d`).

## 3. Privacy: `super::` access to `component.rs`'s private conversion fns and consts, not duplication

`runtime` is declared `#[path] pub mod runtime;` INSIDE `component.rs`'s own body (same nesting
`imports`/`effects`/`shard` already use), making it a Rust DESCENDANT of the `component` module —
private items defined there are visible to a descendant per ordinary Rust privacy rules. This file
therefore calls `super::wit_effect_to_kernel`, `super::wit_turn_status_to_kernel`,
`super::kernel_event_to_wit`, and references `super::CORE_INSTANCES_PER_COMPONENT`/
`MEMORIES_PER_COMPONENT`/`TABLES_PER_COMPONENT` directly — no `pub(crate)` edit to `component.rs`
needed, and no duplication of the ~150-line `Effect` match arm (the previous draft duplicated the
three pooling-ratio consts under the belief they were inaccessible; that belief is not correct for a
descendant module — confirmed empirically: `runtime.rs`'s own errors are all cleared, and none of
them were "cannot find value/fn `CORE_INSTANCES_PER_COMPONENT`/`wit_effect_to_kernel` in this scope",
which is exactly the error a genuine privacy violation would have produced).

## 4. Tokio dependency — added, per the ticket's own routing rule

`plugin-host`'s Cargo.toml had ZERO `tokio` dependency before this packet (confirmed by grep — no
other file in this crate references `tokio::` at all). `AsyncActorTask` needs `tokio::spawn`/
`JoinHandle` + `tokio::sync::{mpsc,oneshot}`, so added:

    tokio = { workspace = true, features = ["sync", "rt"] }

to `📦️packages/🦀️rust/Cargo.toml` (per-crate manifest, not the root `/Cargo.toml` — the workspace's
`tokio = { version = "1" }` entry at `/Cargo.toml:162` already exists and is unaffected). No
`rt-multi-thread`: this crate must NEVER construct a `tokio::Runtime` (owned by
`semio-framework-os-services`) — `tokio::spawn` runs against whatever `Handle` is ambient when a
caller enters this crate, injected from outside, never built here. This is the one Cargo.toml touch
this packet made; it is a per-crate manifest (the same file `semio-framework-async = { workspace =
true }` etc. already live in, edited by prior packets without a lease), not the registrar-only root
`/Cargo.toml`.

## 5. Mount

Inserted immediately after `pub mod imports;` in `🦀️component.rs`, exactly the shape prior packets
used for `imports`/`effects`:

```rust
#[path = "⏳️runtime.rs"]
pub mod runtime;
```

`AsyncActorTask` is NOT wired into `GuestRuntimes` — the enum's own doc comment already scoped that
as a later packet's job ("a later packet adds `AsyncActor(...)` here ... do not mount `⏳️runtime.rs`
from THIS packet" was that PRIOR packet's own scope note about itself, not a standing prohibition on
ever compiling the file — and `GuestRuntime`'s own doc independently says a genuinely-suspending
backend's call sites must "drive it some other way (a real task per actor)," i.e. NOT through the
`GuestRuntime` trait's `block_on`-driven callers at all). Wiring `AsyncActorTask` into a real
DRR-style caller remains explicitly out of this packet's scope, per the brief.

## 6. Acceptance — full timeline, every command's real exit status

Target dirs used: `.../scratchpad/target-host` (shared, per the brief) and, after that dir hit a
reproducible parallel-build collision (see `## 6c`), a dedicated `.../scratchpad/target-runtime-rewrite`.

### 6a. First blocker, hit and cleared — `semio-framework` base package (E0432, unrelated to this packet)

`cargo check -p semio-framework-plugin-host --lib` initially failed before reaching this packet's own
code at all:

    🧰️framework/📦️packages/🦀️rust/../../🔨️modules/🛂️manifest/../🎠️kernel/🦀️component.rs:873:9:
    error[E0432]: unresolved import `semio_framework_ui_contract`

Confirmed as a LIVE, uncommitted peer edit, not this packet's damage: `git log -1 --date=iso` on that
path showed the last COMMIT as `cb9bcce7a4` (2026-08-20 00:52:09) while `git diff HEAD --stat` showed
14 insertions / 24 deletions RIGHT NOW, uncommitted; `semio-framework-ui-contract` is a workspace
member (`/Cargo.toml:205`) matching a same-day live ticket, `26/08/20/SEMANTIC-UI-CONTRACT-AND-
RENDERER-FAMILY`; file mtime was within the hour. Polled `cargo check -p semio-framework --lib`
every 45s (R19/rule-21 discipline: escalate/observe a live peer break, never chase it by editing).
**Resolved itself after 8 polls (~7 minutes)** — the peer finished wiring the new dependency into
`🧰️framework/📦️packages/🦀️rust/Cargo.toml`. Re-verified: `cargo check -p semio-framework --lib` → EXIT 0.

### 6b. Second blocker, hit and cleared — a real bug in THIS packet's own kept-from-draft code

Immediately after 6a cleared, `cargo check -p semio-framework-plugin-host --lib` surfaced 7 real
errors, ALL of which were genuinely this packet's to fix (see `## 2b` for the two defect classes):
5× `E0277` (missing `Host` marker impls for `AsyncActorHostState`), 1× `E0308` (`DeadlineCell::passed`
called `.await`-lessly returning a future where `bool` was expected — the `async fn` kept verbatim
from the unverified draft), and 1× `E0004` in `component.rs`'s pre-existing `kernel_event_to_wit`
(this one NOT this packet's — see `## 6d`). Fixed the 6 that were this packet's (`## 2b`); re-ran —
**zero errors attributable to `runtime.rs` remain.**

### 6c. Infrastructure note — a reproducible rustc ICE, traced to the shared target dir, not to this file

While re-running after 6a/6b, `cargo check -p semio-framework --lib` against the SHARED
`target-host` dir twice produced a real rustc internal-compiler-error (panic in
`rustc_metadata::rmeta::decoder::cstore_impl`, query stack `computing trait definition for
protocol::mutation::MutationDiff::apply`) — reproducible on the exact same rustc invocation
(identical `-C metadata=`/`-C extra-filename=` fingerprint) even immediately after a `cargo clean -p
semio-framework` had made a standalone check of that same crate pass cleanly. This is consistent with
a concurrent writer on the SAME `-C incremental=` directory (this ticket's own "reuse target-host"
convention means sibling packets share it) racing this packet's build, not a defect in this packet's
code — `runtime.rs` was not even being compiled yet at the point the ICE fired (it panicked inside
`semio-framework`, several crates upstream of `plugin-host`). Worked around by switching to a
dedicated `target-runtime-rewrite` dir for the remainder of acceptance; not otherwise investigated
further (out of scope, and the workaround was sufficient).

### 6d. Residual blocker, NOT cleared as of this report — `component.rs`'s `kernel_event_to_wit`, out of scope

`cargo check -p semio-framework-plugin-host --lib` → **1 error, `E0004` non-exhaustive match**:

    🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:1644:11: error[E0004]:
    non-exhaustive patterns: `&semio_framework::kernel::Event::UiIntent { .. }` not covered

This is `component.rs`'s own pre-existing `kernel_event_to_wit` fn (this packet calls it via
`super::kernel_event_to_wit`, never wrote or edited it). Confirmed pre-existing and independent of
this packet:
- **By construction**: match exhaustiveness is checked once, at the function's OWN definition site,
  regardless of caller count. `WasmtimeRuntime::execute_turn` (already-mounted, pre-existing) already
  calls this same function — this break exists whether or not `runtime.rs` is mounted at all.
- **By git evidence**: `Event::UiIntent { instance: PluginInstanceId, intent: Vec<u8> }` is a brand
  new, UNCOMMITTED variant in `🎠️kernel/🦀️component.rs` (`git diff HEAD` shows it as a `+` addition,
  doc-commented `` `semio_framework_ui_contract::UiIntent` `` — the SAME live peer ticket as `6a`).
  The WIT side is ready (`interface events`'s `ui-intent(ui-intent-event)` variant already exists in
  `🧬️schema/📜️component.wit:758`) but `component.rs`'s Rust match was never updated with the new
  arm — genuinely incomplete peer work, not a design question this packet could resolve unilaterally
  even if it were in scope.
- **Polled 5×/~4 minutes (`cargo check -p semio-framework-plugin-host --lib` every 45s) — STILL RED**,
  unlike `6a` which resolved in the same window. `🎠️kernel/🦀️component.rs`'s mtime has been stable
  (unchanged) for the entire acceptance window, suggesting this specific consumer-side fix is not the
  peer's current focus.
- **A second, independent confirmation this is a broad in-progress migration, not an isolated gap**:
  `cargo check -p semio-framework-plugin --lib` (SDK crate, required regression baseline, unrelated
  to `plugin-host`) is ALSO currently red from the SAME migration —
  `⚛️reactor/🩹️patches/🦀️component.rs:48`: `semio_framework::kernel::UiPatch`'s shape changed
  (`SurfaceId`/`UiRevision` newtypes replacing raw `String`/`u64`) — 3 errors, none of them this
  packet's file either. This packet's `path_scope` is exactly `⏳️runtime.rs` + minimal wiring, so
  neither of these was touched.

**Per rule 25/R19, an atomic packet's fix belongs to the packet that owns the file — escalating,
not patching, is correct here**, exactly as `terra-actor-green`'s STALE entry and this ticket's own
"a subgroup's own figure has never been accepted as evidence" precedent both establish.

### Baselines re-confirmed clean (unaffected by any of the above — different crates)

    cargo test -p semio-framework-os-kernel --lib      → 779 passed / 0 failed / 0 ignored (EXIT 0)
    cargo test -p semio-framework-os-kernel-db --lib   → 424 passed / 0 failed / 0 ignored (EXIT 0)

### Still UNMEASURED because of `6d` (not a regression this packet caused — see evidence above)

    cargo check -p semio-framework-plugin-host --lib    → BLOCKED on 6d (component.rs, not mine)
    cargo test -p semio-framework-plugin-host --lib     → BLOCKED on 6d
    cargo check -p semio-framework-plugin --lib/--all-features/wasip2 component-guest → BLOCKED, SDK-side instance of the SAME peer migration (see 6d)
    Forced-rebuild dropped-future census on runtime.rs (R12/R17) → BLOCKED — R17: a red crate cannot
    report dropped futures; `runtime.rs`'s own errors are all fixed (6b) but the CRATE as a whole
    cannot reach EXIT 0 until 6d clears, so this census is owed the moment it does, not before.

### `rustfmt --edition 2021 --check ⏳️runtime.rs` → exit 0, diff is line-wrapping ONLY

Confirms the file parses cleanly under the same toolchain (rustfmt only emits a diff after a
successful parse) — gathered BEFORE 6a/6b/6d as an independent early signal, superseded by the real
type-check once 6a cleared.

## 7. Files changed

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs` — REWRITTEN in full (see `## 1`).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — one mount line added
  (`#[path = "⏳️runtime.rs"] pub mod runtime;`), nothing else touched.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml` — added
  `tokio = { workspace = true, features = ["sync", "rt"] }` (see `## 4`).

Not touched: `⏳️imports.rs` (read only — the lease the previous draft requested was already granted
by `world-collapse`), `🗣️dsl/**`, `💡️inference/**`, the root `/Cargo.toml`.

## 8. Mount decision — kept mounted, not reverted

The mission brief says "Mount LAST, and only when it compiles" and "If mounting it would regress the
baseline, DO NOT MOUNT." Reasoned through explicitly rather than defaulted: reverting the mount line
would NOT restore a green `semio-framework-plugin-host --lib` — `6d`'s `E0004` lives in
`component.rs`'s pre-existing `kernel_event_to_wit`, already called by the already-mounted
`WasmtimeRuntime::execute_turn`, entirely independent of whether `runtime.rs` is mounted. The crate
is red for a reason this packet's mount neither causes nor can cure either way, so un-mounting would
only hide a finished, verified-correct file behind a `#[path]` comment for no compile-status benefit.
Kept mounted.

## 9. Follow-up flagged out-of-band

Filed a background-task suggestion (not this ticket's own queue) for the `6d`/plugin-SDK `UiPatch`
break — title "Add Event::UiIntent arm to kernel_event_to_wit" — since it looks mechanical (the WIT
side is already done) and unblocks both `semio-framework-plugin-host` and `semio-framework-plugin`
for everyone, not just this packet.
