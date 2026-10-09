# Semantic Boundary Verification

## Executed

- Tree-sitter parsed all 45 diff representation owners before and after extraction.
- Tree-sitter parsed the 22 Generation2d replay declarations after extraction.
- Tree-sitter parsed 790 Rust sources across Lowpoly and Procedural without syntax errors at the recorded check. Later small codec/macro edits remain covered by the final source audit below.
- Actual Nx `@semio-tech/repo-lib:test-artifact-io-ownership --skip-nx-cache` completed RED: 4 tests passed, 3 failed, 1 module error. The new semantic reverse test referenced the not-yet-implemented scanner export; two physical codec laws failed in other concurrent ownership scopes and were handed to their owner. This is initial TDD evidence, not the final result.

## Native Runs In Progress

- `bun nx run @semio-tech/energy-model-rs:check --skip-nx-cache`, tool session 24793: four prerequisites completed; the owner reached Cargo preparation after shared queue contention. Last output reports native owner running.
- `bun nx run @semio-tech/lowpoly-lowpoly-rs:test --skip-nx-cache --excludeTaskDependencies`, tool session 95288: dispatched through the registered package router and remained in shared Cargo preparation.

Neither native check has returned a compiler or test result yet. No compiled or runtime pass is asserted. Native processes are owned by these executions; no unrelated processes were interrupted.

## Final Source Audit

Tree-sitter parsed all 497 existing Rust files listed in this agent's four changed-file reports after the final codec and media edits, with no syntax errors. This establishes source syntax only, not compilation.

Both native owners have left Cargo preparation and report owner commands running. Energy session 24793 last reports elapsedMs=150044; Lowpoly session 95288 last reports elapsedMs=70019. The parent task retains these session IDs for final compiler and runtime monitoring.
