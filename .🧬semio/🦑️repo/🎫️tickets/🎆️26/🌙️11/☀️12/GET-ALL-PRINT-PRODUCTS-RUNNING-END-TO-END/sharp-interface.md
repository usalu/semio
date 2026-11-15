# sharp behind an owned Print raster interface

## Convention found
- Print isolates pdf.js and @napi-rs/canvas by exporting owned types/DATA from `🟦️.ts` modules and importing the library only in worker `📜️script.ts` files (`🧰️framework/🛍️products/📓️print/📦️artifacts/📖️pdf`, `🪟️panels/📜️script.ts`); exported types state "without an external PDF implementation type".
- No owned image-processing or SVG-rasterizing interface existed. `🔲️pixels` only owns PNG/JPEG decode/encode (no SVG render, blur, saturate, composite); `sharp` appears elsewhere only in tests as oracle.
- Runtime sharp users were two: `🖼️resources/🟦️.ts` (SVG to PNG) and `🪟️panels/📜️script.ts` (dynamic import; crop, blur, modulate, composite tint).

## Change
- New owned interface `PrintRasterEngineV1` (`svgToPng`, `frostPanel`) with owned request types only.
- New adapter `sharpPrintRasterEngineV1`: the single Print source importing `sharp`.
- `resources/🟦️.ts` keeps validation/policy and calls the interface; panels worker calls `frostPanel` (lazy import of the adapter).
- Tests keep sharp as third-party oracle (allowed).
- Reimplementation note: SVG rendering, gaussian blur, saturation modulate and compositing are not trivial; `🔲️pixels` lacks them, so not attempted now. Blur/modulate/over-composite could later move into `🔲️pixels`; SVG rasterization is the hard part.

## Files
- NEW 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🖼️raster/🟦️.ts
- NEW 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🖼️raster/🔌️sharp/🟦️.ts
- EDIT 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🟦️.ts
- EDIT 🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🪟️panels/📜️script.ts
- EDIT 🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json (two new sorted sources added to both tectonic entries; sharp external kept)

## Tests
- `bun test` resources tests: 4 pass, 0 fail.
- Panels tests and verification helper need a repo process envelope under plain bun (fail with "Original parent Script process invocation required", environmental, not related); covered by the real run.

## Real run
`repo run @semio-tech/print:test-viz --lang=en --output=static --skip-remote-cache` started in background; log/exit in `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️12/GET-ALL-PRINT-PRODUCTS-RUNNING-END-TO-END/🗑️generated/sharp-test-viz.{log,exit}`.

## Follow-up: fixture correction and final run
- A concurrent write had dropped the fixture edit; the type-only `🖼️raster/🟦️.ts` is not in the esbuild graph, so only the adapter `🖼️raster/🔌️sharp/🟦️.ts` is listed, in both tectonic entries. All four entries (sources, owners, externals) verified against the test's own graph function (`compute-boundaries.ts`).
- Final run `repo run @semio-tech/print:test-viz --lang=en --output=stream --skip-remote-cache`: exit code 0 (log `🗑️generated/sharp5-test-viz.log`).
