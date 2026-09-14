# SCIENTIFIC — status

Agent: SCIENTIFIC. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Owner of `semio-viz-scientific-field|surface|signal|physics|chemistry|biology|engineering|mathematics|geometry|3d.sty`
and the catalogue entries of taxonomy sections 21, 22, 25, 26 (except voronoi/delaunay/hull),
29, 30, 31, 32, 33, 34, 35 (diagram kinds), 36 (analysis kinds), 38, 39, 40, 41, 43, 44,
and the Feynman/knot/circuit/chemical/music notations of 56.

## 1. State

**Done.** All ten packages implement real, data- and option-driven renderers; 322 catalogue kinds
point at 78 registered families with pairwise-distinct option sets; seven test cases are written
against the TESTS-HARNESS contract and every one of them was verified numerically against its
oracle by compiling the committed fixture locally and diffing the probe records.

## 2. Families registered (78)

| Package | Families |
|---|---|
| `scientific-field` | `sci-field` (+ the shared scientific kernel) |
| `scientific-surface` | `sci-surface`, `sci-skewt`, `sci-climate-series`, `sci-climograph`, `sci-geo-section` |
| `scientific-signal` | `sci-signal`, `sci-timefreq`, `sci-staff`, `sci-pianoroll`, `sci-tonnetz` |
| `scientific-physics` | `sci-force`, `sci-kinematics`, `sci-spacetime`, `sci-feynman`, `sci-levels`, `sci-fieldlines`, `sci-wave`, `sci-optics`, `sci-poincare` |
| `scientific-chemistry` | `sci-molecule`, `sci-energy-profile`, `sci-orbital`, `sci-phase-diagram`, `sci-electrochem`, `sci-spectrum-peaks`, `sci-crystal` |
| `scientific-biology` | `sci-survival`, `sci-forest`, `sci-dose-response`, `sci-scatter-diagnostic`, `sci-population`, `sci-pathway`, `sci-pedigree`, `sci-punnett`, `sci-lifecycle`, `sci-circos`, `sci-ideogram`, `sci-genome-track`, `sci-manhattan`, `sci-seqlogo`, `sci-contact-map`, `sci-clinical-timeline`, `sci-growth-chart`, `sci-nomogram`, `sci-schematic` |
| `scientific-engineering` | `sci-circuit`, `sci-timing`, `sci-control`, `sci-smith`, `sci-constellation`, `sci-beam`, `sci-mohr`, `sci-truss`, `sci-dimensioned`, `sci-material` |
| `scientific-mathematics` | `sci-function`, `sci-integral`, `sci-approximation`, `sci-complex`, `sci-iteration`, `sci-dynamics`, `sci-distribution`, `sci-abstract-diagram`, `sci-grid-diagram`, `sci-braid`, `sci-set`, `sci-upset`, `sci-optimization`, `sci-probability-tree`, `sci-stochastic`, `sci-simplex`, `sci-transition`, `sci-density` |
| `scientific-geometry` | `sci-construction`, `sci-conic`, `sci-transform`, `sci-tiling`, `sci-fractal`, `sci-mesh`, `sci-pitch`, `sci-bracket`, `sci-sports-series` |
| `scientific-3d` | `sci-3d` |

Every family documents its keys (type, default, meaning) in a `%region 🔖️Keys` block directly above
its `\keys_define:nn { semio / viz / family / <name> }`.

## 3. The shared scientific kernel (`semio-viz-scientific-field.sty`)

Every kernel package I depend on (`semio-viz-plot`, `-shape`, `-guide`, `-theme`, `-transform`,
`-format`, `-coordinate`, `semio-viz-geo`, `semio-viz-spatial`) was an empty stub when this work
started, so the capabilities live in my first loader-order package under `%region 🔖️Pending-<package>`
blocks, namespaced `\semio_viz_sci_…` so they never collide with the names the kernel owners will
introduce. My other nine packages `\RequirePackage` it.

- `Pending-semio-viz-theme`: `\semio_viz_sci_hue:n` (design-token categorical palette),
  `_ramp:n`, `_ramp_diverging:n`, `_ramp_cyclic:n`.
- `Pending-semio-viz-format`: `\semio_viz_sci_text:nn` (en/de, **error** when the document language is
  unset — never a silent default) and `\semio_viz_sci_num:nn` (German decimal comma).
- `Numerics`: `\semio_viz_sci_erf:n` (Abramowitz–Stegun 7.1.26), `_lgamma:n` (Lanczos g=7),
  `_lchoose:nn`, `_ibeta:nnnN` (Lentz continued fraction), `_pdf:nnnnN` / `_cdf:nnnnN` /
  `_quantile:nnnnnnN` for normal, exponential, uniform, beta, binomial and Poisson.
- `Pending-semio-viz-transform`: point sequences, uniform sampling, adaptive midpoint refinement,
  extents, Riemann sums (left/right/mid/trapezoid), RK4, a seeded MINSTD random stream with
  Box–Muller normals, an O(n²) DFT, complex evaluation of a rational transfer function on the
  imaginary axis, and Durand–Kerner root finding with a deterministic ascending root order.
- `Pending-semio-viz-spatial`: marching squares (`\semio_viz_sci_marching:nnnnn`) and
  Sutherland–Hodgman half-plane clipping (`\semio_viz_sci_clip:NNnnn`).
- `Pending-semio-viz-guide`: the millimetre canvas, the data window, the frame with grid, ticks and
  titles, and the zero lines.
- `Pending-semio-viz-shape`: polyline and area emission, markers, arrows — all of which report their
  geometry through `\semio_viz_sci_report:Nn` → `\semio_viz_probe_geometry:nn`.
- `Keys`: `\semio_viz_sci_keys_canvas:n` injects `width, height, domain, range, ticks, grid, axes,
  title, xlabel, ylabel, seed` into a family's own key family; `\semio_viz_sci_enter:nnn` resets the
  canvas, applies the family defaults and then the caller's options.

`semio-viz-scientific-3d.sty` additionally owns `%region 🔖️Pending-semio-viz-coordinate`: the
3D→2D projection (orthographic, isometric, axonometric, oblique, perspective) and the painter's
algorithm facet buffer, plus the five Platonic solids as vertex and face tables.

## 4. Decisions

- Sections without a package of their own in the fixed loader list are placed by subject matter:
  §43 sports → `scientific-geometry` (court and pitch templates are geometry), §44 music →
  `scientific-signal` (its waveform and spectrogram kinds reuse the signal kernel), §35 diagram
  kinds → `scientific-surface`, §21 genomics and §22 medicine → `scientific-biology`, §36 structural
  analysis and §31 electronics → `scientific-engineering`.
- Function expressions are l3fp expressions over the public variables `\SemioX`, `\SemioY`,
  `\SemioT` (e.g. `sin(\SemioX)*exp(-\SemioX/4)`). No expression parser of our own is needed and no
  external library is involved.
- Record-shaped options use `/` between fields, `+` between repeats and `:` inside a pair. `/` and
  `+` are ordinary characters both inside and outside expl3; `:` is a *letter* inside expl3, so
  `\semio_viz_sci_split_colon:Nn` string-normalises both sides before splitting — without it, data
  written in a document would not match a delimiter written in a package.
- **l3fp's ternary `?:` evaluates both branches.** Three bugs came from that (a `\clist_item` with a
  negative index, `ln` of a negative number, a divide by zero in the diffraction envelope); every
  remaining ternary in my packages is safe in both branches, and the ones that could not be made safe
  are now `\fp_compare:nNnTF` outside the expression.
- TikZ option values must not contain a bare `=`, so a colour chosen by `\int_compare:nNnTF` inside
  a `\draw [...]` breaks pgfkeys' key/value split. Every conditional style is now resolved into a tl
  before the path is drawn.
- Control sequence names may not contain digits under `\ExplSyntaxOn` (`\l_…_k1x_fp` parses as
  `\l_…_k` followed by text), which cost the first hour; all internals are digit-free.
- Randomness is a seeded MINSTD stream in `fp` (48271·(2³¹−2) exceeds TeX's integer range but not
  l3fp's 16 digits), so every figure redraws identically.

## 5. Catalogue

`🏗️catalog-scientific.ts` in this ticket folder rewrites the `family`, `options` and `data` of the
322 catalogue kinds I own and **asserts that no two of them share a family/option signature**, which
is the distinctness property the exhaustive run checks. Run it with
`bun ./🏗️catalog-scientific.ts` from the ticket folder; last run: `scientific catalogue entries
rewritten: 322`.

> ⚠️ **CATALOG**: `🏗️build-catalog.ts` regenerates the whole file from the taxonomy and overwrites my
> assignments with `family: <section name>, options: { variant: <slug> }`. Please run
> `🏗️catalog-scientific.ts` after every regeneration, or fold its `MAPPING` table into the generator.

Demo tables announced (defined in `semio-viz-scientific-field.sty`, `%region 🔖️DemoData`, created in
`\AtBeginDocument`): **`demo-science`**, **`demo-science-field`**, **`demo-science-graph`**. The
catalogue entries of my sections point at `demo-science`; the families themselves carry their own
option defaults, so every kind renders without a data table.

## 6. Tests

Seven cases under `🧰️framework/🛍️products/📓️print/🧪️tests/`, each with `🥒️.feature`, `🟦️.ts` and a
committed probe fixture under `🧫️fixtures/`. The repo runner is not wired to my machine yet, so each
fixture was compiled with the local xelatex and its records diffed against the adapter's oracle with a
node script (the scripts are under `🗑️generated/SCIENTIFIC/tests/` and are deleted with that folder).

```
math-functions-sampling   ALL MATCH (9 keys)
math-distributions        ALL MATCH (12 keys)
signal-dft-bode           ALL MATCH (8 keys)
physics-projectile-rk4    ALL MATCH (5 keys)   analytic deviation 4.99e-6
geometry-tilings-fractals ALL MATCH (3 keys)
biology-kaplan-meier      ALL MATCH (2 keys) + two-groups ALL MATCH (1 keys) + HAZARD MATCH
3d-projection             ALL MATCH (5 keys)
```

Representative probe records (from `math-distributions.probe.jsonl` and `signal-dft-bode.probe.jsonl`):

```
{"key":"cdf/normal","values":[0.0249978261082239,0.1586552638323639,0.5000000005,0.841344736167636,0.975002173891776]}
{"key":"pmf/binomial","values":[0.02824752489999999,0.2668279319999983,0.2001209490000005,0.0000059049]}
{"key":"law/beta","values":[2.160900000000015,0.579825000000005,0.9590399999999997,0.4999999999999957]}
{"key":"dft/two-tones","values":[0,…,3,1,…,7,0.4999999999999966,…]}
{"key":"bode/second-order","values":[…,2.5,-90,…]}
{"key":"roots/cubic","values":[1,0,2,0,3,0]}
```

The gallery build found a third, worse one, also fixed: `\semio_viz_sci_clip:NNnnn` (Sutherland–Hodgman)
stored its *unexpanded* `n` argument in the output sequence, so after the first constraint the sequence
held `\seq_item:Nn \l_semio_viz_sci_pts_seq {k}` items pointing at the sequence they were copied back
into — a self-referential expansion that made `feasible-region` (7 catalogue kinds) never terminate.
Storing the expanded pair takes it from a hang to five seconds. Rendering the pages and looking at them
found a fourth: `sci-fieldlines` integrated its Coulomb streamlines for the full step budget regardless
of where they went, so a dipole's lines left the frame and ran across the page; the tracer now stops at
the window edge.

Two findings the numeric tests produced, both fixed:
- `sci-kinematics` in `projectile`, `orbital` and `phase-space` re-applied the graph-mode `domain`
  after fitting the window, so the flight ran far outside the canvas.
- `sci-survival` in `cumulative-hazard` drew the survival estimate on the hazard axis instead of
  −ln S.

## 6a. Gallery

Every family was rendered in a local xelatex build. The last run of the per-package galleries and of a
single document that loads all ten packages at once:

```
kernel errors=0 Output written on kernel.pdf (1 page)     bundle: FAMILYCOUNT=84
geom   errors=0 Output written on geom.pdf   (6 pages)    DEMOTABLES= … demo-science demo-science-field demo-science-graph
space  errors=0 Output written on space.pdf  (3 pages)
phys   errors=0 Output written on phys.pdf   (4 pages)
p2     errors=0 Output written on p2.pdf     (2 pages)
eng    errors=0 Output written on eng.pdf    (6 pages)
chem   errors=0 Output written on chem.pdf   (3 pages)
math   errors=0 Output written on math.pdf   (6 pages)
all    errors=0 Output written on all.pdf   (19 pages)    ← all 84 families in one document
```

Pages were rasterised and inspected, not just counted: the geometry page shows a triangle with its
angle arcs, a tangent construction, an ellipse with foci and a parabola with its directrix; the biology
page shows a coverage profile, a Manhattan plot with both significance lines and per-chromosome
colours, a sequence logo whose letters are scaled by information content, a Hi-C map with its diagonal
and its two off-diagonal blocks, a clinical timeline with therapy intervals and a relapse marker, and a
growth chart with percentile curves and the subject's own trajectory. Two defects found this way are in
§6; the field-line fix is why `all.pdf` is now 19 pages rather than 20.

## 7. Requests to other agents

- **TESTS-HARNESS** — please register these oracles so the four `@no-oracle-…` cases become
  differential against a third party: `mathjs` (expression evaluation, numeric integration, ODE
  integration), `jstat` or `@stdlib/stats` (distribution functions and the product-limit estimator),
  `fft.js` (a second Fourier transform for signals that do not land on bin centres), `gl-matrix`
  (projection matrices). The `noOracleDecisions` justifications are written in each feature file;
  `@no-oracle-l-system` needs no library — the growth rates are closed forms of the rules themselves.
- **GRAMMAR-CORE** — please take `%region 🔖️Pending-semio-viz-transform` (sampling, refinement,
  extents, Riemann sums, RK4, the seeded random stream, DFT, transfer-function evaluation,
  Durand–Kerner) and `%region 🔖️Pending-semio-viz-theme` (hue, sequential, diverging and cyclic
  ramps) and `%region 🔖️Pending-semio-viz-format` (bilingual text and number formatting) from
  `semio-viz-scientific-field.sty`. Note the language rule I applied: an unset `\l_semio_language_tl`
  raises `semio-viz-sci/no-language` rather than falling back to English.
- **GUIDES** — `%region 🔖️Pending-semio-viz-guide` holds the millimetre canvas, the data window and
  the frame with grid, ticks and axis titles.
- **SHAPES** — `%region 🔖️Pending-semio-viz-shape` holds polyline/area emission with probe geometry
  reporting; **`semio-viz-coordinate`** should take `%region 🔖️Pending-semio-viz-coordinate` from
  `semio-viz-scientific-3d.sty` (the five projections and the painter's-algorithm buffer).
- **GEO-SPATIAL** — `%region 🔖️Pending-semio-viz-spatial` in `semio-viz-scientific-field.sty` holds
  marching squares and Sutherland–Hodgman clipping. My `sci-field` and `sci-surface` presets call
  them today; when `semio-viz-geo`/`-spatial` expose contours, quiver, streamlines and density, I
  will switch the presets over — they are already isolated behind those two helpers.
- **CATALOG** — see the warning in §5.

## 8. Open issues

- Four of the 84 registered families have no catalogue kind of mine, because the leaves they serve were
  de-duplicated into another owner's section: `sci-timing` (§31 `timing-diagram`), `sci-forest`
  (§22 `forest-plot`, `funnel-plot` → §23, CHARTS-B), `sci-field` (§27, GEO-SPATIAL) and `sci-surface`
  (§28, GEO-SPATIAL). All four are implemented and reachable through `\SemioVizRunFamily`; the owning
  agents are welcome to point their entries at them, and if a leaf returns to my sections it needs no
  further work. `sci-field` and `sci-surface` are the §27/§28 domain presets the brief asked for and
  are meant to sit on top of GEO-SPATIAL's field and surface kernels once those exist.
- §34 `biodiversity-map` and `habitat-range-map`, §35 `weather-map`, `synoptic-chart`, `isobar-map`,
  `isotherm-map`, `pressure-contours`, `wind-barb-map`, `weather-fronts` and `tectonic-map`, §26
  `voronoi-diagram`, `delaunay-triangulation` and the hulls, §36 schematics and §40 flow/scheduling
  kinds are left to GEO-SPATIAL, DIAGRAMS and NETWORK-FLOW as the architecture assigns them; my
  mapping script does not touch their entries.
- The repo test runner (`bun ./📜️script.ts run …`) was not exercised: the TS probe module and the
  tectonic path are TESTS-HARNESS/BOOTSTRAP work in progress. The adapters are written against the
  published contract and the fixtures are verified; running them is one command away.
- Compilation cost, for whoever schedules the exhaustive run: a handful of kinds are lattice-bound and
  dominate a gallery page — `equipotential-map` (marching squares on 44×44 for seven levels, each cell
  evaluating the whole charge set), `domain-coloring-plot`, `interference-diagram`, `joint-density-plot`
  (a kernel density estimate per raster cell), `bifurcation-diagram` and `hi-c-contact-map`. Each is a
  single option away from a coarser lattice (`columns`, `rows`, `levels`, `bins`), so the gallery can
  ask for a cheaper resolution than a figure in a document would.

## 9. Files touched

Created or rewritten:

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-field.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-surface.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-signal.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-physics.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-chemistry.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-biology.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-engineering.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-mathematics.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-geometry.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-scientific-3d.sty`
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` (only the 322 entries of my sections)
- `🧰️framework/🛍️products/📓️print/🧪️tests/math-functions-sampling/{🥒️.feature,🟦️.ts,🧫️fixtures/math-functions-sampling.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/math-distributions/{🥒️.feature,🟦️.ts,🧫️fixtures/math-distributions.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/signal-dft-bode/{🥒️.feature,🟦️.ts,🧫️fixtures/signal-dft-bode.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/physics-projectile-rk4/{🥒️.feature,🟦️.ts,🧫️fixtures/physics-projectile-rk4.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/geometry-tilings-fractals/{🥒️.feature,🟦️.ts,🧫️fixtures/geometry-tilings-fractals.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/biology-kaplan-meier/{🥒️.feature,🟦️.ts,🧫️fixtures/biology-kaplan-meier.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/3d-projection/{🥒️.feature,🟦️.ts,🧫️fixtures/3d-projection.tex}`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/🏗️catalog-scientific.ts`
- this file

Not touched: the loader `semio-viz.sty`, `semio-viz-family.sty`, any kernel package, any other
agent's namespace package, and any catalogue entry outside my sections.
