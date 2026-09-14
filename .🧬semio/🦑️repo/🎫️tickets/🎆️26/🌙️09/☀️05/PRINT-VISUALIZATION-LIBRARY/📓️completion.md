# Completion — the print visualization library

What the library is at the end of ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`, what has been
verified with a real run, and what is knowingly left. Counts are as measured at the time of writing;
where a number was still moving because another agent was finishing, it says so.

## 1 What it is

`semio-viz` is a grammar of graphics for print: LaTeX3 packages under
`🧰️framework/🛍️products/📓️print/🖋️latex/`, driven by a handcrafted catalogue of chart kinds and
verified numerically against third-party libraries through a probe protocol.

```
data → transform → scale → coordinate system → mark → guide → annotation → theme
```

A *chart kind* is a catalogue preset; it names a *family*; the family composes the grammar and draws.
Nothing in a renderer hard-codes a colour, a stroke width or a font size: those come from the design
tokens through `semio-viz-theme`.

### The numbers

| | |
|---|---|
| LaTeX packages under `🖋️latex/` | **89** — 9 document packages (`semio.cls`, `semio-core`, `semio-window`, `semio-table`, `semio-fonts`, `semio-tokens`, …) and **80 `semio-viz*`** |
| of those, generated | 2 (`semio-viz-catalog.sty`, `semio-viz-catalog-labels.sty`) |
| Taxonomy sections | **80** (`🖼️assets/📊️viz-taxonomy.md`) |
| Taxonomy leaves | **1,966**, each covered exactly once |
| Catalogue chart kinds | **1,738** (`🖼️assets/🔣️viz-catalog.json`) |
| Families | **246**, every one registered by a package |
| Public API | 55 packages, 159 commands, 895 documented keys (`🖼️assets/🔣️viz-api.json`) |
| Gallery documents | **80**, one per section, generated |
| Test cases | **104**, one directory each (`🥒️.feature`, `🟦️.ts`, `🧫️fixtures/`) |
| Gherkin scenarios | **330** |
| Probe fixtures | **110** |
| TypeScript modules / commands | 8 / 6 |

### The package families

- **Kernel**: `semio-viz-data` (tables, roles, CSV, point sets, geometry, demo data), `-scale`,
  `-transform`, `-layout` (30 registered algorithms), `-spatial`, `-coordinate`, `-mark`, `-shape`,
  `-guide`, `-annotation`, `-label`, `-facet`, `-composition`, `-plot`, `-theme`, `-format`,
  `-family`, `-probe`.
- **Namespaces**: `charts-*` (area, bar, dashboard, distribution, financial, funnel, line, polar,
  scatter, statistical, timeline), `matrix-*`, `network-*`, `flow-*`, `geo-*`, `hierarchy-*`,
  `scientific-*` (3d, biology, chemistry, engineering, field, geometry, mathematics, physics,
  signal, surface), `diagram-*` (architecture, concept, flowchart, process, uml), `infographic`,
  `interactionstate`, `text`, `domain`, `table`, `showcase`.
- **Generated**: `semio-viz-catalog`, `semio-viz-catalog-labels`.

## 2 Commands

All from `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript`:

```bash
bun ./📜️script.ts generate              # design-token stylesheet
bun ./📜️script.ts generate viz          # catalogue → 83 artifacts (packages, gallery, viz-api.json)
bun ./📜️script.ts fonts                 # provision the print fonts into ⚡️cache/print-fonts
bun ./📜️script.ts build                 # every registered template, light and dark
bun ./📜️script.ts build viz api         # only the API reference
bun ./📜️script.ts test fundamental      # no TeX: catalogue, schema, taxonomy, generated-file freshness
bun ./📜️script.ts test quick            # kernel probes against the d3 oracles
bun ./📜️script.ts test long             # family probes
bun ./📜️script.ts test viz full         # + the whole gallery build
bun ./📜️script.ts test viz fixtures [section…]   # regenerate the exhaustive gallery fixture
bun ./📜️script.ts test exhaustive       # + every gallery variant against the committed fixture
```

Platform-side, from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`:

```bash
bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/📓️print"
bun ./📜️script.ts parity quick|long --owner "🧰️framework/🛍️products/📓️print"
```

**`--case` mysteriously reports `cases=0`** when the case slug carries its emoji identity — use
`--project test-framework-products-print-da77cb-<slug-without-emoji>` instead.

## 3 Verified runs

Real tails, from this session.

### `test fundamental`

```
$ bun ./📜️script.ts test fundamental
[test] level=fundamental cases=100 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=98
```

### `contract` — no print breach

```
$ bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/📓️print"
1212 high-priority breach(es) across 5 rule(s):
    671  testing/contract
    335  testing/fixture
    159  testing/dependency
     42  testing/oracle
      5  testing/discovery
$ grep -c "print" <that output>
0
```

The sweep is repository-wide and its exit code carries every owner's breaches; **print contributes
none of the 1212**.

### `parity quick` — 100 %

```
$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print"
[test] level=quick cases=100 executed=516 passed=516 failed=0 errored=0 parity=242/242 not-exercised=12
exit=0
```

### `parity long`

```
$ bun ./📜️script.ts parity long --owner "🧰️framework/🛍️products/📓️print"
[test] level=long cases=102 executed=596 passed=596 failed=0 errored=0 parity=276/277
[test] parity failed: …::📚️catalog-coverage::api-reference::typescript::subject (2 differences)
```

The single failure is the `🖼️assets/🔣️viz-api.json` staleness check, which is exactly what it is
for: the file derives from the LaTeX sources and any package edit makes it stale. After
`generate viz`:

```
$ bun ./📜️script.ts parity long --owner "…/📓️print" --project …-catalog-coverage
[test] level=long cases=1 executed=20 passed=20 failed=0 errored=0 parity=10/10
```

Two `…-kinds` cases (`🖱️interactionstate-kinds`, `📰️infographic-kinds`) errored once each on a
loaded machine and pass on their own and in the clean run — they compile an ~80 s document and lose
to the per-adapter budget when several tectonic processes fight for the CPU.

### `test long`

```
$ bun ./📜️script.ts test long
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 128/128
[DEBUG] print: unit tests passed
[test] level=long cases=104 executed=616 passed=616 failed=0 errored=0 parity=286/287
```

The command **could not pass before this session's last fix**, for a reason outside the library:
`runPrintPlatformCases` spawned the platform's `parity long` on the generic 600 s command budget,
and print's `parity long` runs past twenty minutes, so `test long` killed its own child
(`error: spawnSync bun ETIMEDOUT`). It now uses `orchestratorBudgetOpts()` — the preset that exists
for exactly this shape, a script fanning out to leaves that carry their own budgets.

The single parity failure is the `viz-api.json` staleness; after `generate viz` the freshness check
over `vizGeneratedFiles()` is empty (`stale after generate: 0`).

### Every package loads standalone

```
$ python 🔧️standalone-load.py
packages=80 failed=1
--- semio-viz-catalog-labels
    ! Not loaded.
```

`semio-viz-catalog-labels` requires `semio-core`, whose `\begin{document}` hook calls
`\selectlanguage`; babel is set up by the `semio` class, so it loads there and not under plain
`article`. The other 79 load with nothing but `\documentclass{article}`.

### Requirement closure and docstrings

```
$ python 🔧️package-closure.py
packages with a gap: 0
$ python 🔧️docstring-gaps.py
    0  TOTAL over 78 files
```

### The gallery

```
$ bun 🔬gallery-lane.ts … 0
[DEBUG] gallery lane: section 0 ok after 547.6s
```

See §5.

## 4 Known limits, measured

| Limit | Number | Where it comes from |
|---|---|---|
| Force layout, 100 nodes × 300 iterations | **≈ 5.5 min** | l3fp arithmetic; the catalogue caps `iterations` at 60 and `samples` at 96 for this reason |
| `pack` on 211 nodes | **13.0 s** (was 15.0 s) | POLISH took 13 % off with 35 pack layouts; byte-identical probe output. The remaining cost is the algorithm, not the harness |
| Bundle load per compile | **≈ 3.1 s** | `\documentclass{article}\usepackage{semio-viz-hierarchy}` with nothing else in it |
| One gallery section, all four variants | **9.1 min** | measured, §5 |
| One gallery variant | **≈ 137 s** | the same measurement ÷ 4 |
| The exhaustive matrix | **320 compilations ≈ 19 h, one lane** | 80 sections × 2 themes × 2 languages; two lanes collide, they do not halve it |
| Probe compile budget at `quick` | 30 s per adapter | platform-side; a probe now compiles in **one** tectonic pass, no synctex, no PDF |

## 5 The exhaustive gallery — state at hand-over

`test viz full` and `test exhaustive` are **compute-bound, not blocked**. The measurement that
settles the scale:

```
$ bun 🔬gallery-lane.ts lane-probe.json sections.tsv 0
[DEBUG] gallery lane: section 0 ok after 547.6s
0	ok	547.6
```

That is one taxonomy section — four PDF compilations (light/dark × en/de), each parsed back with
pdf.js for its per-kind page text and hashed for reproducibility. Eighty sections is 320 such
compilations.

Sections differ by a factor of three, so no single average describes them:

| section | variants | wall time | per variant |
|---|---|---|---|
| 0 marks | 4 | 547.6 s | 137 s |
| 2 time series | 4 | 449.4 s (3 remaining) | 150 s |
| 3 distributions | 4 | 1739.6 s | 435 s |

**16 of 320 variants are measured** (sections 0–3) at hand-over; at the ~230 s mean of those four,
the remaining 304 are about **19 h** on this machine. The lane is running detached.

**Run one lane, never two.** The task allowed two and two was tried; it does not work, and the
reason is structural: `printGalleryMatrix` compiles every variant through the single staging root
`dist/.semio-gallery-matrix`, which is also on tectonic's search path, so two concurrent lanes read
each other's half-written intermediates. Sections 3–12 each died in four to six seconds — the
"under 20 s means a collision" signature the coordinator wrote down. The two-lane run also produced
five phantom `parity long` failures in the spatial cases, all of which pass alone.

One lane now walks sections 2–79 with `🔬gallery-lane.ts`, resume-safe: it skips every variant
already in its evidence JSON, so it can be interrupted and restarted at any point.
`🔬gallery-merge.ts` then writes
`🧪️tests/🖼️gallery-render/🧫️fixtures/🖼️gallery-render.json` in exactly the shape
`regeneratePrintGalleryFixtures` writes, so `bun ./📜️script.ts test viz fixtures <section>` —
the documented command — reproduces any single section of it.

Per-section wall times are appended to `🗑️generated/FINALIZE/sections.tsv`; the table as it stands
at hand-over, including the failed two-lane rows, is in `📓️status-FINALIZE.md`.

**One real defect the sweep found and FINALIZE fixed before the lanes started**: every multi-word
chart-kind title was welded together in every gallery document, in both languages
(`Annulararc`, `Bidirectionalarrow`, `Referencepoint`), because the generated
`semio-viz-catalog-labels.sty` registered all 1,738 titles inside `\ExplSyntaxOn`, where a space is
ignored. The generator closes expl3 syntax before the titles now. Any fixture measured before that
fix is worthless, which is why the first section-0 run is recorded as a failure and re-measured.

## 6 Deferred, with the reason

- **The exhaustive fixture and `test exhaustive`** — machine time (§5). No known defect in the way.
- **A fourth hierarchy package.** `semio-viz-hierarchy`, `-hierarchy-tiling` and `-hierarchy-packing`
  are one module split by file size, and the two algorithm files require the tree owner back. The
  clean alternative is lifting the tree store, constructor and traversal (~250 lines) into
  `semio-viz-data.sty` under a `\semio_viz_tree_*` prefix.
- **`physics-projectile-rk4`** stays a `specification-vectors` decision: mathjs' `solveODE`
  integrates on its own adaptive grid and the case compares fixed-step RK4 on a five-decimal grid,
  so the flip needs the fixture's step reduced rather than the tolerance widened.
- **Two oracle manifests** (`🔣️oracle.json` and `🔮️oracle/🔣️.json`) — see `📓️integration.md`.
- **`bun nx run` repo-wide** — two puzzle-plugin cases collapse to one nx project name. Not print's
  tree; the underlying `bun ./📜️script.ts …` commands all work.
- **Gallery documents under three minutes** — a document is ~2.3 min today. POLISH measured the
  honest ceiling of kernel work; the rest is l3fp arithmetic in the families themselves.

## Closing addendum (coordinator, 2026-09-06 evening)

After FINALIZE, the GAPS agent closed the verification gaps (`📓️verification.md`): schema documents every implemented family key (2,878 key slots from 296 authored en+de descriptions, enforced by `@id-implemented-keys-documented`), four new cases (`📰️infographic-kinds`, `🖱️interactionstate-kinds`, `🌈️theme-palettes`, `🪶️plot-grammar`), seven dead families deleted and `petri-net` re-pointed, `\SemioVizInterpolate` and a d3-exact quadtree added. Final measured state: 106 cases, `test fundamental` 17/17 (parity 8/8), `parity quick` 550/550 (259/259 pairs), `parity long` 630/630 (294/294 pairs), platform `contract` 0 print breaches, `generate viz` 83 artifacts with nothing stale. Coordinator re-ran `test fundamental` at close: `cases=106 executed=17 passed=17 failed=0 errored=0 parity=8/8`.

Still running at close: the exhaustive gallery sweep (80 sections × light/dark × en/de = 320 variants, ~19 h single-lane on this machine; sections 0–4 done, 449–1,740 s each). Its evidence file is `🗑️generated/FINALIZE/sections.tsv`; when it ends, regenerate the `gallery-render` fixtures with the documented command, run `bun ./📜️script.ts test exhaustive`, and delete the `🗑️generated` folder. Two follow-ups were deliberately left for their own ticket (see `📓️status-GAPS.md`): the `uncertainty` family lacks the pooled-summary diamond and log axis that `sci-forest` had; `beeswarm`/collide should use the new quadtree.
