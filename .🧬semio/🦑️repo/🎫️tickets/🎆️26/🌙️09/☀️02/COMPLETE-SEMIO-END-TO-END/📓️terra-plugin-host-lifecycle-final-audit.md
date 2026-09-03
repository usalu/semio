# Plugin-Host Lifecycle Final Audit

## Verdict

**HOLD — current source is provisionally acceptable; no independent Rust gate or full-suite terminal has been observed.**

Sol reports a current-source focused binary result of twelve Rust laws and a five-trace Bun/AJV
oracle. This audit does not relabel that as an independent runtime result. The replay ownership,
first-reason close funnel, actor retirement, retained-test migration, priority barrier, and
registry reaper source are materially stronger than the predecessor. Two exact acceptance defects
remain in the registered lifecycle boundary:

1. The neutral corpus is not bound to a production Rust subject and its Rust/Gherkin adapter is
   not run by the registered gate.
2. A normal wake-capable detached session nevertheless receives an unconditional one-millisecond
   reaper timer after every `Blocked` close opportunity, so it polls indefinitely while already
   registered for the real wake.

No full-suite exit, all-feature terminal, generated-launch freshness, or cleanup result is claimed
here. A later green full suite cannot close either source defect without the corresponding
production trace binding and no-spin law.

## Evidence boundary

Read-only re-read on 2026-09-03 of the current shared source:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🔣️relay-lifecycle.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/relay-lifecycle/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧪️tests/relay-lifecycle/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/📜️script.ts`

I did not run Cargo or Bun in this pass: the native SocketGrant lane is actively compiling and
this report must not claim the implementer’s output as independent evidence.

## Source-closed portions

### Replay drop accounting and the close funnel

`FixedReplaySeedPage` owns optional storage and decrements its page counter on the one normal
drop path (`🧵️shard/🦀️.rs:408-435`). `MountedReplaySeed` now uses ordinary `Option` owners and
its `Drop` returns only still-reserved ABI bytes (`:584-694`), eliminating the old
`ManuallyDrop`/debug-abort destruction policy. The focused law deliberately drops representative
capture, retained, and closing frontiers under `catch_unwind` and checks the process page/ABI
baselines (`:3716-3744`). This is source-closed, subject to an independently observed run.

`begin_replay_seed_close_owned` stores a reason only if absent, moves the selected seed to
`Closing`, and removes all four live routing-map entries (`:1075-1108`). `fail_replay_seed` enters
that funnel before surfacing the fault (`:1110-1113`); `unregister` marks every matching seed
`ActorLost`, suppresses matching refusal publication, then drops the guest instance (`:1121-1129`,
`:1471-1483`). The direct law proves the first reason wins, actor-owned refusal does not publish,
and counters return to baseline after bounded drain (`:3746-3796`). This satisfies the requested
first-reason/unregister source shape.

### Migration and placement scope

The six stale synchronous shapes are visibly transitioned through `retain_replay_seed` before
guest step/cancel/placement assertions: multi-step spawn (`:2568-2667`), pre-retained effect
cancel (`:2670-2707`), retained effect-cancel failure (`:2710-2755`), payload cancel success
(`:2832-2886`), payload-cancel failure (`:2888-2927`), and exclusive placement (`:2929-3000`).
The source preserves both semantic boundaries: a same-turn pre-retained cancellation is local with
zero guest cancellation/step admissions, while a retained cancellation failure retires the actor.

`FixedOwnerRing` returns generation-qualified owner keys and validates a key against the physical
slot generation (`:720-830`). `pop_next_authority` scans only the leading consecutive
`JobStep` frontier for the actor and returns to FIFO at a non-step barrier (`:958-978`). The law
proves an exclusive step may move ahead of a preceding inline step but cannot cross `Cancel`, and
that the unselected physical generation key remains valid (`:3002-3023`). This is source-closed.

### Reaper ownership and isolation, excluding no-spin

The slot lifecycle distinguishes `Running`, `DrainingForCaller`, and `DetachedForReap`
(`🖥️host/🦀️.rs:3738-3754`). Future drop validates slot generation, begins close, transfers only
to `DetachedForReap`, increments an epoch, and starts a maintenance-lane reaper
(`:3865-3900`, `:4184-4189`). The reaper chooses one detached slot from a rotating cursor and
clears it only after `pump_close` reports complete (`:3903-3923`). Stale generation detach is a
no-op, while the live caller path explicitly rejects detached ownership and retains
`DrainingForCaller` output until its own close completes (`:4030-4058`).

The current direct laws cover one-slot round-robin reclamation plus stale generation
(`:4487-4516`), and a detached slot not stealing a live caller’s exact output (`:4519-4535`).
The process-shaped pending-drop law reaches a detached slot and waits for reaper cleanup without a
second caller `pump` (`:4728-4769`). Those are meaningful source laws, but they do not establish
the separate no-spin constraint below.

## RED-1 — Literal neutral corpus is not a production subject or a registered Rust/Gherkin run

The ticket’s lifecycle requirement is a literal neutral fixture consumed by the production
shard/relay transition API, a Rust/Gherkin subject, and an independent TypeScript oracle. Current
source only provides the final two as independent hand-written interpreters:

- The Rust `subject` reconstructs `state`, `first_reason`, reservation and release counters from
  fixture strings (`🧪️tests/relay-lifecycle/🦀️.rs:45-106`). It contains no
  `ShardLoop`, `MountedReplaySeed`, `GuestRelayMountedRegistry`, or production transition call.
- The TypeScript program similarly parses and interprets those strings, validating the schema via
  AJV (`🧪️tests/relay-lifecycle/🟦️.ts:13-84`). That is a useful independent oracle, not the
  native subject.
- The registered `lifecycle-check` runs only that TypeScript file, twelve direct `--exact` Rust
  library filters, then full `--lib` and all-feature check (`📜️script.ts:18-38`). It does not
  invoke the Rust adapter or the Gherkin feature. The production host unit tests read individual
  fixture fields such as capacity/caller-output/cancel count, not an entire trace driven through
  the registry (`🖥️host/🦀️.rs:4487-4535`, `:4728-4769`).

This is not a claim that the focused production laws are fake. It is a narrower, real acceptance
gap: a fixture edit can keep both model interpreters green while no literal trace verifies the
real host transition boundaries, and the advertised Rust/Gherkin half is not even part of the
registered command.

Required closure: make the Rust subject construct and advance the real production test APIs for
each literal trace (or expose a small test-only transition seam), register its Gherkin run in
`lifecycle-check`, and require an output equivalence with the fixture result. Keep the TS/AJV
interpreter independent; do not derive its expected result from the Rust subject.

## RED-2 — Wake-capable blocked sessions still poll every millisecond

`pump_close` handles a `WorkerJobSession::Blocked` by registering the reaper waker
(`🖥️host/🦀️.rs:3961-3971`). But `GuestRelayMountedReaper::poll` discards that distinction:
every `GuestRelayMountedReap::Blocked`, including that successful registration, schedules the
one-millisecond maintenance timer (`:4143-4153`). Its callback clears the coalescing flag and
wakes the future (`:3925-3938`); the next poll sees the same blocked session and schedules the
next one. A permanently blocked but wake-registered session consequently emits one maintenance
turn per millisecond until the external completion happens.

The timer is appropriate only for a close owner that cannot retain/register a wake. The current
result enum cannot express that difference, and no focused law counts reaper polls/timers before
the actual gate wake. The process-shaped law releases the gate and waits using externally submitted
timer barriers (`:4728-4769`), so it demonstrates eventual cleanup but not absence of spin.

Required closure: have `pump_close` report wake-registered versus timer-fallback blocked state.
For `WorkerJobSession`, retain the registered reaper waker and return pending without a timer; for
a wake-incapable rejection, retain one coalesced delayed maintenance check. Add a deterministic
blocked-before-wake law that advances finite maintenance opportunities and proves zero timer
rechecks/zero duplicate cancellation before the actual wake, then proves exactly one bounded
post-wake retirement.

## Required independent evidence after source repair

Run an uncached registered command in a unique retained ticket target after the native Cargo lane
is free:

```sh
RUSTFLAGS='-Awarnings' CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/plugin-host-lifecycle-terra-target" \
  bun nx run @semio-tech/framework-plugin-host:lifecycle-check --skip-nx-cache
```

Acceptance needs its terminal output to show the schema/oracle, all registered focused laws, the
full plugin-host library suite, and `--all-features` check. It remains conditional on both RED
items being resolved in the current source; compilation or a focused pass alone is not a runtime
acceptance result.

## Pre-repair broad-gate record

Session `78722` is **not** acceptance evidence and must not be conflated with a
later repaired run. Its registered broad lifecycle invocation ended with exit
`142` at the exact 900-second alarm after discovering 240 tests. An earlier,
unrelated effects-capability failure preceded a hang/timeout in
`background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route`;
no later lifecycle outcome was reached. The implementer is repairing the
no-spin/waker and production-fixture seams after that terminal. This report
preserves `78722` as pre-repair RED/incomplete evidence only; a fresh terminal
after those source changes is required for any final verdict.

## Live repair re-read — no-spin code is source-closed, proof and gate remain RED

Current `pump_close` returns `Blocked { wake_registered }`; `reap_one` retains
that bit; and the reaper schedules a one-millisecond fallback only when no wake
was registered (`🖥️host/🦀️.rs:3918-3935,3983-4002,4149-4176`). That directly
supersedes RED-2's source claim that a wake-capable blocked close always gets a
timer. The caller draining path follows the same distinction
(`:4058-4077`).

It does **not** yet close the required no-spin evidence. Test-only
`reaper_polls` and `reaper_timers` counters exist (`:3816-3842,3940-3956,
4162-4165`), but `dropping_a_pending_mounted_future…` drops the future and
immediately releases the gate (`:4750-4776`). It never leaves the registered
wake blocked across deterministic maintenance opportunities nor asserts zero
timer rechecks/one wake-driven retirement. Add that exact pre-release phase and
assert the counters before and after release.

The production-fixture subject remains a model interpreter
(`🧪️tests/relay-lifecycle/🦀️.rs:45-107`); selected host tests read fixture
fields rather than consuming each literal trace through production transitions
(`🖥️host/🦀️.rs:4384-4753`). RED-1 consequently remains live. Moreover the
registered runner currently imports a non-existent duplicate
`…/📚️library/📦️📦️packages/…` path and invokes a non-existent
`../../🧪️🧪️🏔️🦋️tests/♻️relay-lifecycle/🟦️.ts`
(`📦️packages/🦀️rust/📜️script.ts:3,18`); the actual adapters are under
`🧪️tests/relay-lifecycle/`. The gate must be path-repaired before any claimed
run could be evidence.

## Live RED-1 seam re-read — Most literal routing is production-backed; terminal output is still injected

The former handwritten Rust trace interpreter is superseded in the current
adapter: its SUT calls `exercise_relay_lifecycle_trace` and compares that
returned production projection to the neutral expected object
(`🧪️🧪️🏔️🦋️tests/♻️relay-lifecycle/🦀️.rs:26-55`). The replay trace constructs
an actual `MountedReplaySeed` and sends its close events through the real
`ShardLoop::begin_replay_seed_close_owned`/`close_replay_seed_one` helpers
(`🧵️shard/🦀️.rs:2167-2244`). The abandoned trace calls the real mounted
registry's `detach`/reaper and uses a controllable actual `WorkerJobSession`
probe; its release counter increments only from that probe's actual
`close_step`, not when the fixture grants a permit
(`🖥️host/🦀️.rs:3740-3825,4410-4464`). The stale-generation and capacity cases
also exercise the registry's production `detach` and fixed `reserve` rules.

**RED remains for the literal live terminal/output event.** The
`relay-live-terminal-caller-output` branch mounts `GuestRelayMountedOwner::Empty`
and directly writes the terminal bytes, terminal enum and
`DrainingForCaller` lifecycle into the slot
(`🖥️host/🦀️.rs:4467-4484`). It then calls `pump` merely to collect those
already-injected bytes (`:4485-4500`). That bypasses the production
`WorkerJobSession` step/terminal checkout and `finish_outcome` path, so it can
remain green if the real terminal-to-caller-drain transition loses, corrupts or
misattributes the output. Replace that direct mutation with a controllable
real `WorkerJobSession<GuestColdRelayJob>` (or a production-equivalent
test-only worker session) whose terminal is observed through the same
`try_step_on_caller` → `take_terminal`/outcome → `finish_outcome` sequence
used by `PluginInstanceHandle::run_job_on_worker` (`:4579-4628`; registry
transition `:4040-4135`).

The same literal trace's `reap-other` event is also not a competing-owner
exercise: its one and only slot is the caller slot, so `reap_one` is required
to return `Idle` (`:4473-4488`). It never mounts a detached second session for
the rotating reaper to select, close and reclaim. That is weaker than the
fixture's named event and cannot establish live-output isolation under actual
competing reaping. Mount a separately detached controllable probe in the trace,
require reaper progress on that slot, then require the live slot remain
`DrainingForCaller` and return its exact output.

Permanent Rust/Gherkin lifecycle-check registration is still pending, and no
independent terminal is claimed here. This amendment supersedes only the
"all-Rust-interpreter" portion of RED-1, not the live-terminal gap or the
acceptance hold.

## Live seam repair re-read — terminal/caller and competing-reaper REDs are source-closed

The two narrow RED-1 findings immediately above are **superseded in the next
stable source read**. The terminal trace no longer writes a slot's output,
terminal enum, or lifecycle directly. It mounts a controllable real
`WorkerJobSession<GuestRelayLifecycleProbeJob>`; that probe constructs a
`CommitOutput` retained payload and returns `StepOutcome::Complete`
(`🖥️host/🦀️.rs:3770-3787,4433-4464,4524-4550`). The trace then advances the
actual registry `pump` until its production caller-drain state and finally
obtains the bytes through that same `pump` (`:4574-4589`). Thus the literal
`terminal:exact-output` flows through the mounted-session terminal checkout
and caller-drain machinery, rather than through an injected slot value.

`reap-other` now also reserves, mounts and detaches a genuinely competing
controllable worker session, waits for the real reaper to park/reclaim it, and
only then asserts that the original caller-owned slot is still
`DrainingForCaller` (`:4551-4573`). This source law therefore reaches an
actual competing reaper opportunity while preserving the live caller's exact
output. The earlier description of a single-slot `Idle` reaper is historical.

The no-spin proof has also materially landed. The process-shaped pending-drop
law now observes exactly one retained completion waker and zero fallback
timers, advances eight deterministic maintenance opportunities before release
and asserts zero reaper polls/timers, then releases the actual gate and
requires a bounded wake-driven cleanup (`:5195-5276`). The separate
wake-incapable law keeps exactly one coalesced fallback (`:5279-5297`). This
supersedes the prior statement that there was no deterministic pre-release
phase.

`lifecycle-check` resolves the current TypeScript oracle and includes
`neutral_relay_lifecycle_traces_drive_production_machines` among its focused
Rust filters (`📦️packages/🦀️rust/📜️script.ts:20-43`), so the fixture now has a
registered direct production-machine unit path. The separate Rust/Gherkin
adapter/feature is still not invoked by that command; keep that as an
acceptance registration gap until the test-host execution is registered and a
fresh independent gate reaches a terminal. No run is claimed by this audit.

## Live registration re-read — Rust/Gherkin subject is now source-closed

The immediately preceding Rust/Gherkin registration hold is **superseded in
the current script**. After the TypeScript/AJV oracle, `lifecycle-check` now
calls the repository test host's separate subject phase with the exact
fundamental case and Rust implementation:
`subject fundamental --case ♻️relay-lifecycle --implementation rust`
(`📦️packages/🦀️rust/📜️script.ts:20-29`). This does not use the globally-red
contract phase.

That is a non-vacuous feature execution rather than adapter discovery. The
repository subject runner selects the exact discovered case/implementation,
materializes a Rust host with the plugin-host crate enabled as `sut`, and runs
the committed adapter through `semio_repo_test_host::run_main`
(`🦑️repo/🔨️modules/🧪️test/📜️script.ts:219-240,390-479,655-745,848-852`). The
feature expands five `@level-fundamental` trace ids
(`🧪️🧪️🏔️🦋️tests/♻️relay-lifecycle/🥒️.feature:1-18`); the Rust subject parses
each input id, invokes `exercise_relay_lifecycle_trace`, compares its complete
projection to the fixture and checks balanced accounting
(`🧪️🧪️🏔️🦋️tests/♻️relay-lifecycle/🦀️.rs:26-55`). The prior registration gap
is therefore source-closed. A terminal independently run gate is still needed
before an acceptance verdict.

## Current gate repair re-read — source-qualified acceptance hold

The latest registered boundary repairs the prior gate regression. `lifecycle-check` resolves the
physical `🧪️tests/♻️relay-lifecycle/🟦️.ts` oracle, verifies both that path and the repository
test host exist, runs the TS/AJV oracle, and then runs the exact Rust fundamental subject
(`📦️packages/🦀️rust/📜️script.ts:20-31`). The target remains registered as
`@semio-tech/framework-plugin-host:lifecycle-check` in its physical project file and launch entry
`⚖️gate🖥️host♻️lifecycle` (`📋️project.json:7-11`, `.vscode/launch.json:3224-3229`).

All fourteen focused Rust suffixes are no longer loose Cargo filters. For each one, the script
obtains the test list, keeps only `: test` entries whose fully-qualified name ends in that suffix,
requires exactly one match, logs those names, and only then executes each exact FQN using
`--exact` (`📜️script.ts:32-61`). It follows those checks with the full library suite and the
all-features check. This supersedes the prior nonexistent-oracle-path and potentially-vacuous
focused-filter REDs at source level.

The literal trace seam is production-derived rather than a replacement state machine. The replay
case creates `MountedReplaySeed`, uses `ShardLoop::begin_replay_seed_close_owned` and
`close_replay_seed_one`, and verifies process seed/ABI counters return to their observed baseline
(`🧵️shard/🦀️.rs:2170-2243`). Relay cases use `GuestRelayMountedRegistry` plus a real
`WorkerJobSession` control job: detach schedules the actual reaper, terminal output enters a real
`StepOutcome::Complete`, and the registry's `pump` drives terminal checkout, caller drain, and
output ownership (`🖥️host/🦀️.rs:3770-3837,3987-4150,4527-4598`). The competing-reaper trace
mounts/detaches another session and requires the live caller's session to remain draining
(`:4554-4576`). The dedicated source laws also cover stale generation, one opportunity per reaper
turn, no wake-less spin before release, and exactly one coalesced fallback (`:4981-5009,
:5221-5300`).

The independent standalone TypeScript/AJV oracle was rerun read-only on 2026-09-03 and exited
`0`, printing `relay-lifecycle oracle: 5/5`. It validates the schema and independently interprets
the five fixture traces (`🧪️tests/♻️relay-lifecycle/🟦️.ts:13-87`). This is evidence only for the
neutral oracle. I did **not** run Cargo, the Rust subject, the fourteen selected laws, the full
library suite, or all-features check because the shared Cargo lane remains occupied; the current
verdict stays HOLD until an independently owned uncached registered gate has a terminal.

One wording precision remains: the Rust adapter's generic accounting comparison currently checks
the fixture's before/after fields against themselves (`🧪️tests/♻️relay-lifecycle/🦀️.rs:38-40`),
not a relay-side measured counter. That is not a replacement lifecycle interpreter and the replay
production seam separately measures the actual global counters. It should not be advertised as a
runtime accounting measurement for every relay trace; the direct replay/drop laws carry that
measurement obligation.

## Registered subject runtime — current RED: completion wake does not re-park the detached reaper

Session `10674` is historical registration RED: all five expanded outline
examples errored because the Rust adapter registered only the unexpanded
`production-traces` subject. Current source repairs that narrowly: the five
hard-coded ids exactly equal both the feature's Examples table and the neutral
fixture ids, and the adapter registers only their corresponding expanded
`production-traces-{id}` names
(`🧪️tests/♻️relay-lifecycle/🥒️.feature:18-24`,
`🧫️fixtures/🔣️relay-lifecycle.json:12-48`,
`🧪️tests/♻️relay-lifecycle/🦀️.rs:6-19,53-60`). `dispatched(&id, 1)` remains
only a post-subject result marker; the subject first calls the production
`exercise_relay_lifecycle_trace` seam and compares its complete projection
(`:38-50`).

The fresh registered session `59599` reached those five concrete subjects and
is therefore non-vacuous, but it is RED: four passed and
`relay-abandoned-blocked-wake` failed after 11.46 seconds with
`relay lifecycle reaper did not park for its first release`. The fixture is
not the mismatch. It requires a detached owner to register a close waker,
receive `wake` without a release permit, register a fresh close waker, then
consume two releases (`🧫️fixtures/🔣️relay-lifecycle.json:21-27`). That is the
real production path: `detach` begins close and schedules the real registry
reaper (`🖥️host/🦀️.rs:3987-4015`); its reaper drives the real
`WorkerJobSession` close and asks it to register the job-owned close waker
(`:4035-4052,4107-4145`). The probe blocks until **both** `awake` and a release
permit are true, storing the reaper waker otherwise (`:3796-3833`). After the
fixture's lone `wake`, `remaining == 2` and the permit count is zero, so that
second registration is required. The observed timeout shows the actual
reaper did not re-enter/re-park after its completion wake; it is not an
oracle-only or feature-name failure. Lifecycle acceptance is therefore RED
pending a production wake/liveness repair and a fresh terminal rerun.

### Root-cause correction — source-closed; rerun pending

The preceding attribution to a standalone reaper liveness loss is superseded
by the exact worker failure from session `91459`. The probe's zero-permit
closure previously used `then_some(permits - 1)`: its argument was evaluated
eagerly, so a zero permit panicked before the intended blocked result,
stranding the real `WorkerJobSession` in transition. Current source changes
only that arithmetic to `permits.checked_sub(1)`
(`🖥️host/🦀️.rs:3803-3814`). `None` leaves the atomic permit value unchanged and
reaches the existing `Blocked` branch, so the subsequent reaper poll can
register its next exact close waker. The fixture still exercises the same
production registry/reaper/session sequence; it was not relaxed. Session
`8920` is the fresh narrow-law rerun, but has not been claimed by this audit
until its terminal result is available.

### Current full-gate RED — close-waker result is inverted

The later full registered session `70478` is a separate runtime RED: the
neutral oracle was `5/5`, the repository subject was `5/5`, and exact-one
selection found fourteen Rust FQNs; the first eleven focused laws progressed,
then `dropping_a_pending_mounted_future_reaps_without_a_second_foreground_poll`
failed at `🖥️host/🦀️.rs:5253` with `reaper_wake_waits == 0` rather than `1`.

Current source explains that observation exactly. `GuestColdRelayJob` has a
pending `GuestRelayCompletionSlot`, invokes its `register_wake(waker)`, then
returns `false` (`🖥️host/🦀️.rs:3488-3494`). That boolean is propagated by
`WorkerJobSession::register_close_wake` (`🧵️job/🦀️.rs:2900-2915`) to the
mounted registry (`🖥️host/🦀️.rs:4108-4116`). The registry consequently treats
the already-registered completion wake as unavailable and schedules a timer
instead of recording the sole wake-backed park (`:4075-4081`). This is a
production liveness/accounting defect, not a fixture race: the real completion
slot owns the waker and its sender wakes it at `:2985-2993`. The pending-slot
branch must report successful registration. No acceptance can be inferred from
the preceding focused progress.

### Correction to the preceding current-full-gate attribution

The preceding source attribution is incorrect and is retained only as audit
history. It conflated `GuestColdRelayJob`'s inherent `register_wake` helper
(`🖥️host/🦀️.rs:3488-3494`) with the `InteractiveJob::register_close_wake`
trait method used by the close path (`:3693-3698`). `pump_close` calls
`WorkerJobSession::register_close_wake`; once the session is in close phase it
dispatches the trait method, which registers the pending completion-slot waker
and returns `true`. Before that phase the session's generic registration also
returns `Ok(true)` (`🧵️job/🦀️.rs:2900-2916`). Thus the inherent helper's
intentional `false` (which suppresses an immediate post-resume repoll) cannot
by itself reach `park_blocked_reaper` or explain `reaper_wake_waits == 0`.
Changing it to `true` would risk a pending-slot spin and is not supported.

Session `70478` remains an honest runtime RED, but its cause is unresolved by
this source reread. The remaining possibilities are that the detached reaper
did not receive/poll its scheduled turn before the test observation, or that
an earlier close-phase transition returned a status other than the pending
`Blocked` branch. The exact reaper scheduling/phase trace, not this boolean,
needs evidence before assigning a production defect.

### Current pending-cancel lifecycle law — source-qualified closure

The follow-up law now makes the cancellation completion itself pending rather
than changing registry, reaper, or session behavior. Its fresh `MockGuestRuntime`
owns one `relay_cancel_release: Mutex<Option<Arc<MockJobStepGate>>>`; the law
installs it before detach, and the real `cancel_job` consumes that one gate and
awaits it (`🖥️host/🦀️.rs:747-785,829-832,972-985`). The detached production
`GuestColdRelayJob` still travels through `run_guest_relay_request`, the real
`WorkerJobSession`, and `GuestRelayMountedRegistry`; it must first park exactly
once on the real completion waker with zero fallback timers and zero wake-less
rechecks, then `cancel_gate.release()` wakes the completion and the registry
reaps without another foreground poll (`:5235-5294`). It also requires exactly
one cancellation admission and restores the actual instance before shutdown.

The gate is neither shared across tests nor left pending: each test constructs
a new `MockGuestRuntime`, `cancel_job` takes the option exactly once, and this
law releases it before waiting for the admission, empty slot, and available
instance. It therefore fixes the old test race (an immediate mock cancellation
could complete before the intended blocked observation) without weakening the
production-machine subject. Coordinator-reported isolated exact rerun passed;
that is external runtime evidence, not a Cargo invocation by this audit. The
historical session `70478` full-gate RED still requires a later full-gate
terminal on these bytes before lifecycle-wide acceptance.

### Session 7067 broad-library residuals — source attribution

Session `7067` completed all fourteen exact lifecycle selectors on the
current narrow fixes, then entered the unfiltered library phase.  That phase
is still RED: it reported failures in
`suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome`,
`unregister_frame_drops_the_instance_exactly_like_the_direct_call`,
`concurrent_route_rejects_retained_start_panic_cleanup_pending_before_recovery`,
`concurrent_route_rejects_step_failure_cleanup_pending_then_quarantine_is_stable`,
and the mounted fixed-replay worker-count law; five further guest
panic/failure-cleanup cases exceeded sixty seconds.  This is an honest
library-suite hold, not evidence that the fourteen exact selectors were
vacuous.

Two reported failures are outside the mounted-reaper machine.  The suspend
law owns a mock instance and loopback transport and calls `ShardLoop::pump`
directly (`🧵️shard/🦀️.rs:2833-2862`); the unregister-frame law has the same
local loopback topology (`:3328-3345`).  Neither constructs a `WorkerPool`, a
`GuestRelayMountedRegistry`, or a `GuestColdRelayJob`.  They must therefore
be diagnosed as independent shard residuals, rather than attributed to the
lifecycle packet.

The mounted fixed-replay law exposes a concrete full-suite hygiene defect:
it constructs separate native `WorkerPool`s for one, two, four, and
host-default workers (`🧵️shard/🧵️executor/🦀️.rs:751-840`) but calls no
`shutdown` for any of them.  `WorkerPool::shutdown` is explicitly not
automatic on drop and is the only operation that joins its worker threads
(`⏳️async/🦀️.rs:1813-1823`); each pool's worker threads retain the shared
inner state.  Thus the test leaks at least its four pools (and all of their
threads) into the rest of the library process.  This is a material
test-fixture/resource leak and a credible cause of later broad-suite
scheduling starvation; it is not a mounted-reaper transition change.

The current permanent gate itself confirms that the broad phase is ordinary
parallel `cargo test --lib`, with no serialisation argument
(`📦️packages/🦀️rust/📜️script.ts:59-61`).  The concurrent and ordinary
cleanup cases use `plugin_host_worker_pool()` (for example
`🖥️host/🦀️.rs:5571-5607,5611-5693`), which is the one process-global native
pool (`:233-236`; `⏳️async/🦀️.rs:2075-2088`), and wait for timer-barrier
callbacks with an unbounded `receiver.await` (`🖥️host/🦀️.rs:5142-5211`).
Their mocks and instance slots are per-test, but worker availability is not.
Accordingly the five hangs are consistent with global-pool/resource
interference and cannot yet establish a production reaper regression.  The
new pending-cancel law is excluded as a leak source: it uses its own
one-worker pool, releases its one-shot cancellation gate, observes Empty and
available instance, then shuts that pool down (`:5235-5294`).

The exact first cause of each currently failing individual law remains
runtime-pending; no broad-suite failure is accepted or waived here.  A
repaired resource-owning replay fixture and isolated exact terminals are
needed before a fresh library terminal can classify the remaining host
cleanup failures.

### Correction — isolated production-path submission-loss RED

The preceding statement that the cleanup hangs could only be global-pool
interference is superseded.  A fresh isolated exact run of
`background_cleanup_cancel_panic_quarantines_before_the_next_mounted_route`
also hung.  Its sampled binary is retained at
`🗑️generated/lifecycle-exact-panic-hang.sample.txt`: the test executor is
parked while the worker pool is idle, which rules out merely waiting behind
another broad-suite test.  The terminal diagnostic was `WorkerPool: mandatory
submission failed closed: Contended` from `GuestRelayPoolFuture::schedule`.

This is a production liveness defect.  `schedule` first sets `scheduled` and
then calls infallible `WorkerPool::submit` (`🖥️host/🦀️.rs:2857-2865`); the
native pool's `submit` panics when its bounded non-blocking `try_submit`
returns contention (`⏳️async/🦀️.rs:1727-1745`).  That panic drops the sole
submitted task while leaving the retained mounted owner awaiting its
completion slot, so the caller has no remaining wake or terminal result.
The current broad residual is therefore a lifecycle production-path RED, not
a waiver for test parallelism.  The separate replay-test pool leak remains a
real fixture defect but not the root cause of this isolated hang.

The proposed retry must preserve one exact retained owner: transient
`Contended` and `Saturated` should keep `scheduled` asserted while one
`callback_at(now + 1)` retry is armed, and only that callback should clear it
before calling `schedule` again.  Clearing it immediately permits every
concurrent wake to arm another retry timer.  `Shutdown` and `Poisoned` are
terminal, non-retryable conditions: they require an explicit fail-closed
completion/cleanup continuation, not dropping the task or reusing the
guest-panic handler with a false cause.  Required runtime laws are a
deterministic actual saturated private pool that drains and proves one
terminal mounted result/no orphan, an injected or supported one-shot
contention and poison path, and shutdown terminalisation of caller and
reaper.  No post-repair run has been credited.

### Submission-loss repair reread — source-qualified, still bounded RED

The current repair replaces the fatal `submit` with `try_submit`
(`🖥️host/🦀️.rs:2874`).  On `Contended` or `Saturated` it keeps `scheduled`
true, records one direct timer-wheel callback for `now + 1`, and that callback
alone clears the flag before re-entering `schedule` (`:2879-2883`).  Thus
concurrent wakes remain coalesced behind the existing retained owner rather
than arming duplicate retry timers.  `Shutdown` before admission and
`Shutdown`/`Poisoned` returned by admission enter `fail` (`:2867-2869`,
`:2885`), which drops the future and invokes its one-shot failure continuation.
The primary cold-job continuation maps those terminal admissions to an exact
fault and instance quarantine (`:3373-3381`).  This closes the isolated
submission-loss mechanism in source, but is not a runtime pass by this audit.

The permanent lifecycle script now exact-selects
`retained_pool_future_retries_saturation_once_and_terminalizes_shutdown`
(`📦️packages/🦀️rust/📜️script.ts:46`).  Its source constructs a one-worker
production `WorkerPool`, occupies its worker, fills the actual bounded lane,
then proves the retained future completes after release; it separately shuts
the pool and proves a `Shutdown` admission failure while the future never
runs (`🖥️host/🦀️.rs:4914-4965`).  This is a non-vacuous source-level law for
the saturated retry and pre-admission shutdown paths.  No direct controlled
`Contended` or `Poisoned` law is present, so those branches remain
runtime-pending.

There remains one production fail-closed gap: the mounted registry's own
reaper failure continuation clears `reaper_active` and retries only a future
panic (`:4069-4085`).  An admission `Shutdown` or `Poisoned` therefore leaves
already `DetachedForReap` owners neither reaped/quarantined nor placed in a
registry-wide rejected state.  Treating shutdown as process-terminal can
explain the shutdown half, but a poisoned queue is represented as a distinct
admission kind and does not itself establish that all later registry use has
ceased.  This must be terminalized at registry scope or justified by a
production-wide poison-is-terminal invariant, with an exact law.  Lifecycle
acceptance remains held pending that closure and a fresh broad-library terminal.

### Reaper fatal-admission repair — source closure, law scope bounded

The preceding registry-failure finding is superseded in current source.
Fatal reaper admission now latches `reaper_failed`, clears the active flag,
and calls `fail_detached` (`🖥️host/🦀️.rs:4090-4119`).  That operation replaces
only `DetachedForReap` slots with `Empty` while holding their individual slot
mutexes, then drops each extracted owner outside the mutex.  A real
`WorkerJobSession` therefore follows its existing ordinary retirement-drop
path (`🧵️job/🦀️.rs:3030-3050`) rather than remaining owned by the registry;
running and caller-draining slots are deliberately not stolen.  Reservations
check the latch before and after obtaining a slot, mounts recheck it and drop
the raced owner back to `Empty`, and a post-latch detach invokes the same
transfer (`🖥️host/🦀️.rs:4006-4047,4050-4085`).  I found no duplicate-reserve or
lock-order defect in that boundary.

The registered saturation/shutdown law now also constructs a registry on the
stopped pool, detaches a slot, and asserts the latch, an empty released slot,
and later reserve rejection (`:4998-5004`).  It is a real source-level reaper
failure check, but its mounted owner is `Empty`; it does not construct a
pending production `WorkerJobSession` or observe its retirement.  Accordingly
the source transfer is sound by the established drop implementation, while a
claim of runtime-proven session retirement still needs that stronger exact
law.  No terminal was run by this audit.

### Reopened RED — fatal reaper transfer directly drops an admission rejection

The preceding source-closure statement is too broad.  `fail_detached` directly
drops each extracted `GuestRelayMountedSession` (`🖥️host/🦀️.rs:4108-4119`).
That is valid for an admitted `WorkerJobSession`, but not for
`GuestRelayMountedOwner::Rejected`.  A rejection is a normal production owner
when `WorkerJobSession::try_new` exhausts admission capacity
(`🖥️host/🦀️.rs:4819-4824`), and the ordinary close path specifically calls
its `begin_close` and bounded `close_step` before replacing it with `Empty`
(`:4205-4222`).  Its own `Drop` debug-asserts that the job, parameters, and
fault source have already been incrementally closed (`🧵️job/🦀️.rs:2617-2621`);
in a release build the unclosed `ManuallyDrop` fields leak instead.

A rejected mounted future dropped before its first foreground poll can become
`DetachedForReap`; a subsequent reaper `Shutdown` or `Poisoned` admission then
takes the direct-drop path.  The surrounding future failure handler catches
the resulting debug panic, so `fail_detached` stops after that slot while the
latch is set and any later detached slots remain stranded.  The new law uses
only `GuestRelayMountedOwner::Empty`, so it cannot expose this path.  This is
a production fail-closed RED: terminal transfer must close rejected owners
through their bounded retirement path (or store an already-terminal owner),
and an exact law must cover a real rejected owner plus a second detached slot.

### Rejected-owner follow-up — fixed panic, remaining unbounded-input RED

Current source avoids the direct-drop panic by attempting at most eight
`pump_close(session, None)` opportunities for a rejected owner, retaining that
slot if it is not terminal and continuing to later slots
(`🖥️host/🦀️.rs:4108-4129`).  This is an improvement: a malformed/private
rejection no longer aborts the whole transfer, and other detached owners are
still processed.  It does not, however, establish the stated transfer
guarantee.  `run_job_on_worker` accepts an unbounded `Vec<u8>` and public
infer/mutation callers forward their byte slices without an input cap
(`:4808-4834,4880-4891`); `GuestColdRelayJob::close_step` releases only one
16-KiB payload page of its unstarted input per opportunity (`:3700-3715`).

For an ordinary pre-authority rejection, terminal close needs *N* input-page
opportunities plus kind clearing, start removal, rejected-job drop, parameter
drop, and the final completion observation: `N + 5`.  An authority-admission
rejection with its retained fault source needs one further step.  Eight hence
covers at most three 16-KiB input pages, not an arbitrary valid input.  On a
larger rejected request the new code intentionally leaves that detached owner
and its request bytes in a permanently failed registry.  This is a source RED
for a poisoned-but-running pool even though it prevents the prior debug panic.
A truthful exact law needs a test-only admission-reservation control (rather
than filling process-global slots), a multi-page rejected request, a fatal
reaper admission, and assertions that every slot is terminal and no request
owner remains; the production fix must drive bounded continuation over time or
otherwise reject/close the input before it can enter a retained rejection.

### Current hard-bound reread — operation-sized coverage, input-cap hold

The fixed eight-opportunity count has been superseded: fatal transfer now
allows `JOB_PAYLOAD_OPERATION_PAGES + 8` opportunities, i.e. 264 at the
current 16-KiB page/256-page operation limit (`🖥️host/🦀️.rs:4115`).  This
covers a full 4-MiB operation-sized start input plus all subsequent
kind/start/rejected-job/parameter/(possible fault-source)/final close stages,
with spare opportunities.  It closes the previous accounting defect for input
that is actually bounded by `JOB_PAYLOAD_OPERATION_BYTES`.

No current-byte bound establishes that premise.  `run_job_on_worker` accepts
`Vec<u8>` without a length check and the public infer and mutation-plan methods
copy arbitrary caller slices into it (`:4808-4834,4880-4891`).  An input above
4 MiB can therefore still exhaust the 264 close opportunities and remain a
retained `Rejected` detached slot under the fatal latch.  The current repair
is source-qualified only for an operation-capped input; all-input acceptance
still requires a pre-construction cap or an explicit terminal ownership policy
for the over-cap case, with a no-global-capacity-race regression law.

### Current cap-and-drain closure — source green, integrated-law gap remains

The preceding uncapped-input RED is superseded.  `run_job_on_worker` now
rejects input larger than `JOB_PAYLOAD_OPERATION_BYTES` before instance
admission or registry reservation (`🖥️host/🦀️.rs:4808-4813`).  The registered
law proves an over-cap infer makes no mock start admission and independently
drains a maximum 4-MiB pre-start job in `JOB_PAYLOAD_OPERATION_PAGES + 3`
opportunities (`:5021-5049`).  At the rejected wrapper, possible parameter,
process-ledger fault-source, and final-observation stages fit within the
registry's `PAGES + 8` hard bound (`:4116-4126`; `🧵️job/🦀️.rs:2558-2615`),
leaving two spare steps.  I find no remaining source leak or direct-drop panic
for the capped transition.

The exact law remains compositionally strong rather than fully integrated: it
does not cause a real `WorkerJobSessionAdmissionRejected<GuestColdRelayJob>`
to traverse fatal `fail_detached`; it verifies the input gate and underlying
maximum job drain separately.  A clean test-only forced-rejection control at
the job admission reservation point would avoid exhausting process-global
retirement slots and could prove the mounted rejected owner, a second detached
owner, latch, and empty release together.  This is a test-strength/runtime
gap, not a current source blocker; no Cargo terminal was run by this audit.

### Reopened RED — reserve-to-latch mount race directly drops a rejection

The capped `fail_detached` path no longer establishes end-to-end safety.  A
reservation can linearize before `reaper_failed` is latched; after its caller
creates a normal `GuestRelayMountedOwner::Rejected`, `mount` sees the latch,
replaces its reservation with `Empty`, then executes a bare `drop(owner)`
(`🖥️host/🦀️.rs:4050-4065`).  That owner has not yet entered a slot, so the
new bounded rejected-owner close loop never sees it.  It again violates
`WorkerJobSessionAdmissionRejected`'s required incremental-close-before-Drop
in a debug build and leaks its `ManuallyDrop` fields in release
(`🧵️job/🦀️.rs:2617-2621`).

This is a real concurrent fatal-admission path, not a stale source finding.
The latch branch in `mount` must use the same bounded rejected-owner retirement
protocol before dropping the owner; the proposed forced-rejection test seam
should race a reserved owner against the latch and prove no panic/leak.  The
lifecycle scope remains RED until that mount-race transfer is closed.

### Mount-race transfer repair — source closure, future-invariant caveat

The mount-race RED is superseded in current source.  The latch branch now
passes a raced `Rejected` owner through the shared `close_rejected` routine
before it is dropped, and `fail_detached` uses the same routine for stored
owners (`🖥️host/🦀️.rs:4050-4066,4137-4162`).  With the current pre-construction
4-MiB input cap, retirement-slot exhaustion has no fault source and needs at
most 261 wrapper close opportunities; the only alternate constructor is the
authority-admission path, which can add exactly one retained fault source
(`🧵️job/🦀️.rs:2077-2084,2831-2846`).  The 264-opportunity hard bound covers
both without a direct drop of nonterminal `ManuallyDrop` fields.  I find no
current source leak or debug-panic path in the reserve-to-latch race.

This conclusion relies on the two present rejection constructors.  A future
new `WorkerJobSessionAdmissionRejected` state must either preserve the same
bound or cause `mount` to retain/fail safely rather than drop after the helper.
The current law still does not force a real rejected owner through this exact
race, so an integrated forced-rejection fixture remains desirable; this is
test-strength evidence pending, not a source blocker.

### Guest-cold serial cancellation wait — harness pump closure

The two serial guest-cold failures after `drop(WorkerJobSession)` were a test
driver omission, not evidence of a production lifecycle fault.  Drop only
places the still-live session's retirement node in the process-global
retirement slots and raises its wake; it deliberately does not execute close
or guest cancellation (`🧵️job/🦀️.rs:3030-3050`).  A pool timer barrier can
run already-submitted pool work, but cannot consume that separate global
retirement queue.

The current `wait_for_cancel_admission` now makes exactly one bounded
retirement opportunity per its existing 64-iteration wait loop:
`pump_worker_job_retirements(1, 1, JOB_PAYLOAD_PAGE_BYTES)`, followed by the
same timer barrier (`🖥️host/🦀️.rs:5351-5367`).  This is the correct production
shape: the renderer likewise invokes that exact `1, 1, PAGE` pump once per
present step (`📺️renderer/🦀️.rs:14537-14544`).  The pump atomically claims at
most one slot, runs one bounded close step, and restores unfinished work for a
later opportunity (`🧵️job/🦀️.rs:2383-2414`).  For a dropped guest relay, its
first opportunity invokes `begin_close`; subsequent opportunities reach
`GuestColdRelayJob::close_step`, whose cleanup schedules the one cancel job
(`🖥️host/🦀️.rs:3689-3757`), and the following timer barrier lets that scheduled
pool job obtain the mock admission.  It neither drains to completion nor adds
a production-only fallback.

There is one intentional global-queue caveat: the pump scans process-global
slots, so a parallel test can consume another retirement node first.  The
helper remains bounded and observes only its own mock's exact-one admission,
so this is fairness/timing rather than a false positive; focused exact-law
runs should remain one selected law per invocation.  A test must not replace
this with an unbounded drain or a test-only direct cleanup call, either of
which would cease to represent the renderer's bounded retirement contract.
No Cargo run was performed by this audit.

## Current broad-gate residual triage — 235 pass, 7 fail, 1 ignored (coordinator-observed only)

The stated broad terminal is coordinator evidence, not an independently run
result.  I re-read the seven named current test subjects.  Six are stale test
assumptions after the deliberately bounded `pump_primed` contract; one exposes
a real priority-path defect masked by the test double.  None is an excuse to
credit the broad gate green.

### `pump()` return contract is internally inconsistent

`ShardLoop::pump` still says it drains every already-buffered frame and returns
the number of *actors* driven (`🧵️shard/🦀️.rs:1479-1486`).  The implementation
instead admits at most one frame and executes at most one deferred authority
(`:1556-1589,1594-1632,1725-1728`).  Its `usize` therefore counts one bounded
work opportunity, not guest actor turns: `Register`, `Unregister`, `Suspend`,
and an unknown-actor `Event` all correctly return `1`, even though only some
invoke guest code.  The executor does not depend on the number; it uses
`drive_one`'s retained-work state and resubmits (`🧵️executor/🦀️.rs:580-657`).

The exact safe repair is to change the public documentation/name semantics to
“one bounded authority/work opportunity” and make direct tests assert the
observable outcome/state, not the obsolete actor count.  Do **not** restore an
unbounded transport drain merely to satisfy those assertions.

| Failing subject | Current-byte cause | Classification and exact repair |
| --- | --- | --- |
| `pump_reports_an_envelope_for_an_unregistered_actor_as_a_fault_not_a_silent_drop` | An unknown actor is deliberately queued as `Event`, then produces `ShardOutcome::Fault`; that consumes one authority (`🧵️shard/🦀️.rs:2031-2055,1609-1613,1739-1744`). Its `driven == 0` assertion is at `:2609-2622`. | **Stale assertion.** Expect one work opportunity and retain the asserted named fault. |
| `suspend_with_checkpoint_true_surfaces_checkpoint_bytes_in_the_outcome` | `Suspend` calls real `checkpoint`, emits `Checkpoint`, then returns `Ok(1)` (`:1624-1626,2057-2069`), while the law asserts zero at `:2833-2859`. | **Stale assertion.** Expect one bounded authority; preserve byte/operation assertions. |
| `unregister_frame_drops_the_instance_exactly_like_the_direct_call` | A decoded `Unregister` is queued and actually calls `unregister`, then returns `Ok(1)` (`:1927-1928,1603-1608`); the law asserts zero at `:3328-3344`. | **Stale assertion.** Expect one work opportunity and keep the instance-removal check. |
| `register_frame_is_accepted_without_error_and_has_no_local_side_effect` | `Register` is intentionally wire-symmetry-only locally but still occupies and consumes an authority (`:1927,1602`); it returns `1`, not zero (`:3350-3360`). | **Stale assertion.** Expect one consumed administrative opportunity and no instance creation. |
| `job_step_uses_the_owning_actors_last_granted_budget` | The fixture sends a bare empty Grant and a separate raw `JobStep`, then calls one pump (`:3270-3308`). `LoopbackTransport` is LIFO (`:2383-2389`), so it sees the step before the Grant; independently, one pump cannot admit both frames, and the current step path rejects an operation without a retained `JobAuthority` (`:1640-1648,2127-2149`). | **Stale/non-production fixture.** Use FIFO/actual `SharedThreadTransport`; first drive a real spawn/replay to retained authority, then admit its Grant and matching `JobStep` over bounded successive drives before checking the recorded budget. |
| `revoked_capability_cancels_only_its_own_operations_and_actor_survives` | The test swaps an `AlwaysOkRouterHandler` (`⚡️effects/🦀️.rs:1338-1353`), but `run_router_effect_job` ignores that handler and intentionally returns `Fault("...pump is not mounted")` for every non-cancelled call (`:566-571`). Thus the sibling cannot be `Ok` even though its distinct token was not revoked. | **Stale/mis-scoped capability law; existing fail-closed router gap, not evidence that revocation cancels the sibling.** Either mount the real retained router/session pump and exercise two real operations, or test distinct-token cancellation at the operation-context seam. Do not weaken this to two generic errors: it would cease to prove sibling liveness. |

### Interactive priority is a production RED, not merely a stale count

`an_interactive_grant_is_executed_before_background_grants_queued_the_same_pump`
queues five Background frames then one Interactive frame and expects six turns
in one direct pump (`🧵️shard/🦀️.rs:3476-3543`).  Its `LoopbackTransport` uses
`Vec::pop`, so the last Interactive test frame is read first (`:2367-2389`);
after the one-frame change it observes only that one turn.  It no longer
proves priority over a real FIFO backlog.

Production ingress is FIFO `ThreadTransport::try_recv_now`
(`🎭️actor/🦀️.rs:4645-4651`), and `ShardExecutor::run` calls exactly one
`drive_one` per submitted pool job (`🧵️executor/🦀️.rs:580-657`).
`pending_lane_rank` chooses only the WorkerPool submission lane
(`:455-488`); it never reorders frames already written to the FIFO transport.
Consequently a Background frame admitted first is executed before a later
Interactive frame, despite the two deferred rings.  The advertised priority
property is currently false for that normal backlog shape.

Repair production ingress rather than inflating the test: retain the
one-authority execution bound, but establish a bounded lane-aware ingress
selection boundary before `drive_one` (for example fixed per-lane frame rings
owned by `ShardExecutor`, with FIFO within lane and explicit byte/item caps),
or boundedly pre-admit currently-ready frames before selecting one high-priority
authority.  The replacement law must use the actual FIFO executor/transport,
hold five background frames pending, then admit an interactive frame and prove
the next executed outcome is interactive without dropping/duplicating any
background owner.  This remains a production scheduling **RED**.
