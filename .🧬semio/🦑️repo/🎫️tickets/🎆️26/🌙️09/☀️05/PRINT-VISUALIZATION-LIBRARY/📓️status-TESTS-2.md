# TESTS-2 — status

Agent: TESTS-2. Ticket `26/09/05/PRINT-VISUALIZATION-LIBRARY`.
Owns: every case under `🧰️framework/🛍️products/📓️print/🧪️tests/` except `showcase-families`
(FAMILIES-CAPABILITY) and `domain-families` (FAMILIES-DOMAIN); `🔮️oracle/🔣️.json`;
`🔨️modules/🧪️viz-probe/🟦️.ts`; `🎮️commands/🧪️print-pipeline-verification/`; print `📋️project.json`
test targets; print test entries in `.vscode/🧩️launch.seed.jsonc`; `🔨️modules/📊️viz-kernel/` tests.

---

## Plan

1. Twin cases `🧪️tests/<emoji><case>-twin/` for every kernel case + `🧪️tests/kernel-twin-parity/`.
2. Oracle flips: `signal-dft-bode` → fft.js, `3d-projection` → gl-matrix, `math-functions-sampling` → mathjs.
3. Platform run for the whole print owner: `contract`, `parity quick`, `parity long`; triage.
4. Exhaustive: print `test long`, `test viz full`, regenerate `gallery-render` fixtures, distinctness.
5. Wire `📜️script.ts test` levels, nx targets, launch seed, regenerate `launch.json`.

## Done

### 1 — `contract --owner …/📓️print` is at **0 breaches** (was 112)

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
$ bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/📓️print"
1204 high-priority breach(es) across 5 rule(s):
    646  testing/contract
    352  testing/fixture
    159  testing/dependency
     42  testing/oracle
      5  testing/discovery
# print's own share of that set: 0 lines (was 90 taxonomy + 15 dependency + 7 oracle)
```

**90 × `testing/taxonomy` `case-slug`.** `testCaseSlugPattern` requires one leading emoji identity
before the kebab slug (`^(?:…\p{Extended_Pictographic}…)[a-z0-9]+(?:-[a-z0-9]+)*$`), and every print
case was a bare slug — the only owner in the repository whose test tree carried none. All 90 were
renamed to `<emoji><slug>`, one distinct grapheme each (the sibling convention every other owner
already follows: `📚️library/🧪️tests` has 68 cases and 68 distinct leading graphemes). The mapping is
`🔧️case-emoji.tsv` in this folder; the 26 sibling imports between cases (the 22 `-twin` adapters onto
their base case, and the four diagram adapters onto `🚦️diagram-routing`) were rewritten first.
Nothing outside `🧪️tests/` referenced a case by name, and `testProjectName()` strips the leading
identity, so no nx project name changed.

**15 × `testing/dependency` `oracle-in-production`.**
- 14 of them: `📊️viz-kernel/📦️packages/🟦️typescript/📜️script.ts` reached thirteen registered d3
  oracles. The check table (1 558 lines, all thirteen module groups) moved to
  `📊️viz-kernel/📦️packages/🟦️typescript/🔬️probes/🟦️.ts` — `🔬️probes` is the taxonomy's own name for
  an external measurement tool and is test-owned by what it is, so the d3 imports now sit where the
  taxonomy says they belong and `📜️script.ts` keeps only the comparison, the level router and the
  two commands. Not a second script file: it is a module. Verified unchanged afterwards:
  `[viz-kernel] level=long checks=237 passed=237 failed=0 errored=0`.
- 1 of them: `📊️visualization-gallery/🟦️.ts` imported `node:crypto` for `pdfStableHash`, a PDF
  digest no production caller uses — it exists for the gallery render evidence. It and its private
  `inflatePdfStreamBody` moved to `📊️visualization-gallery/🔬️probes/🟦️.ts`; the module no longer
  imports `node:crypto` or `node:zlib`, and `🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts`
  imports the probe.

**7 × `testing/oracle`.**
- `oracle-capability-mismatch` ×3: the three flipped cases already carried `@oracle-gl-matrix`,
  `@oracle-fft-js` and `@oracle-mathjs`, but the registry entries still listed the case slugs as
  their capabilities instead of the capability the features declare. `gl-matrix` →
  `viz-scientific-3d`, `fft-js` → `viz-scientific-signal`, `mathjs` →
  `viz-scientific-function-sampling`, `jstat` → `viz-scientific-distributions`.
- The four library-named no-oracle decisions (`mathjs`, `jstat`, `fft-js`, `gl-matrix`) were
  placeholders for exactly those flips and are deleted.
- `differential-without-evidence` ×4. Every no-oracle decision in the file claimed
  `independent-second-implementation`, a word that is **not in the schema's substitute enum**
  (`specification-vectors`, `metamorphic-laws`, `independent-implementations`, `second-parser`), so
  the manifest was schema-invalid and the claim was invisible to the rule. The platform's
  `independent-implementations` means a **second adapter**, not a second routine inside the one
  adapter, so the four affected decisions now say `specification-vectors`, which is what they
  actually rest on, and their differential scenarios are restated as `@mode-conformance` — the mode
  is declarative only (`TEST_MODES` is never read by the runner), so the same numbers are compared
  at the same tolerance; only the claim is now true.
  - `🧬️biology-kaplan-meier` (3 scenarios): decision renamed `jstat` → `survival-product-limit`.
    jStat has distributions but no product-limit estimator, no censoring and no cumulative hazard.
  - `🏹️physics-projectile-rk4` (2): decision renamed `mathjs` → `projectile-integration`. No
    registered package integrates an ODE.
  - `🌊️field-streamlines` (1) and `📅️diagram-gantt-cpm` (0 differential, claim-only breach).
  - `❄️geometry-tilings-fractals`: its one differential scenario (`regular-lattice`) compares the
    probe against the lattice the feature itself specifies → `@mode-conformance`.
- **`☀️diagram-sunpath` gained a real oracle instead.** The feature already said it would become
  `@oracle-suncalc` "once suncalc is registered" — it *is* registered and installed (1.9.0). The
  adapter's hand-copied mean-anomaly model is deleted; the oracle is `SunCalc.getPosition` on the
  UTC instant the samples name, converted from radians to the degrees the probe emits. Both
  scenarios are now `@mode-differential`, the `solar-position` decision is deleted, and the suncalc
  entry declares `diagram-solar-position`.

### 2 — Seven more twin cases, and the two the twin cannot honestly serve

The 22 `-twin` cases from before the cut are complete. Seven more were added, one per remaining kernel
case that the twin actually implements, each measured against the SAME oracle as the case it doubles
and, where the vectors allow, through that case's own oracle handler:

| twin case | oracle | note |
|---|---|---|
| `🕝️format-time-twin` | d3-time-format | see below |
| `🔻️spatial-delaunay-voronoi-twin` | d3-delaunay | reuses the base oracle |
| `🥥️spatial-hull-twin` | d3-delaunay | reuses the base oracle |
| `🍯️spatial-hexbin-twin` | d3-hexbin | reuses the base oracle |
| `🗻️spatial-contours-density-twin` | d3-contour | `marching-squares` only |
| `🌐️network-circular-arc-twin` | d3-array | `circular` and `arc` only |
| `🥪️network-layered-twin` | dagre | all three scenarios |

Three deliberate absences, each recorded in the twin feature's own description rather than papered
over in an adapter:

- **`kernel-density`** is not in the contours twin. `vizDensity2d` divides the kernel sum by the
  sample size; `\SemioVizDensity` does not. That is a specification question, not a test defect — see
  the note to POLISH in `📓️integration.md`.
- **`shell` and `adjacency`** are not in the circular-arc twin: the twin has no such layout.
- **`✒️mark-geometry`, `🕸️network-algorithms` and `🧭️coordinate-polar-ternary` get no twin at all.**
  The twin has no §0 primitive renderer and no graph-metrics module, and its coordinate systems map
  into a y-DOWN page frame (`coordinateCartesian(FRAME).project(0,0)` is the frame's BOTTOM-left)
  while `semio-viz-coordinate` works in a y-up millimetre frame. Comparing the last one needs a frame
  reconciliation, not an adapter; recorded for whoever takes the coordinate layer next.

**`🕝️format-time-twin` does not reuse the base oracle, and the reason is the finding.** The LaTeX
formatter is handed calendar fields with no zone, so `🕰️format-time` adjudicates with d3's
`utcFormat`. The twin's `vizTimeFormat` reads a `Date` through its LOCAL getters. The first version of
this case failed on three directives (`%H`, `%I`, `%Z`) — a one-hour shift and a zone offset. The
honest answer was to ask d3 the question the twin is answering: the case uses d3's plain `format` over
the same locale definitions and the same instants. `%Z` stays in the vector list precisely because it
is where the two clocks are visible.

### 3 — `👯️kernel-twin-parity`

New case, oracle `print-viz-kernel-twin` registered as `cross-semio-implementation` (supplemental —
the registry's own words: "a second implementation written inside this repository … explicitly NOT
independent evidence"), subject the LaTeX probe. Six scenarios: `plain-matrix`,
`circular-equal-spacing` and `align-justify` compose the base case's LaTeX subject with its `-twin`'s
twin subject over the same tables, so no vector is restated; `scale-linear-and-log`, `geo-mercator`
and `spatial-hull` are the case's own probe documents, built inline.

### 4 — `catalog-coverage`: the API reference moved to `long`

Per the coordinator's decision. `generatedFindings()` now measures only the catalogue-derived
artifacts (`semio-viz-catalog.sty`, `semio-viz-catalog-labels.sty`, the gallery document per section)
at `@level-fundamental`; the new `@id-api-reference` scenario at `@level-long` measures
`🖼️assets/🔣️viz-api.json` against what `generate viz` would write. One command still refreshes both.

### 5 — The bug that was hiding every probe subject's numbers

`parity quick` for the print owner was `parity=192/225` with 33 failures. **Twenty-nine of them were
one mistake, and not in the LaTeX.** INTEGRATION-2's item 20 made the shared `subject()` / `probe()`
helpers return `{ projection }`, and a follow-up script removed the now-double wrap where a scenario
returned a helper's result directly — but left every call site that READS A KEY out of that result
indexing the wrapper:

```ts
(await subject(ctx, "bracket.tex"))["geometry/annotation-bracket"] ?? []   // always []
```

The compiled document held exactly the oracle's numbers — `bracket.probe.jsonl` said
`{"key":"geometry/annotation-bracket","values":[20,12,50,12,2]}` against an oracle of
`[20,12,50,12,2]` — while the subject reported an empty array. `🔧️unwrap-projection-reads.py` fixes
29 such reads across 12 cases, restricted to helpers each file declares as returning
`Promise<{ projection: ProbeProjection }>`. Five more helpers had the opposite defect (declared
wrapped, returning bare) and two more sites bound the wrapper to a name.

Cases that went from red to green with no LaTeX change at all:

```
$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print" --case <case>
🏷️annotation-placement    parity=3/3
🕸️network-algorithms      parity=5/5
⭕️network-circular-arc     parity=4/4
🎚️network-layered         parity=3/3
☀️diagram-sunpath         parity=2/2   (now against the real suncalc oracle)
```

### 6 — Levels, targets and launch entries

`📜️script.ts test <level>` was already routed through `PrintPipelineVerificationCommand`; the
`exhaustive` level was missing `verifyPrintVisualizationBuild()` (the `test viz full` half) and now
runs it before the gallery-render fixture check. `runPrintPlatformCases` keeps to `parity` on purpose
and its docstring now says why: `contract` is a repository-wide sweep whose exit code carries every
owner's breaches, so calling it from print's own suite would fail print for a breach it does not own.

The nx targets (`test`, `test-fundamental`, `test-quick`, `test-long`, `test-exhaustive`, `test-viz`,
`test-viz-full`, `test-viz-fixtures`) and the `🧪️test🖨️print…` launch entries were already complete.
`@semio-tech/print-viz-kernel` had nx targets but **no launch entries**; three were added
(`🧪️test🖨️print📊️viz-kernel⚡️quick` / `🌕️long` / `🌌️exhaustive`) and `.vscode/launch.json` was
regenerated with the registry generator:

```
$ cd 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry && bun ./📜️script.ts generate
plugin registry catalog refreshed (59 plugin crates, 60 playgrounds, 46 framework packages)
.vscode/launch.json regenerated -> C:\git\semio\.vscode\launch.json
```

### 7 — `parity quick` for the print owner: 192/225 → **239/241**

```
$ cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test
$ bun ./📜️script.ts parity quick --owner "🧰️framework/🛍️products/📓️print"
[test] level=quick cases=100 executed=513 passed=503 failed=0 errored=10 parity=239/241 not-exercised=12
[test] parity failed: …::📶️charts-histogram-density::bins::typescript::subject (6 differences)
[test] parity failed: …::📶️charts-histogram-density::bins-coarse::typescript::subject (6 differences)
```

- **The two remaining failures are one LaTeX defect**, reported to POLISH in `📓️integration.md`:
  `\semio_viz_tr_stat_bin:Nn` appends a bin above the data maximum (`[10,11)`, count `0`) that d3
  never emits. The eight bins below it match d3 one-for-one, counts included, so the fixture and the
  oracle are right and the package is wrong.
- **`errored=10` is stale by one run.** Those ten executions are the five adapters whose helper
  returned a bare map (`✒️mark-geometry`, `💬️diagram-sequence`, `📅️diagram-gantt-cpm`,
  `🚦️diagram-routing` ×2, `🛣️diagram-layout-lanes` ×3, and mark-geometry's pair) — found and fixed
  while this run was in flight.
- **`not-exercised=12` is all correct.** Eight `charts-*` cases declare only `@level-long` scenarios,
  and four are no-oracle cases whose evidence the subject phase discharges. Two of the twelve are
  the other agents' new cases (`🏭️domain-families`, `🎪️showcase-families`), which now carry the
  emoji identity the taxonomy requires.
- No `hierarchy-pack` budget kill this run. INTEGRATION-2's item 18 (one tectonic pass instead of
  three, no synctex, no PDF) took the case under the platform's 30 s quick budget on its own, so the
  choice the coordinator asked POLISH and me to make — faster kernel or move to `long` — does not
  need making. `🫧️hierarchy-pack` and `🎈️hierarchy-pack-twin` are both green at `quick`.

### 8 — `parity long` for the print owner: **255/276**, and what the survivors are

```
$ bun ./📜️script.ts parity long --owner "🧰️framework/🛍️products/📓️print"
[test] level=long cases=100 executed=591 passed=562 failed=0 errored=29 parity=255/276
… 21 parity failed lines …
```

Every survivor is triaged, and **all but one are LaTeX defects in packages I do not own**. Reported
to POLISH in `📓️integration.md` with the reproducing command and the exact error:

| what | cases | scenarios |
|---|---|---|
| six `semio-viz-charts-*` / `semio-viz-matrix-*` packages call `\semio_viz_tr_*` without `\RequirePackage{semio-viz-transform}` | `🎻️charts-box-violin`, `🎯️charts-evaluation-curves`, `🔥️charts-heatmap-matrix`, `🎡️charts-polar-radar`, `🥧️charts-pie-donut`, `🔲️charts-quadrant-table` | 16 |
| `\semio@viz@font@{cell,chip,label,title}` used 141 times, defined nowhere | `💬️diagram-sequence`, `📅️diagram-gantt-cpm`, `🚦️diagram-routing`, `🛣️diagram-layout-lanes` | 9 |
| `\semio_viz_tr_stat_bin:Nn` appends a bin above the data maximum | `📶️charts-histogram-density` | 2 |
| `🔣️viz-api.json` is stale — `generate viz` refreshes it | `📚️catalog-coverage::api-reference` | 1 |

Four more errored cases were mine and are fixed since that run:

- `🗓️charts-timeline` — the fixtures still asked for `demo-interval`, which
  `🔧️split-demo-interval.py` renamed to `demo-lane`. Now `executed=2 passed=2 errored=0`.
- `⏱️charts-kpi-gauge` — the adapter asserted a `geometry/arc` record carries exactly five numbers;
  the probe emits six, so a drawn gauge reported `drew no value arc`. Now `parity=2/2`.
- `✒️mark-geometry` — bare-return helper. Now `executed=1 passed=1 errored=0`.
- `📏️guide-axis-ticks` and `🏔️spatial-contours-density` — stale kernel API in the fixtures, above.

### 9 — The print router's own levels

```
$ cd 🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript
$ bun ./📜️script.ts test fundamental
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 128/128
[DEBUG] print: unit tests passed
[test] level=fundamental cases=100 executed=15 passed=15 failed=0 errored=0 parity=7/7 not-exercised=98
exit=0
```

Green end to end, including the coverage check that used to abort the router before it reached the
platform (`AssertionError: actual: [ "\\SemioVizChartKind" ]`).

```
$ bun ./📜️script.ts test quick
[DEBUG] print: viz coverage 1966/1966 leaves through 1738 kinds, API 128/128
[DEBUG] print: unit tests passed
[test] level=quick cases=100 executed=513 passed=504 failed=0 errored=9 parity=239/241 not-exercised=12
[test] parity failed: …::📶️charts-histogram-density::bins::typescript::subject (6 differences)
[test] parity failed: …::📶️charts-histogram-density::bins-coarse::typescript::subject (6 differences)
error: bun ./📜️script.ts parity quick --owner …/📓️print exited with status 1
exit=1
```

The router chains correctly: the fundamental half runs, then the platform. The non-zero exit is the
one LaTeX defect and the nine font-macro casualties, not a wiring fault.

### On `generate viz` and the `api-reference` scenario — deliberately not run

`🔣️viz-api.json` is derived from the LaTeX sources, so it goes stale the moment anyone edits a
package — which POLISH, FAMILIES-DOMAIN and FAMILIES-CAPABILITY are all doing right now. That is
exactly the flapping the coordinator moved out of `fundamental`, and the `long` scenario reporting it
is the scenario working. Refreshing it in the middle of three agents' edits would buy a green line
for a few minutes and risks a torn read for a concurrent `build viz`. The remedy is one command,
recorded for whoever closes the ticket:
`cd 📓️print/📦️packages/🟦️typescript && bun ./📜️script.ts generate viz` with `tasklist | grep
tectonic` empty first.

### 10 — The oracle manifest validates against the schema now

```
$ bun -e '…ajv.compile({ $ref: "root#/$defs/OracleRegistry" })(printOracleManifest)'
valid: true
```

Four invented substitute words kept it invalid: `independent-second-implementation` on seven
decisions, plus `self-consistency-rebuild` and `pdf-text-extraction-probe`. The enum has four
members and each invented word maps onto one — a rebuild that reproduces its own bytes is a
`metamorphic-laws` claim, a PDF text extractor reading the artifact back is a `second-parser`, and a
reference computed inside the one adapter is a `specification-vectors` claim. Nothing in the contract
validates these manifests today, so the typos were silent; noted for everyone in `📓️integration.md`.

### 11 — The case x scenario matrix

Generated from the platform's own result records by `🔧️parity-matrix.py` (execution status from
`⚡️cache/tests/results`, the `differs` mark from a `⚡️cache/tests/diffs` entry newer than that
case's record — the same condition under which the run printed `parity failed`). This is the state
after the final `parity long`.

| case | scenarios | subject × scenario |
|---|---|---|
| `⌛️scale-temporal-twin` | 1 | map ok |
| `⏱️charts-kpi-gauge` | 2 | progress-ring ok, radial-gauge ok |
| `⏳️scale-temporal` | 5 | day-ticks ok, map ok, month-ticks ok, week-ticks ok, year-ticks ok |
| `☀️diagram-sunpath` | 2 | solstice-day-arc ok, southern-winter-arc ok |
| `✒️mark-geometry` | 1 | primitives ok |
| `❄️geometry-tilings-fractals` | 3 | l-system-words ok, penrose-substitution ok, regular-lattice ok |
| `➗️math-functions-sampling` | 3 | adaptive-refinement ok, riemann-rules ok, uniform-sampling ok |
| `➰️shape-curves` | 3 | degenerate ok, interpolators ok, parameters ok |
| `➿️shape-curves-twin` | 3 | degenerate ok, interpolators ok, parameters ok |
| `⭕️network-circular-arc` | 4 | adjacency-degree-order ok, arc-spacing ok, circular-equal-spacing ok, shell-rings ok |
| `🌀️network-force-twin` | 4 | collide-and-axis-forces ok, default-forces-20-ticks ok, parameterised-link-and-charge ok, phyllotaxis-start ok |
| `🌊️field-streamlines` | 1 | rk-integration ok |
| `🌌️charts-scatter-trend` | 2 | bubble-size ok, linear-trend ok |
| `🌍️geo-projections` | 3 | fit-extent ok, forward-projection ok, inverse-projection ok |
| `🌎️geo-projections-twin` | 2 | fit-extent ok, forward-projection ok |
| `🌐️network-circular-arc-twin` | 2 | arc-spacing ok, circular-equal-spacing ok |
| `🌗️shape-arc-pie` | 3 | arc-centroid ok, arc-geometry ok, pie-angles ok |
| `🌘️shape-arc-pie-twin` | 3 | arc-centroid ok, arc-geometry ok, pie-angles ok |
| `🌲️hierarchy-aggregates-twin` | 4 | depth-and-height ok, sort ok, sum-and-count ok, traversal ok |
| `🌳️hierarchy-aggregates` | 5 | depth-and-height ok, path-stratify ok, sort ok, sum-and-count ok, traversal ok |
| `🌴️hierarchy-tree-cluster` | 5 | cluster-node-size ok, cluster-size ok, tree-node-size ok, tree-separation ok, tree-size ok |
| `🍯️spatial-hexbin-twin` | 1 | hexagonal-binning ok |
| `🍰️transform-stack-twin` | 6 | offset-diverging ok, offset-expand ok, offset-silhouette ok, order-ascending ok, order-none ok, order-reverse ok |
| `🎀️shape-links-ribbons` | 2 | links ok, ribbons ok |
| `🎈️hierarchy-pack-twin` | 4 | default-radius ok, explicit-radius ok, padding ok, unbalanced ok |
| `🎋️hierarchy-tree-cluster-twin` | 5 | cluster-node-size ok, cluster-size ok, tree-node-size ok, tree-separation ok, tree-size ok |
| `🎗️network-chord-twin` | 2 | padded-and-sorted ok, plain-matrix ok |
| `🎚️network-layered` | 3 | barycenter-ordering ok, ranks-agree-with-dagre ok, slack-ranks-are-as-soon-as-possible ok |
| `🎡️charts-polar-radar` | 4 | coxcomb ok, polar-bars ok, polar-scatter ok, radar ok |
| `🎨️scale-color` | 6 | diverging-position ok, interpolate-hcl ok, interpolate-lab ok, interpolate-rgb ok, ramp-stops ok, sequential-position ok |
| `🎪️showcase-families` | 2 | capabilities ok, namespaces ok |
| `🎬️render-scene` | 4 | bar-rectangles ok, options-change-the-projection ok, scene-graph-primitives ok, tikz-mirrors-the-scene ok |
| `🎯️charts-evaluation-curves` | 4 | confusion ok, gain ok, pr ok, roc ok |
| `🎻️charts-box-violin` | 4 | letter ok, quartiles ok, violin ok, whiskers ok |
| `🏔️spatial-contours-density` | 2 | kernel-density ok, marching-squares ok |
| `🏭️domain-families` | 2 | every-domain-kind-draws-its-own-geometry ok, term-glyph-size-follows-the-frequency-extent ok |
| `🏷️annotation-placement` | 3 | bracket-normal ok, data-space-anchors ok, reference-extent ok |
| `🏹️physics-projectile-rk4` | 3 | damped-decay ok, harmonic-orbit ok, projectile-path ok |
| `🐝️spatial-hexbin` | 1 | hexagonal-binning ok |
| `👯️kernel-twin-parity` | 6 | align-justify ok, circular-equal-spacing ok, geo-mercator ok, plain-matrix ok, scale-linear-and-log ok, spatial-hull ok |
| `💬️diagram-sequence` | 2 | message-y-positions ok, self-call-keeps-its-step ok |
| `💹️charts-financial` | 2 | candlestick ok, moving-average ok |
| `📅️diagram-gantt-cpm` | 2 | forward-and-backward-pass ok, independent-cpm-reference ok |
| `📈️charts-line-area` | 4 | area-stack ok, linear ok, step-after ok, step-before ok |
| `📉️scale-continuous-twin` | 5 | linear ok, log ok, nice ok, pow ok, symlog ok |
| `📊️charts-bar-layout` | 5 | diverging ok, grouped ok, horizontal ok, percent ok, stacked ok |
| `📋️transform-statistics` | 4 | aggregates ok, kde ok, quantiles ok, regression ok |
| `📏️guide-axis-ticks` | 5 | axis-geometry ok, band-ticks ok, linear-ticks ok, log-ticks ok, tick-format-labels ok |
| `📐️scale-continuous` | 5 | linear ok, log ok, nice ok, pow ok, symlog ok |
| `📚️catalog-coverage` | 9 | api-reference ok, distinctness ok, families ok, generated ok, languages ok, leaves ok, options ok, schema ok, slugs ok |
| `📡️signal-dft-bode` | 3 | dft-magnitudes ok, polynomial-roots ok, transfer-function ok |
| `📶️charts-histogram-density` | 5 | bins **differs**, bins-coarse **differs**, density ok, ecdf ok, ticks ok |
| `🔔️math-distributions` | 3 | continuous-laws ok, discrete-laws ok, quantiles ok |
| `🔗️shape-links-ribbons-twin` | 1 | links ok |
| `🔠️scale-discrete` | 6 | band ok, ordinal ok, point ok, quantile ok, quantize ok, threshold ok |
| `🔡️scale-discrete-twin` | 6 | band ok, ordinal ok, point ok, quantile ok, quantize ok, threshold ok |
| `🔢️format-number` | 2 | de-locale ok, en-locale ok |
| `🔥️charts-heatmap-matrix` | 3 | cells ok, heatmap ok, heatmap-padded ok |
| `🔬️probe-protocol` | 2 | affine-mapping ok, power-mapping ok |
| `🔲️charts-quadrant-table` | 4 | quadrant ok, risk ok, table ok, table-bars ok |
| `🔶️shape-symbols-twin` | 1 | symbol-paths ok |
| `🔷️shape-symbols` | 1 | symbol-paths ok |
| `🔺️spatial-delaunay-voronoi` | 2 | delaunay-triangles ok, voronoi-cells ok |
| `🔻️spatial-delaunay-voronoi-twin` | 2 | delaunay-triangles ok, voronoi-cells ok |
| `🕝️format-time-twin` | 2 | de-locale ok, en-locale ok |
| `🕰️format-time` | 2 | de-locale ok, en-locale ok |
| `🕸️network-algorithms` | 5 | connected-components ok, degrees ok, lcg-stream ok, topological-order ok, traversal-order ok |
| `🖌️scale-color-twin` | 3 | diverging-position ok, interpolate-rgb ok, sequential-position ok |
| `🖼️gallery-render` | 1 | family-option-distinctness ok |
| `🗃️data-csv` | 3 | custom-delimiter ok, header-row ok, quoted-fields ok |
| `🗓️charts-timeline` | 2 | interval ok, swimlane ok |
| `🗝️guide-legend` | 3 | categorical-entries ok, size-entries ok, vertical-layout ok |
| `🗺️geo-path-graticule` | 3 | graticule-lines ok, path-with-resampling ok, path-without-resampling ok |
| `🗻️spatial-contours-density-twin` | 1 | marching-squares ok |
| `🗾️hierarchy-treemap` | 4 | padding ok, ratio-and-round ok, tiling ok, unbalanced ok |
| `🚦️diagram-routing` | 2 | orthogonal-vertical ok, straight-ports ok |
| `🚰️flow-sankey` | 4 | align-center ok, align-justify ok, align-left ok, align-right ok |
| `🚿️flow-sankey-twin` | 4 | align-center ok, align-justify ok, align-left ok, align-right ok |
| `🛣️diagram-layout-lanes` | 3 | direction-transposes-the-grid ok, grid-placement ok, lane-bands ok |
| `🥚️spatial-hull` | 1 | convex-hull ok |
| `🥞️transform-stack` | 6 | offset-diverging ok, offset-expand ok, offset-silhouette ok, order-ascending ok, order-none ok, order-reverse ok |
| `🥥️spatial-hull-twin` | 1 | convex-hull ok |
| `🥧️charts-pie-donut` | 4 | donut **errored**, pie ok, pie-padded ok, pie-unsorted ok |
| `🥪️network-layered-twin` | 3 | barycenter-ordering ok, ranks-agree-with-dagre ok, slack-ranks-are-as-soon-as-possible ok |
| `🧇️hierarchy-treemap-twin` | 3 | padding ok, tiling ok, unbalanced ok |
| `🧊️3d-projection` | 3 | axonometric ok, isometric ok, perspective-and-oblique ok |
| `🧩️composition-concat-inset` | 3 | concat-horizontal ok, dashboard-span ok, inset-rectangle ok |
| `🧬️biology-kaplan-meier` | 3 | cumulative-hazard ok, product-limit ok, two-groups ok |
| `🧭️coordinate-polar-ternary` | 5 | cartesian ok, geographic ok, parallel ok, polar ok, ternary ok |
| `🧮️format-number-twin` | 2 | de-locale ok, en-locale ok |
| `🧱️hierarchy-partition` | 3 | icicle ok, sunburst ok, unbalanced ok |
| `🧲️network-force` | 4 | collide-and-axis-forces ok, default-forces-20-ticks ok, parameterised-link-and-charge ok, phyllotaxis-start ok |
| `🧺️transform-bin-twin` | 4 | count-five ok, count-twenty ok, explicit-thresholds ok, sturges ok |
| `🧾️transform-statistics-twin` | 3 | aggregates ok, kde ok, quantiles ok |
| `🧿️geo-path-graticule-twin` | 1 | path-without-resampling ok |
| `🪜️hierarchy-partition-twin` | 3 | icicle ok, sunburst ok, unbalanced ok |
| `🪟️facet-layout` | 2 | shared-axes ok, wrap-grid ok |
| `🪢️network-chord` | 2 | padded-and-sorted ok, plain-matrix ok |
| `🪣️transform-bin` | 4 | count-five ok, count-twenty ok, explicit-thresholds ok, sturges ok |
| `🫧️hierarchy-pack` | 4 | default-radius ok, explicit-radius ok, padding ok, unbalanced ok |

cases 100, scenarios 315, not green 3

The three that are not green are the two `\semio_viz_tr_stat_bin:Nn` scenarios and
`🥧️charts-pie-donut::donut`, whose `semio-viz-charts-polar.sty` still lacks
`\RequirePackage{semio-viz-mark}`. All three are LaTeX packages, all three are reported.

### 12 — The final runs

```
$ bun ./📜️script.ts parity long --owner "🧰️framework/🛍️products/📓️print"
[test] level=long cases=100 executed=591 passed=590 failed=0 errored=1 parity=273/276
[test] parity failed: …::📶️charts-histogram-density::bins::typescript::subject (6 differences)
[test] parity failed: …::📶️charts-histogram-density::bins-coarse::typescript::subject (6 differences)
[test] parity failed: …::🥧️charts-pie-donut::donut::typescript::subject (1 differences)
```

255/276 with `errored=29` an hour earlier: POLISH's `\RequirePackage{semio-viz-transform}` and the
font macros landed between the two runs, and `📚️catalog-coverage::api-reference` came back green with
them. The last three are two instances of `\semio_viz_tr_stat_bin:Nn`'s trailing empty bin and one
more missing requirement — `semio-viz-charts-polar.sty` calls `\semio_viz_pie_from_seq:Nnnnn` and
does not require `semio-viz-mark`. I audited that whole class rather than reporting it one package at
a time: `🔧️package-closure.py` computes, for every `semio-viz-*.sty`, the macros it calls that are
neither its own nor in its transitive `\RequirePackage` closure. **Twelve packages have a gap**, and
the output is in `📓️integration.md` for POLISH.

```
$ bun ./📜️script.ts contract --owner "🧰️framework/🛍️products/📓️print"
1212 high-priority breach(es) across 5 rule(s):
    671  testing/contract
    335  testing/fixture
    159  testing/dependency
     42  testing/oracle
      5  testing/discovery
# print's own share, re-checked after every change above: 0 lines
```

### 13 — One thing the launch entries cannot be exercised through today

`bun nx run` fails for every project in the workspace:

```
 NX   Failed to process project graph.
The following projects are defined in multiple locations:
- test-s-plugins-puzzle-artifacts-2d-standards-1-subsets-any-00b55c-third-party-puzzle-2d-1:
  - ✏️s/…/🪆️subsets/✳️any/🧪️tests/🌐️third-party-puzzle-2d-1
  - ✏️s/…/🪆️subsets/✳️any/🧪️tests/🕸️third-party-puzzle-2d-1
```

Two puzzle-plugin cases differ only by their leading emoji, and `testProjectName()` strips that
identity by design, so both claim one nx name. Not print's tree; reported in `📓️integration.md`.
Print's own 100 cases are collision-free — 100 cases, 100 distinct project names — and every
underlying command works directly:

```
$ cd 📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript && bun ./📜️script.ts test long
[viz-kernel] level=long checks=237 passed=237 failed=0 errored=0
```

## In progress

(nothing — see Blocked / open)

## Blocked / open

- **Three scenarios are red, all three in LaTeX packages, all three reported to POLISH in
  `📓️integration.md` with the reproducing command.** POLISH already fixed the two big ones during
  this session (`\semio@viz@font@{cell,chip,label,title}`, 141 uses and 0 definitions; and
  `\RequirePackage{semio-viz-transform}` in six chart and matrix packages), which took `parity long`
  from 255/276 to 273/276. What is left:
  - `\semio_viz_tr_stat_bin:Nn` appends a bin above the data maximum — 2 scenarios.
  - `semio-viz-charts-polar.sty` calls `\semio_viz_pie_from_seq:Nnnnn` without requiring
    `semio-viz-mark` — 1 scenario, and one of **twelve** packages whose transitive
    `\RequirePackage` closure does not cover what they call. `🔧️package-closure.py` in this folder
    computes the whole set; the output is in `📓️integration.md`. Two shapes: a genuinely missing
    requirement, and a `\cs_generate_variant:Nn` of another package's function generated in the
    wrong package, so seven packages call a variant that exists only if an unrelated package was
    loaded first. Neither shows up in a full gallery document, only in a minimal one — which is
    exactly what a probe fixture and a single gallery section are.
- **`test viz full`, `test exhaustive` and the `gallery-render` fixture regeneration are deliberately
  not run yet.** They compile the whole gallery, which today cannot render: 25 catalogue families
  still have no renderer (FAMILIES-DOMAIN and FAMILIES-CAPABILITY are writing them as I write
  this), the twelve requirement gaps above kill any section that loads one of those packages
  alone, and INTEGRATION-2 recorded that concurrent `build viz` runs collide on
  `dist/source/<template>` with `EBUSY` — three agents are building right now. Running them in
  that state would spend hours re-reporting known, owned work as failures. The commands, in
  order, once the packages are fixed and the families land:
  ```
  cd 🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript
  tasklist | grep tectonic          # must be empty
  bun ./📜️script.ts generate viz
  bun ./📜️script.ts test viz full
  bun ./📜️script.ts test viz fixtures   # regenerates 🗺️gallery fixtures; never hand-edit them
  bun ./📜️script.ts test exhaustive
  ```
  Distinctness — the check that made the previous library ship 1,857 identical renders — **passes
  today** and needs no gallery build: `verifyPrintGalleryDistinctness()` reports no family whose two
  kinds carry identical options.
- `🗑️generated/` under this ticket was wiped by something outside this session while a run was
  writing into it (the run died with exit 255). Run logs go to the session scratchpad and are copied
  back here.

## Files touched

- `🧰️framework/🛍️products/📓️print/🧪️tests/` — all 90 case directories renamed; 26 adapters' sibling
  imports rewritten; features and adapters of `☀️diagram-sunpath`, `🧬️biology-kaplan-meier`,
  `🏹️physics-projectile-rk4`, `🌊️field-streamlines`, `❄️geometry-tilings-fractals`.
- `🧰️framework/🛍️products/📓️print/🔮️oracle/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️viz-kernel/📦️packages/🟦️typescript/📜️script.ts` and new
  `…/🔬️probes/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts` and new `…/🔬️probes/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/` — seven new twin cases (`🕝️format-time-twin`,
  `🔻️spatial-delaunay-voronoi-twin`, `🥥️spatial-hull-twin`, `🍯️spatial-hexbin-twin`,
  `🗻️spatial-contours-density-twin`, `🌐️network-circular-arc-twin`, `🥪️network-layered-twin`) and
  `👯️kernel-twin-parity`; `📚️catalog-coverage` split; 29 wrapper reads fixed across 12 cases; nine
  bare-return helpers wrapped; three `📏️guide-axis-ticks` fixtures and
  `🏔️spatial-contours-density/🧫️fixtures/density.tex` rewritten onto the API that exists.
- `.vscode/🧩️launch.seed.jsonc` and the regenerated `.vscode/launch.json`
- ticket inputs: `🔧️case-emoji.tsv`, `🔧️oracle-scientific-flip.py`, `🔧️oracle-substitutes.py`,
  `🔧️extract-kernel-probes.py`, `🔧️clone-twin-features.py`, `🔧️register-twin-capabilities-2.py`,
  `🔧️register-twin-oracle.py`, `🔧️unwrap-projection-reads.py`, `🔧️parity-matrix.py`,
  `🔧️oracle-substitute-vocabulary.py`, `🔧️package-closure.py`
