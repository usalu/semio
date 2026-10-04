# Numerical Legend and Font Metrics Parity

The canonical renderer now resolves native legend placement, categorical columns, horizontal advances, categorical versus continuous entry selection, 48 gradient slices, area and radius size legends, line and symbol channels, hatch counts, label gaps, item gaps, explicit spacing, lengths, formatted labels, authored sizes, title origins and opaque TikZ styles. The implementation remains in the existing rendering inference owner. Schema metadata supplies native defaults. Its plain-text measurement dependency is a pure JSON-backed leaf of the existing font catalog owner; browser inference does not import font publication or filesystem staging.

## Current Coordinate, Font, and Line-Box Proof

The early 67-record numerical comparison matched native upward-y coordinates directly and therefore missed a portable downward-y visual error. Actual paired PDF inspection exposed it. The native origin is now projected through `height - y`, including rectangle extent and independent D3 symbol origins. The registered projection red baseline had 23 failures among 160 render checks. The corrected registered exhaustive replay passed 619/619 in 45.2 seconds; the strict replay passed 250 symbols in about 1 minute 34 seconds. These checks precede the final title-line-box change and must be followed by the combined current gate.

PDF.js found two actual authored title-font failures before the native style correction: native Legende/Key selected Anta while their role requires Share Tech Mono. The corrected focused actual compiler replay terminated 0 and verified 4 title and 6 label text faces, all 67 native geometry records, and 3 canonical emitted plans.

Poppler then exposed overlap when a 16-point title reserved only the 2.6 mm swatch height. Cached actual native probes reproduced two title-origin failures, and the independent canonical D3/Canvas oracle reproduced three failures (title, opaque style, large vertical labels). Native and canonical owners now reserve the larger of swatch height and the authored font em line box, converted from TeX points to millimetres, plus the authored gap. This is an explicit line-box reservation; it does not claim glyph-contour or arbitrary-TeX measurement. Default geometry is unchanged where the swatch is already taller.

The current focused native compiler, session 19918, terminated 0: 71 geometry records across 23 neutral controls, 3 canonical font-selected plans, 4 title and 8 label PDF font faces. The independent direct runtime replay passed all 23 legend controls after the three red cases. Actual output is `🗑️generated/legend-native-after/🧪️probe-out/legends.pdf`; terminal log is `🗑️generated/legend-line-box-native-green.log`. Poppler page 8, `🗑️generated/legend-visual-qa/legend-line-box-after-08.png`, confirms title/swatch separation and large vertical-label row separation. Empty trailing pages are excluded from visual evidence.

Native guide production is stable after this proof. The final registered full native gate remains pending the shared finance, grammar, evaluation, solar, and surface family changes. Prior full runs are superseded and are not current closure claims. The separate paired-axis visual finding is retained in `📓️paired-axis-projection-audit-2026-10-04.md`; root owns its correction.
## Earlier Development Evidence

The registered no-cache quick baseline executed 591 checks: 569 passed, all 22 new legend cases failed, and no check errored. The 33 registered font checks passed. The first exhaustive implementation replay executed 619 checks with 612 passed, 2 mismatches and 5 errors; these identified canonical color serialization, token role resolution and an obsolete prior legend position expectation. The corrected registered exhaustive replay passed all 619 checks in 10.6 seconds; a strengthened replay including independent D3 symbol command comparisons passed all 619 in 11.3 seconds. The final font-integrated exhaustive replay passed all 619 checks in 31.7 seconds during concurrent compiler runs. It compares actual Anta labels and Share Tech Mono titles and verifies native selector emission. The registered strict build also passed both canonical/browser and native-runner type checks, reporting 250 symbols in 1 minute 4 seconds.

The registered full native retry 10 passed Rust 11/11, the original mark/scale/typed-gap/network/minor gates, 63 directed graph/title checks and 30 actual font widths, and compiled the 22-control native legend document. It then stopped on result-schema font admission while the parent's schema integration was pending. This 8 minute 28 second run is retained as a truthful failed full gate; it is not counted as a full pass. A current focused canonical font-emission proof and final full replay remain in progress.

The focused actual native legend compilation passed 67 geometry records across all 22 neutral controls. Native categorical origins, columns, horizontal spacing at 14 points, side offsets, explicit placement, domain entry selection, continuous tick placement, gradient bounds, size radii and title displacement matched canonical inference. Its PDF and JSONL are under `🗑️generated/legend-native-after/🧪️probe-out`. This is actual TeX execution, separate from TypeScript source generation.

The native font baseline measured 15 texts at 6.6 and 14 TeX points. All 15 default-size widths passed; all 15 authored 14-point widths failed because native legend measurement always used the default label size. After correcting the existing guide hbox to use authored label size, all 30 actual native widths matched the pure metrics and independent Canvas advances within 0.0001 mm. The native helper terminated successfully. The native measurements include kerning, ligatures, precomposed and combining German accents, zero-advance marks and source-bound Latin substitutions. Diagnostic outputs remain under `🗑️generated/font-native-before`, `font-native-after` and `font-metrics-canvas.jsonl` until ticket cleanup.

## Font Source and Runtime Boundary

The tracked Anta font SHA-256 is `9f3b9723a50c1244d49906b1892b3aad43876ab9a2de80288db47821b0c7b298`; its units per em are 2048. Build-time derivation reads cmap format 4, hmtx advances, GDEF mark classes, Latin ccmp multiple substitutions, liga ligatures and four GPOS kerning lookup tables. The pure runtime leaf shapes the supported en/de plain text from those first-party source-bound records. Arbitrary authored TeX is opaque and is not claimed to have a portable shaping oracle. Staging verifies the tracked digest and requires regenerating the metrics if the font changes.

The permanent derivation is `bun nx run @semio-tech/print:generate-font-metrics`; its launch registration is maintained by the parent agent beside the existing print build commands. The combined inference probes now execute the 30 Canvas width comparisons, metrics schema admission, source digest and ligature checks. Native compiler probes are part of the registered full grammar target. Font selectors read the existing pure catalog: SemioSans is Anta, SemioMono is Share Tech Mono and SemioEmoji is Noto Emoji. Resolved legend text forwards the family into the scene and emits tracked native selectors in TikZ. Arbitrary explicit family names use fontspec. The parent has admitted the optional nonempty text font string in plan and scene schemas and added Canvas consumption in the existing 2D owner.

## Exact Source Inventory

All product-relative paths below are rooted at `🧰️framework/🛍️products/📓️print/`.

- `🔨️modules/🔤print-font-catalog/📏️metrics/🧬️schema/🔣️.json`
- `🔨️modules/🔤print-font-catalog/📏️metrics/🔣️.json`
- `🔨️modules/🔤print-font-catalog/📏️metrics/🟦️.ts`
- `🔨️modules/🔤print-font-catalog/📏️metrics/🧪️tests/🔣️.json`
- `🔨️modules/🔤print-font-catalog/📏️metrics/🧪️tests/🟦️.ts`
- `🔨️modules/🔤print-font-catalog/📜️script.ts`
- `🔨️modules/🔤print-font-catalog/🟦️.ts`
- `📦️packages/🟦️typescript/📋️project.json`
- `📦️packages/🟦️typescript/package.json`
- `🖋️latex/semio-viz-guide.sty`
- `🧬️schema/💡️inferences/🖼️render/🟦️.ts`
- `🧬️schema/💡️inferences/📦️packages/🟦️typescript/🔬️probes/🟦️.ts`
- `🧪️tests/🎬️render-scene/🔣️legend.json`
- `🧪️tests/🎬️render-scene/🧭️legend/🟦️.ts`
- `🧪️tests/🎬️render-scene/🔣️customization.json`
- `🧪️tests/🧬️native-chart-grammar/🟦️.ts`

The parent owns the schema defaults/facade export and launch seed persistence. These entries are related integration work rather than this agent's isolated edits. Earlier native scale, layout, graph, mutation and external registration inventory is retained in `📓️native-final-verification-2026-10-04.md` and `📓️graph-arrow-and-angular-title-2026-10-04.md`.

Current registered strict after the private SunCalc oracle boundary repair: session 45280 terminated 0, both canonical/browser and native-runner checks passed, 250 exported symbols, 1 minute 10 seconds. Log: `🗑️generated/legend-finance-current-strict-after.log`. This includes the latest finance test owner and title/label line-box implementation.

Final canonical replay after the axis role/default corrections terminated 0: exhaustive 630 / 630 checks (171 render checks) in 31.7 seconds, and strict canonical/browser plus native runner/helpers with 250 exports in 1 minute 26 seconds. These supersede the earlier successful 1 minute 10 second strict gate as current combined TypeScript closure. Logs are `🗑️generated/canonical-current-final-exhaustive.log` and `🗑️generated/axis-role-current-strict.log`. The full current native/Rust compiler gate remains held for the concurrent final shared-family freeze; focused native results above remain valid evidence for their recorded source versions.

Latest registered canonical closure after the final legend foreground and shared text-opacity correction: strict session 49810 terminated 0 in 1 minute 37 seconds, checking canonical browser/worker inference and native runner/helpers with 250 exports. Exhaustive session 52999 terminated 0 with 633 / 633 checks in 39.4 seconds, task cache explicitly skipped. Logs: `🗑️generated/canonical-final13-strict.log` and `🗑️generated/canonical-final13-exhaustive.log`. These supersede the prior 630-check/1 minute 26 second gates as current TypeScript proof. The actual focused axis opacity compiler and independent PDF label oracle also terminated 0 after the correction; all 30 paired label colors/opacities passed.

## Final Actual Legend Text Paint Proof

Focused paired compiler session 2901 terminated 0. PDF.js graphics-state/showText inspection passed 18 actual label/title records across native and canonical default light, default dark and explicit `fill=#123456` override cases. Every Palette title used the actual ShareTechMono-Regular face; bb/ccc labels used Anta-Regular. Light foreground is `#001117`, dark foreground `#f7f3e3`, and authored override `#123456`; all actual values matched the retained neutral legend fixture independently of render-plan style objects. Log: `🗑️generated/legend-text-paint-after.log`. PDFs are in `🗑️generated/legend-text-paint/{light,dark}/🧪️probe-out/`.

Poppler dark page 1 was viewed and confirms native/canonical title and label foreground and override paint agree. Image: `🗑️generated/legend-visual-qa/legend-foreground-dark-1.png`. The blank page 2 is excluded from visual evidence. This gate certifies text paint and font roles, not direct TeX versus CSS named-palette interpretation: the direct TeX fixture's named xcolor green and canonical CSS green have different native color vocabularies. Canonical Rust native emission already translates CSS paints to explicit aliases.

New authored proof inputs are retained outside generated cleanup:

- `🔣️axis-style-proof-2026-10-04.json`
- `📥️authored-inputs/axis-style-proof/📜️script.ts`
- `📥️authored-inputs/axis-text-paint/📜️script.ts`
- `📥️authored-inputs/legend-text-paint/📜️script.ts`

The retained compiler helpers explicitly write into `🗑️generated`; their read-side PDF oracle paths also resolve there. The retained axis label oracle was executed again from its retained location and terminated 0. No new permanent product runtime dependency or executable route was introduced.
