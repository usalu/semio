# Final Runtime Example Isolation Audit — 2026-10-06

Read-only follow-up over current authored TS/TSX/Rust source and workspace/package declarations. No runtime tests executed. Concurrent changes mean an initially found code edge may have disappeared before report completion. Excluded dependency/build/generated/ticket trees. Searches were followed with source context so cfg(test), test-only orchestration and source-text examples are not confused with production loading.

## Actionable Findings Sent to Owners

- Rust source-direction oracle helper `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧱️rust-source-direction/🧫️fixtures/🟦️.ts` exported `writeRustLayerOracle`, used by sibling test and the dependency-direction runtime execution test. Functionality was imported from fixtures. Runtime owner notified; file was already absent on final recheck, so this finding is closed by concurrent work.
- Initial plugin scan found imports of Semio geometry, LAS snapshot, DWG snapshot/body/style/header helper modules and WFC suite helper modules physically under fixture directories. Plugin owner was already moving these helpers; final authored file census found no plugin fixture `.ts`/`.rs` helper files.
- Initial Cargo workspace scan found LAS/GIF/PDF1.7/DXF/OBJ `🏭️generator/🧫️fixtures/📦️packages/🦀️rust` package members in `✏️s/Cargo.toml` lines164/170/193/211/218. Plugin owner notified to confirm generator package closure and declarations.
- Print oracle metadata initially referenced executable scripts physically under OS neural retirement, replication wire local-interaction, and value ordered fixture paths. Runtime owner notified to confirm current paths and retarget metadata after script relocation.

## Verified Acceptable Examples

- Store assembly lifetime Rust source examples are read as UTF-8 TEXT by `.../🏪️store/🧷️assembly/⏳️lifetime/🧪️tests/🟦️.ts` lines11–12 and staged for compile/pass/fail checks. Providers, original API and client examples are inert test input source documents; keep them.
- Repo language parser, statute fixer, malformed region, source-command composition and TypeScript path-collection sample code files are parser/fixer input documents, not functionality helpers. Function/definition spelling inside these files is intentional test content.
- Puzzle retained-job fixture includes are in cfg(all(test,...)). Trinity NAKAGIN_CHILD and nakagin_fixture each explicitly cfg(test). TIFF mutation test_case loaders explicitly cfg(test). Procedural generation3d_all_retained_mutation_fixtures_for_test explicitly cfg(test). Spatial session analytic mesh include explicitly cfg(test)+test. OS space SQLite refusal loader explicitly cfg(test)+test. Store history native reads sit in #[test] functions.
- NodeGraph TSX fixture matches are docstrings only, verified with rg -a because file contains a NUL; no fixture import found there.
- Package globs in Hub/S use negative fixture exclusions. Oracle descriptors naming before/after/mutation example files are test input references; preserve them.
- Capability description catalog code is a verification/oracle module, with independent schema validation and fixture proof; plain neutral input fixtures there are acceptable. Stdio artifact package verifier admits individual positive/negative package examples against the genuine package domain; preserve that domain.

## Scope Limits

No new ungated production JSON fixture loader was confirmed in this follow-up. This is not a blanket clean claim: generator/workspace/script metadata closures were handed to owners, and proof-schema classification is covered by the separate structural report. The final filesystem census found26 source-suffixed files under fixture paths; they were predominantly parser/compile-check text examples, with the one executable oracle helper disappearing during review.
