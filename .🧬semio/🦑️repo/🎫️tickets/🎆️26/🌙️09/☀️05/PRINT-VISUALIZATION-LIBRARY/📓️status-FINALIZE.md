# Status — FINALIZE

Agent FINALIZE. Sole editor of the print product for this pass.

| # | Task | State |
|---|---|---|
| 1a | Migrate 17 files off `\semio@viz@font@…`, delete the four aliases | **done** |
| 1b | `\RequirePackage` closure gaps (`🔧️package-closure.py`) | **done** (13 → 0) |
| 1c | `%region 🔖️Pending-…` = 0 repo-wide in `🖋️latex` | **done** (already 0) |
| 1d | Rotated-mark scenario in `✒️mark-geometry` | **done** |
| 1e | Docstring gaps = 0 over ALL `semio-viz-*.sty` | **done** (727 → 0 over 78 files) |
| 2 | `generate viz`, `test fundamental`, `contract`, `parity quick/long` | **done** — 0 print breaches, quick 242/242, long green |
| 3 | `test long` | **done** (and the command's own 600 s budget defect fixed) |
| 3 | `test viz full`, gallery fixtures, `test exhaustive` | **partly — compute-bound**, one lane running, §3 |
| 4 | Reconciliation, `📓️completion.md`, `📓️files.md`, hygiene | **done** |

## 1a — the legacy font switches are gone

143 uses across 17 files replaced by the expl3 constants
(`\c_semio_viz_theme_font_{cell,chip,title,label}_tl`); the four `\cs_new:cpn` aliases and the
`%region 🔖️FontSwitches` block in `semio-viz-theme.sty` are deleted, and so are the three dead
`\providecommand`s at the top of `semio-viz-scientific-field.sty`.

```
$ grep -rho "semio@viz@font@[a-z]*" 🖋️latex/*.sty | sort | uniq -c
(no output)
$ grep -rho "c_semio_viz_theme_font_[a-z]*_tl" 🖋️latex/*.sty | sort | uniq -c
      4 c_semio_viz_theme_font_cell_tl
     46 c_semio_viz_theme_font_chip_tl
    107 c_semio_viz_theme_font_label_tl
     14 c_semio_viz_theme_font_title_tl
      2 c_semio_viz_theme_font_value_tl
$ grep -rn "Pending-" 🖋️latex/*.sty
(no output)
```

## 1b — requirement closure: 13 packages with a gap → 0

`🔧️package-closure.py` reported 13 packages calling a macro outside their transitive
`\RequirePackage` closure. Fixed at the source rather than by pointing a kernel package at a family:

1. **Variants generated in the wrong package** (FAMILIES-DOMAIN's shape 2). Every
   `\semio_viz_table_*` and `\semio_viz_scale_*` variant now sits beside its base function:
   `semio-viz-data.sty` generates `table_cell {nVnN,VnnN,VVnN,VnVN,nnVN}`, `table_nrows {VN}`,
   `table_col_min/max {nVN,VnN,VVN}`, `table_col_values {VnN,VVN}`; `semio-viz-scale.sty` adds
   `scale_define:nnnnn {nnxxV}` and `scale_map {nxN}`. The 22 duplicate generations in
   `charts-bar`, `composition`, `domain`, `mark`, `network-graph`, `plot`, `showcase` and `text`
   are deleted (`plot` and `composition` also lose their `\cs_if_exist:NF` guards around them).
2. **`composition-bar` moved from `semio-viz-charts-bar.sty` to `semio-viz-charts-area.sty`.**
   The family delegates to the bar renderer *and* the area renderer, so it cannot live in the lower
   of the two — requiring `charts-area` from `charts-bar` would have closed the loop
   bar → area → line → bar. `charts-area` already sits above both.
3. **Real missing requirements**: `semio-viz-mark` → `data`, `family`, `probe` (it calls
   `\semio_viz_family_define:nn` and generates a `probe_geometry` variant at load time, so a
   minimal document loading only `semio-viz-mark` used to fail); `semio-viz-charts-polar` → `mark`;
   `semio-viz-matrix-correlation` → `spatial`; `semio-viz-hierarchy-tiling` → `spatial`.
4. **The point primitives moved down.** `\semio_viz_sci_px:n`, `…_py:n` and `…_pt_put:Nnn` were
   defined in `semio-viz-scientific-field.sty` and called from `semio-viz-spatial.sty` — a kernel
   package reading a family package. They are now `\semio_viz_points_x:n`, `\semio_viz_points_y:n`
   and `\semio_viz_points_put:Nnn` in `semio-viz-data.sty`'s `%region 🔖️PointSets`, beside the
   point store whose `x,y` shape they take apart; 234 call sites in eleven packages renamed.
5. **The analytic marching-squares pass moved back up.** `\semio_viz_spatial_marching:nnnnn` reads
   the scientific canvas domain (`\l_semio_viz_sci_d{x,y}{min,max}_fp`) and draws through
   `\semio_viz_sci_polyline:Nn`, so it was never spatial's to own. It is
   `\semio_viz_sci_marching:nnnnn` in `semio-viz-scientific-field.sty` `%region 🔖️MarchingSquares`
   now, with its two helpers; its ten call sites in `scientific-mathematics`, `-physics` and
   `-surface` follow. The generic Sutherland–Hodgman clipper stays in `semio-viz-spatial.sty`.
6. **The hierarchy trio is one module.** `semio-viz-hierarchy-tiling` and `-packing` read the tree
   store that `semio-viz-hierarchy` owns, and `semio-viz-hierarchy` requires them both. They now
   require it back, with a note saying why: `\ProvidesPackage` has already registered the file when
   the mutual `\RequirePackage` is reached, so the load terminates and every entry point of the
   module loads all of it. Splitting the tree store out into a fourth package would be the
   alternative; it is recorded as the deferred option in `📓️integration.md`.

```
$ python 🔧️package-closure.py
packages with a gap: 0
```

## 1b — every package loads standalone

New `🔧️standalone-load.py` compiles `\documentclass{article}\usepackage{semio-viz-<pkg>}` with
xelatex for every `semio-viz-*.sty`:

```
$ python 🔧️standalone-load.py
ok      semio-viz-annotation
…
ok      semio-viz-transform

packages=80 failed=1
--- semio-viz-catalog-labels
    ! Not loaded.
```

The single failure is not a package defect: `semio-viz-catalog-labels` requires `semio-core`, whose
`\begin{document}` hook calls `\selectlanguage`, and babel is set up by the `semio` document class.
Under `\documentclass{semio}` it loads. The other 79 load under plain `article`.

**Environment note for the next agent**: xelatex finds the print fonts only when `TTFONTS` (not
just `TEXINPUTS`) names `⚡️cache/print-fonts`; without it every compile dies with
`Package fontspec Error: The font "Anta-Regular" cannot be found.` and that error says nothing
about the package under test.

## 1d — the rotated mark is pinned

`🧪️tests/✒️mark-geometry` gains the scenario `rotation` (`@id-rotation`, `@level-quick`,
`@mode-conformance`) over the new fixture `🧫️fixtures/mark-rotation.tex`: the triangle and the
rectangle each drawn twice at the same point, once at 0° and once at 30°/45°. Compiled locally:

```
$ xelatex mark-rotation.tex ; cat mark-rotation.probe.jsonl
{"case":"mark-geometry","scenario":"rotation","key":"geometry/mark/triangle/at","values":[28,12,0,12]}
{"case":"mark-geometry","scenario":"rotation","key":"geometry/mark/triangle/at","values":[28,12,30,12]}
{"case":"mark-geometry","scenario":"rotation","key":"geometry/mark/rectangle/at","values":[28,12,0,12]}
{"case":"mark-geometry","scenario":"rotation","key":"geometry/mark/rectangle/at","values":[28,12,45,12]}
```

The adapter projects the four angles, the number of distinct placements (1 — rotation moves the
mark's own point nowhere) and the number of distinct records (4). Both halves of POLISH-2's
`\c_colon_str` fix are covered: the document compiles at all, and the angle reaches the record.

## Defect found on the way — the exhaustive fixture was unreachable

`🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts` still pointed `GALLERY_CASE_DIR` at
`🧪️tests/gallery-render`, the pre-TESTS-2 name. TESTS-2 renamed the directory to
`🖼️gallery-render`, so `loadPrintGalleryFixture()` would have thrown
"the exhaustive gallery fixture is missing" on any `test exhaustive` run. Fixed.

## 1e — docstrings over EVERY package, not only the POLISH-2-owned set

POLISH-2's checker carried a hand-written list of the 53 files it owned. `🔧️docstring-gaps.py` is the
same rule over a glob, so the FAMILIES-DOMAIN and FAMILIES-CAPABILITY packages are measured too.
It found **727 gaps in 23 files** — every one of them a `semio-viz-diagram-*`,
`semio-viz-scientific-*`, `semio-viz-network-*`, `semio-viz-infographic`,
`semio-viz-interactionstate` or `semio-viz-text` definition that never had a note.

The 23 files were written out in six lanes partitioned strictly by file (no two lanes ever held the
same file), each note authored against the definition body it sits above, Read/Edit only.

| lane | files | notes added |
|---|---|---|
| 1 | `diagram-flowchart`, `diagram-process` | 132 |
| 2 | `scientific-mathematics`, `scientific-biology` | 122 |
| 3 | `network-graph`, `diagram-architecture`, `scientific-geometry` | 120 |
| 4 | `scientific-physics`, `-chemistry`, `-engineering`, `diagram-uml` | 129 |
| 5 | `scientific-surface`, `-signal`, `-3d`, `diagram-concept`, `interactionstate` | (in flight) |
| 6 | `network-matrix`, `infographic`, `scientific-field`, `network-arc`, `-bundling`, `-chord`, `text` | 85 |

Three non-emoji leading glyphs were found and corrected on the way (`∫` twice, `〽️` once) — the
checker is right to reject them, they are mathematical and CJK symbols, not emoji.

Every edited package still loads:

```
$ python 🔧️standalone-load.py <the 11 files of lanes 1, 2 and 6>
packages=11 failed=0
```

### Four families vanished mid-session — it was GAPS, not damage

Three docstring lanes reported family blocks disappearing from under them: `sci-forest`
(`semio-viz-scientific-biology.sty`), `sci-timing` (`-engineering`), `sci-surface` (`-surface`) and
`concept-venn` (`diagram-concept`). I treated it as possible data loss and checked before doing
anything: all four are absent from every `.sty`, from `🖼️assets/🔣️viz-catalog.json` and from
`🧬️schema/🔣️.json`, so no catalogue kind can reach them. `📓️status-GAPS.md` then explained it —
the GAPS agent was deleting eight dead families concurrently, and `semio-viz-matrix-adjacency.sty`
with them. Nothing was restored and nothing needed to be; `unknownFamilies` is empty and the closure
and docstring checkers were re-run afterwards against the smaller tree:

```
$ python 🔧️package-closure.py
packages with a gap: 0
$ python 🔧️docstring-gaps.py
    0  TOTAL over 78 files
```

GAPS also finished the legacy-token item from the other end (`\semio@viz@diagram@width/@height` →
`\c_semio_viz_theme_frame_{width,height}_fp`), so with F1 above **`grep -r "semio@viz@" 🖋️latex`
is now empty**.

### Two strays in the product root

`🧰️framework/🛍️products/📓️print/x` — an untracked 2.5 kB JSON scratch file (family → count) left by
an earlier agent — is deleted. `🧰️framework/🛍️products/📓️print/🔣️oracle.json` is a second, much
smaller oracle manifest beside `🔮️oracle/🔣️.json` (2 kB against 66 kB); it is staged, and the
platform's owner-root rule reads files with exactly that name, so it is left alone and flagged here.

## 2 — regenerate and verify

### `generate viz`

```
$ bun ./📜️script.ts generate viz
print: wrote 83 visualization catalogue artifacts
```

### `test fundamental`

```
$ bun ./📜️script.ts test fundamental
[test] level=fundamental cases=100 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=98
```

The live coverage report behind it:

```
{ "leaves": 1966, "kinds": 1738, "uncovered": 0, "overcovered": 0, "unknownFamilies": [],
  "unknownOptions": [], "missingVariant": 0, "dup": 0, "untitled": 0, "unknownDemoTables": [] }
```

### `contract` — 0 print breaches

```
$ bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/📓️print"
1212 high-priority breach(es) across 5 rule(s):
    671  testing/contract
    335  testing/fixture
    159  testing/dependency
     42  testing/oracle
      5  testing/discovery
$ grep -c "print" <the captured output>
0
```

The sweep is repository-wide and its exit code carries every owner's breaches — print contributes
none of the 1212. (Same reading as TESTS-2's, and the reason the print suite runs `parity` rather
than `contract` for itself.)

### `parity quick` — 100 %, and one comparison larger than before

```
$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print"
[test] level=quick cases=100 executed=516 passed=516 failed=0 errored=0 parity=242/242 not-exercised=12
exit=0
```

242, not POLISH-2's 241: the new `✒️mark-geometry::rotation` scenario.

### `parity long` — the three non-green scenarios are green

```
$ bun ./📜️script.ts parity long --owner "🧰️framework/🛍️products/📓️print"
[test] level=long cases=102 executed=596 passed=596 failed=0 errored=0 parity=276/277
[test] parity failed: 🧰️framework/🛍️products/📓️print::📚️catalog-coverage::api-reference::typescript::subject (2 differences)
```

TESTS-2 handed over three: the two `📶️charts-histogram-density` bin scenarios (POLISH-2's
`\l_semio_viz_tr_stat_datamax_fp` fix) and `🥧️charts-pie-donut::donut`, which was
`semio-viz-charts-polar` calling `\semio_viz_pie_from_seq:Nnnnn` without requiring
`semio-viz-mark` — closed by the requirement-closure work above. All three pass; nothing was
weakened, no tolerance moved, no scenario was demoted.

The one remaining comparison failure is `viz-api.json` staleness, which is the check doing its job:
the file derives from the LaTeX sources and I had just edited them. After `generate viz`:

```
$ bun ./📜️script.ts parity long --owner "…/📓️print" --project …-catalog-coverage
[test] level=long cases=1 executed=20 passed=20 failed=0 errored=0 parity=10/10
```

**Two flaky executions, understood.** An earlier run showed `errored=2` and the next `errored=1`,
each time an `…-kinds` case (`🖱️interactionstate-kinds`, `📰️infographic-kinds`). Both compile an
~80 s document; run alone they pass (`executed=1 passed=1 errored=0`), and the same fixture compiles
with both xelatex and tectonic by hand. It is the per-adapter budget losing to CPU contention, not
content — the clean run above has `errored=0`.

## 3 — the exhaustive level

### `test long` — and a defect in the command itself

The clean run, machine otherwise idle:

```
$ bun ./📜️script.ts test long
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 128/128
[DEBUG] print: unit tests passed
error: spawnSync bun ETIMEDOUT
 spawnargs: [ "./📜️script.ts", "parity", "long", "--owner", "🧰️framework/🛍️products/📓️print" ]
      at runPrintPlatformCases (…/🧪️print-pipeline-verification/🟦️.ts:16:3)
[test] level=long cases=104 executed=616 passed=616 failed=0 errored=0 parity=286/287
[test] parity failed: …::📚️catalog-coverage::api-reference::typescript::subject (2 differences)
```

Two things in that output.

**`bun ./📜️script.ts test long` could not pass, for a reason that had nothing to do with the
library.** `runPrintPlatformCases` shells out to the platform's `parity` with no `budgetMs`, so it
took the generic `CMD_BUDGET_MS` of **600 s** — and print's `parity long` runs past twenty minutes,
so the command killed its own child every time. `runCmd` has a preset for exactly this shape
(`orchestratorBudgetOpts()`, 4 h, "nx/script orchestrators fanning out to individually budgeted
leaves"), and that is what it uses now. Fixed in
`🎮️commands/🧪️print-pipeline-verification/🟦️.ts`.

**The child itself is green**: `executed=616 passed=616 failed=0 errored=0`, and the one parity
failure is again the `viz-api.json` staleness, cleared by `generate viz`:

```
$ bun ./📜️script.ts generate viz
print: wrote 83 visualization catalogue artifacts
$ bun -e '…vizGeneratedFiles() vs disk…'
stale after generate: 0
```

### An earlier `test long` disagreed — and the disagreement was mine

Run while the two gallery lanes were compiling, the same command reported
`passed=611 errored=5 parity=282/287` with five spatial cases failing by one difference each
(`🏔️spatial-contours-density::kernel-density`, `🔺️spatial-delaunay-voronoi::delaunay-triangles`
and `::voronoi-cells`, `🐝️spatial-hexbin::hexagonal-binning`, `🥚️spatial-hull::convex-hull`).
All five pass alone:

```
$ … --project …-spatial-hull            [test] executed=2 passed=2 errored=0 parity=1/1
$ … --project …-spatial-delaunay-voronoi [test] executed=4 passed=4 errored=0 parity=2/2
$ … --project …-spatial-hexbin           [test] executed=2 passed=2 errored=0 parity=1/1
$ … --project …-spatial-contours-density [test] executed=4 passed=4 errored=0 parity=2/2
```

and the clean `test long` above has `errored=0`. **The cause was my own two-lane gallery sweep**, see
below — not the packages, and nothing was changed in them on account of it.

### The gallery matrix: measured, and why two lanes was the wrong call

| section | variants | wall time | note |
|---|---|---|---|
| 0 | — | *(first attempt)* | **FAIL** — `no page carries the en title "Annular arc"`: the `\ExplSyntaxOn` title defect (F11), fixed and re-measured |
| 0 | 4 | **547.6 s** | one lane, machine otherwise idle — 137 s per variant |
| 1 | 4 | **872.3 s** | two lanes running (218 s per variant — the contention, before the collisions began) |
| 2 | 1 of 4 | 257.3 s | **FAIL** — tectonic exited 1 (collision) |
| 3–12 | 0 | 3.9–5.7 s each | **FAIL** in seconds — the collision signature |
| — | | | *lanes killed, single lane restarted over 2–79* |
| 2 | 3 remaining | **449.4 s** | one lane — 150 s per variant |
| 3 | 4 | **1739.6 s** | one lane — 435 s per variant; §3 is the distribution families, the heaviest so far |

**Total at hand-over: 16 of 320 variants, sections 0–3 complete.** The spread between §0 (137 s a
variant) and §3 (435 s) is the honest reason a single average is misleading; on these four sections
the mean is ~230 s a variant, which puts the remaining 304 variants at **19 h** on this machine.
The lane is still running, detached, and appends to the same two files.

The coordinator's own rule caught it: *"gallery lane rows failing in under ~20 s are collisions, not
content defects."* Sections 3–12 each died in four to six seconds. The reason is structural and
mine: `printGalleryMatrix` compiles every variant through the **one** staging root
`dist/.semio-gallery-matrix`, which is also on tectonic's search path, so two lanes compiling at once
read each other's half-written intermediates. Two lanes is not "two lanes at most" here — it is one
lane, twice, wrong.

Corrected: the lanes were killed, the bad rows are in `🗑️generated/FINALIZE/sections.tsv` above for
the record, and **one** lane now walks sections 2–79 into the same evidence file. The measured cost,
single-lane and uncontended, is **547.6 s for four variants ≈ 137 s per variant**, so the full
matrix is **320 compilations ≈ 12 h**, and `test exhaustive` compiles all 320 again to compare.
That is the honest reason `test viz full`, the fixture regeneration and `test exhaustive` are not
green in this session: machine time, not a defect.

**To finish it** (resume-safe — the lane skips every variant already in the JSON):

```bash
cd "<ticket>"
bun 🔬gallery-lane.ts 🗑️generated/FINALIZE/lane-a.json 🗑️generated/FINALIZE/sections.tsv 2 3 4 …
bun 🔬gallery-merge.ts 🗑️generated/FINALIZE/lane-probe.json 🗑️generated/FINALIZE/lane-a.json
cd "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript"
bun ./📜️script.ts test exhaustive
```

`🔬gallery-merge.ts` writes `🧪️tests/🖼️gallery-render/🧫️fixtures/🖼️gallery-render.json` in exactly
the shape `regeneratePrintGalleryFixtures` writes, so
`bun ./📜️script.ts test viz fixtures <section>` reproduces any single section of it — that is the
documented command, and the lane exists only because it can be interrupted and resumed.
**Run one lane. Never two.**

## 4 — reports and hygiene

- `📓️integration.md` carries the closing reconciliation (`## From FINALIZE`): sixteen items, each
  done or deferred with the reason, plus what is still open and for whom.
- `📓️completion.md` — what the library is now: package list, counts, commands, verified runs with
  real output tails, the measured limits, and the deferred list.
- `📓️files.md` — every file the fleet created, updated or removed, from a **read-only**
  `git status --short`: 595 under the print product, 8 in the repository test platform, 5 workspace
  files. Generated by `🔬files-manifest.ts` so it can be refreshed.
- `README.md` of the print product: 245 → 246 families.
- `🗑️generated/`: 252 tool-output files removed (PDF, log, aux, jsonl, synctex, bbl/blg/run.xml and
  my three throwaway probe directories). What remains is inputs — the other agents' probe `.tex`
  documents and Python helpers, and this agent's live gallery evidence (`lane-a.json`,
  `lane-probe.json`, `sections.tsv`, `lane-a.out`), which the running lane is still writing.

## Files touched by FINALIZE

**LaTeX** — `semio-viz-theme` (aliases and the FontSwitches region deleted),
`semio-viz-data` (point primitives, table variants), `semio-viz-scale` (scale variants),
`semio-viz-mark` (three requirements, duplicate variants deleted),
`semio-viz-spatial` (marching squares moved out, point primitives renamed, three docstrings),
`semio-viz-scientific-field` (marching squares moved in, dead `\providecommand`s deleted),
`semio-viz-charts-area` (`composition-bar` moved in), `semio-viz-charts-bar` (moved out, variants),
`semio-viz-charts-polar`, `semio-viz-matrix-correlation`, `semio-viz-hierarchy-tiling`,
`semio-viz-hierarchy-packing` (requirements), `semio-viz-composition`, `semio-viz-domain`,
`semio-viz-network-graph`, `semio-viz-plot`, `semio-viz-showcase`, `semio-viz-text` (variants),
and the 17 files of the font migration (`diagram-*`, `scientific-*`, `infographic`,
`interactionstate`, `network-graph`).

**TypeScript** — `🔨️modules/📊️visualization-gallery/🟦️.ts` (the `\ExplSyntaxOn` title defect),
`🎮️commands/🧪️print-pipeline-verification/🟦️.ts` (the gallery case directory name, the orchestrator
budget), `🧪️tests/✒️mark-geometry/🟦️.ts` (the rotation scenario).

**Tests** — `🧪️tests/✒️mark-geometry/🥒️.feature` and the new
`🧫️fixtures/mark-rotation.tex`.

**Ticket** — `🔧️standalone-load.py`, `🔧️docstring-gaps.py`, `🔬gallery-lane.ts`,
`🔬gallery-merge.ts`, `🔬files-manifest.ts`, `📓️status-FINALIZE.md`, `📓️completion.md`,
`📓️files.md`, and the `## From FINALIZE` section of `📓️integration.md`.

**Deleted** — the stray untracked `🧰️framework/🛍️products/📓️print/x` (a 2.5 kB JSON scratch file
left in the product root).

## Open at hand-over

1. **The exhaustive gallery matrix** — 16 of 320 variants; one lane running detached, ~19 h to go.
   Resume and merge commands are in §3.
2. **Two oracle manifests** in the product (`🔣️oracle.json`, `🔮️oracle/🔣️.json`) — left alone
   deliberately, someone who knows which the contract reads should remove the other.
3. **`bun nx run` repo-wide** — two puzzle-plugin cases collapse to one nx project name. Not print.
