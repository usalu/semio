import { createElement, useState, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { cleanup, fireEvent, render } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { deriveToolNodes, resolveWindowActions, partitionWindowMeasures, type ActionArgDef, type ActionDefinition, type AppDefinition, type AppWindowKindDefinition, type ToolDefinition, type WindowMeasure } from "@semio-tech/framework-core";
import { Canvas2dHost, worldToScreenLogical } from "./components/canvas-2d-host.tsx";
import {
  Puzzle2dBoardHost,
  puzzle2dBoardCameraActionArgs,
  buildPuzzle2dSelectionMenuItems,
  coalescePuzzle2dBoardEvents,
  parsePuzzle2dCatalogueDragPayload,
  puzzle2dFixtureDropPreviewJson,
  puzzle2dScreenToWorld,
} from "./components/puzzle-2d-board-host.tsx";
import {
  NodeGraphHost,
  catalogueGhostDescriptorJson,
  computeDagMarqueeOverlay,
  nodeGraphViewportActionArgs,
  parseCatalogueAppDragPayload,
  parseDagSliderOverlays,
  resolveFixtureWidgetInstanceId,
} from "./components/node-graph-host.tsx";
import { SelectionMarquee } from "@semio-tech/ui-react";
import { RasterHost } from "./components/raster-host.tsx";
import { TableHost } from "./components/table-host.tsx";
import { VcsHistoryHost } from "./components/vcs-history-host.tsx";
import { TextEditorHost, buildTextEditorContextMenuItems, lineRangeAt, multiSpanReplace } from "./components/text-editor-host.tsx";
import { World3dHost, brushObjectPlacementArgs, resolveMeshStyle, resolveVortexPointerDownIntent, resolveWorldContextMenuTarget, worldInstancePickBlocked } from "./components/world-3d-host.tsx";
import { parseWorldTerrainStyle } from "./components/world-terrain-layer.tsx";
import {
  NoteCanvasHost,
  noteBlockBounds,
  noteEraseInkPointsInBlock,
  noteHtmlToParagraphs,
  noteParagraphsToHtml,
  noteResizeBounds,
  noteScaleBlockWithinGroup,
  noteClipboardPayload,
  noteBlocksFromClipboardPayload,
  screenToWorld,
  worldToScreen,
  type NoteDocument,
  type NoteInkBlock,
} from "./components/note-canvas-host.tsx";
import {
  appDocumentLabel,
  appWindowDocumentLabel,
  applyUiRefreshResponseToCache,
  buildToolbarRibbonSegments,
  buildUiRefreshRequest,
  dedupeToolNodesById,
  flattenPanelTabLeaves,
  groupToolNodesByCategory,
  initialShellState,
  isFlowGraphScene,
  loadPluginModule,
  mergeRecordPreservingIdentity,
  parseStudioShellPath,
  preserveJsonIdentity,
  reconcileToolPath,
  resolveWindowToolNodes,
  resolveWindowTools,
  frameworkHistoryToolNodes,
  actionStageKey,
  actionRequiresStagedForm,
  resolveKeybindingIntent,
  resolveToolActivation,
  WindowActionPanel,
  shellReducer,
  sortToolNodes,
  spawnedWindowChromeForKind,
  ToolTree,
  type UiRefreshCache,
  UIFind,
  UIFindProvider,
  uiNodeToTreePanelConfig,
  UISearch,
  type UISearchItem,
  useUIFind,
} from "./os-shell.tsx";
import { interpretUiNode } from "./ui-interpreter.tsx";
import type { ToolNode, UiNode } from "@semio-tech/framework-core";

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

describe("framework sync tools", () => {
  it("builds three sync backbone toggles", async () => {
    const { buildFrameworkSyncTools } = await import("@semio-tech/framework-os-core");
    const tools = buildFrameworkSyncTools("file:///demo");
    expect(tools).toHaveLength(3);
    expect(tools.map((tool) => tool.id)).toEqual([
      "framework.sync.file",
      "framework.sync.folder",
      "framework.sync.remote",
    ]);
    expect(tools[0]?.pressed).toBe(true);
  });

  it("has no active toggle when detached", async () => {
    const { buildFrameworkSyncTools } = await import("@semio-tech/framework-os-core");
    const tools = buildFrameworkSyncTools(null);
    expect(tools.every((tool) => !tool.pressed)).toBe(true);
  });

  it("groups File, Folder, and Remote under a single Sync category collection", async () => {
    const { buildFrameworkSyncTools } = await import("@semio-tech/framework-os-core");
    const tools = buildFrameworkSyncTools("file:///demo");
    const grouped = groupToolNodesByCategory(tools as unknown as ToolNode[], ["sync"]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0]).toMatchObject({ id: "sync", kind: "collection" });
    expect(grouped[0].kind === "collection" ? grouped[0].children.map((child) => child.id) : []).toEqual(["framework.sync.file", "framework.sync.folder", "framework.sync.remote"]);
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

  it("toggles the layout slice via an updater function", () => {
    const state = baseState();
    const opened = shellReducer(state, { type: "SET_CORNER_PANEL_VISIBLE", corner: "top-left", value: true });
    const toggled = shellReducer(opened, { type: "SET_CORNER_PANEL_VISIBLE", corner: "top-left", value: (prev) => !prev });
    expect(opened.layout.cornerPanels["top-left"].visible).toBe(true);
    expect(toggled.layout.cornerPanels["top-left"].visible).toBe(false);
    expect(toggled.overlays).toBe(opened.overlays);
  });

  it("resets the dock override, every corner's active path/visible/size, drill-down memory, and tree expansion via RESET_DOCK", () => {
    const state = baseState();
    const rearranged = shellReducer(state, { type: "SET_DOCK_OVERRIDE", value: { version: 1, corners: { "top-left": [{ id: "moved" }], "top-right": [], "bottom-left": [], "bottom-right": [] } } });
    const withPath = shellReducer(rearranged, { type: "SET_CORNER_PANEL_PATH", corner: "top-left", value: ["moved"] });
    const withVisible = shellReducer(withPath, { type: "SET_CORNER_PANEL_VISIBLE", corner: "top-left", value: true });
    const withSize = shellReducer(withVisible, { type: "SET_CORNER_PANEL_SIZE", corner: "top-left", value: 999 });
    const withMemory = shellReducer(withSize, { type: "SET_PANEL_PATH_MEMORY", value: { moved: "child" } });
    const withTreeOpen = shellReducer(withMemory, { type: "SET_TREE_OPEN_STATE", id: "unit:section", open: true });
    const reset = shellReducer(withTreeOpen, { type: "RESET_DOCK" });
    expect(reset.layout.dockOverride).toBeNull();
    expect(reset.layout.cornerPanels["top-left"].path).toEqual([]);
    expect(reset.layout.cornerPanels["top-left"].visible).toBe(false);
    expect(reset.layout.cornerPanels["top-left"].size).toBe(state.layout.cornerPanels["top-left"].size);
    expect(reset.layout.panelPathMemory).toEqual({});
    expect(reset.layout.treeOpenStates).toEqual({});
  });

  it("updates the uiPrefs slice and leaves the sync slice referentially unchanged", () => {
    const state = baseState();
    const next = shellReducer(state, { type: "SET_UI_COMPACT", value: (prev) => !prev });
    expect(next.uiPrefs.uiCompact).toBe(!state.uiPrefs.uiCompact);
    expect(next.sync).toBe(state.sync);
  });

  it("action-panel slice: fold/expand/stage/reset/active-tool update only their own keys and preserve identity on no-ops", () => {
    const state = baseState();

    const folded = shellReducer(state, { type: "SET_ACTION_PANEL_FOLDED", windowId: "w1", value: false });
    expect(folded.actionPanel.foldedByWindowId).toEqual({ w1: false });
    expect(folded.layout).toBe(state.layout);
    // no-op fold keeps the whole slice referentially stable
    expect(shellReducer(folded, { type: "SET_ACTION_PANEL_FOLDED", windowId: "w1", value: false }).actionPanel).toBe(folded.actionPanel);

    const expanded = shellReducer(folded, { type: "SET_ACTION_PANEL_EXPANDED", windowId: "w1", value: "extrude" });
    expect(expanded.actionPanel.expandedByWindowId).toEqual({ w1: "extrude" });
    expect(shellReducer(expanded, { type: "SET_ACTION_PANEL_EXPANDED", windowId: "w1", value: "extrude" }).actionPanel).toBe(expanded.actionPanel);

    const staged = shellReducer(expanded, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 });
    expect(staged.actionPanel.stagedArgsByKey).toEqual({ "w1:extrude": { depth: 3 } });
    const stagedMore = shellReducer(staged, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "segments", value: 2 });
    expect(stagedMore.actionPanel.stagedArgsByKey["w1:extrude"]).toEqual({ depth: 3, segments: 2 });
    expect(shellReducer(stagedMore, { type: "STAGE_ACTION_ARG", windowId: "w1", actionId: "extrude", argId: "depth", value: 3 }).actionPanel).toBe(stagedMore.actionPanel);

    const reset = shellReducer(stagedMore, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" });
    expect(reset.actionPanel.stagedArgsByKey["w1:extrude"]).toBeUndefined();
    // reset keeps the panel expanded
    expect(reset.actionPanel.expandedByWindowId["w1"]).toBe("extrude");
    expect(shellReducer(reset, { type: "RESET_ACTION_ARGS", windowId: "w1", actionId: "extrude" }).actionPanel).toBe(reset.actionPanel);

    const activated = shellReducer(reset, { type: "SET_ACTIVE_TOOL", windowId: "w1", toolId: "pen" });
    expect(activated.actionPanel.activeToolByWindowId).toEqual({ w1: "pen" });
    expect(shellReducer(activated, { type: "SET_ACTIVE_TOOL", windowId: "w1", toolId: "pen" }).actionPanel).toBe(activated.actionPanel);
    const deactivated = shellReducer(activated, { type: "SET_ACTIVE_TOOL", windowId: "w1", toolId: null });
    expect(deactivated.actionPanel.activeToolByWindowId["w1"]).toBeNull();
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
    const previous = { nodes: [{ id: "a", x: 1 }, { id: "b", x: 2 }] };
    const next = { nodes: [{ id: "a", x: 1 }, { id: "b", x: 2 }] };
    expect(preserveJsonIdentity(previous, next)).toBe(previous);
    const moved = { nodes: [{ id: "a", x: 1 }, { id: "b", x: 3 }] };
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

  it("buildUiRefreshRequest for a full scope requests every window/panel/engagements/measures/labels section (toolbars are now registry-derived, not a plugin section)", () => {
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

  it("parses a typed ActionResponse, including requestedEffects, from a plugin handle-action response", async () => {
    const { parseActionResponse } = await import("@semio-tech/framework-core");
    const response = parseActionResponse(
      JSON.stringify({
        output: null,
        operations: [{ diff: { payload: { schemaId: "draw.op", document: { id: "forest" } } } }],
        inverseGroup: { actionId: "setActiveExample:1:0", operations: [], inverseOperations: [] },
        requestedEffects: [{ navigate: { uri: "/studios/forest" } }],
      }),
    );
    expect(response.operations).toHaveLength(1);
    expect(response.requestedEffects).toEqual([{ navigate: { uri: "/studios/forest" } }]);
  });

  it("falls back to an empty ActionResponse for malformed handle-action JSON", async () => {
    const { parseActionResponse } = await import("@semio-tech/framework-core");
    expect(parseActionResponse("not json")).toEqual({ output: null, operations: [], inverseGroup: { actionId: "", operations: [], inverseOperations: [] } });
    expect(parseActionResponse(JSON.stringify({ output: null }))).toEqual({ output: null, operations: [], inverseGroup: { actionId: "", operations: [], inverseOperations: [] } });
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
        return { output: null, operations: [], inverseGroup: { actionId: "", operations: [], inverseOperations: [] } };
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
      controllerId: "puzzle3d-play",
      modes: [],
      windowKinds: [],
      panelTabs: [],
      keybindings: [],
    };
    expect(appDocumentLabel(app.document)).toBe("semio · puzzle · 3d");
    expect(appWindowDocumentLabel(app, "Flow")).toBe("Flow");
    expect(appWindowDocumentLabel(app, "Preview")).toBe("Preview");
    expect(appWindowDocumentLabel(app, "")).toBe("Puzzle 3D");
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

  it("accepts vcs-history component scene nodes", () => {
    const node: UiNode = {
      type: "componentScene",
      surfaceId: "vcs.play.history",
      controllerId: "vcs-play",
      componentKind: "vcs-history",
      vcsHistory: {
        columnsJson: "[]",
      },
    };
    expect(node.componentKind).toBe("vcs-history");
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
    const { declarativeTreeDragController } = await import("./ui-interpreter.tsx");
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
              { id: "meta:tool", role: "meta", tool: "selectDirect" },
              {
                id: "shape-1",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [
                  { kind: "move", to: [0, 0] },
                  { kind: "line", to: [10, 0] },
                  { kind: "line", to: [10, 10] },
                  { kind: "close" },
                ],
                fill: { kind: "linearGradient", x1: 0, y1: 0, x2: 10, y2: 10, stops: [{ offset: 0, color: [1, 0, 0, 1] }, { offset: 1, color: [0, 0, 1, 1] }] },
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
                segments: [
                  { kind: "move", to: [0, 0] },
                  { kind: "line", to: [10, 0] },
                  { kind: "close" },
                ],
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
      createElement(Puzzle2dBoardHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle2d.play.composite.2d-overview",
          controllerId: "puzzle2d-play",
          componentKind: "puzzle2d-board",
          puzzle2dBoard: {
            fixtureJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            kindCatalogsJson: "{}",
            selectionJson: "[]",
            interactive: true,
            selectionMethod: "rectangle",
            gridSnapEnabled: false,
            gridFactor: 1,
            suggestionOffset: 0,
            brushKindWeightsJson: "{}",
            kindCompatibilityJson: "[]",
            lodMode: "automatic",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-puzzle2d-board-host");
  });

  it("uses the live puzzle 2d board camera for wheel persistence actions", () => {
    expect(puzzle2dBoardCameraActionArgs('{"x":345,"y":-123,"zoom":4.25}')).toEqual({
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
    const { flushNow, eventsJson } = coalescePuzzle2dBoardEvents(rows);
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
    const { eventsJson } = coalescePuzzle2dBoardEvents(rows);
    const events = JSON.parse(eventsJson) as { name: string }[];
    expect(events.some((event) => event.name === "nodeMove")).toBe(false);
    expect(events.some((event) => event.name === "nodeDragEnd")).toBe(true);
  });

  it("flushes puzzle 2d board events immediately for select/brushPlace/edge/delete rows, not for camera/nodeMove alone", () => {
    expect(coalescePuzzle2dBoardEvents([{ name: "camera", payload: { x: 0, y: 0, zoom: 1 } }]).flushNow).toBe(false);
    expect(coalescePuzzle2dBoardEvents([{ name: "nodeMove", payload: { id: "alpha", x: 0, y: 0 } }]).flushNow).toBe(false);
    for (const name of ["select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete"]) {
      expect(coalescePuzzle2dBoardEvents([{ name, payload: {} }]).flushNow).toBe(true);
    }
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
        interactionJson: '{"activeTool":"select"}',
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

  it("blocks instance picking for fill and brush engagements but not select", () => {
    expect(worldInstancePickBlocked("brush")).toBe(true);
    expect(worldInstancePickBlocked("fill")).toBe(true);
    expect(worldInstancePickBlocked("select")).toBe(false);
    expect(worldInstancePickBlocked(undefined)).toBe(false);
  });

  it("resolves vortex pointer-down to select in brush mode and connect-drag otherwise", () => {
    expect(resolveVortexPointerDownIntent(true)).toBe("select");
    expect(resolveVortexPointerDownIntent(false)).toBe("connect-drag");
  });

  it("resolves mesh style by premigration priority: disabled > selected > highlighted > hovered > neutral", () => {
    expect(resolveMeshStyle({})).toBe("neutral");
    expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
    expect(resolveMeshStyle({ hovered: true, highlighted: true })).toBe("highlighted");
    expect(resolveMeshStyle({ highlighted: true, selected: true })).toBe("selected");
    expect(resolveMeshStyle({ selected: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ disabled: true, selected: true, highlighted: true, hovered: true })).toBe("disabled");
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
    const result = multiSpanReplace("MATCH (a:Piece) RETURN a.name", [
      { start: 7, end: 8 },
      { start: 23, end: 24 },
    ], "piece");
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
      createElement(VcsHistoryHost, {
        node: {
          type: "componentScene",
          surfaceId: "vcs.play.history",
          controllerId: "vcs-play",
          componentKind: "vcs-history",
          vcsHistory: {
            columnsJson: JSON.stringify(columns),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-vcs-history-host");
    expect(markup).toContain("vcs-history-table");
    expect(markup).toContain('d="M ');
    expect(markup.match(/<circle /g)?.length).toBe(3);
    expect(markup).toContain("branch b");
    expect(markup).toContain("feature-b");
  });

  it("renders raster host canvas surface from document sync scene", () => {
    const markup = renderToStaticMarkup(
      createElement(RasterHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.viewport",
          controllerId: "raster-play",
          componentKind: "raster",
          raster: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeTool: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            viewMode: "composite",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-raster-canvas-surface");
    expect(markup).toContain('data-surface-id="raster.play.viewport"');
    expect(markup).toContain('data-view-mode="composite"');
  });

  it("renders raster navigator host with the composite viewport overlay channel", () => {
    const markup = renderToStaticMarkup(
      createElement(RasterHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.navigator",
          controllerId: "raster-play",
          componentKind: "raster",
          raster: {
            documentSyncJson: '{"schema":"raster.document","id":"raster","layers":[]}',
            assetsJson: "{}",
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            selectionJson: "[]",
            activeTool: "selectMarquee",
            brushSize: 24,
            brushOpacity: 1,
            viewMode: "navigator",
            compositeViewportJson: '{"width":640,"height":480}',
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-raster-canvas-surface");
    expect(markup).toContain('data-view-mode="navigator"');
  });

  it("renders raster host empty fallback without a scene", () => {
    const markup = renderToStaticMarkup(
      createElement(RasterHost, {
        node: {
          type: "componentScene",
          surfaceId: "raster.play.composite",
          controllerId: "raster-play",
          componentKind: "raster",
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-raster-empty");
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
    const pointsJson = JSON.stringify([{ x: 10, y: 20 }, { x: 30, y: 50 }]);
    const overlay = computeDagMarqueeOverlay(pointsJson, false, "rectangle");
    expect(overlay).toEqual({ kind: "rect", x: 10, y: 20, width: 20, height: 30, coverage: "full" });
  });

  it("computes a lasso overlay carrying the raw points for the lasso method", () => {
    const points = [{ x: 10, y: 20 }, { x: 30, y: 50 }, { x: 15, y: 40 }];
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
    const overlay = computeDagMarqueeOverlay(JSON.stringify([{ x: 0, y: 0 }, { x: 40, y: 25 }]), false, "rectangle");
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
    const overlay = computeDagMarqueeOverlay(JSON.stringify([{ x: 0, y: 0 }, { x: 40, y: 25 }, { x: 5, y: 30 }]), false, "lasso");
    if (!overlay || overlay.kind !== "lasso") throw new Error("expected lasso overlay");
    const markup = renderToStaticMarkup(
      createElement(SelectionMarquee, { coverage: overlay.coverage ?? "full", shape: "polygon", points: overlay.points ?? [] }),
    );
    expect(markup).toContain("<polygon");
  });
});

describe("note canvas host", () => {
  const semioNoteDocument: NoteDocument = {
    schema: "note.document",
    id: "semio",
    title: "Semio Note",
    camera: { x: 0, y: 0, zoom: 1 },
    activeTool: "selectDirect",
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
      createElement(NoteCanvasHost, {
        node: {
          type: "componentScene",
          surfaceId: "note.play.composite",
          controllerId: "note-play",
          componentKind: "note-canvas",
          noteCanvas: {
            documentJson: JSON.stringify(semioNoteDocument),
            selectionJson: "[]",
            activeTool: "selectDirect",
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
      componentKind: "note-canvas",
    };
    const compositeMarkup = renderToStaticMarkup(
      createElement(NoteCanvasHost, {
        node: { ...baseNode, noteCanvas: { documentJson: JSON.stringify(semioNoteDocument), selectionJson: "[]", activeTool: "selectDirect", viewMode: "composite", interactive: true } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(compositeMarkup).toContain("note-viewport-grid");

    const navigatorMarkup = renderToStaticMarkup(
      createElement(NoteCanvasHost, {
        node: { ...baseNode, noteCanvas: { documentJson: JSON.stringify(semioNoteDocument), selectionJson: "[]", activeTool: "selectDirect", viewMode: "navigator", interactive: false } },
        onAction: noopAction,
      }) as ReactElement,
    );
    expect(navigatorMarkup).not.toContain("note-viewport-grid");
  });

  it("resizes with a minimum size and scales ink points when a group is resized", () => {
    const fromBounds = { x: 0, y: 0, width: 100, height: 100 };
    const shrunk = noteResizeBounds(fromBounds, "e", -1000, 0);
    expect(shrunk.width).toBe(8);

    const ink: NoteInkBlock = { kind: "ink", id: "ink-1", name: "Ink", x: 0, y: 0, width: 100, height: 100, visible: true, locked: false, points: [[0, 0], [100, 100]], strokeWidth: 2, color: [0, 0, 0, 1] };
    const scaled = noteScaleBlockWithinGroup(ink, { x: 0, y: 0, width: 100, height: 100 }, { x: 0, y: 0, width: 200, height: 50 });
    expect(scaled.kind).toBe("ink");
    if (scaled.kind === "ink") expect(scaled.points).toEqual([[0, 0], [200, 50]]);
  });

  it("splits an ink stroke into fragments when erasing its middle point", () => {
    const ink: NoteInkBlock = { kind: "ink", id: "ink-1", name: "Ink", x: 0, y: 0, width: 80, height: 1, visible: true, locked: false, points: [[0, 0], [40, 0], [80, 0]], strokeWidth: 2, color: [0, 0, 0, 1] };
    const fragments = noteEraseInkPointsInBlock(ink, 40, 0, 5);
    expect(fragments).toHaveLength(0);
    const wideStroke: NoteInkBlock = { ...ink, points: [[0, 0], [10, 0], [40, 0], [70, 0], [80, 0]] };
    const splitFragments = noteEraseInkPointsInBlock(wideStroke, 40, 0, 5);
    expect(splitFragments).toHaveLength(2);
  });

  it("round-trips bold and link marks between paragraphs and html", () => {
    const html = noteParagraphsToHtml([{ runs: [{ text: "hello", bold: true, link: "https://semio.tech" }] }]);
    expect(html).toContain("<strong>");
    expect(html).toContain('href="https://semio.tech"');
  });

  it("round-trips a clipboard payload of note blocks", () => {
    const payload = noteClipboardPayload([semioNoteDocument.blocks[1]!]);
    const parsed = noteBlocksFromClipboardPayload(payload);
    expect(parsed).toHaveLength(1);
    expect(parsed?.[0]?.kind).toBe("math");
  });

  it("computes ink block bounds from its local points", () => {
    const ink: NoteInkBlock = { kind: "ink", id: "ink-1", name: "Ink", x: 10, y: 10, width: 1, height: 1, visible: true, locked: false, points: [[0, 0], [5, 5]], strokeWidth: 2, color: [0, 0, 0, 1] };
    expect(noteBlockBounds(ink)).toEqual({ x: 10, y: 10, width: 5, height: 5 });
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
        tools: [{ id: "static-tool", kind: "button" as const, iconId: "save", controllerId: "cad-play", action: "save" }],
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
    expect(chrome.engagement?.input?.value).toBe("Box");
    expect(chrome.engagement?.possibleEngagements?.[0]?.label).toBe("Box");
    const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
    expect(measuresMarkup).toContain("Render Mode");
  });
});

describe("partitionWindowMeasures", () => {
  const toolGroup = (id: string, activeToolId?: string): WindowMeasure => ({ kind: "group", id, label: id, activeToolId, children: [] });
  const slider = (id: string): WindowMeasure => ({ kind: "slider", id, value: 1, min: 0, max: 2, onChange: { controllerId: "c", action: "a" } });

  it("routes a tagged group to toolOptions only when its tool is active", () => {
    const measures = [toolGroup("brush-params", "brush"), slider("zoom")];
    const active = partitionWindowMeasures(measures, "brush");
    expect(active.toolOptions.map((m) => m.id)).toEqual(["brush-params"]);
    expect(active.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("drops a tagged group from both buckets when a different or no tool is active", () => {
    const measures = [toolGroup("brush-params", "brush"), slider("zoom")];
    const other = partitionWindowMeasures(measures, "fill");
    expect(other.toolOptions).toEqual([]);
    expect(other.general.map((m) => m.id)).toEqual(["zoom"]);
    const none = partitionWindowMeasures(measures, undefined);
    expect(none.toolOptions).toEqual([]);
    expect(none.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("keeps untagged groups and non-group measures in general, unaffected by the active tool", () => {
    const measures = [toolGroup("grid"), slider("zoom")];
    const { general, toolOptions } = partitionWindowMeasures(measures, "brush");
    expect(general.map((m) => m.id)).toEqual(["grid", "zoom"]);
    expect(toolOptions).toEqual([]);
  });

  it("wires a tool-scoped group into spawnedWindowChromeForKind's toolOptions slot only when its tool is active", () => {
    const kind = { id: "w", label: "W", bodyKey: "b", surfaceKind: "raster", options: { engagement: { kind: "none" as const }, measures: [] } } as unknown as AppWindowKindDefinition;
    const brushGroup: WindowMeasure = {
      kind: "group",
      id: "brush-params",
      label: "Brush",
      defaultOpen: true,
      activeToolId: "brush",
      children: [{ kind: "slider", id: "size", label: "Brush size", value: 4, min: 1, max: 10, onChange: { controllerId: "c", action: "setSize" } }],
    };
    const measures = { [kind.id]: [brushGroup] };
    const activeChrome = spawnedWindowChromeForKind(kind, {}, measures, "brush", noopAction);
    expect(renderToStaticMarkup(activeChrome.toolOptions as ReactElement)).toContain("Brush size");
    expect(activeChrome.measures).toBeUndefined();
    const idleChrome = spawnedWindowChromeForKind(kind, {}, measures, "fill", noopAction);
    expect(idleChrome.toolOptions).toBeUndefined();
    expect(idleChrome.measures).toBeUndefined();
  });
});

describe("toolbar ribbon", () => {
  it("sorts tool nodes by order", () => {
    const sorted = sortToolNodes([
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

    const noActive = buildToolbarRibbonSegments(tree, []);
    expect(noActive).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);

    const oneActive = buildToolbarRibbonSegments(tree, ["construct"]);
    expect(oneActive[0]).toMatchObject({ kind: "picker", depth: 0 });
    expect(oneActive[1]).toMatchObject({ kind: "picker", depth: 1, collections: tree[1].children });
    expect(oneActive).toHaveLength(2);

    const twoActive = buildToolbarRibbonSegments(tree, ["construct", "construct-tools"]);
    const toolsSegment = twoActive.find((segment) => segment.kind === "tools" && segment.items.some((item) => item.id === "box"));
    expect(toolsSegment).toMatchObject({ depth: 2 });
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
    expect(buildToolbarRibbonSegments(tree, ["nonexistent"])).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);
  });

  it("emits a picker segment alongside loose leaves at the same depth", () => {
    const segments = buildToolbarRibbonSegments(
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
      { kind: "tools", items: [expect.objectContaining({ id: "undo" })], depth: 0 },
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
    expect(reconcileToolPath(tree, ["a", "x"])).toEqual(["a", "x"]);
    expect(reconcileToolPath(tree, ["a", "gone"])).toEqual(["a"]);
    expect(reconcileToolPath(tree, ["gone"])).toEqual([]);
    expect(reconcileToolPath(tree, [])).toEqual([]);
  });

  it("buckets top-level tool nodes into ordered category collections (uncategorized nodes default to tools now that the Actions category is gone)", () => {
    const grouped = groupToolNodesByCategory([
      { id: "sel", kind: "toggle", iconId: "cursor", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
      { id: "act", kind: "button", iconId: "wand", controllerId: "x", action: "run" },
      { id: "tool", kind: "toggle", iconId: "pen", controllerId: "x", action: "pen" },
      { id: "sync", kind: "toggle", iconId: "cloud", controllerId: "x", action: "sync", category: "sync" },
    ]);
    expect(grouped.map((node) => node.id)).toEqual(["selection", "tools", "history", "sync"]);
    expect(grouped.every((node) => node.kind === "collection")).toBe(true);
  });

  it("drops separator-only category buckets so an empty group never appears as a picker option", () => {
    const grouped = groupToolNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "tools" },
      { id: "sep", kind: "separator" },
    ]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0].id).toBe("tools");
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
    const grouped = groupToolNodesByCategory([selectionCollection]);
    expect(grouped).toEqual([{ ...selectionCollection, order: 0 }]);
    const segments = buildToolbarRibbonSegments(grouped, ["lowpoly-tools-selection"]);
    const toolsSegment = segments.find((segment) => segment.kind === "tools" && segment.items.some((item) => item.id === "mesh"));
    expect(toolsSegment).toBeTruthy();
  });

  it("still wraps a category with multiple top-level nodes in a synthetic collection", () => {
    const grouped = groupToolNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "tools" },
      { id: "b", kind: "button", iconId: "box", controllerId: "x", action: "b", category: "tools" },
    ]);
    expect(grouped).toEqual([{ id: "tools", kind: "collection", iconId: "wrench", text: "tools", order: 0, category: "tools", children: expect.any(Array) }]);
  });

  it("scopes grouping to the given categories only", () => {
    const nodes = [
      { id: "sel", kind: "toggle", iconId: "cursor", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
    ];
    expect(groupToolNodesByCategory(nodes, ["selection", "tools"]).map((node) => node.id)).toEqual(["selection"]);
    expect(groupToolNodesByCategory(nodes, ["tools", "history"]).map((node) => node.id)).toEqual(["history"]);
  });

  it("deduplicates tool nodes by id across window tool lists for a single shared footer entry", () => {
    const history = { id: "s-play.history", kind: "collection" as const, iconId: "clock", category: "history" as const, children: [] };
    const deduped = dedupeToolNodesById([
      [history, { id: "leaf-a", kind: "button" as const, iconId: "box", controllerId: "x", action: "a" }],
      [history],
      [],
    ]);
    expect(deduped).toEqual([history, { id: "leaf-a", kind: "button", iconId: "box", controllerId: "x", action: "a" }]);
  });

  it("renders ribbon toolbar with picker and batched toggles", () => {
    const markup = renderToStaticMarkup(
      createElement(ToolTree, {
        tools: [
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
    expect(markup).toContain('id="ui.toolbar"');
    expect(markup).toContain('data-slot="toggle-group"');
  });

  it("stacks the window toolbar ribbon upward, showing only the base picker row until a group is activated", () => {
    const markup = renderToStaticMarkup(
      createElement(ToolTree, {
        direction: "up",
        tools: [
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

  it("renders ToolTree with a custom id for per-window namespacing", () => {
    const markup = renderToStaticMarkup(
      createElement(ToolTree, {
        id: "ui.toolbar.model",
        tools: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.toolbar.model"');
    expect(markup).not.toContain('id="ui.toolbar"');
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
            presencePeersJson: JSON.stringify([
              { clientId: "client-b", name: "Ada", selectionCount: 2 },
            ]),
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
    const config = uiNodeToTreePanelConfig(
      { type: "tree", sections: [{ id: "catalogue", label: "Catalogue", items: [{ id: "s-play-catalogue.document.draw", label: "Draw" }] }] },
      noopAction,
    );
    expect(config.dragAndDropController).toBeUndefined();
  });

  it("resolves a fixture widget id to its media-graph instance id, independent of selection state", () => {
    const fixtureJson = JSON.stringify({ widgets: [{ id: "widget-1", params: { instanceId: "app-1" } }, { id: "widget-2", params: {} }] });
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

// 🧰 Window Actions & Tools Contract (WS-2): staged argument forms (P1/P2), palette redirect (P3),
// keybinding rule (P4), and registry-derived tool activation (P5).
describe("window action panel — staging and single dispatch (P1/P2)", () => {
  afterEach(() => cleanup());

  const numberArg = (id: string, required: boolean, def?: number): ActionArgDef => ({ id, label: id[0]!.toUpperCase() + id.slice(1), control: { kind: "number" }, required, ...(def === undefined ? {} : { default: def }) });

  const twoArgAction: ActionDefinition = { id: "extrude", label: "Extrude", kind: "operation", inPalette: true, args: [numberArg("depth", true), numberArg("segments", true)] };
  const zeroArgAction: ActionDefinition = { id: "flatten", label: "Flatten", kind: "operation", inPalette: true, args: [] };
  const defaultedAction: ActionDefinition = { id: "bevel", label: "Bevel", kind: "operation", inPalette: true, args: [numberArg("radius", true, 2)] };

  function Harness({ actions, onExecute, disabled }: { actions: readonly ActionDefinition[]; onExecute: (descriptor: unknown) => void; disabled?: boolean }): ReactElement {
    const [expanded, setExpanded] = useState<string | null>(null);
    const [staged, setStaged] = useState<Record<string, Record<string, unknown>>>({});
    return createElement(WindowActionPanel, {
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

  it("stages both args locally, dispatches nothing until Execute, then fires exactly one merged descriptor and keeps staged values", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [twoArgAction], onExecute }));
    fireEvent.click(buttonByText(container, "Extrude…"));
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
    fireEvent.click(buttonByText(required.container, "Extrude…"));
    expect(buttonByText(required.container, "Execute").disabled).toBe(true);
    const inputs = required.container.querySelectorAll('input[type="number"]');
    fireEvent.change(inputs[0]!, { target: { value: "3" } });
    expect(buttonByText(required.container, "Execute").disabled).toBe(true);
    fireEvent.change(inputs[1]!, { target: { value: "2" } });
    expect(buttonByText(required.container, "Execute").disabled).toBe(false);
    cleanup();

    const defaulted = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(buttonByText(defaulted.container, "Bevel…"));
    expect(buttonByText(defaulted.container, "Execute").disabled).toBe(false);
    fireEvent.click(buttonByText(defaulted.container, "Execute"));
    expect(onExecute).toHaveBeenLastCalledWith({ controllerId: "c", action: "bevel", args: { radius: 2 } });
  });

  it("Reset restores defaults while keeping the form expanded", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [defaultedAction], onExecute }));
    fireEvent.click(buttonByText(container, "Bevel…"));
    const input = () => container.querySelector('input[type="number"]') as HTMLInputElement;
    expect(input().value).toBe("2");
    fireEvent.change(input(), { target: { value: "9" } });
    expect(input().value).toBe("9");
    fireEvent.click(buttonByText(container, "Reset"));
    // still expanded (Execute/Reset buttons present) and back to the default effective value
    expect(input().value).toBe("2");
    expect([...container.querySelectorAll("button")].some((b) => b.textContent?.includes("Execute"))).toBe(true);
  });

  it("a zero-arg action row is the execute button and fires immediately with no args object", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute }));
    fireEvent.click(buttonByText(container, "Flatten"));
    expect(onExecute).toHaveBeenCalledTimes(1);
    expect(onExecute).toHaveBeenCalledWith({ controllerId: "c", action: "flatten" });
  });

  it("renders every row disabled when an active tool gates actions", () => {
    const onExecute = vi.fn();
    const { container } = render(createElement(Harness, { actions: [zeroArgAction], onExecute, disabled: true }));
    fireEvent.click(buttonByText(container, "Flatten"));
    expect(onExecute).not.toHaveBeenCalled();
    expect(buttonByText(container, "Flatten").disabled).toBe(true);
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

describe("registry-derived tools and activation (P5)", () => {
  const tools: ToolDefinition[] = [
    { id: "select", label: "Select", iconId: "mouse-pointer", category: "selection", allowsActionsWhileActive: true },
    { id: "brush", label: "Brush", iconId: "brush", group: "paint", category: "tools", allowsActionsWhileActive: false },
    { id: "erase", label: "Erase", iconId: "eraser", group: "paint", category: "tools", allowsActionsWhileActive: false },
  ];
  const app = { controllerId: "draw", tools } satisfies Pick<AppDefinition, "controllerId" | "tools">;

  it("resolveWindowTools scopes to the window kind's refs, falling back to all app tools when unset", () => {
    expect(resolveWindowTools(app, { tools: ["brush"] } as Pick<AppWindowKindDefinition, "tools">).map((t) => t.id)).toEqual(["brush"]);
    expect(resolveWindowTools(app, { tools: [] } as unknown as Pick<AppWindowKindDefinition, "tools">).map((t) => t.id)).toEqual(["select", "brush", "erase"]);
  });

  it("derives grouped toolbar nodes with the active tool pressed and a setActiveTool onChange tagged by window", () => {
    const nodes = resolveWindowToolNodes(app, { tools: [] } as unknown as Pick<AppWindowKindDefinition, "tools">, "brush", "w1");
    const select = nodes.find((node) => node.id === "select");
    expect(select && select.kind === "toggle" ? select.pressed : undefined).toBe(false);
    const paint = nodes.find((node) => node.id === "group:paint");
    expect(paint?.kind).toBe("collection");
    const brush = paint && paint.kind === "collection" ? paint.children.find((child) => child.id === "brush") : undefined;
    expect(brush && brush.kind === "toggle" ? brush.pressed : undefined).toBe(true);
    expect(brush && brush.kind === "toggle" && "onChange" in brush ? brush.onChange : undefined).toEqual({ controllerId: "draw", action: "setActiveTool", args: { toolId: "brush", windowId: "w1" } });
  });

  it("deriveToolNodes twin marks exactly the active tool pressed", () => {
    const nodes = deriveToolNodes("draw", [{ id: "a", label: "A", iconId: "x" }, { id: "b", label: "B", iconId: "y" }], "b");
    expect(nodes.map((node) => (node.kind === "toggle" ? node.pressed : undefined))).toEqual([false, true]);
  });

  it("resolveToolActivation toggles: click activates, re-click or empty deactivates", () => {
    expect(resolveToolActivation(null, "brush")).toBe("brush");
    expect(resolveToolActivation("brush", "erase")).toBe("erase");
    expect(resolveToolActivation("brush", "brush")).toBeNull();
    expect(resolveToolActivation("brush", "")).toBeNull();
    expect(resolveToolActivation(undefined, "")).toBeNull();
  });

  it("resolveWindowActions surfaces panel-eligible actions and frameworkHistoryToolNodes derives History buttons", () => {
    const actionsApp = {
      controllerId: "draw",
      actions: [
        { id: "extrude", label: "Extrude", kind: "operation", inPalette: true, args: [] },
        { id: "undo", label: "Undo", kind: "history", iconId: "undo", inPalette: true, args: [] },
        { id: "setActiveTool", label: "Set Active Tool", kind: "view", inPalette: false, args: [] },
      ] as ActionDefinition[],
      windowKinds: [{ actions: [] as string[] }],
    };
    const resolved = resolveWindowActions(actionsApp, { actions: [] as string[] });
    // orphan operation appears; history + setActiveTool are never panel-eligible orphans
    expect(resolved.map((action) => action.id)).toEqual(["extrude"]);
    const history = frameworkHistoryToolNodes({ controllerId: "draw", actions: actionsApp.actions });
    expect(history).toHaveLength(1);
    expect(history[0]).toMatchObject({ id: "undo", kind: "button", category: "history", onPress: { controllerId: "draw", action: "undo" } });
  });
});
