# Canonical Test Layout Verification

> Current closeout status: the latest whole-repository scan inspected 29,452 sources and found 37 new SPR command/testkit mutation-fixture violations. The earlier 58 plugin/TypeScript findings are repaired and absent from this scan. SPR repairs and a new final scan are pending.

The refactor places authored executable tests under their semantic owner at `<parent>/🧪️tests/<test-name>/<implementation>`. The earlier repository scan returned **zero layout violations across 29,224 authored source files**; the later concurrent additions listed above are pending final verification. The test-layout policy enforces the structure, implementation names, named cases, and owner scope; negative vectors cover legacy filenames, delivery directories, hidden inline tests, and misleading source text.

Legacy test filenames and misplaced bodies were relocated across Rust, TypeScript/JavaScript, Go, and Python. Script self-tests, Storybook interaction tests, and nested Rust test modules were included. Runners, module mounts, imports, fixture paths, and source-policy evidence readers follow the canonical locations. Production source checks remain separate from test-law evidence. The user’s example now lives at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🏛️space-administration/🟦️.tsx`.

## Executed Verification

| Check | Observed result |
| --- | --- |
| Earlier whole-repository test-layout scan through Bun/Nx | 29,224 authored sources; zero findings before the later 58 concurrent findings |
| Layout policy and Rust source-evidence tests | 25 passed; 84 assertions |
| Independent Rust module-resolution oracle | Six valid trees compiled and ran; one deliberately invalid tree rejected |
| Full repository Nx test-contract discovery | 250 projects, including 248 generated canonical case projects |
| Full repository P2a1 policy dispatcher | Live source clean; all 13 hostile mutations verified |
| Surface, styling, and graph moved suites | 45 passed; 1,443 assertions; real build and asset-delivery checks |
| Existing action-bus wire-retirement dispatcher | Passed five ownership cases, five hostile fixtures, and four short-close frontiers |
| Final Rust-only layout and literal-edge audit | 19,860 sources; zero layout findings; final 7,166 canonical test files / 20,740 test edges with zero broken |
| Focused Rust resident and replication Nx targets | 17 and 257 tests passed respectively |
| Browser actor canonical dispatcher | Passed full AJV/JCO/Wasm/JSPI pipeline, emitted modules, and actor pack/stream/close laws |
| JS/TS runner coverage | 518 classified cases with zero unreferenced; one concurrent caching/Wasm case independently verified through its existing dispatcher (519 at that snapshot; the two later TypeScript cases have separately verified callers and runtime) |
| JS/TS move literal-integrity audit | 69 move pairs checked against pre-goal source; zero remaining non-path literal rebases |
| Go MCP and focused CLI Nx targets | Passed through canonical source overlays |
| Canonical Go compiler oracle | Two private-function tests passed; Bun wrapper passed six assertions |
| Stable Python subset | 23 passed |
| Python stage modules | All three canonical modules compiled/imported; package and golden-fixture roots verified |
| Extracted framework fixture functions | Ten passed; two reached the preserved production depth-guard assertion |

The layout/source-evidence suite and caller suites used a private minimal Nx graph that dispatches the real repository code. The whole-repository project inventory and P2a1 dispatcher used the full repository graph. Generated Go `_test.go` names are compiler overlay entries only; no authored legacy test files are needed.

## Preserved Failures and Verification Limits

The reconstructed Rust manifest retains 154 removed legacy test-only input paths whose individual destinations could not be proven after a concurrent cleanup removed temporary execution journals. Body preservation for those inputs is supported by the retained batch records, overall test-attribute counts, and runtime checks; no per-file identity proof is claimed.

This work does not claim every repository test passes. The final Rust literal traversal also recorded 13 production-only path defects outside this test-layout task; no test edge remained broken. Existing source-policy failures remain in Puzzle fill, renderer interactivity, and schema/worker checks; extracted test-law resolution was repaired without weakening the production assertions. The built-tree and runtime-tree fixture checks both reach the same existing `0 !== 9` production guard-count assertion. Python research suites report existing asset/data digest failures and one optional missing `fitz` dependency. External-service and result-writing research harnesses were syntax checked without running their service operations.

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
- [Initial legacy JS/TS attribution](📓️test-layout-legacy-js-ts-attribution-2026-09-08.md)
- [Initial inline and flat-case attribution](📓️test-layout-initial-inline-manifest-2026-09-08.md)

Final executor manifests and ticket closure are being consolidated. Runner classification documents discovery and execution routes; it does not claim that all classified cases ran in this session. No Git commit, checkout, stash, or worktree operation was used.
