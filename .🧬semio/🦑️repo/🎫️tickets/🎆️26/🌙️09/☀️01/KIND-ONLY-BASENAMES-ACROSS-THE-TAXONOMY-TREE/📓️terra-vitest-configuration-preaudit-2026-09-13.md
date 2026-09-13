# Vitest Configuration Consumer Pre-Audit

Date: 2026-09-13  
Status: **accepted for the bounded ownership and native configuration-loading evidence; historical pre-extraction findings and runtime limits are retained below.**

## Current Authority

The live tree currently contains 43 `vitest.config.ts` inputs. They are conventional fixed tool configurations, not anonymous implementation leaves: their ownership must be justified by real runner, tokenizer, schema, editor, and Nx consumers. The root configuration remains deliberately empty and must not become an aggregator.

`runVitest` is the shared executable selector. Its default config is `vitest.config.ts`; package routers may omit the third argument or pass an exact trailing config token. It invokes the workspace-pinned Vitest module under the test budget with the owning bundle root as `cwd`. The configuration-token scanner has independent Bun and TypeScript compiler controls for the explicit, quoted-array, omitted-default, and declaration-default forms. Any move therefore has to preserve both the runner’s default and every exact explicit token, rather than replacing configuration ownership with a broad basename exemption.

The initial 43-config loader baseline was a configuration-load probe only: 42 loaded, and presentation failed because it imported a Vite builder from a styling theme owner. That exact import now points directly to `🎨️styling/🏗️builder/🌐️vite/🟦️.ts`; no compatibility barrel was added. The baseline did not execute suites or establish editor discovery.

## Existing Selector Closure That Must Remain Intact

The retained language-neutral selection fixture gives exact expected identities, not merely glob text:

- Hub Admin: component, Node command, in-source i18n, and DOM-aware setup owners.
- OS MCP: six direct suites plus the in-source helper owner, with only the imported-only standalone helper excluded from direct collection.
- Remodel: three example suites and its schema suite.

The focused native evidence already accepted the MCP eight-unit suite and the registered Admin route at 18/18. Remodel must remain an explicit red limit: its registered route reaches all four configured suites and reports 1,023 passing / 330 failing tests from actual protocol value and serialized-byte differences, not absent path/module resolution. No config or fixture adjustment may hide those failures.

## Required Extraction Closure

Before accepting an owner move, the executor needs an exact owner map for all moved configuration behavior and must preserve:

1. every package router/default or explicit `runVitest` selector and its `cwd`;
2. each config’s real include, includeSource, setup, alias, schema and editor/discovery consumer, with no claim based only on a loaded export;
3. project named inputs and actual registered Nx target inputs for imported owner leaves; and
4. tool configuration grammar: fixed configuration name remains a precise tool contract, while semantic implementations move to anonymous leaves and are imported directly.

The suite should have a language-neutral portable schema/fixture and a native configuration-loader control. A config-load pass must be labeled separately from Vitest collection, test execution, browser/editor integration, and build evidence. The executor must not rerun the broad Admin, MCP, or Remodel native routes simply to demonstrate this closure.

## Boundaries

This preaudit has run no config loader, Nx graph, package command, test suite, browser, editor, or build. It does not treat the historical 43-file count as immutable during concurrent work. The next audit will inspect the current exact owner map and choose a narrow direct consumer control after the move is stable.

## Live-Tree Checkpoint: Incomplete Consumer Closure

Read-only inspection after the first move found the fixture’s 43 declared `ownerPath` values all present and every predecessor `vitest.config.ts` absent. The temporary raw directory scan reported 42 semantic leaves; that scan is not evidence of a missing owner because the fixture’s exact `Bun.file` existence check resolved all 43, including the repository-root `🧪️tests/🎚️config/🟦️.ts` owner.

Acceptance is currently blocked by live consumers rather than owner-file existence:

- `🧰️framework/📦️packages/🦀️rust/📜️script.ts:27` still passes `../🟦️typescript/vitest.config.ts` to `runVitest`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/✅️catalog-verification/🟦️.ts:15` still passes `vitest.config.ts`.
- `.vscode/settings.json:111` still gives the editor the old `**/vitest.config.ts` search pattern.
- No current `📋️project.json` text names an `🧪️tests/🎚️config/🟦️.ts` source input, so cache invalidation cannot yet be attributed to the moved owners.

The new portable test is directionally valuable: it validates Ajv fixture shape, predecessor removal, installed Vite loading, behavioral projection hashes, taxonomy contexts, package/Nx/launch registration, and an editor selector. Its live source check needs a stronger relationship than the current suffix rule: a third `runVitest` string only has to end with `/🧪️tests/🎚️config/🟦️.ts`; it does not resolve the call relative to its bundle and prove membership in the exact 43-owner fixture. The final gate should prove the resolved path is the mapped owner and is included in the applicable Nx inputs.

No direct package or Nx gate was launched at this unstable checkpoint. The executor has the concrete consumer findings and must rebase them before a focused independent run.

## Current Extraction Decision

The executor’s first map resolved each configuration from its effective `test.root`, but root corrected that boundary: `test.root` is behavior data, not configuration source ownership. Each owner instead belongs beneath its enclosing module/plugin/target/root test concern, outside `📦️packages/🟦️typescript`, while preserving the original package root explicitly in configuration data. The currently observed 43 effective roots still need semantic enclosing-root collision review. Because no fixed-name shim is permitted, acceptance must prove every previously defaulted `runVitest` route is rewritten to its exact semantic config path. The shared function may retain a default for unrelated callers only if source-audit evidence proves none of these 43 routes can use it. The earlier 43/43 result in 21.721 seconds used Vite’s bundled default loader, not its native loader: `"native"` was supplied as argument 5, the custom logger position, rather than argument 6. It remains bundled-loader configuration evidence only.

## Current Direct Gate — Pending Test Repair

The current tree now has all 43 anonymous owners, all 43 corresponding project inputs (42 associated project manifests plus the repository-root project), no remaining `vitest.config.ts` source file, the corrected VS Code discovery pattern, and direct framework-Rust and registry router selectors. The current portable source test is materially stronger than the earlier checkpoint: it parses every product TypeScript source with the TypeScript AST, resolves each `runVitest` third argument relative to its bundle root, and requires the resulting path to be an exact fixture `ownerPath`; it also tests the trailing config token with Bun and TypeScript compiler implementations.

I independently ran the ordinary package route `bun ./📜️script.ts test vitest-configuration-ownership` from `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript`. It reached 5 passing and 2 failing tests with 449 expectations in 74.55 seconds. Neither failure identified an owner, consumer, schema, Vite projection, or registration mismatch:

- The deliberately whole-tree AST router scan finished its work after 34.146 seconds but exceeded the test framework's implicit 30-second timeout. It needs a bounded explicit timeout based on the actual full-tree scan, rather than a weakened scan.
- The root no-tests control tests raw source text for the substring `projects:`. The correct in-source explanation says that no `projects` key exists and therefore contains that substring. The test already creates a TypeScript AST; it should assert that the actual `defineConfig` test object has no `projects` property, preserving the explanation.

These are test-level/budget defects. Acceptance remains pending a repaired direct result and the executor's isolated registered Nx result.

## Acceptance

**Accepted for ownership and configuration loading in both bundled and native Vite modes.** The earlier direct failures were repaired at the test boundary: the whole-tree AST source audit has a 60-second bound and the root no-tests assertion uses the parsed configuration property, retaining the accurate source documentation. My subsequent attempted rerun was invalidated by the temporary cross-lane HTML taxonomy contract failure and then exceeded the old 120-second composite wrapper under contention; it is retained as non-product budget provenance only.

The executor's current ordinary package result is **7/7, 450 assertions, 62.59 seconds**. Its earlier isolated registered Nx invocation, `@semio-tech/repo-lib:test-vitest-configuration-ownership --skip-nx-cache`, is **7/7, 450 assertions**, 58.01-second target / 59.0-second Nx run, cache skipped, with private Nx workspace/cache/tmp and daemon/plugins disabled. That run uses Vite’s bundled default loader and remains configuration-loading evidence, not suite, browser, or editor execution.

The loader invocation is now corrected to pass `undefined` for the custom logger and `"native"` as Vite 7’s sixth argument. I independently inspected the live call in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🎚️vitest-configuration-ownership/🟦️.ts:98`: `loadConfigFromFile(..., "silent", undefined, "native")`. The focused corrected native-loader control passed **1/1 with 215 assertions in 95.32 seconds**. The corrected isolated native-loader Nx invocation passed **7/7 with 450 assertions**, with an 83.19-second target and a 1 minute 24 second Nx run, cache skipped. This is registered native Vite configuration-loading evidence; it still does not execute suites, browser/editor behaviour, or a build.

Static review confirms exact direct `runVitest` owner resolution, predecessor removal, current framework-Rust/registry/root/host selectors, all 42 package project inputs plus root, VS Code's `vitest.configSearchPatternInclude`, and schema/fixture/test source-data registration. No broad Vitest suite was used to accept the extraction.
