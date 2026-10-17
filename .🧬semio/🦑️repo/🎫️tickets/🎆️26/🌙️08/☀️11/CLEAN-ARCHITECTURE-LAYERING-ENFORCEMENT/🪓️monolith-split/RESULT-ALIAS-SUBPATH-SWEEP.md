# Result: Alias Subpath Sweep (slice ALIAS-SUBPATH-SWEEP, prefix `ass`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 21:59 to 22:30. Tools: `ass-tools/` (this folder `F`). Logs and raw outputs: `🗑️generated/ass-*` (nothing deleted). No `[DEBUG]` log was added.

STATUS: COMPLETE. Every row below was run.

## 0. Verdict

- Root cause as briefed: a Vite string alias `find: "@semio-tech/x"` substitutes by prefix at a `/` boundary, so `@semio-tech/x/sub` became `<file>/sub` (ENOTDIR, "Failed to resolve import"). Every `@semio-tech/<package>` alias in 28 configs (and the Storybook record) is now an exact-match regex (`/^@semio-tech\/x$/u`) or, for the one config that already had a helper, its `exactPackage(...)`. Subpaths resolve through each package's own `exports` map.
- **Shared builder needs no fix.** `createWorkspaceViteResolveConfig` (`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🏗️builder/🌐️vite/🟦️.ts`) only forwards `extraAliases`; its other alias source, `playgroundSceneHostResolveAliases`, is already `/^@react-three\/fiber$/` and `/^@react-three\/drei$/`. No `📚️library` file was edited for aliases.
- **Load matrix** (24 vitest routes, 435 test files): files that load 327 to **402**; tests collected 6,275 to 8,504; passing 6,101 to 8,329; failing 173 to 173 (identical per suite). 17 of 24 suites now load completely (before: 10). Regressions: 0 (section 3).
- **Vite resolve of subpath samples** (28 alias tables, 374 specifiers, real Vite plugin container): bad (unresolved or wrong file) **227 to 0** (section 4).
- **quiz-react stylesheet:** already solved by the `aje` facade; verified with a real Vite SSR load that `@semio-tech/quiz-react/stylesheet` returns the full 36,350-character stylesheet, byte-equal to the source, not the 25-character `@import` stub (section 5).
- Seven suites still have load failures (cad 9 files, hub-admin 1, hub-tests 4, ui-react 2, mcp 7, renderer-react 9, registry-composition 1). None is an alias problem; causes in section 3.2.

## 1. Inventory

Method: `git -c core.quotepath=false ls-files` plus untracked, `git grep` (no recursive find). Candidates: files named `🎚️config/🟦️.ts`, `🌐️vite/🟦️.ts`, `.storybook/main.ts`, any `vite.config.*` and `vitest.config.*` (only ticket scratch configs exist, ignored), plus every file with `alias` and `@semio-tech/` (186 files; the rest were SQL snapshot tests and other uses of the word). `ass-tools/exports.mjs` read the `exports` of each aliased package (workspace links in `node_modules/@semio-tech`). Packages with subpaths: presentation-react 1, ui-react 11, ui-styling 12, assets 14, framework-os 60, framework 254, quiz-react 2, pets 6, pets-react 2, framework-async 5, framework-actor 13, framework-replication 7, framework-os-mcp 7, framework-3d-js 1, framework-2d-js 16, flow-core 2, wasm `*-rs` packages 1 each. Without subpaths: presentation, canvas-react-renderer, infinite-world-r3f, framework-renderer-react, framework-renderer-wgpu, framework-server, quiz, framework-os-shell, machine.

Rule applied: every string alias of an `@semio-tech/<package>` or `@semio-tech/<package>/<exact subpath>` becomes exact-match, including packages without subpaths (one style, so the next codemod cannot break them). Intentionally kept as prefix strings (they stand for a whole tree or are not packages): the wasm engine trees (`packageTree(...)` in the renderer config, `framework-surface-board-2d-rs` in the os dev builder, `framework-os-node-graph-rs` in the registry composition config), the file alias `@semio-tech/flow-core/🌐️flow-browser.js`, the route strings (`MODULE_PLUGIN_ROUTE`, `/renderer-modules/wgpu`) and the virtual session alias.

| # | Config (short) | Kind | Aliases made exact |
|---|---|---|---:|
| 1 | `🧰️framework/🧪️tests/🎚️config` | vitest | 1 (framework, object turned into array) |
| 2 | framework `⏳️async` tests | vitest | 1 |
| 3 | framework `🎭️actor` tests | vitest | 1 |
| 4 | framework `📡️replication` tests | vitest | 1 |
| 5 | framework `🖱️ui/🎨️styling` tests | vitest | 1 |
| 6 | framework `🖱️ui/🎯️targets/⚛️react` tests | vitest | 1 |
| 7 | framework `🖼️canvas/🎨️react-renderer` tests | vitest | 1 |
| 8 | os `🌉️mcp` tests | vitest | 1 |
| 9 | os tests | vitest | 1 |
| 10 | os `♾️infinite/🌍️world/🎨️r3f` tests | vitest | 2 |
| 11 | os renderer `⚛️react` tests (rhu3 config) | vitest | 2 (`ui-react/test`, `ui-styling`, with its own `exactPackage`) |
| 12 | `🐾️pets` tests | vitest | 1 |
| 13 | `❓️quiz/🎯️targets/⚛️react` tests | vitest | 8 |
| 14 | `🎤️presentation/🎯️targets/⚛️react` tests | vitest | 5 |
| 15 | `❓️quiz` tests, `🎤️presentation` tests, `🖥️server` tests, os `🖥️shell` tests | vitest | 1 each (self aliases of packages without subpaths) |
| 16 | `✏️s/🧑‍💻dev/📐️cad` tests | vitest | 1 (`framework-3d-js`, passed to `createWorkspaceViteResolveConfig`) |
| 17 | `✏️s/🔌️plugins/🎞️animate` tests | vitest | 1 |
| 18 | `🌎️hub` tests, `🌎️hub/🛡️admin` tests | vitest | 1, 3 |
| 19 | mit-bestand `33.projektetage` tests | vitest | 4 (+ section 6) |
| 20 | mit-bestand `33.projektetage` builder | vite | 4 |
| 21 | mit-bestand demonstrator builder | vite | 10 |
| 22 | `🌎️hub/🛡️admin` builder | vite | 3 |
| 23 | `🏢️semio-tech/🎡️play` builder | vite | 9 |
| 24 | os `🧑‍💻dev` builder | vite | 10 |
| 25 | `.storybook/main.ts` | storybook | whole record, via `buildExactAliasEntries` (section 5) |

Already exact before this slice: `🎓️teaching/🏛️architecture/❓️quiz` builder and tests, `🐾️pets` react builder and tests, the `@semio-tech/framework` entries fixed by `sdkb` (four builders), quiz-react's own entry (`aje`). Not touched: `machine`, `3d`, `2d` (empty alias tables), kernel, wgpu, store, registry, window-kits, dev tests (no `@semio-tech` alias).

72 string-to-regex rows in `ass-tools/apply.py` (23 files) plus 5 rows in `ass-tools/apply2.py` (5 files). Every anchor was verified unique before the write (`assert count == 1`), each file re-read right before its write, pre-edit text kept in `🗑️generated/ass-snap/` (`index.json` maps path to snapshot). Two hand edits followed (renderer config `exactPackage`, section 6).

## 2. Why not just remove the aliases

Self aliases (`@semio-tech/framework-actor` to the in-source module) keep the module under test and the module imported by name the same instance; the page aliases (`ui-react` to `🟦️.tsx`) pin the source entry the dev server serves. Exact-match keeps all of that and only stops the prefix swallowing. `ui-react/test` and `ui-react/runtime` stay as exact pins: they point to `🖌️render/🟦️.ts` and `🎠️runtime/🟦️.ts`, not to what `exports` gives (`🟦️.ts` and `🟦️.tsx` facades), so removing them would change the module graph.

## 3. Load matrix (real vitest runs)

Method: `ass-tools/vt.sh <phase> <suite>` runs `node vitest.mjs run --config <owner config> --reporter=json` from the owner package directory with `SEMIO_TEST_LEVEL=long` and an explicit `SEMIO_VITEST_POLICY` (the configs refuse to load without it; its `cwd` must equal the owner package). `vsum.mjs` and `vcmp.mjs` classify a file as not loaded when its result is `failed` with zero test rows. Base = before my first edit (21:59 to 22:08), after = final tree. The five self-alias suites at the bottom were run before through a temporary sibling copy of the snapshot (`🟦️.ass-before.ts`, deleted at once, verified absent from `git status`). Raw reports: `🗑️generated/ass-vt-{base,after}-<suite>.json`, summaries `ass-sum-{base,after}.json`, comparison `ass-vcmp-final.txt`.

### 3.1 Matrix

| Suite | Files | Loaded before | Loaded after | Tests before to after | Passing before to after | Failing before to after |
|---|---:|---:|---:|---|---|---|
| projektetage | 2 | 0 | **2** | 0 to 35 | 0 to 35 | 0 to 0 |
| cad | 13 | 4 | 4 | 12 to 12 | 10 to 10 | 2 to 2 |
| hub-admin | 1 | 0 | 0 | 0 | 0 | 0 |
| hub-tests | 12 | 6 | 8 | 59 to 67 | 49 to 56 | 9 to 9 |
| async | 2 | 2 | 2 | 27 | 27 | 0 |
| actor | 10 | 9 | **10** | 94 to 104 | 94 to 104 | 0 |
| replication | 2 | 2 | 2 | 21 | 21 | 0 |
| ui-styling | 3 | 3 | 3 | 82 | 80 | 2 to 2 |
| ui-react | 45 | 14 | **43** | 299 to 1,166 | 299 to 1,166 | 0 |
| canvas-react-renderer | 1 | 0 | **1** | 0 to 10 | 0 to 10 | 0 |
| quiz-react | 31 | 0 | **31** | 0 to 926 | 0 to 926 | 0 |
| presentation-react | 7 | 1 | **7** | 4 to 140 | 4 to 140 | 0 |
| pets | 26 | 26 | 26 | 1,133 | 1,133 | 0 |
| os | 23 | 23 | 23 | 609 | 499 | 110 to 110 |
| mcp | 10 | 3 | 3 | 28 | 27 | 1 to 1 |
| framework | 6 | 4 | **6** | 172 to 306 | 172 to 306 | 0 |
| infinite-world-r3f | 1 | 0 | **1** | 0 to 103 | 0 to 103 | 0 |
| renderer-react | 215 | 206 | 206 | 3,043 | 3,006 | 37 to 37 |
| registry-composition | 8 | 7 | 7 | 159 | 148 | 11 to 11 |
| server | 1 | 1 | 1 | 18 | 18 | 0 |
| shell | 2 | 2 | 2 | 7 | 6 | 1 to 1 |
| quiz | 11 | 11 | 11 | 451 | 451 | 0 |
| presentation | 2 | 2 | 2 | 56 | 56 | 0 |
| animate | 1 | 1 | 1 | 1 | 1 | 0 |
| **Total** | **435** | **327** | **402** | **6,275 to 8,504** | **6,101 to 8,329** | **173 to 173** |

Suites that load completely after (17): projektetage, async, actor, replication, ui-styling, canvas-react-renderer, quiz-react, presentation-react, pets, os, framework, infinite-world-r3f, server, shell, quiz, presentation, animate. Before (10): async, replication, ui-styling, pets, os, server, shell, quiz, presentation, animate.

Failure sets of suites that loaded before: **identical**. `vcmp.mjs` found 0 files that stopped loading and 0 failing tests in a previously loaded file that did not fail before (a first run flagged one regression, section 6.5, which I fixed and re-ran). The 173 failing tests are the same 173 in both runs, in cad (2), hub-tests (9), ui-styling (2), os (110), mcp (1), renderer-react (37), registry-composition (11), shell (1). 75 files load that did not load before; none of them has a failing test (`vcmp` column `failIn(new)` is 0 everywhere), so the quiz-react route is green at 926 of 926 tests.

### 3.2 Remaining load failures, none alias-caused

From `ass-tools/causes.mjs after`:

| Suite | Files | Cause |
|---|---:|---|
| cad | 9 | workspace package `@semio-tech/cad-js-module-spatial-shape` has no `node_modules/@semio-tech` link (needs `bun install`); `brepjs-opencascade` is not installed |
| hub-admin | 1 | test imports `🌎️hub/🔨️modules/🛡️admin/📦️packages/🧹️environment/🟦️.ts`, which does not exist |
| hub-tests | 4 | stale fixture path `…/📇️directory/🧬️schema/🌱️spa…` (ENOENT), two `paths[0] must be string` in suites that read a missing path, one hub schema module absent from its catalog |
| ui-react | 2 | `🖱️ui/🧪️tests/📐️overlay-flow` has no suite; `Cannot find package 'color'` (undeclared dependency of the ShellScope component test) |
| mcp | 7 | `bun:test` imported under vitest; the `semio-os-mcp` binary gate needs a built binary; two non-suite files in the include list |
| renderer-react | 9 | ENOENT on fixtures that moved under `🧪️tests/🧫️fixtures` (same 9 as the rhu3 baseline) |
| registry-composition | 1 | "Projection test artifact directory required" (needs the generated artifact directory) |

## 4. Real Vite resolve per config

Tool: `ass-tools/resolve-check.mjs`. The builders cannot be loaded as modules (they need staged wasm deliveries and an activation receipt; tried with bun and node, `ass-tools/alias-dump.mjs`, abandoned), so the tool reads the alias objects out of the config source (`{ find: …, replacement: … }` objects and `alias: {…}` blocks), evaluates them in a stub scope that also evaluates the config's top-level `const` lines (so `root`, `repoRoot`, `product` resolve for real), and builds a Vite server (`createServer`, plugin container, `resolveId`) once with the config's aliases and once without any alias. For each aliased `@semio-tech/<package>` it samples the bare specifier and up to 6 `exports` subpaths. Statuses: `ok-node` (alias does not match, node resolution via `exports` finds the file), `ok-bare`, `pinned` (exact-subpath or tree alias on purpose), `ok` (alias and node agree), `UNRESOLVED` (no file; the old prefix alias gives `<file>/sub`), `DIFFERS` (alias file differs from node file without being a pin). "Before" uses the pre-edit text from the snapshot (same location, so relative roots are identical).

| | Before | After |
|---|---:|---:|
| alias tables checked | 28 | 28 |
| specifiers resolved | 374 | 374 |
| UNRESOLVED | 217 | **0** |
| DIFFERS | 10 | **0** |
| ok / ok-node | 8 / 54 | 0 / 279 |
| ok-bare | 80 | 80 |
| pinned | 5 | 15 |

Per config (bad = UNRESOLVED + DIFFERS): framework tests 6 to 0, async 5 to 0, actor 6 to 0, replication 6 to 0, ui-styling 6 to 0, mcp 6 to 0, os 6 to 0, pets 6 to 0, ui-react 6 to 0, canvas 6 to 0, quiz-react 20 to 0, presentation-react 13 to 0, infinite-world-r3f 12 to 0, renderer-react 5 to 0, projektetage tests 2 to 0, projektetage builder 7 to 0, demonstrator builder 22 to 0, cad 1 to 0, hub-admin builder 18 to 0, hub-admin tests 18 to 0, hub tests 6 to 0, play builder 22 to 0, os dev builder 22 to 0, animate 1 to 0; server, shell, quiz, presentation 0 to 0 (no subpaths). Per-spec rows: `🗑️generated/ass-resolve-{base,after}.json`.

Examples (after, real Vite ids): `@semio-tech/framework/manifest` resolves to `🧰️framework/📦️packages/🟦️typescript/🛂️manifest/🟦️.ts` (before: `…/🟦️.ts/manifest`), `@semio-tech/framework-os/modules/directory/io/binary/descriptor-digest` to the framework-os facade (before: `…/🟦️.ts/modules/…`), `@semio-tech/assets/mesh`, `@semio-tech/pets/testing/fixtures/schema-conformance`, `@semio-tech/ui-styling/theme/model`, `@semio-tech/ui-react/i18n` (pinned in quiz-react to its i18n module).

## 5. Storybook and the quiz stylesheet

### 5.1 Storybook

`.storybook/main.ts` merged a plain record into `config.resolve.alias`; a record cannot hold a regex. Fix once: `buildExactAliasEntries(record, existing)` in `.storybook/📖️stories/🧭️coordination/🟦️.ts` turns every key into `^key$` entries and keeps `existing` (array or record) after them; `viteFinal` now ends with `config.resolve.alias = buildExactAliasEntries(aliasRecord, config.resolve.alias)`. The stale comment about prefix substitution in `buildStorybookAliases` was rewritten. New test case in `.storybook/🧪️tests/🧪️scope-resolution/🟦️.ts`: asserts the entries, then runs Vite in a child node process (the ui-react vitest route runs this file in jsdom, where importing `vite` breaks esbuild; I first used a static import and `vcmp` caught that as a regression, section 6.5) and checks that `@semio-tech/assets` resolves to the aliased file and `@semio-tech/assets/mesh` does not start with `<file>/`. Result: 11 of 11 pass (10 before plus the new one), and the ui-react route no longer loses the file.

`.storybook/main.ts` itself cannot be imported at HEAD: it imports `pluginModulesRoot` from `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`, which exports no such name (`builtPluginModulesRootIn` is the only one). Foreign, not fixed. So Storybook was not started. `ass-tools/sb-resolve.mjs` rebuilds its alias record from the same constants and the real scope table (12 keys, 8 of them `@semio-tech`) and resolves 12 specifiers through Vite: **old prefix entries 9 of 12 unresolved, exact entries 0 of 12** (`🗑️generated/ass-sb-resolve.json`).

### 5.2 `quiz-react/🎨️.css`

The state found: `aje` had already replaced the psc3 `?raw` of the package stub by the facade `@semio-tech/quiz-react/stylesheet` (`📦️packages/🟦️typescript/🎨️stylesheet/🟦️.ts` re-exports `../../../🎨️.css?raw`, the real sheet in the target directory). All seven consumers import `quizStylesheet` from it; nobody imports `quiz-react/🎨️.css` with a query any more (`git grep -F`, 0 hits). The package export `./🎨️.css` (the 25-byte `@import "../../🎨️.css"` stub) stays for normal CSS imports, where the `@import` chain is followed. The other two `?raw` imports in `🌗️contrast-states` and `💭️crowd-client` (`🎨️styling/🎨️palette/🎨️.css`, `🖌️ui/🎨️.css`) are relative paths to real files, not stubs. Proof: `ass-tools/css-check.mjs` (Vite SSR load) returns `length 36350, equalsRealCss true, startsWithImport false`; the vitest route agrees (the stylesheet assertions of the 7 consumer suites are among the 926 passing quiz-react tests). Nothing else had to change; the earlier "red at HEAD, 8 of 8" state was the alias problem of section 0.

## 6. Other edits and why

1. **Projektetage tests config** (`♻️mit-bestand/🎤️präsentation/📅️33.projektetage/🧪️tests/🎚️config/🟦️.ts`), three changes beyond the exact aliases, each needed to make the suite load (it had 0 of 2 files loading): `environment: "node"` to `"jsdom"` (the setup file it loads is jsdom polyfills: `window is not defined` in the in-source suite), `server: { fs: { allow: [repoRoot] } }` (the setup file lives in the presentation package outside the project root: "Cannot find module /@fs/…"), and a self alias `@semio-tech/mit-bestand-praesentation-projektetage` to `🟦️.ts` next to the existing `/spec` alias (the package is not linked into `node_modules`; the deck test imports it by name). Result 2 of 2 files, 35 of 35 tests (7 deck + 28 in-source).
2. **Renderer react tests config**: the two entries I converted were rewritten from regex literals to the file's existing `exactPackage(...)` helper, so the file keeps one style.
3. **Capability policy test** (`🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts`): `checkPlaygroundAliasFreshness` scans the os dev builder for `{ find: "…", replacement: path.resolve(repoRoot, "…") }` and checks that every target exists; exact-regex finds would have escaped the scan (sdkb had already dropped one). The pattern now also captures `find: /^…$/u`; it matches 12 entries of the builder (11 exact plus the wasm tree), all existing on disk (verified with the pattern in isolation; the test itself needs cargo and was not run).
4. **Vitest configuration ownership fixture** (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json`, library, parent-held, minimal edit): the gate hashes each owner's `resolve.alias`, so converting aliases changes 20 owner hashes. `ass-tools/ownership-hash.mjs` ports the gate's own `portable`, `behavioralProjection` and `projectionHash`, loads every owner config under a per-owner policy and compares before (snapshot text) and after. Before my edits only 4 of the 20 touched owners matched the fixture (async, actor, presentation, server); the other 16 were already stale (as were 15 of the 26 owners I did not touch; 11 of those match). I updated only those 4 `projectionSha256` values and re-ran the hash: all 4 match again. The gate itself is dead at HEAD for an independent reason: it reads `libraryRoot/🧫️fixtures/…` but the fixture sits under `🧪️tests/🧫️fixtures/…` (ENOENT), so `bun test` of it was not a usable check.
5. **Regression caught and fixed on the way:** my first storybook test used a static `import { createServer } from "vite"`; the ui-react vitest route (jsdom) runs that file and failed on the esbuild invariant, which `vcmp.mjs` reported as the only regression. A `@vitest-environment node` docblock fixed that but broke the ui-react setup file (`Element is not defined`), so the Vite oracle moved to a child process. After the fix the ui-react route has 0 regressions.

## 7. Verification summary

| Check | Result |
|---|---|
| Load matrix, 24 vitest routes, base and after | section 3; 327 to 402 files load, 0 regressions, failing sets identical |
| Real Vite resolve of 374 sampled specifiers over 28 alias tables | 227 bad to 0 |
| Storybook alias record through Vite (12 specifiers) | 9 unresolved to 0 |
| Storybook scope-resolution test | 11 pass (new case included), also loads inside the ui-react route |
| `quiz-react/stylesheet` real SSR load | 36,350 chars, equals the source CSS |
| quiz-react vitest route (red at HEAD) | 31 of 31 files load, 926 of 926 tests pass |
| os dev builder alias freshness pattern | 12 matches, all targets exist |
| Ownership fixture | 4 hashes recomputed; a second run reports all 4 matching |

Not run: the launcher (`bun repo run …`), the Storybook dev server (cannot import at HEAD, section 5.1), the capability-policy test itself (needs cargo), the ownership gate (broken fixture path), the dev servers of the five Vite builders (they need activation and staged wasm; their alias tables were verified through Vite's resolver instead), `bun install` (it would also repair the cad link).

## 8. Foreign findings (not fixed)

1. `.storybook/main.ts` imports a non-existent `pluginModulesRoot` (section 5.1); Storybook cannot start.
2. The ownership gate reads its fixture from the wrong directory and 16 of the 20 alias-bearing owner hashes were stale before this slice (section 6.4).
3. cad: `cad-js-module-spatial-shape` unlinked, `brepjs-opencascade` missing; hub-admin and hub-tests stale paths; mcp `bun:test` under vitest; ui-react `color` undeclared (section 3.2).
4. `registry/composition` config keeps a string alias `@semio-tech/framework-os-node-graph-rs` pointing at the ui-styling builder file (a stub for a wasm tree); it works as a prefix tree stub but the target is odd.
5. The wasm engine aliases (`*-rs` trees) and `flow-core/🌐️flow-browser.js` stay prefix strings on purpose; if one of those packages gains a sub-export that must resolve through `exports`, it needs `packageTree`-style handling per case.
6. Tests in `❓️quiz/🧪️tests` that exercise quiz-react (rhu3/aje finding) still belong in `🎯️targets/⚛️react/🧪️tests`.

## 9. Commands run

| # | Command | Result |
|---|---|---|
| 1 | `git ls-files`/`git grep` inventory, `exports.mjs` | section 1 |
| 2 | `vt.sh base <suite>` for 24 suites (`xargs -P3`), `vsum.mjs base`, `causes.mjs base` | section 3 |
| 3 | `resolve-check.mjs base` (snapshot text) and on the current tree | section 4 |
| 4 | `apply.py --write`, hand edits (renderer helper, capability test, storybook helper, main.ts), `apply2.py --write` | 28 configs |
| 5 | `vt.sh after <suite>` (all, then ui-react twice and projektetage three times), `vsum.mjs after`, `vcmp.mjs`, `causes.mjs after` | section 3 |
| 6 | `node vitest.mjs run --config ass-sb-vitest.config.mjs` (storybook test, 3 iterations) | 11 pass |
| 7 | `sb-resolve.mjs`, `css-check.mjs` | section 5 |
| 8 | `ownership-hash.mjs before`, `after --write`, `after` | section 6.4 |
| 9 | `bun alias-dump.mjs`, `bun sb-alias.mjs` (exploratory, failed on foreign imports; kept unused) | n/a |

## 10. Files changed

- Configs (28): `🧰️framework/🧪️tests/🎚️config/🟦️.ts`; framework modules `⏳️async`, `🎭️actor`, `📡️replication`, `🖱️ui/🎨️styling`, `🖱️ui/🎯️targets/⚛️react`, `🖼️canvas/🎨️react-renderer` (each `🧪️tests/🎚️config/🟦️.ts`); products `💻️os` tests, `🌉️mcp`, `🖥️shell`, `♾️infinite/🌍️world/🎨️r3f`, renderer `⚛️react` tests, `🧑‍💻dev` builder `🌐️vite/🟦️.ts`; `🐾️pets`, `❓️quiz`, `❓️quiz/🎯️targets/⚛️react`, `🎤️presentation`, `🎤️presentation/🎯️targets/⚛️react`, `🖥️server` tests configs; `✏️s/🧑‍💻dev/📐️cad`, `✏️s/🔌️plugins/🎞️animate` tests; `🌎️hub` tests, `🌎️hub/🔨️modules/🛡️admin` tests and builder; `🏢️semio-tech/🎡️play` builder; `♻️mit-bestand` demonstrator builder and `33.projektetage` builder and tests.
- Storybook: `.storybook/main.ts`, `.storybook/📖️stories/🧭️coordination/🟦️.ts`, `.storybook/🧪️tests/🧪️scope-resolution/🟦️.ts`.
- Test and fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🎚️vitest-configuration-ownership/🔣️.json` (4 hashes, library, parent-held).
- Created in `F`: `ass-tools/` (`apply.py`, `apply2.py`, `resolve-check.mjs`, `vt.sh`, `vsum.mjs`, `vcmp.mjs`, `causes.mjs`, `css-check.mjs`, `sb-resolve.mjs`, `ownership-hash.mjs`, `exports.mjs`, `env.sh`, `suites.tsv`, unused `alias-dump.mjs`, `sb-alias.mjs`) and this file. Raw outputs and the snapshot: `🗑️generated/ass-*`.
- Not touched: `AGENTS.md`, `.vscode`, any ticket tool, any git state, `📚️library` code.
