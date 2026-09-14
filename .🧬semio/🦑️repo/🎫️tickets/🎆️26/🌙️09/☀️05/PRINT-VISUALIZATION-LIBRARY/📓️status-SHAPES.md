# SHAPES — status

Owner of `semio-viz-mark.sty`, `semio-viz-shape.sty`, `semio-viz-coordinate.sty`, the taxonomy
section 0 catalogue entries and the seven `mark-*` families, and six test cases.

**State: done.** Everything below was compiled and, where an oracle exists, compared against it.

## Done

### Geometry kernel (`semio-viz-mark.sty`, 2 900 lines)

- **A `d3-path` twin.** `M L C Q A Z` and `rect`'s `M h v h Z`, emitted in d3's own order with d3's
  own implicit move/line-to and two-arc full circle, plus one command beyond the twin (an SVG
  elliptical arc, which `d3-path` has no equivalent for). The buffer records d3-space numbers and
  builds the TikZ fragment in parallel, transformed by an offset and a y sign, so d3 geometry
  reaches the page right way up without the recorded numbers ever being touched.
- **Every `d3-shape` interpolator, transcribed state machine for state machine**: `linear`,
  `linearClosed`, `step`/`stepBefore`/`stepAfter`, `basis`/`basisOpen`/`basisClosed`,
  `bundle(beta)`, `cardinal(tension)`/`cardinalOpen`/`cardinalClosed`,
  `catmullRom(alpha)`/`catmullRomOpen`/`catmullRomClosed`, `monotoneX`/`monotoneY`, `natural`,
  plus semio's own `bezier` (the point list read as a cubic control polygon). `lineStart`,
  `areaStart` and `areaEnd` are exposed so an area runs both boundaries through one interpolator,
  exactly as d3 does.
- **`d3-symbol`'s thirteen draw routines** with their own constants; `size` is an area.
- **`d3-arc`** in full: padding, `padRadius`, `cornerRadius`, the corner-tangent closed form, the
  collapsed sector, the full ring and the counter-clockwise case. Plus `d3-pie`, `d3-shape`'s three
  link generators and `d3-chord`'s ribbon.
- **All fifty-six section 0 primitives** as real, parameterised marks — points, lines, areas, arcs,
  connections, regions and annotation primitives — every one of them with geometry of its own and
  all fifty-six pairwise distinct (checked by the `mark-geometry` case). Convex hulls are Andrew's
  monotone chain; concave hulls are chi-shape digging with a `concavity` key.
- **Geometry probing**: every mark emits `geometry/mark/<kind>/at` (point, rotation, area),
  `geometry/mark/<kind>/style` (fill, stroke, dash, arrow, pattern, opacity) and
  `geometry/mark/<kind>` (its SVG path) through `\semio_viz_probe_geometry:nn` when probing is on.
- **The seven `mark-*` families** (`mark-point`, `-line`, `-area`, `-arc`, `-connector`, `-region`,
  `-annotation`) that the section 0 catalogue entries name, registered at `begindocument` because
  the family registry loads after the kernel. Each reads its bound table through
  `\semio_viz_table_cell:nnnN` — the marks sit where the data puts them.

### Generators (`semio-viz-shape.sty`)

`\SemioVizLine` (with `defined`), `\SemioVizArea` (two boundaries or a baseline, both through the
curve), `\SemioVizArc` (+ `\SemioVizArcCentroid`), `\SemioVizPie` (+ `\SemioVizPieStart/End/Count`),
`\SemioVizLink`, `\SemioVizSymbol`, `\SemioVizRibbon`. Each publishes its path in
`\g_semio_viz_shape_result_tl` and, with `name=`, under `\SemioVizShapePath{name}`; `draw=false`
computes the numbers without putting ink on the page. Every key the generators do not own is
forwarded to `semio / viz / mark`, so they take the full mark style vocabulary without duplicating it.

### Coordinate systems (`semio-viz-coordinate.sty`)

`cartesian` (origin, domain, flipped y), `polar` and `logpolar` (start/end angle, inner/outer
radius, direction, log base), `ternary` and `barycentric` (a at the apex), `parallel` (axis index →
x), `geographic` (the hook `\semio_viz_coordinate_project:nnNN`, with an equirectangular fallback
so a geographic chart is drawable before GEO-SPATIAL installs the real pipeline). One entry point,
`\semio_viz_coordinate_map:nnNN {x-data}{y-data} <fp x> <fp y>`, plus `\SemioVizProject` /
`\SemioVizProjected`.

### Catalogue

All 56 section 0 entries were already present with `family: mark-*` and `options: {variant}`; the
two hull entries were moved off `spatial` onto `mark-region` so section 0 is served entirely by
families this package owns. `d3-chord`, `d3-geo`, `d3-path` and `d3-shape` added as devDependencies
of `📦️packages/🟦️typescript` (all four were already registered oracles and installed).

## Tests (`🧰️framework/🛍️products/📓️print/🧪️tests/`)

| case | scenarios | oracle | fixture result |
|---|---|---|---|
| `shape-curves` | interpolators, parameters, degenerate | `d3-shape` | 26 records, **all match** |
| `shape-arc-pie` | arc-geometry, arc-centroid, pie-angles | `d3-shape` | 19 records, **all match** |
| `shape-symbols` | symbol-paths (13 types × 3 areas) | `d3-shape` | 39 records, **all match** |
| `shape-links-ribbons` | links, ribbons | `d3-shape`, `d3-chord` | 8 records, **all match** |
| `coordinate-polar-ternary` | cartesian, polar, ternary, parallel, geographic | `@no-oracle` | 21 records, conformance |
| `mark-geometry` | primitives | `@no-oracle` | 171 records, 56 kinds, 56 distinct |

Every fixture compiles with **zero errors** under xelatex, and every differential fixture was
compared record for record against the d3 package named in its feature, on the six-decimal emission
grid at 1e-6 relative tolerance. The comparison harness itself is not yet runnable here
(`🔨️modules/🧪️viz-probe/🟦️.ts` + tectonic are TESTS-HARNESS's), so the adapters are written
against the published contract and the same comparison was performed locally with node one-liners.

### Requests to TESTS-HARNESS

- Two `noOracleDecisions` entries, both justified in their feature text:
  - `viz-coordinate` — d3 has no ternary, parallel or log-polar coordinate system, and its polar
    geometry lives inside the shape generators rather than in a queryable mapping. Specification
    vectors from the package's own documented formulas; the geographic rows exercise only the
    equirectangular fallback, since the real projection pipeline is GEO-SPATIAL's and is
    adjudicated by `d3-geo` there.
  - `viz-mark-geometry` — no third-party library draws a taxonomy of print marks. Conformance
    against the taxonomy itself: every section 0 slug present, and no two of them drawing the same
    record. The distinctness half is the one that bites, and it is what caught nine primitives that
    were still sharing an outline.
- New capability tags used: `viz-shape-curves`, `viz-shape-arc`, `viz-shape-symbol`,
  `viz-shape-link`, `viz-coordinate`, `viz-mark-geometry`.

## Decisions

- **`semio-viz-mark.sty` is the geometry kernel.** The loader order is kernel → mark → shape →
  coordinate and nobody edits the loader, so the numeric machinery (path buffer, interpolators,
  symbol and arc geometry) lives in the package that loads first; `semio-viz-shape.sty` exposes the
  public generators on top of it.
- **The probe record for a path is the SVG token stream d3 would have written** — command letters
  as strings, coordinates as numbers. The oracle therefore only tokenises d3's own output instead
  of re-implementing anything, which is what makes these differential rather than self-referential.
- **Two spaces.** The buffer is d3/SVG space (y down, angles radians clockwise from twelve
  o'clock); rendering applies `(ox + x, oy ± y)`. Generator probes are d3-exact; mark probes carry
  the placement separately in the `/at` record.
- **`strokeWidth` is the single stroke-width key.** The briefing named `width=` for `\SemioVizPath`
  and `strokeWidth=` for marks; two names for one thing is an alias, which CLAUDE.md forbids.
- **Text-carrying marks stay on `\SemioVizText`** (kinds extended with `callout` and `leader`),
  never a `text=` key — l3keys splits on commas and German prose is commas.
- `cross` mark = d3's `symbolTimes` (×), `plus` mark = d3's `symbolPlus` (+); `\SemioVizSymbol`
  keeps d3's own meaning of `cross` (the filled plus polygon). Taxonomy and d3 disagree on the
  word; both vocabularies stay intact in their own namespace.
- **Nine section 0 leaves used to share an outline** and were given geometry of their own rather
  than being declared equivalent: a straight connector terminates on discs where a straight line
  does not, a file-less image mark shows its crossed placeholder, an elliptical arc given only a
  radius is flattened, and a curved line is a cardinal spline where the Catmull–Rom spline is the
  named one. The remaining pairs (dot/circle, polygon/filled-path, rectangular/highlight region,
  brace/reference line) differ in ink, which is why the mark probe records style beside geometry.

## Traps found (worth knowing for the other agents)

- **expl3 names must not contain digits.** `\l_..._x0_fp` silently truncates at `x` and the rest
  becomes document text. Every d3 state variable (`_x0.._x5`, `_l01_2a`, `slope3`) needed a letter
  suffix.
- **`\prop_put:Nnn` does not evaluate its key**, so `{ \int_eval:n { n - 1 } }` becomes a literal
  string key that never matches the plain integers a loop writes. Integer-keyed arrays need
  `\prop_put:Nxx` / `\prop_item:Ne`.
- **`&&` inside a boolean expression is not short-circuited** — `\bool_lazy_and_p:nn` is. A guard
  written as `count > 1 && item(count-1)` evaluates the second half anyway and errors.
- **`\bool_do_while:nn` runs the body first**; the while-first form is `\bool_while_do:nn`.
- **`\fp_compare:nTF` takes a comparison chain, not a boolean expression** — `||` and `&&` inside
  it parse as fp operators and give the wrong answer without erroring.
- **`\l_tmpa_fp` / `\l_tmpb_fp` cannot carry a value into a routine that also uses them as
  scratch**; the catmullRomClosed replay clobbered itself that way.
- **A key value reaching `\keys_set` through an `unknown` handler must be re-braced**, or a
  `points={1,2; 3,4}` list is split into keys again.

## Files touched

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-mark.sty` (rewritten)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-shape.sty` (written)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-coordinate.sty` (written)
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` (the two section 0 hull entries)
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/package.json` (four d3 devDependencies)
- `🧰️framework/🛍️products/📓️print/🧪️tests/shape-curves/` (feature, adapter, fixture)
- `🧰️framework/🛍️products/📓️print/🧪️tests/shape-arc-pie/` (feature, adapter, fixture)
- `🧰️framework/🛍️products/📓️print/🧪️tests/shape-symbols/` (feature, adapter, fixture)
- `🧰️framework/🛍️products/📓️print/🧪️tests/shape-links-ribbons/` (feature, adapter, fixture)
- `🧰️framework/🛍️products/📓️print/🧪️tests/coordinate-polar-ternary/` (feature, adapter, fixture)
- `🧰️framework/🛍️products/📓️print/🧪️tests/mark-geometry/` (feature, adapter, fixture)

## Open

- `\semio_viz_mark_demo:n` and `\semio_viz_path_demo:n` are alive only for the legacy
  `semio-viz-layout.sty`; **CATALOG**, report the removal in `📓️status-CATALOG.md` and they go with
  it, together with the legacy `\semio_viz_polar:nn` helper that package still calls.
- The concave hull digs without a segment-intersection test; the two distance constraints keep it
  well behaved on the vectors tested, and the case is declared `@no-oracle`. If GEO-SPATIAL grows a
  real chi-shape in `semio-viz-spatial`, the mark should delegate to it.
- The bundle build (`semio-viz.sty` with the full `semio` document class) reports a pre-existing
  duplicate family registration for `sunburst` from another namespace package, and the repo's fonts
  are not generated yet; neither comes from these three packages.
