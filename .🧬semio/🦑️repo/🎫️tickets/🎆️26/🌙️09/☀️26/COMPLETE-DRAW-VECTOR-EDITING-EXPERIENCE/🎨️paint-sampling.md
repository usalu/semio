# Prepared Paint and Export Verification

The preceding goal turn made concrete progress: schema-first neutral paint samples, Rust/TypeScript prepared gradient ramps, appearance-preserving stop insertion and a passing TypeScript run. This continuation adds real SVG export normalization, executes the previously skipped native PDF raster comparison and repairs Draw callers after current framework API changes. The full editor goal and ticket remain active.

## Paint Contract

`🧬️schema/🎨️fill/🎨️sampling` owns seventeen neutral cases covering solids, linear projection, radial distance, stable unsorted stops, coincident stop transitions, independently interpolated color/alpha, empty and single-stop paint, degenerate gradients and invalid authored values. Preparation owns its stop colors, sorts once and performs logarithmic lookups. The shared ramp now computes inserted gradient stop colors in both implementations. The existing editable-stop limit stays enforced by the editor. Prepared paint accepts immutable scene paint inputs and owns mutable copies privately.

The registered TypeScript fill tests compare mathematical fixture samples with independent Sharp/librsvg SVG pixels and verify twenty-seven before/after stop insertion images. The original one-pixel oracle used subpixel geometry too small for the reference renderer and could render transparent rectangles; it was corrected to full-pixel geometry and explicitly asserts nontransparent reference images. Actual comparisons use premultiplied RGB and alpha within two byte values.

[SVG 1.1 paint servers](https://www.w3.org/TR/SVG11/pservers.html) requires a zero-length linear gradient and zero-radius radial gradient to paint with the last stop. The installed librsvg backend instead produced an averaged linear color and an empty radial result. The sampler follows the specification; the independent test reference explicitly normalizes those two cases to a solid paint. Raw librsvg parity is therefore not claimed for those degenerate cases.

## SVG Production Change

Both authoritative exporters now validate fill paint through `PreparedFill` and emit constant paint directly as solid RGB/alpha. This covers empty and single-stop gradients plus zero-length/radius gradients, avoiding inconsistent consumers. Invalid offsets, colors and negative radii are rejected. Ordinary gradients keep their editable gradient vocabulary. Rust and TypeScript tests consume the same neutral paint cases; TypeScript tests parse exported XML with xmldom and inspect Sharp raster pixels.

RED `tests-svg-constant-paint-red.txt` reproduced unnecessary gradient definitions for single-stop paint. GREEN `tests-svg-constant-paint-green.txt` passed 284 TypeScript tests, 154315 assertions, zero skipped, with the PDF oracle enabled. Native execution of the newly authored SVG regression passed in the complete 479-test native run.

## Native and PDF Evidence

Old native handle 17529 is terminal: 436/447 passed, eleven failed. Its log is `tests-native-isolation-pdf.txt`; failures included stale command/catalog expectations, group-isolation example decoding and blend candidate/test assumptions. Several of those source corrections were already present in the shared checkout when this continuation inspected it; do not attribute them to this turn. Old component handle 93519 is terminal and stopped before completion. Neither old handle is treated as live.

The first new native attempt failed on an obsolete reused Nx graph. A fresh ticket-local graph fixed discovery. Run 31130 then terminated before tests with fifty compiler errors from removed protocol warning APIs, private former locale exports and one default view constructor. Updated twenty-two Draw callers to canonical warning/locale APIs and the test view to an explicit locale/terminology constructor. Fresh native handle 71310 (`tests-native-current-api.txt`) compiled successfully and ran 479 tests: 477 passed and two fixture assertions failed. The SVG IO fixture incorrectly supplied an empty gradient while expecting a gradient definition; it now supplies two real stops. The history reference fixture expected a rectangle name from the starter path; it now explicitly authors a name containing markup and Unicode before checking the reference label. Final native handle 63325 (`tests-native-current-final.txt`) exited 0: all 479 tests passed, zero skipped. This covers the sampler, constant SVG export, node marquee/publication, keyboard deletion, inspector and semantic mutation/history cases registered in that binary.

Native fixture PDFs already emitted by the old run were rendered independently with PDF.js and compared against Sharp SVG across the thirty-eight-scene compositing corpus. Uncached run 16073 passed 283 tests, 149187 assertions, zero skipped; the native PDF raster check executed successfully. This verifies the emitted PDF bytes, not newer uncompiled Rust source or end-user file delivery.

The first oracle retry returned cached output with the PDF check skipped even though the environment requested it. Draw JS tests now disable Nx caching so conditional external-fixture checks cannot be concealed by a prior result. No new executable command was added.

## Remaining Acceptance

PNG still uses the reduced semio bridge and still loses authored appearance; prepared paint is a prerequisite, not completed PNG export. Vector coverage, stroke outlines, text/image rasterization, isolated scene compositing and responsive export jobs remain required. Rebuilt browser editing/download journeys, the complete native suite and the wider acceptance ledger remain open.


## Component Verification Queue

Current `@semio-tech/draw-plugin` lives at `🌎️hub/🧩️compositions/🖍️draw/📦️packages/🦀️rust`. Started the existing Nx describe/materialize-dev targets together, parallelism one, uncached: handle 56989, log `build-draw-current-oct02.txt`. The handle is terminal (exit 130): component-dev refused an outdated `🌎️hub/Cargo.lock` under `--locked`, so describe/materialize did not execute. Started the existing workspace:deps-cargo-lock target to synchronize all discovered Cargo workspace locks, log `deps-cargo-lock-current.txt`. Rebuild only after that prerequisite succeeds. Native test success does not verify the component build, app rendering or completed file delivery. After successful materialization, activate the current Draw React dev target and verify editor journeys in the browser.

Canvas paint still handles degenerate gradients directly through the host API and needs the same authored constant-paint semantics as SVG. PNG remains unchanged. No completion claim or ticket closure is made.

Workspace lock maintenance handle 20725 exited 0 (`deps-cargo-lock-current.txt`). Component describe/materialize was restarted after that completed prerequisite; handle 19635, log `build-draw-current-locked.txt`. Poll the handle before any new component build.


## Canvas Constant Paint — 2026-10-02

The shared fixture corpus now additionally covers collapsed gradients with unsorted and tied stops, empty radial gradients and single-stop radial paint. Framework `constantGradientColor` resolves transparent empty paint, one-stop color and the effective last stop for degenerate geometry, without allocating a sorted stop copy. Both Canvas2dHost paint and framework canvas raster use this domain-neutral helper. Existing Rust prepared paint consumes the same fixture semantics. The independent @napi-rs/canvas pixel regression compares both canvas consumers with Sharp pixels from exported SVG at opacity 1 and 0.35. It reproduced a host shader failure for empty gradient paint before the change.

RED handle 91513 exited 1: 284 passed and the new canvas test failed. GREEN handle 34215 exited 0: 285 tests, 157457 assertions, zero skipped; the PDF oracle was enabled and executed. Framework renderer focused handle 26409 passed 43 tests across three files. Framework 2D handle 82616 passed eight tests. Native handle 31634 passed all 479 tests, zero skipped, including the expanded shared sampler/export fixture corpus. All runs were uncached through the existing Bun/Nx targets. Renderer typecheck handle 99393 failed on five unrelated existing errors: missing wire mutation `line` fields, a missing `idleInstalledServiceStatusV1` identifier and an unknown test seed; no changed paint file was reported. Typecheck success is not claimed.

Component handle 19635 is terminal success: ten describe/materialize tasks in 3m59s (`build-draw-current-locked.txt`). Explicit activation handle 57526 also succeeded. Initial preview 5423 was stopped after shared HMR interrupted the tool menu; replacement stable preview 75879 runs on 6065 with `SEMIO_VITE_HMR=0`, log `preview-stable-oct02.txt`. Browser 2/tab handle `tab` opened the current Draw Editor with the Demo fully fitted to its canvas. Edit Nodes is available and pressed; direct selection chooses Orange Wedge and displays its full appearance/transform inspector. Point drag completion, undo, node marquee and deletion browser results are still being checked. PNG production remains unchanged and incomplete.
