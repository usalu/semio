# Terra Independent Globals Retirement Audit — 2026-08-23

## Decision

**ACCEPT.** The root direct `globals` identity and its only direct configuration use are retired. The verified direct-dependency boundary is **136 = 73 JavaScript + 63 Rust** identities.

This audit read the repository instructions, the governing Interactivity-First Refactor plan, the accepted `terra-next-accepted-dependency-scout-after-pngjs-2026-08-23.md`, and `p10-owned-globals-retirement-2026-08-23.md`. It inspected and tested the live shared tree without changing production sources, Rust, Compose, Dagre, cache, git state, or ticket metadata.

## Live Ownership And Regression Proof

- `.storybook/🟦️lint-tooling.ts` imports only `eslint-plugin-storybook` and `typescript-eslint`. Its factory has five flat-config contributions, zero `languageOptions.globals` properties, and zero `no-undef` rule properties.
- UI React's `🟦️eslint.config.ts` imports that actual factory. The Vitest `includeSource` path resolves to the repository's `.storybook/🟦️lint-tooling.ts`, and the permanent in-source assertion is therefore discovered rather than merely present on disk.
- Focused permanent proof, through Nx: `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- /Users/ueli/Documents/semio/.storybook/🟦️lint-tooling.ts --reporter=verbose` passed **1 file / 1 test** (`does not depend on predefined globals or enable no-undef`).
- Full uncached UI quick test passed **21 files / 724 tests**. Uncached UI lint and typecheck also passed.
- Active ESLint `--print-config` for UI React `📦️index.tsx` reports `hasGlobals=false`, `globalsCount=0`, and `hasNoUndef=false`. The active resolved configuration consequently does not use the removed ambient-name map or enable the rule that would consume it.

## Dependency, Lock, And Absence Evidence

- `bun install --frozen-lockfile` checked **1,945 installs across 1,997 packages** with no changes.
- `bun ./📜️script.ts verify dependencies` reports 136 current identities and no new third-party dependency. Direct list counts independently parsed as **73 JavaScript** (no `globals`) and **63 Rust**. JavaScript parity passed: `manifests=83`, `external-rows=258`, `evidenced=109`, `unowned=149`, `undeclared-imports=0`, `lock-workspaces=44`, `lock-mismatches=0`, `lock-fixtures=5`.
- Live non-Compose/non-ticket scans found zero direct `globals` imports, `globals.browser`, `globals.node`, or manifest `"globals"` declarations.
- The reconciled `bun.lock` contains no `globals` root workspace tuple and no `globals@16.5.0` resolution. `bun pm why globals` correctly reports no matching lockfile package.

## Corrected Lock Reasoning

The scout's older retention expectation is superseded by the live reconciled lock. Its `eslint@10.8.0` entry has **no runtime `globals` dependency edge**. The installed ESLint manifest has `globals: ^16.2.0` only in `devDependencies`, not `dependencies`; a consumer does not install that edge transitively. `globals@16.5.0` is therefore orphaned and correctly absent after reconciliation.

The exact globals-related change against HEAD is two deletions: the root workspace `"globals": "^16.4.0"` row and the `globals@16.5.0` resolution row. The currently staged `bun.lock` patch also includes concurrent, out-of-scope removals for language detector, React Router, Pixelmatch/PNGJS and associated records plus a Cookie resolution rewrite; those are distinct composite-patch hunks and are not attributed to this retirement.

## Hygiene

- Prettier passed for every parseable changed source/config file: `.storybook/🟦️lint-tooling.ts`, `package.json`, and UI React `🧪️vitest.config.ts`. `bun.lock` is Bun-generated and was validated by the frozen install.
- Scoped `git diff --check` passed for working, staged, and HEAD comparisons across the four authorized files. The implementation surface is staged; the scoped working diff is empty.

The governing plan's parity-before-deletion rule is satisfied by the maintained regression proof and all current lint/test/dependency gates. This decision accepts only the direct `globals` retirement, not the Phase 10 end state or unrelated concurrent lock changes.
