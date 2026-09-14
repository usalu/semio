# TS-TWIN — status

Agent: TS-TWIN. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`. Milestone 4 (architecture §7):
**the TypeScript twin of the LaTeX viz kernel**, usable from code like d3, numerically identical to
d3, and available as a second subject implementation for the kernel's algorithms.

Location, mine entirely: `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/`
Package: `@semio-tech/print-viz-kernel` at `📊️viz-kernel/📦️packages/🟦️typescript`
Harness: `bun ./📜️script.ts test [quick|long|exhaustive] [module…]`

**Done: 13 of 13 modules, 237 differential checks, 0 failures.** Registered in
`🔨️modules/🔣️.json` and in the root `workspaces`; `bun install` run; the nx target
`@semio-tech/print-viz-kernel:test-quick` is green; the platform case `🧪️tests/render-scene/` is
green. No other agent's file was modified except the three registration points named below.

## ⚠️ Working-tree sweep (read this if files of mine go missing)

Between two of my tool calls, every untracked file I had created under `📊️viz-kernel/` disappeared
from the working tree (`🧬️schema/🟦️.ts`, `📐scale/🟦️.ts`, `package.json`, `📋️project.json`,
`🟦️.ts`, and all empty module directories), while `📜️script.ts` — written moments later — survived
and is `AM` in the index. That is the signature of a `git add -A` plus `git clean -fd` sweep running
in this repository while the fleet works. I rewrote everything and now mirror the finished module
into the session scratchpad (`…/scratchpad/viz-kernel/`) after every green run, so a second sweep
costs one `cp -r`, not a rewrite. **Nobody should run `git clean` on this tree.**

A second, unrelated environment quirk: writing a file whose name is an emoji grapheme through
Python's `io.open(..., "w")` fails intermittently on this machine with `OSError: [Errno 22] Invalid
argument`, while reading the same path succeeds. Write to a plain-named temporary file and `mv` it.

## Modules

| module | checks | oracle |
|---|---|---|
| `🧬️schema/🟦️.ts` | — | typed twin of the §79 grammar vocabulary |
| `📐scale/` | 18 | `d3-scale`, `d3-array`, `d3-time` |
| `🔢format/` | 41 | `d3-format`, `d3-time-format` (plus `de` conformance) |
| `🧮transform/` | 20 | `d3-array`, `d3-shape` |
| `✒️mark/` | 58 | `d3-shape` |
| `🥧shape/` | 11 | `d3-shape`, `d3-chord` |
| `🧭coordinate/` | 8 | conformance and round-trip (d3 has no coordinate layer) |
| `🌳hierarchy/` | 17 | `d3-hierarchy` |
| `🕸️network/` | 8 | `d3-force`, `d3-chord`, plus invariants for the graph layouts |
| `🌊flow/` | 6 | `d3-sankey` |
| `🌍geo/` | 32 | `d3-geo` |
| `📍spatial/` | 9 | `d3-delaunay`, `d3-hexbin`, `d3-contour` |
| `🎨theme/` | 4 | the token generator that also writes `semio-tokens.sty` |
| `🖼️render/` | 5 | specification vectors (`@no-oracle-render-scene`) |

What is inside, in the vocabulary of architecture §3:

* **📐scale** — `linear, log, pow, sqrt, symlog, identity, ordinal, band, point, quantile, quantize,
  threshold, sequential, diverging, temporal`, with `nice`, `clamp`, `round`, piecewise domains,
  `invert`, the d3-array tick algorithm (`ticks`, `tickIncrement`, `tickStep`, both bisectors) and
  the eight d3-time calendar intervals with `every`, so `scaleTemporal` ticks on real boundaries.
* **🔢format** — the whole d3-format specifier grammar (fill, align, sign, symbol, zero, width,
  comma, precision, `~`, all thirteen types), the SI prefix table, `formatPrefix`, the three
  precision suggestions, and the d3-time-format directives — each in `en` and `de`, no default.
* **🧮transform** — `fsum` (Neumaier), sum, mean, Welford variance and deviation, R-7 quantile,
  extent, cumulative sum; `bin` with Sturges, Scott and Freedman–Diaconis and d3's exact domain
  nicing; group, rollup, pivot, fold, window, normalize; KDE with four kernels and Silverman's
  bandwidth; linear and polynomial regression; and `stack` with **all six orders and all five
  offsets**.
* **✒️mark** — a recording path context and **23 curve interpolators** (linear, closed, three steps,
  three basis, bundle, three cardinal, three Catmull–Rom, both monotones, natural, both bumps) plus
  **13 area-sized symbols**.
* **🥧shape** — line, area with gaps, arc with pad angle *and* rounded corners, `arc.centroid`, pie
  with d3's sort semantics, the three link kinds, ribbon, and the stack re-export.
* **🧭coordinate** — cartesian, polar, log-polar, ternary, barycentric, parallel and geographic,
  each with its inverse where one exists.
* **🌳hierarchy** — hierarchy and stratify, the traversal and aggregation vocabulary, the Buchheim
  tidy tree, the cluster dendrogram, **all six treemap tilings**, the icicle partition, and Welzl
  circle packing driven by the same seeded LCG as `semio-viz-hierarchy.sty`.
* **🕸️network** — a full d3-quadtree (cover, add, visit, visitAfter), the deterministic force
  simulation with link, many-body (Barnes–Hut), center, collide, x, y and radial; the chord layout;
  and the circular, arc and layered (Sugiyama) graph layouts.
* **🌊flow** — the Sankey layout with all four alignments and d3's relaxation schedule, its
  horizontal ribbons, and the alluvial diagram over ordered categorical stages.
* **🌍geo** — **fourteen projections** with d3's rotate → raw → scale/translate pipeline and their
  inverses, the graticule, a GeoJSON path renderer, planar bounds, area and centroid, `fitExtent`,
  `fitSize`, and great-circle distance.
* **📍spatial** — Delaunay (Bowyer–Watson), Voronoi by half-plane clipping, the convex hull,
  hexagonal binning, marching-squares contours with d3's smoothing and hole assignment, and 2D KDE.
* **🎨theme** — the palettes are **not restated**: they are parsed out of
  `renderVisualizationPalette()`, the same generator that writes `semio-tokens.sty`, so the LaTeX
  and the TypeScript subject cannot drift apart. Plus colour parsing, sRGB interpolation, WCAG
  luminance and contrast, and the dash and hatch encodings that carry a series in a grayscale print.
* **🖼️render** — a §79 chart specification into the dependency-free scene graph of
  `🧰️framework/🔨️modules/◻️2d` **and**, from the same resolved item list, into TikZ source text.

## Decisions

1. **No second script file.** CLAUDE.md forbids script files other than `📜️script.ts`, so the whole
   differential harness — the check table, the tolerant comparison, the level selector — lives
   inside the package's `📜️script.ts`. That is also what keeps d3 out of the library: the modules
   import nothing outside this repository, the script imports d3 as a devDependency only.
2. **The curve and shape checks are not string comparisons.** Both sides are driven through the
   kernel's own `vizPathRecorder`, a canvas-shaped path context, so d3 and the twin record the same
   command stream and it is compared numerically, command by command. Diffing an SVG `d` attribute
   would have measured number formatting instead of geometry.
3. **`◻️2d` is imported type-only.** That file itself imports `@semio-tech/framework` at run time;
   `import type` erases, so the render module emits `DrawingScene` values without acquiring the
   dependency and the kernel stays runtime-dependency-free.
4. **The LCG is d3's**, `s ← (1664525·s + 1013904223) mod 2³²`, seed 1, value `s / 2³²` — identical
   to `\semio_viz_hierarchy_lcg_step:` and `\semio_viz_network_random:`. Pack and force therefore
   consume the same stream as the LaTeX subject; `hierarchy/pack` and `hierarchy/lcg-stream` hold it
   there.
5. **No new oracle package.** `d3-quadtree` would have been the natural oracle for the quadtree, but
   it is not in `🔮️oracle/🔣️.json` and adding one is a surveyed decision belonging to
   TESTS-HARNESS. The quadtree is instead checked by its own invariants, and adjudicated in
   substance by the three force checks, which reproduce d3-force to 1e-6 and could not do so if the
   Barnes–Hut tree differed.
6. **Bugs the differential checks caught.** Each vector is in the table because of one of these:
   * `niceDomain` must leave the domain untouched when the tick increment never stabilises — d3 only
     writes the extended bounds in the `step === prestep` branch. Assigning unconditionally niced
     `[12, 12]` to `[NaN, NaN]`. → `scale/linear-nice`.
   * `precisionRound(step, max)` subtracts the step from the max *before* taking the exponent.
   * `formatPrefix` takes the exponent of the value, not of a precision estimate.
   * `%W` and `%U` count week boundaries in `(Dec 31, date]`, so a year that begins on the week's
     first day is in week 1, not week 0. → `format/time-en-%W` on 2024-02-29.
   * A projection's centre offset is `raw(center)`, **not** `rotate ∘ raw (center)`. With that wrong,
     every conic and every rotated projection is wrong. → `geo/project-albers`.
   * `conicProjection`'s default parallels are `[0, 60]` (`π/3`), not `[0, 0]`.
   * `contours` nices the value extent before taking ticks. → `spatial/contours-default-thresholds`.
7. **A GeoJSON polygon must wind counter-clockwise.** d3-geo reads a clockwise ring as the
   complement of the region — which is why the demo polygon in the geo checks winds the way it does.

## The second-subject question, answered

Full write-up in `📓️integration.md` under `## From TS-TWIN`. In short: **a second `typescript`
subject cannot be added to an existing print case** — `testAdapterFileKinds` maps one adapter
filename per language, `TestAdapter` gives each scenario exactly one `subject`, every print case's
single adapter is `🟦️.ts` with `implementation: "typescript"` and its subject already *is* the
LaTeX probe, and `evaluateCrossSubjectParity` only ever compares *different* implementations. Three
routes exist instead; the one I recommend is folding the twin into the existing subject handler so
it asserts against the LaTeX projection before that projection is returned. That edits files owned
by the namespace agents, so it is a request to INTEGRATION-2, not something I did unilaterally.
Route 3 — a new case under print's `🧪️tests/` whose subject is the twin — is proven green by
`render-scene`.

## Verified output tails

```
$ cd 🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript
$ bun ./📜️script.ts test long
[viz-kernel] coordinate  passed=8 failed=0
[viz-kernel] flow        passed=6 failed=0
[viz-kernel] format      passed=41 failed=0
[viz-kernel] geo         passed=32 failed=0
[viz-kernel] hierarchy   passed=17 failed=0
[viz-kernel] mark        passed=58 failed=0
[viz-kernel] network     passed=8 failed=0
[viz-kernel] render      passed=5 failed=0
[viz-kernel] scale       passed=18 failed=0
[viz-kernel] shape       passed=11 failed=0
[viz-kernel] spatial     passed=9 failed=0
[viz-kernel] theme       passed=4 failed=0
[viz-kernel] transform   passed=20 failed=0
[viz-kernel] level=long checks=237 passed=237 failed=0 errored=0

$ bun ./📜️script.ts test exhaustive
[viz-kernel] level=exhaustive checks=237 passed=237 failed=0 errored=0

$ bun ./📜️script.ts build
[viz-kernel] barrel exports 229 symbols
```

```
$ cd C:/git/semio && bun nx run @semio-tech/print-viz-kernel:test-quick
> bun ./📜️script.ts test quick
[viz-kernel] coordinate  passed=8 failed=0
… thirteen module rows …
[viz-kernel] level=quick checks=209 passed=209 failed=0 errored=0

 NX   Successfully ran target test-quick for project @semio-tech/print-viz-kernel
```

```
$ cd C:/git/semio && bun install
bun install v1.4.2 (744846f84)
Saved lockfile
$ bun ./📜️script.ts setup postinstall
1 package installed [1110.00ms]
```

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print" --case render-scene
[test] level=quick cases=1 executed=4 passed=4 failed=0 errored=0 parity=0/0
```

Print's own `📦️packages/🟦️typescript` router does **not** reach the platform at `quick` yet, and it
is not my doing — it aborts before it, in CATALOG's coverage check:

```
$ cd 🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript && bun ./📜️script.ts test quick
AssertionError: Expected values to be strictly deep-equal:
     actual: [ "\\SemioVizChartKind" ]
   expected: []
  at verifyVisualizationCoverage (…/🔨️modules/📊️visualization-gallery/🟦️.ts:347:10)
  at verifyPrintPipelineQuick (…/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts:119:3)
```

A surviving `\SemioVizChartKind` registration is exactly the legacy deletion INTEGRATION-2 still
owns (TESTS-HARNESS §4 records the sibling failure at `fundamental`). It is upstream of anything
this ticket's milestone 4 adds, so the twin's evidence is the two runs above plus the platform case;
this router will pass once the legacy registrations are gone.

`parity=0/0` is correct for a recorded no-oracle case: `runPhases` skips the oracle phase when
`decision.implementation === null`, exactly as it does for `facet-layout` and `composition-geometry`.
Because nothing else would then compare anything, **the `render-scene` subject compares itself
against the feature's vectors and throws, naming every disagreeing key** (`assertMatches` in the
adapter). That is what makes the four scenarios evidence rather than decoration.

Existing print cases, the whole sweep re-run after my registration edits (LaTeX subjects, untouched
by me), one `parity quick --owner "🧰️framework/🛍️products/📓️print" --case <slug>` each:

```
scale-continuous             cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
scale-discrete               cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6
format-number                cases=1 executed=4  passed=4  failed=0 errored=0 parity=2/2
transform-stack              cases=1 executed=12 passed=12 failed=0 errored=0 parity=6/6
shape-arc-pie                cases=1 executed=6  passed=3  failed=0 errored=3 parity=0/3
hierarchy-treemap            (no summary — the run aborted, see below)
network-force                cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
flow-sankey                  cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
geo-projections              cases=1 executed=6  passed=6  failed=0 errored=0 parity=3/3
spatial-delaunay-voronoi     cases=1 executed=4  passed=4  failed=0 errored=0 parity=2/2
```

Eight of the ten are green. The two that are not are **not mine and not caused by me** — my only
edits outside `📊️viz-kernel/` and `🧪️tests/render-scene/` are one `workspaces` line, one
`🔨️modules/🔣️.json` member and one `noOracleDecisions` entry, none of which any of these cases
reads. For the record, so the owners do not have to rediscover them:

* **`shape-arc-pie`** — three of six scenarios `errored` (not `failed`), so the subject never
  produced a projection. Owner: SHAPES / INTEGRATION-2.
* **`hierarchy-treemap`** — the phase aborts with `error: spawnSync bun ETIMEDOUT` out of
  `runPhases`, i.e. the harness's own subprocess deadline expired while the scenario was compiling
  its probe with tectonic. It is a harness timeout on a slow LaTeX compile, not an arithmetic
  disagreement. Owner: HIERARCHY / TESTS-HARNESS. Worth noting that the twin reproduces
  `d3-hierarchy`'s treemap on all six tilings, so if the LaTeX case is ever adjudicated it now has a
  second subject to be adjudicated against.

## Files touched

Created
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🧬️schema/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📐scale/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🔢format/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🧮transform/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/✒️mark/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🥧shape/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🧭coordinate/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🌳hierarchy/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🕸️network/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🌊flow/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🌍geo/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📍spatial/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🎨theme/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/🖼️render/🟦️.ts`
* `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/{package.json,📋️project.json,📜️script.ts,🟦️.ts}`
* `🧰️framework/🛍️products/📓️print/🧪️tests/render-scene/{🥒️.feature,🟦️.ts}`
* `.🧬semio/…/PRINT-VISUALIZATION-LIBRARY/📓️status-TS-TWIN.md`

Updated (three registration points, each re-read immediately before writing)
* `package.json` — one `workspaces` line, nothing reordered
* `🧰️framework/🛍️products/📓️print/🔨️modules/🔣️.json` — one member, existing order preserved
* `🧰️framework/🛍️products/📓️print/🔮️oracle/🔣️.json` — one `noOracleDecisions` entry
* `.🧬semio/…/PRINT-VISUALIZATION-LIBRARY/📓️integration.md` — `## From TS-TWIN` appended
* `bun.lock` — by `bun install`

## Open

* The twin is not yet wired into the existing print cases as a cross-check of the LaTeX subject;
  that needs INTEGRATION-2's go-ahead (see `📓️integration.md`).
* `🖼️render` covers the marks the demo specification exercises — bar, point, line, area, arc, text,
  rule. Facets, legends and annotations are guides the LaTeX kernel draws and the twin does not yet;
  they are additive and do not affect anything shipped.
* The layered (Sugiyama) layout is checked against its own invariants, not against `dagre`. `dagre`
  is a registered oracle, so a differential case is possible later, but its coordinate assignment is
  a different published algorithm and comparing positions would measure the wrong thing.
