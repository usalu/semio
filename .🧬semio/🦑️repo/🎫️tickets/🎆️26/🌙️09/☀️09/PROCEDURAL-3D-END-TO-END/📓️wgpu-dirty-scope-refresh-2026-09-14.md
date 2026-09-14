# wgpu — `UiDirtyScope` refresh and the shared revision authority

Lane `wgpu-dirty-scope-refresh`, 2026-09-14. Follows `📓️wgpu-edit-convergence-perf-2026-09-14.md` §6–§7,
which named the two remaining structural terms of the wgpu ⇄ React edit-mode gap and delivered neither.

---

## 1. TL;DR

**Item 2 landed and is proven. Item 1 is implemented, measured, and deliberately NOT switched on — it
stalls the product, and the reason is a real coupling, not a bug in the scoping.**

* **The frame-build / presentation half is fixed.** Input no longer renumbers the frame generation
  underneath a live frame build, and the presenter's freshness gate now reads the SAME revision
  authority the build was admitted under instead of re-reading a moving one. On 6118: **28 `frame
  build superseded` per converging edit → 0**, and **0 stale-revision quarantines**, where the
  previous lane's attempt at half of this produced `prepared render revision is stale: live=35,
  packet=27` → `worker-present-failed` (a dead surface) and was reverted. The battery's `frame-loop`
  row is **4/4 green: 20 gestures, 8 397 batches / 8 397 frames answered, 0 quarantines, 0 faults**.
* **The `UiDirtyScope` half is a stall on this renderer.** Honouring the scope works exactly as §7
  predicted — a converging edit drops from 137 renders to 48 and the hexagonal column from 13.49 s to
  8.46 s — and it **froze 14 of 16 battery examples mid-solve**. A window body's render is also the
  guest CROSSING that funds the guest's own background evaluation, so narrowing a settle withdraws
  compute from a brep solve that has nothing to do with the UI. Three narrowing rules were built and
  measured; all three stall. The behavioural skip is therefore **not landed**, the blocking dependency
  is named (§4.4), and the scope is threaded, traced and law-covered so the lane that gives the guest
  its own pump can switch it on in one line.
* **React is unaffected and was verified**: 23/23 journey steps on 6018 (hex 3 meshes, others 1), and
  the 31 React dirty-scope laws pass against the kernel predicates this lane made shared.
* One real bug fixed on the way: the native `ProgramBridge` threw away the `ui_scope` the wire frame
  has always carried (§3.3).

What is NOT claimed is §8. **"Every example within 2× of React" is not met and was not approached** —
the change that would have approached it is the one that cannot ship yet.

---

## 2. What was asked, and what each item turned into

| brief | outcome |
|---|---|
| (1) thread `UiDirtyScope` through the wgpu shell's `refresh_ui` exactly as React does | built, law-covered in Rust + TS over a shared oracle, **measured as a product regression, behavioural half reverted** (§4) |
| (2) one revision authority shared by the build and the presenter's freshness gate | **landed and proven** (§5) |
| law: a settle that dirties one window renders exactly that surface | the oracle and both twins exist and are green; the wgpu shell does not yet gate on them (§4.4, §6) |
| law: 100 input events during a build never supersede it and never quarantine the surface | **green, Rust + TS twin over a shared oracle, plus 6118** (§5, §6) |

---

## 3. Item 1 — the scope, end to end

### 3.1 The selection law is now one implementation per language, not one per renderer

React had the whole rule privately in `🛠️ShellHelpers/🟦️.tsx` (`uiRefreshWantsWindow`,
`uiRefreshWantsPanel`, `uiRefreshWantsFlag`, `uiRefreshWantsCatalogue`, `mergeUiDirtyScopeV1`); the
wgpu shell had none of it and a comment claiming it was "a concept the native shell does not have"
(`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4329`, now corrected).

The rule moved to the type that owns it, in both languages:

* `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — `UiDirtySection`, and on `UiDirtyScope`:
  `asks_for_nothing`, `wants_window_body`, `wants_panel_body`, `wants_section`, `wants_catalogue`,
  `merged_with` (first-seen body-key order, no duplicates).
* `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` — `uiDirtyScopeWantsWindowBody` and siblings,
  `uiDirtyScopeAsksForNothing`, `mergeUiDirtyScopes`.
* `🛠️ShellHelpers/🟦️.tsx` now DELEGATES to those four (`mergeUiDirtyScopeV1` is a one-line forward),
  so React and wgpu cannot drift into different ideas of what a settle dirtied.

### 3.2 The shell threads it

`ShellState` gained `owed_refresh_scope: UiDirtyScope` — the union of everything declared since the
last pass, this shell's half of React's `createUiRefreshCoalescerV1` owed slot. Producers only
DECLARE (`owe_refresh`); `refresh_ui` is the one place that takes it.

* `dispatch_action` / `dispatch_command` / the extension door compute
  `result.ui_scope.merged_with(host_effect_earned_scope(&effects))` — the Rust twin of React's
  `hostEffectRefreshScopeV1`: what the GUEST dirtied, unioned with what the HOST dirtied by applying
  that dispatch's own effects, never substituted.
* `apply_mutations` / `apply_ops_inner` carry it, widening to `Full` on a `setPanel` operation
  (`panel_json` feeds every section — React widens identically).
* `settle_ui_chain`'s per-round pass asks for `None`: every action it drained already declared what
  it dirtied.

### 3.3 A real bug found on the way — the native bridge threw the field away

`🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` decoded `AppFrame::Invocation` with `..` and then handed the
shell a hardcoded `UiDirtyScope::default()`. The wire frame has always carried `ui_scope`
(`📡️spr/🧵️channel/🦀️.rs:2201`), so the native shell could never have narrowed anything even in
principle. Now decoded, with `Full` still the answer for an absent or undecodable field. **Fixed and
kept.**

### 3.4 What the guest actually declares — measured, not assumed

The `[DEBUG] wgpu-shell dispatch …` trace this lane added, on a converging hexagonal-column edit:

| dispatch | count | declared scope |
|---|---|---|
| `flowEvalTick` (command) | 7 | `none` |
| `toolRunStart` (action) | 1 | `full` |
| `setActiveExample` (action) | 1 | `none` |

So §7's premise holds exactly: the settles that dominate a converging edit dirty **nothing**, and the
137 renders they paid for were 116 `patched=0` answers.

---

## 4. Why item 1 is not switched on — three rules, three stalls, one cause

### 4.1 Honouring the scope does what §7 promised

One renderer wasm build, scope honoured (`🗑️generated/wgpu-dirty-scope/scope-hex-1/`):

| reading | before (§5 of the previous report) | scope honoured |
|---|---|---|
| `render begin` per converging edit | 137 | **48** |
| refresh passes | 16 | **8** |
| hexagonal column, edit lane | 13.49 s | **8.46 s** |
| page errors | 0 | 0 |

### 4.2 And it froze 14 of 16 examples

The same build, through the battery's own `🐍️wgpu-example-matrix-probe.mjs`
(`🗑️generated/wgpu-verify/examples/`, run `🗑️generated/wgpu-dirty-scope/battery-after.txt`):

```
examples ok=false — 2 of 16 rows pass (hexagonal-mushroom-column, both lanes)
```

Every other row fails on ONE term of the probe's predicate — `geometry !== null`. The surfaces are
live and healthy: `scenePasses=1`, `meshSurfaces` published, `capacity=0`, `faults=0`, `status=null`,
`alert=null`, **0 page errors**. They are simply not finished:

| example (edit) | `facesDone` / `facesTotal` | `ratio` | previous lane's run |
|---|---|---|---|
| box-fillet-preview | 0 / 26 | 0.197 | `ratio` **1**, solid, 9.59 s |
| box-shell-preview | 0 / 12 | 0.400 | `ratio` **1**, solid, 11.96 s |
| sphere-box-fuse | 7 / 7 | 0.585 | `ratio` **1**, solid, 9.47 s |
| rectangle-extrude-volume | 6 / 6 | 0.800 | `ratio` **1**, solid, 15.47 s |
| hexagonal-mushroom-column | 6 / 8 | 0.649 | `ratio` **1**, solid, 13.49 s |

Hexagonal column passes only because its profile is a WIRE and publishes `edgePositions` long before
the solid lands — its solve is unfinished too.

### 4.3 The A/B that settles the cause

Same source tree, one line changed, two renderer wasm builds, nothing else different:

| build | `box-fillet-preview`, edit, 180 s budget | renders |
|---|---|---|
| scope honoured (`🗑️generated/wgpu-dirty-scope/solo-fillet-long/`) | **never converged** | 36 |
| scope forced to `Full` (`…/ab-fillet-fullscope/`) | **21.13 s** | 96 |

**A window body's render is also the guest crossing that funds the guest's own background
evaluation.** `render_with_document` submits a turn; that turn is what pumps the guest's worker pool;
the brep solve advances roughly in proportion to how many surfaces the host re-renders. Narrowing a
settle withdraws compute from a solve the UI has nothing to do with. The hop count is identical
either way (7 `flowEvalTick settled` in both), so this is not a lost re-arm — it is lost compute.

### 4.4 Two narrower rules were built and measured. Both stall.

| rule | result |
|---|---|
| **per surface** — never skip a window whose producer reports `computing` (`…/owes-box-fillet-preview/`) | 36 renders, still no convergence: the solve is funded by crossings to ANY surface, not only to the one that is working |
| **step aside entirely while a producer reports `computing`** (`…/aside-box-fillet-preview/`) | `evaluating=false` on every pass and **zero `"computing":true` lines in the whole run** — most examples never publish that status at all, so there is nothing to step aside on |

So the blocking dependency is named rather than papered over: **the guest's evaluation needs a pump of
its own — the frame loop's, not the refresh's.** That is the `wgpu-progress-visibility` lane's own
subject ("evaluation cadence across frames"). The moment it exists, the three predicates are already
there, already law-covered in both languages, and `refresh_ui` gates on them in one line.

### 4.5 What was therefore landed from item 1

Kept, live and load-bearing: the kernel predicates + union (React runs their TypeScript twins today),
the shared oracle and both twins, the `ui_scope` decode fix (§3.3), the scope threading and its
`[DEBUG]` traces (the evidence above), and the per-surface retirement ordering (`window_ui` remove →
`retire_one_surface_document` INSIDE the loop, never ahead of it) so a future skip cannot retire a
document it then refuses to re-mint.

Reverted: the skip itself. `refresh_ui` renders every surface, exactly as before this lane.

---

## 5. Item 2 — one revision authority, landed

### 5.1 The two halves are one decision

`🪟️winit-app/🦀️.rs:133` already stated the law — the generation "advances when input changes … never
underneath a live one" — and only `redraw_core` honoured it. `enqueue_host_event`/`enqueue_host_metrics`
renumbered on every pointer move, which `poll_runtime_and_resubmit` reads as `frame build superseded`
and cancels the in-flight build from phase 0: 28 supersessions per converging edit, a measured 26 %
tax (§6 of the previous report).

Suppressing that alone was measured **worse**: a build that now runs to completion reaches a presenter
whose `BeginGpu` re-read `presentation_authority.current()`, which `mark_scene_changed` moves several
times per tick, and the correctly-built packet was refused — `offscreen prepared frame admission:
prepared render revision is stale: live=35, packet=27` → `worker-present-failed`, a dead surface.

Both halves now land together:

* **`FrameGenerationHold`** (`🪟️winit-app/🦀️.rs`) — `Free` or `UnderLiveBuild`, from the same
  `frame_build.has_live_session()` predicate `redraw_core` already used. The event is still enqueued
  and the scheduler still invalidated either way: holding the NUMBER is not dropping the INPUT.
* **`RuntimePresentationAuthority::admit_build` / `admitted`** (`🧊️renderer/🦀️.rs`) — the frame build
  writes the witness it will build against when it mints its `FrameBuildCursor`; `AppPresenter::present_step`'s
  admission reads THAT pair instead of the live one. One writer, one reader, no race:
  `poll_runtime_and_resubmit` admits at most one build at a time, and `redraw_core` admits the finished
  frame and takes its first present step inside the same host tick.

`PreparedRenderGate::validate` is untouched — it still refuses a packet that is not the one admitted,
and still catches a packet from a different build. Only the pair it is handed changed owner.

### 5.2 Proof on 6118

| reading | before | after |
|---|---|---|
| `frame build superseded` per converging edit | **28** | **0** |
| `prepared render revision is stale` / `worker-present-failed` | 1 per run when the hold was applied alone | **0** |
| battery `frame-loop` | — | **4/4 green** |
| — 20 gestures run | | `{"gestures":20}` |
| — the frame wire never quarantines | | `{"quarantines":0}` |
| — the frame wire is still answering | | `{"batches":8397,"frames":8397,"answered":true,"faults":0}` |
| — the loop keeps publishing actions | | `{"actions":16,"admits":252,"sweeps":8}` |

Measured across five independent probe runs on the final build
(`🗑️generated/wgpu-dirty-scope/both-hex-{1,2,3}/`, `…/final-{box-fillet-preview,hexagonal-mushroom-column}/`):
`frame build superseded` = 0 and `revision is stale` = 0 in every one.

---

## 6. Laws

| law | where | state |
|---|---|---|
| the `UiDirtyScope` selection + union oracle | `🎠️kernel/🧫️fixtures/🐢️ui-dirty-scope/🔣️.json` — 5 selections, 5 unions, 5 declared laws | new |
| Rust twin | `🎠️kernel/🧪️tests/🐢️ui-dirty-scope/🦀️.rs` — 5 tests | **ran, green** (`cargo test -p semio-framework --lib ui_dirty_scope` → 6 passed incl. 2 pre-existing) |
| TypeScript twin, INDEPENDENT oracles | `🎠️kernel/🧪️tests/🐢️ui-dirty-scope/🟦️.ts` — `zod`'s discriminated union validates every wire shape, `lodash.union` recomputes every body-key union, `fast-deep-equal` compares | **ran, green** (`bun nx run workspace:ui-dirty-scope`) |
| non-vacuity: a shell that renders EVERY surface must FAIL this oracle | both twins | new, green |
| the frame-generation hold + shared authority oracle | `📺️renderer/🧑‍🎨engine/🧫️fixtures/🔢️frame-generation-hold/🔣️.json` — 6 transcripts, 5 declared laws, incl. *a hundred input events during a build neither supersede it nor quarantine the surface* | new |
| Rust twin, over the REAL callbacks and the REAL authority | `🧪️tests/🔢️frame-generation-hold/🦀️.rs` — 3 tests | **ran, green** (`cargo test -p semio-framework-os-renderer-wgpu --lib frame_generation_hold` → 3 passed) |
| TypeScript twin, independent re-derivation | `🧪️tests/🔢️frame-generation-hold/🟦️.ts` — 7 tests | **ran, green** (`nx run …:test-browser-worker` → 6 files, 78 tests) |
| the counterfactual: a presenter that re-reads the LIVE authority quarantines the surface | both twins | new, green |
| a scoped refresh settles only the surfaces it visited | `🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` | new, plus the existing fault law updated |

**The failing-first check was run, not assumed.** With the hold removed from the TypeScript twin
(`generation += 1` unconditionally) the suite fails exactly as intended — `expected { generation: 100
… } to deeply equal { generation: +0 … }` and the same for 40 metrics events — and passes with it.
The dirty-scope oracle's non-vacuity guard is the same check made permanent.

---

## 7. React was verified, not assumed

The kernel additions and the `ShellHelpers` delegation are on React's own path.

* `🐍️journey-probe.mjs` on 6018: **23/23 steps `converged=true`**, hexagonal column 3 meshes, every
  other example 1, generate-mode → Add Generation → back-to-edit → viewer-role all green
  (`🗑️generated/wgpu-dirty-scope/react-journey-1/`).
* The 31 React dirty-scope laws in `🧪️tests/🔬️engine-contract/🟦️.ts` pass against the shared kernel
  predicates — `buildUiRefreshRequest` for full / partial / matches-nothing, catalogue-on-full-only,
  tools-flag-only, "asks for nothing at all on a none scope", the coalescer union, and the
  example-switch completion-scope family.
* That suite is 604/605. The one failure — *"styles the projection pane body like window options"* —
  is `TypeError: Cannot read properties of undefined (reading 'escape')` on `CSS.escape` in jsdom, in
  a puzzle3d projection-pane styling test another lane touched in `7c296af6fd`. It is unrelated to
  anything here, it did not block this lane, and it was not fixed.

---

## 8. What is NOT claimed

1. **"Every example within 2× of React" is not met, and this lane did not move the mean.** The change
   that would have is §4's, and it cannot ship until the guest's evaluation has a pump of its own.
   On the final build the hexagonal column converges in 13.12 s and box-fillet-preview in 12.45 s
   against React's 4 s and 6 s — i.e. essentially the previous lane's numbers, which is the honest
   outcome of landing item 2 and reverting item 1's behaviour.
2. **`UiDirtyScope` is threaded but not honoured.** No surface is skipped today. The scope is computed,
   unioned, traced and law-covered; the `refresh_ui` docstring carries the whole measurement and names
   the blocking dependency.
3. **The guest-compute coupling was diagnosed, not fixed.** That the brep solve advances on UI render
   crossings is measured here (§4.3) and owned elsewhere.
4. **No guest, plugin or React-target behaviour was changed.** The only React-visible edit is
   `ShellHelpers` delegating four predicates to the kernel, verified in §7.
5. **`world3d-editor` is 8/10** on this build — `h1_hover_centre` and `h2_hover_empty` fail with
   `hover: "None"`. Hover is the `wgpu-world3d-gaps` lane's surface; this lane neither caused it (the
   frame-loop and present paths are green) nor fixed it.
6. **Only `--only=examples,frame-loop,world3d-editor` was run.** Every other row in
   `🗑️generated/wgpu-verify/scoreboard.json` is carried from earlier lanes.
7. **No native/winit proof.** Everything on 6118/6018 is the browser worker path. The Rust twins do
   run natively.
8. **The examples battery is slow enough to hit its own 60-minute budget** on a machine with eight to
   ten peer `cargo` processes; the first run of it was killed at 3 600 s with 15 of 16 rows written.

---

## 9. Files

Changed:

* `🧰️framework/🔨️modules/🎠️kernel/🦀️.rs` — `UiDirtySection`; `UiDirtyScope::{asks_for_nothing,
  wants_window_body, wants_panel_body, wants_section, wants_catalogue, merged_with}`; test mount.
* `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` — the TypeScript twins of all of the above.
* `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🟦️.tsx` —
  `mergeUiDirtyScopeV1` and the four `uiRefreshWants*` predicates now delegate to the kernel.
* `…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — `owed_refresh_scope`, `owe_refresh`,
  `host_effect_earned_scope`, `refresh_scope_label`, `refresh_ui(ask)`, scope threading through
  `dispatch_action`/`dispatch_command`/the extension door/`apply_mutations`/`apply_ops_inner`/
  `settle_ui_chain`, per-surface retirement ordering, `settle_surface_faults(faults, visited)`.
* `…/🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs` — decode the wire `ui_scope` (§3.3).
* `…/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs` — `FrameGenerationHold`, `OsHost::frame_generation_hold`,
  both enqueue callbacks, the new test mount.
* `…/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs` — `RuntimePresentationAuthority::{admit_build, admitted}`,
  `RuntimeMailbox::admit_build_presentation_witness`, the build cursor writes it, `present_step`
  reads it.
* `…/🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs` — the new callback signature.
* `…/🧱️elements/🐚️Shell/🧪️tests/🔬️wgpu-shell-boot-isolation/🦀️.rs` — visited-scoped fault settling.
* `…/🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`, `…/📦️packages/🟦️typescript/📜️script.ts` — the new
  vitest suite.
* `📜️script.ts`, `📋️project.json`, `.vscode/launch.json` — the `ui-dirty-scope` / `-native` commands.

Created:

* `🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🐢️ui-dirty-scope/🔣️.json` + `🧪️tests/🐢️ui-dirty-scope/{🦀️.rs,🟦️.ts}`
* `…/🧑‍🎨engine/🧫️fixtures/🔢️frame-generation-hold/🔣️.json` + `…/🧪️tests/🔢️frame-generation-hold/{🦀️.rs,🟦️.ts}`

Evidence, under `🗑️generated/wgpu-dirty-scope/`: `scope-hex-1/`, `both-hex-{1,2,3}/`,
`solo-fillet-long/`, `ab-fillet-fullscope/`, `owes-*/`, `aside-*/`, `final-*/`, `react-journey-1/`,
`battery-after.txt`, `battery-final.txt`.

Report: this file.
