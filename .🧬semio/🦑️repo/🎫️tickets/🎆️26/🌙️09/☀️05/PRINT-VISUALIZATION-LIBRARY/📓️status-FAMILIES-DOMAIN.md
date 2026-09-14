# FAMILIES-DOMAIN — status

Agent: FAMILIES-DOMAIN. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Scope: the 16 long-tail **domain** families and their 71 catalogue kinds, in the two packages
`🖋️latex/semio-viz-text.sty` and `🖋️latex/semio-viz-domain.sty`.

## 0. The exact family list (computed, not assumed)

Read from `🖼️assets/🔣️viz-catalog.json` against every `\SemioVizFamily{…}` in `🖋️latex/*.sty`.
16 families / 71 catalogue kinds:

| family | n | section | package |
|---|---|---|---|
| `text-viz` | 12 | 13 | `semio-viz-text` |
| `spatial-layout` | 10 | 47 | `semio-viz-domain` |
| `engineering-diagram` | 7 | 36 | `semio-viz-domain` |
| `performance` | 6 | 62 | `semio-viz-domain` |
| `schedule` | 5 | 16 | `semio-viz-domain` |
| `optimization` | 5 | 40 | `semio-viz-domain` |
| `niche` | 4 | 61 | `semio-viz-domain` |
| `monitoring` | 4 | 65 | `semio-viz-domain` |
| `notation` | 3 | 56 | `semio-viz-domain` |
| `neural` | 3 | 67 | `semio-viz-domain` |
| `election` | 3 | 70 | `semio-viz-domain` |
| `biology` | 2 | 34 | `semio-viz-domain` |
| `logistics` | 2 | 59 | `semio-viz-domain` |
| `version-control` | 2 | 64 | `semio-viz-domain` |
| `survey` | 2 | 69 | `semio-viz-domain` |
| `analytical` | 1 | 42 | `semio-viz-domain` |

The task brief named a slightly different `text-viz` set (word tree, text arc, document-term
matrix, alignment); the catalogue is the SSOT and those slugs do not exist in it, so the twelve
catalogue slugs are what is implemented.

## 1. Done

**All 16 families are registered and all 71 catalogue kinds render, non-empty and pairwise
distinct within their family** (probe evidence in §4).

- `semio-viz-text.sty` — `text-viz` (§13), seven layouts (`spiral`, `grid`, `matrix`, `lane`,
  `curve`, `band`, `river`) over the four `demo-text-*` tables.
- `semio-viz-domain.sty` — the other fifteen families. New in this session:
  - `engineering-diagram` (§36): one part table, seven schematics — `assembly` bodies,
    `assembly`+`explode`+`leaders` for the exploded view, `gear` pitch circles with one tick per
    tooth read from the data, `kinematic` pin joints with a hatched footing, `linkage` loop,
    `mechanism` (gear discs and link bars told apart by the class column), `truss` chords and
    pinned supports.
  - `optimization` (§40): the assignment tableau with the greedy per-row optimum ringed, the
    activity network with a forward earliest-start relaxation and the critical chain walked back
    from the latest finish, the capacitated `flow` ribbons (a pale capacity band with the flow
    drawn solid inside it), the `transport` bipartite discs sized by their totals, and the machine
    `schedule` lanes.
  - `analytical` (§42): spine, effect box, alternating category bones and cause twigs, built from
    the cause list alone; the effect caption is the bilingual `\semio_viz_dom_lang:nn` default when
    the `effect` key is empty.
  - `notation` (§56): a skeletal formula with real double bonds (two rules offset perpendicular to
    the bond axis), a circuit mesh with orthogonal wires and a resistor/capacitor/inductor/diode/
    source glyph vocabulary, and a five-rule staff with note heads and stems.
  - `logistics` (§59): the origin-destination tableau and the shipment sankey.
  - `niche` (§61): the alluvial plot, the polar area chart (radius carries √value so the sector
    area stays proportional), the horizontal icicle, the Archimedean spiral heat map.
  - `monitoring` (§65): the layered dependency with arrowheads, the latency grid, the call graph
    (node radius = response time, tint = error rate) and the ring topology.
  - `neural` (§67): one channel panel, the small multiples of every channel, and the signed
    saliency grid with the cells above the threshold outlined.
  - `survey` (§69): the same question-by-group grid as sized symbols and as a heat map.
  - `election` (§70): tile choropleth, seat cartogram, swing arrows on a diverging ground.
  - `biology` (§34): the graduated richness survey and the taxon range map.
- **Shared regions** so repeated code sits together: `🔖️Kernel` (canvas, frame, levels, the
  matrix reader, the arrow, the bilingual literal, the index and the conditional column sum),
  `🔖️Partition` (the stratified reader shared by §62 and §61), `🔖️Series` (§16 and §64),
  `🔖️Ribbon` (§59's sankey and §61's alluvial — the only difference is `fill` choosing a node bar
  or a plain axis rule), `🔖️Tile` (§34 and §70).
- **22 new demo tables** with the `demo-` prefix, all registered in the schema's
  `x-semio-demo-tables`: `demo-text-terms`, `demo-text-positions`, `demo-text-flow`,
  `demo-text-topics`, `demo-sprint`, `demo-repo`, `demo-machine`, `demo-services`,
  `demo-services-edges`, `demo-latency`, `demo-neural`, `demo-election`, `demo-biodiversity`,
  `demo-shipments`, `demo-survey`, `demo-assignment`, `demo-activities`, `demo-jobs`,
  `demo-causes`, `demo-molecule`, `demo-circuit`, `demo-score`.
- **Catalogue**: all 71 entries of my sections now carry the distinct option set that was compiled
  and proven distinct, plus the demo table each one reads. Written by
  `🔧️domain-families-catalog.py`, which copies the option sets out of the probe document so the
  catalogue can never drift from what was measured.
- **Schema**: `x-semio-family-options` now documents the full vocabulary of all 16 families with
  type, default and a bilingual description, owner `FAMILIES-DOMAIN`
  (`🔧️domain-families-schema.py`).
- **Test case** `🧪️tests/🏭️domain-families/` with two scenarios, and the `domain-families`
  no-oracle decision in `🔮️oracle/🔣️.json`.

## 2. Decisions

- **The two stubs were byte-damaged** at the start (no backslashes at all — the bash-heredoc
  mangling `📓️integration.md` warns about). Both were rewritten with the Write tool. Since then
  every TeX edit went through Write/Edit, and the two in-place substitutions that were done with a
  Python one-liner (`\semio@stroke@emphasized` → `\semio@stroke@focus`,
  `semio-chrome-border-focus` → `semio-chrome-border-emphasized`) were verified by re-grepping the
  file for its 236 leading backslashes afterwards.
- **`\semio@stroke@emphasized` and `semio-chrome-border-focus` do not exist.** `semio-tokens.sty`
  defines only `hairline`, `default` and `focus` strokes, and the chrome colours only
  `border-normal` and `border-emphasized`. `semio-viz-domain.sty` was the only file using the two
  invented names; both were replaced by existing tokens.
- **`\int_eval:n` is `\numexpr` and does not know `max`.** The activity-network depth relaxation
  used it and produced 54 "Missing number, treated as zero" errors; it now uses `\fp_to_int:n`.
  Worth knowing for the other agents.
- **One kind = one distinct option set, verified by compilation, not by inspection.** Every
  catalogue entry was written from the probe document, and the probe's record streams are hashed
  per kind: 71 cases, 71 distinct signatures, no empty stream.
- **`niche`'s alluvial reads `demo-survey`, `logistics`' sankey reads `demo-shipments`.** They
  share the ribbon renderer, so they must not also share the data, or the two kinds would coincide.
- **`niche`'s icicle grows along the abscissa**, the opposite reading direction of §62's flame
  shapes, so the two families' partition drawings stay visibly different.
- **The d3-array differential is `extent` + `sort`, not `rollup`.** The brief asked for a rollup,
  but the §13 term loader emits one glyph per table row and never aggregates, so a rollup oracle
  would be comparing against something the family does not compute. What d3-array genuinely defines
  here is the size scale's domain (`d3.extent` over the frequencies) and the reading order
  (`d3.sort` with `d3.descending`), and the `word` geometry record carries the glyph body height,
  so that is what the differential scenario compares — exactly, at three decimals. The feature says
  so in full.
- **`🗑️generated/FAMILIES-DOMAIN/` was deleted twice by another agent's cleanup while I was
  working in it.** The probe now lives in the session scratchpad and is mirrored back after every
  green compile.

## 3. Files touched

Created or rewritten:
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-text.sty`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-viz-domain.sty`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏭️domain-families/🥒️.feature`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🏭️domain-families/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/🔧️domain-families-catalog.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️05/PRINT-VISUALIZATION-LIBRARY/🔧️domain-families-schema.py`

Edited (only the entries I own):
- `🧰️framework/🛍️products/📓️print/🖼️assets/🔣️viz-catalog.json` — the 71 entries of my families.
- `🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json` — `x-semio-family-options` for my 16
  families, and 22 entries in `x-semio-demo-tables`.
- `🧰️framework/🛍️products/📓️print/🔮️oracle/🔣️.json` — the `domain-families` no-oracle decision,
  and `viz-domain-families` added to `d3-array`'s capabilities so the differential scenario's
  `@oracle-d3-array` tag resolves.

Generated by `generate viz` (not hand-edited): `semio-viz-catalog.sty`,
`semio-viz-catalog-labels.sty`, `🧾️template/📊️viz-gallery/*`.

## 4. Verified runs

### 4.1 Probe: 71 kinds, 71 distinct signatures, 0 empty

`xelatex` over a probe that runs every catalogue kind of the sixteen families through
`\SemioVizRunFamily` with exactly the catalogue's option set, then hashes each kind's record
stream (tail of the run):

```
biodiversity-map               16 records  2a2d15cb
habitat-range-map              22 records  24f19b23
word-cloud                     14 records  af72957f
tag-cloud                      14 records  1daae291
frequency-cloud                14 records  82a2264d
text-heatmap                   56 records  081e1bca
topic-distribution-chart      168 records  3abbace8
lexical-dispersion-plot        31 records  21f5efcc
concordance-plot               32 records  551ac47a
kwic-visualization             54 records  f183b8a3
sentiment-timeline             11 records  3dffc8e8
vocabulary-growth-curve         2 records  41ee8a3d
topic-evolution-chart          30 records  bc4e3803
topic-river                     8 records  b0f3459a
cases=71 distinct=71 duplicates=0 empty=0
```

### 4.2 `bun ./📜️script.ts generate viz`

```
print: wrote 83 visualization catalogue artifacts
```

### 4.3 `bun ./📜️script.ts test fundamental`

```
[test] level=fundamental cases=98 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=96
```

(The first run failed on `unknownDemoTables` with all 54 of my new kinds listed; the 22 demo
tables were then registered in `x-semio-demo-tables` and the level is green.)

### 4.4 `bun ./📜️script.ts parity long --case 🏭️domain-families`

(The brief said `parity quick`; both scenarios are `@level-long` because each compiles a probe
through tectonic — the coverage one compiles all 71 kinds in a single document — so `long` is the
level that selects them. `--case` matches the folder name including its emoji.)

Run from `🦑️repo/🔨️modules/🧪️test`, tectonic compiling both probes:

```
note: Writing `…\🧪️probe-out\every-domain-kind-draws-its-own-geometry.probe.jsonl` (297 KiB)
note: Writing `…\🧪️probe-out\term-glyph-size-follows-the-frequency-extent.probe.jsonl` (1.40 KiB)
[test] level=long cases=1 executed=4 passed=4 failed=0 errored=0 parity=2/2
```

### 4.5 Family registration across the whole catalogue

`vizCoverageReport()` after `generate viz` — the sixteen domain families were the last unregistered
ones in the catalogue, so `verifyVisualizationFamilies` (level `long`) is now satisfiable:

```
kinds 1738 leaves 1966
unknownFamilies 0 []
unknownOptions 0 []
duplicateSignatures 0
```

### 4.6 Gallery section builds

`bun ./📜️script.ts build viz <section>`, first pass over 13, 16, 34, 36, 40, 42, 47:

```
13: OK
16:
34: ! LaTeX Error: File `semio-viz-chart-distribution.sty' not found.  l.6 \RequirePackage
36: ! LaTeX Error: File `semio-viz-chart-distribution.sty' not found.  l.6 \RequirePackage
40: ! LaTeX Error: File `semio-viz-axis.sty' not found.  l.13 \RequirePackage
42: ! Package semio-viz Error: Unknown viz family 'analytical'.
47: ! Package semio-viz Error: Unknown viz family 'spatial-layout'.
```

**Section 13 (`text-viz`) builds clean.** Every other line of that pass is a collision with the
concurrent legacy deletion and regeneration, not a defect in these families:

- 16 wrote both PDFs and then failed in the dark pass with
  `File 'semio-viz-catalog-labels.sty' not found` at `semio-viz.sty` line 419 — another agent ran
  `generate viz` while my build was reading the generated packages. Both generated files were back
  on disk two minutes later.
- 34 and 36 fail inside the legacy `semio-viz-charts.sty` (dated 2026/08/18), which requires
  `semio-viz-chart-distribution.sty`. Neither file exists in `🖋️latex/` any more — they are the
  legacy chart packages CATALOG is deleting; nothing under my ownership requires them.
- 40 fails on the legacy `semio-viz-axis.sty`, same deletion.
- 42 and 47 produced no fresh log at all, so the helper grepped the stale 07:23 / 05:35 logs from
  before these families existed; the quoted "unknown family" lines are from those stale logs.
  `\SemioVizFamily{analytical}` and `\SemioVizFamily{spatial-layout}` are both registered and both
  render in the probe, and `unknownFamilies` is empty (§4.5).

Re-run of the three sections whose families are mine, once the other agent's regeneration had
settled — all three clean, no source change in between:

```
34: OK
42: OK
47: OK
```

`🌿️viz-34.pdf` / `🌿️viz-34-dark.pdf`, `🐟️viz-42.pdf` / `🐟️viz-42-dark.pdf` and
`🔤️viz-13.pdf` / `🔤️viz-13-dark.pdf` are in `📦️packages/🟦️typescript/dist/`.

### 4.7 Gallery section builds, the remaining twelve sections

`bun ./📜️script.ts build viz <section>` over 16, 36, 40, 56, 59, 61, 62, 64, 65, 67, 69, 70 —
every one clean, light and dark, from the same sources:

```
16: OK
36: OK
40: OK
56: OK
59: OK
61: OK
62: OK
64: OK
65: OK
67: OK
69: OK
70: OK
```

**All sixteen sections of the domain families now build: 13, 16, 34, 36, 40, 42, 47, 56, 59, 61,
62, 64, 65, 67, 69, 70.** Nothing in the first pass's six failures survived a re-run, and no source
was changed between the two passes.

## 5. Open issues / requests to other agents

- POLISH: `semio-tokens.sty` has no `emphasized` stroke between `default` (1.5pt) and `focus`
  (2.25pt); the domain families now use `focus` where a heavier-than-default rule is meant.
- TESTS-2: `viz-domain-families` was added to `d3-array`'s capability list so
  `@capability-viz-domain-families` + `@oracle-d3-array` resolves. Move it if the registry wants
  that capability declared differently.
- Whoever is sweeping `🗑️generated/`: please leave other agents' subfolders alone while they are
  still working.
