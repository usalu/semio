# Print, Docs, Tokens Handoff (audit2-print-docs-tokens.md, all lanes except Python)

Paths: `FW` = `🧰️framework`, `UI` = `FW/🔨️modules/🖱️ui`, `PRINT` = `FW/🛍️products/📓️print`.

## Rows verified, then fixed

| Row | Verdict | Change |
| --- | --- | --- |
| 1 print chip opacity 0.82 | true | `PRINT/🖋️latex/semio-window.sty`: the chip is two layers from tokens, the glass of the level above its frame at `\semio@glass@alpha@<level>` then the foreground veil at `\semio@chip@tint`; `\SemioChipTransparency{false}` flattens the same two layers onto the canvas (xcolor mix). Headings and page bars sit on the page (window glass, alpha 0.88 + veil = 0.892); `\semio@chip@muted` chips of windows, figures and tables sit in a window-level frame (pane glass, 0.76 + veil = 0.784, content share 21.6 percent). No hand-typed value is left. `semio-core.sty` gained the `semio-chrome-pane` alias. |
| 2 CSS hand copy | true | `UI/🎨️styling/📽️projection/🟦️.ts` `emitLevelKnobs` projects all nine `levels` knobs (`--level-shade-step`, `--element-shade-step`, `--hover-shade-step`, `--glass-alpha-step`, `--glass-blur-step`, `--glass-saturate`, `--veil-alpha`, `--veil-blur`, `--chip-tint`) as one `:root` block into `🎨️palette/🎨️.css` and `🤖️generated/🌓️palette-theme/🎨️.css` (only those two regenerated, `print-tokens-emit.mts`; `checkStylingArtifacts()` reports fresh). Hand copies removed from `🖌️ui/🎨️.css`. `ShellScope` test reads the knobs from the palette. |
| 3 theme setter | true | `🌓️theme/🟦️.ts` sets `--chip-tint` from `metrics.chrome.chipTintPercent` (the chrome metrics are an open number record, so no schema entry exists to extend). Test added. |
| 4 LaTeX tokens | true | Generator `PRINT/🔨️modules/🎨print-design-token-paints/🧮️resolution/🟦️.ts`: `chipTintPercent` in the type, `renderChipGlass` emits per-level `\semio@glass@alpha@<level>(@pct)`, `\semio@chip@tint(@pct,@rest@pct)`; `DEFAULT_LEVELS` deleted, a missing `levels` throws; header path fixed; dead `glassPanelAlpha`/`glassPanelBlurPx` removed from the type. `semio-tokens.sty` regenerated. |
| 5 Python tokens | false | Already emitted `STYLING_LEVELS`; Python lane owns it, untouched. |
| 6 C# | true, no change | `🎨️palette/🔷️.cs` is palette only: no C# renderer in the repo draws chrome (the only C# consumer is the package itself). Recorded in `🛂️adapters.manifest.json`. |
| 7, 8 manim | not mine | |
| 9, 10, 19 pets story CSS | partly stale | The radius was already fixed by the products agent. `.card .tab` and `.card .note` now share one chip rule: glass of the next level above `--panel` from `--level-shade-step`, `--glass-alpha-step`, `--glass-blur-step`, `--glass-saturate`, veil from `--chip-tint`; the file imports the palette stylesheet for the knobs. `.note` moved inside the card (was `bottom: 100%`, dashed tab above). `.prop` radius 6px to 0. Not browser-checked. |
| 11 junctions and snap | rejected | `createSharedBorderJunctionResolver` and `createSharedBorderSnap` are used by `WindowChrome/shared-border.ts:500,635`. Kept; `design.md` now says so (removed the "full-span snapping" and "junction cases" from the removed list). |
| 12 two `shared-border.ts` | rejected | Different roles (primitives in `WindowFrame`, compositor in `WindowChrome`) and a rename would touch live files of other lanes; no collision exists today. |
| 13, 14 React prose | true | `chrome-safe-area` comment no longer names a gap; "cap row" is now "chip row" in doc comments of `WindowChrome` and `ContextMenu` (slot names unchanged). |
| 15 dead tokens | true | Removed `opacities.glassPanelAlpha/MenuAlpha/WindowOptionsAlpha` and `metrics.chrome.glassPanelBlurPx/WindowOptionsBlurPx` from `🌓️theme/🔣️.json` (no consumer anywhere outside tickets). |
| 16 Rust comment | true | Reworded in `🎭️theme/🧩️component/🦀️.rs` (comment only). |
| 17, 18 | no action | |
| Docs | | `PRINT/README.md` English and German describe the token-derived glass and the two levels; no other doc described outside chips. |

## Tests

- Page chrome (`PRINT/🧪️tests/🏷️page-chrome/`: `🟦️.ts`, `🔣️.json`, `🥒️.feature`): glass layers, level, flatten, no hand-typed value, and `verifyChipGlassTokens` (mathjs arithmetic against `levels` for every level, content share 5 to 50 percent).
- Token paints (`🔨️modules/🎨print-design-token-paints/🧪️tests/🟦️.ts`): generated sheet equals the file, mathjs alpha and tint, rejects missing levels.
- Quick pipeline source assertions: new chip fill assertion (`⚡️quick-pipeline-verification/🟦️.ts`).
- UI: `styling-resolve` (all nine knobs from JSON, none hand-declared), `theme-resolve` (chip tint override), `ShellScope` (knobs read from the palette).
- Run (vitest through `lockf -k /private/tmp/claude-501/semio-vitest.lock`, `print-tokens-run.sh`, configs `print-tokens-vitest.config.ts`, `bands-dialect-tint-vitest.config.ts`): page chrome, logo, frame junctions, macro staging, token paints, assignment, styling-resolve, TUI emphasis, theme-resolve: all pass (print 62, styling 22, theme 35, ShellScope 10).
- Not run: the full `verifyPrintPipelineSourcesV1`; it stops in the `repo` project-inference bootstrap (`inferProjectInputs` export), unrelated to these files. Its new assertion was checked against the file directly.

## Portable fixture build

`print-build.ts` (this folder): `node print-build.ts <dir> <main.tex> <name> [pages] [scale] [light|dark]`; picks the pinned tectonic by platform triple under `.cache/b/t/m/print`, the cached bundle and fonts, stages with pictographs stripped, rasterises with the PDFKit swift script on macOS and `pdftoppm` elsewhere. Fixtures: `print2-fixtures/photo.tex` (chips over a photo, a figure and a table; the photo is passed through `SEMIO_PRINT_EXTRA`, not copied), `print2-fixtures/assignment.tex`, the LaTeX ticket's `junctions.tex`, the report template in light and dark. Compiled with zero errors (tectonic 0.16.9, macOS arm64). Windows and Linux were not run.

Looked at and saved (<300 KB) in `evidence/print2/`: `junctions-p1.png`, `assignment-p1.png` (matrix chips in the header row, the column divider shows through the title chip), `report-light-p5-chips-over-strip.png`, `report-dark-p5-chips-over-strip.png` (chips over saturated red, green and blue: the colour shows through in both appearances, labels legible), `photo-chip-crop.png` and `photo-p1.jpg` (beam and sky visible under the figure chip). Edge laws unchanged: junction page still shares every edge.

## Open

- Print has no backdrop blur; the blur steps stay on screen only.
- Windows and Linux runs of `print-build.ts`, the pets story in a browser, and a full `repo test` are not done.
- `🗑️generated/` of this ticket was deleted by someone else during the run; outputs were kept in `/private/tmp/claude-501/semio-print2`.
