# 🏎️ Kernel Performance — evaluate → tessellate → mesh-encode, 2026-09-13

Lane: **kernel-performance**. Ticket `26/09/09/PROCEDURAL-3D-END-TO-END`.
Owned here: the brep kernel, the evaluate job, the tessellator, the mesh encoding.
Not touched: host/UI dispatch, plugin staging, the React playground (other lanes own those).

---

## 1. TL;DR

1. **The example-geometry lane was 0/17 before this lane started.** Every case died in
   `retire_host` with "flow host retirement did not reach terminal-empty". Root cause located and
   fixed in `FlowHostRetirement::close_page`: it drove `FlowRetirement` with `close_step` alone and
   never paid the frontier reservation that shape *requires*, so `close_step` answered `Blocked`
   forever — a **silent infinite spin**, not an error. One reserve-then-close branch fixed it.
   (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs:2372`.)

2. **The dominant cost of the whole chain was arbitrary-precision rational arithmetic in the exact
   geometric predicates.** A `sample(1)` profile of `🍩️sphere-cut-with-torus` put **99.3 % of a 25 s
   window** under one call — `FlowHost::evaluate` → `Cut::evaluate` → `Brep::boolean_sync` →
   `stitch_selected_faces` → `mass_properties::…::ear_clip` — and nearly all of the *self* time in
   `semio_framework_number::Natural`'s `gcd` / `normalize` / `checked_sub` plus the malloc traffic of
   the limb vectors every `Rational` operation allocates.
   The exact path was rewritten as **nonoverlapping floating-point expansion arithmetic** (Shewchuk
   error-free transformations): allocation-free, bignum-free, and *exact in the same sense* — so
   every sign it returns is the sign the rationals returned.
   (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➡️vector/⚖️predicates/🦀️.rs`.)

3. **Measured effect (native `test` profile, same machine, same methodology):**

   | metric | before | after | factor |
   | --- | ---: | ---: | ---: |
   | whole `example-geometry` lane (17 cases) | 104.90 s | **6.66 s** | 15.7x |
   | `sphere_cut_with_torus` geometry case | 61.539 s | **4.286 s** | 14.4x |
   | `delivery_sphere_cut_with_torus` | 40.315 s | **1.161 s** | 34.7x |
   | torus preview-LOD tessellate step | 5 904 667 us | **110 408 us** | 53.5x |
   | `sphere-box-fuse` preview-LOD step | 336 236 us | **10 867 us** | 30.9x |
   | `box-fillet-preview` preview-LOD step | 227 100 us | **58 959 us** | 3.9x |

4. **The geometry did not move.** All 17 laws pass, and the committed `[STATS]` line of every example
   is byte-identical before and after — including the `parry3d` third-party volumes and bounding
   boxes — with one exception that is *not* caused by this work (§6).

5. **New laws.** Every bundled example's fixture now carries a `budget` row
   (`maxEvaluateMicros` / `maxTessellateMicros` / `maxPreviewTessellateMicros`) asserted by the Rust
   lane and contract-checked by the TypeScript twin. Five new **third-party oracle** laws hold the
   new exact predicate path to `num-rational`'s arbitrary-precision `BigRational` on 4 900 randomized
   and adversarial configurations.

6. **What was tried and reverted:** a Bowyer–Watson cavity/proximity rework of the tessellator. It is
   exactly result-preserving and asymptotically better, but an interleaved A/B of two binaries under
   identical load measured it **1.3x–4x SLOWER** in the native unoptimized profile. It is reverted.
   The tessellator is now the top remaining cost and §7 states exactly what it needs.

---

## 2. Method

Everything was measured natively, in the foreground, on this machine, with the `test` (unoptimized)
profile — the same profile the lane runs in.

* **Profiler**: macOS `sample(1)` attached to the running test binary, parsed into inclusive/self
  tables. No external profiling dependency. Raw captures in `🗑️generated/kernel-perf/sample-*.txt`.
* **Timing**: `std::time::Instant` around the two phases, printed as `[BUDGET]`; the delivery lane
  already printed per-step `[DELIVERY] stepMicros`.
* **Contention**: this machine is a shared build host. During this work the load average ranged from
  23 to **198** as peer agents compiled. A single wall reading of a single-threaded phase inflates
  2x–4x under that, and I measured a 19x spread on one example across eight runs. **Every comparison
  below is a minimum over N runs**, and A/B comparisons were done by building *both* variants,
  copying the two executables aside, and running them **interleaved** so both see the same load.
  `🐍️example-phase-timings.py` is the harness that does this.

---

## 3. Root cause 1 — the retirement spin (blocker)

`✏️s/…/📚️examples/🧪️tests/🧩️geometry/🦀️.rs:322` panicked for all 17 cases. The panic is a bounded
`for _ in 0..1_000_000` loop giving up, so the real symptom was a *non-terminating ladder*.

Instrumenting `FlowHostRetirement::close_page`'s branches (temporary `[DEBUG]` prints, removed)
showed it entering the `domain` branch forever with no fault raised:

* `🧰️framework/…/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🦀️.rs:325` — `FlowRetirement::close_step`
  returns `SnapshotRetirementStep::Blocked` (**not** an error) while `next_allocation_bytes()` still
  names a frontier page the decomposition of the current owner needs.
* `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` — `FlowHostRetirement::close_page`'s `domain` branch called
  `close_step` and treated everything but `Err` as progress. It never reserved.
* The only correct driver in the tree was `FlowRetirement::retire_cold`, which *does* drain
  `next_allocation_bytes` → `reserve_allocation` before closing. This was long-standing, not a fresh
  peer break: the same shape is in `ebbace9b32` (2026-09-09) and every commit since.

Fix: the `domain` branch now pays the reservation first, exactly as `retire_cold` does. Effect on one
case alone: `rectangle_wire_preview` 3.432 s → 0.020 s, because the 1 000 000 spin iterations were
being charged to every example.

---

## 4. Root cause 2 — exact predicates on bignum rationals (the real one)

### 4.1 What the profile said

`sample` over `sphere_cut_with_torus_evaluates_to_the_difference_volume`, 20 890 samples on the test
thread, **100 %** under `FlowHost::evaluate`:

```
FlowHost::evaluate → evaluate_channels_budgeted → Registry::dispatch
  → flow_extension_brep::Cut::evaluate → Brep::boolean_sync → step_boolean_job_sync
    → BooleanJob::step → stitch_selected_faces            (11 945 / 20 890 = 57 %)
      → mass_properties::shell_signed_volume → face_volume → loop_volume → loop_moments
        → ear_clip → point_in_or_on_triangle → orient2d → orient2d_exact
          → Rational::from_f64 → Rational::normalize → Natural::gcd …
```

Top *self* time was, in order: `slice::Iter::next_back` (limb comparison), `_xzm_free`,
`Natural::checked_sub`, `Natural::shr`, `_xzm_xzone_malloc_tiny`, `Natural::gcd`,
`Natural::normalize`, `Natural::trailing_zeros`. Every one of them is bignum machinery or the
allocator serving it.

### 4.2 Why the "cold path" premise was false

The module's own doc claimed the exact path was cold, and therefore that simplicity beat speed:

> *"the exact path is cold, so raw simplicity beats squeezing out its last microsecond"*

That premise does not survive contact with `ear_clip`. `loop_uv_polygon` samples each coedge at a
curvature-adaptive count derived from `chord_tol`; at the `1e-3` tolerance `stitch_selected_faces`
passes, a 2.2-radius circle becomes ~100 segments, so a trimmed sphere/torus face's UV polygon has
hundreds of vertices, **many of them exactly collinear** (a dense discretisation of a smooth curve
produces exactly-collinear triples constantly). For those triples `orient2d`'s determinant is
*exactly zero*, the interval filter `filtered_sign` can never certify a zero, and the predicate
escalates to the exact path **deterministically, on essentially every call**. Ear clipping then calls
it O(n²) times per loop. The cold path was the hot path.

### 4.3 The fix

`orient2d_exact`, `orient3d_exact`, `in_circle2d_exact` and `sign_of_dot_exact` are now computed with
expansion arithmetic — `two_sum` / `two_diff` / Dekker-split `two_product`, `expansion_sum`
(Shewchuk's `fast_expansion_sum_zeroelim`) and `scale_expansion` (`scale_expansion_zeroelim`) — over
fixed-size stack arrays. A nonoverlapping expansion's largest component carries its sign, so
`expansion_sign` reads the last component; a non-finite leading component fails loudly rather than
silently reporting `Zero`, preserving the old `expect`'s loudness.

* No allocation, no GCD, no limb vectors, no `semio_framework_number` — that dependency is dropped
  from the crate's `[dependencies]` because nothing else in it used `Rational`.
* Dekker splitting rather than `f64::mul_add`: `mul_add` lowers to a **libm call** on
  `wasm32`, and the guest is where this has to be fast.
* Both paths are exact, so the returned sign is identical for every finite input. That is what makes
  this a pure performance change and not a geometry change.

### 4.4 Evidence it is still exact

| lane | what it establishes | count |
| --- | --- | ---: |
| `predicates::tests::quick::*` (pre-existing) | the cheap filter never answers where it must escalate | 11 000 configurations |
| `predicates::tests::oracle::*` (**new**) | the expansion path returns the same sign as `num-rational`'s arbitrary-precision `BigRational` | 2 000 orient2d + 600 orient3d + 300 incircle + 2 000 dot + 15 ULP-boundary cases |
| `predicates::tests::*` (pre-existing) | named degenerate/ULP cases | 8 |
| `example-geometry` (17 cases) | end-to-end geometry, with `parry3d` recomputing volume/CoM/AABB | 17 |

`num-rational` is a **test-only** dependency (`[dev-dependencies]`, never production-reachable — the
same standing `parry3d` already has here). It is a genuinely independent oracle: a different
representation (arbitrary-precision rationals) and a different algorithm, written outside this
repository, reading the same `f64` inputs losslessly.

Run: `cargo test -p semio-s-artifact-stdio-semio --lib predicates` → **16 passed, 0 failed**.

---

## 5. Profile, before and after, per example

Native `test` profile. `evaluate` = `FlowHost::evaluate` on a cold kernel; `tessellate` =
`tessellate_geometry` at the fixture's own tolerance from a cold mesh cache; `preview` = the sum of
the budgeted `tessellate_step_envelope_json` steps at the LOD the live preview asks for — the only
one of the three the app actually waits on. All values are microseconds, minimum over the runs
captured in `🗑️generated/kernel-perf/`.

### Before (single run; the lane was failing, so these are the phase costs that were reachable)

| example | preview-LOD tessellate step(s) | round trips |
| --- | ---: | ---: |
| sphere-cut-with-torus | 5 904 667 | 1 |
| sphere-box-fuse | 336 236 + 526 | 2 |
| box-fillet-preview | 227 100 + 3 021 | 2 |
| box-shell-preview | 2 878 | 1 |
| hexagonal-mushroom-column | 1 384 | 1 |
| face-sweep-extrude | 860 | 1 |
| rectangle-extrude-volume | 822 | 1 |
| rectangle-wire-preview | 39 | 1 |

### After

| example | evaluate | tessellate (fixture tol) | preview LOD |
| --- | ---: | ---: | ---: |
| sphere-cut-with-torus | 386 315 | 792 678 | 69 882 |
| sphere-box-fuse | 21 699 | 211 151 | 10 969 |
| box-fillet-preview | 2 297 | 102 094 | 18 371 |
| box-shell-preview | 1 539 | 2 630 | 2 736 |
| hexagonal-mushroom-column | 993 | 1 330 | 1 409 |
| face-sweep-extrude | 979 | 778 | 804 |
| rectangle-extrude-volume | 1 004 | 743 | 825 |
| rectangle-wire-preview | 304 | 17 | 43 |

The user-facing pair (`evaluate` + `preview LOD`) is now **≤ 456 ms** for the worst example and
**≤ 33 ms** for six of the eight, natively at `opt-level = 0`. The guest runs the identical code at
`opt-level = 2` (root `Cargo.toml`'s `[profile.wasm-dev.package]` overrides).

---

## 6. The laws added

### 6.1 `budget` row per example

`🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/<example>/🧫️fixtures/🧩️example/🔣️.json` gains:

```json
"budget": {
  "maxEvaluateMicros": 1545000,
  "maxTessellateMicros": 3171000,
  "maxPreviewTessellateMicros": 279000
}
```

* **Rust** (`📚️examples/🧪️tests/🧩️geometry/🦀️.rs`) times the phases and asserts each against its
  ceiling, and checks the row's own contract (`assert_budget_contract`).
* **TypeScript twin** (`…/🟦️.ts`) contract-checks the same row independently
  (`assertBudgetContract`), against `EXAMPLE_BUDGET_INTERACTIVE_CEILING_MICROS` (2 s, for the two
  phases a user waits on) and `EXAMPLE_BUDGET_FIDELITY_CEILING_MICROS` (8 s, for the test-only
  fine-tolerance tessellation nobody waits on).
* Ceilings are `4 x` the measured minimum, floored at 20 ms so a sub-millisecond example never gets
  a ceiling machine jitter alone can cross. They are set by `🐍️example-budget-fixtures.py` from the
  captured run logs, and are only ever lowered after a measured improvement.
* **Contention handling**: a phase that overruns is re-measured up to `TIMING_ATTEMPTS = 3` times and
  the *minimum* is judged. Contention is retried away; an algorithmic regression fails all three.
  This is stated in the code, not implied.

### 6.2 Geometry equality

No new oracle was needed for "the result did not change" — the existing one is the right one. The
`[STATS]` lines of all eight examples were diffed before/after the predicate rewrite and are
**identical**, including `parry3d`'s independently recomputed volume and AABB, and all eight
`[DELIVERY]` rows (triangles, edge segments, chunks, pack bytes, published payload) are identical
too.

**One difference, and it is not ours:** `sphere-box-fuse`'s `parryVolume` printed 9.692928314 before
and 9.692923546 after. Re-running the *same* binary three times gives 9.692929268 / 9.692923546 /
9.692928314 — **the oracle's own number is run-to-run nondeterministic**, from mesh ordering that
varies with hash iteration, amplified by `parry3d`'s `f32` accumulation. Our own
`volume`/`signedVolume` is bit-stable at 9.692918877 across every run. This is a **pre-existing**
defect worth its own ticket; it is inside the fixture tolerance and the lane is green either way.

### 6.3 Final green run

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly \
  --test example-geometry -- --nocapture --test-threads=1
test result: ok. 17 passed; 0 failed. finished in 13.70s          (load average 17-25)
```

with every budget met on the first attempt, no retry needed:

| example | evaluate / ceiling | tessellate / ceiling |
| --- | ---: | ---: |
| sphere-cut-with-torus | 1 018 237 / 1 545 000 | 1 802 983 / 3 171 000 |
| sphere-box-fuse | 22 122 / 87 000 | 527 467 / 845 000 |
| box-fillet-preview | 3 671 / 20 000 | 190 016 / 408 000 |
| hexagonal-mushroom-column | 2 700 / 20 000 | 3 299 / 20 000 |
| box-shell-preview | 2 630 / 20 000 | 2 672 / 20 000 |
| rectangle-extrude-volume | 1 256 / 20 000 | 786 / 20 000 |
| face-sweep-extrude | 1 019 / 20 000 | 789 / 20 000 |
| rectangle-wire-preview | 391 / 20 000 | 27 / 20 000 |

```
cargo test -p semio-s-artifact-stdio-semio --lib predicates
test result: ok. 16 passed; 0 failed.
```

This closes `📓️audit-gates-status-2026-09-13.md` item 2a, which measured 16/17 against the seeded
placeholder ceiling (`tessellate_geometry took 2308598 us against a 2000000 us ceiling`) while this
lane's budgets were still mid-flight. That ceiling was never a measured number — it was the uniform
`--seed` placeholder `🐍️example-budget-fixtures.py` writes before the first measuring run. The
committed ceilings are measured, per-example, and split by phase (§6.1).

### 6.4 `cargo test -p semio-s-artifact-stdio-semio` (whole crate): 2474 passed, 47 failed — NOT ours

The crate's full suite is red, and I checked rather than assumed. Restoring the pre-rewrite predicate
file (`git show 5b6f77afcf:…/⚖️predicates/🦀️.rs`, the `Rational` version) together with its
`semio-framework-number` dependency and re-running the failing names reproduces **the same
failures**:

```
cylinder_round_trips_through_snapshot          FAILED (both)
sphere_round_trips_through_snapshot            FAILED (both)
torus_round_trips_through_snapshot             FAILED (both)
foreign_string_ids_mint_fresh_labels           FAILED (both)
drawing::…::op_binary_roundtrip_law            FAILED (both)
```

Their messages are not sign decisions either: `same-parameter-violated: pcurve and 3D curve disagree
by 5.43` (a whole-unit geometric disagreement, which no exact-sign predicate can produce) and
`unknown keyword "create-layer"` (a peer's in-flight mutation-vocabulary edit). The remaining 42 are
`mutations::component` fixture-canonicalisation and `object`/`kit`/`value` subset cases, all outside
this lane. **Attribution: pre-existing / peer-owned, reproduced against the old code.**

### 6.5 Dependency truth gate

```
bun nx run workspace:verify -- dependencies literal-external
zero-target=0 literal-external=194 meets-target=false
oracle-conflicts=15 toolchain-owner-conflicts=2
```

Identical to the numbers `📓️audit-gates-status-2026-09-13.md` recorded before this lane's changes
(`current=194, oracle-conflicts=15, toolchain-owner-conflicts=2`). The new `num-rational` dev-dependency
adds **nothing** to either count because it is registered as a test oracle in
`🧊️brep/🔮️oracles/🔣️.json` (`num-rational-brep-exact-predicates`), which is what makes
`dependencyClassifyOracleEntry` classify it `test-oracle` rather than literal-external. The gate stays
red for the same pre-existing reasons it was already red; this lane did not move it.

---

## 7. What was tried and REVERTED

`refine_adaptive` + `insert_point_into` in
`🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs` carry two quadratic terms:

* `insert_point_into` rebuilds the **entire** triangle vector on every Steiner insertion (one fresh
  allocation plus a full copy of every surviving triangle, per point).
* `too_close` linearly scans **every** UV sample already placed, per candidate, and that set grows
  with every point it admits (bounded by `MAX_INTERIOR_POINTS = 20 000`).

I replaced both — an in-place cavity compaction preserving the exact triangle order, and a uniform
hash grid at the proximity tolerance whose 3x3 neighbourhood is provably complete. Both are exactly
result-preserving and all 17 laws stayed green.

**It is reverted, because it measured slower.** Two binaries were built, copied aside and run
interleaved; minimum of 8 runs each, `evaluate` (untouched by the change) as the contention control:

| example | tessellate before | tessellate after | ratio | evaluate control ratio |
| --- | ---: | ---: | ---: | ---: |
| sphere-cut-with-torus | 1 076 759 | 3 137 000 | **2.91x** | 1.53x |
| sphere-box-fuse | 273 754 | 1 087 095 | **3.97x** | 1.09x |
| box-fillet-preview | 102 094 | 136 410 | **1.34x** | 1.03x |
| box-shell-preview | 2 669 | 2 804 | 1.05x | 1.02x |
| rectangle-extrude-volume | 782 | 782 | 1.00x | 0.99x |

Read: the hash grid's nine `HashMap` probes per candidate (SipHash, `opt-level = 0`) cost far more
than the linear scan they replace at the point counts these examples produce, and that swamps the
allocation saved in the cavity. The asymptotics are right and the constants are wrong *for this
profile*; a correct version needs an open-addressed or flat index rather than `std`'s `HashMap`, and
must be measured in the `wasm-dev` profile the guest actually runs, not only at `opt-level = 0`.
I am not claiming the rework as an improvement, so it is out of the tree.

---

## 8. Architecture debts located but NOT fixed

Stated so they are not mistaken for done.

1. **`ear_clip` is O(n³) in exact-predicate calls** (`💡️inferences/📏mass-properties/🦀️.rs:708`): the
   ear search scans every corner and each candidate ear is tested against every remaining vertex.
   The predicates are now ~100x cheaper each, which is why this stopped being the bottleneck, but the
   complexity is unchanged. It needs an AABB/strip prefilter on the containment test.
2. **`stitch_selected_faces` runs a full mass-property quadrature to get a SIGN**
   (`🔺️diff/🔀️boolean/🦀️.rs:1981`): `shell_signed_volume(body, shell, 1e-3)` — adaptive quadrature
   over ear-clipped UV triangulations of every face, at a `1e-3` chord tolerance — and then uses only
   `< 0.0`. The sample count scales as `1/sqrt(tol)`, so the tolerance is buying three digits nobody
   reads. This is the single largest remaining item in `evaluate`.
3. **The imprint phase is O(|faces_a| x |faces_b|) with no spatial index**
   (`🔺️diff/🔀️boolean/🦀️.rs:729-771`): `imprint_face_pair` does reject on an AABB overlap, but it
   recomputes `face_aabb(body, fb)` — which walks the face's loops, coedges and curves — **inside**
   the pair loop. Both the recomputation (should be an O(A+B) prepass) and the pairing (should be a
   BVH query) are addressable. It is not a bottleneck for the eight bundled examples, whose solids
   have 1–6 faces each; it will be for anything realistic.
4. **Preview mesh ordering is nondeterministic** — see §6.2.
5. **No cross-tick memoisation of unchanged upstream nodes.** The ticket brief names it; the evaluate
   chain still re-evaluates from the top each tick. It was not reached: after (2) and (3) the
   evaluate phase is small enough that the win is in the tessellator instead.

---

## 9. Files

Changed:

* `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `FlowHostRetirement::close_page`
  pays `FlowRetirement`'s frontier reservation before closing (fixes the 0/17 spin).
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➡️vector/⚖️predicates/🦀️.rs`
  — exact path rewritten as expansion arithmetic; module doc records why.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/➡️vector/⚖️predicates/🧪️tests/🔬️unit/🦀️.rs`
  — new `oracle` module, five third-party laws.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust/Cargo.toml` — `num-rational` added as
  a test-only oracle; `semio-framework-number` dropped (no longer used by this crate).
* `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs`
  — phase timing, `[BUDGET]` line, budget laws, retry-under-contention, cold mesh cache before the
  timed tessellation.
* `…/📚️examples/🧪️tests/🧩️geometry/🟦️.ts` — `ExampleBudgetExpectation`, `assertBudgetContract`, the
  two ceiling constants; also removed the twin's stale `blocked-on-fillet-kernel` standing, which the
  Rust side had already dropped.
* `…/📚️examples/<all eight>/🧫️fixtures/🧩️example/🔣️.json` — `budget` row.
* `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🔮️oracles/🔣️.json` —
  `num-rational-brep-exact-predicates` registered, so the dependency truth gate classifies it
  `test-oracle` and the oracle-conflict count does not move (§6.5).

Added (ticket folder):

* `🐍️example-phase-timings.py` — min-of-N phase timing harness (the contention-robust measurement).
* `🐍️example-budget-fixtures.py` — writes the `budget` rows from captured run logs.
* this report.

Reverted after measurement: `…/🧊️brep/🧬️schema/💡️inferences/🧩tessellation/🦀️.rs` (§7).

---

## 10. What is NOT claimed

* **Nothing here was measured in the browser.** I did not restage the plugin and did not run the
  React playground — two other lanes own restaging, and this lane's brief is to prove the work
  natively and by the shared laws. The browser numbers in `📓️summary-2026-09-12.md` (torus 78 s) are
  *not* re-measured here. What is established is that the identical code, on the identical inputs,
  costs 14x–53x less than it did; the playground figure should follow, and the next restage will
  show by how much.
* **No claim that every example is under 10 s in the playground.** That is a browser measurement and
  belongs to whoever restages next. The native user-facing pair is now ≤ 456 ms worst case.
* **No claim that the tessellator was improved.** It was not; the attempt is reverted (§7) and it is
  now the largest remaining cost.
* **No claim about `wasm-dev` profile behaviour.** Every number here is `opt-level = 0` native. The
  predicate change removes allocation and bignum work, which is profile-independent in kind but not
  in magnitude.
* **No claim that the boolean or tessellation algorithms are asymptotically fixed.** §8 lists four
  located, unaddressed complexity defects.
* **Timing numbers carry real uncertainty.** This machine ran at load 23–198 throughout. Minima over
  N runs and interleaved A/B are what every comparison rests on; single readings in the raw logs vary
  by up to 19x and should not be quoted.
* **No claim that `cargo test -p semio-s-artifact-stdio-semio` is green.** It is 2474/47. §6.4 shows
  those 47 reproduce against the pre-rewrite code, so they are not this lane's — but they are also not
  fixed by this lane, and nobody should read "17/17 example-geometry" as "the kernel crate is green".
* **No claim that the dependency truth gate is green.** It is red at `literal-external=194`,
  unchanged by this lane (§6.5).
