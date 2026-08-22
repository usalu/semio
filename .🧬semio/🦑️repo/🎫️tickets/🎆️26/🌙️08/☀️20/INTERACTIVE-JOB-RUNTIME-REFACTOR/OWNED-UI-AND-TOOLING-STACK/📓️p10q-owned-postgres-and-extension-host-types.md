# P10q Owned Postgres and Extension-Host Types

## Outcome

- Replaced the coordinator's direct `pg` declaration-package boundary with a concise workspace-owned structural constructor and pool contract loaded through `createRequire`.
- Removed `@types/pg` while retaining the existing `OwnedDatabasePool` public API.
- Replaced the VS Code extension test suite's Mocha declaration-package dependency with its exact owned host contract: `suite`, `test`, `suiteSetup`, asynchronous bodies, and the suite timeout context.
- Removed `@types/mocha`; the actual extension-host test runner remains unchanged.

## Validation

- `bun install --ignore-scripts` passed after each manifest removal and updated the lockfile without installing replacement packages.
- Coordinator quick target passed (no test files).
- Focused TypeScript validation of `🟦️server-implementations.ts` passed with `tsc --noEmit`, bundler resolution, Node platform types, and no `pg` declarations.
- The VS Code package's TypeScript diagnostics contain no missing Mocha global or owned-contract error after removal.
- The dependency freeze passed at **169** identities, down 69 from baseline, and records both `js:@types/pg` and `js:@types/mocha` as removals.
- JS manifest/source parity passed with 83 manifests, 290 external rows, 141 evidenced, 149 unowned, and 0 undeclared imports.

## Existing VS Code Package Gate Defects

- The Nx build target invokes Vite without a config and fails on a nonexistent `index.html`; its second configured build references a nonexistent `🟦️vite.test.config.ts`.
- The lint target references a nonexistent `🟦️eslint.config.ts`.
- The package TypeScript graph already contains unrelated explicit-`.ts` configuration errors, incomplete extension declarations/implementations, and stale test exports. These are recorded as red package gates and are not represented as passing evidence for this dependency packet.

## Boundary

No external implementation type escapes either owned contract. Removing these declaration packages does not remove the active PostgreSQL runtime or the VS Code/Mocha extension-host runtime; those are separate Phase 10 implementation packets.
