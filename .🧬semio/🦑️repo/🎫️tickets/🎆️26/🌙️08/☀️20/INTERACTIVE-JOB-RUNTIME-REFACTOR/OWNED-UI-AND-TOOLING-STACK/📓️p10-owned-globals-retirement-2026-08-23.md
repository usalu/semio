# P10 Owned Globals Retirement — 2026-08-23

## Status

**IMPLEMENTATION GATES COMPLETE; AUDIT PENDING.** This packet retires only the root direct `globals` tooling identity and its one active configuration binding. It does not accept Phase 10, claim the zero-dependency end state, or accept any Rust, Cargo, Compose, Dagre, ticket-metadata, coordinator, or unrelated dependency work. A separate independent audit owns acceptance.

The governing Phase 10 rule requires parity evidence before deleting an outgoing dependency. The accepted Terra scout identified `globals` as a one-binding leaf: `.storybook/🟦️lint-tooling.ts` imported browser and Node name maps, while the active UI React lint rules did not enable `no-undef`. The legacy map and an in-memory empty map were therefore compared against the complete lint target before source, manifest, and lock ownership were removed.

## Authorized Surface

- `.storybook/🟦️lint-tooling.ts`
- `package.json`
- `bun.lock`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts`
- this implementation report

No new script file was created. No Cargo command was run. No Rust, Compose, Dagre, ticket metadata/checklist, `AGENTS.md`, launch configuration, cache, or git state was modified by this packet.

## Implementation

`.storybook/🟦️lint-tooling.ts` no longer imports `globals` and no longer supplies `languageOptions.globals`. The TypeScript parser, `ecmaVersion`, source type, JSX parser option, file selectors, ignores, and Storybook recommended configuration are unchanged. No active lint rule was added, removed, or changed.

The root `devDependencies.globals` declaration and corresponding root workspace lock row were removed. Bun reconciliation also removed the now-unreachable `globals@16.5.0` resolution; the corrected lock reasoning is documented below.

## Permanent Regression Proof

An in-source Vitest assertion now builds the actual `createUiReactLintConfig()` result and requires both of these facts across every flat-config contribution:

- no `languageOptions.globals` property exists;
- no `rules["no-undef"]` property exists.

The existing UI React Vitest configuration now lists `.storybook/🟦️lint-tooling.ts` in `includeSource`. This is executable coverage, not an ornamental test:

- the ordinary uncached quick target passed **21 files and 724 tests**, one file and one test above the scout's 20-file/723-test baseline;
- a focused invocation through the same permanent Nx `test-quick` target explicitly reported `owned UI React lint config > does not depend on predefined globals or enable no-undef` and passed **1 file / 1 test**.

The active post-removal `--print-config` result for `📦️index.tsx` independently reported:

```json
{ "hasGlobals": false, "hasNoUndef": false, "globalsCount": 0 }
```

## Required Pre-Deletion Differential

Before editing, ESLint's API linted the same complete UI React package twice with the imported active flat configuration:

| Configuration                                                              | Files | Errors | Warnings | `no-undef` diagnostics |
| -------------------------------------------------------------------------- | ----: | -----: | -------: | ---------------------: |
| Existing `globals.browser` + `globals.node` map                            |    10 |      0 |        0 |                      0 |
| Same config with only `languageOptions.globals` replaced in memory by `{}` |    10 |      0 |        0 |                      0 |

The summaries were structurally identical. No production file was changed for this differential and every parser, selector, ignore, parser option, and rule contribution remained the same.

## Commands and Results

| Command                                                                                                                                       | Result                                                                                                                                      |
| --------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- |
| uncached in-memory ESLint legacy-versus-empty-map differential                                                                                | PASS; both configurations produced `{files:10, errors:0, warnings:0, noUndef:0}`.                                                           |
| `bun install`                                                                                                                                 | PASS; lockfile reconciled, one direct package removed.                                                                                      |
| `bun install --frozen-lockfile` after reconciliation                                                                                          | PASS; 1,945 installs across 1,997 packages checked, no changes.                                                                             |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache`                                                                                | PASS; 21 files, 724/724 tests. Existing `NO_COLOR`/`FORCE_COLOR` messages are non-failing Bun warnings.                                     |
| `bun x nx run @semio-tech/ui-react:test-quick --skip-nx-cache -- /Users/ueli/Documents/semio/.storybook/🟦️lint-tooling.ts --reporter=verbose` | PASS; the named permanent config regression ran, 1 file, 1/1 test.                                                                          |
| `bun x nx run @semio-tech/ui-react:lint --skip-nx-cache`                                                                                      | PASS; target executed without cache.                                                                                                        |
| `bun x nx run @semio-tech/ui-react:typecheck --skip-nx-cache`                                                                                 | PASS.                                                                                                                                       |
| post-removal active ESLint `--print-config` probe                                                                                             | PASS; no globals property, no `no-undef` property, zero predefined names.                                                                   |
| `bun ./📜️script.ts verify dependencies` after frozen install                                                                                  | PASS; baseline 238, current 136, 102 removed, no new third-party identities.                                                                |
| `bun ./📜️script.ts verify dependencies list js`                                                                                               | PASS; **73 JavaScript** identities and no direct `globals` identity.                                                                        |
| `bun ./📜️script.ts verify dependencies list rust`                                                                                             | PASS; unchanged **63 Rust** identities.                                                                                                     |
| `bun ./📜️script.ts verify dependencies parity js` after frozen install                                                                        | PASS; 83 manifests, 258 external rows, 109 evidenced, 149 unowned, 0 undeclared imports, 44 lock workspaces, 0 lock mismatches, 5 fixtures. |
| non-Compose/non-ticket scan for `from "globals"`, `globals.browser`, and `globals.node`                                                       | PASS by absence; exit 0 from the negative assertion with no matches.                                                                        |
| non-Compose/non-ticket `package.json` scan for a `"globals"` key                                                                              | PASS by absence; exit 0 from the negative assertion with no matches.                                                                        |
| `bun x prettier --check .storybook/🟦️lint-tooling.ts package.json bun.lock <UI React Vitest config>`                                          | Not an applicable aggregate gate: Prettier exits 2 because it cannot infer a parser for `bun.lock`.                                         |
| `bun x prettier --check .storybook/🟦️lint-tooling.ts package.json <UI React Vitest config>`                                                   | PASS; every parseable modified file matches Prettier. `bun.lock` was generated by Bun and frozen-lock validated.                            |
| `git diff --check -- .storybook/🟦️lint-tooling.ts package.json bun.lock <UI React Vitest config>`                                             | PASS.                                                                                                                                       |
| `git diff --check`                                                                                                                            | PASS across the unstaged concurrently dirty worktree at the implementation gate.                                                            |
| `git diff --cached --check -- <four production/config files>` after concurrent staging                                                        | PASS for the complete scoped staged implementation.                                                                                         |
| `git diff --cached --check` after concurrent staging                                                                                          | Unrelated global audit residual: other packets' pre-existing ticket Markdown has trailing-whitespace/new-blank-line findings.               |

## Corrected Lock Boundary

The accepted scout expected `globals@16.5.0` to remain transitively for ESLint. That expectation came from stale installed-package evidence and does not describe the reconciled lock:

- the live `bun.lock` resolves `eslint@10.8.0`;
- that lock entry's runtime dependency map contains no `globals` edge;
- the installed ESLint manifest mentions `globals` only in `devDependencies`, which is not installed transitively for a package consumer;
- `bun pm why globals` reports no package matching `globals` in the lock after reconciliation;
- `bun install` therefore correctly removed both the root workspace row and the now-orphaned `globals@16.5.0` resolution;
- the subsequent frozen install and JavaScript parity gate both pass.

The packet-local working-tree `bun.lock` delta against the pre-packet index contains exactly those two removals: the root workspace dependency row and the orphaned package resolution. No unrelated lock row changed in this packet.

## Dependency Boundary and Residuals

The provisional accepted direct boundary is now **136 third-party identities = 73 JavaScript + 63 Rust**. This packet claims only the one-identity `globals` reduction from 137 and does not attribute or accept concurrent dependency work.

The shared worktree and index were already broadly dirty, including staged UI React, Rust UI, and `bun.lock` work. Those unrelated changes were preserved. The source/manifest absence scans exclude the governing plan's Compose and ticket-material boundaries.

The packet-scoped working and staged diff checks are clean. The later global staged check is not clean because unrelated ticket Markdown already staged by other packets contains trailing whitespace; those files were outside authorization and were not edited here. This is an audit residual, not a `globals` implementation failure.

Residual contractual facts:

- active lint behavior still depends on the existing TypeScript parser and Storybook rule implementations;
- `no-undef` and predefined ambient-name maps remain intentionally absent;
- the permanent quick test will fail if either assumption is reintroduced;
- Phase 10 and the zero-dependency end state remain open;
- independent audit and coordinator acceptance remain pending.
