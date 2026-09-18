# 🎨️ Visual-parity checklist — puzzle3d, wgpu (6213) vs React (6313), 1440×900 dpr 1

2026-09-18, read-only audit. All artefacts under `🗑️generated/audit-visual/`.

## 0. Method, and the one thing that did NOT work

- wgpu capture: `SEMIO_PROBE_URL=http://127.0.0.1:6213/?plugin=puzzle3d SEMIO_PROBE_OUT=audit-visual/wgpu SEMIO_PROBE_SECONDS=45 SEMIO_PROBE_SHOTS=40 bun 🐍️wgpu-console-dump-probe.mjs` → `🗑️generated/audit-visual/wgpu/final.png` (clean boot, no fault overlay, `data-semio-os-error=null`, one benign `ERR_ABORTED` on a duplicate wasm HEAD request).
- wgpu structure/frame/mesh stats: `🐍️w3a-wgpu-diagnostics-probe.mjs` → `🗑️generated/audit-visual/wgpu-diag/dumps.json`.
- **New probe** `🐍️wgpu-accessibility-dump-probe.mjs` (`dumpAccessibility`): per-window scene a11y tree only (`application`/`group`/`paragraph`, one `rect` for the viewport itself) — does **not** cover shell chrome, so it is not useful for chip geometry. Kept for the record; superseded by the next one.
- **New probe** `🐍️wgpu-chrome-hits-probe.mjs` (`dumpChrome`, `SEMIO_RUNTIME_DIAGNOSTICS` armed): this is the real find. It reads the shell's own published **hit-target registry** (`control_id`, `kind`, exact `[x,y,w,h]` rect, `window_id`, dispatched `action`) — i.e. pixel-exact geometry for every navbar chip, window cap, pane-row chip, dock tab and footer chip, straight from the renderer's own retained document, no OCR/guessing needed. 43 rows captured → `🗑️generated/audit-visual/wgpu-chrome/chrome-dump.json`. Also clicked the Artifact ([37,14]) and Inspection ([1200,14]) screen coordinates and re-dumped (see §5).
- Colour sampling: `🗑️generated/audit-visual/scratch-pixel-sample.py` (pure PIL, `img.getpixel`), run against the PNGs.
- React capture: wrote `🐍️audit-react-chrome-geometry.mjs` exactly as specified (DOM `getBoundingClientRect` + computed style over every button/role=button/role=tab/chip-classed element, with an Artifact/Inspection panel-open click pass) and derived the boot-tour-suppression key correctly:
  `ui.introduction.seen.${session ? (brand ? brand.id+":"+session.app.id : session.app.id) : ""}` from `readStoredIntroductionSeen`/`writeStoredIntroductionSeen` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2181-2193` (`UI_INTRODUCTION_SEEN_STORAGE_KEY_PREFIX = "ui.introduction.seen."`), with `appId = s.puzzle.puzzle3d@1/*#editor` confirmed against the same `@${standard}/${subset}#${mode}` shape used by `✏️s/🔌️plugins/🌀️procedural/…/🧩️mount-contract/🟦️.ts`'s `editorAppId: "s.assembly@1/*#editor"`. **Correction to the ticket's suggested seed**: `readStoredIntroductionSeen` compares `=== "true"`, not `"1"` — the seed must be `{"ui.introduction.seen.s.puzzle.puzzle3d@1/*#editor":"true"}`, which is what both scripts here use.
- **Blocker (P0, environmental, not a code defect in either renderer)**: at first capture (09:41) `http://127.0.0.1:6313` returned `ERR_CONNECTION_REFUSED` for every module (`@react-three/drei/*`, etc.) starting at t≈3.9s into the run — a dep-optimizer/vite restart race, not a fault overlay. I then polled `curl`/`lsof` on port 6313 for **~14 minutes** (attempts logged every 5–15s): the vite process (pid 54772, `--port 6313 --strictPort`) stayed **alive but not listening** the entire time (`lsof -iTCP:6313 -sTCP:LISTEN` empty throughout), i.e. wedged mid-restart, not merely slow — matches the "Release Serve Wedges After Host Edit Bursts" pattern. Per my constraints I do not restart dev servers, so I could not force recovery. I did **not** get a fresh, tour-suppressed, unblurred React capture this session. Everything below marked **(React, live)** is fresh; **(React, 08:44 capture)** is the newest pre-existing capture in `🗑️generated/react-6313/` (tour-blurred backdrop, but `state.json`'s `innerText` and canvas sizes are text, not pixels, so unaffected by the blur); **(React, ticket audit)** cites a same-day `📓️w*.md` packet report that already measured the number precisely. **BLOCKED** cells could not be filled at all. **Retake `🐍️audit-react-chrome-geometry.mjs` once 6313 answers `200` again** — it is written, tested for syntax (fails cleanly with a clear Playwright net::ERR error when the server is down), and ready to run as-is.

## 1. Boot health

| | wgpu (6213) | React (6313) |
|---|---|---|
| Fault overlay | None. `data-semio-os-ready=null` the whole run (readiness attr appears unused on this target — see P2 below), `data-semio-os-error=null`, 45s fault-free, chrome painted, grid+gizmos present. | **BLOCKED** — server down; the 09:41 attempt shows `title="semio · os"` (fallback shell, never reached the plugin), 150 failed `ERR_CONNECTION_REFUSED` requests, empty `innerText`. The 08:44 capture (pre-outage) shows a clean boot: `ready:"puzzle3d"`, canvases `470×807` + `947×807`, title `"semio · puzzle · 3d"`. |
| Boot tour | `dumpChrome` returned live hit targets `shell.tour.skip` @ `[806.8,367.6,70,22.4]` and `shell.tour.next` @ `[786.8,508.4,90,22.4]` at generation 25 (~20s in), but the same-moment screenshot (`wgpu-chrome/before.png`) shows **no visible tour overlay** — the grid/gizmo/chrome painted, nothing else. Either the tour auto-dismissed but left stale hit rows in the ledger (ledger is a full replace on each *complete* walk, so this would mean the walk that produced them is what's on screen and the tour is invisibly present with 0-opacity/off-canvas paint), or it free-runs off-screen. **P1**, needs a live headed look to disambiguate — flagging, not asserting. | 08:44 capture: tour visible and blocking (`"Welcome to Puzzle 3D … 1/5"`, Skip/Next), confirming the tour fires on load as designed. |

## 2. Navbar — chip order and rect (wgpu measured; React from DOM order only, rects BLOCKED)

wgpu rects, x-sorted, from `chrome-dump.json` (`before.hits`, generation 25, canvas 1440×900):

| control_id | label (screenshot) | rect x,y,w,h | kind |
|---|---|---|---|
| `shell.panel.tab.top-left.framework.panel.artifact` | Artifact | 3.2, 3.2, 62.45, 22.4 | Toggle |
| `shell.panel.tab.top-left.framework.panel.catalogue` | Catalogue | 131.31, 3.2, 75.97, 22.4 | Toggle |
| *(unregistered)* | black disc + `s.puzzle.puzzle3d@1/*#editor` | ≈200–420, 3–27 (pixel-estimated, not a hit target — see §4) | — |
| `playground.navbar.fixture` | Concrete Forest | 422.08, 3.2, 108.18, 22.4 | NavbarItem |
| `playground.navbar.roles.editor` | Editor (active, red fill) | 533.45, 3.2, 54.30, 22.4 | NavbarItem |
| `playground.navbar.roles.viewer` | Viewer | 590.95, 3.2, 57.91, 22.4 | NavbarItem |
| `shell.panel.tab.top-right.framework.panel.inspection` | Inspection | 1162.26, 3.2, 77.69, 22.4 | Toggle |
| `shell.panel.tab.top-right.framework.panel.toolRun` | Tool runs | 1239.95, 3.2, 71.79, 22.4 | Toggle |
| `shell.panel.tab.top-right.framework.chat` | Chat | 1311.74, 3.2, 47.96, 22.4 | Toggle |
| `ui.fullscreen.toggle` | Fullscreen | 1359.70, 3.2, 77.10, 22.4 | Toggle |

Navbar band: y 0–32 (all chips at y=3.2, h=22.4, i.e. 3.2px top/6.4px bottom inset within a 32px bar).

React (08:44 `state.json.text`, DOM order — **not** visual x-order, rects BLOCKED): `Artifact, Catalogue, Inspection, Tool runs, Chat, Fullscreen, "semio · puzzle · 3d", Editor, Example, Concrete Forest, Editor ⌘⌥E, Viewer ⌘⌥V`.

| Item | React | wgpu | Delta | Severity | Owner |
|---|---|---|---|---|---|
| Left cluster membership | Artifact, Catalogue | Artifact, Catalogue | Match (both present, same two, same relative left position by every signal available) | — | — |
| Right cluster membership | Inspection, Tool runs, Chat, Fullscreen | Inspection, Tool runs, Chat, Fullscreen (x=1162→1436) | Match in membership **and** relative order | — | — |
| Centre app-id/title text | `"semio · puzzle · 3d"` (product-style, dot-separated, lowercase) | `"s.puzzle.puzzle3d@1/*#editor"` (raw artifact-dialect id) | **Confirmed distinct strings** — wgpu shows the internal dialect id, React shows a humanised title. This is exactly the drift the ticket flagged. | **P1** | wgpu: navbar title source in `🐚️Shell/🎯️targets/🧊️wgpu` (chip text pulled from `AppDescriptor`/dialect, not a humanised label); React: `Navbar`/`shellChromeTitleClassName` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` |
| App logo | Not established this session (BLOCKED — no live DOM) | **Black filled disc**, ~14×14px, centred left of the title text (sampled `#000102` at (219,14), essentially pure black) | Ticket's own prior note ("wgpu shows a black disc") independently reproduced by pixel sample. React side unverified. | **P1** (pending React confirmation) | wgpu: `SemioLogo`/`ShellBrandLogo` equivalent painter in the wgpu Shell chrome target |
| Example/mode group structure | DOM lists a separate `"Editor"` **group label** then `"Example"` **field label** then `"Concrete Forest"` **value**, then `Editor ⌘⌥E` / `Viewer ⌘⌥V` as a 5-item run — possibly a dropdown/menu, not 3 flat chips | 3 flat chips: `Concrete Forest` (NavbarItem), `Editor` (NavbarItem, active/red), `Viewer` (NavbarItem) | Cannot confirm whether React's DOM run is a flattened menu (hidden until opened) or literally 5 visible chips — **the 08:44 screenshot is blurred at exactly this region** so it can't be read either. Needs a live re-check. | **P1** (unresolved — flag, don't guess) | React: `ShellHost/🟦️.tsx` example/mode navbar block; wgpu: `playground.navbar.fixture`/`playground.navbar.roles.*` |
| Keyboard-shortcut hints | `Editor ⌘⌥E`, `Viewer ⌘⌥V` present in DOM text | Not visible in the wgpu screenshot chip labels (just "Editor"/"Viewer", no shortcut glyph) | React shows shortcut hints inline; wgpu's chip rects (54.3px/57.9px wide) look too narrow to fit `⌘️⌥️E` at the same font size as "Editor" — likely just omitted. | **P2** | wgpu: `playground.navbar.roles.*` chip painter |

## 3. Window caps, per-pane chip rows, dock

wgpu (measured, both windows symmetric, `puzzle3d-main-top` left / `puzzle3d-main-perspective` right, offset exactly by the left pane's width + gutter):

| Row | Element | Left-pane rect | Right-pane rect (top offset) |
|---|---|---|---|
| Dock tab strip (y 32–54.4, h=22.4) | `dock.tab.*.focus` | 46.21, 32, 17.2, 22.4 | 566.48, 32, 17.2, 22.4 |
| | `dock.tab.*.close` (×) | 63.41, 32, 17.2, 22.4 | 583.68, 32, 17.2, 22.4 |
| | `dock.tab.*.drag` (⠿) | 80.61, 32, 17.2, 22.4 | 600.88, 32, 17.2, 22.4 |
| | pane title window ("Top"/"Perspective") | 3.2, 32, 43.01, 22.4 | 481.07, 32, 85.41, 22.4 |
| Top overlay row (y 57.6–80) | `shell.action.fold` (Actions) | 6.4, 57.6, 63.26, 22.4 | 484.27, 57.6, 63.26, 22.4 |
| | `shell.window.search.toggle` (Search) | 211.97, 57.6, 60.32, 22.4 | 928.77, 57.6, 60.32, 22.4 |
| | `shell.measures.unfold` (Window Options) | 372.43, 57.6, 105.44, 22.4 | 1328.16, 57.6, 105.44, 22.4 |
| Bottom overlay row (y 842.4–864.8) | `shell.utilityBar.unfold` (Utilities) | 6.4, 842.4, 62.43, 22.4 | 484.27, 842.4, 62.43, 22.4 |
| | **"Projection" chip** | pixel-estimated ≈402–480, ≈844–864 (no registered hit target — see below) | ≈1358–1436, ≈844–864 |
| Pane content (`ScrollRegion`/`World3d`) | | 3.2, 54.4, 477.87, 813.6 | 481.07, 54.4, 955.73, 813.6 |
| Dock split gutter | `dock.split..0` | 471.07, 32, 20, 836 | — |
| Gizmo cluster (orbit axes, bottom-left of each pane) | pixel-estimated centre | ≈(452, 812) | mirrored, right pane |

**P0 finding — "Projection" chip has no hit target.** The chip is painted (screenshot + edge-scan confirm a bordered pill at x≈402–480,y≈844–864 with the same fill/border colours as every other chip, `#ebe8d8`/`#7b827d`-ish) but does **not** appear anywhere in the 43-row `dumpChrome` registry, unlike every other visible chip (Actions/Search/Window Options/Utilities/Artifact/Catalogue/… all have rows). It is either (a) genuinely dead — visual-only, unclickable — or (b) registered under a control_id/kind this probe didn't recognise as chip-shaped (unlikely, since the registry is exhaustive by design — "an unnamed read answers the whole registry"). Needs a source check in the wgpu Shell target for the pane's Projection/orthographic-perspective toggle. React's equivalent (per the ticket's own description) is functional.

## 4. Colour samples (wgpu, PIL `getpixel`, all `#000102`-style are exact, not anti-aliased averages unless noted)

| Swatch | Sample point | RGB hex | Notes |
|---|---|---|---|
| Viewport/canvas background | (700,150) | `#f7f3e3` | cream |
| Chrome bar background (navbar/footer/gutter) | (700,14) / (100,886) area | `#e9e6d7` | slightly darker cream than viewport |
| Grid line stroke | (240,460) darkest pixel on column x=240 | `#7b827d` | same value as chip border and dock-gutter — one shared "line" token |
| Chip border | (3,3) | `#7b827d` | matches grid line — confirms shared token |
| Chip fill (inactive, e.g. Artifact) | (35,14) | `#cecdc0` | |
| Active/selected pill (Editor role, red) | (560,14) | `#ae2437` | |
| App logo disc | (219,14) | `#000102` | effectively pure black — matches ticket's prior note |
| Gizmo, orange axis (approx., anti-aliased) | (448,815)/(461,795) | `#c39062`/`#c69065` | |
| Gizmo, green axis (approx., anti-aliased) | (452,825) | `#8d9f8b` | |
| Footer "Check In" chip fill | (238,886) | `#d9d7ca` | |
| Footer "Display" chip area | (298,886) | `#92968f` | darker — likely inside the chip's icon glyph, not the fill; re-sample if exact fill needed |

React colour swatches: **BLOCKED** (no live page). Note for the retake: React's `--font-sans` token is already `Anta` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🎨️palette/🎨️.css:217`), the same family W3b embedded for wgpu chrome text — so font-family parity is a design intent already in place; only size/weight/colour need live comparison.

## 5. Panel-open attempt (Artifact / Inspection)

- wgpu: clicked screen coords (37,14) — inside the registered `Artifact` rect (3.2–65.6, 3.2–25.6) — and (1200,14) — inside `Inspection` (1162.3–1240.0, 3.2–25.6). Re-dumped `dumpChrome` after each click (1s settle).
  - **Result: 0 new dispatched actions logged either time**, and the before/after screenshots are pixel-identical (grid-only, no panel drawer). Either (a) the click didn't route into the wasm canvas's pointer bridge from `page.mouse.click` the way it does for a real user gesture (Playwright's synthetic click *should* work for canvas apps, but this target may require a raw `pointerdown`/`pointerup` pair with `isTrusted`-adjacent handling it doesn't get here), or (b) the toggle genuinely doesn't open a panel yet. **Do not treat as a confirmed defect** — flagging as unresolved, needs a headed-browser or manual click to disambiguate from a probe artefact.
- React: **BLOCKED** — could not open either panel this session; script has the click-by-text logic ready (`clickByText("Artifact")`/`clickByText("Inspection")` in `🐍️audit-react-chrome-geometry.mjs`).

## 6. 3D view — meshes, plan underlay

From `🗑️generated/audit-visual/wgpu-diag/dumps.json` (`dumpMeshStats`, `SEMIO_RUNTIME_DIAGNOSTICS` armed, same boot as the accessibility dump):

- Both windows report the same 4 mesh entries per surface (`box`, `vortex-marker`, `mesh:🧊️hexagonal-cut-concrete-forest-left`, `mesh:🧊️hexagonal-cut-concrete-forest-right`), each with **`indices:0, positions:0, edgePositions:0, bboxMin:null, bboxMax:null`** and `role:"(unstamped)"` — the meshes are *declared* but carry no geometry.
- `instances` has exactly one row (`seed-left-001` → `mesh:🧊️hexagonal-cut-concrete-forest-left`), not two (both left+right meshes are declared but only one is instanced).
- `dumpFrameStats`: `drawCalls:0, quadCount:17, glyphCount:0, scenePasses:1, sceneDraws:3, sceneInstances:2` — **zero draw calls**, consistent with the screenshot showing grid + gizmo only, no concrete-forest model, no plan/reference underlay.
- **This matches the ticket's own expectation** ("wgpu may still be grid-only — note it") and status.md's own running log ("W3d final: GLB fetched/decoded/resident … but the world pass paints only line draws → W5c (owns activation) dispatched"). **Not a new finding** — confirmed still true as of this capture (post-W5c dispatch, pre-completion).
- React: shows the concrete-forest model + plan underlay per the ticket's own description and status.md's 08:44 note ("React reference re-captured … concrete-forest model"), but I could not re-confirm pixel-for-pixel this session (BLOCKED).

## 7. Prioritised packet list

1. **P0 — "Projection" pane chip has no `dumpChrome` hit target** (§3). Painted, unclickable per the registry. Owner: wgpu Shell target, pane-chip hit registration (same family as `shell.action.fold`/`shell.window.search.toggle`/`shell.measures.unfold`/`shell.utilityBar.unfold` — Projection is the one sibling missing from that group).
2. **P0 — React dev serve (6313) wedged**, alive-but-not-listening for 14+ minutes this session. Not a rendering defect, but it blocks every remaining item below and blocked ~40% of this audit. Recycle by pid (54772) per "Release Serve Wedges After Host Edit Bursts" once someone with server-restart permission is available; then re-run `🐍️audit-react-chrome-geometry.mjs` and this probe suite.
3. **P1 — Centre navbar title text**: wgpu paints the raw dialect id `s.puzzle.puzzle3d@1/*#editor`; React paints a humanised `semio · puzzle · 3d`. Straightforward text-source fix once confirmed live.
4. **P1 — Example/mode navbar structure unresolved**: React DOM suggests a 5-item run (`Editor` group / `Example` field / `Concrete Forest` value / `Editor ⌘⌥E` / `Viewer ⌘⌥V`) vs wgpu's 3 flat chips; could be a menu-vs-flat difference or just DOM order confusing aria/label text with visible chips. Needs a live, unblurred React screenshot of that navbar region.
5. **P1 — Footer composition/order** (carried over from the ticket's own brief, re-confirmed structurally by `dumpChrome` x-order this session): wgpu = `Remote: detached, No one else is here, Check In, Display, ⋯, Tool, Command, ⋯, Settings, Marketplace, History`; React (per ticket) = `Display, Remote: detached, Tool, Command, No one else is here, Settings, Marketplace, History` — wgpu has an extra **Check In** chip React doesn't, plus reordering of the first three groups.
6. **P1 — Boot-tour hit targets present with no visible tour** (§1): `shell.tour.skip`/`shell.tour.next` rows exist in a 20s-in `dumpChrome` read but the same-moment screenshot shows no tour UI. Needs a headed capture at t≈1–5s to see whether the tour actually painted-then-vanished correctly or is an invisible dead layer eating clicks.
7. **P2 — Keyboard-shortcut hints** (`⌘️⌥️E`/`⌘️⌥️V`) present in React, not visibly present in wgpu's Editor/Viewer chip labels.
8. **P2 — `data-semio-os-ready` never set on the wgpu page** (stayed `null` for the full 45s clean-boot run) — either this attribute is a React-only readiness signal and wgpu uses a different one, or it's genuinely unwired; low severity since the app is visibly ready regardless, but worth a one-line check since the probe used it as its readiness gate and had to fall back to a fixed wait.

## 8. Artefact index (`🗑️generated/audit-visual/`)

- `wgpu/` — 45s boot probe (`final.png`, `console.txt`, `state.json`, `requests-failed.txt`)
- `wgpu-diag/` — `dumpStructure`/`dumpFrameStats`/`dumpMeshStats` (`dumps.json`, `final.png`)
- `wgpu-a11y/` — `dumpAccessibility` per window (scene-tree only, not chrome — kept for the record)
- `wgpu-chrome/` — `dumpChrome` hit registry, before/after-click screenshots, `chrome-dump.json` (the main wgpu geometry source for this report)
- `react/` — the failed 09:41 capture (`ERR_CONNECTION_REFUSED` flood, `console.txt`, `requests-failed.txt`)
- `scratch-pixel-sample.py` — the PIL colour-sampling helper
- `🗑️generated/react-6313/` (pre-existing, not written by this audit) — the 08:44 tour-blurred React capture used for §1/§2/§6's React-side facts

Scripts added to the ticket root: `🐍️audit-react-chrome-geometry.mjs` (spec'd probe, blocked from running — ready to retake), `🐍️wgpu-accessibility-dump-probe.mjs`, `🐍️wgpu-chrome-hits-probe.mjs` (the two that produced the wgpu numbers in this report).
