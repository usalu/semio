# Seam Map: ShellHost, World3dHost, ShellHelpers (TSX monoliths)

Read-only audit. No source file was edited; no builds, nx, tsc, or git write commands were run.
Directory below is `E = 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/`.

| Monolith | Lines | Export lines | Import statements (approx.) | Consumer files | Distinct names consumed |
|---|---|---|---|---|---|
| `E/🏛️ShellHost/🟦️.tsx` | 13,692 | 62 | ~107 | 32 | 57 |
| `E/🌐️World3dHost/🟦️.tsx` | 8,551 | 181 | ~26 | 27 | 71 |
| `E/🛠️ShellHelpers/🟦️.tsx` | 6,708 | 326 | ~37 | 31 | 115 |

Line numbers were read on 2026-10-15 (file mtimes: ShellHost 13 Oct 17:49, World3dHost 13 Oct 21:07, ShellHelpers 6 Oct 13:44). Other agents edit these files concurrently. Re-scan line numbers immediately before each extraction. Scripts and raw tables live in `🗑️generated/seam-audit/` next to this file (`consumers.py`, `coupling.py`, `segments.py`, `show.py`, `*.json`, `appendix-tables.md`).

## 0. Summary

- **ShellHost** has 2,277 module-level lines (lines 1-2276) and one 11,415-line closure, `FrameworkOsShellInner` (2277-13691). Moving module-level code alone leaves the parent at about 11,400 lines, because the closure dominates. The closure must also become custom hooks that share one typed core object. With that, the parent lands near 900-1,000 lines.
- **World3dHost**: module-level code 177-6254 (6,078 lines) moves cleanly into 9 child modules. The `World3dHost` component (6256-8550, 2,295 lines) stays in the parent, which lands near 2,400 lines. No hook refactor is needed to meet the target.
- **ShellHelpers** is a flat module with no closure. Its content (6,462 lines) moves into 8 child modules. The parent becomes a re-export barrel of about 250 lines.
- **Blocking issue:** the three monoliths already sit in import cycles with each other and with `Shell`, `Interpreter`, and `ShellSync`-adjacent code (section 5). New child modules will reproduce those cycles unless Phase 0 (section 7) cuts them first.
- **Taxonomy:** child folders under `🧱️elements/<Element>/` are checked against whitelists in `🔣️taxonomy.json`. Most existing ShellHost children are not registered, and every new child in this plan needs registration (section 2). Path budget is already tight: an existing ShellHost path is 254 UTF-8 bytes against a taxonomy `maxPathBytes` of 240.

## 1. TypeScript child-module convention (observed)

**1.1 Layout.** Children are folders named `<single emoji><kebab-slug>/` containing `🟦️.ts` (pure) or `🟦️.tsx` (React). Nesting is allowed; example: `E/🏛️ShellHost/🗨️dialog-origin/🛂️admission/📄️document/🟦️.ts`.
- Depth-1 child module folders: ShellHost 17 (`⌨️window-scope`, `🔀️surface-switch`, `🌱️artifact-creation`, `🗨️dialog-origin`, `🩺️fault`, ...), ShellHelpers 5 (`⏪️time-travel`, `⏯️tool-run-panel`, `🎚️measure-controls`, `📌️panel`, `🧩️contributions`), World3dHost 2 (`⏯️tool-run-trace`, `📏️modelling`).
- Across all element folders, 13 contain at least one depth-1 child module.
- Test and fixture siblings follow generic names used everywhere: `🧪️tests`, `🧫️fixtures`, `🧬️schema`.

**1.2 Import specifiers.** Relative, with emoji path segments, always with explicit extension.
- Parent to child: `import { prepareDocumentSurfaceV1 } from "./🔀️surface-switch/📄️document/🟦️.ts";` (ShellHost:5).
- Cross-element: `from "../🌐️World3dHost/🟦️.tsx"` (ShellHost:508), `from "../🛠️ShellHelpers/🟦️.tsx"` (ShellHost:742).
- Depth rule: a file moved from `E/🏛️ShellHost/🟦️.tsx` to `E/🏛️ShellHost/<child>/🟦️.ts` gains one `../` on every relative specifier. Automate this rewrite; do not hand-edit.
- Package imports use `@semio-tech/...`.

**1.3 Barrels and re-exports.**
- No `export * from` exists in any element file (0 matches across `E`).
- Re-exports are explicit named lists. Examples: `ShellHelpers:1857` `export { buildSpacePanelState, ... } from "./📌️panel/🟦️.ts";`; `ShellHelpers:4249` re-exports from `../🔌️PluginRuntime/🟦️.tsx`; `ShellHost:1512` `export type { DirectoryCommandErrorCodeV1, DirectoryCommandReceiptV1 };`.
- The plan keeps this style: each parent gets an explicit `export { ... } from "./<child>/🟦️.ts(x)"` list covering every current `export` name.

**1.4 Header and docstring style.**
- Some children open with `// #region 🧲️Header`, a breadcrumb `// 🎨️ framework/products/os/modules/renderer/engine/elements/<Element>/<child>/component.tsx`, a JSDoc, and `// #endregion 🧲️Header`. 7 of 24 non-test ShellHost child files use this form.
- The JSDoc opens with an emoji (for example `/** 🔀️ The navbar's two switching axes ...`), matching AGENTS.md. Use the same form in new files.

**1.5 Purity precedent (cycle avoidance) already in the repo.**
- `🔀️surface-switch/🟦️.ts` header: "no React, no shell imports, so a law can drive it".
- `📌️panel/🟦️.ts` header: importing `🛠️ShellHelpers/🟦️.tsx` from a test hits the `ShellHelpers -> Shell -> ShellHost` cycle and fails with `Cannot access '__vite_ssr_import_8__' before initialization`.
- `🐚️Shell/🟦️.tsx:67-73`: `shellLabel` is imported directly from `ShellHelpers`' implementation file, "not via the barrel", for the same reason.
- Conclusion: the codebase already relies on leaf modules and direct-implementation imports. The plan extends that pattern.

## 2. Taxonomy rules for children of `🧱️elements/<Element>/`

Source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Checked by reading the JSON with python (no validator run).

**2.1 Admissible names.**
- `elementsDirName` = `🧱️elements`; `semanticDirectoryKinds.elements` slugPattern `^elements$`.
- `semanticDirectoryMemberKinds.members-of-elements` (91 names) admits element folder names, including `🛠️ShellHelpers`, `🏛️ShellHost`, `🌐️World3dHost`.
- Child names admitted under an element folder:
  - `members-of-members-of-elements` (owner `members-of-elements`): `🧭️gumball`, `🗂️local-catalog`, `📎️local-folders`.
  - `os-renderer-artifact-creation`: `🌱️artifact-creation`.
  - `os-renderer-packed-text`: `🧳️packed-text`.
  - Directory kinds with `parentKindIds` containing `members-of-elements`: `ui-ports-interactive-jobs` (`📡️interactive-jobs`), `ui-diagram-layout` (`📐️layout`), `component-crate` (`🧩️component`).
  - Test, fixture, and schema folders (`tests`, `fixtures`, `schema`) appear under all three monoliths. I did not confirm from the taxonomy that they are admitted directly under element folders; treat them as established by usage, not by a rule I verified.
- `typescript-source` (`🟦️` with `.ts`, `.tsx`, `.d.ts`, ...) and `story` (`.story.tsx`) are the file kinds in use.

**2.2 Not admitted.** These child folders have zero occurrences of their slug anywhere in `🔣️taxonomy.json` (string search, ignoring emoji): ShellHost `⌨️window-scope`, `🎟️invite-capability`, `🎯️input-ledger`, `👥️presence-scope`, `📇️directory-bootstrap`, `📇️space-directory`, `📣️replay-refusal`, `🔀️surface-switch`, `🗨️dialog-origin`, `🧭️route-ledger`, `🪟️mounted-window-refresh`, `🪟️spawned-program`, `🪪️host-bootstrap`; ShellHelpers `⏯️tool-run-panel`, `🎚️measure-controls`, `📌️panel`; World3dHost `📏️modelling`. Enforcement status is unverified (validator not run). Either the validator does not check these folders or they are already failing.

**2.3 Consequence for this plan.** Every new child folder named below is marked `NEW` and must be registered before it validates. Two ways: (a) append the slug to `members-of-members-of-elements.memberNames`, or (b) add a `semanticDirectoryKinds` entry with `parentKindIds: ["members-of-elements"]`. Pick emoji that are unique among siblings of the same parent.

**2.4 Name and path rules.**
- Emoji: one grapheme, followed by VS16 (`variationSelectorPolicy.requiredAfterEmoji: true`). `pathEmojiPolicy.genericEmojiIdentities` lists `📁️ 📂️ 📄️`; avoid those for new children.
- Slugs: kebab-case (`^[a-z0-9]...`).
- Collision policy compares names by byte, NFC, case-fold, VS16-fold, and kind. `maxPathBytes` = 240.
- Measured: deepest existing ShellHost path is 254 UTF-8 bytes (`E/🏛️ShellHost/🧫️fixtures/🗨️dialog-origin/🛂️admission/📄️document/🧪️tests/🧫️fixtures/🧫️folder-archive-persistence/🔣️.json`). Already above 240. AGENTS.md uses 256 characters. Every new child in this plan sits at depth 1 (about 160-175 bytes for `E/🏛️ShellHost/<child>/🟦️.tsx`). Do not add deeper nesting.

## 3. Consumers of the three entry points

Full lists with imported names are in Appendix D.

- **ShellHost** (32 files, 57 names): the React target `🎯️targets/⚛️react/🟦️.tsx` (about 40 names, including `FrameworkOsShell`, `TutorialRecorder`, `shellActorId`, `directoryCommandFromAction`); `🐚️Shell/🟦️.tsx:74` (`FrameworkOsShell`); 12 element host files plus World3dHost import `useMapContextMenuSpecs` and/or the window contexts (`Interpreter`, `Board2dHost`, `BlockListHost`, `EventFeedHost`, `Table`, `TiledMapHost`, `NodeGraph`, `GraphTimelineHost`, `DiffViewHost`, `Canvas2dHost`, `Paint2dHost`, `InkCanvasHost`); `✏️TextEditor` imports `useAppKeybindingsByActionId`; many tests under `🧪️tests` import internals (`dispatchInvokeExtensionEffect`, `captureSpaceArtifactCreationCatalogAuthorityV1`, `RUNTIME_DIAGNOSTICS_KEY`, ...).
- **World3dHost** (27 files, 71 names): `🎯️targets/⚛️react` (about 30 names); `Interpreter` (`WindowInstanceIdContext`, `World3dHost`, `leftoverWorldSelectionOverlayV1`); many world3d tests under `🧪️tests`; `✏️s/...` gis presentation files import `ServiceMountedViewV1` from ShellHost.
- **ShellHelpers** (31 files, 115 names): `🎯️targets/⚛️react` (about 45 names); `UtilityTree`, `Board2dHost`, `Interpreter`, `ChromePanels`, `ShellSearch`, `TextEditor`; the puzzle3d brush-mesh test in `✏️s/🔌️plugins/🧩️puzzle/...` imports 12 names; many tests import internals.

## 4. ShellHost (`E/🏛️ShellHost/🟦️.tsx`, 13,692 lines)

### 4.1 Top-level sections (module level, lines 1-2276)

Declaration counts from `/usr/bin/grep -nE '^(export )?(default )?(async )?(function|const|let|class|interface|type|enum) '`: 123 top-level lines; 946 two-space-indented declarations (689 of them inside the closure).

| Lines | Contents | Kind | Lines |
|---|---|---|---|
| 1-6 | First import block | imports | 6 |
| 7-13 | `#region Header` JSDoc | header | 7 |
| 15-821 | `#region Adapters`: about 107 import statements through line 820, plus `scopeRuntimeKey` (239), `hubSessionStorageV1` (250) | imports, helpers | 807 |
| 823-863 | BuiltNodeReconciler: `withLocalizedWindowKindLabels` (830), `flattenPanelTabNodes` (840), `panelDefinitionPath` (847), `requiredHostPanelLeafId` (857) | pure | 41 |
| 865-1036 | `#region FrameworkOsShell`: contexts `SetWindowTitleContext` (867), `SetWindowIconContext` (870); types `ShellCatalogProbe` (874), `ServiceMountedViewV1` (885); `shellCatalogProbePlugins` (891); `EMPTY_KEYS_BY_ACTION_ID` Map (902); `TRANSIENT_NOTICE_TONE_CLASS` (909); `SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS` (917); `scheduleShellTransientNoticeDismissV1` (920); constants (926-939); `mutationCodeLabelKey` (939) | contexts, types, consts, pure | 172 |
| 973-1034 | `#region OperationSettle`: settle constants (977-984), `shellUncommittedEditCountV1` (991), `StartedTypedOperationV1` (1004), `startedTypedOperationV1` (1007), typed-operation cancellation (1024-1031) | pure, consts | 62 |
| 1037-1061 | `WINDOW_FAULT_LABEL_KEYS` (1038), `WindowFaultStatus` component (1051) | React | 25 |
| 1062-1086 | `AppKeybindingsContext` (1062), `useAppKeybindingsByActionId` (1065), `useMapContextMenuSpecs` (1070), `seededActiveWindowId` (1080) | context, hooks | 25 |
| 1087-1166 | `invocationPluginIdV1` (1087), `windowActionInvocation` (1092), module Map `RELEASED_POINTERS_BY_WINDOW` (1119), `windowHostEventHandlersV1` (1124), encode helpers (1140-1153), `appOwnsCommand` (1154), `appCommandTakesPageRun` (1162) | pure, module state | 80 |
| 1167-1208 | `#region TutorialOverlayHosts`: `tutorialAssetSrcToUrl` (1171), `TutorialCaptionsHost` (1178), `TutorialVideoOverlayHost` (1187), `TutorialGhostPointerHost` (1202) | React | 42 |
| 1210-1357 | `#region TutorialRecorder`: `OBSERVED_INTERACTION_ACTION_IDS` (1211), `tutorialInteractionSelectionActions` (1214), `diffTutorialUiSnapshot` (1238), `tutorialCameraPoseEquals` (1276), `class TutorialRecorder` (1291) | pure, class | 148 |
| 1359-1396 | `FrameworkOsShellProps` interface (1365) | type | 38 |
| 1397-1472 | `announcedPageStreams` (1397), `readViteSEnv` (1415, `import.meta.env`), `hubBootstrapOriginV1` (1428), `identityMutationEnvelope` (1445), `decodeIdentityPayload` (1462) | pure | 76 |
| 1474-1510 | `shellActorId` (1474), `canonicalSurfaceId` (1480), `reloadRetainsActiveApp` (1485), space index constants (1495-1497), `findDialectApp` (1503), `SURFACE_ROLE_ICON_IDS` (1510) | pure, const | 37 |
| 1512-1729 | `#region SpaceArtifactCreation` family: `DirectoryCommandResultSlotV1` type, `retainDirectoryCommandResult` (1523), `mintDirectoryCommandRequestId` (1535), `SpaceArtifactCreation*` types and helpers (1544-1708), `directoryCommandFromAction` (1709) | types, pure | 218 |
| 1731-1870 | `#region SpaceAdministration`: `ShellSpaceAdministrationStateV1` (1735), `shellSpaceAdministrationOpening` (1750), capability/reduce/request helpers (1764-1847), `SHELL_SPACE_ADMINISTRATION_INVITE_TTL_SECS` (1848), `DirectoryInviteClipboardV1` (1852), `copyDirectoryInviteCapabilityV1` (1858) | pure, async | 140 |
| 1877-2110 | `resolveShellScopeStorage` (1877), `commandStallLineV1` (1896), `HUB_CATALOG_REFRESH_MS` (1909), `pluginInstallConcurrency` (1911), `#region RuntimeDiagnostics` 1917-1980 (`RUNTIME_DIAGNOSTICS_KEY` 1929, module `let` pair 1946-1947, `setRuntimeDiagnostics` 1951, `runtimeDiagnosticsEnabled` 1960), `#region InvokeExtensionDispatch` 1981-2107 (`captureExtensionCompletion` 1982, `EXTENSION_WORKER_LOST_FAULT` 1995, `runCapturedExtensionEffect` 1997, `runInvokeExtensionEffect` 2066, `extensionProgramForV1` 2072, `dispatchInvokeExtensionEffect` 2091) | pure, module state, async | 234 |
| 2112-2146 | `FrameworkOsShell` (public component wrapper) | React | 35 |
| 2148-2239 | `#region BuiltNodeStoreCache`: `BuiltNodeStoreCacheV1` type (2170), `createBuiltNodeStoreCacheV1` (2182), `publishBuiltNodesV1` (2225), `publishPanelBodiesV1` (2232) | pure | 92 |
| 2242-2276 | `SPAWNED_FOCUS_FRAME_BUDGET` (2242), `mintPreferenceLaneIdV1` (2250), `logUnlessRetiredV1` (2254), `tellSyncAttachFailureV1` (2262), `SpawnedRefreshSlotV1` (2275) | pure | 35 |
| 2277-13691 | `FrameworkOsShellInner` closure (component body; declarations at 2-space indent) | React | 11,415 |
| 13692 | `#endregion FrameworkOsShell` | | 1 |

Top-level line count check: 1-2276 is 2,276 lines; 2277-13692 is 11,416 lines; total 13,692.

### 4.2 Closure segments (lines 2277-13691)

Full table with in/out name counts is Appendix A. Summary of the regions that matter for the split:
- The `(no-region)` core, lines 2304-2605 (302 lines, 72 declarations), declares shared state and refs. Out-count 126: most other segments read it.
- Lines 2800-4030 (1,231 lines, 147 declarations): identity, hub session, directory home, document opening, browser-actor UI, plugin install refs. Out-count 144.
- Lines 6289-8744 (2,456 lines, 41 declarations): `refreshSpawnedUi*`, `applyHostEffects` (6864-7489), `applyShellUri` (7490), `openDocument`/`closeDocument` (7632/7780), local-folder and sync targets (7871-7961), `spawnProgram` (7962), `onAction` (7998-8673). In-count 192, the widest seam in the file.
- Lines 13370-13691 (322 lines, JSX return) read 116 names and export none.

### 4.3 Proposed partition (ShellHost)

**Module-level children.** Every range below is moved verbatim (after the depth rewrite). The parent re-exports every `export` declared in these ranges.

| New module (NEW = to register, section 2) | Lines moved | Lines | Contents and exports the parent must re-export | Imports it needs (after Phase 0) |
|---|---|---|---|---|
| `E/🏛️ShellHost/🔖️shell-model/🟦️.tsx` (NEW, React, cycle breaker) | 823-1357 | 535 | Contexts `SetWindowTitleContext`, `SetWindowIconContext`; keybinding hook `useAppKeybindingsByActionId`; `useMapContextMenuSpecs`; `TutorialRecorder`; `invocationPluginIdV1`; `windowHostEventHandlersV1`; `appOwnsCommand`; `appCommandTakesPageRun`; `tutorialInteractionSelectionActions`; `startedTypedOperationV1`; `typedOperationCancellation*`; `scheduleShellTransientNoticeDismissV1`; `shellCatalogProbePlugins` | ShellHelpers labels leaf (after Phase 0), `../🎥️...` tutorial types; never the ShellHost parent |
| `E/🏛️ShellHost/🆔️identity-directory/🟦️.ts(x)` (NEW) | 1359-1876 | 518 | `FrameworkOsShellProps` (1365); `shellActorId`, `canonicalSurfaceId`, `reloadRetainsActiveApp`, `SURFACE_ROLE_ICON_IDS`; `DirectoryCommandResultSlotV1` family; `SpaceArtifactCreation*` family; `directoryCommandFromAction`; `ShellSpaceAdministrationStateV1` and helpers; `copyDirectoryInviteCapabilityV1` | existing `../../../../📇️directory/...` imports (depth +1); `import.meta.env` at 1415 stays valid under Vite |
| `E/🏛️ShellHost/🔌️shell-backend/🟦️.ts` (NEW, pure/async) | 1877-2110 and 2148-2276 | 363 | `setRuntimeDiagnostics`, `runtimeDiagnosticsEnabled`, `RUNTIME_DIAGNOSTICS_KEY`, `EXTENSION_WORKER_LOST_FAULT`, `runInvokeExtensionEffect`, `extensionProgramForV1`, `dispatchInvokeExtensionEffect`, `BuiltNodeStoreCacheV1`, `createBuiltNodeStoreCacheV1`, `publishBuiltNodesV1`, `publishPanelBodiesV1` | Owns module `let runtimeDiagnosticsOverride/Resolved` (1946-1947): single owner |
| Parent `E/🏛️ShellHost/🟦️.tsx` (keeps) | 1-6 (rewritten), 7-13, 2112-2146, 2277-2605, 13370-13691 | ~800-1,000 | `FrameworkOsShell`; core state (2304-2605) that builds `ShellHostCore`; JSX (13370-13691); `export { ... } from` lists for all moved names; the existing `export type` (1512 moves to identity-directory, re-exported) | all child hooks and modules |

Module-level residue after the three children: `OperationSettle` consts (973-1034) move to `shell-model` as well (included in its range).

**Closure hook modules.** Each closure range becomes `export function useXxx(core: ShellHostCore)` in its own child module, called from the parent in the original order. The hook returns the names later ranges read (see Appendix A for in/out).

| ID | Range | Lines | Proposed module (NEW) | Contents |
|---|---|---|---|---|
| H1 | 2606-2799 | 194 | `E/🏛️ShellHost/🧵️session-work/🟦️.ts` | OperationSettle, SurfaceSwitch, TreeWindows, BuiltNodeStores regions |
| H2 | 2800-3108 | 309 | `E/🏛️ShellHost/🆔️identity-binding/🟦️.ts` | uiDevice/theme/driver, agent bridge, identity state, hub session, directory home refs |
| H3 | 3109-3511 | 403 | `E/🏛️ShellHost/🗃️document-ports/🟦️.ts` | hub connection port, inference port, document opening, archive, browser-actor UI, space directory refs, plugin supervisor refs |
| H4 | 3512-4030 | 519 | `E/🏛️ShellHost/🧵️backbone-worker/🟦️.ts` | `ensureBackboneWorker` and worker wiring. Line 3514 `new Worker(new URL("../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url))` needs `../` +1 |
| H5 | 4031-5391 | 1,361 | `E/🏛️ShellHost/🔌️plugin-runtime/🟦️.ts` | LocalCatalog region, space administration dispatch, inference retire, shell route (4420-4460), PluginRuntime region (4466-5210), spawned apps (5211-5391) |
| H6 | 5392-6288 | 897 | `E/🏛️ShellHost/🎯️input-and-refresh/🟦️.ts` | InputLedger (5392-5436, defines `globalThis.__semioInputLedger` at 5431), register/utility/tool (5437-5864), ContributionsPush (5865-5985), UiRefreshCoalescing (6225-6288) |
| H7 | 6289-7489 | 1,201 | `E/🏛️ShellHost/🪟️spawned-session/🟦️.ts` | `refreshSpawnedUi*`, session keys, `updateSpacePanel`, `retireSessionInstance`, `switchToPluginApp`, `applyHostEffects` (6864-7489). Widest seam, do late |
| H8 | 7490-8744 | 1,255 | `E/🏛️ShellHost/🧭️document-opening/🟦️.ts` | `applyShellUri`, `openDocument`/`closeDocument`, local folders and sync targets, `spawnProgram`, `onAction` (7998-8673), `noteShellCommand`. Widest seam, do late |
| H9 | 8745-9566 | 822 | `E/🏛️ShellHost/🎥️tutorial-orchestration/🟦️.ts` | TutorialOrchestration (8745-9161), DirectoryLane (9163-9193), ui preference projection effects (9323-9350), layout/mode switching (9351-9566) |
| H10 | 9567-10300 | 734 | `E/🏛️ShellHost/🔖️surface-roles/🟦️.ts` | SurfaceRoles (9567-10031), ThemeMutators (10033-10250), DriverMutators (10252-10299) |
| H11 | 10301-10781 | 481 | `E/🏛️ShellHost/🗂️settings-task-manager/🟦️.ts` | keybinding overrides, settings and marketplace hosts, framework tabs, task-manager refs, DocumentTransfer (10631-10778) |
| H12 | 10782-11317 | 536 | `E/🏛️ShellHost/🧾️check-in/🟦️.ts` | agent approvals, app keydown, browser-actor panel selection, CheckIn region (11049-11294), footer history refs (11295-11317) |
| H13 | 11318-11712 | 395 | `E/🏛️ShellHost/🧰️footer-command-bar/🟦️.ts` | SyncLeaf (11319-11371), mode/example/role switchers, resolved commands and `onCommand` (11498-11666), command tabs |
| H14 | 11713-12492 | 780 | `E/🏛️ShellHost/🧭️dock-assembly/🟦️.ts` | DockAssembly (11713-12400), mobile panel (12401-12439), PanelTabBarHosting (12440-12492) |
| H15 | 12493-13369 | 877 | `E/🏛️ShellHost/🧭️navbar-mode-windows/🟦️.ts` | navbarItems/searchItems/modeWindows/effectiveModeLayout (12493-12944), canvas and footer items (12945-13152), dev probes ReadinessBeacon and CatalogSmokeProbe (13153-13266), ShellContextMenu (13268-13369) |

Hook ids H1-H15 cover lines 2606-13369 contiguously. Parent keeps 2277-2605 and 13370-13691.

Note: H15 includes the dev probes (13153-13266). Those probes read `window.__semioOsCatalogProbe` (docstring at 872) and must keep their dev-only guard.

**Hook order.** The parent must call H1 through H15 in the original textual order. React hook call order is then unchanged, and forward references that the closure resolves through `*Ref` indirection (for example `refreshUiRef`, `applyShellUriRef`, `dispatchCheckpointRef`, `replayShellOwnedCommandRef`) keep working.

**Typed core.** `ShellHostCore` is built in the parent from 2304-2605 (state, refs, reducers, memos, dispatch). H2 onward receive it. Hooks that feed later hooks return a named bag. Type the bag explicitly; do not pass `any`.

### 4.4 Hazards (ShellHost)

- **Cycles to cut in Phase 0** (see section 5 for the global list): `ShellHost <-> World3dHost` (World3dHost:171 imports `runtimeDiagnosticsEnabled`, `SetWindowIconContext`, `SetWindowTitleContext`, `useMapContextMenuSpecs`; ShellHost:508 imports World3dHost); `ShellHost <-> Interpreter` (ShellHost:454-455, Interpreter:171 `useMapContextMenuSpecs`); `ShellHost <-> Shell` (ShellHost:493 and Shell:74 `FrameworkOsShell`; ShellHelpers:216-230 imports runtime values from Shell).
- **Module-evaluation-time calls.** ShellHost:2153 `const PENDING_WINDOW_UI_NODE = pendingWindowUiNode();` and ShellHost:1211 `new Set([...])` read imported bindings during module evaluation. Moving their dependencies is fine only while those modules stay cycle-free.
- **Worker URL and Vite-specific imports.**
  - ShellHost:791 `import pluginModuleStoreWorkerUrl from "../../../../🔌️plugin/📇️registry/🌎️hub-source/👷️service-worker/🟦️.ts?worker&url";`
  - ShellHost:3514 `new Worker(new URL("../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url), { type: "module" })`.
  - Both resolve relative to the file location. In a child at depth 1 they need one more `../`. Verify in the bundle output; no runtime run was done here.
- **`import.meta.env` uses:** ShellHost:1400, 1417, 1965, 4500, 5380. These keep working after a move (Vite replaces them), but must stay in files the Vite build compiles.
- **Global side effects in closure:** `Object.defineProperty(globalThis, "__semioInputLedger", ...)` at 5431 (H6); `window.addEventListener("pointermove", trackPointer, ...)` at 4414 (H5); dev probe `window.__semioOsCatalogProbe` (docstring 872, H15).
- **Module-level mutable state.** Each must have exactly one owner module:
  - `EMPTY_KEYS_BY_ACTION_ID` (902, Map, read-only use).
  - `RELEASED_POINTERS_BY_WINDOW` (1119, Map, mutated).
  - `runtimeDiagnosticsOverride` and `runtimeDiagnosticsResolved` (1946-1947, `let`).
- **TDZ and ordering.** Closure-level `const` declared after a callback that reads it is safe only while the callback runs after the declaration. Keep the `*Ref` pattern. Do not move a block so that a module-evaluation-time read sees an uninitialized binding.
- **Hook rules.** Do not wrap hook blocks inside conditionals while extracting. Confirm each block's hooks are unconditional before moving it.
- **Public API.** All 62 `export` lines remain exported from the parent. Consumers in Appendix D (including the React target and tests) must keep compiling without edits, apart from any deliberate redirect in Phase 0.
- **Path budget.** Depth-1 children only (see 2.4).

## 5. World3dHost (`E/🌐️World3dHost/🟦️.tsx`, 8,551 lines)

### 5.1 Top-level sections (module level, lines 1-6255)

Declaration counts: 377 top-level lines. Component body `World3dHost` is 6266-8550 (2,285 lines) plus its JSDoc at 6256-6265.

| Lines | Contents | Kind |
|---|---|---|
| 1-8 | header, first import (`publishedPageUrl`), `#region Header` | header, import |
| 9-175 | `#region Adapters`: imports (about 26 statements), `const { useFrame, useLoader, useThree } = sceneHostPort.fiber;` (115, module-eval destructuring) | imports, port binding |
| 177-437 | `#region WorldSceneParsing` types: `WorldMeshData` (179), `WorldMeshAttribute` (201), `WorldCameraRecord`, `WorldMeshRecord`, `WorldInstanceRecord` (223), selection, hover, interaction, LOD, vortex, attraction, target volume, reference, point-cloud, engagement-preview record types | types |
| 438-753 | `#region WorldMeshPaint`: `MESH_STYLE_PAINT` (452), `resolveMeshStylePalette` (465), `useMeshStylePalette` (479), `resolveMeshStyle` (500), `semanticColorsFromPalette` (555); nested `#region WorldInstanceCelebrate` 565-753: module Map/Set and `let` version/timer (567-570), celebrate API (607-634), `CelebratingConicMaterial` (697) | pure, React, module state |
| 754-939 | `#region WorldViewportCamera`: `WorldParsedCameraState` (756), `parseWorldProjectionField` (759), `parseCameraState` (770), `mergeWorldViewportCamera` (796), `world3dViewportCameraSeedKey` (811), camera DOM and dispatch helpers (846-934) | pure |
| 940-1068 | env/frame/fit record types and parsers (941-991), `WORLD3D_FRAME_BOUNDS_MARGIN` (997), `world3dFrameDistanceForRadius` (1009), `world3dBoundsRadius` (1018), `fitCameraFromBounds` (1022), `world3dAutoFitOwed` (1047), `world3dAutoFitKey` (1056) | pure |
| 1069-1182 | `WorldAutoFit` component (1069), `WorldAcceptedFrameDiagnostics` (1127-1182) | React |
| 1183-1287 | `world3dFrameCameraFromBounds` (1183), `world3dTableScaleInstances` (1195), `world3dFrameCameraFromInstances` (1212), `autofitCameraFromInstances` (1231), `seedPendingWorldProjectionCamera` (1265) | pure |
| 1288-1353 | `WorldProjectionContentFrame` component (1288) | React |
| 1354-1500 | mesh kind geometry and parse: `worldMeshKindGeometry` (1354), `meshDataFromKind` (1383), `parseMeshes` (1407), `splitJsonArrayElements` (1423), residency types and `advanceWorldMeshResidency` (1470-1500) | pure |
| 1501-2161 | instances, selection, leftover overlay: `parseInstances` (1501), `WORLD_INSTANCE_RECORD_TEXT` WeakMap (1510), record interning (1523), delta and residency (1533-1594), `parseSelection` (1596), `#leftover` module state (1604-1632: Map, `let`, Set, WeakMap), publish/subscribe (1665-1780), merges (1853-1915), `parseJsonArray` (1963), selection domains (1973-1993), `mapContextMenuSpecs` (2010), parse interaction/LOD (2045-2063), context-menu target, pick and raycast helpers (2066-2139), surface attribute helpers (2143-2161) | pure, module state |
| 2162-2592 | geometry and appearance: `geometryFromMesh` (2162), `buildWorldMeshAppearance` (2259), `MeshVisuals` (2311), overlays (2330-2425), `PaintTexturedMesh` component (2435), `GLB_MESH_LOCAL_BOUNDS` Map (2503), `glbMeshFrameCorners` (2506), `world3dInstanceLocalCorners` (2542), `GLB_EDGE_GEOMETRY_CACHE` WeakMap (2565), `applyGlbMeshEdgeBorders` (2568), `cloneWorldGlbSurface` (2595) | pure, React, module state |
| 2593-2916 | GLB surface components: `disposeWorldGlbSurface` (2631), `GlbInstanceMesh` (2639), `GlbSubElementLayer` (2714), `extractGlbCollisionMesh` (2807), `BrushMeshRegistrar` (2849), `ToolRunTraceGlbGeometry` (2861), `WorldToolRunTrace` (2883) | React |
| 2917-3313 | gumball math and paint/gumball gesture state machines: `isWorldTransformGumballMode` (2917), `gumballKindForTransformMode` (2921), `world3dGumballSelectionArgsV1` (2937), delta helpers (2948-2995), `WorldPaint*` types and `worldPaintStep` (2995-3031), `WorldGumball*` types and `worldGumballStep` (3033-3118), live-preview helpers (3120-3198), owner-pose helpers (3212-3273), `gumballConfigForTransformMode` (3274), `worldGumballConfigForProjection` (3287), `useUiCanvasHovered` hook (3292) | pure, hook |
| 3314-3369 | `SceneGumball` component | React |
| 3370-3832 | `WorldInstanceNode` (memo component, 3370), `useWorldMeshVisuals` (3853) plus constants (3834-3837) | React |
| 3833-4399 | `#region WorldInstancesLayer`: `WorldInstancesLayer` component (3895) | React |
| 4401-4447 | `#region WorldPointCloudLayer` | React |
| 4449-4625 | `#region WorldVortexMarkers`: `WorldVortexMarkers` component (4499) and helpers | React |
| 4626-5153 | `WorldConnectRubberBand` (4628), `WorldVoxelPreviewBox` (4643), `WorldAttractionLines` (4652), `worldGhostMeshUrl` (4671), `DemandInvalidateOnToken` (4678), `CatalogueDropPreviewInvalidate` (4687), `WorldSurfaceStatusPane` (4707), `WorldComputeStatusPane` (4715), `EngagementPreviewLayer` (4781), `suggestionMenuItems` (4839), marquee constants and pure picks (4862-5153) | React, pure |
| 5154-5386 | `CameraRefBridge` (5154), `IntroductionWorldResolverBridge` (5175), `RaycasterPickTuning` (5269), `WorldGumballHitStamp` (5279), `WorldVortexHitStamp` (5321), `paneSuffixFromSurfaceId` (5351), `worldRayFromNdc` (5360), `raycastGroundPoint` (5375) | React, pure |
| 5387-5591 | `#region CatalogueDrop`: payload types and parse (5388-5410), `snapWorldPointToGrid` (5411), `#region WorldRelocateGesture` 5417-5471, `#region WorldVolumeBrushGesture` 5473-5490, `#region WorldCatalogueDropPreviewStore` 5503-5568 (module Map/Set, `let`-free), `CatalogueDropGhost` (5570) | pure, React, module state |
| 5593-5648 | `#region WorldSelectionPreviewStore`: `worldSelectionPreviewByController` Map (5601), listeners Set (5602), subscribe/get/set helpers | store, module state |
| 5650-5768 | `#region WorldGumballTransformPreviewStore`: Map (5662), listeners Set (5663), `worldVorticesWithGumballPreview` (5722), `worldAttractionsWithGumballPreview` (5745) | store, module state |
| 5770-5903 | `#region WorldInstanceChromeStore`: `WorldInstanceChromeSnapshot`, `createWorldInstanceChromeStore` (5825), `WorldInstanceChromeContext` (5878, `createContext`), `useWorldInstanceChrome` (5886) | store, React context |
| 5904-6064 | `axisDragParam` (5909), `world3dProjectionPaneElementId` (5933), `WorldOrbitProjectionSwitchPane` (5938), `#region WorldWindowInstance`: `WindowInstanceIdContext` (5972), `World3dWindowViewStoreV1` (5987), `World3dWindowViewRegistryV1` (6011), `World3dWindowViewStoreContext` (6033), pending projection Map (6038) and register/peek/clear (6041-6063) | React, store, module state |
| 6065-6254 | `#region WorldInteractionDomain`: `WORLD3D_DEFAULT_INTERACTION_GRANULARITY` (6069), hover and selection action args (6074-6222), `WORLD3D_DEFAULT_MARKER_GRANULARITY` (6234), marker interaction target (6251), module Set `world3dSuggestionsRightDownRoutes` (6108) and window listeners (6142-6143) | pure, module state, side effects |
| 6256-8550 | `World3dHost` component (6266-8550; JSDoc at 6256-6265) | React |

### 5.2 Proposed partition (World3dHost)

Every range in 177-6254 belongs to exactly one module below. The parent keeps 6255-8551 (plus its own imports) and re-exports every `export` declared in the moved ranges.

| ID | New module (NEW) | Lines moved | Lines | Kind | Contents |
|---|---|---|---|---|---|
| W-a | `E/🌐️World3dHost/🧬️scene-schema/🟦️.ts` (NEW) | 177-437, 754-1068, 1183-1287 | 681 | pure | scene record types, camera parse/merge/framing, env/frame/fit parsing, auto-fit keys, pending camera seed |
| W-b | `E/🌐️World3dHost/🎨️mesh-render/🟦️.tsx` (NEW) | 438-753, 1069-1182, 1288-1353, 1354-1500, 2162-2592 | 1,074 | React and pure | mesh paint and celebrate (module state 567-570), auto-fit and frame diagnostics components, projection content frame, mesh kind and residency, geometry and appearance, GLB bounds and styling |
| W-c | `E/🌐️World3dHost/🧩️glb-surface/🟦️.tsx` (NEW) | 2593-2916 | 324 | React | GLB instance and sub-element layers, collision mesh, brush mesh registrar, tool-run trace geometry |
| W-d | `E/🌐️World3dHost/🔗️instance-selection/🟦️.ts` (NEW) | 1501-2161, 6065-6254 | 851 | pure, module state | instance parse/intern, selection and leftover overlay (module state 1604-1632), interaction parse, raycast helpers, world interaction domain (6065-6254, including module Set at 6108 and window listeners at 6142-6143 for the suggestions routing) |
| W-e | `E/🌐️World3dHost/🧮️gumball-math/🟦️.ts` (NEW) | 2917-3313, 5650-5768 | 514 | pure, hook, module state | gumball and paint gesture state machines, live preview math, owner poses, `useUiCanvasHovered` (3292), gumball transform preview store (module Map 5662, listeners 5663) |
| W-f | `E/🌐️World3dHost/🧱️instance-layers/🟦️.tsx` (NEW) | 3314-4625, 5770-5903 | 1,446 | React | `SceneGumball`, `WorldInstanceNode`, `WorldInstancesLayer`, point cloud, vortex markers, instance chrome store and context (5770-5903) |
| W-g | `E/🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx` (NEW) | 4626-5386 | 761 | React and pure | connect rubber band, voxel preview, attraction lines, status panes, engagement preview, suggestion items, marquee math, camera and introduction bridges, raycast hit stamps |
| W-h | `E/🌐️World3dHost/🗂️catalogue-drop/🟦️.tsx` (NEW) | 5387-5649 | 263 | React, module state | catalogue drop payload and grid snap, relocate gesture, volume brush gesture, catalogue drop preview store (5503-5568), `CatalogueDropGhost` (5570), selection preview store (5593-5648) |
| W-i | `E/🌐️World3dHost/🪟️window-view-store/🟦️.tsx` (NEW) | 5904-6064 | 161 | React, module state | projection pane switch, `WindowInstanceIdContext`, retained viewport store and registry, `World3dWindowViewStoreContext`, pending projection registry (module Map 6038) |
| Parent | `E/🌐️World3dHost/🟦️.tsx` (keeps) | 6255-8551 | ~2,400 with imports | React | `World3dHost` component. Imports rewritten to child modules; re-exports for all moved names |

Coverage: W-a through W-i sum to 6,075 lines. The remaining 3 lines are blank or endregion markers inside 177-6254.

Why W-d, W-e, W-f, and W-h, W-i are shaped this way: the chunk graph has the cycle `178-3831 <-> 3833-4399 <-> 4626-5386 <-> 5387-5591 <-> 5650-5768 <-> 5770-5903 <-> 6065-6254` (section 5.4). The reassignments above remove each back-edge: the instance chrome store (5770-5903) moves beside the instance layers that use it (W-f); the gumball transform preview store (5650-5768) moves beside the gumball math it reads (W-e); the interaction domain (6065-6254) moves beside instance selection (W-d). Confirm with the scripts before moving.

### 5.3 Hazards (World3dHost)

- **Module-level state, one owner each:**
  - `celebratingWorldInstanceUntil` Map (567), `celebratingWorldInstanceListeners` Set (568), `let celebratingWorldInstanceVersion` (569), `let celebratingWorldInstanceTimer` (570): W-b.
  - `worldMeshKindData` Map (1378): W-b.
  - `WORLD_INSTANCE_RECORD_TEXT` WeakMap (1510): W-d. Interning identity depends on a single instance.
  - `leftoverWorldOverlayByWindow` Map (1627), `let leftoverWorldDocumentOverlay` (1628), `leftoverWorldSelectionListeners` Set (1629), `leftoverWorldWindowOverlayMemo` WeakMap (1632): W-d.
  - `GLB_MESH_LOCAL_BOUNDS` Map (2503), `GLB_EDGE_GEOMETRY_CACHE` WeakMap (2565): W-b.
  - `WORLD_INSTANCE_QUATERNION` WeakMap (3837): W-f.
  - `worldCatalogueDropPreviewByController`, `worldCatalogueDropPreviewListeners`, `worldCatalogueDropHostHitTests` (5505-5507): W-h.
  - `worldSelectionPreviewByController`, `worldSelectionPreviewListeners` (5601-5602): W-h.
  - `worldGumballTransformPreviewByController`, `worldGumballTransformPreviewListeners` (5662-5663): W-e.
  - `pendingWorldProjectionByWindowId` Map (6038): W-i.
  - `world3dSuggestionsRightDownRoutes` Set (6108): W-d.
- **Module-evaluation-time reads:** `const { useFrame, useLoader, useThree } = sceneHostPort.fiber;` (115), `WorldInstanceNode = reactHostPort.memo(...)` (3370), `WorldInstanceChromeContext = reactHostPort.createContext(...)` (5878), `WindowInstanceIdContext = createContext(...)` (5972), `World3dWindowViewStoreContext = createContext(...)` (6033). Each context is a single module-level object; duplicating the module would split context identity. Keep one module per context.
- **Global listeners and timers:** `window.setTimeout` (599), `window.addEventListener("pointerdown" | "contextmenu", ..., true)` (6142-6143) installed on first use. Keep the install and remove logic with the functions that own it.
- **Existing cycles through the parent (must be cut in Phase 0):** `World3dHost` (line 170) imports `contextMenuGroupLabel`, `createCoalescingActionDispatcher`, `declareSurfaceCancelAction`, `world3dMarqueeOverlayShape`, `puzzle3dBrushMesh*` from ShellHelpers; `ShellHelpers:209-211` imports `registerPendingWorldProjection` from `World3dHost`. Redirect ShellHelpers' import to `W-i` and World3dHost's ShellHelpers imports to the new `🏷️labels` module (section 6).
- **`World3dHost` (line 171) imports from ShellHost:** `runtimeDiagnosticsEnabled`, `SetWindowIconContext`, `SetWindowTitleContext`, `useMapContextMenuSpecs`. Keep this import in the parent only (all uses are inside the component), and redirect in Phase 0 to `shell-model` and `shell-backend` leaves.
- **Import from tool-run-trace child:** ShellHost:509 imports `toolRunTraceCursorViewState` from `🌐️World3dHost/⏯️tool-run-trace/🟦️.tsx`. Unchanged by this plan.
- **`ShellHost:497-508` imports window-view contexts and registries from `World3dHost/🟦️.tsx`.** After the split, ShellHost may import from the leaf modules (W-i) instead; decide in Phase 0.

### 5.4 Chunk-level cycle detection (module-level)

A Tarjan SCC over the 20 module-level segments of World3dHost (`segs-world3d-module.json`) gives one strongly connected component with 7 members: `178-3831`, `3833-4399`, `4626-5386`, `5387-5591`, `5650-5768`, `5770-5903`, `6065-6254`. Section 5.2 breaks that component. Re-run the script after each move; the target is zero SCCs of size greater than 1 among the new modules.

## 6. ShellHelpers (`E/🛠️ShellHelpers/🟦️.tsx`, 6,708 lines)

### 6.1 Top-level sections

Declarations: 463 top-level lines. The file is one outer `//#region ShellHelpers` (245-6708) containing nested regions. The module is flat: no closure, no component body.

| Lines | Contents |
|---|---|
| 1-244 | Import block (37 statements), `#region Header`, `#region Adapters` (13-243) |
| 246-1206 | General plumbing: `DEFAULT_PANEL_WIDTH_PX` (248), framework category ids (251-256), `PANEL_TAB_BAR_HOSTS` (259), history and reserved-id helpers (285-396), undeclared-action diagnostics (423-466), `ShellCommandInverse` (468), labels (499-511), active example (516-566), leftover interaction views (580-711), presence client and heartbeat consts (713-734), `useUIHistory` (834), media-export download helpers (892-941), `requestFileOpen` (999), natural file browser consts (1033-1040), blob read (1066-1109), effect dispatch (1142-1206) |
| 1207-1223 | `#region 📥️ChunkedImport` |
| 1224-1320 | `dispatchOpenedFiles` (1234), `importOpenedFilesV1` (1266), `scheduleDispatchAction` (1307) |
| 1321-1803 | `#region RequestMediaFrames` (BMFF parser 1322-1355, Tier 1 1357-1623, Tier 2 1625-1712, `requestMediaFramesSourceV1` (1749), `runMediaFramesV1` (1760)) |
| 1804-2124 | Route and app labels: `isStudioMode` (1805), `ShellRoute` type (1809), `parseShellRoute` (1812), `appBreadcrumb` (1829), `resolveAppBreadcrumb` (1834), `resolveArtifactByAppId` (1839), `appWindowLabel` (1847), `panelAnchorForGroup` (1860), framework settings integration (1868-1890), framework layout seeds and conversion (1892-2124) |
| 2125-2160 | `#region WindowLayoutChangeClassify` |
| 2161-2703 | Window engagement, plugin install, panel tab definitions, refusal and notice labels: `windowEngagementControlToSpec` (2162), plugin install types (2214-2256), `loadPluginModuleResilient` (2257), install band texts (2293-2331), viewport engagement (2334-2370), `categoryTabIcon` (2407), `panelTabDefinitionToNode` (2429), `BrowserActorPanelHostV1` (2472), refusal/notice labels (2485-2624), browser-actor UI helpers (2619-2703) |
| 2704-2778 | `#region 🧰️UtilityRegistry`: `resolveUtilities` (2710), `resolveUtilityGroupLabel` (2723), `utilityDefinitionToSpec` (2728), `tagSetActiveUtilityWindow` (2741), `resolveUtilityNodes` (2756) |
| 2779-2842 | `spawnedWindowChromeForKind` (2781), `uiInputField`, `uiIntentPayload`, `uiIntentToActionDescriptor` (2827), `actionBindingToActionDescriptor` (2835) |
| 2843-3130 | `#region 🪟️TreeWindows`: `createTreeWindowSchedulerV1` (2902), `TreeWindowHostV1` (3044), `createTreeWindowContextV1` (3051), tree open-state helpers (3061) |
| 3076-3124 | `PanelTreeConfigCacheV1` (3076), `cachedTreePanelConfigV1` (3078), cached store/body-store/actor helpers (3090-3124), `panelActionRoutesThroughHostV1` (3102) |
| 3125-3169 | `let pendingPanelUiNodeSingleton` (3125), `pendingPanelUiNodeV1` (3126), `uiNodeToTreePanelConfig` (3138), `interpretedTreePanelConfigV1` (3146) |
| 3170-3264 | Shell labels: `shellTabIcon` (3170), `shellLabel` (3192), `shellLabelIfDefined` (3198), `syncShellLabelLocale` (3211), `shellLabelLocale` (3225), `currentShellLocaleV1` (3231), `ribbonParentLabel` (3241), `contextMenuGroupLabel` (3252) |
| 3265-3337 | `FrozenLabel` type and `frozenLabelText` (3267), surface-role and app-mode texts (3272-3316), open-with, default-apps, viewer read-only, none-option texts (3320-3337) |
| 3338-3351 | Command ids (`OPEN_ARTIFACT_WITH_*`, `OPEN_TASK_MANAGER_COMMAND_ID` 3344, `OPEN_HUB_COMMAND_ID` 3350) |
| 3352-3499 | `#region 📤️DocumentTransfer` (archive filenames, natural file format, notices) |
| 3504-3548 | `isMutationKindDefinition`, `filterDefinitionsForRole`, `OpenWithEntry`, `groupOpenWithEntries` |
| 3550-3741 | `#region 🔖️CheckInAndSyncStatus`: `AutoCheckinScheduler` class (3567), `automaticCheckinWaitsV1`, `useCheckpointOnCloseV1` (3656), check-in and sync status texts |
| 3742-3897 | Manifest and app label resolution: `FRAMEWORK_PANEL_TAB_LABEL_KEYS` (3744), `resolvePanelTabLabel` (3753), `EMPTY_APP_LABELS_OVERLAY` (3759), `EMPTY_APP_CATALOGUE` (3774), `synthesizeLocalizedLabel` (3777), `resolveManifestLabel` (3786), `resolveAppLabel` (3798), resolved definition types (3823-3830), `resolveActionArgDef` (3846), `resolveDialogDefinition` (3860), `resolveIntroductionDefinition` (3880) |
| 3898-4060 | `#region 🎥️TutorialUiBridge`: tutorial snapshot capture and apply (3900-4060) |
| 4061-4139 | `shellTerminologyLabel` (4063), `createLatestAsyncDispatcher` (4069), `createDirectionalAsyncDispatcher` (4093), `world3dMarqueeOverlayShape` (4134) |
| 4140-4217 | `#region 🛑️ExtensionRequestCancellation`: module Maps `inFlightExtensionRequestsByActor` (4152), `declaredSurfaceCancelActions` (4160), begin/abort/count/declare helpers |
| 4218-4282 | `createInFlightSkippingInterval` (4224), `createCoalescingActionDispatcher` (4255) |
| 4283-4587 | `#region 🥽️Puzzle3dBrushMeshUpload`: constants (4288-4318), `Puzzle3dBrushMeshRegistry` class (4349), singleton `puzzle3dBrushMeshRegistry` (4455), page and queue helpers (4459-4587) |
| 4588-4924 | Window measures and selection utility: `windowMeasureTreeContainsId` (4590), `WindowMeasureSlider` and friends (4609-4723), `windowMeasuresToTreeItems` (4676), `SelectionUtilityOptions` (4774), `windowMeasureDomId` (4860), `qualifyWindowMeasureIds` (4867), `windowMeasuresChrome` (4875), `utilityNodeTreeContainsId` (4896), `utilityBarNode` (4900) |
| 4925-5601 | `#region 🧰️WindowActionPane`: staged-arg helpers (4928-4960), staged field components (4961-5155), `renderStagedArgControl` (5156), `actionRequiresStagedForm` (5247), keyboard helpers (5253-5335), clipboard and paste (5345-5370), action categories (5376-5420), `WindowActionPane` component (5520), `windowActionPaneNode` (5562) |
| 5603-6095 | `#region 🎛️CommandRegistry`: `ResolvedCommand` (5605), command address and keybinding helpers (5611-5656), `resolveCommands` (5656), categories (5694-5711), `buildOsCommands` (5736), `dispatchOsCommand` (5915), `COMMAND_CATEGORY_ICON = shellTabIcon("wrench")` (5969, module-eval), `buildCommandCategoryTree` (5984), `buildCommandCategoryTabs` (6066) |
| 6097-6235 | `#region 🛠️ToolRegistry`: `buildToolTree` (6107), `buildToolTabs` (6129), tool-tab selection (6147-6186), tool-leaf helpers (6192-6233) |
| 6236-6288 | `uiJsonDeepEqual` (6238), `preserveJsonIdentity` (6268), `mergeRecordPreservingIdentity` (6278) |
| 6289-6707 | `#region UiRefresh`: `UiRefreshCache` (6291), `browserActorDispatchUiScopeV1` (6304), `WINDOW_CONFIG_RAIL_ACTION_IDS` Set (6310), dirty-scope algebra (6438-6488), `createUiRefreshCoalescerV1` (6541), refresh request builders (6667-6707) |
| 6708 | `#endregion ShellHelpers` |

### 6.2 Proposed partition (ShellHelpers)

Each range is assigned to exactly one module. The parent becomes a barrel that re-exports every `export` declared in the moved ranges, plus its existing `export { ... } from "./📌️panel/🟦️.ts"` (1857) and `export ... from "../🔌️PluginRuntime/🟦️.tsx"` (4249) lines.

| ID | New module (NEW) | Lines moved | Lines (approx.) | Contents | Depends on (must not form a cycle) |
|---|---|---|---|---|---|
| SH-1 | `E/🛠️ShellHelpers/🏷️labels/🟦️.ts(x)` (NEW, leaf) | 3170-3264, 3265-3351, 3742-3897 | ~340 | `shellLabel` family, `shellTabIcon`, `ribbonParentLabel`, `contextMenuGroupLabel`, frozen label texts, command id constants, manifest and app label resolution, resolved definition types | none inside ShellHelpers (after moving `EMPTY_SHELL_*` in Phase 0) |
| SH-3 | `E/🛠️ShellHelpers/🧰️effects-and-io/🟦️.ts` (NEW) | 246-1803 | 1,558 | general plumbing, presence, `useUIHistory`, downloads, file requests, effect dispatch, chunked import, media frames | SH-1 |
| SH-4 | `E/🛠️ShellHelpers/🧭️route-and-window-layout/🟦️.ts` (NEW) | 1804-2703, 3076-3124 | ~900 | route and breadcrumbs, layout seeds, layout change classification, window engagement, plugin install, panel tab definitions, refusal and notice labels, `PanelTreeConfigCacheV1` and cached tree-panel helpers (moved from 3076-3124 to break the SH-4 <-> SH-5 cycle) | SH-1, SH-3 (`import type` from SH-5 is allowed) |
| SH-5 | `E/🛠️ShellHelpers/🪟️tree-windows-and-utilities/🟦️.ts(x)` (NEW) | 2704-2842, 2843-3075, 3125-3169, 4860-4924 | ~480 | utility registry, UI intent helpers, `createTreeWindowSchedulerV1`, `TreeWindowHostV1`, `createTreeWindowContextV1`, `pendingPanelUiNodeSingleton` (module `let` 3125), `uiNodeToTreePanelConfig`, window measure chrome helpers (4860-4924, moved so that SH-5 does not import SH-7) | SH-1, SH-3, SH-4 |
| SH-6 | `E/🛠️ShellHelpers/🎛️registries/🟦️.tsx` (NEW, React) | 4925-6235 | 1,311 | `WindowActionPane`, staged-arg controls, action categories, `CommandRegistry`, `COMMAND_CATEGORY_ICON` (module-eval `shellTabIcon("wrench")` at 5969), tool registry and tool tabs | SH-1, SH-3, SH-5, SH-7 (measure tree), SH-8 (only if `dispatchOsCommand` is read there; verify) |
| SH-7 | `E/🛠️ShellHelpers/🧩️measures-and-cancel/🟦️.tsx` (NEW) | 4140-4859 | ~720 | extension request cancellation (module Maps 4152/4160), in-flight interval and coalescing dispatchers (4218-4282), `Puzzle3dBrushMeshUpload` with singleton registry (4283-4587), window measure controls and `SelectionUtilityOptions` (4588-4859) | SH-1, SH-5, SH-8 |
| SH-8 | `E/🛠️ShellHelpers/🧾️surface-role-checkin-tutorial/🟦️.ts(x)` (NEW) | 3352-3548, 3550-3741, 3898-4139 | ~630 | `DocumentTransfer` (3352-3499), `isMutationKindDefinition` and `OpenWithEntry` (3504-3548), `AutoCheckinScheduler` and check-in status (3550-3741), tutorial UI bridge (3898-4060), `shellTerminologyLabel`, latest and directional async dispatchers (4061-4139) | SH-1, SH-3, SH-6 (only if `DocumentTransfer` reads `dispatchOsCommand`; verify) |
| SH-9 | `E/🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts` (NEW) | 6236-6707 | 472 | JSON identity helpers, UI refresh cache, dirty-scope algebra, coalescer, refresh request builders | SH-3 (`downloadMediaExport`) |
| Parent | `E/🛠️ShellHelpers/🟦️.tsx` (keeps) | 1-8, barrel lines | ~250 | `export { ... } from "./<child>/🟦️.ts"` for all 326 export declarations; existing `export ... from` at 1857 and 4249 kept | all children |

Coverage note: the ranges above are declaration-aligned. Verify that their union equals 246-6707 with the script before moving. Items marked "verify" are based on the chunk-level graph and need a symbol check.

Line 1-244 import block: every import is either re-pointed to the child that uses it, or dropped if unused.

### 6.3 Hazards (ShellHelpers)

- **Large SCC at chunk level.** The chunk graph (`segs-shellhelpers.json`) has one strongly connected component with 14 chunks: `246-1206`, `1224-1320`, `2161-2703`, `2704-2778`, `2779-2842`, `2843-3130`, `3131-3257`, `3258-3548`, `3550-3741`, `3742-3897`, `4061-4139`, `4588-4924`, `4925-5601`, `5603-6095`. The partition above breaks it with these cuts:
  - Move the label cluster (3170-3264, 3265-3356, 3742-3897) into SH-1 (leaf). This removes the back-edges that go from label code into the window and tree chunks.
  - Move `PanelTreeConfigCacheV1` and the cached tree-panel helpers (3076-3124) into SH-4. This removes `2161-2703 <-> 2843-3130`.
  - Move `windowMeasuresChrome` and its helpers (4860-4924) into SH-5. This removes `2704-2842 <-> 4588-4924`.
  - Move `createDirectionalAsyncDispatcher` (4093, in 4061-4139) with SH-8. This removes `4588-4924 <- 4061-4139`.
- **Runtime imports from Shell (ShellHelpers:216-230).** `ShellFaultBoundary`, `EMPTY_SHELL_LOCKS`, and other runtime bindings come from `🐚️Shell/🟦️.tsx`. Shell imports `FrameworkOsShell` from ShellHost, so `ShellHelpers -> Shell -> ShellHost -> ShellHelpers` is a cycle. Phase 0 must move those runtime bindings to a leaf (or make ShellHelpers' use type-only). Decide before moving SH-6 or SH-7.
- **Imports from World3dHost (ShellHelpers:209-211).** `registerPendingWorldProjection` comes from the World3dHost parent. Redirect to W-i (section 5.2).
- **Imports from Interpreter (ShellHelpers:241).** `TreeWindowContext`, `wireLabel`, and types come from `🗣️Interpreter/🟦️.tsx`, which imports `shellLabel` back from ShellHelpers (Interpreter:170). After SH-1 exists, Interpreter should import SH-1 directly, which breaks that edge.
- **Module-evaluation-time work:**
  - `shellSegmentedDownloadSinkFactory` (930), `mediaExportEncodingText = wireMediaExportEncoding` (899).
  - `MEDIA_FRAMES_UNCANCELLABLE_RUN_V1` (1719) calls `new AbortController()` at import time.
  - `puzzle3dBrushMeshRegistry = new Puzzle3dBrushMeshRegistry()` (4455).
  - `COMMAND_CATEGORY_ICON = shellTabIcon("wrench")` (5969) calls into SH-1 at import time. SH-1 must be evaluated before SH-6.
- **Module-level mutable state, one owner each:** `let pendingPanelUiNodeSingleton` (3125, SH-5); `inFlightExtensionRequestsByActor` Map (4152) and `declaredSurfaceCancelActions` Map (4160, SH-7); `puzzle3dBrushMeshRegistry` singleton (4455, SH-7); `WINDOW_CONFIG_RAIL_ACTION_IDS` Set (6310, SH-9) and other module Sets (396, 711, 2524, 2627), read-only.
- **Public surface.** 326 `export` lines. Test and editor consumers import internals (Appendix D). Keep every name exported from the barrel.

## 7. Ordered extraction sequence

Each step is an independent change. After each one, re-run the coverage and cycle script on the new layout and confirm zero new SCCs. Do not run builds or tests until the whole phase is complete (these were not run in this audit).

**Phase 0 - cut existing cycles (no body moves; only leaf moves and import redirects).**
1. ShellHost contexts and hooks (`SetWindowTitleContext`, `SetWindowIconContext`, `useAppKeybindingsByActionId`, `useMapContextMenuSpecs`) and runtime diagnostics (`runtimeDiagnosticsEnabled`, `setRuntimeDiagnostics`): move to the NEW `shell-model` and `shell-backend` leaves; redirect the 12 element-host consumers, World3dHost:171, and Interpreter:171.
2. ShellHelpers labels (`shellLabel` family, `shellTabIcon`, `contextMenuGroupLabel`, `ribbonParentLabel`, frozen texts): move to the NEW `labels` leaf (SH-1); redirect Interpreter:170, World3dHost:170, and the Shell:72-73 direct import to the leaf.
3. ShellHelpers runtime imports from Shell (216-230): move to a leaf or make type-only.
4. ShellHelpers:209-211 `registerPendingWorldProjection`: redirect to W-i. Then W-i's module must be the only import site.
5. Re-run `segments.py` / `coupling.py` against the moved leaves. Confirm the graph between the three files has no cycle through the parents.

**Phase 1 - World3dHost module-level (lowest risk).**
6. Create W-a (`scene-schema`), W-d (`instance-selection`), W-e (`gumball-math`) first. They have the fewest external dependencies.
7. Create W-b (`mesh-render`), W-c (`glb-surface`), W-i (`window-view-store`), W-h (`catalogue-drop`).
8. Create W-f (`instance-layers`) and W-g (`marquee-and-bridges`), then reduce the World3dHost parent to 6255-8551 plus imports (expect about 2,400 lines).
9. Re-export every moved `export` from the parent. Run the scripts; confirm `World3dHost/🟦️.tsx` < 2,500 lines.

**Phase 2 - ShellHelpers module-level.**
10. Create SH-1 (labels) and SH-3 (effects-and-io) first.
11. Create SH-4, SH-5, SH-9.
12. Create SH-7 and SH-8, then SH-6.
13. Replace the ShellHelpers parent with the barrel (about 250 lines). Confirm the 326 names.

**Phase 3 - ShellHost module-level.**
14. Create `shell-model` (823-1357), `identity-directory` (1359-1882), `shell-backend` (1877-2110 and 2148-2276).
15. Confirm parent references via the barrel; keep `FrameworkOsShell` and the `FrameworkOsShellProps` type in the parent or re-export them.

**Phase 4 - ShellHost closure hooks (largest effort).**
16. Introduce `ShellHostCore` as an explicit type. Build it in the parent from 2304-2605 with no behavior change.
17. Extract hooks H1-H6 in order (small and cycle-free first), keeping the parent call order.
18. Extract H9-H15 (UI-only regions, lower coupling).
19. Extract H7 and H8 last (widest seams, 192 in-names). Cover them with the in/out bag.
20. Trim the parent to 2277-2605 core, the `FrameworkOsShell` wrapper, and the JSX 13370-13691. Expect 800-1,000 lines.

**Phase 5 - finish.**
21. Register every NEW folder in `🔣️taxonomy.json` (section 2.3); check the path budget (240 bytes, section 2.4).
22. Rewrite depth-dependent specifiers (section 1.2) and the two URL-based imports (ShellHost:791, 3514).
23. Run the repo's own gates through `bun repo run ...` (not run in this audit) and check that no test imports a symbol that was removed from a barrel.

## 7a. Verification plan for later (not run)

- Coverage: the union of all moved ranges equals the original module-level or closure range; no line lost or duplicated.
- Cycles: `segments.py` plus the `coupling.py` SCC routine over the new layout must report no SCC among the new modules.
- Exports: each original `export` name in the three monoliths is exported from the parent (grep the `export` lines of the new parent against the original list).
- Consumers: the Appendix D import lists still resolve.
- Types and behavior: the repo's own type-check and test gates. Nothing here says they pass.

## 8. Open questions and unverified points

- Taxonomy enforcement status for existing children (section 2.2) is unknown; the validator was not run.
- Chunk-level cycle results are over-approximate (name collisions possible). Some SH cuts are marked "verify". Symbol-level confirmation is needed before moving each cut.
- ShellSync, AgentBridge, Puzzle plugins, and `✏️TextEditor` have additional imports into these files not analysed in depth.
- Line numbers drift; re-scan before each move.
- ShellHost:2153 `pendingWindowUiNode` import origin was not traced; confirm it is not in a cycle before moving 2148-2276.
- AGENTS.md rule "no comments inside definitions" applies to moved code; do not add comments into moved definitions.

## Appendix (generated tables)

Auto-generated from the scripts in `🗑️generated/seam-audit/` (kept for traceability; the generated folder itself must not be deleted by this audit).

## A. ShellHost closure segments (FrameworkOsShellInner, lines 2304-13691)

Segments are `#region` blocks (`R:`) and the code between them (`G:`). `in` = distinct closure-level names this segment reads from other segments; `out` = distinct names other segments read from it. Derived by `segments.py` (ownership by 2-space-indented declaration; tokens preceded by `.` ignored). Counts are approximate (name collisions possible).

| first | last | lines | decls | in | out | segment |
|---|---|---|---|---|---|---|
| 2304 | 2605 | 302 | 72 | 11 | 126 | G:2304-2605 |
| 2606 | 2653 | 48 | 4 | 1 | 3 | R:🏁️OperationSettle |
| 2654 | 2682 | 29 | 6 | 5 | 7 | R:🔀️SurfaceSwitch |
| 2683 | 2708 | 26 | 10 | 4 | 10 | G:2683-2708 |
| 2709 | 2765 | 57 | 7 | 3 | 6 | R:🪟️TreeWindows |
| 2766 | 2784 | 19 | 3 | 5 | 3 | G:2766-2784 |
| 2785 | 2799 | 15 | 2 | 4 | 2 | R:🔖️BuiltNodeStores |
| 2800 | 4030 | 1231 | 147 | 56 | 144 | G:2800-4030 |
| 4031 | 4109 | 79 | 3 | 7 | 3 | R:🗂️LocalCatalog |
| 4110 | 4465 | 356 | 17 | 55 | 21 | G:4110-4465 |
| 4466 | 5210 | 745 | 28 | 42 | 23 | R:🔌️PluginRuntime |
| 5211 | 5391 | 181 | 40 | 27 | 33 | G:5211-5391 |
| 5392 | 5436 | 45 | 9 | 10 | 9 | R:🎯️InputLedger |
| 5437 | 5864 | 428 | 33 | 75 | 30 | G:5437-5864 |
| 5865 | 5985 | 121 | 4 | 15 | 2 | R:🧩️ContributionsPush |
| 5986 | 6224 | 239 | 3 | 46 | 3 | G:5986-6224 |
| 6225 | 6288 | 64 | 7 | 7 | 3 | R:🤝️UiRefreshCoalescing |
| 6289 | 8744 | 2456 | 41 | 192 | 24 | G:6289-8744 |
| 8745 | 9161 | 417 | 18 | 36 | 11 | R:🎥️TutorialOrchestration |
| 9162 | 9162 | 1 | 0 | 0 | 0 | G:9162-9162 |
| 9163 | 9193 | 31 | 1 | 8 | 1 | R:🔖️DirectoryLane |
| 9194 | 9322 | 129 | 2 | 23 | 0 | G:9194-9322 |
| 9323 | 9350 | 28 | 0 | 4 | 0 | R:🎨️ Ui preference projection effects |
| 9351 | 9566 | 216 | 9 | 35 | 5 | G:9351-9566 |
| 9567 | 10031 | 465 | 34 | 42 | 19 | R:🔖️SurfaceRoles |
| 10032 | 10032 | 1 | 0 | 0 | 0 | G:10032-10032 |
| 10033 | 10250 | 218 | 30 | 28 | 24 | R:🔖️ThemeMutators |
| 10251 | 10251 | 1 | 0 | 0 | 0 | G:10251-10251 |
| 10252 | 10299 | 48 | 6 | 6 | 6 | R:🚗️DriverMutators |
| 10300 | 10630 | 331 | 28 | 72 | 7 | G:10300-10630 |
| 10631 | 10778 | 148 | 9 | 14 | 1 | R:📤️DocumentTransfer |
| 10779 | 11048 | 270 | 19 | 48 | 9 | G:10779-11048 |
| 11049 | 11294 | 246 | 31 | 46 | 22 | R:🔖️CheckIn — ticket §C5 "when the user edits an artifact, the |
| 11295 | 11309 | 15 | 3 | 4 | 1 | G:11295-11309 |
| 11310 | 11317 | 8 | 1 | 12 | 1 | R:🧰️FooterUtilityLeaves — bottom-right's History leaf: the fra |
| 11318 | 11318 | 1 | 0 | 0 | 0 | G:11318-11318 |
| 11319 | 11371 | 53 | 2 | 15 | 1 | R:🔄️SyncLeaf — bottom-left's sync tab, replacing the old float |
| 11372 | 11712 | 341 | 17 | 69 | 11 | G:11372-11712 |
| 11713 | 12400 | 688 | 35 | 67 | 10 | R:🧭️DockAssembly — default four-corner arrangement (the two mi |
| 12401 | 12439 | 39 | 2 | 22 | 2 | G:12401-12439 |
| 12440 | 12492 | 53 | 2 | 15 | 1 | R:🎛️PanelTabBarHosting — `buildPanelSelectionProps` is the sin |
| 12493 | 13152 | 660 | 12 | 109 | 7 | G:12493-13152 |
| 13153 | 13203 | 51 | 0 | 7 | 0 | R:🔖️ReadinessBeacon |
| 13204 | 13204 | 1 | 0 | 0 | 0 | G:13204-13204 |
| 13205 | 13266 | 62 | 0 | 11 | 0 | R:🔖️CatalogSmokeProbe |
| 13267 | 13267 | 1 | 0 | 0 | 0 | G:13267-13267 |
| 13268 | 13369 | 102 | 2 | 12 | 1 | R:🖱️ShellContextMenu |
| 13370 | 13691 | 322 | 0 | 116 | 0 | G:13370-13691 |

## B. ShellHelpers module-level segments (lines 246-6707)

| first | last | lines | decls | in | out | segment | depends on (segment labels) |
|---|---|---|---|---|---|---|---|
| 246 | 1206 | 961 | 78 | 2 | 8 | G:246-1206 | G:1224-1320; G:3742-3897 |
| 1207 | 1223 | 17 | 0 | 0 | 0 | R:📥️ChunkedImport |  |
| 1224 | 1320 | 97 | 4 | 1 | 2 | G:1224-1320 | G:246-1206 |
| 1321 | 1803 | 483 | 33 | 3 | 0 | R:RequestMediaFrames | G:1224-1320; G:246-1206 |
| 1804 | 2124 | 321 | 21 | 3 | 2 | G:1804-2124 | G:246-1206; G:3742-3897 |
| 2125 | 2160 | 36 | 6 | 0 | 0 | R:WindowLayoutChangeClassify |  |
| 2161 | 2703 | 543 | 54 | 12 | 4 | G:2161-2703 | G:3131-3257; G:3742-3897; R:👁️✏️SurfaceRoleLabels; R:🪟️TreeWindows |
| 2704 | 2778 | 75 | 5 | 4 | 3 | R:🧰️UtilityRegistry | G:3131-3257; G:3742-3897 |
| 2779 | 2842 | 64 | 5 | 4 | 1 | G:2779-2842 | G:2161-2703; G:4588-4924 |
| 2843 | 3130 | 288 | 18 | 5 | 6 | R:🪟️TreeWindows | G:2161-2703; G:2779-2842; G:3131-3257; R:🔖️CheckInAndSyncStatus |
| 3131 | 3257 | 127 | 10 | 6 | 6 | G:3131-3257 | G:2161-2703; G:2779-2842; R:🧰️UtilityRegistry; R:🧰️WindowActionPane; R:🪟️TreeWindows |
| 3258 | 3548 | 291 | 41 | 4 | 8 | R:👁️✏️SurfaceRoleLabels | G:3131-3257; G:3742-3897; R:🎛️CommandRegistry |
| 3549 | 3549 | 1 | 0 | 0 | 0 | G:3549-3549 |  |
| 3550 | 3741 | 192 | 22 | 3 | 1 | R:🔖️CheckInAndSyncStatus | R:👁️✏️SurfaceRoleLabels |
| 3742 | 3897 | 156 | 16 | 1 | 9 | G:3742-3897 | G:3131-3257 |
| 3898 | 4060 | 163 | 6 | 2 | 0 | R:🎥️TutorialUiBridge | G:1804-2124 |
| 4061 | 4139 | 79 | 4 | 1 | 2 | G:4061-4139 | G:3131-3257 |
| 4140 | 4217 | 78 | 7 | 0 | 0 | R:🛑️ExtensionRequestCancellation |  |
| 4218 | 4282 | 65 | 2 | 0 | 0 | G:4218-4282 |  |
| 4283 | 4587 | 305 | 20 | 0 | 0 | R:🥽️Puzzle3dBrushMeshUpload |  |
| 4588 | 4924 | 337 | 19 | 2 | 3 | G:4588-4924 | G:4061-4139; R:🧰️UtilityRegistry |
| 4925 | 5601 | 677 | 35 | 16 | 2 | R:🧰️WindowActionPane | G:246-1206; G:3131-3257; G:3742-3897; G:4588-4924; R:🎛️CommandRegistry; R:🧰️UtilityRegistry |
| 5602 | 5602 | 1 | 0 | 0 | 0 | G:5602-5602 |  |
| 5603 | 6095 | 493 | 19 | 16 | 6 | R:🎛️CommandRegistry | G:246-1206; G:3131-3257; G:3742-3897; G:4061-4139; R:👁️✏️SurfaceRoleLabels; R:🧰️UtilityRegistry |
| 6096 | 6096 | 1 | 0 | 0 | 0 | G:6096-6096 |  |
| 6097 | 6235 | 139 | 10 | 5 | 0 | R:🛠️ToolRegistry | G:246-1206; G:3131-3257; G:3742-3897; G:4588-4924; R:🎛️CommandRegistry |
| 6236 | 6288 | 53 | 3 | 0 | 0 | G:6236-6288 |  |
| 6289 | 6707 | 419 | 24 | 1 | 0 | R:UiRefresh | G:246-1206 |

## C. World3dHost module-level segments (lines 178-6255)

| first | last | lines | decls | in | out | segment | depends on (segment labels) |
|---|---|---|---|---|---|---|---|
| 178 | 3831 | 3654 | 233 | 6 | 42 | R:WorldSceneParsing | G:4626-5386; R:WorldGumballTransformPrevi; R:WorldInstanceChromeStore; R:WorldInstancesLayer; R:🎯️WorldInteractionDomain |
| 3832 | 3832 | 1 | 0 | 0 | 0 | G:3832-3832 |  |
| 3833 | 4399 | 567 | 6 | 33 | 2 | R:WorldInstancesLayer | G:4626-5386; R:WorldGumballTransformPrevi; R:WorldInstanceChromeStore; R:WorldSceneParsing; R:🎯️WorldInteractionDomain |
| 4400 | 4400 | 1 | 0 | 0 | 0 | G:4400-4400 |  |
| 4401 | 4447 | 47 | 3 | 2 | 0 | R:WorldPointCloudLayer | R:WorldSceneParsing |
| 4448 | 4448 | 1 | 0 | 0 | 0 | G:4448-4448 |  |
| 4449 | 4625 | 177 | 6 | 4 | 0 | R:WorldVortexMarkers | R:WorldSceneParsing |
| 4626 | 5386 | 761 | 35 | 12 | 7 | G:4626-5386 | R:CatalogueDrop; R:WorldSceneParsing |
| 5387 | 5591 | 205 | 24 | 9 | 2 | R:CatalogueDrop | G:4626-5386; R:WorldSceneParsing |
| 5592 | 5592 | 1 | 0 | 0 | 0 | G:5592-5592 |  |
| 5593 | 5648 | 56 | 10 | 0 | 0 | R:WorldSelectionPreviewStore |  |
| 5649 | 5649 | 1 | 0 | 0 | 0 | G:5649-5649 |  |
| 5650 | 5768 | 119 | 13 | 6 | 4 | R:WorldGumballTransformPreviewStore | R:WorldSceneParsing |
| 5769 | 5769 | 1 | 0 | 0 | 0 | G:5769-5769 |  |
| 5770 | 5903 | 134 | 9 | 1 | 4 | R:WorldInstanceChromeStore | R:WorldInstancesLayer |
| 5904 | 5967 | 64 | 3 | 2 | 0 | G:5904-5967 | G:4626-5386; R:WorldWindowInstance |
| 5968 | 6063 | 96 | 10 | 1 | 1 | R:WorldWindowInstance | R:WorldSceneParsing |
| 6064 | 6064 | 1 | 0 | 0 | 0 | G:6064-6064 |  |
| 6065 | 6254 | 190 | 23 | 3 | 2 | R:🎯️WorldInteractionDomain | R:CatalogueDrop; R:WorldSceneParsing |
| 6255 | 6255 | 1 | 0 | 0 | 0 | G:6255-6255 |  |

## D. Consumers of the three element entries (framework and editor trees, excluding node_modules, dist, .🧬semio)

### ShellHost (32 files)

- `🧱️elements/🗣️Interpreter/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🖥️Board2dHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🧩️BlockListHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🐚️Shell/🟦️.tsx`: { FrameworkOsShell }
- `🧱️elements/📡️EventFeedHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/📊️Table/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🧭️TiledMapHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🕸️NodeGraph/🟦️.tsx`: { useAppKeybindingsByActionId, useMapContextMenuSpecs }
- `🧱️elements/🌳️GraphTimelineHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🔺️DiffViewHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/📐️Canvas2dHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🌐️World3dHost/🟦️.tsx`: { runtimeDiagnosticsEnabled, SetWindowIconContext, SetWindowTitleContext, useMapContextMenuSpecs }
- `🧱️elements/🌐️World3dHost/🧪️tests/🧩️component/🟦️.tsx`: { setRuntimeDiagnostics }
- `🧱️elements/✏️TextEditor/🟦️.tsx`: { useAppKeybindingsByActionId }
- `🧱️elements/✏️TextEditor/🧪️tests/📝️explicit-draft/🟦️.test.ts`: { startedTypedOperationV1, typedOperationCancellationActionV1, typedOperationCancellationOwnerIsCurrentV1 }
- `🧱️elements/🖌️Paint2dHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🧱️elements/🖋️InkCanvasHost/🟦️.tsx`: { useMapContextMenuSpecs }
- `🎯️targets/⚛️react/🟦️.tsx`: { SetWindowTitleContext, SetWindowIconContext, useAppKeybindingsByActionId, useMapContextMenuSpecs, TutorialRecorder, type FrameworkOsShellProps, FrameworkOsShe
- `🧪️tests/🏛️space-administration/🟦️.tsx`: { copyDirectoryInviteCapabilityV1, reduceShellSpaceAdministrationState, shellSpaceAdministrationOpening, shellSpaceAdministrationRequest, type ShellSpaceAdminis
- `🧪️tests/📥️inbound-request/🟦️.ts`: { EXTENSION_WORKER_LOST_FAULT, runInvokeExtensionEffect }
- `🧪️tests/🧯️router-plugin-faults/🟦️.ts`: { shellCatalogProbePlugins }
- `🧪️tests/🖱️world3d-interaction/🟦️.tsx`: { SetWindowIconContext, SetWindowTitleContext }
- `🧪️tests/⚡️quick/🟦️.ts`: { BootstrapStatusNotice, ExecutionTargetStatusNotice, reduceBootstrapUiState, resolveRequiredHostApps, type BootstrapUiStatus, }; { extensionProgramForV1, invocationPluginIdV1, spaceIndexOpeningArgsV1 }
- `🧪️tests/🔬️engine-contract/🟦️.ts`: { dispatchInvokeExtensionEffect, RUNTIME_DIAGNOSTICS_KEY, runtimeDiagnosticsEnabled, setRuntimeDiagnostics, artifactKindChoiceDraftRetirementsV1, captureSpaceAr; { ARTIFACT_CREATION_PROGRESS_CAPACITY, ARTIFACT_CREATION_PROGRESS_TEXT_V1, ArtifactCreationCatalogNotice, ArtifactCreationProgressNotice, artifactCreationProgre; { OwnedShellDialog, type OwnedShellDialogProps }; { appCommandTakesPageRun, appOwnsCommand }
- `🧪️tests/🪟️window-host-events/🟦️.ts`: { windowHostEventHandlersV1 }
- `🧪️tests/🪆️embedded-browser/⚛️oracle/🟦️.tsx`: { FrameworkOsShell }
- `🧪️tests/⏰️shell-chrome-deadline/🟦️.tsx`: { scheduleShellTransientNoticeDismissV1, SHELL_TRANSIENT_NOTICE_AUTO_DISMISS_MS }
- `🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts`: (no named import captured)
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/🧩️service-composition/🧪️tests/🟦️.ts`: (no named import captured)
- `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts`: (no named import captured)
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🪟️presentation/🟦️.tsx`: type { ServiceMountedViewV1 }; type { InferencePortUiAction, InstalledServicePresentationV1 }
- `✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/📦️packages/🟦️typescript/📜️script.ts`: (no named import captured)

### World3dHost (27 files)

- `🧱️elements/🗣️Interpreter/🟦️.tsx`: { leftoverTreeItemSelectedV1, leftoverWorldSelectionOverlayV1, subscribeLeftoverWorldSelectionV1 }; { WindowInstanceIdContext, World3dHost }
- `🧱️elements/🖥️Board2dHost/🟦️.tsx`: { WindowInstanceIdContext }; { useToolRunTraceCursorEcho }
- `🧱️elements/🧩️BlockListHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/📡️EventFeedHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/📊️Table/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/🏛️ShellHost/🟦️.tsx`: { clearPendingWorldProjection, clearPendingWorldProjections, mapContextMenuSpecs, registerPendingWorldProjection, leftoverOverlayCarryingSelectionV1, leftoverOv; { toolRunTraceCursorViewState }
- `🧱️elements/🧭️TiledMapHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/🕸️NodeGraph/🟦️.tsx`: { mapContextMenuSpecs, parseJsonArray, parseSelectionDomainsFromSession, selectionGroupsFromDomains, WindowInstanceIdContext }
- `🧱️elements/🌳️GraphTimelineHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/🔺️DiffViewHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/📐️Canvas2dHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🧱️elements/🎣️suggestion-submenu/🧪️tests/🧩️component/🟦️.ts`: { suggestionMenuItems }
- `🧱️elements/✏️TextEditor/🟦️.tsx`: { mapContextMenuSpecs }
- `🧱️elements/🖌️Paint2dHost/🟦️.tsx`: { WindowInstanceIdContext, world3dHoverActionArgs, world3dSelectionActionArgs }
- `🧱️elements/🛠️ShellHelpers/🟦️.tsx`: { registerPendingWorldProjection }
- `🧱️elements/🖋️InkCanvasHost/🟦️.tsx`: { WindowInstanceIdContext }
- `🔮️oracles/🎨️world3d-scene-shading/📜️script.ts`: (no named import captured)
- `🎯️targets/⚛️react/🟦️.tsx`: { world3dHoverActionArgs, world3dSelectionActionArgs, interactionTargetsForInstances, WORLD3D_DEFAULT_INTERACTION_GRANULARITY, WORLD3D_DEFAULT_MARKER_GRANULARIT
- `🧪️tests/🚚️world3d-instance-delta/🟦️.ts`: { advanceWorldInstanceResidency, parseWorldInstanceDelta, type WorldInstanceRecord, type WorldInstanceResidencyV1 }
- `🧪️tests/🖱️world3d-interaction/🟦️.tsx`: { clearPendingWorldProjection, clearPendingWorldProjections, componentPickMergeMode, leftoverBrushRetainGuestHoverV1, registerPendingWorldProjection, WindowInst
- `🧪️tests/🎯️world3d-pick-bounds/🟦️.ts`: { hoveredInteractionTargetV1, mergeWorldSelectionWithLeftoverV1, resolveClickInstanceIdFromProjected, world3dInstanceLocalCorners }
- `🧪️tests/🧊️world3d-mesh-residency/🟦️.ts`: { advanceWorldMeshResidency, buildMeshVisuals, disposeMeshVisuals, splitJsonArrayElements, type WorldMeshResidencyV1 }
- `🧪️tests/🔬️engine-contract/🟦️.ts`: { worldGumballStep, WORLD_GUMBALL_IDLE, type WorldGumballEvent, type WorldGumballTargets }; { worldHoverPaintIdV1 }; { worldInstanceHighlighted }; { WorldOrbitProjectionSwitchPane, world3dProjectionPaneElementId, resolveClickInstanceIdFromProjected, world3dInstancePickUsesInteractionDomain, world3dMarqueeP; { meshDataFromKind }
- `🧪️tests/🎨️world3d-inline-surface/🟦️.ts`: {buildMeshVisuals,disposeMeshVisuals,cloneWorldGlbSurface,disposeWorldGlbSurface,advanceWorldMeshResidency,WorldSurfaceStatusPane}
- `🧪️tests/📐️projection-render-parity/🟦️.ts`: { parseCameraState, world3dFitProjectionContent }
- `🧪️tests/🎥️world3d-camera-framing/🟦️.ts`: { world3dAutoFitKey, world3dAutoFitOwed, world3dBoundsRadius, world3dFrameCameraFromBounds }
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧪️tests/🧪️interactivity-tool-run-policy/🟦️.ts`: (no named import captured)

### ShellHelpers (31 files)

- `🧱️elements/🎛️UtilityTree/🟦️.tsx`: { SelectionUtilityOptions }
- `🧱️elements/🎛️UtilityTree/🧪️tests/🎛️picker-explicit-press/🟦️.tsx`: { SelectionUtilityOptions }
- `🧱️elements/🗣️Interpreter/🟦️.tsx`: { shellLabel }
- `🧱️elements/🖥️Board2dHost/🟦️.tsx`: { createCoalescingActionDispatcher }
- `🧱️elements/🐚️Shell/🟦️.tsx`: initialization-order bug documented in ui-react's // 🧱️elements/🫀️core/Ports/🟦️.tsx header comment (see 📋️w0-status.md's "W3 follow-up" section). import { shell; { DEFAULT_PANEL_WIDTH_PX, mergeRecordPreservingIdentity }; { PRESENCE_CLIENT_STORAGE_KEY, EMPTY_APP_LABELS_OVERLAY, EMPTY_APP_CATALOGUE }
- `🧱️elements/🏛️ShellHost/🟦️.tsx`: { DEFAULT_PANEL_WIDTH_PX, EMPTY_APP_CATALOGUE, EMPTY_APP_LABELS_OVERLAY, FRAMEWORK_CATEGORY_COMMAND_ID, FRAMEWORK_CATEGORY_DISPLAY_ID, FRAMEWORK_CATEGORY_TOOL_I; { HistoryReprojectionStatus, TimeTravelBand, timeTravelIndicatorTextV1, timeTravelPeerPresenceV1, timeTravelRowActionRefusalsV1, TimeTravelWindowIndicator, useT
- `🧱️elements/🏛️ShellHost/📎️local-folders/🧪️tests/🧩️component/🟦️.tsx`: { shellLabel, syncAttachDocumentIdV1, syncShellLabelLocale }
- `🧱️elements/🖼️IconRenderHost/🧪️tests/📥️download/🟦️.ts`: { downloadDataUrl }
- `🧱️elements/📐️Canvas2dHost/🧪️tests/🖱️input-contract/🟦️.tsx`: {keyboardEventMatchesChord}
- `🧱️elements/🌐️World3dHost/🟦️.tsx`: { contextMenuGroupLabel, createCoalescingActionDispatcher, declareSurfaceCancelAction, world3dMarqueeOverlayShape, type Puzzle3dBrushMeshPage, puzzle3dAnnouncea
- `🧱️elements/📌️ChromePanels/🟦️.tsx`: { defaultAppsSettingsTabLabel, defaultAppsSettingsTabText, driverDisplayLabel, noneOptionText, shellLabel, shellLabelIfDefined, shellLabelLocale, shellTabIcon, 
- `🧱️elements/✏️TextEditor/🟦️.tsx`: { shellLabel }
- `🧱️elements/🔎️ShellSearch/🧪️tests/🧩️component/🟦️.tsx`: { buildOsCommands }
- `🎯️targets/⚛️react/🟦️.tsx`: { NOTE_WORLD_NAVIGATION_ACTION_ID, buildNoteShellCommandAction, isShellOwnedCommandId, encodeEffectActionInvocation, encodeEffectCommandInvocation, TUTORIAL_REC
- `🧪️tests/👥️scoped-presence/🟦️.tsx`: { PRESENCE_EPHEMERAL_SNAPSHOT_DEADLINE_MS, PRESENCE_HEARTBEAT_INTERVAL_MS, PRESENCE_LIVE_VIEW_BEAT_INTERVAL_MS, presenceEphemeralSnapshotWithinBoundV1, presence
- `🧪️tests/🎥️tutorial-bridge/🟦️.ts`: { applyTutorialUiChangeToShell, applyTutorialUiSnapshotToShell, captureTutorialUiSnapshot, type TutorialUiBridgeContext }
- `🧪️tests/📥️inbound-request/🟦️.ts`: { abortExtensionRequestsForActor, declareSurfaceCancelAction, inFlightExtensionRequestCount, isDeclaredSurfaceCancelAction }
- `🧪️tests/🪟️app-mode-layouts/🟦️.ts`: is the evaluation order, not a dependency. import "../../🧱️elements/🐚️Shell/🟦️.tsx"; import { resolveFrameworkLayoutSeed }
- `🧪️tests/🌐️settings-locale-panel-refresh/🟦️.tsx`: { shellLabel }
- `🧪️tests/🚪️ingress-generation-gate/🟦️.ts`: { createUiRefreshCoalescerV1, uiDirtyScopeCoveredByV1, uiRefreshAlreadyAnsweredV1 }
- `🧪️tests/♿️action-argument-gate/🟦️.ts`: { buildActionCategoryTree, buildCommandCategoryTree, syncShellLabelLocale, type ResolvedActionDefinition, type ResolvedCommand }
- `🧪️tests/⚡️quick/🟦️.ts`: { pluginAvailabilityRouteV1, pluginInstallBandTextV1, pluginInstallProgressTotalV1 }
- `🧪️tests/🔬️engine-contract/🟦️.ts`: { DOWNLOAD_MEDIA_EXPORT_REVOKE_MS, EMPTY_APP_LABELS_OVERLAY, SET_ACTIVE_EXAMPLE_ACTION_ID, appSwitchesExamples, appOffersRegisteredExamples, frameworkOwnsExampl; { windowActionPaneNode, applyTutorialUiChangeToShell, applyTutorialUiSnapshotToShell, browserActorDispatchUiScopeV1, browserActorWindowConfigDispatchUiScopeV1, 
- `🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts`: { uiIntentToActionDescriptor }
- `🧪️tests/🎚️window-measure-controls/🟦️.tsx`: { WindowMeasureNumber, WindowMeasureSelect, WindowMeasureToggle }; { renderWindowMeasuresTree }
- `🧪️tests/🪟️spawned-program-session/🟦️.tsx`: { applyUiRefreshResponseToCache, buildUiRefreshRequest, EMPTY_APP_LABELS_OVERLAY, frameworkLayoutDeclaredInstances, historyPatchShouldApplyV1, resolveFrameworkL
- `🧪️tests/🚗️driver-editor/🟦️.ts`: { driverDisplayLabel }
- `🧪️tests/🎛️command-panel/🟦️.tsx`: { buildCommandCategoryTree, commandAddressKey, type ResolvedCommand }
- `🧪️tests/⌨️os-command-shortcuts/🟦️.ts`: { buildOsCommands, keyboardEventMatchesChord, controlOwnsKeyboardEvent }
- `🧪️tests/🪟️maximize-resync/🟦️.tsx`: { resolveFrameworkLayoutSeed, retitleWindowLayoutNode }
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🥽️brush-mesh-upload/🟦️.ts`: { PUZZLE3D_MESH_COMMAND_RAW_BYTES, PUZZLE3D_MESH_PAGE_VALUES, PUZZLE3D_MESH_REUPLOAD_CLAIMS, PUZZLE3D_MESH_UPLOAD_MAX_PAGES, PUZZLE3D_MESH_UPLOAD_SLOTS, puzzle3

