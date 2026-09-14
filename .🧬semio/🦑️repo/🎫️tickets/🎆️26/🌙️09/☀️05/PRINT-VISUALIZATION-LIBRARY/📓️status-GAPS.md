# GAPS — status

Agent: GAPS. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
All four gaps `📓️verification.md` records are closed. Sections 1 to 4 are the four tasks, in the
order they were done; section 6 is the verification, run after the last change.

---

## 1. Dead families — decided and executed (task 3, done)

`📓️verification.md` listed eight registered-but-unused families. Each was measured against the
family that already serves the catalogue kinds it would take over.

| family | decision | evidence |
|---|---|---|
| `notation-petri` | **kept, kind re-pointed** | It reads a `kind` column (`place`/`transition`), draws places as circles carrying the **token marking dot** and transitions as bars, and routes the arcs of a separate edge table. `state-machine` `mode=petri` only alternates circle and square **by node index parity** (`semio-viz-network-graph.sty`, `\semio_viz_sm_node_draw:n`) and draws no tokens at all — it cannot express a marking, so it is not a Petri net. `petri-net` now names `notation-petri`. |
| `concept-venn` | deleted | It plotted literal `x/y/r` circles from a demo table. `sci-set` (which already serves `venn-diagram`, `multi-set-venn-diagram`, `area-proportional-venn-diagram`) solves the two-set lens-area equation by 60 bisection steps (`\semio_viz_math_lens_distance:nn`) and has Venn, Euler and area-proportional modes for two to four sets. |
| `sci-forest` | deleted | Its studies live in the **option list** as a clist, against the catalogue's `data`-table convention. `uncertainty` `mark=forest` (`semio-viz-charts-statistical.sty`) is table-driven, draws the weight-sized box, the CI whiskers with caps, the null reference line and the row labels on the shared cartesian frame, and already serves `forest-plot` and `meta-analysis-forest-plot` — plus `mark=funnel`. |
| `sci-timing` | deleted | Its digital `0/1/x/z` waveform patterns are also clist literals. The only catalogue kind is `timing-diagram`, which covers **both** `15/timing-diagram` (UML) and `31/timing-diagram`; `uml-timing` renders state lanes over a time axis from a node table plus a segment table, which reads correctly for both leaves. |
| `sci-field` | deleted (family only) | 3 glyph modes plus streamlines over an analytic `u`/`v` pair. `spatial-vector-field` serves all **16** §27 kinds with 9 render modes (`quiver, direction, streamline, streakline, particle, lic, gradient, curl, divergence`) over 5 named fields. `semio-viz-scientific-field.sty` is the shared `semio_viz_sci_*` kernel of all ten scientific packages, so only the family block (lines 843–965) went; the kernel stayed. |
| `sci-surface` | deleted | 6 analytic modes, no 3D projection. `spatial-scalar-field` serves all **17** §28 kinds with 13 surface modes including `mesh`, `shaded`, `wireframe`, `waterfall`, `curtain`, `heightmap`, `slice` and the `tilt`/`turn` projection. The other four families of `semio-viz-scientific-surface.sty` were kept, and `\l_semio_viz_surf_mode_tl` / `…_expr_tl` (which they share) stayed in the Parsing region. |
| `adjacency` | deleted (whole package) | 16 keys, node order `input|degree|alphabetical`. `adjacency-matrix` (`semio-viz-network-matrix.sty`) binds through the network kernel, has 38 keys, orders by `index|name|degree|group|component|cluster`, and draws block hairlines and reserved label gutters; it already serves 8 catalogue kinds. `semio-viz-matrix-adjacency.sty` held nothing else, so the file went, together with its `\RequirePackage` in `semio-viz.sty` and in the `🎪️showcase-families` namespace fixture. |
| `annotated-chart` | deleted | A second, `variant`-keyed implementation of exactly the §50 kinds that the `mode`-keyed `annotated` family (`semio-viz-charts-line.sty`) already serves — all **11** of them, on the cartesian frame with its 27 inherited keys. `semio-viz-annotation.sty` keeps its annotation kernel and `\SemioVizAnnotate`. |

None of the eight had a `x-semio-family-options` entry and none was named by a catalogue kind, so
nothing had to be removed from the schema; `notation-petri` **gained** one (12 keys, en+de) plus the
two demo tables `demo-petri` and `demo-petri-edges`. No test referenced any of the eight (the
`adjacency` hits under `🧪️tests` are the `\SemioVizLayout{adjacency}` layout algorithm and the §76
namespace kind of the same name, both untouched).

### Also in task 3

- **`%region 🔖️Keys` for `alluvial` and `parallel-sets`** — both were the only families without one.
  Each now documents all 37 keys it accepts (the network kernel's, the flow kernel's, and the two it
  re-binds), with the defaults read out of `\semio_viz_net_keys_reset:` and `\semio_viz_snk_keys_reset:`
  rather than from the previous prose, which claimed `nodeSort=none, nodeAlign=left` for `alluvial`
  where the renderer sets neither.
- **The last 4 legacy `\semio@viz@` tokens** are gone. `\semio@viz@diagram@width/@height` were
  `\providecommand` fallbacks inside `semio-viz-diagram-flowchart.sty`; they are now
  `\c_semio_viz_theme_frame_width_fp` / `…_height_fp` in a new `%region 🔖️Frame` of
  `semio-viz-theme.sty`, which the flowchart package already requires. `grep -rn 'semio@viz@' *.sty`
  is empty.

## 2. Schema configurability debt — closed (task 1, done)

**Measured, not estimated.** `🔧️keys-from-sty.ts` resolves, per family, the keys its option list
actually accepts, and that is four different things at once:

1. the literal `\keys_define:nn { semio / viz / family / <name> }` blocks;
2. the blocks a shared-vocabulary helper installs on a path it is *handed* — `\semio_viz_net_keys_add:n`,
   `\semio_viz_snk_keys_add:n`, `\semio_viz_sci_keys_canvas:n { <family> }` (whose path template is
   `semio / viz / family / #1`, not a bare `#1`), and `\semio_viz_cart_family_keys:n`, which stores the
   path in a `tl` and installs through `\exp_args:NV \keys_define:nn \l_semio_viz_cart_keys_module_tl`;
3. the path an `unknown .code:n` handler forwards to — `semio / viz / family / mark` and
   `semio / viz / family / shape` both forward everything they do not name to `semio / viz / mark`;
4. every kernel path the family body hands **its own option list** to — `semio / viz / family / common`
   (33 families), `semio / viz / geo / base` (17), `semio / viz / scale` (12), `semio / viz / diagram`
   (the whole `flow`/`swimlane` vocabulary), and the `\keys_set_known:` routing chain of
   `\semio_viz_hierarchy_family_apply:nn`, which walks one list through four paths in turn.

Against that, the schema documented **2 878 fewer key slots than the families implement** — far more
than `📓️verification.md`'s 117 families, because the delegated vocabularies were invisible to the
earlier count — and **one key the family does not implement**.

Those 2 878 slots come from only **296 distinct declaration sites**: a key means the same thing in
every family that takes it from the same `\keys_define` block. So the authoring surface is 296 en+de
descriptions (`🔧️gaps-authored-{a,b,c}.json`, merged into `🔧️gaps-authored.json`), and
`🔧️gaps-worksheet.ts` fans them out — plus 435 more taken from a family that already documented the
same block's key, 33 of which had two competing wordings and take the more common one.

**The phantom.** `spatial-vector-field` declared no `points` key — its `unknown .code:n = { }`
swallowed it — yet 16 §27 catalogue kinds set `points=demo-points` on it and the schema documented
it. The family samples an analytic field on a lattice and never reads a point set, so the key was
removed from the schema and from those 16 catalogue entries; the entries stay pairwise distinct in
`render`, `field`, `arrow`, `seeds` and `dt`.

### The test that keeps it closed

`@id-implemented-keys-documented` (`@level-fundamental`, `@mode-conformance`) in
`🧪️tests/📚️catalog-coverage/`. The subject is a **port of the extractor into product code** —
`vizImplementedFamilyKeys()` in `🔨️modules/📊️visualization-gallery/🟦️.ts` — feeding two new
`vizCoverageReport()` findings, `undocumentedOptions` and `phantomOptions`. Both directions are
asserted, so neither an undocumented key nor an invented one can come back. The existing
`@id-options` scenario only ever checked that a key a *catalogue entry sets* is declared, which is
why 117 families could document a fraction of what they implement and still pass.

**The extractor had a bug of its own, and the test found it.** `keysOfBody` matched a key name only
after `,` or at the very start of the block, which silently dropped **the first key of any block
whose opening line is a comment** — including `data` in `\semio_viz_sci_keys_canvas:n`, where the
comment above it explains why a scientific family records a table name at all. That one key is worth
82 family slots, and it surfaced not from re-reading the extractor but from `🖱️interactionstate-kinds`
failing to compile: `state-selection` really does reject `data`, and once the extractor stopped
lying about the other 81 families the difference was visible. Comments are now stripped before the
key scan, in the ticket-local extractor and in the ported one alike.

## 3. The four missing cases (task 2, done)

`📓️verification.md` recorded four holes: no case for `theme`, none for `plot`, and the namespaces
`infographic` and `interactionstate` untested. All four now exist and pass.

| case | level | mode | oracle |
|---|---|---|---|
| `📰️infographic-kinds` | long | conformance | `@no-oracle-infographic-kinds` |
| `🖱️interactionstate-kinds` | long | conformance | `@no-oracle-interactionstate-kinds` |
| `🌈️theme-palettes` | quick | 6 × differential | `d3-scale`, `d3-interpolate`, `d3-color` |
| `🪶️plot-grammar` | quick | 1 conformance + 3 differential | `d3-scale`, `print-viz-kernel-twin` |

### What the two conformance cases had to fix before they could pass

They were written the way `showcase-families` is — the catalogue is the specification, every kind
must emit geometry, no two kinds of a family may emit the same geometry — and that immediately
found five defects that no other case could see, because a family case reads its own geometry back:

1. **Four of the eleven §44 kinds and three of the fifteen §45 kinds emitted no geometry at all.**
   `infographic-list`, `infographic-illustration` and `state-selection` called
   `\semio_viz_probe_geometry:nn` nowhere, so their output was invisible to the harness. They now
   emit `diagram-badge`, `diagram-silhouette` + `diagram-vertex` + `diagram-callout`, and
   `diagram-selection` + `diagram-tooltip`.
2. **`state-selection` rejected the option every catalogue kind passes it.** Its l3keys block had no
   `data` key and no `unknown` handler, so `\SemioVizChart{selected-node-network-state}` — which is
   what the §45 gallery page runs — died with *the key 'semio/viz/family/state-selection/data' is
   unknown*. Every other family either declares `data` or swallows it. It now declares it, bound to
   the node table, exactly as `semio / viz / diagram` documents.
3. **Four `state-sequence` kinds rendered identically** as far as any measurement could tell:
   `diagram-frame` carried the panel origin and the normalised value but not the `depiction`, the
   ghost opacity or the arrow flag, which is all that separates `transition-sequence`,
   `keyframe-sequence`, `morph-sequence` and `animated-path-frames`. The record now carries them.
4. **`anatomical-infographic` and `annotated-illustration` were the same picture.** The annotated
   one sets `unit=8`; the anatomical one takes the default, which on the 80×40 gallery frame is
   `min(80,40)/5` — also 8. Both drew the fallback ellipse at the same size with the same callouts.
   `@id-distinctness` passes them because their *option lists* differ; only geometry catches it. An
   anatomical infographic should not be an ellipse in the first place, so it now carries a torso
   outline of its own, and the two are distinct because they draw different things.
5. **Three §76 namespace kinds pointed at families this ticket deleted.** `semio-viz-showcase.sty`
   mapped the `adjacency`, `field` and `surface` namespaces onto `adjacency`, `sci-field` and
   `sci-surface`; they now name `adjacency-matrix`, `spatial-vector-field` and
   `spatial-scalar-field`, the families that serve those namespaces' catalogue kinds. This is how
   `🎪️showcase-families` started failing, and it is the one regression the deletions caused.

Both cases **assert inside the subject**. The platform runs only the subject phase of a case under a
`specification-vectors` no-oracle decision (`📜️script.ts` line 852: the decision "discharges itself
inside the scenarios"), so an adapter that merely returns two projections and never compares them
has no teeth — I verified this by leaving the mismatch in and watching the case report `passed`.
With `assert.deepEqual` in the subject the same state reports `failed`.

### What the two differential cases measure

`🌈️theme-palettes` is the colour kernel every family reads its hues from, and nothing else
adjudicates it: the **wrap** against `d3-scale`'s `scaleOrdinal` over the presence (12), brand (7)
and gray (7) palettes and the 8 hatches, the **ramp** against `d3-interpolate`'s `piecewise` in
`d3-color`'s Lab and HCL on the rendered hexadecimal character by character, and the **stops**
against `d3-color` parsing and re-formatting every declared stop of both appearances. The theme's
own default, `interpolator=oklab`, has no d3 reference at all; what it is held to is the law every
interpolation space owes its stop list — a five-stop ramp returns those five colours unchanged at
0, ¼, ½, ¾ and 1 — with `d3-color` as the reference for "unchanged". All six passed first run.

`🪶️plot-grammar` measures `\SemioVizPlot`: that all sixteen declared encoding channels bind and only
the ones an option list names (reading the channel registry directly, before a plot runs, so a typo
must become an unbound channel and not a silently accepted key); that a numeric channel is placed by
a linear scale over the column extent and a categorical one on `scaleBand`'s band centre, both onto
the guide package's plot rectangle; and that the TypeScript twin's `planVizChart` resolves the same
specification onto the same millimetres. All four passed first run.

## 4. The d3 parity surface (task 4, done)

`📓️verification.md` listed two d3 modules with no public equivalent: `interpolate` and `quadtree`.

### `\SemioVizInterpolate{kind}{a}{b}{t}` (`semio-viz-scale.sty`)

The kernel already had every interpolator — the ramp code has interpolated colours since the scale
package existed — but nothing outside the kernel could reach them, so a document that wanted a
number half way between two others had to declare a scale to get it. The command dispatches on
`number`, `round`, the four colour spaces `rgb`/`lab`/`hcl`/`oklab`, and `array` for two equally
long comma lists. `🔀️interpolate-kinds` measures the switch against `d3-interpolate` — one d3
function per kind — and all four scenarios passed on the first run. OKLab has no d3 reference and is
measured in `🌈️theme-palettes` instead.

### `\SemioVizQuadtree{table}[x=,y=]` and `\SemioVizQuadtreeFind{x}{y}[radius]` (`semio-viz-spatial.sty`)

A real quadtree, not a decorative name: d3's cover rule (floor the first corner into a unit square,
then double it towards any point outside, choosing which corner stays put from the point's
quadrant), d3's quadrant numbering, leaves that hold a chain of coincident points and subdivide when
a point of their own arrives, and a `find` that walks the tree depth first, nearest quadrant first,
pruning any quadrant that cannot hold anything closer than the best candidate so far.

`🌳️spatial-quadtree` compares it with `d3-quadtree` on all three observable facts and they agree
exactly, including the two that are structural rather than behavioural:

| table | extent | points | nodes |
|---|---|---|---|
| `qt-scatter` (7 points) | `[[0,1],[16,17]]` | 7 | 11 |
| `qt-shifted` (4 points straddling the origin) | `[[-7,-7],[25,25]]` | 4 | 6 |

The **node count** is what makes this more than an answer check: counting the nodes
`d3.quadtree.visit` walks and the nodes this implementation allocated says the two trees have the
same *shape*. Two implementations that merely "cover the data" would agree on neither number —
`qt-shifted`'s extent is nowhere near its bounding box.

`d3-quadtree` was present in the workspace only as a dependency of the `d3-force` oracle. It is now
a declared devDependency and a registered oracle (`ISC`, 3.0.1, test-only, engine family `d3`,
`productionReachable: false`); `bun ./📜️script.ts dependency` classifies it as
`test-oracle js:d3-quadtree@3.0.1` and raises nothing.

### Four expl3 traps this cost, worth recording

1. **An expl3 variable name may not contain a digit.** `\fp_new:N \g_semio_viz_qt_x0_fp` ends the
   name at the `0` and the rest lands in the document. The cover corners are `xlo`/`ylo`/`xhi`/`yhi`.
2. **`\int_if_odd_p:n` is a boolean predicate, not a number**, so it cannot be the condition of an
   `fp` ternary — `\int_mod:nn {q} {2} ?` can.
3. **`\int_eval:n` inside `\use:c` inside `\edef` is `\the` inside `\csname` inside `\edef`**, which
   TeX refuses. The indexed accessors evaluate the index with `\exp_args:Nf` *before* the name is
   built.
4. **Recursion through scratch variables is the real hazard.** Every argument of the insertion and
   the query is frozen into a literal at the call boundary (`\semio_viz_qt_call:Nnnnnnn`), because
   the box coordinates were expressions over `\l_…_xm_fp` and the node indices were `\l_…_other_int`
   — the very variables the next level of recursion assigns first.

## 5. Tooling

`🔧️keys-from-sty.ts` (ticket-local) resolves, per family, the keys its option list actually accepts:
the literal `\keys_define:nn { semio / viz / family / <name> }` blocks, the blocks a
`…_keys_add:n`-style helper installs on a path it takes as `#1` (including the one that goes through
`\exp_args:NV \keys_define:nn \l_…_module_tl`), and the paths the family body forwards its own
option list to with `\keys_set:nn { … } { … #1 … }`. `--diff` compares that with the schema.

Three more, all kept:

- `🔧️gaps-worksheet.ts` — attributes every undocumented key to the `\keys_define` block that
  declares it, fans one authored description out over every family that takes the key from that
  block, and refuses to write anything while a block is still unauthored.
- `🔧️gaps-authored-{a,b,c}.json`, merged into `🔧️gaps-authored.json` — the 296 handcrafted en+de
  descriptions, keyed by `<package>#<block ordinal>|<key>` so an edit elsewhere in a package cannot
  silently re-point one.
- `🔧️gaps-schema.ts` — writes the schema and the catalogue back in their exact on-disk format
  (`JSON.stringify(…, null, 2)` plus a trailing newline, verified by round-trip first).

The pipeline is `bun 🔧️gaps-worksheet.ts todo` (what is still unauthored) → `… docs` (writes the
1.2 MB `🔧️gaps-key-docs.json`) → `bun 🔧️gaps-schema.ts keys` (applies it). The intermediate is
regenerable in one command and is not kept.

## 6. Verification

Every command below was run after the last change; the output is verbatim.

```
$ bun ./📜️script.ts generate viz
print: wrote 83 visualization catalogue artifacts

$ bun ./📜️script.ts test fundamental
[test] level=fundamental cases=106 executed=17 passed=17 failed=0 errored=0 parity=8/8 not-exercised=104

$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print"
[test] level=quick cases=106 executed=550 passed=550 failed=0 errored=0 parity=259/259 not-exercised=14

$ bun ./📜️script.ts parity long --owner "🧰️framework/🛍️products/📓️print"
[test] level=long cases=106 executed=630 passed=630 failed=0 errored=0 parity=294/294

$ bun ./📜️script.ts contract     # print rows of ⚡️cache/breaches/testing.json
print breaches: 0
```

The long level is what actually exercises the six new cases end to end — the two conformance ones
only run there — and it also re-renders the exhaustive gallery and the whole showcase, which is
where a deleted family would surface. It is green.

`📓️status-POLISH-2.md` recorded `241/241, 513/513` before this work: the six new cases add 18 parity
pairs and 37 scenarios, and nothing that was green went red.

The dependency survey classifies the one new package correctly:

```
$ bun ./📜️script.ts dependency
[dependency] test-oracle js:d3-quadtree@3.0.1 (d3-quadtree)
```

It exits 1 on a long list of `NEW production-reachable` Rust, Go and JavaScript packages belonging to
*other* products; none of them is touched here and all of them predate this ticket.

## 7. Files touched

- `🖋️latex/semio-viz-scientific-field.sty` — `sci-field` family removed, kernel kept
- `🖋️latex/semio-viz-scientific-surface.sty` — `sci-surface` family and its private state removed
- `🖋️latex/semio-viz-scientific-engineering.sty` — `sci-timing` removed
- `🖋️latex/semio-viz-scientific-biology.sty` — `sci-forest` removed, `\l_semio_viz_bio_log_bool` re-declared for `sci-contact-map`
- `🖋️latex/semio-viz-diagram-concept.sty` — `concept-venn` and its demo table removed
- `🖋️latex/semio-viz-annotation.sty` — `annotated-chart` family removed
- `🖋️latex/semio-viz-matrix-adjacency.sty` — **deleted**
- `🖋️latex/semio-viz.sty` — its `\RequirePackage` removed
- `🖋️latex/semio-viz-flow-alluvial.sty`, `semio-viz-flow-parallelsets.sty` — `%region 🔖️Keys` added
- `🖋️latex/semio-viz-theme.sty` — `%region 🔖️Frame` with the two frame-fallback constants
- `🖋️latex/semio-viz-diagram-flowchart.sty` — legacy tokens replaced by those constants
- `🖋️latex/semio-viz-showcase.sty` — the `adjacency`, `field` and `surface` namespaces re-pointed
- `🖋️latex/semio-viz-infographic.sty` — `diagram-badge`, `diagram-silhouette`, `diagram-vertex` and
  `diagram-callout` probe records added, so §44 becomes measurable
- `🖋️latex/semio-viz-interactionstate.sty` — `diagram-selection` and `diagram-tooltip` records added,
  `diagram-frame` enriched with the depiction, the ghost opacity and the arrow flag, `data` accepted
  by `state-selection`
- `🖋️latex/semio-viz-scale.sty` — `%region 🔖️Interpolate` and `\SemioVizInterpolate`
- `🖋️latex/semio-viz-spatial.sty` — `%region 🔖️Quadtree`, `\SemioVizQuadtree`, `\SemioVizQuadtreeFind`
- `🧪️tests/🎪️showcase-families/🧫️fixtures/showcase-namespaces.tex` — dropped the deleted package
- `🧪️tests/📚️catalog-coverage/` — the `@id-implemented-keys-documented` scenario and its adapter
- `🔨️modules/📊️visualization-gallery/🟦️.ts` — `%region 🔖️ImplementedKeys` with
  `vizImplementedFamilyKeys()`, and the `undocumentedOptions` / `phantomOptions` findings
- **six new cases**: `🧪️tests/📰️infographic-kinds/`, `🖱️interactionstate-kinds/`, `🌈️theme-palettes/`,
  `🪶️plot-grammar/`, `🔀️interpolate-kinds/`, `🌳️spatial-quadtree/` (feature, adapter, fixtures)
- `🔮️oracle/🔣️.json` — two `@no-oracle-` decisions, the `d3-quadtree` oracle, six capability additions
- `📦️packages/🟦️typescript/package.json` + `bun.lock` — `d3-quadtree` 3.0.1 as a devDependency
- `🧬️schema/🔣️.json` — `notation-petri` family options, `demo-petri`, `demo-petri-edges`, and the
  2 878 key documentation entries of §2
- `🖼️assets/🔣️viz-catalog.json` — `petri-net` re-pointed, `points` dropped from 16
  `spatial-vector-field` kinds, `anatomical-infographic` given its own outline
- 83 regenerated catalogue artifacts
- `🔨️modules/📊️visualization-gallery/🟦️.ts` — `registeredVizFamilies()` skips commented
  registrations, so it and `vizImplementedFamilyKeys()` both report 246 families
- `🧾️template/📊️viz-api/🔓️viz-api.tex` — the two new public commands documented, en and de
- ticket-local, kept: `🔧️keys-from-sty.ts`, `🔧️gaps-worksheet.ts`, `🔧️gaps-schema.ts`,
  `🔧️gaps-authored{,-a,-b,-c}.json`, and the four rows added to `🔧️case-emoji.tsv`

## 8. Open

Nothing blocking. Two things a later agent may want:

- **`uncertainty` has no pooled-summary diamond and no log axis.** Deleting `sci-forest` (§1) cost
  nothing that a catalogue kind used, but those two refinements existed only there. Both are small
  additions to `\semio_viz_unc_rows:` if §22 ever wants them.
- **`beeswarm` and the force collide step still scan every placed point.** The quadtree they should
  stand on now exists and is adjudicated (§4); re-pointing them at it is a behaviour change to
  `🐝️spatial-hexbin` and `🧲️network-force` vectors, so it belongs in its own ticket rather than
  riding along with this one.
