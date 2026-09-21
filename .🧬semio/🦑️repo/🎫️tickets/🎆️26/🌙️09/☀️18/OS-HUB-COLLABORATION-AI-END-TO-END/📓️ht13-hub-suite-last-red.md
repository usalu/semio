# HT13 — the hub suite's last red: `admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen`

Slice HT13 of ticket `26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END`.
Input: coordinator run `s7a` (03:46) — `🗑️generated/coordinator-hub-nextest-full-s7a.txt`, **321 — 320 passed / 1 failed**.

## 1. The panic, decoded

```
FAIL [ 5.720s] (278/321) semio-hub::bin/os-hub tests::admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen
thread 'hub-socket-test' panicked at 🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:2147:23:
  no server frame before 5s deadline
thread 'tests::admin_removal_…' panicked at 🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:470:10:
  socket test: Any { .. }
```

- `:2147` is the `Err(_) => panic!("no server frame before 5s deadline")` arm of the shared helper
  `next_server_frame` (`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs:2136-2150`). It is **not** the law's own line:
  the helper is called 49× across the suite and 8× inside this law, so the capture never said
  *which* frame was missing. §5 fixes that.
- `:470` is `run_socket_test`'s `.join().expect("socket test")` — the outer thread simply re-raises
  the inner panic. It carries no information of its own.
- **This is a different failure mode from every earlier round.** HT4–HT10 all failed this law in
  `stop_recovery_server` with `database shutdown deadline elapsed in phase PoolUse`. HT10 §13 closed
  that root (the sync-hello deadline timer holds a `Weak`) and the law was **GREEN at 00:28**.
  The 03:46 red never reaches the teardown at all — it dies in the socket phase, near the top.

### Arithmetic that rules scheduling jitter out

libtest reports `finished in 5.70s` for the test function, and exactly one 5.000 s deadline elapsed.
So **everything before the stalled wait took 0.70 s**: the sqlite connect, the `db::Database` open,
the admin authorization, three sessions, the member upserts, the genesis seed, two open-plan
exchanges, `spawn_restartable_server`, two websocket connects and the frames that did arrive. A
process that does all of that in 0.70 s was **not** starved by the fleet. Then a single frame failed
to appear for a further 5 s. That is a stall, not latency.

## 2. What did NOT change

| candidate | verdict |
|---|---|
| a peer's in-flight edit in the socket/presence/admin path | **no.** `git status --short -- 🌎️hub` at 04:0x: the only working-tree hub edits are `💡️inference/🏃️runtime` (+tests), three trusted-catalog fixtures, `📋️project.json`/`📜️script.ts`, and a **one-line** `🔬️bin-unit` change (`browser-jco-1.27.0-jspi` → `1.34.0` in `native_openable_stdio_bundle`). None of them is in this law's path. |
| HT11 / HT12 closer & disposer work | **no.** HT11 touched `🌎️hub/💡️inference/🏃️runtime` + `🧰️framework/…/🏪️store`; HT12 touched `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` only. Neither is on the socket/hello/presence path. |
| the hub socket handler itself | **no.** `🌎️hub/🏗️bootstrap/🦀️.rs` is last committed in `HEAD` (`48b9d63cf6`, 2026-09-20 23:20) and has **no** working-tree diff. It is bit-identical between the 00:28 green and the 03:46 red. |
| `🧰️framework/…/🛢️db/🔄️sync/🦀️.rs` | changed vs `HEAD`, but the diff is exactly HT10's `Weak` deadline timer + the `DATABASE_SYNC_HELLO_STATE_LIVE` census. Both were already in the tree at 00:28. |

**Same code, green at 00:28, red at 03:46 under load ≈ 40.** So the defect is a latent race that load
makes reachable — category (b)'s *timing*, but with a product root, not a short timeout.

## 3. Where the missing frame comes from

`🌎️hub/🏗️bootstrap/🦀️.rs`, the document socket handler:

| line | step |
|---|---|
| `:4812` | `state.db.hello(db_id, frontier, session_id, actor, 64 KiB).await` — a `DatabaseSyncHelloFuture` |
| `:4820` | `hello_session.take_welcome()` → the `Welcome` frame |
| `:4874` | `sender.send(welcome_bytes)` under a 2 s timeout |
| `:4888-4928` | `loop { hello_session.next_frame().await }` — the bootstrap drain, **the one await in this handler with no bound at all** |
| `:4933` | `encode(&ServerFrame::Session { … })` — contract §C7.3: after `Welcome` *and its follow-up bootstrap frames*, before any `Presence` |

So `Welcome` and `Session` are separated by an unbounded drain of the framework's sync-hello state
machine. The law's `Welcome` wait uses the file's 15 s `SOCKET_RENDEZVOUS_HANG_GUARD` and passed;
the very next wait — the `Session` frame — is the first one that sits behind that drain and the
first that uses the leftover 5 s helper.

## 4. Root — a lost wakeup in the sync-hello driver

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs`, `DatabaseSyncHelloState::drive_one`
(pre-fix `:1605-1618`):

```rust
let wake = self.wake_requested.swap(false, AcqRel);                     // (A)
self.driver.store(DatabaseSyncHelloDriverAuthority::Idle as u8, Release); // (B)
if pending || wake || self.demand.load(Acquire) { self.schedule(); }      // (C)
```

and the waker (`impl std::task::Wake`, `:1555-1563`) plus `schedule()` (`:1571-1577`):

```rust
self.wake_requested.store(true, Release); self.schedule();
// schedule(): if driver.compare_exchange(Idle, Queued, …).is_err() { return; }
```

`schedule()` deliberately returns when the driver is busy and **delegates the re-drive to (C)**.
Two windows lose it:

1. **Between (A) and (B) — no memory reordering required.** Another thread stores
   `wake_requested = true` and calls `schedule()`; the CAS reads `Driving`, fails, returns. (A)
   already swapped `false` out, and (C) never re-reads `wake_requested`. The wake is dropped.
2. **Across (B)/(C) — StoreLoad.** (B) is a `Release` store and (C) an `Acquire` load of a
   *different* location, so aarch64 (and x86's store buffer) may order (C) before (B). A
   concurrent `demand.store(true, Release); schedule();` (`DatabaseSyncHelloNextFuture::poll`,
   `:2243`) can then have its CAS observe `Driving` while (C) observes `demand == false`.

Either way the hello state machine parks with nothing scheduled. The **only** thing that ever
re-drives it is the `DATABASE_SYNC_HELLO_DEADLINE_MS` callback — **30 000 ms** (`:310`). So a lost
wakeup is observed by every caller as *a frame that never arrives*, for 30 s. Every deadline in the
hub socket handler (2 s) and in this test file (5 s / 15 s / the 25 s `LAW_BODY_HANG_GUARD`) is
shorter than that, so the first bound to expire always reports "no frame", never the real cause.

The waker fires from the pool worker running the inner `database_sync_hello_execute` sub-futures —
a *different* OS thread from the one running `drive_one`. With one busy worker the two windows are
sub-microsecond and essentially unreachable; under fleet load ≈ 40, with the `WorkerPool` threads
descheduled mid-window, they are reachable. That is exactly the 00:28-green / 03:46-red shape.

## 5. Fix

### 5.1 Product — the driver handoff never drops a signal

`🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs`

| line (post-fix) | change |
|---|---|
| `:1618-1624` `drive_one`'s release | the two signals are **re-read after** the driver goes `Idle`: `let signalled = self.wake_requested.load(SeqCst) || self.demand.load(SeqCst);` and the reschedule is `pending \|\| wake \|\| signalled`. This is what closes window 1 — a wake raised between the `swap` and the release is no longer swallowed. |
| `:1614-1615` | the `swap` of `wake_requested` and the `store` of `Idle` are `SeqCst`. |
| `:1555`, `:1560` `Wake::wake` / `wake_by_ref` | `wake_requested.store(true, SeqCst)`. |
| `:1578` `schedule`'s CAS | `Idle → Queued` is `SeqCst/SeqCst`, with a docstring stating the handoff law. |
| `:2257` `DatabaseSyncHelloNextFuture::poll` | `demand.store(true, SeqCst)`. |
| `:1600-1606` | new `drive_one` docstring naming the window and the 30 s consequence. |

With all six on `SeqCst` the two halves share one total order, so the classic proof holds: if the
signaller's CAS reads a busy driver then the releasing thread's loads are guaranteed to read the
signal; if the releasing thread's loads read no signal then the CAS is guaranteed to read `Idle` and
queue the turn itself. Exactly one of the two always re-drives the hello. It is a completion
guarantee, not a longer bound: **no timeout anywhere was raised.**

### 5.2 The law now names its phase

`🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs`: `next_server_frame` is a thin wrapper over a new
`next_server_frame_at(ws, phase)` whose five refusal arms all carry `at {phase}`. The 5 s bound is
**unchanged** — it is the bound that caught this defect, and raising it would have hidden a 30 s
stall behind the law's own 25 s `LAW_BODY_HANG_GUARD` instead. The seven waits inside
`admin_removal_revokes_visible_plan_presence_and_target_after_sqlite_reopen` are now labelled
(`🐍️ht13-name-frame-phases.py`, region-guarded to the law's own body):

`member session frame after the sync-hello bootstrap drain` · `observer welcome` ·
`observer session frame after the sync-hello bootstrap drain` ·
`observer normalized presence of the member` · `member echo of its own normalized presence` ·
`observer withdrawal presence after the admin removal` ·
`observer presence after the removal, still live`

The other 42 call sites keep the wrapper and print `at unnamed`.

## 6. Runs

Two sessions: the first batch at 03:57-04:05 (cut by the ~04:10 coordinator outage, rule 29), the
second at 10:28-10:37 after the resume. **The captures below are the second batch** — the first
batch's run 1 is quoted separately because its timing is evidence.

| # | command | capture | result |
|---|---|---|---|
| check | `cargo check -p semio-hub --all-targets --keep-going` (10:28) | `🗑️generated/ht13-check.txt` | **exit 0**, `Finished dev profile in 6m 26s`, **334 warnings** (58 in `bin "os-hub" test`) — the warning bodies are the proof the tree was really type-checked, not short-circuited |
| 1 | `cargo nextest run -p semio-hub --bin os-hub -E 'test(admin_removal_…)'` (10:35) | `ht13-law-run1.txt` | **PASS [1.743s]** — `1 test run: 1 passed, 128 skipped` |
| 2 | same (10:36) | `ht13-law-run2.txt` | **PASS [1.886s]** |
| 3 | same (10:36) | `ht13-law-run3.txt` | **PASS [1.500s]** |

All three in `CARGO_TARGET_DIR=.🧬semio/🦑️repo/⚡️cache/cargo/target-ht13`, one at a time, foreground,
load ≈ 10.

### 6.1 The first batch's run 1 corroborates §1's arithmetic

At 03:57, before peers grew the handler and the suite, the same law ran **PASS [0.647s]** end to end
— sqlite, database open, three sessions, genesis, two plans, a server, two sockets, the admin
removal, the full shutdown and the sqlite reopen with four routes re-probed. The s7a red spent
**0.70 s** doing that same work and then waited 5.000 s for a single frame. The law was nowhere near
a bound it owns; the 03:46 run stalled.

### 6.2 The margin has shrunk, and that is worth watching

Between 04:05 and 10:28 peers added **+143 lines to `🌎️hub/🏗️bootstrap/🦀️.rs`** and ~180 to the
bin-unit suite (two new laws: `126 tests skipped` → `128`). The law's own runtime went **0.647 s →
1.50-1.89 s**, i.e. from ~7.7× headroom under the 5 s frame bound to ~2.8×. That is still healthy,
but the bound this law fails on is now closer than it was, and §5.2's phase labels are what will
distinguish "genuinely slower" from "stalled again" in the next red.

### 6.3 Two peer breakages hit this slice (neither is mine, one was unblocked)

1. `✏️s/…/🌍️gis/🗿️artifacts/🗺️gismap/…/🎮️commands/🎨️example/🦀️.rs:33` — `parse_dsl(source.document_json())`
   stopped compiling because a peer changed `ExampleSource::document_json()` from `-> &str` to
   `-> String` in `🧰️framework/…/🔌️plugin/🦀️.rs` (deferred `ExampleSourceBody`, working-tree only)
   without sweeping this call site. `semio-hub` depends on it, so **no hub build of any kind was
   possible.** I applied the compiler's own one-character fix (`&source.document_json()`) to unblock
   the lane — the coordinator's rerun needed it too. Still the only edit of mine outside §8's first
   two rows, and still present at 10:28.
2. `semio-framework-os-kernel` — at 04:03-04:05, six then twelve `🏪️store/🧬️schema/🧬️mutations/**`
   leaves still declared `fn label(&self) -> String` against a trait a peer had already moved to
   `LocalizedLabel` (`🎯️restore-active-space-alternative`, `📌️commit-space-checkpoint`,
   `🌿️create-space-alternative`, `🔀️switch-space-alternative`, `🧹️remove-space-alternative`,
   `🗑️remove-space-checkpoint`, …). That is slice U3's in-flight localisation sweep and the repair is
   authored localisation keys, not a cast, so **I did not touch it.** It cost the first batch's runs
   2 and 3; by 10:28 the sweep had landed and the tree compiled clean.

## 7. Honest gaps

- **The fix is proven correct by construction, not by reproduction.** Nobody has reproduced the
  03:46 stall on demand: it is a two-instruction window between two OS threads that needs the
  `WorkerPool` worker to be descheduled inside it. Four green runs at load ≈ 10 say the fix did not
  regress the law; they do **not** say the race is gone — the s7a red itself came after the law had
  been green at 00:28 on identical code. What closes the case is repeated green coordinator runs of
  the whole suite under real fleet load, and the absence of
  `no server frame before 5s deadline at <phase>`. Until then this is a named, argued root with a
  minimal fix, not a demonstrated cure.
- **Four runs at load ≈ 10 are a weak load sample.** s7a ran at load ≈ 40 with 10-way nextest
  concurrency; my runs had the machine nearly to themselves. Rule 26 forbids me the full suite, so
  the load-matched evidence can only come from the coordinator.
- **The phase is still unproven.** §3 argues from the handler's structure that the stalled wait is
  the `Session` frame behind the unbounded bootstrap drain, and §1's arithmetic says it is one of
  the first waits. If a future red says `at observer withdrawal presence after the admin removal`
  instead, the root is in the presence-withdrawal broadcast, not in the sync hello, and §4 is wrong
  about *which* wait even though the defect it names is real either way.
- **A second, independent defect is named but NOT fixed.** `🌎️hub/🏗️bootstrap/🦀️.rs:4888-4928`,
  the `while hello_session.next_frame().await` bootstrap drain, is the **only** await in that
  handler with no bound — every other one (welcome send, binding gate, revalidate, frame send,
  client-frame handling) has a 2 s timeout. A hub whose sync-hello stalls therefore leaves a real
  browser peer holding a `Welcome` with no `Session`, forever, with no close frame and no reason
  code. That is a live-hub defect, not a test artifact. I left it alone deliberately: bounding it
  now would convert this exact hang into a close frame and **hide** whether §5.1 actually cured the
  root. It should be bounded (close `1013`, a named `bootstrap-unavailable` reason) in a follow-up,
  once one or more clean full-suite runs have confirmed §5.1.
- **No deterministic regression law for the lost wakeup.** The window has no seam, and the state's
  driver is private to its module. The honest shape would be to lift the release decision into a
  named pure predicate in `🛢️db/🔄️sync` and put a unit law on it (pre-fix = the `late_wake` argument
  ignored); I did not do it because it needs its own `semio-framework-os-db` test build and rule 26's
  economy, and it would still not exercise the real threads.
- **Nothing here was observed on a live hub.** No hub was booted, no serve started, no process
  killed. Rule 26 was honoured: the only `cargo nextest` was the single granted law, one at a time,
  in `⚡️cache/cargo/target-ht13`.
- The 30 s `DATABASE_SYNC_HELLO_DEADLINE_MS` recovery is what made every earlier capture of this
  family mute: it is longer than every hub and test bound, so the first expiring bound always
  reported "no frame". That asymmetry is worth revisiting on its own.

## 8. Files changed

| file | change | mine? |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` | **product root fix** — `drive_one`'s driver release re-reads `wake_requested`/`demand` after going `Idle`; the handoff's four operations are `SeqCst`; two new docstrings (`schedule`, `drive_one`) state the law | yes |
| `🌎️hub/🧪️tests/🔬️bin-unit/🦀️.rs` | `next_server_frame_at(ws, phase)` + wrapper; the failing law's seven waits labelled | yes |
| `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎨️example/🦀️.rs` | one `&` — unblocks every `semio-hub` build after a peer's `document_json() -> String` change | peer's breakage, my one-character unblock |
| `🐍️ht13-name-frame-phases.py` (ticket folder) | the region-guarded labelling script | yes |

Captures: `🗑️generated/ht13-check.txt`, `ht13-law-run1.txt`, `ht13-law-run2.txt`, `ht13-law-run3.txt`.

**needs coordinator hub rerun.**
