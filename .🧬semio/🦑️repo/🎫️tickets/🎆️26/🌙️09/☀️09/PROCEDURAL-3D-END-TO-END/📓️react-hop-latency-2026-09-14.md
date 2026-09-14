# ⏱️ React per-hop round-trip latency — instrumented, attributed, and what it costs (lane `react-hop-latency`, 2026-09-14)

Opus lane on port **6021** (`📜️serve-generation3d-react-6021.sh`, host TS vite-live, `SEMIO_VITE_HMR=0`).
Answers `📓️react-perf-ceilings-audit-2026-09-14.md` §2d/§2f and §3 items 1, 2, 5, 6. Host-only: no guest
wasm restaged, no Rust touched, no git command run.

---

## 0. The blocker, and when it cleared

The lane opened against the `framework.panel.toolRun: DuplicateSiblingKey` fault the coordinator walk
recorded (`📓️coordinator-walk-2026-09-14.md` item 2). First poll 17:12
(`🗑️generated/react-hop/boot/`): **17 faults, nothing converged** — every extension dispatch completion
died and the preview sat at `Computing 5/7`. Second poll 17:2x
(`🗑️generated/react-hop/poll2/`): the boot example **converged, 3 meshes, `nodesDone 7/7 ratio 1`**,
with 11 residual `DuplicateSiblingKey` lines that no longer stop a chain. Every number below was taken
after that. The fault itself belongs to lane `react-current-tree-battery`; this lane neither fixed nor
claims it.

---

## 1. What a hop is made of — the instrument

A hop is one `flowEvalTick` and everything the host does with its answer. There was no per-stage
instrument for it on the React door (§4 item 1 of the audit: "that probe does not exist"). There is now.

**`🧰️framework/🔨️modules/⏱️trace/🟦️.ts`** (new — the browser twin of that module's Rust span
vocabulary) publishes every stage as a User Timing `performance.measure` under `semio.hop.*`, with a
`detail` naming the instance and action, plus a bounded ring the laws read without a browser. Stages,
declared once in `🧫️fixtures/🪃️hop-stages/🔣️.json` and read by both the host and the probe:

| stage | owner | covers |
|---|---|---|
| `encode` | `PluginRuntime.performInvocation` | pack-encoding the invocation + admitting the view context |
| `channel` | `PluginRuntime.performInvocation` | the worker crossing: post, guest turn, reply frames |
| `decode` | `PluginRuntime.performInvocation` | reply frames → `InvocationResponse` |
| `invoke` | `PluginRuntime.performInvocation` | the whole dispatch |
| `refresh` | `ShellHost` ui-refresh lane | one whole refresh pass |
| `refresh.turn` | `PluginRuntime.refreshUi` | serialized actor ingress + the guest turn that re-renders |
| `refresh.project` | `PluginRuntime.ownedUiRefreshResponse` | retained surfaces → the bodies the shell asked for |
| `refresh.slots` | `ShellHost.runUiRefreshPass` | external-slot resolution on changed bodies |
| `refresh.apply` | `ShellHost.runUiRefreshPass` | the React state dispatches that apply a response |
| `commit` | `ShellHost` | React commit + paint (dispatch → next frame) |
| `arm` | `ShellHelpers.scheduleDispatchAction` | scheduling the next dispatch → it running |
| `mesh.decode` | `World3dHost.WorldInstancesLayer` | building the three.js buffers a scene changed |

The tracer never changes what a hop settles or its order — it only opens and closes spans; a measure
sink that throws is swallowed (law below). Cost is one `performance.now()` pair + one `measure` per
span.

**`🐍️react-hop-cost-probe.mjs`** (new, in this ticket folder) runs every bundled example on one page,
clears and re-reads those measures per step, and adds CDP `Performance.getMetrics` deltas for the page
(`ScriptDuration`, `TaskDuration`, `RecalcStyleDuration`, `LayoutDuration`). It writes
`hops.json` + `table.md` (+ `spans-*.json` with `SEMIO_PROBE_RAW_SPANS=1`) under
`🗑️generated/react-hop/<out>/`.

```
cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d \
  SEMIO_PROBE_OUT=react-hop/before bun 🐍️react-hop-cost-probe.mjs
```

**Gap, honestly flagged**: the probe's shard-worker column reads `0` in every run. Worker targets are
listed by `/json/list` and attach over CDP, but the `Performance` domain answers nothing on them, so
guest compute could not be separated from message-passing *from outside*. The split that mattered was
obtained differently — see §2.3.

---

## 2. The per-hop breakdown, before

`🗑️generated/react-hop/before/` (10 steps, 57 `flowEvalTick` hops, 10/10 converged, 79.3 s).
`refresh.turn`/`refresh.project` come from `🗑️generated/react-hop/split/` (same tree, the run that
added those two stages).

### 2.1 Per step

| step | s | hops | tick ms/hop | inter-hop gap ms/hop | channel ms/hop | refresh.guest ms/hop | commit ms/hop | arm ms/hop | mesh.decode ms/hop | recalc ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| boot | 16.3 | 7 | 517 | 692 | 1126 | 1330 | 149 | 46 | 0 | 534 |
| No example | 4.2 | 2 | 396 | 0 | 761 | 684 | 28 | 45 | 0 | 56 |
| Hexagonal Mushroom Column | 4.5 | 3 | 360 | 344 | 583 | 831 | 71 | 20 | 0 | 68 |
| Rectangle Extrude Volume | 10.6 | 8 | 486 | 492 | 569 | 1005 | 87 | 10 | 0 | 396 |
| Sphere Cut With Torus | 9.4 | 7 | 386 | 513 | 484 | 926 | 76 | 24 | 0 | 245 |
| Box Fillet Preview | 6.1 | 6 | 330 | 350 | 424 | 725 | 65 | 17 | 0 | 164 |
| Sphere Box Fuse | 8.3 | 7 | 399 | 440 | 489 | 872 | 121 | 18 | 0 | 254 |
| Face Sweep Extrude | 9.4 | 8 | 416 | 460 | 512 | 911 | 104 | 14 | 0 | 320 |
| Rectangle Wire Preview | 4.2 | 3 | 254 | 144 | 394 | 510 | 66 | 15 | 0 | 60 |
| Box Shell Preview | 6.2 | 6 | 313 | 329 | 395 | 674 | 78 | 21 | 0 | 166 |

### 2.2 Per stage, aggregated (count × mean ms)

| stage | before | comment |
|---|---|---|
| `invoke` | 74 × **442 ms** | the whole dispatch |
| `channel` | 74 × **442 ms** | the dispatch IS the crossing — nothing else in it is measurable |
| `encode` | 65 × **0 ms** | the pack codec is not a cost; the audit's "JSON where pack exists" candidate is dead |
| `decode` | 74 × **0 ms** | so is reply decode |
| `refresh` | 119 × **435 ms** | **2.09 passes per hop** |
| `refresh.turn` | 114 × **420 ms** | ~99 % of a pass |
| `refresh.project` | 100 × **2 ms** | host-side projection is free |
| `refresh.apply` | 103 × **0 ms** | the React dispatches are free |
| `commit` | 103 × **51 ms** | React commit + paint |
| `arm` | 61 × **20 ms** | the continuation scheduler is not the cost; the "timer/RAF-quantised arm" candidate is dead |
| `mesh.decode` | 23 × **0 ms** | geometry construction is not the cost (audit §2b tail, item 7: answered) |

**Wall per hop: 1 391 ms.** Three of the audit's four §3-item-1 candidates are now excluded by
measurement: the codec (0 ms), the arm (20 ms), and the r3f rebuild (0 ms).

### 2.3 Where the ~1.4 s actually goes

A hop is **three serialized guest turns**, and nothing else:

```
[invoke flowEvalTick 532ms] → [refresh 541ms] → [refresh 533ms] → [arm 2ms] → [invoke flowEvalTick …]
```

(read off `🗑️generated/react-hop/split/spans-edit-Face-Sweep-Extrude.json`; every `refresh` there is
`scope: "full"`, `events: 14`, and each starts within 5 ms of the previous one ending).

The two refresh passes per hop are real and both `full`. With diagnostics armed
(`🗑️generated/react-hop/scopes/console.txt`, `SEMIO_PROBE_GUEST_DIAGNOSTICS=1`):

- `[DEBUG] applyHostEffects refresh {"declared":{"kind":"full"},"scope":{"kind":"full"}}` — **the
  guest's `flowEvalTick` completion declares `full`**, so `hostEffectRefreshScopeV1` has nothing to
  narrow. (The audit's §2e claim that React "already scopes its refresh" is true of the mechanism and
  false of this path: the mechanism faithfully carries a scope the guest declares as everything.)
- The extension answers declare `{"kind":"none"}` and cost nothing — that half is already right.
- `[DEBUG] refreshUi sections {"asked":["procedural-main","procedural-preview","generation3d-generations",
  "generation3d-generate-form","generation3d-generate-preview"],"changed":["procedural-preview"]}` —
  **five window bodies re-rendered per pass, one of them actually changed**, twice per hop, while the
  user is in edit mode where only two of those five windows are mounted at all.

Page-side CPU corroborates that this is not all guest compute: `Rectangle Extrude Volume` spends
**3.0 s of page `ScriptDuration` and 6.1 s of page `TaskDuration` inside a 10.6 s step** — and
`refresh.project`, `refresh.apply`, `refresh.slots`, `encode`, `decode` and `mesh.decode` together
account for ~2 ms of it. The page-side cost therefore lives *inside* `refresh.turn` and `channel`: the
shard-client crossing and the retained-UI patch application, not in anything the shell does after.

---

## 3. What was cut

### 3.1 The mounted-window fetch law (landed)

**The defect**: `runUiRefreshPass` built its request from `sessionWindowInstances(app, extra)`, which
returns every window kind the app **declares**. In the generation3d editor that is five windows; the
edit-mode layout mounts two. So every pass asked the guest to render `generation3d-generations`,
`generation3d-generate-form` and `generation3d-generate-preview` — the last of which serializes its
own mesh payload — for a user who is looking at neither, twice per hop.

**The fix** (`🔨️modules/🎠️kernel/🟦️.ts`, beside the other `UiDirtyScope` predicates both shells share,
because the wgpu `refresh_ui` window walk owes the same rule):

- `windowLayoutWindowIdsV1(layout)` — every window id the mode layout actually mounts, background
  stack tabs included.
- `partitionRefreshWindowInstancesV1(declared, mounted)` — splits declared instances into *fetched* and
  *skipped*; an empty `mounted` set (first pass, session switch) fetches everything.

and in `🏛️ShellHost/🟦️.tsx`:

- `mountedWindowIdsRef` is assigned every render from `effectiveModeLayout`.
- the pass fetches only mounted windows and **drops the cached body of every window it skipped** —
  that drop is what makes the skip safe: nothing can later serve a stale body, and the pass that
  fetches the window once it *is* mounted asks with no hash and gets a whole one.
- an effect beside `effectiveModeLayout` re-fetches, in a scope naming exactly them, any windows a
  pass skipped that the layout has since mounted — so a mode switch that races a refresh cannot leave
  a freshly mounted pane on a pending body.

**Live proof it does what it says** (`🗑️generated/react-hop/mounted2/console.txt`):

```
[DEBUG] refreshUi mounted-window fetch {"mounted":["procedural-main","procedural-preview"],
  "fetched":["procedural-main","procedural-preview"],
  "skipped":["generation3d-generations","generation3d-generate-form","generation3d-generate-preview"]}
```

(that `[DEBUG]` line was temporary and has been removed again.)

**Invariant kept**: the journey probe on 6021 after the change is **23/23 converged with every mesh
oracle green** (`🗑️generated/react-hop/journey-after/`), including `generate-mode`, `generate-added`,
`back-to-edit` and all eight `view:*` steps — i.e. the generate-mode and viewer windows still receive
their bodies the moment they are mounted. Settle order relative to `history patch applied` is
untouched: nothing in this lane changes what a tick settles, only which bodies a refresh asks for.

### 3.2 What it bought — measured, and less than hoped

| | before | after (run A) | after (run B) |
|---|---:|---:|---:|
| dir | `react-hop/before` | `react-hop/after3` | `react-hop/after` |
| wall, 10 steps | **79.3 s** | **72.1 s** | **75.0 s** |
| hops | 57 | 51 | 51 |
| wall per hop | 1 391 ms | 1 414 ms | 1 472 ms |
| `refresh` mean | 435 ms | **401 ms** | **422 ms** |
| refreshes per hop | 2.09 | 2.22 | 2.27 |
| `channel` mean | 442 ms | 397 ms | 407 ms |
| `commit` mean | 51 ms | 54 ms | 55 ms |
| `arm` mean | 20 ms | 21 ms | 21 ms |

Per example (wall seconds, before → after B): boot 16.3 → 14.3, No example 4.2 → 3.0, Hexagonal 4.5 →
5.2, Rectangle Extrude 10.6 → 8.4, Sphere Cut 9.4 → 8.1, Box Fillet 6.1 → 7.1, Sphere Box Fuse 8.3 →
7.5, Face Sweep 9.4 → 10.2, Rectangle Wire 4.2 → 4.1, Box Shell 6.2 → 7.1.

**Verdict, stated plainly: a 5–9 % total-wall improvement, and the per-hop cost did not move.** Cutting
the rendered sections of a pass by 60 % cut the pass's cost by 3–8 %. That is the finding, not a
disappointment to be buried: **a `refresh` pass costs what it costs per TURN, not per section.** The
change is kept because it is correct and removes real work (three whole window renders per pass, one of
them a mesh payload), and because it is the precondition for the next cut — but it is not the cut.

---

## 4. The cut that is left, named with its mechanism

**Three serialized guest turns per hop (1 dispatch + 2 full refreshes) at ~420 ms each is the whole
1.4 s.** §3.2 proves the lever is the *number* of turns, not their width. The two refresh passes are
not a coalescer bug — the lane behaves as designed (`decision: owed` → `pass`): each pass is requested
by a different `flowEvalTick` completion (two chains are armed per hop, one per flow window), and the
second request arrives ~4 ms after the first pass has already submitted its guest turn, so the lane
correctly owes it a pass of its own.

It is nonetheless **redundant**: between the first pass's turn submission and the second pass's request,
nothing crossed into the guest — both completions are host-side callbacks of turns that had already
settled. The sound gate is therefore:

> A refresh request whose scope is covered by the pass currently in flight, and for which **no
> guest-mutating ingress was submitted for that instance since that pass submitted its own turn**, is
> already answered by it.

That needs one thing this lane did not build: a per-instance *guest ingress generation* exported from
`🔌️PluginRuntime/🟦️.tsx` (bumped at `performInvocation` and at the extension-completion publication,
**not** at `refreshUi`'s own turn), read by the lane in `🏛️ShellHost/🟦️.tsx` before it runs an owed
pass. Expected: 3 turns per hop → 2, i.e. ~420 ms of ~1 400 ms. Not attempted here — the gate must also
account for host-owned render inputs that change with no guest ingress (view state, active utility,
locale), and getting that wrong under-refreshes the shell. It is the single highest-value item left on
the React door and it is fully diagnosed above.

The other half — `channel` at 442 ms for a dispatch whose guest work is measured natively in
microseconds (`📓️kernel-performance-2026-09-13.md` §5) — remains unattributed between message-passing
and guest turn, because the worker's `Performance` domain answered nothing (§1). Attributing it needs a
`semio.hop.*` span inside the shard worker itself, which means rebuilding `🟨️shard-worker.js` — out of
this host-only lane's scope.

---

## 5. Audit item 2 — per-mesh memoization in `WorldInstances`, with a dispose law (landed)

**The defect** (audit §2c, confirmed): `WorldInstancesLayer`'s `geometries` / `borderGeometries` /
`vertexPickByMeshId` / `edgeGeometryByMeshId` were each a `useMemo` keyed on the whole `meshes` array,
which `parseMeshes(scene.meshesJson)` rebuilds from scratch whenever any byte of the payload moves — so
one changed mesh of three reallocated all three. And **nothing disposed any of them**: every refresh of
every surface leaked a `BufferGeometry`, an `EdgesGeometry`, a vertex-pick buffer and an edge buffer per
mesh.

**The fix** (`🌐️World3dHost/🟦️.tsx`), mirroring the instance-delta lane's own residency pattern:

- `splitJsonArrayElements(json)` — splits a JSON array's TEXT into element texts with a balanced scan
  (strings, escapes, nesting), or `null` for anything it cannot account for.
- `advanceWorldMeshResidency(previous, meshesJson)` — decides mesh identity **on the wire text**, so a
  refresh that republishes an unchanged mesh hands back the SAME record object. A payload it cannot
  split falls back to the whole-document parse, which is always correct and merely rebuilds everything.
- `buildMeshVisuals(record)` / `disposeMeshVisuals(visuals)` — one mesh id's geometry, border,
  vertex-pick and edge buffers, built and released as one.
- `WorldInstancesLayer` keeps a per-mesh-id cache keyed on the record's identity, builds only what
  changed, and retires the rest: **dispose-on-remove runs in an effect after the commit that stopped
  referencing them** (a geometry retired during render is still in the mounted scene until React
  commits the new one), plus a dispose-all on unmount.

**Measured**: `mesh.decode` is 23 × **0 ms** before and after — at 1–3 meshes per surface this is below
the noise floor, exactly as the audit predicted ("low measured urgency today"). The win is the
allocation count and the leak, not wall time. `meshesLen` for the largest example (Sphere Cut With
Torus, 88 230 chars) still decodes in ≤ 2 ms.

---

## 6. Audit item 6 — `RecalcStyleDuration` on the current tree, re-verified and gated

Measured over four independent full-example runs on the current tree, as a CDP `Performance.getMetrics`
delta per step:

| run | wall | RecalcStyleDuration | share of wall |
|---|---:|---:|---:|
| `react-hop/before` | 79.3 s | 2 264 ms | **2.86 %** |
| `react-hop/split` | 106.5 s | 2 541 ms | **2.39 %** |
| `react-hop/after3` | 72.1 s | 2 029 ms | **2.81 %** |
| `react-hop/after` | 75.0 s | 2 143 ms | **2.86 %** |

`LayoutDuration` is 9–77 ms per step. **The 2026-09-12 fix holds**: nothing resembling the 112 s-of-129 s
style-recalc storm (`project-root-css-animations-recalc-style-storm`) is present. The probe prints the
gate line itself on every run — `RecalcStyleDuration over the run: N ms = X% of wall (gate: under 5%)` —
so the next lane to touch styling gets the regression signal for free.

---

## 7. Audit item 5 — React re-render breadth: partially answered, not claimed

What the instrument now says: `refresh.apply` (every React state dispatch a refresh performs) is
**103 × 0 ms**, and `commit` (dispatch → the next frame the compositor runs, which contains React's own
reconcile and commit plus layout and paint) is **103 × 51 ms**, ~4 % of a hop. So the shell's React work
is not a cost centre at this scale, and the identity-preserving merge documented in `runUiRefreshPass`
is doing its job.

What is **not** answered: which components re-render. No React Profiler capture was taken and no
component was named. The audit's hypothesis — a wide context whose identity changes on every dispatch —
is neither confirmed nor refuted here; the measurement above only bounds the total. **Not claimed.**

---

## 8. Files changed

| file | change |
|---|---|
| `🧰️framework/🔨️modules/⏱️trace/🟦️.ts` | **new** — the browser hop tracer, stage vocabulary, breakdown helpers |
| `🧰️framework/🔨️modules/⏱️trace/🧫️fixtures/🪃️hop-stages/🔣️.json` | **new** — the language-agnostic stage declaration |
| `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` | `windowLayoutWindowIdsV1`, `partitionRefreshWindowInstancesV1` |
| `🧰️framework/…/🧱️elements/🔌️PluginRuntime/🟦️.tsx` | `encode`/`channel`/`decode`/`invoke` spans in `performInvocation`; `refresh.turn`/`refresh.project` spans in `refreshUi` |
| `🧰️framework/…/🧱️elements/🏛️ShellHost/🟦️.tsx` | `refresh`/`refresh.guest`/`refresh.slots`/`refresh.apply`/`commit` spans; mounted-window fetch + skipped-cache drop + remount re-fetch effect |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🟦️.tsx` | `arm` span in `scheduleDispatchAction` |
| `🧰️framework/…/🧱️elements/🛠️ShellHelpers/🧫️fixtures/🪟️mounted-window-fetch.json` | **new** — the mounted-window fetch declaration |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` | mesh residency (`splitJsonArrayElements`, `advanceWorldMeshResidency`), `buildMeshVisuals`/`disposeMeshVisuals`, per-mesh-id cache with dispose-on-remove, `mesh.decode` span |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🧫️fixtures/🧊️mesh-residency.json` | **new** |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/⏱️hop-trace/🟦️.ts` | **new law** |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🧊️world3d-mesh-residency/🟦️.ts` | **new law** |
| `🧰️framework/…/🧑‍🎨engine/🧪️tests/🪟️mounted-window-fetch/🟦️.ts` | **new law** |
| `🧰️framework/…/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` | registers the three new suites |
| `<ticket>/🐍️react-hop-cost-probe.mjs` | **new** — the per-hop cost probe |

---

## 9. Laws, with output

All through `cd 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript && bun ./📜️script.ts test long <filter>`.

```
$ … test long "hop-trace"
 Test Files  1 passed (1)
      Tests  9 passed (9)

$ … test long "world3d-mesh-residency"
 Test Files  1 passed (1)
      Tests  17 passed (17)

$ … test long "mounted-window-fetch"
 Test Files  1 passed (1)
      Tests  9 passed (9)
```

Whole React engine corpus, after every change:

```
$ … test long
 Test Files  2 failed | 42 passed (44)
      Tests  5 failed | 1147 passed (1152)
```

The five reds are **pre-existing and peer-owned**, none in a file or function this lane touched (the
same five, minus the +9 this lane added to the green count, were red on the corpus run taken before the
kernel move):

1. `🎟️resident-refresh-budget > carries the measured six-surface census` — `fixture.laws.length` is 5,
   the assertion expects 4: a peer added a law to the JSON fixture.
2. `🔬️engine-contract > per-window element ids > styles the projection pane body…` —
   `TypeError: undefined is not an object (evaluating 'CSS.escape')`, a jsdom environment gap.
3. `🔬️engine-contract > window-kind action scoping > scopes each generation window's own actions…` —
   expects `translateSelection:procedural-preview`, gets
   `translateSelection:generation3d-generate-preview,procedural-preview`; this is the open
   "viewer mints `translateSelection`" item in `📓️status.md` (09:15 entry).
4–5. `🔬️engine-contract > shell option locks` ×2 — pure-function assertions on
   `leftoverOverlayCarryingSelectionV1` / `mergeWorldSelectionWithLeftoverV1`, a peer's live
   leftover-selection change.

Runtime, on 6021:

```
$ SEMIO_PROBE_URL=http://127.0.0.1:6021/?plugin=generation3d bun 🐍️journey-probe.mjs
DONE steps 23 meshSteps 20      # 23/23 converged, every mesh oracle green
```

---

## 10. Not claimed

- **The target is not met.** Asked for: every example converging in ≤ half its current time and hop cost
  well under 300 ms. Delivered: 79.3 s → 72.1–75.0 s over ten steps (5–9 %), and a hop still costs
  ~1 400 ms. §4 names the remaining cut, its mechanism, its expected size (~420 ms of ~1 400 ms) and the
  one piece of machinery it needs.
- The `DuplicateSiblingKey` fault (§0) was neither fixed nor investigated by this lane; residual lines
  are still on the console.
- The split of `channel` between message-passing and guest compute is **unmeasured** — the worker
  `Performance` domain returned nothing, and instrumenting the shard worker means rebuilding
  `🟨️shard-worker.js`, which this host-only lane did not do. Every `worker ms` column in the probe's
  tables reads `0` for that reason, not because the worker is idle.
- No React Profiler capture was taken; no needlessly re-rendering component is named (§7).
- The mounted-window narrowing's measured effect (3–8 % on the refresh pass) is **within run-to-run
  noise on the per-hop figure**; only the total-wall and boot numbers moved consistently. It is kept on
  correctness grounds, not on a performance claim.
- `mesh.decode` measured 0 ms before and after: the per-mesh memoization is an allocation and
  dispose-leak fix, and no wall-time win for it is claimed at today's 1–3 meshes per surface.
- Nothing about the wgpu door was measured or changed.
