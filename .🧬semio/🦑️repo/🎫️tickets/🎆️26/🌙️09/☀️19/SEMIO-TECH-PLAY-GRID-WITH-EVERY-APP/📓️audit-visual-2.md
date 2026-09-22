# 📓️ Visual Audit #2 — Every Pane, Play Dev Server :6033 (2026-09-22, fresh 03:04 activation)

Read-only audit (topic `audit-visual-2`, no repo edits, no builds, no server restarts). Same method as
`📓️audit-visual.md` (2026-09-21): headless chromium via the repo's playwright
(`PLAYWRIGHT_BROWSERS_PATH=.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`, args `--use-angle=metal --enable-gpu
--ignore-gpu-blocklist --enable-unsafe-webgpu`, viewport 1440×900), one page at a time, fresh browser **and**
context per pane, against `http://127.0.0.1:6033/#<variant>`, waiting for `[data-shell-id]`
`data-shell-ready`/`data-shell-error`, 120 s shell-outcome timeout, 6 s post-ready settle, console + page-error
capture, and a regex over console+page-errors for `refused|panic|rejected|closure|Incomplete|authority is busy`
(case-insensitive). All **69 panes** of the current catalog (`🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime/🔣️.json`,
9 groups incl. the new `documents` group of 9 stdio panes) were probed against the 2026-09-22 03:04
`activate-dev` generation. Scratch: `$T/🗑️generated/audit-visual-2/` (`probe.mjs`, `panes.tsv`, `results.ndjson`
— resumable, one line per pane — `console/<variant>.txt`, `canvas-crops/<variant>.c<N>.png`,
`screenshots/<variant>.png`).

**Server**: answered HTTP 200 the whole run (~13 min, one full pass, no restarts, no interruptions). The served
page itself carries a `[stale] 2 staged plugin module(s)` warning for `animate` and `writer` (staged bytes older
than their current source) — noted, not acted on.

## Methodology change from audit #1 (important — read before trusting the `default-example-ok` column)

Audit #1's `default-example-ok` word-matched the catalog's kebab-case example **id** against the navbar
trigger's rendered **label** text, which audit #1's own retraction (`📓️default-example.md`) found unreliable
(`tower-stack` renders "Tower With A Cantilever"; `demo` renders "Reuse Map"). This run does **not** repeat
that mistake and does not need the label-resolution workaround either: Radix's `SelectItem` DOM carries
`data-value="<id>"` and `data-state="checked"` directly (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔽️Select/🟦️.tsx:723,725`),
so the probe opens the dropdown and reads **which option's raw id is checked**, then compares that id to the
catalog's curated id — id-to-id, never id-to-label. This is strictly more reliable than a label comparison and
should be preferred over both audit #1's original check and its label-based fix if this probe is reused.

**A second finding forced manual correction of the automated canvas-crop verdict for every pane**: the
crop-variance heuristic (which replaced audit #1's whole-shell-screenshot heuristic to avoid chrome-UI false
positives) still produces both false positives (a persistent ~1px red accent line at the top of every canvas
crop — present on blank AND populated panes alike — pushes variance just over the 15 threshold on an otherwise
truly blank canvas, e.g. `raster`, `architect`) and false negatives (small/sparse text — e.g. the stdio panes'
line-numbered source view — sits under the threshold despite being real content). **Every verdict below was
manually confirmed against its `canvas-crops/*.png`**, not taken from the automated field; the table's
`verdict` column reflects the manual read, and disagreements with the automated field are called out in `note`.

## Result

**66/69 panes reached `data-shell-ready`; 3 panes (`wfc2d`, `wfc3d`, `grid3d`) hit a NEW wasm-trap boot panic not
present in audit #1.** 1 pane (`playbook`) logs one new `refused` input (console `[warning]`, not `[error]`) that
also was not present in audit #1. Console-error counts collapsed to **0 on 68/69 panes** (was 16–220 on 44/60
panes in audit #1) — the console-spam fix (`📓️console-spam.md`, applied 2026-09-21 15:30) is confirmed live.
Of the 7 content defects audit #1 found: **2 are fixed and verified live** (`block3d`, `animate`), **3 are
unchanged — still broken, despite topic reports claiming the fix landed on disk** (`raster`, `architect`,
`reasoning-wires`), and **2 are unchanged and still expected/gated, not new** (`generation2d`, `mathematical`,
plus `demonstrator` which was already expected-empty). See the DIFF and Broken-panes sections below.

## Per-pane table

`errors` = raw Playwright console `[error]`-type message count. `refused/panics` = lines matching
`refused|panic|rejected|closure|Incomplete|authority is busy` across console + page errors (this run's one
`playbook` hit is a `[warning]`, included because it matches the regex, called out in its note).
`default-example-ok`: `OK` = the dropdown option whose raw `data-value` equals the catalog's curated `example`
id has `data-state="checked"` (id-to-id, see methodology above); `MISMATCH` = a different option is checked;
`n/a (no picker)` = gated (`PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES` in
`🏢️semio-tech/🎡️play/🧪️tests/🧪️playpanedefaults/🟦️.ts`) or the app declares no `setActiveExample`; `n/a` =
catalog declares no `example` for this pane.

| variant | outcome | ready-s | errors | refused/panics | verdict | default-example-ok | note |
|---|---|---|---|---|---|---|---|
| **design** | | | | | | | |
| cad | ready | 2.9 | 0 | 0 | content | OK | |
| generation3d | ready | 3.1 | 0 | 0 | content | OK | |
| generation2d | ready | 3.0 | 0 | 0 | **blank** | n/a (no picker) | unchanged from audit #1: canvas still shows only the "Empty canvas" placeholder text, crop-verified; gated, expected |
| flow | ready | 2.7 | 0 | 0 | content | n/a | crop-verified: Number(3.0)/Math.add nodes render |
| lowpoly | ready | 2.2 | 0 | 0 | content | n/a (no picker) | |
| remodel | ready | 2.3 | 0 | 0 | content | OK | |
| draw | ready | 2.0 | 0 | 0 | content | OK | |
| raster | ready | 2.5 | 0 | 0 | **blank** | OK | **unchanged from audit #1**: automated verdict said "content" (crop variance 29.85) but both crops are still fully blank beige — the variance came from a ~1px red UI accent line, not content. `📓️raster.md` reports the composite-canvas root cause "found and fixed" but explicitly flagged "NOT yet verified in the browser"; this run proves it is still not live |
| layout | ready | 2.5 | 0 | 0 | content | n/a (no picker) | |
| shooting | ready | 2.5 | 0 | 0 | content | OK | |
| **assembly** | | | | | | | |
| puzzle2d | ready | 2.7 | 0 | 0 | content | OK | |
| puzzle3d | ready | 4.1 | 0 | 0 | content | OK* | crop-verified real content (Concrete Forest assembly); *the dropdown-options read raced the popup animation and came back empty, so `checkedValue` is null — trigger text "Concrete Forest" matches the catalog id by word, so this is a probe timing gap, not a MISMATCH |
| puzzle5d | ready | 3.5 | 0 | 0 | content | OK | 3D viewport now shows a rendered base/slab (previously gizmo+grid only); a secondary board canvas still shows only a small sparse ring of capsules, same low-density content as audit #1 |
| block2d | ready | 2.7 | 0 | 0 | content | OK | no `<canvas>` (0) — 2D block editor is DOM/SVG-based, not a rendering gap |
| block3d | ready | 2.8 | 0 | 0 | **content — FIXED** | OK | crop-verified: real slab-on-columns geometry now renders, properly framed. Confirms `📓️block-puzzle.md`'s `block3d_world_fit_revision` fix is live |
| block5d | ready | 2.9 | 0 | 0 | content | OK | no `<canvas>` (0), same as block2d |
| **generative** | | | | | | | |
| wfc2d | **error** | 3.7 | 5 | 2 | **error — NEW** | n/a (boot failed) | wasm `unreachable` trap: guest panic "app-owned tool factories must preserve exact owner/controller/schema/tool authority" / `interactive-job.publication-contract`, `…/🔌️plugin/🦀️.rs:22508:68`. Not a page error before — audit #1 had this pane fully green |
| wfc3d | **error** | 3.4 | 5 | 3 | **error — NEW** | n/a (boot failed) | same wfc plugin crate, different panic: "tool proof catalog must exactly join migrated generated declarations to live concrete factories" / `interactive-job.catalog-incomplete`, `…/🔌️plugin/🦀️.rs:22512:18` |
| grid2d | ready | 3.2 | 0 | 0 | content | OK | same `🀄️wfc` plugin crate as wfc2d/wfc3d/grid3d, boots clean |
| grid3d | **error** | 2.9 | 5 | 2 | **error — NEW** | n/a (boot failed) | same crate, same panic signature as wfc2d (`…/🔌️plugin/🦀️.rs:22508:68`, publication-contract fault) |
| bitmap | ready | 5.7 | 0 | 0 | content | OK | same crate, boots clean |
| **engineering** | | | | | | | |
| fem2d | ready | 3.5 | 0 | 0 | content | OK | |
| fem3d | ready | 2.5 | 0 | 0 | content | OK | |
| energy | ready | 2.5 | 0 | 0 | content | OK | crop-verified: BESTEST 600 building volume with windows |
| process3d | ready | 4.5 | 0 | 0 | content | OK | crop-verified: beam/joint geometry |
| sourcing | ready | 2.8 | 0 | 0 | content | OK | 3D preview canvas itself is near-empty (perspective grid only) but 100 DOM text nodes / 1277 elements carry the real curation-table content — not a rendering gap |
| gis2d | ready | 6.9 | 0 | 0 | content | OK | picker trigger reads "Reuse Map" for curated id `demo` — confirms audit #1's retracted MISMATCH stays retracted, now via the robust id-to-id check (checked option's `data-value === "demo"`) |
| gis3d | ready | 3.2 | 0 | 0 | content | n/a (no picker) | crop-verified: real terrain mesh renders |
| architect | ready | 2.9 | 0 | 0 | **content-degraded** | n/a (no picker) | **unchanged from audit #1**: automated verdict said "content" (crop variance 17.3) but both crops still show only the stuck "Waiting !... !..." placeholder, crop-verified. `📓️knowledge-children.md` reports the derived-children fix landed but browser verification "NOT done yet" — this run proves it is still not live |
| **standards** | | | | | | | |
| din4108 | ready | 5.2 | 0 | 0 | content | n/a | |
| din16798 | ready | 3.0 | 0 | 0 | content | n/a | |
| din18599 | ready | 3.4 | 0 | 0 | content | n/a | |
| en1990 | ready | 3.2 | 0 | 0 | content | n/a | |
| en1991 | ready | 3.3 | 0 | 0 | content | n/a | |
| en1992 | ready | 3.4 | 0 | 0 | content | n/a | |
| en1993 | ready | 2.8 | 0 | 0 | content | n/a | |
| en1994 | ready | 3.0 | 0 | 0 | content | n/a | |
| en1995 | ready | 3.1 | 0 | 0 | content | n/a | |
| en1996 | ready | 3.0 | 0 | 0 | content | n/a | |
| en1997 | ready | 3.0 | 0 | 0 | content | n/a | |
| en1998 | ready | 3.0 | 0 | 0 | content | n/a | |
| en1999 | ready | 3.4 | 0 | 0 | content | n/a | |
| iso16757 | ready | 3.3 | 0 | 0 | content | n/a | |
| vdi3805 | ready | 2.9 | 0 | 0 | content | n/a | |
| **knowledge** | | | | | | | |
| note | ready | 2.7 | 0 | 0 | content | OK | |
| writer | ready | 2.6 | 0 | 0 | content | OK | crop-verified: real query text renders ("MATCH (a:Piece)-[r:Connection]->(b:Piece) WHERE a.name = 'core' RETURN a.name, b.name") — `📓️audit-followups.md` #5 had found the Jack editor window entirely blank (`text_len=0`) at 14:04 on 09-21; this pane now shows real content |
| forms | ready | 2.4 | 0 | 0 | content | OK | |
| mathematical | ready | 5.0 | 0 | 0 | **blank** | n/a (no picker) | unchanged from audit #1: crop confirmed still empty grid + bare cursor, gated, expected |
| reasoning-wires | ready | 3.7 | 0 | 0 | **blank** | OK | **unchanged from audit #1**: crop confirmed the canvas is still fully empty grid, no wires/nodes, despite picker correctly showing "Demo" selected. `📓️knowledge.md` §2.5 reports the demo-asset root cause "found and fixed" but explicitly says the fix needs a `reasoning-wires` describe + activation it did not run — this run proves it is still not live |
| dag | ready | 3.1 | 0 | 0 | content | n/a (no picker) | crop-verified: DAG node boxes render |
| imperative | ready | 4.0 | 0 | 0 | content | n/a (no picker) | |
| playbook | ready | 3.2 | 1 | **1 — NEW** | content | OK | one console `[warning]` (not `[error]`) matches the refused/panic regex: `input #1 setActiveExample refused: dispatch-failed … presence local read requires a live exact local retirement owner` — same defect class `📓️engineering.md` §3.3 documents for `imperative` (missing presence-retirement-factory override); audit #1 had 0 refused/panics for playbook |
| trinity-jack | ready | 4.2 | 0 | 0 | content | OK | |
| trinity-rewriting | ready | 3.1 | 0 | 0 | content | n/a (no picker) | |
| **documents** (new group since audit #1) | | | | | | | |
| stdio | ready | 3.2 | 0 | 0 | content | OK | automated verdict said "blank" (crop variance 8.57) — crop-verified WRONG: real markdown source renders (line-numbered: `# Title`, blockquote, code fence, etc.), just sparse relative to the 1440×900 crop |
| stdio-txt | ready | 4.2 | 0 | 0 | content | OK | automated "blank" overridden: crop shows real text "Hello, stdio.txt!" |
| stdio-csv | ready | 4.7 | 0 | 0 | content | OK | |
| stdio-tsv | ready | 3.9 | 0 | 0 | content | OK | |
| stdio-json | ready | 3.8 | 0 | 0 | content | OK | |
| stdio-json-i | ready | 3.5 | 0 | 0 | content | OK | |
| stdio-xml | ready | 4.0 | 0 | 0 | content | OK | |
| stdio-xml-valid | ready | 4.2 | 0 | 0 | content | OK | |
| stdio-html | ready | 4.3 | 0 | 0 | content | OK | automated "blank" overridden: crop shows real HTML source (`<!DOCTYPE html>`, `<p>Hello from the <b>semio</b> html demo.</p>`) |
| **media** | | | | | | | |
| animate | ready | 2.6 | 0 | 0 | **content — FIXED** | OK | crop-verified: real labelled tiles render (`tile-r0-c1`, `tile-r1-c0`, …) where audit #1 found only an empty grid. Confirms `📓️knowledge-children.md`'s animate demo-content fix is live (despite the page's own `[stale]` banner naming `animate` — the staged bytes evidently already carry this fix) |
| sequence | ready | 2.4 | 0 | 0 | content | n/a (no picker) | |
| vcs | ready | 2.9 | 0 | 0 | content | n/a (no picker) | |
| demonstrator | ready | 3.3 | 0 | 0 | blank | n/a | unchanged: canvas shows only the genesis title text "playground.playground", no curated example declared — expected empty state, not a defect |
| **workspace** | | | | | | | |
| home | ready | 2.4 | 0 | 0 | content | n/a (no picker) | |
| space | ready | 2.4 | 0 | 0 | content | n/a (no picker) | |

## DIFF against audit #1 (📓️audit-visual.md, 2026-09-21)

### Fixed and verified live (2)
- **`block3d`** — blank canvas → real geometry, correctly framed. `📓️block-puzzle.md`'s `block3d_world_fit_revision`
  fix is confirmed live.
- **`animate`** — empty tile grid → real labelled tiles. `📓️knowledge-children.md`'s animate demo-content fix is
  confirmed live.
- **Console-error spam** — audit #1 found 44/60 panes emitting a fixed 16+ bogus `console.error` lines per boot
  (a log-bridge severity bug, `📓️console-spam.md`). This run: **0 errors on 68/69 panes** (`puzzle3d` alone was
  220 in audit #1, now 0; `writer` 52→0; `fem3d` 51→0; `fem2d` 32→0). The applied fix
  (`patchPreview2ShimGuestLogLineRelease`) is confirmed live across the whole grid.
- **`wfc3d`/`gis2d` "wrong default example"** — audit #1's retraction (`📓️default-example.md`) already
  established these were false positives of a flawed id-vs-label check. This run's id-vs-id check reconfirms
  `gis2d` boots exactly the curated `demo` (rendered as "Reuse Map"). `wfc3d` itself now fails to boot at all
  (see regressions below), so its picker could not be re-checked this run.
- **`documents` group (9 stdio panes)** — did not exist in audit #1's 60-pane catalog. All 9 now boot clean (0
  errors, 0 refused/panics) with real per-format demo content, confirming `📓️play-stdio.md` and
  `📓️stdio-examples.md`.

### Still broken — unchanged despite topic reports claiming the fix landed on disk (3)
- **`raster`** — composite/navigator canvases are still fully blank. `📓️raster.md` names the root cause and a
  fix but flags explicitly that browser verification was not done because it needs the raster wasm lane
  rebuilt + activated; this run is direct evidence that re-activation has not (yet) happened, or did not pick
  up the fix.
- **`architect`** — still stuck on the "Waiting !... !..." placeholder. `📓️knowledge-children.md` reports the
  derived-children fix landed but "browser verification is NOT done yet"; same conclusion as raster.
- **`reasoning-wires`** — canvas is still fully empty (no wires/nodes). `📓️knowledge.md` §2.5 reports the root
  cause found and the demo asset regenerated, but explicitly did not request the `reasoning-wires` activation
  lane; same conclusion.

### Still empty — unchanged and expected/gated, not new defects (3)
- **`generation2d`** — still the "Empty canvas" placeholder (app declares no `setActiveExample`, gated).
- **`mathematical`** — still empty grid + bare cursor (same gate).
- **`demonstrator`** — still the bare genesis title, no curated example in the catalog by design.

### New regressions not present in audit #1 (4 panes)
- **`wfc2d`, `wfc3d`, `grid3d`** — all three now fail to reach `data-shell-ready` at all (wasm `unreachable`
  trap during app/editor registration). Audit #1 had these fully green (0 errors, 0 refused/panics, verdict
  content). The other two apps sharing the exact same `🀄️wfc` plugin crate build — `grid2d` and `bitmap` — boot
  fine, so this is not a whole-crate build failure but a per-app registration defect hitting 3 of 5 apps.
  Corroborating evidence found in `📓️design.md` §1: native testing of this same crate family hit a
  `semio-s-artifact-wfc-bitmap` **compile error** that blocked the `design` topic from ever getting a test result
  for wfc2d/wfc3d/grid2d/grid3d — i.e. this crate family was never actually fixed or verified by any fleet
  topic this session; it is a pre-existing, unaddressed gap that a fresh activation has now surfaced as a live
  boot failure for 3 of its 5 panes.
- **`playbook`** — one new refused input (`[warning]`, matches the audit's regex): `setActiveExample refused:
  dispatch-failed … presence local read requires a live exact local retirement owner`. Same defect class as
  the `imperative` presence-retirement-factory gap `📓️engineering.md` §3.3 already documents (missing
  `build_presence_local_root_retirement_factory` override) — not previously seen on `playbook` in audit #1.

### Methodology corrections in this run (not defects, but change how much to trust each field)
- Audit #1's `default-example-ok` compared the catalog's kebab-case id against the navbar trigger's rendered
  **label**, a comparison its own retraction called unreliable. This run compares the checked dropdown option's
  raw `data-value` id directly against the catalog id — id-to-id — which is strictly more reliable and needs no
  label-resolution step.
- The canvas-crop-variance heuristic (successor to audit #1's whole-shell-screenshot heuristic) still produces
  both false positives (`raster`, `architect` — a ~1px persistent UI accent line pushes variance over threshold
  on an otherwise blank canvas) and false negatives (all three `blank`-flagged `documents` panes — sparse
  line-numbered source text sits under the variance threshold despite being real content). Every verdict in
  this report was manually confirmed against its saved crop; none were taken from the automated field alone.

## Broken panes, grouped by symptom class

**1. Boot failure — wasm `unreachable` trap during app/editor registration (NEW, 3 panes, same plugin crate)**
- `wfc2d`, `grid3d` — identical fault: "app-owned tool factories must preserve exact owner/controller/schema/tool
  authority" (`interactive-job.publication-contract`), `…/🔌️plugin/🦀️.rs:22508:68`.
- `wfc3d` — different fault, same crate: "tool proof catalog must exactly join migrated generated declarations
  to live concrete factories" (`interactive-job.catalog-incomplete`), `…/🔌️plugin/🦀️.rs:22512:18`.
- `grid2d` and `bitmap`, built from the same `🀄️wfc` crate, boot clean — this is a per-app registration gap, not
  a whole-component build break. `📓️design.md` §1 records that native testing of this exact crate family never
  got past a `semio-s-artifact-wfc-bitmap` compile error this session, so none of wfc2d/wfc3d/grid2d/grid3d ever
  had a passing native test run to catch this — it reached the play grid unverified.

**2. Content still fully blank despite a topic report claiming the fix landed on disk (3 panes, all pre-existing
from audit #1, all needing a specific plugin's activation lane re-run that has not happened)**
- `raster` — `📓️raster.md` fix pending the raster wasm lane's re-activation.
- `architect` — `📓️knowledge-children.md` fix pending browser verification / re-activation.
- `reasoning-wires` — `📓️knowledge.md` fix pending the `reasoning-wires` describe + activation request.

**3. Content still empty, gated by design, not a new defect (3 panes, unchanged from audit #1)**
- `generation2d`, `mathematical` — editor declares no `setActiveExample`.
- `demonstrator` — catalog declares no curated `example`.

**4. New refused input (1 pane)**
- `playbook` — `setActiveExample` refused on boot, missing presence-retirement-factory override (same class as
  the already-documented `imperative` gap).

## What to re-run once fixed

- `raster`, `architect`, `reasoning-wires`: each needs its own plugin's `activate.request` honoured and a fresh
  `:6033` recycle, then a targeted re-probe of just those 3 panes.
- `wfc2d`, `wfc3d`, `grid3d`: needs the `semio-s-artifact-wfc-bitmap` compile error (`📓️design.md` §1) fixed and
  the whole `🀄️wfc` crate family run through native tests for the first time this session, before another play
  activation.
- `playbook`: needs a presence-retirement-factory override on `PlaybookApp`, mirroring whatever fix lands for
  `imperative` (`📓️engineering.md` §3.3).
