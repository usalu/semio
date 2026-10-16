# Result: Interpreter registry inversion and split, TS kernel split (slice `ikt`)

Executor: Sonnet, foreground, no sub-agents, no modifying git command, no ticket tool. Worked 2026-10-16 05:00 to 06:40. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `IP` = `E/🗣️Interpreter`, `K` = `🧰️framework/🔨️modules/🎠️kernel`. Tools: `ikt-tools/` (`importers.mjs`, `repoint.mjs`, `split.mjs`/`write.mjs`/`analyze.mjs`, `restore-regions.mjs`, `glue.mjs`, `coverage.mjs`, `multiset.mjs`, `scopediff.mjs`, `vitest.sh`, `tsc.sh`, `taxonomy.sh`, `cfg-interpreter.mjs`, `cfg-kernel.mjs`). Logs and snapshots: `🗑️generated/ikt-*` and `🗑️generated/ikt/`.

## 0. Verdict

| Target | Result |
|---|---|
| Interpreter inversion | done. `IP/🟦️.tsx` imports no concrete host (section 2). The composition root `E/🪢️ElementHosts` registers the 14 scene hosts, the media transport extension and the document selection. 11 new tests pass. |
| Interpreter split | done. 3,259 lines became a 150-line entry (`InterpretedUiNode`, `interpretUiNode`, the in-source test block) plus 18 module children (58 to 586 lines) plus the registry leaf. |
| Kernel split | done. The 4,038-line `K/🟦️.ts` is deleted; 34 children (11 to 595 lines) hold its 280 code units; the package entry `K/📦️packages/🟦️typescript/🟦️.ts` lists the public names and the 11 pre-existing re-exports. |
| Importers | every importer re-pointed (23 Interpreter, 56 kernel); the coordinator's note about `🛂️manifest/🟦️.ts` was already satisfied by the first re-point (section 5). |
| Failure sets | renderer suite: strictly smaller than baseline, nothing new. Kernel suite: identical (91 of 91 pass). Type-check: no new error, none in any file I created. |

## 1. Baseline (recorded before the first edit, 05:05 to 05:16)

- `IP/🟦️.tsx`: 3,259 lines, 156 units, sha256 `9748a9a4c841...` (snapshot `🗑️generated/ikt/orig-Interpreter.tsx`). Importers (resolved relative specifiers over `git ls-files` plus untracked, `importers.mjs`): 20 files, 24 specifiers (`importers-interpreter-baseline.tsv`).
- `K/🟦️.ts`: 4,038 lines, 291 units, sha256 `89f531857095...` (`orig-kernel.ts`). Importers: 52 files, 76 specifiers (`importers-kernel-baseline.tsv`) plus the framework root barrel `export *`. A peer changed one import line (`StoragePort` source) at 06:01; the split used that text (`base-kernel.ts`).
- Type-check: `node_modules/.bin/tsc --noEmit -p tsconfig-baseline.json` (77 files: both targets, importers, React barrel, ShellHost, World3dHost, Shell, NodeGraph, Table): 73 errors, none in `Interpreter`, none caused by the kernel parent (`tsc-baseline.log`).
- Renderer suite (`SEMIO_TEST_LEVEL=long`, hand-built `SEMIO_VITEST_POLICY`, `ikt-tools/vitest.sh renderer`): 184 files, 31 failed files, 44 failed tests of 3,022 (`🗑️generated/ikt-vitest-renderer-baseline.log`, normalized `fn-renderer-baseline.txt`).
- Kernel suite (`vitest.sh kernel`, config `K/🧪️tests/🎚️config/🟦️.ts`): 9 files, 91 tests, all pass. `bun test K/🧪️tests/📦️receipt`: 6 pass. The other kernel test modules are `registerTests*` libraries run by the in-source blocks or by the root script's `verify` routes.
- Taxonomy: scope `IP` baseline `clean=false errors=3` (`reference-preimage-unreadable` on three Interpreter test files). Scope `K` crashes inside the normalizer both before and after (`Required taxonomy tokens[index - 1] is missing`, exit 1), so it cannot gate this work.

## 2. Interpreter inversion

Finding: `IP/🟦️.tsx` imported 14 scene hosts (Canvas2d, World3d, NodeGraph, TextEditor, Table, Paint2d, TiledMap, Board2d, IconRender, InkCanvas, GraphTimeline, BlockList, DiffView, EventFeed), `MediaTransportHost` (extension) and three document-selection functions from `World3dHost`. Each host already imported only Interpreter leaves, never the parent.

Design (`IP/🗺️element-host-registry/🟦️.ts`, a leaf with no value imports, so it is always evaluated first):

| Contract | Shape |
|---|---|
| scene hosts | `registerComponentSceneHostV1(kind, () => Component)` returns a retire function; `resolveComponentSceneHost(kind)` reads the thunk at render time (the old comment's cycle hazard: a host captured at module load can be `undefined`). The old `resolveComponentSceneHost` switch and its region `ComponentSceneHostRegistry` moved here. |
| extension hosts | `registerExtensionHostV1((extensionId) => Component or undefined)` and `resolveExtensionHost`; the id comparison happens at render time. |
| document selection | `registerLeftoverSelectionPortV1({ subscribe, overlay, itemSelected })` with stable module-level readers `subscribeLeftoverSelectionV1`, `leftoverSelectionOverlayV1`, `leftoverItemSelectedV1` for `useSyncExternalStore`; default is "no selection". |

Composition root: `E/🪢️ElementHosts/🟦️.ts` exports `registerElementHostsV1()` (registers all of the above, returns the retire function, every host and constant read lazily). It is called at module load by the two roots: the React package barrel `🎯️targets/⚛️react/🟦️.tsx` and the shell `🏛️ShellHost/🟦️.tsx` (the shell mounts windows). `ExtensionView` now asks `resolveExtensionHost`; the tree rows ask the selection port.

Load-order finding (it broke my first two designs, both caught by the new test): the roots are inside the hosts' import cycle (`ElementHosts` to `World3dHost` to ... `Shell` to `ShellHost` to `ElementHosts`), so a root runs while host modules are still evaluating. Eager reads (`MEDIA_TRANSPORT_EXTENSION_ID`, the port's three functions) and module state in `ElementHosts` itself threw `Cannot access ... before initialization`. Final rule: registration reads nothing eagerly and keeps no state outside the leaf.

Evidence that no host is imported any more: `git grep` over `IP` children for the 15 host module specifiers returns only `🪢️ElementHosts` and the registry test (the test imports the hosts to compare identities). Remaining non-host wrong-direction edge, not fixed: `ShellFaultBoundary` is still imported from `🐚️Shell/🟦️.tsx` (in `🔌️surface-bridge` and `🔤️display-views`); moving it needs the Shell parent (owned by `ri2`).

## 3. Interpreter split (module-level children, DAG verified: 0 cycles)

Cut with the hash-guarded tool of the siblings after phase 0 (the inversion above); regions restored by `restore-regions.mjs` (21 original region names, 23 in the tree, 0 missing, 0 unbalanced). Children are `IP/<dir>/🟦️.ts(x)`; every file opens with a docstring starting with its folder emoji; no comments inside definitions.

| Child | Lines | Holds |
|---|---:|---|
| `👥️presence-overlay` | 58 | presence overlay types and contexts, `usePresenceOverlayEntry`, `useUiPresenceOverlay` |
| `🌲️tree-panel-boundary` | 167 | panel tree payload to `TreePanelConfig`, catalogue drag MIME, drag controller |
| `🧷️interpreter-context` | 41 | `UiInterpreterContext`, `PluginSurfaceActionsContext`, `interpLabel` |
| `🔌️surface-bridge` | 222 | kind tables, host identity, scene node assembly, gesture funnel, registry-resolved host render |
| `🚚️surface-scene-lanes` | 101 | lane carrier cache and lane text assembly |
| `🗄️virtual-file-system` | 101 | VFS navigation and host |
| `📐️layout-accessibility` | 156 | layout and style props, ARIA props |
| `⚡️action-dispatch` | 73 | control icon, `dispatchTrigger`, continuous trigger lane (hook), `toUiValue` |
| `🎛️declarative-controls` | 153 | declarative control boundary |
| `🔤️display-views` | 160 | text, button, key-value, image, progress, extension, unknown views, busy shell |
| `✍️input-views` | 476 | commit draft and picker release hooks, input, select, toggle, slider, stepper, ring, icon select |
| `🌳️tree-walk` | 70 | tree item collection, row tones, status section |
| `🪪️stable-dom-ids` | 57 | DOM ids and React reconciliation keys |
| `🪟️tree-windows` | 586 | window context, measurement, request math, `useTreeWindowObserver` (hook), pick and open-state rules |
| `🧾️row-actions` | 127 | row action builders, button, `useRowActionAdmissionV1` (hook) |
| `🖼️surface-views` | 135 | carrier epoch hook, paged surface, accessibility shell, surface view |
| `📊️table-window` | 81 | windowed table rules |
| `🧱️node-tree` | 582 | `UiNodeView`, the component switch, container, tree and table views, tree item rows |
| parent `🟦️.tsx` | 150 | header, `interpretUiNode`, `InterpretedUiNode`, the unchanged in-source test block |

Cycle decision: `UiNodeView` to `renderComponent` to {container, tree, table views} to `UiNodeView` is one unit-level strongly connected component. Cutting it needed a global renderer slot, so the recursive views stay together with the dispatch in `🧱️node-tree`; every non-recursive view is a leaf it imports. The hooks that exist as standalone `use*` functions each sit in their concern's child; `TableView` (227 lines, one closure of handlers) was not decomposed into a hook chain, it stays inside `🧱️node-tree` (below the size limit; follow-up if wanted).

Losslessness: `coverage.mjs` over p0 (the original after the inversion): 155 of 155 top-level units appear exactly once in the tree. `multiset.mjs` (line multiset ignoring blank, region, import and export-from lines): 43 lines "lost" against p0, all import-name lines redistributed by the splitter plus the three header lines I edited; no code line lost.

## 4. Kernel split (34 children, DAG verified: 0 cycles)

Public API: the 227 exported names plus the 11 pre-existing `export ... from` statements are listed by `K/📦️packages/🟦️typescript/🟦️.ts` (generated by `glue.mjs`, explicit names, value and type exports apart); the framework root barrel now does `export * from` that file. The parent `K/🟦️.ts` is deleted, no alias is left.

| Child (`K/<dir>/🟦️.ts`) | Lines | Holds |
|---|---:|---|
| `🛂️descriptor-admission` | 118 | descriptor fetch and admission, package descriptor, package assets lookup |
| `📡️turn-outcome-broadcast` | 76 | `TurnOutcome`, `createTurnOutcomeBroadcast` (+ its in-source test block) |
| `🪝️plugin-wasm-handle` | 37 | `PluginDispatchHintV1`, `PluginWasmHandle` |
| `🧾️contributions-json` | 57 | build, scope, layout resolution |
| `🗂️plugin-registry` | 79 | registry entry, dependency pins, `expandPluginRegistry` (+ test block) |
| `🔖️artifact-contribution` | 51 | contribution metadata types |
| `🔀️dialect-equality` | 15 | shared dialect and app-ref equality |
| `🧭️app-router` | 224 | `AppRouter`, surface fault codes (+ test block) |
| `🚪️io-router` | 228 | IO entry graph, `ioRun`, `ioIdentify` (+ test block) |
| `📖️opening-resolver` | 121 | opening preferences and `resolveOpeningApp` |
| `📚️plugin-catalog` | 105 | catalog targets, host config, activation prefixes |
| `🧬️mutation-vocabulary` | 53 | timestamps, diffs, undo, mutations, app events |
| `🚨️fault` | 74 | severity, fault types, `SemioFaultError` |
| `⚡️effect` | 117 | the `Effect` union, service payload |
| `🎬️media-export` | 46 | export encodings and decoding |
| `🎥️video-render-program` | 170 | program limits, scene types, admission, wire decoding |
| `🧵️video-render-job` | 93 | job events, rows, ledger |
| `🗄️file-open-import` | 77 | import chunks and argument envelope |
| `🛠️spawned-job` | 70 | placement, drive limits, completion, error |
| `🎯️ui-dirty-scope` | 125 | dirty scope predicates, window layout ids, merge |
| `🕰️history` | 213 | history wire types, reprojection, patch |
| `🏷️notice-labels` | 144 | history and framework fault notices |
| `🧮️invocation-response` | 49 | response type and parser |
| `🔄️merge-outcome` | 151 | merge policy, conflict, reports |
| `🌉️plugin-backbone-routing` | 83 | main-thread relay (module-load global assignment), inbound push, routes |
| `🚦️activation-reason` | 68 | reason type, activation event envelope, reason for an app id |
| `🧯️memory-pressure` | 54 | memory probe and resident cap |
| `⏱️runtime-metrics` | 38 | metrics rows, snapshot, due rule |
| `🎛️activation-registry` | 595 | `ActivationRegistry`, grants, default fetchers (+ test block) |
| `🔌️plugin-source` | 250 | dev, bundled, extension, multiplexed sources (+ test block) |
| `🎮️playground-boot` | 155 | playground resolution and boot planner |
| `🕸️plugin-graph` | 257 | version pins, validation, load order, graph, dependency fault message |
| `📍️instance-directory` | 37 | instance reference and directory |
| `🧲️artifact-routers` | 149 | ownership, conflict-checked registry, mutation and inference routers |

Cycle cuts made by moving an item to its rightful owner: `PluginHostConfig` (type, was in the playground region) moved to `📚️plugin-catalog`, whose `PluginCatalog` names it; `activationReasonForAppId` moved from the catalog region to `🚦️activation-reason`, because `ActivationReason` belongs to activation and the catalog must not import it back.

Behaviour notes: (1) `🌉️plugin-backbone-routing` assigns `globalThis.__semioMainThreadPluginBackboneOutbound` at module load; the module is imported by anyone who registers a route and by the package entry, which is every path that could use the relay. (2) The `🎭️actor` imports (`ShardClient`, `TurnScheduler`) are unchanged, in `🎛️activation-registry` only.

In-source tests: the six `import.meta.vitest` blocks moved with their subjects; their `./🧪️tests/...` specifiers became `../🧪️tests/...` and `directory` now names the kernel root (two `dirname` calls; `new URL("..")` failed under jsdom). `TestSource.directory` replaces `source.url` for the one fixture lookup. Configs: `K/🧪️tests/🎚️config/🟦️.ts` lists the six children in `includeSource` (coverage `*/🟦️.ts`); `🧰️framework/🧪️tests/🎚️config/🟦️.ts` the same in both keys.

Losslessness: `coverage.mjs`: 274 of 280 non-export-statement units appear exactly once; the other 6 are the in-source blocks, matched by their `registerTestsN` tag after the intended specifier rewrite; the 11 export-from statements are in the package entry verbatim. `multiset.mjs`: 20 lines "lost": the 12 block lines just named, the 8-line multi-line `import type {` of the original (regenerated per child) and the header docstring (moved to the package entry); no code line lost.

## 5. Importers, readers, config (every re-point)

All importer specifiers were rewritten by AST (`repoint.mjs`: named imports and re-exports split per owning child, dynamic `import()` destructures, `typeof import("...")` inside `Pick<...>` split into `&` of Picks, `import("...").Type` qualifiers). Bare string needles (`git grep -F '🎠️kernel/🟦️.ts'`) now find only Rust and JSON comments, listed under hand-offs.

- Interpreter importers (23 files): React barrel `🎯️targets/⚛️react/🟦️.tsx`; tests `♿️editable-controls`, `♿️retained-toggle-semantics`, `⚙️puzzle3d-settings-document`, `🎚️window-measure-controls`, `🎞️timeline-paint-vfs-hosts`, `🎬️playbook-scene-showcase`, `🎬️row-action-admission`, `📋️renderer-types-and-forms`, `📖️procedural-descriptors`, `🔒️shell-option-locks` (peers' split of engine-contract), `🗄️stdio-details-tree`; ShellHost parent, `📋️panel-hosting`, `🧩️shell-core-state`; ShellHelpers `⏪️time-travel`, its component test, `🌳️tree-windows`, `🧪️tests/🪟️tree-windows`; four Interpreter tests (`continuous-presses`, `number-keyboard-law`, `text-keyboard-law`, `unknown-component-placeholder`). Edits inside `ri2`-owned `🏛️ShellHost/**`: those three import lines plus two lines in the parent (`import { registerElementHostsV1 }` and its call).
- Kernel importers (56 files): `📦️packages/🟦️typescript/🎠️kernel/🟦️.ts` (a peer's new SDK entry), `🧩️extensions`, the 17 kernel tests, `🎭️actor/🖼️wire-turn` (2), `🛂️manifest/🟦️.ts` (now `⚡️effect`, so the coordinator's `generate-graphql-types` failure is resolved), `🛂️manifest/🧪️tests/🧪️fault-notices`, the wgpu `🧩️plugin-modules`, PluginRuntime (7 files), ShellHelpers (7), TaskManager (2), VideoRenderHost (2), Shell `🧩️catalog`, the plugin registry composition test, the dev staging test, `backbone-envelope-io`, `generation3d` `interaction-scope`, and the peers' new tests `shard-loss-and-boot`, `document-pack-wire-adapter`, `extension-invocation-wire`, `turn-dispatch-lifecycle`, `typed-operation-completion`, `ui-intake-admission`, `mounted-window-fetch`, `window-lifecycle-template-drag`, `plugin-load-progress`. Log: `🗑️generated/ikt-repoint-kernel.log`.
- Hand re-points: `🌳️wgpu-document-reconcile` test now `readModuleTreeSource` for the React Interpreter source; `🎚️ring-press` (UI test) reads `✍️input-views` and strips the `export` the splitter added before compiling the `RingView` declaration (5 of 5 pass); `ReactTarget 📜️script.ts` surface-retention oracle reads the Interpreter module tree; `🧪️tests/📦️receipt` "the general kernel imports no specific receipt implementation" now checks the package entry and every kernel child module (it read `K/🟦️.ts`); `🧪️createturnoutcomebroadcast` (dropped the dead `KernelTestModule`, `source.directory`), `🧪️tests/📦️receipt` dynamic import split.
- Config and inputs: `🖱️ui/📦️packages/🦀️rust/📋️project.json` (Interpreter glob), `🖱️ui/.../rust/📋️project.json` and `renderer wgpu 📦️packages/🟦️typescript/📋️project.json` (kernel glob `🎠️kernel/*/🟦️.ts`), React target test config (new `elementSuite("🗣️Interpreter", "🗺️element-host-registry", "tsx")`), root `K`-types check in the mutation-outcome law (`TS kernel types` surface now `🔄️merge-outcome`), dependency-cruiser self-check `🏗️construction/🟨️.cjs` (kernel entry and activation registry), `🔣️taxonomy.json` (19 Interpreter child names in `members-of-members-of-elements`, `🪢️ElementHosts` in `members-of-elements`, and the 31 runtime-closure kernel module paths replacing the parent in the two `wgpu-frame-worker` `sourceModulePaths` lists, computed by `wgpu-closure.mjs`).
- Comments: TS comments naming `🎠️kernel/🟦️.ts` re-pointed in 14 files.

## 6. Verification, baseline vs after

| Check | Baseline | After |
|---|---|---|
| tsc, scoped to my files (Interpreter tree, ElementHosts, kernel tree, every importer old and new; `scopediff.mjs`) | 27 errors | 24 errors, 0 new; the 3 gone are peers' fixes |
| tsc `--noUnusedLocals`, same scope | n/a | 1 hit in my files (`childKey`, unused in the original too); no unused generated import |
| renderer suite (full) | 31 files / 44 tests failed of 3,022 | 31 files / 40 tests failed of 3,033 (11 new tests, all pass); failing set equal to baseline minus 4 flaky tests (3 Paint2d `selection-focus`, `embedded-mount`); no test fails that did not fail before (`fn-renderer-final.txt` vs `fn-renderer-baseline.txt`) |
| kernel suite (`vitest.sh kernel`) | 9 files / 91 tests | 14 files / 91 tests, all pass (the one in-source file became six) |
| `bun test` `📦️receipt` | 6 pass | 6 pass (after the reader fix; 5 pass before it) |
| kernel contracts run through the children: `testUiDirtyScopeContract`, `testVideoRenderJobContract`, `testFileOpenImportContract` | n/a | all three ran to completion |
| ring-press (`bun test`) | not recorded | 5 pass |
| dependency graph | n/a | Interpreter tree 0 SCCs of size above 1 over 26 module files; kernel tree 0 SCCs over 44 module files (`dagcheck.mjs`); `grep` for the 15 host module specifiers in the Interpreter parent and children finds none |
| taxonomy `IP` | 3 errors | the same 3 errors, none naming a new folder; `🪢️ElementHosts` scope `clean=true errors=0`; `K` scope crashes before and after |
| max path length | n/a | longest repo-relative path of any file under `IP` or `K` is 180 characters |

Runtime proof of the registry wiring: `IP/🧪️tests/🗺️element-host-registry/🟦️.tsx` (11 tests, passes). It checks: importing the shell module registers all 14 kinds with the host modules' own components, the media transport extension and the world's document selection; a real `diff-view` surface mounts through `UiNodeView` with no placeholder; with the registry cleared an unregistered kind shows a visible placeholder (never blank); a probe registered for `diff-view` receives the assembled `componentScene` node and `onAction`, and is gone after its retire function; newest registration wins and retiring the older one is a no-op; a thunk that resolves later still mounts; extension resolver and selection port default to nothing; `registerElementHostsV1()` and its retire function cover exactly those registrations and `publishLeftoverWorldSelectionV1` reaches the port. The renderer tests that mount hosts through the Interpreter (`🧪️tests/🎚️window-measure-controls`, `⚙️puzzle3d-settings-document`, `Interpreter/🧪️tests/🪪️container-node-ids` with a node-graph surface, ShellHost mounts) are in the unchanged-or-smaller failure set.

## 7. Not done, hand-offs, found state of peers

1. Rust doc comments and two JSON notes still name `🎠️kernel/🟦️.ts`: `K/🎞️media-jobs/🦀️.rs` (5), `K/🖼️ui/🦀️.rs`, four `K/🧪️tests/*/🦀️.rs`, `🖱️ui/📜️declaration/📐️layout/🦀️.rs`, `🔌️plugin/🖥️host/**` (3), two `generation3d`/`flow-operators` Rust tests, `K/🧬️schema/🔣️history-patch/🔣️.json`, `🧪️tests/🧫️fixtures/🪟️app-mode-layouts/🔣️.json`. They are comments or descriptive text, not readers; I left them to the Rust owners and the fixture owners.
2. `wgpu-frame-worker` generated artifacts (worker bundles and `catalogSha256`) embed kernel code; they were already drifting at baseline (the six `package-integration` failures). The `sourceModulePaths` allow-list is updated, but the artifacts need `generate-wgpu`.
3. `K/📦️packages/🟦️typescript/🟦️.ts` is a list, not a barrel kept for compatibility: it is the declared package entry (`"." -> ./🟦️.ts`), formerly `export *` of the deleted file.
4. `🧰️framework/📦️packages/🟦️typescript/🎠️kernel/🟦️.ts` (a peer's `@semio-tech/framework/kernel` SDK entry) was re-pointed to `🎯️ui-dirty-scope`; its `package.json` subpath `./kernel` is a different file and is unchanged.
5. Peers rewrote the tree while I worked: `🔬️engine-contract` was split into many test directories, the root `📜️script.ts` was split (my edit survived into `⚖️laws/🧩️plugin-consistency/🎯️mutation-outcome`), `🏛️ShellHost` children changed. Importer lists were rescanned immediately before each write. The framework-level vitest config (`🧰️framework/🧪️tests/🎚️config`) found no files when run with the hand-built policy, so only the kernel config was used for the in-source suites.
6. The first `bun test` run of ring-press died with a Bun segmentation fault while the machine ran out of file table (`ENFILE`); reruns are clean.
7. Not run: browser or GPU boot, `bun repo run ...`, ESLint `react-hooks`, repo-wide `verify taxonomy`, Rust gates, `generate-wgpu`.

## 8. Files touched

Created: `IP/🗺️element-host-registry`, 18 Interpreter children (section 3), `IP/🧪️tests/🗺️element-host-registry`, `E/🪢️ElementHosts`, 34 kernel children (section 4).
Rewritten: `IP/🟦️.tsx` (3,259 to 150 lines), `K/📦️packages/🟦️typescript/🟦️.ts` (package entry).
Deleted: `K/🟦️.ts`.
Edited: the importers, readers, configs, taxonomy and comments listed in section 5.
Temporary: `ikt-tools/`, `🗑️generated/ikt-*`, `🗑️generated/ikt/` (no `[DEBUG]` logging was added to any source file).
