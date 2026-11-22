# Framework Type Drift 1122

Source: strict `tsc --noEmit` pretest (6 errors), see `release-stage-pretest-1122.md`.

## Fixes
1. `RepositoryCompilerStoreV1` (describe emitter): the public entry `repo/library/packages/typescript/storage/enforcement/compiler` re-exported the functions but not the type, which lives in `library/caching/storage/enforcement/compiler`. Added `type RepositoryCompilerStoreV1` to that explicit re-export (no alias).
2. `ResourceLeaseControl` (vite caching): the type import from `process/packages/typescript/leases` was missing; added `type ResourceLeaseControl` to the existing import.
3. `UnitRecordV1` (component-pass): already fixed by another agent, now clean.
4. `toWellFormed` / `isWellFormed` (3 errors): artifacts of the pretest tsconfig `lib: ES2023`; with `ES2024` they vanish. Not source defects.

## Proof
`tsconfig-drift-1122.json` (strict, noEmit, lib ES2024; play e2e/site/playrelease plus the three touched files) reports 0 errors. Before: 6, after: 0.
No other errors under `framework/modules/process` or `semio-tech/play`.
