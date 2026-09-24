# WP-H6 — Hub Open-Plan Admin Revocation Hang (0 % CPU, 70 min)

Slice: H6 (session 10). Private cargo target: `.tmp-ticket/wp-h6/target`. Captures: `wp-h6/generated/`.

## Status

| Item | Status |
|------|--------|
| 1. Reproduce the hang under load | Current tree: **0 hangs in 38,800 law runs** (12×300×3 + 10×400×7). The hung binary (pid 92025, 05:11) no longer exists, and its race is fixed in today's tree. It was reproduced deterministically at pool level on a copy of the 05:11 parking code (below) |
| 2. Thread stacks | Taken on the reproduced pre-fix pool: worker-0 `Condvar::wait` (sleeper with no timeout), worker-1 `Condvar::wait_timeout` (keeper on the 600 s far deadline), caller in `recv_timeout`, 0.0 % CPU (`far-keeper-prefix-sample.txt`). c8's `sample` could not attach because an ad-hoc test binary has no `get-task-allow` (see Log) |
| 3. Root cause + law pinning it | Root cause: a lost timer re-arm in `⏳️async/🔔️worker-parking`. R7 fixed the product side (10:19). New pool law `a_timer_re_armed_by_a_firing_callback_never_waits_behind_a_far_keeper`: **red on the 05:11 code (chain 0 stalled 10 s), green on today's code**. Hub open-plan tokio laws now carry `LawHangWatchdogV1` |
| 4. 200× loop, hub quick + long, full chain | 200× the 7 chain bin laws (1,400 runs): **0 hangs, 0 failures**. Hub quick **327/327**, long **336/336**. Chain: every stage passed, but the bun process then never exited (below) |
| 5. (coordinator add-on) kernel `os_store::sync` parallel-libtest flake | Three root causes found and fixed. Parallel libtest went from **5–10 % red to 100/100 green** (plus 50/50 with `--nocapture`). Full kernel lib **1196/1196** |
| 6. (coordinator add-on) hub quick + long on the final tree | DONE after the kernel fixes: quick **327/327** (10 skipped), long **336/336** (1 skipped) (`hub-quick-3.txt`, `hub-long-3.txt`) |

## Root cause

**What happened (forensics, unified log).** In `browser-document-open-check` at 05:10, cargo relinked the hub bin test
binary `os_hub-6335236f37e0415a` between law 2 and law 3. There were six relinks (04:58, 05:03, 05:06, 05:08, 05:09 and
05:11:37) while peers edited the tree. The last one ran as pid 92025, the revocation law, and it hung. So the hung
binary was built from the 05:11 tree, which nobody else tested.

**What that tree contained.** G6 replaced the pool's 4 ms idle poll with `WorkerParking` at about 04:47–04:52
(`wp-g6/generated/g6-async-tests1.txt` 04:51 already runs the new parking laws). That design has no poll interval: a lost
wake-up is now permanent. R7 later fixed a lost timer re-arm in it (fixture case
`firing-earlier-deadline-rearms-stale-keeper`, file time 10:19; it is absent from G6's 07:45 capture). So the
05:11 binary had G6's parking without R7's fix.

**The race (05:11 code).** A pool has one worker firing a timer callback (W) and one idle worker (B):
1. W pops timer `t_i` and runs its callback outside the wheel lock. The wheel now holds only a far deadline (for the
   hub: e.g. `DATABASE_CREATE_CATALOG_DEADLINE_MS` 30 s, the sync-hello deadline, lease renewals).
2. Work ingress wakes B. B finds no job and parks with `timer_due = far`. No keeper is armed, so B becomes the timer
   keeper and calls `wait_timeout(far)`.
3. The callback re-arms `t_{i+1} = now + 1 ms` (the db's retry chains all do this). Registration comes from the firing
   worker, so the hook calls `signal_timer_from_firing_worker`. The 05:11 code only bumped the signal and did not notify
   the keeper.
4. W rescans and nothing is due yet. It parks with `timer_due = 1 ms`, but a keeper exists, so W becomes a plain
   sleeper (`Condvar::wait`, no timeout). B keeps sleeping on the far deadline, and the chain waits for it at 0 % CPU.

**This path runs in the open-plan laws.** On today's binary (debuggable copy, lldb breakpoint counts,
`lldb-timers-3.txt`), one process running the seven open-plan bin laws hit `WorkerPool::callback_at` 21× (11×
`DatabaseCapabilityOpenState::retry_submission_at`, 5× `DatabaseCatalogBootstrapState::arm_retry`, 1×
`db_io_submit_job`, 4× boxed) and `WorkerParking::signal_timer_from_firing_worker` 1×. Each `test_state()` builds
its own 2-worker pool (`hub_worker_pool()` under `cfg(test)`), which is the W/B shape above.

**Why it is rare.** Step 2 needs B to park inside W's callback window, which lasts microseconds. Load widens that
window. One chain run in ~10 hung; on the current tree, 38,800 runs gave 0.

## Law (schema-first)

- `🧰️framework/🔨️modules/⏳️async/🔔️worker-parking/🧫️fixtures/🔣️.json` + `🧬️schema/🔣️.json`: `farKeeper
  {workers 2, farDeadlineMs 600000, chains 20, rearmsPerChain 4, intervalMs 5, chainBoundMs 10000}` (required; `workers`
  const 2).
- `…/⏳️async/🧪️tests/🔬️native-pool-unit/🦀️.rs`: `a_timer_re_armed_by_a_firing_callback_never_waits_behind_a_far_keeper`.
  A 600 s far deadline is armed. Each chain callback submits one no-op job, which wakes the sibling, and waits until
  the sibling has parked (`parking.sleeps()` moved), so the sibling is the keeper on the far deadline. Only then does
  the callback re-arm. Every chain must finish within 10 s. This is deterministic, not a stress loop.
- `…/⏳️async/📦️packages/🦀️rust/📜️script.ts` `worker-parking-check`: the oracle asserts `farDeadlineMs > 6 × chainBoundMs`
  (a parked chain cannot hide inside the bound). The law is in the `--native` exact-law list.
- Red/green proof in a standalone copy of `⏳️async` + `⏱️trace` (`wp-h6/prefix-probe/`, own workspace and target, so no
  shared tree or build-dir is touched). `parking-prefix.rs` is today's parking minus R7's `timer_moved` re-arm, which is
  the 05:11 design. `parking-fixed.rs` is today's file:

| Parking | Law result | Capture |
|---|---|---|
| 05:11 design | **FAILED**: "chain 0 waited more than 10s behind the far keeper" (10.02 s); R7's protocol law also fails (600 s keeper wait) | `far-keeper-prefix-1.txt`, `far-keeper-prefix-sample.txt` |
| today | **5/5 ok** (0.95–1.43 s) | `far-keeper-fixed-probe.txt` |

- Hub: the four `#[tokio::test]` open-plan laws in `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` arm `LawHangWatchdogV1`. The revocation law
  names its phases (`hub state open`, `session revocation`, `share space and document announce`, `share revocation`,
  `hub state teardown`). A future wedge now aborts in 25 s with a named phase. Before, it held the hub mutex for 70 min.

## Kernel `os_store::sync` flake (coordinator add-on)

Measured before the fix: 1/20, 3/50 and 4/40 parallel runs red on the current binary, in `folder_external_edit_delivers_remote_operations`
(`persisted edit on disk not satisfied before 5s`), `fixtures_replay_matches_expected_events` (`no codec registered for
fixture schema "demo/v1"` / `seed snapshot … on disk not satisfied`), `two_hosts_converge_through_hub` and
`retained_readiness_wake_after_turn_release_is_observed_once`. While a persistence law stalled, `sample` showed every pool
worker parked and idle. So nothing was queued, and the actor had lost its wake. I found where with temporary `[DEBUG]`
traces (runner pointer, mailbox, relay, store flush), all removed since:

1. **The actor idled with a queued store owner (main cause, product).** The store pushes `Genesis` and then
   `Mutations` onto the channel backbone, and each push wakes the actor once. The test's `LocalMutations` command runs
   `handle_cmd`, which relays ONE owner (`Genesis`) and spends a wake. The phase rotation then goes straight to `Status`
   and returns `Idle` while `Mutations` is still queued. No wake is left, so the actor sleeps until the test drops the
   store (trace `ksync-trace-fail5.txt`: store `sent Ok` → relay `genesis` → `idle` → `mutations` relayed only after the
   test's panic). Fix (`🏪️store/🔄️sync/🦀️.rs` `drive_one`): `Status` idles only after, in the same turn, it sees an
   empty mailbox (`has_pending`) and an empty outbound (`relay_one_backbone` → `Ok(false)`). Otherwise it stays runnable
   in the `Backbone` phase. The `Backbone` phase also stays put while it relayed an owner or hit transient contention,
   instead of dropping the `Err`. A send that races the empty check still sets `wake_requested`, which the release swap
   observes.
2. **Mailbox and readiness wakes could turn stale (product).** `schedule()` read `turn_generation` and
   `request_wake` compared it again. When a turn completed between those two reads, the generation moved on and the wake
   was dropped. Fix: `schedule()` (mailbox, backbone, readiness, close) has no generation. Only a turn future's own
   waker and its deadline go through `request_wake(generation)`.
3. **The shared codec registration raced (test).** `ensure_demo_codec_registered` used `AtomicBool::swap`, so a
   concurrent second caller returned before the first had registered `demo/v1`. Fix: `std::sync::Once::call_once`.

| Command (parallel libtest, `os_store::sync`, 58 laws) | Result | Capture |
|---|---|---|
| before, current binary | 19/20, 47/50 | `ksync-fail-*.txt` (swept) |
| after the codec `Once` only | 45/50 | `ksync-after-50.txt` |
| after fix 2 as well | 49/50 | `ksync-fix2-50.txt` |
| **final (1+2+3)** | **100/100**; `--nocapture` (the timing that failed on the first run before) **50/50** | `ksync-final-100.txt` |
| full kernel lib `--features sync,ureq` (parallel) | **1196/1196** | `kernel-lib-full-1.txt` |
| `cargo check -p semio-framework-os-kernel` native `--features sync,ureq` and `--target wasm32-wasip2` (rule 13) | EXIT 0 / EXIT 0 | console |

## Chain exit leak (found, not fixed)
`browser-document-open-check-2.txt`: oracle, Chromium runtime, OS test-quick, plan oracle, parity and all 8 server laws
passed, and the final "… current server laws passed" line printed at 16:02. After that, the `bun ./📜️script.ts
browser-document-open-check` process never exited: it idled in `kevent64` with a `CFThreadLoop` (FSEvents) thread and a live
`esbuild --service` child, and held the hub mutex. Killing the esbuild child did not help, so at 16:23 I killed my own
bun pid (25782), which is why the phase reports `CHAIN rc=1`. The leak is a handle left open by
`proveBrowserDocumentOpenRuntime` (Vite os-dev server / plugins). I routed it as a separate task chip.

## Log
- 13:36 hub mutex was held by h4 (dead pid), build queued (pid 90974), 9 min cold build.
- 13:46–14:30 loops on the current binary: 12 lanes × 300 × 3 laws (L) and 10 lanes × 400 × 7 laws (C, the chain's bin
  laws in chain order): 0 hangs, 0 failures. One 16-lane `taskpolicy -b` attempt (B) only starved itself at load 159: RN
  state, no output. I killed it and it is not counted.
- Unified log: the chain's cargo relinked the binary at 05:11:37, and pid 92025 was the hung law. c8's `sample` at 05:29/05:59
  left only a sandbox line. lldb/`sample` refuse ad-hoc test binaries ("Not allowed to attach"). A copy signed with
  `com.apple.security.get-task-allow` (`wp-h6/get-task-allow.plist`) can be attached. Launching it under lldb fails
  with EPERM on `~/Documents` (TCC), so attach by `--waitfor` instead.

## Evidence

| Command | Result | Capture |
|---|---|---|
| `cargo test -p semio-hub --bin os-hub --no-run` (hub mutex, cold) | EXIT 0, 9m02s | `build-1.txt` |
| loop L: 12 × 300 × {ledger, revocation, receipt-exchange} | 10,800 runs, 0 hangs, 0 fails | `loop-L*.txt` |
| loop C: 10 × 400 × 7 chain bin laws | 28,000 runs, 0 hangs, 0 fails | `loop-C*.txt` |
| lldb breakpoint counts (open-plan laws, today's code) | `callback_at` 21, `signal_timer_from_firing_worker` 1, `signal_timer` 20, `hand_off_timers` 1 | `lldb-timers-3.txt` |
| probe law on 05:11 parking | FAILED (chain 0 stalled) | `far-keeper-prefix-1.txt` |
| `sample` of the stalled probe | 2 pool workers in `__psynch_cvwait` (wait / wait_timeout), 0.0 % CPU | `far-keeper-prefix-sample.txt` |
| probe law on today's parking | 5/5 ok | `far-keeper-fixed-probe.txt` |
| `cargo check -p semio-framework-async --tests` (rule 13) | EXIT 0 (28 warnings) | `async-check-1.txt` |
| `cargo test -p semio-framework-async --lib -- native_pool worker_parking` | **16/16** | `async-laws-1.txt` |
| `bun nx run @semio-tech/framework-async-rs:worker-parking-native-check` | EXIT 0; oracle AJV=1 far-keeper-chains=20; 6 exact laws | `worker-parking-native-check-3.txt` |
| `cargo check -p semio-hub --bin os-hub --tests` | EXIT 0 (bin test 65 warnings) | console |
| `cargo test -p semio-hub --bin os-hub --no-run` (hub mutex, with the watchdog edit) | EXIT 0 | `build-2.txt` |
| 200× the 7 chain bin laws, chain order (new binary) | **1,400 runs, 0 hangs, 0 fails** | `loop-F.txt` |
| hub `test quick` (1st try) | 327/327 | `hub-quick-1.txt` |
| hub `test long` / chain (1st try) | did not compile: peer half-edit in `semio-s-artifact-stdio-dwg` (`DwgGeometryEntity`), which t9 fixed at 15:36 | `hub-long-1.txt`, `browser-document-open-check-1.txt` |
| hub `test quick` / `test long` (2nd try, before the kernel fixes) | **327/327** (10 skipped) / **336/336** (1 skipped) | `hub-quick-2.txt`, `hub-long-2.txt` |
| `bun nx run os-hub:browser-document-open-check` (2nd try) | every stage passed; the process did not exit (see above) | `browser-document-open-check-2.txt` |

## Files changed
- `🧰️framework/🔨️modules/⏳️async/🔔️worker-parking/🧫️fixtures/🔣️.json`, `…/🧬️schema/🔣️.json` — `farKeeper`.
- `🧰️framework/🔨️modules/⏳️async/🧪️tests/🔬️native-pool-unit/🦀️.rs` — far-keeper law.
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts` — oracle assertion + native law list.
- `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` — `LawHangWatchdogV1` on the four tokio open-plan laws, revocation phases.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs` — `drive_one` idle only on empty mailbox + outbound; `Backbone`
  phase drains/retries; generation-free `schedule()`, turn-scoped `request_wake`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` — `ensure_demo_codec_registered` via `Once`.
- Ticket inputs: `wp-h6/hub-phase.sh`, `hub-phase-2.sh`, `hub-phase-3.sh`.
- Ticket inputs: `wp-h6/loop.sh`, `wp-h6/far-keeper-law.rs`, `wp-h6/lldb-*.txt`, `wp-h6/get-task-allow.plist`,
  `wp-h6/prefix-probe/{Cargo.toml,parking-prefix.rs,parking-fixed.rs}`.

## Pids

| pid | what |
|---|---|
| 90974 | cold hub test build through the hub mutex (done) |
| loop lanes L1–L12, C1–C10 | done; B1–B16 killed by me (starved) |
| 3809, 21408 | hub phases 1+2 through the mutex (done; 25782/25801 = my lingering chain bun + esbuild, killed by me) |
| 46505 | hub phase 3 (quick + long on the final tree, done) |

No process or lock of mine is running. `wp-h6/target`, `wp-h6/bin`, `wp-h6/🗑️generated` and the probe `target`/`build` dirs are deleted (rule 14).

## Gaps
- The 05:11 binary no longer exists, so its own stacks could not be taken. The mechanism is proven on a copy of the 05:11 parking code (red), and it is fixed in today's tree (green).
- The kernel actor fixes rely on the 100/100 + 50/50 parallel gates. No deterministic single-law pin was added for the relay-then-idle interleaving.
- The chain's bun process does not exit after passing (routed as a task chip).
