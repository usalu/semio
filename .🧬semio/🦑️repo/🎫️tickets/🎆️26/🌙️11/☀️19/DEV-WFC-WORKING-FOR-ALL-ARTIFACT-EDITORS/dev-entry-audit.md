# Dev entry audit: WFC playground variants and dev routes

Read-only audit. No repo files were edited and no git commands were run. Paths are relative to `/Users/ueli/Documents/semio` unless absolute. Emoji path segments are the repo's real directory names.

## 1. Where WFC playground variants and ports are declared

**AGENTS.md is wrong.** `✏️s/🔌️plugins/🀄️wfc/AGENTS.md` (Play harness section) says the ports live in `📦️packages/🦀️rust/Cargo.toml` "and nowhere else". That file does not exist: `✏️s/🔌️plugins/🀄️wfc/📦️packages/` contains only `🟦️typescript`.

**Actual declaration.** Playground variants are `[[package.metadata.semio.playground]]` blocks in the hub composition component crate:

- `🌎️hub/🧩️compositions/🀄️wfc/🧩️component/📦️packages/🦀️rust/Cargo.toml`
  - crate `semio-hub-wfc-component`, `deployment-directory = "🀄️wfc"`, `component-kind = "plugin"`, `role = "hub"`
  - all five WFC variants are registered:

| variant | ports (react / wgpu) | app | aliases | line of `ports` |
| --- | --- | --- | --- | --- |
| bitmap | 6041 / 6141 | `s.wfc.bitmap@1/*#editor` | `wfc bitmap` | 25 |
| grid2d | 6042 / 6142 | `s.wfc.grid2d@1/*#editor` | `wfc 2d grid` | 32 |
| wfc2d | 6043 / 6143 | `s.wfc.wfc2d@1/*#editor` | `wfc 2d` | 39 |
| grid3d | 6044 / 6144 | `s.wfc.grid3d@1/*#editor` | `wfc 3d grid` | 46 |
| wfc3d | 6045 / 6145 | `s.wfc.wfc3d@1/*#editor` | `wfc 3d` | 53 |

  Every row has `compositionConfigPath = "✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts"`. The WFC `README.md` table (lines 14-18) matches these ports.

**Puzzle comparison.** `🌎️hub/🧩️compositions/🧩️puzzle/🧩️component/📦️packages/🦀️rust/Cargo.toml` declares puzzle2d/3d/5d the same way (react 6012-6014, wgpu 6112-6114). Puzzle rows also carry `devContribution` and `engines`; WFC rows have neither. Neither plugin's own Cargo.toml (`✏️s/🔌️plugins/🀄️wfc/Cargo.toml`, `✏️s/🔌️plugins/🧩️puzzle/Cargo.toml`) declares playgrounds.

**Port uniqueness.** Ports 6040 (gis), 6041-6045 (wfc) and 6012-6014 (puzzle) appear once each. No collisions found among Cargo.toml `ports = { react = ... }` rows.

**Variant to plugin mapping, and the generated catalog.**

- Parser: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🔎️discovery/🟦️.ts` (`parsePlaygroundBlock`, lines 24-51; discovery of `[[package.metadata.semio.playground]]` at about line 130). It reads each crate manifest.
- Consumer: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts`. `playgroundCatalog` is at lines 68-69. `resolveCatalogFilterPluginId` at lines 79-94 maps a variant to its `pluginId`.
- Generated output: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts` is gitignored (`.gitignore:97`). In this worktree it has `PLAYGROUND_BUILD_TARGETS = []`. `🤖️generated/🚀️playgrounds.json` and `🤖️generated/🎠️playgrounds.json` are also empty lists. Both files have mtime 2026-11-15 02:07.
  - This means the local generated catalog currently has no playground rows, puzzle included. Regenerate (the registry refresh is `ensurePluginRegistry`, reachable as `plugin registry` from `🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:171`) before trusting any variant resolution.
- Catalog fixture: `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests/🧫️fixtures/🔣️.json` lists `wfc` in `installedTargetIds` (line 76) and `wfc2d`, `wfc3d` in the variant list (lines 233-234). It is a test fixture, not the declaration.
- `📋️build-owners.json` maps the WFC TS and Rust packages (`@semio-tech/wfc-*`, `semio-s-artifact-wfc-*`). It has no variant or port data.

## 2. OS dev router `defaultCommand: "dev"`

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`.

- Registered commands (lines 80-175): verify-staged, typecheck, ticket-probe, playground-session, publication-check, publication-worker, source-watch-check, distribution-output-check, prepare, activate, serve, serve-hold, cold-boot-check, canonical-bootstrap-folder-mirror-check, closed-browser-component-factory-check, test, verify, bench, generate, scale-fixture, preview-generated, distribution, canonical-architecture, index-lint, host-handle-lint, channel-version, parity, plugin. There is no `dev`.
- Line 180: `runScriptMain(router,{invocation:original,argv,defaultCommand:"dev"})`.

Behavior, traced through the code:

1. `🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts:21`: with no argv, `argv = ["dev"]`.
2. Line 22: `router.hasCommands()` is true, so there is no usage throw here.
3. Line 23: `router.run(["dev"])`.
4. `🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts:85-90`: `dev` is not in the command map, so it throws `unknown command "dev"; usage: ...`. This happens before `advanceScriptInvocation`, so there are no side effects.

**Verdict: a real but harmless bug.** The default is dead config. A bare invocation fails with a non-zero exit and an error that names a command the user never typed, instead of printing usage. Nothing runs and nothing is corrupted. Fix by dropping `defaultCommand` or by making the default print the usage line.

Stale expectation: older docs say `dev <plugin>` routes through this script (`🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/📓️research/📝️build-and-run.md:96-98`). Puzzle smoke logs ran `dev puzzle2d` (`🎫️tickets/🎆️26/🌙️08/☀️06/APPS-RUNNING-END-TO-END/🧪smoke-results.json`). That route no longer exists in the router, so `dev <plugin>` would now fail as unknown.

## 3. Root `repo ./📜️script.ts dev <route>`

File: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🛠️dev/🟦️.ts`, `DevScript.run` lines 24-46.

- Routes: `time-travel` (25), `storybook` (32), `storybook-static` (36), `mcp` (40, with `stdio`, `repo`, and the inspector).
- Anything else (line 44) prints `[dev] unknown route ...: expected storybook, storybook-static, time-travel or mcp. Playgrounds are dashboard commands: repo dashboard run playground:<variant> --param renderer=react` and exits 1. So `dev wfc` exits 1 with that message.

**Docs and tickets mentioning `dev <plugin>` or `dev wfc`:**

- No doc mentions `dev wfc`.
- `dev <plugin>` appears in `🎫️tickets/🎆️26/🌙️08/☀️29/GIS-MAP-END-TO-END/📓️research/📝️build-and-run.md:96-98` (stale, see section 2).
- Puzzle tickets use `dev puzzle3d` (`🎫️tickets/🎆️26/🌙️08/☀️06/S-AND-PLUGINS-END-TO-END/🧪lane-dev-puzzle3d-boot.txt:4`), `dev puzzle2d` (`🎫️…/☀️29/S-END-TO-END/📓️status.md:138`) and `dev-puzzle3d-react-dev` (`🎫️…/🌙️09/☀️02/PUZZLE-3D-END-TO-END/📓️2026-09-09-wave-R-react-boot-pipeline.md:41, 265`).
- WFC `README.md` lines 23-25 use `repo run @semio-tech/framework-os-dev:dev-wfc2d-react-prototype`, `dev-grid3d-react-prototype` and `dev-wfc3d-react-prototype`. **No such targets exist** in any `project.json` in the repo (grep found none). The README commands are stale.
- The `dashboard run playground:<variant>` path from the error message is the current one. I did not run it.

**Storybook.** There is no `wfc` storybook scope.

- `.storybook/📖️stories/🧭️coordination/🟦️.ts`: hand-curated scopes are ui, framework, framework/hosts, framework/os, puzzle, puzzle/2d, puzzle/3d, puzzle/5d, remodel, infinite. Generated scopes come only from package manifests that opt in with `semio.storybook`, and no WFC package does.
- `resolveActiveScopes` (line 291) throws on an unknown token, so `dev storybook wfc` fails.
- The WFC plugin-matrix story `export const Wfc` (`✏️s/🧑‍💻dev/📖️stories/🎭️plugins/🧪️.story.tsx:75`) is reachable only through the `framework/os` scope glob `../✏️s/🧑‍💻dev/📖️stories/**/🧪️.story.tsx` (coordination line 184). The working route is `dev storybook framework os`, not `wfc`.

## 4. WFC artifact packages, Rust crates, and time-travel scenarios

**Plugin TS package:** `✏️s/🔌️plugins/🀄️wfc/📦️packages/🟦️typescript`, `@semio-tech/wfc-js`. Also present: `🪶️sqlite/`.

**Artifact TS packages** (each has `package.json`, `📋️project.json`, `📜️script.ts`):

- bitmap: `@semio-tech/wfc-bitmap` at `🗿️artifacts/🖼️bitmap/📦️packages/🟦️typescript`
- grid2d: `@semio-tech/wfc-grid2d` at `🗿️artifacts/🔲️grid2d/📦️packages/🟦️typescript`
- 2d: `@semio-tech/wfc-2d` at `🗿️artifacts/◻️2d/📦️packages/🟦️typescript`
- grid3d: `@semio-tech/wfc-grid3d` at `🗿️artifacts/🧱️grid3d/📦️packages/🟦️typescript`
- 3d: `@semio-tech/wfc-3d` at `🗿️artifacts/🧊️3d/📦️packages/🟦️typescript`

All six are listed in `package.json` lines 144-149.

**Rust crates:**

- Solver: `⚙️engine/📦️packages/🦀️rust` (`@semio-tech/wfc-engine-rs` in build-owners).
- Per-artifact package crates `🗿️artifacts/<a>/📦️packages/🦀️rust`: `semio-s-artifact-wfc-bitmap`, `-grid2d`, `-2d`, `-grid3d`, `-3d`.
- Bridge crates `🏭️bridge`: for example `semio-s-artifact-wfc-{bitmap,grid2d,2d,grid3d,3d}-mutation-bridge`.
- Editor crates, observed in a head-60 listing (not exhaustive): `semio-s-artifact-wfc-bitmap-bitmap`, `semio-s-artifact-wfc-grid3d-editor`, `semio-s-artifact-wfc-3d-wfc3d`, plus several `-transient`, `-config` and `-schema` crates under `🏅️standards/🔖️1/🪆️subsets/✳️any/`.
- Hub component: `semio-hub-wfc-component` (see section 1).

**Time-travel.** WFC has no time-travel scenario.

- `✏️s/🔌️plugins/🀄️wfc/🧪️tests/` has 12 directories. The only TypeScript test is `🔬️typescript-package-commands/🟦️.ts`. There is no `⏪️time-travel` directory, and no `time-travel` reference under the WFC plugin.
- Puzzle has `✏️s/🔌️plugins/🧩️puzzle/🧪️tests/⏪️time-travel/🟦️.ts`, a 21-step puzzle2d journey. It is run with `repo run @semio-tech/framework-os-dev:time-travel -- --scenario <path>` (doc at line 87). The framework runner `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧪️time-travel/🟦️.ts` replays `🧫️fixtures/🧫️time-travel/🔣️.json` scenarios through the generic reducer.
- A WFC scenario would be new work. The reducer is framework-generic, so no framework change is implied.

## Caveats

- Nothing was executed. Findings are from reading the source and grepping.
- The generated playground catalog being empty is an observation about this worktree's gitignored output, not a claim about committed code.
- The editor crate list in section 4 is partial.
