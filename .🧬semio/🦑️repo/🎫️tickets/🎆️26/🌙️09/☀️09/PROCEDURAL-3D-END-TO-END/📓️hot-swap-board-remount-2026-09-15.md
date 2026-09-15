# ♻️ Hot-swap board remount, the boot selection that erased itself, and a convergence budget that measures the product

Lane `hot-swap-board-remount`, 2026-09-15, React door of 🧊️generation3d on **:6023** (the lane's own port,
recycled at 17:55 before every verdict).

---

## 0. What this lane owns, and what it landed

| # | item | state |
|---|---|---|
| 1 | a dev hot swap leaves the Flow window without its board | **root-caused, fixed at three sites, 1 law (8 assertions) + a Playwright proof — §1, §5.1** |
| 1b | the two `no actor for instance N` errors every hot swap left behind | **fixed, the typed retirement drop the sibling lanes already had — §1.4** |
| 2 | a boot-time `interactionSelect` lands before the node-graph host mounts and inverts the selection | **root-caused (it is the React-Flow fallback board, not the wasm one), fixed, 5 new laws — §2** |
| 3 | `Hexagonal Mushroom Column · select` red | **root-caused, fixed, 3 new laws; 55/58 → 58/58 — §3** |
| 4 | the journey `converged` predicate is load-flaky (13 → 21 → 13 of 23) | **replaced by a poll-counted budget + a law over 9 recorded sequences; 25/25 in 116 s — §4** |

**Gate GREEN.** `SEMIO_BATTERY_URL=http://127.0.0.1:6023/?plugin=generation3d SEMIO_BATTERY_ROOT=react-hotswap
bun 🐍️react-battery.mjs --only=journey,interact,flow-window`:

```
[DEBUG] battery ■ journey     ok=true exit=0 132s steps=25/25 pageerrors=0
[DEBUG] battery ■ interact    ok=true exit=0 136s steps=58/58 pageerrors=0
[DEBUG] battery ■ flow-window ok=true exit=0  75s steps=3/3   pageerrors=0
[DEBUG] BATTERY DONE green=3/3 red=[] 343s pageerrors=0
```

Evidence `🗑️generated/react-hotswap/scoreboard.json` (19:12), `🗑️generated/react-hotswap-battery.txt`.

---

## 1. Item 1 — the Flow window comes back over a corpse, and nothing ever re-attaches it

### 1.1 How a hot swap was reproduced without a five-minute restage

`semioPluginHotSwapVitePlugin`'s `♻️hot-swap.json` watcher is **not registered in the dev vite config** — the
live availability source is `semioActivationVitePlugin`, which watches the ACTIVATION RECEIPT
(`🧑‍💻dev/📦️packages/🟦️typescript/dist/runtime/react/dev/generation3d/activation/🔣️receipt.json`) and sends one
`built` event per plugin whose `artifactSha256` changed. Writing the hot-swap marker therefore does nothing at
all; verified by subscribing to `/🔌️plugin-modules/watch` with `curl` and writing the marker twice (0 events)
versus flipping a receipt digest (1 `built` event, immediately).

Two halves are needed for a page to actually swap, and this is a fact worth keeping:

- the **digest** is what makes the server SEND (`prior.get(pluginId) !== row.artifactSha256`),
- the **`rebuiltAt`** is what makes the shell ROUTE it to a hot swap — `routePluginAvailability` answers `drop`
  for an event that is not newer than the load it already has.

`🐍️hot-swap-remount-probe.mjs` moves both, in a read-modify-write of whatever a peer's restage has left on
disk, and flips the digest back on the next swap (the flip is its own inverse), so an even number of swaps
leaves the shared receipt's digests exactly as it found them.

**Also measured:** a page that first loaded a plugin unbusted (`loadedRebuiltAt: undefined`) hot-swaps ONCE on
its own, ~20 s in, from the availability stream's connect-time snapshot — the fixture's
`busted-build-over-an-unbusted-first-load-hot-swaps` row. So every generation3d page already takes one hot swap
per boot, which is why the peers' 120 s idle windows kept landing in it
(`📓️flow-surface-followup-2026-09-15.md` §6.3).

### 1.2 The defect

`reloadPlugin` (`🏛️ShellHost/🟦️.tsx`) destroys the session-owning plugin's live instance, swaps the module, and
only then re-establishes the session. The shell state kept naming the DESTROYED instance for that whole
stretch, and the canvas was announced live at the commit point — before `establishPrimarySession`:

```ts
// before
dispatch({ type: "UPSERT_LOADED_PLUGIN", value: { handle: newHandle, manifest: newHandle.manifest } });
dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });      // ← canvas live again…
dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
committed = true;
if (ownsSession) await establishPrimarySession(newHandle);               // …session still names instance 1
```

`resolvePluginCanvasStatus(!!session, …)` reads exactly that status, so the windows re-opened over the corpse.
Measured on :6023 before the change (`🗑️generated/hot-swap-all/console.txt`):

```
29414 node-graph host unmount surface=window:procedural-main
29834 hot-swap procedural {pluginId: procedural, version: 0.1.0, …}
30930 [DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-math, … no actor for instance 1}
30930 [DEBUG] invokeExtension dispatch failed {extensionId: flow-extension-brep, … no actor for instance 1}
31705 dropped typed-operation completion subscription for retired instance procedural#1
31705 dropped typed-operation progress   subscription for retired instance procedural#1
31749 node-graph host mount surface=window:procedural-main          ← the board, over instance 1
31758 dropped action interactionSelect          for retired instance procedural#1
31758 dropped local interaction observation     for retired instance procedural#1
31975 node-graph surface ready … 7 6                                 ← painted from the STALE retained tree
33133 command ingress lane {"instanceId":2, …}                        ← the successor arrives 1.4 s later
```

**And there is no second mount.** The node-graph surface effect is keyed on `[surfaceId]`, and
`window:procedural-main` is the same string on both instances — that keying is itself a law
(`🧪️tests/🧲️engine-surface-retention`), so it is not negotiable. The successor session therefore never produces
a remount, and the board stays bound to the instance it mounted against. On a slower page that is a **blank
window**, because the opening-camera dispatch inside `attachCanvas().then(...)` throws into the attach's own
catch and `surfaceReadyRef` never becomes true — measured verbatim in a peer's console
(`🗑️generated/flow-inline/journey2/console.txt`, +306.6 s):

```
306606 node-graph host mount surface=window:procedural-main
306680 node-graph attach called surface=window:procedural-main 966 807 1
306681 node-graph fit on open surface=window:procedural-main {"x":530,"y":-46,"zoom":0.93}
306681 node-graph attach failed  surface=window:procedural-main  … no channel for instance 1
308412 pageerror                 Error: … no channel for instance 1
```

The gap between the unmount and the wasted remount is what the peers reported as "the Flow window has no
node-graph host at all": **9.5 s** (`react-sweep2/panel-i18n`), **85.4 s** (`host-refresh/served-journey`),
**82.3 s** (`flow-inline/journey2`).

### 1.3 The fix — an ordering, not a retry

Three statements in `reloadPlugin`:

```ts
// after
if (ownsSession && activeSession) {
  dispatch({ type: "SET_SESSION", value: null });                      // 1. stop naming it while it is alive
  await current.handle.destroyApp(activeSession.instanceId).catch(() => {});
}
…
dispatch({ type: "UPSERT_LOADED_PLUGIN", … });
committed = true;
if (ownsSession) await establishPrimarySession(newHandle);             // 2. successor FIRST
dispatch({ type: "SET_PLUGIN_STATUS", pluginId, value: "loaded" });    // 3. live canvas AFTER it
dispatch({ type: "SET_PLUGIN_SUPERVISOR", pluginId, value: ownsSession ? "running" : "loaded" });
```

With no session and the status still `reloading`, `resolvePluginCanvasStatus` keeps the canvas in its reload
state, so no window can mount at all until the successor exists. The post-commit catch branch now announces
`"loaded"` too, so a predecessor that refuses to retire cannot leave the plugin reading `reloading` forever.

### 1.4 The last two errors a hot swap left behind

`invokeExtension`'s rejection path dropped quietly for a SEALED instance only. A hot swap retires rather than
seals, and an extension round trip outlives the swap by construction, so the guest's in-flight `evaluate` calls
landed as two `no actor for instance 1` errors on every single swap. They now go through the same typed
retirement (`isPluginInstanceRetiredV1`) its two sibling subscriptions and the local-interaction read already
used, and read as one `[DEBUG] dropped extension answer …/evaluate for retired instance procedural#1`.

### 1.5 After

`🗑️generated/hot-swap-final/` — two swaps on one page:

```
 3952 node-graph host mount    surface=window:procedural-main
 4343 node-graph surface ready surface=window:procedural-main 7 6
17350 node-graph host unmount  surface=window:procedural-main
17449 hot-swap procedural {…}
20928 node-graph host mount    surface=window:procedural-main          ← 3.6 s
21127 node-graph surface ready surface=window:procedural-main 7 6
53873 node-graph host unmount  surface=window:procedural-main
54034 hot-swap procedural {…}
58605 node-graph host mount    surface=window:procedural-main          ← 4.6 s
58792 node-graph surface ready surface=window:procedural-main 7 6
```

| reading | before (`hot-swap-all`, 18:0x) | after (`hot-swap-final`, 19:15) |
|---|---|---|
| board back with its 7 nodes after a swap | **no** — the mount lands on the retired instance | **yes**, twice, 3.6 s and 4.6 s |
| `node-graph host mount` count over 2 swaps | 2 (one of them wasted) | **3** — one per presentation, each over a live instance |
| `no channel for instance N` | present on peers' pages, 1 page error | **0** |
| `no actor for instance N` | **2 per swap** | **0** |
| dispatches refused for the retired instance at mount | **4** | **0** |
| page errors | 0 here, 1–2 on peers' pages | **0** |

---

## 2. Item 2 — the boot-time `interactionSelect` is the FALLBACK board publishing an emptiness it never had

The peers recorded an `interactionSelect` at t = 2 538 ms, fourteen milliseconds before `node-graph host
mount`, with no pointer event and **no `[DEBUG] flow interaction publish` line anywhere** — i.e. not the wasm
board. A temporary ingress log plus a console hook that prints the JS stack of every such dispatch
(`🐍️boot-selection-recon.mjs`, `🐍️world-pick-recon.mjs`) named it on the first run:

```
7402 [DEBUG] interactionSelect ingress {"instanceId":1,"actionId":"interactionSelect",
        "args":{"surfaceId":"window:procedural-main","domainId":"graph","targets":"[]","merge":"replace", …}}
  at 🕸️NodeGraph/🟦️.tsx:805  ← dispatch
  at Object.onSelectionChange [as current] (🕸️NodeGraph/🟦️.tsx:914)
7417 [DEBUG] node-graph host mount surface=window:procedural-main
```

It is `DiagramGraphFallback` — the React-Flow board the Flow window shows before the wasm board takes over.
**React Flow fires `onSelectionChange` once at mount, with an empty list**, and the handler dispatched it
unconditionally. So the surface published a selection it had never formed, as a `replace`, and any selection
the plugin had restored a few milliseconds earlier was erased by a board that had just appeared.

The fix is the ledger this file already owns, used the way its own docstring describes — *"marks that arrive
FROM the plugin are what the plugin already holds, so they set the baseline without owing a hop"* — plus the
other half, which was missing: the mounted nodes must CARRY that selection, or the mount-time report is empty
however good the baseline is.

```tsx
const sceneSelection = useMemo(() => [...(scene.selection ?? [])], [scene.selection]);
const interactionLedger = useMemo(() => createNodeGraphInteractionLedger(), []);
if (adoptedSelectionRef.current !== sceneSelectionKey) { …; interactionLedger.adoptSelection({ nodeIds: sceneSelection }); }
const initialNodes = useMemo(() => workflowNodesToDiagramNodes(parsedNodes, sceneSelection), [parsedNodes, sceneSelection]);
…
onSelectionChange={(selection) => {
  const nodeIds = selection.nodes.map((entry) => entry.id);
  if (interactionLedger.publishSelection({ nodeIds })) dispatch(nodeGraphActions.select, …);
}}
```

A selection dispatched before the board existed is therefore APPLIED at mount (the nodes mount selected) and
owes no hop; a real pick after mount still publishes, and a real deselect after that still publishes.

**After:** zero `interactionSelect` ingress lines in the whole 90 s two-swap console, boot included — where
before there was one per boot and one per board remount. And the survival is visible in the product: the
`interact` row now reads `selectedIdsBefore: ["extrude@solid"]` on the boot example, i.e. the plugin's restored
selection reaches the user's first click instead of being wiped before it.

---

## 3. Item 3 — `Hexagonal Mushroom Column · select`: hover and select disagreed about the same point

`🐍️world-pick-recon.mjs` reads both halves at one point. Sweeping the pointer resolves a target; clicking that
very point ten milliseconds later resolves nothing:

```
[DEBUG] instancesAtHit=[{"id":"profile@wire#0",…},{"id":"extrusion-axis@vectorOut#0",…},{"id":"extrude@solid#0",…}]
[DEBUG] hit={"point":[1264,330],"hovered":"extrude@solid"} afterClick=[] active=null
```

The dispatch stack says where the click went:

```
[DEBUG] interactionSelect ingress {"args":{"surfaceId":"window:procedural-preview","targets":"[]","merge":"replace", …}}
  at 🌐️World3dHost/🟦️.tsx (dispatch) … at onPointerMissed (react-three-fiber)
```

Hover rides R3F's `onPointerMove` against the real geometry (`handleInstancePointerMove` →
`dispatchInstanceHover`); "did this click hit anything" is R3F's own `onPointerMissed` bookkeeping. They
disagreed, so every click on that pane went to the BACKGROUND handler, which dispatches
`interactionSelect targets:[]`. That is why the object could be hovered and never selected — and why, once
§2 let the boot selection survive, the same click ERASED it, which is exactly the "toggles the object OFF"
the peers saw.

**The rule the background path now obeys: a pane never clears at a point where it is publishing a hover
target.** The hover is resolved by the same pane at the same point, so a click there is a pick of it; empty
canvas has no hover and still clears; marker layers (a vortex `kind:fullId`, a `reference:id`) are excluded
because they own dedicated pick handlers.

One refinement was needed, and it is the difference between a green run and a flaky one. Reading
`selection.hoveredId` — the GUEST's echo — races: it lags a round trip and any unrelated refresh can clear it,
and a run where it had just been cleared cleared the selection anyway (`react-hotswap-interact`, 18:53:
`selectedIdsBefore: ["extrude@solid"] → selectedIds: []`). The pane now keeps what its OWN raycast last
resolved in `hoveredInstanceInteractionIdRef`, set by `handleInstancePointerMove` and cleared by the mesh's
`onPointerOut`, and prefers it.

| row | before (`react-sweep-closing` 13:05, and 18:35 here) | after (19:12) |
|---|---|---|
| `Hexagonal Mushroom Column · select` | red, `selectedIds: []` | **green**, `["extrude@solid"]`, `activeObjectId` set |
| `Hexagonal Mushroom Column · inspector` | red, `idRow: null` | **green**, `Id: extrude` |
| `Rectangle Wire Preview · select` / `· inspector` | red since 13:05 | **green** |
| `interact` total | 54–55/58 | **58/58** |

---

## 4. Item 4 — a convergence budget that measures the product, not the box

### 4.1 Two separate causes, and only one of them was load

The journey's `converged` predicate polled once a second inside a FIXED `seconds` budget, and the same green
build converged 13 → 21 → 13 of 23 rows across three runs with every mesh oracle green in all three. Replacing
the budget exposed the second cause immediately: with the new predicate the first run reported
**`stalled`, `no published movement for 20 polls`, `meshOk: true`** on nine rows in 322 s instead of burning 45
minutes, and the recorded samples name the term:

```
45660 converged=False oracleOk=True  previews=[{phase:"idle", ratio:1, computing:null, meshes:3, triangles:20}]  widgets=0
```

The preview was settled and the geometry was exactly right. `widgets=0` is the whole failure: the row's graph
oracle read `data-fixture-json`, which a peers' sweep renamed to `data-host-snapshot-json`, and
`scene.hostSnapshotJson` is absent on this stage anyway — so **both attribute lanes answer `null` and the
oracle could never be satisfied**. It is the same renamed lane that took `flow-reorganize` down in
`📓️flow-surface-followup-2026-09-15.md` §6.3. The row now reads the graph the surface is actually PAINTING,
from `window.__semioFlowGraphProbe[surfaceId].nodeIds()`, with both attributes as fallbacks.

### 4.2 The budget

`🐍️convergence-budget.mjs` is a pure module the probe and the law share:

- **The budget is counted in POLLS, not seconds.** A busy machine stretches every poll, so a poll-counted
  budget stretches with it automatically. The caller's old `seconds` survives only as the poll CEILING.
- **A poll only counts against the step once the preview has stopped moving.** `convergenceSignatureV1` is the
  published status of every preview (phase, ratio, `computing`, fault, mesh count, triangles) plus the picker
  label, the painted graph, the node statuses and the oracle's own verdict — no wall clock, no internal
  counter. Any change is movement.
- **A stall is still a stall.** `stallPolls = 20` consecutive polls with no movement is red, whatever the load,
  and the verdict NAMES it (`no published movement for 20 polls`) instead of reporting a bare timeout.
- `convergenceLoadFactorV1` reads the median poll interval, so the load is measured rather than assumed.

### 4.3 The law, over recorded sequences

`🧪️convergence-budget-law.mjs` replays `🧫️convergence-sequences.json` — **9 real per-poll recordings** from the
18:39 journey run (`recordedFrom` names the file and the run) — and derives two labelled families from them:
each sequence stretched by a load factor, and each sequence cut before convergence and then left publishing its
last reading, which is literally what a stalled preview does.

```
✓ recorded "boot" → converged  … (9 sequences)
✓ every converging sequence still converges at 2× / 4× / 8× poll latency
✓ the load factor is READ, not assumed, at 2× / 4× / 8×
✓ "<row>" frozen after N polls is STALLED                              (9 rows)
✓ "<row>" stall is reported before the poll ceiling                    (9 rows)
✓ "<row>" stays STALLED at 6× poll latency — load never rescues a dead preview   (9 rows)
✓ a pause SHORTER than the stall budget is not yet a stall :: "running"
[DEBUG] convergence budget law GREEN (9 recorded sequences)
```

### 4.4 Before / after

| reading | before | after |
|---|---|---|
| journey rows converged | 13 → 21 → 13 of 23, mesh oracles green in all three | **25/25**, twice (116 s, 132 s) |
| journey wall time | 45 min budget, routinely spent | **116–149 s** |
| a red row's reason | "did not converge in 60 s" | `stalled — no published movement for 20 polls`, with the sample that froze |

---

## 5. Laws, with output

### 5.1 `♻️ hot-swap session handover` — new suite (8 assertions)

`🧑‍🎨engine/🧪️tests/♻️hot-swap-session-handover/🟦️.ts` + `🧫️fixtures/♻️hot-swap-session-handover/🔣️.json`.
A reference mirror replays each authored step plan and counts the two things that decide whether a board comes
back; the measured defect is authored as its own case so the law is not vacuous, and two source scans pin the
ordering in `reloadPlugin` and the `[surfaceId]` keying that makes the ordering the fix.

```
$ bun test "./🧪️tests/♻️hot-swap-session-handover/🟦️.ts"
[DEBUG] hot-swap handover retire-session → destroy-instance → swap-module → establish-session → present-canvas :: mounts=1 overRetired=0 endsOnInstance=2
[DEBUG] hot-swap handover destroy-instance → swap-module → present-canvas → establish-session                  :: mounts=1 overRetired=1 endsOnInstance=2
[DEBUG] hot-swap handover retire-session → destroy-instance → swap-module → present-canvas → establish-session :: mounts=1 overRetired=0 endsOnInstance=2
[DEBUG] hot-swap handover swap-module                                                                          :: mounts=0 overRetired=0 endsOnInstance=1
 8 pass  0 fail  18 expect() calls
```

### 5.2 `🫱️ flow node-graph boot selection` — 5 new laws in the existing publication suite

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts 🫱️interaction-publication
 ✓ flow node-graph boot selection > mounts the board carrying the selection the plugin already holds
 ✓ flow node-graph boot selection > owes the plugin NOTHING for the mount-time report of a selection the plugin itself made
 ✓ flow node-graph boot selection > counter-proof: a board that does not reflect the adopted selection publishes the empty one that erased it
 ✓ flow node-graph boot selection > still publishes a real user pick made after mount, and a real deselect after that
 ✓ flow node-graph boot selection > re-adopts a selection the plugin changes underneath the board without owing a hop
[DEBUG] boot selection: adopted=["column-preview"] mountReport=["column-preview"] owed=0
 Test Files  1 passed (1)   Tests  18 passed (18)
```

### 5.3 `🎯️ a background click never clears at a point the pane is hovering` — 3 new laws

```
$ SEMIO_TEST_LEVEL=long bunx vitest run --config …/⚛️react/🧪️tests/🎚️config/🟦️.ts world3d-pick-bounds
 Test Files  1 passed (1)   Tests  13 passed (13)
```

### 5.4 `⚖️ convergence budget` — §4.3, 45 checks over 9 recorded sequences

```
$ bun 🧪️convergence-budget-law.mjs   →   [DEBUG] convergence budget law GREEN (9 recorded sequences)
```

### 5.5 Playwright proof

```
$ SEMIO_PROBE_URL=http://127.0.0.1:6023/?plugin=generation3d SEMIO_PROBE_OUT=hot-swap-final \
  SEMIO_PROBE_SWAPS=2 bun 🐍️hot-swap-remount-probe.mjs
[DEBUG] 1-boot:          host=true probe=true nodes=7 mounts=1 swapSeen=false in 2s
[DEBUG] 2-after-swap-1:  host=true probe=true nodes=7 mounts=2 swapSeen=true  in 3s
[DEBUG] 3-after-swap-2:  host=true probe=true nodes=7 mounts=3 swapSeen=true  in 4s
[DEBUG] VERDICT {"recovered":true,"pageErrors":0}
EXIT=0
```

### 5.6 Neighbouring suites re-run after the change

`tool-run-trace` 10/10, `world3d-pick-bounds` 13/13, `🫱️interaction-publication` 18/18, `♻️hot-swap-session-handover` 8/8.
`🌐️World3dHost` component suites 7/8 under concurrent battery load (one 5 s vitest timeout in
`⏯️tool-run-trace`, a file this lane never opened) and **10/10 when re-run unloaded** — see §6.

---

## 6. Not claimed

- **The R3F `onPointerMissed` disagreement is FIXED at the pane's decision, not explained at its origin.**
  The pane's hover raycast resolves `extrude@solid` and R3F reports the click at the same point as a miss; the
  mesh is raycastable (hover proves it), carries an `onClick`, and `pickEnabled`/`instancePickEnabled` are both
  true (`activeUtility: "select"`, no marquee hold, no provisional set — `ToolRunProvisionalIdsContext` has no
  production provider at all, so `useToolRunProvisional` is always false). Why R3F counted zero intersections
  for the click is **not** established. The rule this lane landed — never clear where you are hovering — is a
  consistency rule that is correct on its own terms, and it is what makes the row green; it is not a fix to
  whatever makes the two hit tests disagree.
- **`ToolRunProvisionalIdsContext` has no `.Provider` anywhere in the tree.** Its default empty set means
  `provisional` picking exclusion is dead code on every renderer. Found while tracing §3; not this lane's, not
  fixed.
- **8 reds in `🔬️engine-contract` are pre-existing or peer-owned, and none is this lane's.** 619/627 pass. The
  two that touch files this lane edited are demonstrably not this lane's: `FlowGraphCanvasHost` slider dispatch
  belongs to a peer's uncommitted slider-readout work in the same file (`dagSliderValueText`,
  `fontScreenPx`/`gapScreenPx`, a readout `<span>` in `GraphSliderOverlays` — all in `git diff`, none of it
  written here), and `worldSurfaceSelectionDomV1` returning 13 keys against an 11-key fixture is a mismatch
  between the COMMITTED product and the COMMITTED fixture (`git diff` on that function: empty). The other six
  (gis dialect projection, tutorial track schema ref, two ink-canvas host renders, per-window element ids,
  window-kind action scoping) touch nothing this lane opened.
- **`Sphere Box Fuse · hover/select/inspector` went red in one of the three battery runs** (35 swept points, no
  mesh, `offered` naming ids the pane never painted) and green in the other two. That is the known world3d
  delivery flake named in `📓️flow-surface-followup-2026-09-15.md` §7; nothing here touches delivery, and the
  gate run is the one with it green.
- **Three of the reds this lane turned green were turned green by fixing the READING, not the product**, and
  they are marked as such: the journey's graph oracle read a renamed DOM attribute (§4.1); the `interact`
  probe opened the Inspection panel with a click whose failure it swallowed and a fixed 1.5 s wait, so the
  FIRST example was measured against a panel that was never opened (`rows: []` twice, all seven later examples
  green); and its `inspector` step read the panel once instead of polling it. Those three are probe fixes. The
  `select` rows (§3) and everything in §1 and §2 are product fixes.
- **The wasm board's own `emitInteractionState` was not changed.** It already goes through the ledger
  (`📓️flow-surface-followup-2026-09-15.md` §8) and it never produced the boot dispatch — the whole point of §2
  is that the fallback board did.
- **The hot swap is still ~3–5 s of empty Flow window**, because that is what re-creating the instance costs
  (`createApp` plus the guest's own boot). This lane makes the window come back, deterministically and over the
  right instance; it does not make the successor boot faster.
- **A hot-swap event that arrives while another is in flight for the same plugin is still dropped**
  (`pluginOpInFlightRef` early-return in `reloadPlugin`). Measured — a second receipt republish 20 s after the
  first produced no swap at all — named here, not fixed: it needs a queue, and a queue is a different lane's
  contract.
- **The shared activation receipt was perturbed and restored, and peers' open pages paid for it.** The probe's
  swap trigger rewrites `…/activation/🔣️receipt.json`, which every generation3d serve watches, so each write is
  a `built` event in every open page. An even number of swaps leaves the digests as found; the exceptions are
  named here rather than hidden: one SSE verification wrote a single unpaired flip (~17:50), one `SWAP_ALL` run
  left the ten non-procedural digests rotated by two (~18:05) — both wiped by a peer's real restage that landed
  after them — and a repair attempt at 19:30 rotated all eleven rows on a mistaken reconstruction and was
  reverted byte-for-byte in the same minute (procedural `85e418…`, flow `7f566e…`, the 19:15 values). The file
  now holds exactly what the last real restage wrote; the cost was a handful of extra hot swaps in peers' open
  pages, which is the very event this lane makes recoverable. `rebuiltAt` is monotonically advanced by design —
  a restage does the same — and is not restored.
- **Only macOS was exercised**, and only the React target. The wgpu door shares `reloadPlugin`'s contract
  through the same fixture but was not run (wgpu is on hold for this lane).
- **The battery's `interact` row was run five times** across the day (55/58, 57/58, 57/58, 58/58, 58/58); the
  two 57/58 runs are the Inspection-panel reading fixed between them, and the 55/58 is the `Sphere Box Fuse`
  flake. No run after the final probe change has been red for a reason attributable to this lane.

---

## 7. Files

Product:

| file | change |
|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` | `reloadPlugin` retires the session before destroying its instance and announces the live canvas only after `establishPrimarySession`; the post-commit catch restores `"loaded"`; `invokeExtension` drops a typed retirement instead of logging `no actor for instance N` |
| `🧰️framework/…/🧱️elements/🕸️NodeGraph/🟦️.tsx` | `DiagramGraphFallback` adopts the plugin's selection into a `createNodeGraphInteractionLedger` baseline and reflects it onto the mounted nodes; both its publication sites go through the ledger; `workflowNodesToDiagramNodes` takes the selection and is exported |
| `🧰️framework/…/🧱️elements/🌐️World3dHost/🟦️.tsx` | `hoveredInteractionTargetV1` (new, exported); `handleEmptyClick` picks the hovered target instead of clearing; `hoveredInstanceInteractionIdRef` keeps the pane's own raycast answer so the decision does not race the guest's echo |

Laws:

- `🧰️framework/…/🧑‍🎨engine/🧪️tests/♻️hot-swap-session-handover/🟦️.ts` (**new**, 8) + `🧫️fixtures/♻️hot-swap-session-handover/🔣️.json` (**new**)
- `🧰️framework/…/🧱️elements/🕸️NodeGraph/🧪️tests/🫱️interaction-publication/🟦️.ts` (**5 new**)
- `🧰️framework/…/🧑‍🎨engine/🧪️tests/🎯️world3d-pick-bounds/🟦️.ts` (**3 new**)

Ticket:

- `🐍️hot-swap-remount-probe.mjs` (**new**) — the Playwright proof and the receipt-driven hot-swap trigger
- `🐍️convergence-budget.mjs` (**new**), `🧪️convergence-budget-law.mjs` (**new**), `🧫️convergence-sequences.json` (**new**, 9 recorded sequences)
- `🐍️boot-selection-recon.mjs` (**new**), `🐍️world-pick-recon.mjs` (**new**) — the two stack-hook recons that named §2 and §3
- `🐍️journey-probe.mjs` — poll-counted budget, recorded samples per row, graph oracle reads the painted node ids
- `🐍️interaction-matrix-probe.mjs` — the Inspection panel is opened by its result and the inspector step polls
- `📜️run-hotswap-battery.sh` (**new**)
- evidence: `🗑️generated/hot-swap-{before,all,after,final}/`, `🗑️generated/select-repro/{before,after}/`,
  `🗑️generated/world-pick{,-stack,-flags,-toolrun}/`, `🗑️generated/boot-selection/`,
  `🗑️generated/react-hotswap/`, `🗑️generated/react-hotswap-interact/`, `🗑️generated/react-hotswap-battery.txt`
