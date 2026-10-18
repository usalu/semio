# Result demo: stale references of the mit-bestand demonstrator (static, no builds)

Executor `demo`, 2026-10-18. Source: `audit-mit-bestand-demonstrator.md` (S1, S3, S4, S5, S10, S11).

## Finding on the launch "generator"

There is no live generator for `.vscode/launch.json`. The seed `.vscode/🧩️launch.seed.jsonc` holds the authored entries plus 89 inert placeholder strings (`"@generated:<variant>:<react|wgpu>"`, `devLaunchers`, `projectLaunchers`). The expander that turned these into the 288+ playground entries lived in `…/🔌️plugin/📇️registry/🚀️launch/🟦️.ts` and was deleted in commit `2604f70cac1`. The only existing `launch` generator (`dashboard … launch generate <ticket>`) projects ticket commands only. Both files are therefore hand-maintained, and I edited both consistently with one text-level rewrite (`demo-rewrite-launch.ts`, same folder as this report, kept as input). Line formatting, entry order, names and groups are untouched; nothing else in the 9 MB file changed (diff = command lines, 4 inserted env lines, 213 mode values, renamed port keys).

## 1. `workspace:dev -- <variant>` (S1)

Replacement route (inferred targets of `@semio-tech/framework-os-dev`, `🏗️build/🕸️graph/📥️inference/🟨️.mjs` L1453-1457 `dev-<variant>-react-<profile>` and L1461 `dev-<variant>-wgpu-<profile>`; project attachment L1645/L1771):

`bun repo run @semio-tech/framework-os-dev:dev-<variant>-<react|wgpu>-prototype --skip-cache --configuration=prototype`

(the same flag shape as the existing `serve-…-react-prototype` / `activate-…-react-prototype` entries). The renderer comes from the entry's own `SEMIO_RENDERER`.

- launch.json: 335 `workspace:dev -- …` entries rewritten (330 playground, 5 mcp). seed: 21 (16 playground, 5 mcp). Remaining `workspace:dev -- ` occurrences: 0 in both.
- Aliases resolved through the Cargo `aliases` rows: `3d`->puzzle3d, `5d`->puzzle5d, `procedural 3d`->generation3d. Extra words `fixture concrete|base-icon` and `served` have no consumer in the new route and were dropped.
- Entries that still used the dead `CAD_JS_RENDERER_PLAY_PORT`, `PUZZLE_3D_PLAY_PORT`, `PUZZLE_5D_PLAY_PORT`, `SHOOTING_PLAY_PORT` now carry `S_OS_PORT` (what `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` L176 reads); otherwise the server would have listened on 6066 while `serverReadyAction` waits on 6013 etc.
- puzzle3d/puzzle5d "concrete forest" entries gained `SEMIO_DEFAULT_EXAMPLE=concrete-forest` (example folder `concrete-forest` exists under both artifacts; mechanism = the existing capsule-dream entries). cad and shooting "fixture" entries got no example id (no matching example folder found).
- mcp family (the `dev` command still exists for these, but `workspace:dev` is not a root target):
  - `mcp` -> `workspace:dev-mcp`; `mcp repo` -> `workspace:dev-mcp-repo` (both root targets).
  - `mcp stdio cursor|os` -> `workspace:dev-mcp -- stdio cursor|os` (`DevScript.runMcp` handles `stdio`).
  - `mcp http os` -> `@semio-tech/framework-os-mcp-rs:dev` (DevScript rejects `http`; its own docstring names this target; target exists, `continuous`).

## 2. Unsuffixed `prepare-<variant>-react` (S3)

`♻️mit-bestand/🧺️demonstrator/📋️project.json`:
- `build.dependsOn[0]`: `prepare` -> `prepare-release` (the site is a release product: `pipeline.site.profile = release`).
- Removed the stale `prepare` target. It was an unsuffixed duplicate of `prepare-release` with non-existent deps; nothing else referenced it.
- Schema-first test contract: `🔨️modules/🧩️runtime/🧬️schema/🔣️.json` + `🧫️pipeline.json` gained required `wasmTargets` (the four `:wasm` deps that `prepare-<profile>`/`build` already declared and the test did not know). `🧪️tests/🟦️.ts` now asserts `build.dependsOn == ["prepare-release", ...wasmTargets]`, no `prepare` target, and `prepare-<profile>.dependsOn == [variant lanes…, ...wasmTargets]` (these two assertions were already out of sync with the project file).
- Bonus (S10): `pipeline.site.output` `dist/site` -> `dist/release/site` (schema const + json; unused by code, matches `build.outputs`).

## 3. Runtime hardcoded `dist/…` paths (S4, S5)

- `🔨️modules/🧩️runtime/📜️script.ts` `PreparationScript`: module root now `builtPluginModulesRootIn(repoRoot, mode)`, sessions via `playgroundSessionsRootV1` + `playgroundSessionStagedOutputPath(sessionsRoot, variant)` (all from `@semio-tech/framework-os-dev/activation`, i.e. `<cache>/products/browser/<mode>/{modules,sessions}`; the `session-<variant>` target output `.cache/b/p/m/browser/{mode}/sessions/<variant>` matches). Removed the unused `plugin` path.
- Same stale root was also in `🔨️modules/🧩️runtime/📦️assets/🟦️.ts` (`demonstratorRuntimeAssetSources`): now `builtPluginModulesRootIn(workspace, profile)`.

## 4. `SEMIO_BUILD_MODE=ship` (S11)

`ship` is rejected by `readBuildMode` (`prototype|release` only; repo tests list `dev`/`ship` as rejected legacy values). It occurred in 213 entries (engine-producers gate included) in each of launch.json and the seed; all are `ship` -> `release` (old `ship` = optimized shipping build; none of them passes `--configuration`, so no conflict). The 42/26 `null` values are intentional env-unsets and untouched.

## Verification (static only; no builds, no cargo, no nx)

- `python3` JSON(C) parse of `.vscode/launch.json` (13588 configurations) and the seed: OK. Zero `workspace:dev -- ` and zero `"ship"` left in both.
- Every rewritten `…:dev-<variant>-<react|wgpu>-prototype`: variant checked against the playground rows parsed from all 44 hub/stdio Cargo.toml (`bun` + `Bun.TOML`, 156 variants): 0 unknown (330 + 16 entries). Target name pattern read from the inference code (see above).
- `workspace:dev-mcp`, `dev-mcp-repo`, `@semio-tech/framework-os-mcp-rs:dev`: present in the project.json files.
- Demonstrator `project.json`, `🧫️pipeline.json`, schema: JSON valid; all local `dependsOn` names resolve; all 9 pipeline variants are playground rows. `bun build --no-bundle` transpile of `📜️script.ts`, `📦️assets/🟦️.ts`, `🧪️tests/🟦️.ts`: OK.
- Not run: `bun repo run …` of any rewritten target, the runtime catalog test, a real prepare/serve (per rules).

## Files changed

- `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`
- `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/{📜️script.ts, 📦️assets/🟦️.ts, 🧪️tests/🟦️.ts, 🧫️pipeline.json, 🧬️schema/🔣️.json}`
- new in ticket: `demo-rewrite-launch.ts` (one-shot rewrite tool, input script)

## Still open

- `workspace:dev-mcp-engine` (launch.json "🛠️dev-mcp-engine"): no such root target and no `engine` route in `DevScript`; no successor identified.
- The seed still carries 89 `@generated:*` placeholders and `devLaunchers`/`projectLaunchers` metadata with no expander; whoever restores a generator must emit the new route (or drop the metadata).
- Stale `workspace:dev` references outside the launch files: dashboard oracle `🎛️dashboard/🧪️tests/🗺️coverage/🔮️oracle/🟦️.ts` L261, its registry fixtures, README L135, and the dev error text (`bun run dashboard run playground:<variant>`) in `🛠️dev/🟦️.ts`. Root `📋️project.json` L109/L338 contain a `"ship"` option id (not checked).
- Fixture selection ("fixture concrete|base-icon" for cad/shooting) has no new equivalent.
- S6-S8, S12-S14 (memory notes, cosmetic names/headers) untouched; `e2e.owner` in `🧫️pipeline.json` is an id, not a target, left as is.
- `build` has no name-fixed build mode (`fixedTargetBuildMode` only knows `*-release`); `🧫️invocations.json` expects release. Not changed here.
