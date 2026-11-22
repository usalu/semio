# Native TypeScript Report: process plugin

All commands run with bun v1.4.2 directly from the repo root (repo build tooling bypassed per user instruction).

## Commands and results
1. Catalog generator (the `s-dev-composition:generate` body): `bun native-generate.ts` (calls `GenerateScript.run` with the catalog publication; the Script constructor needs a repo process envelope, so the wrapper bypasses it). Result: OK, 74 plugin crates, 158 playgrounds, `🧩️catalog/🤖️generated` refreshed.
2. stdio format claims (generate dependency): `bun native-stdio-claims.ts`. Result: OK.
3. `bun test` on dev-route, demo-session and demo example tests (the `process-js:test` files): 5 pass, 0 fail, 3 files.
4. `checkRetainedRoutes` (second half of `runTestScript`): `bun native-retained-routes.ts`. Result: OK, routes=34 migrated=34, oracle independent.
5. Host-capacity: `bun test` over all 38 files under `🏗️build/🌡️host-capacity/🧪️tests`: 297 pass, 2 skip, 0 fail.

## Root causes / findings
- Host-capacity policy schema: already consistent. Schema, decision code (`hostSwapStopV1`), and `policy.json` all use `activeBasisPointsPerSecond`, `outBasisPointsPerSecond`, `shortBasisPoints`; no remaining `*MegabytesPerSecond` policy fields anywhere under framework or `✏️s`. No change needed.
- process3d examples (demo-session, demo): both pass (item 3 covers them); the earlier failure did not reproduce.

## Files changed
- Created (ticket folder only): `native-generate.ts`, `native-stdio-claims.ts`, `native-retained-routes.ts`.
- Regenerated: `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/*`, `✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🤖️generated/stdio-format-claims.json`.
- No source changes.
