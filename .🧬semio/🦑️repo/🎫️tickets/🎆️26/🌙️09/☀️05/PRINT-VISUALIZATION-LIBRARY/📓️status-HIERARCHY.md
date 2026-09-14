# Status — HIERARCHY

Owner of the hierarchy layout kernel `semio-viz-hierarchy.sty`, the namespace packages
`semio-viz-hierarchy-{tree,dendrogram,treemap,partition,pack}.sty`, the catalogue entries of
taxonomy §7 and the packing kinds of §47, the hierarchy layouts of §75/§78, and the five
`hierarchy-*` test cases. `semio-tree.sty` (document trees for reports) is untouched.

## Done

### Kernel — `semio-viz-hierarchy.sty` (≈3,100 lines, expl3)

All five d3-hierarchy transforms are reimplemented line for line against `d3-hierarchy@3.1.2`'s
own sources and **match d3 to 4e-16 relative on every fixture measured** (see §Tests):

- **`tree`** — Reingold–Tilford with the Buchheim/Walker/Leipert improvements: `treeRoot`,
  `firstWalk`, `executeShifts`, `apportion` with the four contour cursors and the thread pointers,
  `moveSubtree`, `nextAncestor`, `secondWalk`, and both extent branches (`size` normalisation with
  the left/right/bottom extremes, `nodeSize` scaling). `separation` as `separationSame`/
  `separationOther` plus `separationDepth` (d3's commented-out `radialSeparation`).
- **`cluster`** — `meanX`/`maxY`, `leafLeft`/`leafRight`, both extent branches.
- **`treemap`** — the per-depth padding stack of `positionNode`, plus the tilings `squarify`
  (`squarifyRatio` with the golden ratio), `resquarify`, `slice`, `dice`, `slice-dice`, `binary`
  (prefix sums + binary search + recursive split), `round`, `padding`/`paddingInner`/`paddingOuter`/
  `paddingTop|Right|Bottom|Left`, `ratio`.
- **`partition`** — the `height + 1` rows, `treemapDice` per parent, `padding`, `round`; the
  sunburst is the same transform over the extent `2π × radius`.
- **`pack`** — `packSiblingsRandom`'s front chain (place / intersects / score / splice /
  re-pick), Welzl's `packEncloseRandom` with `extendBasis`, `encloseBasis1|2|3`, `enclosesWeak`,
  `enclosesNot`, and **d3's own LCG** (a = 1664525, c = 1013904223, m = 2^32, seed 1) shared by
  every enclosure of one `pack()` call, so the shuffle consumes the same stream in the same order.
  Both branches: default `sqrt(value)` (two passes) and an explicit `radius` (one pass).
- Construction: `\SemioVizHierarchyBuild{name}[data=,id=,parent=,value=,path=,delimiter=,
  aggregate=,sort=]`, stratifying a table (id/parent/value, defaults read from the table roles
  `\SemioVizHierarchy` declared in `semio-viz-data`) or a delimited `path` column.
- Aggregates and structure: `sum`, `count`, `depth`, `height`, `sort` (value/height/name, ascending
  and descending, stable), `ancestors`, `descendants`, `leaves`, `links` (breadth-first, as d3's
  node iterator is), the preorder (`eachBefore`) and the postorder (`eachAfter`).
- Renderer iterators: `\semio_viz_hierarchy_map_nodes:nn` binds `id`, `parent`, `depth`, `height`,
  `value`, `x`, `y`, `r`, `x0`, `y0`, `x1`, `y1`, leaf/root flags and the child count before the
  body; `\semio_viz_hierarchy_map_links:nn` binds source/target. Public:
  `\SemioVizHierarchyMapNodes`, `\SemioVizHierarchyMapLinks`, `\SemioVizHierarchyField`.
- Geometry probe: every layout emits `geometry/hierarchy/<algorithm>`, every mark emits
  `geometry/node/<kind>`, `geometry/rect`, `geometry/circle`, `geometry/arc`, `geometry/polygon`,
  `geometry/link/<kind>` through `\semio_viz_probe_geometry:nn` when `\SemioVizProbeOn` is set.

### Families

| package | family | covers |
|---|---|---|
| `-tree` | `tree` | rooted/binary/n-ary/ordered/unordered/top-down/bottom-up/left-right/radial/circular/balloon/cone/hyperbolic trees, org-chart, family tree, genealogy, pedigree, ancestor, descendant, mind map, concept tree, taxonomy, classification tree, decision tree, syntax/parse/AST, game/search tree, trie, merkle |
| `-tree` | `phylogram` | phylogenetic tree, phylogram, chronogram, cladogram (`scale=length\|depth\|equal`, `lengthColumn`) |
| `-dendrogram` | `dendrogram` | dendrogram, horizontal/vertical/radial/circular/cluster dendrogram (`heightColumn`, `scale=cluster\|height`) |
| `-treemap` | `treemap` | treemap, squarified, slice-and-dice, strip, binary, voronoi, rectangular packing |
| `-partition` | `partition` | icicle plot, partition diagram, partition layout |
| `-partition` | `sunburst` | sunburst |
| `-pack` | `pack` | circular treemap, packing layout, circle packing, bubble packing, pack layout |

Every family takes one flat option list routed through four key sets (`semio / viz / family /
<name>`, `semio / viz / hierarchy`, `… / hierarchy / layout`, `… / hierarchy / render`), documented
in a `%region 🔖️Keys` block at the top of each package and in the `🔖️LayoutKeys` / `🔖️RenderKeys`
regions of the kernel.

### Catalogue

62 entries rewritten in `🖼️assets/🔣️viz-catalog.json` (§7 all 52 leaves, §47 `packing-layout`,
`circle-packing`, `bubble-packing`, `rectangular-packing`, §75 `tree|cluster|treemap|partition|
pack-layout`, §78 `cluster`), each with a real family, a distinct option set and one of the demo
tables. Titles and `covers` were left as CATALOG generated them. **No two of the 62 share a
geometry projection** — verified by probing each kind under its own scenario (see §Tests).

### Two own algorithms (no d3 counterpart, marked in the source)

- `tile=strip` — the Bederson strip treemap: `squarifyRatio`'s row breaks with the row orientation
  fixed, so every row is a horizontal strip.
- `tile=voronoi` — a capacity-constrained power diagram (Balzer): sites on a scrambled grid, cells
  by successive half-plane clipping of the parent rectangle against every radical axis, then a
  Lloyd step onto the cell centroid and a weight step towards the target area, `relaxation`
  iterations. Cells are stored as the `poly` field (an `x,y,x,y,…` millimetre clist) and drawn as
  polygons; the bounding box still lands in `x0..y1` so a generic renderer keeps working. Lives
  under `%region 🔖️Pending-semio-viz-spatial` and should move to `semio-viz-spatial` once
  GEO-SPATIAL owns half-plane clipping.

## Tests

Five cases under `🧰️framework/🛍️products/📓️print/🧪️tests/`, each with `🥒️.feature`, `🟦️.ts`
(oracle `d3-hierarchy`, comparison `viz-probe-v1`, level `quick`, mode `differential`) and a
committed probe fixture: `hierarchy-tree-cluster` (5 scenarios), `hierarchy-treemap` (4),
`hierarchy-partition` (3), `hierarchy-pack` (4), `hierarchy-aggregates` (5) — 21 scenarios.

The repo test platform could not be run from here (the TypeScript toolchain is still being
bootstrapped), so every scenario was verified locally with xelatex plus a node comparison against
`d3-hierarchy@3.1.2` that makes exactly the same projections at the same 1e-6 tolerance. The script
is kept in this ticket folder as `🔬verify-hierarchy.mjs`. Real output of the last run:

```
tree-size: ok (6 keys)                 partition/icicle: ok (13 keys)
tree-node-size: ok (4 keys)            partition/sunburst: ok (5 keys)
tree-separation: ok (2 keys)           partition/unbalanced: ok (5 keys)
cluster-size: ok (6 keys)              pack/default-radius: ok (7 keys)
cluster-node-size: ok (4 keys)         pack/padding: ok (3 keys)
treemap/tiling: ok (25 keys)           pack/explicit-radius: ok (6 keys)
treemap/padding: ok (8 keys)           pack/unbalanced: ok (7 keys)
treemap/ratio-and-round: ok (8 keys)   aggregates/sum-and-count: ok (6 keys)
treemap/unbalanced: ok (9 keys)        aggregates/depth-and-height: ok (6 keys)
                                       aggregates/sort: ok (5 keys)
                                       aggregates/traversal: ok (5 keys)
                                       aggregates/path-stratify: ok (3 keys)
```

An earlier direct comparison of the raw geometry probe (before the fixtures existed) reported the
worst relative deviation per layout:

```
tree: ok worst=3.41e-16     cluster: ok worst=4.26e-16   treemap: ok worst=2.15e-16
partition: ok worst=1.93e-16                             pack: ok worst=3.84e-16
```

## Gallery

A gallery document generated straight from the 62 catalogue entries (family + options + demo table,
76×42 mm each) compiles to 11 pages and every kind draws. Distinctness was checked mechanically by
probing each kind under its own scenario and hashing its geometry records:

```
Output written on gallery.pdf (11 pages).
kinds probed: 62
duplicate projections within a family: none
```

## Performance and complexity

Measured with xelatex on a 211-node hierarchy (root, 15 branches, 13 leaves each); the baseline
(package load + table rows + construction + sum) is 1.55 s, so the layout cost is the difference:

| layout | total | layout only | complexity |
|---|---|---|---|
| construction + sum + order | 1.55 s | — | O(n) walks, O(1) identifier lookup, O(n) row reads |
| `tree` | 1.82 s | 0.27 s | O(n) walks plus O(n) contour steps |
| `cluster` | 1.83 s | 0.28 s | O(n) |
| `treemap` | 2.00 s | 0.45 s | O(n) plus O(k) per parent (O(k log k) for `binary`) |
| `partition` | 2.21 s | 0.66 s | O(n) |
| `pack` | 20.2 s | 18.7 s | O(k) front chain plus expected-linear Welzl per parent |

The four rectangular layouts are well inside the "seconds for ~200 nodes" budget. `pack` is the
outlier and the cost is l3fp, not the algorithm: a 13-circle `packSiblings` runs a few thousand
floating-point evaluations, d3's default-radius branch packs the whole hierarchy twice, and l3fp
costs ~0.3 ms per evaluated expression. Reducing the predicate work (one discriminant per
`intersects`/`enclosesWeak`/`enclosesNot` instead of two, one successor lookup per `score` instead
of eight) took it from 22.1 s to 20.2 s; the rest is inherent to fixed-point arithmetic in TeX.
Real figures are far smaller — the 62-kind gallery, six of whose kinds are packs, builds in 33 s
in total, and the committed `hierarchy-pack` fixture (seven packs of a 13-node hierarchy) compiles
in 4.7 s.

## Vocabulary announced to other agents

- **Demo tables** added under `%region 🔖️DemoData` of `semio-viz-hierarchy.sty`:
  `demo-hierarchy-deep` (13 nodes, four branches, three levels — the default for §7 kinds),
  `demo-hierarchy-unbalanced` (10 nodes, one four-level branch beside a single leaf — the fixture
  that exercises the Buchheim contour threading), `demo-hierarchy-path` (a delimited `path` column),
  `demo-hierarchy-branch` (branch `length` column for phylograms). GRAMMAR-CORE's shared
  `demo-hierarchy` is used unchanged where a small tree is enough.
- **Public commands**: `\SemioVizHierarchyBuild{name}[keys]`,
  `\SemioVizHierarchyLayout{tree|cluster|treemap|partition|pack}{in}{out}[opts]`,
  `\SemioVizHierarchyMapNodes{name}{code}`, `\SemioVizHierarchyMapLinks{name}{code}`,
  `\SemioVizHierarchyField{name}{field}{index}`. Passing a table name as `in` builds it on first
  use with the roles the table declares.
- **Families**: `tree`, `phylogram`, `dendrogram`, `treemap`, `partition`, `sunburst`, `pack`.
- **Probe keys**: `geometry/hierarchy/<algorithm>` (index, id, depth, value, x, y, r, x0, y0, x1,
  y1 per node, in preorder), `geometry/node/<kind>`, `geometry/link/<kind>`, `geometry/rect`,
  `geometry/circle`, `geometry/arc`, `geometry/polygon`.

## Requests to other agents

- **GRAMMAR-CORE (`semio-viz-data`)**: `\semio_viz_table_cell:nnnN` routes the row through
  `\clist_set:Nx`, which **silently drops empty cells** — so a stratified table whose root has an
  empty `parent` cell reads its columns shifted. The hierarchy kernel reads cells itself with
  `\seq_set_split:NnV` (`%region 🔖️Pending-semio-viz-data`, `\semio_viz_hierarchy_cell:nnnN` /
  `\semio_viz_hierarchy_row:nn`); the fix belongs in `semio-viz-data`. Also: `\SemioVizHierarchy`
  is yours (a table role declaration); the stratifier is `\SemioVizHierarchyBuild` here, and it
  reads your `id`/`parent`/`value`/`label` roles as defaults.
- **GRAMMAR-CORE (`semio-viz-theme`)**: `\semio_viz_theme_color:nN` is provided under
  `%region 🔖️Pending-semio-viz-theme` behind a `\cs_if_exist:NF` guard, together with
  `\providecolor` fallbacks for `semio-chrome-foreground|text-normal|canvas` so a bare probe of the
  package works without the semio class. Delete the guarded block when the theme lands.
- **TESTS-HARNESS**: `semio-viz-probe` ships `\semio_viz_probe_geometry:nx` but not `:xx`, and a
  geometry key that names the mark it came from has to be expanded; the variant is generated here
  under `%region 🔖️Pending-semio-viz-probe`. Please add it upstream.
- **GUIDES**: `\l_semio_viz_width_fp` / `\l_semio_viz_height_fp` are declared here with
  `\fp_zero_new:N` as well (`%region 🔖️Pending-semio-viz-guide`) so a standalone probe works.
- **Layout kernel owner / CATALOG**: the architecture gives `\SemioVizLayout{algorithm}{in}{out}
  [opts]` to `semio-viz-layout`, but the legacy `semio-viz-layout.sty` still owns
  `\SemioVizLayout{family}[opts]` and is used by `♻️mit-bestand/📋️bericht/📎️anhang/📈️skalierung.tex`.
  The hierarchy transforms are therefore exposed as `\SemioVizHierarchyLayout` with the new
  signature; when the legacy file is deleted, `\SemioVizLayout` should dispatch
  `tree|cluster|treemap|partition|pack` to `\semio_viz_hierarchy_layout:nnnn`.
- **CATALOG**: `78/tree`, `78/partition`, `78/pack` and `78/treemap` are currently `covers` of
  section 76 and 7 entries. Their hierarchy-transform behaviour now lives in the `75/*-layout`
  entries; please re-point those covers if the coverage check wants them on the layouts.
- **SHAPES/GUIDES**: the hierarchy renderers draw with TikZ primitives today. Once
  `\SemioVizLink`, `\SemioVizArc`, `\SemioVizSymbol` and the label guides exist, the drawing
  helpers in `%region 🔖️Render` / `🔖️RenderNodes` of `semio-viz-hierarchy.sty` should be rewritten
  onto them; the geometry probe keys are already the ones a shape-based renderer would emit.

## Traps found (worth knowing)

- **expl3 names must not contain digits** — `\l_..._x0_fp` truncates at the `0`. Confirmed
  independently by SHAPES; every `x0/x1/y0/y1/a2/b2/d1..d3/i0/i1` variable here carries letter
  suffixes instead (the *field* names `x0`, `y1`, … are strings inside `:c` and are fine).
- **l3fp binds `^` tighter than unary minus**, so `\get:nnn{h}{x}{i} ^ 2` squares a negative
  abscissa to a negative number. Every power base is parenthesised.
- **`\int_eval:n` has no ternary** and **`\fp_compare:nNnTF` takes a single relation character**
  (`<=` needs `\fp_compare:nTF { a <= b }`).
- **`:` is a letter under `\ExplSyntaxOn`**, so a TikZ polar coordinate `(90:12)` emitted from
  expl3 is parsed as a node name; `\c_colon_str` restores catcode 12.
- **`\str_case:Vn` has no else branch** — the third group is inserted literally into the output.
  `\str_case:VnF` is the one with a fallback.
- **`\exp_args:NnV` expands one argument**, not the remaining two; a three-name call needs a
  generated `:nVV` variant.
- **l3prop reads are a linear scan** of the whole list, which makes a 200-row stratification
  quadratic. The identifier index is one control sequence per (build, identifier) instead.

## Files touched

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-hierarchy.sty` (kernel, rewritten)
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-hierarchy-tree.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-hierarchy-dendrogram.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-hierarchy-treemap.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-hierarchy-partition.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-hierarchy-pack.sty`
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` (62 entries of §7, §47, §75, §78)
- `🧰️framework/🛍️products/📓️print/🧪️tests/hierarchy-tree-cluster/{🥒️.feature,🟦️.ts,🧫️fixtures/hierarchy-tree-cluster.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/hierarchy-treemap/{🥒️.feature,🟦️.ts,🧫️fixtures/hierarchy-treemap.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/hierarchy-partition/{🥒️.feature,🟦️.ts,🧫️fixtures/hierarchy-partition.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/hierarchy-pack/{🥒️.feature,🟦️.ts,🧫️fixtures/hierarchy-pack.tex}`
- `🧰️framework/🛍️products/📓️print/🧪️tests/hierarchy-aggregates/{🥒️.feature,🟦️.ts,🧫️fixtures/hierarchy-aggregates.tex}`
- `.🧬semio/🦑️repo/🎫️tickets/…/PRINT-VISUALIZATION-LIBRARY/🔬verify-hierarchy.mjs` (local oracle run)
- `.🧬semio/🦑️repo/🎫️tickets/…/PRINT-VISUALIZATION-LIBRARY/📓️status-HIERARCHY.md` (this file)

No nx target and no launch entry was added: the five cases are discovered by the test platform and
run under the existing `test-quick` level.

## Open issues

- `pack` on a 200-node hierarchy takes ~19 s (see §Performance). Acceptable for print figures, but
  if the exhaustive level ever packs a large fixture it will dominate the run.
- The adapters are written against the TESTS-HARNESS contract but have never been executed by the
  platform; `compileVizProbe`, `probeProjection` and `roundProbeNumbers` are used exactly as §1.4
  documents them.
- `tile=voronoi` has no oracle. When the case is promoted to the platform it needs a
  `@no-oracle-voronoi-treemap` decision with the specification vectors; it is not part of the five
  committed cases today.
