# Result: Renderer hosts only UI (slice RENDERER-HOSTS-UI, prefix `rhu`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Worked 2026-10-16 12:52 to 14:40. Tools: `rhu-tools/`. Logs and raw outputs: `🗑️generated/rhu-*` (nothing deleted).

Aliases: `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `RT` = `E/..` (the engine folder), `SDK` = `🧰️framework/📦️packages/🟦️typescript` (package `@semio-tech/framework`), `OS` = `🧰️framework/🛍️products/💻️os`, `F` = `🧰️framework/🔨️modules`.

## 0. Verdict

Rule `renderer-hosts-only-ui`, renderer scope (the full audit command gave the same 707 as the scoped run at baseline, section 1):

| | Baseline | Final | Change |
|---|---:|---:|---:|
| all edges | 707 | **535** | -172 |
| production edges | 467 | **326** | -141 |
| of which resolver artefact (`react`, `react-dom` through the dependency-image symlink, section 2) | 126 | 126 | not counted |
| **real production edges** | **341** | **200** | **-141 (-41 percent)** |
| of which parent-owned (Actor 34, Media 4) | 38 | 38 | listed in section 6, not touched |
| of which tooling entries (script, builder, test config) | 24 | 6 | -18 (re-pointed to SDK entries) |
| **real, actionable, not parent-owned, not tooling** | 279 | **156** | -123 |
| test and story edges | 240 | 209 | -31 (52 are the react artefact) |

- The mechanism that removed the 141: the rule explicitly allows `^🧰️framework/📦️packages/` (the framework package surface, the SDK). The renderer reached framework modules by relative paths and by workspace package names (`@semio-tech/canvas-react-renderer`, `@semio-tech/assets`, `@semio-tech/framework-replication`, `@semio-tech/framework-os`); those imports now go through SDK entries (13 new, 8 extended), and two os wire modules (`🧾️app-channel`, `🪢️wire-contracts`) were lowered into the framework next to the peer's `🎁️packs` so the renderer reaches them the same way. Section 3.
- What I did not do: **no runtime-injected port was added**. The 200 remaining production edges are, apart from the 38 parent-owned and 6 tooling edges, os subsystems whose types and codecs the shell needs. They cannot be removed by an injection inside the renderer roots alone (sections 4 and 5 explain why and give the order of work). The existing `bootFrameworkOs(options, execution)` is the seam to extend.
- Verification (section 7): `tsc` on the renderer closure 41 errors to 40 (identical set minus one fixed, plus two errors in a peer's file edited during the run); renderer vitest: same 30 failed files, failing set identical to the baseline; SDK surface law 5 of 5 pass (496 expects) plus a new identity test; real Vite container resolves all new specifiers.

## 1. Baseline (before the first edit, 12:55 to 13:10)

| What | Command | Result |
|---|---|---|
| Rule scan, renderer scope | `rhu-tools/cruise.sh <label> scoped`: `dependency-cruise.mjs RT/🎯️targets/⚛️react RT/🧱️elements --do-not-follow '^(?!<renderer roots>)' --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json --output-to …` | `renderer-hosts-only-ui` **707** unique from-to edges: 467 production, 240 tests and stories. 706 modules, 2540 dependencies. |
| Rule scan, exact audit command | `rhu-tools/cruise.sh base-full full`: `dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config … --output-type err` (13:08) | 13418 errors over all rules, 17278 modules, 47232 dependencies; `renderer-hosts-only-ui` **707**. The scoped run is exact for this rule (same config, same `from`). |
| Type-check | `node_modules/typescript/bin/tsc -p RT/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json --noEmit --incremental false --pretty false` (the renderer closure: the react target and every element) | **41** errors (`rhu-tsc-baseline.log`); 27 are one `ShardBudget.maxFrames` shape error in `UiDocumentStore` tests |
| Renderer vitest | `rhu-tools/vitest.sh renderer` (copy of `ikt-tools/vitest.sh`, config `RT/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`, `SEMIO_TEST_LEVEL=long`, hand-built `SEMIO_VITEST_POLICY`) | 214 files: **30 failed**, 184 passed; 3033 tests: **37 failed**, 2996 passed (`rhu-fn-renderer-baseline.txt`) |

The audit counted 661 edges (457 production). The tree moved since (the kernel split, new tests, peers); at my start the same rule gave 707 (467 production).

## 2. The `.cache/b/d/m` edges are a resolver artefact, not source edges

Evidence (`rhu-tools/edges.mjs`, `classify.mjs`, Node `lstat` and `realpath`):
- The repository root `node_modules` is a symlink into the mounted dependency image: `node_modules -> …/.cache/b/d/m/j/e53350b0bed685d50b82/node_modules`. The cruiser resolves a bare import to the real path, so `import "react"` becomes `.cache/b/d/m/j/<hash>/node_modules/react/index.js`.
- The rule allows `^node_modules/react(?:-dom)?/` and `^node_modules/(?:vite|vitest)/` (`RENDERER_HOST_ALLOWED_RESOLVED_PATHS`, `🏗️construction/🟨️.cjs:26`). Of the 178 `react` and `react-dom` edges of the renderer (179 import statements), **zero** resolve to a plain `node_modules/` path, so the allow-list can never match while the root link exists. 126 production and 52 test edges.
- No source file is involved, no generated code is imported, and no tsconfig path or symlink inside the renderer tree is used (`engine/node_modules` is a plain folder holding only `.vite`). Nothing to fix in source; these are not counted above.
- The other `.cache` edges (48 test edges: `three` 15, `playwright` 11, `sharp` 6, `fast-deep-equal` 4, `fast-json-patch` 3, `typescript` 2, `jsdom`, `esbuild`, `gl-matrix`, `@iarna/toml`, `pngjs`, `lodash`, `@tailwindcss/oxide`) are real third-party imports of tests that the rule would also forbid with correct paths. **No production file imports a third-party package other than `react` and `react-dom`.**

Policy fix the parent needs (not applied; the policy is admitted and reviewed elsewhere): in `RENDERER_HOST_ALLOWED_RESOLVED_PATHS` add the real path of the dependency link, for example `const NODE_MODULES_REAL = path.relative(process.cwd(), fs.realpathSync("node_modules"))` and the entry `^<escaped NODE_MODULES_REAL>/(?:react|react-dom|vite|vitest)(?:/|$)` next to the existing plain forms. Do **not** switch the cruiser option `preserveSymlinks` on: every workspace link `node_modules/@semio-tech/*` would stop resolving to its source tree and every other rule would break. The same fix applies to any other rule whose allow-list names `node_modules/...` (check `RESOLVED_NODE_BUILTIN_PATH` neighbours).

## 3. What changed

### 3.1 Framework modules: SDK entries instead of relative paths (-116 production edges in the first step)

`rhu-tools/repoint.mjs` rewrites imports and re-exports by TypeScript AST. For each name of each import it asks the checker which SDK entry exports the very same declaration (`rhu-tools/origins2.mjs`: declaration identity, not name equality, so a same-named export of another module is never picked), picks one entry for the whole declaration (specific entries before the root barrel), splits the declaration when no single entry covers it, and keeps names without an entry on the old specifier. Type-only modifiers, aliases and multi-line formatting are kept. 102 renderer files (223 import and re-export statements) changed; the list is in `🗑️generated/rhu-repoint-*.log`.

New SDK entries (`SDK/package.json` `exports`, each a facade of explicit named re-exports, no `export *`):

| Entry | Facade | Source module | Names |
|---|---|---|---:|
| `./assets` | `🖼️assets/🟦️.ts` | `F/🖼️assets/📦️packages/🟦️typescript` | 1 |
| `./assets/mesh` | `🖼️assets/🥽️mesh/🟦️.ts` | `F/🖼️assets/🥽️mesh` (+ `📇️catalog`) | 2 |
| `./async/continuation` | `⏳️async/🪃️continuation/🟦️.ts` | `F/⏳️async/🪃️continuation` | 3 |
| `./canvas/react-renderer` | `🖼️canvas/🎨️react-renderer/🟦️.ts` | `F/🖼️canvas/🎨️react-renderer` | 7 |
| `./io/stream-mux` | `🚪️io/🔀️stream-mux/🟦️.ts` | `F/🚪️io/🔀️stream-mux` | 5 |
| `./raster/video` | `🖌️raster/🎥️video/🟦️.ts` | `F/🖌️raster/🎥️video` | 8 |
| `./time-travel` | `⏪️time-travel/🟦️.ts` | `F/⏪️time-travel` | 2 |
| `./tool-machine` | `🛠️tool-machine/🟦️.ts` | `F/🛠️tool-machine` | 2 |
| `./tool-run` | `⏯️tool-run/🟦️.ts` | `F/⏯️tool-run` | 18 |
| `./trace` | `⏱️trace/🟦️.ts` | `F/⏱️trace` | 2 |
| `./value/resident` | `🌱️value/💾️resident/🟦️.ts` | `F/🌱️value/💾️resident` | 1 |
| `./wire-contracts` | `🪢️wire-contracts/🟦️.ts` | `F/🪢️wire-contracts` (moved, 3.2) | 13 |
| `./app-channel` | `🧾️app-channel/🟦️.ts` | `F/🧾️app-channel` (moved, 3.2) | 11 |

Extended entries (names added to an existing facade; the plugin-facing names are unchanged): `./2d` (+1), `./interaction` (+3), `./io/base64` (+2), `./pixels/editing` (+6), `./pixels/selection` (+1), `./replication` (+29, from the replication package entry), `./packs` (+12, a peer's entry), `./document/check-in` (+2, a peer's entry). Tool: `rhu-tools/mkfacades.mjs` with `facade-spec*.json`.

The mapping needed no new name for kernel, process and manifest: the root barrel and the existing `./process/*`, `./compiler/syntax/module-tree`, `./schema/oracles/validator`, `./canvas/runtime-assets`, `./value/decode`, `./value/ordered/numeric` entries already export them.

Scripts (`📜️script.ts` of the react target, Scenes camera, NodeGraph, Paint2dHost editing) keep the bootstrap convention of `RESULT-SDK-BOUNDARY.md` rule 7: relative paths to the facade file, not package specifiers (`rhu-tools/bootstrap-relative.mjs`). They resolve under the allowed prefix, so those 18 edges are gone too.

### 3.2 Two os wire modules lowered into the framework

`OS/🔨️modules/🧾️app-channel/🟦️.ts` and `OS/🔨️modules/🪢️wire-contracts/🟦️.ts` have a closed closure (`wire-contracts` reaches only `packs`; `app-channel` reaches `wire-contracts` and `packs`; `rhu-tools/closure.mjs`: 1 and 2 os-internal files), no Rust twin and no importer outside `OS/🟦️.ts`, `🧵️backbone`, the browser-bundle action handoff and one test. They moved to `F/🧾️app-channel` and `F/🪢️wire-contracts` beside the peer's `F/🎁️packs` (same shape as the peer's move). The move used the peer's AST tool `dlo-tools/move.ts` with `rhu-tools/spec-wire.json` (9 files, 17 specifier edits, snapshot `🗑️generated/rhu-snap-wire.json`). Pins updated by hand: `OS/📦️packages/🟦️typescript/📋️project.json` (2 inputs), the wgpu package `📋️project.json` (2), the fixture `OS/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json` (2), and the sorted `sourceModulePaths` list of `🔣️taxonomy.json` (2 entries re-placed in byte order, validated sorted and unique). Their folder names stay registered in `members-of-modules`.

### 3.3 Test configuration (necessary consequence)

`RT/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:268` aliased `@semio-tech/framework` as a prefix, so `@semio-tech/framework/<sub>` became `<barrel file>/<sub>` and 149 test files failed to load. The alias is now `find: /^@semio-tech\/framework$/u` (the form the four repo Vite builders already use since `RESULT-SDK-BOUNDARY.md` section 4.4). Two sibling vitest configs (quiz, presentation) keep the old prefix alias; neither references the renderer (`git grep` for `framework-renderer-react` and `📺️renderer` in both products is empty).

### 3.4 New test

`SDK/🧪️tests/🔌️sdk-surface/🟦️.ts`: "every facade value export is the very binding of the module it names (runtime identity in a fresh process)". TypeScript's parser (third-party) lists each facade's value re-exports; a fresh Bun process per facade imports the facade and the source module and compares `===`. It runs over all facades (more than 100 with value exports), takes 5 seconds, and has a positive and a negative control (a deliberately wrong pair reports the wrong name).

## 4. Why the remaining os edges cannot be injected from inside the renderer roots

- The rule matches `engine/(🎯️targets/⚛️react|🧱️elements)/`. A file that imports an os module is flagged wherever it sits inside those roots, including the "composition" side of a port. So a registry (the `🪢️ElementHosts` pattern of `RESULT-INTERPRETER-KERNEL-TS.md`) is fine inside the roots, but the code that fills it has to live outside them and has to be called by an app entry (`✏️s/🧑‍💻dev/🚀️entry/⚛️react`, `🏢️semio-tech/🎡️play`, `♻️mit-bestand/🧺️demonstrator`, storybook, and a vitest setup file). Those entries are not mine.
- The existing seam is `bootFrameworkOs(options, execution)` (`E/🐚️Shell/🟦️.tsx:1266`): `FrameworkOsBootExecution` already carries `backboneWorkerFactory` and `documentServices`, `FrameworkOsBootOptions` carries `surfaceSessionFactories`. New ports belong there, threaded to the elements through a React context.
- A port also needs its types. `BackboneWorkerRequest`, `DirectoryCommand`, `ShellState`, the wasm session classes and the like are used in 20 to 40 files each. They must either be defined in an allowed place (framework SDK or UI) or be re-declared structurally in the renderer. For protocol types the first is right (that is what `packs`, `wire-contracts`, `app-channel` did); it needs the owning module closure to move.
- Several owners are bound by constraints that make a move or a port a design step of their own, not a mechanical one: Rust twins in the same folder (mcp bridge and channel, shell reducer, dag-input board), workspace packages whose `node_modules` link lives in the dependency image (r3f), and pins in 5 to 8 registries per module (deployment).

## 5. Remaining real production edges (200), by family, with the next step

| Family | Edges | Owner | Next step |
|---|---:|---|---|
| Actor (`🎭️actor/📮️shard-client` 14, `🖼️wire-turn` 10, package entry 4+1, `🚪️lifetime/🩹️patch` 2, `🧵️shard-runtime` 2, `segmented-download` 1) | 34 | parent | listed in section 6; ready spec `rhu-tools/handoff-facade-spec-actor-media.json`, dry run proven |
| Media (`engine/🎬️media` channel and lifecycle 2, `window-kits/🎬️media` 1, `kernel/🎬️media-export` 1) | 4 | parent | listed in section 6; `./kernel/media-export` is in the same ready spec |
| OS wire stack through `@semio-tech/framework-os` (backbone 26 names, directory commands) | 35 | os fleet | HO-1 |
| Directory feature modules (`sign-in`, `spaces`, `first-run`, `delegations`, `session-refresh`, `local-session`, `access-policy`) | 25 | os fleet | HO-1 (hub account and session port) |
| Browser-bundle action and patch handoff, inference opening | 12 | os fleet | HO-1 |
| Plugin registry: hub-source resolution 7, deployment 7+2, registry 2, hub source 1, service worker 1 | 20 | plugin fleet | HO-2 |
| Actor document binding (`plugin/📡️backbone/🔗️binding`, 54 importers) | 6 | plugin fleet | HO-3 |
| `@semio-tech/infinite-world-r3f` | 12 | os fleet | HO-4 |
| dag-input (NodeGraph selection and hover wire) | 8 | os fleet | HO-5 |
| `🎚️config` mutations | 12 | config owner | HO-6 |
| Engine shared leaves (`🎚️UiPreferences` 4, `💾️resident` 2, `🧵️work` 1, oracle script 1) | 8 | renderer | HO-6 |
| mcp bridge and channel (TS twin of the Rust wire) | 8 | mcp owner | HO-7 |
| Shell state reducer (TS twin) | 3 | shell owner | HO-7 |
| `🏪️store/👷️worker` constants | 3 | store (parent) | HO-1 |
| WasmSessionLoader wasm binding modules (`framework_surface.js`, `flow_core.js`, flow browser, `node-graph-rs`) | 4 | renderer | HO-8 |
| Tooling entries to the repo library (`📜️script.ts`) | 6 | policy | HO-9 |

Hand-offs, each with the measured facts:

- **HO-1, lower the os wire stack as one unit.** `OS/🔨️modules/🧵️backbone` (1631 lines) has a closure of 27 os-internal files: `📇️directory/{🧬️schema,🚪️io,🔌️client}`, `🔌️plugin/🌐️browser-bundle/{🎯️action-handoff,🩹️patch-handoff,🪟️view-context,🧵️child}`, `🔌️plugin/🖨️describe`, `💡️inference/🚪️opening`, `📞️directory-client`. Its only importers are `OS/🟦️.ts` and two os tests, so the unit can move with few edits. After it moves, add SDK entries for `backbone`, `directory` and the hub account modules; the renderer re-points with `rhu-tools/repoint.mjs`. Clears about 70 production edges (all of the first three rows). Cost: the pins (taxonomy `sourceModulePaths` and `inputPatterns`, the wgpu and os `📋️project.json` inputs) as in 3.2. The peer slice `dlo` already moved `packs`, `document/*` and `inference/service` along this route.
- **HO-2, published page URL and hub source.** `publishedPageUrl`, `readPublishedPageOrigins`, `relocatePublishedRequestUrl`, `installPublishedPageFetch` (a module-load fetch patch) are four neutral functions inside `OS/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts`; 6 renderer files, the wgpu frame worker, the play site page generator and its test use them. Proposed home `F/🖼️assets/🌐️published-page` with entry `./assets/published-page`; the deployment module keeps a side-effect import so the fetch patch stays installed for its other importers (52 import lines in the repo). I did not move it: deployment is pinned in `wgpu 📋️project.json:203`, `🧑‍💻dev 📋️project.json:15`, `🦑️repo library 📋️project.json:1348`, `🔣️taxonomy.json` (2 lists) and the fixture `os-dev-composition-ownership` (2 places), and I cannot run those oracles. `hub-source/🔍️resolution` (`parseHubProgramIdV1` and the catalog closure helpers) has 7 renderer importers and 1 other, closure 7 files.
- **HO-3, actor document binding.** 54 importers; a neutral contract, but the move is the plugin fleet's decision.
- **HO-4, r3f is a UI package filed under os.** It imports only `@semio-tech/framework`, `ui-react`, `ui-styling`, has no os importer (renderer 19 files, 2 stories) and belongs under `🖱️ui`. A package move changes the workspace list, `📋️build-owners.json`, the lockfile and the `node_modules/@semio-tech/infinite-world-r3f` link inside the dependency image. I did not move it: the image cannot be touched and `bun install` was not run. Alternative with no move: policy allows the package path.
- **HO-5, dag-input.** `decodeDagSelectionJson`, `decodeDagHoverJson` and the refusal types are used only by NodeGraph (5 renderer files) plus 2 other-src importers of the schema file; closure 0 to 5. Candidate for NodeGraph ownership once the 2 other importers are looked at.
- **HO-6, config and preferences.** The ui-preference part of `🎚️config/🧬️schema/🧬️mutations` (setTheme, setDriver, setAppearance, setLocale, setLayout, setTerminology, `UiPreferencesConfigMutation`) and `engine/🎚️UiPreferences` form one feature (preference model plus lane sync). The model belongs in `🖱️ui` (its types already come from `@semio-tech/ui-react`); the storage and lane sync stay os. Do not move `engine/🎚️UiPreferences` into `elements`: its own os imports would become renderer edges (net loss). `engine/💾️resident` and `engine/🧵️work` are one-line singletons shared by the react and wgpu targets; they are the first candidates for a port (`resident: OwnedResidentLedger`, `work: RootOriginalBootstrapOwner`) through `bootFrameworkOs` `execution`, because the framework types are now reachable through `./value/resident` and the actor entry.
- **HO-7, TS twins of Rust wires.** `🌉️mcp/🧵️bridge` and `🐚️channel`, `🖥️shell`, `♾️infinite/🎲️board dag-input`: the TS file sits next to its Rust SSOT and fixtures, and `interactivity-audit` reads the mcp paths. Moving one half splits the twin. These need a port whose types are the generated schema mirrors, or a decision to publish the generated TS twin through an SDK entry.
- **HO-8, wasm bindings.** The loader imports `@semio-tech/framework-surface-rs`, `framework-os-node-graph-rs`, `flow-core`, `framework-editor-rs` and uses `typeof import(...)` of them. The seam `surfaceSessionFactories` already exists; the loader's own defaults are what is flagged.
- **HO-9, policy.** (a) add the tooling exemption `pathNot: BOOTSTRAP_TOOLING_ENTRY_PATH` to this rule as the cross-technology rules already have (6 production and 1 test edge are repo-library imports of `📜️script.ts`); (b) exempt `🧪️tests/**`, stories and `🧫️fixtures` (209 edges: 52 artefact, 48 third-party and 6 builtin imports, about 43 json fixtures, the rest os and framework types in dynamic imports and type queries) or re-home the fixtures; (c) the react realpath fix of section 2.

Taxonomy: the new SDK facade folders (`🖼️assets`, `🖼️assets/🥽️mesh`, `⏳️async/🪃️continuation`, `🖼️canvas/🎨️react-renderer`, `🚪️io/🔀️stream-mux`, `🖌️raster/🎥️video`, `⏪️time-travel` entry file, `🛠️tool-machine`, `⏯️tool-run`, `⏱️trace`, `🌱️value/💾️resident`, `🪢️wire-contracts`, `🧾️app-channel`) are not registered in `🔣️taxonomy.json`, as in `RESULT-SDK-BOUNDARY-2.md` section 10; run the sweep for these names.

## 6. Parent-owned edges (listed, not touched)

Production, 38 edges. Importer files are under `E/`.

| Target | Edges | Importers |
|---|---:|---|
| `F/🎭️actor/📮️shard-client` | 14 | `🔌️PluginRuntime` (`🟦️.tsx`, `📬️typed-result`, `🔁️turn-settle`, `🖼️surface-refresh`, `🧵️turn-dispatch`, `🛰️runtime-singletons`, `🫀️load-progress`, `🧩️module-loader`, `🧺️handle-ui-intake`, `🪢️handle-scope`, `🚚️handle-turn-drive`), `📃️UiDocumentStore/📥️intake`, `🏛️ShellHost/🔌️extension-runtime`, `🏛️ShellHost/🚀️plugin-lifecycle` |
| `F/🎭️actor/🖼️wire-turn` | 10 | `🔌️PluginRuntime` (`🟦️.tsx`, `🪝️wire-codec`, `🏁️operation-completion`, `📬️typed-result`, `🔁️turn-settle`, `🪟️retained-ui-patch`, `🧵️turn-dispatch`, `🧩️module-loader`, `🚚️handle-turn-drive`), `🛠️ShellHelpers/🔩️plumbing` |
| `F/🎭️actor/📦️packages/🟦️typescript/🟦️.ts` | 4 | `🔌️PluginRuntime` (`🟦️.tsx`, `🔁️turn-settle`, `🧵️turn-dispatch`, `🪢️handle-scope`) |
| `F/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime/🩹️patch` | 2 | `🔌️PluginRuntime` (`🟦️.tsx`, `🖼️surface-refresh`) |
| `F/🎭️actor/🧵️shard-runtime` | 2 | `🔌️PluginRuntime` (`🟦️.tsx`, `🛰️runtime-singletons`) |
| `F/🎭️actor/📮️shard-client/📤️segmented-download` | 1 | `📤️SegmentedDownload` |
| `F/🎭️actor/📦️packages/🟦️typescript/🚪️lifetime` | 1 | `🔌️PluginRuntime/🧵️turn-dispatch` |
| `RT/🎬️media/📡️channel` | 1 | `🔌️PluginRuntime/🔗️channel-adapter` |
| `RT/🎬️media/🚚️lifecycle` | 1 | `🎬️MediaTransportHost` |
| `OS/🔌️plugin/🪟️window-kits/🎬️media` | 1 | `🎬️MediaTransportHost` |
| `F/🎠️kernel/🎬️media-export` | 1 | `🛠️ShellHelpers/🔩️plumbing` |

Test edges: Actor 16, Media 3 (the same importers' tests).

These are mechanically fixable with the same tool once the owner agrees: `SPEC=rhu-tools/handoff-facade-spec-actor-media.json PLAN=🗑️generated/rhu-plan-actor.json ORG=🗑️generated/rhu-origins-actor.json node rhu-tools/mkfacades.mjs` (dry run, proven: it plans `./actor`, `./actor/lifetime`, `./actor/lifetime/patch`, `./actor/shard-client`, `./actor/shard-client/segmented-download`, `./actor/shard-runtime`, `./actor/wire-turn`, `./kernel/media-export`), then `repoint.mjs`. The media modules under `OS/…/🎬️media` are renderer-engine files and need a port or a move. One finding for the actor owner: `Lane` is **not exported** by the actor package entry (`tsc` TS2305 in `🔌️PluginRuntime` `🧵️turn-dispatch`, `🪢️handle-scope`, `🔁️turn-settle`, present at baseline).

## 7. Verification (every command and its real result)

| # | Command | Result |
|---|---|---|
| 1 | `rhu-tools/cruise.sh base2-scoped scoped` (before) and `final2-scoped scoped` (after the last edit, 14:12) | 707 to **535**; prod 467 to 326; `rhu-tools/final-table.mjs` classification in `🗑️generated/rhu-final2-scoped-edges.jsonl` |
| 2 | `rhu-tools/cruise.sh base-full full` (exact audit command, 13:08) and `final-full full` (14:33) | 707 (equal to the scoped run) and **535** (equal to the scoped run); section 8 |
| 3 | `tsc -p …/tsconfig.json --noEmit --incremental false` before, after step 1 (40), after step 4 (40), final | 41 to 40. The set difference to the baseline is one removed error (`Dock/…/browser-acceptance/📜️script.ts`, a peer's fix) and, in the last run only, two errors `TS2304 Cannot find name 'T'` in `F/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts`, a peer's file written at 13:56 during that run. No error in a file I edited; no new error caused by an import I changed. |
| 4 | `rhu-tools/vitest.sh renderer` baseline, then after step 4 | 214 files, 30 failed both times; tests 37 failed (baseline) vs 39 (after): the two extra were `package-integration` > "owns the browser accessibility import…" and "validates explicit browser entry identities…", failing with `Invalid taxonomy schema: generatorContracts["jco-package-adapter"].inputPatterns must be unique and lexically ordered`, a peer's taxonomy edit in flight. A rerun of that file alone gave the baseline's 6 failures, same messages (`rhu-vitest-pkgint.log`), and the taxonomy validates sorted and unique. First after-run (before the vitest alias fix of 3.3): 149 files failed to load (`Failed to resolve import "@semio-tech/framework/…"`), which is how the alias problem was found. |
| 5 | `bun test ./🧪️tests/🔌️sdk-surface/🟦️.ts` in `SDK` | 5 pass, 0 fail, 496 expects (was 439 in `RESULT-SDK-BOUNDARY-2.md`), including loading every one of the 161 entries in a fresh process |
| 6 | the new identity test (3.4) with `-t 'runtime identity'` | 1 pass; negative control (`bun -e` with a wrong pair) reports the wrong name |
| 7 | `node rhu-tools/vite-resolve.mjs`: a real Vite plugin container with the repo builders' alias, importer `E/🔌️PluginRuntime/🟦️.tsx`, 24 specifiers | 24 of 24 resolved to the facade files, 0 unresolved |
| 8 | `bun build` of the four re-pointed `📜️script.ts` | NodeGraph, Paint2dHost, Scenes camera bundle; the react target script stops in `playwright-core` internals (`chromium-bidi` optional dependency), not at an import I changed |
| 9 | `rhu-tools/vitest-os.sh final` (os vitest project, the peer's command) vs the peer's `dlo-os-vitest-after.json` | 609 tests, 498 pass, 111 fail vs 499 / 110. 4 newly failing, 3 fixed. The 4: two `app-channel-codec` tests (`ENOENT` of fixtures `📡️channel/🔖️channel-version.json` and `📡️spr/🧵️channel/…/🎬️media-export-wire🔣️.json`: another peer is renaming `📡️spr`), `PluginGraph` and `directory-client-http` (fixture counts 0). None reads a path I moved. |
| 10 | facade loads: `bun -e "await import('@semio-tech/framework/<sub>')"` for the 17 new or extended entries, in `SDK` | all load |

Runtime proof for ports: none, because no port was added. What the renderer suites prove is the re-pointing: the same 3033 tests mount `PluginRuntime`, `ShellHost`, `World3dHost`, `NodeGraph`, `Paint2dHost`, `Canvas2dHost`, `TiledMapHost` and the others through the SDK entries, with a failing set equal to the baseline. The `vi.mock("@semio-tech/canvas-react-renderer")` calls of two tests still apply: the facade imports the same module id.

## 8. Final full cruiser run (the exact audit command)

`rhu-tools/cruise.sh final-full full`, 14:06 to 14:33 (three attempts died on `ENOENT` while peers deleted files; the fourth completed): `dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config … --output-type err`, exit code 82 (13394 errors mod 256), 17454 modules, 48290 dependencies. `renderer-hosts-only-ui` **535**, equal to the scoped run; 226 of its lines contain `.cache/` (178 react and react-dom artefacts plus 48 third-party test imports).

Other rules, base run (13:08) to final run, by line count (`🗑️generated/rhu-base-full-err.txt` and `rhu-final-full-err.txt`): total 13418 to 13394. Moved while my changes landed (a share of the decreases can belong to the peers' simultaneous moves; I did not attribute them line by line): `no-circular` 651 to 588 (-63), `no-cross-package-relative-@semio-tech/framework-kernel` 97 to 65, `-framework-os` and `-framework-os-kernel` 359 to 342 each, `-framework-tool-run-rs` 12 to 7, `-framework-async` 16 to 12, `-framework-io-base64-rs` 10 to 6, `-semio-framework-pixels` 17 to 13 (package specifiers replace relative paths into other packages). Not caused by me (peer files, checked by diffing the line sets for the three rules that went up): `not-to-unlisted` +3 (plugin tests importing `three`/`dagre`), `-framework-renderer-wgpu` +2 (new engine tests and a story), `-framework-schema` +2 (peers' moved files), `plugins-framework-sdk-only` -11 and `no-cross-technology-✏️s-to-🧰️framework` -6 (plugin files, not mine).

## 9. Found state, peers, foreign issues

- No predecessor of this slice. Peer `dlo` (os lowering) moved `packs`, `document/check-in`, `document/scope`, `inference/service` into the framework and added their SDK entries while I worked; my first run saw `isTerminalDocumentCheckInPhaseV1` as unresolvable until their entry appeared, and I extended `./packs` and `./document/check-in` additively. Other fleet slices were active in the same tree (tool folders `otc-tools`, `ts3-tools`, `osm-tools`, `pfa-tools`, `pfb-tools`, `rcr-tools`, `frs-tools` appeared during the run); `🔣️taxonomy.json` and the SDK `package.json` changed under me several times, so every write of mine re-read the file immediately before and changed one anchored place.
- Foreign errors: the two `TS2304` in `💾️storage/🟦️.ts` (peer, 13:56), the `Lane` export (actor owner), 27 `ShardBudget.maxFrames` shape errors in `UiDocumentStore` tests (baseline).
- The package `@semio-tech/framework-renderer-react` still lists `@semio-tech/assets`, `@semio-tech/canvas-react-renderer` as dependencies although source no longer imports them (only the vitest alias and two `vi.mock` strings do). `@semio-tech/framework-replication` was imported but never listed. I left `package.json` and the lockfiles alone.
- `quiz` and `presentation` vitest configs still carry the prefix alias (section 3.3).

## 10. Files changed

- 102 renderer files (imports and re-exports only; line counts equal per file by construction): `E/**`, `RT/🎯️targets/⚛️react/🟦️.tsx`, the four `📜️script.ts`.
- `RT/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts` (alias).
- SDK: `package.json` (+13 `exports`), 13 new facades, 8 extended facades, `🧪️tests/🔌️sdk-surface/🟦️.ts` (+1 test).
- Moved: `OS/🔨️modules/🧾️app-channel/🟦️.ts` and `OS/🔨️modules/🪢️wire-contracts/🟦️.ts` to `F/`; 7 importers re-pointed (`OS/🟦️.ts`, `🧵️backbone`, 4 browser-bundle files, `🧪️backbone-envelope-io`); pins in 4 files (3.2).
- No file deleted. No `[DEBUG]` log was added. Temporary files only in `rhu-tools/` and `🗑️generated/rhu-*`.

## 11. Tools (`rhu-tools/`)

`cruise.sh`, `edges.mjs`, `classify.mjs`, `final-table.mjs` (rule scan, classification), `plan1.mjs`, `origins.mjs`, `origins2.mjs`, `plan2.mjs`, `refresh.sh` (name and origin tables), `facade-spec*.json`, `mkfacades.mjs`, `addfacade.mjs` (SDK entries), `repoint.mjs` (AST re-pointer, `PARTIAL=1`, `PREVIEW=1`), `bootstrap-relative.mjs`, `spec-wire.json` (module move, run with `dlo-tools/move.ts`), `closure.mjs`, `revdeps.mjs`, `selfcontained.mjs` (closure and importer facts), `vite-resolve.mjs`, `vitest.sh`, `vitest-os.sh`, `failnorm.mjs`, `handoff-facade-spec-actor-media.json`.
