# HT15 — the hub suite's four inference reds at rerun s7h (21:41)

Source: `🗑️generated/coordinator-hub-nextest-full-s7h.txt` (binary 21:18, run under load ≈ 80–120,
**156.6 s** wall for the whole suite instead of the usual **31 s**), 327 run / 323 passed (1 leaky) / 4 failed.

## 1. The four panics, verbatim

| # | law | panic site | message |
|---|---|---|---|
| L1 | `inference::runtime::tests::gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply` | `💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:671` | `the sole retained driver reaches the paused publisher: Elapsed(())` |
| L2 | `inference::sqlite::tests::gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once` | `💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs:402:154` | `called Result::unwrap() on an Err value: Expired` |
| L3 | `inference::wal::tests::chain::inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch` | `💡️inference/🧾️wal/🧪️tests/⛓️chain/🦀️.rs:232` | `actual retained WAL "one-segment-three-commits" left: false right: true` |
| L4 | `inference::wal::tests::inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces` | `💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs:365:27` | `unexpected WAL outcome Expired` |

## 2. Prior-run timings — the same four laws, same binary lineage

| law | s7e | s7f | s7g (green, 16:38) | **s7h (red)** |
|---|---|---|---|---|
| L1 | 2.609 s | 1.452 s | 1.273 s | **17.900 s** |
| L2 | 1.255 s | 0.623 s | 0.713 s | **14.264 s** |
| L3 | 3.645 s | 1.789 s | 1.379 s | **11.003 s** |
| L4 | 4.825 s | 3.280 s | 1.648 s | **18.228 s** |
| whole suite | 32.5 s | 31.3 s | 31.4 s | **156.6 s** |

Nothing in the four panics names a hub, kernel or db symbol. Every one names a **clock**: `Elapsed`,
`Expired`, `Expired`, and an `Ok(Some(_))` that became `Err(Expired)`. The suite ran 5× slower and
these four laws ran 10–14× slower — they are the four laws in `inference` whose own runtime is
within one order of magnitude of a bound they carry.

## 3. Alone-runs at the current load

Measured with the s7h-lineage lib test binary rebuilt in the private dir
(`CARGO_TARGET_DIR=…/target-ht15`, `cargo nextest run -p semio-hub --lib --no-run`, 10 m 21 s,
99 warnings — `🗑️generated/ht15-build-1.txt`), each law run **alone**, three times, at
**load 68–77** — the same fleet load band that produced s7h. Capture:
`🗑️generated/ht15-alone-baseline.txt`.

| law | run 1 | run 2 | run 3 | verdict |
|---|---|---|---|---|
| L1 | ok 5.01 s | ok 5.08 s | ok 4.74 s | **green alone 3/3** |
| L2 | ok 2.94 s | ok 2.51 s | ok 2.33 s | **green alone 3/3** |
| L3 | ok 5.58 s | ok 5.00 s | ok 4.87 s | **green alone 3/3** |
| L4 | ok 9.72 s | ok 13.09 s | ok 11.74 s | **green alone 3/3** |

All four are case **(a)**: a wall-clock dependence, not a semantic regression from KN1's kernel
fixes, FP8's `HashProjection`, FL2's store work or the artifact-authority slices. The alone-runs
say more than "green": **L1 takes 5.0 s while carrying two 5 s bounds, and L4 takes 9.7–13.1 s while
every one of its traces carries a 2 s one.** These laws were already running at the edge of their own bounds
with nothing else on the machine but the fleet; s7h only pushed them over.

## 4. Root

Two distinct timers, one shape.

**(i) `InferenceOperationControlV1::new(2000, 64)` — L2, L3, L4.**
`🌎️hub/💡️inference/🦀️.rs:54` turns `lifetime_ms` into `deadline = Instant::now() + 2 s`, and
`checkpoint` (`:81`) answers `Expired` the moment the wall clock passes it. That clock runs while the
operation is *not* running — every millisecond the verification loses to the other 80 things on the
machine is charged against it. The three laws that died on it do not test expiry at all:

- L4's trace loop matches `verified | absent | stale | cancelled | bounds | invalid` and has **no
  `expired` arm** — `Expired` falls into `Err(other) => panic!("unexpected WAL outcome {other:?}")`
  (`🧾️wal/🧪️tests/🔬️unit/🦀️.rs:365` at s7h).
- L2 panics in the shared helper `committed_fixture_witness` (`:402`), which `.unwrap()`s a witness
  the fixture guarantees exists.
- L3 asserts `matches!(result, Ok(Some(_))) == case["accepted"]` (`⛓️chain/🦀️.rs:232`); `Err(Expired)`
  reads as "the chain was rejected", so a busy machine is indistinguishable from CRC tampering.

The laws that *do* test the deadline are `…proof_holds_its_admitted_slot…` (`🔬️unit`) and
`…chain_cancellation_retires_hashing…` (`⛓️chain`), whose fixtures carry `"expected": "expired"`
rows. Those keep `new(2000, 64)` — for them the clock is the subject, not the hazard.

**(ii) `tokio::time::timeout(5 s, …)` over two completion waits — L1.**
`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:664` wrapped `select! { publisher_entered.notified() , &mut retry }`
and `:680` wrapped `&mut retry`. Neither bound is the law: the law is the **ordering** —
"the fresh retry does not complete before the retained publisher pauses", then "the fresh retry
joins the one publication". The `select!` already states the first as an ordering fact and panics on
the wrong order; the 5 s wrapper only added a second, weaker verdict that a loaded scheduler decides.

## 5. Fixes

### 5.1 Product — a control whose bound is work, not the calendar

`🌎️hub/💡️inference/🦀️.rs`

| line | change |
|---|---|
| `:43` | `deadline: Option<Instant>` (was `Instant`) |
| `:56` | `new(lifetime_ms, work_limit)` keeps its exact contract and delegates the rest to `work_bounded` |
| `:64` | **new** `work_bounded(work_limit)` — one operation whose only bounds are its work-unit budget and explicit cancellation |
| `:80` | `checkpoint` answers `Expired` only when a deadline exists and has passed |
| `:101` | `interruption()` awaits cancellation alone when there is no deadline, instead of racing `sleep_until` |

This is not a test accommodation dressed as product: the hub already has five callers that mean
"this operation has no client lifetime" and are forced to spell it as a clock —
`🏃️runtime/🦀️.rs:3679, :3698, :3734, :3767, :3809` all pass `schema::JOB_MAX_LIFETIME_MS`. They are
left alone here (not my slice), but they are the shape `work_bounded` names.

Nothing is loosened. `new` still rejects `lifetime_ms == 0`, `> JOB_MAX_LIFETIME_MS`, `work_limit == 0`
and `work_limit > 65_536`; the work-limit refusal (`InferenceErrorV1::Bounds`) and the cancellation
refusal are untouched, and every production route still constructs its control with `new`.

### 5.2 The three WAL laws take the work-bounded control

| file | line | change |
|---|---|---|
| `🧾️wal/🧪️tests/🔬️unit/🦀️.rs` | `:347` | **L4**'s per-trace control → `work_bounded(64)` |
| `🧾️wal/🧪️tests/🔬️unit/🦀️.rs` | `:453` | **L2**'s `committed_fixture_witness` → `work_bounded(64)` |
| `🧾️wal/🧪️tests/⛓️chain/🦀️.rs` | `:233` | **L3**'s per-case verification → `work_bounded(64)` |
| `🧾️wal/🧪️tests/🔬️unit/🦀️.rs` | `:494` | `…rejects_hash_matched_noncanonical_or_wrong_actor_commands` — same defect, not yet red |
| `🧾️wal/🧪️tests/⛓️chain/🦀️.rs` | `:319` | `…retainedBoundaries` loop — same defect, not yet red |

`🔬️unit:513` and `⛓️chain:254` deliberately **keep** `new(2000, 64)`: their fixtures carry
`"expected": "expired"` rows and the deadline is what those two laws are about.

### 5.3 The wall clocks that were left holding the laws up

Removing the control's deadline moves the binding constraint onto the `tokio::time::timeout`
wrappers those loops carry (2 s over storage admission, 4 s over one verification, 2 s over a
`yield_now` convergence). Those are 2–4 s bounds over steps that already ran ~1.8 s each at s7h —
the same defect one layer out — so they go too, and what replaces them is the fact each step is
actually waiting for:

| file | line | before | after |
|---|---|---|---|
| `🔬️unit` | `:344` | `timeout(2 s, storage(…))` | `storage(…).await` |
| `🔬️unit` | `:360` | watcher spun inside `timeout(2 s, …)` | spins until the threshold **or the verification returns** (`returned: AtomicBool`, set at `:377`) |
| `🔬️unit` | `:376` | `timeout(4 s, verify)` | `verify.await` |
| `🔬️unit` | `:378` | — | **new** assert: the verification did not return before its interruption threshold, named by trace |
| `🔬️unit` | `:437` | `timeout(2 s, while verifier.active() != 0)` | the bare convergence loop |
| `🔬️unit` | `:491` | `timeout(2 s, storage_with_event)` / `timeout(4 s, verify)` | plain awaits |

The new `:378` assert is what keeps the watcher change honest: the old `timeout(2 s).unwrap()`
inside the spawned watcher turned "the verification finished before it reached the cancellation
threshold" into an anonymous `Elapsed` panic in a task; now that case ends the watcher cleanly and
fails the law by name.

### 5.4 L1 — the ordering fact replaces the 5 s clock

`🏃️runtime/🧪️tests/🔬️unit/🦀️.rs`

| line | change |
|---|---|
| `:529` | the law arms `RuntimeLawHangWatchdogV1` — the OS-thread guard this module already owns — as its **first** statement, so it drops last |
| `:665` | `timeout(5 s, select! { publisher_entered , &mut retry })` → the bare `select!`. The law's claim is the ordering, and the `retry` arm still panics on the wrong order |
| `:678` | `timeout(5 s, &mut retry)` → `(&mut retry).await` |

### 5.5 What still names a wedge

Deleting a law's clock must not turn a genuine hang into an anonymous 300 s nextest kill, so both
files keep an OS-thread guard that watches from **outside** the runtime (the runtime module's own
`RuntimeLawHangWatchdogV1` docstring explains why an in-runtime `tokio::time::timeout` cannot):

- `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:529` — the existing watchdog, armed for L1.
- `🧾️wal/🧪️tests/🔬️unit/🦀️.rs:11–57` — **new** `WalLawWedgeGuardV1`, the same pattern for the WAL
  laws, used by L4, L3 (through `mod chain`) and the hostile-command law.

Their budgets (600 s) decide nothing about any law — they are ~50× the slowest measured run and
~2000× a per-step bound — they only decide how long a wedged suite waits before it prints which law
and which phase wedged, and aborts.

## 6. The "1 leaky"

`semio-hub artifact_authority::tests::authority_diagnostics_are_utf8_safe_and_fixed_bounded_before_retention`
(`🌎️hub/🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs:482`). **It leaks nothing, and its teardown has
nothing to fix** — measured, not argued:

- The whole body is three lines of pure string arithmetic: `bounded_message("é".repeat(MAX))`, a
  length assert and a `is_char_boundary` assert. It is a plain `#[test]`, not `#[tokio::test]`.
- `grep -n "spawn|Command::new|worker_pool|thread::"` over that entire test module: **no hits**. The
  crate has no `#[ctor]`, no `lazy_static`, no global pool (`grep` over `🌎️hub/`: no hits).
- nextest's `leaky` verdict is not "a process was left behind" observed directly — it is "the
  process's stdout/stderr pipe was still open `leak-timeout` after the process exited".
  `.config/nextest.toml` sets only `slow-timeout` per profile, so `leak-timeout` is the **default
  100 ms**.
- The test took **0.293 s** in s7h against **0.039 s** in s7g — 7.5× — and it is the **first and only
  LEAK in eight full runs** (`grep -c "LEAK \["` over `…-s7a`…`-s7h`: 0,0,0,0,0,0,0,**1**).

So the leak is the third face of the same fact as §4: a 100 ms harness grace period measured against
a machine that was running everything 5–11× slow. No code change is warranted and none was made. If
the coordinator wants it to stop appearing under fleet load, the knob is `leak-timeout` in
`.config/nextest.toml` — a repo-wide harness setting that belongs to whoever owns that file, not to
this law.

## 7. Verification

The account session limit cut this slice at ~22:57 on 2026-09-21 (mid `cargo check`); everything
below was re-run from 01:28 on 2026-09-22. The edits themselves survived — auto-commit took them, so
`git diff --stat 🌎️hub` is empty and `git log -- 🌎️hub/💡️inference/🦀️.rs` carries them.

| # | command | result | capture |
|---|---|---|---|
| V1 | `CARGO_TARGET_DIR=…/target-ht15 CARGO_INCREMENTAL=0 cargo check -p semio-hub --all-targets --keep-going` (01:28 → 01:35, 7 m 29 s, 204 units) | **Finished, 0 errors, 351 warnings** — lib, `bin "os-hub"`, lib test and bin test targets all expanded | `🗑️generated/ht15-check-3.txt` |
| V2 | `cargo nextest run -p semio-hub --lib --no-run` (01:36 → 01:42, 5 m 56 s) | **Finished, 104 warnings** (lib test) | `🗑️generated/ht15-build-2.txt` |
| V3 | the four laws, each alone, 3× at load 37–40 | **12/12 ok** | `🗑️generated/ht15-alone-after-fix.txt` |
| V4 | the three neighbour laws — the one I also converted and the **two that keep their 2 s deadline on purpose** — each alone, 3× | **9/9 ok**: `…rejects_hash_matched_noncanonical_or_wrong_actor_commands` 0.40–0.56 s (converted), `…chain::quick::…cancellation_retires_hashing…` 2.48–2.85 s (deadline kept), `…quick::…dropped_caller_cancels_and_finishes_retained_replay_before_release` 2.30–2.50 s (deadline kept) | same capture |
| V5 | 8 concurrent copies of the four laws, post-fix | **32/32 ok**, 7.5–7.8 s per worker | `🗑️generated/ht15-stress-after-fix.txt` |
| V6 | the identical 8-way and 20-way stress on the **pre-fix** binary (21:37, s7h lineage) | **green too** — 11.1–11.4 s (8-way), 14.4–16.3 s (20-way) | `🗑️generated/ht15-stress-before-fix.txt` |

V3, per law, against §3's pre-fix alone-runs:

| law | pre-fix alone (load 68–77) | post-fix alone (load 37–40) |
|---|---|---|
| L1 | 5.01 / 5.08 / 4.74 s | **1.05 / 0.96 / 1.03 s** |
| L2 | 2.94 / 2.51 / 2.33 s | **0.51 / 0.32 / 0.33 s** |
| L3 | 5.58 / 5.00 / 4.87 s | **1.37 / 1.30 / 1.17 s** |
| L4 | 9.72 / 13.09 / 11.74 s | **1.69 / 1.81 / 1.73 s** |

### What V5/V6 do and do not prove

They are a **controlled A/B on one machine state**: the same four laws, the same concurrency, two
binaries a few hours apart. Read honestly:

- **They did not reproduce the red.** 20 concurrent copies at load 27–48 is not s7h's condition
  (nextest's own parallelism over 327 laws *plus* a fleet at 80–120 with a dozen `rustc` resident).
  I did not push past 20 — that is a shared machine with peers on it. **So the fix is not proven by
  a reproduced failure; it is proven by the panics' own text (§4) plus the removal of every bound
  they named.**
- **They do give a real margin number.** At identical concurrency the fixed laws run **32 % faster**
  (7.6 s vs 11.2 s at 8-way for the same four laws). Dropping the per-step `tokio::time::timeout`
  wrappers removes a timer registration per step and the `timeout`-wrapped `yield_now` spins, so the
  laws are not merely immune to the clock, they are further from it.
- **The decisive check is structural, and it is exact**: after the fix, L2, L3 and L4 construct no
  `Instant` deadline at all (`work_bounded`), and L1 contains no `tokio::time::timeout`. There is no
  wall-clock value left in any of the four that a loaded machine can push past — the only remaining
  clocks are the two OS-thread wedge guards at 600 s, which abort with a name and cannot produce a
  law verdict.

**A coordinator hub rerun at real fleet load is the only measurement that closes this.**

## 8. Files changed

| file | change |
|---|---|
| `🌎️hub/💡️inference/🦀️.rs` | `InferenceOperationControlV1`: optional deadline + `work_bounded(work_limit)`; `checkpoint` and `interruption` honour the absence of a deadline |
| `🌎️hub/💡️inference/🧾️wal/🧪️tests/🔬️unit/🦀️.rs` | new `WalLawWedgeGuardV1`; L4, `committed_fixture_witness` (L2's panic site) and the hostile-command law take the work-bounded control and lose their per-step wall clocks; new threshold assert |
| `🌎️hub/💡️inference/🧾️wal/🧪️tests/⛓️chain/🦀️.rs` | L3 and the retained-boundary loop take the work-bounded control; L3 arms the wedge guard |
| `🌎️hub/💡️inference/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | L1 arms the module's existing hang watchdog and drops its two 5 s completion clocks |

No law was deleted, `#[ignore]`d, or had an expectation loosened. No timeout was lengthened as a
cure: the two laws whose subject **is** the deadline keep the exact `new(2000, 64)` they had.

## 9. Honest gaps

- **The four laws are re-measured alone, not in the loaded suite.** A suite rerun is the only thing
  that proves the fix at s7h's concurrency; that is the coordinator's run.
- **L1 keeps one product clock I did not touch**: `DatabaseShutdownControl::for_timeout(5 s)` at
  `🏃️runtime/🧪️tests/🔬️unit/🦀️.rs:~834`, in the law's teardown. It did not fire at s7h, it is a
  product shutdown budget rather than a law bound, and changing it is a different slice.
- **Two laws still carry second-scale clocks by design** —
  `inference::wal::tests::quick::inference_wal_proof_dropped_caller_cancels_and_finishes_retained_replay_before_release`
  and `inference::wal::tests::chain::quick::inference_wal_chain_cancellation_retires_hashing_and_compacted_suffix_is_not_a_genesis_proof`.
  Their `"deadline"` rows need the 2 s control, but their
  `"cancel"` rows wrap the future in `timeout(1 s)` and their convergence loops in `timeout(2 s)`;
  those three bounds are the same hazard as §4(i) and would be the next to redden under a heavier
  fleet. Not taken here (they were green at s7h and the deadline cannot simply be removed from them).
- **The leaky verdict is diagnosed, not reproduced.** I did not run the suite under a synthetic
  100 ms-pipe-close stall; the case rests on the test body, the absent spawn, the 0.039 → 0.293 s
  timing and the 0-in-7 prior runs.
- **cargo deadlock at 22:38 on 2026-09-21** (rule 27(b)): my `cargo check -p semio-hub --lib` sat
  23 min with **no child process at all** and `sample 98849 1` in
  `prebuild_lock_exclusive → flock` while 6 unrelated `rustc` ran — so rule 23(a)'s "no rustc
  anywhere" test would have missed it. Killed by pid and rerun; the rerun was cut minutes later by
  the account session limit, and V1 above is the clean re-run at 01:28.
- The first `cargo check -p semio-hub --all-targets` at 22:06 died on **peer churn, not hub code**: 33
  errors, all in `semio-s-plugin-stdio` and `semio-s-artifact-gis-gismap`, headed by
  `can't find crate for semio_framework_plugin` (a concurrently evicted rlib) which then collapses
  the `labels!`-generated `Gis2dPlayLabels` and everything importing it. Zero errors named any
  `🌎️hub` path. Capture `🗑️generated/ht15-check-1.txt`. **Superseded by V1**, which compiled all 204
  units clean — so that reading is now confirmed rather than inferred.
