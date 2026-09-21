# 📓️ Visual Audit — Every Pane, Play Dev Server :6033 (2026-09-21, session 5)

Read-only audit (topic `audit-visual`, no repo edits). Headless chromium via the repo's playwright
(`PLAYWRIGHT_BROWSERS_PATH=.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, args `--use-angle=metal --enable-gpu
--ignore-gpu-blocklist --enable-unsafe-webgpu`, viewport 1440×900), one page at a time, against
`http://127.0.0.1:6033/#<variant>`, using the acceptance suite's own selectors (`[data-shell-id]`,
`data-shell-ready`/`data-shell-error`, 120 s shell-outcome timeout, 6 s post-ready settle). All 60 panes from the
catalog (`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`, 8 groups) were probed; `results.ndjson` has one line
per pane, resumable. Scratch: `$T/🗑️generated/audit-visual/` (probes, raw console captures under `console/`,
whole-shell screenshots under `screenshots/`, cropped canvas-element screenshots under `canvas-crops/` used to
calibrate/verify verdicts, `results.ndjson`, `example-results.ndjson`, `canvas-results.ndjson`, `final-merged.json`).

**Server**: the dev server was crash-looping on a stale-activation-lane error for ~8 minutes at session start
(`Stale play activation lane: flow is <sha> in demonstrator but <sha> in flow — run bun nx run
@semio-tech/framework-os-dev:activate-flow-react-dev`, `serve-6033-supervised.txt`), caused by concurrent peer
work on the flow plugin. Per the read-only/no-restart rule I only polled (curl) until the coordinator's own
supervisor produced a stable HTTP 200 at 14:00:23; no restart or fix was performed by this audit. From then on
the server answered normally for the whole run (~40 min, two full passes + spot-checks).

## Result

**60/60 panes reached `data-shell-ready`. 0 page errors, 0 console `[error]` lines matching
`refused|panic|rejected|closure|Incomplete|authority is busy` (cross-checked directly against every raw
`console/<variant>.txt` with `grep -liE`, independent of the per-pane computed field — zero files matched).**
This is a large improvement over the 2026-09-19 acceptance run 3 baseline in `📓️app-boot-defects.md` (10 apps
with refused/panic-class failures: cad, generation2d, process3d, architect, writer, animate, shooting, raster,
flow, block2d) — every one of those now shows 0 refused/panic lines here, consistent with the "Resolutions"
already logged in that file.

However, **content correctness is not the same as "no error"**: 7 panes have a real content defect found only by
visual inspection — 4 confirm the correct example is selected yet render a fully blank canvas (`raster`,
`block3d`, `reasoning-wires`, `animate`), 1 is stuck on a "Waiting" placeholder (`architect`), and 2 show the
wrong default example (`wfc3d`, `gis2d`) — plus 3 more panes are blank but that is architecturally expected
(gated: no `setActiveExample`, or no curated example declared at all) rather than a new defect. See Broken panes
below. Automated screenshot/canvas-variance verdicts were cross-checked by hand (cropped `<canvas>`-element
screenshots, not whole-shell screenshots, to avoid chrome-UI pixels producing false "content" positives — see
Methodology notes).

## Per-pane table

`errors` = raw Playwright `console` `[error]`-type message count (see note below — most of this is a logging
bug, not real breakage). `refused/panics` = lines matching `refused|panic|rejected|closure|Incomplete|authority
is busy` (regex from the ticket brief) across console + page errors — 0 for every pane. `default-example-ok`:
`OK` = navbar example-select trigger text matches the catalog's curated `example` id (word-wise, case-insensitive);
`MISMATCH` = trigger shows a different example; `n/a (no picker)` = the pane's editor declares no
`setActiveExample` action (confirmed against `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` in
`🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts`), so the picker never renders and this is expected, not a
new defect; `n/a` = catalog declares no `example` for this pane at all.

| variant | outcome | ready-s | errors | refused/panics | verdict | default-example-ok | note |
|---|---|---|---|---|---|---|---|
| **design** | | | | | | | |
| cad | ready | 20.8 | 16 | 0 | content | OK | |
| generation3d | ready | 28.3 | 16 | 0 | content | OK | |
| generation2d | ready | 19.2 | 16 | 0 | **blank** | n/a (no picker) | canvas literally renders the placeholder text "Empty canvas" (expected: editor declares no setActiveExample per the playpanedefaults gate, so the demo never loads) |
| flow | ready | 25.4 | 0 | 0 | content | n/a | catalog declares no `example` for flow |
| lowpoly | ready | 6.0 | 0 | 0 | content | n/a (no picker) | |
| remodel | ready | 7.4 | 16 | 0 | content | OK | |
| draw | ready | 16.5 | 16 | 0 | content | OK | |
| raster | ready | 13.6 | 16 | 0 | **blank** | OK | picker correctly shows "Demo" selected, but BOTH canvases (Composite + Navigator) render fully blank/uniform beige after ready+6s settle — confirmed with a direct `<canvas>`-element screenshot crop, not just the whole-shell screenshot. Matches the known follow-up already logged in `app-boot-defects.md` ("composite canvas shows no visible content for the demo in the browser pane") — still true today |
| layout | ready | 18.3 | 0 | 0 | content | n/a (no picker) | |
| shooting | ready | 10.1 | 16 | 0 | content | OK | |
| **assembly** | | | | | | | |
| puzzle2d | ready | 5.0 | 16 | 0 | content | OK | crop-verified: a secondary canvas shows the assembled Nakagin Capsule Tower chain |
| puzzle3d | ready | 5.7 | 220 | 0 | content | OK | high error count is almost entirely the logging-spam pattern (see below), repeated per registered brush mesh |
| puzzle5d | ready | 8.0 | 8 | 0 | content | OK | 3D viewport canvas is empty (gizmo+grid only) but a secondary board canvas shows a small ring of assembled capsules — real content but very sparse/small relative to the 1440×900 viewport; possible camera-framing issue, flagged with lower confidence |
| block2d | ready | 5.0 | 16 | 0 | content | OK | |
| block3d | ready | 5.2 | 16 | 0 | **blank** | OK | picker correctly shows the example selected, but the canvas crop shows only the camera gizmo + a handful of faint unlabelled markers — no visible block geometry from the curated example |
| block5d | ready | 12.3 | 16 | 0 | content | OK | |
| **generative** | | | | | | | |
| wfc2d | ready | 13.2 | 16 | 0 | content | OK | |
| wfc3d | ready | 11.9 | 16 | 0 | content | **MISMATCH** | picker shows "Tower With A Cantilever" instead of the catalog's curated `tower-stack` |
| grid2d | ready | 16.8 | 16 | 0 | content | OK | |
| grid3d | ready | 7.5 | 16 | 0 | content | OK | |
| bitmap | ready | 7.4 | 16 | 0 | content | OK | |
| **engineering** | | | | | | | |
| fem2d | ready | 5.3 | 32 | 0 | content | OK | |
| fem3d | ready | 5.6 | 51 | 0 | content | OK | |
| energy | ready | 6.1 | 16 | 0 | content | OK | |
| process3d | ready | 5.5 | 16 | 0 | content | OK | |
| sourcing | ready | 5.7 | 16 | 0 | content | OK | |
| gis2d | ready | 6.6 | 16 | 0 | content | **MISMATCH** | picker shows "Reuse Map" instead of the catalog's curated `demo` |
| gis3d | ready | 5.2 | 0 | 0 | content | n/a (no picker) | crop-verified: real terrain mesh renders |
| architect | ready | 3.6 | 16 | 0 | **content-degraded** | n/a (no picker) | BOTH canvases render a stuck "Waiting  !...  !..." placeholder instead of the curated demo content, confirmed by crop after ready+6s settle — gated (no setActiveExample) and `app-boot-defects.md` already notes "derived children load empty" as an open follow-up here |
| **standards** | | | | | | | |
| din4108 | ready | 5.5 | 0 | 0 | content | n/a | crop-verified: real JSON inputs + rule-check results table (19 rows) |
| din16798 | ready | 3.9 | 0 | 0 | content | n/a | |
| din18599 | ready | 3.8 | 0 | 0 | content | n/a | |
| en1990 | ready | 3.4 | 0 | 0 | content | n/a | |
| en1991 | ready | 4.6 | 0 | 0 | content | n/a | |
| en1992 | ready | 4.7 | 0 | 0 | content | n/a | |
| en1993 | ready | 5.2 | 0 | 0 | content | n/a | |
| en1994 | ready | 5.3 | 0 | 0 | content | n/a | |
| en1995 | ready | 5.5 | 0 | 0 | content | n/a | |
| en1996 | ready | 5.6 | 0 | 0 | content | n/a | |
| en1997 | ready | 4.7 | 0 | 0 | content | n/a | |
| en1998 | ready | 5.1 | 0 | 0 | content | n/a | |
| en1999 | ready | 4.4 | 0 | 0 | content | n/a | |
| iso16757 | ready | 4.3 | 0 | 0 | content | n/a | |
| vdi3805 | ready | 5.6 | 0 | 0 | content | n/a | |
| **knowledge** | | | | | | | |
| note | ready | 3.0 | 16 | 0 | content | OK | |
| writer | ready | 4.0 | 52 | 0 | content | OK | |
| forms | ready | 4.4 | 16 | 0 | content | OK | |
| mathematical | ready | 3.8 | 16 | 0 | **blank** | n/a (no picker) | canvas shows only the empty grid plus a single insertion-caret dot, no equation content (expected: gated, no setActiveExample) |
| reasoning-wires | ready | 5.8 | 16 | 0 | **blank** | OK | canvas shows only the empty background grid, no wire/node content, confirmed by crop |
| dag | ready | 5.2 | 16 | 0 | content | n/a (no picker) | crop-verified: real DAG node/edge listing renders |
| imperative | ready | 3.2 | 16 | 0 | content | n/a (no picker) | |
| playbook | ready | 5.0 | 16 | 0 | content | OK | |
| trinity-jack | ready | 4.1 | 16 | 0 | content | OK | |
| trinity-rewriting | ready | 5.1 | 16 | 0 | content | n/a (no picker) | |
| **media** | | | | | | | |
| animate | ready | 4.7 | 16 | 0 | **blank** | OK | canvas shows only the empty background grid, no timeline/scene content, confirmed by crop |
| sequence | ready | 5.2 | 16 | 0 | content | n/a (no picker) | |
| vcs | ready | 3.9 | 16 | 0 | content | n/a (no picker) | |
| demonstrator | ready | 4.6 | 0 | 0 | **blank** | n/a | canvas is empty (genesis playground); catalog defines no curated `example` for this pane, so an empty canvas may be by design rather than a defect |
| **workspace** | | | | | | | |
| home | ready | 2.9 | 0 | 0 | content | n/a (no picker) | crop-verified: "No studios yet. Create one from the navbar." — legitimate empty-state text |
| space | ready | 2.6 | 0 | 0 | content | n/a (no picker) | |

## Broken panes, grouped by symptom class

**1. Picker confirms the correct curated example is selected, but the canvas renders fully blank (real
content-rendering defect, no gating explanation — 4 panes)**
- `raster` — picker shows "Demo" correctly; both Composite and Navigator canvases are uniformly blank. Already a
  known open follow-up in `app-boot-defects.md`.
- `block3d` — picker shows "Hexagonal Cut Concrete Forest Left" correctly; canvas shows only the camera gizmo,
  no block geometry.
- `reasoning-wires` — picker shows "Demo" correctly; canvas shows only the empty background grid, no wires/nodes.
  Not in the `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` gate list, so this is **not** an expected gap.
- `animate` — picker shows "Demo" correctly; canvas shows only the empty background grid, no timeline/scene
  content. Also not in the gate list — **not** an expected gap.

These four are the same symptom: the app confirms it selected/loaded the right document, but nothing paints.
Worth investigating together as one class — possibly a shared render-pipeline or camera/viewport-framing issue
rather than four independent defects.

**2. Stuck "Waiting" placeholder instead of content (1 pane)**
- `architect` — both canvases show "Waiting  !...  !..." after ready+6s settle, never resolving to the demo.
  `architect` IS in the gate list (no `setActiveExample`, so the curated example can't be selected at all), and
  `app-boot-defects.md` already notes "derived children load empty" as an open follow-up here — this placeholder
  is consistent with that, i.e. likely the same root cause, now visually confirmed.

**3. Wrong default example shown (picker/example-resolution bug — 2 panes)** — ❌️ **RETRACTED 2026-09-21
by topic `default-example`, see `📓️default-example.md`.** Both panes boot EXACTLY the curated example;
this section's two `MISMATCH` verdicts are false positives of `default-example-ok`, which word-matched the
kebab-case example id against the navbar trigger's rendered LABEL. Labels are authored prose:
`tower-stack` renders "Tower With A Cantilever" and `demo` renders "Reuse Map". Verified in a fresh
browser context on :6033 — the curated row carries `data-state="checked"`, and for `wfc3d` it is the
THIRD published example, so `resolveBootExampleId`'s `exampleOptions[0]` fallback is ruled out.
- `wfc3d` — shows "Tower With A Cantilever" instead of the catalog's curated `tower-stack`.
- `gis2d` — shows "Reuse Map" instead of the catalog's curated `demo`.
- Both render *some* content (not blank), so this is a curated-default selection bug, not a boot failure —
  likely a stale localStorage/last-used example overriding `resolveBootExampleId`'s catalog default, but that
  was not instrumented/verified in this read-only audit.

**4. Gated panes with no picker, empty by design (expected, not a new defect — 3 panes)**
- `generation2d`, `mathematical` — editors declare no `setActiveExample` action (confirmed in
  `PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES`), so the curated catalog example never loads; canvas literally shows
  an "Empty canvas" / bare-cursor placeholder, which is the app's own genesis-document empty state, not a crash.
- `demonstrator` — catalog defines no curated `example` for this pane at all (`"example"` field absent), so an
  empty genesis playground may be the intended default state rather than a defect.

**5. Console `[error]`-type spam from a logging-severity bug (44/60 panes affected, not a rendering defect)**
Nearly every canvas-based pane logs a recurring diagnostic (`typed-operation slots instance=…`,
`<app>.utility.publish action=registerBrushMesh …`, etc.) where the FIRST fragment goes to `console.debug`
(prefixed `[DEBUG] `, correctly filtered by the acceptance suite's `significantConsoleErrors`) but every
subsequent interpolated argument of the *same* logical message is logged separately via `console.error` with no
prefix (e.g. `[error] 1`, `[error]  live=`, `[error] 1`, `[error] /`, `[error] 64`, `[error]  peak=`, `[error] 1`).
This produces a fixed baseline of exactly 16 "errors" per pane that hits it (two instances of an 8-fragment
message), and scales up with repeated events (puzzle3d 220, writer 52, fem3d 51, fem2d 32 — repeated
`registerBrushMesh`/similar calls). **This is not functional breakage** (0 refused/panics, 0 page errors, verdict
still `content` where content genuinely renders) but it **would fail the strict acceptance suite** on every one
of the 44 affected panes, since none of these fragments start with `[DEBUG] ` or match the 40x-resource-error
filter. Evidence: `$T/🗑️generated/audit-visual/console/<variant>.txt` for any of the 44 (e.g. `cad.txt` lines
63–80, `puzzle3d.txt` lines 63–102).

## Methodology notes / caveats

- **Canvas content verdicts required manual correction.** A first-pass heuristic (whole-shell screenshot pixel
  variance via sharp) produced false "content" positives for panes whose canvas is actually blank, because
  floating chrome UI (toolbar buttons, grid-line backgrounds) contributes enough pixel variance on its own. I
  re-checked with `<canvas>`-element screenshots (`element.screenshot()`, which still captures whatever the
  browser paints in that screen region, including any DOM overlaid on the canvas, but not the surrounding
  chrome) and manually inspected the crops for every pane whose canvas variance sat in the ambiguous 5–30 range
  (calibrated against known-good crops like `cad` and known-blank crops like `raster`). Crops are saved in
  `$T/🗑️generated/audit-visual/canvas-crops/` for `cad`, `raster`, `animate`, `demonstrator`, `reasoning-wires`,
  `block3d`, `generation2d`, `puzzle2d`, `dag`, `mathematical`, `architect`, `flow`, `puzzle5d`, `gis3d`.
  In-page `getContext('2d').drawImage(canvas,...)` sampling of the live WebGL/WebGPU canvases (as the brief
  suggested) always returned variance 0 for every pane, expectedly — these canvases are not
  `preserveDrawingBuffer`, so a same-page `drawImage` read-back is blank regardless of what's on screen; that
  signal was discarded in favor of the screenshot-crop method.
- The 15 standards panes (`din*`/`en*`/`iso16757`/`vdi3805`) and `home`/`space` render no `<canvas>` at all
  (DOM/JSON-table UI); I visually confirmed `din4108` and `home` render real content and extended that
  confidence to the rest of the standards group (identical app shape, identical `errorCount=0`/`readySeconds`
  pattern) rather than crop-checking all 15 individually.
- `default-example-ok` uses the navbar trigger's rendered label text (`#playground.navbar.fixture`), matched
  word-wise against the catalog's kebab-case `example` id — e.g. `hexagonal-mushroom-column` →
  "Hexagonal Mushroom Column". This needed a 6 s settle after shell-ready (a 3 s settle produced one false
  negative for `cad`, corrected by re-running with 6 s).
- Two full sequential passes were run (main content/error probe, then a lighter example-picker-only probe, then
  a canvas-crop verification pass) — all one page at a time, never concurrent, per the brief. The dev server
  answered normally throughout except for the pre-run crash loop described above; no restart was requested or
  performed.
- I did not run the play unit suite or the strict `bun nx run @semio-tech/semio-tech-play:test-e2e` acceptance
  suite myself (out of scope for this read-only visual audit) — the console-error-spam finding above predicts it
  would currently fail on ~44/60 panes for a reason unrelated to rendering correctness.
