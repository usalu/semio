# Result: Renderer hosts only UI, second pass (slice RENDERER-HOSTS-UI-2, prefix `rhu2`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. 2026-10-16 14:34 to about 18:40. Tools: `rhu2-tools/`. Logs and raw outputs: `🗑️generated/rhu2-*` (nothing deleted, no `[DEBUG]` log added).

Aliases: `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `RT` = `E/..` (the engine folder), `SDK` = `🧰️framework/📦️packages/🟦️typescript` (package `@semio-tech/framework`), `OS` = `🧰️framework/🛍️products/💻️os`, `FM` = `🧰️framework/🔨️modules`, `F` = this folder.

STATUS: COMPLETE. Every row below was run; the final full-cruiser run finished 18:25.

## 0. Verdict

Rule `renderer-hosts-only-ui`, renderer scope. The scoped cruiser run is exact for this rule (the previous slice proved it equal to the full audit command, and the baseline pair of this slice agrees: 535 both).

| | Baseline (14:37) | After units 1 | After unit 2 | After unit 3 | After unit 4 (final scoped, 17:22) |
|---|---:|---:|---:|---:|---:|
| all edges | 535 | 522 | 451 | 408 | **397** |
| production edges | 326 | 309 | 253 | 213 | **202** |
| of which resolver artefact (`react` through the dependency-image link) | 126 | 126 | 127 | 127 | 127 |
| of which parent-owned (Actor 34, Media 4) | 38 | 38 | 38 | 38 | 38 |
| of which tooling entries (`📜️script.ts`) | 6 | 6 | 6 | 6 | 6 |
| **real, actionable, not parent-owned, not tooling** | **156** | 139 | 82 | 42 | **31** |
| test and story edges | 209 | 213 | 198 | 195 | 195 |

- The 124 production edges removed (156 to 31 actionable) all went the way the rule and the brief prescribe: the neutral wire and contract modules the renderer needs were lowered into `FM` (56 files moved, 10 files created by extraction or split), exposed through 31 new explicit SDK entries, and both the renderer and the os subsystem were re-pointed to them. 0 shims, 0 re-export barrels.
- No runtime port was added. Section 4 explains why for each remaining family, with the measured closure, and gives the port design where one is the right answer (hub plugin source factory, access policy).
- Exact audit cruiser command (section 8), full tree, baseline 14:35 to 15:04 against final 18:05 to 18:25 (after the three folder renames; the first final run at 17:14 was killed by its 50-minute alarm under machine load 50 and a second one was started with a 7000 s alarm): `renderer-hosts-only-ui` 535 to **391** (-144), `framework-no-products` 149 to 167 (+18, none from this slice, see section 8), `no-circular` 583 to **574** (-9), total violations 13926 to 10596 (peers' concurrent work dominates that number).
- Verification (section 7): `tsc` renderer closure: the same six test-file errors as before, plus seven errors in peers' files that the closure follows (`jobserver` 2, `caching/vite` 5); `tsc` semantic diagnostics over 1094 root files 113 to 113 (one react-target `📜️script.ts` diagnostic gone, one added in `🌎️hub/🧪️tests/🎛️run`, a peer's `VitestPolicyV1` change); os vitest 609 tests, 113 fail before and 110 after, no new failure, every file SAME or better; renderer vitest 38 failed tests before, 29 after, with 12 test files that cannot load because a peer's in-flight `🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts` entry (written 18:28 by the fixture-entry slice) yields non-file URLs (section 7 row 5); SDK surface law 6 of 6 (591 expects, was 498); a real Vite plugin container resolves all 32 new specifiers.

## 1. Baselines (before the first edit, 14:35 to 15:12)

| What | Command | Result |
|---|---|---|
| Rule scan, scoped | `rhu2-tools/cruise.sh base-scoped scoped` (`dependency-cruise.mjs RT/🎯️targets/⚛️react RT/🧱️elements --do-not-follow '^(?!<renderer roots>)' --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json`) | `renderer-hosts-only-ui` **535** (326 production, 209 tests and stories), same as the previous slice's final |
| Rule scan, exact audit command | `rhu2-tools/cruise.sh base-full full`: `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs 🧰️framework ✏️s 🌎️hub ♻️mit-bestand --config … --output-type err` (14:35 to 15:04) | 13926 errors, 17520 modules, 49071 dependencies; `renderer-hosts-only-ui` **535**, `framework-no-products` **149**, `no-circular` **583** (`🗑️generated/rhu2-base-full-rules.txt`) |
| Type-check, renderer | `node node_modules/typescript/bin/tsc -p RT/🎯️targets/⚛️react/📦️packages/🟦️typescript/tsconfig.json --noEmit --incremental false --pretty false` | 7 errors (`rhu2-tsc-baseline2.log`); the first run (14:45) showed 41 because a peer's NodeGraph loader was mid-edit |
| Type-check, lowering scope | `sdk2-tools/mk-tsconfig.ts` + `sdk2-tools/tsc-sem.ts` over 1074 root files (the moved modules' old and new homes, their importers, the os tests, the hub tests, the renderer, the SDK facades) | 113 semantic diagnostics (`rhu2-tscsem-base.txt`) |
| Renderer vitest | `rhu2-tools/vitest.sh renderer` with the wrapper config below | 214 files: **31 failed**, 183 passed; 3033 tests: **38 failed**, 2995 passed (`rhu2-fn-renderer-baseline.txt`, 47 names) |
| os vitest | `rhu2-tools/vitest-os.sh base` (`OS/🧪️tests/🎚️config/🟦️.ts`) | 609 tests, 496 pass, 113 fail, 22 files |
| SDK surface law | `bun test ./🧪️tests/🔌️sdk-surface/🟦️.ts` in `SDK` | 6 pass, 0 fail, 498 expects |

Foreign fault found while taking the baseline, handled by a test-only wrapper (not by editing the peer's file): the unwrapped renderer vitest run failed to load **124 of 214 files** (`Failed to resolve import "@semio-tech/framework-os-node-graph-rs/framework_os_node_graph.js"` from a NodeGraph loader a peer created at 14:3x). The renderer vitest config aliases the four wasm packages as string prefixes, so `pkg/file.js` became `<stub>/file.js`. `rhu2-tools/work/vitest.react.config.ts` re-exports the same config with those four aliases matching their whole subpath tree (regex form, the same fix `RESULT-SDK-BOUNDARY.md` section 4.4 applied to `@semio-tech/framework`). Both the before and the after runs use it. The parent should apply the same one-line change to `RT/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`.

## 2. Method

1. Order of work: backbone and directory commands, plugin registry, config, mcp, then the leaves; r3f and the shell twin were examined and left (section 4).
2. A "lowering" here moves the TypeScript module (types, codecs and the pure behaviour they need) to `FM` next to its peers, or, where a module mixes neutral behaviour with a JSON-schema authority that a Rust twin owns, **splits** it: the neutral half moves, the schema-bound half stays in `OS` and imports the neutral half. No JSON file that a Rust crate reads moved; the Rust twins stay where they are (the TS and Rust twins now live in different trees, the same hand-off as `RESULT-DIRECTION-LEFTOVERS.md`).
3. Mechanics, all AST based (`rhu2-tools/`): `move.ts` (a copy of `dlo-tools/move.ts` that also scans the old folder and its parent, because tests import the module as `../🟦️.ts`), `rewire.mjs` (re-points importers by name: static, dynamic, `import type()` and bare package specifiers, with a destination per name), `assemble-schema.mjs` (statement-level split of the directory schema), `edges-for.mjs` + `plan1.mjs` + `plan3.mjs` + `mkfacades.mjs` + `repoint.mjs` (SDK facade and renderer re-pointing by declaration identity), `repath.ts` (pin strings, arrays kept sorted), `unresolved.mjs`, `fnp-check.mjs` (no moved or created framework file imports `🛍️products`: 253 files, 0), `stale-names.mjs` (no importer still names something that moved: 0).
4. Every step re-read each file immediately before the edit; each tool re-reads at write time and skips a file that changed under it (none did).

## 3. What changed

### 3.1 Units and their effect on the rule

| Unit | Moved or extracted | New home | SDK entries | Production edges |
|---|---|---|---|---:|
| 1. Browser actor handoffs | `🔌️plugin/🌐️browser-bundle/🎯️action-handoff` (root, `🎛️command`, `📤️publication`, `📮️requests`, `🧭️intent`), `🩹️patch-handoff`, `🧵️child/🧬️schema`, `💡️inference/🚪️opening` | `FM/🌐️browser-bundle/**`, `FM/💡️inference/🚪️opening` | `./browser-bundle/action-handoff`, `…/requests`, `…/publication`, `./browser-bundle/patch-handoff`, `./browser-bundle/child/schema`, `./inference/opening` | -12 |
| 1b. Published page | `readPublishedPageOrigins`, `publishedPageUrl`, `relocatePublishedRequestUrl`, `installPublishedPageFetch` cut out of `🔌️plugin/📇️registry/📦️deployment`; deployment keeps one side-effect import so the module-load fetch patch is installed exactly as before | `FM/🖼️assets/🌐️published-page` | `./assets/published-page` | -5 |
| 2. Directory wire stack | read model `📇️directory/🟦️.ts`; the **directory schema split** in two (below); `🌐️browser-actor`, `🪪️session-authority-v1` (schema and io), `🚪️io/📝️text`, `🔌️client/🚪️socket-close` (TS and its TS-only JSON), `🔐️sign-in`, `🏘️spaces`, `🎓️first-run`, `🤖️delegations`, `🎫️local-session`, `🪪️session-refresh` and `🪪️session-port`; `📞️directory-client`; `🛰️backbone`; `🪟️view-context`; `component-receipt` value types; the session notice component (to `E/🏛️ShellHost/🔖️session-notice`) | `FM/📇️directory/**`, `FM/📞️directory-client`, `FM/🛰️backbone`, `FM/🌐️browser-bundle/🪟️view-context`, `FM/🏷️component-receipt` | `./backbone`, `./directory`, `./directory/schema`, `./directory/schema/session-authority-v1`, `./directory/io/text`, `./directory/sign-in`, `./directory/spaces`, `./directory/first-run`, `./directory/delegations`, `./directory/local-session`, `./directory/session-refresh`, `./directory/session-refresh/session-port`, `./directory-client` (+ `./inference/service` extended) | -56 |
| 3a. Backbone document binding | `🔌️plugin/📡️backbone/🔗️binding` | `FM/🛰️backbone/🔗️binding` | `./backbone/binding` | -6 |
| 3b. mcp wire twins | `🤖️mcp/🧵️bridge`, `🤖️mcp/🐚️channel` | `FM/🤖️mcp/**` | `./mcp/bridge`, `./mcp/channel` | -8 |
| 3c. dag-input | schema types, io decoders, `📤️selection`, `📊️progress` | `FM/♾️infinite/🎲️board/**` | `./infinite/board/schema/dag-input`, `./infinite/board/io/text/dag-input` | -8 |
| 3d. Renderer composition leaves | `engine/💾️resident`, `engine/🧵️work` (+ its stock JSON) | `FM/🖼️canvas/💾️renderer-resident`, `FM/🖼️canvas/🧵️renderer-work` | `./canvas/renderer-resident`, `./canvas/renderer-work` | -3 |
| 3e. OS config mutations | config schema root **split**; 14 mutation leaves (opening, merge policy, identity, local catalog, local folders, ui preferences); the dispatching aggregator **split**; identity and local-catalog config lanes extracted from the store worker (`🔖️ConfigLane`) | `FM/🎚️config/🧬️schema/**`, `FM/🛰️backbone/🪪️config-lane` | `./config/mutations`, `./backbone/config-lane` | -15 |
| 4. Plugin module contract | trusted-module value types and byte-level file admission cut out of `📦️deployment/🧬️schema`; hub-source store constants (+ their JSON, which no Rust crate reads) cut out of `🌎️hub-source/🧬️schema`; `🌎️hub-source/🔍️resolution` | `FM/🔌️plugin-module/**` | `./plugin-module`, `./plugin-module/hub-source/resolution` | -9 |
| Inline fixes | `import("@semio-tech/framework-os").InstalledServiceStatusV1` and the NodeGraph loader's relative `runtime-assets` import (a peer's new file, one specifier) re-pointed to existing SDK entries | none | none | -2 |

The unit rows are the net change measured by the scoped run after each unit; they sum to 124 (a peer's new NodeGraph loader added one framework edge during unit 1, which the inline fix in the last row removed).

### 3.2 The splits (why they were needed and how they are held lossless)

- **Directory schema** (`📇️directory/🧬️schema/🟦️.ts`, 1671 lines). The renderer needs about 60 of its names; 11 of its statements (document-open plan, execution-target lease parsers and comparators, 349 lines) depend on the `component-receipt` JSON schema authority, which the Rust crate `semio-framework-plugin-component-receipt` owns. The file was split by TypeScript statements: the neutral wire schema (all interfaces, constants and pure functions, 1322 lines) is `FM/📇️directory/🧬️schema/🟦️.ts`; the receipt-bound parsers (`parseDocumentOpenPackageV1`, `parseDocumentOpenPlanV1`, `parseDocumentExecutionTargetLeaseFieldsV1`, `leaseFieldsFromPlanV1`, `sameLeaseFieldsV1`, `sameExecutionTargetV1`, `DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES` and their private helpers) are `OS/🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts` (356 lines). The receipt value **type** was lowered to `FM/🏷️component-receipt/🟦️.ts` (types only) so the lease types are expressible in the framework; the receipt schema, validators and canonical encoding did not move. 54 importers were re-pointed by name. Line multiset (`🗑️generated/rhu2-snap-schema-original.ts` against the two new files): only import and export specifier lines, the two region markers and the new header differ; both files keep their `//#region` markers balanced (11/11 and 1/1).
- **Config root and aggregator.** The JSON-bound items (`osUiPreferencesSchemaDocument`, `UI_PREFERENCE_DATA_CLASSES`, `UI_PREFERENCE_MUTATION_KEYS`, `uiPreferenceMutationDataClassV1`) stay in `OS/🎚️config/🧬️schema/🟦️.ts` and `…/🧬️mutations/🟦️.ts`; everything else moved.
- **Plugin module contract and hub-source store schema.** The value shapes, the refusal class, `utf8OrderV1`, `verifyTrustedPluginModuleFileV1` and the two digest helpers moved; the field validators and the canonical encoding (JSON-bound) stay. `hub-source/🧬️schema` has no Rust twin, so its JSON moved with the five constants; the store-record validation stays in `OS` and reads the JSON from its new place.

### 3.3 Pins and readers updated (exact strings, arrays kept sorted and unique)

- `🔣️taxonomy.json`: 33 `sourceModulePaths`/`inputPatterns` arrays checked after every edit (`rhu2-tools/taxcheck.mjs`: 0 unsorted, 0 duplicate, every `sourceModulePaths` file exists); about 33 lines re-placed.
- `RT/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json` (19 input lines), `OS/📦️packages/🟦️typescript/📋️project.json` (2), fixtures `⚙️config-graph.json` (3), `🧬️generator-ownership/🔣️.json` (5), `🖥️os-source-topology/🔣️.json` (1), `🎯️dag-input/🧪️tests/🧫️fixtures/receivers.json` (3), `🗣️dsl/✨️derive/🗂️schema-document-index.json` (1, the hub-source schema document).
- Channel-version census: the two moved files that hold a version literal (`FM/🌐️browser-bundle/🎯️action-handoff/🟦️.ts`, `FM/📇️directory/🧬️schema/🟦️.ts`) moved from the os consumer list to the framework package list (`FM/🛂️manifest/🔖️channel-version/📇️consumers.json`). I ran the real census (`rhu2-tools/work/census.ts`): 10 findings, none about these two files (the findings are older: `🧾️app-channel/🟦️.ts` unregistered since the previous slice, fixtures, a missing `🧾️describe` JSON).
- Source readers and path strings: the os vitest coverage list (3 entries), `OS/📦️packages/🟦️typescript/📜️script.ts` (reads `patch-handoff` text), three hub tests (`invite-redemption-checks` imports the directory module by path; `directory-authority-checks` and `space-administration-checks` read the schema text), the view-model test that spawns a child on the resident module, the play page test, four handoff/child tests whose fixtures are found relative to the module-under-test URL (kept pointing at the old folder, where the fixtures stay), two config tests with extensionless dynamic imports, two `here()` schema paths. Rust doc comments that name a moved TS twin were corrected in three files (comments only).

### 3.4 SDK surface

31 new entries (all explicit named re-exports, no `export *`, only names a renderer file imports): `./browser-bundle/action-handoff`, `./browser-bundle/action-handoff/requests`, `./browser-bundle/action-handoff/publication`, `./browser-bundle/patch-handoff`, `./browser-bundle/child/schema`, `./inference/opening`, `./assets/published-page`, `./backbone`, `./directory`, `./directory/schema`, `./directory/schema/session-authority-v1`, `./directory/io/text`, `./directory/sign-in`, `./directory/spaces`, `./directory/first-run`, `./directory/delegations`, `./directory/local-session`, `./directory/session-refresh`, `./directory/session-refresh/session-port`, `./directory-client`, `./backbone/binding`, `./mcp/bridge`, `./mcp/channel`, `./infinite/board/schema/dag-input`, `./infinite/board/io/text/dag-input`, `./canvas/renderer-resident`, `./canvas/renderer-work`, `./config/mutations`, `./backbone/config-lane`, `./plugin-module`, `./plugin-module/hub-source/resolution`; `./inference/service` gained two type names. The `exports` map grew from 161 to 192.

## 4. What is left (31 actionable production edges) and the next step for each

| Family | Edges | Why it did not move now | Next step |
|---|---:|---|---|
| `@semio-tech/infinite-world-r3f` | 12 (+3 tests) | A workspace package. Its link `node_modules/@semio-tech/infinite-world-r3f` lives in the dependency image and is an absolute symlink to the source folder, so moving the folder leaves `tsc` unable to resolve it until `bun install` relinks. An SDK facade over it would add a `framework-no-products` edge. | Move the package under `FM/🖱️ui` (it imports only `framework`, `ui-react`, `ui-styling`) in the same step as `bun install`, the workspace list and `📋️build-owners.json`; or let the policy allow the package path. |
| Hub plugin source (`createHubPluginSource`, `HUB_SAME_ORIGIN_MOUNT`, the `?worker&url` service-worker import) and `MODULE_EXTENSION_ROUTE` | 3 | Behaviour, not types: a service worker plus a persistent store, JSON-schema bound validators. | Port: add `hubPluginSource?: (options) => HubPluginSourceV1` and `extensionRoute` to `FrameworkOsBootExecution` (the existing seam, threaded `bootFrameworkOs` to `FrameworkOsShell` to `ShellHost` like `backboneWorkerFactory`), supplied by `OS/🔨️modules/🧑‍💻dev/🟦️.ts` and `✏️s/🧑‍💻dev/🚀️entry/⚛️react/🟦️.ts`, which then own the worker URL import. |
| Plugin registry root (`pluginCatalogV1`, `admitPluginCatalogV1`) | 2 | `📇️registry/🧬️schema/🔣️.json` is a TS-only scope but is read by path from 8 other tests (plugins, hub, dev). | Move TS and JSON together and repath those 8 readers (the hub-source move shows the recipe). |
| Access policy (`directoryAccessPermits`, the policy JSON) | 2 | JSON-bound schema with a Rust twin, plus the policy data file. | Port: `sharedDocumentOpeningRoleV1` takes the set of roles that may write; the composition root derives it from the policy. |
| `engine/🎚️UiPreferences` | 4 (+1 oracle script) | Needs `UI_PREFERENCE_MUTATION_KEYS` and `UI_PREFERENCE_DATA_CLASSES`, both read from JSON that a Rust crate owns. Moving it into `elements` would add two os edges for four removed. | Pass `dataClassOf(mutation)` into the four lane functions (`parseUiPreferenceEnvelopeV1`, `queueUiPreferenceLaneV1`, `commitUiPreferenceOnLaneV1`, `foldUiPreferenceLanePageV1`) from the composition root, then move the module to `FM/🖱️ui`. |
| Shell state reducer (`OS/🔨️modules/🖥️shell`) | 3 | Its state types are generated into the os tree by `bun nx run @semio-tech/framework-os-shell-rs:typegen`; the generator target path is part of the Rust registry. | Change the typegen output to the framework tree first, then move reducer and types. |
| WasmSessionLoader bindings (`framework_surface.js`, `flow_core.js`, flow browser, the NodeGraph package) | 4 (+1 peer file) | A peer rewrote the NodeGraph loader to a package import while this slice ran; the existing seam is `surfaceSessionFactories`. | Make the loader default-free and supply the factories from the composition root. |
| Tooling entries to the repo library (`📜️script.ts`) | 6 | Policy (HO-9a of the previous result). | Parent. |
| Parent-owned Actor and Media | 38 | Not touched. | Parent (`handoff-facade-spec-actor-media.json` is still valid). |

## 5. Decisions worth reviewing

- Module placement uses the os path with `🛍️products/💻️os/🔨️modules` replaced by `🔨️modules` (for example `FM/📇️directory/🧬️schema`), so names that the taxonomy already registers under os are reused. `FM/🔌️plugin-module`, `FM/🏷️component-receipt`, `FM/🌐️browser-bundle`, `FM/🤖️mcp`, `FM/♾️infinite`, `FM/🎚️config`, `FM/📇️directory`, `FM/🛰️backbone`, `FM/📞️directory-client`, `FM/🖼️assets/🌐️published-page`, `FM/🖼️canvas/💾️renderer-resident`, `FM/🖼️canvas/🧵️renderer-work`, `FM/🛰️backbone/🪪️config-lane` and `FM/🛰️backbone/🔗️binding` are new folder names to register (hand-off 1).
- `engine/💾️resident` and `engine/🧵️work` became framework modules instead of ports: they are two shared singletons (one ledger, one stock) that the React and WGPU compositions must share byte for byte; a module-level singleton in the framework keeps that identity without a boot-order dependency.
- Test-only names that have no production user were not given SDK entries (the remaining test edges are mostly JSON fixtures and test-only deep imports, the policy question of HO-9b).

## 6. Hand-offs

1. Taxonomy sweep for the new folder names of section 5 (the previous slices did the same; `tx2` style simulator).
2. Regenerate the schema catalog (`bun ./📜️script.ts schema generate`): scopes whose `🟦️.ts` moved or was split (`os.directory`, `os.plugin.browser-bundle.*`, `os.plugin.registry.hub-source`, `os.plugin.registry.deployment`, the config mutation leaves, `board.dag-input`) list hashes of files that are gone or changed. The catalog was already stale for `os.directory`, `os.shell`, `os.plugin.describe.component-receipt` and 30 more rows before this slice.
3. `RT/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`: regex aliases for the four wasm packages (section 1).
4. Rust docs and Rust twins: the TS twin of each moved scope now lives under `FM`; a later pass can move the Rust twins and their JSON to the same folders.
5. `🎠️kernel/📖️opening-resolver` duplicates config types ("duplicated, not imported"); they can now be imported from `FM/🎚️config/🧬️schema`.
6. Re-run the renderer vitest (section 7 row 5) when the ui fixture entry of the peer slice is finished, and compare with `rhu2-fn-renderer-baseline.txt`.

## 7. Verification (every command and its real result)

| # | Command | Result |
|---|---|---|
| 1 | `rhu2-tools/cruise.sh <label> scoped` at baseline, after each unit and at the end | 535, 522, 451, 408, 397 (production 326, 309, 253, 213, 202) |
| 2 | `rhu2-tools/cruise.sh base-full full` and `final-full full` (exact audit command) | section 8 |
| 3 | `tsc -p .../tsconfig.json --noEmit --incremental false` on the renderer closure | baseline 7 errors; final after the renames (`rhu2-tsc-final2.log`) 13 `error TS` lines: the same six test-file errors (TextEditor draft `splices`, `textEditorSyncPackV1`, World3dHost overload, MediaTransportHost lifecycle, Canvas2dHost jsdom declaration, TaskManager `maxFrames`; none in a file this slice edited) and seven in peers' files that the closure follows: native-build `jobserver` 2 (TS7006) and repo library `caching/vite` 5 (`leaseDirectory`, a peer's in-flight cache-lease change). 0 errors mention a moved or renamed path |
| 4 | `sdk2-tools/tsc-sem.ts` over 1074 (before) and 1094 (after the renames) root files, compared by `rhu2-tools/tscdiff.mjs` with the move records | before 113, after 113, removed 1 (a react-target `script.ts` diagnostic), added 1: `hub/tests/run` TS2724 `VitestPolicyV1` (a peer's change to the repo vitest policy; I did not touch the file). One run at stage 3 caught 11 real errors (two os config tests used extensionless dynamic imports of the split aggregator); fixed |
| 5 | `VITEST_CONFIG=rhu2-tools/work/vitest.react.config.ts zsh rhu2-tools/vitest.sh renderer` (`rhu2-vt-renderer-final2.log`, `rhu2-fn-renderer-final2.txt`) | before 214 files, 31 failed files, 38 failed tests; after 38 failed files, **29 failed tests** (2900 passed of 2929; 168 s). The diff of failing test names shows 0 names that were not failing before and 18 baseline failures that now pass. The 7 additional failed files are 12 test files that fail to **load** (no tests ran), `TypeError: The URL must be of scheme file` at `readFileSync(<name>FixtureUrl)`: retained-toggle-semantics, puzzle3d-settings-document, diff-view-produced-surface, resident-refresh-budget, world3d-camera-framing, icon-svg-lighting, world3d-glb-outline, world3d-scene-shading, projection-render-parity, browser-frame-transport, ink-canvas-clipboard, ink-canvas-editing. Cause: they import the ui module's public fixture entry (`ui/tests/fixtures/` entry file) that a peer slice (test fixture public entries) wrote at 18:28, during my run; its `*FixtureUrl` exports are not file URLs under vitest. I did not touch these tests or that file. A re-run of just those 12 files at 18:31 fails the same way (`rhu2-vt-renderer-final3.log`); they loaded at baseline. This row needs a re-run once that peer finishes |
| 6 | `rhu2-tools/vitest-os.sh base` and `final2` compared by `dlo-tools/vitest-diff.cjs` | 609 tests, 496 pass and 113 fail before; **499 pass and 110 fail after** (run after the renames, 18:10); no file with a new failure (2 fixed in plugin-graph-and-routers, 1 in actor-transport). An earlier run after stage 1 had caught three newly failing handoff/child tests (they find fixtures relative to the module-under-test URL); fixed and re-run (4 of 4 pass) |
| 7 | `bun test ./tests/sdk-surface` in `SDK`, run after the renames | 6 pass, 0 fail, 723 expects (591 before the renames; a peer added entries), including loading every entry in a fresh process and the runtime identity test of every facade |
| 8 | `BASELINE=… node rhu2-tools/vite-resolve2.mjs` (a real Vite plugin container with the repo builders' exact-match alias, importer `E/🔌️PluginRuntime/🟦️.tsx`) | 32 of 32 new specifiers (the root plus the 31 new entries) resolve to the facade files, 0 unresolved |
| 9 | `bun test` of the moved pure modules' own suites: `🤖️mcp/🧵️bridge/🧪️tests` (2 pass, 879 expects), `♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧪️tests` (5 pass, 68076 expects) | pass |
| 10 | `rhu2-tools/unresolved.mjs` over all changed files and the old and new folders; `rhu2-tools/fnp-check.mjs`; `rhu2-tools/stale-names.mjs` for the five split or emptied os files; `rhu2-tools/taxcheck.mjs` | 0 unresolved relative imports; 253 framework files with 0 imports into `🛍️products`; 0 stale names; taxonomy arrays valid |

## 8. Final full cruiser run (the exact audit command)

Command (`rhu2-tools/cruise.sh final-full full`): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs <framework> <s> <hub> <mit-bestand> --config <repo lint dependency-boundaries config> --output-type err`, the same roots and config as the baseline (the literal command is in `rhu2-tools/cruise.sh`). Outputs: `generated/rhu2-base-full-err.txt` (14:35 to 15:04, 17520 modules) and `rhu2-final-full-err.txt` (18:05 to 18:25, 17900 modules, exit 100 as the baseline's 102), rule counts in `rhu2-*-full-rules.txt`. The first final attempt (17:14) was invalid: the folder renames changed the tree under it and its 50-minute alarm fired at machine load 50; the second run used a 7000 s alarm and no source edit of mine happened during it.

| Rule | Baseline | Final | Delta |
|---|---:|---:|---:|
| `renderer-hosts-only-ui` | 535 | **391** | -144 |
| `framework-no-products` | 149 | 167 | +18 |
| `no-circular` | 583 | **574** | -9 |
| all violations | 13926 | 10596 | -3330 (peers' concurrent work dominates) |

`renderer-hosts-only-ui`: 195 edges gone and 51 new by exact source-and-target text. The 51 are re-spellings and tree movement, not new coupling: 25 point at test fixture files of framework and os modules (test files whose imports were re-pointed to the fixture folders; the same sources carried an edge before), 9 are the `react` resolver artefact (new spelling for moved or renamed files such as the ShellHost session notice), 5 are tooling entries (`script.ts`), and 12 are others: Actor wire and shard-client edges owned by the parent (PluginRuntime, TaskManager), the NodeGraph loader and its tests that a peer created during the slice, `plugin-lifecycle` to the hub source factory (documented in section 4), and two UiDocumentStore tests whose inline `import()` type forms and dynamic imports now spell the framework path.

`framework-no-products`: +18 edges, none from a file this slice created, moved or edited (24 new, 6 gone). The new ones are: the SDK package's own products-mirroring facade folder to library files (4, the SDK-boundary slice), `script.ts` entries of the rust, typescript and react packages to the repo library (7, the class of the previous result's hand-off), framework native-build and process test files to the repo library build test support (11, a peer's native-build test support) and one retirement-context test to an os plugin test-support file. My own check (`rhu2-tools/fnp-check.mjs`) over the 258 files I placed in the framework modules reports 1 violation, which is a peer's edit in `SDK/script.ts` (an import of the repo library).

`no-circular`: 583 to 574.

## 9. Found state, peers, foreign issues

- No predecessor of this slice (the previous slice `rhu` was complete). Peers changed files under me: the NodeGraph loader and its runtime package (created while I measured), `🔣️taxonomy.json`, `SDK/package.json` (a peer added a script line). Every write of mine re-read the file immediately before and changed anchored places.
- Foreign: the wasm alias prefix (section 1), the repo vitest-policy export (section 7 row 4), the older channel-version census findings (section 3.3), `rhu2-tools/work/census.ts` shows them.
- Foreign, found at the end: (1) `SDK/script.ts` imports the repo library (the only `framework-no-products` violation among files I touched or created, and not my edit); (2) 12 renderer test files cannot load since a peer's ui fixture entry was written at 18:28 (section 7 row 5); (3) renderer `tsc` errors in the native-build `jobserver` and the repo library `caching/vite` files (section 7 row 3); (4) the final-run `framework-no-products` growth of 18 (section 8).
- Folder names: the first drafts of `FM/backbone`, `FM/mcp` and `FM/plugin-module` reused emojis of sibling folders; they were renamed to the backbone satellite, robot and plug emojis, with moves, repaths, `package.json` exports and facades updated; the final cruiser run and all checks in section 7 were taken after that.
- The auto-commit stages work continuously, so `git status` cannot attribute my changes; counts here come from the tools' own tables.

## 10. Files changed

- Moved (56: 53 TypeScript and 3 TS-only JSON) and 10 files created by extraction or split: listed per unit in section 3.1; the move records are `rhu2-tools/spec-*.json`.
- Re-pointed importers: about 110 renderer files (imports and exports only) and about 250 file edits in os, hub, dev and plugin tests and modules (specifiers and a few path strings; the steps overlap), listed by the tools' tables in `🗑️generated/rhu2-s*-edges.jsonl` and the pre-edit snapshots `🗑️generated/rhu2-snap-*.json` (197 unique files).
- SDK: `package.json` (+31 `exports`), 31 new facades, 1 extended.
- Pins and readers: section 3.3.
- Not touched: the Actor and Media edges, the policy file `🟨️.cjs`, `AGENTS.md`, `.vscode/launch.json` (no new test entry point was created).

## 11. Tools (`rhu2-tools/`)

`cruise.sh`, `edges.mjs`, `final-table.mjs`, `prod-targets.mjs`, `rulecount.sh`, `edges-for.mjs`, `plan1.mjs`, `plan3.mjs`, `plan-origins.mjs`, `origins2.mjs`, `refresh.sh`, `mkfacades.mjs`, `repoint.mjs`, `rewire.mjs`, `barrel-map.mjs`, `move.ts`, `split-schema.mjs`, `assemble-schema.mjs`, `clos.mjs`, `importers-of.mjs`, `names-of.mjs`, `unresolved.mjs`, `stale-names.mjs`, `fnp-check.mjs`, `taxcheck.mjs`, `pins.mjs`, `roots.sh`, `tscdiff.mjs`, `vite-resolve2.mjs`, `vitest.sh`, `vitest-os.sh`, `failnorm.mjs`, `spec-*.json`, `map-*.json`, `facade-spec-s*.json`, `work/vitest.react.config.ts`, `work/census.ts`.
