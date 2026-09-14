# React oracle hardening — generation3d, 2026-09-14 (lane `react-oracle-hardening`, port 6022)

Scope: the **React** renderer of the procedural 3d (generation3d) playground. Owns items 1–4 of §4 in
`📓️example-oracle-strength-audit-2026-09-14.md` and items 1 and 3 of §5 in
`📓️window-coverage-audit-2026-09-14.md`. Everything below was run on `http://127.0.0.1:6022/?plugin=generation3d`
(direct vite serve, `SEMIO_VITE_HMR=0`). Evidence under `🗑️generated/react-oracle/` — this lane never
wrote to `🗑️generated/react-verify/`, which belongs to lane `react-current-tree-battery`.

---

## 0. The one-line version

Every mesh check in the React battery is now compared to the committed fixture, hover/select/orbit/fit
cover all 8 examples with **value** assertions instead of console regexes, the `io-surface` gate can
fail, and every probe is gated on a clean page. Three product defects surfaced and **two are fixed**:
React world3d camera navigation was dead (every gesture), and the preview dispatched an action the app
never declares. The third — a chrome panel swallowing the world-view overlay rail's clicks, which
blocks both `Frame visible` and the preview **Cancel** — is diagnosed with exact coordinates and handed
on; it is not this lane's surface to re-layout.

---

## 1. The oracle, and where it comes from

New shared module `🐍️example-oracle.mjs` reads the same committed fixtures the native
`example-geometry` test reads — `📚️examples/<dir>/🧫️fixtures/🧩️example/🔣️.json`, schema
`s.procedural.generation3d.example-geometry/v1` — and states, per example:

| field | native line it mirrors |
|---|---|
| `delivery.meshes` (exact mesh **and** instance count) | `📚️examples/🧪️tests/🧩️geometry/🦀️.rs:854-855` |
| `delivery.minTriangles` / `minEdgeSegments` (floors on the PREVIEW mesh) | `🦀️.rs:849-850` |
| `expect.boundingBoxMin/Max` ± tolerance | `🦀️.rs:568-569`, bounds computed by `bounds()` (`🦀️.rs:450`) over `positions`, or `edgePositions` when the mesh has no triangles |
| preview mesh id `eval-{preview.node}@{preview.channel}#0` | published by `✏️editor/🦀️.rs:2852`, viewer `👁️viewer/…/👁️preview/🦀️.rs:243,303` |

The module also documents the DOM lanes it measures against, which no probe in this ticket had written
down before:

- `data-meshes-json` — `[{ id, data?: { positions, normals, colors?, uvs?, indices, faceIds?, vertexIds?, edgePositions?, edgeIds? }, url?, kind? }]`; flat `[x,y,z,…]` buffers, `indices.length/3` triangles, `edgePositions.length/6` edge segments.
- `data-instances-json` — `[{ id, meshId, position, rotation, scale, label, interactionId, selected, hovered }]`; `interactionId` is the id a pick reports.
- `data-selection-json` — `worldSurfaceSelectionDomV1` (`World3dHost/🟦️.tsx:1438`): `{ selectedIds, activeObjectId, targetVolumeIds, referenceSelectedId, hoverTarget: {domain,id}|null, hoveredVortexFullId, hoveredKindId, gumballActive, gumballTarget, transformMode, activeUtility }`.
- `data-guest-selection-json` — the same, as the GUEST sent it, before the host's leftover overlay.
- `data-camera-json` / `data-viewport-camera-json` — `{ position:[x,y,z], target:[x,y,z], up, zoom, fov, projection }`.

**Correction to the audit.** §4 item 3 proposed reading `data-interaction-json` for hover/selection
values. That attribute is the per-window utility/brush record and publishes **`{}`** on every
generation3d run measured here (also `{}` in the last `react-verify` run's own `summary.json`) — a
verdict reading it could only ever be vacuous. The lane that carries a hover or a selection VALUE is
`data-selection-json`. Every new assertion below reads that one.

### 1.1 The bounding-box budget is asymmetric, and why

A tessellation of these solids can only INSCRIBE the true surface, so **overshoot** is held to the
fixture's own `boundingBoxTolerance` — a delivered extent reaching past `expect.boundingBoxMin/Max` is
a wrong scale, transform or example, never a level of detail. Measured across all 8 examples in both
roles, no runtime extent ever exceeded its fixture's.

**Shortfall** gets a stated envelope, because `delivery` commits no extent of its own and the delivered
mesh is demonstrably coarser than the `expect` tessellation: `sphere-box-fuse` delivers its sphere pole
at exactly `-1.2` but its equator at `-1.1863` (0.0137 short, on an example whose
`tessellationTolerance` is 0.0025), and `sphere-cut-with-torus` hits `±2.2` and `+2.1475` exactly while
falling 0.047 short on one axis. The envelope is
`max(boundingBoxTolerance + tessellationTolerance, 2% of the box's largest extent)`.

**Not claimed:** that 2% is a *verified* number. The clean end of this is a committed
`delivery.boundingBoxMin/Max/Tolerance` authored from `run_delivery`'s own payload in
`📚️examples/🧪️tests/🧩️geometry/🦀️.rs`, after which both halves are exact and the envelope goes away.
That needs a native `cargo test --test example-geometry` run this lane did not make (the fleet was
building the same crate for a peer at the time).

The envelope is not vacuous — checked by grading real payloads against the wrong oracles:

| payload | graded against | verdict |
|---|---|---|
| box-shell (1 mesh, 24 tri, 0..2³) | `sphere-box-fuse` | RED — `triangles 24 below floor 440`, `bbox max axis 0 reaches 0.5 past 1.5` |
| box-shell | `rectangle-extrude-volume` | RED — `bbox max axis 2 falls 1 short of 3, beyond the 0.06 LOD envelope` |
| box-shell | `hexagonal-mushroom-column` | RED — `meshes 1 vs oracle 3`, `instances 1 vs oracle 3` |
| box-shell with every extent halved | its own oracle | RED on all three axes |

---

## 2. Item 1 — every example × role vs the oracle

`🐍️journey-probe.mjs` now loads the oracle per picked example, computes per-preview mesh numbers in
the page (the position buffers never cross the bridge), and `converged()` **requires** exact
`delivery.meshes`, exact instance count, the delivery floors and the extent — for every non-empty
example, in edit AND viewer role. Rows also carry a `expectSurface`, so a row that names a mode's own
preview window can no longer settle on whichever window is still mounted from the previous one.

Full run: `🗑️generated/react-oracle/journey-1/results.json` (first, pre-fix) and the battery's own
`🗑️generated/react-oracle/journey/` (final). Numbers are identical in both roles.

| example | oracle meshes / minTri / minEdge | oracle bbox min → max | edit role measured | viewer role measured | verdict |
|---|---|---|---|---|---|
| hexagonal-mushroom-column | 3 / 20 / 18 | [-0.5, -0.43301, 0] → [0.5, 0.43301, 6] | 3 meshes, 3 inst, 20 tri, 18 edges, [-0.5, -0.43301, 0]→[0.5, 0.43301, 6] | identical | ✅ exact |
| rectangle-extrude-volume | 1 / 12 / 12 | [0,0,0] → [2,2,3] | 1/1, 12 tri, 12 edges, [0,0,0]→[2,2,3] | identical | ✅ exact |
| face-sweep-extrude | 1 / 12 / 12 | [0,0,0] → [2,1.5,4] | 1/1, 12 tri, 12 edges, [0,0,0]→[2,1.5,4] | identical | ✅ exact |
| box-shell-preview | 1 / 24 / 24 | [0,0,0] → [2,2,2] | 1/1, 24 tri, 24 edges, [0,0,0]→[2,2,2] | identical | ✅ exact |
| box-fillet-preview | 1 / 92 / 72 | [0,0,0] → [2,2,2] | 1/1, 92 tri, 72 edges, [0,0,0]→[2,2,2] | identical | ✅ exact |
| rectangle-wire-preview | 1 / 0 / 4 | [0,0,0] → [2,1.5,0] | 1/1, 0 tri, 4 edges, [0,0,0]→[2,1.5,0] | identical | ✅ exact (wire: extent from `edgePositions`) |
| sphere-box-fuse | 1 / 440 / 25 | [-1.2,-1.2,-1.2] → [1.5,1.5,1.5] | 1/1, 440 tri, 25 edges, [-1.18629, -1.18177, -1.2]→[1.5,1.5,1.5] | identical | ⚠️ inside the LOD envelope (see §1.1) |
| sphere-cut-with-torus | 1 / 1114 / 45 | [-2.1475,-2.1475,-2.2] → [2.1475,2.1475,2.2] | 1/1, 1114 tri, 45 edges, [-2.10057, -2.13574, -2.2]→[2.1475, 2.13574, 2.2] | identical | ⚠️ inside the LOD envelope |
| "No example" | 0 meshes, 0 instances | — | 0/0 | 0/0 | ✅ empty state holds |

**The four viewer-role rows that used to pass with 0 meshes now deliver the oracle.** The audit's §2
named `view:Hexagonal Mushroom Column`, `view:Sphere Cut With Torus`, `view:Sphere Box Fuse` and
`view:Rectangle Wire Preview` as passing on an empty canvas
(`🗑️generated/react-verify/scoreboard.json`, 15:18). On the current tree all four deliver exactly their
committed mesh count with the right geometry, and the gate is now armed so it cannot regress silently.
**Not claimed:** that this lane fixed them — the 0-mesh condition was already gone when this lane first
measured on the current tree; what this lane added is the assertion.

`view:No example` failed the first run for a probe reason, not a product one: the `viewer-role` row had
settled on the still-mounted EDIT preview, so the next pick was made against a surface that then
swapped under it. The `expectSurface` gate above fixes that; the row is green in the final run.

Journey, final: **23/23 rows converged, 0 page errors, 0 shell faults** — 9 edit rows, 9 view rows,
boot, generate-mode, generate-added, back-to-edit, viewer-role.

---

## 3. Item 2 — hover / select / inspector / orbit / fit over all 8 examples

The `interact` lane no longer runs `🔍️browser-probe.ts --example=box-shell-preview` graded by regexes
over the run's console dump. It runs a new `🐍️interaction-matrix-probe.mjs` that loops **all 8
examples** and decides each hop on a published value:

| hop | what it asserts now | what it asserted before |
|---|---|---|
| `payload` | the delivered meshes match the committed fixture | nothing (`meshCount > 0`) |
| `selection-reset` | after the example switch, every still-selected id belongs to the example now on screen | — (new) |
| `hover` | the pane publishes a `hoverTarget` whose **id is one of this example's own** instance/`interactionId`/mesh ids; the probe sweeps a grid until the pointer is over geometry and reports the point | `/hoverTarget\|interactionHover/` over the whole console |
| `select` | clicking that same point leaves `selectedIds` **exactly** `[hovered]` and `activeObjectId === hovered` (a plain click is a REPLACE merge) | `/selectedIds\|interactionSelect/` over the whole console |
| `inspector` | the Inspection panel's own `…procedural-play-inspector.id` row then names the selected widget | — (new) |
| `orbit` | Alt+right drag moves the camera POSITION, keeps its target and keeps its distance — i.e. an orbit, not a pan or a no-op | `/setCamera/` over the whole console |
| `fit` | `Frame visible` reproduces the product's own fit rule (`fitCameraFromBounds`, `World3dHost/🟦️.tsx:860`: target = the framed AABB centre, distance = `max(radius*1.8, 2)`) over the payload — which, with `payload` green, means the camera frames the fixture's committed box | — (new) |

The old lane's "drag publishes a camera" step is a worked example of why this mattered: it
**left**-dragged, which in this product is the marquee and never the camera (`resolveWorldOrbitMouseButtonsIdle`
returns `LEFT: null`; orbit is Alt+right, pan is Shift+right or middle) — and it passed anyway, because
`setCamera` appears in the console at boot, before any gesture.

Result of the battery's `interact` lane, 8 examples × 7 hops:

| hop | green |
|---|---|
| payload | **8/8** |
| hover | **8/8** — every example published a hover target that is one of its own ids |
| selection-reset | 2/8 |
| select | 1/8 |
| inspector | 5/8 |
| orbit | 4/8 (see §4.4 — 4 of those reds were a probe artifact, now fixed) |
| fit | 0/8 (blocked, §4.3) |

---

## 4. Product defects found

### 4.1 FIXED — React world3d camera navigation was completely dead

**Symptom.** On the generation3d edit preview, no camera gesture moved the camera:
`data-viewport-camera-json` stayed on its seed pose `{position:[4,-4,3], target:[0,0,0]}` after wheel
zoom, Alt+right (orbit), Shift+right (pan) and middle-drag (pan). No `setCamera` was ever dispatched.
A DOM listener installed on the canvas recorded **zero** `pointerdown` and **zero** `wheel` events.

**Root cause.** `WorldCanvas` renders `<Canvas eventSource={…}>`, and `@react-three/fiber` stamps
`pointerEvents: 'none'` on the canvas whenever an external event source is given
(`node_modules/@react-three/fiber/dist/react-three-fiber.esm.js:124`). Three bindings were nonetheless
attached to `gl.domElement`: the `OrbitControls` instance itself, the Alt/Shift right-button mapping,
and the custom wheel hook. All three were listening on an element the browser never routes an event to.

**Fix** (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`):
- new `useWorldPointerTarget()` — the r3f `events.connected` element, falling back to `gl.domElement`;
- `WorldOrbitControlsBridge` constructs `ThreeOrbitControls(camera, pointerTarget)`;
- `WorldOrbitGated` passes `pointerTarget` to `useWorldOrbitRightMouseBindings`;
- `WorldCanvas` binds its wheel hook to the event source in an effect (and the now-redundant
  `wheelCleanupRef` is gone).

**Proof.** Same probe, same port, after the fix: wheel → `[3.258, -3.258, 2.4435]`, Alt+right drag →
`[0.6557, -3.8686, 3.4357]`, both with `setCamera` dispatched and a history entry
(`🗑️generated/react-oracle/` recon output). This is framework-wide, so every React world3d window
regains camera navigation, not only generation3d.

### 4.2 FIXED — the preview dispatched `setHover`, an action the app never declares

**Symptom.** `semio: app "s.procedural.generation3d@1/*#editor" dropped action "setHover" dispatched
from window kind "procedural-preview": no window kind declares it` — a console error on component-hover
hits, with the hover itself lost.

**Root cause.** `World3dHost`'s `handleComponentHover` dispatched `setHover` unconditionally.
`setHover` is the DOMAINLESS hover lane; an app that declares an interaction domain (generation3d
declares `graph`) declares `interactionHover` instead and never `setHover`. The sibling
`dispatchInstanceHover` already forks on `interactionDomainId` — `handleComponentHover` did not.

**Fix** (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`):
when a domain is declared, a component hit publishes through `dispatchInstanceHover(args.objectId)`,
which carries the domain, the window's own declared granularity and the coalescing gate; the domainless
`setHover` path is kept only for apps that declare no domain.

### 4.3 OPEN — a chrome panel swallows the world-view overlay rail, blocking `Frame visible` AND the preview `Cancel`

**Symptom.** Two affordances in the world pane's own top-right overlay rail cannot be clicked:
- `[data-slot="world-frame-instances"]` (`Frame visible`) — `fit` is 0/8, every example, every run;
- `[data-slot="world-compute-cancel"]` (`Cancel` on the compute-status pane) — this is what made the
  battery's `cancel-preview` probe exit 1 (`🗑️generated/react-oracle/cancel-preview/run.txt`).

**Evidence.** The button is in the DOM, visible, enabled, stable, `pointer-events: auto`, at
(1355, 83) 75×16. `document.elementFromPoint` on its own centre returns a chrome panel, and Playwright
names the interceptor outright: `<div dir="rtl" data-slot="panel" data-anchor="top-right"
data-ghost-region="true" data-panel-visible="true" data-active-tab-id="framework.panel.toolRun"
id="framework.panelTab.framework.panel.toolRun" class="… z-panel …"> subtree intercepts pointer
events`. The panel's rect is (1137, 3) 300×120 and it paints real content
("Inspection / Tool runs / Chat / Collapse / …"), so this is not an empty ghost that should be
transparent — it is two overlays claiming the same corner. The rail is `z-40` inside the window's own
`z-window` stacking context; the panel is `z-panel` in the app root's, so the rail can never win on
z-index alone, and `windowChromeClearedTopOffset` (the rail's top offset) only clears the WINDOW's own
chrome — the framework has no reserved-inset concept for anchored chrome panels over window content.

**Why this lane did not fix it.** The correct fix is either a new framework "window content safe area
reserved by anchored chrome panels" or a re-anchoring decision for the rail; both are chrome-layout
ownership, and the same panel is mid-flight in lane `react-current-tree-battery` (its `Tool runs` tab
landed in `f2fc008ee2`). Blocking `Cancel` also violates the standing rule that every expensive
operation must offer cancellation, so it should be triaged with that weight.
**Owners:** the panel/chrome layering surface, plus `World3dHost/🟦️.tsx:6726-6737` (the rail). As of
19:05 a lane named `chrome-panel-safe-area` exists on port 6022, which is where this belongs.

### 4.4 OPEN, smaller — selection is not pruned when the example changes

**Symptom, measured across the whole matrix.** Selecting in one example and switching to the next
leaves the old id standing: `selection-reset` is 2/8 and `select` is 1/8. Concretely, after picking in
`Box Shell Preview`, `shell@solid` was still in `selectedIds` for every later example —
`Sphere Cut With Torus` reported `["brep_bool_cut_5@solid", "shell@solid"]` after a plain (REPLACE)
click, `Rectangle Extrude Volume` reported `["shell@solid"]` (the new pick did not land at all), and
`Box Fillet Preview` reported `["shell@solid"]` while the Inspection panel described `shell`, a widget
no longer in the document.

**Where the claim and the behaviour part.** `🎮️commands/🎨️set-active-example/🦀️.rs:15-17` states
"`graph`'s selection resets on its own — the framework prunes it against the new fixture's
`interaction_topology`". The editor does declare `HierarchyProvider::Topology` for `graph`
(`✏️editor/🦀️.rs:2498`) and its topology does emit `{nodeId}@{portId}` handles
(`✏️editor/🦀️.rs:2085-2115`) — the exact id shape the preview selects — and the framework's pruning
path is `build_full_interaction_topology` → `validate_state`
(`🧰️framework/…/🔌️plugin/🦀️.rs:23606-23645`). So the mechanism exists and is wired, but the stale id
survives.

**Not claimed:** which half is at fault. The probe now records `data-guest-selection-json` alongside
`data-selection-json` in the `select` and `selection-reset` rows precisely to split "the guest's own
selection kept the id" from "the host's leftover overlay kept it" — that evidence lands in the next
`interact` run and was not yet available when this was written.

### 4.5 Observed flake, not reproduced on demand — a cold boot that never starts

The battery's `boot` probe recorded 4 console lines in 100 s and zero surfaces
(`🗑️generated/react-oracle/boot/console.txt`): `[vite] connected`, the React DevTools notice, then
`[DEBUG] buildShardClientOptions heartbeatTimeoutMs=120000` and silence. Three other
`🐍️console-dump-probe.mjs` runs on the same port the same hour booted normally. Consistent with the
known slow/flaky cold boot of this dev preview; recorded, not diagnosed.

---

## 5. Item 3 — battery integrity

### 5.1 `io-surface` can now fail

`🐍️io-surface-probe.mjs`'s `record()` never set a top-level `ok`/`error`, so
`🐍️react-battery.mjs`'s `x.ok !== undefined ? Boolean(x.ok) : !x.error` read `undefined`/`undefined` on
all six rows — the P0 import/export gate was structurally incapable of failing. Both halves are fixed:
every row now states its own verdict from a measured value, and the battery reads `Boolean(x.ok)`, so a
row that carries no verdict is red rather than green.

| row | verdict now |
|---|---|
| `boot` | the boot loop actually reached a preview + a published graph |
| `published` | both action rows exist AND carry labels |
| `export-form-open` | the staged form's `#format` control exists |
| `export` | no error and `bytes > 0` |
| `import` | the picker opened AND `imported-geometry` was planted |
| `import-settled` | `imported-geometry` still in the graph and no visible fault |

Live run, green on real numbers: `STL Mesh` → `generation3d.stl`, **3810 bytes**, head
`solid generation3d-preview / facet normal 0 0 -1 …`; import planted
`["imported-source","imported-geometry","imported-preview"]`.

### 5.2 Every probe is gated on a clean page

The gate is applied **centrally** in the battery loop rather than in sixteen verdicts, so a probe added
later inherits it: after each probe's own verdict, two decided steps are appended —
`no page errors` (`pageerrors === 0`) and `no shell faults` (`FAULT_RE` matches === 0) — with any
probe-supplied duplicate of those two names dropped first.

`🐍️customization-persistence-probe.mjs` was also hardened: it published `pageerrors: 0` without ever
registering a listener, so its zero meant "not observed". It now records console and page errors to
`console.txt` and the battery counts them.

**What the gate would have caught.** Replaying it over the last `react-verify` scoreboard
(2026-09-14T13:18:41Z, reported `green=16/16`): **6 of 16 probes** carried page errors or faults and
would now be red — `journey` (2), `gaps` (4), `generate-mode`, `flow-wire`, `role-switch`, `i18n-a11y`
(1 each): the recurring `TypeError: Cannot read properties of null (reading 'addEventListener')` and
one `FlowMessageRejected`.

**On the current tree those faults are gone.** Every probe in the run below reported `pageerrors: 0`
and `faults: []`, so the gate is armed and green rather than armed and red. **Not claimed:** that this
lane fixed the `addEventListener` TypeError or `FlowMessageRejected` — neither reproduced once on 6022
across ~10 browser runs today; a stack-capturing probe (`🐍️page-fault-stack-probe.mjs`) is left in the
ticket so the next occurrence is caught with a call site instead of a bare message.

---

## 6. Battery result

`SEMIO_BATTERY_URL=http://127.0.0.1:6022/?plugin=generation3d SEMIO_BATTERY_ROOT=react-oracle bun 🐍️react-battery.mjs`
→ `🗑️generated/react-oracle/scoreboard.json`, log `🗑️generated/react-oracle/battery-run.txt`.
(`SEMIO_BATTERY_ROOT` already existed when this lane arrived — a peer had added it; this lane used it
rather than adding a second knob, so `🗑️generated/react-verify/` was never touched.)

| probe | ok | reds |
|---|---|---|
| boot | ✗ | `surfaces mount` — the cold-boot flake, §4.5 |
| journey | ✓ | — (23/23, incl. all 16 example × role oracle rows) |
| interact | ✗ | `fit` 8×, `select` 7×, `selection-reset` 6×, `inspector` 3×, `orbit` 4× — §4.3, §4.4 |
| generate-mode | ✗ | `add-2` (one generation row after two Add clicks) — **not this lane's** |
| flow-window | ✓ | — |
| flow-wire | ✗ | `redraw restores the wire` — **not this lane's** |
| flow-reorganize | ✓ | — |
| cancel-preview | ✗ | exit 1: the `Cancel` button is click-intercepted — §4.3 |
| keyboard-verbs | ✗ | `baseline` (`flowEvalTick` fires on a quiet shell) — **not this lane's** |
| io-surface | ✓ | — now on real verdicts, §5.1 |
| export-encoding | ✓ | — |
| status-parity | ✓ | — |
| role-switch | ✓ | — |
| i18n-a11y | ✓ | — |
| customization-persistence | ✓ | — |
| gaps | ✗ | every step, on one page error — **not this lane's**, see below |

**`green=9/16, red=[boot, interact, generate-mode, flow-wire, cancel-preview, keyboard-verbs, gaps], 2262 s, pageerrors=1`.**

Fifteen of the sixteen probes recorded **0 page errors and 0 shell faults**. The sixteenth, `gaps`,
was turned red by the new gate on a genuine page error — `SyntaxError: The requested module
'/@fs/…/🧰️framework/🔨️modules/⏱️trace/🟦️.ts' does not provide an export named 'hopTraceEpochNowMs'` —
a peer's host-TS edit landing mid-run; the export is present in the tree now, so the run caught a
transient, not a standing defect. That is the gate doing its job: before it, that probe would have
reported its steps and the battery would have scored the broken module green.

The `interact` row's own folder is empty: a re-run of that lane was started against the 19:00 restage
and **aborted** when the coordinator reassigned port 6022 to lane `chrome-panel-safe-area` and reported
that the 19:00 wasm boots into a faulted preview (`command ingress exceeds 64 pages`, lane
`contributions-ingress-ceiling` fixing). The battery wipes a probe's folder before running it, so the
abort left it bare; the 18:18 run's 58 step details are intact inside `scoreboard.json` and recovered
to `🗑️generated/react-oracle/interact/results.json`. No interaction number in this report comes from
the faulted 19:00 stage.

---

## 7. Files changed

Product:
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx` — `useWorldPointerTarget`, orbit controls + right-mouse bindings + wheel hook moved off the `pointer-events:none` canvas onto the r3f event source (§4.1).
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx` — `handleComponentHover` publishes through the declared interaction domain instead of the undeclared `setHover` (§4.2).

Probes and battery (all in the ticket folder):
- `🐍️example-oracle.mjs` — **new**; the committed-fixture oracle, the page-side mesh reader, `gradeMeshes`, `gradeCameraFit`, and the DOM-lane schemas.
- `🐍️interaction-matrix-probe.mjs` — **new**; all 8 examples × payload/selection-reset/hover/select/inspector/orbit/fit on published values.
- `🐍️page-fault-stack-probe.mjs` — **new**; captures `error.stack` and surrounding console for page faults.
- `🐍️fit-orbit-recon.mjs`, `🐍️fit-blocker-recon.mjs` — **new**; the recon that isolated §4.1 and §4.3.
- `🐍️journey-probe.mjs` — oracle-gated convergence, per-row `oracle`/`measured`/`meshReasons`, per-row required preview surface.
- `🐍️react-battery.mjs` — `interact` lane repointed at the matrix probe; `io-surface` verdict reads a real `ok`; central page-error/shell-fault gate; `customization-persistence` reads real counts.
- `🐍️io-surface-probe.mjs` — every row states its own verdict.
- `🐍️customization-persistence-probe.mjs` — real console/page-error listeners and a `console.txt`.

---

## 8. Not claimed

- That the two `⚠️` bbox rows in §2 are correct *to the fixture's own tolerance* — they are inside a
  stated LOD envelope, not inside `expect.boundingBoxTolerance`. The native `example-geometry` test was
  **not run** by this lane; the committed `delivery` extent that would make this exact is the follow-up.
- That this lane fixed the viewer-role 0-mesh gap, the `addEventListener` TypeError or
  `FlowMessageRejected` — all three were already absent when this lane first measured the current tree.
  What is new is that each is now asserted, so a return is a red row.
- That §4.4's stale selection is a guest bug or a host bug — the splitting evidence
  (`data-guest-selection-json`) is wired into `🐍️interaction-matrix-probe.mjs` but **never reached a
  completed run**: the run that would have captured it was aborted (§6). The same commit also makes the
  `orbit` hop drag AWAY from the polar clamp, which should retire the 4 `orbit` reds in §3 — those
  cameras had been walked onto the +Z pole by eight same-direction drags, where a further drag into the
  clamp is correctly a no-op. Both changes are **unmeasured**; re-run
  `bun 🐍️react-battery.mjs --only=interact` on a healthy stage to settle them.
- Anything about the wgpu renderer. Untouched by this lane.
- Any statement about `🗑️generated/react-verify/` — that is lane `react-current-tree-battery`'s
  evidence and was only read, never written.
