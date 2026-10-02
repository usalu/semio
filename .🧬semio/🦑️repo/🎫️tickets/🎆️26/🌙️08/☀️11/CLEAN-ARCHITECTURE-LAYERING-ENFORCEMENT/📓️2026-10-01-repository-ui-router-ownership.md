# Repository UI Router Ownership

The two generic React router imports into Repo are physically removed. Workspace UI primitive and chrome i18n censuses, the original root Storybook composition, and their dependency policy now belong to `framework/products/repo/library/🖱️ui`. The generic router retains exactly lint, test, typecheck, and canonical architecture, using neutral owned execution, routing, budget and explicit Vitest policy interfaces.

The new taxonomy kind is `repo-library-ui`, with the existing `repo-server-library` parent. There is one physical implementation of each moved census and Storybook handler. All 25 original census declarations retain their exact TypeScript AST text, including existing roots, allowlists, refusal cases and source scanning rules. The private normalized path join preserves these comparisons on Windows. Both live censuses explicitly author `cache: false`; their inputs include the actual workspace or product source closure they inspect.

## Registered Routes

| Former generic route | Current concrete RepoLibrary target | Seed order |
| --- | --- | --- |
| Ownership proof | `@semio-tech/repo-lib:test-ui-router-ownership` | 900.05725 |
| `check-ui-primitives` | `@semio-tech/repo-lib:check-ui-primitives` | 900.05726 |
| `check-chrome-i18n` | `@semio-tech/repo-lib:check-chrome-i18n` | 900.05727 |
| `dev` | `@semio-tech/repo-lib:ui-storybook-dev` | 900.05728 |
| `build` | `@semio-tech/repo-lib:ui-storybook-build` | 900.05729 |

The four former generic project targets are deleted. The root chrome caller directly invokes the specific RepoLibrary target. The launch seed names and commands reference the concrete targets; generated launch files remain owned by final integration.

## Actual Evidence

All logs are in this ticket's `🗑️generated/goal-strict-boundary` directory. Commands used the existing `bun run nx run <target>` route.

| Session | Actual result | Log |
| --- | --- | --- |
| 12380 | Harness RED: esbuild refused the Unicode regex flag; corrected the independent loader's filter. | `ui-router-ownership-red.log` |
| 75567 | Producer RED before relocation: 1 pass, 3 fail; the independent loader refused the actual generic router's Repo barrel and Repo entrypoint imports. | `ui-router-ownership-red-2.log` |
| 56205 | 3 pass, 1 fail: Node exposed bundled `import.meta.url` resource relocation. The loader now preserves each actual module's URL/directory through TypeScript AST projection. | `ui-router-ownership-current.log` |
| 46835 | GREEN, 4 laws / 79 assertions, 937 ms; native Node loaded the actual generic router and its four commands. Temporary `[DEBUG]` observation recorded and then removed. | `ui-router-ownership-native-current.log` |
| 82027 | GREEN, 5 laws / 92 assertions, 321 ms. Added original Storybook handlers' actual argument, root and environment execution proof. | `ui-router-ownership-final.log` |
| 44681 | RED for missing explicit live census cache refusal; all other laws passed. | `ui-router-ownership-live-census-red.log` |
| 17639 | Final GREEN, 5 laws / 94 assertions, 360 ms; Nx 830 ms, zero cache hits. | `ui-router-ownership-final-live.log` |
| 66806 | Chrome census GREEN, three original allowlisted files, Nx 766 ms. | `ui-check-chrome-i18n-current.log` |
| 90330 | UI primitive census RED, 75 actual findings, Nx 28.4 seconds; includes three stale original allowlist entries. All remain visible without additions to allowlists or dropped checks. | `ui-check-ui-primitives-current.log` |
| 8901 | Retained generic canonical architecture GREEN, two tests, Nx 1.8 seconds. | `ui-retained-canonical-current.log` |

The language-neutral JSON corpus and strict Draft-07 schema describe current owners, four retained routes, four moved routes, the five product areas, original census compiler projections and two Storybook command/environment cases. Installed Ajv validates the schema, installed TypeScript validates exact declarations and executes unchanged Storybook method bodies, and installed esbuild bundles the actual generic router with a refusal plugin for every product area and the real `@repo-lib` alias. Native Node executes that bundle and reports the expected unknown command plus all four retained routes. Injected imports into all five product areas and the Repo alias are independently refused. Installed module targets remain neutral.

The full Storybook server and production build were not started. Their original composition handlers execute against a recording command port, preserving root command arguments, caller tails, polling values and the explicit UI build scope. Global typecheck, fresh full Rule 39 graph and catalog/launch regeneration remain with root integration; this report does not claim those whole-workspace verdicts. The observed UI primitive census failures are independent existing contract findings, not hidden by the ownership extraction.

## File Inventory

Created:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🧫️fixtures/🧭️router-ownership/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🧬️schema/🧭️router-ownership/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🖱️ui/🧪️tests/🧭️router-ownership/🟦️.ts`

Updated, preserving simultaneous unrelated changes:

- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- `📜️script.ts`
- `.vscode/🧩️launch.seed.jsonc`

No AGENTS changes, modifying Git commands, worktrees, runtime dependencies, forwarding APIs, dependency exemptions, policy baselines or timeout changes were introduced.
