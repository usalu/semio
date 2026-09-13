# 🔒️ Lib Suite Deadlock & Delivery — 2026-09-13 (lane: lib-suite-deadlock-delivery)

Acts on `📓️gates-green-2026-09-13.md` §4 and §5. Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
`repo` MCP did not connect this session (`-32602 invalid initialize params`), `semio` MCP
`CONNECTION_CLOSED` — no ticket lifecycle call was made, `📓️status.md` and `🎫️ticket.json` were not
edited, no modifying git command was run, no dev server was started, stopped or restaged, and
nothing under `🗑️generated/` that this lane did not create was read-modified or deleted. This lane's
own logs live in `🗑️generated/lib-suite/`.

## TL;DR

| item | before | after |
|---|---|---|
| the `--lib` "deadlock" | reported as one test taking the serial lock and never releasing it | **that is not what it is** — measured with `sample(1)`: the suite CONVOYS on the serial lock and the framework's 30 s fixture deadlines expire inside the convoy (§1) |
| the accused test, alone | "never terminates" | `an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain` **passes in 27.35 s**, `--test-threads=1`, unskipped (§1.1) |
| lock discipline | **three** process-global serial mutexes over **one** process-global state, registry installation outside the critical section, nested acquisition = silent hang | **one** lock, three surface doors onto it, installation inside the section, nested acquisition refused loudly — plus 5 new laws (§2) |
| the suite ABORTING | at test 395 of 438 the binary died: `panic in a destructor during cleanup` — 43 laws never ran | a test guard's `Drop` no longer panics inside an unwind; the failing restore is printed and still asserted where asserting is not fatal (§1.4) |
| `delivery_box_fillet_preview` | reported 3 round trips, attributed to the 17:45 `🧵️preview-eval` edit | **2 round trips measured here**, and that edit is **not on this test's code path at all** (§3); the count is wall-clock-derived and its law is now load-robust |
| evaluate budget law | single contended wall reading vs a fixed ceiling, best-of-3 | ceiling scaled by an **in-process calibrated load factor**, measured only when a raw reading overruns (§4) |
| `--lib` unskipped, `--test-threads=1` | never measured unskipped — gates-green's best was `413 passed; 18 failed; 1 filtered out` of 432 in 237 s **with one test skipped** | **`444 passed; 5 failed; 0 filtered out` of 449 in 204 s**, nothing skipped (§5.2) |
| `--test example-geometry` | `17 passed; 0 failed` in 2.62 s (19:49, this lane's own baseline) | **`18 passed; 0 failed` in 4.26 s** (21:31) — the 17 laws plus the new calibration law, `delivery_box_fillet_preview` at `roundTrips=2` (§5.1) |
| `@semio-tech/procedural-js:test` | 35 pass / 0 fail | `35 pass`, `0 fail` with the new calibration contract (§4.4) |

---

## 1. The `--lib` suite does not deadlock — it convoys

### 1.1 The accused test is innocent

`📓️gates-green-2026-09-13.md` §5.1 concluded that
`an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain` "acquires the serial lock
and never releases it". Run alone, unskipped, `--test-threads=1`
(`🗑️generated/lib-suite/probe-run.txt`):

```
test editor::generation3d::commands::flow_eval_tick::tests::an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 431 filtered out; finished in 27.35s
```

It terminates, it passes, and its guard is released like any other. It is slow (27 s), which is why
it is *visible* in a stall window, but it is not the holder of anything.

### 1.2 What the stall actually is, measured

`📜️lib-deadlock-probe.sh` runs the whole test binary at the default thread count and, if it is still
alive after N seconds, attaches macOS `sample(1)` to it. The capture
(`🗑️generated/lib-suite/probe-sample.txt`, 150 s into an unskipped run) shows **10 libtest threads**
on this 10-core host:

* **9 of 10 blocked**, every one of them in
  `…unit_tests::serial_execution::lock → std::sync::Mutex::lock → _pthread_mutex_firstfit_lock_wait
  → __psynch_mutexwait`.
* **1 running** — `editor::generation3d::commands::scale_selection::tests::scale_s…` — and of its
  1529 samples **862 (56 %) sit in `cthread_yield` → `swtch_pri`**, i.e. a `std::thread::yield_now()`
  spin inside the framework's fixture settle ladder.
* The `semio-pool-worker-0..8` threads are parked on their own condvar with no work.

So the picture is: **`--test-threads=N` buys no parallelism at all**, because every law in the binary
takes the same process-global mutex; and the one law that is running spends most of its time in a
yield-spin. A thread that yields parks in `swtch_pri`, which is why `ps` reads ~0 % CPU for the whole
binary, and the framework's fixture ladders carry **30-second wall deadlines**
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:6727`, `settle_registered_typed_operation`),
which is why the output goes byte-identical for 30+ seconds at a time. "Zero progress at 0.0 % CPU"
is exactly what a convoy + yield-spin + a 30 s deadline looks like from the outside. It is not a
deadlock; nothing is waiting on anything a running thread will never release.

The consequence is real even though the diagnosis changes: running this suite at the default thread
count both takes forever AND fails laws that would pass, because the 30 s deadlines are burned
queueing. Unskipped, default threads, 12 minutes (`🗑️generated/lib-suite/before-parallel-partial.txt`):
**31 passed, 42 failed** out of 438 — and every one of those 42 is
`registered fixture typed operation did not retire within 30 seconds` or
`…did not reach its terminal-empty close witness`, i.e. the close-ladder family, produced by the
convoy rather than by the laws.

### 1.3 The lock-discipline defects that are real

Three, all at the owning layer, all fixed in §2:

1. **Three process-global serial mutexes over one process-global state.**
   * `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` — `serial_execution::lock`, ~160 call sites.
   * `…/👁️viewer/🧪️tests/🔬️unit/🦀️.rs:17` — `context::lock`, 33 call sites.
   * `🧊️generation3d/🧪️tests/🔬️publication-authority/🦀️.rs:13` — `publication_authority::lock`, 1 call site.

   All three guard the SAME things: the flow extension registry, the neural kernel cache
   `FlowHost::evaluate` reads, `tessellate_geometry`'s mesh cache, and the fixed four-slot
   publication lease table. Holding different locks is not mutual exclusion — an editor law and a
   viewer law ran concurrently over one kernel cache. And the lease table's admission is a
   `try_lock` that answers `generation3d-publication.contended`
   (`…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:325`) — the exact failure gate 5 reports — which ANY law
   holding a retained tool job can provoke, while exactly one law took the door meant to prevent it.
   Two disjoint locks over one resource is also all a lock-order inversion needs.

2. **The registry mutation ran OUTSIDE the critical section.** Both doors read
   `flow_operators::installed(); TEST_SERIAL.lock()…` — `installed()` rewrites the process-global
   extension registry (`register_linked_flow_extension_installer` ×2,
   `install_flow_extension_manifest` ×2) *before* taking the lock that exists to serialise exactly
   that state.

3. **A nested acquisition was a silent hang.** `std::sync::Mutex` is not reentrant, so one law
   taking the lock and calling a helper that takes it again does not fail — it stops the binary
   forever with no attribution. That is precisely the symptom that was mistaken for a leaked guard,
   and nothing in the tree could tell the two apart.

Release itself was never the defect: `MutexGuard`'s `Drop` releases on normal return, on early
return and on a panicking unwind, and both doors already swallowed poisoning. §2 states all three as
laws anyway, because "the guard is released" is the assumption every other law rests on.

### 1.4 A fourth one, found by running the suite unskipped: the binary ABORTS at test 395

The first unskipped `--test-threads=1` run got to 395 of 438 and then died:

```
thread 'viewer::…::eval_chain_tests::a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted'
  panicked at …/🔌️plugin/🦀️.rs:6857: registered fixture did not reach its exact terminal-empty witness,
  last pending close authority: document store close awaits a retained reader or owner
thread '…' panicked at …/🧪️tests/🔬️flow-operators/🦀️.rs:141:
  the law leaves the process-wide registry as it found it: "flow.registry-retirement-full"
thread '…' panicked at core/src/panicking.rs:233: panic in a destructor during cleanup
thread caused non-unwinding panic. aborting.
```

Two panics, one abort. The FIRST is the close-ladder family — flow-eval-session-retirement's, not
this lane's. The SECOND is `UnlinkedFlowExtensions::drop`
(`🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs`), the guard that retires the linked operator packs
for the length of a law and puts them back: it restored the registry with two bare
`.expect("the law leaves the process-wide registry as it found it")` calls. **A `Drop` that panics
while its thread is already unwinding is a non-unwinding panic — the process aborts on the spot**, so
a diagnostic about a law that had already reported its own failure cost the remaining **43 laws**,
which were never run. The `assert!` three lines below those two `.expect()`s was already written
`std::thread::panicking() || …`, so the hazard was known at that site; the two `.expect()`s simply
were not covered.

Fixed at that layer: the restore is now one `Result` chain, a failure is printed unconditionally
(so the cause is never lost) and raised as an assertion on every path where raising it is not fatal:

```rust
assert!(std::thread::panicking() || restored.is_ok(), "the law leaves the process-wide registry as it found it: {restored:?}");
```

A failing restore is still a failure. It just no longer takes 43 unrelated laws with it. Lawed by
`a_failing_law_does_not_abort_the_binary_through_the_registry_guard` (§2.1).

---

## 2. The fix: one lock, three doors, a loud refusal

New module `crate::test_serial`
(`✏️s/…/🧊️generation3d/🧪️tests/🔬️serial/🦀️.rs`, registered in the crate root's `🧪️Tests` region):

* **One** `static TEST_SERIAL: Mutex<()>` for the whole test binary.
* `flow_operators::installed()` now runs **inside** the critical section.
* An `OWNER: AtomicU64` records the holding thread's token (minted per-thread from a counter, since
  `ThreadId::as_u64` is unstable and thread addresses are reused). `lock()` asserts the calling
  thread is not already the owner, so a nested acquisition costs **one named failure** instead of the
  run. `TestSerialGuard::drop` clears the record, so release and the record can never disagree.
* Poisoning is still swallowed — a law that panicked has already reported itself and must not cascade.

The three surface doors keep their names and their docstrings and now delegate:

* `crate::editor::generation3d::unit_tests::serial_execution::lock()`
* `crate::viewer::generation3d::unit_tests::context::lock()` — reaches `crate::test_serial`, **not**
  `::editor::`, so `policyViewerPurityBreaches` is untouched
* `crate::publication_authority::lock()`

No call site changed; every one of the ~194 `let _serial = …` bindings is unaffected.

### 2.1 The laws (`🧪️tests/🔬️serial/🧪️tests/🔬️unit/🦀️.rs`)

Each one would HANG rather than fail if the discipline regressed, so each reaches its verdict on the
calling thread in bounded time.

| law | states |
|---|---|
| `a_panicking_law_releases_the_serial_lock` | a law that panics while holding it releases it, the owner record is cleared, and the lock is takeable again |
| `an_early_returned_law_releases_the_serial_lock` | the `?`/`return` path releases it too |
| `a_nested_acquisition_is_refused_instead_of_hanging` | a second acquisition on the same thread panics with a named message and leaves the outer guard intact |
| `every_surfaces_door_opens_the_same_lock` | holding the editor door makes the viewer door AND the publication door refuse — the three can never drift back apart |
| `a_failing_law_does_not_abort_the_binary_through_the_registry_guard` | a law that panics while the linked operator packs are retired still gets them back, and the guard's `Drop` does not take the binary down with it (§1.4) |

Run (`🗑️generated/lib-suite/serial-laws-2.txt`, `--test-threads=1`, foreground):

```
running 5 tests
test test_serial::tests::a_failing_law_does_not_abort_the_binary_through_the_registry_guard ... ok
test test_serial::tests::a_nested_acquisition_is_refused_instead_of_hanging ... ok
test test_serial::tests::a_panicking_law_releases_the_serial_lock ... ok
test test_serial::tests::an_early_returned_law_releases_the_serial_lock ... ok
test test_serial::tests::every_surfaces_door_opens_the_same_lock ... ok
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 434 filtered out; finished in 0.09s
```

The binary now declares **439** tests (434 + these 5).

---

## 3. `delivery_box_fillet_preview` — 2 round trips, and the accused edit is not on its path

### 3.1 Measured

`🗑️generated/lib-suite/before-example-geometry.txt` (19:49, foreground, `--test-threads=1`):

```
[DELIVERY] box-fillet-preview roundTrips=2 chunks=1 packBase64Bytes=8408 triangles=92 edgeSegments=72
           phase=complete diagnostics=None payloadMeshes=1 payloadInstances=1 payloadTriangles=92
           payloadEdgeSegments=72 stepMicros=[16950, 1284] totalMicros=18234
test result: ok. 17 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.62s
```

Two round trips, against `"maxRoundTrips": 2`. Re-measured at 20:00 under load: `roundTrips=2`,
`stepMicros=[15107, 1338]`. There is no delivery regression to restore on this machine.

### 3.2 The 17:45 `🧵️preview-eval/🦀️.rs` edit cannot have caused it

`run_delivery` (`…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`, the only producer of `roundTrips`) drives
the transfer directly:

```
session.note_pending_tessellate(node_hash, handle) → session.next_tessellate_chunk(node_hash)
→ brep_geometry::tessellate_step_envelope_json(handle, tolerance, BUDGET, WALL_MICROS, chunk)
→ session.resolve_preview_tessellate(node_hash, &envelope)   // Working → loop, else break
```

It never calls `preview_eval::resolve_tessellate`, never calls `preview_eval::tick_is_unfinished`
and never calls `preview_eval::evaluate_tick` — the three things the viewer-generate-status-parity
lane changed at 17:45 (`📓️viewer-generate-status-parity-2026-09-13.md` §4). **That lane's settle-path
fix is not on this test's code path, and this lane did not touch it**; its 53 laws are untouched by
everything here.

### 3.3 What the count actually depends on — and the fix

`tessellate_step_envelope_json`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs:721`):

```rust
let deadline = default_now_us().map(|now| now.saturating_add(wall_micros));
let mut outcome = tessellate_step(handle, tolerance, budget);
while matches!(outcome, TessellationStepOutcome::Working { .. }) {
    let Some(deadline) = deadline else { break };
    if default_now_us().is_none_or(|now| now >= deadline) { break; }
    outcome = tessellate_step(handle, tolerance, budget);
}
```

with `TESSELLATE_STEP_WALL_MICROS = 6_000` (`:963`). **How many kernel steps fit into one round trip
is decided by a wall clock.** A machine running the kernel half as fast fits half as many
`tessellate_step` calls into one round trip's 6 ms, and the same unchanged geometry costs more round
trips. `roundTrips` is therefore a wall-clock reading wearing an integer's clothes, and judging a
single contended one is the same mistake as judging a single contended microsecond reading — which
is what gates-green's `roundTrips=3` at 17:48, taken with eight peer cargo runs on the host, was.

The law is now (in `…/🧪️tests/🧩️geometry/🦀️.rs`):

* `best_delivery` — the preview microseconds AND the round trips are both improved by re-running,
  up to `TIMING_ATTEMPTS`, and the minimum of each is judged. An in-budget lane still pays for
  exactly one delivery.
* `assert_round_trip_budget` — if the best of those still overruns, the budget is scaled by the
  machine's calibrated load factor (§4) and the overrun is printed rather than believed blindly.

Counting round trips stays exactly as strict a promise as it was: one round trip is one whole
`flowEvalTick`, the committed number stays `2`, and code that genuinely needs a third one on an idle
machine still fails.

---

## 4. The budget law, made load-robust without weakening it

### 4.1 Why best-of-3 was not enough

`📓️kernel-performance-2026-09-13.md` §6.1 already retries an overrun up to `TIMING_ATTEMPTS = 3` and
judges the minimum. Under the fleet loads this repository actually runs at, all three attempts
overrun together: gates-green measured
`FlowHost::evaluate took 1798354 us (best of 3) against a 1545000 us ceiling` with eight peer cargo
runs resident, while the same test on this machine at 19:49 measured **379 391 µs** — a 4.7x spread
on identical code. A ceiling that a 4.7x machine spread can cross is not a strict law, it is a noisy
one, and a noisy law gets ignored.

### 4.2 The correction is a scale, not a looser ceiling

New fixture, read by both halves:
`…/📚️examples/🧫️fixtures/⏱️budget-calibration/🔣️.json`

```json
{
  "schema": "s.procedural.generation3d.example-budget-calibration/v1",
  "rounds": 3584,
  "checksum": "0x24d974393bfa7c15",
  "referenceMicros": 101180,
  "maximumLoadFactor": 16.0
}
```

The Rust half runs a fixed, kernel-independent workload — an LCG feeding an `orient2d`-shaped
cross-product sum plus one owned `Vec` per round, so it exercises the same mix the kernel's cost is
made of (floating-point multiply/add chains plus allocator traffic) without touching one line of
kernel code any lane may legitimately make faster. Only the integer state is checksummed; the
floating-point accumulator is returned so the work cannot be elided but never asserted, because
`a*b+c` may be contracted to an FMA on one target and not another.

`machine_load_factor() = median(3 timed passes) / referenceMicros`, clamped to `[1.0, 16.0]`, and it
is computed **only when a raw reading has already overrun** — a green lane pays nothing. A genuine
algorithmic regression inflates the phase without inflating the calibration and still fails at any
load; contention inflates both and cancels. The factor may only ever widen a ceiling, never narrow
one, because the ceilings are regression guards with deliberate headroom rather than targets.

### 4.3 The statistic was chosen by measurement, not by taste

The first attempt was `min` of five ~1.8 ms passes. Measured: it read **1811 µs idle and 1811 µs
under heavy load** — byte for byte its idle reading — while a long phase on the same machine at the
same moment was an order of magnitude over its ceiling. A workload short enough to fit inside one
scheduling quantum is never preempted, so its minimum measures a machine nobody's phase ran on. The
pass is therefore sized to ~100 ms, long enough to be preempted the way a real phase is, and the
**median of three** is believed. That finding is written into the code at `CALIBRATION_PASSES`.

The reference was derived by linear extrapolation from a clean 64-round reading (1807 µs × 56) and
then confirmed directly on the built binary: five consecutive readings with peers active gave
`102795 / 106855 / 114071 / 120922 / 158727 µs` → factors `1.02 … 1.57`. The calibration tracks
machine load in the live fleet, which is the whole point.

`the_budget_calibration_is_deterministic_and_never_narrows_a_ceiling` is the new law: the workload is
deterministic within one process, it still produces the committed checksum (otherwise its reading is
not comparable to the reference), and the factor stays inside its declared range. It prints its raw
reading, so any lane that reports a budget failure can say in one line whether the machine was the
cause.

### 4.4 The TypeScript twin

`…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts` gains `EXAMPLE_BUDGET_CALIBRATION_PATH`,
`ExampleBudgetCalibration`, `readBudgetCalibration()` and `assertBudgetCalibrationContract()`, called
from `assertBudgetContract`, so the row's own invariants are derived independently of the Rust run —
a zero reference would scale every ceiling to infinity, a factor below one would narrow a ceiling,
and a checksum that is not a 64-bit literal cannot prove the workload is the one the reference was
measured on.

```
bun nx run @semio-tech/procedural-js:test
35 pass, 0 fail, 448 expect() calls — Ran 35 tests across 11 files
```

---

## 5. Runs

Every count below was produced by a foreground run in this session.

### 5.1 `--test example-geometry`

| when | tree | result |
|---|---|---|
| 19:49 | before this lane's geometry-test changes | `17 passed; 0 failed` in **2.62 s** |
| 20:12 | after them, and after a peer's 19:58 retirement edit | `1 passed; 17 failed` in 284 s — **all 17** panic `flow host retirement did not reach terminal-empty` at `🧩️geometry/🦀️.rs:531`, inside `retire_host`, before any budget assertion runs |
| **20:24** | after them, and after the peer's 20:14 follow-up | **`18 passed; 0 failed; 0 filtered out` in 3.70 s** — the 17 original laws plus this lane's `the_budget_calibration_is_deterministic_and_never_narrows_a_ceiling` |

| **21:31** | final state of this lane's work | **`18 passed; 0 failed; 0 filtered out` in 4.26 s**, `delivery_box_fillet_preview` still at `roundTrips=2` |

The 20:24 green run, byte for byte:

```
[BUDGET] box-fillet-preview evaluateMicros=3825 budget=20000 tessellateMicros=79666 budget=408000
[BUDGET] sphere-cut-with-torus evaluateMicros=437998 budget=1545000 tessellateMicros=1073997 budget=3171000
[DELIVERY] box-fillet-preview roundTrips=2 chunks=1 packBase64Bytes=8408 triangles=92 edgeSegments=72
           phase=complete … stepMicros=[17192, 1317] totalMicros=18509
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.70s
```

No `[CALIBRATION]` line appears, which is the design working: not one phase overran its raw ceiling,
so not one calibration pass was paid for.

**The 20:12 red run was a peer's, and it recovered without this lane touching it.** The panic family is
the one `📓️kernel-performance-2026-09-13.md` §3 fixed in `FlowHostRetirement::close_page` (the unpaid
frontier reservation). `sample(1)` on the stuck process
(`🗑️generated/lib-suite/serial-stuck-sample.txt`) put the whole test thread inside
`FlowHost::retire_cold → FlowHostRetirement::close_page → FlowRetirement::close_page →
ErasedSnapshotRetirement::close_step → PagedList::release_empty_page → release`, **15+ frames of
self-recursion** in `semio_framework_artifact_flow_flow::retained`. Both producing files
(`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs`, `🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs`) were
rewritten at **19:58:52**, between the green run and the red one, by the
**flow-eval-session-retirement** lane, which owns the reserve-then-close frontier class and whose
report (`📓️flow-eval-session-retirement-2026-09-13.md`) is still a skeleton. Their 20:14:10 follow-up
to `🧵️retained/🦀️.rs` cleared it. Per the brief this lane did not touch any of it, and nothing this
lane changed is on `retire_host`'s path: `crate::test_serial` is `#[cfg(test)]` inside the plugin
crate and the example-geometry binary does not link it, and the calibration runs strictly after
`retire_host`.

That panic is the exact family `📓️kernel-performance-2026-09-13.md` §3 fixed in
`FlowHostRetirement::close_page` (the unpaid frontier reservation). The two files that produce it —
`🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` and `🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs` —
were both rewritten at **19:58:52**, between the green run and the red one, by the
**flow-eval-session-retirement** lane, which owns the reserve-then-close frontier class and whose
report (`📓️flow-eval-session-retirement-2026-09-13.md`) is still a skeleton. Per the brief this lane
did not touch it. Nothing this lane changed is in `retire_host`'s path: `crate::test_serial` is
`#[cfg(test)]` inside the plugin crate and the example-geometry binary does not use it, and the
calibration runs strictly after `retire_host`.

### 5.2 `--lib --features component-app-assembly`, unskipped, `--test-threads=1`

```
running 449 tests
test result: FAILED. 444 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 204.36s
```

`0 filtered out` — nothing skipped, including
`an_uncontributed_graph_arms_no_tick_while_a_served_one_keeps_its_chain`, which passes, as do all ten
of the laws gates-green found "stuck". Against that report's
`413 passed; 18 failed; 1 filtered out` in 237 s (with the skip), taken 2½ hours earlier on a tree
that has moved under every lane since.

| # | failing law | message | owner |
|---|---|---|---|
| 1 | `commands::navigate_graph::tests::every_arrow_walk_lands_where_the_fixture_says` | `a fork upstream takes the first candidate in reading order, not in wire order: left ["sides"] right ["radius"]` | **node-graph-wire-drag** — this command and its fixture were created live at 21:09, mid-run |
| 2 | `component::unit_tests::generation_preview_is_one_app_transient_shared_by_two_generation_windows` | `Generation3d preview ownership runtime: "Generation3d preview operation did not finish"` | **flow-eval-session-retirement** — retained-operation settle; deterministic, reproduces alone |
| 3 | `component::unit_tests::two_instances_converge_disjoint_widget_moves` | `module.vcs … remote snapshot merge is fail-closed until the app-owned streaming envelope decoder and persistent candidate transaction are terminal-authorized` | store **`module.vcs`** — the brief's named bucket |
| 4 | `component::unit_tests::vcs_artifact_app_non_empty_retained_maintenance_swap_is_authoritative_and_fail_closed` | `P3 production envelope load did not reach terminal` (it read `generation3d-publication.contended` in the 21:02 run) | **`generation3d-publication.contended`** — the brief's named bucket, sharpened in §5.3 |
| 5 | `viewer::…::eval_chain_tests::a_late_contributions_install_re_arms_the_viewer_evaluation_the_empty_registry_faulted` | `an empty closure is a legal registry state …: "flow.registry-retirement-full"` | **registry retirement** — §5.4 |

Everything else gates-green listed is gone: the five typed-operation retirement timeouts, the two
terminal-empty close witnesses, the registry-retirement law, the two viewer-preview render laws, the
two viewer-preview instance laws, the wireframe show-mode law, **both `nodeGraphViewport` bridge
laws** and the selection-transform law all pass now. Those were their owners' to fix, and they did.

### 5.3 `generation3d-publication.contended` is not test-to-test contention

Worth handing to whoever owns the lease table. Run **alone**, as the only test in the binary, that law
still failed with `generation3d-publication.contended` (`🗑️generated/lib-suite/` isolation runs,
21:00). With one test thread and now one serial lock there is no other law running, so the
`try_lock` at `…/🧬️schema/🧬️mutations/💾️binary/🦀️.rs:325` is losing to something inside the app's own
worker pool, not to a sibling law. A `try_lock` that answers "contended" to a legitimate concurrent
caller cannot be made reliable by serialising tests harder.

### 5.4 `flow.registry-retirement-full` is accumulated debt — and the harness must NOT pay it

Law 5 **passes when run alone** and fails at position ~400 of the suite.
`begin_flow_registry_replacement` (`🧰️framework/…/🌊️flow/📔️registry/🦀️.rs:232`) refuses once
`RETIRED_REGISTRY_CAPACITY` (16) retired versions are queued; a served host pumps that queue and a
test binary does not, so the seventeenth law that replaces the contribution map fails on sixteen
earlier laws' debt.

The obvious fix — draining the queue from the serial guard with the crate's own public
`retire_flow_extension_registries_step` — **is wrong, and this lane measured it rather than reasoning
about it.** The drain was implemented, then A/B'd in the SAME binary on the SAME tree, back to back
(`🗑️generated/lib-suite/ab-nodrain.txt`, `ab-drain.txt`):

```
without the drain:  443 passed;  7 failed; 0 filtered out; finished in 121.71s
with the drain:     375 passed; 75 failed; 0 filtered out; finished in  49.26s
```

— a cascade of `flow neuron kind info cache: PoisonError { .. }` (36×) and
`job-session.terminal-fault` (28×) beginning at the 21st law, because a retired registry version is
still referenced by live sessions and by the catalogue cache. The drain was removed; the reason it
must stay removed is written at its site in `🧪️tests/🔬️serial/🦀️.rs` so the next lane does not
re-discover it the expensive way. The queue's depth belongs to the registry-retirement frontier.

---

## 6. A peer compile break fixed forward

At 19:07 the react-i18n-a11y-customization lane's `Generation3dLabels` migration left the plugin
crate's test build with three errors that blocked every measurement in this report. Fixed forward,
minimally, without touching the migration itself:

* `…/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs` — added the missing
  `use crate::editor::generation3d::terminology::Generation3dLabels;` (the signature at `:78` already
  named the type; only the import was absent).
* `…/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs:105,179` — the two `render(...)`
  call sites, untouched since 2026-09-12, now pass the sixth argument the producer at `:360` grew:
  `crate::editor::generation3d::terminology::generation3d_labels(&ViewModel::default())`, the same
  spelling the sibling flow-window laws already use.

At 21:00 the node-graph-wire-drag lane's new `nodeGraphViewport` law blocked every build with
`E0277: the trait bound dsl::DslValue: From<{float}> is not satisfied` ×3:

* `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:1188` — `1.0.into()` / `2.0.into()` / `0.0.into()` now read
  `dsl::DslValue::float(1.0)` etc. `DslValue` has `From` for integers only; floats go through the
  named constructor (`🧰️framework/🔨️modules/🌱️value/🦀️.rs:140`).

Earlier, at 19:02, a `rim_handle_anchor_hit` break in
`🧰️framework/…/♾️infinite/🎲️board/🔌️ports/➡️directed/🕸️dag/🦀️.rs`, and at 21:05 a
`couldn't read …/🧭️navigate-graph/🧪️tests/🔬️unit/🦀️.rs` from a `mod tests;` whose file did not exist
yet, each blocked a build for a few minutes. Both were their owners' in-flight work and both were
gone when I re-read; this lane changed nothing in either.

---

## 7. Files

Changed by this lane:

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️flow-operators/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️serial/🦀️.rs` (new)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️serial/🧪️tests/🔬️unit/🦀️.rs` (new)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🧪️tests/🔬️publication-authority/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧫️fixtures/⏱️budget-calibration/🔣️.json` (new)
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🟦️.ts`

Fixed forward for a peer (§6):

- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/👁️preview/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/👁️preview/🧪️tests/🔬️unit/🦀️.rs`
- `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (the `DslValue::float` line; this file is also where the editor door lives, which is this lane's own change)

Scripts (durable, ticket root):

- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📜️lib-deadlock-probe.sh`
- `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/📜️lib-suite-run.sh`

## 8. What is NOT claimed

- **The five remaining `--lib` failures are NOT fixed.** §5.2 attributes each one; four belong to
  named owners (node-graph-wire-drag, flow-eval-session-retirement, `module.vcs`,
  `generation3d-publication.contended`/registry retirement) and this lane touched none of them. The
  counts are this tree at 21:29, which every lane is still moving.
- **The suite is still a convoy at the default thread count, and this lane did not change that.**
  One lock over one process-global state is correct; it is also, by construction, zero parallelism.
  The right invocation of this suite is `--test-threads=1`, which is what every measurement here
  used. Making the suite genuinely parallel would mean giving each law its own registry and kernel
  cache — a different lane's design question, not a lock-discipline fix.
- **`generation3d-publication.contended` was sharpened, not diagnosed.** §5.3 proves it is not
  test-to-test contention; it does not identify which pool worker holds the lease `try_lock`.
- **The abort-proofing of `UnlinkedFlowExtensions::drop` is proven by the mechanism and by the run
  that no longer aborts, not by a law that forces the failure.** Forcing it needs a full
  registry-retirement queue, and §5.4 measured that filling that queue poisons the process-global
  catalogue for every later law — so a law that did it would cost more than it proved. What IS
  lawed is the release path (`a_failing_law_does_not_abort_the_binary_through_the_registry_guard`),
  and what is observed is `UnlinkedFlowExtensions: the process-wide registry was not restored:
  flow.registry-retirement-full` printed in the 21:29 run with the binary carrying on to the end.
- **The calibration reference (`referenceMicros: 101180`) is this machine's.** It was derived by
  linear extrapolation from a clean 64-round reading and confirmed against five direct readings
  (min 102 795 µs). It is exactly as machine-specific as the ceilings it scales — and it makes those
  ceilings LESS machine-specific than they were, because the ratio travels. No claim is made that
  the factor absorbs every load level; it divides out contention proportionally, and the run that
  needs it prints its raw reading so the next reader can judge.
- **`delivery_box_fillet_preview` was never reproduced at 3 round trips here.** Every measurement in
  this session read 2. The claim is about the MECHANISM — the count is produced by a wall-clock
  deadline inside `tessellate_step_envelope_json` — which is read off the code at
  `📐️brep-geometry/🦀️.rs:721-728`, not off a reproduction. The 17:45 `🧵️preview-eval` edit is
  excluded by code path, not by bisection.
- **No synthetic load was generated for any measurement in this report.** An earlier load experiment
  in this session did spawn busy-loop processes and leaked them; they were killed, none remain, and
  nothing in this report rests on them. The calibration's load-tracking is shown by readings taken
  under the fleet's own load (1.02x → 1.57x within minutes, §4.3).
- **`--test-threads=1` was not registered in `launch.json`.** Neither the `--lib` suite nor
  `--test example-geometry` has ever had an entry there, and this lane introduced no new repo
  command — only two ticket-local shell scripts. Registering the two cargo gates as nx targets is a
  real follow-up and is deliberately not smuggled into this lane's diff.
- **`dependencies literal-external`, the repo-wide typecheck and the React engine suite** were
  outside this lane's brief and were not run.
- **No runtime browser verification** was done; nothing here is claimed to have been observed in a
  served app.
- No modifying git command was run. No dev server was started, stopped or restaged. No ticket
  lifecycle call was made, and `📓️status.md` / `🎫️ticket.json` were not edited. Nothing under
  `🗑️generated/` that this lane did not create was touched.
