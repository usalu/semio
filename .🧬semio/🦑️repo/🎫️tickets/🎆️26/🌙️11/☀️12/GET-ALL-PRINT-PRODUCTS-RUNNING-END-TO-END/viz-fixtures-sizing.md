# 🧮️ Sizing of `test-viz-fixtures` (read-only investigation)

Root: `/Users/Shared/semio/🧰️framework/🛍️products/📓️print`

## Dispatch
- Nx target: `📦️packages/🟦️typescript/📋️project.json:388-410` runs `repo ./📜️script.ts test viz fixtures` with `cache: true`, `outputs: []`.
- Command: `🎮️commands/🧪️print-pipeline-verification/🟦️.ts:40-43` calls `writePrintGalleryEvidence(execution, segments.slice(2))`.
- Core: `🔨️modules/📊️visualization-gallery/🧪️testing/📏️measurement/🟦️.ts`
  - `printGalleryMatrix` (line 22): sections under `🧾️template/📊️viz-gallery/` × `VIZ_GALLERY_THEMES` [light, dark] × `VIZ_LANGUAGES` [en, de].
  - `measurePrintGalleryVariant` (line 51): one `withCompiledPrintProbeV1` per variant, which runs one tectonic compile (`🔨️modules/🖨️tectonic-template-compilation/🟦️.ts:59`).

## Answers
1. **Formula:** compilations = sections × 2 themes × 2 languages. Sections = 80 `viz-N.tex` files in `🧾️template/📊️viz-gallery` (`visualizationTemplates`, line 48, excludes `-dark`, and `viz-api` is not in the matrix). **320 compilations per full run.** At the observed ~16 s each that is about 85 min, which fits the 100-minute run. The committed fixture `🧪️tests/🧫️fixtures/🖼️gallery-render/🖼️gallery-render.json` has `variants: {}`, so it contributes 0.
2. **Sequential by design.** Line 53 is `for (const [index, variant] of matrix.entries()) { … await measurePrintGalleryVariant(…) }`. There is no concurrency setting or pool in this path. The only parallel-capacity mechanism is in the Nx/capacity layer (the log shows "grown from 4 to 5 of 8 job(s)"), but this command admits one job.
3. **No resume.** Every matrix row is re-measured unconditionally. With a section filter, line 53 seeds from the committed oracle, but the selected rows still recompile. The file is written only at the end (`gallery-evidence.json`, `flag: "wx"`), so a crash or cancel loses all progress. Nx `cache: true` with empty outputs caches only successful runs.
4. **test-viz-full does not fan out to 86 Nx tasks, as far as I can see.** `project.json:305-320` runs `test viz full` as one script. `build-viz` (lines 244-258) runs `complete visualizations` (`🔨️modules/🖨️tectonic-template-compilation/📜️script.ts:10`), a sequential `for … await` over 81 catalog documents. `verifyPrintVisualizationBuild` (`🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts:17-19`) is also a sequential loop over 81 templates. I could not find the number 86 anywhere in these files. The catalog has 81 visualization documents (`🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json`, 81 `"visualizations"` entries). That is the same count as `visualizationTemplates()` (80 gallery + 1 api), and it does not include the compile-per-variant matrix.
5. **The partial log does not support the premise.** `r7-viz.log` (1655 lines) contains zero `gallery` lines and zero per-compilation `tectonic` lines. It ends at ~6131 s with a SIGINT while still in dependency/toolchain stages (`print:toolchain:prepare`, `deps-tectonic`, `deps-tex`, "still working … source.served"). The gallery loop never started, so the estimate is **0 compilations completed**. The 100-minute run appears to have been spent in prerequisites, not rendering. This should be checked on the machine before sizing decisions.

## Consequences for the ticket
- Even when it starts, a full `test viz fixtures` run is 320 serial compiles (~85 min) with no resume.
- Possible speed-up: bounded parallel compiles inside `writePrintGalleryEvidence`, plus per-variant evidence files so reruns can skip completed rows.
