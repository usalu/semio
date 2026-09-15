# Tool Run Pace Storm And The Frozen Run Pill — lane `tool-run-pace-storm` (2026-09-14/15)

Closes coordinator-walk rows 12, 16 and 18 (`📓️coordinator-walk-2026-09-14.md`): the endless
`toolRunPace` action storm on a quiet, converged React generation3d editor, and the run pill that froze at
the first phase and disagreed with the preview about how many meshes were delivered.

- Focused probe: `🐍️tool-run-pill-recon.mjs` (new)
- Restage script: `📜️restage-pace-storm.sh` (new)
- Runners: `🔍️preview-eval-observations.ts`, `🔍️tool-run-terminal-quiet.ts` (new)
- Runtime logs: `🗑️generated/pace-storm/`, `🗑️generated/react-pace/`

---

## 1. TL;DR

**Two defects, two layers, and the storm's own mechanism was deleted from the working tree by its owner
twenty minutes before this lane started.**

1. **The storm** is the pace wake of the peer's tool-run pacing facility re-arming itself on a run that was
   already `Finalized`. The wake was gated on PRESENTATION, never on the run's own liveness, so a run with
   nothing left to present re-armed at the presentation-retry interval forever — the 100 ms retry is exactly
   the ~10/s cadence the coordinator measured (seq 107…116 within seconds). **The owning peer removed the
   whole pacing facility from the working tree at 23:23:56**, before this lane read a line of it. This lane
   did NOT revert that removal and did NOT re-add pacing. It added the durable guard the storm was missing:
   a framework law that **a terminal run arms nothing**, with a TypeScript twin.
2. **The pill froze at `Evaluating nodes (1/2)`** because generation3d's own observation answered
   `PreviewEvalRunStage::Evaluate` whenever `meshes == tessellated` — true of a run that has not reached a
   mesh yet (`0 == 0`) and equally true of a run whose every mesh is DONE. The stage fell back to the first
   one at the exact moment the run converged. **Fixed**: nodes are being evaluated while any node is queued
   or computing, or while no mesh exists at all; anything else is tessellation and stays there.
3. **The pill's mesh count was short** because the census counted only channel items carrying a kernel
   handle. An inline point or vector marker carries none — it is built without a kernel round trip — so the
   hex column's vector channel was invisible to the pill while the preview published it as one of three
   meshes. **Fixed**: the census counts every geometry-bearing channel item, and a handle-less one is READY
   the moment it is observed. The pill now agrees with the preview's delivery AND with the committed example
   oracle (`delivery.meshes` 3 for the hex column, 1 for the sphere cut).

Measured on 6027 after the restage: **0 `toolRunPace` lines per minute** on a converged shell (98 console
lines in 120 s, against 2 800 dropped lines in minutes at 23:25), pill
`Finalized · Tessellating meshes (2/2)` / `7 nodes evaluated, 3 meshes tessellated`, React battery
`green=3/3 red=[] pageerrors=0`.

---

## 2. Root cause

### 2.1 The pace storm

`toolRunPace` is **not** one of the seven declared §2.5 actions (`TOOL_RUN_ACTION_IDS`). It was an INTERNAL
wake the peer's pacing facility handed the host as `Effect::DispatchAction { action, delay_ms }`, which the
host answers by dispatching the id straight back into the guest. Its design, from the owner's own record
(`26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📓️status.md`, entries 19:38 and 20:10–21:05):

- `ViewModel.tool_run_units_per_second`; the React host stamped `TOOL_RUN_VISIBLE_UNITS_PER_SECOND` (20)
  into BOTH dispatch view states, so every run of every app on the React shell was a PACED run.
- A paced running run steps with fuel 1, sets `pace_due_us` after each visible unit and hands the host one
  wake delayed by the interval; `pace_wake_outstanding` keeps at most one wake in flight.
- A **presentation gate**: the run is ready only when the interval elapsed AND every scene window its trace
  reaches echoed a cursor at or after the trace's newest page, or stalled `TOOL_RUN_TRACE_STALL_REFRESHES`
  (4) refreshes. **A wake that arrives before presentation re-arms at
  `TOOL_RUN_PACE_PRESENTATION_RETRY_MS` (100).**

The re-arm was keyed on the presentation gate alone. Nothing in that chain asks whether the run is still
alive, and `toolRunPace` — being no declared action — never passed the legality rule that refuses every
tool-run action but Start and Dismiss on a terminal run. On generation3d's converged editor the gate can
never be satisfied and never be abandoned:

- the `previewEval` run is `Finalized`, so it publishes no further trace page for a renderer to echo;
- the stall counter advances per REFRESH, and a quiet shell refreshes for nothing, so it never reaches 4;
- so every 100 ms the wake came back, found the gate unsatisfied, and armed the next one.

**Every pace answer armed the next pace.** 1000 ms / 100 ms = 10 per second, which is the cadence the walk
measured (seq 107 → 116 within seconds, 2 800 console lines dropped in minutes) on a shell nobody was
touching. It is the same shape as the `toolRunStart` storm of
`📓️preview-rearm-after-inspector-edit-2026-09-14.md` — **a latch read once per RETRY instead of once per
ANSWER** — with one aggravation: that latch at least belonged to a declared action, and this one had no
terminality guard at all.

This reconstruction is from the owner's design record plus the measured cadence and the surviving
`TOOL_RUN_TRACE_STALL_REFRESHES` constant. The implementation itself was **already gone** when this lane
opened the files (§2.3), so no line of the pacing code was read by this lane.

### 2.2 The pill

| # | site | what was wrong |
|---|------|----------------|
| 1 | `…/🧊️generation3d/…/🧵️preview-eval/⏯️tool-run/🦀️.rs` `PreviewEvalObservation::stage` | `meshes == tessellated` meant BOTH "no mesh yet" and "every mesh done", so the stage fell back to `Evaluate` the moment the run converged and the pill read `Finalized · Evaluating nodes (1/2)` over a finished evaluation |
| 2 | same file, `observe_preview_eval` | `.filter(\|item\| !item.handle.is_empty())` dropped every INLINE point/vector channel item from the census, while the preview publishes one mesh per geometry-bearing channel item including those; the hex column read `2 meshes tessellated` for a delivery of three |

The two are one defect twice over: with the inline marker missing, the hex column's census was `2 == 2`,
which is also why its stage answered `Evaluate`. The sphere cut has a single solid channel, so its count was
right by accident and only its stage was wrong.

The framework panel was NOT at fault — a previous lane had already moved the announcement throttle off the
text (`🔌️plugin/⏯️tool-run/🦀️.rs` `tool_run_panel_group`), so the pill renders whatever stage the guest
publishes. It published the wrong one.

### 2.3 What the owning peer had already done

At **23:23:56**, before this lane started, the owner of the Tool runs panel removed the pacing facility from
the working tree (uncommitted; the index snapshot at `842764d694`, 23:24:44, caught the removal half-done):

- `🧰️framework/🔨️modules/⏯️tool-run/{🦀️.rs,🟦️.ts}` — `TOOL_RUN_VISIBLE_UNITS_PER_SECOND`
- `🧰️framework/🔨️modules/🛂️manifest/{🦀️.rs,🟦️.ts}` — `ViewModel::tool_run_units_per_second` and its bound check
- `🧰️framework/…/🏛️ShellHost/🟦️.tsx` — the host's pace stamp in both dispatch view states
- `🧰️framework/…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` — the field in all six view-state constructions
- `🧰️framework/…/🔌️plugin/{🧪️tests/🔬️tool-run/🦀️.rs,🧫️fixtures/⏯️tool-run/🔣️.json}` — the `pace` rows and laws

`grep` over `🧰️framework` and `✏️s` finds no `toolRunPace`, `TOOL_RUN_PACE`, `tool_run_units_per_second`,
`pace_due_us` or `pace_wake_outstanding` anywhere in the working tree except the three docstrings this lane
wrote (`🗑️generated/pace-storm/evidence.txt`). The storm was therefore already extinct on 6027 **before this
lane restaged anything**: the host serves its TypeScript live, so the removed stamp was already in the
served transform (`curl` of the `/@fs/` ShellHost module: 0 occurrences) while the staged 20:14 guest still
carried the verb. This lane's first measurement on the converged sphere example already read 0.

**Nothing of that removal was reverted and pacing was not re-added.** The facility is the peer's to shape;
what this lane owes it is the law that catches the defect if it comes back.

---

## 3. The fixes

### 3.1 App — `✏️s/…/🧊️generation3d/…/🧵️preview-eval/⏯️tool-run/🦀️.rs`

- `PreviewEvalObservation::stage` reads `self.meshes == 0` rather than `self.meshes == self.tessellated`.
  A run that has produced no mesh is evaluating; a run that has produced any is tessellating and STAYS
  there, so a terminal run shows its terminal stage.
- `observe_preview_eval` maps every channel item to a state instead of filtering the handle-less ones out:
  a handle asks the session, and an inline marker — minted with an empty handle by
  `collect_preview_channel_items` and built without a kernel round trip by `point_marker_mesh` — is `Ready`
  the moment it is observed. The census now equals what the preview publishes, which is the only number a
  reader can check against the screen.

### 3.2 Framework — the guard the storm was missing

New law + fixture table `terminalQuiet` stating what no pacing implementation may break again.

### 3.3 Not touched

The framework ToolRun panel, the React `⏯️tool-run-panel` helper, the wgpu panel twin and every pacing file
the peer is editing.

---

## 4. Laws

Language-agnostic: a fixture table, a Rust law over the real ledger, and an independent TypeScript twin.

| fixture table | rows | Rust | TS twin |
|---|---|---|---|
| `🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json` → `terminalQuiet` | 2 (a read-only run that finalized itself, a mutating run the user aborted) × 64 driver turns | `a_terminal_run_arms_no_further_work_however_many_turns_the_host_takes` | `🔌️plugin/🧪️tests/🛑️terminal-quiet/🟦️.ts` |
| `…/🧊️generation3d/…/🧫️fixtures/⏯️preview-eval-run.json` → `observations` | 6 → **8** (one expectation corrected, two rows added) | `every_observation_row_answers_its_verdicts_census_stage_and_progress` | `…/⏱️flow-eval-tick/🧪️tests/🔬️unit/🟦️.ts` |

**The terminal-quiet law.** A run that is OVER cannot ask to be driven again. Every wake a run hands the
host is a REQUEST the host answers by dispatching an action back into the guest, so a request a terminal run
keeps re-making is an endless action storm on a quiet shell. The Rust law drives a real ledger: it starts
the run, brings it to a terminal state, drains the effects that belong to getting there, then takes 64
further driver turns and asserts the ledger reports no pending work on any of them and hands the host
**zero** effects. The TypeScript twin reads the same claim off the framework's own reducer
(`⏯️tool-run/🟦️.ts`'s `ToolRunMachine`): from each of `finalized`, `aborted` and `faulted`, of the eighteen
event keys only `start` (a NEW run), `dismiss` and `closed` are admitted at all, none of them yields a
scheduling effect, and `isToolRunActionLegal` offers no action but Start and Dismiss.

```
test component::app::tool_run_tests::a_terminal_run_arms_no_further_work_however_many_turns_the_host_takes ... ok
test result: ok. 1 passed; 0 failed

toolRun terminalQuiet turns=64 rows=2 terminalStates=finalized,aborted,faulted
  admitted=finalized/start->spawnJob finalized/dismiss->clearTrace finalized/closed->retireAll
           aborted/start->spawnJob aborted/dismiss->clearTrace aborted/closed->retireAll
           faulted/start->spawnJob faulted/dismiss->clearTrace faulted/closed->retireAll
```

**The observation law.** `every-mesh-ready-is-tessellated` had `stage: "evaluate"` committed for a run whose
single mesh was done — the defect pinned as an expectation; it now reads `tessellate`. Two rows were added:
`an-inline-marker-channel-is-one-published-mesh` is the hex column (a wire handle, an inline vector, a solid
handle → 3 meshes, 3 tessellated, stage `tessellate`, progress 6/6), and
`an-inline-marker-beside-a-mesh-that-has-not-arrived` holds the marker ready while the solid beside it is
still pending (2 meshes, 1 tessellated).

```
cargo test -p semio-s-artifact-procedural-generation3d --features component-app-assembly --lib -- preview_eval::tests
  → 14 passed; 0 failed
bun 🔍️preview-eval-observations.ts
  → generation3d preview-eval-run entityKeys=7 observations=8 scheduling=10 status=5 runEffects=10
```

**Red first.** With the twin's two lines put back to the old rule and the fixture left as it now stands:

```
AssertionError: every-mesh-ready-is-tessellated: census
```

---

## 5. Runtime proof on 6027

Port 6027 is this lane's (`curl` 200, `screen -S g3dreact6027`, host TypeScript served live). Guest
restaged in the foreground, `📜️restage-pace-storm.sh`, `EXIT=0 attempt=1`
(`🗑️generated/pace-storm/restage.txt`); staged component at 23:59.

### 5.1 The storm

`SEMIO_PROBE_URL=http://127.0.0.1:6027/?plugin=generation3d SEMIO_PROBE_SECONDS=120
SEMIO_PROBE_OUT=pace-storm/after bun 🐍️console-dump-probe.mjs`

| session | `toolRunPace` lines | console lines |
|---|---|---|
| coordinator walk row 18, 23:25, 20:14 guest | storm: seq 107…116 within seconds, **2 800 lines dropped in minutes** | dropped |
| this lane, 6027 before the restage (host stamp already removed by the peer) | **0** in 120 s | 102 |
| this lane, 6027 after the restage | **0** in 120 s | 98 |

Per-minute after convergence, from `🐍️tool-run-pill-recon.mjs`: **0** on the hex column and **0** on
`sphere-cut-with-torus`. The whole 120 s converged session carries six action invocations in total
(`toolRunStart`, `setActiveExample`, `interactionSelect`, `setContributions`).

### 5.2 The pill

Read out of the live DOM (`panel:framework.toolRun/framework.toolRun.1`) with the preview's own published
delivery beside it.

| example | pill before (walk rows 12, 16) | pill after | preview delivers | oracle `delivery.meshes` |
|---|---|---|---|---|
| hexagonal mushroom column | `Finalized · Evaluating nodes (1/2)` / `7 nodes evaluated, 2 meshes tessellated` | `Finalized · Tessellating meshes (2/2)` / `7 nodes evaluated, 3 meshes tessellated` | 3 (`wire`, `vector`, `solid`) | **3** |
| sphere cut with torus | `Finalized · Evaluating nodes (1/2)` / `6 nodes evaluated, 1 meshes tessellated` | `Finalized · Tessellating meshes (2/2)` / `6 nodes evaluated, 1 meshes tessellated` | 1 (`solid`) | **1** |

The progress bar reads `Tessellating meshes (2/2): 10 of 10 nodes and meshes (100 %)` on the hex column and
`7 of 7` on the sphere cut. **The preview was right and the pill was wrong**; the pill now agrees with the
preview's delivery and with the committed oracle on both.

### 5.3 The battery

`SEMIO_BATTERY_URL=http://127.0.0.1:6027/?plugin=generation3d SEMIO_BATTERY_ROOT=react-pace
bun 🐍️react-battery.mjs --only=keyboard-verbs,status-parity,cancel-preview`

```
battery ■ keyboard-verbs   ok=true steps=…  pageerrors=0
battery ■ cancel-preview   ok=true          pageerrors=0
battery ■ status-parity    ok=true 200s steps=12/12 pageerrors=0
BATTERY DONE green=3/3 red=[] 240s pageerrors=0
```

`status-parity`'s `boot` row reads `["window:procedural-preview", 3, "idle"]` — the three hex-column meshes
the pill now counts.

---

## 6. The wgpu twin

By code reading, the wgpu surface obeys the same law and needs no change of its own:

- `🐚️Shell/🧪️tests/⏯️wgpu-tool-run-panel/🦀️.rs` paints the SAME golden panel fixture
  (`🔌️plugin/🧫️fixtures/⏯️tool-run/🪧️panel-running.json`) the Rust plugin law pins and the React helper
  renders. It holds no arming, scheduling or pacing logic of its own, so the pill fix — which is entirely in
  the guest's observation — reaches it unchanged.
- The wgpu shell has **no timer wheel**: every `Effect::DispatchAction` is pushed onto `deferred_actions`
  with `delay_ms` dropped (`🎯️targets/🧊️wgpu/🦀️.rs`, and the doc note at `:657` says so outright). A paced
  wake there would storm HARDER than on React, not less — a re-armed wake with no delay at all. The peer's
  own record already listed wgpu pacing as unbuilt for exactly this reason; the terminal-quiet law is the
  guard that keeps a future wgpu pace from re-arming a dead run.
- The peer's removal also took `tool_run_units_per_second` out of all six wgpu view-state constructions, so
  the field no longer exists to stamp.

Runtime, 6118 at 01:10 behind the shared wgpu gate (`🗑️generated/pace-storm/wgpu-battery.txt`, scoreboard
`🗑️generated/wgpu-pace/scoreboard.json`), against the guest a peer staged at 01:04 — which carries this
lane's `⏯️tool-run/🦀️.rs` change:

```
SEMIO_BATTERY_URL=http://127.0.0.1:6118/?plugin=generation3d SEMIO_BATTERY_ROOT=wgpu-pace \
  bun 🐍️wgpu-battery.mjs --only=chrome,status-a11y-i18n
battery ■ chrome            ok=false 144s steps=4/5 pageerrors=0   red: the world3d cancel control is declared
battery ■ status-a11y-i18n  ok=false 160s steps=8/9 pageerrors=0   red: accessibility:live
BATTERY DONE green=0/2 red=["chrome","status-a11y-i18n"] 304s pageerrors=0
```

**Both reds are pre-existing and are this ticket's own already-recorded wgpu reds**, not regressions of this
lane: `the world3d cancel control is declared` is red in `🗑️generated/wgpu-perf/scoreboard-before.json`
(07:18) and `accessibility:live` in `🗑️generated/wgpu-genfit/scoreboard-before-1532.json` (13:04), both long
before this change. This run is strictly BETTER than the 07:18 baseline (3 chrome reds → 1, 5 a11y reds → 1),
and `status:pill-while-computing`, red at 07:18, is green here. No step that reads the run pill is red.

---

## 7. Files

Created:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🛑️terminal-quiet/🟦️.ts`
- `🐍️tool-run-pill-recon.mjs`, `📜️restage-pace-storm.sh`, `🔍️preview-eval-observations.ts`,
  `🔍️tool-run-terminal-quiet.ts`, `📓️tool-run-pace-storm-2026-09-14.md` (this report)

Updated:
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/⏯️tool-run/🦀️.rs`
  — the stage rule, the inline-marker census
- `✏️s/…/🪆️subsets/✳️any/🧫️fixtures/⏯️preview-eval-run.json` — `observations` 6 → 8 rows, one expectation
  corrected
- `✏️s/…/✏️editor/🎮️commands/⏱️flow-eval-tick/🧪️tests/🔬️unit/🟦️.ts` — the TypeScript twin of both
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/⏯️tool-run/🔣️.json` — `terminalQuiet`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️tool-run/🦀️.rs` — the terminal-quiet law

---

## 8. What is NOT claimed

- **The storm was never reproduced by this lane.** Its mechanism had been removed from the working tree at
  23:23:56, and the React host serves its TypeScript live, so the pace stamp was already out of the served
  ShellHost transform when this lane's first probe ran. The record of the storm is the coordinator's walk
  row 18; what this lane measured is 0 before and 0 after, and what it reasoned about is the owner's own
  design record. No line of the pacing implementation was read.
- **No claim that the pacing facility is finished or correctly removed.** Its owner is mid-removal: the
  index at `842764d694` still carries `TOOL_RUN_PACE_ACTION_ID` in `🔬️tool-run/🦀️.rs` against a constant
  that exists nowhere, so HEAD's plugin test binary does not compile as committed. The WORKING TREE does —
  the two cargo suites in §4 were built and run from it. Whether pacing returns, and in what shape, is the
  peer's call; this lane only left the law it must satisfy.
- **The terminal-quiet law is a guard, not the pace fix.** It states that a terminal run arms nothing, over
  the framework's real ledger and its own reducer. It does NOT state the second half of the brief's rule —
  "a running run arms at most one pace per progress publication" — because there is no pacing mechanism in
  the tree to state it over, and inventing one to hold a law would be re-adding the peer's feature.
- **No wgpu browser evidence of the pill itself.** §6's argument is code reading plus the shared golden
  fixture; the guest change is surface-neutral and must reach wgpu identically, but the 6118 run is the
  chrome/a11y pair, not a pill reading, and its two reds were neither investigated nor fixed here (they are
  this ticket's pre-existing wgpu reds and belong to the wgpu lanes).
- **The 6118 run did not measure pace lines.** It ran against a guest a peer staged at 01:04; the React
  measurements of §5 are on this lane's own 23:59 restage on 6027.
- **The `Attempts` tree still reads `No data`** in the React panel (`traceRows: 0` in every recon), and the
  step log holds one row. That is the peer's open item from their own §Open list, untouched here.
- **Nothing about the role-switch regression** of walk rows 18 and 19 (the `Viewer` control doing nothing,
  instance 2 appearing). It shares row 18 with the storm but is a different defect and a different lane.
- **The whole cargo workspace was not built.** `semio-framework-plugin`'s and
  `semio-s-artifact-procedural-generation3d`'s lib test binaries compile — that is what running their suites
  proves — and no other crate was compiled against these edits.
- **`observe_preview_eval`'s new inline-marker state is a decision, not a measurement.** An inline marker is
  built in the guest without a kernel round trip, so nothing can make it "pending"; if a future inline
  geometry ever did need work, it would need its own state, not the handle filter back.
