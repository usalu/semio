# Result: re-pointing every importer to the two phase 0 leaves, and removing the compatibility re-exports

Executed 2026-10-15 by the Sonnet executor. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`. The leaves are `E/🗣️Interpreter/🌉️scene-host-contract/🟦️.ts` (`CONTRACT`) and `E/🗣️Interpreter/🖱️surface-menu/🟦️.ts` (`MENU`). Both are unchanged by this task. Tooling and logs live in `🪓️monolith-split/🗑️generated/leafrewire/` and `🪓️monolith-split/🗑️generated/leafrewire-*` (temporary helpers, not repo tooling).

## 1. Outcome at a glance

- 27 files re-pointed; 93 name imports moved to a leaf; 24 re-export specifiers removed from 6 old modules; 4 specifiers narrowed from a barrel to its child.
- Zero re-exports of a moved name remain anywhere. Re-running the tool in plan mode reports 0 pending edits.
- `tsc` (scoped, 95 root files): 105 errors before, 105 after, identical set (one error moved from line 558 to 559 in `World3dHost/🧪️tests/🧩️component/🟦️.tsx` because the re-pointed import added a line).
- Full renderer vitest: 40 failed / 2981 passed before; 40 failed / 2981 passed after. The failure sets differ only by flakes (section 5).
- dependency-cruiser `no-circular` over `E`: 469 to 445 violations; seven element hosts (Table, GraphTimelineHost, EventFeedHost, DiffViewHost, BlockListHost, TiledMapHost, TextEditor) have no `no-circular` violation left and `Paint2dHost` fell from 5 to 1, `Interpreter` cycle edges fell from 22 to 13 and `World3dHost` from 29 to 18 (section 4).

## 2. Moved names and where each one lives now

| Leaf | Names (definition lives here, importers read it here) |
|---|---|
| `🌉️scene-host-contract` | `wireLabel`, `AppCatalogueContext`, `useAppCatalogue`, `WindowInstanceIdContext`, `parseSceneJsonField`, `parseJsonArray`, `parseSelectionDomainsFromSession`, `selectionGroupsFromDomains` |
| `🖱️surface-menu` | `ShellContextMenuFallbackContext`, `useShellContextMenuFallback`, `SurfaceContextMenuResult`, `surfaceContextMenuTitleKey`, `openSurfaceContextMenu`, `AppKeybindingsContext`, `useAppKeybindingsByActionId`, `useMapContextMenuSpecs`, `mapContextMenuSpecs` |

### 2.1 Removed re-exports (exported API of each touched barrel, before and after, by the TS checker with the pre-edit text overlaid)

`exports.mjs` builds one program from the current tree and one with the 27 pre-edit file texts overlaid, then diffs `getExportsOfModule` of each barrel.

| Module | Exports before | after | Removed (these only; nothing added) |
|---|---|---|---|
| `E/🗣️Interpreter/🟦️.tsx` | 63 | 54 | `wireLabel`, `parseSceneJsonField`, `useAppCatalogue`, `AppCatalogueContext`, `openSurfaceContextMenu`, `surfaceContextMenuTitleKey`, `SurfaceContextMenuResult`, `useShellContextMenuFallback`, `ShellContextMenuFallbackContext` |
| `E/🏛️ShellHost/🟦️.tsx` | 63 | 61 | `useAppKeybindingsByActionId`, `useMapContextMenuSpecs` |
| `E/🏛️ShellHost/🎚️shell-model/🟦️.tsx` | 16 | 13 | `AppKeybindingsContext`, `useAppKeybindingsByActionId`, `useMapContextMenuSpecs` |
| `E/🌐️World3dHost/🟦️.tsx` | 181 | 176 | `WindowInstanceIdContext`, `mapContextMenuSpecs`, `parseJsonArray`, `parseSelectionDomainsFromSession`, `selectionGroupsFromDomains` |
| `E/🌐️World3dHost/🔗️instance-selection/🟦️.ts` | 59 | 55 | `mapContextMenuSpecs`, `parseJsonArray`, `parseSelectionDomainsFromSession`, `selectionGroupsFromDomains` |
| `E/🌐️World3dHost/🪟️window-view-store/🟦️.tsx` | 12 | 11 | `WindowInstanceIdContext` |

Every other barrel I edited (React target, the hosts) has an unchanged export surface. In `E/../🎯️targets/⚛️react/🟦️.tsx` I changed only the import statements (they now read the leaves); its explicit `export { ... }` lists are untouched, so the target's public API, and therefore the `📦️packages/🟦️typescript` facade (`export *`) and `🧪️tests/🔬️engine-contract` which imports `mapContextMenuSpecs` and `surfaceContextMenuTitleKey` through that facade, are unchanged. I left that one test import on purpose: it exercises the public target API, not an old module.

### 2.2 Importers re-pointed (27 files; every one an import-line edit)

Elements: `Board2dHost`, `BlockListHost`, `Canvas2dHost`, `DiffViewHost`, `EventFeedHost`, `GraphTimelineHost`, `InkCanvasHost`, `Paint2dHost`, `Table`, `TextEditor`, `TiledMapHost`, `Interpreter` (its own two `export {} from` blocks removed, `WindowInstanceIdContext` now from the contract leaf instead of `World3dHost`, which also removes that `Interpreter -> World3dHost` name edge), `ShellHost` parent (`AppCatalogueContext`, `ShellContextMenuFallbackContext`, `AppKeybindingsContext` from the leaves; the two hook re-exports removed), `ShellHost/🎚️shell-model` (the `export {} from` line removed), `ShellHost/📋️panel-hosting`, `ShellHost/🖼️canvas-footer`, `ShellHelpers/🛣️route-and-chrome`, `World3dHost` parent, `World3dHost/🔗️instance-selection`, `World3dHost/🪟️window-view-store`, `World3dHost/🎨️mesh-render`, `World3dHost/🧱️instance-layers`.
Tests: `E/../🧪️tests/🔬️engine-contract` (3 names, from `Interpreter`), `E/../🧪️tests/🖱️world3d-interaction`, `World3dHost/🧪️tests/🤏️multi-touch`, `World3dHost/🧪️tests/🧩️component`.
React target: `E/../🎯️targets/⚛️react/🟦️.tsx` (its `Interpreter`, `ShellHost` and `World3dHost` barrel imports replaced by two leaf imports: 8 names now come from `🖱️surface-menu`, 6 from `🌉️scene-host-contract`).

Not touched, and why: `NodeGraph` and its children already read the leaves (phase 0). A repo-wide `git grep --untracked` for all 17 names over `*.ts *.tsx *.mts *.js *.mjs *.cjs *.jsx *.svelte *.vue *.md` finds no other importer; the only non-root hits are five `.cursor/plans/*.md` prose files. `ShellHelpers/🏷️labels` already imports the contract leaf. Tests that read source text (`🖱️ui/🧪️tests/🎚️ring-press`, `🔍️discovery/🧪️tests/🧪️interactivity-tool-run-policy`) and the `gismap` script do not depend on any of the removed re-export lines. No `vi.mock`/`mock.module` targets any of the three barrels, so redirecting imports does not bypass a mock. No dynamic `import()` destructures a moved name.

## 3. Barrel-to-child narrowing (task item 2)

Rule: for each import of `🏛️ShellHost/🟦️.tsx` or `🗣️Interpreter/🟦️.tsx` in a non-test element file, the TS checker resolves each imported name to its declaration; if the declaration sits in a file below that barrel's folder, the import goes to the first-level child that exports it.

Applied (4 specifiers):

| File | Name | Was | Now |
|---|---|---|---|
| `World3dHost/🎨️mesh-render` | `runtimeDiagnosticsEnabled` | `ShellHost` barrel | `ShellHost/🔌️extension-runtime` |
| `World3dHost/🧱️instance-layers` | `runtimeDiagnosticsEnabled` | `ShellHost` barrel | `ShellHost/🔌️extension-runtime` |
| `World3dHost` parent | `SetWindowTitleContext`, `SetWindowIconContext` | `ShellHost` barrel | `ShellHost/🎚️shell-model` |

Why so few: after the contract rewiring, every remaining element import of these two barrels names something declared in the barrel file itself (not in a child), so there is nothing narrower to import. Remaining ones (`remaining.mjs`): `Interpreter` barrel: `InterpretedUiNode`, `TreeWindowContext`, `TreeWindowContextValue`, `TreeWindowReportV1`, `UiPresenceOverlay*`, `PluginSurfaceActionsContext`, `RowActionRefusalsContext` (all declared in `🗣️Interpreter/🟦️.tsx` itself; the Interpreter split is still pending); `ShellHost` barrel: `FrameworkOsShell` (declared in the parent, imported by `Shell`).

Honest note on `runtimeDiagnosticsEnabled`: `ShellHost/🔌️extension-runtime` itself imports `🐚️Shell/🟦️.tsx`, so this narrowing does not leave a cycle, it only changes which path closes it. The right fix is moving that predicate into a shell-neutral leaf; I did not do it (out of scope, behaviour-neutral either way).

Candidates not applied (outside the named barrels): 20 specifiers in 9 files import the `World3dHost` barrel for names declared in its children `🔗️instance-selection` and `🪟️window-view-store` (ShellHost children `🧩️shell-core-state`, `🖱️shell-actions`, `🎮️focus-and-input`, `🏗️dock-assembly`, `📋️panel-hosting`, `📐️layout-hotkeys`, `🖼️canvas-footer`; `Interpreter`; `ShellHelpers/🛣️route-and-chrome`). `WITH_WORLD3D=1 node rewire.mjs plan` reports them. These do not touch the contract leaves, so I left them for a follow-up decision (instance-selection reaches `ShellHelpers`, so some would stay inside cycles).

## 4. dependency-cruiser

Command (before and after, identical): `node node_modules/dependency-cruiser/bin/dependency-cruise.mjs E --config 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs --output-type json`, files `🗑️generated/leafrewire-depcruise-{baseline,after}.json`, comparison `leafrewire-cyc-compare.log`. 1,451 modules cruised in both.

`no-circular` violations, `from` under each element (only rows that changed):

| Element | Before | After |
|---|---|---|
| whole run | 469 | 445 |
| `🖌️Paint2dHost` | 5 | 1 |
| `🖥️Board2dHost` | 5 | 2 |
| `✏️TextEditor` | 4 | 0 |
| `🖋️InkCanvasHost` | 4 | 1 |
| `🌳️GraphTimelineHost` | 3 | 0 |
| `📊️Table` | 3 | 0 |
| `📐️Canvas2dHost` | 3 | 1 |
| `📡️EventFeedHost` | 3 | 0 |
| `🔺️DiffViewHost` | 3 | 0 |
| `🧩️BlockListHost` | 2 | 0 |
| `🧭️TiledMapHost` | 2 | 0 |
| `🗣️Interpreter` | 7 | 11 |
| `🏛️ShellHost` | 186 | 195 |
| `🌐️World3dHost` | 28 | 27 |
| `🛠️ShellHelpers` | 21 | 20 |

The `Interpreter` and `ShellHost` rows rose, so the raw count alone is misleading. The cruiser reports one violation per distinct cycle path, not per module. I therefore also computed strongly connected components (Tarjan over the cruiser JSON, `scc.mjs`, `sccdiff.mjs`, log `leafrewire-cyc-compare.log` and inline above):

- Modules inside an SCC: the largest SCC dropped from 69 to 58 modules. `sccdiff.mjs` lists 12 modules that left a cycle: `GraphTimelineHost`, `Table`, `EventFeedHost`, `DiffViewHost`, `BlockListHost`, `TiledMapHost`, `Paint2dHost/✍️editing`, `World3dHost/🧮️gumball-math`, `World3dHost/🧩️glb-surface` (also gone from the graph through a peer change), `ShellHelpers/🔄️ui-refresh`, `ShellHost/🎚️shell-model`, an `Interpreter` test. `TextEditor`, `Canvas2dHost`, `Board2dHost` and `InkCanvasHost` keep their SCC membership through other imports but lost cycle edges.
- Edges inside SCCs per element (before to after): `Interpreter` 22 to 13, `World3dHost` 29 to 18, `ShellHost` 187 to 184, `Canvas2dHost` 5 to 2, `Board2dHost` 5 to 2, `Paint2dHost` 6 to 1, `InkCanvasHost` 4 to 1, `Table`/`GraphTimelineHost`/`EventFeedHost`/`DiffViewHost`/`BlockListHost`/`TiledMapHost` 2 or 3 to 0.
- Why `ShellHost` and `Interpreter` path counts rose: the `ShellHost` row includes modules that peers created or changed during this window (`ShellHost/🎬️tutorial-recorder`, `ShellHost/🏁️operation-settle` are new modules, `ShellHelpers/🎛️command-registry` joined an SCC; none of the three is a file I edited). The 22 modules in `(outside)` that joined cycles are repo tooling under `🦑️repo`/`🔌️plugin` and not related. The `runtimeDiagnosticsEnabled` narrowing multiplies cycle paths that already existed through `World3dHost` (section 3).
- Caveat: baseline and after were measured about 20 minutes apart on a tree several agents edit concurrently, so I could not isolate my edits from peer edits perfectly. I attribute the 12 modules that left cycles to this change because each of them is a file or a direct importer of a file I re-pointed; I did not run a baseline-with-peer-changes-reverted experiment (not possible without git writes).

## 5. Verification, baseline vs after

### 5.1 Typecheck

`node_modules/.bin/tsc --noEmit -p 🗑️generated/leafrewire-tsconfig.json` (extends `💻️os/tsconfig.json`; `files` = the 93 root files that mention any moved name or import one of the three barrels, plus the two leaves; dependencies pulled in transitively). Logs `leafrewire-tsc-baseline.log` and `leafrewire-tsc-after.log`.
- Baseline 105 errors; after 105 errors. `diff` of the sorted `error TS` lines: only the line-number shift described above. No new TS2305/TS2307/TS2724/TS2614 (the 6 hits of those codes are in both logs).
- Liveness: the check resolves and reports real errors in this tree (105), and the moved names resolve through `getAliasedSymbol` in the same program. I did not inject a deliberate fault in this run; the fault-injection proof of this exact tsc setup is in `RESULT-NODEGRAPH-TSX.md` section 7.1.

### 5.2 Vitest

`🗑️generated/leafrewire/vitest.sh` (same recipe as `nodegraph-vitest.sh`, default renderer config `⚛️react/🧪️tests/🎚️config/🟦️.ts`, `SEMIO_TEST_LEVEL=long`). Logs `leafrewire-vitest-baseline.log` (31 files failed, 40 tests failed, 2981 passed, 160 s) and `leafrewire-vitest-after.log` (30 files failed, 40 tests failed, 2981 passed, 263 s). Failure-line sets: `leafrewire-fails-{baseline,after}.txt`.

Differences between the two failure sets (each is a flake, none touches an import I changed):
- Failed only before: `🧪️tests/📥️wgpu-intake-budget` (timing) and `🧪️tests/🪆️embedded-mount` "renders two idle real ShellHosts" (the known real-mount flake).
- Failed only after: two tests in `Paint2dHost/✍️editing/🧪️tests/🖱️selection-focus` ("selection survives focus moving to Apply", "selection combination cancels with Deselect after shape preparation"). The suite run was slower (263 s versus 160 s) and Paint2dHost is a file I re-pointed, so I re-ran the file in isolation: `vitest run selection-focus`: 1 file, 53 of 53 tests passed (`leafrewire-vitest-selection-focus.log`). So these are load-timing flakes, not regressions.
- All other failures (49 lines in both) are the same pre-existing set (WGPU/Chromium timing, `package-integration`, `browser-frame-transport`, `settings-general-layout`, engine-contract declaration tests, the missing `♿️editable-controls` fixture, and so on).

### 5.3 Not run

Taxonomy validator and eslint were not run; the edits are import-line edits in existing files and no file or folder was created or renamed. I did not run `bun test` for `♻️hot-swap-session-handover` or the Rust crates (no Rust file touched by me; `git diff` shows peer edits to `🦀️.rs` files under `🎞️Scenes`, which are not mine).

## 6. Method and safety

`rewire.mjs` (TypeScript API, `plan` and `apply` modes): AST-located import and export statements, range edits only (specifier removal with its comma, new `import { ... } from "<leaf>"` appended after the anchor statement, or merged into an existing leaf import by inserting specifiers after the last element). No statement was regenerated wholesale, so formatting, `//#region` marker lines that a peer inserts, and neighbouring lines are untouched. Apply re-reads each file immediately before writing and refuses when its text differs from the analysed text; 5 `World3dHost` files hit that guard in the first run (a peer was inserting region markers) and were applied cleanly in an immediate second run. Statements containing comments would be skipped (none were). A dry run (`DRY=<dir>`) diffed all 27 files before the real apply. Debug logs: none added. Nothing was deleted; the `🗑️generated` folder is left in place.

## 7. Leftovers

- `runtimeDiagnosticsEnabled` belongs in a shell-neutral leaf (section 3).
- 20 `World3dHost` barrel specifiers can be narrowed to its children (section 3).
- The `Interpreter` and `ShellHost` parents still declare the `Tree*`, `UiPresenceOverlay*`, `PluginSurfaceActionsContext`, `FrameworkOsShell` names that the other elements import, so those barrels remain the cycle hubs until they are split.

## 8. Files

Created: this file; scratch under `🪓️monolith-split/🗑️generated/leafrewire/` and `🪓️monolith-split/🗑️generated/leafrewire-*` (tooling, logs, JSON).
Updated: the 27 source files listed in 2.2 (all under `E` and `E/../🧪️tests`, `E/../🎯️targets/⚛️react/🟦️.tsx`).
Removed: nothing.
