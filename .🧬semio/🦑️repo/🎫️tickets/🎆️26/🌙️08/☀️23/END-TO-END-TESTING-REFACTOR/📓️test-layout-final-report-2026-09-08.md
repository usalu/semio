# Canonical Test Layout Verification

> Final layout verification: **zero violations across 29,983 authored source files**, completed at 2026-09-08T23:19:15.450Z. All identified late additions are included. The repository ticket is closed through the real MCP with all 9,689 authored paths submitted.

The refactor places authored executable tests under their semantic owner at `<parent>/🧪️tests/<test-name>/<implementation>`. The final repository scan returned **zero layout violations across 29,983 authored source files**. The test-layout policy enforces the structure, implementation names, named cases, and owner scope; negative vectors cover legacy filenames, delivery directories, hidden inline tests, and misleading source text.

Legacy test filenames and misplaced bodies were relocated across Rust, TypeScript/JavaScript, Go, and Python. Script self-tests, Storybook interaction tests, nested Rust test modules, and executable Rust documentation examples were included. Runners, module mounts, imports, fixture paths, and source-policy evidence readers follow the canonical locations. Production source checks remain separate from test-law evidence. The user’s example now lives at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx`.

## Executed Verification

| Check | Observed result |
| --- | --- |
| Final whole-repository test-layout scan through Bun/Nx | 29,983 authored sources; zero findings |
| Layout policy and Rust source-evidence tests | 25 passed; 84 assertions |
| Independent Rust module-resolution oracle | Six valid trees compiled and ran; one deliberately invalid tree rejected |
| Full repository Nx test-contract discovery | 250 projects, including 248 generated canonical case projects |
| Full repository P2a1 policy dispatcher | Live source clean; all 13 hostile mutations verified |
| Surface, styling, and graph moved suites | 45 passed; 1,443 assertions; real build and asset-delivery checks |
| Existing action-bus wire-retirement dispatcher | Passed five ownership cases, five hostile fixtures, and four short-close frontiers |
| Earlier Rust-only layout and literal-edge audit | 19,860 sources; zero layout findings; final 7,166 canonical test files / 20,740 test edges with zero broken |
| Focused Rust resident and replication Nx targets | 17 and 257 tests passed respectively |
| Browser actor canonical dispatcher | Passed full AJV/JCO/Wasm/JSPI pipeline, emitted modules, and actor pack/stream/close laws |
| JS/TS runner coverage | 518 classified cases with zero unreferenced; one concurrent caching/Wasm case independently verified through its existing dispatcher (519 at that snapshot; the later TypeScript, Binaryen, CI baseline, and plugin optimization cases have separately verified callers and runtime) |
| JS/TS move literal-integrity audit | 69 move pairs checked against pre-goal source; zero remaining non-path literal rebases |
| Go MCP and focused CLI Nx targets | Passed through canonical source overlays |
| Canonical Go compiler oracle | Two private-function tests passed; Bun wrapper passed six assertions |
| Stable Python subset | 23 passed |
| Python stage modules | All three canonical modules compiled/imported; package and golden-fixture roots verified |
| Extracted framework fixture functions | Ten passed; two reached the preserved production depth-guard assertion |
| Late plugin mutation-fixture extraction | All 16 relocated laws passed; binary dependency evidence covers all 52 extant changed Rust sources |
| Late SPR mutation-fixture extraction | Exact preservation of 13 bodies and 24 fixtures verified; all 13 laws and the registry consumer passed |
| Late TypeScript semantic cases | Playground preferences and production browser artifacts passed with schema/lodash and Vite/Nx cache checks |
| Binaryen toolchain semantic case | Schema/platform checks, cancellation, mocked checksum rejection, and output cleanup passed |
| Plugin native optimization semantic case | Native Binaryen output matches independent JavaScript Binaryen bytes and executes the expected WebAssembly result |
| CI baseline semantic case | 11 selection vectors, fallback, validation, cancellation, and read-only native Git ancestry passed |
| Renderer typed-result-page case | One test passed against actual extracted production declarations and the existing neutral result-lane fixture |
| UI image-builder Rustdoc cases | Both passed through real Cargo Rustdoc: expected NoAlt compile rejection and described/decorative runtime success |
| Complementary Rustdoc layout audit | 19,874 authored Rust files; zero executable inline line-doc fences; both documentation consumers include canonical test sources |

The layout/source-evidence suite and caller suites used a private minimal Nx graph that dispatches the real repository code. The whole-repository project inventory and P2a1 dispatcher used the full repository graph. Generated Go `_test.go` names are compiler overlay entries only; no authored legacy test files are needed.

## Preserved Failures and Verification Limits

The reconstructed Rust manifest retains 154 removed legacy test-only input paths whose individual destinations could not be proven after a concurrent cleanup removed temporary execution journals. Body preservation for those inputs is supported by the retained batch records, overall test-attribute counts, and runtime checks; no per-file identity proof is claimed.

This work does not claim every repository test passes. The plugin `test-quick` target stopped at its existing `writer loses the returned completion owner` source oracle before Cargo; the 16 extracted plugin laws were executed separately using the completed, dependency-validated Cargo test binary. The renderer page check compiles the actual relevant production declarations and canonical test in a Cargo fixture; it does not establish a full WGPU renderer build. The final Rust literal traversal also recorded 13 production-only path defects outside this test-layout task; no test edge remained broken. Existing source-policy failures remain in Puzzle fill, renderer interactivity, and schema/worker checks; extracted test-law resolution was repaired without weakening the production assertions. The built-tree and runtime-tree fixture checks both reach the same existing `0 !== 9` production guard-count assertion. Python research suites report existing asset/data digest failures and one optional missing `fitz` dependency. External-service and result-writing research harnesses were syntax checked without running their service operations.

The full layering gate reported unrelated repository-wide excess references. Only proven moved-test entries were retargeted: renderer and library caps stayed at 2 and 39; the Rust DSL cap of 18 was split exactly; zero-reference old entries were removed. Total allowance decreased from 466 to 277. No unrelated baseline was regenerated or increased.

## Detailed Evidence

- [Layout policy and compiler checks](📓️test-layout-policy-review-2026-09-08.md)
- [Whole-repository scan](📓️test-layout-current-snapshot-2026-09-08.md)
- [Independent final source audit](📓️test-layout-final-current-audit-2026-09-08.md)
- [Current runner map](📓️test-layout-runner-map-2026-09-08.md)
- [Independent runner closeout](📓️test-layout-final-closeout-audit-2026-09-08.md)
- [Runner snapshot reconciliation](📓️runner-inventory-scope-audit-2026-09-08.md)
- [Moved caller runtime checks](📓️test-layout-caller-runtime-2026-09-08.md)
- [Rust source-policy integration and P2a1 runtime](📓️test-layout-source-evidence-execution-2026-09-08.md)
- [Rust extraction, physical traversal, and runtime](📓️test-layout-rust-migration-2026-09-08.md)
- [Native language checks and limitations](📓️test-layout-native-migration-2026-09-08.md)
- [Layering provenance](📓️test-layout-layering-baseline-audit-2026-09-08.md)
- [Applied layering update](📓️test-layout-layering-update-2026-09-08.md)
- [Late TypeScript case verification](📓️test-layout-late-typescript-2026-09-08.md)
- [Late plugin mutation extraction](📓️test-layout-late-rust-mutation-fixtures-2026-09-09.md)
- [Late SPR preservation and runtime](📓️test-layout-late-spr-2026-09-09.md)
- [Binaryen case verification](📓️test-layout-late-binaryen-2026-09-09.md)
- [Plugin native optimization case verification](📓️test-layout-late-native-optimization-2026-09-09.md)
- [CI baseline case verification](📓️test-layout-late-ci-baseline-2026-09-09.md)
- [Renderer and UI documentation cases](📓️test-layout-final-source-cases-2026-09-09.md)
- [Rustdoc source audit](📓️test-layout-rustdoc-audit-2026-09-09.md)
- [Initial legacy JS/TS attribution](📓️test-layout-legacy-js-ts-attribution-2026-09-08.md)
- [Initial inline and flat-case attribution](📓️test-layout-initial-inline-manifest-2026-09-08.md)

The [complete authored-file manifest](📓️test-layout-complete-authored-files-2026-09-08.md) combines explicit executor and coordinator arrays, including removed paths. The [closure record](📓️test-layout-ticket-closure-2026-09-08.md) records the real repository MCP transport and records the complete submitted file count and persisted ticket status. Runner classification documents discovery and execution routes; it does not claim that all classified cases ran in this session. No Git commit, checkout, stash, or worktree operation was used.
