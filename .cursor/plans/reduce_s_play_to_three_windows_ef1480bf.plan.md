---
name: Reduce S Play to Three Windows
overview: "Reduce the S/OS playground default layout from 7 windows down to exactly 3: Media Graph, Media VFS, and Compiled DAG. Fully delete the App Host, Launcher, History, and Jack window kinds and all their dedicated glue code, including the now-unused SAppHostRouter technology-embedding component and s/react's SProgramLauncherPanel/SStudioHistoryPanel/SAppHostSurface."
todos: []
isProject: false
---

## Scope confirmed with user

- Fully delete App Host, Launcher, History, and Jack window kinds (constants, surfaces, body registrations, controller state/commands, dedicated UI components) — not just remove from the default layout.
- Dropping "App Host" means there is no replacement way to open/edit an app instance's content in the OS shell — instances become purely structural nodes on the Media Graph (apps are still instantiated via catalogue drag-and-drop onto the graph, which already works).
- History's undo/redo/checkpoint and Jack's query language are dropped entirely (not relocated). Note: the footer toolbar's Undo/Redo/Checkpoint buttons (`buildSPlayToolbarTools` in [s/core/js/index.ts](s/core/js/index.ts)) are a separate mechanism from the History _window_ and are unaffected — they will remain visible.
- `SAppHostRouter`/`SAppHostContent`/`SAppHostSurface` (in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)) and `SProgramLauncherPanel`/`SStudioHistoryPanel`/`SAppHostSurface` (in [s/react/index.tsx](s/react/index.tsx)) become fully unused once the windows are removed — delete them entirely, including the now-broken re-export in [framework/product/os/renderer/react/index.tsx](framework/product/os/renderer/react/index.tsx) (`OsAppHostRouter` + `SAppHostRouter` re-export; leave `resolveOsAppDefinitionForInstance`/`resolveOsAppDefinition` in place since they don't depend on the deleted component).

## Result layout

`S_PLAY_LAYOUT` in [s/core/js/index.ts](s/core/js/index.ts) becomes just:

```ts
export const S_PLAY_LAYOUT = createDefaultLayout([S_PLAY_WINDOW_MEDIA_GRAPH, S_PLAY_WINDOW_MEDIA_VFS, S_PLAY_WINDOW_COMPILED_DAG], "row", [40, 30, 30], ["Media Graph", "Media VFS", "Compiled DAG"]);
```

## Changes in [s/core/js/index.ts](s/core/js/index.ts)

- Remove constants: `S_PLAY_SURFACE_APP_HOST`, `S_PLAY_SURFACE_LAUNCHER`, `S_PLAY_SURFACE_HISTORY`, `S_PLAY_BODY_LAUNCHER`, `S_PLAY_BODY_HISTORY`, `S_PLAY_WINDOW_LAUNCHER`, `S_PLAY_WINDOW_HISTORY`, `S_PLAY_BODY_APP_HOST`, `S_PLAY_WINDOW_APP_HOST`, `S_PLAY_WINDOW_JACK`, `S_PLAY_BODY_JACK`, `S_PLAY_SURFACE_JACK`, `S_PLAY_DEFAULT_JACK_QUERY`. Keep Media Graph/Media VFS/Compiled DAG constants.
- Delete `sPlayMediaGraphForJack()` and `sPlayJackBoardFixtureJson()` (Jack-only, no other consumers).
- `SPlayController`: remove private fields `launcherProgramId`, `launcherEngagementInput`, `historyEngagementInput`, `appHostEngagementInput`, `jackEngagementInput`, `focusedInstanceId`, `jackBridge` (and its `bindPointerFocus`/`setJackQueryText` calls in the constructor).
- Remove `syncJackFixtureJson()`, `syncJackGraphSelect()`, `getJackQueryText()`, `getWriterDocumentJack()`, `getJackHoverOccurrences()`, `getJackSelectOccurrences()`, `getHoverEpoch()`, `getSelectEpoch()`, `getGraphHighlightedNodeIds()`, `getFocusedInstanceId()`, `getActiveInstance()` (all now-unused after App Host/Jack removal). Keep `getActiveInstanceId()` (still drives Media Graph node highlighting).
- Simplify `subscribeSnapshot()` to drop the `jackBridge.subscribe(listener)` unsubscribe wiring.
- In the constructor, drop the `onOpenInstance` option passed to `OsMediaGraphVirtualFileSystemController` (was only used to set `focusedInstanceId`/drive the App Host drill-in — opening a file in the VFS tree becomes a no-op, consistent with "no way to open app content").
- Remove `appHostMeasures()`, `appHostEngagement()`, `launcherMeasures()`, `launcherEngagement()`, `historyMeasures()`, `historyEngagement()`, `jackEngagement()`. Keep `mediaGraphMeasures()`, `mediaGraphEngagement()`, `compiledDagEngagement()`.
- `rebuildShellMode()`: trim `mainMode.windowKinds` to just Media Graph, Media VFS, Compiled DAG `WindowKindRuntime` entries.
- `run()`: remove the early-return blocks for `setJackQuery`, `setJackHover`, `setJackSelect`, `setGraphHover`, `setGraphSelect`, `runJackQuery`, and switch cases `appHostEngagementInput`, `appHostEngagementSubmit`, `launcherEngagementInput`, `launcherEngagementSubmit`, `jackEngagementInput`, `historyEngagementInput`, `historyEngagementSubmit`, `setLauncherProgram`, `openInstance`, `closeFocusedInstance`. Remove the now-dead `this.syncJackGraphSelect()` calls inside `setMediaNodeSelection`, `setAppInstanceSelection`, `selectInstance`, and the `this.syncJackFixtureJson()` calls in the constructor, store-subscribe callback, and `setActiveExample`.
- Remove now-unused imports: `JackHoverBridge` (from `@semio-tech/framework-playground-core`), `runJackOnMediaGraph` (from `@semio-tech/graph-dsl-core`; keep `wireLiteralFromDagFixtureJson`, still used by `getCompiledWireLiteral()`).
- `registerSPlayDeclarativeBodies()`: remove the `registerWindowBody` calls for `S_PLAY_BODY_APP_HOST`, `S_PLAY_BODY_LAUNCHER`, `S_PLAY_BODY_HISTORY`, `S_PLAY_BODY_JACK`. Keep Media Graph and Compiled DAG registrations (`buildWriterWindowBody` import stays, still used by Compiled DAG).
- Test suite (`import.meta.vitest` block): remove the `"openInstance and closeFocusedInstance toggle drill-in focus"` test case. No other existing tests reference App Host/Launcher/History/Jack.
- `buildSPlayToolbarTools()` is untouched (footer toolbar, not a window).

## Changes in [framework/product/playground/renderer/react/index.tsx](framework/product/playground/renderer/react/index.tsx)

- Remove imports: `S_PLAY_SURFACE_APP_HOST`, `S_PLAY_SURFACE_HISTORY`, `S_PLAY_SURFACE_LAUNCHER`, `S_PLAY_SURFACE_JACK`, `S_PLAY_BODY_JACK`, `S_PLAY_WINDOW_JACK`, `SAppHostSurface`, `SProgramLauncherPanel`, `SStudioHistoryPanel`.
- Delete `SAppHostContent`, `SAppHostRouter` (the ~20-case technology-embedding switch), `SAppHostSurfaceHost`, `SLauncherSurfaceHost`, `SHistorySurfaceHost`, `SPlayJackSurfaceHost`.
- `SSSurfaceHost`: drop the `appHost`/`launcher`/`history` view branches, keep only `mediaGraph` (and its fallback).
- `SMediaGraphSurfaceHost`: remove the `onOpenInstance` prop passed to `SMediaGraphCanvas` (drop the `openInstance` command dispatch — no replacement).
- `SPlayInner`: remove the `focusedInstanceId` branch entirely (the full-viewport "← Back to Media Graph" drill-in view with `SAppHostContent`); it always renders the tiled `PlaygroundView`.
- `registerSPlaySurfaceHosts()`: remove `registerUiSSurfaceHost(S_PLAY_SURFACE_APP_HOST, ...)`, `registerUiSSurfaceHost(S_PLAY_SURFACE_LAUNCHER, ...)`, `registerUiSSurfaceHost(S_PLAY_SURFACE_HISTORY, ...)`, `registerUiWriterSurfaceHost(S_PLAY_SURFACE_JACK, ...)`.
- Verify no leftover unused imports (e.g. technology-specific imports that `SAppHostRouter`'s cases pulled in, like `defaultDrawDocument`/`drawDocumentFromJson`/etc., `SPuzzle3dHost`, `PresentationDeck` — check each for other consumers in the file before removing; several are likely shared with other technology playgrounds in this same file and must stay).

## Changes in [s/react/index.tsx](s/react/index.tsx)

- Delete the `//#region 🔖ProgramLauncher` block (`SProgramLauncherPanel`).
- Delete the `//#region 🔖History` block (`SStudioHistoryPanel`) and its now-unused imports `HistoryTable` (`@semio-tech/vcs-react`), `buildOsHistoryColumns` (`@semio-tech/framework-os-core`).
- Delete the `//#region 🔖AppHost` block (`SAppHostSurface`).
- Keep `SMediaGraphCanvas` as-is (including its optional `onOpenInstance` prop/`handlePointerUp` — this is a generic, reusable DagCanvas capability; S play's call site simply stops passing it, per above).
- Check `useStudioStore`/`useDispatchStudioCommand`/`useStudioProjection`/`useStudioGeneration` are still used by `SMediaGraphCanvas` or elsewhere after deletions (they are used by the deleted panels too — confirm at least one remaining consumer, otherwise these become dead and should be removed too).

## Changes in [framework/product/os/renderer/react/index.tsx](framework/product/os/renderer/react/index.tsx)

- Remove `OsAppHostRouter` function and the `SAppHostRouter` import/re-export (its only implementation is being deleted). Leave `resolveOsAppDefinitionForInstance`/`resolveOsAppDefinition` untouched.

## Verification

- `bun nx run @semio-tech/s-core:test` and the playground renderer's test target pass.
- Boot the OS dev playground (port 6066) via Playwright/browser tooling and confirm only 3 windows render (Media Graph, Media VFS, Compiled DAG), dragging an app from the Workbench catalogue tab onto the Media Graph still spawns a node, and no console errors reference the deleted window kinds/components.
  </plan>
  <todos>[{"id": "core-constants-layout", "content": "s/core/js/index.ts: trim S_PLAY_LAYOUT to 3 windows, remove App Host/Launcher/History/Jack constants"}, {"id": "core-controller-state", "content": "s/core/js/index.ts: remove App Host/Launcher/History/Jack controller state, methods, run() cases, and jackBridge wiring"}, {"id": "core-tests", "content": "s/core/js/index.ts: remove openInstance/closeFocusedInstance test case"}, {"id": "renderer-remove-windows", "content": "framework/product/playground/renderer/react/index.tsx: delete SAppHostRouter/SAppHostContent/SAppHostSurfaceHost/SLauncherSurfaceHost/SHistorySurfaceHost/SPlayJackSurfaceHost, trim SSSurfaceHost, SPlayInner drill-in, registerSPlaySurfaceHosts, imports"}, {"id": "s-react-cleanup", "content": "s/react/index.tsx: delete SProgramLauncherPanel, SStudioHistoryPanel, SAppHostSurface regions and now-unused imports"}, {"id": "os-renderer-cleanup", "content": "framework/product/os/renderer/react/index.tsx: remove OsAppHostRouter/SAppHostRouter re-export"}, {"id": "verify", "content": "Run s-core/playground renderer tests, boot OS dev playground, verify 3 windows and drag-and-drop spawn still work"}]
