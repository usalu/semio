# Status — NETWORK-FLOW

Owner of the network and flow kernels, their namespace packages, the catalogue entries of taxonomy
sections 8, 9, the network/tree and matrix kinds of 13, 57, 58, 63, 64, 66, 67 and the network
layouts of §75/§78, and six test cases under `🧪️tests/`.

## Done

1. **Network kernel** `🖋️latex/semio-viz-network.sty` — graph store over GRAMMAR-CORE's graph
   tables, the graph algorithms, and eleven layouts (`force`, `circular`, `shell`, `grid`,
   `spectral`, `layered`, `radial`, `arc`, `chord`, `bundling`, `adjacency`), plus the public
   `\SemioVizGraphMetrics`, `\SemioVizGraphOrder`, `\SemioVizGraphSeed`, `\SemioVizGraphRandom`.
2. **Flow kernel** `🖋️latex/semio-viz-flow.sty` — `sankey`, `alluvial`, `parallel-sets`.
3. **Seventeen families** across the eight namespace packages (below), all registered with
   `\SemioVizFamily`, all consuming their data and their options.
4. **132 catalogue kinds** covering **169 taxonomy leaves**, each with a distinct family + option
   set, written by `🔧️network-flow-catalog.py`; the matching family option vocabularies written into
   `🧬️schema/🔣️.json` by `🔧️network-flow-schema.py`.
5. **Six test cases**, all green through the repo test platform against `d3-force`, `d3-chord`,
   `d3-sankey`, `dagre` and `d3-array`.
6. **All 132 kinds render, and all 132 render differently** — measured, see *Gallery* below.
7. **`\msg_new:nnn { semio-viz } { unknown-layout }` collision fixed** — the bug TESTS-HARNESS
   reported as blocking every gallery document. All five messages of the two kernels are now
   `\msg_if_exist:nnF`-guarded, so `semio-viz-layout.sty` and this kernel coexist.

Not done, and flagged rather than faked: the ~1 minute budget for 100 nodes × 300 force iterations
(see *Performance*), and seventeen chart-shaped leaves inside my sections that need a chart owner
(see *Requests*).

## The tests, with their real output

Run with the repo platform, not by hand:

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print" --case <case>

network-force        [test] level=quick cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
network-chord        [test] level=quick cases=1 executed=4  passed=4  failed=0 errored=0 parity=2/2
flow-sankey          [test] level=quick cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
network-layered      [test] level=quick cases=1 executed=6  passed=6  failed=0 errored=0 parity=3/3
network-circular-arc [test] level=quick cases=1 executed=8  passed=8  failed=0 errored=0 parity=4/4
network-algorithms   [test] level=quick cases=1 executed=10 passed=10 failed=0 errored=0 parity=5/5
```

Twenty-two scenarios, `parity=22/22`. That run is after every kernel and family change below, and
`network-force` and `network-algorithms` were run once more after the last one.

`bun "<ticket>/🔬sync-oracle-capabilities.ts"` was run afterwards, as the harness asks:

```
[DEBUG] d3-array += network-graph-algorithms, network-placement-layouts
[DEBUG] d3-sankey += flow-sankey-layout
[DEBUG] dagre += network-layered-layout
[DEBUG] oracle capability sync: 5 added across 14 oracle(s)
```

What each case actually measures:

- **`network-force`** (`@oracle-d3-force`, `viz-probe-coarse-v1`) — the phyllotaxis start with zero
  ticks, then twenty ticks of link + many-body + center, twelve ticks with collide/x/y, and fifteen
  ticks with a non-default link distance and charge. **The oracle sets `.theta(0)`**: d3's default
  `theta` 0.9 is a Barnes–Hut approximation, this kernel evaluates the many-body force exactly, and
  `theta(0)` is what makes d3 do the same. Measured agreement after twenty ticks, before rounding:

  ```
  tex   1  0.4043579892120597   1.786690169700593
  d3    1  0.40435798921206295  1.786690169700661
  tex   8 -29.5443122544059    -25.67720496298554
  d3    8 -29.544312254405675  -25.67720496298576
  ```
- **`network-chord`** (`@oracle-d3-chord`, `viz-probe-v1`) — group angles, subgroup arcs and ribbon
  endpoints, plain and with `padAngle(0.05).sortGroups(descending).sortSubgroups(ascending)`.
- **`flow-sankey`** (`@oracle-d3-sankey`, `viz-probe-v1`) — every node `x0,y0,x1,y1` and every link
  `y0,y1,width`, for **all four** `nodeAlign`s. Measured `max abs diff 2.27e-13` on coordinates of
  magnitude ~500 before the case was moved onto the platform.
- **`network-layered`** (`@oracle-dagre`, `viz-probe-exact-v1`) — ranks against dagre's
  `ranker: "longest-path"`, plus two conformance scenarios. Two differences are **specified, not
  hidden**: dagre's longest-path is *as late as possible* while this kernel's is *as soon as
  possible* (they agree on a graph without slack, which is the differential scenario; the slack graph
  is a conformance scenario), and the within-layer ordering is not compared to dagre at all because
  barycentre and median-plus-transpose are two heuristics for the same NP-hard problem.
  dagre drops `rank` from the node label once it has turned it into a coordinate, so the adapter
  reads the rank back as the position of the node's row among the distinct row centres.
- **`network-circular-arc`** (`@oracle-d3-array`, `viz-probe-v1`) — `circular`, `shell`, `arc` and
  the `adjacency` permutation as conformance vectors, with the expected sequence *generated* from
  the definition on the oracle side rather than copied from a run.
- **`network-algorithms`** (`@oracle-d3-array`, `viz-probe-exact-v1`) — degrees, components, Kahn
  order, BFS and DFS against an independent implementation written in the adapter, plus the LCG
  stream as a conformance vector. `graphology` is not a registered oracle and adding a package needs
  a survey, so the reference is a second implementation; the feature says so and writes out every
  tie-breaking rule both sides implement.

## Gallery, and what it caught

`🔧️network-flow-gallery.py` renders every one of the 132 catalogue kinds through
`\SemioVizRunFamily` with the options the catalogue records, and hashes each kind's probe stream, so
"renders" and "renders differently" are both measured. It exists because the generated gallery loads
`semio-viz.sty`, which pulls in every namespace, and cannot currently be built (see *Requests*).

The first run was the useful one:

```
kinds: 132, rendered: 132, distinct projections: 118
identical projections: bayesian-network == argument-map; causal-dag == attack-graph;
  data-flow-graph == dag; force-directed-graph == directed-graph;
  layered-sugiyama-graph == data-flow-graph; markov-chain-diagram == automaton;
  undirected-graph == force-directed-graph; money-flow-diagram == energy-flow-diagram;
  sankey-diagram == money-flow-diagram; word-tree == syntax-tree;
  binary-search-tree == word-tree; suffix-tree == binary-search-tree;
  attack-tree == bayesian-network; provenance-graph == layered-sugiyama-graph
```

Fourteen pairs of kinds that carried different options and drew the same picture — the exact failure
mode the rebuild exists to remove, and one that no amount of reading the catalogue would have found.
The fixes were of three kinds, and the first two are the interesting ones:

1. **The probe was under-reporting the drawing.** An arrow head, a fill and a caption are part of the
   primitive, so `geometry/link` and `geometry/curve` now carry `arrow`/`plain` and the stroke,
   `geometry/node` the shape and fill, `geometry/poly` the fill, and `geometry/text` the caption.
   Without those, `directed=true` and `labels=true` were invisible to any test.
2. **Three options were declared and never read.** `neural-network`'s `mode` now shifts the palette,
   so a CNN and a transformer of the same shape are different diagrams; `state-machine`'s `mode` now
   also picks the transition stroke (attack = danger, argument = success, markov = accent) instead of
   only mattering for Petri nets and factor graphs; and `state-machine` was **overriding the caller's
   `shape`** on every node, so `shape=box` on an argument map and `shape=diamond` on an attack tree
   were silently ignored — the role and the reading now override the glyph only when they have
   something to say. `commit-graph`'s `laneGap` never reached the geometry at all and was removed
   rather than faked. The `shape` one was caught by looking at the rendered page, not by the hash:
   the kinds were already distinct through their stroke, so only the picture showed it.
3. **Nine catalogue entries got an option that actually changes the drawing** — `demo-graph-dag`'s
   weights are all one, so `weightWidth` alone drew identical widths, and `demo-flow` has one node
   per depth, so all four `nodeAlign`s put it in the same column.

The tree modes of `data-structure` also got their own glyph and hue per mode
(`suffix-tree` a box, `parse-tree` a diamond, `bst` its own fill), because a binary search tree and a
suffix tree are different diagrams, not the same diagram with a different name.

After those fixes:

```
$ python 🔧️network-flow-gallery.py
kinds: 132, rendered: 132, distinct projections: 132
```

The rendered pages were also looked at, which is how the `shape` bug surfaced. Rerun the script to
regenerate the check; it writes into `🗑️generated/NETWORK-FLOW/gallery/` and needs no other agent's
package. **Run it alone** — a concurrent tectonic and xelatex on this machine kill each other's runs.

## Decisions

- **`\SemioVizGraph` is GRAMMAR-CORE's, and their model won.** A graph is an *edge table* carrying
  the roles `source`, `target`, `weight` and optionally `nodes` / `edges`. The kernel consumes that
  through `\semio_viz_network_use:nn {table} {overrides}`; when no `nodes` role is present the node
  set is derived from the endpoints in first-appearance order, source before target, per edge row —
  which is the order a caller building d3's node array from the same edge list gets, and what makes
  every oracle comparison index-aligned. The second argument is a per-call binding override
  (`weight=value` for `demo-flow`, say), read with `\keys_set_known:nn` so a layout's own options
  pass straight through.
- **Layout registry is shared and guarded.** `semio-viz-layout` (kernel) does not exist as a live
  package, so `%region 🔖️Pending-semio-viz-layout` in `semio-viz-network.sty` creates
  `\semio_viz_layout_define:nn`, `\semio_viz_layout_run:nnnn` and `\SemioVizLayout` behind
  `\cs_if_exist:NF` / `\ProvideDocumentCommand`. HIERARCHY, GEO-SPATIAL and the eventual kernel owner
  can create or take over the same registry without a clash; whoever owns it should move that region
  verbatim.
- **Kernels do not load `semio-viz-plot`.** A layout kernel is pure computation: `semio-viz-network`
  requires only `semio-viz-data` and `semio-viz-probe`. The namespace packages require what they
  actually compose (`-family`, `-mark`, `-probe`, `tikz`, `semio-tokens`).
- **Option state never leaks between calls.** l3keys has no "reset to initial", so every key family
  declares its defaults once in a `\tl_const:Nn \c_semio_viz_*_defaults_tl` list applied before the
  caller's options (`\semio_viz_network_keys:nNn`). Two calls of one layout with different options
  therefore differ only in what the caller passed — which is exactly what the catalogue's
  distinctness rule needs.
- **Determinism.** The only randomness is d3's LCG (`a=1664525`, `c=1013904223`, `m=2^32`), **seed 1**
  (key `seed=`), used where d3 uses it — `jiggle()` for coincident coordinates — and for the
  `random` layout. Verified against d3's own stream in `network-algorithms`.
- **`many-body` is exact, `theta` is not implemented.** Documented on the key and in the feature.
- **Spectral is an approximation**: power iteration on `cI − L` (c = 2·maxdeg+1) deflated against the
  constant vector, `power-iterations` (default 200) controls accuracy.
- **Bundling emits control points, not a sampled curve.** The `bundling` layout writes the
  beta-relaxed control polygon (`edge, step, x, y`); the B-spline evaluation sits in
  `semio-viz-network-bundling.sty` under `%region 🔖️Pending-semio-viz-shape` and moves to SHAPES as
  the `bundle` curve.
- **Node fill and stroke are part of the probe record.** `geometry/node` carries shape and fill,
  `geometry/link` the stroke, `geometry/poly` the fill. Without that, two kinds that differ only in
  colour (`binary-search-tree` vs `red-black-tree`) produced byte-identical projections and the
  distinctness test could not see the difference. Other family owners will want the same.
- **`graph-layout` and `connection-matrix` are not families.** The brief lists both among the
  families, but every `graph-layout` leaf is the `graph` family with a different `layout=`, and
  `connection-matrix` is `adjacency-matrix` with `scaleBy=binary, diagonal=true` — which is precisely
  the catalogue design ("a kind is a family plus its options") and what keeps the presets honestly
  distinct. Same for the ego/community presets and most of the specialised graphs. Seventeen families
  cover the brief's list; the ones that got their own family are the ones that draw something the
  `graph` renderer cannot (`state-machine`'s start markers and transition captions,
  `neural-network`'s layer table, `commit-graph`'s lanes, `data-structure`'s cell strips,
  `schema-graph`'s mode-driven layout, and the matrix, arc, chord, bundling and flow renderers).
- **Synonym leaves are merged, not duplicated.** 169 leaves become 132 kinds: `social-network` and
  `collaboration-network` are one kind with two `covers` entries, because two kinds of one family
  that can only render identically is the exact failure the rebuild exists to remove. The generator
  asserts this: it refuses two kinds of one family with the same data and the same options apart
  from `variant`.
- **`variant` is carried but is not the discriminator.** The schema requires it on every entry, so
  every family accepts it and the gallery can caption with it — but the geometry is decided by the
  other options, and the generator's assertion above is what enforces that.

## Performance — measured, and short of the brief's target

Per force tick: `O(n²)` many-body pairs (once per unordered pair) + `O(m · link-iterations)` link
terms + `O(n)` per other force, so `O(iterations · (n² + m))` l3fp evaluations — four per pair after
the inner loop was rewritten (fused distance and `distanceMin` clamp through the fp ternary, half the
pairs by symmetry, row-local accumulation written back once).

| n | iterations | wall clock |
|---|---|---|
| 100 | 300 | **5 min 30 s** |
| 100 | 30 | 2 min 12 s |
| 100 | 10 | 42 s |
| 8 | 20 | 1.6 s |

**The brief's "~100 nodes × 300 iterations in about a minute" is not reachable with an exact
`many-body` in l3fp** — an l3fp evaluation costs ~100 µs on this machine and 300 ticks at n=100 is
millions of them. (The numbers above were taken while the rest of the fleet was compiling, so they
are an upper bound, but not by an order of magnitude.) `iterations=` is exposed on the layout and on
every family, and the catalogue presets sit at 60–160 iterations on graphs of ten nodes, which is
seconds. Closing the gap needs either a real Barnes–Hut quadtree (`theta` > 0, ~n log n — but then
the oracle is d3-with-default-theta and the exactness the brief also asks for goes) or fixed-point
arithmetic (loses d3 parity). Both are cross-cutting calls, so this is flagged rather than taken.

## Requests to other agents

- **CHARTS-B / CHARTS-A** — `semio-viz-charts-polar.sty:71: LaTeX3 Error: Control sequence
  \l_semio_viz_pie_pad_fp already defined.` This is what blocks `bun ./📜️script.ts` (the report
  build) and every document that loads `semio-viz.sty` today, including the generated gallery.
- **GEO-SPATIAL / TESTS-HARNESS** — `parity quick` for the whole print owner does not finish: it dies
  in `geo-path-graticule` with `spawnSync bun ETIMEDOUT` on the adapter host, so the run never reaches
  a summary line. Every case here was therefore run with `--case`, which is why the results above are
  six lines and not one.
- **Every family owner** — put the *style* of a primitive into its probe record, not only its
  coordinates. Fourteen of my kinds looked identical to the harness purely because the arrow head,
  the fill and the caption were missing from the record; the distinctness test cannot see what the
  probe does not emit, and comparing option objects does not catch an option that is declared and
  never read.
- **CATALOG** — `🖼️assets/🔣️viz-catalog.json` does not validate against `🧬️schema/🔣️.json` today:
  **1276 of the 1738 entries** are missing the `options.variant` the schema requires (`bar`, `dot`,
  `line`, … — every family whose generated entries kept an empty option object). None of the failures
  is in my sections; measured with `ajv/dist/2020` over the committed schema and catalogue.
- **CATALOG** — the placeholder families your generator invented for my sections (`graph-layout`,
  `graph-dense`, `graph-edge`, `graph-community`, `graph-special`, `formal-language`, `security`,
  `lineage`, `database`, `transform-network`, `flow`) are gone from `x-semio-family-options`; the 132
  entries now name the seventeen families this namespace registers. `neural`, `version-control` and
  `text-viz` are left in place because leaves outside my scope still reference them (next item).
- **CHARTS-A / CHARTS-B** — seventeen leaves inside my sections are charts, not networks, and I left
  them with the placeholder family rather than force them into a graph family:
  `13/word-cloud`, `13/tag-cloud`, `13/frequency-cloud`, `13/concordance-plot`,
  `13/kwic-visualization`, `13/text-heatmap`, `13/topic-distribution-chart`, `13/topic-river`,
  `13/topic-evolution-chart`, `13/sentiment-timeline`, `13/lexical-dispersion-plot`,
  `13/vocabulary-growth-curve` (family `text-viz`), `64/churn-chart`, `64/code-frequency-chart`
  (family `version-control`), `67/activation-map`, `67/feature-map`, `67/saliency-map`
  (family `neural`). They need a real owner and a real family.
- **GEO-SPATIAL** — `9/flow-map` is yours per the brief ("delegates to GEO-SPATIAL projection"); it is
  not in my 132 and I have not touched its entry.
- **SHAPES** — the uniform cubic B-spline in `semio-viz-network-bundling.sty`
  (`%region 🔖️Pending-semio-viz-shape`) is the `bundle` curve of taxonomy §0; move it when
  `semio-viz-shape` owns the interpolators, and the bundling family will call yours instead.
- **GRAMMAR-CORE** — please keep the `nodes` / `edges` roles on graph tables, and consider a `group`
  role so node grouping is role-driven rather than column-name-driven (today the kernel falls back to
  a literal `group` column).

## Files touched

Kernels and namespace packages, all under `🧰️framework/🛍️products/📓️print/🖋️latex/`:

- `semio-viz-network.sty` — network kernel (rewritten from stub)
- `semio-viz-flow.sty` — flow kernel (rewritten from stub)
- `semio-viz-network-graph.sty` — shared network canvas, palette and drawing primitives, and the
  families `graph`, `state-machine`, `neural-network`, `commit-graph`, `schema-graph`,
  `data-structure`; demo tables `demo-graph-community`, `demo-graph-dag`, `demo-graph-bipartite`,
  `demo-graph-signed`, `demo-automaton`, `demo-layers`, `demo-commits`, `demo-structure`,
  `demo-flow-stages`
- `semio-viz-network-matrix.sty` — `adjacency-matrix`, `node-link-matrix`, `biofabric`
- `semio-viz-network-arc.sty` — `arc-diagram`, `hive-plot`
- `semio-viz-network-chord.sty` — `chord`, `dependency-wheel`
- `semio-viz-network-bundling.sty` — `edge-bundled`
- `semio-viz-flow-sankey.sty` — `sankey`
- `semio-viz-flow-alluvial.sty` — `alluvial`
- `semio-viz-flow-parallelsets.sty` — `parallel-sets`

Catalogue, schema and tests:

- `🖼️assets/🔣️viz-catalog.json` — 132 entries rewritten for my sections
- `🧬️schema/🔣️.json` — `x-semio-family-options` for my seventeen families, eleven placeholders dropped
- `🧪️tests/network-force/`, `network-chord/`, `flow-sankey/`, `network-layered/`,
  `network-circular-arc/`, `network-algorithms/` — feature + adapter each
- `🔮️oracle/🔣️.json` — capabilities added by `🔬sync-oracle-capabilities.ts`

Ticket scripts (inputs, kept):

- `🔧️network-flow-catalog.py` — the 132 kinds and the merge into the catalogue, with the
  slug-uniqueness, leaf-coverage and family-option distinctness assertions
- `🔧️network-flow-schema.py` — the family option vocabularies
- `🔧️network-flow-gallery.py` — the render-and-distinctness check

These are ticket-local one-shot generators, so they live in the ticket folder as the briefing asks.
They are deliberately **not** repository scripts: nothing in the build depends on them, their output
(the catalogue, the schema) is committed and hand-checkable. If the fleet decides the catalogue should
be regenerated rather than edited, they belong in print's `📜️script.ts` as `generate viz catalog`,
rewritten in TypeScript like the rest of the toolchain — that is a CATALOG-level decision, not mine to
take unilaterally.

No new nx target and no new launch entry were needed: the six cases run through the existing test
platform routing, and the kernels are plain `.sty` files the loader already requires.
