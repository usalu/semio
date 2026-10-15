# Result: NodeGraph cycle cut, module-level split and `FlowGraphCanvasHost` hook decomposition

Executed 2026-10-15 by the Sonnet executor. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `NG` = `E/🕸️NodeGraph`.
Seam map used: `SEAM-MAP-TSX-TIER2.md` section 2.F. Approach and tooling reused from `RESULT-SHELLHOST-SPLIT.md` and `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md`; region restore follows section 9 of `RESULT-OS-TS-R3F.md`. All tooling, snapshots and logs live in `🪓️monolith-split/🗑️generated/nodegraph-split/` and `🪓️monolith-split/🗑️generated/nodegraph-*` (temporary helpers, not repo tooling).

## 1. Outcome at a glance

| File | Before | After |
|---|---|---|
| `NG/🟦️.tsx` | 4,005 lines | 627 lines: header, import block, 7 `export ... from` lists, the unchanged `NodeGraphHost` cluster (`useClient`, `PresencePeersOverlay`, `NODE_GRAPH_HOST_CLASS`, `nodeGraphSurfaceSelectionDomV1`, `NodeGraphHost`) and a thin `FlowGraphCanvasHost` (7 hook calls, one destructure, the original 276-line JSX `return`) |
| module-level children | 0 | 8 modules, 76 to 860 lines |
| closure hook children | 0 | 7 modules, 66 to 291 lines |
| Phase 0 leaves (under `E/🗣️Interpreter`) | 0 | 2 modules (79 and 148 lines) |

Targets met: parent at most about 1,000 lines (627), every child at most about 2,500 lines (largest 860). The cruiser reports `🕸️NodeGraph` in no import cycle any more (3 violations before, 0 after; the 86-module strongly connected component that contained it no longer does).

Public API is unchanged. Checked with the TypeScript checker (`getExportsOfModule`, name plus value/type kind), original text vs final parent: 86 exports, identical, 0 unresolved (`exports-orig.txt` vs `exports-final.txt`). A second check also compares each export's printed type (`exports-types.mjs`): identical for 85 of 86; the 86th is `FlowGraphCanvasHost`, printed as `(props: FlowGraphCanvasHostProps) => Element` instead of the same parameter type spelled inline (the alias body is the original inline type, verbatim).

## 2. Phase 0: the runtime cycle

### 2.1 Finding

Baseline cruiser run over `E` (dependency-cruiser, repo config `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs`, `--output-type json`, `nodegraph-depcruise-baseline.json`, 1,406 modules): `🕸️NodeGraph/🟦️.tsx` sat in a strongly connected component of 86 modules together with `Interpreter`, `ShellHost` and all its children, `ShellHelpers` and its children, `World3dHost` and its children, `Table`, `Board2dHost`, `TiledMapHost`, `PluginRuntime` and others. NodeGraph's three offending imports were:

- `Interpreter` barrel: `openSurfaceContextMenu`, `parseSceneJsonField`, `useAppCatalogue`, `useShellContextMenuFallback`, `SurfaceContextMenuResult`.
- `ShellHost` barrel: `useAppKeybindingsByActionId`, `useMapContextMenuSpecs`.
- `World3dHost` barrel (the seam map missed this one): `mapContextMenuSpecs`, `parseJsonArray`, `parseSelectionDomainsFromSession`, `selectionGroupsFromDomains`, `WindowInstanceIdContext`. `World3dHost` imports `Interpreter` (`wireLabel`) and `ShellHost`, so this edge alone kept NodeGraph in the loop even after the first two were cut.

### 2.2 Cut: two shell-neutral leaves, declarations moved, nothing duplicated

Taxonomy-wise the leaves are children of `Interpreter` (the element that already owned the scene-host contexts; the same member-of-element registration the siblings use). Neither leaf imports `Interpreter`, `ShellHost`, `World3dHost` or `ShellHelpers`' barrel.

| Leaf (`E/🗣️Interpreter/<dir>/🟦️.ts`) | Lines | Holds (moved verbatim) | Former home |
|---|---|---|---|
| `🌉️scene-host-contract` | 79 | `wireLabel`; `AppCatalogueContext`, `useAppCatalogue`; `WindowInstanceIdContext`; `parseSceneJsonField`; `parseJsonArray`, `parseSelectionDomainsFromSession`, `selectionGroupsFromDomains` | `Interpreter`, `World3dHost/🪟️window-view-store`, `World3dHost/🔗️instance-selection` |
| `🖱️surface-menu` | 148 | `ShellContextMenuFallbackContext`, `useShellContextMenuFallback`, `SurfaceContextMenuResult`, the two title-key tables, `surfaceContextMenuTitleKey`, `openSurfaceContextMenu`; `AppKeybindingsContext`, `useAppKeybindingsByActionId`, `useMapContextMenuSpecs`; `mapContextMenuSpecs` | `Interpreter`, `ShellHost/🎚️shell-model`, `World3dHost/🔗️instance-selection` |

`🖱️surface-menu` imports `contextMenuGroupLabel` from the concrete child `ShellHelpers/🏷️labels` and `wireLabel` from the contract leaf. `🏷️labels` used to import `wireLabel` from the `Interpreter` barrel (the `labels -> Interpreter -> ShellHelpers` cycle of the seam map); it now imports the contract leaf, so there is no `labels <-> leaf` loop.

Edits outside `NG` (every one is a deletion of the moved block plus a re-export so every existing import specifier and the React target's barrel lists stay valid; each file re-read right before the edit, anchors asserted unique, `phase0.mjs`):

- `Interpreter/🟦️.tsx` (3,361 to 3,272 lines): the blocks are gone, `export { ... } from` the two leaves added after the adapter region, its own uses import from the leaves, and its `useMapContextMenuSpecs` import now comes from the leaf instead of `ShellHost` (that also removes an `Interpreter -> ShellHost` edge). Imports left unused by the move (`ContextMenuItem`, `AppCatalogue`, `decodeScenePackField`) were removed.
- `ShellHost/🎚️shell-model/🟦️.tsx` (the only ShellHost child touched, re-read just before the edit): removed `EMPTY_KEYS_BY_ACTION_ID`, `AppKeybindingsContext`, `useAppKeybindingsByActionId`, `useMapContextMenuSpecs` and its `World3dHost` barrel import; added one `export { ... } from` the leaf. The ShellHost parent is untouched (it re-exports the two hooks from this child and imports `AppKeybindingsContext` from it).
- `World3dHost/🔗️instance-selection/🟦️.ts` (-82 lines) and `World3dHost/🪟️window-view-store/🟦️.tsx` (-3 lines): moved blocks removed, `export { ... } from` the leaves added (the World3dHost parent and its siblings keep importing through these children), now-unused imports removed.
- `ShellHelpers/🏷️labels/🟦️.tsx`: the one import line.
- `NG/🟦️.tsx`: the three imports above now come from the two leaves.

### 2.3 Measured effect (dependency-cruiser, `no-circular` violations whose `from` is under each element; the same command before and after)

| Element under `E` | Baseline | After Phase 0 (`nodegraph-depcruise-p0.json`) | Final tree (`nodegraph-depcruise-final.json`) |
|---|---:|---:|---:|
| whole cruise, all `no-circular` | 519 | 505 | 463 |
| `🕸️NodeGraph` | 3 | 0 | 0 |
| `🛠️ShellHelpers` | 31 | 23 | 21 |
| `🌐️World3dHost` | 29 | 28 | 28 |
| `🏛️ShellHost` | 213 | 212 | 187 |
| `✏️TextEditor` | 5 | 4 | 4 |
| `🗣️Interpreter` | 8 | 8 | 8 |

Strongly connected components (own Tarjan over the cruiser JSON, `scc.mjs`, `scc-diff.mjs`): the component containing NodeGraph had 86 modules at baseline; after Phase 0 NodeGraph is in no component, and the large component has 81 modules. The modules that left it between baseline and Phase 0 are `NodeGraph`, `ShellHelpers/🏷️labels`, `TiledMapHost`, and the ShellHost children `📨️window-invocation`, `🔌️extension-runtime`, `🖱️shell-actions`; one module joined (`📐️Canvas2dHost/🧭️gumball`, a peer's new file). Between Phase 0 and the final run the whole-cruise total and the `ShellHost`/`PluginRuntime` rows fell further, but I made no edits to those trees in that interval: those changes are peers' (for example the `PluginRuntime` row 22 to 0). Only the NodeGraph row and the leaf-related rows can be attributed to this work, and the comparison baseline to Phase 0 also contains whatever peers changed in that 25-minute window.

Remaining cycles (not worsened, out of scope): `Interpreter <-> ShellHost` through the parent barrels, `World3dHost <-> ShellHost/ShellHelpers`, `ShellHost <-> Shell`. NodeGraph has no edge into any of them any more.

## 3. Phase 1: module-level split (8 children)

Cut with the hash-guarded tool of the siblings (`analyze.mjs`, `split.mjs`, `write.mjs`, config `cfg-nodegraph.mjs`; analysis of the post-Phase-0 text `p0-NodeGraph.tsx`). 157 top-level units: 156 appear exactly once across parent plus children (`coverage-nodegraph.mjs`); the 157th is `FlowGraphCanvasHost`, whose text was then replaced by the hook decomposition of section 4 (proved equivalent there). Module-level mutable state each has one owner: the six `Map`/`WeakMap`/`Set` registries (`activeFlowTasks`, `flowSessionDeliveredPayloads`, `flowSessionSendingPayloads`, `flowSharedPayloadDigests`, `flowSessionSharedCoalescers`, `flowSessionSharedIssues`) live only in `🔄️flow-session-sync`. Each child opens with a JSDoc whose first token is the folder emoji and links back to the parent; no comments were added inside definitions.

| Child (`NG/<dir>/🟦️.ts(x)`) | Lines | Holds |
|---|---:|---|
| `🧰️graph-core` (ts) | 366 | session and pick contract, port type rules, viewport and selection/hover action arguments and publishing, keyboard handling (`handleGraphKeyboard`), edit rows (`nodeGraphEditRows`, `flowAddWidgetArgs`), pinch-wheel plan, `flowJsonText`/`flowBoolean` |
| `🪄️catalogue-spotlight` | 304 | catalogue drag payloads, items, groups, sections, ranking, `FlowSpotlight` |
| `🔷️diagram-node` | 305 | workflow-to-diagram mapping, `WorkflowDiagramNode`, `DiagramGraphFallback` |
| `🧭️wasm-graph-surface` | 507 | `WasmGraphSurface` |
| `🖼️dag-overlays` | 860 | overlay types, geometry, content framing, parsing, label painting, marquee, slider lanes and align chrome |
| `🔄️flow-session-sync` (ts) | 374 | flow task tracking, shared payload coalescing and delivery, scene to session sync, camera startup and refit |
| `🫱️interaction-publication` (ts) | 76 | interaction ledger |
| `🧷️flow-canvas-contract` (ts) | 83 | render admission, camera gesture rule, unpresentable verdict, canvas key |

Deviations from the seam map: the map proposed `🎚️node-graph-host` and `🌊️flow-canvas-host` children. I first cut them, then removed both (files deleted, names unregistered): `readModuleTreeSource` (used by the readers) only follows relative imports below the declaring directory, so a parent that does not import every child directly hides them from the tree readers. Keeping `NodeGraphHost` and the thin `FlowGraphCanvasHost` in the parent, as `ShellHost` keeps `FrameworkOsShellInner`, makes the parent import all 15 children (8 modules plus 7 hooks); `WasmGraphSurface` is reached through the parent's own import. The camera-gesture/canvas-key helpers are in `🧷️flow-canvas-contract` because the hooks need them and a hook must not import the module that imports it. `🗂️types-and-parsing` was absorbed into `🧰️graph-core` as in the map.

DAG (`graph-check-dir.mjs`): 16 files, 0 strongly connected components, 0 children importing the parent. Longest new path 166 UTF-8 bytes (limit 240).

## 4. Phase 2: `FlowGraphCanvasHost` to hooks (7 children)

Reused the ShellHost hook tooling (`fg-lib.mjs`, `fg-plan.mjs`, `fg-emit.mjs`, `fg-verify.mjs`, ranges in `fg-config.mjs`; copies of `hooks-*.mjs` with the function name, hook root and naming parametrised). The component had 98 statements (97 before the `return`) and 89 hook calls.

`FlowGraphCanvasHost(props)` now does exactly this: `const core = {} as FlowCore;`, `Object.assign(core, useFlowSurfaceState(props));`, six more `Object.assign(core, useX(core))` in the original statement order, one destructure of the 50 names the JSX reads, then the original JSX `return` verbatim. Each hook module exports `FlowCoreAfter<X> = FlowCoreAfter<Previous> & ReturnType<typeof useX>` and takes the previous link as its parameter, so every binding keeps its inferred type (no `any`). Hooks are unconditional, in original order, bodies are the original text of their ranges, dependency arrays are unchanged. The first hook takes the props with the original destructuring (type alias `FlowGraphCanvasHostProps`, exported by it; no `core` parameter because it reads no slot). No `let`, no function declarations and no closure-local types in the closure, so values travel by the bag exactly.

| # | Folder (`NG/<dir>`) | Hook | Orig. statements | Lines | Reads | Fwd | Outputs |
|---|---|---|---|---:|---:|---:|---:|
| 1 | `🧱️flow-surface-state` | `useFlowSurfaceState` | 0-34 | 150 | props | 0 | 48 |
| 2 | `🎬️flow-gestures` | `useFlowDispatchAndGestures` | 35-57 | 150 | 7 | 1 read | 17 |
| 3 | `🎨️flow-overlay-render` | `useFlowOverlayAndRender` | 58-67 | 162 | 17 | 0 | 2 |
| 4 | `📏️flow-canvas-surface` | `useFlowCanvasSurface` | 68-71 | 83 | 12 | 0 | 1 |
| 5 | `🧾️flow-interaction-lane` | `useFlowInteractionPublication` | 72-76 | 66 | 8 | 0 | 5 |
| 6 | `🔌️flow-session-attach` | `useFlowSessionAttach` | 77-82 | 220 | 25 | 0 | 0 |
| 7 | `🎯️flow-pick-spotlight` | `useFlowPickAndSpotlight` | 83-96 | 291 | 19 | declares 1 | 8 |

Concern per hook: 1 refs, state, catalogue and scene refs, introduction resolver and geometry probe; 2 `dispatch`, menu dispatch builder, gesture reasons, `beginGesture`/`endGesture`, camera publication, wheel gesture and recognisers; 3 coalesced overlay paint and frame presentation; 4 unpresentable canvas replacement and backing-store size; 5 interaction ledger, pointer-up and selection/hover publication; 6 keyboard port, session creation, one-time attach, scene application and sync; 7 appearance sync, pick interaction, catalogue spotlight, drag and drop, double click.

Forward reference (the one hard problem, as in ShellHost): `buildFlowMenuDispatch` (hook 2) calls `openSpotlightAtClient`, a `const` declared later in hook 7. The earlier hook reads `core.openSpotlightAtClient` at that exact identifier and the declaring hook assigns `core.openSpotlightAtClient = openSpotlightAtClient;` on the line right after the declaration; the slot is typed by `FlowForwardSlots` (declared in hook 1). Each render builds a fresh `core`, so a callback created in render N reads render N's binding, as the closure did. There are 0 eager forward references.

Equivalence (`fg-verify.mjs`): the original closure text of statements 0 to 96 (comments included) equals the concatenation of the 7 hook bodies after undoing the scaffolding (destructure, bag, the `core.x` rewrite, the slot assignment), identical after whitespace normalisation (40,548 characters on both sides); the hook call order equals the hook order (7); the original JSX `return` slice is present verbatim in the new parent.

## 5. Region markers (coordinator follow-up)

The splitters had dropped every top-level `//#region`/`//#endregion` line. The original has 27 distinct region names (54 marker lines, none indented, including the `🧲️Header` pair); after the first two phases the tree held 1 of them (`🧲️Header`), 26 were missing. They are restored by `restore-regions-ng.mjs`, which takes the original marker lines verbatim (including the spacing quirks, for example `// #region 🔌️Adapters` with a space and the others without), and for every original region pair:

- finds every top-level unit lying inside the pair, locates it in the file that now holds it (by declared name, with its leading comment block), and wraps the first to last such unit in that file with the exact begin and end lines; a region whose members now live in several children is repeated, balanced, in each of them (for example `Parsing`: `🧰️graph-core`, `🪄️catalogue-spotlight`, `🔷️diagram-node`; `🔖️NodeGraphHost` in all 16 files);
- for the region that contained the former `FlowGraphCanvasHost` body (`FlowGraphCanvasHost`, `🔖️flow-graph-canvas-host`, `🔖️NodeGraphHost`) it also wraps the code of each of the 7 hook modules (everything after the import block);
- wraps the parent's import block in `🔌️Adapters`, which held only imports.

Order at a shared offset: ends before begins, inner end first, outer begin first. Because every region is contiguous in the original and children keep the original unit order, the span of a region inside one file never contains a foreign unit, so nesting stays proper.

Checks (`regnames.mjs`, `regcheck.mjs`, inline balance script): distinct region names original 27, tree 27, missing 0 (superset, and equal); every one of the 16 files is balanced (0 unclosed, 0 mismatched); with the added marker lines removed, each file is byte-identical to the file before the restore (all 16). Marker lines: 54 in the original, 134 in the tree (the extra 80 are the repeated begin/end pairs of regions that span children; none can be collapsed into one file). The two Phase 0 leaves and the files outside `NG` have no original regions, so they carry none.

## 6. Taxonomy registration

Validator rule (as established by the siblings): a child folder name must be a member of `semanticDirectoryMemberKinds."members-of-members-of-elements".memberNames` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. I appended 17 names (one textual hunk each time, anchors asserted unique, JSON re-parsed, nothing else reformatted; `taxonomy-add.mjs`; two names I had registered for the abandoned `🎚️node-graph-host` and `🌊️flow-canvas-host` children were removed again with `taxonomy-remove.mjs`): `🌉️scene-host-contract 🖱️surface-menu 🧰️graph-core 🪄️catalogue-spotlight 🔷️diagram-node 🧭️wasm-graph-surface 🖼️dag-overlays 🔄️flow-session-sync 🫱️interaction-publication 🧷️flow-canvas-contract 🧱️flow-surface-state 🎬️flow-gestures 🎨️flow-overlay-render 📏️flow-canvas-surface 🧾️flow-interaction-lane 🔌️flow-session-attach 🎯️flow-pick-spotlight`. Every name starts with an RGI emoji followed by U+FE0F, the emoji is already a folder prefix elsewhere in the repo, and no emoji repeats among the siblings of one parent (`NG`'s existing children `⚠️ 📖️ 📡️ 📦️ 🧩️ 🧪️` and the file emoji `🟦️`/`🦀️` are not reused; `Interpreter`'s existing children are not reused).

Validator (`bun ./📜️script.ts verify taxonomy report --scope <path>`, logs `🗑️generated/nodegraph-taxonomy-after.log` and `nodegraph-taxonomy-interpreter-after.log`, run on the final tree):
- scope `NG`: `clean=false errors=9`. None names a new folder or file. They are: 5 `directory-kind-unresolved` on pre-existing unregistered directories (`🧪️tests/🧩️suite`, `🧪️tests/🤏️pinch-wheel`, `🧪️tests/🖱️scroll-gesture`, `📡️scene`, `⚠️refusal`), 1 `normalization-move-required` (`🧪️tests/🧫️fixtures/🧫️pick-target/🔣️.json`), 1 `fixed-source-disposition-unresolved` (`📦️packages/🦀️rust/📜️script.ts`), 1 `generator-preview-invalid` (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`) and 1 `reference-edit-required` at a position inside `🧪️tests/🔬️engine-contract/🟦️.ts` (a structured reference the validator wants moved; the file is also edited by peers, and my one-line change there is a plain source read, not a structured reference).
- scope `E/🗣️Interpreter`: `clean=false errors=24`; 0 of them name `🌉️scene-host-contract` or `🖱️surface-menu`.
I ran no scope baseline before registering, so there is no "before" count; the evidence is that no finding names one of the 17 new folders or their files (same argument as the ShellHost and ShellHelpers reports).

## 7. Verification, baseline vs after

Baselines were recorded before the first edit. All commands ran in the foreground of a background script with an `alarm` cap; logs are in `🗑️generated/nodegraph-*`.

### 7.1 Typecheck

`node_modules/.bin/tsc --noEmit -p 🗑️generated/nodegraph-tsconfig.json` (scoped config extending `💻️os/tsconfig.json`; `files` = the NodeGraph, Interpreter, ShellHost, World3dHost, ShellHelpers, TextEditor parents, the React target and the 8 test files that touch NodeGraph; about 2 minutes).
- Baseline (`nodegraph-tsc-baseline.log`): 34 errors, 0 in NodeGraph (27 `UiDocumentStore/🧪️tests/🧪️typedwire`, 5 `PluginRuntime` children, 2 `WasmSessionLoader`).
- After Phase 0, after Phase 1, after Phase 2 and on the final tree (`nodegraph-tsc-final2.log`, run after the region restore and the test re-pointing): 34 errors, sorted diff against the baseline empty.
- Proof the check is live: the first Phase 0 run reported `TS2304 Cannot find name 'createContext'` in `window-view-store` (my own too-greedy import edit) and five cascading `TS2339`/`TS7006` errors in the `World3dHost` parent; the fix brought the count back to 34. A `--noUnusedLocals` run on the final tree reports in NodeGraph only declarations that were already unused in the original (`GraphContextMenuItem`, `flattenFlowCatalogueItems`, `named`); no generated import or destructure name is unused.

### 7.2 Dependency-cruiser

See 2.3. Commands: `node_modules/.bin/depcruise E --config <repo config> --output-type json`, whole `E`, three runs. Final tree: no component contains NodeGraph.

### 7.3 Unit tests

Config of the React target (`SEMIO_TEST_LEVEL=long`, policy built by hand in `nodegraph-vitest.sh`, as the siblings). Suites that read NodeGraph but are not in the long include list (`🫳️node-graph-gestures`, `🔗️node-graph-wire-edit`, `🎥️tutorial-bridge`) run through `nodegraph-vitest-extra.config.ts`, which merges them into the real config; `♻️hot-swap-session-handover` imports `bun:test` and runs with `bun test`.

Full renderer suite (184 files, 3,021 tests):

| Run | Files failed / passed | Tests failed / passed | Log |
|---|---|---|---|
| Baseline (before any edit) | 31 / 153 | 48 / 2,973 | `nodegraph-vitest-full-baseline.log` |
| After split, before region restore | 31 / 153 | 40 / 2,981 | `nodegraph-vitest-full-after.log` |
| Final tree | 29 / 155 | 38 / 2,983 | `nodegraph-vitest-full-after2.log` |

Failure-set diff, baseline vs final (`nodegraph-fails-baseline.txt`, `nodegraph-fails-after2.txt`): 0 test names failing only in the final run. The 10 names failing only in the baseline are 8 `Paint2dHost/✍️editing/...selection-focus` tests, `embedded-mount > renders two idle real ShellHosts...` and `engine-contract > scheduleDispatchAction (D2)`: tests that flap between runs on this loaded machine (the same `selection-focus` set and the `embedded-mount` test flapped in the ShellHost and ShellHelpers reports; in the intermediate run one `selection-focus` test failed that had passed at baseline). None involves NodeGraph.

Suites that touch NodeGraph, baseline vs final (`nodegraph-vitest-targeted-baseline.log`, `nodegraph-vitest-targeted-after2.log`): 9 files / 71 tests, baseline 3 files failed, 3 tests failed, 68 passed; final 3 files failed, 3 tests failed, 68 passed, the same three tests in both:

| File | Tests | Result before / after |
|---|---:|---|
| `🧪️tests/🫳️node-graph-gestures` | 12 | 11 pass, 1 fail (`lets the HOST journal every content change...`, reads a Rust file, pre-existing) / identical |
| `🧪️tests/🛑️scene-pointer-cancellation` | 5 | 4 pass, 1 fail (`preserves the physical pointer...`, Rust-side needle, pre-existing) / identical |
| `NG/🧪️tests/🤏️pinch-wheel` | 4 | 3 pass, 1 fail (`the notch factors are the schema camera tokens`, Rust-side needle, pre-existing) / identical |
| `🧪️tests/🔗️node-graph-wire-edit` | 10 | pass / pass |
| `🧪️tests/🧪️node-graph-edit-rows` | 4 | pass / pass |
| `🧪️tests/🕸️node-graph-domain-interaction` | 5 | pass / pass |
| `🧪️tests/🎥️tutorial-bridge` | 5 | pass / pass |
| `NG/🧪️tests/🖱️scroll-gesture` | 8 | pass / pass |
| `NG/🧪️tests/🫱️interaction-publication` | 18 | pass / pass |
| `🧪️tests/♻️hot-swap-session-handover` (`bun test`) | 8 | 7 pass, 1 fail (`the node-graph surface is keyed on its surfaceId alone`: the sought `[TRACE]` log line does not exist in the original either) / identical |
| `🧪️tests/🔬️engine-contract` (715 tests, both NodeGraph source readers inside) | | 5 fail at baseline / 4 fail in the final run (the 4 are among the 5; the fifth, `scheduleDispatchAction (D2)`, flaps); none involves NodeGraph; the two NodeGraph tests (`Escape on a graph surface...`, `a graph surface publishes the selection it paints...`) pass in both |

TextEditor's `📝️explicit-draft` (imports `useClient` through the NodeGraph barrel) passes before and after (28 and 21 tests). Real-mount evidence for the hook decomposition: `engine-contract` mounts `NodeGraphHost` with flow scenes via `createElement(NodeGraphHost, ...)` (lines 4623, 4694, 4739, 9008) and asserts on `.semio-node-graph-host` and its `z-30` overlay; those tests are in the 710 passing ones. They run all 7 hooks of `FlowGraphCanvasHost` in jsdom with the stub wasm engine; there was no browser or GPU boot (see 8).

An intermediate, failing run is worth recording because it proves the readers are live: after the first hook cut the NodeGraph parent did not import the wasm surface and the hook modules, `readModuleTreeSource(NG parent)` therefore missed `emitInteractionState` (found 0 times instead of at least 2), and `node-graph-gestures > publishes the flow-graph viewport only after a camera gesture settles` failed with `expected 0 to be greater than or equal to 2`. Moving the host component into the parent fixed that structurally (section 3).

## 8. Source-text readers of `NG/🟦️.tsx`

Every reader found with `git grep` (outside tickets) over the repo. Re-pointed to the child that holds the needle where the test slices around a needle; `readModuleTreeSource(parent)` (the repo's own tree reader, which now reaches all 15 children) for cross-module and absence checks. Needles verified by `grep -F` on the new tree before editing and by the runs above.

| Reader | Needles | Now |
|---|---|---|
| `🛑️scene-pointer-cancellation` (`?raw`) | slice from `className="absolute inset-0 z-30 touch-none"` to `onWheel=` | `?raw` of `NG/🧭️wasm-graph-surface/🟦️.tsx` (the first occurrence in the original was `WasmGraphSurface`); fixture `🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json` field `source` for `nodeGraph` updated to that child |
| `🔗️node-graph-wire-edit` | `operation: "connect"` block; `issueFlowGestureStep(session.pointerUpScreen(` ... `handleGesturePointerUp();` and `setPointerCapture`/`releasePointerCapture` windows; `not.toContain("setHostSnapshot")` | `🔷️diagram-node` (connect fields); the parent (the JSX that holds the press and release; windows and slices unchanged); module tree for the absence check |
| `🧪️node-graph-edit-rows` | absence of `setHostSnapshot`, `commitFixture`, `session.addWidget(` | module tree |
| `🫳️node-graph-gestures` | `onNodeDragStop={` ... `onConnect=` with `nodeDragRow(`; absence of `commitFixture`/`hostSnapshotChanged`, presence of `dispatchGraphEdits();`; every `const emitInteractionState = useCallback` (at least 2) and `publishCameraRef.current()` | `🔷️diagram-node`; module tree; module tree (the two ledgers live in `🧭️wasm-graph-surface` and `🧾️flow-interaction-lane`) |
| `🔬️engine-contract` | `function handleGraphKeyboard` body and the `setMediaNodeSelection` absence; dynamic import of `nodeGraphSurfaceSelectionDomV1` | module tree for the text (needle in `🧰️graph-core`); the dynamic import still resolves through the parent barrel |
| `♻️hot-swap-session-handover` (`bun test`) | `console.log("[TRACE] node-graph host unmount ...` | module tree (still failing exactly as at baseline, needle absent everywhere) |

Not readers of the text, left alone: `🎥️tutorial-bridge`, `🕸️node-graph-domain-interaction`, `🧪️node-graph-edit-rows` imports, `TextEditor` and the React target (all import through the barrel). Fixtures that name the NodeGraph file in descriptive fields and that I updated to the files that now hold the code: `🖼️canvas/🧪️tests/🧫️fixtures/🏷️label-fit` and `📷️camera-fit` (the `dag-overlays` child) and `♾️infinite/🎲️board/🚪️io/📝️text/🎯️dag-input/🧪️tests/🧫️fixtures/receivers.json` (`typedSources` now lists the six children that use the typed decoders; `typedRegions` lists `FlowGraphCanvasHost`, its 7 hooks and `syncFlowSessionStructureFromScene` with their original forbidden calls). I found no code that reads `receivers.json` (only ticket scripts), so that edit is data-only and unverified by a run. `receivers.json`'s earlier `World3dHost` entry is left (it names the parent, which still exports the moved functions).

## 9. Residuals and notes

- `FlowCore` is created with `{} as FlowCore` and filled hook by hook; that is the one type assertion (same trade-off as ShellHost), and it is what makes the forward slot and the single per-render core possible.
- Pre-existing cycles through the shell parents remain (2.3). `World3dHost` instance-layers etc. still import `parseJsonArray` through the `instance-selection` re-export; I did not redirect other elements (Table, Board2dHost, ...) to the leaves, so they still import `useMapContextMenuSpecs`/`openSurfaceContextMenu` from the `ShellHost`/`Interpreter` barrels; that is the natural follow-up.
- The leaves re-export through `Interpreter`, `ShellHost/shell-model` and the two `World3dHost` children rather than updating each importer, to keep every public path and the React target's barrel lists intact; this is not a compatibility alias layer, the names have exactly one definition.
- Not run: `bun repo run ...`, a Vite dev server or browser boot of a graph window (module-evaluation order of the new files is evidenced only by vitest/jsdom), the GPU/wasm engines (the stub from the config replaces `@semio-tech/framework-os-node-graph-rs`), any ESLint rule set (`react-hooks` lint not run; the hooks are plain `use*` functions called unconditionally), the repo-wide `verify taxonomy`, Rust gates, regeneration of the distribution manifest in `🧑‍💻dev/📤️distribution/` (generated artifact).
- The git index showed staged-then-deleted entries for the two abandoned child files (an automatic stager ran in between); the working tree is what the results above describe. No modifying git command was run by me.
- Debug logging: none added. `🗑️generated` folders were left in place; the tool and log inventory is in section 10.

## 10. Files touched

Created (under `NG`): 8 module-level children and 7 hook children, listed in sections 3 and 4. Created (under `E/🗣️Interpreter`): `🌉️scene-host-contract/🟦️.ts`, `🖱️surface-menu/🟦️.ts`.
Rewritten: `NG/🟦️.tsx` (4,005 to 627 lines).
Edited: `Interpreter/🟦️.tsx`, `ShellHost/🎚️shell-model/🟦️.tsx`, `World3dHost/🔗️instance-selection/🟦️.ts`, `World3dHost/🪟️window-view-store/🟦️.tsx`, `ShellHelpers/🏷️labels/🟦️.tsx`; `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (+17 names); the readers `🧪️tests/{🔗️node-graph-wire-edit,🧪️node-graph-edit-rows,🫳️node-graph-gestures,🛑️scene-pointer-cancellation,🔬️engine-contract,♻️hot-swap-session-handover}/🟦️.ts` (engine-contract and hot-swap: one line each); fixtures `🧪️tests/🧫️fixtures/🛑️scene-pointer-cancellation/🔣️.json`, the two `🖼️canvas` fixtures and `receivers.json`.
Temporary tooling and evidence (in `🪓️monolith-split/🗑️generated/nodegraph-split/`): `phase0.mjs`, `cfg-nodegraph*.mjs`, `fg-*.mjs`, `restore-regions-ng.mjs`, `regnames.mjs`, `regcheck.mjs`, `scc*.mjs`, `cyc-count.mjs`, `exports*.mjs`, `coverage-nodegraph.mjs`, `graph-check-dir.mjs`, `taxonomy-add.mjs`, `taxonomy-remove.mjs`, snapshots `orig-NodeGraph.tsx`, `p0-NodeGraph.tsx`, `p1-FlowCanvasHost.tsx`, `p1b-NodeGraph.tsx`, and the logs `nodegraph-*` one level up.
