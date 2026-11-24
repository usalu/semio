# Geometry job cost: emblem plus one stroked ellipse (2026-11-24)

Read-only analysis of the isolated job `semio.draw.mounted-vector`. Nothing was compiled or run; every count below is derived by reading code, and the ones marked (estimate) must be confirmed with the job's own work counter (see "How to measure").

Paths: D = `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any`; S = `D/🧬️schema/🎬️scene`; F = `🧰️framework/🔨️modules`.

## 0. Headline

1. The premise "each step computes up to 8 ms" is wrong. The guest crossing gets a fresh 2 ms wall slice and 6,000,000 wasm instructions per step (section 2). At 2 ms, 1,230 steps are at most about 2.5 s of guest compute. The observed 20 to 30 s therefore means roughly 85 to 90 percent of the wall time is host cadence (about 16 ms per crossing, 3 outer job turns per crossing), not geometry. Both effects multiply: the amount of work (section 1) and the duty cycle (section 2).
2. Work amount: the ellipse costs about 1,000 flatten points at tolerance 0.001, and the stroke is built as a union of 2,048 tiny primitive polygons by a 4-level union tree whose classification does a ray query per atomic edge. Bookkeeping (copying points, grants, receipts) is linear and about 1 to 2 percent of the iterations; the boolean unions dominate.
3. No safe constant or bookkeeping change exists that cannot alter prepared geometry or committed oracles. No source file was edited. Everything actionable is a proposal below.

## 1. Phase cost for the ellipse (rx 64, ry 48, stroked)

Assumption: the ellipse reaches the paint job as `Move`, 2x `Arc` (half arcs, rotation 0), `Close`, as the SVG importer builds it (`D/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/📄️document/🦀️.rs` lines 93 and 99). I did not find the editor shape-to-path conversion; if the editor emits cubics the counts shift but not the conclusion. Layer transform assumed identity.

### 1a. Flatten (`F/◻️2d/🛤️path/📏️flatten/🦀️.rs`)
- An arc node is accepted when `span <= PI` and `2*bound*sin^2(span/4) <= tolerance`, with `bound = |A u| + |A v| = rx + ry = 112` for identity. Otherwise it splits in half (binary, depth up to 32).
- Tolerance 0.001 needs span <= 0.00845 rad. A half arc has span PI, so depth 9: 512 points per half arc, 1,024 points per ellipse. Flatten iterations: one per tree node, (2^10 - 1) per half, about 2,046 for the ellipse.
- Computed table (same formula):

| tolerance | depth | points per ellipse | flatten iterations |
|---|---|---|---|
| 0.001 (today) | 9 | 1024 | 2046 |
| 0.003125 (0.1 / zoom 32) | 8 | 512 | 1022 |
| 0.01 | 7 | 256 | 510 |
| 0.1 (0.1 / zoom 1) | 6 | 128 | 254 |
| 0.25 | 5 | 64 | 126 |

Doubling is by powers of two, so the point count only changes in steps of 2.

### 1b. Scene bookkeeping (`S/🎨️paint/🦀️.rs`, `S/🎨️paint/📋️prepare/🦀️.rs`)
Linear, one iteration per item, each iteration being a full `advance(1)`:
- `contours`: 1 iteration per point = 1,024 (copy flat points into stroke input).
- `fill` (only if fill): 1 per point + 1 begin = about 1,025.
- `polygons`: 1 per outline polygon + 1 per polygon point (see 1c) = 2,048 + 7,168 = about 9,200.
- `strokeCanonical` grouping: 1 per polygon = 2,048 (inside the union job).
- `strokeAccount`: 1 per remaining point, about 2,000 after the union.
Total bookkeeping about 13,000 iterations. Not quadratic; no repeated scans were found (all pops are from the end, vectors grow by doubling).

### 1c. Stroke outline (`F/◻️2d/🛤️path/🖊️stroke/🦀️.rs`)
- Closed contour, n = 1,024 edges. Per edge: one rectangle polygon (4 points) and one join. Round join emits a `Round` primitive; its sweep is about 2*PI/1024 = 0.0061 rad, so the arc tolerance test passes at the first node and the wedge polygon is `[center, start, end]` (3 points). Per edge the outline job runs 5 iterations (enqueue, rectangle, round birth, round point, round done) = about 5,100 iterations.
- Output: 2,048 polygons, 4,096 + 3,072 = 7,168 points. Round caps do not apply to a closed run.

### 1d. Union of the stroke (`S/🎨️paint/🦀️.rs`, `StrokeCanonicalJob`, plus `F/◻️2d/🔀️booleans/🦀️.rs`)
This is the dominant phase. The 2,048 primitive polygons are merged by a union tree: wave 1 groups of 4 (`first` wave) = 512 `BooleanJob`s of about 14 edges; later waves groups of 16: 32 jobs, then 2 jobs, then 1 job. Each `BooleanJob` is: edge insertion, Morton-sorted BVH build (heap push, pop, levels), BVH self-intersection query per edge, atomic edge splitting with grid interning and B-tree style index inserts, then classification per atomic edge: a nearest-edge query, an axis-aligned ray query over the BVH, a winding fold per operand, then boundary tracing, compaction, emit.
Per-edge classification is the cost centre: a ray query visits every BVH node whose box intersects the ray strip, which is O(sqrt n) for a ring-like arrangement, so each level costs O(n^1.5), and each of the 4 waves re-classifies all edges that survived the previous wave.

Estimated iterations (estimate, not measured; each BooleanJob step is one outer iteration):
- wave 1: 512 jobs x about 1,000 = about 0.5M
- wave 2: 32 jobs x about 14,000 = about 0.45M
- wave 3: 2 jobs x about 80,000 = about 0.16M
- wave 4: 1 job x about 160,000 = about 0.16M
- fill canonicalisation (single 1,024-edge polygon union through `begin_canonical`): about 0.15M to 0.2M
Total about 1.3M to 2M outer iterations (estimate). The fill union alone costs more than all scene bookkeeping combined; the stroke union costs about 7x the fill union.

Scaling: the dominant terms are about n log n to n^1.5 in the point count n. Halving n (tolerance x4) cuts iterations by about 2.5x to 3x; 8x fewer points (tolerance 0.1) cuts them by about 20x.

### 1e. Per-iteration overhead
Every iteration calls `ScenePaintJob::advance_original` which calls `advance_original_scene` which builds a `RetainedWorkLedger`, opens an original effect (`StepContext::with_original_effect_at` in `F/🧵️job/⏱️context/🦀️.rs` line 353: loan, prepare, issue, claim, settle, commit, finish) and folds a receipt, for a single `advance(1, ...)` unit (`S/🎨️paint/📋️prepare/🦀️.rs` `advance_original`, `self.advance(1, work, effect)`). The per-unit effect machinery is likely larger than the boolean unit it pays for, especially in an `opt-level = 1` guest with debug assertions. Also `GeometryJob::advance` reads the raw clock twice per iteration (`semio_framework_job::default_now_us`, `S/🧵️mounted/🦀️.rs` line 88), and the effect `ready` closure reads it again; `StepContext` has a strided clock (`ClockStride`, 64 us target interval) that the loop does not use.

## 2. What a step charges and when it ends

`GeometryJob::advance` (`S/🧵️mounted/🦀️.rs` lines 85 to 111):
- `deadline = min(start + 8000 us, cx.deadline_us())`. `cx.deadline_us()` is set by the host, not by this file: `step_guest_job` builds fresh `JobFunds` per step (`F/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🎟️instance-slots-leases-and-cancel-helpers/🦀️.rs` lines 178 to 190) with `wall_us = RELAY_JOB_BUDGET.deadline_ms * 1000`, and `RELAY_JOB_BUDGET.deadline_ms = USER_VISIBLE_LANE_WALL_US / 1000 = 2` (`.../🏊️pool-future-requests-and-completion-slot/🦀️.rs` line 7; constants in `F/🧵️job/🕰️clock-identity-and-budget/🦀️.rs` lines 166 to 175). So the effective deadline is about 2 ms after the host issued the crossing; the 8000 in the guest is dead code. The host test `the_host_watchdog_is_not_the_guests_cooperative_grant` pins this 2 ms grant.
- Fuel: `work_fuel = RELAY_JOB_WORK_FUEL = 1 << 24` logical units (`cx.fuel_remaining()`), and wasm instruction fuel `USER_VISIBLE_LANE_FUEL = 6,000,000` per crossing (`JobFunds::self_issued(..., RELAY_JOB_WORK_FUEL, RELAY_JOB_BUDGET.fuel)`). The wasm instruction budget is the only fuel that could end a 2 ms slice early on an unoptimised guest; it is not checked in guest code (it is enforced by the host). The 65,536 iteration cap in the loop never binds.
- Retained grant per crossing: `JobFunds::SELF_ISSUED_GRANT` = 1M items, 64 MiB copy, 64 MiB capacity, 64 MiB release, depth 64; it does not bind for this job.
- Per iteration charge: `advance_original_scene` folds `cpu = max(last_turn_work, performed)`; `ScenePaintJob::advance` adds at least 1 per iteration (`if self.last_turn_work == before_cpu { += 1 }`) and `GeometryJob` adds the same into `state.work`, which is also the progress payload (little endian u64) emitted when it changed. Children add their own `last_turn_work` (boolean `step_work.max(1)`, flatten, stroke).
- A step ends early on: the 2 ms cx deadline (checked by the guest loop and by `cx.should_yield()` inside `advance_original_scene`, which returns the idle result), fuel exhaustion (`should_yield` includes `fuel_exhausted`), `actual == 0` which returns `Running(None)` at once (effect not admitted, ledger blocked on an absent grant, or `cx.should_yield()`), `shell.try_borrow_mut()` failing (a concurrent query borrowing the shell: `Running(None)` at once, zero work), and any error or completion.
- `if actual == 0 { return Running(None) }` appears in both branches (producer: `S/🧵️mounted/🦀️.rs` line 111; cache: line 102). Ending the step on zero work is correct, but it wastes an entire 16 ms host cadence if it triggers spuriously, and the code never records which cause it was.
- Cadence: the host relay job (`F/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🔀️post-turn-relay/🥶️cold-relay-job/🦀️.rs` lines 100 to 150) alternates: submit request, yield, poll the completion slot (yield while pending), take completion, yield. One guest crossing therefore costs at least 2 to 3 outer job turns, which is consistent with the measured 16 ms between steps against at most 2 ms of compute. A guest slice of 2 ms of 16 ms is a duty cycle of about 12 percent.

## 3. Tolerance 0.001

- Justification found: none in comments. It appears as a literal in `S/🧵️mounted/🦀️.rs` line 102 and as the value in tests (`S/🎨️paint/📋️prepare/🧪️tests`, `D/✏️editor/🧪️tests/🔬️unit/🎛️selection-laws`, `D/✏️editor/🧪️tests/🔬️canvas-tool`, `S/🎨️paint/📋️prepare/🎯️query/🧪️tests`), so oracles and fixtures were written against 0.001. Export uses 0.005 (`D/🚪️io/📤️export/📦️owned/🦀️.rs` line 86). The valid range is 1e-6 to 16 (`precision()`, `validate`, flatten).
- The only rule in the code that ties flatness to the viewport is picking: `trace-pointer-job` (`D/✏️editor/🪆️1-any/🎮️commands/🖱️canvas-pointer-down/🧵️trace-pointer-job/🦀️.rs` line 244) requires `required_flatness = clamp((8 px / zoom) / 80, 1e-6, 16) = 0.1 / zoom`, and `PreparedRegionQueryJob::new` refuses prepared regions with `stamp.flatness > required_flatness`. 0.001 therefore satisfies picks up to zoom 100, which is 3x beyond the maximum.
- Canvas zoom range: `ZOOM_MIN = 0.05`, `ZOOM_MAX = 32.0` (`F/🖱️ui/🎨️styling/🔤️tokens/🦀️.rs` lines 175 to 176; `CANVAS_CAMERA_ZOOM_MIN/MAX` in `F/🖼️canvas/📷️camera/🦀️.rs`). The pick constant is `DRAWING_PICK_TOLERANCE_PX = 8.0`.
- Other tolerances in the repo: pick 8 px / zoom; Boolean topology epsilon `max(1e-12, magnitude * f64::EPSILON * 32)` which must stay <= flatness / 16 (`topology_epsilon`), so a coarser flatness is always admissible there; export 0.005; simplify tolerance range 1e-6 to 1e6.
- Viewport-relative rule: device error = flatness * zoom pixels. A budget of 0.1 px at the maximum zoom gives flatness = 0.1 / 32 = 0.003125, which also satisfies the pick rule at every zoom (0.1 / 32). For this ellipse that is 512 points (2x fewer). A zoom-following rule `flatness = 0.1 / zoom` gives 0.1 at zoom 1 (128 points, 8x fewer, about 20x less boolean work) and 2.0 at zoom 0.05. But the stamp, the `with_visual` borrow and `PreparedRegionStamp.flatness` are exact-match contracts, so zoom must drive a rebuild and the two levels of detail must coexist; that is a redesign (proposal 3), not a constant change.

## 4. Per-layer reuse

- No per-layer cache exists. `reconcile` (`S/🧵️mounted/🦀️.rs`) starts a new `State`/`GeometryJob` for every new `SceneIdentity` (instance, base, generation, canonical revision); the previous candidate is cancelled; the last complete `PreparedScene` stays borrowable via `with_visual` until the new one completes, so the canvas shows the old picture, not nothing. Every document revision therefore re-runs `DocumentVectorJob` (about 90 steps for the emblem) and every layer's paint from zero.
- Reusable pieces exist: `PreparedScene.geometry[i]` aligns with `plan.nodes[i]`; each `PreparedGeometry` holds `PaintedPathRegions` (Clone, PartialEq); nodes carry `id`, `transform`, `content`. Missing: `DocumentSceneNode`/`DocumentSceneContent` have no `PartialEq` or content digest; `PreparedGeometry` is not `Clone`; no job can borrow the previous scene while building the next (each scene owns its plan and sits in its own shell); retirement ladders (`PreparedSceneCloseJob`) assume sole ownership.
- Clean long-term design: a node-level content-addressed geometry cache. Key: (flatness, node transform, fill flag and rule, stroke style, segment list digest). The paint job asks the cache before starting `PaintedPathPrepareJob`; on a hit it admits a clone of the stored `PaintedPathRegions` charged as copy bytes. The cache lives beside the registry per instance (not inside one `State`), bounded by points, evicted by identity. A document edit then costs only the changed layers. This is the largest single win (the emblem plus an unchanged ellipse would redo nothing when a third layer changes). It needs an event-sourced layer revision (the document already knows which layer an event touched) as the invalidation source instead of a content digest.

## 5. Build profile

From `F/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🦀️profile/🧬️schema/🔣️.json` (settings table):
- `prototype-wasm` (the iterated plugin guest, cargo profile `wasm-prototype`): `optLevel "1"`, `debug 0`, `codegenUnits 256`, `incremental true`, `lto off`; preserved: debug-assertions and overflow-checks stay on, so the guest runs with overflow checks and assertions at opt-level 1, with no cross-crate inlining (256 units, no LTO). The older copy of the root manifest in the ticket tree also lists per-package overrides to opt-level 2 for flow, neural, replication and pack; none for the 2d crates.
- `prototype-component` and `release-wasm`: `optLevel "s"` (size), thin LTO, 16 codegen units, no debug assertions. Release is size-optimised, not speed-optimised: a release guest will be faster than the prototype by the effect of LTO, inlining of the ledger/effect closures and removal of checks, but is not opt-level 3. `release-native` is `optLevel 3`.
- So the profile accounts for a constant factor on the order of 2x to 5x (estimate; the ledger and effect layers are deeply generic and benefit most from LTO and inlining) and not for the 100x seen between emblem and ellipse; work amount and the 2 ms duty cycle explain that.

## 6. Edits made

None. Candidates considered and rejected as not safe without compiling or measuring:
- Dropping the dead `8_000` in `GeometryJob::advance`: no effect.
- Replacing the two raw clock reads per iteration by `cx.should_yield()`: small, unmeasurable gain, timing-only, but the effect `ready` closure reads the raw clock anyway.
- Changing 0.001 or the wave sizes (4 then 16): changes prepared geometry, vertex order and committed oracles.
- Batching units per effect: see proposal 1; uncertain accounting.

## 7. Proposals, ordered by value over risk

1. Measure first (no code risk): read the job's own work counter. `GeometryJob` already emits `state.work` as the progress payload; the existing `probe-latency-trace.mjs` can log it per step. Report: total work units at completion, work units per step, and wall per step. That separates "too many units" from "too few units per 16 ms" and tells whether (2) or (3) below matters most.
2. Raise the guest slice for this job kind or batch more per crossing. The host grant (2 ms, 6M instructions) is the lever for the duty cycle: for a background geometry kind a 4 ms to 6 ms slice, or letting the relay job step again immediately when the guest returns `Running` with work done (no yield between crossings), would multiply throughput by up to 3x to 6x without touching geometry. This is a host policy change in `RELAY_JOB_BUDGET`/`cold-relay-job` and needs the 8 ms interactive-step contract reviewed (the comment in the clock module calls 8 ms the hard ceiling).
3. Batch units inside one effect: add `ScenePaintJob::advance_original_batch(cx, units)` that calls `self.advance(units.min(fuel), ...)` inside the one `advance_original_scene` effect (the function already takes `budget`). Must cap `units` by `cx.fuel_remaining()` (an effect that performs more work than its ceiling is refused) and keep the `actual == 0` rule. Expected gain: removes most of the per-unit ledger and receipt setup (section 1e). Not applied because accounting equality with the existing work-law tests (`S/🎨️paint/📋️prepare/🧪️tests/🎟️work`) cannot be verified without running them.
4. Stroke without a 2,048-polygon union: for a stroke with no dashes, closed or open, with round or miter joins, the exact offset outline can be built directly (left and right offset polylines plus caps) and only self-intersections need a union. That removes about 90 percent of section 1d. It changes output order and so needs new oracles; do it behind the schema-first geometry contract with a second implementation (third-party library) as the language-agnostic test required by CLAUDE.md.
5. Viewport-relative flatness with levels of detail (section 3), preparing a coarse level first (0.1 / zoom at the current bucket) and refining in the background, stamped per level.
6. Node-level geometry cache and layer-granular invalidation (section 4).
7. Profile: add `[profile.wasm-prototype.package.<2d crates>] opt-level = 2` for the flatten, stroke, booleans and `semio-framework-value-persistent` work ledger crates, following the existing precedent for flow and neural; no behaviour change, only the iterate profile. The build tool authored this in `interactive-wasm-package-constraints`; I did not find where that list lives in the tree.

## How to measure (nothing needs a browser pane from me)
- Total work units: sum of the progress payloads (u64 LE) or the last one; with the step count that gives units per step.
- Guest time per crossing: a `[DEBUG]` line in `GeometryJob::advance` printing `default_now_us() - started` at return, once per step, not per iteration.
- Boolean breakdown: `BooleanJob::progress()` exposes the phase; sample the phase histogram while the job runs.
