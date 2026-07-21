import { createElement, useState, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import {
  deriveUtilityNodes,
  resolveWindowActions,
  partitionWindowMeasures,
  type ActionArgDef,
  type ActionDefinition,
  type AppDefinition,
  type AppModeDefinition,
  type AppWindowKindDefinition,
  type CommandDefinition,
  type UtilityDefinition,
  type WindowMeasure,
  type UtilityNode,
  type UiNode,
} from "@semio-tech/framework-core";
import { ENTWERFEN_MIT_BESTAND_BRAND } from "../../product/os/dev/brand/index.ts";
import { SelectionMarquee, ZUKUNFT_BAU_PROJECT_URL } from "@semio-tech/ui-react";
import {
  Canvas2dHost,
  worldToScreenLogical,
  Board2dHost,
  board2dCameraActionArgs,
  buildPuzzle2dSelectionMenuItems,
  beginPuzzle2dPeerGesture,
  collectPuzzle2dLiveMirrorOps,
  coalesceBoard2dEvents,
  endPuzzle2dPeerGesture,
  notifyPuzzle2dPeersGestureEnded,
  parsePuzzle2dCatalogueDragPayload,
  board2dPeers,
  puzzle2dFixtureDropPreviewJson,
  puzzle2dPeerOwnsGesture,
  puzzle2dScreenToWorld,
  pushPuzzle2dLiveMirrorOps,
  registerBoard2dPeer,
  unregisterBoard2dPeer,
  NodeGraphHost,
  catalogueGhostDescriptorJson,
  computeDagMarqueeOverlay,
  nodeGraphViewportActionArgs,
  parseCatalogueAppDragPayload,
  parseDagSliderOverlays,
  resolveFixtureWidgetInstanceId,
  Paint2dHost,
  TableHost,
  GraphTimelineHost,
  TextEditorHost,
  buildTextEditorContextMenuItems,
  lineRangeAt,
  multiSpanReplace,
  World3dHost,
  brushObjectPlacementArgs,
  parsePuzzle3dCatalogueDragPayload,
  mergeWorldViewportCamera,
  resolveMeshStyle,
  resolveMeshSelectionPreviewStyle,
  resolveVortexPointerDownIntent,
  worldMeshMaterialRevision,
  worldVortexMaterialRevision,
  resolveWorldSelectionMergeMode,
  resolveWorldContextMenuTarget,
  shouldReattachWorldViewportCamera,
  snapWorldPointToGrid,
  world3dViewportCameraSeedKey,
  worldInstancePickBlocked,
  parseWorldTerrainStyle,
  InkCanvasHost,
  inkItemBounds,
  eraseInkStrokePointsInItem,
  inkHtmlToParagraphs,
  inkParagraphsToHtml,
  inkResizeBounds,
  inkScaleItemWithinGroup,
  inkClipboardPayload,
  inkItemsFromClipboardPayload,
  screenToWorld,
  worldToScreen,
  type InkDocument,
  type InkStrokeItem,
  appDocumentLabel,
  appWindowDocumentLabel,
  applyUiRefreshResponseToCache,
  resolveAppDocument,
  buildUtilityRibbonSegments,
  buildUiRefreshRequest,
  dedupeUtilityNodesById,
  flattenPanelTabLeaves,
  groupUtilityNodesByCategory,
  initialShellState,
  isFlowGraphScene,
  loadPluginModule,
  mergeRecordPreservingIdentity,
  parseStudioShellPath,
  preserveJsonIdentity,
  reconcileUtilityPath,
  resolveUtilityNodes,
  resolveUtilities,
  frameworkHistoryUtilityNodes,
  actionStageKey,
  actionRequiresStagedForm,
  resolveKeybindingIntent,
  resolveUtilityActivation,
  WindowActionPane,
  resolveCommands,
  commandCategories,
  buildCommandCategoryTree,
  buildCommandCategoryTabs,
  buildOsCommands,
  createLatestAsyncDispatcher,
  createInFlightSkippingInterval,
  dispatchOsCommand,
  mergeShellLockSources,
  resolveShellDefaults,
  resolveShellLocks,
  shouldPersistIntroductionSeen,
  shouldReplayIntroductionOnLoad,
  isEphemeralShellBrand,
  clearDurableShellStorage,
  type ResolvedCommand,
  shellReducer,
  sortUtilityNodes,
  spawnedWindowChromeForKind,
  UtilityTree,
  type UiRefreshCache,
  UIFind,
  UIFindProvider,
  uiNodeToTreePanelConfig,
  UISearch,
  type UISearchItem,
  useUIFind,
  interpretUiNode,
  dispatchOpenedFiles,
  scheduleDispatchAction,
  sampleMediaFrameTimestampsMs,
  runTier2VideoFrames,
  runRequestMediaFrames,
  createFrameworkDisplayPanelTabs,
  type DisplayHostApi,
} from "./index.tsx";
import { decodeWorldProjectionTemplateId } from "@semio-tech/infinite-world-r3f";

//#region 🔌jsdom polyfills
// cmdk (used by UISearch/UIFind's CommandDialog) calls ResizeObserver on mount; jsdom does not implement it.
class ResizeObserverMock {
  observe() {}
  unobserve() {}
  disconnect() {}
}
if (!globalThis.ResizeObserver) globalThis.ResizeObserver = ResizeObserverMock as unknown as typeof ResizeObserver;
// cmdk calls scrollIntoView on the active item; jsdom does not implement it.
if (!Element.prototype.scrollIntoView) Element.prototype.scrollIntoView = () => {};
//#endregion 🔌jsdom polyfills

const noopAction = () => {};

describe("framework sync utilities", () => {
  it("builds three sync backbone toggles", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os-core");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    expect(utilities).toHaveLength(3);
    expect(utilities.map((utility) => utility.id)).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
    expect(utilities[0]?.pressed).toBe(true);
  });

  it("has no active toggle when detached", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os-core");
    const utilities = buildFrameworkSyncUtilities(null);
    expect(utilities.every((utility) => !utility.pressed)).toBe(true);
  });

  it("groups File, Folder, and Remote under a single Sync category collection", async () => {
    const { buildFrameworkSyncUtilities } = await import("@semio-tech/framework-os-core");
    const utilities = buildFrameworkSyncUtilities("file:///demo");
    const grouped = groupUtilityNodesByCategory(utilities as unknown as UtilityNode[], ["sync"]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0]).toMatchObject({ id: "sync", kind: "collection" });
    expect(grouped[0].kind === "collection" ? grouped[0].children.map((child) => child.id) : []).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
  });
});

describe("live measure dispatch", () => {
  it("serializes document updates and skips stale slider values", async () => {
    const values: number[] = [];
    let finishFirst = () => {};
    const dispatch = createLatestAsyncDispatcher((value: number) => {
      values.push(value);
      if (value === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
    });

    dispatch(1);
    dispatch(2);
    dispatch(19);
    dispatch(24);
    expect(values).toEqual([1]);

    finishFirst();
    await vi.waitFor(() => expect(values).toEqual([1, 24]));
  });
});

describe("in-flight skipping interval", () => {
  it("drops overlapping ticks instead of queueing them behind a slow run", async () => {
    const runs: number[] = [];
    let finishFirst = () => {};
    const timers: Array<() => void> = [];
    const stop = createInFlightSkippingInterval(
      () => {
        runs.push(runs.length + 1);
        if (runs.length === 1) return new Promise<void>((resolve) => (finishFirst = resolve));
      },
      10,
      (fn) => {
        timers.push(fn as () => void);
        return 1 as unknown as ReturnType<typeof setInterval>;
      },
      () => {},
    );

    expect(timers).toHaveLength(1);
    timers[0]!();
    timers[0]!();
    timers[0]!();
    expect(runs).toEqual([1]);

    finishFirst();
    await Promise.resolve();
    timers[0]!();
    expect(runs).toEqual([1, 2]);
    stop();
  });
});

describe("shell store reducer", () => {
  const baseState = () => initialShellState({ plugins: [] });

  it("toggles the overlays slice via a direct value without touching unrelated slices", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_SEARCH_OPEN", value: true });
    expect(next.overlays.searchOpen).toBe(true);
    expect(next.overlays.findOpen).toBe(false);
    expect(next.pluginRuntime).toBe(state.pluginRuntime);
    expect(next.uiPrefs).toBe(state.uiPrefs);
  });

  it("starts, advances, and dismisses an introduction via SET_INTRODUCTION_STEP without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.introductionStepIndex).toBeNull();
    const started = shellReducer(state, { type: "SET_INTRODUCTION_STEP", value: 0 });
    expect(started.overlays.introductionStepIndex).toBe(0);
    expect(started.layout).toBe(state.layout);
    const advanced = shellReducer(started, { type: "SET_INTRODUCTION_STEP", value: (prev) => (prev ?? 0) + 1 });
    expect(advanced.overlays.introductionStepIndex).toBe(1);
    const dismissed = shellReducer(advanced, { type: "SET_INTRODUCTION_STEP", value: null });
    expect(dismissed.overlays.introductionStepIndex).toBeNull();
  });

  it("opens, replaces, and closes a dialog via SET_DIALOG without touching unrelated slices", () => {
    const state = baseState();
    expect(state.overlays.dialog).toBeNull();
    const opened = shellReducer(state, { type: "SET_DIALOG", value: { dialogId: "addObject", seedArgs: { objectKind: "Object" } } });
    expect(opened.overlays.dialog).toEqual({ dialogId: "addObject", seedArgs: { objectKind: "Object" } });
    expect(opened.layout).toBe(state.layout);
    expect(opened.pluginRuntime).toBe(state.pluginRuntime);
    const replaced = shellReducer(opened, { type: "SET_DIALOG", value: { dialogId: "confirmDelete" } });
    expect(replaced.overlays.dialog).toEqual({ dialogId: "confirmDelete" });
    const closed = shellReducer(replaced, { type: "SET_DIALOG", value: null });
    expect(closed.overlays.dialog).toBeNull();
  });

  it("toggles the layout slice via an updater function", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const toggled = shellReducer(opened, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: (prev) => !prev });
    expect(opened.layout.panels["top-left"].visible).toBe(true);
    expect(toggled.layout.panels["top-left"].visible).toBe(false);
    expect(toggled.overlays).toBe(opened.overlays);
  });

  it("toggles a middle anchor via SET_PANEL_VISIBLE the same way as a corner", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_PANEL_VISIBLE", anchor: "top-middle", value: true });
    expect(opened.layout.panels["top-middle"].visible).toBe(true);
    expect(opened.layout.panels["top-left"].visible).toBe(state.layout.panels["top-left"].visible);
  });

  it("resets the dock override, every anchor's active path/visible/size, drill-down memory, and tree expansion via RESET_DOCK", () => {
    const state = baseState();
    const rearranged = shellReducer(state, {
      type: "SET_DOCK_OVERRIDE",
      value: {
        version: 3,
        anchors: { "top-left": [{ id: "moved" }], "top-middle": [], "top-right": [], "right-middle": [], "bottom-right": [], "bottom-middle": [], "bottom-left": [], "left-middle": [] },
      },
    });
    const withPath = shellReducer(rearranged, { type: "SET_PANEL_PATH", anchor: "top-left", value: ["moved"] });
    const withVisible = shellReducer(withPath, { type: "SET_PANEL_VISIBLE", anchor: "top-left", value: true });
    const withSize = shellReducer(withVisible, { type: "SET_PANEL_SIZE", anchor: "top-left", value: 999 });
    const withMemory = shellReducer(withSize, { type: "SET_PANEL_PATH_MEMORY", value: { moved: "child" } });
    const withTreeOpen = shellReducer(withMemory, { type: "SET_TREE_OPEN_STATE", id: "unit:section", open: true });
    const reset = shellReducer(withTreeOpen, { type: "RESET_DOCK" });
    expect(reset.layout.dockOverride).toBeNull();
    expect(reset.layout.panels["top-left"].path).toEqual([]);
    expect(reset.layout.panels["top-left"].visible).toBe(false);
    expect(reset.layout.panels["top-left"].size).toBe(state.layout.panels["top-left"].size);
    expect(reset.layout.panelPathMemory).toEqual({});
    expect(reset.layout.treeOpenStates).toEqual({});
  });

  it("HYDRATE_DOCK_UI restores a persisted size for the top-middle anchor", () => {
    const state = baseState();
    const hydrated = shellReducer(state, { type: "HYDRATE_DOCK_UI", value: { version: 3, anchors: { "top-middle": { visible: true, size: 420 } } } });
    expect(hydrated.layout.panels["top-middle"].visible).toBe(true);
    expect(hydrated.layout.panels["top-middle"].size).toBe(420);
    expect(hydrated.layout.panels["top-left"]).toEqual(state.layout.panels["top-left"]);
  });

  it("updates the uiPrefs slice and leaves the sync slice referentially unchanged", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_UI_COMPACT", value: (prev) => !prev });
    expect(next.uiPrefs.uiCompact).toBe(!state.uiPrefs.uiCompact);
    expect(next.sync).toBe(state.sync);
  });

  it("action-panel slice: fold/expand/stage/reset/active-utility update only their own keys and preserve identity on no-ops", () => {
    const state = baseState();

    const folded = shellReducer(state, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false });
    expect(folded.actionPane.foldedByWindowId).toEqual({ w1: false });
    expect(folded.layout).toBe(state.layout);
    // no-op fold keeps the whole slice referentially stable
    expect(shellReducer(folded, { type: "SET_ACTION_PANE_FOLDED", windowId: "w1", value: false }).actionPane).toBe(folded.actionPane);

    const expanded = shellReducer(folded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" });
    expect(expanded.actionPane.expandedByWindowId).toEqual({ w1: "extrude" });
    expect(shellReducer(expanded, { type: "SET_ACTION_PANE_EXPANDED", windowId: "w1", value: "extrude" }).actionPane).toBe(expanded.actionPane);

    const staged = shellReducer(expanded, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 });
    expect(staged.actionPane.stagedArgsByKey).toEqual({ "w1:extrude": { depth: 3 } });
    const stagedMore = shellReducer(staged, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "segments", value: 2 });
    expect(stagedMore.actionPane.stagedArgsByKey["w1:extrude"]).toEqual({ depth: 3, segments: 2 });
    expect(shellReducer(stagedMore, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 }).actionPane).toBe(stagedMore.actionPane);

    const reset = shellReducer(stagedMore, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" });
    expect(reset.actionPane.stagedArgsByKey["w1:extrude"]).toBeUndefined();
    // reset keeps the panel expanded
    expect(reset.actionPane.expandedByWindowId["w1"]).toBe("extrude");
    expect(shellReducer(reset, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" }).actionPane).toBe(reset.actionPane);

    const activated = shellReducer(reset, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" });
    expect(activated.actionPane.activeUtilityByWindowId).toEqual({ w1: "pen" });
    expect(shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: "pen" }).actionPane).toBe(activated.actionPane);
    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_UTILITY", windowId: "w1", utilityId: null });
    expect(deactivated.actionPane.activeUtilityByWindowId["w1"]).toBeNull();
  });

  it("commandPanel slice: expand/collapse and stage/reset update only their own keys and preserve identity on no-ops (category active/fold state now lives in layout.panels['bottom-middle'], not this slice)", () => {
    const state = baseState();

    const expanded = shellReducer(state, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");
    expect(expanded.layout).toBe(state.layout);
    expect(shellReducer(expanded, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" }).commandPanel).toBe(expanded.commandPanel);

    const staged = shellReducer(expanded, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" });
    expect(staged.commandPanel.stagedArgsByCommandId).toEqual({ "os.setThemeId": { themeId: "semio" } });
    expect(shellReducer(staged, { type: "STAGE_COMMAND_ARG", commandId: "os.setThemeId", argId: "themeId", value: "semio" }).commandPanel).toBe(staged.commandPanel);

    const reset = shellReducer(staged, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" });
    expect(reset.commandPanel.stagedArgsByCommandId["os.setThemeId"]).toBeUndefined();
    expect(shellReducer(reset, { type: "RESET_COMMAND_ARGS", commandId: "os.setThemeId" }).commandPanel).toBe(reset.commandPanel);
  });

  it("command palette category is the bottom-middle anchor's own SET_PANEL_PATH; the UI's category-switch handler additionally dispatches SET_COMMAND_EXPANDED:null (reproducing the old single-action collapse-on-switch behavior across two actions)", () => {
    const state = baseState();
    const onCategory = shellReducer(state, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.appearance"] });
    expect(onCategory.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.appearance"]);

    const expanded = shellReducer(onCategory, { type: "SET_COMMAND_EXPANDED", value: "os.setThemeId" });
    expect(expanded.commandPanel.expandedCommandId).toBe("os.setThemeId");

    const switchedPath = shellReducer(expanded, { type: "SET_PANEL_PATH", anchor: "bottom-middle", value: ["framework.category.command", "command.category.layout"] });
    const switched = shellReducer(switchedPath, { type: "SET_COMMAND_EXPANDED", value: null });
    expect(switched.layout.panels["bottom-middle"].path).toEqual(["framework.category.command", "command.category.layout"]);
    expect(switched.commandPanel.expandedCommandId).toBeNull();
  });
});

// 🐢 Puzzle 2D performance round 2: the per-interaction full-shell refresh cascade was dominated by
// React reconciling freshly-parsed-but-structurally-identical UiNode/engagement/measure trees on every
// action (select/camera/nodeMove). These helpers let unchanged bodies keep their object identity across
// a `refreshUi` so `InterpretedUiNode`'s `React.memo` (ui-interpreter.tsx) and `modeWindows`'s
// `useMemo` (os-shell.tsx) can bail instead of reconciling the whole shell every time.
describe("ui identity preservation (puzzle 2d perf)", () => {
  it("preserveJsonIdentity reuses the previous reference for structurally-equal values", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
  });

  it("preserveJsonIdentity returns the new reference when content actually differs", () => {
    const previous = { type: "text", value: "hello" };
    const next = { type: "text", value: "goodbye" };
    expect(preserveJsonIdentity(previous, next)).toBe(next);
  });

  it("preserveJsonIdentity treats nested arrays/objects structurally, not just top-level fields", () => {
    const previous = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    const next = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 2 },
      ],
    };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
    const moved = {
      nodes: [
        { id: "a", x: 1 },
        { id: "b", x: 3 },
      ],
    };
    expect(preserveJsonIdentity(previous, moved)).toBe(moved);
  });

  it("preserveJsonIdentity treats undefined previous as always-changed", () => {
    const next = { type: "text", value: "hello" };
    expect(preserveJsonIdentity(undefined, next)).toBe(next);
  });

  it("mergeRecordPreservingIdentity reuses the whole previous record when every key is unchanged", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(merged).toBe(prev);
  });

  it("mergeRecordPreservingIdentity reuses per-key references, replacing only the changed key", () => {
    const prev = { overview: { type: "text", value: "a" }, detail: { type: "text", value: "b" } };
    const merged = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "changed" }],
    ]);
    expect(merged).not.toBe(prev);
    expect(merged.overview).toBe(prev.overview);
    expect(merged.detail).not.toBe(prev.detail);
  });

  it("mergeRecordPreservingIdentity treats a key being added or removed as a change", () => {
    const prev = { overview: { type: "text", value: "a" } };
    const withNewKey = mergeRecordPreservingIdentity(prev, [
      ["overview", { type: "text", value: "a" }],
      ["detail", { type: "text", value: "b" }],
    ]);
    expect(withNewKey).not.toBe(prev);
    expect(withNewKey.overview).toBe(prev.overview);
  });
});

// 🐢 Puzzle 2D performance round 3: the batched, hash-conditional `refresh-ui` protocol that replaces
// ~12 sequential per-section WASM calls with one round trip. `buildUiRefreshRequest` restricts what's
// asked for by scope and attaches known hashes; `applyUiRefreshResponseToCache` writes back only the
// sections the plugin actually says changed.
describe("batched ui refresh request/response (puzzle 2d perf round 3)", () => {
  const windowKinds = [
    { id: "overview", bodyKey: "puzzle2d.play.overview" },
    { id: "detail", bodyKey: "puzzle2d.play.detail" },
  ];
  const panelTabLeaves = [{ kind: { kind: "app" as const, id: "framework.panel.document" }, bodyKey: "puzzle2d.play.layers" }];

  it("buildUiRefreshRequest for a full scope requests every window/panel/engagements/measures/labels section (utility bars are now registry-derived, not a plugin section)", () => {
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview", "detail"]);
    expect(request?.panels?.map((p) => p.key)).toEqual(["framework.panel.document"]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeDefined();
    expect(request?.labels).toBeDefined();
  });

  it("buildUiRefreshRequest for none returns null", () => {
    expect(buildUiRefreshRequest({ kind: "none" }, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest for a partial scope requests only the listed window/panel bodies and flags", () => {
    const scope = { kind: "partial" as const, windowBodies: ["puzzle2d.play.overview"], panelBodies: [], engagements: true };
    const request = buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map());
    expect(request?.windows?.map((w) => w.key)).toEqual(["overview"]);
    expect(request?.panels).toEqual([]);
    expect(request?.engagements).toBeDefined();
    expect(request?.measures).toBeUndefined();
    expect(request?.labels).toBeUndefined();
  });

  it("buildUiRefreshRequest returns null for a partial scope that matches nothing in this app", () => {
    const scope = { kind: "partial" as const, windowBodies: ["some-other-app.body"] };
    expect(buildUiRefreshRequest(scope, windowKinds, panelTabLeaves, {}, new Map())).toBeNull();
  });

  it("buildUiRefreshRequest attaches the cached hash for a section that was already fetched once", () => {
    const cache: UiRefreshCache = new Map([["window:overview", { hash: "abc123", value: { type: "text", value: "x" } }]]);
    const request = buildUiRefreshRequest({ kind: "full" }, windowKinds, panelTabLeaves, {}, cache);
    expect(request?.windows?.find((w) => w.key === "overview")?.hash).toBe("abc123");
    expect(request?.windows?.find((w) => w.key === "detail")?.hash).toBeUndefined();
  });

  it("applyUiRefreshResponseToCache writes changed sections and ignores hash-only (unchanged) ones", () => {
    const cache: UiRefreshCache = new Map([["window:detail", { hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } }]]);
    applyUiRefreshResponseToCache(cache, {
      windows: [
        { key: "overview", hash: "new-hash", value: { type: "text", value: "fresh" } },
        { key: "detail", hash: "old-hash" }, // unchanged: no `value` in the response
      ],
      engagements: { key: "engagements", hash: "eng-hash", value: { overview: {} } },
    });
    expect(cache.get("window:overview")).toEqual({ hash: "new-hash", value: { type: "text", value: "fresh" } });
    // Unchanged section: cache entry is untouched (still the old hash/value, not overwritten with nothing).
    expect(cache.get("window:detail")).toEqual({ hash: "old-hash", value: { type: "text", value: "stale-should-not-be-touched" } });
    expect(cache.get("engagements")).toEqual({ hash: "eng-hash", value: { overview: {} } });
  });
});

describe("framework plugin runtime", () => {
  it("preserves batched UI refreshes through the React plugin adapter", async () => {
    const moduleUrl = `data:application/javascript,${encodeURIComponent("export function semio_plugin_manifest(){return JSON.stringify({pluginId:'mock-refresh',label:'Mock Refresh',version:'0',apps:[],programs:[],examples:[]})};export function semio_plugin_refresh_ui(instanceId,requestJson){return JSON.stringify({windows:[{key:'overview',hash:'fresh',value:{instanceId,request:JSON.parse(requestJson)}}]})}")}`;
    const handle = await loadPluginModule("mock-refresh", moduleUrl);
    await expect(handle.refreshUi(7, { viewState: {} })).resolves.toEqual({
      windows: [{ key: "overview", hash: "fresh", value: { instanceId: 7, request: { viewState: {} } } }],
    });
  });

  it("loads plugin modules through framework-core", async () => {
    const { loadPluginModule } = await import("@semio-tech/framework-core");
    const handle = await loadPluginModule("mock", "data:application/javascript,export function semio_plugin_manifest(){return JSON.stringify({pluginId:'mock',label:'Mock',version:'0',apps:[],programs:[],examples:[]})}");
    expect(handle.manifest.pluginId).toBe("mock");
  });

  it("parses a typed InvocationResponse, including requestedEffects, from a plugin handle-action response", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework-core");
    const response = parseInvocationResponse(
      JSON.stringify({
        output: null,
        operations: [{ diff: { payload: { schemaId: "draw.op", document: { id: "forest" } } } }],
        inverseGroup: { invocationId: "setActiveExample:1:0", operations: [], inverseOperations: [] },
        requestedEffects: [{ navigate: { uri: "/studios/forest" } }],
      }),
    );
    expect(response.operations).toHaveLength(1);
    expect(response.requestedEffects).toEqual([{ navigate: { uri: "/studios/forest" } }]);
  });

  it("falls back to an empty InvocationResponse for malformed handle-action JSON", async () => {
    const { parseInvocationResponse } = await import("@semio-tech/framework-core");
    expect(parseInvocationResponse("not json")).toEqual({ output: null, operations: [], inverseGroup: { invocationId: "", operations: [], inverseOperations: [] } });
    expect(parseInvocationResponse(JSON.stringify({ output: null }))).toEqual({ output: null, operations: [], inverseGroup: { invocationId: "", operations: [], inverseOperations: [] } });
  });

  it("serializes concurrent plugin wasm handle calls", async () => {
    const { withSerializedPluginWasmHandle } = await import("@semio-tech/framework-core");
    let inFlight = 0;
    let maxInFlight = 0;
    const handle = withSerializedPluginWasmHandle({
      pluginId: "mock",
      manifest: { pluginId: "mock", label: "Mock", version: "0", apps: [], programs: [], examples: [] },
      createApp: async () => 1,
      destroyApp: async () => {},
      handleAction: async () => {
        inFlight += 1;
        maxInFlight = Math.max(maxInFlight, inFlight);
        await new Promise((resolve) => setTimeout(resolve, 5));
        inFlight -= 1;
        return { output: null, operations: [], inverseGroup: { invocationId: "", operations: [], inverseOperations: [] } };
      },
      render: async () => ({ type: "text", value: "x" }),
      refreshUi: async () => ({}),
      dispose: () => {},
    });
    await Promise.all([handle.handleAction(1, "{}", {}), handle.handleAction(1, "{}", {}), handle.handleAction(1, "{}", {})]);
    expect(maxInFlight).toBe(1);
  });
});

describe("framework renderer types", () => {
  it("keeps window tabs concise while retaining the app fallback", () => {
    const app = {
      id: "puzzle3d-play",
      label: "Puzzle 3D",
      document: ["semio", "puzzle", "3d"],
      terminologyDocuments: { reuse: ["Entwerfen mit Bestand", "Aggregator"] },
      controllerId: "puzzle3d-play",
      modes: [],
      windowKinds: [],
      panelTabs: [],
      keybindings: [],
    };
    expect(appDocumentLabel(app.document)).toBe("semio · puzzle · 3d");
    expect(appWindowDocumentLabel(app, "native", "Flow")).toBe("Flow");
    expect(appWindowDocumentLabel(app, "native", "Preview")).toBe("Preview");
    expect(appWindowDocumentLabel(app, "native", "")).toBe("Puzzle 3D");
    expect(appWindowDocumentLabel(app, "reuse", "")).toBe("Aggregator");
    expect(resolveAppDocument(app, "native")).toEqual(["semio", "puzzle", "3d"]);
    expect(resolveAppDocument(app, "reuse")).toEqual(["Entwerfen mit Bestand", "Aggregator"]);
    expect(appDocumentLabel(resolveAppDocument(app, "reuse"))).toBe("Entwerfen mit Bestand · Aggregator");
  });

  it("flattens a recursive panelTabs tree to its leaves, depth-first", () => {
    const tabs = [
      { id: "framework.panel.document", label: "Document", group: "workbench", bodyKey: "doc" },
      {
        id: "framework.panel.catalogue",
        label: "Catalogue",
        group: "workbench",
        children: [
          { id: "framework.panel.catalogue.words", label: "Words", group: "workbench", bodyKey: "words" },
          { id: "framework.panel.catalogue.headings", label: "Headings", group: "workbench", bodyKey: "headings" },
        ],
      },
    ];
    const leaves = flattenPanelTabLeaves(tabs);
    expect(leaves.map((tab) => tab.id)).toEqual(["framework.panel.document", "framework.panel.catalogue.words", "framework.panel.catalogue.headings"]);
    expect(leaves.every((tab) => Boolean(tab.bodyKey))).toBe(true);
  });

  it("accepts component scene nodes", () => {
    const node: UiNode = {
      type: "componentScene",
      surfaceId: "draw.play.composite",
      controllerId: "draw-play",
      componentKind: "canvas-2d",
      canvas2d: {
        cameraX: 0,
        cameraY: 0,
        zoom: 1,
        layersJson: "[]",
      },
    };
    expect(node.componentKind).toBe("canvas-2d");
  });

  it("accepts graph-timeline component scene nodes", () => {
    const node: UiNode = {
      type: "componentScene",
      surfaceId: "vcs.play.history",
      controllerId: "vcs-play",
      componentKind: "graph-timeline",
      graphTimeline: {
        columnsJson: "[]",
      },
    };
    expect(node.componentKind).toBe("graph-timeline");
  });
});

describe("framework external slots", () => {
  it("resolves external slots through contributor plugins", async () => {
    const { resolveExternalSlots } = await import("@semio-tech/framework-core");
    const handle = {
      pluginId: "forms-module-procedural",
      manifest: { pluginId: "forms-module-procedural", label: "Module", version: "0", apps: [], programs: [], examples: [] },
      createApp: async () => 7,
      destroyApp: async () => {},
      handleAction: async () => [],
      render: async () => ({ type: "text", value: "fallback" }),
      renderWithDocument: async (_instanceId: number, bodyKey: string) => ({
        type: "text",
        value: `resolved:${bodyKey}`,
      }),
      refreshUi: async () => ({}),
      dispose: () => {},
    };
    const resolved = await resolveExternalSlots(
      {
        type: "externalSlot",
        pluginId: "forms-module-procedural",
        appId: "forms-module-procedural",
        bodyKey: "preview",
        paramsJson: "{}",
      },
      {
        plugins: new Map([["forms-module-procedural", handle]]),
        contributorInstances: new Map(),
        viewState: {},
      },
    );
    expect(resolved).toEqual({ type: "text", value: "resolved:preview" });
  });

  it("renders external slot fallback text when unresolved", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "externalSlot",
          pluginId: "missing-module",
          appId: "missing-module",
          bodyKey: "preview",
          paramsJson: "{}",
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain("Extension unavailable: missing-module");
  });
});

describe("declarative forms parity", () => {
  it("renders field description, required marker and inline error", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "field",
          id: "forms-try.name",
          label: "Name",
          description: "Your full name",
          required: true,
          error: "Name is required",
          child: {
            type: "input",
            id: "forms-try.name.input",
            inputKind: "text",
            value: "",
            onChange: { controllerId: "forms-play", action: "setTryValue" },
          },
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain("Your full name");
    expect(markup).toContain("Name is required");
    expect(markup).toContain("*");
    expect(markup).toContain('data-slot="field-error"');
  });

  it("renders slider unit readout", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "slider",
          id: "forms-try.volume.slider",
          value: 60,
          min: 0,
          max: 100,
          step: 5,
          unit: "%",
          onChange: { controllerId: "forms-play", action: "setTryValue" },
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain("60 %");
  });

  it("renders numberStepper as a single-border Stepper control, not hand-rolled double-bordered buttons", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "numberStepper",
          id: "forms-try.height.stepper",
          value: 3,
          step: 1,
          uniform: true,
          onAbsolute: { controllerId: "forms-play", action: "setTryValueAbsolute" },
          onDelta: { controllerId: "forms-play", action: "setTryValueDelta" },
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain('data-slot="stepper-group"');
    expect(markup).toContain('data-slot="stepper-minus"');
    expect(markup).toContain('data-slot="stepper-plus"');
    expect(markup).not.toContain("border-border");
  });

  it("shows the mixed-values placeholder on a non-uniform numberStepper", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "numberStepper",
          id: "forms-try.height.stepper",
          value: 0,
          step: 1,
          uniform: false,
          onAbsolute: { controllerId: "forms-play", action: "setTryValueAbsolute" },
          onDelta: { controllerId: "forms-play", action: "setTryValueDelta" },
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain('data-mixed="true"');
  });

  it("renders a group node as a labeled section nesting its child controls (Origin > X/Y/Z steppers)", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "group",
          id: "puzzle3d-play-inspector.object.origin",
          label: "Origin",
          defaultOpen: true,
          children: [
            {
              type: "field",
              id: "puzzle3d-play-inspector.object.origin.x",
              label: "X",
              child: { type: "numberStepper", id: "puzzle3d-play-inspector.object.origin.x", value: 1, step: 0.1, uniform: true, onAbsolute: { controllerId: "puzzle3d-play", action: "patchInspector" }, onDelta: { controllerId: "puzzle3d-play", action: "patchInspector" } },
            },
            {
              type: "field",
              id: "puzzle3d-play-inspector.object.origin.y",
              label: "Y",
              child: { type: "numberStepper", id: "puzzle3d-play-inspector.object.origin.y", value: 2, step: 0.1, uniform: true, onAbsolute: { controllerId: "puzzle3d-play", action: "patchInspector" }, onDelta: { controllerId: "puzzle3d-play", action: "patchInspector" } },
            },
          ],
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain(">Origin</h2>");
    expect(markup).toContain(">X</label>");
    expect(markup).toContain(">Y</label>");
    expect(markup).toContain('data-slot="stepper-group"');
  });

  it("tokenizes stack node gap/padding instead of hardcoded rem inline styles, and keeps separators off raw border-border", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "stack",
          direction: "vertical",
          id: "forms-blueprint.section.q1",
          gap: "tight",
          children: [{ type: "text", value: "text · q1" }, { type: "separator" }],
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain("gap-single");
    expect(markup).not.toContain("style=");
    expect(markup).not.toContain("border-border");
  });

  it("passes number bounds and file accept to inputs", () => {
    const numberMarkup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "input",
          id: "forms-try.age.input",
          inputKind: "number",
          value: "28",
          min: 13,
          max: 120,
          step: 1,
          onChange: { controllerId: "forms-play", action: "setTryValue" },
        },
        { onAction: noopAction },
      ),
    );
    expect(numberMarkup).toContain('min="13"');
    expect(numberMarkup).toContain('max="120"');
    const fileMarkup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "input",
          id: "forms-try.resume.input",
          inputKind: "file",
          value: "",
          accept: ".pdf,.doc",
          onChange: { controllerId: "forms-play", action: "setTryValue" },
        },
        { onAction: noopAction },
      ),
    );
    expect(fileMarkup).toContain('accept=".pdf,.doc"');
  });

  it("disables gated wizard buttons", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "button",
          id: "forms-try.next",
          iconId: "chevron-right",
          label: "Next",
          disabled: true,
          action: { controllerId: "forms-play", action: "nextStep" },
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain("disabled");
  });

  it("renders selectable builder cards with selection ring", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "stack",
          direction: "vertical",
          id: "forms-blueprint.card.q1",
          selected: true,
          activate: { controllerId: "forms-play", action: "setSelection" },
          children: [{ type: "text", value: "text · q1" }],
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain('data-ui-stack="forms-blueprint.card.q1"');
    expect(markup).toContain('role="button"');
    expect(markup).toContain("ring-primary");
  });

  it("renders image nodes from url sources", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "image",
          id: "forms-try.avatar.image",
          src: "https://example.com/avatar.png",
          alt: "Avatar",
        },
        { onAction: noopAction },
      ),
    );
    expect(markup).toContain('src="https://example.com/avatar.png"');
    expect(markup).toContain('alt="Avatar"');
  });

  it("dispatches the tree drop action with payload, target and position", async () => {
    const { declarativeTreeDragController } = await import("./index.tsx");
    const dispatched: unknown[] = [];
    const controller = declarativeTreeDragController(
      {
        type: "tree",
        sections: [{ id: "steps", items: [{ id: "forms-play-document.step.s1", label: "Inputs" }] }],
        dropAction: { controllerId: "forms-play", action: "dropQuestionKind" },
      },
      (action) => {
        dispatched.push(action);
      },
    );
    controller?.handleDrop?.({
      target: { id: "forms-play-document.step.s1", label: "Inputs" },
      targetKind: "item",
      data: {
        "application/vnd.code.tree.item": '["x"]',
        "application/x-semio-forms-question-kind": '{"kind":"slider"}',
      },
      sourceItems: [],
      section: { id: "steps", label: "Steps", items: [] },
      dropPosition: "after",
    });
    expect(dispatched).toEqual([
      {
        controllerId: "forms-play",
        action: "dropQuestionKind",
        args: { kind: "slider", targetId: "forms-play-document.step.s1", dropPosition: "after" },
      },
    ]);
  });
});

describe("framework renderer hosts", () => {
  it("renders node graph host from media graph scene json", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.media-graph",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodesJson: JSON.stringify([
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ]),
            edgesJson: "[]",
            viewportJson: '{"x":0,"y":0,"zoom":1}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("renders editable node graph host with find items", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.media-graph",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodesJson: JSON.stringify([
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ]),
            edgesJson: "[]",
            viewportJson: '{"x":0,"y":0,"zoom":1}',
            editable: true,
            findItemsJson: JSON.stringify([{ id: "app-a", label: "Draw", category: "Media graph" }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });

  it("uses the live session camera for node graph wheel viewport actions", () => {
    expect(nodeGraphViewportActionArgs('{"x":12,"y":24,"zoom":1.75}')).toEqual({
      viewportJson: '{"x":12,"y":24,"zoom":1.75}',
    });
  });

  it("parses slider overlay state json for flow graph hosts", () => {
    const sliders = parseDagSliderOverlays(
      JSON.stringify({
        camera: { x: 0, y: 0, zoom: 1 },
        sliders: [
          {
            widgetId: "slider_2",
            value: 2.2,
            min: 0,
            max: 10,
            step: 0.1,
            x: 100,
            y: 50,
            w: 120,
            h: 8,
          },
        ],
      }),
    );
    expect(sliders).toHaveLength(1);
    expect(sliders[0]?.widgetId).toBe("slider_2");
    expect(sliders[0]?.value).toBe(2.2);
  });

  it("renders canvas 2d host with infinite canvas session", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([{ id: "layer-1", name: "Layer 1", x: 0, y: 0, width: 120, height: 80 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });

  it("renders canvas 2d host with draw gradient/blend/overlay/meta scene records", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([
              { id: "meta:utility", role: "meta", utility: "selectDirect" },
              {
                id: "shape-1",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "line", to: [10, 10] }, { kind: "close" }],
                fill: {
                  kind: "linearGradient",
                  x1: 0,
                  y1: 0,
                  x2: 10,
                  y2: 10,
                  stops: [
                    { offset: 0, color: [1, 0, 0, 1] },
                    { offset: 1, color: [0, 0, 1, 1] },
                  ],
                },
                stroke: { color: [0, 0, 0, 1], width: 1, cap: "round", join: "round" },
                opacity: 1,
                blendMode: "multiply",
                visible: true,
                fillRule: "evenodd",
              },
              {
                id: "overlay:sel:shape-1",
                role: "overlay",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "close" }],
                fill: { kind: "solid", color: [0.98, 0.75, 0.14, 0.16] },
                stroke: { color: [0.98, 0.75, 0.14, 0.95], width: 2 },
                opacity: 1,
                blendMode: "normal",
                visible: true,
                fillRule: "evenodd",
              },
            ]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });

  it("renders puzzle 2d board host shell", () => {
    const markup = renderToStaticMarkup(
      createElement(Board2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle2d.play.composite.2d-overview",
          controllerId: "puzzle2d-play",
          componentKind: "board-2d",
          board2d: {
            fixtureJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            glyphCatalogsJson: "{}",
            selectionJson: "[]",
            interactive: true,
            selectionMethod: "rectangle",
            gridSnapEnabled: false,
            gridFactor: 1,
            suggestionOffset: 0,
            brushWeightsJson: "{}",
            placementCompatibilityJson: "[]",
            lodMode: "automatic",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-board-2d-host");
  });

  it("uses the live puzzle 2d board camera for wheel persistence actions", () => {
    expect(board2dCameraActionArgs('{"x":345,"y":-123,"zoom":4.25}')).toEqual({
      camera: { x: 345, y: -123, zoom: 4.25 },
    });
  });

  it("coalesces puzzle 2d board events: drops transients, keeps the latest camera, coalesces nodeMove per id", () => {
    const rows = [
      { name: "preselect", payload: { ids: ["a"] } },
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } },
      { name: "nodeMove", payload: { id: "alpha", x: 20, y: 20 } },
      { name: "nodeMove", payload: { id: "beta", x: 5, y: 5 } },
    ];
    const { flushNow, eventsJson } = coalesceBoard2dEvents(rows);
    const events = JSON.parse(eventsJson) as { name: string; payload: Record<string, unknown> }[];
    expect(flushNow).toBe(false);
    expect(events.find((event) => event.name === "preselect")).toBeUndefined();
    const cameraEvents = events.filter((event) => event.name === "camera");
    expect(cameraEvents).toHaveLength(1);
    expect(cameraEvents[0]?.payload).toEqual({ x: 2, y: 2, zoom: 1.5 });
    const alphaMoves = events.filter((event) => event.name === "nodeMove" && event.payload.id === "alpha");
    expect(alphaMoves).toHaveLength(1);
    expect(alphaMoves[0]?.payload).toEqual({ id: "alpha", x: 20, y: 20 });
  });

  it("coalesces puzzle 2d board events: drops nodeMove rows once a nodeDragEnd follows", () => {
    const rows = [
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "nodeDragEnd", payload: { moves: [{ id: "alpha", x: 20, y: 20 }] } },
    ];
    const { eventsJson } = coalesceBoard2dEvents(rows);
    const events = JSON.parse(eventsJson) as { name: string }[];
    expect(events.some((event) => event.name === "nodeMove")).toBe(false);
    expect(events.some((event) => event.name === "nodeDragEnd")).toBe(true);
  });

  it("flushes puzzle 2d board events immediately for select/brushPlace/edge/delete rows, not for camera/nodeMove alone", () => {
    expect(coalesceBoard2dEvents([{ name: "camera", payload: { x: 0, y: 0, zoom: 1 } }]).flushNow).toBe(false);
    expect(coalesceBoard2dEvents([{ name: "nodeMove", payload: { id: "alpha", x: 0, y: 0 } }]).flushNow).toBe(false);
    for (const name of ["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"]) {
      expect(coalesceBoard2dEvents([{ name, payload: {} }]).flushNow).toBe(true);
    }
  });

  it("collects live mirror ops: coalesces nodeMove to the latest per id, ignores unrelated rows", () => {
    const ops = collectPuzzle2dLiveMirrorOps([
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 1, y: 1 } },
      { name: "brushPreview", payload: {} },
      { name: "nodeMove", payload: { id: "alpha", x: 9, y: 9 } },
      { name: "nodeMove", payload: { id: "beta", x: 2, y: 2 } },
    ]);
    expect(ops.positions).toEqual([
      { id: "alpha", x: 9, y: 9 },
      { id: "beta", x: 2, y: 2 },
    ]);
    expect(ops.selectionIds).toBeNull();
    expect(ops.preselect).toBeNull();
    expect(ops.clearPreselect).toBe(false);
  });

  it("collects live mirror ops: nodeDragEnd.moves produce final positions", () => {
    const ops = collectPuzzle2dLiveMirrorOps([
      { name: "nodeMove", payload: { id: "alpha", x: 1, y: 1 } },
      {
        name: "nodeDragEnd",
        payload: {
          moves: [
            { id: "alpha", x: 20, y: 20 },
            { id: "beta", x: 5, y: 5 },
          ],
        },
      },
    ]);
    expect(ops.positions).toEqual([
      { id: "alpha", x: 20, y: 20 },
      { id: "beta", x: 5, y: 5 },
    ]);
  });

  it("collects live mirror ops: preselect sets the live highlight, select/preselectCancel commit selection and clear it", () => {
    expect(collectPuzzle2dLiveMirrorOps([{ name: "preselect", payload: { ids: ["a", "b"], removedIds: ["c"] } }])).toMatchObject({
      preselect: { ids: ["a", "b"], removedIds: ["c"] },
      clearPreselect: false,
      selectionIds: null,
    });
    expect(collectPuzzle2dLiveMirrorOps([{ name: "select", payload: { ids: ["a"] } }])).toMatchObject({
      selectionIds: ["a"],
      preselect: null,
      clearPreselect: true,
    });
    expect(collectPuzzle2dLiveMirrorOps([{ name: "preselectCancel", payload: { ids: ["a", "b"] } }])).toMatchObject({
      selectionIds: ["a", "b"],
      preselect: null,
      clearPreselect: true,
    });
  });

  it("peer registry: registers/unregisters, excludes own surfaceId and other controllerIds", () => {
    const peerA = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerB = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerOther = { session: {} as never, onPeerGestureEnded: () => {} };
    registerBoard2dPeer("puzzle2d-play", "pane.a", peerA);
    registerBoard2dPeer("puzzle2d-play", "pane.b", peerB);
    registerBoard2dPeer("other-controller", "pane.a", peerOther);

    expect(board2dPeers("puzzle2d-play", "pane.a")).toEqual([peerB]);
    expect(board2dPeers("puzzle2d-play", "pane.b")).toEqual([peerA]);
    expect(board2dPeers("other-controller", "pane.z")).toEqual([peerOther]);

    unregisterBoard2dPeer("puzzle2d-play", "pane.b");
    expect(board2dPeers("puzzle2d-play", "pane.a")).toEqual([]);

    unregisterBoard2dPeer("puzzle2d-play", "pane.a");
    unregisterBoard2dPeer("other-controller", "pane.a");
  });

  it("peer gesture ownership: begin/end tracks the owning surfaceId; a pane never defers against its own gesture", () => {
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.a")).toBe(false);
    beginPuzzle2dPeerGesture("puzzle2d-play", "pane.a");
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.a")).toBe(false);
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.b")).toBe(true);
    endPuzzle2dPeerGesture("puzzle2d-play", "pane.a");
    expect(puzzle2dPeerOwnsGesture("puzzle2d-play", "pane.b")).toBe(false);
  });

  it("pushes live mirror ops into peer sessions, skipping the source pane", () => {
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setNodePositionsJson: (json: string) => calls.push({ pane, method: "setNodePositionsJson", arg: json }),
        setSelectionIdsJsonSilent: (json: string) => calls.push({ pane, method: "setSelectionIdsJsonSilent", arg: json }),
        setPreselectStateJsonSilent: (json: string) => calls.push({ pane, method: "setPreselectStateJsonSilent", arg: json }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer("mirror-test", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer("mirror-test", "pane.sibling", makePeer("pane.sibling"));

    pushPuzzle2dLiveMirrorOps("mirror-test", "pane.source", {
      positions: [{ id: "alpha", x: 1, y: 2 }],
      selectionIds: ["alpha"],
      preselect: null,
      clearPreselect: false,
    });

    expect(calls).toEqual([
      { pane: "pane.sibling", method: "setNodePositionsJson", arg: JSON.stringify([{ id: "alpha", x: 1, y: 2 }]) },
      { pane: "pane.sibling", method: "setSelectionIdsJsonSilent", arg: JSON.stringify(["alpha"]) },
    ]);

    unregisterBoard2dPeer("mirror-test", "pane.source");
    unregisterBoard2dPeer("mirror-test", "pane.sibling");
  });

  it("notifies peers when a gesture ends, skipping the source pane, passing whether it flushed", () => {
    const ended: { pane: string; flushed: boolean }[] = [];
    registerBoard2dPeer("notify-test", "pane.source", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.source", flushed }) });
    registerBoard2dPeer("notify-test", "pane.sibling", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.sibling", flushed }) });

    notifyPuzzle2dPeersGestureEnded("notify-test", "pane.source", true);
    expect(ended).toEqual([{ pane: "pane.sibling", flushed: true }]);

    unregisterBoard2dPeer("notify-test", "pane.source");
    unregisterBoard2dPeer("notify-test", "pane.sibling");
  });

  it("builds a select-all menu when nothing is selected", () => {
    const items = buildPuzzle2dSelectionMenuItems(JSON.stringify({ nodes: [], edges: [] }), "[]");
    expect(items).toEqual([{ id: "selectAll", label: "Select all", action: "selectAll" }]);
  });

  it("builds the full selection menu with Hide/Lock/Duplicate/SelectSameKind/ZoomToSelection/Delete for a visible unlocked node", () => {
    const fixture = { nodes: [{ id: "alpha", nodeKind: "seed" }], edges: [] };
    const items = buildPuzzle2dSelectionMenuItems(JSON.stringify(fixture), JSON.stringify(["alpha"]));
    expect(items.map((item) => item.id)).toEqual(["toggleHidden", "toggleLocked", "duplicate", "selectSameKind", "focusSelection", "deleteSelection"]);
    expect(items.find((item) => item.id === "toggleHidden")).toMatchObject({ label: "Hide", args: { flag: "hidden", value: true } });
    expect(items.find((item) => item.id === "toggleLocked")).toMatchObject({ label: "Lock", args: { flag: "locked", value: true } });
    expect(items.find((item) => item.id === "duplicate")).toMatchObject({ disabled: false });
    expect(items.find((item) => item.id === "deleteSelection")).toMatchObject({ destructive: true });
  });

  it("flips the selection menu labels to Show/Unlock for an already hidden and locked node", () => {
    const fixture = { nodes: [{ id: "alpha", nodeKind: "seed", hidden: true, locked: true }], edges: [] };
    const items = buildPuzzle2dSelectionMenuItems(JSON.stringify(fixture), JSON.stringify(["alpha"]));
    expect(items.find((item) => item.id === "toggleHidden")).toMatchObject({ label: "Show", args: { flag: "hidden", value: false } });
    expect(items.find((item) => item.id === "toggleLocked")).toMatchObject({ label: "Unlock", args: { flag: "locked", value: false } });
  });

  it("disables Duplicate when the selection is only a handle, not a node", () => {
    const fixture = { nodes: [{ id: "alpha", nodeKind: "seed", handles: [{ id: "alpha:v0", handleKind: "port" }] }], edges: [] };
    const items = buildPuzzle2dSelectionMenuItems(JSON.stringify(fixture), JSON.stringify(["alpha:v0"]));
    expect(items.find((item) => item.id === "duplicate")).toMatchObject({ disabled: true });
  });

  it("parses a catalogue drag payload and builds a drop-preview JSON", () => {
    const encoded = JSON.stringify({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 });
    const payload = parsePuzzle2dCatalogueDragPayload(encoded);
    expect(payload).toEqual({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24, width: undefined, height: undefined, iconKind: undefined });
    expect(payload).not.toBeNull();
    expect(JSON.parse(puzzle2dFixtureDropPreviewJson(payload!, 100, 200))).toMatchObject({ nodeKind: "seed", screenX: 100, screenY: 200, shape: "circle", radius: 24 });
  });

  it("rejects a catalogue drag payload without a kindId", () => {
    expect(parsePuzzle2dCatalogueDragPayload(JSON.stringify({ catalogSlice: "nodes" }))).toBeNull();
    expect(parsePuzzle2dCatalogueDragPayload(null)).toBeNull();
  });

  it("parses a puzzle 3d catalogue drag payload and snaps drop origins to the grid", () => {
    const encoded = JSON.stringify({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(parsePuzzle3dCatalogueDragPayload(encoded)).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], true, 1)).toEqual([1, 3, 0]);
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], false, 1)).toEqual([1.2, 2.7, 0]);
    expect(parsePuzzle3dCatalogueDragPayload(JSON.stringify({ meshUrl: "puzzle3d://capsule" }))).toBeNull();
  });

  it("inverts the canonical screen-to-world transform for a fixture drop", () => {
    const cameraJson = JSON.stringify({ x: 120, y: 80, zoom: 2 });
    const world = puzzle2dScreenToWorld(cameraJson, { w: 800, h: 600 }, { x: 400, y: 300 });
    expect(world).toEqual({ x: 120, y: 80 });
  });

  it("maps a world-centered node inside the viewport with canonical camera math", () => {
    const camera = { x: 120, y: 80, zoom: 2 };
    const viewportWidth = 800;
    const viewportHeight = 600;
    const screen = worldToScreenLogical(120, 80, camera, viewportWidth, viewportHeight);
    expect(screen.x).toBeCloseTo(viewportWidth * 0.5, 5);
    expect(screen.y).toBeCloseTo(viewportHeight * 0.5, 5);
    const layersJson = JSON.stringify([
      {
        id: "node-a",
        kind: "circle",
        role: "node",
        color: "#336699",
        selected: true,
        x: 110,
        y: 70,
        width: 20,
        height: 20,
      },
    ]);
    expect(layersJson).toContain('"role":"node"');
    expect(layersJson).toContain('"selected":true');
  });

  it("renders world 3d empty state without mounting r3f canvas", () => {
    const markup = renderToStaticMarkup(
      createElement(World3dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle.play.world",
          controllerId: "puzzle-play",
          componentKind: "world-3d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-world-3d-empty");
  });

  it("keeps world-3d orbit camera seed local per viewport once detached", () => {
    const sceneCamera = '{"position":[1,2,3],"target":[0,0,0],"zoom":1}';
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).toBe(sceneCamera);
    expect(world3dViewportCameraSeedKey(sceneCamera, 1)).toBe("viewport:1");
    expect(world3dViewportCameraSeedKey(sceneCamera, 2)).toBe("viewport:2");
    expect(world3dViewportCameraSeedKey(sceneCamera, 0)).not.toBe(world3dViewportCameraSeedKey(sceneCamera, 1));
    expect(shouldReattachWorldViewportCamera(sceneCamera, sceneCamera)).toBe(false);
    expect(shouldReattachWorldViewportCamera(sceneCamera, '{"position":[9,9,9],"target":[0,0,0],"zoom":1}')).toBe(true);
    const merged = mergeWorldViewportCamera(
      { position: [1, 2, 3], target: [0, 0, 0], zoom: 1, projection: "perspective", fov: 45, explicitProjection: true, up: [0, 0, 1] },
      { position: [4, 5, 6], target: [1, 1, 1], zoom: 2, projection: "orthographic", up: [0, 1, 0] },
    );
    expect(merged.position).toEqual([4, 5, 6]);
    expect(merged.target).toEqual([1, 1, 1]);
    expect(merged.zoom).toBe(2);
    expect(merged.projection).toBe("orthographic");
    expect(merged.fov).toBe(45);
    expect(merged.explicitProjection).toBe(true);
    expect(merged.up).toEqual([0, 1, 0]);
  });

  it("accepts extended world 3d scene fields", () => {
    const node: UiNode = {
      type: "componentScene",
      surfaceId: "puzzle.3d.play.viewport",
      controllerId: "puzzle3d-play",
      componentKind: "world-3d",
      world3d: {
        cameraJson: "{}",
        meshesJson: "[]",
        instancesJson: "[]",
        selectionJson: "{}",
        vorticesJson: "[]",
        attractionsJson: "[]",
        targetVolumesJson: "[]",
        referencesJson: "[]",
        brushPreviewJson: undefined,
        interactionJson: '{"activeUtility":"select"}',
        engagementPreviewJson: '[{"kind":"point","role":"origin","position":[0,0,0]},{"kind":"box-preview","role":"preview","cornerA":[0,0,0],"cornerB":[2,2,0]}]',
        contextMenuJson: "[]",
        terrainJson: '{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":1.5,"colorRamp":"hypsometric","minZoom":6,"maxZoom":14}',
      },
    };
    expect(node.world3d?.meshesJson).toBe("[]");
    expect(node.world3d?.vorticesJson).toBe("[]");
    expect(node.world3d?.interactionJson).toContain("select");
    expect(node.world3d?.engagementPreviewJson).toContain("box-preview");
    expect(node.world3d?.contextMenuJson).toBe("[]");
    expect(node.world3d?.terrainJson).toContain("hypsometric");
  });

  it("parses GIS 3D terrain style JSON, defaulting missing fields, and rejects a missing tileUrlTemplate", () => {
    expect(parseWorldTerrainStyle(undefined)).toBeNull();
    expect(parseWorldTerrainStyle("not json")).toBeNull();
    expect(parseWorldTerrainStyle('{"projectOriginLon":1}')).toBeNull();
    const style = parseWorldTerrainStyle('{"tileUrlTemplate":"/dem/{z}/{x}/{y}.png","projectOriginLon":9.7382,"projectOriginLat":52.3759,"exaggeration":2}');
    expect(style).toMatchObject({
      tileUrlTemplate: "/dem/{z}/{x}/{y}.png",
      projectOriginLon: 9.7382,
      projectOriginLat: 52.3759,
      exaggeration: 2,
      colorRamp: "hypsometric",
      minZoom: 6,
      maxZoom: 14,
    });
  });

  it("blocks instance picking for fill and brush engagements but not move", () => {
    expect(worldInstancePickBlocked("brush")).toBe(true);
    expect(worldInstancePickBlocked("fill")).toBe(true);
    expect(worldInstancePickBlocked("move")).toBe(false);
    expect(worldInstancePickBlocked(undefined)).toBe(false);
  });

  it("resolves vortex pointer-down to select in brush or vertex mode and click-or-drag otherwise", () => {
    expect(resolveVortexPointerDownIntent(true)).toBe("select");
    expect(resolveVortexPointerDownIntent(false, "vertex")).toBe("select");
    expect(resolveVortexPointerDownIntent(false)).toBe("click-or-drag");
    expect(resolveVortexPointerDownIntent(false, "mesh")).toBe("click-or-drag");
  });

  it("revisions vortex materials when selection or hover state changes", () => {
    expect(worldVortexMaterialRevision()).toBe("neutral");
    expect(worldVortexMaterialRevision(false, true)).toBe("hovered");
    expect(worldVortexMaterialRevision(true, true)).toBe("selected");
  });

  it("revisions world mesh materials when style kind changes so deselection clears selected paint", () => {
    expect(worldMeshMaterialRevision("selected")).toBe("selected");
    expect(worldMeshMaterialRevision("neutral")).toBe("neutral");
    expect(worldMeshMaterialRevision("hovered")).toBe("hovered");
    expect(worldMeshMaterialRevision(resolveMeshStyle({ selected: false, hovered: false }))).toBe("neutral");
  });

  it("uses the world surface selection mode instead of a stale shared invertive mode", () => {
    const previousMode = (globalThis as any).__selectionMode;
    (globalThis as any).__selectionMode = "invertive";
    try {
      expect(resolveWorldSelectionMergeMode("default", {})).toBe("default");
      expect(resolveWorldSelectionMergeMode("default", { shiftKey: true })).toBe("additive");
      expect(resolveWorldSelectionMergeMode("invertive", {})).toBe("invertive");
    } finally {
      (globalThis as any).__selectionMode = previousMode;
    }
  });

  it("resolves mesh style by premigration priority: disabled > selected > highlighted > hovered > neutral", () => {
    expect(resolveMeshStyle({})).toBe("neutral");
    expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
    expect(resolveMeshStyle({ hovered: true, highlighted: true })).toBe("highlighted");
    expect(resolveMeshStyle({ highlighted: true, selected: true })).toBe("selected");
    expect(resolveMeshStyle({ selected: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ disabled: true, selected: true, highlighted: true, hovered: true })).toBe("disabled");
  });

  it("renders the new group selection as active and only objects leaving the old selection as highlighted", () => {
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, false)).toBe("highlighted");
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, false)).toBe("neutral");
    expect(resolveMeshSelectionPreviewStyle({ selected: true, disabled: true }, false)).toBe("disabled");
  });

  it("builds addBrushObject args from a parsed brush preview, or null when there is nothing to place", () => {
    expect(brushObjectPlacementArgs(null)).toBeNull();
    const args = brushObjectPlacementArgs({
      targetVortexFullId: "seed-left-001:v0",
      objectKindId: "hex-concrete",
      sourceVortexIndex: 2,
      origin: [1, 2, 3],
      orientation: [0, 0, 0, 1],
      scale: 1,
    });
    expect(args).toMatchObject({
      targetVortexFullId: "seed-left-001:v0",
      objectKindId: "hex-concrete",
      sourceVortexIndex: 2,
      origin: [1, 2, 3],
      orientation: [0, 0, 0, 1],
      scale: 1,
    });
  });

  it("defaults sourceVortexIndex to 0 when the brush preview omits it", () => {
    const args = brushObjectPlacementArgs({ targetVortexFullId: "seed-left-001:v0", objectKindId: "hex-concrete" });
    expect(args).toMatchObject({ sourceVortexIndex: 0 });
  });

  it("resolves the right-click context menu target by priority: vortex, then object, then reference", () => {
    expect(resolveWorldContextMenuTarget({ hoveredVortexFullId: "seed-left-001:v0" }, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({
      kind: "vortex",
      id: "seed-left-001:v0",
    });
    expect(resolveWorldContextMenuTarget({}, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "reference:ref-1" })).toEqual({ kind: "reference", id: "ref-1" });
    expect(resolveWorldContextMenuTarget({}, {})).toBeNull();
  });

  it("renders text editor host", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "hello",
            language: "jack",
            tokensJson: JSON.stringify([{ class: "ident", start: 0, end: 5 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
    expect(markup).toContain("hello");
  });

  it("renders text editor host with hover/newline/rename scene fields", () => {
    const markup = renderToStaticMarkup(
      createElement(TextEditorHost, {
        node: {
          type: "componentScene",
          surfaceId: "writer.play.editor",
          controllerId: "writer-play",
          componentKind: "text-editor",
          textEditor: {
            buffer: "MATCH (a:Piece) RETURN a.name",
            language: "jack",
            hoverJson: '{"start":0,"end":5}',
            newlineGatesJson: "[30]",
            renameJson: '{"name":"a","occurrences":[{"start":7,"end":8}]}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-text-editor-host");
  });

  it("buildTextEditorContextMenuItems prepends suggest when completions are available", () => {
    const items = buildTextEditorContextMenuItems(
      { canSuggest: true, hasSelection: false, canRename: false, pickTargets: [] },
      {
        suggest: () => {},
        selectToken: () => {},
        selectLine: () => {},
        selectAll: () => {},
        rename: () => {},
        cut: () => {},
        copy: () => {},
        paste: () => {},
        format: () => {},
        lint: () => {},
        pickTarget: () => {},
      },
    );
    expect(items[0]?.id).toBe("writer-suggest");
    expect(items[0]?.label).toBe("Suggest completions");
  });

  it("buildTextEditorContextMenuItems includes pick rows when multiple targets overlap", () => {
    const items = buildTextEditorContextMenuItems(
      {
        canSuggest: false,
        hasSelection: false,
        canRename: false,
        pickTargets: [
          { domain: "line", id: "0", label: "Line 1" },
          { domain: "token", id: "0:5", label: "MATCH" },
        ],
      },
      {
        suggest: () => {},
        selectToken: () => {},
        selectLine: () => {},
        selectAll: () => {},
        rename: () => {},
        cut: () => {},
        copy: () => {},
        paste: () => {},
        format: () => {},
        lint: () => {},
        pickTarget: () => {},
      },
    );
    expect(items.some((item) => item.id === "writer-pick-token-0:5")).toBe(true);
  });

  it("multiSpanReplace renames every occurrence and remaps spans", () => {
    const result = multiSpanReplace(
      "MATCH (a:Piece) RETURN a.name",
      [
        { start: 7, end: 8 },
        { start: 23, end: 24 },
      ],
      "piece",
    );
    expect(result.text).toBe("MATCH (piece:Piece) RETURN piece.name");
    expect(result.occurrences).toEqual([
      { start: 7, end: 12 },
      { start: 23, end: 28 },
    ]);
  });

  it("lineRangeAt finds the line containing an offset", () => {
    const text = "MATCH (a)\nWHERE a.x = 1\nRETURN a";
    const range = lineRangeAt(text, 15);
    expect(text.slice(range.start, range.end)).toBe("WHERE a.x = 1");
  });

  it("renders table host with ui-react table", () => {
    const markup = renderToStaticMarkup(
      createElement(TableHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.catalogue",
          controllerId: "s-play",
          componentKind: "table",
          table: {
            columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
            rowsJson: JSON.stringify([{ label: "Draw" }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-table-host");
    expect(markup).toContain("Draw");
  });

  it("renders vcs history host with an ancestor graph fork", () => {
    const columns = [
      {
        checkpointId: "c3",
        timestamp: "3",
        labels: ["feature-b"],
        authors: [],
        parentCheckpointId: "c2",
        description: "branch b",
        lane: 2,
        alternativeIds: ["b"],
      },
      {
        checkpointId: "c2",
        timestamp: "2",
        labels: ["feature-a"],
        authors: [],
        parentCheckpointId: "c1",
        description: "branch a",
        lane: 1,
        alternativeIds: ["a"],
      },
      {
        checkpointId: "c1",
        timestamp: "1",
        labels: ["main"],
        authors: [],
        description: "root",
        lane: 0,
        alternativeIds: [],
      },
    ];
    const markup = renderToStaticMarkup(
      createElement(GraphTimelineHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history",
          controllerId: "vcs-play",
          componentKind: "graph-timeline",
          graphTimeline: {
            columnsJson: JSON.stringify(columns),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-graph-timeline-host");
    expect(markup).toContain("graph-timeline-table");
    expect(markup).toContain('d="M ');
    expect(markup.match(/<circle /g)?.length).toBe(3);
    expect(markup).toContain("branch b");
    expect(markup).toContain("feature-b");
  });

  it("renders paint-2d host canvas surface from document sync scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-surface-id="raster.play.viewport"');
    expect(markup).toContain('data-view-mode="composite"');
  });

  it("renders paint-2d navigator host with the composite viewport overlay channel", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.navigator",
          controllerId: "raster-play",
          componentKind: "paint-2d",
          paint2d: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeUtility: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            viewMode: "navigator",
            compositeViewportJson: '{"width":640,"height":480}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-canvas-surface");
    expect(markup).toContain('data-view-mode="navigator"');
  });

  it("renders paint-2d host empty fallback without a scene", () => {
    const markup = renderToStaticMarkup(
      createElement(Paint2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.composite",
          controllerId: "raster-play",
          componentKind: "paint-2d",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-paint-2d-empty");
  });

  it("interprets virtual file system component scenes", () => {
    const markup = renderToStaticMarkup(
      interpretUiNode(
        {
          type: "componentScene",
          surfaceId: "s.play.media-vfs",
          controllerId: "s-play",
          componentKind: "virtualFileSystem",
          virtualFileSystem: {
            schemaJson: JSON.stringify({
              fileNodeKinds: {
                instance: { id: "instance", name: "Instance", descriptors: [] },
              },
              descriptorKinds: {},
              descriptorColumnIds: [],
            }),
            rowsJson: JSON.stringify([
              {
                id: "row-1",
                fileNodeKindId: "instance",
                name: "Draw",
                path: "/draw",
                level: 0,
              },
            ]),
          },
        },
        { onAction: noopAction },
      ) as ReactElement,
    );
    expect(markup).toContain("Draw");
  });
});

describe("dag marquee overlay", () => {
  it("computes a rect overlay with numeric bounds for the rectangle method", () => {
    const pointsJson = JSON.stringify([
      { x: 10, y: 20 },
      { x: 30, y: 50 },
    ]);
    const overlay = computeDagMarqueeOverlay(pointsJson, false, "rectangle");
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "full" });
  });

  it("computes a lasso overlay carrying the raw points for the lasso method", () => {
    const points = [
      { x: 10, y: 20 },
      { x: 30, y: 50 },
      { x: 15, y: 40 },
    ];
    const overlay = computeDagMarqueeOverlay(JSON.stringify(points), true, "lasso");
    expect(overlay).toEqual({ kind: "lasso", points, coverage: "partial" });
  });

  it("returns null for fewer than two points", () => {
    expect(computeDagMarqueeOverlay(JSON.stringify([{ x: 0, y: 0 }]), false, "rectangle")).toBeNull();
  });

  // Regression: node-graph-host.tsx used to pass `shape={{ shape: "polygon", points }}` (a single
  // nested-object prop) instead of separate `shape`/`points` props, so `props.shape === "rect"` was
  // always false and the polygon branch read `props.points` as undefined — crashing on every marquee
  // drag and tripping the shell's render error boundary (visible as an interaction "reset").
  it("renders a rect overlay from a computeDagMarqueeOverlay rect result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        { x: 0, y: 0 },
        { x: 40, y: 25 },
      ]),
      false,
      "rectangle",
    );
    if (!overlay || overlay.kind !== "rect") throw new Error("expected rect overlay");
    const markup = renderToStaticMarkup(
      createElement(SelectionMarquee, {
        coverage: overlay.coverage ?? "full",
        shape: "rect",
        rect: { x: overlay.x ?? 0, y: overlay.y ?? 0, width: overlay.width ?? 0, height: overlay.height ?? 0 },
      }),
    );
    expect(markup).toContain("<rect");
  });

  it("renders a polygon overlay from a computeDagMarqueeOverlay lasso result without crashing", () => {
    const overlay = computeDagMarqueeOverlay(
      JSON.stringify([
        { x: 0, y: 0 },
        { x: 40, y: 25 },
        { x: 5, y: 30 },
      ]),
      false,
      "lasso",
    );
    if (!overlay || overlay.kind !== "lasso") throw new Error("expected lasso overlay");
    const markup = renderToStaticMarkup(createElement(SelectionMarquee, { coverage: overlay.coverage ?? "full", shape: "polygon", points: overlay.points ?? [] }));
    expect(markup).toContain("<polygon");
  });
});

describe("ink canvas host", () => {
  const semioInkDocument: InkDocument = {
    schema: "ink.document",
    id: "semio",
    title: "Semio Note",
    camera: { x: 0, y: 0, zoom: 1 },
    activeUtility: "selectDirect",
    gridVisible: true,
    snapEnabled: false,
    pencilWidth: 3,
    eraserRadius: 12,
    blocks: [
      {
        kind: "text",
        id: "welcome-text",
        name: "Welcome",
        x: 80,
        y: 80,
        width: 360,
        height: 120,
        visible: true,
        locked: false,
        paragraphs: [{ runs: [{ text: "Welcome to Note — an infinite canvas for text, images, tables, math, and pencil ink." }] }],
        fontSize: 20,
        fontWeight: "normal",
        align: "left",
      },
      { kind: "math", id: "welcome-math", name: "Equation", x: 80, y: 240, width: 240, height: 80, visible: true, locked: false, tex: "E = mc^2", displayMode: true },
      {
        kind: "table",
        id: "welcome-table",
        name: "Blocks",
        x: 80,
        y: 360,
        width: 360,
        height: 140,
        visible: true,
        locked: false,
        columns: ["Block", "Description"],
        rows: [
          [{ content: "Text" }, { content: "Rich text blocks" }],
          [{ content: "Math" }, { content: "TeX equations" }],
          [{ content: "Ink" }, { content: "Freehand pencil strokes" }],
        ],
      },
    ],
  };

  it("renders the semio example composite scene with rich text, table, and math fallback", () => {
    const markup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: {
          type: "componentScene",
          surfaceId: "note.play.composite",
          controllerId: "note-play",
          componentKind: "ink-canvas",
          inkCanvas: {
            documentJson: JSON.stringify(semioInkDocument),
            selectionJson: "[]",
            activeUtility: "selectDirect",
            viewMode: "composite",
            interactive: true,
          },
        },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(markup).toContain("Welcome to Note");
    expect(markup).toContain("<table");
    expect(markup).toMatch(/\$\$E = mc\^2\$\$|annotation encoding="application\/x-tex">E = mc\^2</);
    expect(markup).toContain('data-surface-id="note.play.composite"');
  });

  it("shows the grid pattern in composite mode but not in navigator mode", () => {
    const baseNode = {
      type: "componentScene" as const,
      surfaceId: "note.play.composite",
      controllerId: "note-play",
      componentKind: "ink-canvas",
    };
    const compositeMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "composite", interactive: true } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(compositeMarkup).toContain("ink-viewport-grid");

    const navigatorMarkup = renderToStaticMarkup(
      createElement(InkCanvasHost, {
        node: { ...baseNode, inkCanvas: { documentJson: JSON.stringify(semioInkDocument), selectionJson: "[]", activeUtility: "selectDirect", viewMode: "navigator", interactive: false } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(navigatorMarkup).not.toContain("ink-viewport-grid");
  });

  it("resizes with a minimum size and scales ink points when a group is resized", () => {
    const fromBounds = { x: 0, y: 0, width: 100, height: 100 };
    const shrunk = inkResizeBounds(fromBounds, "e", -1000, 0);
    expect(shrunk.width).toBe(8);

    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 100,
      height: 100,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [100, 100],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const scaled = inkScaleItemWithinGroup(ink, { x: 0, y: 0, width: 100, height: 100 }, { x: 0, y: 0, width: 200, height: 50 });
    expect(scaled.kind).toBe("stroke");
    if (scaled.kind === "stroke")
      expect(scaled.points).toEqual([
        [0, 0],
        [200, 50],
      ]);
  });

  it("splits an ink stroke into fragments when erasing its middle point", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 0,
      y: 0,
      width: 80,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [40, 0],
        [80, 0],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    const fragments = eraseInkStrokePointsInItem(ink, 40, 0, 5);
    expect(fragments).toHaveLength(0);
    const wideStroke: InkStrokeItem = {
      ...ink,
      points: [
        [0, 0],
        [10, 0],
        [40, 0],
        [70, 0],
        [80, 0],
      ],
    };
    const splitFragments = eraseInkStrokePointsInItem(wideStroke, 40, 0, 5);
    expect(splitFragments).toHaveLength(2);
  });

  it("round-trips bold and link marks between paragraphs and html", () => {
    const html = inkParagraphsToHtml([{ runs: [{ text: "hello", bold: true, link: "https://semio.tech" }] }]);
    expect(html).toContain("<strong>");
    expect(html).toContain('href="https://semio.tech"');
  });

  it("round-trips a clipboard payload of note blocks", () => {
    const payload = inkClipboardPayload([semioInkDocument.blocks[1]!]);
    const parsed = inkItemsFromClipboardPayload(payload);
    expect(parsed).toHaveLength(1);
    expect(parsed?.[0]?.kind).toBe("math");
  });

  it("computes ink block bounds from its local points", () => {
    const ink: InkStrokeItem = {
      kind: "stroke",
      id: "ink-1",
      name: "Ink",
      x: 10,
      y: 10,
      width: 1,
      height: 1,
      visible: true,
      locked: false,
      points: [
        [0, 0],
        [5, 5],
      ],
      strokeWidth: 2,
      color: [0, 0, 0, 1],
    };
    expect(inkItemBounds(ink)).toEqual({ x: 10, y: 10, width: 5, height: 5 });
  });

  it("applies the canonical wheel-zoom camera formula symmetrically for screen<->world conversion", () => {
    const camera = { x: 50, y: 50, zoom: 2 };
    const world = screenToWorld(camera, 150, 150);
    expect(world).toEqual([50, 50]);
    expect(worldToScreen(camera, 50, 50)).toEqual({ x: 150, y: 150 });
  });
});

describe("spawned window chrome", () => {
  const app = {
    id: "cad-play",
    label: "CAD",
    document: ["semio", "cad"],
    controllerId: "cad-play",
    defaultModeId: "edit",
    modes: [
      {
        id: "edit",
        label: "Edit",
        utilities: [{ id: "static-utility", kind: "button" as const, iconId: "save", controllerId: "cad-play", action: "save" }],
      },
    ],
    windowKinds: [
      {
        id: "cad-window-shape",
        label: "Shape",
        bodyKey: "shape",
        options: {
          engagement: {
            kind: "some" as const,
            value: {
              input: {
                id: "engagement-input",
                placeholder: "Action",
                onChange: { controllerId: "cad-play", action: "engagementInput" },
              },
              possibleEngagements: [{ id: "box", label: "Box", action: { controllerId: "cad-play", action: "startBox" } }],
            },
          },
          measures: [{ id: "render-mode", kind: "select" as const, label: "Render Mode", value: "shaded", items: [], onChange: { controllerId: "cad-play", action: "setRenderMode" } }],
        },
      },
    ],
    panelTabs: [],
    keybindings: [],
  };

  it("builds spawned engagement and measures chrome from plugin contributions", () => {
    const kind = app.windowKinds[0]!;
    const engagements = {
      [kind.id]: {
        input: {
          id: "engagement-input",
          value: "Box",
          placeholder: "Action",
          onChange: { controllerId: "cad-play", action: "engagementInput" },
        },
        possibleEngagements: [{ id: "box", label: "Box", detail: "b", action: { controllerId: "cad-play", action: "startBox" } }],
      },
    };
    const measures = { [kind.id]: kind.options.measures ?? [] };
    const chrome = spawnedWindowChromeForKind(kind, engagements, measures, undefined, noopAction);
    expect(chrome.search?.input?.value).toBe("Box");
    expect(chrome.search?.possibles?.[0]?.label).toBe("Box");
    const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
    expect(measuresMarkup).toContain("Render Mode");
  });
});

describe("partitionWindowMeasures", () => {
  const utilityGroup = (id: string, activeUtilityId?: string): WindowMeasure => ({ kind: "group", id, label: id, activeUtilityId, children: [] });
  const slider = (id: string): WindowMeasure => ({ kind: "slider", id, value: 1, min: 0, max: 2, onChange: { controllerId: "c", action: "a" } });

  it("routes a tagged group to utilityOptions only when its utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush"), slider("zoom")];
    const active = partitionWindowMeasures(measures, "brush");
    expect(active.utilityOptions.map((m) => m.id)).toEqual(["brush-params"]);
    expect(active.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("drops a tagged group from both buckets when a different or no utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush"), slider("zoom")];
    const other = partitionWindowMeasures(measures, "fill");
    expect(other.utilityOptions).toEqual([]);
    expect(other.general.map((m) => m.id)).toEqual(["zoom"]);
    const none = partitionWindowMeasures(measures, undefined);
    expect(none.utilityOptions).toEqual([]);
    expect(none.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("keeps untagged groups and non-group measures in general, unaffected by the active utility", () => {
    const measures = [utilityGroup("grid"), slider("zoom")];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "brush");
    expect(general.map((m) => m.id)).toEqual(["grid", "zoom"]);
    expect(utilityOptions).toEqual([]);
  });

  it("wires a utility-scoped group into spawnedWindowChromeForKind's utilityOptions slot only when its utility is active", () => {
    const kind = { id: "w", label: "W", bodyKey: "b", surfaceKind: "paint-2d", options: { engagement: { kind: "none" as const }, measures: [] } } as unknown as AppWindowKindDefinition;
    const brushGroup: WindowMeasure = {
      kind: "group",
      id: "brush-params",
      label: "Brush",
      defaultOpen: true,
      activeUtilityId: "brush",
      children: [{ kind: "slider", id: "size", label: "Brush size", value: 4, min: 1, max: 10, onChange: { controllerId: "c", action: "setSize" } }],
    };
    const measures = { [kind.id]: [brushGroup] };
    const activeChrome = spawnedWindowChromeForKind(kind, {}, measures, "brush", noopAction);
    const activeMarkup = renderToStaticMarkup(activeChrome.utilityOptions as ReactElement);
    expect(activeMarkup).toContain("Brush size");
    expect(activeMarkup).toContain('data-direction="up"');
    expect(activeChrome.measures).toBeUndefined();
    const idleChrome = spawnedWindowChromeForKind(kind, {}, measures, "fill", noopAction);
    expect(idleChrome.utilityOptions).toBeUndefined();
    expect(idleChrome.measures).toBeUndefined();
  });

  it("parses the real ui_wgpu camelCase wire JSON and routes a utility-scoped fill group into utilityOptions (snake_case divergence regression guard)", () => {
    // Verbatim shape of `ui_wgpu::WindowMeasure`'s serde wire after the D-4 `rename_all_fields = "camelCase"`
    // fix: a fill-utility slider group tagged with `activeUtilityId`, plus an untagged toggle. This is the exact
    // class of payload whose snake_case↔camelCase divergence made the puzzle fill slider invisible in React.
    const wireJson =
      '[{"kind":"group","id":"fill-params","label":"Fill","activeUtilityId":"fill","children":[{"kind":"slider","id":"fillCount","label":"Count","value":3,"min":1,"max":9,"step":1,"onChange":{"controllerId":"puzzle","action":"setFillCount"}}]},{"kind":"toggle","id":"grid","iconId":"icon.grid","pressed":true,"onChange":{"controllerId":"puzzle","action":"toggleGrid"}}]';
    const measures = JSON.parse(wireJson) as WindowMeasure[];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "fill");
    expect(utilityOptions.map((m) => m.id)).toEqual(["fill-params"]);
    const fillGroup = utilityOptions[0];
    expect(fillGroup.kind).toBe("group");
    if (fillGroup.kind === "group") {
      expect(fillGroup.activeUtilityId).toBe("fill");
      expect(fillGroup.children[0]).toMatchObject({ kind: "slider", id: "fillCount", onChange: { action: "setFillCount" } });
    }
    expect(general.map((m) => m.id)).toEqual(["grid"]);
    const gridToggle = general[0];
    expect(gridToggle.kind === "toggle" && gridToggle.iconId).toBe("icon.grid");

    // Regression guard for the fixed bug: the pre-fix snake_case wire leaves `activeUtilityId` undefined, so the
    // tagged group silently falls through to `general` and the fill slider never reaches the Utility Options rail.
    const legacyJson = wireJson
      .replace(/"activeUtilityId"/g, '"active_utility_id"')
      .replace(/"onChange"/g, '"on_change"')
      .replace(/"iconId"/g, '"icon_id"');
    const legacy = JSON.parse(legacyJson) as WindowMeasure[];
    const legacyPartition = partitionWindowMeasures(legacy, "fill");
    expect(legacyPartition.utilityOptions).toEqual([]);
    expect(legacyPartition.general.map((m) => m.id)).toEqual(["fill-params", "grid"]);
  });
});

describe("utility ribbon", () => {
  it("sorts utility nodes by order", () => {
    const sorted = sortUtilityNodes([
      { id: "b", kind: "button", iconId: "box", order: 2, controllerId: "x", action: "b" },
      { id: "a", kind: "button", iconId: "box", order: 1, controllerId: "x", action: "a" },
    ]);
    expect(sorted.map((node) => node.id)).toEqual(["a", "b"]);
  });

  it("recurses into a collection level only when the path names one of its collections", () => {
    const tree = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [
          {
            id: "view-tools",
            kind: "collection",
            iconId: "zoom-in",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
        ],
      },
      {
        id: "construct",
        kind: "collection",
        iconId: "box",
        children: [
          {
            id: "construct-tools",
            kind: "collection",
            iconId: "box",
            children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
          },
        ],
      },
    ];

    const noActive = buildUtilityRibbonSegments(tree, []);
    expect(noActive).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);

    const oneActive = buildUtilityRibbonSegments(tree, ["construct"]);
    expect(oneActive[0]).toMatchObject({ kind: "picker", depth: 0 });
    expect(oneActive[1]).toMatchObject({ kind: "picker", depth: 1, collections: tree[1].children });
    expect(oneActive).toHaveLength(2);

    const twoActive = buildUtilityRibbonSegments(tree, ["construct", "construct-tools"]);
    const utilitiesSegment = twoActive.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "box"));
    expect(utilitiesSegment).toMatchObject({ depth: 2 });
  });

  it("ignores a path entry that no longer names an enabled collection at that level", () => {
    const tree = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
      },
    ];
    expect(buildUtilityRibbonSegments(tree, ["nonexistent"])).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);
  });

  it("emits a picker segment alongside loose leaves at the same depth", () => {
    const segments = buildUtilityRibbonSegments(
      [
        { id: "undo", kind: "button", iconId: "undo", controllerId: "x", action: "undo" },
        {
          id: "view",
          kind: "collection",
          iconId: "eye",
          children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
        },
      ],
      [],
    );
    expect(segments).toEqual([
      { kind: "picker", collections: [expect.objectContaining({ id: "view" })], depth: 0 },
      { kind: "utilities", items: [expect.objectContaining({ id: "undo" })], depth: 0 },
    ]);
  });

  it("reconciles an active path by truncating at the first stale entry instead of substituting a default", () => {
    const tree = [
      {
        id: "a",
        kind: "collection",
        iconId: "box",
        children: [
          { id: "x", kind: "collection", iconId: "box", children: [{ id: "leaf", kind: "button", iconId: "box", controllerId: "c", action: "act" }] },
          { id: "y", kind: "collection", iconId: "box", children: [] },
        ],
      },
      { id: "b", kind: "collection", iconId: "box", children: [] },
    ];
    expect(reconcileUtilityPath(tree, ["a", "x"])).toEqual(["a", "x"]);
    expect(reconcileUtilityPath(tree, ["a", "gone"])).toEqual(["a"]);
    expect(reconcileUtilityPath(tree, ["gone"])).toEqual([]);
    expect(reconcileUtilityPath(tree, [])).toEqual([]);
  });

  it("buckets top-level utility nodes into ordered category collections (uncategorized nodes default to tools now that the Actions category is gone)", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "sel", kind: "toggle", iconId: "cursor", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
      { id: "act", kind: "button", iconId: "wand", controllerId: "x", action: "run" },
      { id: "tool", kind: "toggle", iconId: "pen", controllerId: "x", action: "pen" },
      { id: "sync", kind: "toggle", iconId: "cloud", controllerId: "x", action: "sync", category: "sync" },
    ]);
    expect(grouped.map((node) => node.id)).toEqual(["selection", "utilities", "history", "sync"]);
    expect(grouped.every((node) => node.kind === "collection")).toBe(true);
  });

  it("drops separator-only category buckets so an empty group never appears as a picker option", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "sep", kind: "separator" },
    ]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0].id).toBe("utilities");
  });

  it("reuses a category's single already-meaningful collection instead of re-wrapping it, avoiding a duplicate-looking picker level", () => {
    const selectionCollection = {
      id: "lowpoly-tools-selection",
      kind: "collection" as const,
      iconId: "mouse-pointer",
      label: "Selection",
      category: "selection" as const,
      children: [{ id: "mesh", kind: "toggle" as const, iconId: "box", controllerId: "x", action: "mesh" }],
    };
    const grouped = groupUtilityNodesByCategory([selectionCollection]);
    expect(grouped).toEqual([{ ...selectionCollection, order: 0 }]);
    const segments = buildUtilityRibbonSegments(grouped, ["lowpoly-tools-selection"]);
    const utilitiesSegment = segments.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "mesh"));
    expect(utilitiesSegment).toBeTruthy();
  });

  it("still wraps a category with multiple top-level nodes in a synthetic collection", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "b", kind: "button", iconId: "box", controllerId: "x", action: "b", category: "utilities" },
    ]);
    expect(grouped).toEqual([{ id: "utilities", kind: "collection", iconId: "wrench", text: "utilities", order: 0, category: "utilities", children: expect.any(Array) }]);
  });

  it("scopes grouping to the given categories only", () => {
    const nodes = [
      { id: "sel", kind: "toggle", iconId: "cursor", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
    ];
    expect(groupUtilityNodesByCategory(nodes, ["selection", "utilities"]).map((node) => node.id)).toEqual(["selection"]);
    expect(groupUtilityNodesByCategory(nodes, ["utilities", "history"]).map((node) => node.id)).toEqual(["history"]);
  });

  it("deduplicates utility nodes by id across window utility lists for a single shared footer entry", () => {
    const history = { id: "s-play.history", kind: "collection" as const, iconId: "clock", category: "history" as const, children: [] };
    const deduped = dedupeUtilityNodesById([[history, { id: "leaf-a", kind: "button" as const, iconId: "box", controllerId: "x", action: "a" }], [history], []]);
    expect(deduped).toEqual([history, { id: "leaf-a", kind: "button", iconId: "box", controllerId: "x", action: "a" }]);
  });

  it("renders utility ribbon with picker and batched toggles", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [
              {
                id: "view-tools",
                kind: "collection",
                iconId: "eye",
                children: [
                  { id: "show-edges", kind: "toggle", iconId: "box", pressed: true, controllerId: "x", action: "edges" },
                  { id: "show-faces", kind: "toggle", iconId: "square", pressed: false, controllerId: "x", action: "faces" },
                ],
              },
            ],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities"');
    expect(markup).toContain('data-slot="toggle-group"');
  });

  it("stacks the window utility bar ribbon upward, showing only the base picker row until a group is activated", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        direction: "up",
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-slot="ribbon"');
    expect(markup).toContain('data-direction="up"');
    expect(markup).toContain("flex-col-reverse");
    // No active path given, so neither group is expanded: exactly one ribbon row (the base picker).
    expect(markup.match(/data-slot="ribbon-row"/g)?.length).toBe(1);
    expect(markup).toContain('data-slot="toggle-group"');
    expect(markup).not.toContain('id="zoom-in"');
  });

  it("renders UtilityTree with a custom id for per-window namespacing", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.model",
        utilities: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities.model"');
    expect(markup).not.toContain('id="ui.utilities"');
  });

  it("renders utilityOptions as an extra ribbon row when direction is up", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.w",
        direction: "up",
        utilities: [{ id: "brush", kind: "toggle", iconId: "brush", pressed: true, controllerId: "x", action: "setActiveUtility" }],
        utilityOptions: createElement("span", { "data-testid": "brush-options" }, "Brush size"),
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-testid="brush-options"');
    expect(markup).toContain("Brush size");
    expect(markup).toContain('data-variable-height="true"');
    expect(markup).toContain("h-auto min-h-medium");
    expect(markup).toContain("items-start");
    const variableZoneClass = markup.match(/data-variable-height="true" class="([^"]*)"/)?.[1].split(" ") ?? [];
    expect(variableZoneClass).toContain("h-auto");
    expect(variableZoneClass).not.toContain("h-medium");
    expect(markup).toContain("h-auto items-start");
  });
});

describe("s media graph flow routing", () => {
  it("selects the flow engine for scenes with engine flow capabilities", () => {
    expect(isFlowGraphScene('{"engine":"flow","spotlight":false,"noteEdit":false}')).toBe(true);
    expect(isFlowGraphScene('{"spotlight":false,"noteEdit":false,"clusters":false}')).toBe(false);
    expect(isFlowGraphScene(undefined)).toBe(false);
  });

  it("renders presence peers from the scene payload", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.media-graph",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodesJson: "[]",
            edgesJson: "[]",
            viewportJson: '{"x":0,"y":0,"zoom":1}',
            presencePeersJson: JSON.stringify([{ clientId: "client-b", name: "Ada", selectionCount: 2 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("Ada");
    expect(markup).toContain("2 selected");
  });

  it("parses a catalogue app drag payload, ignoring extra keys", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ programId: "s.system", appId: "draw", label: "Draw", extra: "x" }))).toEqual({
      programId: "s.system",
      appId: "draw",
      label: "Draw",
    });
  });

  it("rejects catalogue app drag payloads missing programId/appId, and garbage", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ appId: "draw" }))).toBeNull();
    expect(parseCatalogueAppDragPayload(JSON.stringify({ kind: "neuron" }))).toBeNull();
    expect(parseCatalogueAppDragPayload("not json")).toBeNull();
  });

  it("builds a ghost neuron descriptor, preferring label over appId", () => {
    expect(JSON.parse(catalogueGhostDescriptorJson({ programId: "s.system", appId: "draw", label: "Draw" }))).toEqual({ kind: "neuron", neuronKind: "Draw" });
    expect(JSON.parse(catalogueGhostDescriptorJson({ programId: "s.system", appId: "draw" }))).toEqual({ kind: "neuron", neuronKind: "draw" });
  });

  it("attaches a drag-and-drop controller to tree panels whose items carry drag data", () => {
    const config = uiNodeToTreePanelConfig(
      {
        type: "tree",
        sections: [
          {
            id: "catalogue",
            label: "Catalogue",
            items: [{ id: "s-play-catalogue.document.draw", label: "Draw", draggable: true, dragData: { "application/x-semio-catalogue-item": '{"programId":"s.system","appId":"draw"}' } }],
          },
        ],
      },
      noopAction,
    );
    expect(config.dragAndDropController).toBeDefined();
  });

  it("omits the drag-and-drop controller for tree panels without drag data", () => {
    const config = uiNodeToTreePanelConfig({ type: "tree", sections: [{ id: "catalogue", label: "Catalogue", items: [{ id: "s-play-catalogue.document.draw", label: "Draw" }] }] }, noopAction);
    expect(config.dragAndDropController).toBeUndefined();
  });

  it("resolves a fixture widget id to its media-graph instance id, independent of selection state", () => {
    const fixtureJson = JSON.stringify({
      widgets: [
        { id: "widget-1", params: { instanceId: "app-1" } },
        { id: "widget-2", params: {} },
      ],
    });
    expect(resolveFixtureWidgetInstanceId(fixtureJson, "widget-1")).toBe("app-1");
    expect(resolveFixtureWidgetInstanceId(fixtureJson, "widget-2")).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId(fixtureJson, "missing-widget")).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId(fixtureJson, undefined)).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId(undefined, "widget-1")).toBeUndefined();
    expect(resolveFixtureWidgetInstanceId("not json", "widget-1")).toBeUndefined();
  });

  it("parses studio and studio+instance shell paths, and rejects non-studio routes", () => {
    expect(parseStudioShellPath("/studios/my-studio")).toEqual({ studioId: "my-studio", instanceId: undefined });
    expect(parseStudioShellPath("/studios/my-studio/instances/inst-1")).toEqual({ studioId: "my-studio", instanceId: "inst-1" });
    expect(parseStudioShellPath("/")).toBeNull();
    expect(parseStudioShellPath("/studios/my-studio/instances/inst-1/extra")).toBeNull();
  });
});

describe("ui search/find (fuse re-export from @semio-tech/ui-react)", () => {
  // Command dialogs render via a Radix Portal into `document.body`, not into the render() container, so assertions query `document.body`.
  // This package's vitest config has no shared setupFile, so tests here clean up their own portal-rendered DOM.
  afterEach(async () => {
    const { cleanup } = await import("@testing-library/react");
    cleanup();
  });

  it("UISearch renders all items and fuzzy-filters them via the shared Fuse re-export", async () => {
    const { render, fireEvent } = await import("@testing-library/react");
    const items: UISearchItem[] = [
      { id: "a", label: "Alpha", category: "Test", onSelect: noopAction },
      { id: "b", label: "Bravo", category: "Test", onSelect: noopAction },
    ];
    render(createElement(UIFindProvider, null, createElement(UISearch, { items, open: true, onOpenChange: noopAction })));
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).toContain("Bravo");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "alp" } });
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).not.toContain("Bravo");
  });

  it("UIFind renders and fuzzy-filters items registered on its context via the shared Fuse re-export", async () => {
    const { render, fireEvent, act } = await import("@testing-library/react");
    let contextValue: ReturnType<typeof useUIFind> | undefined;
    const Harness = () => {
      contextValue = useUIFind();
      return createElement(UIFind, { open: true, onOpenChange: noopAction });
    };
    render(createElement(UIFindProvider, null, createElement(Harness)));
    act(() => {
      contextValue!.setFindItems([
        { id: "1", label: "Chair", category: "Test" },
        { id: "2", label: "Table", category: "Test" },
      ]);
    });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).toContain("Table");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "cha" } });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).not.toContain("Table");
  });
});

// 🧰 Window Actions & Utilities Contract (WS-2): staged argument forms (P1/P2), palette redirect (P3),
// keybinding rule (P4), and registry-derived utility activation (P5).
describe("window action panel — staging and single dispatch (P1/P2)", () => {
  afterEach(() => cleanup());

  const numberArg = (id: string, required: boolean, def?: number): ActionArgDef => ({ id, label: id[0]!.toUpperCase() + id.slice(1), control: { kind: "number" }, required, ...(def === undefined ? {} : { default: def }) });

  const twoArgAction: ActionDefinition = { id: "extrude", label: "Extrude", kind: "operation", inPalette: true, args: [numberArg("depth", true), numberArg("segments", true)] };
  const zeroArgAction: ActionDefinition = { id: "flatten", label: "Flatten", kind: "operation", inPalette: true, args: [] };
  const defaultedAction: ActionDefinition = { id: "bevel", label: "Bevel", kind: "operation", inPalette: true, args: [numberArg("radius", true, 2)] };

  function Harness({ actions, onExecute, disabled }: { actions: readonly ActionDefinition[]; onExecute: (descriptor: unknown) => void; disabled?: boolean }): ReactElement {
    const [expanded, setExpanded] = useState<string | null>(null);
    const [staged, setStaged] = useState<Record<string, Record<string, unknown>>>({});
    return createElement(WindowActionPane, {
      windowId: "w1",
      controllerId: "c",
      actions,
      expandedActionId: expanded,
      stagedArgsByKey: staged,
      disabled: Boolean(disabled),
      onExpandedChange: setExpanded,
      onStageArg: (actionId, argId, value) => setStaged((prev) => ({ ...prev, [actionStageKey("w1", actionId)]: { ...(prev[actionStageKey("w1", actionId)] ?? {}), [argId]: value } })),
      onResetArgs: (actionId) =>
        setStaged((prev) => {
          const next = { ...prev };
          delete next[actionStageKey("w1", actionId)];
          return next;
        }),
      onExecute,
    });
  }

  const buttonByText = (container: HTMLElement, text: string): HTMLButtonElement => {
    const match = [...container.querySelectorAll("button")].find((button) => button.textContent?.includes(text));
    if (!match) throw new Error(`button "${text}" not found`);
    return match as HTMLButtonElement;
  };

  // 🌳 Action rows render as Tree items (`role="treeitem"`), not `<button>`s — only the Execute/Reset
  // form actions render as real buttons. Row clicks (fire a zero-arg action, toggle an arg-carrying one)
  // go through this helper instead of `buttonByText`.
  const rowByText = (container: HTMLElement, text: string): HTMLElement => {
    const match = [...container.querySelectorAll('[role="treeitem"]')].find((row) => row.querySelector('[data-slot="tree-label"]')?.textContent?.trim() === text);
    if (!match) throw new Error(`tree row "${text}" not found`);
    return match as HTMLElement;
  };

  it("stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Extrude…"));
    const inputs = container.querySelectorAll('input[type="number"]');
    expect(inputs).toHaveLength(2);
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(onExecute).not.toHaveBeenCalled();
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "extrude", args: { depth: 3, segments: 2 } });
    // staged values survive Execute (tweak-and-repeat): the inputs still hold their values
    expect((container.querySelectorAll('input[type="number"]')[0] as HTMLInputElement).value).toBe("3");
    fireEvent.click(buttonByText(container, "Execute"));
    expect(onExecute).toHaveBeenCalledTimes(2);
  });

  it("gates Execute on required args, but a default-satisfied required arg counts without staging", () => {
    const onExecute = vi.fn();
    const required = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(rowByText(required.container, "Extrude…"));
    expect(buttonByText(required.container, "Execute").disabled).toBe(true);
    const inputs = required.container.querySelectorAll('input[type="number"]');
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    expect(buttonByText(required.container, "Execute").disabled).toBe(true);
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(buttonByText(required.container, "Execute").disabled).toBe(false);
    cleanup();

    const defaulted = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(defaulted.container, "Bevel…"));
    expect(buttonByText(defaulted.container, "Execute").disabled).toBe(false);
    fireEvent.click(buttonByText(defaulted.container, "Execute"));
    expect(onExecute).toHaveBeenLastCalledWith({ controllerId: "c", action: "bevel", args: { radius: 2 } });
  });

  it("Reset restores defaults while keeping the form expanded", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(rowByText(container, "Bevel…"));
    const input = () => container.querySelector('input[type="number"]') as HTMLInputElement;
    expect(input().value).toBe("2");
    fireEvent.change(input(), { target: { value: "9" } });
    expect(input().value).toBe("9");
    fireEvent.click(buttonByText(container, "Reset"));
    // still expanded (Execute/Reset buttons present) and back to the default effective value
    expect(input().value).toBe("2");
    expect([...container.querySelectorAll("button")].some((b) => b.textContent?.includes("Execute"))).toBe(true);
  });

  it("a zero-arg action row fires immediately with no args object", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "flatten" });
  });

  it("renders every row disabled when an active utility gates actions", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute, disabled: true }));
    fireEvent.click(rowByText(container, "Flatten"));
    expect(onExecute).not.toHaveBeenCalled();
  });

  it("groups actions into category sections like the command panel", () => {
    const createAction: ActionDefinition = { id: "box", label: "Box", kind: "operation", inPalette: true, category: "create", args: [] };
    const transformAction: ActionDefinition = { id: "move", label: "Move", kind: "operation", inPalette: true, category: "transform", args: [] };
    const historyAction: ActionDefinition = { id: "undo", label: "Undo", kind: "history", inPalette: true, args: [] };
    const uncategorizedAction: ActionDefinition = { id: "flatten2", label: "Flatten2", kind: "operation", inPalette: true, args: [] };
    const { container } = render(createElement(Harness, { actions: [createAction, transformAction, historyAction, uncategorizedAction], onExecute: vi.fn() }));
    const textOf = (text: string) => [...container.querySelectorAll("*")].some((el) => el.textContent?.trim() === text && el.children.length === 0);
    expect(textOf("Create")).toBe(true);
    expect(textOf("Transform")).toBe(true);
    expect(textOf("History")).toBe(true);
    expect(textOf("Actions")).toBe(true);
    expect(rowByText(container, "Box")).toBeTruthy();
    expect(rowByText(container, "Move")).toBeTruthy();
    expect(rowByText(container, "Undo")).toBeTruthy();
    expect(rowByText(container, "Flatten2")).toBeTruthy();
  });
});

describe("palette redirect and keybinding rule (P3/P4)", () => {
  const argAction: ActionDefinition = { id: "extrude", label: "Extrude", kind: "operation", inPalette: true, args: [{ id: "depth", label: "Depth", control: { kind: "number" }, required: true }] };
  const zeroAction: ActionDefinition = { id: "flatten", label: "Flatten", kind: "operation", inPalette: true, args: [] };

  it("only arg-carrying actions redirect to a staged form (P3 decision)", () => {
    expect(actionRequiresStagedForm(argAction)).toBe(true);
    expect(actionRequiresStagedForm(zeroAction)).toBe(false);
  });

  it("keybinding intent: arg-less fires, arg-action opens unless already expanded and valid then executes (P4)", () => {
    expect(resolveKeybindingIntent(zeroAction, null, {})).toEqual({ kind: "fire" });
    expect(resolveKeybindingIntent(undefined, null, {})).toEqual({ kind: "fire" });
    // not expanded → open
    expect(resolveKeybindingIntent(argAction, null, {})).toEqual({ kind: "open", actionId: "extrude" });
    expect(resolveKeybindingIntent(argAction, "other", { depth: 3 })).toEqual({ kind: "open", actionId: "extrude" });
    // expanded but required arg missing → stays open, never silent-fires
    expect(resolveKeybindingIntent(argAction, "extrude", {})).toEqual({ kind: "open", actionId: "extrude" });
    // expanded and valid → execute with merged effective args
    expect(resolveKeybindingIntent(argAction, "extrude", { depth: 4 })).toEqual({ kind: "execute", actionId: "extrude", args: { depth: 4 } });
  });
});

describe("registry-derived utilities and activation (P5)", () => {
  const utilities: UtilityDefinition[] = [
    { id: "select", label: "Select", iconId: "mouse-pointer", category: "selection", allowsActionsWhileActive: true },
    { id: "brush", label: "Brush", iconId: "brush", group: "paint", category: "utilities", allowsActionsWhileActive: false },
    { id: "erase", label: "Erase", iconId: "eraser", group: "paint", category: "utilities", allowsActionsWhileActive: false },
  ];
  const app = { controllerId: "draw", utilities } satisfies Pick<AppDefinition, "controllerId" | "utilities">;

  it("resolveUtilities scopes to the window kind's refs, falling back to all app utilities when unset", () => {
    expect(resolveUtilities(app, { utilities: ["brush"] } as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["brush"]);
    expect(resolveUtilities(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["select", "brush", "erase"]);
  });

  it("derives grouped utility nodes with the active utility pressed and a setActiveUtility onChange tagged by window", () => {
    const nodes = resolveUtilityNodes(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">, "brush", "w1");
    const select = nodes.find((node) => node.id === "select");
    expect(select && select.kind === "toggle" ? select.pressed : undefined).toBe(false);
    const paint = nodes.find((node) => node.id === "group:paint");
    expect(paint?.kind).toBe("collection");
    const brush = paint && paint.kind === "collection" ? paint.children.find((child) => child.id === "brush") : undefined;
    expect(brush && brush.kind === "toggle" ? brush.pressed : undefined).toBe(true);
    expect(brush && brush.kind === "toggle" && "onChange" in brush ? brush.onChange : undefined).toEqual({ controllerId: "draw", action: "setActiveUtility", args: { utilityId: "brush", windowId: "w1" } });
  });

  it("deriveUtilityNodes twin marks exactly the active utility pressed", () => {
    const nodes = deriveUtilityNodes(
      "draw",
      [
        { id: "a", label: "A", iconId: "x" },
        { id: "b", label: "B", iconId: "y" },
      ],
      "b",
    );
    expect(nodes.map((node) => (node.kind === "toggle" ? node.pressed : undefined))).toEqual([false, true]);
  });

  it("resolveUtilityActivation toggles: click activates, re-click or empty deactivates", () => {
    expect(resolveUtilityActivation(null, "brush")).toBe("brush");
    expect(resolveUtilityActivation("brush", "erase")).toBe("erase");
    expect(resolveUtilityActivation("brush", "brush")).toBeNull();
    expect(resolveUtilityActivation("brush", "")).toBeNull();
    expect(resolveUtilityActivation(undefined, "")).toBeNull();
  });

  it("resolveWindowActions surfaces panel-eligible actions and frameworkHistoryUtilityNodes derives History buttons", () => {
    const actionsApp = {
      controllerId: "draw",
      actions: [
        { id: "extrude", label: "Extrude", kind: "operation", inPalette: true, args: [] },
        { id: "undo", label: "Undo", kind: "history", iconId: "undo", inPalette: true, args: [] },
        { id: "setActiveUtility", label: "Set Active Utility", kind: "view", inPalette: false, args: [] },
      ] as ActionDefinition[],
      windowKinds: [{ actions: [] as string[] }],
    };
    const resolved = resolveWindowActions(actionsApp, { actions: [] as string[] });
    // orphan operation appears; history + setActiveUtility are never panel-eligible orphans
    expect(resolved.map((action) => action.id)).toEqual(["extrude"]);
    const history = frameworkHistoryUtilityNodes({ controllerId: "draw", actions: actionsApp.actions });
    expect(history).toHaveLength(1);
    expect(history[0]).toMatchObject({ id: "undo", kind: "button", category: "history", onPress: { controllerId: "draw", action: "undo" } });
  });
});

describe("resolveCommands / commandCategories (footer command panel registry)", () => {
  const osCommands: CommandDefinition[] = [{ id: "os.setThemeId", label: "Set Theme", scope: "os", category: "appearance", inPalette: true, args: [] }];
  const pluginManifest = { commands: [{ id: "plugin.export", label: "Export", scope: "plugin", category: "document", inPalette: true, args: [] }] as CommandDefinition[] };
  const app = {
    commands: [
      { id: "app.resetGrid", label: "Reset Grid", scope: "app", category: "document", inPalette: true, args: [] },
      { id: "mode.focus", label: "Focus", scope: "mode", category: "view", inPalette: true, args: [] },
      { id: "mode.paintOnly", label: "Paint Only", scope: "mode", category: "view", inPalette: true, args: [] },
    ] as CommandDefinition[],
    modes: [
      { id: "edit", label: "Edit", commands: ["mode.focus"] },
      { id: "paint", label: "Paint", commands: ["mode.paintOnly"] },
    ] as AppModeDefinition[],
  };

  it("aggregates os + plugin + app-scope + active-mode's mode-scope commands, excluding other modes' mode-scope commands", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "plugin.export", "app.resetGrid", "mode.focus"]);
    expect(resolved.find((entry) => entry.definition.id === "os.setThemeId")?.source).toEqual({ kind: "os" });
    expect(resolved.find((entry) => entry.definition.id === "app.resetGrid")?.source).toEqual({ kind: "app" });
    expect(resolved.find((entry) => entry.definition.id === "mode.focus")?.source).toEqual({ kind: "mode", modeId: "edit" });
  });

  it("switching the active mode swaps which mode-scope commands resolve", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "paint");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "plugin.export", "app.resetGrid", "mode.paintOnly"]);
  });

  it("resolves only os commands with no session (null plugin manifest / app)", () => {
    const resolved = resolveCommands(osCommands, null, null, "");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId"]);
  });

  it("commandCategories orders and dedupes categories by first appearance", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(commandCategories(resolved)).toEqual([
      { id: "appearance", label: "Appearance" },
      { id: "document", label: "Document" },
      { id: "view", label: "View" },
    ]);
  });
});

describe("Introduce App command", () => {
  it("is available only for apps with an introduction", () => {
    expect(buildOsCommands([], [], true).find((command) => command.id === "os.introduceApp")).toMatchObject({ label: "Introduce App", scope: "os", category: "app", args: [] });
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.introduceApp")).toBe(false);
  });

  it("starts the introduction at its first step", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.introduceApp", undefined, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).toHaveBeenCalledWith({ type: "SET_INTRODUCTION_STEP", value: 0 });
  });
});

describe("shell option locks (SEMIO_LOCKED_*)", () => {
  it("resolves valid locale/appearance and falls back with a warning on invalid values while staying locked", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    expect(resolveShellLocks({ locale: "de" })).toMatchObject({ locale: "de" });
    expect(resolveShellLocks({ locale: "fr" })).toMatchObject({ locale: "en" });
    expect(resolveShellLocks({ appearance: "dark" })).toMatchObject({ appearance: "dark" });
    expect(resolveShellLocks({ appearance: "bogus" })).toMatchObject({ appearance: "system" });
    expect(warn).toHaveBeenCalled();
    warn.mockRestore();
  });

  it("accepts any non-empty terminology id verbatim (app-declared ids can't be validated at boot)", () => {
    expect(resolveShellLocks({ terminology: "reuse" })).toMatchObject({ terminology: "reuse" });
    expect(resolveShellLocks({ terminology: "some-app-declared-id" })).toMatchObject({ terminology: "some-app-declared-id" });
    expect(resolveShellLocks({ terminology: "" })).toEqual({});
  });

  it("returns an empty object for undefined locks", () => {
    expect(resolveShellLocks(undefined)).toEqual({});
  });

  it("initialShellState applies locked values over stored/default prefs", () => {
    const state = initialShellState({ plugins: [], locks: { exampleId: "concrete-forest", locale: "de", terminology: "reuse", themeId: "semio", appearance: "dark" } });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    expect(state.uiPrefs.uiLocale).toBe("de");
    expect(state.uiPrefs.uiTerminology).toBe("reuse");
    expect(state.uiPrefs.uiThemeId).toBe("semio");
    expect(state.uiPrefs.uiAppearance).toBe("dark");
  });

  it("mergeShellLockSources keeps brand locks locked and lets a defined env lock win per key", () => {
    expect(mergeShellLockSources({ locale: "de", terminology: "reuse" }, { locale: "en", themeId: undefined })).toEqual({ locale: "en", terminology: "reuse" });
    expect(mergeShellLockSources({ locale: "de" }, undefined)).toEqual({ locale: "de" });
    expect(mergeShellLockSources(undefined, { locale: "en" })).toEqual({ locale: "en" });
    expect(mergeShellLockSources(undefined, undefined)).toBeUndefined();
  });

  it("resolveShellDefaults prefers env defaults over brand defaults and initialShellState seeds without locking", () => {
    const brand = { id: "entwerfen-mit-bestand", windowTitle: "Entwerfen mit Bestand · Aggregator", defaults: { exampleId: "concrete-forest" } };
    expect(resolveShellDefaults(brand, { exampleId: "nakagin-capsule-tower" })).toEqual({ exampleId: "nakagin-capsule-tower" });
    expect(resolveShellDefaults(brand, undefined)).toEqual({ exampleId: "concrete-forest" });
    expect(resolveShellDefaults(undefined, undefined)).toEqual({ exampleId: undefined });
    const state = initialShellState({ plugins: [], defaults: { exampleId: "concrete-forest" } });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    const locked = initialShellState({ plugins: [], locks: { exampleId: "nakagin-capsule-tower" }, defaults: { exampleId: "concrete-forest" } });
    expect(locked.layout.activeExampleId).toBe("nakagin-capsule-tower");
  });

  it("shouldReplayIntroductionOnLoad opts a brand into replaying its tour after every window refresh", () => {
    expect(shouldReplayIntroductionOnLoad(undefined)).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain", replayIntroductionOnLoad: false })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "entwerfen-mit-bestand", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "plain", windowTitle: "Plain" })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "entwerfen-mit-bestand", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(false);
    expect(ENTWERFEN_MIT_BESTAND_BRAND.replayIntroductionOnLoad).toBe(true);
  });

  it("isEphemeralShellBrand skips durable shell state so a refresh boots from brand defaults only", () => {
    expect(isEphemeralShellBrand(undefined)).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain", ephemeral: true })).toBe(true);
    expect(isEphemeralShellBrand(ENTWERFEN_MIT_BESTAND_BRAND)).toBe(true);
    expect(shouldReplayIntroductionOnLoad(ENTWERFEN_MIT_BESTAND_BRAND)).toBe(true);
    expect(shouldPersistIntroductionSeen(ENTWERFEN_MIT_BESTAND_BRAND)).toBe(false);
    const ephemeralState = initialShellState({
      plugins: [],
      locks: { locale: "de", terminology: "reuse", themeId: "semio" },
      defaults: { exampleId: "concrete-forest" },
      ephemeral: true,
    });
    expect(ephemeralState.layout.activeExampleId).toBe("concrete-forest");
    expect(ephemeralState.uiPrefs.uiLocale).toBe("de");
    expect(ephemeralState.uiPrefs.uiTerminology).toBe("reuse");
    expect(ephemeralState.uiPrefs.uiThemeId).toBe("semio");
    expect(ephemeralState.uiPrefs.uiAppearance).toBe("system");
    expect(ephemeralState.uiPrefs.uiLayout).toBe("desktop");
    expect(ephemeralState.uiPrefs.uiCustomThemes).toEqual({});
    expect(ephemeralState.layout.dockOverride).toBeNull();
    expect(ephemeralState.layout.shellLayout).toBeNull();
    localStorage.setItem("ui.chrome.appearance", "dark");
    localStorage.setItem("semio.os.dock", "{}");
    localStorage.setItem("ui.introduction.seen.entwerfen-mit-bestand:puzzle3d-play", "true");
    clearDurableShellStorage();
    expect(localStorage.getItem("ui.chrome.appearance")).toBeNull();
    expect(localStorage.getItem("semio.os.dock")).toBeNull();
    expect(localStorage.getItem("ui.introduction.seen.entwerfen-mit-bestand:puzzle3d-play")).toBeNull();
  });

  it("ENTWERFEN_MIT_BESTAND_BRAND introduction opens with a project-demonstrator welcome, prototype notice, and funding credit before the app tour", () => {
    const steps = ENTWERFEN_MIT_BESTAND_BRAND.introduction!.steps;
    expect(steps.map((step) => step.id)).toEqual(["welcome", "prototype", "funding", "viewport", "catalogue", "add-object", "move-utility"]);
    expect(steps.find((step) => step.id === "catalogue")?.placement).toBe("right");
    expect(steps.find((step) => step.id === "add-object")).toMatchObject({
      anchor: { kind: "panelFirstDraggable", id: "framework.panel.catalogue" },
      placement: "right",
      advance: { kind: "action", id: "addObjectKind" },
      cutouts: [{ kind: "windowKind", id: "puzzle3d-main" }],
    });
    expect(steps.find((step) => step.id === "add-object")?.body).toMatch(/Drag-and-Drop|ziehen/i);

    const funding = steps.find((step) => step.id === "funding")!;
    expect(funding.logos).toHaveLength(3);
    for (const logo of funding.logos!) {
      expect(logo.src).toMatch(/^\/asset\/logo\//);
      expect(logo.darkSrc).toMatch(/^\/asset\/logo\//);
      expect(logo.alt).toBeTruthy();
    }
    const zukunftBauLogo = funding.logos!.find((logo) => logo.href === ZUKUNFT_BAU_PROJECT_URL);
    expect(zukunftBauLogo).toBeDefined();
  });

  it("buildOsCommands omits only the commands for locked prefs", () => {
    const ids = buildOsCommands([], [], false, { locale: "de", appearance: "dark" }).map((c) => c.id);
    expect(ids).not.toContain("os.setLocale");
    expect(ids).not.toContain("os.setAppearance");
    expect(ids).toContain("os.setTerminology");
    expect(ids).toContain("os.setThemeId");
  });

  it("dispatchOsCommand is a no-op for a locked pref even if invoked directly", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.setLocale", { locale: "de" }, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never, { locale: "en" });
    expect(dispatch).not.toHaveBeenCalled();
  });
});

describe("buildCommandCategoryTree / buildCommandCategoryTabs (command palette as a real bottom-middle Panel)", () => {
  const zeroArgCommand: ResolvedCommand = { definition: { id: "os.resetDock", label: "Reset Dock", scope: "os", category: "layout", inPalette: true, args: [] }, source: { kind: "os" } };
  const argCommand: ResolvedCommand = {
    definition: { id: "os.setThemeId", label: "Set Theme", scope: "os", category: "appearance", inPalette: true, args: [{ id: "themeId", label: "Theme", control: { kind: "text" }, required: true }] },
    source: { kind: "os" },
  };
  const secondArgCommand: ResolvedCommand = {
    definition: { id: "os.setAppearance", label: "Set Appearance", scope: "os", category: "appearance", inPalette: true, args: [{ id: "appearance", label: "Appearance", control: { kind: "text" }, required: true }] },
    source: { kind: "os" },
  };
  const singletonArgCommand: ResolvedCommand = {
    definition: { id: "os.setExpertise", label: "Set Expertise", scope: "os", category: "general", inPalette: true, args: [{ id: "expertise", label: "Expertise", control: { kind: "text" }, required: true }] },
    source: { kind: "os" },
  };

  it("a zero-arg command row fires onExecute directly on click; only one command-list section is present when nothing is expanded", () => {
    const onExecute = vi.fn();
    const tree = buildCommandCategoryTree([zeroArgCommand], null, {}, onExecute, vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    const row = tree.sections[0]!.items!.find((item) => item.id === "command.os.resetDock")!;
    expect(row.label).toBe("Reset Dock");
    row.onClick?.({} as never, {} as never);
    expect(onExecute).toHaveBeenCalledWith(zeroArgCommand);
  });

  it("auto-expands a singleton arg-carrying category into a flat form with section actions and no disclosure list", () => {
    const tree = buildCommandCategoryTree([singletonArgCommand], null, {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(tree.sections).toHaveLength(1);
    expect(tree.sections[0]!.id).toBe("command.category.general.form");
    expect(tree.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.setExpertise.arg.expertise"]);
    expect(tree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.setExpertise-execute", "command-os.setExpertise-reset"]);
  });

  it("an arg-carrying command row toggles expansion instead of executing, and a synthetic arg-form section only appears while expanded", () => {
    const onToggleExpanded = vi.fn();
    const collapsedTree = buildCommandCategoryTree([argCommand, secondArgCommand], null, {}, vi.fn(), onToggleExpanded, vi.fn(), vi.fn());
    expect(collapsedTree.sections).toHaveLength(1);
    const collapsedRow = collapsedTree.sections[0]!.items!.find((item) => item.id === "command.os.setThemeId")!;
    expect(collapsedRow.label).toBe("Set Theme…");
    collapsedRow.onClick?.({} as never, {} as never);
    expect(onToggleExpanded).toHaveBeenCalledWith("os.setThemeId");

    const expandedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os.setThemeId", {}, vi.fn(), vi.fn(), vi.fn(), vi.fn());
    expect(expandedTree.sections).toHaveLength(2);
    const formItems = expandedTree.sections[0]!.items!;
    expect(formItems.find((item) => item.id === "command.os.setThemeId.arg.themeId")?.label).toBe("Theme");
    expect(expandedTree.sections[0]!.actions?.map((action) => action.id)).toEqual(["command-os.setThemeId-execute", "command-os.setThemeId-reset"]);
    expect(expandedTree.sections[1]!.items?.map((item) => item.id)).toEqual(["command.os.setAppearance"]);
  });

  it("Execute is disabled until the required arg is staged, and calling it passes the effective (staged) args; Reset dispatches onResetArgs", () => {
    const onExecute = vi.fn();
    const onStageArg = vi.fn();
    const onResetArgs = vi.fn();

    const missingTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os.setThemeId", {}, onExecute, vi.fn(), onStageArg, onResetArgs);
    const missingExecute = missingTree.sections[0]!.actions!.find((action) => action.id === "command-os.setThemeId-execute")!;
    expect(missingExecute.disabled).toBe(true);

    const stagedTree = buildCommandCategoryTree([argCommand, secondArgCommand], "os.setThemeId", { "os.setThemeId": { themeId: "semio" } }, onExecute, vi.fn(), onStageArg, onResetArgs);
    const stagedExecute = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.setThemeId-execute")!;
    const stagedReset = stagedTree.sections[0]!.actions!.find((action) => action.id === "command-os.setThemeId-reset")!;
    expect(stagedExecute.disabled).toBe(false);
    stagedExecute.onClick();
    expect(onExecute).toHaveBeenCalledWith(argCommand, { themeId: "semio" });
    stagedReset.onClick();
    expect(onResetArgs).toHaveBeenCalledWith("os.setThemeId");
  });

  it("buildCommandCategoryTabs builds one namespaced PanelTabLeaf per category, whose lazily-resolved tree only contains that category's commands", () => {
    const categories = [
      { id: "layout", label: "Layout" },
      { id: "appearance", label: "Appearance" },
    ];
    const expandedRef = { current: null as string | null };
    const stagedRef = { current: {} as Readonly<Record<string, Readonly<Record<string, unknown>>>> };
    const onCommand = vi.fn();
    const dispatch = vi.fn();
    const tabs = buildCommandCategoryTabs([zeroArgCommand, argCommand], categories, expandedRef, stagedRef, onCommand, dispatch);
    expect(tabs.map((tab) => tab.id)).toEqual(["command.category.layout", "command.category.appearance"]);
    expect(tabs.every((tab) => tab.kind === "leaf")).toBe(true);

    const layoutLeaf = tabs[0]!;
    expect(layoutLeaf.kind).toBe("leaf");
    const resolved = layoutLeaf.kind === "leaf" ? (layoutLeaf.trees[0]!.tree as { resolveTree: () => { sections: { items?: { id: string }[] }[] } }).resolveTree() : { sections: [] };
    expect(resolved.sections[0]!.items?.map((item) => item.id)).toEqual(["command.os.resetDock"]);

    // Executing routes through the injected onCommand with the command's own source.
    const executeRow = resolved.sections[0]!.items!.find((item: { id: string }) => item.id === "command.os.resetDock") as unknown as { onClick: (event: never, context: never) => void };
    executeRow.onClick({} as never, {} as never);
    expect(onCommand).toHaveBeenCalledWith({ kind: "os" }, "os.resetDock", undefined);
  });
});

describe("host effect dispatch (D2 DispatchAction, D3 RequestFileOpen.multiple, D5 RequestMediaFrames)", () => {
  afterEach(() => {
    vi.useRealTimers();
  });

  it("scheduleDispatchAction (D2): fires dispatchOne with action/args only after delayMs elapses", () => {
    vi.useFakeTimers();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction("advanceReconstruction", { jobId: "job-1" }, 250, dispatchOne);
    expect(dispatchOne).not.toHaveBeenCalled();
    vi.advanceTimersByTime(249);
    expect(dispatchOne).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("advanceReconstruction", { jobId: "job-1" });
  });

  it("scheduleDispatchAction (D2): delayMs 0 still defers to a scheduled tick, not a synchronous call", () => {
    vi.useFakeTimers();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    scheduleDispatchAction("tick", undefined, 0, dispatchOne);
    expect(dispatchOne).not.toHaveBeenCalled();
    vi.advanceTimersByTime(0);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("tick", undefined);
  });

  it("dispatchOpenedFiles (D3): single-file (multiple=false) makes exactly one call with {payload, name} and no index/total", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    await dispatchOpenedFiles([{ contents: "abc", name: "a.png" }], "importFramePayload", false, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledExactlyOnceWith("importFramePayload", { payload: "abc", name: "a.png" });
  });

  it("dispatchOpenedFiles (D3): multiple=true dispatches once per file, in order, each extended with {index, total}", async () => {
    const calls: unknown[][] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: unknown) => {
      calls.push([action, args]);
    });
    const opened = [
      { contents: "a", name: "a.png" },
      { contents: "b", name: "b.png" },
      { contents: "c", name: "c.png" },
    ];
    await dispatchOpenedFiles(opened, "importFramePayload", true, dispatchOne);
    expect(dispatchOne).toHaveBeenCalledTimes(3);
    expect(calls).toEqual([
      ["importFramePayload", { payload: "a", name: "a.png", index: 0, total: 3 }],
      ["importFramePayload", { payload: "b", name: "b.png", index: 1, total: 3 }],
      ["importFramePayload", { payload: "c", name: "c.png", index: 2, total: 3 }],
    ]);
  });

  it("sampleMediaFrameTimestampsMs (D5): steps by sampleStride/fpsHint seconds, capped at maxFrames", () => {
    // Expected timestamps mirror the implementation's own `k * stepMs` computation exactly (bit-for-bit)
    // rather than independently-derived literals — plain float division/multiplication isn't perfectly
    // associative, so e.g. `k * (5 / 30 * 1000)` and `(k * 5000) / 30` can differ by a ULP.
    const stepAt5_30 = (5 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(1000, 5, 0, 30)).toEqual([0, 1, 2, 3, 4, 5].map((k) => k * stepAt5_30));
    expect(sampleMediaFrameTimestampsMs(1000, 5, 2, 30)).toEqual([0, stepAt5_30]);
    // sampleStride 0 floors to 1, fpsHint 0 falls back to 30 — never divides by zero.
    const stepAt1_30 = (1 / 30) * 1000;
    expect(sampleMediaFrameTimestampsMs(100, 0, 0, 0)).toEqual([0, 1, 2].map((k) => k * stepAt1_30));
    expect(sampleMediaFrameTimestampsMs(0, 5, 10, 30)).toEqual([]);
  });

  //#region 🔌jsdom media mocks
  /** 🎞️ jsdom has no real media decoder — `<video>`'s `duration`/`videoWidth`/`videoHeight` are
   * read-only getters that never change from a `src` assignment, and `currentTime` is a no-op setter
   * that never fires `seeked`. This stubs both to the minimum needed for `runTier2VideoFrames`'s
   * seek-and-capture loop: `currentTime` synchronously (via microtask) fires `seeked`, mirroring how a
   * real browser resolves a seek asynchronously without needing fake timers in these tests. */
  function mockVideoElement(durationMs: number, width: number, height: number): HTMLVideoElement {
    const video = document.createElement("video");
    Object.defineProperty(video, "duration", { value: durationMs / 1000, configurable: true });
    Object.defineProperty(video, "videoWidth", { value: width, configurable: true });
    Object.defineProperty(video, "videoHeight", { value: height, configurable: true });
    Object.defineProperty(video, "readyState", { value: 1, configurable: true });
    Object.defineProperty(video, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        queueMicrotask(() => video.dispatchEvent(new Event("seeked")));
      },
    });
    return video;
  }

  function mockCanvasCapture(): void {
    HTMLCanvasElement.prototype.getContext = vi.fn().mockReturnValue({ drawImage: vi.fn() }) as unknown as typeof HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.toDataURL = vi.fn().mockReturnValue("data:image/jpeg;base64,ZmFrZQ==");
  }
  //#endregion 🔌jsdom media mocks

  it("runTier2VideoFrames (D5): dispatches frameAction once per sampled timestamp, in order, then doneAction exactly once", async () => {
    mockCanvasCapture();
    const video = mockVideoElement(200, 64, 48);
    const calls: { action: string; args: Record<string, unknown> }[] = [];
    const dispatchOne = vi.fn().mockImplementation(async (action: string, args: Record<string, unknown>) => {
      calls.push({ action, args });
    });
    await runTier2VideoFrames(video, { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 5, maxFrames: 0, maxLongEdgePx: 0, fpsHint: 30, args: { streamId: "s1" } }, "clip.mp4", dispatchOne);
    const frameCalls = calls.filter((call) => call.action === "frame");
    const doneCalls = calls.filter((call) => call.action === "done");
    expect(frameCalls.length).toBeGreaterThan(0);
    expect(doneCalls).toHaveLength(1);
    // frame/done ordering: every frame dispatch precedes the single done dispatch.
    expect(calls.at(-1)!.action).toBe("done");
    expect(frameCalls.map((call) => call.args.index)).toEqual(frameCalls.map((_, index) => index));
    expect(frameCalls[0]!.args).toMatchObject({ payload: "data:image/jpeg;base64,ZmFrZQ==", name: "clip.mp4", streamId: "s1" });
    expect(doneCalls[0]!.args).toMatchObject({ name: "clip.mp4", frameCount: frameCalls.length, sampledCount: frameCalls.length, streamId: "s1" });
  });

  it("runRequestMediaFrames (D5): Tier 2 failure (video element throws mid-seek) ⇒ dispatches fallbackAction exactly once with raw bytes as a data URL, no frame/done calls", async () => {
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    const throwingVideo = mockVideoElement(1000, 16, 16);
    Object.defineProperty(throwingVideo, "currentTime", {
      configurable: true,
      get() {
        return 0;
      },
      set() {
        throw new Error("decode failed");
      },
    });
    await runRequestMediaFrames(
      { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 },
      "video/mp4",
      payload,
      dispatchOne,
      () => throwingVideo,
    );
    expect(dispatchOne).toHaveBeenCalledTimes(1);
    const [action, args] = dispatchOne.mock.calls[0]! as [string, Record<string, unknown>];
    expect(action).toBe("fallback");
    expect(args.name).toBe("video");
    expect(String(args.payload)).toMatch(/^data:video\/mp4;base64,/);
  });

  it("runRequestMediaFrames (D5): payload bytes in hand ⇒ Tier 2 seek-capture runs, ending in doneAction (no picker needed)", async () => {
    mockCanvasCapture();
    const dispatchOne = vi.fn().mockResolvedValue(undefined);
    const payload = "data:video/mp4;base64," + btoa("not a real mp4 but bytes exist");
    await runRequestMediaFrames(
      { frameAction: "frame", doneAction: "done", fallbackAction: "fallback", sampleStride: 1, maxFrames: 2, maxLongEdgePx: 0, fpsHint: 30 },
      "video/mp4",
      payload,
      dispatchOne,
      () => mockVideoElement(1000, 16, 16),
    );
    const actions = dispatchOne.mock.calls.map((call) => call[0] as string);
    expect(actions.at(-1)).toBe("done");
    expect(actions.filter((action) => action === "frame").length).toBeGreaterThan(0);
  });
});

describe("Display Windows tab — projection drag templates", () => {
  function windowsTreeSections(windowKinds: DisplayHostApi["windowKinds"]) {
    const host: DisplayHostApi = {
      windowKinds,
      namedLayouts: [],
      userLayouts: [],
      saveCurrentLayout: () => {},
      applyNamedLayout: () => {},
      deleteUserLayout: () => {},
      layoutSaveLabel: "",
      setLayoutSaveLabel: () => {},
    };
    const tabs = createFrameworkDisplayPanelTabs(() => host);
    type WindowsTreeSections = { readonly sections: readonly { readonly id: string; readonly items?: readonly unknown[] }[] };
    type LeafWithTrees = { readonly trees: readonly { readonly tree: { readonly resolveTree: () => WindowsTreeSections } }[] };
    const windowsTab = tabs.find((tab) => tab.id === "framework.display.windows") as unknown as LeafWithTrees;
    return windowsTab.trees[0]!.tree.resolveTree().sections;
  }

  it("nests the full Parallel/Perspective projection taxonomy under a world-3d window kind", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", surfaceKind: "world-3d" }]);
    expect(sections).toHaveLength(1);
    const items = sections[0]!.items as { readonly id: string; readonly label?: string; readonly items?: readonly { readonly id: string }[] }[];
    expect(items[0]!.id).toBe("framework.display.windows.puzzle3d-main.kind");
    const [, parallel, perspective] = items;
    expect(parallel!.label).toBe("Parallel");
    expect(perspective!.label).toBe("Perspective");
    expect(parallel!.items!.map((row) => row.id.split(".").pop())).toEqual(["orthographic", "axonometric", "oblique"]);
  });

  it("keeps a flat single drag entry for non-world-3d window kinds", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.items).toHaveLength(1);
  });

  it("each projection leaf's drag payload decodes back to its WorldProjectionSpec", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as { readonly items?: readonly { readonly items?: readonly unknown[]; readonly dragData?: Record<string, string> }[] }[];
    const orthographic = items[1]!.items![0]! as { readonly items: readonly { readonly dragData: Record<string, string> }[] };
    const topLeaf = orthographic.items[1]!;
    const payload = JSON.parse(topLeaf.dragData["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(payload.windowKindId).toBe("puzzle3d-main");
    expect(decodeWorldProjectionTemplateId(payload.templateId)).toEqual({ kind: "orthographic", view: "top" });
  });
});
