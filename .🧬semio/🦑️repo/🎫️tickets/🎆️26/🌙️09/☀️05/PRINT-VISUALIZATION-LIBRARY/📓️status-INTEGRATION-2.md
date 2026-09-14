# INTEGRATION-2 — status

Agent: INTEGRATION-2. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Scope: finish INTEGRATION-1's open items, dissolve the remaining `Pending-` regions, build the
layout dispatcher, fold the catalogue scripts, get the platform and the gallery green.

The item-by-item reconciliation of every request addressed to integration is in
`📓️integration.md`, section "From INTEGRATION-2 — final reconciliation". This file carries the
work, the real output tails, and what is left.

---

## 0. One interruption worth recording

At about **2026-09-06 03:51** a `git pull` on `🐙ueli/⛳wip` (run outside this session) auto-stashed
the entire working tree and did not restore it: the tree fell back to the 78 legacy
`semio-viz-chart-*.sty` files and every fleet package, test and ticket document vanished into
`stash@{0}` (583 files). I did not touch git — `CLAUDE.md` forbids it and two commits had landed in
the meantime — and instead exported a read-only snapshot of the stashed tree
(`git archive`, 629 print files + 66 ticket files) to the session scratchpad as insurance, and wrote
up the state. The repo owner restored the stash a few minutes later; everything came back, including
my own edits up to the last one. One file is still `UU` in the index:
`🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` — its conflict markers are gone and it parses, so the
test platform runs, but the merge is not recorded as resolved.

Two practical lessons, both now in `📓️integration.md`: python on this machine cannot write through
an absolute path containing emoji (`OSError: [Errno 22]`) unless `PYTHONUTF8=1` is set and the script
`os.chdir()`s to the owning directory first; and concurrent `build viz …` runs collide on
`dist/source/<template>`.

---

## 1. Done

### 1.1 A one-minute bundle load check
`🗑️generated/INTEGRATION-2/load-check.ts` renders a probe under
`\documentclass[type=report,theme=light,language=en]{semio}` with `\usepackage{semio-viz}`, compiles
it with the repo tectonic and prints the first LaTeX error. Every refactor below was checked with it.
Two defects it found immediately:

- `semio-core.sty:119` used `\RenewDocumentCommand{\subtitle}` although nothing defines `\subtitle`
  first, so the package could not be loaded outside a class that happens to provide it →
  `\DeclareDocumentCommand`.
- Digits inside expl3 variable names: `\l_…_x0_fp` parses as `\l_…_x` followed by a typeset `0`,
  which surfaces as `Missing \begin{document}`. My option variables are `xlo/ylo/xhi/yhi`.

### 1.2 All 21 `Pending-` regions dissolved

Eight moved into the kernel package that owns them, thirteen renamed into an owned region with a
note that says why they stay. `grep -rn "Pending-" 🖋️latex/*.sty` returns nothing.

**Moved into a kernel package**

| from | to |
|---|---|
| `hierarchy`, `diagram-flowchart` row readers | `semio-viz-data.sty`: `\semio_viz_table_cell:nnnN` now splits the row with `\seq_set_split:NnV` instead of a comma list, so an empty cell no longer shifts every column after it (HIERARCHY's and DIAGRAMS' shared defect). New: `\semio_viz_table_row_load:nn`, `\semio_viz_table_row_cell:nN`, `\semio_viz_table_col_find:nnN` (silent), `\semio_viz_table_col_exist:nnTF`, `\semio_viz_table_col_index_opt:nnN`, `\semio_viz_table_cell_opt:nnnN`. 375 call sites rewired across nine packages |
| `geo` geometry collections (183 lines) | `semio-viz-data.sty` `%region 🔖️Geometry` — `\semio_viz_geom_*` plus `\SemioVizGeoCollection/Polygon/Line/PointPart/FromTable`, with the `unknown-geometry` message |
| `spatial` point sets and value grids (62 lines) | `semio-viz-data.sty` `%region 🔖️PointSets`; prefixes `semio_viz_spatial_points_*` → `semio_viz_points_*`, `…_grid_*` → `semio_viz_grid_*`, `\l_semio_viz_spatial_name_tl` → `\l_semio_viz_store_name_tl` |
| `scientific-field` `🔖️Numerics` (174 lines) | `semio-viz-transform.sty` `%region 🔖️Distributions` — erf, Lanczos log-gamma, log-binomial, Lentz incomplete beta, pdf/cdf/quantile; prefix `semio_viz_tr_dist_`, plus the general `\semio_viz_tr_finite:nTF` |
| `scientific-field` marching squares + Sutherland–Hodgman (84 lines) | `semio-viz-spatial.sty` `%region 🔖️FieldGeometry`; prefix `semio_viz_spatial_` |
| `scientific-field` ramps | `semio-viz-theme.sty`: `\semio_viz_theme_ramp:n`, `…_ramp_diverging:n`, `…_ramp_cyclic:n` (all expandable, for TikZ option lists) |
| `diagram-flowchart` + `scientific-field` number and language helpers | `semio-viz-format.sty`: `\semio_viz_format_round:nn`, `\semio_viz_format_mm:n` (the 4-decimal millimetre grid), `\semio_viz_format_count:n`, `\semio_viz_format_localized:nn`, `\semio_viz_format_decimal:nn` |
| `hierarchy` probe variant | `semio-viz-probe.sty` generates `\semio_viz_probe_geometry:nn { nx , xx }` |

**Renamed, with the reason in the region note**

`hierarchy` → `🔖️Constructor`, `🔖️VoronoiTreemap`; `diagram-flowchart` → `🔖️Chrome`, `🔖️Words`;
`network-graph` → `🔖️Chrome`; `network` → `🔖️Keys`; `network-bundling` → `🔖️Spline`;
`scientific-field` → `🔖️Chrome`, `🔖️Language`, `🔖️Samples`, `🔖️Numerics`, `🔖️Canvas`, `🔖️Paths`;
`scientific-3d` → `🔖️Camera`; `geo-choropleth` → `🔖️Classes`.

Three of those are decisions worth naming:

- **`geo-choropleth`** did not just get renamed: the hand-rolled quantile/threshold class rules are
  **replaced** by the scale kernel. The class rule is registered as the scale `semio-choropleth`
  (`quantile`/`threshold`/`quantize`) whose range is the evenly spaced ramp positions, and
  `\semio_viz_scale_binned_item:nnN` reads the class. That fixed a real bug: `quantize` was the
  documented default but fell through to the continuous normalisation, so `scale=quantize` and no
  `scale` rendered identically. The per-value position prop stays because the fill runs inside a
  TikZ option list, which cannot call a protected macro.
- **`network-bundling`'s spline** is *not* merged into the mark kernel: the kernel's `bundle`
  interpolator emits Bézier path syntax, while edge bundling needs the sampled polyline for both the
  drawn path and the numeric probe record it is compared on.
- **`hierarchy`'s voronoi treemap** keeps its own half-plane clip: the kernel's
  `\semio_viz_spatial_clip:NNnnn` clips point sequences, this one clips the flat `poly` clist once
  per site pair per iteration, and converting inside that loop costs more than the routine.

`\l_semio_viz_width_fp` / `\l_semio_viz_height_fp` are now declared once, plainly, in
`semio-viz-guide.sty`; the `\AddToHook{begindocument}` INTEGRATION-1 left there and the two
`\cs_if_exist:NF` twins in `hierarchy` and `diagram-flowchart` are gone.

### 1.3 The layout dispatcher (architecture §3)

New kernel package **`semio-viz-layout.sty`**, required by `semio-viz.sty` right after
`semio-viz-data` and by every domain kernel; it depends on none of them.

- `\semio_viz_layout_define:nn {algorithm} {code}` — in `code`, `#1` input, `#2` output, `#3` options
- `\semio_viz_layout_run:nnnn` (+ `nVnV`, `VVnV`, `nnnV`, `VnnV`, so the duplicate
  `\cs_generate_variant` lines in `flow-sankey` and `network-graph` are gone)
- `\semio_viz_layout_exist:nTF`, `\SemioVizLayout{algorithm}{in}{out}[opts]`, `\SemioVizLayoutAlgorithms`
- messages `unknown-layout` (which lists what *is* registered) and `duplicate-layout`

**30 algorithms**: hierarchy `tree cluster treemap partition pack`; network `force circular shell
grid spectral layered radial arc chord bundling adjacency`; flow `sankey alluvial parallel-sets`;
geo `projection`; spatial `delaunay voronoi hull hexbin contour density beeswarm jitter`;
transform `stack bin`.

Behind that registration:

- **spatial** gained `%region 🔖️LayoutRegistry`: an l3keys family `semio / viz / spatial / layout`
  (`x0 y0 x1 y1 radius amount threshold bandwidth cell width height`) and publishers that write the
  result into the **named** `out` store — triangles, voronoi cells, hull and contour rings become
  geometry collections, hexbin centres and swarm positions become point sets, the density grid is
  aliased onto `out`. Before this the algorithms only wrote fixed global sequences.
- **geo** gained `projection`: fits (d3 `fitExtent`) and projects every part of a geometry into plane
  millimetres, storing the result as a geometry collection, so a renderer draws projected parts
  exactly like source parts.
- **transform**: `stack` and `bin` are transforms here and layout algorithms in §75; both names now
  reach `\semio_viz_transform:nnn`.

`\SemioVizHierarchyLayout` is **deleted**; its 35 call sites (four hierarchy fixtures and the
package's own key documentation) say `\SemioVizLayout`.

### 1.4 The probe harness actually runs

Three defects, each of which alone made every LaTeX-probe case unrunnable:

1. **The subject shape.** The test host reads `outcome.projection`; 48 adapters returned the bare
   projection map, so the host hashed `undefined` and every probe subject errored. **56 adapters**
   now return `{ projection }`.
2. **The oracle grid.** The shared `grid()` helper snapped every value below one emission step to
   zero while the probe rounds it onto the grid — `0.3^10 = 5.9049e-6` is emitted as `0.00001` and
   the oracle claimed `0`. Fixed in the **23** adapters that carried it.
3. **The empty document.** A TeX run with no page writes no files at all, and **76** committed
   fixtures never called `\SemioVizProbePage`. `semio-viz-probe.sty` now emits the identifying line
   itself at `\AtEndDocument` when the document produced none.

And the budget, which is what `geo-path-graticule`'s `spawnSync bun ETIMEDOUT` really was: not the
tectonic budget (20 min) but the platform's quick-level budget (30 s) against a probe that spent
21.9 s. `compilePrintTexOnce` promised one pass and asked tectonic for three, plus synctex and a PDF
that no probe reads. It now compiles a probe in one pass with none of that:

```
[DEBUG] math-distributions records 12 ms 4911
[DEBUG] signal-dft-bode   records  8 ms 4965
```

### 1.5 First oracle flip, verified

`math-distributions` is `@oracle-jstat` instead of `@no-oracle-jstat`. jStat computes the six laws
from its own error function, log-gamma and incomplete beta, and inverts the continuous ones in
closed form where the probe bisects its own distribution function; the hand-written TypeScript
reference is deleted.

```
[test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
```

### 1.6 Catalogue and schema

- **`test fundamental` green.** `semio-viz-catalog.sty` was stale: 30 `timeline` kinds still bound
  `data=demo-interval` while the catalogue and the family default say `demo-lane` (`demo-interval`
  is CHARTS-B's confidence-interval table).
- **The `variant` rule** is now the schema's, in one place: `vizCoverageReport()` gained
  `missingVariant`, which flags an entry only when its family's vocabulary declares the key, and both
  `catalog-coverage` scenarios read it instead of re-implementing it. The rule is written into the
  schema's `$comment`.
- **`registeredVizFamilies()` was wrong**, which is where INTEGRATION-1's "236 families still
  awaiting a registration" came from: it matched `\SemioVizFamily{name}` without the space every
  package actually writes, and did not know the internal `\semio_viz_family_define:nn` that a kernel
  package loaded *before* the registry has to use. 236 → 25 after fixing both, and 25 is the true
  number.
- **`composition-bar`** implemented (§6 stacked bar, 100 % stacked bar, stacked area), and
  `stacked-area` rebound from `demo-parts` (which has no numeric x) to `demo-multiseries`.
- **`generate viz` writes `🖼️assets/🔣️viz-api.json`** (DOCS' request): 83 artifacts now, 49 packages,
  245 families. The generated-file marker check understands JSON, since a JSON file cannot carry a
  `% 🤖 Generated` comment.
- The `🏗️catalog-*.ts` ticket scripts were **already** unnecessary: `generate viz` reads only the
  catalogue, the schema and the taxonomy, and nothing under the product references a ticket script.
- The hull coverage question needed no change: the §0 marks cover `0/`, `26/` and `75/convex-hull`
  once each, `78/hull` is the layout entry, coverage is exactly-once.

### 1.7 Platform contract

31 print breaches. Nine were malformed Gherkin — `*` is a step keyword, so a bullet list in a Feature
*description* parses as a step outside a scenario; the bullets in `network-algorithms` and
`network-circular-arc` are en dashes now. Eight are the remaining oracle flips (§2). **Fourteen
belong to TS-TWIN**: `🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/📜️script.ts` imports the
registered oracles d3-array, d3-chord, d3-contour, d3-delaunay, d3-hierarchy and d3-shape from
production source.

### 1.8 Gallery defects fixed

Five, each of which killed a whole section:

| section | defect |
|---|---|
| 4 | the catalogue called the scatter shape channel `symbol`, the family declares `shape`, and the key was never declared → renamed in the 12 entries (and `symbol-coded-scatter-plot` now actually uses a different shape). The same 12 also bound `data=demo`, which has no `x`/`y` column → `demo-scatter` |
| 3 | `\str_case:eF` is not a generated variant; the table family's total label now goes through `\semio_viz_format_localized:nn` |
| 6 | family `composition-bar` had no renderer |
| 15 | the catalogue binds `data=` and `variant=` on every kind, and `semio / viz / diagram` declared neither |
| 21–44 | the same for the scientific namespace: `\semio_viz_sci_keys_canvas:n`, the key block every `sci-*` family shares, had no `data` key. One edit covers all of them |
| — | `semio-viz-spatial.sty` declared the `unknown-pointset`/`unknown-grid` messages that moved to `semio-viz-data.sty` |

Demo cost capped in the catalogue per the coordinator's force-layout decision: `iterations` ≤ 60
(10 entries) and `samples` ≤ 96 (2 entries).

---

## 2. Verified runs

### `bun ./📜️script.ts generate viz`
```
print: wrote 83 visualization catalogue artifacts
```

### `bun ./📜️script.ts test fundamental`
```
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 128/128
[DEBUG] print: 25 families still awaiting a \SemioVizFamily registration: text-viz schedule biology engineering-diagram optimization analytical spatial-layout notation…
[DEBUG] print: unit tests passed
[test] level=fundamental cases=68 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=66
```

Re-run an hour later, while another agent was adding the missing families, it reads
`cases=77 … parity=6/7` with only `catalog-coverage::generated` failing: `🔣️viz-api.json` is derived
from the LaTeX sources, so **every package edit makes it stale until `generate viz` runs again**.
That is the check working, and it is a new obligation `generate viz` did not carry before — written
up in `📓️integration.md`. It settles when the package edits stop; the family count was already down
from 25 to 22 by then.

### `bun ./📜️script.ts parity quick --owner "…/📓️print" --case math-distributions`
```
[test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
```

### bundle load check (`🗑️generated/INTEGRATION-2/load-check.ts`)
```
[DEBUG] load-check: 1 records, bundle loads
```
also with, in one document and without error:
```
\SemioVizPointSet{lc-pts}{1,1; 4,2; 2,5; 6,6; 3,3}
\SemioVizLayout{hull}{lc-pts}{lc-hull}
\SemioVizLayout{jitter}{lc-pts}{lc-jit}[amount=1]
\SemioVizLayout{hexbin}{lc-pts}{lc-hex}[radius=2]
\SemioVizLayout{delaunay}{lc-pts}{lc-tri}
\SemioVizLayout{voronoi}{lc-pts}{lc-vor}[x0=0,y0=0,x1=8,y1=8]
\SemioVizLayout{tree}{demo-hierarchy}{lc-tree}[size={100,60}]
\SemioVizChart{scatter-plot} \SemioVizChart{frequency-table-visualization}
\SemioVizChart{stacked-bar} \SemioVizChart{100percent-stacked-bar} \SemioVizChart{stacked-area}
\SemioVizChart{activity-diagram} \SemioVizChart{choropleth-map}
```

### `bun ./📜️script.ts contract --owner "…/📓️print"`
31 print breaches at the start; 9 fixed, 8 are the oracle flips, 14 are TS-TWIN's.

---

## 3. Gallery sweep

`🗑️generated/INTEGRATION-2/build-sections.sh` runs one lane of sections and records exit code, wall
time and the first LaTeX error; lane A takes the odd sections, lane B the even ones, lane C the
re-runs. Per-section times below are **both themes together** (light and dark).

| section | s | result |
|---|---|---|
| 1 | 517 | ok |
| 2 | 492 | ok |
| 3 | 102 | `\str_case:eF` — **fixed**, rebuilding |
| 4 | 39 | scatter `symbol` key — **fixed**, rebuilding |
| 5 | 512 | ok |
| 6 | 38 | family `composition-bar` — **fixed**, rebuilding |
| 7 | 620 | ok |
| 8 | **2685** | ok — the network section, built *before* the `iterations` cap |
| 9 | 34 | read a half-written `semio-viz-catalog.sty` (my regeneration collided with the running build) — **rebuilding** |
| 11 | 502 | ok |
| 13 | 49 | family `text-viz` — **unregistered family, open** |
| 15 | 41 | `semio / viz / diagram` had no `data`/`variant` key — **fixed**, rebuilding |
| 17 | 649 | ok |
| 19 | 567 | ok |
| 21 | 38 | the shared scientific key block had no `data` key — **fixed** (one edit covers every `sci-*` family, so §21–§44) |
| 23 | 491 | ok |

At 05:32 another agent added `semio-viz-showcase.sty`, `semio-viz-text.sty` and
`semio-viz-domain.sty` (the packages meant to hold the 25 unregistered families) **with every
backslash stripped by a Bash heredoc**, plus three matching loader lines. `\usepackage{semio-viz}`
stopped loading, so every build from that moment died in ten seconds with
`semio-viz-showcase.sty:1: LaTeX Error: Missing \begin{document}` — sections 4, 6, 9, 15 and 27 are
recorded as failures for that reason alone. I rewrote the three package files with the Write tool
(leaving their empty `%region 🔖️Families` untouched), the loader they had already repaired
themselves, verified the bundle loads again, warned them in `📓️integration.md`, and started lane D
on 4, 6, 9, 15 and 21.

Lanes C and D re-verified the fixes against real builds: **§3 `rc=0` (1284 s)** and **§4 `rc=0`
(550 s)**. §6 reached 424 s and the dark variant before failing, so `composition-bar` renders; what
it hit there was another agent's in-flight rename of the palette count to
`\c_semio_viz_theme_presence_int`, caught between `semio-viz-theme.sty` using it and
`semio-tokens.sty` defining it. Both sides are consistent again.

**Most late failures in the tables are collisions, not defects.** Three signatures, all of them
another process writing a file this build was reading: `Missing \begin{document}` pointing into
`semio-viz-catalog.sty` (my `generate viz`), into `semio-tokens.sty` or `semio-viz-showcase/text.sty`
(the other agent's writes), and any failure that ends in under ~20 s. A real content defect takes the
section at least a minute to reach. Re-run before believing any row.

The lanes were still running when I finished; `sections/{A,B,C,D}.tsv` is the live record — one row per
section as `<section> <exit code> <seconds> <first LaTeX error>`, and a `viz-<n>.log` survives only
for a section that failed. Re-run a lane with
`bash 🗑️generated/INTEGRATION-2/build-sections.sh <lane> <section…>`.

**The ~3-minute target is not met.** The sections that build take 8–10 minutes for light and dark
together, i.e. 4–5 minutes each, and §8 took 45. Capping `iterations` and `samples` removed the worst
outliers (§8 should now be a fraction of 2685 s), but the floor is the l3fp kernel itself, not the
demo sizes. A section built alone is also faster than one built against two other tectonic processes,
which is how most of these numbers were measured.

**Do not run `generate viz` while a lane is building**: the truncate-and-write of
`semio-viz-catalog.sty` can be read half-written, which is exactly what killed §9 (`Missing
\begin{document}` pointing into the generated package is always this, never a real LaTeX fault).

---

## 4. Left open

1. **25 families, 234 catalogue kinds, have no renderer** — the largest remaining piece of
   namespace work. Nine are the §74–79 capability families (`namespace` 54 kinds, `encoding` 32,
   `grammar` 23, `scale` 15, `figure` 12, `transform-data` 9, `layout-algorithm` 7,
   `transform-statistical` 6, `shape` 5) and sixteen are long-tail domain families (`text-viz` 12,
   `spatial-layout` 10, `engineering-diagram` 7, `performance` 6, `optimization` 5, `schedule` 5,
   `monitoring` 4, `niche` 4, `election` 3, `neural` 3, `notation` 3, `biology` 2, `logistics` 2,
   `survey` 2, `version-control` 2, `analytical` 1). Every gallery section containing one dies with
   `Unknown viz family`, and `test long` gates on the registration.
2. **Three oracle flips**: `physics-projectile-rk4` (mathjs), `signal-dft-bode` (fft-js),
   `3d-projection` (gl-matrix) — the same shape of work as `math-distributions`, one case each.
3. **`parity quick` for the whole owner** has not been run end to end; only `math-distributions` was
   verified after the harness fixes. The three harness defects in §1.4 were blocking every case, so
   the previous "not exercised" readings say nothing about the cases themselves.
4. **`test long` and `test viz full`** not run.
5. **Gallery sections 16–79** not built.
6. **The ~3-minute gallery target** (§3).

---

## 5. Files touched

Created: `🖋️latex/semio-viz-layout.sty`, `🖼️assets/🔣️viz-api.json` (generated).

Edited, `🖋️latex/`: `semio-core.sty`, `semio-viz.sty`, `semio-viz-data.sty`, `semio-viz-format.sty`,
`semio-viz-theme.sty`, `semio-viz-guide.sty`, `semio-viz-probe.sty`, `semio-viz-transform.sty`,
`semio-viz-spatial.sty`, `semio-viz-geo.sty`, `semio-viz-geo-choropleth.sty`,
`semio-viz-geo-contours.sty`, `semio-viz-geo-symbols.sty`, `semio-viz-hierarchy.sty`,
`semio-viz-hierarchy-tree.sty`, `semio-viz-hierarchy-dendrogram.sty`, `semio-viz-network.sty`,
`semio-viz-network-graph.sty`, `semio-viz-network-bundling.sty`, `semio-viz-flow-sankey.sty`,
`semio-viz-matrix-correlation.sty`, `semio-viz-charts-bar.sty`, `semio-viz-charts-scatter.sty`,
`semio-viz-table.sty`, `semio-viz-diagram-flowchart.sty`, `semio-viz-diagram-architecture.sty`,
`semio-viz-diagram-concept.sty`, `semio-viz-diagram-process.sty`, `semio-viz-diagram-uml.sty`,
`semio-viz-infographic.sty`, `semio-viz-interactionstate.sty`, `semio-viz-scientific-field.sty`,
`semio-viz-scientific-3d.sty`, `semio-viz-scientific-biology.sty`,
`semio-viz-scientific-geometry.sty`, `semio-viz-scientific-mathematics.sty`,
`semio-viz-scientific-physics.sty`, `semio-viz-scientific-surface.sty`.

Edited, elsewhere: `🖼️assets/🔣️viz-catalog.json`, `🧬️schema/🔣️.json`,
`🔨️modules/📊️visualization-gallery/🟦️.ts`, `🔨️modules/🖨️tectonic-template-compilation/🟦️.ts`,
`🧪️tests/catalog-coverage/🟦️.ts`, `🧪️tests/math-distributions/{🥒️.feature,🟦️.ts}`,
`🧪️tests/network-algorithms/🥒️.feature`, `🧪️tests/network-circular-arc/🥒️.feature`,
56 probe adapters (`{ projection }` wrapping), 23 of them also for the `grid()` epsilon,
`🧪️tests/hierarchy-{pack,partition,tree-cluster,treemap}/🧫️fixtures/*.tex`,
and the generated `semio-viz-catalog.sty`, `semio-viz-catalog-labels.sty`, the 80 gallery documents
and `🔣️viz-api.json`.

Ticket inputs kept: `🔧️rename-cs.py` (literal control-sequence rename across packages) and, under
`🗑️generated/INTEGRATION-2/`, `load-check.ts`, `build-sections.sh`, `move-region.py`,
`wrap-projection.py`, `fix-grid-epsilon.py`, `fix-gherkin-bullets.py`, `fix-scatter-shape.py`,
`fix-catalog-data.py`, `cap-demo-cost.py`, `schema-comment.py`.
