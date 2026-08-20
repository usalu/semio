# 📓️ terra — shard-lane report (packet `shard-lane`, executor "terra")

## 0. Scope

Path scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/**`. `🎭️actor` touched: **NO**
(read only throughout — confirmed via `git diff --stat` showing zero changes there both before and
after sol's scope-adjustment message withdrawing my conditional edit permission there).

## 1. Root-cause diagnosis — verdict

**CONFIRMED**, with two corrections to the mechanism as literally proposed (both cost nothing —
piece 1/2 are built and working without either change).

- `ShardLoop::pump`/`pump_primed` (`🧵️shard/🦀️component.rs`) drained the transport fully, then
  iterated `events_by_actor: HashMap<u64, Vec<Event>>` — HashMap iteration order, not meaningfully
  "arrival order" even before this packet. Confirmed by reading.
- Nothing preempted a running wasm turn once started, in the sense that matters: a turn that hit
  `TurnFault::DeadlineExceeded` was surfaced as `ShardOutcome::Fault` (kernel-escalation-worthy), not
  a graceful re-grant. Confirmed by reading `WasmtimeRuntime::execute_turn`'s error mapping
  (`🦀️component.rs:1173-1186`, pre-existing) and the old `pump_primed` turn loop's unconditional
  `Err(fault) => ShardOutcome::Fault`.
- **Correction 1**: `ShardFrame::Grant` does NOT need a new `lane` field to fix this — every
  `Envelope` bundled inside a `Grant` already carries its own `.lane` (`semio_framework_actor::
  Envelope.lane`), set once per actor by the kernel's DRR `Scheduler`. See §4 for why a literal
  `Grant`-level field would have broken two live call sites outside this packet's scope.
- **Correction 2**: the epoch-preemption ARMING piece 2 asked me to add (`Store::
  set_epoch_deadline` from the grant's `budget.wall_ms`/`deadline_ms`) was ALREADY present and
  already fed from the DRR grant's own budget via the existing `turn_budget_from_grant` bridge
  (`🧵️shard/🦀️component.rs:126-128`, consumed at `🦀️component.rs:1135/1156/1240`). What was
  genuinely missing was the GRACEFUL-YIELD behavior on top of that existing arming — see §3.

## 2. Piece 1 — lane-priority execution inside the shard

File: `🧵️shard/🦀️component.rs`.

- `ShardLoop` gained `actor_lanes: HashMap<u64, semio_framework_actor::Lane>` (struct field), kept
  current by one new line at the top of `Self::dispatch_envelope` (`self.actor_lanes.insert
  (envelope.to.0, envelope.lane)`) — covers both `ShardFrame::Grant`'s bundled envelopes and
  standalone `ShardFrame::Envelope` passthrough frames, since both paths already funnel through
  `dispatch_envelope`. Cleared in `unregister`.
- `Self::is_high_priority_lane` / `Self::actor_lane`: classify Interactive/UserVisible as high
  priority, Background/Maintenance as low; an actor never seen falls back to Maintenance (same
  fallback convention `granted_budget` already used).
- `pump_primed`: the old single `for (actor_id, events) in events_by_actor { execute turn inline }`
  loop is replaced with two `VecDeque<u64>` queues (`interactive_queue`, `background_queue`),
  classified via the new `Self::enqueue_by_lane`. The turn-execution body itself is factored into
  `Self::execute_turn_for` — byte-identical to the old inline body, plus the piece 2 fault-handling
  change (§3).
- Drain order: `interactive_queue` is drained EXHAUSTIVELY before a single `background_queue` grant
  is taken. After each background turn, the transport is re-checked non-blocking
  (`self.transport.recv()`, the same "never blocks past what's currently buffered" contract used
  everywhere else in this file) and any newly-arrived actor is classified and enqueued — an
  interactive grant that lands mid-pump, while a background turn is running, jumps the queue at the
  very next turn boundary rather than waiting for the next `pump()` call.

## 3. Piece 2 — epoch preemption bounds a single background turn

Files: `🧵️shard/🦀️component.rs` (the fix) + `🦀️component.rs` (`MockGuestRuntime` scripting support).

- Arming was already correct and untouched (§1 correction 2) — `WasmtimeRuntime::execute_turn`/
  `step_job` already call `store.set_epoch_deadline(budget.deadline_ms)` every turn, fed from the
  grant's own DRR-computed budget, ticked by the existing 1 ms `EpochTicker`.
- The gap: no `epoch_deadline_callback` is installed, so wasmtime's default epoch behavior applies —
  an unconditional TRAP — which `execute_turn` already catches and maps to `TurnFault::
  DeadlineExceeded` (pre-existing). The old `pump_primed` then turned ANY `TurnFault`, including this
  one, into `ShardOutcome::Fault`, which the kernel's failure-escalation path would treat as a real
  actor failure — even though a wasm-bytecode-boundary epoch interrupt leaves the `Store`/
  `GuestInstance` fully usable and nothing was actually lost.
- Fix, in `Self::execute_turn_for`: a new match arm, `Err(TurnFault::DeadlineExceeded) =>
  ShardOutcome::Turn{ result: TurnResult{ status: TurnStatus::MoreWork, .. }, .. }`, placed BEFORE
  the catch-all `Err(fault) => ShardOutcome::Fault`. Every other `TurnFault` variant (`Trapped`,
  `FuelExhausted`, `Exhausted`, `Host`) is untouched — still a real fault. The actor's `GuestInstance`
  is never unregistered by this path, so it stays registered and re-grantable on the next tick.
- Test-double support: `MockGuestRuntime` (`🦀️component.rs`) gained `ScriptedOutcome::
  DeadlineExceeded` + `script_deadline_exceeded(actor)`, additive; both `execute_turn` and `step_job`
  match arms updated so neither becomes non-exhaustive.

## 4. Wire change deliberately AVOIDED — lease-request open

The brief asked for `lane: Lane` directly on `ShardFrame::Grant`. I did not add it: two LIVE call
sites outside this packet's `path_scope` construct `ShardFrame::Grant` with a full struct literal
and would have failed to compile with a new required field —

- `🧰️framework/🛍️products/💻️os/🖥️host/🎠️activation.rs:196` (`NativeKernelRuntime::
  tick_and_dispatch` — the real dispatch loop the native bench's own `budget_4_and_5` path uses)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/
  🧊️wgpu/🎠️runtime.rs:211` (the wgpu-native host's equivalent dispatch loop)

Both import `ShardFrame` from `semio_framework_plugin_host::shard` (confirmed via `grep`/`use`
lines) and neither is under `🔌️plugin/🖥️host/**`. Since every `Envelope` a `Grant` carries already
has its own `.lane` (set once per actor by the kernel's DRR `Scheduler`), the classification
information a `Grant`-level field would have carried is already on the wire — see `Self::
actor_lane`'s doc comment in `🧵️shard/🦀️component.rs`. **`lease-request` open against those two
files** in case a future packet still wants the field for symmetry/robustness (e.g. so a
zero-envelope `Grant` also carries a lane) — not required for correctness of piece 1/2 as built.

## 5. `presence` field (M2/`sdk-wire`) — three sites, decided per site

`kernel::TurnResult` gained `presence: Vec<ui_contract::PresenceUpdate>` from a sibling packet
mid-session, breaking 3+2 construction sites in my crate (`cargo check --lib` initially showed 3;
`--all-targets`/`test` surfaced 2 more `#[cfg(test)]` sites the same command hides — rule 26).

**Two sites are the real turn path, and I decided them, not defaulted them blind:**

- `🦀️component.rs`'s `WasmtimeRuntime::execute_turn` (~line 1219) and `⏳️runtime.rs`'s
  `convert_poll_success` (~line 265): both build a kernel `TurnResult` from `wit_turn_result`, whose
  WIT type (`component.wit`'s `reactor.turn-result`) DOES carry a `presence: list<presence-update>`
  field the guest can populate — so this is real, potentially-non-empty guest data, not an
  obviously-absent one. I read the actual shapes on both sides before deciding:
  - WIT `presence-update { peer: pack }` wraps a pack-encoded `📡️replication/📡️wire::PresencePeer`
    — the **collaboration-roster** record (actor id, `connected_at_ms`, drag-ghost, interaction,
    per-window views, …).
  - `kernel::TurnResult.presence: Vec<ui_contract::PresenceUpdate>` wants the **render-plane**,
    `(surface, node_key)`-addressed hover/selection channel (`surface`, `node_key`, `own:
    OwnPresence`, `peers: Vec<PeerMark>`, `ttl_ms`) — a structurally different record. That field's
    own doc comment (`🎠️kernel/🦀️component.rs:918-923`) explicitly calls these TWO DIFFERENT
    channels, distinct from "the roster's own replication channel".
  - No `PresencePeer → PresenceUpdate` conversion exists anywhere in this repo — I grepped
    `kernel_turn_result_to_wit`, the function name that kernel doc comment points at as the
    WIT-repoint pair, and it is referenced in exactly that one doc comment, nowhere else. It has not
    been built (it belongs on the guest/SDK side, forward direction, not the host).
  - Decision: `presence: Vec::new()` at both sites, WITH a comment explaining this is a real,
    load-bearing wire-shape mismatch between the `wit-flip` packet's WIT declaration and the M2
    kernel design — not a "nothing to carry" default. Inventing a mapping here would be guessing at
    an unspecified schema; that is worse than an honest, documented gap. Flagged for a
    coordinator-level decision, not swept.
- `🦀️component.rs`'s `MockGuestRuntime::idle_turn()` (test helper) and 4 more `#[cfg(test)]`
  `TurnResult` literals in `🧵️shard/🦀️component.rs` (my own new `execute_turn_for` piece-2 arm,
  `RecordingRuntime::execute_turn`, two `to_actor_turn_result` test fixtures): all `presence: Vec::
  new()`/`vec![]` — no ambiguity, either synthetic test data or (my own piece-2 arm) a turn that
  never actually ran, so there is genuinely nothing to carry.

## 6. `⚡️effects/🦀️component.rs`'s `resolve_ready(wheel.disarm(timer_id))` — removed

At sol's request, after sol's own R9-revert of `TimerWheel::arm`/`disarm`/`armed_count` in
`🛎️services` (unowned this wave, sol fixed it directly) removed the `MutexGuard`-across-await shape
that `resolve_ready` existed to bridge. Replaced `resolve_ready(wheel.disarm(timer_id))` with a plain
`wheel.disarm(timer_id).await` — compiles clean (verified, no E0277). The now-unused `resolve_ready`
helper function (the file's only call site) was deleted entirely — one fewer E5 exception in this
crate. Repo-wide grep confirmed this was the ONLY `resolve_ready(` call site in `🔌️plugin/🖥️host/**`
(sol's "4 sites" was 4 *lines* — the def, its `unreachable!`, one doc comment, one call — not 4 call
sites).

## 7. New shard-level tests

`🧵️shard/🦀️component.rs`, `//#region 🔖️LanePriorityAndEpochYield` (end of `mod tests`):

- `an_interactive_grant_is_executed_before_background_grants_queued_the_same_pump` — 5
  Background-lane actors' grants queued on the wire BEFORE 1 Interactive-lane actor's grant (worst
  case for the old FIFO/HashMap-order pump), all landing in ONE `pump()` call. Asserts the
  interactive actor's `ShardOutcome::Turn` is the FIRST of the 6 outcomes sent, with the actual
  scripted `fuel_used` value checked (not just the actor id) to rule out a coincidental match.
- `a_turn_that_hits_its_epoch_deadline_yields_more_work_not_a_fault_and_stays_registered` — scripts
  `TurnFault::DeadlineExceeded`, asserts the outcome is `ShardOutcome::Turn{status: MoreWork}` (never
  `Fault`) and the actor is still `is_registered` afterward.

Both pass. Both exercise the mechanism directly, no mocked-away scheduler.

## 8. Test/check results — all commands run FOREGROUND, exit codes pasted

```
$ CARGO_TARGET_DIR=.../scratchpad/target-shard cargo check -p semio-framework-plugin-host --lib --message-format=short
EXIT: 0

$ CARGO_TARGET_DIR=.../scratchpad/target-shard cargo check -p semio-framework-plugin-host --all-targets --message-format=short
EXIT: 0   (0 errors both before AND after the resolve_ready removal, re-verified)

$ CARGO_TARGET_DIR=.../scratchpad/target-shard cargo test -p semio-framework-plugin-host --lib
test result: ok. 127 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out
```
Floor was **125 passed / 0 failed / 1 ignored** — now **127** (+2, my two new tests by name above; no
other test moved, confirmed by diffing the pass list against the run before my test additions).

```
$ CARGO_TARGET_DIR=.../scratchpad/target-shard cargo test -p semio-framework-actor
test result: ok. 70 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```
**70/0**, unchanged floor. Both named regression tests present and passing:
`component::tests::quick::interactive_actor_avoids_a_shard_saturated_by_cpu_bound_actors` and
`component::tests::quick::mailbox_backpressure_drops_lower_priority_lane_to_admit_interactive`. Read
both bodies to confirm neither is vacuous: the first activates 6 real background actors across 3
real shards, drives them past `is_saturating`'s threshold via real `kernel.complete` calls, then
asserts the interactive actor's ACTUAL pinned shard (`kernel.actor_record`) is the one clean shard —
not an assumption. The second builds a real 2-capacity `Mailbox`, enqueues 3 real envelopes, and
asserts the concrete `Backpressure::Dropped{lane: Maintenance}` return plus `mailbox.len() == 2` —
both genuinely observe scheduler/mailbox state, not an empty scheduler.

## 9. Dropped-future census (R12/R13/R17)

Forced rebuild per R12 (`cargo clean -p semio-framework-plugin-host && cargo check -p
semio-framework-plugin-host --lib --message-format=short`): **0** occurrences of `unused implementer
of` (verified the phrase itself, not the short `Future` form R12 warns is silently wrong). Re-ran
`--lib`/`--all-targets`/`--lib` tests after the clean rebuild — all still EXIT 0 / 127 passed.

Static `grep -rn "let _ = "` over the whole crate: all pre-existing sites (most already carry an
explicit R13/R14-corollary tag or are genuinely sync `JoinHandle::join`/`recv_timeout`/`child.kill()`
calls); my own diff introduces ZERO new `let _ = ` sites.

## 10. Bench (`budget_4_and_5`) — NOT RUN, honest reason

Attempted the real native bench (reusing the committed `🧫️fixtures/🔌️scale/🤖️generated/
🔣️registry.json`, 2550 records, 354 `cpu`-profile actors — well past the 40 needed):

```
$ CARGO_TARGET_DIR=.../scratchpad/target-shard-bench cargo build -p semio-framework-os-scale-fixture --target wasm32-wasip2 --features component-guest
error[E0432]: unresolved import `semio::framework::ui::PatchReplace`
error[E0560]: struct `ui::UiPatch` has no field named `kind`
error[E0599]: no variant, associated function, or constant named `Replace` found for enum `PatchOp`
error[E0063]: missing field `presence` in initializer of `TurnResult`
error: could not compile `semio-framework-os-scale-fixture` (lib) due to 4 previous errors
```

`semio-framework-os-scale-fixture` (`🧰️framework/🛍️products/💻️os/🧫️fixtures/🔌️scale/🦀️component.rs`)
is OUTSIDE this packet's `path_scope` (not `🔌️plugin/🖥️host/**`) and is independently broken: stale
`PatchOp`/`PatchReplace`/`UiPatch.kind` naming (a WIT patch-op shape that has since changed
elsewhere, this fixture never updated) PLUS the same `presence` field gap as §5, but in a crate I
have no mandate to fix. This means `budget_4_and_5` cannot be built, let alone run, from my scope —
**reported honestly as NOT RUN rather than guessed**, per this ticket's own rule 11 and the packet
brief's explicit "say so plainly" instruction. This is a real, separate finding worth flagging to a
crate owner: the native bench pipeline (`bun ./📜️script.ts bench plugins --renderer native`) is
currently broken independent of this packet's changes.

**p95 numbers: before / after piece 1 / after piece 2 — none measured.** The mechanism is proven at
the shard level (§7) and the crate-level regression floors are green (§8), but the headline
runtime-measured p95 claim could not be produced from this packet's scope. If restoring
`semio-framework-os-scale-fixture` is itself in scope for a future packet, re-running
`budget_4_and_5` (three times: HEAD before this packet's changes, after piece 1 alone, after piece
1+2) is the very next step and should be cheap once that crate compiles — the wgpu-native dispatch
path (`🎠️runtime.rs`) already routes through `ShardLoop::pump`, so no additional wiring is needed to
observe piece 1/2's effect once the fixture builds.

## Files touched (all within `🔌️plugin/🖥️host/**`)

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs` — pieces 1+2,
  new tests, `presence: Vec::new()` at 5 sites (1 real synthetic-turn site + 4 test fixtures).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs` — `MockGuestRuntime`
  `DeadlineExceeded` scripting support, `presence: Vec::new()` at 2 sites (1 real turn path +
  1 test helper).
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⏳️runtime.rs` — `presence: Vec::new()`
  at 1 real turn-path site.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs` — removed the
  `resolve_ready` E5 bridge for `TimerWheel::disarm`, now a plain `.await`.

## Open lease-requests (not applied, awaiting coordinator decision)

1. `ShardFrame::Grant.lane` field, for symmetry/robustness — see §4. Files: `💻️os/🖥️host/
   🎠️activation.rs:196`, `📺️renderer/…/🎯️targets/🧊️wgpu/🎠️runtime.rs:211`.
2. `PresencePeer → ui_contract::PresenceUpdate` conversion (or a ruling that render-plane presence
   genuinely does not flow through `poll`'s `turn-result.presence` at all, and the WIT field should
   be repurposed/removed) — see §5. Affects `🦀️component.rs`'s `execute_turn` and `⏳️runtime.rs`'s
   `convert_poll_success`.
3. `semio-framework-os-scale-fixture` is broken (stale `PatchOp` shape + missing `presence` field) —
   see §10. Blocks `budget_4_and_5` for anyone, not just this packet.
