# Staffage preview and card interlock follow-up

The live overview reported style writes on archive list cells and section wrappers, plus repeated changes to thumbnail `data-time` and `aria-label`. A live archive pane remains mounted under the layered overview, so its rendering work also affects the overview.

- Placed figures canonicalize looping drawing queries to nanosecond precision. Frame 1 of a one-second clip at 12 fps formerly produced different floating-point phases 50 cycles apart. The regression reproduces the separate drawing objects and verifies that the cache now reuses the same scene. Whole-cycle travel continues to use the original query.
- Inert row previews use the shared visible-figure frame driver. Clock ticks paint the fitted SVG directly from the same evaluator and depiction options. The card names the asset, and a conventional-symbol drawing retains its caption. Drawing failures reach the existing preview error boundary. The regression parses the expected frame with a DOM template and compares the full SVG while checking that the React commit count stays fixed.
- CardStack measures its settled geometry relative to the stack. Rechecking a translated stack previously reproduced eight needless style mutations. The same check now writes zero styles. Chip widths and heights contribute to the signature. Card discovery stays cached across geometry changes, and observers skip SVG artwork before inspecting changed markup. Adding a colliding footer chip without growing the row now changes the lift from 30 px to 2 px.

CardStack, LayeredOverview, WindowChrome and border-presentation passed 237 tests across 13 files.

## Cold drawings and the ambient clock

A Bun microbenchmark draws 24 clock instants each of the line and solid elevation of 39 authored assets, covering the featured frieze and category previews. The cold end-to-end p95 was 136.25 ms over 1,872 samples and the slowest frame took 1,432.76 ms on date palm. Replaying identical draws returned the cached scene and SVG in about 0.02 ms or less. Phase caching alone could not keep the main thread responsive while the artwork first loaded.

The drawing evaluator now prepares the elevation for the paper bounds and computes front and plan projections and kinematics only when a consumer requests them. The regular evaluation entry point still prepares every projection and kinematics immediately. The same benchmark then measured 28.03 ms cold p95. Date palm's line elevation dropped to 231.64 ms p95, and jogger's to 5.83 ms. Individual cold scenes remained too expensive to run on the main thread.

The ambient clock now queues eligible figures for a separate animation callback with a cooperative 5 ms dispatch budget. One module worker evaluates, depicts and serializes figures. It receives each asset once for the lifetime of its registered figures and releases it when the last figure leaves. Only the latest pending request for each figure is retained and in-progress results are drawn before the queue advances. This lets expensive figures progress while input and border work remain on the main thread. Leaving the viewport or pausing cancels queued work, and releasing a figure prevents a late reply from painting. Interactive clocks continue to step synchronously.

The full staffage run before the worker changes passed 281 tests. Two catalog tests failed outside the drawing changes. One expected all assets to be valid and the other detected authored negative zero becoming positive zero in compact JSON. The first run also encountered host disk exhaustion during Vite transforms and was rerun after space was reclaimed.

After integrating the worker, all 65 clock, viewpoint, and archive-grid tests passed. The evaluator's full-state, representation and solid geometry suites added 40 passing tests. Release-mode Vite emitted a 234.34 kB ES module worker, and both the source worker and the emitted worker returned the identical 18,121-character SVG and geometry to direct drawing in the Bun Worker smoke test. The browser-only Worker bundle uses the rendering modules directly to exclude the Node catalog and cast entry points.

Reproducible ticket inputs are `staffage-drawing-bench.ts`, `staffage-worker-build.ts`, `staffage-worker-entry.ts` and `staffage-worker-smoke.ts`.

The final full staffage run passed 284 of 286 tests. The two failures were in catalog loading and catalog payload comparison. The first reported authored volume-validation issues such as copied thermal solids at multiple scales. The second found a difference between the compact asset content served through Vite and the freshly read authored payload. All clock, viewpoint, grid and drawing tests passed in this run.

## Closed asset editors

Warm asset panes behind the glass mounted the full viewer and started a private playback clock. Each clock updated the transport and timeline. The viewer also published a frame every 400 ms, causing the asset page to render again and React to rewrite form attributes. An overview trace recorded 930 aria-valuetext or value updates and thousands of input name and type writes in 15 seconds.

The closed asset pane now displays its title, summary, facts and a fitted drawing of the presentation pose. Opening the matching asset address mounts the viewer, transport, timeline and export controls. Closing the asset disposes the viewer clock. Thumbnails in the archive cards continue to use the ambient drawing worker.

The new language-agnostic regression reproduced an editor mounted behind the glass. It now verifies the closed preview, working frame stepping in the opened transport, and clock disposal on close. All 44 tests passed across the detail, archive grid, 3D viewer, playground and viewpoint suites.

The remaining closed pane audit found that category friezes and row thumbnails were already painted through the shared ambient worker, with no React clock subscription per preview. The closed comparison nonetheless mounted a paused transport and computed a shared live comparison frame. It now displays its two still drawings through the same preview worker and mounts the transport, inspector and editable controls only when open. The playground already gates its editor and private clock on the open state. Closed browse and category panes now mount their filters only when they can accept input.

A closed nested category had bypassed the staggered overview card loader. It now starts its own request when revealed or opened. The hero now keeps its featured category requests alive as each sibling category finishes. The previous effect cancelled the remaining requests on every completed category. Each issue reproduced in a language-agnostic regression before the fix. The seven targeted suites passed all 61 tests, and the React package type check passed.

The final full React run passed 289 of 290 tests across 30 files. Its sole failure was the authored archive check at its 120-second test limit. The UI lifecycle and catalog payload regressions passed.
