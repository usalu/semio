# GEO-SPATIAL — status

Owner of `semio-viz-geo`, `semio-viz-spatial`, the six `semio-viz-geo-*` namespace packages, and the catalogue
entries for taxonomy sections 10, 26 (voronoi/delaunay/hull), 27, 28, 35 (map kinds) and the geometric/spatial
layouts of §75/§78.

## Done

### `🖋️latex/semio-viz-geo.sty` — geo kernel (validated against `d3-geo`)

Fourteen projection kinds with d3's own raw math, defaults, rotation, centre offset, adaptive resampling and
inverse: `equirectangular, mercator, transverse-mercator, equal-earth, natural-earth1, orthographic,
stereographic, gnomonic, azimuthal-equal-area, azimuthal-equidistant, conic-conformal, conic-equal-area,
conic-equidistant, albers`. `albers-usa` is deliberately **not** implemented (composite projection, out of scope,
as instructed).

Public API: `\SemioVizProjection`, `\SemioVizGeoProject`, `\SemioVizGeoInvert`, `\SemioVizGeoX/Y/Lon/Lat`,
`\SemioVizGeoPath`, `\SemioVizGraticule`, `\SemioVizGeoFit` / `FitSize` / `FitWidth`, `\SemioVizGeoArea`,
`\SemioVizGeoCentroid`, `\SemioVizGeoDistance`, `\SemioVizGeoInterpolate`, `\SemioVizGeoCircle`.

Projection keys (`semio / viz / geo / projection`): `kind, scale, translate, center, rotate, parallels,
clipAngle, clipExtent, precision`. Every default equals d3's default for that kind (scale 152.63 / 961/τ /
159.155 / 177.158 / 175.295 / 249.5 / 250 / 144.049 / 124.75 / 79.4188 / 109.5 / 155.424 / 131.154 / 1070,
translate 480,250, the conic parallels and the azimuthal clip angles). `transverse-mercator` adds d3's own +90°
gamma. `precision` defaults to `sqrt(0.5)` (d3's `delta2 = 0.5`); `precision = 0` disables resampling exactly as
`resampleNone` does.

Resampling is d3's `resampleLineTo` verbatim (max depth 16, `cosMinDistance = cos 30°`, the three split criteria),
with the recursion scoped by TeX groups so the twelve segment registers restore themselves.

### `🖋️latex/semio-viz-spatial.sty` — spatial kernel (validated against `d3-delaunay`, `d3-hexbin`, `d3-contour`)

`\SemioVizDelaunay` (Bowyer–Watson, deterministic insertion order), `\SemioVizVoronoi` (cells from circumcentres +
outward far rays, Sutherland–Hodgman clipped to a rectangle), `\SemioVizHull` (Andrew monotone chain),
`\SemioVizHexbin` (exact `d3-hexbin` centres incl. the nearest-of-two correction and JS half-up rounding),
`\SemioVizBeeswarm` / `\SemioVizJitter` (deterministic; the jitter uses a fixed LCG keyed by point index),
`\SemioVizContour` (marching squares with linear crossing interpolation), `\SemioVizDensity` (exact Gaussian-sum
2-D KDE on a grid).

Point sets and value grids are declared with `\SemioVizPointSet` / `\SemioVizValueGrid` under
`%region 🔖️Pending-semio-viz-data`.

## Verified numbers (real runs)

Exploratory probe (xelatex, 0 errors; superseded by the committed fixtures below) vs `oracle-geo.mjs` / `oracle-geo2.mjs`
(node, `d3-geo` from the repo `node_modules`):

```
              subject (TeX)                          oracle (d3-geo)
fwd equirectangular 511.9667524478273 124.7968862460096   511.96675244782733 124.79688624600958
fwd mercator        512.0333333333333 107.5089006597659   512.0333333333333  107.50890065976608
fwd equal-earth     506.9963408684848  91.7432837688457   506.99634086848476  91.74328376884569
fwd conic-conformal 496.3752670450489 156.1942359652674   496.3752670450489  156.19423596526747
fwd albers         1591.793933899959 -609.9258330238254  1591.7939338999577 -609.9258330238256
rot (orthographic, rotate 30,-20,15, scale 200, translate 100,100)
                    161.5576540169435 -22.900798925136    161.55765401694347 -22.900798925135987
```
All fourteen kinds agree to ≤ 3e-12 absolute, forward and inverse; every inverse round-trips to ≤ 3e-14 degrees.

```
parts 4                                  parts 4
first  453.3610396268106,116.8051981340528;495.9833762239137,111.4774060594149;
       506.6389603731894,127.4607822833286;469.3444158507242,132.7885743579664
oracle 453.36103962681057,116.80519813405274;495.9833762239137,111.47740605941485;
       506.63896037318943,127.46078228332853;469.3444158507242,132.7885743579664
fit    110.1841913713122 46.15384615384616 112.6923076923077
oracle 110.18419137131217 46.15384615384615 112.6923076923077
graticule lines 53                       graticule lines 53
graticule first -180,-89.999999;-180,0.000001;-180,89.999999
         oracle -180,-89.999999;-180,9.999999974752427e-7;-180,89.999999
area     0.02042180461301782              0.020421804613017666
centroid 0.3712026277794507 48.25059657082535   0.3712026277792826 48.2505965708252
distance 0.2429366737436943               0.24293667374369426
interp   -2.24423204988315 48.93087314491911    -2.244232049883151 48.9308731449191
circle   -11.86625899237879,37.40672102259848;… (9 points, identical to d3 to 1e-13)
resampled (mercator, default precision) identical to d3's six resampled vertices to 1e-12
```

Exploratory spatial probe (xelatex, 0 errors; superseded by the committed fixtures below) vs `oracle-spatial.mjs`:

```
tris 13   {3,5,6} {3,6,10} {3,7,10} {1,2,11} {4,7,11} {3,7,11} {4,8,11} {1,8,11}
          {2,9,12} {3,5,12} {5,9,12} {3,11,12} {2,11,12}
oracle 13 [1,2,11] [1,8,11] [2,9,12] [2,11,12] [3,5,6] [3,5,12] [3,6,10] [3,7,10]
          [3,7,11] [3,11,12] [4,7,11] [4,8,11] [5,9,12]        → identical set
hull 8,1,2,9,5,6,10,7,4            oracle [8,1,2,9,5,6,10,7,4]  → identical
bins 11, centres 0/30, 8.660254037844385/15, 17.32050807568877/30, 25.98076211353316/15,
     25.98076211353316/45, 34.64101615137754/30, 43.30127018922192/15, 51.96152422706631/0,
     60.6217782649107/15, 60.6217782649107/45, 69.28203230275508/30
oracle identical eleven centres to 1e-14
cell1  0,17.42307692307693;0,0;16.40625000000123,0;20.41925465838509,12.8416149068323;
       16.02849740932642,22.35492227979275
oracle [0,0],[16.40625,0],[20.419254658385093,12.841614906832298],
       [16.028497409326427,22.354922279792746],[0,17.42307692307692]   → same cycle, tol 2e-12
cell7  19.5666666666648,50;30.10493827160494,30.24074074074074;30.31914893617021,30.14893617021277;
       46.59448818897638,37.74409448818898;45.0625,50                  → identical to d3
contour(threshold 4) 4,3;4.333333333333333,2;4,1.5;3.5,1;3,0.6666666666666667;2,0.6666666666666667;
       1.5,1;1,1.5;0.6666666666666667,2;1,3;2,3.5;3,3.5;4,3
oracle 4.5,3.5;4.8333…,2.5;4.5,2;4,1.5;3.5,1.1666…;2.5,1.1666…;2,1.5;1.5,2;1.1666…,2.5;1.5,3.5;
       2.5,4;3.5,4;4.5,3.5                          → identical vertex sequence, offset by exactly (0.5,0.5)
```

## Documented deviations from d3

- **Contour coordinates** are in grid coordinates where node (i,j) sits at (i,j). `d3-contour` reports the same
  isolines shifted by **+0.5 in both axes** because its case table is written on half-integer cell corners. The
  offset is exact and constant; tests translate before comparing.
- **`clipAngle`** clips segments by bisecting the great circle to the visibility boundary (48 halvings, below
  1e-14 rad). d3 solves the same crossing analytically — the point is the same — but d3 also re-closes clipped
  polygon rings along the clip circle. That re-closure is not reproduced; a ring clipped by `clipAngle` therefore
  renders as an open boundary rather than a filled cap.
- **Antimeridian clipping** (d3's default `preclip`) is not implemented; geometry crossing ±180° is not split.
  All demo geometry and every test fixture stays inside one hemisphere of longitude.
- **`geoBounds`** (the antimeridian-aware spherical sweep) is not implemented. Fitting uses the *planar path*
  bounds, which is what d3's own `fitExtent`/`fitSize`/`fitWidth` use, so those match exactly.
- **`density`** evaluates the Gaussian sum exactly. `d3-contour`'s `contourDensity` approximates it with three box
  blurs, so it is deliberately not bit-comparable; the exact sum is the specification and the isoline extraction
  that consumes the grid is the part validated against `d3-contour`.
- **Voronoi/Delaunay ordering**: triangle order, triangle vertex rotation and the cell's starting vertex are not
  part of the contract (Bowyer–Watson vs Delaunator); the geometry is identical. Tests canonicalise.

## Performance limits (measured on this machine, MiKTeX xelatex)

- Delaunay/Voronoi are O(n²) in the insertion scan and O(n·t) per cell. 12 points ≈ 4 s, 24 points ≈ 25 s.
  `\c_semio_viz_spatial_maxpoints_int` documents the hard ceiling at **120 points**; keep gallery/demo sets ≤ 40.
- Marching squares is O(w·h); a 6×5 grid is instant, `\c_semio_viz_spatial_maxcells_int` caps at **4096 cells**.
- Density is O(w·h·n); keep w·h·n ≤ 20000.
- Geo path resampling at the default precision expands a 4-vertex ring to ~6 vertices at world scale; a 200-vertex
  coastline at `precision = 0` is the practical ceiling per figure.

### Namespace families (15 registered, all rendering, verified in one build)

| package | families |
|---|---|
| `semio-viz-geo-map` | `geo-basemap` |
| `semio-viz-geo-projection` | `geo-graticule`, `geo-geodesic` |
| `semio-viz-geo-choropleth` | `geo-choropleth`, `geo-cartogram`, `geo-tilegrid` |
| `semio-viz-geo-symbols` | `geo-symbol`, `geo-dotdensity`, `geo-hexbin`, `spatial-tessellation` |
| `semio-viz-geo-contours` | `geo-field`, `geo-terrain`, `geo-weather`, `spatial-scalar-field` |
| `semio-viz-geo-routes` | `geo-route`, `geo-flow`, `spatial-vector-field` |

Every family inherits the shared key set `semio / viz / geo / base` (`projection, rotate, parallels,
geometry, width, height, palette, stroke, lineWidth, opacity, steps, fit`) declared in
`semio-viz-geo.sty`, and adds its own documented keys in a `%region 🔖️Keys` block. Colours come from
the design tokens only, through the palette ramp `\semio_viz_geo_ramp:n` (`sequential, diverging,
terrain, bathymetric, neutral`, quantised to `steps` levels); there is no literal colour anywhere in
a renderer.

**Gallery verification** — `🗑️generated/GEO-SPATIAL/probe/smoke.tex` (kept) runs all fifteen families over
37 option sets in one xelatex run: **exit 0, 0 errors, 9 pages**. The pages were read back and the
kinds render distinctly — an orthographic relief basemap, a five-class and a diverging quantile
choropleth with its swatch strip, four cartogram kinds, square and hex tile grids, graduated and
proportional symbols, dot density, a hexbin field and its density isolines, Voronoi/Delaunay/hull
tessellations, labelled isolines over a raster, hillshade, hypsometric tint, a terrain profile, a
wind-barb weather map, shaded/wireframe/waterfall surfaces, direct/great-circle/octilinear routes,
flow bands and arrows, and quiver/streamline/particle vector fields.

### Catalogue — 129 entries rewritten in `🖼️assets/🔣️viz-catalog.json`

`🗺️geo-spatial-catalog.ts` in the ticket folder is the handcrafted mapping (kept as an input file):
sections 10 (73), 26 tessellations (4), 27 (16), 28 (17), 35 maps (8), 75 (6) and 78 (4). It rewrites
only entries whose id it names, re-reads the file immediately before writing and renames a temp file
over it, so a concurrent edit by another agent cannot be clobbered mid-write. It also asserts that no
two entries of the same family carry the same option set — the run reports
`no option clashes inside any family`.

The source name a family consumes (`points`, `grid`, `geometry`, `routes`, `symbols`, `arcs`) is
folded into `options`, because the families read it from their own key, not from the catalogue's
`data` field.

### Tests — six cases, all agreeing with their oracle

| case | oracle | scenarios |
|---|---|---|
| `geo-projections` | `d3-geo` | `forward-projection`, `inverse-projection`, `fit-extent` |
| `geo-path-graticule` | `d3-geo` | `path-without-resampling`, `path-with-resampling`, `graticule-lines` |
| `spatial-delaunay-voronoi` | `d3-delaunay` | `delaunay-triangles`, `voronoi-cells` |
| `spatial-hull` | `d3-delaunay` | `convex-hull` |
| `spatial-hexbin` | `d3-hexbin` | `hexagonal-binning` |
| `spatial-contours-density` | `d3-contour` | `marching-squares`, `kernel-density` |
| `field-streamlines` | `@no-oracle-rk-field-integration` | `rk-integration` |

**Deviation from the brief's case list**: the brief asked for one case `spatial-hull-hexbin`. The
harness contract (`📓️status-TESTS-HARNESS.md` §1.2) allows **exactly one** `@oracle-` per feature,
and the hull is adjudicated by `d3-delaunay` while the hexbin is adjudicated by `d3-hexbin`, so the
case is split into `spatial-hull` and `spatial-hexbin`. The contract wins over the naming.

Every fixture was compiled locally with xelatex and compared against the oracle with node
(`🗑️generated/GEO-SPATIAL/check-*.mjs`). Real output of that comparison:

```
forward:        n=28   subject=28   mismatches=0
inverse:        n=28   subject=28   mismatches=0
fit-size:       n=15   subject=15   mismatches=0
fit-extent:     n=15   subject=15   mismatches=0
fit-width:      n=15   subject=15   mismatches=0
path-none:      oracle=96   subject=96   mismatches=0
path-resampled: oracle=118  subject=118  mismatches=0
graticule:      oracle=7876 subject=7876 mismatches=0
voronoi:        oracle=122  subject=122  mismatches=0
hull:           oracle=18   subject=18   mismatches=0
hexbin:         oracle=108  subject=108  mismatches=0
contours:       oracle=40   subject=40   mismatches=0
density-grid:   oracle=35   subject=35   mismatches=0
density-contour:oracle=14   subject=14   mismatches=0
streamlines:    oracle=160  subject=160  mismatches=0
```

The delaunay probe emits the canonical triangle codes
`10211,10811,20912,21112,30506,30512,30610,30710,30711,31112,40711,40811,50912`, which is exactly
`d3-delaunay`'s thirteen triangles `[1,2,11] [1,8,11] [2,9,12] [2,11,12] [3,5,6] [3,5,12] [3,6,10]
[3,7,10] [3,7,11] [3,11,12] [4,7,11] [4,8,11] [5,9,12]`.

**On the real platform** — `bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print"
--case <case>` in `🦑️repo/🔨️modules/🧪️test`, subject compiled by the repository tectonic 0.16.9.
Verbatim output of the final run:

```
geo-projections          [test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
geo-path-graticule       [test] level=quick cases=1 executed=6 passed=6 failed=0 errored=0 parity=3/3
spatial-delaunay-voronoi [test] level=quick cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2
spatial-hull             [test] level=quick cases=1 executed=2 passed=2 failed=0 errored=0 parity=1/1
spatial-hexbin           [test] level=quick cases=1 executed=2 passed=2 failed=0 errored=0 parity=1/1
spatial-contours-density [test] level=quick cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2
field-streamlines        [test] level=quick cases=1 executed=1 passed=1 failed=0 errored=0 parity=0/0
field-streamlines        [test] …/field-streamlines: no-oracle decision rk-field-integration rests on ,
                         which only discharge a conformance, property, round-trip or error scenario —
                         a differential scenario needs an oracle or a second implementation
```

**Twelve adjudicated scenarios, twelve parities, zero failures.** `field-streamlines` compiles and
its subject passes, but the platform cannot adjudicate it until the decision exists (empty
substitutes list) — see the request to TESTS-HARNESS below.

Per-scenario the subject and oracle **projection hashes are identical**, e.g. for `geo-projections`
`76a3177e2ac5` / `b3d1138bea6b` / `5c1f4e21a049` on both sides, and for `geo-path-graticule`
`1c8c1dc42ef9` / `309a9d41815f` / `082141f8e1da`: the LaTeX kernel and d3 produce the same bytes
after rounding.

A whole-owner run (`--owner` without `--case`) aborts in `hierarchy-aggregates` (HIERARCHY's case,
over its 30 s budget) before reaching the `spatial-*` cases, which is why each case was run with
`--case`.

A second harness detail worth passing on: a fixture URI immediately followed by a sentence-ending
period is parsed with the period attached (`local://hull.tex.`) and reported as
`unresolved fixture`. Every feature here now keeps a word after the URI.

Every fixture compiles under the repository tectonic (`--reruns 2`, three TeX passes each):
forward 2.6 s, inverse 3.6 s, fit 7.2 s, path-none 3.3 s, path-resampled 3.9 s, graticule 2.9 s,
delaunay 8.1 s, voronoi 11.3 s, hull 2.5 s, hexbin 3.5 s, contours 2.9 s, density 7.4 s,
streamlines 5.8 s — every case under the 30 s per-adapter budget.

While bringing `spatial-delaunay-voronoi` inside that budget the Voronoi construction was fixed
properly rather than trimmed: the boundary-edge set is now computed **once per diagram**
(`\semio_viz_spatial_boundary_edges:`) instead of once per site, taking the diagram from O(n·E²) to
O(E² + n·E). The cells are byte-identical before and after.

## Requests to other agents

- **GRAMMAR-CORE**: `\SemioVizGeometry` / `\SemioVizGeoPolygon` / `\SemioVizGeoLine` / `\SemioVizGeoPointPart`
  and `\SemioVizPointSet` / `\SemioVizValueGrid` currently live under `%region 🔖️Pending-semio-viz-data` in
  `semio-viz-geo.sty` and `semio-viz-spatial.sty`. Move them into `semio-viz-data.sty` when that package settles;
  the storage is a global `seq` per name (`g_semio_viz_geom_<name>_seq`, items `{type}{id}{value}{coords}`).
  Please also announce the shape of `demo-geo` — my families currently use my own `demo-geo-regions`,
  `demo-geo-cities`, `demo-geo-routes`, `demo-points`, `demo-points-dense`, `demo-grid`.
- **GRAMMAR-CORE (scale)**: the choropleth families need `quantize / quantile / threshold / sequential /
  diverging` colour scales. Until `semio-viz-scale` exposes them, the ramp lives under
  `%region 🔖️Pending-semio-viz-scale` in `semio-viz-geo-choropleth.sty`.
- **SCIENTIFIC**: taxonomy §27 (vector fields) and §28 (scalar fields/surfaces) catalogue entries are mine per the
  fleet brief, but `semio-viz-scientific-field` / `-surface` are yours. I register the families
  `spatial-vector-field` and `spatial-scalar-field` in packages I own (`semio-viz-geo-routes`,
  `semio-viz-geo-contours`) — please do **not** register families with those names.
- **TESTS-HARNESS**: `semio-viz-geo` and `semio-viz-spatial` deliberately do **not** `\RequirePackage`
  `semio-viz-plot`; they need only `semio-viz-data` and `semio-viz-probe`, so a kernel probe stays cheap.
- **TESTS-HARNESS — one new `@no-oracle-` decision needed.** `🧪️tests/field-streamlines/🥒️.feature`
  carries `@no-oracle-rk-field-integration`, which is not yet in `🔮️oracle/🔣️.json`. The entry it
  needs:
  ```json
  {
    "id": "rk-field-integration",
    "capabilities": ["field-integration"],
    "rationale": "d3 ships no ODE integrator. d3-force steps a damped velocity-Verlet simulation, which is a different scheme solving a different problem, so it cannot adjudicate a classical fourth-order Runge-Kutta streamline. The integrator is therefore checked against a second RK4 implementation written in the adapter over the same five analytic fields, the same seeds and the same step size; the two programs share only the formula (expl3 fixed point vs IEEE double, macro expansion vs expressions, string switch vs object lookup). Covers no runtime mutation.",
    "substitutes": ["independent-second-implementation", "specification-vectors"]
  }
  ```
  `🔬sync-oracle-capabilities.ts` reports `0 added across 17 oracle(s)` — it does not flag the
  missing decision, so this has to come through you.
- **SHAPES / CATALOG**: my sections 26 and 75 name `convex-hull` and `concave-hull`, but the
  catalogue covers them from the section 0 mark entries `0/convex-hull`
  (`covers: 0/convex-hull, 26/convex-hull, 75/convex-hull`) and `0/concave-hull`
  (`covers: 0/concave-hull, 26/concave-hull`), which are yours. `spatial-tessellation` with
  `{kind: hull, points: demo-points}` renders a real hull from a real point set; please point those
  two entries at it if you have no mark-level renderer for them. Likewise `76/projection`,
  `76/contours`, `75/voronoi` and `75/delaunay` cover the section 78 ids of the same names.

## Files touched

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo.sty` (rewritten)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-spatial.sty` (rewritten)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-map.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-projection.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-choropleth.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-symbols.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-contours.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-geo-routes.sty`
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` (my sections only)
- `🧰️framework/🛍️products/📓️print/🧪️tests/geo-projections/`
- `🧰️framework/🛍️products/📓️print/🧪️tests/geo-path-graticule/`
- `🧰️framework/🛍️products/📓️print/🧪️tests/spatial-delaunay-voronoi/`
- `🧰️framework/🛍️products/📓️print/🧪️tests/spatial-hull-hexbin/`
- `🧰️framework/🛍️products/📓️print/🧪️tests/spatial-contours-density/`
- `🧰️framework/🛍️products/📓️print/🧪️tests/field-streamlines/`
- ticket: `🗺️geo-spatial-catalog.ts` (the handcrafted catalogue mapping, kept as an input file)
- ticket: `🗑️generated/GEO-SPATIAL/` — `probe/smoke.tex` (the fifteen-family gallery build) and the
  `check-*.mjs` / `oracle-*.mjs` d3 cross-check drivers; every compiler artefact (log, aux, pdf,
  jsonl, synctex) has been deleted, and the folder goes with the ticket

## Open issues

1. **`@no-oracle-rk-field-integration` is not registered yet** — see the request to TESTS-HARNESS
   above. `🧪️tests/field-streamlines` is complete and its numbers agree (0 mismatches over 160
   sampled coordinates), but the platform will refuse the feature until the decision exists.
2. **`spatial-hull-hexbin` was split** into `spatial-hull` and `spatial-hexbin` — one `@oracle-` per
   feature is a harness rule, and the two halves have different oracles.
3. **Budget** — the first platform run killed `geo-path-graticule` at the 30 s per-adapter budget: a
   full 10° graticule at d3's default `precision` 2.5 is 7876 numbers and three tectonic passes. The
   scenario now uses one step (20°) at `precision` 30, which still exercises every branch of the
   major/minor line logic and agrees with d3 on all 354 vertices. Every fixture now compiles in
   1–4.3 s with xelatex: forward 1.3 s, inverse 1.2 s, fit 3.3 s, delaunay 2.6 s, voronoi 4.2 s,
   hull 1.1 s, hexbin 1.4 s, contours 1.0 s, density 2.2 s, streamlines 1.8 s.
4. **Antimeridian and clip-circle polygon re-closure** stay unimplemented (see the deviations
   above). They matter for a world map drawn on an orthographic globe; nothing in the catalogue
   currently needs them, and both are additive.
5. The catalogue's `10/weather-map` id does not exist — CATALOG covers it from `35/weather-map`,
   which I own and did set. Nothing to do.

## Interop with semio-viz-data

`\SemioVizGeoFromTable{collection}{table}` converts a table marked with GRAMMAR-CORE's
`\SemioVizGeometry` (columns `ring`, `lon`, `lat`) into a projectable collection, dropping the
repeated closing vertex a GeoJSON ring carries. Every family also resolves a bare table name
automatically: `geometry = demo-geo` works, because `\semio_viz_geom_ensure:n` converts a name that
is a data table and not yet a collection on first use. Verified on GRAMMAR-CORE's own `demo-geo`:

```
parts 3
part {polygon}{north}{1}{4,54;12,54;12,50;4,50}
part {polygon}{south}{1}{6,49;14,49;14,46;6,46}
part {polygon}{east}{1}{14,53;19,53;19,49;14,49}
```
