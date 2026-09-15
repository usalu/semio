# 🫀️ A runtime-owned settle pump for the wgpu host — lane `wgpu-host-settle-pump` (2026-09-14)

Three lanes handed this one the same blocking dependency in three different words:

* `📓️wgpu-progress-visibility-2026-09-14.md` §8.1 — *the boot example converges before anything paints*,
  owning layer named (`settle_boot` → `flush_deferred_actions` to a fixed point), **not fixed**;
* the same report §8.2 — *the 6118 preview wedges mid-tessellation and then goes deaf to
  `setActiveExample`*, **not fixed**;
* `📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4 — `UiDirtyScope` narrowing is *implemented, measured,
  and switched off*, because a window body's render is also the guest crossing that funds the guest's
  own background solve. Its §4.4 names the fix in one sentence: **"the guest's evaluation needs a pump
  of its own — the frame loop's, not the refresh's."**

This lane builds that pump, switches the narrowing on, and proves both on 6118.

Evidence: `🗑️generated/wgpu-settle/`, `🗑️generated/wgpu-verify/scoreboard.json`.
Probe (new): `🐍️wgpu-settle-pump-probe.mjs`.

## 0. TL;DR

| # | the ask | before | after |
|---|---|---|---|
| a | the boot example must NOT converge inside `boot_shell` | `boot_shell leave` **7 226 ms**, chrome at 9 212 ms (§8.1) | **1 776 / 1 846 / 1 775 ms** across three examples — the same cost as a boot with NO example (1 880 ms), i.e. the convergence is entirely out of boot. Chrome lands at 3 060 ms and the **status pill is live at 5.1–5.5 s** carrying `phase=computing label="Computing · 0/1 (0%)" ratio=Some(0.0)` |
| b | the preview must never wedge mid-tessellation | `meshingFaces 36/56` frozen 200 s, deaf to `setActiveExample`; battery `status:settled` **red** | battery `status:settled` **green**; `settle pump wedge` = **0** in every captured run — nothing wedged, so the watchdog never had to fire |
| c | switch `UiDirtyScope` narrowing ON with no example freezing | narrowing froze **14 of 16** examples mid-solve (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4.2) | **16 / 16 pass, both lanes** (`examples` 32/32). The seven `flowEvalTick` hops of a converging edit answer `refresh scope=none rendered=0`, and the pump's own `refresh scope=partial windows=[procedural.play.preview] … rendered=1` is the only crossing they cost |

This lane's own full `bun 🐍️wgpu-battery.mjs` (07:21→08:42, **0 page errors in all 16 rows**):
**examples 32/32 ✓, status-a11y-i18n 9/9 ✓** (both `status:settled` and `accessibility:live`, the
latter never green before), **frame-loop 4/4 ✓**, and — after §5.7's fix for the one regression this
lane caused — **io 6/6 ✓** and **deferred-commit 3/3 ✓**. §6 names every remaining red and whose it is.

Also in this report: §7 answers the frame-gate wedge handed over by `wgpu-wheel-zoom-a11y-live` —
**not the pump's**, with the dating that proves it, plus the two `[DEBUG]` names that lane asked for.

---

## 1. Root cause — one authority, three symptoms

The wgpu shell had **no settle lane at all.** Every guest chain was converged inside the call that
started it, and the three callers were the boot, the input path, and nothing else:

| caller | what it did | symptom |
|---|---|---|
| `settle_boot` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) | `refresh_ui(Full)` then `flush_deferred_actions()` — `settle_ui_chain` to a fixed point, up to `SHELL_SETTLE_ROUNDS` whole-shell refreshes | an `?example=` boot ran its whole evaluation inside `boot_shell`: `boot_shell leave 7 226 ms`, chrome at 9 212 ms, against 1 880 ms / 4 101 ms for the same build with no example (§8.1) |
| `handle_pointer_button` / `handle_pointer_move` / the retained press | the same synchronous `flush_deferred_actions` | post-boot, a chain converged **only while the user moved the mouse**; a booted example never finished for a user who did not |
| — | nothing drove a chain the guest re-armed on its own | a run left non-terminal blocked every restart, so the preview froze at `meshingFaces 36/56 ratio=0.64912283` and ignored `setActiveExample` for 200 s (§8.2) |

And because the ONLY thing that crossed into the guest was `refresh_ui`, the refresh could not narrow:
withdrawing a render withdrew compute from a brep solve that had nothing to do with the UI — 14 of 16
examples frozen mid-solve, `box-fillet-preview` never converging in 180 s at 36 renders against
21.13 s at 96 renders with the scope forced to `Full` (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4.3).

**One authority was missing: a lane that a producer DECLARES into and the frame loop DRIVES.** React
has had exactly that since `createUiRefreshCoalescerV1` (`🛠️ShellHelpers/🟦️.tsx`): at most one pass in
flight, at most one owed follow-up carrying the union, and — property 3 — *a pass may ASK for another
pass; it may never WAIT for one*. Its drain loop runs on the browser's own event loop, so React never
converges a chain inside the call that started it and the page paints between passes. The wgpu shell
had the DECLARING half of that (this ticket's `owed_refresh_scope`, added by the dirty-scope lane) and
none of the DRIVING half.

---

## 2. The design

### 2.1 The two halves, and who owns each

```
producer  ──owe_settle()──▶  ShellSettlePump (declared)
                                   │
frame-finish boundary ──settle_pump_pending()──▶ FrameFinishCursor.settle
                                   │
browser tick ──has_pending_settle()──▶ request_frame          ← keeps an event-driven shell ticking
                                   │
FrameDeferredCursor ──FrameDeferredWork::Settle──▶ settle_pump_step()   ← exactly ONE step per frame
```

* **`ShellSettlePump`** (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`) — the shell's half. `owe_settle` is the only
  way to arm it, and it is the settle twin of `owe_refresh`: producers DECLARE, they never converge.
* **`FrameDeferredWork::Settle`** (`🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`) — the runtime's half. The
  frame-finish boundary re-reads `settle_pump_pending()` every frame (never a latch carried across
  one) and the frame's deferred owner hands the shell exactly one step, **after** that frame's own
  input actions, its chrome maintenance, its sync pump and its tutorial flush.
* **`RuntimeMailbox::has_pending_settle`** + the browser tick's `request_frame` — the term that makes
  the lane real on an event-driven shell. Without it the shell settles the instant the last apply
  drains and the pump would run only while something else happened to ask for a frame.

### 2.2 What one step does

`settle_pump_step` is one bounded step and nothing more:

1. **drain** one round of armed work (deferred actions, parked extension answers, parked file opens,
   an owed shell URI);
2. if anything ran, or a refresh scope is owed, **take that one refresh pass** and stop. The next
   frame takes the next step — which is what lets the chrome, the GPU present and the status pill stay
   live through a convergence that used to run to a fixed point inside `boot_shell`;
3. if nothing was armed, **cross**: for every live World3d surface whose own published status says
   `computing`, ask `refresh_ui` for exactly that producer's window body. That is the crossing the
   guest's background solve is funded by, made once per FRAME rather than once per settle round.

### 2.3 The wedge watchdog

Per producer the pump keeps a **witness** — `phase|unitsDone|unitsTotal|facesDone|facesTotal|inFlight`,
read through the same `world3d_compute_status` parser the status pill reads — and one verdict per step:

| the producer's witness | verdict |
|---|---|
| moved | `Fund` — spend this step's crossing on its window body, and reset the whole watch |
| frozen, fewer than `SHELL_SETTLE_STALL_STEPS` (240) steps | `Fund` |
| frozen past the budget, `cancellable`, drives left | `Terminal` — dispatch the producer's OWN published `cancelAction` with its own `cancelArgs` |
| frozen past the budget, no `cancelAction` or `SHELL_SETTLE_TERMINAL_DRIVES` (3) drives spent | `Stand` — stop paying for crossings until the witness moves again |

Two deliberate refusals:

* **the shell learns no domain verb from code.** The terminal drive is whatever id the surface's own
  status contract published, exactly as the cancel control does. A producer that offers no way to end
  its own run is never driven.
* **`Stand` exists because a producer may lie.** The served generation3d guest publishes
  `computing: true` on **560 consecutive samples** of a window in which nothing evaluated at all
  (`📓️wgpu-progress-visibility-2026-09-14.md` §3.1). A pump that believed that flag would pin the host
  at one guest crossing per frame forever. A stood-down producer is not `computing` as far as
  `settle_pump_pending` is concerned, so the shell goes quiet.

### 2.4 `UiDirtyScope` narrowing, switched on

With the crossing owned by the pump, `refresh_ui` is free to render only what a settle dirtied. It now
gates on the kernel predicates the dirty-scope lane made shared — `wants_window_body`,
`wants_panel_body`, `wants_section(Measures)`, `wants_section(Engagements)` — whose TypeScript twins
React already runs against the same fixture. A surface's retirement stays INSIDE the per-surface loop,
so a skipped surface keeps the exact document it owns instead of being retired and never re-minted.

One widening had to land with it. `apply_ops_inner` already widened to `Full` on a `setPanel`
operation; it now widens on a **`setDocument`** too. The generation3d editor's own `setActiveExample`
is the measured case — it replaces the whole fixture through artifact mutations and declares
`UiDirtyScope::None` (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §3.4) — so a shell that honours the
scope without this would leave every window painting the previous document. A `flowEvalTick` emits no
`setDocument`, so the seven hops of a converging edit still narrow to nothing, which is the whole
116-of-137 `patched=0` saving the dirty-scope lane measured.

---

## 3. Files changed

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
  — `ShellSettleStep`, `ShellSettleVerdict`, `ShellSettleWatch`, `ShellSettlePump`,
  `settle_pump_owes`, `settle_watch_verdict`, `settle_progress_signature`,
  `ShellState::{owe_settle, settle_pump_pending, settle_pump_step, settle_pump_cross,
  live_compute_surfaces, window_body_key}`; `settle_boot` arms instead of converging; `refresh_ui`
  honours the scope.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`
  — `FrameDeferredWork::Settle`, the `settle` flag through `FrameFinishCursor` /
  `FrameDeferredCursor::{new, take_next, terminal_is_empty, close_step}`, and
  `RuntimeMailbox::has_pending_settle`.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs`
  — the browser tick's `request_frame` owes a frame while a settle is owed.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧾️frame-action-ledger/🦀️.rs`,
  `…/🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` — the new `FrameDeferredCursor::new` arity.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`
  — the new vitest suite.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs`
  — `EngineCanvasPresenter::note_realize_stall` and its two call sites (§7.2, diagnostics only).
* the shell's `ShellState::drain_gesture_bound_work` and its three gesture call sites (§5.7).
* the renderer's `AppPresentPhase::Engine` error arm — one `[DEBUG]` where `retained_fault` is stored
  (§7.2, diagnostics only).

Created:

* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫀️settle-pump/🔣️.json` — the
  language-agnostic oracle.
* `…/🧪️tests/🫀️settle-pump/🦀️.rs` and `…/🧪️tests/🫀️settle-pump/🟦️.ts` — the two twins.
* `🐍️wgpu-settle-pump-probe.mjs`, this report.

A peer lane (`wgpu-generate-add-port-fit`) extended the same oracle mid-session with its `gestureRows`
and landed the input half — `handle_pointer_button`, `route_retained_pointer_press`,
`dispatch_tree_selection` and the renderer's `handle_pointer_move` now `owe_settle()` and return
instead of calling `flush_deferred_actions`. Their rows are answered by the Rust twin's
`an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch`. Nothing of
theirs was reverted.

---

## 4. Laws

Shared, language-neutral oracle:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🫀️settle-pump/🔣️.json` — **7 `owesRows`,
5 `watchRows`, 4 `frameRows`, 4 `gestureRows`, 3 `gestureBoundRows`, 12 declared laws**. Two independent implementations
answer it.

| # | law | where | what it drives |
|---|---|---|---|
| 1 | `the_frame_owes_a_settle_step_exactly_while_a_chain_is_live` | `🧪️tests/🫀️settle-pump/🦀️.rs` | the shell's own `settle_pump_owes`, over all five terms + the two refusals |
| 2 | `a_producer_is_funded_while_it_advances_and_driven_to_a_terminal_state_when_its_witness_freezes` | same | the shell's own `settle_watch_verdict`, over statuses parsed by the shipped `world3d_compute_status` — the same parser the pill reads |
| 3 | `a_frame_drains_its_input_actions_before_it_takes_a_settle_step` | same | the REAL `FrameDeferredCursor::{new, take_next, terminal_is_empty}` |
| 4 | `the_boot_arms_the_settle_lane_instead_of_converging_and_the_frame_loop_drives_it` | same | `settle_boot`'s body, the frame-finish boundary and the deferred owner, read as source |
| 5 | `the_refresh_honours_the_dirty_scope_now_that_the_pump_funds_the_guest` | same | `refresh_ui`'s four narrowing gates |
| 6 | `an_input_gesture_declares_the_chain_and_never_converges_it_inside_its_own_dispatch` | same | the peer lane's `gestureRows` — four entry points that must not reach `flush_deferred_actions` |
| 7 | `a_gesture_runs_its_user_activation_bound_work_before_it_returns` | same | the three doors a file picker can be asked for through, and that the drain takes `pending_file_opens` and NOTHING else |
| 8 | `every_declared_law_is_answered_here` | same | the oracle declares ≥ 12 laws and one consumer per language |
| 9 | the TypeScript twin, independent re-derivation | `🧪️tests/🫀️settle-pump/🟦️.ts` | 7 tests; the witness is built through the SHIPPED `world3dComputeStatusV1` React itself reads |

Non-vacuity is pinned in both twins: the oracle carries a settled shell that must owe **nothing**
(`a-settled-shell-owes-no-settle-step-at-all`) and a frame that carries **no** settle work, so a pump
that always said yes — one guest crossing per frame forever — fails the suite.

Counts actually run, foreground:

```
cargo test -p semio-framework-os-renderer-wgpu --lib settle_pump     8 passed
bunx vitest … 🧪️tests/🫀️settle-pump/🟦️.ts                             7 passed
```

**The failing-first check was run, not assumed.** With the `computing` term removed from the
TypeScript `settlePumpOwes`, the suite fails exactly where it should —
`a-live-producer-owes-a-step-even-with-nothing-armed: expected false to be true` — and passes with it
restored.

---

## 5. Runtime

All readings are from the renderer wasm this lane built (`🗑️generated/wgpu-settle/wasm-build-2.txt`,
dist republished 2026-09-15 00:08, 81 962 769 B — verified to carry the pump by
`grep -a "wgpu-shell settle pump"` on the served `_bg.wasm`). Screenshots are not evidence on 6118:
the canvas is an `OffscreenCanvas` owned by the frame Worker and captures blank. Every number below is
a `[DEBUG]` trace, a battery verdict or an introspection export.

The staged wgpu guest never moved: every file under `🧑‍💻dev/🔌️plugin-modules/🌀️procedural/` carries
mtime **2026-09-12 06:56**. The before and after scoreboards therefore differ in the renderer wasm and
nothing else.

Artefacts, all under `🗑️generated/wgpu-settle/`: `scoreboard-before.json` (the 15:12 run, before this
lane), `scoreboard-full-after.json` (**this lane's own full battery**, 2026-09-15 07:21→08:42),
`examples-after.json`, `boot-after/` (this lane's own probe), `io-chrome-rerun.txt` and
`regressed-rows-run.txt` (the re-runs of §5.6), `peer-console-2026-09-15/excerpts.txt`, and
`wasm-build-{1..4}.txt`.

### 5.1 The boot no longer converges — and shows progress instead

One `?example=` boot, three examples, one clock (`🗑️generated/wgpu-verify/examples/<id>/edit/console.txt`):

```
2186  wgpu-shell dispatch action=setActiveExample scope=none
2329  wgpu-shell refresh scope=none rendered=0
3060  wgpu-shell refresh scope=full rendered=7          ← the setDocument widening: the example paints
3061  wgpu-worker boot_shell leave 1776 ms              ← boot LEAVES here, nothing converged
5071  wgpu world3d status pill surface=procedural-preview phase=computing
      label="Computing · 0/1 (0%)" ratio=Some(0.0) computing=true rect=148x22+981,58
9213  wgpu-shell settle pump {"crossings":0,"step":"Drained","steps":1,"watching":0}
12552 wgpu world3d status pill surface=procedural-preview phase=samplingEdges ratio=None computing=true
12735 wgpu-shell refresh scope=partial windows=[procedural.play.preview] … rendered=1   ← the pump's crossing
15252 wgpu-shell settle pump {"crossings":1,"step":"Quiescent","steps":3,"watching":0}
```

| reading | before (`📓️wgpu-progress-visibility-2026-09-14.md` §8.1) | after |
|---|---|---|
| `boot_shell leave`, `?example=` boot | **7 226 ms** | **1 776 ms** (sphere-cut) / **1 846 ms** (box-fillet) / **1 775 ms** (hex) |
| `boot_shell leave`, no example, same build | 1 880 ms | — (the example boot now costs the same) |
| chrome live | 9 212 ms | 3 060 ms |
| first status pill | none at all | **5 071 / 5 534 / 5 275 ms**, non-idle, with a ratio |

The whole 5.3 s that used to run inside `boot_shell` is now outside it, in front of a live shell with a
live progress pill — which is what the goal's "progress and cancellation, visible to the user" asks
for.

### 5.2 The pump runs, and nothing wedges

`settle pump wedge` = **0** and `exceeds 64 pages` = **0** in every console this lane read. The
watchdog's terminal drive never had to fire, because no producer froze: the battery's own
`status:settled` step — **red since the wedge was first measured** — now reports
`producerStatus {"phase":"idle", …}` after the example switch and passes.

| battery row (shared scoreboard) | before, 15:12 | after, 22:42, this build |
|---|---|---|
| `boot` | 2/3 ✗ | **3/3 ✓** |
| `no-example` | 0/2 ✗ | **2/2 ✓** |
| `chrome` | 5/5 ✓ | 5/5 ✓ |
| `frame-loop` | 4/4 ✓ — 7 898 batches / 7 898 frames, **0 quarantines, 0 faults** | 4/4 ✓ |
| `examples` | 16/16 ✓ | **32/32 ✓** (16 rows × both lanes) |
| `status-a11y-i18n` | 7/9 ✗ | **8/9** — `status:settled` and `status:pill-while-computing` green, `accessibility:live` red |
| `node-gestures` | 6/8 ✗ | 8/8 ✓ |

### 5.3 What the narrowing costs, and what it saves

Per whole example run (boot + convergence), from the same consoles:

| example (edit) | `render begin` | `refresh scope=` passes | of those `rendered=0` | settle-pump steps | wedges |
|---|---|---|---|---|---|
| sphere-cut-with-torus | 50 | 12 | 4 | 3 | 0 |
| box-fillet-preview | 42 | 9 | 3 | 3 | 0 |
| hexagonal-mushroom-column | 56 | 11 | 3 | 3 | 0 |

Against the two lanes that measured this before: `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7
measured **137 `renderSurface` calls per converging edit, 116 of them `patched=0`**, and
`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §4.1 measured the scope-honoured build at **48 renders /
8 refresh passes — and 14 of 16 examples frozen**. This build lands in the same band as §4.1's table
(42–56 renders, 9–12 passes) **without the freeze**, which is exactly what §4.1 promised and §4.2 said
could not be had.

### 5.4 Convergence seconds, before and after

| example | lane | before s | after s | | example | lane | before s | after s |
|---|---|---|---|---|---|---|---|---|
| hexagonal-mushroom-column | edit | 11.63 | **9.38** | | hexagonal-mushroom-column | viewer | 8.15 | **6.89** |
| rectangle-extrude-volume | edit | 16.66 | **10.78** | | rectangle-extrude-volume | viewer | 9.55 | 9.91 |
| rectangle-wire-preview | edit | 9.07 | 12.12 | | rectangle-wire-preview | viewer | 5.64 | 6.11 |
| box-shell-preview | edit | 12.35 | 13.69 | | box-shell-preview | viewer | 6.74 | **6.18** |
| box-fillet-preview | edit | 11.89 | **9.66** | | box-fillet-preview | viewer | 6.66 | 6.88 |
| sphere-cut-with-torus | edit | 13.12 | 13.89 | | sphere-cut-with-torus | viewer | 6.59 | 9.25 |
| sphere-box-fuse | edit | 13.97 | **9.65** | | sphere-box-fuse | viewer | 6.84 | 9.01 |
| face-sweep-extrude | edit | 12.76 | **11.36** | | face-sweep-extrude | viewer | 5.34 | 5.59 |

Mean edit lane **12.68 s → 11.32 s**; mean viewer lane 6.94 s → 7.48 s.

These two runs ARE single-variable: every file the serve stages for the guest
(`🧑‍💻dev/🔌️plugin-modules/🌀️procedural/`) carries mtime **2026-09-12 06:56** — the wgpu guest was never
restaged, by this lane or any other, so the only thing that changed between the two scoreboards is the
renderer wasm. What the spread does not support is a SPEED-UP claim: five of eight edit rows got
faster and three slower, three of eight viewer rows faster and five slower, and the per-example
differences are larger than either mean. The honest claim is **no regression and no freeze** — which,
against a narrowing that froze 14 of 16 examples the last time it was switched on, is the whole
point.

### 5.5 This lane's own capture

`SEMIO_PROBE_EXAMPLE=sphere-cut-with-torus bun 🐍️wgpu-settle-pump-probe.mjs`, 100 s,
`🗑️generated/wgpu-settle/boot-after/` (2026-09-15 07:19, 0 page errors, 0 `exceeds 64 pages`,
0 `setContributions command failed`):

```json
"boot":  { "firstChromeMs": 3152, "firstPreviewRenderMs": 3156, "bootShellLeaveMs": 4139,
           "firstStatusPillMs": 7612, "firstToolRunStartMs": 7800, "firstTessellateMs": 12364 }
"pump":  { "steps": 3, "crossings": 1, "wedges": 0 }
"cost":  { "renderBegin": 50, "renderBeginMain": 7, "renderBeginPreview": 8,
           "refreshPasses": 11, "frameBuildSuperseded": 0, "revisionStale": 0 }
```

The ordering is the claim: **`toolRunStart` at 7 800 ms and the first `tessellate` at 12 364 ms are
both AFTER `boot_shell leave` at 4 139 ms**, with the chrome painted at 3 152 ms and a live
`phase=computing` pill at 7 612 ms. The before this replaces (`📓️wgpu-progress-visibility-2026-09-14.md`
§8.1) is the exact opposite ordering: the first hop at 4 688 ms and the `tessellate` answered at
8 584 ms, both INSIDE a `boot_shell` that left at t=8 856 having taken 7 226 ms, with chrome at
9 212 ms.

The eleven refresh passes of that run read: `none rendered=0` ×3, `full rendered=7` ×7, and one
`partial windows=[procedural.play.preview] panels=[] utilities=false tools=false engagements=false
measures=false labels=false rendered=1` — the pump's own crossing, naming exactly the producer that
was still computing.

### 5.6 The full battery, and the rows this lane moved

`bun 🐍️wgpu-battery.mjs` (no `--only`), 2026-09-15 07:21 → 08:42, **0 page errors in every one of the
16 rows**. `🗑️generated/wgpu-settle/scoreboard-full-after.json`, total **224/328 steps, 7/16 rows
green**.

| row | before, 15:12 | this lane's full battery | after §5.7's two fixes |
|---|---|---|---|
| `examples` | 16/16 ✓ | **32/32 ✓** | — |
| `status-a11y-i18n` | 7/9 ✗ | **9/9 ✓** — `status:settled` AND `accessibility:live` both green | — |
| `frame-loop` | 4/4 ✓ | 4/4 ✓ | — |
| `generate-add` | 2/2 ✓ | 2/2 ✓ | — |
| `generation-roster` | — | 5/5 ✓ | — |
| `catalogue` | 3/3 ✓ | 3/3 ✓ | — |
| `port-fit` | 8/8 ✓ | 8/8 ✓ | — |
| `deferred-commit` | 3/3 ✓ | 2/3 ✗ | **3/3 ✓** |
| `io` | 6/6 ✓ | 4/6 ✗ | **6/6 ✓** |
| `chrome` | 5/5 ✓ | 4/5 ✗ | 4/5 ✗ (§6.5) |
| `boot` | 2/3 ✗ | 2/3 ✗ — the identical `framework.panel.toolRun … retained document ingress reached its terminal fault` string as the before | — |
| `no-example` | 0/2 ✗ | 0/2 ✗ | — |
| `spawn-job` | 8/8 ✓ | 7/8 ✗ (§6.6) | 7/8 ✗ |
| `node-gestures` | 6/8 ✗ | 5/8 ✗ (§6.6) | 5/8 ✗ |
| `world3d-editor` / `world3d-viewer` | 9/10, 10/10 over a 10-step probe | 70/115, 67/115 over a probe another lane grew to 115 steps | — |

**`status-a11y-i18n` 9/9 is the row that closes §8.2 from the other side**: `status:settled` reports
the producer reaching `phase: idle`, and `accessibility:live` — red in every previous run of that row,
including the three the `wgpu-wheel-zoom-a11y-live` lane measured — passed here, which means the host
did not stop admitting frames in this run.

### 5.7 One regression this lane caused, found by the full battery and fixed

Moving the input path off `flush_deferred_actions` (this lane's `gestureRows`) also moved the FILE
PICKER off the gesture, and a browser grants `input.click()` its user activation to the task that
handled the click or the key and to no task after it. Measured:

```
io  "the import opens a real file picker"  {"inputs": [], "choosers": 0}   ← no element ever created
io  "the picked file REPLACES the graph"   {"importedNodes": [], "graphChanged": false}
```

`ShellState::drain_gesture_bound_work` is the fix, and it is deliberately the narrowest possible: it
takes `pending_file_opens` and nothing else, and it is called from the three doors a picker request can
arrive through — `handle_pointer_button`, `route_retained_pointer_press`, and `handle_keyboard_async`.
The third is the one that mattered: the `io` probe reaches `importDocumentRequest` through **`mod+o`**,
so a fix covering only the pointer paths left the row red (measured: still 4/6), and adding the chord
took it to **6/6**, the picker carrying its real `accept=".stl,.obj,.ply,.gltf,.dwg,.json"` and the
import replacing the graph with `imported-source` / `imported-geometry` / `imported-preview`.
`deferred-commit` recovered to 3/3 in the same re-run. Law: `gestureBoundRows` in the oracle, answered
by `a_gesture_runs_its_user_activation_bound_work_before_it_returns`.

---

## 6. What is NOT claimed

1. **§5.4 is not a speed-up claim.** The two runs are single-variable (the staged wgpu guest is
   unchanged since 2026-09-12 06:56), but five of eight edit rows got faster and three slower, with a
   per-example spread wider than either mean. The claim is *no regression and no freeze*, not that the
   pump made the guest faster. What IS unambiguous are the STRUCTURAL readings — `boot_shell leave`,
   `refresh scope=… rendered=`, the `settle pump` traces and the wedge count — because none of them
   exists in a build without this lane's code.
2. **The full battery was run once, not repeated.** `bun 🐍️wgpu-battery.mjs` end to end
   (07:21→08:42, §5.6) plus targeted re-runs of the regressed rows. Row-level flakiness is real on this
   port — `no-example` read 0/2, then 2/2 in a peer run on the same renderer and guest, then 0/2 again
   — so a single green row is evidence, not proof, and the rows this lane claims (`examples`,
   `status-a11y-i18n`, `frame-loop`, `io`) are the ones it also has console-level mechanism for.
3. **`accessibility:live` passed once here and had never passed before.** That row depends on the host
   not wedging (§7), and this lane did not fix the wedge — it did not occur in this run. No claim is
   made that it is closed.
4. **The wedge watchdog is proven by law, not at runtime.** Nothing wedged on this build, so
   `ShellSettleVerdict::Terminal` and `ShellSettleVerdict::Stand` never fired in a browser. Their
   arithmetic — including the exact step the abort lands on and the three-drive ceiling — is pinned by
   the oracle and both twins, and the gesture it dispatches is the one `📓️wgpu-progress-visibility-2026-09-14.md`
   §7.1 measured to unblock a wedged run.
5. **`chrome` 4/5 is NOT fixed and may be this lane's.** The red step greps the console for
   `shell.world3d.cancel`, an id the shell prints only when a pointer HITS the control. The control is
   declared exactly while the producer publishes `cancellable`, and that window is now SHORT because
   the chain converges instead of hanging — the probe's own console shows the preview reaching
   `phase=computing` twice and settling. Whether the probe simply sweeps after the window closed or the
   control is genuinely absent is not determined here. What this lane refused to do is make the shell
   declare a cancel affordance for work that is not running, to satisfy a probe written against the
   previous permanently-`cancellable` status.
6. **`spawn-job` 7/8 (`s1_hover` — `pumps 0, actions 0, hover null`) and `node-gestures` 5/8 (wire
   connect/cut = 0) regressed against the before and are NOT diagnosed.** Both are gesture-driven and
   both plausibly share §5.7's cause class — work that used to converge inside the gesture now lands a
   frame later, so a probe that samples immediately after the gesture sees nothing. The difference from
   `io` is that no mechanism was measured here, so no fix was attempted and none is claimed.
7. **`boot` 2/3 and `no-example` 0/2 are red in the BEFORE scoreboard too**, the former with the
   byte-identical `framework.panel.toolRun … retained document ingress reached its terminal fault`
   string. Not this lane's, not diagnosed.
8. **`world3d-editor` / `world3d-viewer` are not comparable across the two scoreboards** — another lane
   grew that probe from 10 steps to 115 between them.
9. **`flush_deferred_actions` now has zero production callers.** The pump replaced the boot's call and
   the peer lane replaced the input path's. It is left standing because that peer's own `gestureRows`
   premise names it as the synchronous door an embedding host may take, and rewriting a law a peer
   wrote in the same hour is worse than the dead code. Removing it — with `settle_ui_chain`, `settling`
   and `SHELL_SETTLE_ROUNDS` — is a clean follow-up.
10. **No native / winit proof.** `RuntimeMailbox::has_pending_settle` is `cfg(target_arch = "wasm32")`,
   because only the browser tick turns a pending predicate into `request_frame`. The native frame loop
   has no equivalent term yet, so on that target the pump runs only while frames are already coming.
   The Rust laws do run natively.
11. **No screenshot is offered as evidence** — the OffscreenCanvas captures blank headless.
12. **Two wgpu engine vitest suites fail at HEAD and were not fixed**: `🖌️wgpu-document-owner-move`
   (`terminal_is_fault()` occurs 3× in the shell where its fixture expects 2) and
   `⏱️wgpu-worker-step-budget`. `git diff HEAD` shows neither symbol in this lane's diff. The five
   `🧩️package-integration` failures are `ReferenceError: Bun is not defined` under `bunx vitest`, not a
   defect.

---

## 7. The frame-gate wedge handed over by `wgpu-wheel-zoom-a11y-live` — whose it is

`📓️wgpu-wheel-zoom-a11y-live-2026-09-14.md` §3.4/§6.2 hands this lane a second wedge: between t≈85 s
and t≈200 s the host stops admitting frames — `os_host frame gate blocked=true pending=true
phase=Some(Engine)` or `phase=Some(Aborted) retained-fault=true` — and everything downstream of a
paint (the retained-document ingress, the accessibility projection, the mirror) freezes with it.

### 7.1 It is not the settle pump's, and the evidence is dated

| the pump | the wedge |
|---|---|
| declares work, takes ONE bounded step per frame, spends at most one guest crossing | lives in `AppPresentCursor` — `AppPresentPhase::Engine` / `Aborted` — which is the PRESENTATION ladder |
| its only frame-loop contract is `has_pending_settle()` → `request_frame`: it ASKS for frames | the gate refuses to ADMIT a build while a presentation is pending; asking harder changes nothing |
| first existed in a served build at 2026-09-15 00:08 | measured on the **15:00 renderer build** in a peer run at 15:30, i.e. hours before the pump existed, and identically at 19:58 and 01:01 |

So: measured before the pump, in a layer the pump never touches, through a gate the pump cannot open.
**Not fixed here, and not claimed as this lane's.**

### 7.2 What this lane did land for it — the two missing names

The handover's own request was one `[DEBUG]` line in each of two places, because the wedge reports a
PHASE and never a REASON. Both are now in the tree; neither changes any behaviour.

* `🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs` — `EngineCanvasPresenter::note_realize_stall`, called
  from both `Ok(false)` arms of `realize_step` and reset the moment either arm is passed. On a
  power-of-two cadence from the 64th consecutive answer it prints
  `[DEBUG] engine realize stalled arm=metrics-invalidation-scan|slot-retirement steps=N scan=…`,
  which is exactly the choice §3.4 could not make by inspection.
* `🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — the `AppPresentPhase::Engine` error arm now prints
  `[DEBUG] os_host present aborted engine=N fault=…` where it stores `retained_fault`. That string was
  previously surfaced only by `present_step`'s `Err` return, which is unreachable while `pending` is
  still held by the `Aborted` phase the same line enters — so the reason an abort happened was never
  printed at all.

Read by inspection while adding them, and worth recording: `invalidate_primary_metrics_step` is
**bounded** — its index walks to `ENGINE_SURFACE_CAPACITY` and then clears — so the
`metrics-invalidation-scan` arm can only hold forever if `observe_primary_metrics_generation` re-arms
it faster than it drains, and its one caller is `AppSurfaceResizePhase::Apply`, i.e. a real surface
resize. On a headless probe with no resize after boot that arm should be unreachable, which makes
`slot-retirement` the likelier holder — but "likelier by inspection" is not a measurement, and the
trace above is there to settle it on the next build.

**Type-checked, not yet runtime-proven**:
`cargo check -p semio-framework-os-renderer-wgpu --lib --target wasm32-unknown-unknown` →
`Finished dev profile … 46 warnings`. The renderer wasm serving 6118 during this lane's verification
predates these two lines, so no capture in this report contains them.
