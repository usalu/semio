# Result tsc-stage: staged fixes for the frozen-library type errors (2026-10-20 00:30 to 01:15)

Executor `tsc-stage`. HARD FREEZE respected: nothing under `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/` (or any `package.json`, `bun.lock`, `🔣️taxonomy.json`, root `📜️script.ts`) was edited. Everything is staged in `C/tsc-stage-tools/` and applied by `C/tsc-stage-tools/apply.sh` after the lift. No cargo, no sub-agent, no modifying git command, no dependency sync started. Scratch output under `C/🗑️generated/tsc-stage-*` was deleted at the end.

## Result

| config | before (this run's baseline) | with the staged patches | note |
| --- | ---: | ---: | --- |
| repo (`🦑️repo/tsconfig.json`, 5,709 files) | 48 | **0** (5,716 files) | all 48 listed in `result-tsc5.md` |
| repo library (`📚️library/📦️packages/🟦️typescript/tsconfig.json`) | 48 (same set) | **0** | |
| repo test | 2 (tsc5) | **0** | |
| hub typescript | 2 (tsc5) | **0** | `package-projection` only |
| hub admin | 2 (tsc5) | **0** | |
| os | 3 (tsc5) | **1** | the 2 `package-projection` ones are gone; the last is the stale generated `🌊️flow/🕸️bindings/flow_core.js` (`semioSetRuntimeAssets`), regeneration needs cargo + wasm-bindgen, not stageable |
| laws barrel consumer (`🧪️tests/🧪️diff-only-law-gate/🟦️.ts` through `@semio-tech/repo-lib/laws`, scratch tsconfig) | 8 | **0** | 3 missing exports + 3 cascading implicit-any `record` + 2 `package-projection` |

No new error appeared in any re-run. Not re-run with the patches: plugins (`✏️s/tsconfig.json`: 2 of its 25 are `package-projection`) and root `tsconfig.json` (48 of its 455); same files, same fix.

## Method

`tsc-stage-tools/tscheck.mjs` builds the real program of a tsconfig through the TypeScript API and serves the patched text for the patched paths from an overlay (`stage.py export`), so every file exists once under its real path (a symlink mirror or `paths` swap would load duplicate copies of patched modules). The overlay mechanism is proven by the laws config: unpatched 8 errors, patched 0. Baselines: `tsc -p` via `runs.sh` (tsc5's flags). `patches.json` is the source of truth (path -> ordered anchored ops, each anchor must occur exactly once, already-applied ops are skipped, so apply is idempotent). `staged/NN-name` are the full patched copies (`staged/index.json` maps them to repo paths).

## The patches (22 files, all under `📚️library/`)

Root causes, no casts to `any`, no compat aliases. `expect<string>(x)` and friends are widening type arguments of `expect<T>` (the value stays typed, only the matcher's accepted fixture type is widened to what the JSON fixture really is).

1. `🔍️discovery/🎁️package-projection/🟦️.ts`: the two `Buffer.byteLength(path) > collisionPolicy.maxPathBytes` become `pathBudgetCodes(path, taxonomy.collisionPolicy).length > 0` (tax3 ruling: 256 UTF-16 units per path, 255 UTF-8 bytes per component; `pathBudgetCodes` was already imported for the adapter check). Clears hub, hub admin, repo test, os, plugins, root.
2. `📦️packages/🟦️typescript/⚖️laws/🟦️.ts`: the `diff-only-law` export list gains `policyHostApplyBreaches`, `policyHostApplyFileBreaches`, `policyRowEmitterBreaches`.
3. `⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/📜️script.ts`: `commands` is a `ScriptRouter` built by registering each fixture name (was an object literal).
4. `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts`: `expect<string>(classify(...))` (2 sites).
5. `⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/🟦️.ts`: `Object.freeze<CacheProbeToolV1>({...})`; the inferred overload of `Object.freeze` dropped the contextual type, which caused all 8 implicit-any errors.
6. `⚡️caching/🚀️bootstrap/🛠️tools/🔁️probe/🧪️tests/🟦️.ts`: the JSON probe document is typed as `CacheProbeSpecV1` once (`version: 1` literal), the 3 call sites use it.
7. `⚡️caching/🚀️bootstrap/🛠️tools/🧪️tests/🟦️.ts`: `../📜️script.ts` -> `../🟦️.ts` (5 occurrences, including the `new URL(...)` source reads that would otherwise fail at run time; the module moved).
8. `⚡️caching/🧪️tests/🎚️build-mode/🟦️.ts`: `@semio-tech/nx-bootstrap-tools/script` -> relative `../../🚀️bootstrap/🛠️tools/🟦️.ts` (the package.json export `./script` points at the deleted `📜️script.ts`, see Open).
9. `🏗️build/📜️script.ts`: `run:`/`repair:` of `withStoreReceiptsV1` discard the `0` of `runBazelTasksV1` (`async () => { await ...; }`; the function only ever returns `0`, failures throw).
10. `🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🟦️.ts`: `type BazelTaskV1` imported from `../../📋️task-plan/🟦️.ts` (its owner).
11. `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📥️actions/🟦️.ts`: 5 `expect<...>` widenings, `links: Record<string,string>`.
12. `🏗️build/🔧️tools/🏗️bazel/🧪️tests/📦️snapshot/🟦️.ts`: dropped `version` and `runtime` from the 3 `captureBazelSnapshotV1` contexts (the context is `{workspace, task}`; the implementation reads nothing else).
13. `🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/🟦️.ts`: 3 `expect<unknown>` widenings.
14. `🏗️build/🛠️cmake/🧪️tests/🟦️.ts`: `dotenv` is declared nowhere and not installed; the independent oracle is now `parseEnv` from `node:util` (system library, dotenv format). Ran: `bun test ./…/🛠️cmake/🧪️tests/🟦️.ts -t "compiler environment vectors"` on the patched copy: 1 pass, 4 vectors, 7 expects.
15. `🏗️build/🧪️tests/🕸️graph/🟦️.ts`: `targets:{[owner.name==="fixture-app"?"test":"build"]:{}}` (the conditional of two object literals was not assignable to `Record<string, ...>`).
16. `🏗️build/🧭️invocation/🧪️tests/🟦️.ts`: `expect<string|null|undefined>` on the build mode.
17. `🏗️build/🧾️store-receipt/🧪️tests/🟦️.ts`: 3 widenings.
18. `🧪️tests/🎛️dashboard-control/🟦️.ts`: 4 widenings (`args` as `readonly string[]|undefined`, the others `string`).
19. `🧪️tests/🥾️cross-platform-bootstrap/🟦️.ts`: the local `Fixture` type was missing `nativePins` and `windowsLongPaths`, which the fixture JSON has and the test reads (3 errors with one cause).
20. `🧪️tests/🦑️repo-source-ownership/🟦️.ts`, 21. `🧪️tests/🧮️path-budget/🟦️.ts`: one widening each.
22. `⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: removes the `[DEBUG] provider mismatch` `console.error` deps4 left at the provider-instance throw (the `if` now reads `if(...)throw Error("Same canonical Native and prospective Source provider instance required");`) and the `{ownerRoot,memberRoot}=relation` destructuring that only fed it. Apply only after the sync exits (`apply.sh` refuses while `bun repo dependencies sync` / `deps4-*.sh` runs).

Run on patched copies (scratch mirror outside the repo): `🧮️path-budget` 45 pass, `🔁️probe` tests 3 pass, `🧾️store-receipt` 13 pass / 1 fail; the failing test ("every declared receipt output ... publishes through a recording publisher", demonstrator `📋️project.json` build) fails identically on the unpatched tree (pre-existing, unrelated).

## Overlaps and apply order

- `walkperf` (`C/walkperf-tools/apply.sh`) patches `📦️dependencies/💾️storage/🟦️.ts` (my item 22), `🏗️build/🚀️bootstrap/🟦️.ts`, the workspaces cjs/ts. Checked by applying both op lists in both orders on the current storage file: identical result, no `[DEBUG]` left. Either order works; recommended: item 22 first (removes the debug), then walkperf.
- `tax-stage` (`C/tax-stage-tools/library-43.txt`, restructure of 43 library `📜️script.ts` into a `📜️script.ts` router skeleton + sibling `🟦️.ts` owner module) lists two files I patch: `🏗️build/📜️script.ts` (item 9) and `⚡️caching/📦️artifacts/🟦️typescript/🧪️tests/🧭️commands/📜️script.ts` (item 3). My ops are small character-level anchored hunks. If tax-stage applies first and moves the text into the sibling `🟦️.ts`, `apply.sh` retargets any `📜️script.ts` patch whose anchors are gone to `<dir>/🟦️.ts` automatically (`apply.sh check` prints `(retargeted to 🟦️.ts)` for such a file). Recommended order: tax-stage restructure, then this apply; run `apply.sh check` before `apply.sh apply`, and re-run `tscheck`/the repo config after.
- `tax3` `apply-1..4`: touches `mutation-projection`, `taxonomy`, `taxonomy-contracts`, generators, `plan-contracts`, `fixed-contracts`, `inventory-planning`, `repository-scan`, `package-source-classification`, `path-emoji-statutes` and new files; no file in common with these 22. `package-projection` is only imported by a new tax3 module, not edited. Order irrelevant.
- Other staged work I found referencing these paths (lay15 narrow export keys, fsevents open-partitions dependencies, bzl-breadth adapter follow-ups) are not staged as patches yet; re-run `apply.sh check` after them.

Apply: `C/tsc-stage-tools/apply.sh apply --dry-run --ignore-sync` (dry run), then `C/tsc-stage-tools/apply.sh apply` once the freeze is lifted and the sync has exited; afterwards `tsc -p 🧰️framework/🛍️products/🦑️repo/tsconfig.json --noEmit` should print 0 errors (the dry run was verified against the current tree: 22 files "would patch", 1 base drift handled by the anchors).

## Open (not stageable)

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/package.json` exports `"./script": "./📜️script.ts"` but that file no longer exists (body is `🟦️.ts`); package.json is the deps owner's: change the export (or drop it) and the sibling `"./bazel/script"`, `"./wasm/script"` entries need the same check after tax-stage. Importers found for `nx-bootstrap-tools/script`: only the build-mode test (fixed relatively, item 8).
- os: the stale `flow_core.js` output (needs cargo + wasm-bindgen); not in this task.
- Other agents' `[DEBUG]` logs still sit in tests (`🏗️build/🛠️cmake/🧪️tests/🟦️.ts` `verified N shared compiler environment vectors`, `🏗️build/🧭️invocation/🧪️tests/🟦️.ts` `neutral E2E`); not mine, not removed.
- Not run: plugins and root configs with the overlay; bun tests other than the four listed.

## Verification commands

- Baselines: `C/tsc-stage-tools/runs.sh <tsconfig> <tag>` (`node node_modules/typescript/bin/tsc -p … --noEmit --incremental --pretty false --extendedDiagnostics`): repo 48 errors (5,709 files, 367 s), repo library 48 (5,029 files).
- Patched: `C/tsc-stage-tools/runcheck.sh <tsconfig> <tag>` (`node tscheck.mjs <tsconfig> <overlay>`): repo 0 (5,716 files, 585 s), repo library 0, repo test 0, hub 0, hub admin 0, os 1, laws consumer 0 (scratch tsconfig, 445 files).
- ENFILE hit the shell once during the runs (peer processes); the hub admin result of that moment (TS2318 global types) was discarded and re-run, the numbers above come from clean runs.
