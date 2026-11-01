STATUS: UNVERIFIED, BLOCKED UPSTREAM. No gis crate (semio-hub-gis, semio-hub-gis-component, gismap, gisterrain) has been compiled yet; none shows an error of its own either, because cargo stops at red framework dependencies owned by other lanes.

## Checks (hub wasm32-wasip2 unless noted; logs in C/🗑️generated/1101/)
- gis-1: red at framework value-clone-source (admission 🦀️.rs returned Err from a RetainedWorkOutcome fn). Fixed by me: `return failure(ValueError::invariant(..),0)`.
- gis-2: red at framework value-native-owned (framework lane, since fixed).
- gis-3 (after framework-green): 7 red framework crates: plugin-time-travel, plugin-retained-command, plugin-typing-edit, plugin-guest-contract, infinite-board-engine, flow-flow-model, stdio-contract-native-print.
- gis-4 (gismap workspace wasm, -p semio-s-artifact-gis-gismap): red at plugin-instance-history (2) and stdio-contract-native-print (1); peers were already fixing both at 07:53.
- gis-5 (hub wasm, after those fixes): only 2 red crates remain: `semio-framework-plugin-instance-render` (11 errors, infinite-board ports scene 🦀️.rs:8) and `semio-framework-artifact-flow-flow-copy` (10 errors, flow retained/copy 🦀️.rs:499-516). Owners: plugtools / flowext. semio-hub-gis itself not reached.

## Not done
- gismap and gisterrain wasm check, native tests, hub-gis inference.rs compile verification.
- Tile path (26/09/17 DEMONSTRATOR-GIS-MAP-STATIC-TILES, build-side Vite `bundle` mode) not examined: no source change in my lane touches it.

## Next
Rerun `1101-tools/chk.sh gis-6 🌎️hub/Cargo.toml wasm -p semio-hub-gis -p semio-hub-gis-component` once instance-render and flow-copy are green, port inference.rs by compiler output, then gismap/gisterrain wasm and native tests.

## Update 10:15
- Only edit of mine is the `value-clone-source` admission failure() return. Every other red crate belongs to another lane.
- gis-6 (hub wasm): red at stdio-contract-media-export (1) and flow-host-read (6). gis-7 (hub wasm, later): stdio contract crates now compile; red are ~15 stdio artifact io/model crates (deflate-examples, wav-io-registry, semio-*-snapshot, bmp-editor-2; lane describe). semio-hub-gis not reached.
- gis-8 (gismap workspace wasm, `-p semio-s-artifact-gis-gismap`): sole red crate `semio-framework-plugin-instance-dispatch` (2 errors, staging/typed-command 🦀️.rs:309 `close_demands(..)?` returns MemberValueError with no From to Fault; :310 `OriginalTurnFailure` lacks Display). Lane plugcore. gismap-demo-asset and the stdio contracts compile.
- Static tile review: fix from 26/09/17 is present in source: `♻️mit-bestand/🧺️demonstrator/🔨️modules/📦️site/🗺️tile-serve-mode/🟦️.ts` (`bundle` for vite build, fetch for serve; z<=10), wired in `🏗️builder/🌐️vite/🟦️.ts:111`, test in `🧪️tests/🧪️demonstratormaptiles`. `.🧬semio/🗺️map` holds openfreemap-vt, osm-tiles, terrarium-dem. No `dist` exists here, so `dist/site/vt/3/1/2.pbf` is UNVERIFIED until the coordinator's demonstrator build.
- Still UNVERIFIED: hub-gis/inference.rs, hub-gis-component, gismap, gisterrain wasm; native tests.
Logs: gis-{1..8}.txt.

## Update (after desc-w7)
- gis-9 (hub wasm): red only in stdio artifacts: pdf-schema-4 (6, unresolved `editor::pdf17*::modes::edit::windows`), docx-base-model (1, `unwrap_or_else` on RetainedWorkOutcome), tiff-mutation-regressions (2). semio-hub-gis not reached.
- gis-10 (gismap workspace wasm): instance-dispatch is no longer red; now red: stdio pdf-schema-5 (24 errors, mirror of pdf `vt` subset: missing `semio_s_artifact_stdio_pdf_testkit`, `PdfComposer/PdfAnalyzer/PdfParts` imports, `blank_pdf_snapshot`, `definition`, `PDF17A_EDIT_MODE_ID`) and pdf-mut-family-15 (4). Both pdf lane (mid-refactor). gismap is not reached; no error inside any gis crate has been seen.
- STILL UNVERIFIED: hub-gis, hub-gis-component, gismap, gisterrain wasm; native tests. Logs gis-9.txt, gis-10.txt.

## Update (gis-11)
- gis-11 (hub wasm) ran 3811 s and was stopped by the ingress deadline (rc 1). Red crates: stdio pdf-schema-4 (2: unresolved `semio_s_artifact_stdio_pdf_mut_family_11`, missing `import::decode_pdf`; pdf lane) and framework-os-flow-extensions (4: `ExtensionResourceOwner::take_work` removed from the trait, private `RetainedEvaluation.work`; flow lane). docx/tiff are fixed. semio-hub-gis not reached.
- The gismap dependency chain pulls stdio pdf-schema, so a native gismap test run has the same blocker (gis-10); not started.
- STILL UNVERIFIED: hub-gis, hub-gis-component, gismap, gisterrain wasm; native tests.
