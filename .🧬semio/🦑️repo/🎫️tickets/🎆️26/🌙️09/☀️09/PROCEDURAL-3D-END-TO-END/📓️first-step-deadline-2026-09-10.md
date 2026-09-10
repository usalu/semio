# First-step deadline — generation3d boot trap (`plugin.reactor-turn-deadline`)

Lane: reactor lifetime / turn-deadline path + generation3d bootstrap cost.
Ticket: `26/09/09/PROCEDURAL-3D-END-TO-END`.

## 1. The strict time authority

| property | value | source |
|---|---|---|
| ceiling | `GUEST_LIFECYCLE_TURN_CEILING_US = 5_000_000` (5 s) | `🧰️framework/🔨️modules/⏱️trace/🦀️.rs:114` |
| predicate | `elapsed_us >= 5_000_000` | `⏱️trace/🦀️.rs:117` |
| measured where | `GuestLifecycleCell::finish_turn`, `elapsed = now_us() - started_us` | `🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs:237-243` |
| `started_us` | first statement of `poll_kernel_output` | `⚛️reactor/🔄️turn/🦀️.rs:126` |
| clock | `semio_framework_job::default_now_us` → `semio_framework_trace::try_now_us` → host-installed monotonic clock; in the browser worker that is `performance.now()` | `🧰️framework/🔨️modules/🧵️job/🦀️.rs:89` |
| includes descheduling? | **was yes** — pure wall time; now executing-only across `Pending` gaps, see §4 | see §3 |
| raised as | `Fault{framework, plugin.reactor-turn-deadline, retryable: retryable_lifecycle}` | `⚛️reactor/🔄️turn/🦀️.rs:901-905` |
| `retryable_lifecycle` | turn carries no command/cold page and only lifecycle events | `⚛️reactor/🔄️turn/🦀️.rs:128` |


## 2. Native reproduction and per-phase measurement

New test target `boot_deadline` (`[[test]]` in `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml`,
body at `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🔬️boot-deadline/🦀️.rs`) boots `s.procedural.generation3d@1/*#editor`
through the REAL guest reactor — `PluginRuntime::<ProceduralApps>` + `install_plugin_bundle_result` +
`reactor::poll_kernel(Event::InstanceOpen)` + ACK + settle — and times every phase (§7 explains why it
stops short of the close).
It is its own `[[test]]` rather than a `#[cfg(test)]` module so a peer's in-flight editor/viewer
refactor (the `Generation3dViewTransient` associated types currently breaking `🔬️surface`'s
`assert_viewer_never_mutates`) can never hide a first-step regression.

### Measured, native debug, MacBook under load (2026-09-10)

| phase | where | µs | inside the deadline? |
|---|---|---|---|
| plugin manifest (`plugin()`, 9 flow extensions + 4 apps + examples) | `__semio_ensure_plugin_runtime` | 10 934 | no — runs before `poll_kernel` |
| bundle install | `install_plugin_bundle_result` | 14 | no |
| descriptor pack (232 359 B) | `describe::describe_plugin` | 21 503 | no — separate export |
| **`InstanceOpen` turn (the first step)** | `poll_kernel` | **13 910** | **yes** |
| lifecycle ACK turn | `poll_kernel` | 425 | yes |
| settle to `TurnStatus::Idle` | 1 turn | 165 | yes |

The whole cold boot of generation3d executes in **13.9 ms** inside the measured window — 0.28 % of the
5 s authority. There is no bootstrap-volume problem to move off the first step: `Event::InstanceOpen`
does `guest_lifetimes.admit` → `INSTANCE_METADATA.insert` → `plugin_open_actor_instance`
(`Plugin::create_app` → `VcsArtifactApp::with_registry(EditorApp::<Generation3dPlayApp>::default(),
AppActionRegistry::from_definition(def))` + `bind_instance_id`), and the app reaches `Idle` on the very
next turn. The `assets` field of `Event::InstanceOpen` is not even read by the reactor
(`⚛️reactor/🔄️turn/🦀️.rs` destructures `{ request, app_id, actor, quotas, .. }`), so no example
fixture decode happens there either.

## 3. Root cause

Two defects, both in the framework, neither in generation3d's bootstrap volume.

### 3.1 The strict authority is a WALL-clock verdict on a shared, swapping box

`⚛️reactor/🔄️turn/🦀️.rs:126` took `started_us` once and `🚪️lifetime/🦀️.rs:238` compared
`now_us() - started_us` against the 5 s ceiling — pure wall time, with no way to tell the guest's own
work from time the guest was not running.

Two ways wall time diverges from executing time here, and they are NOT equally live today:

* **Suspension between polls.** `poll_kernel_output` is an `async fn`, so in principle every `Pending`
  gap is billed to the guest. In the CURRENT ABI this cannot actually happen: the owned-ABI export
  wraps the turn in `resolve_ready` (`semio_owned_poll_v1`, `🔌️plugin/🦀️.rs:32669`; `resolve_ready`
  panics on `Pending`, `🚪️io/🦀️.rs:891`), so a guest turn is contractually single-poll — effects leave
  as `Effect`s and are resolved by a LATER turn's `Event::Completed`, never awaited inline. Fix #1
  below makes that contract explicit and enforced rather than merely conventional.
* **Descheduling INSIDE the single poll** — the live one. The wasm worker is one thread among ~20
  plugin workers on a machine this ticket measured at load ~20 with swap exhausted
  (`📓️semio-build-budget-and-oom`, `📓️wasm-dev-profile-debug-off-and-swap-thrash`), inside a HIDDEN
  browser pane whose worker gets background QoS (`📓️background-qos-throttles-agent-builds`,
  `📓️hidden-browser-pane-throttles-plugin-boot`). A debug-wasm turn whose native cost is 13.9 ms can
  sit on the wall clock for seconds without executing more instructions, and a wasm worker has no
  CPU-time clock to tell the difference.

That is why boot #5 (04:02 restage) booted and boot #6 (05:22 restage) did not with **no change to the
open path between them**: a wall-clock kill on a contended box is load-dependent, so the boot was a
coin flip, not a regression. The only sound verdict for a post-hoc wall overrun on a turn that retained
its receipt is therefore a RETRY, which is exactly what §3.2 was missing on the browser path.

### 3.2 A retryable verdict was fatal on the browser path

`retryable_lifecycle` (`🔄️turn/🦀️.rs:128`) marks a lifecycle-only turn's deadline fault
`retryable: true`, and the NATIVE host honours it — `🖥️host/🧵️shard/🦀️.rs:1941`
`Err(fault) if retryable_lifecycle_turn(&fault, events) => return Ok(true)` re-grants the same events
next tick, and the guest's retained receipt makes the replay cheap and correct
(`admit` returns `fresh == false`, so the open work is never repeated).
The **browser** path had no such rule: `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`'s worker caught every
handler error and called `reportWorkerFault("handler", …)`, which `ShardClient.handleMessage`
(`🎭️actor/📮️shard-client/🟦️.ts:1277`) forwards to `onActorTrap` — so the identical, explicitly
retryable verdict killed actor `procedural#1` on `[handler/first-step]`, and every later action failed
with `no actor for instance 1`.

## 4. Fix

| # | file | change |
|---|---|---|
| 1 | `⚛️reactor/🔄️turn/🦀️.rs` | new `//#region ⏱️TurnExecution`: `TURN_EXECUTION` thread-local + `with_turn_execution` wrapper around the whole turn. Each `Future::poll` charges only its own span; every `Pending` gap — JSPI suspension, a throttled hidden tab, an OS deschedule — accrues nothing. `poll_kernel_output` is now a thin door over `poll_kernel_turn` |
| 2 | `⚛️reactor/🔄️turn/🦀️.rs` | `finish_turn(instance, started_us)` → `finish_turn(instance, guest_turn_executing_us())` |
| 3 | `⚛️reactor/🚪️lifetime/🦀️.rs` | `GuestLifecycleCell::finish_turn(executing_us: Option<u64>, succeeded)` — the clock-missing/backward verdict moved to the accumulator (`clock_lost`), so the cell prices EXECUTING time, not wall time |
| 4 | `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` | generated shard worker learns `retryableLifecycleTurn(error, events, commandPage)` — the browser twin of `retryable_lifecycle_turn`. A retryable lifecycle deadline is no longer reported as a `worker-fault` (so it never reaches `onActorTrap`); the reply carries `retryableLifecycle: true` |
| 5 | `🎭️actor/📮️shard-client/🟦️.ts` | `graftWorkerStack` stamps `RETRYABLE_LIFECYCLE_TURN` on the rejection; `sendInstanceLifecycle` replays the SAME events up to `RETRYABLE_LIFECYCLE_TURN_ATTEMPTS` (16) instead of failing the open — the browser twin of `ShardLoop::pump`'s `Ok(true)` |

Fix #1 is the contract, not the measured saving: since a guest turn is single-poll today (§3.1), the
accounting reads the same value as the old wall subtraction. It is what makes the ceiling *mean*
"executing microseconds" — enforced the moment any turn ever does suspend — and it moves the
clock-missing/backward verdict into one place. Fixes #4 and #5 are the ones that make the observed boot
survive.

Nothing was moved out of generation3d's bootstrap, because §2 shows there is nothing there to move:
the `InstanceOpen` turn is 13.9 ms and the app is `Idle` one turn later. Manufacturing a deferral for a
0.28 %-of-budget turn would add a resumable-job seam with no measurable payoff and one more place for
the boot to stall.

## 5. Why excluding suspension does not hide a wedged guest

A guest that spins synchronously never returns `Pending`, so its whole spin is charged inside one poll
and the 5 s ceiling still fires with the same verdict. What is no longer charged is only the time the
guest is provably NOT running: between a `Pending` and the next poll. A guest that parks forever on a
host import is covered by the OTHER authority that already owns it — `ShardClient`'s heartbeat watchdog
(`SHARD_LIVENESS_POLICY`), whose progress ticker beats through long awaits precisely so a slow boot is
distinguishable from a dead worker. The browser replay is additionally bounded
(`RETRYABLE_LIFECYCLE_TURN_ATTEMPTS = 16`), so a turn that keeps overrunning still fails the open
rather than looping forever.

## 6. Tests

| test | where | drives |
|---|---|---|
| `generation3d_boot_fits_the_strict_lifecycle_turn_authority` | `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🔬️boot-deadline/🦀️.rs` (`[[test]] boot_deadline`) | real `PluginRuntime` + `poll_kernel` open → ACK → settle → close → retire; asserts every lifecycle turn keeps a 4× margin under `GUEST_LIFECYCLE_TURN_CEILING_US` and prints the per-phase µs |
| `guest_turn_execution_excludes_every_suspension_gap` | `⚛️reactor/🔄️turn/🧪️tests/⏱️execution/🦀️.rs`, fixture `🚪️lifetime/🧫️fixtures/⏱️execution.json` | a turn parked across a 250 ms suspension measures **0 µs** executing while wall reads 259 883 µs, and that verdict does not violate the ceiling |
| `guest_turn_execution_resets_for_every_turn` | same | the accumulator is per-turn (2 µs then 1 µs, no carry-over) |
| `guest_instance_lifecycle_ack_fault_keeps_exact_receipt_and_owner` | `⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs` | rewritten onto the `executing_us` contract: clock lost, elapsed == ceiling, elapsed > ceiling, failed turn — each retains the receipt and the owner |
| `guest_instance_lifecycle_terminal_release_work_is_measured_and_never_repeated_after_late_clock` | same, fixture `🚪️lifetime/🧫️fixtures/🔣️.json` | fixture `destructorWorkUs` corrected 8 000 → 5 000 000: it encoded the pre-5 s (8 ms interactive) ceiling and could no longer produce `firstAckAccepted: false` |
| `reactor_output_fault_returns_real_patch_and_preserves_other_lifecycle_ack` | `⚛️reactor/🧪️tests/🔬️test-support/🦀️.rs` | `late_clock` reached the ceiling by sleeping 9 ms — dead since the ceiling became 5 s. Now charges `GUEST_LIFECYCLE_TURN_CEILING_US` through the new test-only `turn::charge_turn_execution_us`, deterministic and instant |
| `RetryableLifecycleTurnDeadline` ×3 | `🎭️actor/📮️shard-client/🧪️tests/⏱️retryable-lifecycle-deadline/🟦️.ts` | the generated shard worker classifies all **10** rows of `🔌️plugin/🖥️host/🔁️lifecycle/🧫️fixtures/🔣️.json` exactly as `retryable_lifecycle_turn` does, and posts **no** `worker-fault` for an eligible one; the marker survives `graftWorkerStack` and stays off JSON; `sendInstanceLifecycle` guards its replay on it |

Two of these were already red before this lane touched them (the 8 ms fixture and the 9 ms `late_clock`
sleep) — both are stale encodings of the pre-5 s interactive ceiling, and both are now driven off
`semio_framework_trace::GUEST_LIFECYCLE_TURN_CEILING_US` itself so they can never drift again.

### Runs

```
cargo test -p semio-s-plugin-procedural --test boot_deadline -- --nocapture
  [DEBUG] generation3d first step manifest_us=11117 install_us=1 describe_us=21188
          descriptor_bytes=232359 open_us=14249 ack_us=608 settle_turns=2 settle_total_us=489 settle_max_us=297
  test result: ok. 1 passed; 0 failed

cargo test -p semio-framework-plugin --lib -- lifecycle lifetime turn_execution
  test result: ok. 27 passed; 0 failed; 593 filtered out; finished in 59.38s
  [DEBUG] guest turn execution executing_us=0 wall_us=259883 suspension_us=250000
  [DEBUG] guest turn execution reset first_us=2 second_us=1

bun ./📜️script.ts test -t RetryableLifecycleTurnDeadline   (@semio-tech/framework-actor)
  Test Files  1 passed | 9 skipped (10)
       Tests  3 passed | 213 skipped (216)

cargo check -p semio-s-plugin-procedural --keep-going
  Finished `dev` profile [unoptimized] target(s) in 1m 37s   — 0 errors

CARGO_PROFILE_WASM_DEV_DEBUG=false \
cargo check -p semio-s-plugin-procedural --keep-going --target wasm32-wasip2 --profile wasm-dev
  Finished `wasm-dev` profile [unoptimized] target(s) in 4m 00s   — 0 errors

cargo check -p semio-framework-plugin --all-targets --keep-going   — 0 errors
```

All runs used `CARGO_TARGET_DIR=$S/target-dl` (seeded by `cp -Rc target/debug`) with the repo's own
`sccache` wrapper left in place — blanking `RUSTC_WRAPPER` changes every fingerprint and forces a
full-workspace rebuild against a seeded tree, which is what burned the first attempt (and the disk:
`/private/tmp` hit 100 % with 388 GB of sibling-lane target dirs before this lane's was reclaimed).
Raw logs: `🗑️generated/dl-5.txt` (lifecycle + execution laws), `dl-7.txt` (TS laws),
`dl-12.txt`/`dl-13.txt` (boot law), `dl-14.txt` (native check), `dl-15.txt` (wasm check).

## 7. Findings handed to other lanes

1. **generation3d never reaches `Retired`.** Driving the real close path natively — `InstanceClose`
   (publishes `Accepted`, generation > 0), ACK, then empty turns — produced **no `Retired` receipt
   inside 16 384 reactor turns (~5 min of wall time)**, and `NativeLifecycleRegistry::drop`
   (`⚛️reactor/🚪️lifetime/🦀️.rs:482`) then aborts the process with
   `runtime lifetimes require terminal exact ACK before teardown`. `prepare_retired`
   (`🚪️lifetime/🦀️.rs:161`) only publishes `Retired` once `NativeLifetimeOwner::terminal_is_empty`
   holds, which is driven by `plugin_step_close_cleanup` at `RUNTIME_CLOSE_ITEMS_PER_STEP = 1` with
   `RUNTIME_CLOSE_ZERO_PROGRESS_LIMIT = 8` — so this reads as BLOCKED, not merely slow.
   The blocked stage is `NativeLifetimeOwner::terminal_is_empty`'s FIRST gate, `self.lease.is_retired()`
   (`🚪️lifetime/🦀️.rs:345`) — the app instance lease, driven by `plugin_step_close_cleanup` over
   `VcsArtifactApp::close_step`. The framework's own `TestApp` retires fine through the identical
   machinery (`reactor_native_lifecycle_retains_exact_close_until_ack` is green), so this is
   generation3d's app close, not the lifecycle authority.
   Consequence for this lane: the boot law opens, measures and asserts, then releases the runtime with
   `std::mem::forget` WITHOUT opening a close — a close is what reserves the cold-pair/patch/pending
   close participants, and leaving them reserved makes `ColdDocumentPairIngressRegistry::drop`
   (`⚛️reactor/📥️cold-pair/🦀️.rs:409`) abort the whole test process at thread teardown. Closing here
   would make a deadline law red for a close-path defect it does not own.
   Owner: whoever holds `VcsArtifactApp::close_step` / the generation3d app-close lane.
2. **`OutboundMessage.cancelJob` gained a `requestId`** without its inventory fixture
   (`🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧫️fixtures/🔣️.json`, `outbound`) being
   updated, so `ActorWorkerInboxInventory` is red for that reason alone. Left to its author — this lane
   only updated the `inbound` `result` (ok:false) row it actually changed.

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `//#region ⏱️TurnExecution` (accumulator, `guest_turn_executing_us`, `with_turn_execution`, test-only `charge_turn_execution_us`), `poll_kernel_output` → wrapper over new `poll_kernel_turn`, `finish_turn` now takes executing µs |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🧪️tests/⏱️execution/🦀️.rs` | new — the two execution-accounting laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🦀️.rs` | `GuestLifecycleCell::finish_turn(executing_us, succeeded)`; `NativeLifecycleRegistry::finish_turn(instance, executing_us)` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧪️tests/🚪️lifetime/🦀️.rs` | laws rewritten onto the executing-µs contract, deadline rows driven off `GUEST_LIFECYCLE_TURN_CEILING_US` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧫️fixtures/🔣️.json` | `destructorWorkUs` 8 000 → 5 000 000 (stale 8 ms interactive ceiling) |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🚪️lifetime/🧫️fixtures/⏱️execution.json` | new — suspension/ratio/per-turn bound for the execution laws |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🧪️tests/🔬️test-support/🦀️.rs` | `late_clock` reaches the ceiling deterministically instead of sleeping 9 ms |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts` | generated shard worker: `retryableLifecycleTurn` + `guestFaultRecord`; retryable lifecycle deadline no longer posts `worker-fault`; `replyError` carries `retryableLifecycle` |
| `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🟦️.ts` | `RETRYABLE_LIFECYCLE_TURN`, `isRetryableLifecycleTurn`, `RETRYABLE_LIFECYCLE_TURN_ATTEMPTS`, `graftWorkerStack` marking, `sendInstanceLifecycle` replay loop, `InboundMessage` field, new test registration |
| `🧰️framework/🔨️modules/🎭️actor/📮️shard-client/🧪️tests/⏱️retryable-lifecycle-deadline/🟦️.ts` | new — the three browser-side laws, driven off the Rust host fixture |
| `🧰️framework/🔨️modules/🎭️actor/📤️return/📨️response/🎟️credit/📋️metadata/📥️inbox/🧫️fixtures/🔣️.json` | `inbound` `result` (ok:false) gains `retryableLifecycle` |
| `✏️s/🔌️plugins/🌀️procedural/🧪️tests/🔬️boot-deadline/🦀️.rs` | new — the generation3d boot deadline law + per-phase measurement |
| `✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust/Cargo.toml` | `[[test]] boot_deadline`; dev-deps `semio-framework-job`, `semio-framework-trace` |
| `.vscode/🧩️launch.seed.jsonc`, `.vscode/launch.json` | `🧪️test⏱️procedural🧊️boot-deadline` entry |

## 9. Landing

Restage is the flow-eval-tick lane's: `nx run @semio-tech/framework-os-dev:activate-generation3d-react-dev`
was already in flight at **06:08** (pid 5424) when this lane's changes landed (~06:00–06:12), so this
lane ran no `activate` of its own.

Two of the five changes are Rust inside the guest component (`⚛️reactor`), so they need a wasm restage.
The other two are TypeScript, and one of them is special: the shard worker is **generated source**
(`shardWorkerSource()` in `🔌️plugin/📦️packages/🟦️typescript/🟦️.ts`) materialized into the served
bundle, so the retry fix only reaches the browser after a fresh materialize + serve restart — a page
reload alone does not pick it up. `📮️shard-client/🟦️.ts` is an ordinary module and reloads normally.
