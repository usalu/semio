# 📓️ terra-services report — packet R2 (`semio-framework-os-services`)

Executor `terra-services`. New crate at `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/` — the
sibling of `semio-framework-async` (packet R1) where `tokio` actually lives, and nowhere else.

## delivered

Regions 1–4 are fully wired and behaviourally tested. Regions 5–7 are interface-complete with
correct quota/backpressure accounting but a deliberately-thin transport/wiring layer, as the packet
brief allowed. Region 8 is complete and used to close the loop on region 3.

1. **`🚂️TokioHostRuntime`** — `impl HostAsyncRuntime`. Builds ONE `tokio::runtime::Runtime` via
   `Builder::new_multi_thread().worker_threads(plan.io_workers).max_blocking_threads(plan.compute)`.
   Constructor is `new(plan: ThreadPlan, budget: &ThreadBudget)` — checks out `IoWorker`/`Compute`
   threads from `budget` at construction, calls `available_parallelism` nowhere. `now_ms`/
   `sleep_until` are anchored to one `epoch: tokio::time::Instant` read from INSIDE the runtime's own
   context at construction, so the driven clock is self-consistent regardless of caller context.
   Fully wired.
2. **`🌳️ScopeTable`** (private — see `## tokio-containment evidence`) — root scope per package,
   child scope per actor, one `tokio::task::JoinSet` per scope, `Arc`-shared so `cancel_scope` can
   return a `'static` boxed future without borrowing `&self`. `cancel_scope` cancels the target scope
   and every descendant it can reach via a parent/child adjacency map, drains each scope's `JoinSet`
   within the grace period via `tokio::time::timeout`, and reports `finished`/`cancelled`/`leaked`
   truthfully — anything still in the set when the grace timeout fires is counted `leaked` and THEN
   force-aborted (so it does not become a real OS-level leak), never silently folded into
   `finished`. `Park` is implemented as a poll (`await_live_or_cancelled`, 20ms interval) at the top
   of every spawned task/blocking-op wrapper — new work spawned while parked waits there before
   running its real body; in-flight work (already past the gate) is untouched by a later park.
   Documented limitation: `CancelToken` (frozen R1 interface) exposes no unpark notification, so this
   is a poll, not an event wake. Fully wired and tested (transitive cancel, leaked-vs-finished,
   park-holds-then-runs — all three pass against a REAL `TokioHostRuntime`, not a fake).
3. **`⏲️TimerWheel`** — split exactly as asked: `WheelCore` is a pure struct (`BinaryHeap` +
   `HashMap`, `arm`/`disarm`/`pop_expired`/`next_expiry_ms`, every method takes `now_ms` as a
   parameter, zero tokio, zero clock) plus a thin `TimerWheel::spawn_driver` task that sleeps until
   the next expiry (or wakes early on `arm` via `tokio::sync::Notify`), pops what's due, and posts
   each firing through `CompletionSink::complete` — the only re-entry path. Per-plugin timer-count
   quota enforced in `arm` before any insertion. Repeats catch up (`while next_expiry <= now_ms`)
   rather than drifting. Fully wired, including the driver, tested end-to-end against
   `semio_framework_async::testkit::ManualRuntime` (no tokio needed for that test at all).
4. **`🧮️ComputePool`** — `tokio::sync::Semaphore` sized to `plan.compute` gates admission;
   `run_blocking` races BOTH the admission wait and the result wait against
   `runtime.sleep_until(ctx.deadline_ms)` via `tokio::select!`, returning `ComputeError::
   DeadlineExceeded` on either loss. Honest limitation stated in the error's own doc: the losing
   blocking OS thread is not forcibly killed (blocking threads are not preemptible) — only the
   async-side wait is abandoned. Fully wired; burst-concurrency bound and deadline-race are both
   tested against a REAL `TokioHostRuntime` (see `## honest gaps` for why `ManualRuntime` cannot
   exercise either).
5. **`🌐️HttpPool`** — per-package `network_bytes_per_min` token bucket + per-actor
   `outstanding_requests` cap, both enforced BEFORE the pool ever calls `ComputePool::run_blocking`.
   `HttpTransport` is a trait (not a concrete client), so this crate adds no new HTTP dependency;
   `UnwiredHttpTransport` is the default and fails loudly. Quota/backpressure accounting is real and
   tested; the transport itself is interface-only — see `## honest gaps`.
6. **`💾️StorageScheduler`** — `BTreeMap<u8, VecDeque<StorageJob>>` keyed by `ctx.lane` (ascending —
   lower lane dispatches first), a plain `AtomicU32` in-flight counter (no tokio semaphore needed),
   and a REENTRANT `storage_try_dispatch` free function called from `submit` and again from every
   job's own completion closure — no separate background polling task exists. Per-plugin byte quota
   reserved before queuing, released on completion whether the op succeeds or fails. Deadline racing
   (unlike `ComputePool`) is NOT wired here — the brief only asked for it on `ComputePool`; see
   `## honest gaps`.
7. **`📮️EventRouter`** — `HashMap<Topic, Vec<Subscriber>>` plus a per-`(topic, actor)` `Mailbox`
   built from that subscriber's own `ChannelPolicy`. `LatestWins`/`Coalesced` collapse in place;
   `LosslessBounded` rejects at `cap` rather than growing; `ByteCredit` spends a running budget.
   `subscribe`/`unsubscribe`/`publish`/`send_message`/`drain` all present. Draining a mailbox into a
   real `CompletionSink::complete` call with a real actor generation is NOT wired — see
   `## honest gaps`; this region's own contract (the routing/backpressure DECISION) is fully tested.
8. **`CompletionSink`** — trait plus `MockCompletionSink` test double (records every call in order).
   This is the ONLY re-entry path exercised by `TimerWheel::spawn_driver` in this packet; no type in
   this crate holds or calls a `Kernel`. Fully wired and used for real by region 3's own test.

## commands + exit codes

All three run from
`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust` with
`CARGO_TARGET_DIR=<ticket>/🎯️target-r2`, foreground, one turn each (no backgrounding, no Monitor).

```
$ cargo check -p semio-framework-os-services --all-targets
    Checking semio-framework-os-services v0.1.0 (…/🛎️services/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
exit=0
```

```
$ cargo test -p semio-framework-os-services
running 26 tests
test component::tests::event_router_latest_wins_collapses_older_pending_value ... ok
test component::tests::event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket ... ok
test component::tests::event_router_coalesced_collapses_same_key_but_queues_distinct_keys ... ok
test component::tests::event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth ... ok
test component::tests::cancel_scope_cancels_child_scopes_transitively ... ok
test component::tests::event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber ... ok
test component::tests::http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called ... ok
test component::tests::mock_completion_sink_records_calls_in_order ... ok
test component::tests::timer_wheel_driver_posts_a_fired_timer_through_the_completion_sink ... ok
test component::tests::tokio_host_runtime_checks_out_io_and_compute_threads_from_the_budget ... ok
test component::tests::storage_scheduler_rejects_over_budget_submit_with_a_typed_error_and_untouched_usage ... ok
test component::tests::wheel_core_disarm_frees_quota_for_a_new_arm ... ok
test component::tests::storage_scheduler_dispatches_highest_priority_lane_first_despite_submit_order ... ok
test component::tests::wheel_core_disarm_prevents_a_future_fire ... ok
test component::tests::wheel_core_pop_expired_respects_now_ms_boundary ... ok
test component::tests::wheel_core_pop_expired_fires_in_expiry_order_not_arm_order ... ok
test component::tests::wheel_core_rejects_arm_past_the_per_plugin_quota_with_a_typed_error ... ok
test component::tests::wheel_core_repeat_rearms_and_catches_up_without_drift_accumulation ... ok
test component::tests::tokio_host_runtime_now_ms_advances_monotonically ... ok
test component::tests::cancel_scope_counts_a_cooperative_task_as_finished ... ok
test component::tests::run_blocking_deadline_actually_fires_and_the_late_result_is_not_awaited ... ok
test component::tests::http_pool_rejects_past_the_per_actor_outstanding_cap ... ok
test component::tests::cancel_scope_reports_leaked_task_that_ignores_cancellation_not_finished ... ok
test component::tests::park_holds_new_work_until_unparked ... ok
test component::tests::run_blocking_never_exceeds_the_compute_bound_under_a_burst ... ok
test component::tests::storage_scheduler_never_exceeds_max_in_flight ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

Doc-tests semio_framework_os_services
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
exit=0
```

```
$ cargo clippy -p semio-framework-os-services --all-targets -- -D warnings
    Checking semio-framework-os-services v0.1.0 (…/🛎️services/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.37s
exit=0
```

**Also verified beyond the mandated three**, since `[lints] workspace = true` isn't wired yet (see
`## lease-requests`) and plain `cargo clippy` only ran its own defaults: I re-ran clippy with every
extra `[workspace.lints.clippy]` lint the root `Cargo.toml` enables
(`cloned_instead_of_copied`, `inefficient_to_string`, `map_unwrap_or`, `needless_pass_by_value`,
`semicolon_if_nothing_returned`, `unnecessary_wraps`, `redundant_clone`) plus the `[workspace.lints.
rust]` set, all under `-D warnings`. First pass found 13 real hits (needless-pass-by-value on
`ScopeTable`'s internal `spawn_scoped`/`run_blocking`/`TimerWheel::spawn_driver`/
`storage_try_dispatch`, two `map(..).unwrap_or(..)` chains, and several genuinely-redundant
`.clone()` calls in both source and tests) — all fixed in place; the same command now exits 0. This
means the crate will not regress the moment the registrar adds `[lints] workspace = true`.

## tokio-containment evidence

```
$ grep -nE 'tokio' 🦀️component.rs | grep -v '^\s*[0-9]*://' | wc -l
40
$ grep -nE '^\s*pub (fn|struct|trait|enum|type)' 🦀️component.rs | grep -c 'tokio::'
0
$ grep -nE '^\s*pub [a-z_]+:\s*tokio' 🦀️component.rs
(no output — no public struct FIELD of a tokio type either)
```

All 40 occurrences are either: this file's own module-doc prose, or inside `impl` bodies of private
types (`ScopeTable`/`ScopeTableInner`/`StorageState`/`storage_try_dispatch`), or inside `#[cfg(test)]
mod tests`. Every `pub struct`/`pub fn`/`pub trait`/`pub enum` signature line was inspected by hand
(listed in full below) — none names a `tokio::` path. One deliberate structural choice worth
recording: `StorageTicket` originally wrapped its `tokio::sync::oneshot::Receiver` as an unnamed
tuple-struct field (`pub struct StorageTicket(tokio::sync::oneshot::Receiver<…>);`), which is a
PRIVATE field (no `pub` before the type) and therefore not actually exposed — but it put the literal
substring `tokio::` on the same line as `pub struct`, which a naive same-line grep could misread. Re-
written as a named-field struct with the tokio-typed field on its own (still private) line, so the
struct's own declaration line is clean:

```rust
pub struct StorageTicket {
    receiver: tokio::sync::oneshot::Receiver<Result<Vec<u8>, StorageError>>,
}
```

Full list of every `pub fn`/`pub struct`/`pub trait`/`pub enum` signature line in the file (55 total,
via `grep -nE '^\s*pub (fn|struct|trait|enum|type)' 🦀️component.rs`) — spot-read every one; none
contains `tokio::`. `TokioHostRuntime` the TYPE NAME is the one place "tokio" appears in a public
signature at all, which is this crate's own type (exactly what packet R2 exists to name) and not the
external-library leak the rule prohibits.

## thread-budget evidence

`TokioHostRuntime::new` takes `ThreadPlan` as a constructor argument and never calls
`std::thread::available_parallelism` (grepped: zero occurrences in the whole file):

```rust
pub fn new(plan: ThreadPlan, budget: &ThreadBudget) -> Result<TokioHostRuntime, RuntimeBuildError> {
    budget.checkout(ThreadRole::IoWorker, plan.io_workers);
    budget.checkout(ThreadRole::Compute, plan.compute);
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(plan.io_workers.max(1) as usize)
        .max_blocking_threads(plan.compute.max(1) as usize)
        ...
```

Test `tokio_host_runtime_checks_out_io_and_compute_threads_from_the_budget` asserts the budget's
`IoWorker`/`Compute` remaining counts drop to exactly `0` after construction, AND that `Shard`
(a role this crate does not own) is untouched — proving the checkout is scoped to only the two roles
this crate is responsible for, not a blanket draw.

## lease-requests

**Mandatory — workspace membership** (same pattern R1/F1 used): the crate currently builds standalone
via a temporary `[workspace]` opt-out table + a mirrored `[workspace.dependencies]` block in its own
`Cargo.toml` (`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml`).
Requesting sol:
1. Add member path `"🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust"` to root
   `Cargo.toml`.
2. Add `[workspace.dependencies]` alias `semio-framework-os-services = { path = "…" }`.
3. Delete this crate's own `[workspace]` table and its mirrored `[workspace.dependencies]` block
   (the real root aliases for `tokio` and `semio-framework-async` already exist and are what this
   crate's `workspace = true` lines will resolve against once membership lands).
4. Add `[lints] workspace = true` to this crate's `Cargo.toml`.
5. Delete this crate's local `Cargo.lock`
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.lock`).

No other lease requests — every file this packet touched is inside its own owned path prefix.

## honest gaps

- **`HttpPool` has no real transport.** `HttpTransport` is a trait with only `UnwiredHttpTransport`
  (fails loudly) shipped. The brief explicitly allowed this ("a real connection pool can replace the
  innards invisibly later") and asked the quota/backpressure contract to be correct instead — that
  part is fully tested (`http_pool_rejects_past_the_per_actor_outstanding_cap`,
  `http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called`). A later packet wires
  a real `ureq`-backed (or better) `HttpTransport`, reusing the `📇️directory/🔌️client` blocking-
  thread technique via this crate's own `ComputePool`.
- **`TokenBucket::refill` has no scheduler.** The per-minute replenishment tick is not driven by
  anything in this packet — `HttpPool::refill_package_budget` exists as the hook a later timer-driven
  packet calls (naturally, via `TimerWheel` itself, once that wiring is written).
- **`EventRouter` does not call `CompletionSink`.** `publish`/`send_message` decide and queue into a
  `Mailbox`; draining that mailbox into a real `CompletionSink::complete` call needs the actor's
  CURRENT `OperationContext.generation` (the kernel/turn generation), which is separate from
  `ActorId`'s own packed 14-bit restart-generation and has no source of truth in this crate — that
  lives in the kernel, which supplies a fresh `OperationContext` per turn. Later-packet wiring. The
  routing/backpressure DECISION itself (what `EventRouter::publish`/`drain` actually decide) is what's
  tested, per the packet's own acceptance list.
- **`StorageScheduler` does not race `ctx.deadline_ms`.** Only `ComputePool` was asked to enforce
  deadlines; storage ops queue and run in priority order with no deadline preemption. A later packet
  can add the same `tokio::select!`-against-`sleep_until` technique `ComputePool` already
  demonstrates, if a storage op ever needs one.
- **`TimerWheel::spawn_driver`'s completion attribution is per-arm, not per-plugin-default.**
  `arm` takes `actor`/`generation`/`lane` explicitly (added beyond the mission text's bare `arm(ctx,
  id, at_ms, repeat)` signature) so each firing can be attributed correctly — without this, every
  firing from every plugin would have been reported under one arbitrary fixed identity, which would
  have been a worse, dishonest simplification. Recorded here as a deliberate signature deviation from
  the packet's literal text, not an oversight.
- **`ScopeTable::cancel_scope`'s `Park` holding is a poll, not an event wake** (`PARK_POLL_INTERVAL_MS
  = 20`), because `CancelToken` (frozen R1 interface) exposes no unpark notification — documented
  inline at `await_live_or_cancelled`'s definition.
- **`ScopeTable` assumes at most one OPEN scope per `ScopeOwner`** at a time (root-per-package,
  child-per-actor, per the design) — re-opening the same owner replaces the owner-index entry; no
  caller in this packet does that, so it is a documented limitation rather than an exercised bug.

## naming collisions checked (per the standing five-strikes rule)

**Superseded by the `## coordinator follow-up` section below** — this packet originally invented
local `PluginId`/`ServiceActorId` newtypes to avoid same-name collisions with
`semio_framework_actor::PackageId`/`ActorId`. The coordinator corrected this: since this crate
already depends on nothing but `semio-framework-async` + tokio, the "avoidance" was actually two
types for one concept, which is worse than a name collision. Both newtypes are deleted; this crate
now depends on `semio-framework-actor` directly and uses its `PackageId`/`ActorId` throughout.

- `TimerId`/`HttpResponse` still exist as unrelated types in other crates this crate does not depend
  on (`🔄️machine`, a plugin FSM, and `📇️directory/🔌️client` respectively) — kept as-is since there is
  no dependency-graph collision, only a coincidental name; noted here for visibility per the standing
  instruction to flag discoveries even when a rename isn't warranted.

## files touched (all newly created — no existing file was edited)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🦀️component.rs` (1,662 lines, after `cargo fmt`
  against the repo's `rustfmt.toml` — `max_width = 250`, which is why struct-literal constructors
  collapse to one line rather than the narrower default; also after relocating every inline `//`
  comment discovered inside a function/match-arm body into a `///` doc comment on the enclosing
  item, per this repo's "no comments inside definitions" rule)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust/📋️project.json`

No TS package directory was created — this crate is a native host-process implementation detail with
no wire types needing a TS mirror (same pattern as its sibling `🔌️plugin/🖥️host`), per the packet
brief's "+ a TS package dir only if you actually put something in it."

## coordinator follow-up

Accepted the correction without reservation — no evidence surfaced that changes it. Applied:

1. **`Cargo.toml`**: added `semio-framework-actor = { workspace = true }` to `[dependencies]`, with a
   doc comment recording why (pure crate, no tokio/threads, builds for `wasm32-unknown-unknown`,
   matches `🔌️plugin/🖥️host`'s existing precedent).
2. **`🦀️component.rs`**: deleted the `PackageId`/`ActorId` local newtypes (the old `//#region
   🪪️Vocabulary` block) and every use of them; added `use semio_framework_actor::{ActorId,
   PackageId};` and mechanically renamed every call site (`PluginId` → `PackageId`, `ServiceActorId` →
   `ActorId` — both drop-in renames, since both pairs share the same underlying shape:
   `PackageId(pub String)` / `ActorId(pub u64)`). Rewrote the crate-doc naming paragraph to explain
   the actual boundary instead: `PackageId`/`ActorId` are used for genuine plugin/actor identity
   ([`TimerWheel`]'s quota, [`StorageScheduler`]'s byte budget, [`HttpPool`]'s outstanding cap,
   [`EventRouter`]'s subscribers), while `CompletionSink::complete` and `TimerFired`'s
   `actor`/`generation` fields deliberately stay bare `u64`/`u16` — those mirror
   `OperationContext`'s own untyped re-entry shape, and `OperationContext.generation` (the kernel/turn
   generation) is a genuinely different concept from `ActorId`'s packed 14-bit restart-generation, so
   converting one into the other there would be a category error, not a typing improvement. This
   distinction is why the fix touched `PackageId`/`ActorId` throughout the file but did NOT touch
   `TimerWheel::arm`'s `actor: u64, generation: u16` parameters or `CompletionSink`'s signature.

**A real, unrelated peer collision surfaced while re-verifying — reported, not worked around.**
Immediately after adding the dependency, `cargo check -p semio-framework-os-services` failed
through `semio-framework-actor` itself:
```
error[E0164]: expected tuple struct or tuple variant, found struct variant `Backpressure::Dropped`
error[E0164]: expected tuple struct or tuple variant, found struct variant `TurnStatus::Faulted`
error[E0533]: expected value, found struct variant `Backpressure::Dropped`
```
Checked before touching anything: `git diff --stat HEAD -- 🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
showed live, growing uncommitted churn (76 → 79 → 108 insertions across three ~10s polls; the actor
crate's own `component.rs` grew from 2,963 to 3,018 lines across a further three ~15s polls, with
`mtime` advancing each time) — a peer session actively mid-edit on `Backpressure`/`TurnStatus`'s enum
variant shapes, seconds old each time, not a stale/abandoned breakage. Not our file, not our
regression, and per the ticket's own "don't chase a moving target" rule this was a target still
moving, so nothing was touched. Waited (no cargo processes were running to "wait out" a lock on —
this was pure source churn) and re-ran `cargo check` once more a short time later: it passed clean,
confirming the peer had finished. All three acceptance commands below were run immediately after
that, back to back, to minimize the risk of re-observing another mid-flight state.

### re-verified acceptance (after the correction, as a real workspace member — no `cd`, no
`[workspace]` opt-out needed)

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-r2 cargo check -p semio-framework-os-services --all-targets
    Finished `dev` profile [unoptimized] target(s) in 0.25s
exit=0
```

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-r2 cargo test -p semio-framework-os-services
running 26 tests
test component::tests::event_router_byte_credit_rejects_when_insufficient_and_admits_after_refund_style_new_bucket ... ok
test component::tests::event_router_latest_wins_collapses_older_pending_value ... ok
test component::tests::event_router_lossless_bounded_rejects_at_cap_without_unbounded_growth ... ok
test component::tests::event_router_coalesced_collapses_same_key_but_queues_distinct_keys ... ok
test component::tests::event_router_unsubscribe_removes_the_mailbox_and_future_publishes_see_no_subscriber ... ok
test component::tests::mock_completion_sink_records_calls_in_order ... ok
test component::tests::cancel_scope_cancels_child_scopes_transitively ... ok
test component::tests::http_pool_rejects_when_byte_budget_exhausted_and_transport_is_never_called ... ok
test component::tests::timer_wheel_driver_posts_a_fired_timer_through_the_completion_sink ... ok
test component::tests::storage_scheduler_rejects_over_budget_submit_with_a_typed_error_and_untouched_usage ... ok
test component::tests::storage_scheduler_dispatches_highest_priority_lane_first_despite_submit_order ... ok
test component::tests::tokio_host_runtime_checks_out_io_and_compute_threads_from_the_budget ... ok
test component::tests::wheel_core_disarm_frees_quota_for_a_new_arm ... ok
test component::tests::wheel_core_disarm_prevents_a_future_fire ... ok
test component::tests::wheel_core_pop_expired_fires_in_expiry_order_not_arm_order ... ok
test component::tests::wheel_core_pop_expired_respects_now_ms_boundary ... ok
test component::tests::wheel_core_rejects_arm_past_the_per_plugin_quota_with_a_typed_error ... ok
test component::tests::wheel_core_repeat_rearms_and_catches_up_without_drift_accumulation ... ok
test component::tests::tokio_host_runtime_now_ms_advances_monotonically ... ok
test component::tests::cancel_scope_counts_a_cooperative_task_as_finished ... ok
test component::tests::http_pool_rejects_past_the_per_actor_outstanding_cap ... ok
test component::tests::run_blocking_deadline_actually_fires_and_the_late_result_is_not_awaited ... ok
test component::tests::cancel_scope_reports_leaked_task_that_ignores_cancellation_not_finished ... ok
test component::tests::park_holds_new_work_until_unparked ... ok
test component::tests::run_blocking_never_exceeds_the_compute_bound_under_a_burst ... ok
test component::tests::storage_scheduler_never_exceeds_max_in_flight ... ok

test result: ok. 26 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

Doc-tests semio_framework_os_services
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
exit=0
```

```
$ CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-r2 cargo clippy -p semio-framework-os-services --all-targets -- -D warnings
    Checking semio-framework-actor v0.1.0 (…)
    Checking semio-framework-os-services v0.1.0 (…)
    Finished `dev` profile [unoptimized] target(s) in 0.86s
exit=0
```

Baseline held: **26 passed / 0 failed**, unchanged from before the correction.

### tokio-containment re-check (adding the actor dependency did not change this)

```
$ grep -nE '^\s*pub (fn|struct|trait|enum|type)' 🦀️component.rs | grep -c 'tokio::'
0
$ grep -nE '^\s*pub [a-z_]+:\s*tokio' 🦀️component.rs | wc -l
0
$ wc -l 🦀️component.rs
1655
```

`PackageId`/`ActorId` now appear 33/22 times respectively (all real usages); zero `PluginId`/
`ServiceActorId` remain anywhere in the file.

Also re-ran `cargo fmt -p semio-framework-os-services --check` after the rename — exit 0, no diff.
