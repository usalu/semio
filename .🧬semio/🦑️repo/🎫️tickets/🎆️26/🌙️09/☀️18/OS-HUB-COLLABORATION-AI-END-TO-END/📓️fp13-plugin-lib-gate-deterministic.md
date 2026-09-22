# FP13 — `⚖️gate🔌️plugin🦀️lib` deterministic (827/0 on every run)

Slice FP13, 2026-09-22 (session 8). Continues FP12 (`📓️fp12-plugin-lib-zero-red.md`, 825/0 in 8 of
16 serial runs). Method: private `CARGO_TARGET_DIR=…/⚡️cache/cargo/target-fp13`, shared build-dir,
`CARGO_INCREMENTAL=0`, `CARGO_BUILD_JOBS=4`, one cargo at a time; the lib unittests binary is copied
out and driven directly with `--test-threads=1`. Every number is read from a `🗑️generated/fp13-*`
capture.

## 0. Coordinator notes

- **No `proposed-flow-retained.diff.md` exists.** Checked at 17:33 under
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/` (only
  `📓️flow.md`, 21 416 B, 2026-09-21 16:33) and under `.🧬semio/🦑️repo/⚡️cache/play-fleet/`
  (no match). Nothing from the peer session was applied, and no file under `✏️s/🔌️plugins/🌊️flow/**`
  or `🌊️flow/**` was touched by this slice.
- **Second item (coordinator, 17:5x): the typed-operation publication stall witness.** Landed as
  §2b, with a measurement that CHANGES the peer's diagnosis — read §2b before acting on it. The
  peer memo is `…/☀️19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP/📓️block-puzzle.md` §9.11; I read it as
  data, wrote the fix myself against the current code, and touched nothing of theirs.
- The fix in §2 is in `🔌️plugin/🦀️.rs` **product** code and in `🧵️job/🦀️.rs`; both are files peers
  edit continuously (`🔌️plugin/🦀️.rs` moved by +59 lines between two greps five minutes apart), so
  every edit was made through a unique textual anchor, never a line number.

## 1. Root cause of `instance busy or poisoned`

**It is not a leaked instance, not a poisoned lock and not a global counter. It is a live
cross-thread race between the host lane and the process-wide maintenance worker pool, and it is a
product defect, not a test artefact.**

The chain, read at HEAD this slice (anchors, not line numbers — a peer moved the file twice while I
read it):

1. `RuntimeAppCell::instance` is a `std::sync::Mutex<AppInstance<PA>>`.
2. `plugin_step_live_cleanup` → `plugin_step_live_cleanup_cell` submits a maintenance job to
   `runtime_close_pool()`, which is
   `semio_framework_async::process_worker_pool(WorkerPoolConfig::new(ProcessKind::InteractiveNative, cores))`
   — **one pool per PROCESS, with real OS threads**, shared by every law in the lib-test binary
   however few test threads `--test-threads` allows.
3. On one of those threads, `RuntimeLiveCleanupJob::step` takes `cell.instance.try_lock()` and holds
   the guard across the whole `instance.app.maintenance_step(1, RUNTIME_CLOSE_BYTES_PER_STEP)`.
   `RuntimeCloseCleanupJob::step` and `runtime_close_retire_cell` do the same on the close lane.
4. Meanwhile the host lane calls `plugin_acknowledge_typed_operation_result` →
   `with_instances_mut` → `find_instance` → `cell.instance.try_lock()`, which is a **try**-lock
   whose `Err` — of either kind — became
   `plugin_internal_fault("instance busy or poisoned: {instance_id}")`.

So a host crossing was refused with a `plugin.internal` fault purely because a background
maintenance turn happened to hold the app at that microsecond. Nothing retries it: in the suite it
is `.unwrap()` on a law, and in the product it is a user's acknowledged result page turning into a
window fault.

**The two laws FP12 and FP10 saw failing are exactly this, and nothing else.** Both drive
`plugin_step_live_cleanup(&runtime)` and then `plugin_acknowledge_typed_operation_result(...)`
`.unwrap()` in the same loop body:

| law | site of the panic | fault in FP12's capture |
|---|---|---|
| `retained_operation_continues_after_command_admission_until_publication_and_retirement` | the `plugin_acknowledge_typed_operation_result(&runtime, page.token).await.unwrap()` inside the page loop | `instance busy or poisoned: 7` (`fp12-round6-serial.txt:5`, `fp12-round7-serial.txt:5`) |
| `concurrent_typed_operations_hand_every_presentable_page_to_one_turn` | the identical ACK line in its own page loop | same class (FP12 §1) |
| `an_abandoned_ingress_owner_never_answers_the_command_the_host_is_driving` (FP10 §6) | a host crossing on instance 4024 | `instance busy or poisoned: 4024` |

The asymmetry is the bug: every lane that yields already handles contention without faulting
(`RuntimeLiveCleanupJob::step` sets `contended` and yields; `pending_typed_operation_instance` and
`plugin_continue_typed_operations` report `TypedOperationScan { contended: true }`), while the ONE
lane that cannot yield — the host crossing — was the one that refused.

## 2. Landed fix — the host lane outranks the background lanes

All of it is in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, and it is one protocol,
not a retry. `RuntimeAppCell` gains two counters and the two ways of taking the app become two named
guards:

- `host_crossings: AtomicU32` — how many HOST crossings want or hold the app.
- `background_holds: AtomicU32` — how many BACKGROUND turns want or hold it, raised BEFORE the
  try-lock and lowered after the release.
- `RuntimeAppCell::host_instance() -> Option<HostInstanceGuard<'_, PA>>` registers the crossing
  first (which stops any new background turn dead), then try-locks; on `WouldBlock` it waits out a
  background turn — and ONLY a background turn (`background_holds != 0`) — with `yield_now`,
  bounded by `RUNTIME_HOST_CONTENTION_YIELDS = 4_096`. `None` keeps the exact meaning the bare
  try-lock had at a host site: the app is held by something that will not release for this crossing
  — the host lane itself (reentrancy, or a submitted media export that holds the app across its
  awaits) or a poisoned mutex.
- `RuntimeAppCell::background_instance() -> Result<BackgroundInstanceGuard<'_, PA>, RuntimeInstanceRefusal>`
  never waits: a live host crossing, or a taken lock, is `Contended`, and the lane's own `Yield` is
  what a contended turn already costs.
- **Both guards release the mutex BEFORE unregistering** (`Option<MutexGuard>` + `take()` in
  `Drop`, because `Drop::drop` runs before field drops). The other order leaves a window in which a
  waiting host crossing reads zero background holds while the mutex is still taken and refuses the
  very turn it was waiting out — that window is the whole bug reintroduced, one layer down.

Call sites moved (no message changed, so the React classifier's
`isPluginInstanceBusyFaultV1("plugin.internal: instance busy or poisoned: N")` twin still matches):

- host: `find_instance` (its return type is now `HostInstanceGuard<'a, PA>`; every caller reaches
  `instance.app` / `instance.surface_contexts` through `DerefMut`), `plugin_handle_action`, the four
  media entries (`plugin_submit_media_export` + 3), and the `plugin_exchange` action branch — 7
  sites, each still `.ok_or_else(|| …the same fault…)`.
- background: `RuntimeLiveCleanupJob::step`, `RuntimeCloseCleanupJob::step`,
  `runtime_close_retire_cell` — 3 sites, each mapping `Contended` onto the `Yield`/`Ready` it
  already returned and `Poisoned` onto the fault it already returned.
- The six `#[expect(clippy::await_holding_lock, reason = "…competing production instance access
  uses try_lock and refuses or yields instead of waiting.")]` attributes are gone: the reason they
  carried is no longer true (a host crossing now waits out a background turn), and the held type is
  no longer a bare `MutexGuard`. The truth they recorded moved into `HostInstanceGuard`'s own
  docstring, where it belongs.
- `plugin_continue_typed_operations` and `pending_typed_operation_instance` were deliberately LEFT
  on the bare try-lock: they already answer contention with `TypedOperationScan { contended: true }`
  and their callers loop on it, and laws read that flag.

**New law** (`…/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`):
`a_host_crossing_outranks_every_background_hold_on_the_same_app` — a scoped thread takes
`background_instance()` and holds it until it observes `host_crossings != 0`; the main thread then
asserts `background_holds == 1`, that `host_instance()` returns `Some` (it waits the holder out
instead of being refused), that while the crossing is live `background_instance()` answers
`Err(Contended)`, and that the app is free to both lanes again afterwards. It fails deterministically
against the pre-fix code, which is the point.

## 2b. Second item — the typed-operation publication stall witness

**Landed.** `MountedTypedCommandFullOperation` gains `publication_progress: u64` and
`publication_checkpoint: Option<store::ArtifactStoreOneItemCheckpoint>`;
`note_publication_checkpoint` raises the progress ONLY when the store's own checkpoint (cursor,
completed items, completed bytes, digest) differs from the previous one, and it is called from the
`ArtifactStoreOneItemAdvance::Progress(checkpoint)` arm of
`advance_typed_operation_publication_unit` — the arm that previously discarded the checkpoint.
`typed_operation_stall_witness` now answers FOUR facts `(operation_id, stage, flags,
publication_progress)`, `typed_operation_stall_streak` compares all four, and the one-`u64` key it
remembers between turns folds the progress term through `TYPED_OPERATION_STALL_PROGRESS_MIX`. No
ceiling was raised: `TYPED_OPERATION_STALL_FAULT_CEILING` is still 4 096 and an owner that cannot
advance still leaves the term constant and still faults. The witness's docstring now carries the
puzzle5d measurement and why the progress term is the fourth fact.

**Law landed:** `a_many_mutation_publication_folds_every_mutation_and_is_never_terminated_as_stalled`
(contract test file) — one `BulkEdit { rows: 4_400 }` edit (new `TestCommand` variant, its
`test_command_id`, `test_app_reduce`, `KeyedTestJob::step` and `KeyedTestCommandDisposer` arms),
driven with `TypedOperationGrant::UNIT` so the law's own counter IS the publication-unit count. It
asserts no `Fault` lane page, a terminal page, more than the guard's 64-unit witness floor, all
4 400 mutations in the document, and a clean destroy.

### The measurement that contradicts the memo's model — **read this before acting on §9.11**

The law was first written exactly as the coordinator asked (fold > 4 096 units in ONE publication).
It cannot be written that way, and the reason is a product ceiling, not a fixture limit:

| measurement | result | capture |
|---|---|---|
| 4 400 `SetCount` mutations in one edit, `TypedOperationGrant::UNIT` | the whole publication spends **126 publication units** and completes | `fp13-round3/4-serial.txt`, `fp13-control-no-progress-term.txt` (the 21:12 reading) |
| 150 000 mutations in one edit | **refused at admission**: `plugin.internal␟batched preparation footprint exceeds its fixed item or byte capacity` | `fp13-control-no-progress-term.txt` (the 21:15 reading) |
| the ceiling behind that refusal | `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES = 1 048 576` and `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS = 65 536` (`🏪️store/🦀️.rs`, `ArtifactStoreOneItemFootprint::is_admissible`) | — |
| one publication unit's own grant | `ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: TYPED_OPERATION_RESULT_PAGE_BYTES = 4 096 }` | — |

**So ONE batched document publication can spend at most ≈ 1 MiB / 4 KiB ≈ 256 publication units.**
It is structurally impossible for a single batched document publication to reach 4 096 units, so
puzzle5d's ≈ 19 000 units do NOT come from that arm.

**And the control run says the same thing from the other side.** With the progress term removed
(`Some((operation_id, stage, flags, 0))`, rebuilt at 21:08), the 4 400-mutation law still PASSES,
and with `SEMIO_RUNTIME_DIAGNOSTICS=1` the guard's own `[DEBUG] typed-operation … advanced N units
with no change` line never prints at all (`fp13-control-streak.txt`) — the streak never even
reaches its 64-unit floor, because for a healthy durable-store fold the first three facts do NOT
stay constant: result pages are minted and `result_page`/`publication` flags move every few units.

**What that leaves.** The landed progress term is correct and strictly strengthens the witness — a
store checkpoint that moves can never again be read as a stall — but **this slice cannot claim it
is the cure for puzzle5d**, and it does not cover the child-group lane: the witness flags have no
bit for `pending_child_publication` at all, and `publish_mounted_typed_child_operation_unit`'s
phases (`Ready`/`Dispatching`/`Committed`/`Acknowledged`/`Closing`) move without touching any of
the four facts. puzzle5d's `capsule-dream` is 2 880 `create_part` + 2 865 `connect_grips`, i.e.
CHILD emits, so the child-group lane is the prime suspect for the 19 000 units. The next step is an
instrumented run of the real switch that prints which arm of
`advance_typed_operation_publication_unit` each unit takes; whoever does it should add that arm's
own monotone term to the witness exactly as §2b added the store's.

## 3. Ten consecutive serial runs — **827 / 0, twenty times**

`.🧬semio/🦑️repo/⚡️cache/cargo/target-fp13/bin/fp13-lib --test-threads=1`, the binary linked at
21:04 from the final source state, ten runs back to back (`📜️fp13-rounds.sh 11 20`):

| round | result | wall | load at start |
|---|---|---|---|
| 11 | **827 passed / 0 failed** | 21.75 s | 29.27 |
| 12 | **827 / 0** | 22.57 s | 32.22 |
| 13 | **827 / 0** | 21.37 s | 38.39 |
| 14 | **827 / 0** | 25.12 s | 43.12 |
| 15 | **827 / 0** | 25.21 s | 44.64 |
| 16 | **827 / 0** | 20.15 s | 41.85 |
| 17 | **827 / 0** | 18.26 s | 49.92 |
| 18 | **827 / 0** | 13.88 s | 45.74 |
| 19 | **827 / 0** | 13.43 s | 41.95 |
| 20 | **827 / 0** | 15.05 s | 35.64 |

Those ten ran on the 21:04 binary. §2b's law was then re-expressed (its grant changed to
`TypedOperationGrant::UNIT` and its row count back to 4 400), so the ten runs were REPEATED on the
final binary linked at 21:18 — `📜️fp13-rounds.sh 21 30`, rounds 21…30, **827 / 0 every time**
(13.04–13.30 s, load 9.6–13.4), captures `🗑️generated/fp13-round{21…30}-serial.txt`. **Twenty
consecutive green serial runs in total, ten of them on the exact source state this report
describes.**

Captures `🗑️generated/fp13-round{11…20}-serial.txt`. **827 = FP12's 825 + the two laws this slice
added.** The runs were made under load 29–50, i.e. inside and above the 21–60 band in which FP12
measured the flake in 5 of 16 runs; the two laws that carried it
(`retained_operation_continues_after_command_admission_until_publication_and_retirement`,
`concurrent_typed_operations_hand_every_presentable_page_to_one_turn`) passed in all ten.

Development rounds before that, on earlier binaries, for the record: round 1/2 825 + my 2 new laws
red (teardown and fixture, `fp13-round{1,2}-serial.txt`), round 3 826/1, round 4 826/1, round 5
**827/0** (`fp13-round{3,4,5}-serial.txt`). Each red was in a law THIS slice wrote; no pre-existing
law failed in any round of this slice.

## 4. `semio-framework-job` process-global byte counter

**The counter KN3 named is `JOB_PAYLOAD_PROCESS_OWNED_BYTES`** (`🧰️framework/🔨️modules/🧵️job/🦀️.rs`),
and the law that forced `--test-threads=1` is
`retained_payload_max_plus_one_zero_grant_nested_and_exact_close_are_owned`
(`🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs`), which snapshotted the static before its ladder and
demanded the same absolute value afterwards (KN3 §6.1: 27/1 in parallel, 28/0 serial).

**It is NOT made per-test or per-app, and that is the answer, not a dodge.** `JOB_PAYLOAD_PROCESS_BYTES`
(64 MiB) is a real ceiling over the whole process — it is what stops N concurrent operations from
each taking their own 16 MiB operation budget until the process dies. Per-test or per-app budgets
would multiply the ceiling by the number of owners and delete the invariant.

What was wrong is the *observable*, not the budget: a static that every concurrent operation moves
cannot be sampled by one owner as an absolute. `JobPayloadOperationLedger::reserve` and
`::release` are the ONLY two mutators of the static (verified by grep: after this change the
identifier appears exactly three times outside its own docstring — the definition and those two),
and each moves the static and that ledger's own `bytes` by the same page in the same call. So the
static is exactly the sum of every live ledger's share, and a ledger reading zero has returned
everything it ever took.

Landed:
- `JobPayloadOperationLedger::process_share_bytes()` — this ledger's own share of the process
  budget, with the docstring that states why it, and not the static's absolute value, is what an
  owner may assert.
- The static gains the docstring saying it is a process ceiling, stays private, and is sampled by
  nobody.
- The law now opens with `ledger.process_share_bytes() == 0` (a fresh ledger holds none of the
  budget) and closes with `ledger.process_share_bytes() == 0` after the exact close ladder. Every
  other clause is untouched; nothing was `#[ignore]`d, skipped or loosened, and the new clauses hold
  under any number of test threads because they read only this operation's own accounting.

Measured: `cargo test -p semio-framework-job --lib` with DEFAULT (parallel) threads, six
consecutive runs at 17:47–17:48: **28 passed / 0 failed** every time
(`fp13-job-parallel-1.txt`, `fp13-job-parallel-rounds.txt`; round 4's line was lost by the capture
script's grep, the other five are in the file). KN3's serial figure was 28/0 and its parallel figure
27/1, so the suite is now deterministic without `--test-threads=1` and lost no law.

## 5. Files changed

Product code (3 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` — §2's whole priority protocol
  (`RuntimeInstanceRefusal`, `RUNTIME_HOST_CONTENTION_YIELDS`, `HostInstanceGuard`,
  `BackgroundInstanceGuard`, `RuntimeAppCell::{host_crossings, background_holds, host_instance,
  background_instance}`, 7 host call sites, 3 background call sites, 6 stale
  `#[expect(clippy::await_holding_lock)]` attributes removed); §2b's witness
  (`MountedTypedCommandFullOperation::{publication_progress, publication_checkpoint,
  note_publication_checkpoint}`, the `Progress(checkpoint)` arm, the 4-fact
  `typed_operation_stall_witness` / `typed_operation_stall_streak`,
  `TYPED_OPERATION_STALL_PROGRESS_MIX`)
- `🧰️framework/🔨️modules/🧵️job/🦀️.rs` — §4's `JobPayloadOperationLedger::process_share_bytes` and
  the docstring on `JOB_PAYLOAD_PROCESS_OWNED_BYTES`

Laws / fixtures (3 files):
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`
  — the two new laws, the `TestCommand::BulkEdit` variant and its four fixture arms
  (`test_command_id`, `test_app_reduce`, `KeyedTestJob::step`, `KeyedTestCommandDisposer`'s
  `close_step` + `terminal_is_empty`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🦀️.rs`
  — four `MountedTypedCommandFullOperation` literals carry the two new fields
- `🧰️framework/🔨️modules/🧵️job/🧪️tests/🔬️retained-ownership/🦀️.rs` — §4's two clauses

Ticket folder: `📜️fp13-build.sh`, `📜️fp13-rounds.sh`, this report; captures
`🗑️generated/fp13-*` (`check-errors-{1,2}`, `link-err-*`, `bin-path-*`, `round{1…6,11…30}-serial`,
`job-parallel-1`, `job-parallel-rounds`, `job-parallel-final`, `job-fail`,
`control-no-progress-term`, `control-streak`, `dependent-checks`, `dependent-native`).

### Dependent re-checks (product code changed this slice, so all three were run AFTER the final state)

| check | result | capture |
|---|---|---|
| `cargo check -p semio-framework-plugin --lib` (plain, no `cfg(test)`) | **green** — real `Checking` line, `Finished dev in 6.66s`, 59 warnings, 21:20 | `fp13-dependent-checks.txt` |
| `cargo check -p semio-framework-plugin --lib --target wasm32-wasip2 --features component-guest` | **green** — real `Checking` line, `Finished dev in 7.00s`, 61 warnings, 21:21 (this is the check that matters for `std::thread::yield_now` in `host_instance`) | `fp13-dependent-checks.txt` |
| `cargo check -p semio-s-plugin-note --lib` (native dependent plugin) | **green**, `Finished dev in 42.60s`, 21:22 | `fp13-dependent-native.txt` |
| `cargo test -p semio-framework-job --lib` (default threads) ×6 | **28 / 0** six times, 21:21 | `fp13-job-parallel-final.txt` |

The lib-test check itself reports **370 warnings** (FP12's baseline was 364; the six added are
pre-existing lints in files peers touched, none of them names anything this slice wrote — grepped
for every new identifier, zero hits).

## 6. Honest gaps

- **The gate is deterministic by measurement, not by proof.** Twenty consecutive serial runs are
  827/0 (ten of them on the final source state, under load 9–50), and the mechanism behind FP12's
  flake is named, fixed and covered by its own law. What is NOT claimed: that no other
  process-wide state in the binary can ever produce an order-dependent failure. FP11 §3.7's
  inventory (`TASK_RECORDS`, `TASK_RESUMES`, `TASK_EXECUTOR`, `REACTOR_EXECUTOR`, `REGISTRY`,
  `COMMAND_INGRESS`, `INSTANCE_METADATA`, `ARMED_TIMERS`, `REACTOR_CLOSES`, `PATCHES`, `PRESENCE`)
  is untouched, and so is the one concrete leak it names — `drain_task_resumes`
  (`⚛️reactor/🔄️turn/🦀️.rs`) still pushes a resume back onto `TASK_RESUMES` and `continue`s
  whenever `native_close_key` errs, so a resume for an instance with no live lifetime is retried by
  every later turn for ever. I left it alone deliberately: it was not the cause, the ten runs are
  green without touching it, and retiring such a resume is a semantic decision (a resume queued
  before a lifetime is acknowledged would be dropped) that needs its own slice.
- **The default-threads plugin suite was not measured.** The gate and every run in §3 use
  `--test-threads=1`, as FP5–FP12 did. §2's fix makes a host crossing robust against the
  background pool, not against a second TEST thread driving the same runtime.
- **§2's bounded wait can still refuse.** `host_instance` gives up after
  `RUNTIME_HOST_CONTENTION_YIELDS = 4 096` scheduler yields and returns the same refusal as before.
  That backstop was never reached in any run of this slice (no law failed on `instance busy or
  poisoned` once), but it is a ceiling, so the refusal is not structurally impossible — only
  impossible while every background lane honours its own `RUNTIME_CLOSE_INNER_GRANT_US` step
  budget.
- **`plugin_continue_typed_operations` and `pending_typed_operation_instance` were deliberately
  left on the bare try-lock** (§2). They answer contention with `TypedOperationScan { contended:
  true }` rather than a fault, and laws read that flag (`spent += u64::from(scan.runnable ||
  !scan.contended)` and its ceiling assert in the pacing law), so moving them onto the host
  protocol would change what those laws measure. They are not a fault source.
- **The six `#[expect(clippy::await_holding_lock)]` attributes were removed, and clippy was NOT
  re-run.** The held type is now `HostInstanceGuard`, not a bare `MutexGuard`, so clippy's
  `await_holding_lock` almost certainly no longer fires there — but "almost certainly" is the
  honest word: I did not spend a clippy build to confirm it, and if it does still fire the six
  sites will each report one warning until someone restores the attributes with an accurate reason.
- **§2b is landed but is NOT proven to cure puzzle5d, and the peer memo's model is wrong in one
  measurable way** (§2b, whole section): a single batched document publication is capped at ≈ 256
  publication units by `ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES`, so it cannot be the arm that spends
  19 000, and with the progress term REMOVED the 4 400-mutation law still passes and the guard's
  streak never even reaches its 64-unit floor. The coordinator's "add a law that folds > 4 096
  units in one publication" is therefore NOT delivered as asked — it is not constructible — and the
  law that did land proves the weaker, true statement. The child-group lane is named as the prime
  suspect with the reason; nobody should close the puzzle5d item on the strength of this slice.
- **No ceiling was raised and no law was `#[ignore]`d, deleted or loosened.** Two laws were added.
  The only law text changed outside my own is §4's two clauses in `🔬️retained-ownership`, which
  are strictly stronger (they hold at any thread count).
- **§4 keeps the process ceiling process-wide on purpose** and makes the OBSERVABLE per-owner. That
  is a deliberate reading of "make it per-test or per-app": a per-app payload budget would multiply
  the 64 MiB process ceiling by the number of apps and delete the invariant.
- **One transient `cargo test -p semio-framework-job --lib` failure at 21:21** printed
  `error: test failed` with no test-result line (`fp13-job-fail.txt` holds the immediate re-run,
  which was 28/0, as were the six runs after it). It happened while a plain `cargo check` of
  another crate shared the build dir in the same shell command; I could not reproduce it in seven
  subsequent runs and I am not claiming to know what it was.
- **Nothing under `✏️s/🔌️plugins/🌊️flow/**` or `🌊️flow/**` was touched**, and no
  `proposed-flow-retained.diff.md` exists to apply (§0). Flow's `retained::*` lane was not re-run
  this slice — FP12's 8/6 reading stands unverified by me.
- Native only. No wasm component build, no `activate`, no serve, no browser: every claim here is a
  native law run or a native/wasm32 `cargo check`.
