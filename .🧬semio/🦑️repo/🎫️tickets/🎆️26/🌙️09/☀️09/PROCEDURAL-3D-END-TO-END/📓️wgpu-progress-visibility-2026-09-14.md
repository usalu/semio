# Progress And Cancellation, Visible To The User — lane `wgpu-progress-visibility` (2026-09-14)

The goal requires progress and cancellation for every expensive operation, VISIBLE to the user.
`📓️wgpu-a11y-status-i18n-runtime-2026-09-14.md` §3 and `📓️wgpu-edit-convergence-perf-2026-09-14.md`
§8 both root-caused the wgpu silence to the PRODUCER and deliberately left it. This lane measured
both renderers with one probe, fixed the producer at the owning layer, and found and fixed a second,
independent defect: **the wgpu cancel control could never be pressed at all.**

- Probe (new): `🐍️progress-visibility-probe.mjs` — one script, both renderers, same gesture
- Evidence: `🗑️generated/wgpu-progress/{wgpu-base,wgpu-boot-axis,wgpu-cancel-base,wgpu-cancel-after,react-base,react-smoke,react-state-1449}/`,
  `🗑️generated/wgpu-verify/{status-a11y-i18n,chrome}/`, `🗑️generated/react-verify/{cancel-preview,status-parity}/`

---

## 1. TL;DR

| # | item | measured before | root cause | after |
|---|---|---|---|---|
| 1 | ⛓️ the producer publishes `idle` for a whole evaluation | 6118, live example switch: **560/560 samples `phase:"idle" inFlight:0 ratio:1.0 facesTotal:0`** | the status projection reads only two ledgers, and **both are empty at every hop boundary** — which is where every status is built (§4.1) | ✅ fixed at the owning layer: a third, chain-level ledger (§5.1). Laws in Rust ×4, TS ×1, over a shared timeline fixture (§6). ❌ **not yet runtime-proven** — the guest restage is gated by another lane (§7.3) |
| 2 | 🛑️ the wgpu cancel control is unpressable | the control paints, hit-tests as `NavbarItem("shell.world3d.cancel::procedural-preview")`, and a click dispatched **nothing**, every time | `🧊️renderer/🦀️.rs` claimed the PRESS for the World3d surface on `bounds.contains` alone and returned before the shell saw it; the RELEASE path does reach the shell, but `handle_shell_hit` fires on the press (§4.2) | ✅ **fixed and runtime-proven on 6118**: `[DEBUG] shell world3d cancel {"action":"toolRunAbort"}` → `plugin_exchange actionId=toolRunAbort` (§7.1) |
| 3 | ⏳️ the pill never shows phase + ratio on wgpu | `status:pill-while-computing` **red** since 2026-09-13 | the pill was always correct; there was nothing non-idle to paint | ✅ **green**: `Meshing faces · 36/56 (65%)`, `ratio=Some(0.64912283)`, `195x22+981,58` (§7.1) |
| 4 | 📉 React's published ratio counts BACKWARDS | 6018: `Computing 7/7 (100%)` → `4/6 (67%)` → `3/6 (50%)` → `2/6 (33%)` in one evaluation | the budgeted-eval ledger aggregates its LIVE rows only, so a node finishing shrinks BOTH halves of the fraction (§4.3) | ✅ fixed in the same projection; the monotone chain census owns the fraction, the finest ledger still owns the phase name |
| 5 | 🎨️ the boot example converges before anything paints | `?example=…`: chrome live at **9 212 ms**, `boot_shell leave 7 226 ms` at t=8 856 — the whole chain runs 4.7 s→8.8 s INSIDE boot | `settle_boot` drives `flush_deferred_actions` to a fixed point before the first frame, and the post-boot settle pump is input-driven only (§8.1) | ❌ **measured, root-caused, NOT fixed** — owning layer named and handed on (§8.1) |
| 6 | 🧊 the 6118 preview wedges mid-tessellation | `meshingFaces 36/56 ratio=0.64912283 meshes=596` frozen for 200 s, then deaf to `setActiveExample` | the guest stops arming hops after its last `tessellate`; the non-terminal run then blocks every restart | ❌ **measured, NOT fixed, NOT mine** — present on the served guest BEFORE this lane's renderer build (§8.2) |

**Gates.** `bun 🐍️wgpu-battery.mjs --only=status-a11y-i18n,chrome` → **chrome 5/5**, **status-a11y-i18n 7/9**
(pill-while-computing turned green; `status:settled` and `accessibility:live` are red from item 6, §7.2).
`bun 🐍️react-battery.mjs --only=cancel-preview,status-parity` → **16/16 green, 0 reds** (§7.3).

⚠️ **Screenshots are not evidence on 6118.** The canvas is an `OffscreenCanvas` owned by the frame
Worker and headless Chromium captures it blank. Every wgpu claim below rests on the DOM, on the
renderer's introspection exports, or on its own `[DEBUG]` traces.

---

## 2. The probe

`🐍️progress-visibility-probe.mjs` boots, waits until the preview surface is actually PAINTING (not
merely mounted), drives the shell's own example picker to `Sphere Cut With Torus`, and samples the
preview status contract every 250 ms until it settles — recording
`phase / ratio / unitsDone / unitsTotal / facesDone / inFlight / computing / cancellable` plus the
pill and the cancel control's presence, on both renderers, from one code path:

* **react (6018)** — straight off the DOM contract: `[data-status-json]`,
  `[data-slot="world-compute-status"]`, `[data-slot="world-compute-cancel"]`.
* **wgpu (6118)** — off the shell's own `[DEBUG] world3d surface=… status=Some("…")` publication
  trace and `[DEBUG] wgpu world3d status pill …`; the cancel control is located by sweeping the
  surface's overlay row with the shell's `os_host pointer hit` trace, never a guessed pixel.

Axes: `SEMIO_PROBE_TARGET`, `SEMIO_PROBE_CANCEL=1` (click the cancel the first sample it is offered),
`SEMIO_PROBE_BOOT_EXAMPLE=1` (boot straight into the example, to measure paint-vs-convergence).

---

## 3. The truth on both renderers, before

### 3.1 wgpu — a live example switch publishes nothing to show

`🗑️generated/wgpu-progress/wgpu-base/` (boot with no example, wait until the surface paints at
7 299 ms, then pick `sphere-cut-with-torus` from the shell's own picker at 76 832 ms, watch 140 s):

```
watchedSamples 560   nonIdleSamples 0   distinctPhases ["idle"]
maxInFlight 0        maxFacesTotal 0    ratio 1.0 on every sample
cancellableSamples 546   pillSamples 15
```

Every one of the 567 `world3d surface=procedural-preview` publications carried, byte for byte:

```json
{"computing":true,"phase":"idle","phaseLabel":{"en":"Idle","de":"Bereit"},
 "progress":{"unitsDone":0,"unitsTotal":0,"facesDone":0,"facesTotal":0,"inFlight":0,
             "evalUnitsDone":0,"evalUnitsTotal":0,"ratio":1.0},
 "cancellable":true,"cancelAction":"toolRunAbort","cancelArgs":{"runId":"1","generation":0}}
```

The one pill the shell painted from it read **"Idle"** — with `computing=true` and `ratio=None`. A
status chip that says *Idle* while the app is busy is worse than no chip.

The chain underneath was real and healthy: `toolRunStart` at 77 004 ms, six `flowEvalTick` hops, four
`evaluate` round trips and one `tessellate` (38 749 bytes) answered between 77.3 s and 80.0 s. Six
hops of live kernel work, and not one of them was visible.

### 3.2 React — the same probe could not run: the page did not boot

`🗑️generated/wgpu-progress/{react-base,react-smoke}/` (11:20 and 11:47) — every load died on

```
pageerror SyntaxError: The requested module '/@fs/…/🌐️World3dHost/🟦️.tsx'
          does not provide an export named 'beginInteractivePluginAction'
```

The symbol exists nowhere in source (`grep` over the whole tree: only a prebuilt bundle under
`📤️distribution/`), so **6018 was serving a stale transform after a peer's export rename** — a wedged
vite module graph, not a source defect. Reported to the coordinator, who recycled the serve; only the
coordinator recycles it. This cost this lane its React baseline for the first half of the session.

### 3.3 React, once it was back — progress runs BACKWARDS

`🗑️generated/wgpu-progress/react-state-1449/` (14:49, healthy, 0 page errors). React converges in
7.5 s to 88 230 bytes of meshes, paints a pill on **33/33** samples and offers a Cancel on 19 — and
publishes this:

| t | phase | pill |
|---|---|---|
| 18.5 s | computing | `Computing 7/7 (100%)` |
| 19.7 s | computing | `Computing 4/6 (67%)` |
| 21.7 s | computing | `Computing 3/6 (50%)` |
| 22.8 s | computing | `Computing 2/6 (33%)` |
| 24.3 s | samplingEdges | `Sampling edges` (ratio 0) |
| 25.6 s | computing | `Computing 6/6 (100%)` |
| 26.0 s | idle | — |

`ratiosMonotone: false`. The brief's premise — "React carries phase/progress/cancellable, wgpu loses
it" — is half right: React's chrome is complete and its PHASE is live, but its **progress counts
down**, which is the same class of defect as publishing `idle`. `progress.nodesDone` is absent from
that capture, which is how I know the guest served at 14:49 does not yet contain this lane's fix.

---

## 4. Root causes, at file:line

### 4.1 Both ledgers are empty exactly where every status is built

`✏️s/…/🧵️preview-eval/🦀️.rs:835` `preview_progress_status_json_for` derived `phase`, `inFlight` and
`ratio` from exactly two sources:

* `FlowEvalSession::preview_tessellate_status` — reads `pending_tessellate_by_hash`
  (`🌊️flow/🖥️host/🦀️.rs:3340`), which is populated and drained INSIDE one hop;
* `FlowEvalSession::preview_eval_status` — reads `eval_progress_by_hash`
  (`🌊️flow/🖥️host/🦀️.rs:3471`), which only gains a row once an extension has already answered
  `done: false` (`resolve_preview_eval`, `:3445`).

An `evaluate` round trip **parked at the geometry extension** — where a boolean preview spends its
whole slow half — is recorded in NEITHER. And `preview_scene_status_json:807` took `computing` from
`FlowEvalSession::pending`, which is `tick_scheduled` alone (`🌊️flow/🖥️host/🦀️.rs:3099`) — false at
every hop boundary, because *a window waiting on an extension answer is marked `owed`, not `armed`*
(`arm_window_tick`, `:3116`).

What IS live throughout is the per-window tick latch — `armed`, `in_flight`, `owed`, `unfinished`
(`FlowEvalWindowTickLatch`, `🌊️flow/🖥️host/🦀️.rs:2716`). **Nothing read it.**

### 4.2 The wgpu cancel control could never be pressed

`🧊️renderer/🦀️.rs`, the pointer-button handler: on RELEASE the shell is called first (`:13840`) and
only then may a World3d surface claim the point; on PRESS the order is reversed — the world3d loop
runs first and `if world_consumed { return; }` fires on `bounds.contains(x, y)` alone. But
`handle_shell_hit` (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7494`), which owns the
`shell.world3d.cancel::…` arm (`:7555`), is deliberately driven from the PRESS.

The overlay row is painted INSIDE the surface rect (`surface_overlay_controls_for` anchors it at
`bounds.x + gap, bounds.y + gap`). So the only pointer phase that reached the shell over a World3d
surface was the one the shell ignores. Measured, `🗑️generated/wgpu-progress/wgpu-cancel-base/`:

```
126126 os_host pointer hit x=1214 y=68 hit=Some((NavbarItem, Some("shell.world3d.cancel::procedural-preview")))
126126 wgpu-shell pointer button x=1214 y=68 down=false … hit=Some((NavbarItem, …))
        ← no `down=true` line, and `grep -c "shell world3d cancel"` = 0
```

The same defect makes every future World3d overlay control decorative, not just this one.

### 4.3 The budgeted-eval ledger is not a progress ledger

`preview_eval_status` (`🌊️flow/🖥️host/🦀️.rs:3471`) aggregates its LIVE rows: a node that finishes is
REMOVED, so `units_done` and `units_total` both shrink and the published fraction wanders. That is
§3.3's countdown, exactly.

---

## 5. The fixes

### 5.1 A chain-level ledger, and a monotone denominator

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`

* new `PreviewChainStatus { nodes_done, nodes_total, in_flight, working }` with `units()` and
  `ratio()`;
* new `FlowEvalSession::preview_chain_status()` — `working` is `tick_scheduled` OR any latch
  `armed | owed | unfinished | in_flight > 0`; `in_flight` sums the parked extension requests; the
  denominator is the session's OWN published per-node census (`status_json`, rebuilt on every
  `sync`/`tick`), where a node leaves `queued`/`computing`/`stale` exactly once per chain and never
  returns — the one quantity that only ever grows inside one evaluation.
* `units()` raises the denominator to `done + 1` while the chain works, so a census that has seen
  every node settle still cannot publish `1.0` while an answer is outstanding. The outstanding round
  trips are deliberately NOT in the denominator: counting them made the fraction oscillate with every
  park and fold (`2/3` parked, `2/4` settled), i.e. the ratio went backwards — caught by the law
  before it ever shipped.

### 5.2 The projection reads it

`✏️s/…/🧵️preview-eval/🦀️.rs`

* `preview_scene_status_json` takes `computing` from the chain ledger, not from `tick_scheduled`;
* `preview_progress_status_json_for` gains a `show_chain_phase` branch (`computing` / `Berechnen`)
  and a `chain_progress` rule: **the PHASE is the finest name available; the PROGRESS is the only
  ledger that spans the whole evaluation.** The chain census owns `unitsDone`/`unitsTotal`/`ratio`
  whenever it exists; the finer ledgers still name the phase and still carry
  `evalUnitsDone`/`evalUnitsTotal`; `nodesDone`/`nodesTotal` are published beside them.
* `inFlight` is `max(chain, tessellate + eval)`, never the sum — the three ledgers overlap by
  construction.

No consumer changed: `World3dComputeStatusV1` and its Rust twin already declare
`phase`/`unitsDone`/`unitsTotal`/`inFlight`/`ratio`, and the wgpu pill already prints
`n/m (x%)` from them. Both renderers get this for free.

### 5.3 The press reaches the shell

* `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — new `ShellState::pointer_press_belongs_to_shell_chrome(hit)`,
  a pure predicate over hit KINDS (`NavbarItem | DropdownItem | ContextMenu | Select`), never a
  control-id list: the shell must not learn which overlay controls exist.
* `🧊️renderer/🦀️.rs` — the press path asks it BEFORE any surface may claim the point, which makes
  press and release symmetric.

### 5.4 One stale asset, fixed rather than worked around

`🧫️fixtures/⏱️evaluate-budget.json` had three rows declaring `phase: "idle", ratio: 1.0` while their
own `owesHop` was **true** — the defect written down as a law. They now declare the chain phase, with
a note recording that these rows drive a session with no published census (so the ratio they pin is
`0.0` while working, never `1.0`), and that the census-present case is pinned by the new laws.

---

## 6. Laws

Shared, language-neutral fixture: a new `progressTimeline` section in
`🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` — one long evaluation frame by frame
(`evaluation`, 6 frames, `minimumNonIdleFrames: 4`) plus a `cancelled` lane (4 frames). Three
independent implementations answer it.

| # | law | where | run |
|---|---|---|---|
| 1 | `the_chain_ledger_is_live_at_a_hop_boundary_and_its_census_only_grows` | `🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` | `cargo test -p semio-framework-os-flow --lib -- the_chain_ledger` → **1 passed**. Parked state measured as `{nodes_done: 2, nodes_total: 4, in_flight: 1, working: true}` while BOTH finer ledgers report `in_flight: 0` |
| 2 | `a_live_evaluation_publishes_non_idle_frames_with_a_monotone_ratio_and_an_offered_abort` | `🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` | 8 frames, **7 non-idle**, monotone, ≥ the shared fixture's minimum |
| 3 | `an_abort_while_the_chain_is_live_settles_on_cancelled` | same | `live="computing" cancelled="cancelled"` |
| 4 | `a_long_evaluation_publishes_a_monotone_run_of_non_idle_frames_and_then_settles` | `🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` | 4 non-idle frames, ratio monotone to 0.75, settled; cancelled lane settles on `cancelled` |
| 5 | `a_press_on_the_overlay_chrome_over_a_surface_belongs_to_the_shell` | same | the control is proven to be painted INSIDE the surface rect, the predicate answers, and the renderer source is asserted to consult it BEFORE the world3d claim |
| 6 | `publishes a monotone run of non-idle frames while a long evaluation is in flight, then settles` (TS twin) | `🧪️tests/🔬️engine-contract/🟦️.ts` | via the shipped `world3dComputeStatusV1` parser React itself reads |

Counts actually run (all foreground):

```
cargo test -p semio-framework-os-flow --lib -- the_chain_ledger                 1 passed
cargo test -p semio-framework-os-renderer-wgpu --lib -- shell_chrome_parity    26 passed
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib
        -- budget a_live_evaluation_publishes an_abort_while status            18 passed
SEMIO_TEST_LEVEL=long vitest … --testNamePattern="world3d compute status pane|world3d cancel contract"
                                                                               7 passed
```

---

## 7. Runtime

### 7.1 6118 — the cancel now lands, and the pill shows phase + ratio

Renderer wasm rebuilt (`Successfully ran target wasm`, dist republished 14:36,
`semio-framework-os-renderer-wgpu_bg.wasm` 81 677 351 B). Same probe, same click point, before and
after:

| | `🗑️generated/wgpu-progress/wgpu-cancel-base/` (before) | `🗑️generated/wgpu-progress/wgpu-cancel-after/` (after) |
|---|---|---|
| control located | `shell.world3d.cancel::procedural-preview` @ `[1214,68]` | identical |
| `wgpu-shell pointer button … down=true` | **absent** | `119326 … hit=Some((NavbarItem, Some("shell.world3d.cancel::procedural-preview"), None))` |
| `[DEBUG] shell world3d cancel` | **0 occurrences** | `119327 {"action":"toolRunAbort","surface":"procedural-preview"}` |
| guest saw it | — | `119327 plugin_exchange actionId=toolRunAbort branch=spawn-admit` |
| after the abort | nothing, forever | the wedged chain RESUMES: `flowEvalTick`, `evaluate req=7`, `req=8`, … |

The pill, from the battery's own artifact:

```
[DEBUG] wgpu world3d status pill surface=procedural-preview phase=meshingFaces
        label="Meshing faces · 36/56 (65%)" ratio=Some(0.64912283) computing=true rect=195x22+981,58
```

### 7.2 6118 battery

```
bun 🐍️wgpu-battery.mjs --only=status-a11y-i18n,chrome
  → chrome                5/5, 171 s, 0 page errors
      ✓ navbar control ids  ✓ example picker row  ✓ mode/role buttons
      ✓ a chord replans the dock   ✓ the world3d cancel control is declared
  → status-a11y-i18n      7/9, 196 s, 0 page errors
      ✓ boot  ✓ accessibility:mirror (66 nodes, 4 windows)  ✓ accessibility:per-window
      ✓ status:trigger    ✓ status:pill-while-computing   ← the red this lane was asked to close
      ✗ status:settled    ✗ accessibility:live            ← both from §8.2, not from this lane
      ✓ locale:palette-dispatch   ✓ locale:german-via-palette
```

`status:pill-while-computing` was red in `📓️wgpu-a11y-status-i18n-runtime-2026-09-14.md` (8/9) and is
now **pass**. The two reds are one cause: the served guest wedges at `meshingFaces 36/56` and never
settles, so the example switch never changes the document and the mirror's labels never change
(`before` and `after` are both 66 nodes / 47 labels, `arrived: []`, `left: []`).

### 7.3 6018 — React

```
bun 🐍️react-battery.mjs --only=cancel-preview,status-parity
  → cancel-preview  2/2   ✓ a cancellable frame is published {"frames":4}
                          ✓ the cancel affordance exists {"rows":4}
  → status-parity  10/10  boot, edit:sphere-cut, edit:no-example, edit:box-fillet, generate-mode,
                          generate-added, back-to-edit, viewer-role, view:sphere-box-fuse, view:no-example
  → BATTERY DONE green=16/16 red=[]
```

Green on the guest as currently served — i.e. **this lane introduced no React regression.** The
coordinator's journey gate (23/23, hex 3 meshes) is not reported here because **this lane performed
no guest restage**: the coordinator gated restaging on `📓️react-view-state-u64-carrier-2026-09-14.md`,
which did not exist at 15:05. The producer fix (§5.1, §5.2) is therefore proven by law but **not yet
proven at runtime on either renderer** — it needs one restage, and the journey gate must be run
immediately after it.

---

## 8. Measured, root-caused, NOT fixed

### 8.1 The boot example still converges before anything paints

`🗑️generated/wgpu-progress/wgpu-boot-axis/` (`?example=sphere-cut-with-torus`):

```
4 688 ms  the first flowEvalTick hop
8 584 ms  tessellate answered, 38 749 bytes
8 856 ms  wgpu-worker boot_shell leave 7 226 ms      ← the whole chain ran inside boot
9 212 ms  chrome live
```

Against the same build with no example: `boot_shell leave 1 880 ms`, chrome live at 4 101 ms. The
5.3 s delta is the convergence, and it is entirely pre-paint.

Owning layer: `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:3971` `settle_boot`, which calls
`apply_boot_example().await` and then drives `flush_deferred_actions` → `settle_ui_chain` to a fixed
point before the first frame. Deferring the example alone is NOT a fix: post-boot the settle chain is
driven only from input handlers (`🧊️renderer/🦀️.rs`, `handle_pointer_move` → `flush_deferred_actions`),
so a booted example would then never converge for a user who does not move the mouse. The machinery a
real fix needs already exists — `FrameDeferredWork` / `FrameDeferredCursor` (`🧊️renderer/🦀️.rs:9160`),
which already drains one deferred ACTION per frame — so the shape is a `SettleChain` work kind driven
per frame, with `settle_boot` arming it instead of converging. **Not attempted here**: it restructures
the frame/refresh authority that lane `wgpu-dirty-scope-refresh` owns and is editing concurrently, and
a half-landed version would regress boot for every app.

### 8.2 The 6118 preview wedges mid-tessellation, and then goes deaf

Not this lane's, and not caused by this lane's renderer build — the identical frozen numbers were
already measured on the PREVIOUS bytes (`wgpu-cancel-base`, before the 14:36 build):
`meshingFaces`, `ratio 0.64912283`, `unitsDone 36 / unitsTotal 56`, `meshes 596`, unchanged for 200 s.

In the battery capture the last `tessellate` answer is at t=11 s and there are only 7
`flowEvalTick settled` in the whole 196 s run; the pick at t≈161 s dispatches `setActiveExample` once
and arms **no** hop. The mechanism is visible in §7.1's after-column: the run left standing is
non-terminal, so `preview_eval_run_effects`
(`🧵️preview-eval/⏯️tool-run/🦀️.rs`) never starts a fresh one — and the abort this lane made reachable
is precisely what unblocks it.

### 8.3 An abort does not settle `cancelled` — it restarts

Measured in `wgpu-cancel-after`: `toolRunAbort` lands, the run is retired, and within 400 ms the
surface starts a NEW evaluation (`flowEvalTick`, `evaluate req=7/8`) instead of settling on
`phase: "cancelled"`. Cause: `preview_eval_run_effects`'s start arm
(`None | Some(Finalized | Aborted | Faulted) if owed || link.restart_owed`) — the window's unfinished
debt is still owed the instant the abort lands, so the very next poll restarts the work the user just
stopped. The session already has the latch a fix would use (`FlowEvalSession::preview_cancelled`,
`🌊️flow/🖥️host/🦀️.rs`), but wiring it into the restart policy risks the preview re-arm behaviour
`📓️preview-rearm-after-inspector-edit-2026-09-14.md` establishes, so it is named here rather than
guessed at late in the lane.

### 8.4 A peer break fixed forward: none needed

Two peer breaks blocked this lane and both resolved without my editing their code: the 6018 stale
transform (§3.2, coordinator recycled the serve) and a missing
`semio_framework::tool_run_panel_new_runs` export that failed one renderer wasm build at 14:26 — the
peer added the re-export at 14:25 mid-build and the retry succeeded.

`cargo test -p semio-framework-os-flow --lib` reports **37 failures** in `drawing`, `vcs`,
`wasm_session`, `extensions::wasm` and unrelated `host::tests` rows (e.g.
`derive_twice_same_node_is_same_handle`, `left: 1, right: 2`). `git diff HEAD` over that whole crate
is **two files, +80/-6** — this lane's chain ledger and its one new law — so those failures are not
this lane's. Flagged, not adopted.

---

## 9. Files

Changed:

* `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` — `PreviewChainStatus`, `preview_chain_status()`
* `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs` — law 1
* `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs` — the projection
* `✏️s/…/🧵️preview-eval/🧪️tests/🔬️unit/🦀️.rs` — laws 2, 3
* `✏️s/…/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/⏱️evaluate-budget.json` — three stale rows
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `pointer_press_belongs_to_shell_chrome`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — the press path asks first
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧫️fixtures/🛑️surface-controls/🔣️.json` — `progressTimeline`
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-chrome-parity/🦀️.rs` — laws 4, 5
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts` — law 6

Added: `🐍️progress-visibility-probe.mjs`, this report.

---

## 10. What is NOT claimed

1. **The producer fix is not runtime-proven.** It is proven by six laws across three implementations
   and compiles into both targets, but no guest restage was performed (§7.3), so no 6118 or 6018
   capture in this report shows `phase: "computing"` coming from the chain ledger, and none shows
   `progress.nodesDone`. The next step is one restage + the coordinator's 23/23 journey gate + a
   re-run of `🐍️progress-visibility-probe.mjs` on both ports.
2. **`status:settled` and `accessibility:live` are not claimed fixed** — §8.2, a guest-side wedge that
   predates this lane's build.
3. **Boot-before-convergence is not fixed** — §8.1, measured and handed on with the machinery named.
4. **Cancel does not yet settle `cancelled`** — §8.3. What IS proven is that the gesture reaches the
   run at all, which it could not before.
5. **No screenshot is offered as wgpu evidence** — the OffscreenCanvas captures blank headless.
6. **The 37 flow-crate test failures are not diagnosed**, only shown to be outside this lane's diff.
