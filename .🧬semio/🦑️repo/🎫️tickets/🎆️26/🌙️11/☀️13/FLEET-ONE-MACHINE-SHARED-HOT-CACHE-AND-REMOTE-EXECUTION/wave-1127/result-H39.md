# H39 result: pruning custody race no longer fails a build (STAGED, not landed)

Status 05:05: LANDED in slot 05:00 (lock landing.lock.pruning taken and released). Scratch-copy test was infeasible (the law needs the real checkout for source admission), so tests ran live after landing.

## Root cause
`retireInactiveCargoStoreOwnedV1` (caching/pruning/budget/execution) threw `Inactive Cargo unit changed under physical custody` when the recensus under physical custody found any planned unit changed/locked/gone. A unit used meanwhile is active by definition; the throw propagated through `recoverCompactCompilerHeadroomV1` into the caller's compile, failing the Nx task.

## Fix (staged: `T/🧰️scripts/H39/apply.py`, anchored, atomic per file, asserts each anchor once)
1. `execution/🟦️.ts`: changed/locked/missing units are dropped from the plan (`retained`), described with `describeCacheRecencyChangeV1` and written bilingual (en/de note) to `<turn>/skipped.json`; only retained units are deleted. Result shape unchanged (units/plannedUnits differ), so schema and result law stay valid. Docstring bilingual note.
2. `conditional/🟦️.ts` (Windows sibling, same pattern, `Conditional Cargo unit changed before kernel custody`): units changed at recensus are filtered out instead of throwing.
3. Tests: `compact` law now asserts retirement resolves with units 0, the changed unit is kept, `skipped.json` has en+de note and the exact diagnostic (bytes changed, not missing, not locked) -- this is the race test (output rewritten between plan and execute via `onCustodyHeld`). `recensus` test title/expectation updated (no rejection regex).
Sibling grep ("changed under", "ownership changed", custody throws) in the caching area: only these two recensus throws; `Inactive retirement ownership changed` (delete loop) checks plan-time facts (`lockHeld` already excluded by the planner) so it is unreachable by the race and left as an invariant; `Conditional compiler deletion bound changed` likewise.

## Critical path
The only build-path caller is `recoverCompactCompilerHeadroomV1` (called on headroom pressure from `withCompilerWorkspaceLifetimeV1`). With the race no longer throwing, it returns after the pass; if headroom is still short the compile itself reports the shortage honestly. Other errors there (abort, lease timeout, custody loss) are real and still propagate.

## Verified / unverified
- Verified: all anchors match exactly once on a scratch mirror; all 7 touched/adjacent files parse (`bun build --no-bundle`).
- UNVERIFIED: `repo typecheck`, the darwin compact law run (needs the real tree), Windows path (no mock covers the filter). Run at landing: apply.py, `repo typecheck`, dangling-imports, then `test-cargo-retirement` / the recensus law.
- Landing steps: `mkdir T/landing.lock.pruning` + owner, `python3 T/🧰️scripts/H39/apply.py` from repo root, typecheck, release lock, append `## Wave 1127 H39: pruning custody race` to landings.md.

## Landing verification (05:00-05:05)
- `repo typecheck` on the 4 touched files: passed (8 files incl. importers; 16 unrelated errors elsewhere not counted).
- `repo test-file` recensus law (runs the compact law, i.e. the race test): 1 pass, 0 fail. A first attempt failed transiently on a peer's in-flight `cache/volume` build, retry green.
- Dangling imports: only new import is `readdirSync` from node:fs in a test; typecheck covers it. No separate dangling-imports command exists in `repo --help`.
- Still unverified: Windows conditional filter.
