# Published PDF Glyph Policy — 2026-10-04

The actual source-covered viz33 molecular-orbital PDF contains U+FFFF extraction placeholders where its two stock sigma labels failed to render. Root independently rasterized and inspected the boxes and extracted U+FFFF / U+FFFF* with pypdf. The retained actual before PDF is49302bytes with SHA256 `beadb7d319feae2db49f354d6f77185f5615082db39b21d245a206992ee35b88`.

Registered `@semio-tech/print:test-viz --skip-nx-cache` before retry22668 exited1 with `AssertionError: Missing expected exception`: independent PDF.js actually extracted U+FFFF from that PDF and the previous admission did not reject it. First attempt68704 failed only because the builder removed its temporary stage before inspection; it is not counted as feature-red evidence. The retry used an owned copy under generated/pdf-glyph-regression.

The existing pipeline helper now rejects the66 Unicode noncharacters (U+FDD0..FDEF and FFFE/FFFF at every Unicode plane end) and the explicit replacement character U+FFFD in extracted published page text. The language-agnostic PDF consumption fixture declares67 rejected scalar values and six accepted strings covering English/German accents, Greek mathematical characters, superscripts, a supplementary emoji, private-use glyphs and the valid supplementary-plane scalar U+1FFFD. Valid private-use and supplementary scalars remain admitted. Intentionally empty pages retain the prior document-wide nonempty-text contract; glyph validation does not invent a per-page text requirement.

Each full consumer page applies the policy and names its PDF/page and invalid U+ code point in a failure. Quick verification runs neutral policy vectors and, only when the existing test environment supplies an actual before PDF, proves that independent PDF.js extracts and rejects the real compiler counterexample. The existing registered test-viz and test-viz-full commands carry the checks, so no new command/script/launch route is required. The correction changes only existing test ownership; no runtime module/dependency or source admitted by the294-file native compiler inventory changes.

This policy detects invalid extraction markers, including the observed missing-font case. It does not equate readable extraction with complete geometric or physical-font coverage; actual raster, native font-role and containment checks remain independent required evidence. Root owns the chemistry schema/default glyph and rung-boundary correction. Registered after88707 is live; no after PASS is asserted yet.

Changed files:

- 🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🧫️pdf-consumption.json
- 🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts

## Registered After Result

Registered no-cache after88707 completed exit0. Independent PDF.js observed and rejected the actual missing-glyph page in before33; all six accepted neutral strings and67 forbidden scalar values passed. The same canonical test-viz run passed605/605 current quick inference checks and1966/1966 taxonomy leaves via1738kinds, API131/131. The policy predicate and per-page consumer call are stable. A subsequent scoped default-root correction routes all existing pipeline temporary font/bundle/macro tests to ticket/generated/print-pipeline when SEMIO_TICKET_DIR is supplied; the final registered full gate will replay that current default. The original after run cleaned its older dist scratch paths in existing finally clauses.

The current ticket-default registered replay98215 completed exit0 in2m34s. The log contains actual ticket/generated/print-pipeline font scratch paths and successful cleanup, one actual PDF.js missing-glyph page rejected, six accepted vectors,67 rejected scalars,605/605 inference checks and1966/1966 leaves through1738 kinds/API131. This closes the scoped scratch-default replay; final all162 PDF consumption remains required after source-frozen catalogue rebuilding.

