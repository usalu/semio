# 📓️ Visual Audit #3 — Every Pane, Play Dev Server :6033 (2026-09-23, 03:23 full activation)

Read-only audit (topic `audit-visual-3`, no repo edits outside this file and its scratch dir, no builds, no
server restarts). The predecessor `audit-visual-3` session that started this ticket file never delivered a
probe result (an external sweep wiped its scratch mid-run at 16:34 on 09-22, then a 40-minute machine-overload
wait was still running when it stopped); this is a full fresh run, superseding that incident log below.

**Method** (mirrors `📓️audit-visual-2.md`'s navigation/console/screenshot method, but reads paint the way the
strict acceptance suite now does — see `📓️play-runtime.md` §"the canvas census was a false green"): headless
Chromium via the repo's Playwright (`PLAYWRIGHT_BROWSERS_PATH=.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright`,
args `--use-angle=metal --enable-gpu --ignore-gpu-blocklist --enable-unsafe-webgpu`, viewport 1440×900), one
page at a time, fresh browser **and** context per pane, against `http://127.0.0.1:6033/#<variant>`. For each
pane: wait for `[data-shell-id]` `data-shell-ready`/`data-shell-error`/`data-shell-not-found` (120 s timeout),
`networkidle` (15 s) then a 3 s settle, console + page-error capture, a regex over console+page-errors for
`refused|panic|rejected|closure|Incomplete|authority is busy`, the exact `readPaintWitnesses`/`paintWitness`
algorithm ported verbatim from `🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts` (per-window: `data-meshes-json`/
`data-instances-json` on 3D world surfaces, `data-layers-json`/`data-assets-json` + `MINIMUM_IMAGE_ASSET_EDGE=8`
on raster/paint2d surfaces, text/element count on plain DOM `[data-slot="window-body"]` windows, 4 s poll at
250 ms), the curated-example chrome label (`#playground.navbar.fixture`, id-to-label not id-to-id), every
distinct `/mesh/*.glb` request path (puzzle5d wrong-document check), any media-extension asset answered
`text/html` (SPA-fallback check), and a full-shell screenshot. Scratch:
`.🧬semio/🦑️repo/⚡️cache/play-fleet/audit-visual-3/` (`probe.mjs`, `panes.tsv`, `run-batch.sh`,
`results.ndjson`, `console/<variant>.txt`, `screenshots/<variant>.png`, `topic-map.json`).

**Important methodology caveat found this run**: the DOM-fallback branch of the paint witness (used whenever
a window has no `[data-surface-id]`) counts ANY non-empty text as "painted" — including a lone toolbar
button's label ("Actions", 17 characters). Several genuinely-empty 2D canvas windows (`reasoning-wires`,
`mathematical` before its fix, `demonstrator`) therefore witness as `painted=true` purely from chrome text,
**not** from their canvas content. Every pane whose witness showed this 17-character chrome-only signature
(16 panes) was manually re-checked against its saved screenshot; disagreements are called out per-row below.
This is the same class of blind spot audit-visual-2 found in the canvas-crop-variance heuristic it replaced —
the witness is more precise for windows that publish a surface (3D worlds, raster) but is a liveness check,
not a content check, for everything else.

## Server / activation

:6033 answered HTTP 200 throughout the run, serving the **03:23** full activation (`218` tasks, 02:42→03:23,
described in `📓️status.md`: "framework decoder fix, raster real emblem, flow drivers, gis descriptor etc. all
baked"). No newer `describe-activate-*` log exists in `⚡️cache/play-fleet/activation/` as of this run's
completion, so this probe and the 03:37 strict-acceptance run (`📓️acceptance-runs.md`) are the same build.

## Acceptance-suite cross-check (independent confirmation)

The direct-Playwright strict acceptance suite on this exact activation
(`⚡️cache/play-fleet/e2e/test-e2e-direct-0330.txt`, `📓️acceptance-runs.md`) scored **67/70** (67 pass, 3 fail):
`generation2d`, `raster`, `gis3d`. This audit's independent, separately-written probe — same activation, same
witness algorithm, different script — reproduces **exactly the same 3 panes failing for exactly the same
reasons** (refused input on `generation2d` and `raster`, shell error on `gis3d`; see rows below), and 0 other
panes disagree. This is a genuine second measurement, not a re-read of the same log.

**Acceptance-equivalent tally over the 69 panes probed** (overview-card test not included, only per-pane
boots): 66 pass clean (ready, painted, no refused input, no SPA-fallback asset) + 2 ready-but-refused
(`generation2d`, `raster`) + 1 boot failure (`gis3d`) = **66/69 clean, 3 failing**, matching the acceptance
run's proportions (67/70, the 70th test being the overview card, which this audit does not run).

## Per-pane table

`errors` = raw Playwright console `[error]`-type count (excludes 40x resource 404s and `[DEBUG] ` lines).
`refused` = console lines matching `input #\d+ .* refused:` (the acceptance suite's own hard-fail signal).
`verdict`: `content` = the pane paints (witness, manually cross-checked where the DOM-fallback signature was
ambiguous); `blank` = confirmed empty; `error` = shell never reached ready. `example label`: `OK` = chrome
renders the catalog's exact `exampleLabel`; `n/a` = catalog declares no curated `example`; `n/a (no picker)` =
app has no `setActiveExample`/no fixture element; `MISMATCH` = a different label rendered. `topic` = the
fleet-brief topic that owns this pane's plugin/app (from `📋️fleet-brief-v5.md` + `📓️status.md`'s topic
roster); `peer-owned (…)` = guest code outside every topic's write scope (norm/space/procedural/sequence/
playbook — brief v5's no-touch list).

| variant | outcome | ready-s | errors | refused | verdict | example label | topic | note |
|---|---|---|---|---|---|---|---|---|
| **design** | | | | | | | | |
| cad | ready | 2.7 | 0 | 0 | content | OK | cad-content |  |
| generation3d | ready | 2 | 0 | 0 | content | OK | cad-content |  |
| generation2d | ready | 1.9 | 0 | 1 | content | OK | peer-owned (procedural) | **REGRESSED (new)**: `setActiveExample` refused — `generation2d-config-unsupported-mutation` (peer-owned `procedural` guest: the regenerated descriptor now publishes an action the guest refuses). Flow window still shows the boot `Number(3.0)/Add` nodes (unchanged), Preview still "Empty canvas" (expected, no picker) — same as audit-2 content-wise, but now with a refused-input toast banner that audit-2 did not have. |
| flow | ready | 2.1 | 0 | 0 | content | n/a | flow |  |
| lowpoly | ready | 2.2 | 0 | 0 | content | n/a (no picker) | design |  |
| remodel | ready | 2 | 0 | 0 | content | OK | design |  |
| draw | ready | 1.9 | 0 | 0 | content | OK | design |  |
| raster | ready | 2 | 0 | 1 | blank | OK | raster | **REGRESSED (new failure mode)**: `setActiveExample` refused — `raster-store.mutation-asset-capacity` (the real 512×512/25 039 B emblem exceeds the store's asset bound). Composite AND navigator both `0 layers, 0 assets` (worse than audit-2's 2×2 placeholder, because the curated example never lands at all now). `📓️raster.md` already has the fix on disk (pages `Bytes` retirement from the tail, raises `add-layer-asset`'s bound to the 256 KiB envelope ceiling) and touched `describe.request`/`activate.request` for raster at 03:40 — after this 03:23 activation, so not yet live. |
| layout | ready | 1.9 | 0 | 0 | content | n/a (no picker) | design |  |
| shooting | ready | 1.9 | 0 | 0 | content | OK | media |  |
| **assembly** | | | | | | | | |
| puzzle2d | ready | 2.7 | 0 | 0 | content | OK | block-puzzle |  |
| puzzle3d | ready | 2.4 | 0 | 0 | content | OK | block-puzzle |  |
| puzzle5d | ready | 2.3 | 0 | 0 | content | OK | block-puzzle | **STILL BROKEN, unchanged**: picker reads "Capsule Dream" and the 3D window paints (3 meshes, 1 instance) — but the mesh is `/mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb`, puzzle5d's own BOOT document (`concrete-forest`), not one of Capsule Dream's 34 distinct `/mesh/🧊️*.glb` urls — only **1 distinct** `/mesh/*.glb` request was seen all session. `setActiveExample` never lands the curated document (`📓️block-puzzle.md` §9.5); root cause still open ("deserves its own ticket"), not a capacity refusal. |
| block2d | ready | 1.9 | 0 | 0 | content | OK | block-puzzle |  |
| block3d | ready | 2 | 0 | 0 | content | OK | block-puzzle |  |
| block5d | ready | 2 | 0 | 0 | content | OK | block-puzzle |  |
| **generative** | | | | | | | | |
| wfc2d | ready | 2.2 | 0 | 0 | content | OK | design | **FIXED, verified live** (was audit-2's NEW wasm-trap boot panic): Preview window renders real coloured hex tiles (hex-0..hex-5). Matches `📓️design.md`'s session-6 fix + the 15:47/03:23 acceptance runs both passing. |
| wfc3d | ready | 2.3 | 0 | 0 | content | OK | design | **FIXED, verified live** (was audit-2's NEW wasm-trap boot panic, different fault code than wfc2d/grid3d): boots clean, 0 errors/refused. |
| grid2d | ready | 2.2 | 0 | 0 | content | OK | design |  |
| grid3d | ready | 2.2 | 0 | 0 | content | OK | design | **FIXED, verified live** (was audit-2's NEW wasm-trap boot panic, same fault code as wfc2d): boots clean, 0 errors/refused. |
| bitmap | ready | 2.2 | 0 | 0 | content | OK | design |  |
| **engineering** | | | | | | | | |
| fem2d | ready | 2 | 0 | 0 | content | OK | engineering | Screenshot-verified real truss model (Model window) + stress/force results (Results window) — content confirmed beyond the witness's chrome-text signature. |
| fem3d | ready | 2 | 0 | 0 | content | OK | engineering |  |
| energy | ready | 2.3 | 0 | 0 | content | OK | engineering |  |
| process3d | ready | 2.1 | 0 | 0 | content | OK | engineering |  |
| sourcing | ready | 2.2 | 0 | 0 | content | OK | engineering | 3D viewport itself is near-empty (perspective grid only, same as audit-2) but the curation-table DOM content paints — not a rendering gap, per play-runtime.md's own `sourcing` example. |
| gis2d | ready | 2.8 | 0 | 0 | content | OK | engineering | picker trigger label "Reuse Map" for curated id `demo` — same as audit-2 (id-to-label difference is expected, not a mismatch; label text OK here compares literal `exampleLabel` "Reuse Map" to rendered text, both equal). |
| gis3d | error | 2.8 | 5 | 0 | error | n/a (no picker) | engineering | **REGRESSED (new)**: shell never reaches ready — wasm `unreachable` trap, `interactive-job.catalog-incomplete` ("a migrated generated command lacks its exact owner-local bounded reducer proof"), `…/🔌️plugin/🦀️.rs:22853`. Same fault-code family as audit-2's wfc2d/grid3d panic. Cause: the predecessor's new gisterrain `set_active_example` command (landed on disk, committed) reached the 03:23 descriptor mid-edit — declared but not joined to a live factory proof. gis2d (same plugin crate) boots clean, so this is per-app, not whole-crate. |
| architect | ready | 2.4 | 0 | 0 | content | OK | knowledge-children | **FIXED, verified live** (was audit-2's stuck "Waiting !... !..." placeholder): Adjacency tree shows Reception/Waiting nodes, Register panel shows both cards with elements sub-rows, Graph window draws two connected node boxes. Matches `📓️knowledge-children.md`'s `semio-s-artifact-architect-program` 2087/0 GREEN claim — now confirmed in the browser, not just natively. |
| **standards** | | | | | | | | |
| din4108 | ready | 2.8 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| din16798 | ready | 2.8 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| din18599 | ready | 2.4 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1990 | ready | 2.3 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1991 | ready | 2.5 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1992 | ready | 2.3 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1993 | ready | 2.5 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1994 | ready | 2.2 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1995 | ready | 2.2 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1996 | ready | 2.4 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1997 | ready | 2.3 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1998 | ready | 2.3 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| en1999 | ready | 2.4 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| iso16757 | ready | 2.3 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| vdi3805 | ready | 2.3 | 0 | 0 | content | n/a | peer-owned (norm) |  |
| **knowledge** | | | | | | | | |
| note | ready | 2 | 0 | 0 | content | OK | knowledge |  |
| writer | ready | 2.1 | 0 | 0 | content | OK | knowledge-children |  |
| forms | ready | 2 | 0 | 0 | content | OK | knowledge |  |
| mathematical | ready | 2.2 | 0 | 0 | content | OK | knowledge | **FIXED, verified live** (was audit-2's "blank grid + bare cursor", gated/expected): Graph window now shows real equation nodes (A#0, D#3, C#2 + boxes), Geometry window shows a real polygon with labelled points. Screenshot-verified (the DOM witness text alone — 17 chars, matching just the "Actions" chrome button — would not have proven this; confirmed by eye). |
| reasoning-wires | ready | 2.1 | 0 | 0 | blank | OK | knowledge | **STILL BROKEN, unchanged**: the automated witness reports `painted=true` (DOM fallback counts the 17-char "Actions" toolbar text + 51 chrome elements as content — a witness blind spot: this canvas window has no `[data-surface-id]` marker, so the witness never inspects the canvas itself). Screenshot confirms the canvas is still a fully empty grid, no wires/nodes — same as audit-2. `📓️knowledge.md` §2.5's wires fix requested `activate.request/reasoning-wires` but whether it landed in the 03:23 rebuild is unconfirmed; visually it has not. |
| dag | ready | 2.1 | 0 | 0 | content | OK | knowledge | Screenshot-verified real DAG node boxes + DSL text panel — content confirmed. |
| imperative | ready | 2 | 0 | 0 | content | OK | knowledge |  |
| playbook | ready | 2 | 0 | 0 | content | OK | peer-owned (playbook) | audit-2's new `[warning]` refused-input regression (`setActiveExample refused: … presence local read requires a live exact local retirement owner`) is **NOT reproduced** this run — 0 refused/panics. Appears fixed (or was a transient/peer-slice defect since resolved); consistent with the 15:47 and 03:23 acceptance runs both passing playbook. |
| trinity-jack | ready | 2.5 | 0 | 0 | content | OK | knowledge |  |
| trinity-rewriting | ready | 2.4 | 0 | 0 | content | OK | knowledge |  |
| **documents** | | | | | | | | |
| stdio | ready | 2.7 | 0 | 0 | content | OK | stdio |  |
| stdio-txt | ready | 2.6 | 0 | 0 | content | OK | stdio |  |
| stdio-csv | ready | 2.6 | 0 | 0 | content | OK | stdio |  |
| stdio-tsv | ready | 3.1 | 0 | 0 | content | OK | stdio |  |
| stdio-json | ready | 2.7 | 0 | 0 | content | OK | stdio |  |
| stdio-json-i | ready | 2.6 | 0 | 0 | content | OK | stdio |  |
| stdio-xml | ready | 2.6 | 0 | 0 | content | OK | stdio |  |
| stdio-xml-valid | ready | 2.7 | 0 | 0 | content | OK | stdio |  |
| stdio-html | ready | 2.7 | 0 | 0 | content | OK | stdio |  |
| **media** | | | | | | | | |
| animate | ready | 2.4 | 0 | 0 | content | OK | knowledge-children | **FIXED, still live** (was audit-2's confirmed fix): Tile editor shows real labelled photo tiles (tile-r0-c0 … tile-r1-c3) with actual building imagery. No SPA-fallback assets this run (the `🖼️bauteilbörse.png` defect flagged at 17:00/`📓️status.md` 04:10 is not reproduced here). |
| sequence | ready | 2.2 | 0 | 0 | content | OK | peer-owned (sequence) | Screenshot-verified real script/DSL text (`state.set`, `log.print`, step wiring) — content confirmed. |
| vcs | ready | 2.4 | 0 | 0 | content | OK | media | Screenshot-verified real "VCS Demo · Counter 2" document + commit/branch/undo/redo chrome — content confirmed. |
| demonstrator | ready | 2.6 | 0 | 0 | content | n/a | media | Still the bare genesis title "playground.playground", no curated example — same as audit-2's "expected empty state, not a defect". The witness's DOM fallback counts the title text as "painted" (nonzero length), same as the official acceptance suite's own definition — not a new defect either way. |
| **workspace** | | | | | | | | |
| home | ready | 2.5 | 0 | 0 | content | n/a (no picker) | workspace |  |
| space | ready | 2.1 | 0 | 0 | content | n/a (no picker) | peer-owned (space) |  |

## Special checks

### puzzle5d — wrong-document defect, STILL BROKEN (unchanged from audit-visual-2)

The picker correctly reads **"Capsule Dream"** and the 3D window paints real geometry (3 meshes, 1 instance)
— so the boot-liveness witness alone would call this pane healthy. It is not: the only mesh ever requested
this session was `/mesh/🏚️abbau-aufbau/👈️hexagonal-cut-concrete-forest-left.glb` — puzzle5d's own BOOT
document (`concrete-forest`), not any of Capsule Dream's 34 distinct `/mesh/🧊️*.glb` urls. **Zero** of the 34
expected urls were requested; **one** distinct `/mesh/*.glb` request total. `setActiveExample` for
capsule-dream never actually lands the document — confirmed identical to `📓️block-puzzle.md` §9.5's live
probe from 11:39 on the 03:04 activation. `block-puzzle.md` records this as diagnosed-not-fixed ("deserves
its own ticket"), and this run proves it is still live on the 03:23 activation, three activations later.

### raster — regressed to a NEW failure mode (was "blank", now "refused")

Audit-visual-2 found raster's composite blank with a 2×2 placeholder asset (`assetsJson` carried
`semio-emblem` but decoded nothing). This run: `setActiveExample` is now **refused outright** —
`raster-store.mutation-asset-capacity` — because the raster topic's fix (replacing the 2×2 placeholder with
the real 512×512/25 039-byte shipped emblem, `📓️raster.md` §"the real emblem…") made the curated example's
payload exceed the store's per-grant asset-capacity bound, a defect the raster topic diagnosed and already
fixed on disk (pages `RasterRetirementOwner::Bytes` retirement from the tail; raises `add-layer-asset`'s
candidate/digest bound to the 256 KiB envelope ceiling) — but touched its `describe.request`/`activate.request`
at 03:40, seventeen minutes **after** this 03:23 activation, so the fix is not live on the build this audit
probed. Composite and navigator both witness `0 layers, 0 assets` (worse than audit-2's `2 layers, 1 assets`,
because the curated example never lands on the store now instead of landing with a dead placeholder).

### gis3d — regressed to a boot failure (was "content", now "error")

Both audit-visual-2 (03:04 build) and the 15:47/22:20 activations booted gis3d to ready with content (no
example picker, terrain from the genesis document). This 03:23 build fails to boot at all: a wasm
`unreachable` trap, fault `interactive-job.catalog-incomplete` — "a migrated generated command lacks its
exact owner-local bounded reducer proof" — the same fault-code family (`FaultCode` origin `Framework`,
`interactive-job.*`) as audit-visual-2's wfc2d/grid3d NEW-regression panics. `📓️engineering.md` §11 records
the cause exactly: the predecessor's gisterrain `setActiveExample` command (a schema-first fix, committed by
the 00:05 auto-commit) reached this activation's regenerated descriptor **mid-edit** — the command is declared
in the descriptor but its concrete factory proof is not yet joined to it. `gis2d`, built from the same
`semio-s-artifact-gis-gisterrain` crate, boots clean — this is a per-app registration gap, not a whole-crate
break, exactly the same shape as audit-visual-2's wfc2d/wfc3d/grid3d incident (three of five apps in one
crate family failing while the other two boot fine).

### generation2d — new refused input, content unchanged (peer-owned)

Audit-visual-2 recorded 0 errors/refused for generation2d. This run: `setActiveExample` is refused —
`generation2d-config-unsupported-mutation`, visible as a "The input could not be delivered. Close" toast in
the chrome. The Flow window still shows the same boot-time `Number(3.0)/Add` node graph as before, and the
Preview canvas still reads "Empty canvas" (expected: this app declares no picker-driven content gate, per
audit-visual-2). Per `📓️status.md` 03:37: this is the regenerated gisterrain-style descriptor now publishing
an action generation2d's peer-owned `procedural` guest code refuses — guest code is on the brief's no-touch
list, so this is flagged for the peer, not fixed by any topic this session (no peer session was reachable in
`ListAgents` as of the last status update).

## DIFF against audit-visual-2 (📓️audit-visual-2.md, 2026-09-22 03:04 build)

### Fixed and verified live (5)
- **`architect`** — audit-2's stuck "Waiting !... !..." placeholder → real Adjacency/Register/Graph content
  (node boxes, connected pair, Reception/Waiting cards). Confirms `📓️knowledge-children.md`'s
  `semio-s-artifact-architect-program` 2087/0 GREEN claim, now verified in the browser.
- **`mathematical`** — audit-2's "blank grid + bare cursor" (gated/expected) → real equation-graph nodes
  (A#0/D#3/C#2 boxes) and a real geometry polygon with labelled points. Was classified "expected empty,
  gated" in audit-2; this run shows it is no longer empty at all — a genuine content fix, screenshot-verified
  (the witness's own DOM-fallback text signature would not have proven this by itself — see methodology note).
- **`wfc2d`, `wfc3d`, `grid3d`** — audit-2's NEW wasm-trap boot panics (3 of 5 wfc-family apps) are gone; all
  three boot clean with real content (wfc2d: coloured hex tiles hex-0..hex-5, screenshot-verified). Confirms
  `📓️design.md`'s session-6 fix and both the 15:47 and 03:23 acceptance runs passing all three.
- **`playbook`**'s audit-2 NEW `[warning]` refused-input regression (`presence local read requires a live
  exact local retirement owner`) is not reproduced — 0 refused/panics this run.
- **`animate`** remains fixed (confirmed again: real labelled photo tiles, no SPA-fallback asset this run).

### Still broken — unchanged from audit-visual-2 (2)
- **`puzzle5d`** — loads the wrong document (concrete-forest mesh under the "Capsule Dream" picker); see
  Special checks above. `📓️block-puzzle.md` still marks this diagnosed-not-fixed.
- **`reasoning-wires`** — canvas still fully empty (grid only, no wires/nodes); the paint witness's DOM
  fallback reads `painted=true` from chrome text alone (see methodology caveat), but the screenshot is
  unchanged from audit-2's confirmed-blank verdict.

### Still empty — unchanged and expected/gated, not new (1)
- **`demonstrator`** — still the bare genesis title, no curated example by design (same as audit-2).

### New regressions not present in audit-visual-2 (3 panes)
- **`raster`** — was "blank" (dead placeholder), now "refused" (real emblem exceeds the store's asset-capacity
  bound) — a different failure mode, not a re-occurrence of the old one. Fix is on disk, not yet activated
  (requested 03:40, after this 03:23 build).
- **`gis3d`** — was clean content-with-no-picker, now a hard boot failure (wasm trap,
  `interactive-job.catalog-incomplete`) — the gisterrain `setActiveExample` command landed on disk mid-edit
  relative to this activation's descriptor regeneration.
- **`generation2d`** — was 0 errors/refused, now 1 refused input (`generation2d-config-unsupported-mutation`)
  from the peer-owned `procedural` guest; visible content unchanged.

### Resolved since audit-visual-2's own prior regressions (from its DIFF vs audit #1)
audit-visual-2 flagged `wfc2d`/`wfc3d`/`grid3d` (boot panics) and `playbook` (refused input) as NEW
regressions relative to audit #1; this run confirms all four are now fixed (see "Fixed and verified live"
above) — so the fleet round-tripped through and out of that regression within the 09-22→09-23 session.

## Acceptance summary line

**67/70 strict acceptance on the 03:23 activation** (`📓️acceptance-runs.md`), reds: `generation2d`, `raster`,
`gis3d` — independently reproduced by this audit's own 69-pane probe (different script, same activation, same
3 panes fail for the same documented reasons, 0 disagreements).

## Still-broken and regressed — final lists

**Still broken (2, unchanged from audit-visual-2):** `puzzle5d` (wrong document — block-puzzle topic),
`reasoning-wires` (empty canvas — knowledge topic).

**Regressed since audit-visual-2 (3, all new this activation):** `raster` (raster topic — fix on disk, pending
re-activation after 03:40), `gis3d` (engineering topic — gisterrain descriptor mid-edit), `generation2d`
(peer-owned procedural — no reachable peer session to route it to).
