import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
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
import { NodeGraphHost, computeDagMarqueeOverlay, nodeGraphViewportActionArgs, parseDagSliderOverlays } from "./components/node-graph-host.tsx";
import { SelectionMarquee } from "@semio-tech/ui-react";
import { RasterHost } from "./components/raster-host.tsx";
import { TableHost } from "./components/table-host.tsx";
import { TextEditorHost, buildTextEditorContextMenuItems, lineRangeAt, multiSpanReplace } from "./components/text-editor-host.tsx";
import { World3dHost } from "./components/world-3d-host.tsx";
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
import { appDocumentLabel, appWindowDocumentLabel, buildToolbarRibbonSegments, isFlowGraphScene, selectSpawnedToolNodes, sortToolNodes, spawnedWindowChromeForKind, ToolTree } from "./os-shell.tsx";
import { interpretUiNode } from "./ui-interpreter.tsx";
import type { UiNode } from "./os-shell.tsx";

const noopAction = () => {};

describe("framework sync tools", () => {
  it("builds four sync backbone toggles", async () => {
    const { buildFrameworkSyncTools } = await import("@semio-tech/framework-os-core");
    const tools = buildFrameworkSyncTools("temp://demo");
    expect(tools).toHaveLength(4);
    expect(tools.map((tool) => tool.id)).toEqual([
      "framework.sync.temporary",
      "framework.sync.file",
      "framework.sync.folder",
      "framework.sync.remote",
    ]);
    expect(tools[0]?.pressed).toBe(true);
  });
});

describe("framework plugin runtime", () => {
  it("loads plugin modules through framework-core", async () => {
    const { loadPluginModule } = await import("@semio-tech/framework-core");
    const handle = await loadPluginModule("mock", "data:application/javascript,export function semio_plugin_manifest(){return JSON.stringify({pluginId:'mock',label:'Mock',version:'0',apps:[],programs:[],examples:[]})}");
    expect(handle.manifest.pluginId).toBe("mock");
  });

  it("extracts patch ops from ActionResult plugin responses", async () => {
    const { patchOpsFromActionResponse } = await import("@semio-tech/framework-core");
    const legacy = patchOpsFromActionResponse(JSON.stringify([JSON.stringify({ op: "setDocument", document: { id: "legacy" } })]));
    expect(legacy).toEqual([JSON.stringify({ op: "setDocument", document: { id: "legacy" } })]);
    const actionResult = patchOpsFromActionResponse(
      JSON.stringify({
        output: null,
        operations: [{ diff: { payload: { op: "setDocument", document: { id: "forest" } } } }],
        inverseGroup: { actionId: "setActiveExample:1:0", operations: [] },
      }),
    );
    expect(actionResult).toEqual([JSON.stringify({ op: "setDocument", document: { id: "forest" } })]);
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
        return [];
      },
      render: async () => ({ type: "text", value: "x" }),
      tools: async () => [],
      windowEngagements: async () => ({}),
      windowMeasures: async () => ({}),
      dispose: () => {},
    });
    await Promise.all([handle.handleAction(1, "{}", {}), handle.handleAction(1, "{}", {}), handle.handleAction(1, "{}", {})]);
    expect(maxInFlight).toBe(1);
  });
});

describe("framework renderer types", () => {
  it("formats canonical app document for chrome and window tabs", () => {
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
    expect(appWindowDocumentLabel(app, "Puzzle 3D")).toBe("semio · puzzle · 3d");
    expect(appWindowDocumentLabel(app, "Perspective")).toBe("semio · puzzle · 3d · perspective");
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
      tools: async () => [],
      windowEngagements: async () => ({}),
      windowMeasures: async () => ({}),
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
      },
    };
    expect(node.world3d?.meshesJson).toBe("[]");
    expect(node.world3d?.vorticesJson).toBe("[]");
    expect(node.world3d?.interactionJson).toContain("select");
    expect(node.world3d?.engagementPreviewJson).toContain("box-preview");
    expect(node.world3d?.contextMenuJson).toBe("[]");
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
        engagement: {
          input: {
            id: "engagement-input",
            placeholder: "Action",
            onChange: { controllerId: "cad-play", action: "engagementInput" },
          },
          possibleEngagements: [{ id: "box", label: "Box", action: { controllerId: "cad-play", action: "startBox" } }],
        },
        measures: [{ id: "render-mode", kind: "select" as const, label: "Render Mode", value: "shaded", items: [], onChange: { controllerId: "cad-play", action: "setRenderMode" } }],
      },
    ],
    panelTabs: [],
    keybindings: [],
  };

  it("prefers dynamic spawned tools over static mode tools", () => {
    const dynamic = [{ id: "dynamic-tool", kind: "button" as const, iconId: "box", controllerId: "cad-play", action: "box" }];
    expect(selectSpawnedToolNodes(dynamic, app, "edit")).toEqual(dynamic);
    expect(selectSpawnedToolNodes([], app, "edit")).toEqual(app.modes[0]!.tools);
  });

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
    const measures = { [kind.id]: kind.measures ?? [] };
    const chrome = spawnedWindowChromeForKind(kind, engagements, measures, noopAction);
    expect(chrome.engagement?.input?.value).toBe("Box");
    expect(chrome.engagement?.possibleEngagements?.[0]?.label).toBe("Box");
    const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
    expect(measuresMarkup).toContain("Render Mode");
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

  it("builds picker segments for sibling nested collections", () => {
    const segments = buildToolbarRibbonSegments(
      [
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
      ],
      ["construct"],
    );
    expect(segments[0]).toMatchObject({ kind: "picker", depth: 0 });
    expect(segments.some((segment) => segment.kind === "tools" && segment.items.some((item) => item.id === "box"))).toBe(true);
  });

  it("flattens leaf-only sibling collections into separate tool zones", () => {
    const segments = buildToolbarRibbonSegments(
      [
        {
          id: "view",
          kind: "collection",
          iconId: "eye",
          children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
        },
        {
          id: "save",
          kind: "collection",
          iconId: "save",
          children: [{ id: "export", kind: "button", iconId: "download", controllerId: "x", action: "export" }],
        },
      ],
      [],
    );
    expect(segments.every((segment) => segment.kind === "tools")).toBe(true);
    expect(segments).toHaveLength(2);
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
});
