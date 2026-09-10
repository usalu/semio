# Wave S2 — where the 15–35 s between the example click and the world repaint is spent

Ticket `26/09/02/PUZZLE-3D-END-TO-END`, wave W-S2 (continuation of W-P / W-P4 / W-R / W-P5), 2026-09-10.
Written incrementally while the wave ran — measurements first. Predecessors:
`📓️2026-09-09-wave-P-paged-scene-payload.md`, `📓️2026-09-10-wave-P4-lane-intake.md`,
`📓️2026-09-10-wave-R-intake-delta-cost.md`, `📓️2026-09-10-wave-P5-lanes-render.md`.

## 0 Conditions

- Repo `/Users/ueli/Documents/semio`, detached HEAD, live tree shared with peers. No `git commit` /
  `stash` / `checkout` was run; the ticket is NOT closed; `🗑️generated` was not touched.
- The repo MCP server did not connect this session (`repo (-32602): invalid initialize params`), so the
  ticket folder is managed on disk.
- Private cargo target `…/scratchpad/target-p3d-f`, `RUSTC_WRAPPER=""`, `CARGO_INCREMENTAL=0`,
  `RUST_MIN_STACK=134217728`. Every cargo/vitest run backgrounded to `…/scratchpad/ws2-*.txt`.
- `grep -a` throughout (BSD grep calls the emoji-bearing sources BINARY, W-P4 §1).
- Machine load average 47 during the whole wave.
- The dev target at `:6013` was down for most of the wave (nothing LISTENing while a
  `bun … script.ts dev` rebuild was in flight). It answered for one window, which is when the
  before-measurement of §2.3 was taken with this wave's own probe
  (`🔍️ws2-scene-latency-probe.ts`); by the time the fix was in, the target no longer booted a page
  (`start []` after 187 s), and it serves a stale wasm anyway — see §7.
- The whole workspace was rebuilt from low-level crates repeatedly by peers' edits, so every cargo run
  in §6 cost 10–20 minutes; two of the queued gate runs were dropped at hand-over and are named there.

## 1 The inherited symptom

W-P5 established, by three headless probe runs against the live release target, that the Nakagin
switch is not broken but SLOW: `setActiveExample` acks at the guest in ~0.6 s, the following
`refreshUi` in ~0.6 s, the inspection panel repaints inside ~1 s, and `World3dHost`'s
`data-instances-json` goes from 266 B / 1 instance to 54 254 B / 180 instances **15–35 s after the
click**, on every run. Also `scene.fitJson` is never published, so a document swap never refits the
camera.

This wave attributes that gap and fixes the dominant term.

## 2 Measurements

### 2.1 The guest encoder is 0.002 % of the gap

`cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly ws2_measure` (a temporary
`[DEBUG]` measurement in `🔬️example-switch`, native **debug** build, load 47):

```
[DEBUG] ws2 encode forest:  instances_json=270B in 79us   split_lanes=3 lanes in 19us  scene_surface=3 children in 276us
[DEBUG] ws2 encode nakagin: instances_json=55154B in 6041us split_lanes=3 lanes in 241us scene_surface=3 children in 557us
[DEBUG] ws2 render round 0: 378009us nodes=26 payload=56794B
[DEBUG] ws2 render round 1: 364478us … round 2: 99189us … round 3: 181753us … round 4: 272192us
```

So for the 180-object Nakagin document: `split_lanes` **241 µs**, the whole `scene_surface`
(split + 11 paged carriers + the spine encode) **557 µs**, and a full guest render of the composite
window body (which contains it) 99–378 ms in an unoptimised native build. The `instances_json`
build (6 ms) is cached in production behind `fixture_geometry_fingerprint`. Two windows: well under
1 s, matching the 0.6 s guest ack W-P5 measured. **The encoder is not the gap.**

### 2.2 The host intake of the real (packed) surface is ~0.4 s, not 2.3 s

W-P4/W-R measured 669 403 / 671 321 intake steps for a Nakagin-scale surface, but on a SYNTHETIC
145-node tree of UNPACKED 512-byte leaves. Production packs 33 slices per leaf
(`section_text_chunks`), so the real surface is **26 nodes** (measured: `Nakagin world-3d surface
presented 26 nodes`). Driving that exact shape through the real `OwnedUiPatchIntake`
(`📃️UiDocumentStore/🧪️tests/🧪️typedwire`, temporary `[DEBUG]` measurement):

```
[DEBUG] ws2 packed first publication  nodes=26 carried=56794 steps=105416 close=2
        symbol-edit=43674 validation=19230 input=13811 scenes=6659 symbol-old-close=4283 typed-normalize=1891
[DEBUG] ws2 packed one-lane republish records=5              steps=18565  close=2
        symbol-edit=8821 input=1978 scenes=1232 symbol-old-close=867 symbol-lookup=760 hash=601
```

**105 416 steps** for the first publication (6.4× cheaper than W-R's synthetic figure), ≈ 0.36 s at
the 3.4 µs/step W-P4 measured in-browser, i.e. ≈ 0.7 s for both windows; 18 565 steps for a one-lane
republish. At `PLUGIN_UI_INTAKE_YIELD_STRIDE = 1024` that is 103 macrotasks, not 653.

### 2.3 The browser: 24.3 s, of which 5.4 s is main-thread CPU and 8 799 worker messages

`🔍️ws2-scene-latency-probe.ts` (this wave, kept in the ticket folder) against the live release target
at `:6013` — a separate headless chromium, no server started, no interactive tab touched. It adds
three instruments no source edit can be needed for: a `longtask` PerformanceObserver, a
`Worker.prototype.postMessage` counter (one entry per host→shard message, i.e. per guest turn round
trip) and console capture.

```
[35.1s] → nakagin at page 34265ms
[59.4s] landed after 24.3s: [{"bytes":54254,"instances":180},{"bytes":54254,"instances":180}]
[60.7s] gap=24.3s longtaskCpu=5.4s tasks=21 workerPosts=8799
[60.7s] perSecond=["0s:216ms/2t/574p","1s:251ms/2t/73p","2s:144ms/1t/468p","3s:0ms/0t/461p","4s:585ms/1t/48p",
         "5s:145ms/1t/474p","6s:205ms/1t/503p","7s:177ms/1t/496p","8s:245ms/1t/506p","9s:0ms/0t/77p",
         "10s:476ms/1t/429p","11s:280ms/1t/506p","12s:0ms/0t/110p","13s:202ms/1t/840p","14s:448ms/1t/65p",
         "15s:397ms/1t/469p","16s:301ms/1t/681p","17s:0ms/0t/1092p","18s:257ms/2t/634p","19s:0ms/0t/2p",
         "20s:0ms/0t/10p","21s:545ms/1t/102p","22s:0ms/0t/108p","23s:531ms/2t/71p"]
```

and in the same window the runtime's own trace:

```
53.4s [DEBUG] settle puzzle#1 continuation 512  status=more-work acks=0 drain=true
53.9s [DEBUG] settle puzzle#1 continuation 1024 status=more-work acks=0 drain=true
54.3s [DEBUG] settle puzzle#1 continuation 1536 status=more-work acks=0 drain=true
```

Three facts follow, and together they name the cost:

1. **8 799 worker messages in 24.3 s** — ~360/s, i.e. the gap is ~4 400 host↔guest turn round trips
   (a `submitPluginTurn` post plus its reply), at ~5 ms each.
2. **The guest asks for those turns.** `settlePluginTurn` runs a `drain=true` continuation loop and
   the trace shows 1 536 consecutive continuations in 0.9 s with **`acks=0`**: the guest keeps
   answering `MoreWork` while publishing NOTHING.
3. **The main thread is not the bottleneck**: 5.4 s of long tasks in 24.3 s (22 %) — the intake
   (§2.2), the projection and the React commit. The other 78 % is round-trip latency.

This is the same shape the 2026-09-09 20:35 live trace already recorded for this exact document
switch (`📓️2026-09-09-runtime-verification.md`): `reactor more-work streak=8192 … typed_operation=false
**reconcile=true** resumes=false executor_pending=false command_ingress=false lifecycle=false
effects=0` — the retained-surface reconcile is the ONLY pending source, for thousands of turns.

### 2.4 What holds `reconcile` true: retirement priced at ONE item per reactor turn

Reading the turn (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs`) with those numbers in hand:

```rust
PATCHES.with(|patches| { for _ in 0..PATCH_CLOSE_UNITS_PER_TURN { if patches.close_step() { break } } });   // 8 units
…
with_pending_patches(|pending| pending.borrow_mut().close_step())                                            // ONE unit
…
let reconcile_work = … Ok(more || patches.has_publishable_work() || with_pending_patches(|p| p.borrow().has_unpublished()));
let more_work = … || reconcile_work || …;
```

- `PendingPatchAuthority::has_unpublished()` is TRUE while any slot is `acknowledged` — i.e. from the
  moment the host ACKs a published patch until that patch's owner is fully retired.
- `PendingPatchAuthority::close_step()` retires exactly `close_slot_step(index, 1, 4096)` — **one item
  per call** — and the turn calls it **once**.
- `PatchTracker::close_step()` likewise returns after ONE unit, and the turn drives at most 8; every
  unit underneath it is hardcoded to one item too (`ReadySlot::close_step` →
  `outputs.close_step(1, 4096)`; `SurfaceReconcileTerminal::close_step` →
  `close_step_with_grant(1, 4096)`; and `close_surface_patch_owner` passes a literal `1` for items
  even when its caller granted more).

So **retiring one published world-3d patch costs one host round trip per eight retirement units — or
per single unit on the pending and kernel ladders**. Everything
else in the same turn is priced per PAGE (`SURFACE_RECONCILE_PAGE_BYTES` = 32 KiB) or per 1 024
reconcile opportunities; only retirement is priced per item per turn — and each of those turns is a
round trip the user waits for.

### 2.5 The count: 2 255 reconcile steps (3 turns) against 1 091 retirement units (1 091 turns)

Driven natively through the REAL `PatchTracker` with the reactor's own per-turn budget
(`⚛️reactor/🧪️tests/🔬️reconcile-budget`, `cargo test -p semio-framework-plugin --lib`):

```
[DEBUG] ws2 reconcile turns=3 steps=2255 opportunitiesPerTurn=1024
[DEBUG] ws2 retirement ops=27 patchUnits=1091 readyUnits=0 publishedUnits=7 trackerCloseTurns=1
[DEBUG] nakagin publication reconciled in 3 turns (2255 steps) and retires in 5 turns / 1092 units,
        against 137 turns / 1092 units at the pre-W-S2 pacing
```

**The attribution table for the 24.3 s gap:**

| term | measured | turns / round trips it costs | share of the gap |
| --- | --- | --- | --- |
| guest `split_lanes` + `scene_surface` (11 carriers, 56 794 B) | 557 µs (native debug) | 0 | ~0 % |
| guest render of the whole composite body | 99–378 ms ×2 windows | inside the same turn | ≈ 3 % |
| **reconcile of the published surface** | **2 255 steps → 3 turns** | 3 ×2 windows | ≈ 0.1 % |
| **retirement of the emitted patch** | **1 092 units**, at 8 units/turn (`PATCHES`) and ONE unit/turn (pending authority, kernel turn-patch arena) | **137 ×2 windows, and worse on the one-unit ladders** | **≈ 45–60 %** |
| host intake of the lane set | 105 416 steps ≈ 0.36 s ×2 | 1 ack round trip each | ≈ 3 % |
| host projection + React commit + everything else on the main thread | (5.4 s of long tasks total, incl. intake) | — | ≈ 22 % |
| the remaining round-trip latency (other surfaces, panels, sections, repeated Full refreshes) | 8 799 worker messages total | — | rest |

The reconcile — the thing three previous waves optimised — is 3 turns. The retirement of the very
same patch is 137. That is the defect.

## 3 The fix: retirement is bounded work per TURN, not one item per turn

The architectural statement, and the only one this wave makes:

> Every stage of a reactor turn is priced per PAGE — the reconciler publishes
> `SURFACE_RECONCILE_PAGE_BYTES` at a time, the intake takes 256 items / 64 KiB at a time, the
> reconcile loop gets 1 024 opportunities and the turn's own wall-clock budget. Retirement was the
> single stage priced per ITEM per TURN, and a turn is a host round trip. It is now priced the same
> way as everything else: a bounded RUN of page-sized units, cut short by the turn's own deadline.

| file | change |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` | `close_surface_patch_owner` passes the caller's `items` grant to `patch.close_step` instead of the literal `1` it hardcoded — every ready/published/acknowledged patch owner above it was already being handed a grant it then ignored. |
| `🔌️plugin/⚛️reactor/📨️pending/🦀️.rs` | `PendingPatchAuthority::close_step(items, bytes)` takes the grant and hands it to `close_slot_step`/`close_instance_step` (both already honoured one). New docstring: an acknowledged slot holds the whole turn in `MoreWork`, so its retirement rate IS the user-visible latency. |
| `🔌️plugin/⚛️reactor/🩹️patches/🦀️.rs` | `PatchTracker::close_step(items, bytes)` and `ReadySlot::close_step(items, bytes)` take the grant and pass it to `SurfaceReconcileOutputs::close_step` / the output reservation. |
| `🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` | `PATCH_CLOSE_UNITS_PER_TURN` 8 → **64**; new `PATCH_RETIREMENT_ITEMS_PER_UNIT` = 1 024 and `PATCH_RETIREMENT_BYTES_PER_UNIT` = `SURFACE_RECONCILE_PAGE_BYTES`; new `retire_until_complete` / `retire_while_progress` drivers run each entry of the turn's close ladder for a bounded RUN instead of one unit, and every run re-reads the clock every `PATCH_CLOSE_DEADLINE_STRIDE` = 8 units against the turn's own `budget.deadline_ms` — the same shape the reconcile loop twenty lines below already had. The pending-patch retirement (which was ONE unit per turn) and the `close_ui_document_page` / `close_ui_value_page` ladders now use their existing `…_with_grant` variants with the page grant. |

| `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` | new `close_ui_turn_patch_owner_with_grant(items, bytes)` (the old `…_one` is now a `(1, 4096)` call of it) and `UiTurnPatchRetireArena::close_one(items, bytes)`. This arena is where the emitted patch actually lands after the host returns it — the 1 091 units of §2.5 — and it hardcoded one item. |

Not touched, deliberately: `SurfaceReconcileTerminal::close_step` and `MountedTreeTerminal::close_step`
retire one owner per unit and have no grant variant; they are now driven 64 units per turn instead of
8, which is the same 8× the rest of the ladder gets without reaching into
`SurfaceReconcileRetained`'s own close state machine. `close_ui_turn_patch_transport_one` and
`close_table_rows_view_one` report progress in shapes this wave did not measure and were left alone.

## 4 The camera never followed the document — the `fit` lane

`World3dHost` has always had a one-shot auto-fit (`WorldAutoFit`, keyed `${fit.revision}:${meshes}`),
but **nothing in this editor ever published `scene.fitJson`**, so the camera after a document swap was
whatever the previous document left in `cameraJson`. Nakagin happens to sit inside Concrete Forest's
framing (bounds x[-23.45, 0] y[-12.55, 0] z[0, 39.58]); a fixture centred elsewhere would simply be
off-screen. W-P5 §7 flagged it; this wave publishes it.

- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs` — new
  `world3d_fit_json(revision, padding)`, the producer-side twin of the host's `WorldFitRecord`.
- `✏️editor/…/🪟️windows/🧊️main/🦀️.rs` — new `world_fit_revision(fixture)`: FNV over the fixture's
  **identity** (`schema`, `domain`, the kind catalogs), never its geometry, and `render` now publishes
  `scene.fit_json = world3d_fit_json(world_fit_revision(…), PUZZLE3D_FIT_PADDING)`.

The identity-vs-geometry distinction is the whole design: keyed on geometry, the camera would jump on
every object move; keyed on identity, it refits exactly when the document is replaced. The `fit` lane
is a `World3dSceneLane` already, so it rides as its own carrier and costs one more tiny lane.

## 5 The laws

### 5.1 Native — turn count (`🔌️plugin/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs`)

`a_nakagin_scale_world_publication_reconciles_and_retires_within_a_handful_of_reactor_turns` builds a
Nakagin-scale world scene, publishes it through the REAL `PatchTracker` at the reactor's own per-turn
opportunity budget, and then retires the emitted patch at the reactor's own per-turn retirement
pacing, counting TURNS — i.e. host round trips.

- `reconcile_turns <= 8` (measured 3, over 2 255 drive steps).
- `items > 512` — the patch must be document-scaled for the next clause to mean anything (measured
  1 091).
- `dripped_turns > 64` at the pre-wave pacing (one item per unit, 8 units per turn) — measured
  **137 turns**.
- `granted_turns <= 8` at `PATCH_CLOSE_UNITS_PER_TURN` units of `PATCH_RETIREMENT_ITEMS_PER_UNIT` /
  `PATCH_RETIREMENT_BYTES_PER_UNIT` — measured **5**.

Measured: `[DEBUG] nakagin publication reconciled in 3 turns (2255 steps) and retires in 5 turns /
1092 units, against 137 turns / 1092 units at the pre-W-S2 pacing` — **27× fewer round trips per
published surface**. The law reads the PRODUCTION constants, so it is the constants it pins:
restoring `PATCH_CLOSE_UNITS_PER_TURN = 8` puts it back at 137 and the last clause fails (measured at
the intermediate value 64: 18 turns, which also fails the ≤ 8 bound). Fails before / passes after.

### 5.2 Native — the fit lane (`✏️editor/🧪️tests/🔬️example-switch/🦀️.rs`)

`a_document_swap_republishes_the_camera_fit_lane_and_an_object_edit_does_not` asserts on the lane the
host actually reads (the reassembled scene, via `world_surface_carrier_census`): the boot document
publishes a `fit` carrier with `enabled: true` and a revision, the Nakagin switch publishes a
DIFFERENT revision, and deleting an object afterwards leaves that revision untouched. Fails before
this wave for the simplest possible reason — there was no `fit` lane at all, so the first clause
panics on the missing payload.

### 5.3 TS — host cost per lane set (`📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx`)

`OwnedIntake takes a whole packed world-3d lane set in at a bounded cost per carried byte` drives the
production shape (11 lanes at their measured byte counts, packed 33 slices per leaf, 26 nodes) through
a real `ShardClient` UI-patch authority and asserts `steps <= 3 × carriedBytes` and that a one-lane
re-publish stays under a quarter of the first publication.

Per-BYTE is the point: W-P4/W-R's laws are priced per NODE, which silently absorbs the one regression
that actually matters here — unpacking the leaves keeps the per-node cost and multiplies the node
count. Measured both ways: packed **105 416 steps / 56 794 B = 1.86 per byte** (passes); the same lane
set with one 512-byte slice per leaf — the shape `section_text_chunks` produced before the pack loop —
**626 490 steps = 11.03 per byte** (fails, `expected 626490 to be less than or equal to 170382`), and
it also fails the law's node-count clause at 135 nodes against 32.

## 6 Verification

Every run backgrounded to `…/scratchpad/ws2-*.txt`. Rust envelope: `RUSTC_WRAPPER=""`,
`CARGO_INCREMENTAL=0`, `RUST_MIN_STACK=134217728`,
`CARGO_TARGET_DIR=…/scratchpad/target-p3d-f`, `-j 4`, `--test-threads=2`.

| command | result |
| --- | --- |
| `cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4 example_switch` | **6 passed / 0 failed** — includes the new §5.2 fit law: `[DEBUG] fit lane revision forest=2383401358 nakagin=1178560245 after-edit=1178560245` |
| — the same law before the fit lane existed | **1 failed** — no `fit` payload on the assembled scene at all |
| `cargo test -p semio-s-artifact-puzzle-3d … -j 4 nakagin` | **not run to completion** — it was queued behind a peer-invalidated rebuild and stopped at hand-over. Its two load-bearing laws (`nakagin_world3d_surface_fits_reconcile_node_cap`, `the_nakagin_switch_assembles_every_object_onto_a_mesh_…`) live in `🔬️example-switch` and are inside the 6 passed above — so the extra `fit` carrier is already proven to stay inside `UI_DOCUMENT_NODES` |
| `cargo check -p semio-s-plugin-puzzle --target wasm32-wasip2` | **`Checking semio-framework-plugin` clean, 0 errors** at the point the run was stopped (it had passed the crate this wave changed and was grinding through unrelated stdio artifacts) |
| `cargo test -p semio-framework-plugin -j 4 --lib -- --test-threads=1 a_nakagin_scale_world_publication` | **1 passed / 0 failed** — the §5.1 law, `3 turns` reconcile, `5 turns / 1092 units` retirement against `137 turns` at the pre-wave pacing |
| — the same law at `PATCH_CLOSE_UNITS_PER_TURN = 64` | **1 failed**: `observed 18 turns over 1092 units against 137 turns at one item per unit and 8 units per turn` |
| `cargo test -p semio-framework-plugin -j 4 --lib -- --test-threads=1 a_settled_reactor_turn_retains_nothing` | **1 passed** — `[DEBUG] settled reactor turn retention: per_turn=0 B`: the extra per-turn retirement work retains nothing (it fails at 3 008 B/turn only when the whole 630-test lib runs at `--test-threads=2`, because `retained_heap_bytes()` is process-wide and another test's allocations land in the window) |
| `cargo test -p semio-framework-plugin -j 4 --lib -- --test-threads=1 patches` / `… pending` | **42 passed / 3 failed** and **16 passed / 2 failed**, built WITH this wave's grant threading. All five failures are the peer breakage named below (`app-definition.*`, `interactive-job.missing-factory`, `app id testkit-txn must be a canonical surface id`) and none is in `🩹️patches` or `📨️pending` themselves; the tests in those modules call `close_step(1, 4096)` explicitly, so the later `PATCH_CLOSE_UNITS_PER_TURN` 64 → 256 change does not touch them |
| `cargo test -p semio-framework-plugin -j 4 --lib` (whole lib) | **peer-blocked**: 54 failures, all from other lanes' in-flight work (`app-definition.interactive-job-classification: unclassified interactive command …`, `app id testkit-txn must be a canonical surface id`, `interactive-job.missing-factory`), plus a `CommandPageSet::try_new()` arity break in `🔬️plugin-runtime-paged-command-ingress` that a peer fixed mid-wave. None is in `🩹️patches`, `📨️pending`, `🔬️reconcile-budget` or any file this wave touched |
| `cargo test -p semio-framework-ui-scene -j 4` | **not run** — queued behind the shared target-dir lock all wave and dropped at hand-over; this wave changed one line in `♻️reconcile.rs` (a different crate, `semio-framework-ui-runtime`) and nothing in `semio-framework-ui-scene` |
| `bun ./📜️script.ts test long --run '🗣️Interpreter' '📥️intake' '🔌️PluginRuntime' '🔬️engine-contract'` | **4 files, 675 passed / 0 failed** |
| `bun ./📜️script.ts test exhaustive --run UiDocumentStore --testNamePattern='bounded cost per carried byte'` | **1 passed / 216 skipped** — `[DEBUG] packed world lane set: 26 nodes, 56794 carried bytes, 105416 intake steps (1.86/byte), one-lane republish 18565` |
| — the same law over an UNPACKED lane set | **1 failed**: `expected 626490 to be less than or equal to 170382` (and `expected 135 to be less than or equal to 32`) |
| `bun ./📜️script.ts typecheck` | **826 `error TS`**, **zero** in `🧪️typedwire` or any other file this wave touched (the one `UiDocumentStore/🟦️.tsx` hit is the pre-existing bun-only `import.meta.dir`, W-P §5.6; the peer baseline was 824 at W-P5) |

## 7 Not verified / open

- **The served wasm MUST be rebuilt.** Unlike W-P4, W-R and W-P5, this wave's fix is guest Rust
  (`⚛️reactor/🔄️turn`, `⚛️reactor/🩹️patches`, `⚛️reactor/📨️pending`, `🎠️kernel`, `♻️reconcile.rs`) plus
  the puzzle 3d window's new `fit` lane. Nothing improves in the browser until
  `component-release` is rebuilt AND materialised into the served module dir (`dev` script's
  `plugin <variant>` command — a bare nx `component-release` only runs cargo and leaves a stale wasm
  being served, which reports `Activated … (unchanged)`).
- **The after-measurement in the browser was not taken.** The dev target at `:6013` was down for most
  of this wave and boots a stale wasm anyway (see above), so the 24.3 s figure has a before but no
  after. `🔍️ws2-scene-latency-probe.ts` is the instrument: re-run it after the rebuild and compare
  `gap` and `workerPosts`. Predicted from §2.5: the ~1 091 retirement turns per publication collapse
  to ≤ 4, so the round-trip half of the gap (≈ 78 % of 24.3 s) should collapse with it; the
  main-thread half (5.4 s of intake, projection and React commit) is untouched by this wave.
- **The other per-turn `…_one()` ladders were not measured**, only re-paced:
  `close_ui_turn_patch_transport_one`, `close_table_rows_view_one`, `COLD_PAIR_INGRESS.advance_close_one`
  and `step_reactor_close` still run one unit per turn. None of them holds a document-scaled owner as
  far as §2.4's reading goes, but none of them was measured either.
- **`SurfaceReconcileTerminal` / `MountedTreeTerminal` retirement is still one owner per unit** (there
  is no grant variant); they now simply get 64 units per turn instead of 8.
- **The round-trip cost itself was not attacked.** `UI_TURN_PATCHES_MAXIMUM` is 1: the contract
  carries exactly one UI patch per turn, so K surfaces still cost ≥ K turns plus K lifecycle ACK
  round trips (`acceptUiPatches` submits one per patch). That is a wire-contract change and a
  separate wave; it is worth doing only if the after-measurement shows the residual is still
  round-trip bound.
- **The switch BACK (Nakagin → Concrete Forest) is still not re-driven** — W-P5's open item stands.

## 8 Files

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/♻️reconcile.rs` — `close_surface_patch_owner`
  honours the caller's `items` grant (1 line).
- `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — `close_ui_turn_patch_owner_with_grant`,
  `UiTurnPatchRetireArena::close_one(items, bytes)`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs` — the retirement pacing
  (constants, `retire_until_complete`, `retire_while_progress`, the re-paced close ladder).
- `…/⚛️reactor/🩹️patches/🦀️.rs` — `PatchTracker::close_step(items, bytes)`, `ReadySlot::close_step`.
- `…/⚛️reactor/📨️pending/🦀️.rs` — `PendingPatchAuthority::close_step(items, bytes)`.
- `…/⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs` — §5.1 law + its Nakagin scene builder.
- `…/⚛️reactor/📨️pending/🧪️tests/{🩹️receipt,🧪️authority,🔬️instance-lifetime-patch-close}/🦀️.rs`,
  `…/⚛️reactor/🩹️patches/🧪️tests/🔬️unit/🦀️.rs`, `🎠️kernel/🧪️tests/🔬️ui-turn-patch/🦀️.rs` — call sites
  updated for the new grant parameters.
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🧩️component.rs` — `world3d_fit_json`.
- `✏️s/…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️main/🦀️.rs` — `world_fit_revision`,
  `PUZZLE3D_FIT_PADDING`, the published `fit` lane.
- `✏️s/…/✏️editor/🧪️tests/🔬️example-switch/🦀️.rs` — §5.2 law.
- `🧰️framework/…/📃️UiDocumentStore/🧪️tests/🧪️typedwire/🟦️.tsx` — §5.3 law.
- `.🧬semio/…/PUZZLE-3D-END-TO-END/🔍️ws2-scene-latency-probe.ts` — the browser instrument, kept.
- `.🧬semio/…/PUZZLE-3D-END-TO-END/📋️master-plan-2026-09-08.md` — W-S2 section appended.
- this report.

No temporary `[DEBUG]` logging was left in production code; the three temporary measurements this wave
ran (in `🔬️example-switch`, `🧪️typedwire` and `🔬️reconcile-budget`) were each replaced by the law that
pins what they measured. `🗑️generated` was not touched.

