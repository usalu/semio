# Axis Customization and Defaults Verification

The canonical numerical axis renderer now resolves every axis option registered by `semio-viz-guide.sty` into owned geometry or explicit backend style. No native stylesheet was edited by this owner. Native legend and angular-title verification remain coordinated with the native owner.

## Contract and Ownership

The shared schema owns axis defaults under `ChartGuide.x-semio-guide-defaults.axis`: tick size 1.4 mm, outer caps 1.4 mm, tick padding 0.8 mm, title gap 5 mm, label size 6.6 TeX points and title size 7.2 TeX points. The chart snapshot exports `VIZ_AXIS_DEFAULTS` directly from those first-party schema values; numerical inference consumes that facade. The native constants were read directly from the current guide reset and theme font roles. A focused native compiler probe measures those exact reset registers and actual selected font sizes against the schema.

Authored option fields override the corresponding typed guide fields. `domainLine` overrides `domain`; `labelRotation` overrides `labelRotate`, matching the native sorted field emission. Option tick values use the native comma-list contract, preserve ordering, and an empty list uses generated ticks. Localized typed titles remain language-explicit; native option title text overrides them.

## Registered Axis Key Inventory

| Native keys | Canonical behavior |
| --- | --- |
| scale, orient, offset, title | Common guide context applies explicit options before typed fields. |
| ticks, tickValues, tickFormat | Shared tick and label preparation applies explicit options before typed controls and scale settings. |
| stroke, fill, strokeWidth, labelSize, titleSize | Axis primitives carry authored paint, stroke width and physical text size. |
| gridStroke, gridDash | Grid line paint and dash override the axis paint. |
| domain, domainLine | Axis domain defaults on; primary alias wins; straight domains include native outer caps. |
| labelRotation, labelRotate, labelAlign, labels | Aliases, anchor choice, label rotation and label suppression are honored. |
| at, from, to | Cartesian axis location and arbitrary segment endpoints are honored. |
| center, innerRadius, outerRadius, angle | Radial and angular geometry use authored center/radii; radial angle selects its segment. |
| startAngle, endAngle | Angular domain arcs and title anchors use the authored angular span. |
| titleAnchor, titleGap | Titles follow domain endpoints/arc, with outward tick size plus gap; native title rotation is zero. |
| tickSize, tickSizeOuter, tickPadding | Major tick lengths, domain caps and label offsets use schema-owned native defaults. |
| minor, minorTicks, minorSize | Interior mapped-position subdivisions use authored count/length and produce no labels. |
| grid, gridLength | Grid generation honors explicit length, including segment and polar normals. |
| broken, breakGap, breakMarks | Removed values are omitted; geometry is compressed and native break marks are emitted. |
| style | Opaque native TikZ text is retained in owned `tikzStyle` and appended after generated path/node options. |

Arbitrary native TikZ style is backend-specific. The portable numerical scene retains resolved geometry and typed styling; it cannot execute TeX macros. This is represented explicitly rather than approximating arbitrary PGF syntax. The closed inference result schema admits the optional string on every primitive variant, while the portable scene remains closed.

## Neutral Tests and Before Evidence

The new `render-scene/axis.json` contains 24 language-neutral cases. Nineteen cover overrides, axis location, caps, grid length, segment/radial/angular geometry, minor ticks, break gap/marks and native style. Five additional cases constrain native defaults and angular title anchors. Existing renderer cases and Gherkin census expectations were handcrafted for capped native defaults: the two demo axes now produce 18 lines rather than 14; a two-category band axis now produces seven primitives rather than five.

The independent path oracle feeds authored neutral line endpoints through D3 shape's line generator and owned path recording; existing guide cases independently use D3 scales and formatting. Each new subject executes the canonical asynchronous worker and checks published geometry/text, style preservation, and emitter output. Numerical comparisons allow 1e-9 tolerance.

A controlled diagnostic restored only the preceding cartesian axis implementation in the private helper, preserving unrelated concurrent source. This explicitly reconstructed baseline is recorded as such: 53 selected checks, 38 passes and 15 failures. The implemented helper was then restored, producing 53/53 passing checks. The registered pre-style baseline separately produced 113/114 render checks, with `axis-native-style` failing because all style fields were absent and emitted style was false.

The default red stage selected 58 axis checks, with six failures: five actual geometry/font default mismatches and one neutral fixture padding transcription introduced while making old cases explicit. The fixture padding was corrected; the implementation then passed 58/58 actual worker checks. This transcription mismatch is not claimed as a product defect.

## Runtime and Registered Gates

- Registered render quick check: **119/119 passed**, zero errors.
- Registered exhaustive check: **545/545 passed**, zero errors; Nx target succeeded in 54.8 seconds.
- Final strict canonical and separately registered native grammar source checks: running after schema facade integration.
- Opaque style compiler proof: the canonical emitted TikZ executes the custom PGF key exactly five times (two major ticks, two labels and one title). Native comparison is running.
- Native schema-default register and physical-font proof: running.

The first standalone style probe used an article class and encountered a missing Latin Modern 12 metric before drawing. The established full `semio` class with staged first-party fonts compiled successfully. The first count projection used the literal-value protocol, which intentionally returned the unexpanded count token; it was corrected to the numerical evaluation protocol. Neither failed diagnostic is treated as successful runtime proof.

## Exact Owned Changes for This Wave

- `🧰️framework/🛍️products/📓️print/🧬️schema/📸️snapshot/📊️chart/🟦️.ts`: first-party schema default facade.
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🖼️render/🟦️.ts`: common guide overrides, private axis geometry, native defaults, title placement, opaque TikZ style emission. Native owner's concurrent legend changes are outside this owner's attribution.
- `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts`: neutral axis fixtures, canonical worker subjects, D3 path oracle, capped scene census and native-default guide oracle.
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️axis.json`: 24 neutral axis cases.
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️customization.json`: handcrafted band-axis default expectation.
- `🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🥒️.feature`: registered axis vectors, documented defaults and capped census.

Root-owned schema additions are recorded separately: the shared `ChartGuide` default metadata and closed `RenderItem.tikzStyle` admission. Prior nice/guide controls and strict native runner registration remain documented in `📓️derived-scale-and-guide-controls-audit-2026-10-04.md`. No permanent standalone scripts, runtime dependencies, compatibility wrappers, migration files, modifying git commands, worktrees or AGENTS edits were introduced.

## Text Baselines, Empty Titles and Context Defaults

The text-only closed IR extension carries `baseline` (`alphabetic`, `middle`, `top`, `bottom`). Scene nodes preserve it; TikZ uses exact base/north/south anchors with west/east combinations. Axis automatic alignment selects the outward top/bottom baseline for a vertical-dominant normal, while horizontal normals and explicit `labelAlign` use middle. The native guide label role has no explicit inner-separation override; canonical TikZ therefore retains actual PGF anchor semantics instead of guessing glyph metrics. Generic text remains middle.

Five actual axis worker comparisons failed before baseline propagation because plan/scene were middle and TikZ emitted center, while native semantics require north/south. A sixth meaningful failure was an empty native title option publishing an extra empty text item. Both are fixed. Twelve new direct IR combinations initially used a 10 mm chart with oversized default margins, causing harness exceptions; the neutral base now explicitly sets zero margins. Those exceptions are not product defects. Thirteen direct IR cases now cover every baseline/anchor combination plus absent baseline, and an additional axis case confirms explicit alignment suppresses automatic vertical baseline selection.

The registered render gate passed 132/132 after the baseline implementation; subsequent explicit-alignment/default-text additions were included in the exhaustive replay: **560/560 passed**, zero errors, Nx success in 1m39s.

A separate default-context red stage selected 62 axis checks: 60 passed and two failed. An omitted scale threw `unknown guide scale` under preceding common context; an omitted orientation on a y-encoded layer silently chose left although the native axis reset chooses bottom. Schema-owned context defaults and their fix are pending in this section.

Actual native style compiler projections are complete: **canonical 5 callbacks; native 5 callbacks**. Actual native schema-default compiler projections are complete: **[1.4,1.4,0.8,5]**, label **6.6 pt**, title **7.2 pt**, all equal the shared schema.

The initial default font probe used an unavailable expl3 `cs_use:c` name; it was corrected to `use:c`. Two concurrent font-publication attempts failed on Windows with `EPERM` at `process/artifacts/publication` line36: `print/📦️packages/🟦️typescript/dist/fonts.stage-BEXr6I` and `fonts.stage-pnJYXH` could not replace the open `dist/fonts`. The root owner is fixing identical concurrent font staging. A retry already active before that fix completed successfully after the consuming compiler exited. No failing attempt is called a pass.

The first final strict build passed canonical inference and the separately registered native grammar runner, exporting **249 own symbols** in **1m49s**. A later strict replay includes text baseline and remains in progress.

Additional exact owned file: `🧰️framework/🛍️products/📓️print/🧪️tests/🎬️render-scene/🔣️baseline.json` contains thirteen neutral baseline projections. Root-owned closed inference schema now admits baseline only on text primitives.

## Retained Native Probe Inputs

The standalone probe documents use `documentClass: "semio"`, `documentClassOptions: "type=paper,language=en"`, package `semio-viz`, and preamble title/author/date. This supplies the staged first-party fonts.

The style input defines a PGF key and counter:

```tex
\newcount\AuditStyleCount
\tikzset{semio audit style/.code={\global\advance\AuditStyleCount by1}}
```

The canonical input is the async worker's emitted picture for `axis-native-style`. The independent native input declares the 100×80 frame registers, then uses:

```tex
\begin{tikzpicture}[x=1mm,y=1mm]
\SemioVizScale{x}{linear}{0,10}{10,90}
\SemioVizAxis[scale=x,orient=bottom,tickValues={0,10},tickSize=2,tickPadding=3,domainLine=false,title={Styled},labelSize=11,titleSize=14,style={line width=0.6mm,semio audit style}]
\end{tikzpicture}
\SemioVizProbeEval{style/calls}{\the\AuditStyleCount}
```

Both recorded five callback evaluations. The default probe input is retained here in full:

```tex
\ExplSyntaxOn
\semio_viz_guide_keys_reset:
\SemioVizProbeEval{axis/defaults}{\l_semio_viz_axis_tick_size_fp,\l_semio_viz_axis_tick_size_outer_fp,\l_semio_viz_axis_tick_padding_fp,\l_semio_viz_axis_title_gap_fp}
\tl_use:N\c_semio_viz_theme_font_label_tl
\SemioVizProbeEval{axis/label-size}{\use:c{f@size}}
\tl_use:N\c_semio_viz_theme_font_title_tl
\SemioVizProbeEval{axis/title-size}{\use:c{f@size}}
\ExplSyntaxOff
```

The subsequent strict replay including the baseline extension passed both canonical and native grammar checks and exported 249 symbols; Nx succeeded in 2m04s. Context-default additions require a final replay after their fix.

## Final Context and Native Boolean Literal Closure

The schema context defaults (`scale="x"`, axis `orient="bottom"`) are now consumed by the owned facade. The same selected context replay passed **62/62** after its two preceding failures. No y-channel-derived historical orientation remains.

Current axis neutral inventory totals **30 cases**, and the direct baseline inventory totals **13 cases**. Native option values `"true"`/`"false"` are valid primitives consumed by native boolean keys. Two neutral cases demonstrated that the preceding numerical renderer ignored minor/grid/domain true literals and treated label false as truthy: **64 selected axis checks, 62 passes and 2 failures**. Root's independent registered print replay observed the same two failures: **536 checks, 534 passes, 2 failures, zero errors**.

A private first-party `guideBoolean` now accepts typed booleans and the native true/false literals for the declared controls and rejects unrelated primitives. It is an option value resolver, with no generic TikZ parser, compatibility wrapper or external runtime dependency. A numeric label boolean (`labels:1`) was exercised through canonical async inference and returned incomplete with no published lines/text.

Actual native compiler proof after the concurrent font staging fix recorded grid endpoints **[10,10,10,13,90,10,90,13]**, minor position **[50,10]**, and native minor/labels/domain/grid flags **[1,0,0,1]**. Canonical async geometry is the same under the owned downward viewport convention: **[10,70,10,67,90,70,90,67]**, with minor tick **[50,70,50,70.7]**. The selected canonical replay then passed **64/64**.

The first diagnostic used an incorrect grid probe key and read boolean registers after their picture group had ended. The observed geometry was already correct; the retained probe was corrected to `geometry/grid-line`, read flags inside the picture, and used 1e-8 rounding for native decimal arithmetic. Those diagnostic projection failures are not implementation failures.

The retained native boolean input is an article document with package `semio-viz-guide`, geometry probes enabled, frame width100/height80/pad10, scale linear domain0,10 range10,90 and this axis command:

```tex
\SemioVizAxis[scale=x,orient=bottom,tickValues={0,10},minor=true,minorTicks=2,labels=false,domainLine=false,grid=true,gridLength=3,tickSize=0]
```

The numerical and actual native boolean proofs are complete. Terminal registered exhaustive and strict replays are running after this final scoped change. The native owner's separate actual angular-title red stage confirmed all three previous anchors at (-7,0), versus neutral expected (67,40), (50,57), (33,40); the numerical arc position is retained, while native correction belongs to that owner.

## Terminal Owner Verification

After every scoped axis/control/default/baseline/boolean change, registered Nx exhaustive inference passed **564/564**, with **138 render**, **114 scale**, and **16 mutation** checks, zero mismatches and zero errors. Nx exited successfully in **10.3 seconds**. The terminal strict build failed in 1m38s; exact diagnostics are being repaired. The preceding strict replay passed249 exports, but it is not substituted for the current terminal gate. Both gates used the validated ticket-local graph, disabled daemon/plugin isolation and executed the existing package `📜️script.ts` targets.

The native boolean/compiler proofs and schema default/font proofs are actual runtime evidence, not inferred from source. The same terminal inference suite includes the previously verified independent timer cancellation and actual browser worker checks. Remaining native legend, angular-title completion, catalogue QA and global ticket closure belong to their coordinating owners; this owner introduces no claim about unfinished parallel work.

The source inventory above, plus the retained async/unknown/nice reports, constitutes the complete ownership handoff. No additional controls audit or production edits are planned in this lane.


Terminal strict diagnostic is a newly concurrent transitive catalogue test: `print/🧪️tests/🖼️family-containment/🟦️.ts(46,29) TS18048 entry.labels possibly undefined`. The canonical source check passed. The catalogue owner is fixing its narrowing; the strict replay will confirm its source through the registered native grammar closure. The earlier draft terminal pass text was corrected immediately after the tool returned its nonzero status; no failed gate is substituted by the prior successful one.

## Final Strict Repair and Complete Handoff Inventory

The family test now explicitly checks `entry.labels !== undefined` after its property-presence guard. The final registered strict replay **succeeded**, with **249 own barrel symbols**, checking both canonical inference/worker and the separately registered native grammar runner plus its transitive compiler/publisher/family sources. Nx exited0 in **59.6 seconds** (58.6s task execution), no cache hit. The preceding failed replay remains recorded; Nx's flaky-task notice reflects the source change between the failed and repaired checks. No compiler success is inferred from a duration line.

Current registered commands are:

```text
bun x nx run @semio-tech/print-viz-inference:test-exhaustive
bun x nx run @semio-tech/print-viz-inference:test --args='quick render'
bun x nx run @semio-tech/print-viz-inference:build
```

The canonical runtime API is `await inferVizChart(request, {signal,onProgress})` in the owned inference entry. The registered exhaustive command exercises all16 mutation/control checks, including an independently scheduled Bun timer and actual Chromium worker cancellation. The browser worker constructor remains the literal bundler-discoverable `new Worker(new URL("./🧵️worker/🟦️.ts", import.meta.url), {type:"module"})`; the system Node path uses owned worker-thread interfaces. The separately recorded actual Node24.14.1 run consumes bundled `main.mjs` and bundled `🧵️worker/🟦️.ts`, not unbundled JSON-importing TypeScript. Its successful placement/cancellation projection is retained in the async report.

Complete owner source/test inventory across this execution lane follows (all relative to `🧰️framework/🛍️products/📓️print/`):

- `README.md`
- `🧬️schema/🔣️.json`: band/point unknown rejection; root subsequently owns closed IR/default metadata changes.
- `🧬️schema/📸️snapshot/📊️chart/🟦️.ts`: schema-owned axis default facade.
- `🧬️schema/💡️inferences/🟦️.ts`: canonical async inference and owned public result/control types.
- `🧬️schema/💡️inferences/🧵️worker/🟦️.ts`: worker-owned planning/progress/admission.
- `🧬️schema/💡️inferences/📐scale/🟦️.ts`: D3 unknown/missing semantics and authored nice-method controls.
- `🧬️schema/💡️inferences/🖼️render/🟦️.ts`: missing positions/paint, common guide context and axis controls/defaults/baselines/style. Root physical-unit and native legend changes retain their separate attribution.
- `🧬️schema/💡️inferences/🎨theme/🟦️.ts`: pure paint import and first-party font stack resolution.
- `🧬️schema/💡️inferences/📦️packages/🟦️typescript/📜️script.ts`: worker build entry and separately registered native grammar strict closure.
- `🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts`: runtime/differential registrations; concurrent root contract checks preserved.
- `🔨️modules/🎨print-design-token-paints/🟦️.ts`
- `🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts`
- `🧪️tests/🧬️chart-mutations/🟦️.ts`
- `🧪️tests/🧬️chart-mutations/🥒️.feature`
- `🧪️tests/🎨️authored-color-scales/❔️unknown.json`
- `🧪️tests/🎨️authored-color-scales/❔️render.json`
- `🧪️tests/🎨️authored-color-scales/✨️nice.json`
- `🧪️tests/🎨️authored-color-scales/🟦️.ts`
- `🧪️tests/🎨️authored-color-scales/🥒️.feature`
- `🧪️tests/🎬️render-scene/🔣️customization.json`
- `🧪️tests/🎬️render-scene/🔣️axis.json`
- `🧪️tests/🎬️render-scene/🔣️baseline.json`
- `🧪️tests/🎬️render-scene/🥒️.feature`
- `🧪️tests/🖼️family-containment/🟦️.ts`: shared one-line labels narrowing only, coordinated with catalogue owner.

Retained owner audits are `📓️async-inference-audit-2026-10-04.md`, `📓️guide-control-parity-audit-2026-10-04.md`, `📓️unknown-scale-parity-audit-2026-10-04.md`, `📓️derived-scale-and-guide-controls-audit-2026-10-04.md` and this audit. Generated compiler/debug outputs remain under the ticket generated folder for coordinator cleanup. Meaningful neutral input files remain permanent in the existing test areas; native probe inputs are reproduced above in this retained audit. No modifying git command, worktree, AGENTS edit, external runtime dependency, standalone permanent script or compatibility API was added.
