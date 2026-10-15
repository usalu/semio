# Result: ShellHost module-level split and `FrameworkOsShellInner` hook decomposition

Executed 2026-10-15 by the Sonnet executor. `E` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`, `SH` = `E/🏛️ShellHost`.
Seam map used: `SEAM-MAP-SHELLHOST-TSX.md` (ShellHost section and cycle analysis). Approach and scripts reused from `RESULT-SHELLHELPERS-WORLD3DHOST-SPLIT.md`. All tooling and logs live in `🪓️monolith-split/🗑️generated/shellhost-split/` and `🪓️monolith-split/🗑️generated/shellhost-*`.

## 1. Outcome at a glance

| File | Before | After |
|---|---|---|
| `SH/🟦️.tsx` | 13,707 lines (about 2,300 module-level lines plus a 11,416-line `FrameworkOsShellInner` closure) | 591 lines: header, imports, 9 explicit `export ... from` lists, `FrameworkOsShell` (unchanged), `ShellCore` type, a thin `FrameworkOsShellInner` (21 hook calls, one destructure, the original 320-line JSX `return`) |
| module-level children | 0 | 9 modules, 69-281 lines each |
| closure hook children | 0 | 21 modules, 395-888 lines each (largest: `🖱️shell-actions` 888) |

Both targets are met: parent at most about 1,000 lines (591), every child at most about 2,500 lines (888).

Public API is unchanged. Checked with the TypeScript checker (`getExportsOfModule`, name plus value/type kind), original text vs the final parent: 63 exports, identical, 0 unresolved (`🗑️generated/shellhost-split/exports-orig.txt` vs `exports-final.txt`).

## 2. Phase 1: module-level split (9 children)

Cut with the sibling's hash-guarded tool (`split.mjs`/`write.mjs`, config `cfg-shellhost.mjs`, region-marker stripping restricted to column 0 so the closure's indented `//#region` markers survive). 123 top-level units: 121 appear exactly once across parent plus children (`coverage-shellhost.mjs`), 1 is the `export type { ... }` line (kept in the parent, rewritten as an `export type { ... } from "@semio-tech/framework-os"`), 1 is `FrameworkOsShellInner`, whose text after phase 1 is byte-identical to the original (exact string compare). Each child opens with a JSDoc whose first token is the folder emoji and links back to the parent. No comments were added inside definitions.

| Child (`SH/<dir>/🟦️.ts(x)`) | Lines | Holds (original line range) |
|---|---|---|
| `🎚️shell-model` (tsx) | 170 | `SetWindowTitle/IconContext`, `ShellCatalogProbe`, `ServiceMountedViewV1`, notice constants, `mutationCodeLabelKey`, `WindowFaultStatus`, `AppKeybindingsContext`, `useAppKeybindingsByActionId`, `useMapContextMenuSpecs`, `seededActiveWindowId` (865-1085) |
| `🏁️operation-settle` | 69 | settle constants, `shellUncommittedEditCountV1`, `StartedTypedOperationV1`, typed-operation cancellation (973-1036) |
| `📨️window-invocation` | 101 | invocation encoding, `RELEASED_POINTERS_BY_WINDOW` (the only module-level mutable state here), `windowHostEventHandlersV1`, `appOwnsCommand`, `appCommandTakesPageRun` (1087-1167) |
| `🎬️tutorial-recorder` (tsx) | 222 | tutorial overlay hosts, `diffTutorialUiSnapshot`, `TutorialRecorder` (1169-1358) |
| `🔐️identity-binding` | 185 | `scopeRuntimeKey`, `hubSessionStorageV1`, `FrameworkOsShellProps`, announced streams, `readViteSEnv` (`import.meta.env`), hub origin, identity envelope, `shellActorId`, `canonicalSurfaceId`, `reloadRetainsActiveApp`, `resolveShellScopeStorage` |
| `🧾️space-artifact-creation` | 281 | space-index constants and finders, `SURFACE_ROLE_ICON_IDS`, directory command result slots, catalog authority/projection/request helpers, `directoryCommandFromAction` |
| `🏠️space-administration` | 162 | space administration state, reducer, request builder, invite clipboard copy |
| `🔌️extension-runtime` | 234 | `commandStallLineV1`, runtime diagnostics (module `let` pair `runtimeDiagnosticsOverride/Resolved` lives only here), invoke-extension dispatch |
| `🧱️built-node-stores` | 185 | reconciler helpers, `PENDING_WINDOW_UI_NODE` (module-evaluation call kept), `BuiltNodeStoreCacheV1`, refresh budgets |

Module-level mutable state each has exactly one owner (see the table). Top-level side effects: the only evaluation-time calls (`new Set(...)` in `tutorial-recorder`, `pendingWindowUiNode()` in `built-node-stores`) are pure and moved together with their dependencies.

Deviation from the seam map: 9 smaller cohesive children instead of 3 larger ones (`shell-model`, `identity-directory`, `shell-backend`), so that unit-level dependencies stay acyclic without moving code against its concern.

## 3. Phase 2: closure to hooks (21 children)

### 3.1 Design

`FrameworkOsShellInner(props)` now does exactly this:

```ts
const core = {} as ShellCore;
Object.assign(core, useShellCoreState(props, core));
Object.assign(core, useIdentityAndDocumentPorts(core));
... 19 more `Object.assign(core, useX(core))` in the original statement order ...
const { <the 115 names the JSX reads> } = core;
return ( ...original JSX, verbatim... );
```

- **One typed core object.** Each hook module exports `ShellCoreAfter<X> = ShellCoreAfter<Previous> & ReturnType<typeof useX>`; `useX(core: ShellCoreAfter<Previous>)` takes the type of everything produced before it, so every binding keeps its inferred type (no `any`, no hand-written types for ordinary bindings). `ShellCore` in the parent is the last link of that chain. The chain is a linear, type-only import path: no import cycle.
- **Verbatim ranges.** The 898 closure statements before the `return` are partitioned into 21 contiguous ranges (cut at region boundaries; a `//#endregion` line stays with the hook that holds its `//#region`). Each hook body is the original text of its range. The only differences inside a body: (a) a leading `const { ... } = core;` that binds the earlier bindings the range reads, (b) the closing `return { ... };` bag of the bindings later ranges or the render read, (c) the 10 closure-local `type` aliases hoisted to the declaring module (exported when another hook uses them), (d) the forward-slot rewrites of 3.2, (e) the one `new URL("../../../../🏪️store/👷️worker/🟦️.ts", import.meta.url)` and the `?worker&url` import re-relativised for the deeper folder (both targets verified to exist).
- **Hook order.** Hooks are called unconditionally, in the original textual order; inside each hook the statements keep their original order. Dependency arrays are unchanged and still list the same names, now bound by the destructure, so `useCallback`/`useMemo` identity semantics are unchanged (each render builds a fresh `core`, exactly as it built fresh closure variables).
- **Props.** `useShellCoreState` receives the props and keeps the original parameter destructuring with its defaults (`documentServices = []`, `suppressAutoIntroduction = false`), so defaults are evaluated per render as before; the props type became `FrameworkOsShellInnerProps` (exported by that module).
- **No `let`, no hoisted function declarations, no writes to closure bindings in the closure** (analysis: 794 `const` declarations plus 10 `type`), so copying values through the bag is exact.

### 3.2 Forward references (the one hard problem)

37 deferred forward references (a callback built earlier reads a `const` declared later; legal in the single closure because the callback runs after render) cross the new hook boundaries for 18 names. They cannot be bag values (the bag is built after the hook returns). Solution, semantics-preserving: the earlier hook reads `core.<name>` instead of `<name>` at those exact identifier positions (shorthand properties become `name: core.name`), and the declaring hook assigns `core.<name> = <name>;` on the line right after the declaration statement, so the value is visible as soon as the original binding would have been initialised. The 18 names are typed explicitly by `ShellForwardSlots` (in `🧩️shell-core-state`, with the effect-owner and `onAction` signatures spelled out once): `loadedPluginsRef`, `shellStateRef`, `applyHostEffectsRef`, `spawnedIdsRef`, `documentCheckInOwnersRef`, `setDocumentCheckInUi`, `documentTransferRef`, `spawnedAppsRef`, `activeUtilityByWindowIdRef`, `applyHostEffects`, `onActionRef`, `tutorialRunRef`, `tutorialTransitionEpochRef`, `tutorialClockRef`, `isViewerReadOnlyFault`, `isMutationRejectedFault`, `showTransientNotice`, `showMutationRejectedNotice`. There are 0 eager forward references (a TDZ error in the original). Every declaring statement's assignment is checked by the compiler against the explicit slot type.

The slots object is the parent's `core` itself (`useShellCoreState(props, core)` takes it as a second parameter and reads slots through it).

### 3.3 Hook decomposition table

Original ranges are lines of the original 13,707-line file (`🗑️generated/shellhost-split/orig-ShellHost.tsx`, statement indices 0-897). "Reads" = bindings destructured from `core` (earlier hooks, including forward-slot names declared earlier); "Fwd reads" = forward-slot names read through `core.<name>`; "Declares slots" = forward names this hook assigns; "Outputs" = size of the returned bag.

| # | Folder (`SH/<dir>`) | Hook | Original lines | Orig. lines | New lines | Reads | Fwd reads | Declares slots | Outputs |
|---|---|---|---|---|---|---|---|---|---|
| 1 | `🧩️shell-core-state` | `useShellCoreState` | 2318-2812 | 495 | 724 | props | 1 | - | 165 |
| 2 | `🔒️identity-document-ports` | `useIdentityAndDocumentPorts` | 2814-3434 | 621 | 794 | 17 | 4 | - | 127 |
| 3 | `🧵️backbone-worker` | `useBackboneWorker` | 3435-4043 | 609 | 706 | 57 | 4 | `loadedPluginsRef` | 19 |
| 4 | `🛂️directory-route` | `useDirectoryAndRoute` | 4046-4478 | 433 | 511 | 58 | 0 | - | 24 |
| 5 | `🚀️plugin-lifecycle` | `usePluginLifecycle` | 4484-5222 | 739 | 829 | 35 | 2 | - | 22 |
| 6 | `🎮️focus-and-input` | `useFocusAndInput` | 5229-5877 | 649 | 781 | 68 | 0 | `shellStateRef`, `spawnedIdsRef`, `spawnedAppsRef`, `activeUtilityByWindowIdRef` | 60 |
| 7 | `🔁️contributions-refresh` | `useContributionsAndRefresh` | 5883-6326 | 444 | 530 | 38 | 1 | `applyHostEffectsRef` | 5 |
| 8 | `🛫️spawned-session` | `useSpawnedSession` | 6328-6876 | 549 | 653 | 63 | 0 | - | 7 |
| 9 | `⚡️host-effects` | `useApplyHostEffects` | 6878-7502 | 625 | 719 | 71 | 2 | `applyHostEffects` | 0 |
| 10 | `🗄️shell-uri-documents` | `useShellUriAndDocuments` | 7504-8006 | 503 | 582 | 72 | 3 | - | 11 |
| 11 | `🖱️shell-actions` | `useShellActions` | 8013-8758 | 746 | 887 | 78 | 5 | `onActionRef` | 5 |
| 12 | `🎥️tutorial-orchestration` | `useTutorialOrchestration` | 8763-9175 | 413 | 476 | 33 | 0 | `tutorialRunRef`, `tutorialTransitionEpochRef`, `tutorialClockRef` | 8 |
| 13 | `📐️layout-hotkeys` | `useDirectoryLaneAndLayout` | 9187-9580 | 394 | 479 | 50 | 0 | - | 5 |
| 14 | `📬️artifact-opening` | `useArtifactOpening` | 9588-10045 | 458 | 556 | 35 | 0 | `isViewerReadOnlyFault`, `isMutationRejectedFault`, `showTransientNotice`, `showMutationRejectedNotice` | 13 |
| 15 | `🎨️theme-driver-settings` | `useThemeDriverSettings` | 10049-10644 | 596 | 726 | 66 | 0 | - | 12 |
| 16 | `📤️document-transfer-tabs` | `useDocumentTransferAndTabs` | 10647-11062 | 416 | 527 | 53 | 0 | `documentTransferRef` | 9 |
| 17 | `✅️check-in-sync` | `useCheckInAndSync` | 11070-11385 | 316 | 394 | 54 | 0 | `documentCheckInOwnersRef`, `setDocumentCheckInUi` | 19 |
| 18 | `🔎️command-bar` | `useCommandBar` | 11388-11726 | 339 | 441 | 65 | 0 | - | 10 |
| 19 | `🏗️dock-assembly` | `useDockAssembly` | 11729-12414 | 686 | 771 | 63 | 0 | - | 9 |
| 20 | `📋️panel-hosting` | `usePanelHosting` | 12417-12958 | 542 | 657 | 91 | 0 | - | 9 |
| 21 | `🖼️canvas-footer` | `useCanvasAndFooter` | 12960-13383 | 424 | 509 | 67 | 0 | - | 4 |

Concern per hook: 1 base state (scope, reducer, history, panel/dialog/tutorial state, operation settle, surface switch, tree windows, built-node stores); 2 identity, hub session, document ports, archives, browser-actor ui, space directory; 3 `ensureBackboneWorker`; 4 directory/space-administration/inference intents, shell uri and route, plugin source; 5 primary session, plugin install/reload/uninstall, extension ledger; 6 spawned apps, focused program, introductions, input ledger, utility/tool registers, context-menu request; 7 contributions push, ui refresh pass and lane; 8 spawned ui refresh, space panel, plugin app switch; 9 `applyHostEffects`; 10 `applyShellUri`, document open/close, local folders, sync targets, `spawnProgram`; 11 `onAction`; 12 tutorial orchestration; 13 directory lane, hotkeys, layouts, mode/role switching, template drop, display host; 14 app router, activation, default app, artifact opening, conflicts, transient notices, remote merge; 15 theme/driver mutators, preference lane, settings/marketplace/task-manager hosts; 16 document transfer, approvals, keydown, workbench and details tabs; 17 presence, hub link, check-in, history and sync tabs; 18 mode/example/role switchers, commands, command and tool tabs; 19 default dock, persistence, drops, introduction overrides, panel reveal effects; 20 mobile panel, panel tab bar hosting, navbar/search items, mode windows; 21 canvas, footer, panel props, readiness beacon, catalog smoke probe, shell context menu.

Deviation from the seam map: 21 hooks instead of H1-H15 (cuts re-derived on the current 13,707-line file with a compiler-based dependency analysis; the map's ranges were line estimates). Everything below 900 lines.

### 3.4 Module DAG

Parent -> every child; hook k -> hook k-1 (type only, the chain) plus earlier hooks only for hoisted `type` aliases; no child imports the parent. Import-graph SCC check over parent plus 30 children (`graph-check-shellhost.mjs`): 31 files, 0 SCCs of size greater than 1, 0 children importing the parent. Longest new path 166 UTF-8 bytes (limit 240).

## 4. Taxonomy registration

Rule (verified by the sibling, baseline: existing unlisted children are pre-existing failures): a child folder name must be in `semanticDirectoryMemberKinds."members-of-members-of-elements".memberNames` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. I appended the 30 new names after `🪟️window-view-store` (one textual hunk, anchor asserted unique, nothing else reformatted; every name starts with an RGI emoji followed by U+FE0F, no emoji is shared with an existing `SH` sibling or with another new child, none equals `📁️ 📂️ 📄️` or the file emoji `🟦️`, all emoji are already folder prefixes elsewhere in the repo):

`🎚️shell-model 🏁️operation-settle 📨️window-invocation 🎬️tutorial-recorder 🔐️identity-binding 🧾️space-artifact-creation 🏠️space-administration 🔌️extension-runtime 🧱️built-node-stores 🧩️shell-core-state 🔒️identity-document-ports 🧵️backbone-worker 🛂️directory-route 🚀️plugin-lifecycle 🎮️focus-and-input 🔁️contributions-refresh 🛫️spawned-session ⚡️host-effects 🗄️shell-uri-documents 🖱️shell-actions 🎥️tutorial-orchestration 📐️layout-hotkeys 📬️artifact-opening 🎨️theme-driver-settings 📤️document-transfer-tabs ✅️check-in-sync 🔎️command-bar 🏗️dock-assembly 📋️panel-hosting 🖼️canvas-footer`.

Validator (`bun ./📜️script.ts verify taxonomy report --scope SH`, log `🗑️generated/shellhost-taxonomy-after.log`): `clean=false errors=58`. None of the 58 concerns a new folder or a new file. They are: 44 (40 `directory-kind-unresolved`, 3 `path-too-long`, 1 `normalization-move-required`) on pre-existing, unregistered `SH` folders (`⌨️window-scope`, `🌱️artifact-creation/🚪️ready-opening`, `🎟️invite-capability`, `🎯️input-ledger`, `👥️presence-scope`, `📇️directory-bootstrap`, `📇️space-directory`, `📣️replay-refusal`, `🔀️surface-switch`, `🗨️dialog-origin/...`, `🧭️opening`, `🧭️route-ledger`, `🩺️fault`, `🪟️mounted-window-refresh`, `🪟️spawned-program`, `🪪️host-bootstrap`, their fixture mirrors, and 3 over-long fixture paths), 1 `generator-preview-invalid` (`✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`), and 13 `reference-edit-required`: 12 in `🧪️tests/🔌️plugin-runtime` (peer's concurrent PluginRuntime split) plus 1 in `🧪️tests/🔬️engine-contract` at `typescript-path:123:41` (a peer edit; the sibling saw the same finding at 169:37). I ran no ShellHost-scope baseline before registering, so there is no "before" number; the evidence is that no finding names one of the 30 new folders or their files.

## 5. Verification, baseline vs after (all foreground except where noted, capped)

### 5.1 Typecheck

`node_modules/.bin/tsc --noEmit -p 🗑️generated/shellhost-tsconfig.json` (scoped config extending `💻️os/tsconfig.json`, `files` = the ShellHost parent; the program pulls in the whole renderer, about 22 s).
- Baseline before any edit (`shellhost-tsc-baseline.log`): 32 errors, 0 in ShellHost (27 `UiDocumentStore/🧪️tests/🧪️typedwire`, 3 `PluginRuntime`, 2 `WasmSessionLoader`). The two errors a peer introduced at 9191/9192 in the previous run were already gone when I started.
- After phase 1 (`shellhost-tsc-after-p1.log`): identical 32 (sorted diff empty).
- After phase 2 (`shellhost-tsc-final.log`): 34 errors, 0 in ShellHost. The 2 extra are the PluginRuntime errors: a peer split `🔌️PluginRuntime` into new children (`🔁️turn-settle`, `🛰️runtime-singletons`, `🧩️module-loader`, `🧵️turn-dispatch`, `🪢️handle-scope`) while I worked, so its 3 baseline errors became 5 in those children; none involves my files.
- Proof that the check is live (not a silent zero): the first phase-2 attempt produced 8 errors (TS2304 `core` undefined, TS7006 implicit any) in `🧩️shell-core-state`, fixed by giving `useShellCoreState` the slots parameter; and a `--noUnusedLocals` run (`shellhost-tsc-unused-after.log`) reports 9 `TS6133/6198` hits in hook bodies, each a declaration that was already unused in the original closure (`validationError`, `activePanelTabId`, `req`, `aborted`, `owners`, `restored`, `hotSwapEvent`, `conflicts`, one all-unused destructure); it reports no unused import and no unused `core` destructure name, so every generated import and destructure is exact.

### 5.2 Structural equivalence checks (`hooks-verify.mjs`, `coverage-shellhost.mjs`)

- Original closure (statements 0-897, all comments included) vs the concatenation of the 21 hook bodies after undoing the generated scaffolding (destructure, bag, `core.x` rewrites, slot assignments, hoisted types): identical after whitespace normalisation (687,720 vs 687,723 characters; the 3 extra characters are the extra `../` in the one `new URL`). Hoisted local types: 10 of 10 found.
- Hook call order in the parent equals the hook order: true, 21 calls. The original JSX `return` slice is present verbatim in the new parent.
- Module-level units: 121 of 121 exactly once (section 2).

### 5.3 Unit tests

Full renderer suite (same config and `SEMIO_VITEST_POLICY` as the sibling, `shellhost-vitest.sh`): 184 files. Baseline = the ORIGINAL ShellHost text (a verbatim copy with absolute specifiers, aliased over the parent by `shellhost-vitest-orig.config.ts`; alias activation confirmed by a `[DEBUG]` line), after = the split tree.

| Run | Files failed / passed | Tests failed / passed | Log |
|---|---|---|---|
| Baseline, original ShellHost | 31 / 153 | 40 / 2,869 (of 2,909) | `shellhost-vitest-full-baseline-originals.log` |
| After split | 33 / 151 | 45 / 2,976 (of 3,021) | `shellhost-vitest-full-after.log` |

The test count differs because a peer added tests between the two runs. Failure-set diff (`shellhost-fails-baseline.txt` vs `shellhost-fails-after.txt`):
- Gone (flaky, not related): `♿️editable-controls` (file-level), 2 `Paint2dHost/✍️editing/...selection-focus` tests.
- 7 new failures, all source-text readers of `SH/🟦️.tsx` (section 6). Nothing else is new: every other failing test is in the baseline failure list too (the pre-existing set the sibling documented: `wgpu-*`, `package-integration`, `browser-frame-transport`, `engine-contract` x4, `scoped-presence`, `settings-general-layout`, `window-measure-controls`, `ShellHelpers` command-rejection/fault-notices, and others).
- Proof the 7 are needle failures only: with a vitest alias that serves the concatenation of the new parent and all new children as the `?raw` text of `SH/🟦️.tsx` (`shellhost-vitest-concat.config.ts`, alias confirmed by a `[DEBUG]` line), the 4 reader files give 48 passed, 1 failed (log `shellhost-vitest-concat-readers.log`); all 7 reader tests pass. The 1 failure is `embedded-mount > renders two idle real ShellHosts`, see below.
- ShellHost's own test (`📎️local-folders/🧪️tests/🧩️component`, 11 tests) passes before and after. There are no ShellHost stories (`*.story.tsx` under `SH`: none).
- Real mount evidence: `embedded-mount > renders two idle real ShellHosts and retires only the selected root` mounts the real `FrameworkOsShell` (all 21 hooks execute, readiness beacon `data-shell-ready` is set on both roots) and passed in the full after run. It is timing-flaky (default `vi.waitFor` timeout 1 s): in isolation, 4 runs per variant, the original failed 1 of 4 and the split 0 of 4 (`shellhost-flaky.log`); in two non-isolated runs the split failed 1 of 2 and the concat run failed once. I treat it as a pre-existing flake, not as a regression, but I could not prove the absence of a small timing change.

## 6. Source-text readers of `SH/🟦️.tsx` (for the peer re-pointing readers; not fixed here)

BREAK (read the parent via `?raw`, `E/../🧪️tests/<dir>/🟦️.ts`; these are the 7 new failures):

| Test file | Needle(s) | Now lives in |
|---|---|---|
| `🎭️actor-window-actions` (2 tests) | `<InterpretedUiNode ... onAction={onActionStable}` JSX windows; `const onAction = useCallback(` .. `const onActionStable = useCallback(`; `{ stores: new Map(currentBrowserActorUi.panels), onIntent: browserActorPanelIntent, onAction: onActionStable }` | `📋️panel-hosting` (JSX), `🖱️shell-actions`, `📤️document-transfer-tabs` |
| `🧭️route-ledger` (1) | `if (route === null \|\| !shellRouteLedgerRef.current.shouldApply(route)) return;`, `shellRouteLedgerRef.current.settle(...)`, two `shellRouteLedgerRef.current.invalidate()` sites | `🗄️shell-uri-documents` (apply guard, 2 invalidates), `🧩️shell-core-state` (settle), `🚀️plugin-lifecycle` (invalidate), `🛂️directory-route` |
| `🩺️window-fault` (3) | `${WINDOW_FAULT_ATTRIBUTE}={fault.class}`/`role="status"`; `liveInstanceWindowFaultV1(error, pluginSupervisorByIdRef.current[pluginId], ...)`; `windowUiByWindowId[kind.id] ?? PENDING_WINDOW_UI_NODE`; `current[instance.id] ?? pendingWindowUiNode()`; `scopeContributionsJson(loadedForScope, ...)`, `encodeAppCommandInvocation(... "setContributions" ...)`, `page: 0, pageCount: 1` | `🎚️shell-model` (fault attribute), `🧵️backbone-worker`, `📋️panel-hosting`, `🔁️contributions-refresh` |
| `🪆️embedded-mount` (1) | parses the parent and takes the argument of the `shouldStartIntroduction(...)` call | `🎮️focus-and-input` |

OK because they walk the module tree (`readModuleTreeSource`): `E/../📦️packages/🟦️typescript/📜️script.ts:159`, `✏️s/.../🗺️gismap/📦️packages/🟦️typescript/📜️script.ts:471,526`, `🌎️hub/📦️packages/🦀️rust/📜️script.ts:14268,16203`. The loader of `engine-contract` (`SH` read at `🔬️engine-contract/🟦️.ts:11334`) did not newly fail. Comment-only mentions (no read): `🧑‍💻dev/🧪️tests/🔬️catalog-smoke`, `✏️s/🧪️tests/🎭️storybook-end-to-end`, `🎡️play/🧪️tests/🧪️playpanecoverage`, `🌎️hub/🧪️tests/🤝️dev-collaboration`, `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance`. The ReadinessBeacon, window-fault and `onAction` needles that gates reference by name moved to `🖼️canvas-footer`, `🎚️shell-model` and `🖱️shell-actions`.

## 7. Residuals and notes

- `ShellCore` is created with `{} as ShellCore` and filled hook by hook; this is the one type assertion, and it is what makes the forward-slot mechanism and the single mutable-per-render core possible. Every other binding is typed by inference from the hook that declares it.
- `useShellCoreState(props, core)` takes the core as an extra parameter only to reach the slots.
- Unchanged by design: the closure's `useRef`/dependency patterns, the `*Ref` indirections, and every comment, including comments that mention "below"/"above" across what are now module boundaries.
- Hook order is the original statement order; no range was left in the component (all 898 statements lifted). Statements in expression position that mutate refs (`xRef.current = ...`) stay in the hook that holds the ref's declaration range.
- Pre-existing import cycles are unchanged in shape: `ShellHost -> Shell -> ShellHost` (hook `🧩️shell-core-state` and Phase-1 children import `🐚️Shell`; `Shell` imports `FrameworkOsShell` from the parent), `ShellHost <-> ShellHelpers`, `ShellHost <-> World3dHost/Interpreter`. Module-evaluation order of the cyclic imports now differs slightly (the importing code sits in children); the full suite and the real-mount test are the only runtime evidence. Phase 0 import redirects of the seam map are not done (out of scope).
- Not run: `bun repo run ...` (broken in this session per the sibling's report), a Vite dev server or browser boot of the shell, the repo-wide `verify taxonomy`, Rust gates, any ESLint rule set (no `react-hooks` lint was run; hooks are plain `use*` functions called unconditionally). Worker URL and `?worker&url` specifiers were verified to resolve by file existence only, not by a bundle build.
- Peers touched `SH` while I worked: none changed `🟦️.tsx` between my analysis and each hash-guarded write (both writes aborted if the file hash had changed). The embedded mount test, `PluginRuntime`, `engine-contract` and taxonomy findings above are attributed to peers.
- Debug logging: none added. `🗑️generated` folders were left in place.
- Tooling kept for traceability in `🗑️generated/shellhost-split/`: `hooks-lib.mjs` (compiler-based statement/reference analysis), `hooks-plan.mjs`, `hooks-config.mjs` (the 21 ranges), `hooks-slots.mjs` (forward-slot types), `hooks-emit.mjs` (hash-guarded emitter), `hooks-verify.mjs`, `fwd-types.mjs`, `closure-analyze.mjs`, `graph-check-shellhost.mjs`, `coverage-shellhost.mjs`, `make-orig-abs.mjs`, `cfg-shellhost.mjs` plus copies of the sibling's `split.mjs`/`analyze.mjs`/`write.mjs`/`exports.mjs`, and the original text `orig-ShellHost.tsx` and the phase-1 text `p1-ShellHost.tsx`. These are temporary helper scripts in the ticket folder, not repo tooling.

## 8. Files touched

Created: 30 child modules under `SH/` (9 module-level, 21 hook), listed in sections 2 and 3.3.
Rewritten: `SH/🟦️.tsx` (13,707 to 591 lines).
Edited outside `SH`: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json` (+30 names).

## 9. Follow-up: the 7 `?raw` source-text tests fixed (coordinator request)

No vitest alias that fakes the parent text is used any more. Each test reads the child module that now holds its needle; where a test asserts across modules or asserts absence, it checks the concatenation of the whole ShellHost tree (parent plus every depth-1 child) built with `import.meta.glob<string>([parent, "*/🟦️.ts", "*/🟦️.tsx"], { query: "?raw", import: "default", eager: true })`, so "must not contain" needles still cover every module. Test files edited (all under `E/../🧪️tests/`):

| Test | Change |
|---|---|
| `🎭️actor-window-actions/🟦️.ts` (2 tests) | AST scan of `InterpretedUiNode` windows now parses `📋️panel-hosting` (`?raw`); the `{ stores: new Map(currentBrowserActorUi.panels), ... }` needle reads `📤️document-transfer-tabs`; the `onAction`..`onActionStable` funnel slice reads `🖱️shell-actions`; `refuseBrowserActorActionDescriptor` absence checks the whole tree (plus a guard that the tree text is not empty) |
| `🧭️route-ledger/🟦️.ts` (1) | `shouldApply` guard reads `🗄️shell-uri-documents`; `settle(...)` reads `🧩️shell-core-state`; the two-site `invalidate()` count, the `rememberOpenSpaceId(null)` regex and both absence checks use the whole tree (the invalidate sites are split over `🚀️plugin-lifecycle` and `🛂️directory-route`) |
| `🩺️window-fault/🟦️.ts` (3) | DOM-contract test (attribute, `role="status"`, `refreshUi(...).catch`, three `liveInstanceWindowFaultV1` sites, i18n keys; spread over `🎚️shell-model`, `🧵️backbone-worker`, `🖼️canvas-footer` and others) uses the whole tree; pending-body test: `?? PENDING_WINDOW_UI_NODE` reads `📋️panel-hosting`, `?? pendingWindowUiNode()` reads `🔁️contributions-refresh`, the absence regex uses the tree; contributions test: the four absence needles use the tree, the four positive needles read `🔁️contributions-refresh` |
| `🪆️embedded-mount/🟦️.ts` (1) | the `shouldStartIntroduction(...)` AST scan parses `🎮️focus-and-input` |

Results:
- The 4 files together (`shellhost-vitest-readers-fixed.log`): 48 passed, 1 failed. All 7 target tests pass. The 1 failure is the pre-existing flaky real-mount test (`embedded-mount > renders two idle real ShellHosts ...`, 1 s `vi.waitFor`).
- Flake comparison, whole `embedded-mount` file, 5 alternating runs each (`shellhost-flaky2.log`): original ShellHost (alias) failed that one test in 3 of 5 runs, the split tree in 2 of 5, with the same alternating pattern (runs 1, 3 fail for both). So it is a pre-existing flake, equally present in the original; not a regression.
- Full renderer suite after the fix (`shellhost-vitest-full-after2.log`): 33 files / 41 tests failed, 151 files passed, 2,980 tests passed (of 3,021). Baseline (original ShellHost) was 31 files / 40 tests failed (of 2,909; peers added tests since). Diff against the baseline failure list (`shellhost-fails-baseline.txt` vs `shellhost-fails-after2.txt`):
  - None of the 7 `?raw` tests fails any more.
  - Not in baseline but unrelated to ShellHost: the flaky `embedded-mount` real-mount test above, and three Chromium/WebGPU tests (`prepared-readback`, `text-input-oracle`, `asset-cancellation`) that failed under the heavy load of this run (496 s versus 363 s earlier); re-run on their own they pass, 3 files / 33 tests (`shellhost-vitest-chromium-rerun.log`).
  - In the baseline only (flaky, gone): `editable-controls`, two `Paint2dHost` selection-focus tests.
