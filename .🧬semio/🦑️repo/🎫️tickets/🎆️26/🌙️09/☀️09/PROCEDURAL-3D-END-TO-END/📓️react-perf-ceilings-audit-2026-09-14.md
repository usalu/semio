# React Renderer — Where generation3d Hits Performance Limits Today (audit, 2026-09-14)

Read-only Sonnet audit lane `react-perf-ceilings-audit`. No file other than this report was written; no
build, browser probe, or git command was run. Every number below is read off files already on disk —
lane reports under this ticket folder, `results.json`/`console.txt` under `🗑️generated/`, and the source
tree. "Latest" React journey = `🗑️generated/react-u64/journey/` (2026-09-14, from
`📓️react-view-state-u64-carrier-2026-09-14.md`, the newest full 23/23 run); `🗑️generated/react-verify/journey/`
(2026-09-13, from `📓️react-end-to-end-verification-2026-09-13.md`) is used only as a delta check — it
predates the u64-carrier and dirty-scope-kernel fixes and its `viewer-role`/`view:*` rows show 0 meshes
where the newer run shows the correct counts, confirming those fixes actually moved this journey.

---

## 1. Per-example table — latest React journey (`🗑️generated/react-u64/journey/`)

Wall seconds and mesh counts are the probe's own published numbers
(`results.json`); "hops" = `performInvocation settled {"actionId":"flowEvalTick", …}` lines inside that
step's time window in `console.txt` (the flow graph's own per-node settle rounds — **not** mesh
round trips, see §2b); "mesh debug len" = `data-status-json.debug.meshesLen`, the serialized
`meshesJson` character length the guest published for that surface (`🐍️journey-probe.mjs:21,30`).

| step | s | hops (flowEvalTick) | meshes | mesh debug len (chars) |
|---|---:|---:|---:|---:|
| boot (Hexagonal Mushroom Column) | 11.3 | 7 | 3 | 3 644 |
| edit: No example | 3.0 | 2 | 0 | 2 |
| edit: Hexagonal Mushroom Column | 4.0 | 3 | 3 | 3 644 |
| edit: Rectangle Extrude Volume | 9.1 | 7 | 1 | 1 089 |
| edit: Sphere Cut With Torus | 8.0 | 7 | 1 | 88 230 |
| edit: Box Fillet Preview | 7.1 | 6 | 1 | 19 231 |
| edit: Sphere Box Fuse | 8.1 | 7 | 1 | 32 860 |
| edit: Face Sweep Extrude | 10.1 | 8 | 1 | 1 089 |
| edit: Rectangle Wire Preview | 4.1 | 3 | 1 | 203 |
| edit: Box Shell Preview | 6.1 | 6 | 1 | 4 339 |
| generate-mode | 3.0 | 0 | 0 | 2 |
| generate-added (Add Generation) | 3.0 | 2 | 1 | 4 339 |
| back-to-edit | 3.1 | 0 | 1 | 4 339 |
| viewer-role | 5.1 | 5 | 1 | 4 339 |
| view: No example | 3.0 | 1 | 0 | 2 |
| view: Hexagonal Mushroom Column | 4.1 | 6 | 3 | 3 644 |
| view: Rectangle Extrude Volume | 4.0 | 7 | 1 | 1 089 |
| view: Sphere Cut With Torus | 4.0 | 6 | 1 | 88 230 |
| view: Box Fillet Preview | 4.0 | 5 | 1 | 19 231 |
| view: Sphere Box Fuse | 4.0 | 6 | 1 | 32 860 |
| view: Face Sweep Extrude | 4.1 | 7 | 1 | 1 089 |
| view: Rectangle Wire Preview | 3.0 | 2 | 1 | 203 |
| view: Box Shell Preview | 4.1 | 5 | 1 | 4 339 |

23/23 converged, 120 s total, 0 page errors, 0 worker faults
(`📓️react-view-state-u64-carrier-2026-09-14.md` §5.2). Wall time does **not** track mesh size — Box
Fillet Preview (19 231 chars) is faster (7.1 s) than Face Sweep Extrude (1 089 chars, 10.1 s); it tracks
hop count almost exactly (Pearson-by-eye: 8/10.1s, 7/9.1s, 7/8.1s, 7/8.0s vs 3/4.0s, 2/3.0s). The edit
lane is 1.5–3× the same example in view mode with identical hop counts in most rows (e.g. Sphere Cut
With Torus: 7 hops / 8.0 s edit vs 6 hops / 4.0 s view) — the extra edit-lane second(s) are the widget
graph settling on FIRST load of that example (contributions install, `setContributions`, first
`toolRunStart`), not per-hop cost; §2 breaks this down.

The `wgpu-edit-convergence-perf` lane's own React column (`📓️wgpu-edit-convergence-perf-2026-09-14.md`
§5, dated the same day, drawn from `📓️react-end-to-end-verification-2026-09-13.md` before the fixes in
this table) reads 3–8 s per edit-lane example — consistent with the numbers above; the wgpu shell is
still 1.35×–3.37× slower per example even after that lane's own fixes, so the React numbers in this
table are the ceiling wgpu is racing against, not a moving target.

---

## 2. Measured cost centers, with the evidence line for each

### a. Guest evaluation (brep kernel) — no longer dominant, but not free

`📓️kernel-performance-2026-09-13.md` rewrote the exact geometric predicates (`orient2d_exact` etc.,
`✏️s/…/🧊️brep/🧬️schema/📸️snapshot/➡️vector/⚖️predicates/🦀️.rs`) from arbitrary-precision rationals to
allocation-free Shewchuk expansion arithmetic after a profile showed **99.3% of a 25 s window** under
`Rational::gcd`/`normalize` inside `stitch_selected_faces`'s ear-clipping (§4.1 of that report). Native
`test`-profile evaluate cost for `sphere-cut-with-torus` dropped **61.5 s → 4.29 s**; the preview-LOD
tessellate step for the same example dropped **5.90 s → 0.11 s** (§1.3, §5). Before that fix,
`📓️extension-evaluate-budget-2026-09-12.md` §1.1 measured `validate_body`'s two per-entity checks
(`check_solid_orientation` 1.55 s, `check_degenerate_geometry` 1.61 s) as the dominant cost inside one
`evaluate` call, and one such call ran long enough (16.3 s of worker silence) to trip the watchdog and
kill the shard — fixed by making `evaluate` budgeted/resumable
(`EVALUATE_STEP_WALL_MICROS: u64 = 2_000_000`,
`✏️s/…/🧵️preview-eval/🦀️.rs:132`). **Residual**: even after both fixes, `sphere-cut-with-torus` still
costs 386 µs (evaluate) + 793 µs (fixture-tolerance tessellate) + 70 µs (preview LOD) natively,
unoptimized (`📓️kernel-performance-2026-09-13.md` §5 "After" table) — three orders of magnitude below
its 8.0 s wall time in §1's table, which is the tell that **evaluation is no longer the ceiling; the
round trip is** (§2b, §2d).

### b. Mesh crossing/decoding — fixed to ≤2 round trips, ≤1 chunk; hop count is now dominated by the flow-graph tick chain, not mesh transfer

`📓️preview-mesh-delivery-2026-09-12.md` found the mesh body was pinned to a 4 KiB *gesture* quota
shared with 24 unrelated routes (cause A) and budgeted in kernel units rather than wall time (cause B),
costing up to 10 round trips / 10 chunks for `sphere-cut-with-torus`. Fix: a dedicated
`GENERATION3D_FLOW_EVAL_TOOL_IDS` route at `GENERATION3D_FLOW_EVAL_RAW_BYTES = 65_536` bytes
(`✏️editor/🦀️.rs:391`; also the viewer's own `GENERATION3D_VIEW_FLOW_EVAL_RAW_BYTES = 65_536`,
`👁️viewer/🦀️.rs:278`) and `MESH_PACK_CHUNK_BASE64_CHARS: usize = 48 * 1024`
(`🌊️flow/📐️brep-geometry/🦀️.rs:974`, up from 4 KiB) — one guest-contiguous page under the 64 KiB
ceiling `dlmalloc` never returns (`project-guest-contiguous-request-ceiling`). Measured after
(§3.4 of that report): every one of the 8 examples now delivers in **≤2 tessellate round trips and
exactly 1 mesh chunk**, `sphere-cut-with-torus` 10→1. The gesture quota (`GENERATION3D_RETAINED_RAW_BYTES
= 8_192`, `✏️editor/🦀️.rs:385`) is untouched and asserted unchanged. **What this means for §1's hop
counts**: with mesh transfer down to 1–2 round trips, the 3–8 `flowEvalTick` hops per example in §1 are
the flow graph's own per-node dependency chain (each upstream node — extrude, fillet, boolean — gets its
own settle round), not mesh chunking. No lane has measured or reduced the per-node hop count itself
(§4 item 3, §5).

On the decode side, geometry construction is `geometryFromMesh`
(`🧰️framework/…/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:1647`): one `Float32Array` copy
each for positions/normals/uvs/colors plus `setIndex`, called once per mesh from a `useMemo` keyed on
the parsed mesh array (§2c) — this is O(mesh size), not O(chunk count), and was not separately profiled
by any lane; §5 names the probe that would.

### c. r3f scene rebuilds — the memo boundary is the WHOLE mesh array, not per-mesh identity

`🧰️framework/…/🌐️World3dHost/🟦️.tsx:2841-2860` (`WorldInstances`, the component every preview surface's
meshes render through): `geometries`, `borderGeometries`, `vertexPickByMeshId`, and `edgeGeometryByMeshId`
are each a `useMemo` keyed on `meshes` — the single array `parseMeshes(scene?.meshesJson ?? "[]")`
returns, itself `useMemo`-cached on the JSON **string** `scene.meshesJson`
(`🟦️.tsx:1139`, `🟦️.tsx:5005`). So the moment ANY mesh in a surface's payload changes byte-for-byte
(e.g. a node upstream of a 3-mesh hexagonal-column re-evaluates one of its 3 solids), the string changes,
`parseMeshes` returns a brand-new array, and all four `useMemo`s rebuild for **every** mesh in that
surface, not just the changed one — reallocating `BufferGeometry`/`EdgesGeometry`/vertex-pick arrays for
meshes whose content did not move. No lane has measured the wasted-rebuild cost directly (the mesh
counts here are small, 1–3 per surface, so it has not been visible against the 3–8 s wall times), but it
is the one r3f-side allocation pattern in this file that does not degrade gracefully as mesh count
grows — see §4 item 6.

### d. The round-trip cycle itself — the dominant, measured cost

Each `flowEvalTick` hop in §1 is a full `performInvocation` → `command ingress lane` →
`command ingress settled` → `refreshUi` → next `flowEvalTick` cycle
(`🗑️generated/react-verify/boot/console.txt:1416-3641` shows this shape at boot: 12
`command ingress` events and one `flowEvalTick` chain before the first mesh paints). At native,
sub-millisecond guest-evaluation cost (§2a) and ≤2 mesh round trips (§2b), a 6–8-hop example still
costs 7–10 wall seconds — i.e. roughly 1 second per hop, which is host/guest message-passing and React
re-render overhead, not compute. This is the same shape the wgpu lane diagnosed on its own side
(`📓️wgpu-edit-convergence-perf-2026-09-14.md` §2.1: one wgpu "slice boundary" cost 15.46 ms against an
unthrottled task's 0.0115 ms, a 1345× gap) — no lane has run the React-side equivalent instrumentation
(a per-hop timing breakdown of `performInvocation`→`refreshUi`→next-tick) to say which of {shard
message-passing, JSON encode/decode, `refreshUi`'s own DOM/React commit} is the ~1 s/hop cost. §5 names
this as the single largest unmeasured item.

### e. `refresh_ui` with `patched=0` — a wgpu-only pathology; React already scopes its refresh

`📓️wgpu-edit-convergence-perf-2026-09-14.md` §7 measured the wgpu shell's `refresh_ui`
(`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4171`) walking every live window and panel unconditionally —
137 `renderSurface` calls per converging edit, 116 of them `patched=0`. **React does not do this.**
`🏛️ShellHost/🟦️.tsx` threads a `UiDirtyScope` through `refreshUi` (`useCallback` at `🟦️.tsx:4941`,
`buildUiRefreshRequest` + a `preserveJsonIdentity`-cached response, visible in the locale-refresh path at
`🟦️.tsx:4970-4977` and the spawned-instance path at `🟦️.tsx:5010-5030`) and the shared kernel predicates
`uiDirtyScopeWantsWindowBody`/`wants_panel_body`/`wants_section`/`wants_catalogue`
(`🧰️framework/🔨️modules/🎠️kernel/🦀️.rs`, `🟦️.tsx` twin) that the wgpu lane had to backfill for its own
shell (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §3.1: "React had the whole rule privately in
`ShellHelpers`... the wgpu shell had none of it"). This is confirmed, not assumed: the wgpu lane's own
dispatch trace showed the settles that dominate a converging edit (`flowEvalTick`) declare scope `none`
seven times per edit and still render every surface on wgpu — the analogous React trace was not
re-captured in this audit, but the code path (§ above) has no unconditional per-surface walk to find.

### f. React re-renders — dirty-scope narrows the HOST refresh; component-tree re-render breadth was not separately audited

§2e establishes that `refreshUi` only asks the guest to re-render surfaces a dispatch actually dirtied.
What was **not** measured by any lane: whether `ShellHost`'s own React tree (the ~4300-line component
housing `refreshUi`) re-renders broadly on every dispatch regardless of scope — e.g. via one large
context/state object whose reference changes on every `dispatch()` call, which would re-render every
consumer even when `UiDirtyScope` correctly told the GUEST to patch only one surface. No React Profiler
capture or `why-did-you-render`-style trace exists under `🗑️generated/` for any React run in this
ticket. This is the second half of §5's top unmeasured item.

### g. Style recalc — found and fixed in session 2; a known regression class, not currently reproduced

`📓️summary-2026-09-12.md` item 4: nine infinite CSS animations on `:root` over inherited registered
custom properties forced full-document style recalcs every frame, costing **112 s of a 129 s** hop
before being moved onto painting elements with `inherits: false`
(memory: `project-root-css-animations-recalc-style-storm`, "10 s/hop; check RecalcStyleDuration before
blaming wasm"). No 2026-09-13/14 report re-measures `RecalcStyleDuration` on the current tree, and §1's
per-hop costs (~1 s, not ~10 s) are consistent with this staying fixed, but it was not explicitly
re-verified this session — see §5.

### h. Worker/shard resilience cost — fixed, but the recovery path itself is still expensive when it fires

`📓️react-view-state-u64-carrier-2026-09-14.md` measured **608** `shard 0 worker fault [handler/turn]`
frames caused by a `Float(1.0)` where the guest's `FromValue` unsigned arm required an exact `u64`
(`🌱️value/🔁️codec/🦀️.rs:74`) — every dispatch on the React door failed until fixed at the crossing
(`🛂️manifest/🟦️.ts:1025` `viewContextWithIntegerCarriers`). Separately,
`📓️extension-evaluate-budget-2026-09-12.md` §1.2 found that a watchdog-killed shard's `restoreActor`
(`🎠️kernel/🟦️.ts:2157`) restores the actor but not the app (`createApp` never re-runs), so every later
command failed `plugin.command-page-invalid` until the user reloaded — fixed by budgeting `evaluate`
itself so the watchdog rarely fires. Both are closed; neither lane measured the cost of a shard restart
when the watchdog DOES still fire (e.g. a future example whose exact-predicate path is not yet as cheap
as `sphere-cut-with-torus`'s).

### i. Preview re-arm and the `toolRunStart` storm — fixed, but three of six defects lived at the app/editor layer this same renderer owns

`📓️preview-rearm-after-inspector-edit-2026-09-14.md` closed a six-defect chain (four at the framework
tool-run layer, two app-side) that meant an Inspection-panel slider edit never re-evaluated the preview
at all, and — mid-fix — a wrongly-keyed latch produced **3 568 `toolRunStart` requests in 45 s**
(§5.4, run8, reverted before landing) against a steady-state of ~1 start per edit after the fix. The
pre-existing storm this lane inherited was **1 053 starts in 242 s** (§9 of
`📓️react-generate-chord-restage-2026-09-14.md`, not reproduced fresh this session but not contradicted
by any later measurement). This is the clearest evidence in the whole ticket that a broken re-arm latch
is a bigger user-visible cost than any single compute step — 1 000+ wasted round trips dwarfs the ~1 s
each one costs per §2d.

---

## 3. Top 8 architecture optimizations, ranked by expected user-visible gain

Each entry: the file/function that owns the change, the invariant it must not break, and how to measure
whether it worked.

1. **Instrument and cut the ~1 s/hop round-trip cost (§2d).** No lane has broken down what a
   `flowEvalTick` hop actually spends its ~1 s on once guest compute and mesh transfer are both fixed
   to sub-second cost. This is the single largest remaining, *unattributed* term in the whole table (7
   hops × ~1 s dominates an 8 s example). *Owning file*: the hop lives across
   `🔌️PluginRuntime/🟦️.tsx` (`performInvocation`) and `🏛️ShellHost/🟦️.tsx` (`refreshUi`,
   `🟦️.tsx:4941`). *Invariant*: no change here may alter what a tick settles or its ordering relative to
   `history patch applied` — only its wall cost. *Measure*: a per-hop timing probe like
   `🐍️wgpu-raf-cost-probe.mjs`'s technique (CDP-level timing inside the live isolate) run against the
   React shard worker, isolating message-passing time from React commit time from JSON codec time.

2. **Per-mesh-id memoization in `WorldInstances` instead of whole-array (§2c).** Key `geometries`
   /`borderGeometries`/`vertexPickByMeshId`/`edgeGeometryByMeshId` on each mesh's own id+contentHash
   (already present on the wire per `preview-mesh-delivery`'s handle-hashing) rather than on the
   `meshes` array reference, so an N-mesh surface with 1 changed mesh reallocates 1 `BufferGeometry`,
   not N. *Owning file/function*: `🧰️framework/…/🌐️World3dHost/🟦️.tsx:2841` (`WorldInstances`) and
   `geometryFromMesh` (`🟦️.tsx:1647`). *Invariant*: geometry identity per mesh id must stay stable
   across renders that did not touch that mesh (existing dispose-on-unmount effects must still fire for
   truly removed ids — do not leak). *Measure*: three.js `renderer.info.memory.geometries` allocation
   count across a single-node edit on a multi-mesh example (Hexagonal Mushroom Column, 3 meshes) before
   and after — expect 1 allocation instead of 3 when only one mesh moves. Low measured urgency today
   (mesh counts are 1–3), ranked here for when generate/multi-body examples grow that count.

3. **Reduce the flow graph's own per-node hop count.** §1 shows 6–8 `flowEvalTick` settles per
   multi-node example (Sphere Cut With Torus, Face Sweep Extrude) vs 2–3 for simple ones (Rectangle Wire
   Preview). With mesh transfer now ≤2 round trips (§2b), hop count is now dominated by how many
   dependency-graph nodes get their own settle round rather than being coalesced. *Owning file*: the
   flow evaluation driver in `🧰️framework/…/🌊️flow/🖥️host/🦀️.rs` (the same file that owns
   `tick_scheduled`, `🦀️.rs:2739` and the status ledger `preview_eval_status`, `🦀️.rs:3471`).
   *Invariant*: a coalesced multi-node tick must still publish the SAME intermediate/final geometry a
   user could cancel out of mid-chain — do not silently make evaluation uncancellable. *Measure*: the
   `hops(flowEvalTick)` column in §1's own table, re-run after the change; expect it to drop toward 2–3
   for every example.

4. **Fix the backwards progress ratio (found by `wgpu-progress-visibility`, item 4).**
   `preview_eval_status`'s budgeted-eval ledger aggregates only its LIVE rows, so a finishing node
   shrinks both numerator and denominator — `Computing 7/7 (100%)` → `4/6 (67%)` mid-evaluation
   (`📓️wgpu-progress-visibility-2026-09-14.md` §4.3, `🌊️flow/🖥️host/🦀️.rs:3471`). This is a
   correctness bug more than a speed one, but it is the top *perceived*-performance defect: on the
   longest-running examples (Sphere Cut With Torus, 7 hops / 8 s) a user watching the percentage go
   backwards will believe the app is stalled or looping. *Owning function*: same file, "the monotone
   chain census" fix the wgpu lane already designed and landed for its own shell — confirm it reads
   through the React `World3dComputeStatusV1` contract too (`status-parity` battery step). *Invariant*:
   ratio must be monotonically non-decreasing within one evaluation. *Measure*: `status-parity` battery
   step plus a direct ratio-sequence assertion (`ratio[i] <= ratio[i+1]`) over one long example.

5. **A React Profiler / re-render-breadth capture (§2f).** No lane has established whether `ShellHost`'s
   own component tree re-renders broadly per dispatch regardless of `UiDirtyScope`. If it does, the
   host-side win of dirty-scope refresh (§2e) is partly cancelled by unnecessary React reconciliation
   work on every tick. *Owning file*: `🏛️ShellHost/🟦️.tsx` — look for one wide context/reducer whose
   value identity changes on every `dispatch()`. *Invariant*: none yet — this is a measurement task, not
   a change. *Measure*: React DevTools Profiler (or a `useWhyDidYouUpdate` shim) recording render counts
   per component across one `flowEvalTick` chain; compare re-rendered component count to the number of
   surfaces `UiDirtyScope` actually named.

6. **Confirm `RecalcStyleDuration` stays near zero (§2g).** The 09-12 fix moved 9 infinite `:root`
   animations off inherited custom properties, cutting 112 s of a 129 s hop to near-zero. Nothing in the
   09-13/09-14 reports re-measures it on the CURRENT tree, and CSS/animation code is exactly the kind of
   file peers touch without knowing this history. *Owning file*: wherever the moved animations now live
   (painting elements, per `📓️summary-2026-09-12.md` item 4 — not re-cited by path in any 09-14 report,
   needs re-location). *Invariant*: animated custom properties must stay `inherits: false` and off
   `:root`. *Measure*: Chrome performance trace, `RecalcStyleDuration` summed over one journey run;
   regression gate at, say, 5% of wall time.

7. **Decode-path profiling for `geometryFromMesh` at realistic mesh sizes (§2b tail).** All measured
   examples are 1–3 meshes; nothing in this ticket has profiled `Float32Array`-copy + `setIndex` cost at
   the sizes a denser generative example (many small instances, or a much higher tessellation tolerance)
   would produce. *Owning function*: `geometryFromMesh`, `🟦️.tsx:1647`. *Invariant*: none — pure
   profiling. *Measure*: a synthetic fixture with e.g. 50 small meshes run through the same
   `journey-probe.mjs` convergence predicate, timing the `useMemo` in `WorldInstances` directly (already
   instrumentable via `performance.mark` around `🟦️.tsx:2841-2849`).

8. **Watchdog-restart cost when it still fires (§2h tail).** `evaluate` is now budgeted so the 16 s
   watchdog trip that killed a shard mid-boolean is rare, but no lane measured how expensive a shard
   restart is in wall time when the budget IS exceeded (e.g. by a future example with an as-yet-unoptimized
   exact-predicate hot path). *Owning file*: `ActivationRegistry.restoreActor`
   (`🎠️kernel/🟦️.ts:2157`) and the reactor's command-page predicate
   (`⚛️reactor/🔄️turn/🦀️.rs:884`). *Invariant*: a restored actor must still refuse stale command pages
   (the fix that prevents `plugin.command-page-invalid` loops must not be undone in the name of speed).
   *Measure*: force a watchdog trip (an artificially slow `evaluate` fixture) and time boot-to-recovered.

---

## 4. What is NOT known, and the probe that would measure it

1. **The ~1 s/hop attribution (§2d, ranked #1 above).** Not known: how much of one `flowEvalTick` hop's
   wall cost is shard message-passing, JSON/pack codec, vs React commit/DOM. *Probe*: a CDP-attached
   timing probe on the React shard worker's own isolate, same technique as
   `🐍️wgpu-raf-cost-probe.mjs` used for the wgpu frame worker
   (`📓️wgpu-edit-convergence-perf-2026-09-14.md` §2.1) — that probe does not exist for the React door.

2. **React component re-render breadth (§2f, ranked #5).** Not known: whether `ShellHost`'s tree
   re-renders beyond what `UiDirtyScope` scoped. *Probe*: a React Profiler capture (or an injected
   `useWhyDidYouUpdate`) across one `journey-probe.mjs` run, correlated against the `[DEBUG]` dirty-scope
   trace the wgpu lane already added on its own side (`📓️wgpu-dirty-scope-refresh-2026-09-14.md` §3.4)
   — no such trace exists for the React door yet.

3. **Whether mesh geometry rebuild cost (§2c) is currently negligible or already showing up in the ~1
   s/hop figure.** Not known: current examples are too small (1–3 meshes) to tell `useMemo`-rebuild cost
   apart from round-trip cost. *Probe*: item 7 of §3 — a synthetic many-mesh fixture, or
   `performance.mark`/`measure` bracketing added around `🟦️.tsx:2841-2849` in a live run of Sphere Cut
   With Torus (88 230-char mesh payload, the largest in §1's table) to see if it is already visible
   there.

4. **`RecalcStyleDuration` on the current tree (§2g).** Not known: whether the 09-12 CSS fix still holds
   after two more days of peer edits to styling-adjacent files. *Probe*: a Chrome performance trace over
   one `journey-probe.mjs` run, filtered to `RecalcStyle` events, summed and compared against the ~1
   s/hop baseline in §1.

5. **Whether the backwards-ratio fix (§3 item 4) actually reads through the React
   `World3dComputeStatusV1` contract**, not just the wgpu shell's own pill.
   `📓️wgpu-progress-visibility-2026-09-14.md` fixed the projection at the shared layer
   (`🌊️flow/🖥️host/🦀️.rs:3471`) and reports the React `status-parity`/`cancel-preview` battery steps
   green (§7.3 of that report), but does not show a ratio *sequence* from a React run — only that a
   status object is non-null at each sample. *Probe*: `status-parity`'s own capture, extended to record
   every sample's ratio (not just presence) across one long-running example (Sphere Cut With Torus) and
   assert monotonicity.

6. **Cold-boot cost breakdown for the React door specifically.** wgpu's `boot_shell` was profiled
   hop-by-hop (`📓️wgpu-edit-convergence-perf-2026-09-14.md` §2); the React boot step in §1's table (11.3
   s, 7 hops) has never had the same treatment — contributions install (`1525`–`2981` in
   `🗑️generated/react-verify/boot/console.txt`, a 248 827-char pack), first `setContributions`, and first
   `toolRunStart` are all inside that one step and not individually timed. *Probe*: the same per-phase
   `[DEBUG]` timestamp accounting `wgpu-edit-perf-probe.mjs` used, adapted to the React console's own
   `performInvocation`/`command ingress`/`refreshUi` lines (already present, just not yet summed by
   phase).

---

## 5. Files referenced

Lane reports (all under this ticket folder):
`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, `📓️react-view-state-u64-carrier-2026-09-14.md`,
`📓️wgpu-edit-convergence-perf-2026-09-14.md`, `📓️wgpu-dirty-scope-refresh-2026-09-14.md`,
`📓️wgpu-progress-visibility-2026-09-14.md`, `📓️summary-2026-09-12.md`,
`📓️preview-mesh-delivery-2026-09-12.md`, `📓️kernel-performance-2026-09-13.md`,
`📓️extension-evaluate-budget-2026-09-12.md`, `📓️react-end-to-end-verification-2026-09-13.md`,
`📓️close-ladder-budget-2026-09-12.md`, `📓️react-generate-chord-restage-2026-09-14.md`.

Runtime evidence: `🗑️generated/react-u64/journey/{results.json,console.txt}` (latest, used for §1),
`🗑️generated/react-verify/journey/results.json` (delta check), `🗑️generated/react-verify/boot/console.txt`
(boot-phase citations in §2d, §4.6).

Source, generation3d/host:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx`,
`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`,
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` (mesh
decode + `WorldInstances`, §2b–c),
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx` (`refreshUi`,
§2e–f),
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs` (`tick_scheduled`,
`preview_eval_status`),
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📐️brep-geometry/🦀️.rs` (`MESH_PACK_CHUNK_BASE64_CHARS`),
`✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️preview-eval/🦀️.rs`
(`EVALUATE_STEP_WALL_MICROS`, `owe_attached_previews*`),
`✏️s/…/✏️editor/🦀️.rs` (route byte budgets), `✏️s/…/👁️viewer/🦀️.rs` (viewer route byte budget).
