import { useCallback, useEffect, useMemo, useRef, useState, type DragEvent, type KeyboardEvent } from "react";
import {
  CATALOGUE_DRAG_MIME,
  CanvasPickMenu,
  ContextMenuController,
  Diagram,
  getActiveCatalogueDragPayload,
  Handle,
  Position,
  SelectionMarquee,
  Slider,
  useCanvasPickInteraction,
  useCanvasAppearanceSync,
  type CanvasPickTarget,
  type Edge,
  type Node,
  type NodeProps,
  type NodeTypes,
} from "@semio-tech/ui-react";
import { GraphWasmCanvas, type GraphWasmSession } from "@semio-tech/infinite-cavas-react-renderer";
import { syncSessionCanvasTheme } from "@semio-tech/ui-styling";
import type { ActionDescriptor, ComponentSceneHostProps, NodeGraphScene, PresencePeer, UiComponentSceneNode } from "@semio-tech/framework-core";
import { createFlowSession, createGraphSession, isFlowGraphScene, nodeGraphActions, useUIFindSafe, type FlowWasmSession } from "../os-shell.tsx";

//#region Types
type MediaGraphPort = {
  readonly id: string;
  readonly resourceKind?: string;
  readonly direction?: string;
  readonly label?: string;
};

type MediaGraphNodeRecord = {
  readonly id: string;
  readonly instanceId?: string;
  readonly label?: string;
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly inputs?: readonly MediaGraphPort[];
  readonly outputs?: readonly MediaGraphPort[];
};

type MediaGraphEdgeRecord = {
  readonly id: string;
  readonly sourceNodeId: string;
  readonly sourcePortId: string;
  readonly targetNodeId: string;
  readonly targetPortId: string;
};

type MediaGraphNodeData = {
  readonly label: string;
  readonly inputs: readonly MediaGraphPort[];
  readonly outputs: readonly MediaGraphPort[];
  readonly width: number;
  readonly height: number;
};

type DiagramViewport = { readonly x: number; readonly y: number; readonly zoom: number };

type GraphFindItem = { readonly id: string; readonly label: string; readonly category?: string };

type GraphContextMenuItem = {
  readonly id: string;
  readonly label: string;
  readonly action: string;
  readonly args?: Record<string, unknown>;
};

type FrameworkGraphSession = GraphWasmSession & {
  syncFromSceneJson(json: string): void;
  pointerDownScreen(sx: number, sy: number, button: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerMoveScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  pointerUpScreen(sx: number, sy: number, shift: boolean, ctrlOrMeta: boolean, alt: boolean): void;
  wheelScreen(sx: number, sy: number, deltaX: number, deltaY: number, zoomGesture: boolean): void;
  labelOverlayPaintStateJson(): string;
  paramOverlayPaintStateJson(): string;
  stepperOverlayStateJson(): string;
  sliderOverlayStateJson(): string;
  selectionUnionBoundsScreenJson(): string;
  selectionPreviewPointsJson(): string;
  selectionPreviewCrossing(): boolean;
  selectedNodeIdsJson(): string;
  hoveredNodeId(): string | null | undefined;
  hoveredChannelJson(): string;
  cameraJson(): string;
  takePendingOpenInstanceId(): string | null | undefined;
  pickTargetsAtScreenJson(sx: number, sy: number): string;
  setHover?(widgetId: string | null): void;
  setHoverChannel?(widgetId: string | null, port?: string | null): void;
  alignSelection?(mode: string): void;
  fixtureJson?(): string;
  setCanvasThemeJson?(json: string): void;
};
//#endregion Types

//#region Viewport
export function nodeGraphViewportActionArgs(cameraJson: string): { readonly viewportJson: string } {
  return { viewportJson: cameraJson };
}
//#endregion Viewport

//#region Parsing
function parseViewport(viewportJson: string): DiagramViewport {
  try {
    const parsed = JSON.parse(viewportJson) as Partial<DiagramViewport>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

function parseJsonArray<T>(json: string | undefined): readonly T[] {
  if (!json) return [];
  try {
    return JSON.parse(json) as T[];
  } catch {
    return [];
  }
}

/** @emoji 🔎 Resolves a flow fixture widget id to the media-graph instance id it previews, used to open an app instance without depending on plugin-side selection state. */
export function resolveFixtureWidgetInstanceId(fixtureJson: string | undefined, widgetId: string | undefined | null): string | undefined {
  if (!fixtureJson || !widgetId) return undefined;
  try {
    const fixture = JSON.parse(fixtureJson) as {
      readonly widgets?: readonly { readonly id?: string; readonly params?: { readonly instanceId?: string } }[];
    };
    return fixture.widgets?.find((widget) => widget.id === widgetId)?.params?.instanceId;
  } catch {
    return undefined;
  }
}

export interface CatalogueAppDragPayload {
  readonly programId: string;
  readonly appId: string;
  readonly label?: string;
}

/** @emoji 🎯 Parses a catalogue drag payload; returns null for non-catalogue-app payloads (garbage/legacy descriptors). */
export function parseCatalogueAppDragPayload(raw: string): CatalogueAppDragPayload | null {
  try {
    const parsed = JSON.parse(raw) as { readonly programId?: string; readonly appId?: string; readonly label?: string };
    if (!parsed.programId || !parsed.appId) return null;
    return { programId: parsed.programId, appId: parsed.appId, label: parsed.label };
  } catch {
    return null;
  }
}

/** @emoji 👻 Builds the ghost widget descriptor shown while a catalogue app is dragged over the media graph. */
export function catalogueGhostDescriptorJson(payload: CatalogueAppDragPayload): string {
  return JSON.stringify({ kind: "neuron", neuronKind: payload.label ?? payload.appId });
}

function portLabel(port: MediaGraphPort): string {
  if (port.label) return port.label;
  const segments = port.id.split(":");
  return segments[segments.length - 1] ?? port.id;
}

function mediaGraphNodesToDiagramNodes(records: readonly MediaGraphNodeRecord[]): Node<MediaGraphNodeData>[] {
  return records.map((record) => ({
    id: record.id,
    type: "mediaGraph",
    position: { x: record.x ?? 0, y: record.y ?? 0 },
    data: {
      label: record.label?.trim() || record.instanceId || record.id,
      inputs: record.inputs ?? [],
      outputs: record.outputs ?? [],
      width: record.width ?? 180,
      height: record.height ?? 72,
    },
  }));
}

function mediaGraphEdgesToDiagramEdges(records: readonly MediaGraphEdgeRecord[]): Edge[] {
  return records.map((record) => ({
    id: record.id,
    source: record.sourceNodeId,
    target: record.targetNodeId,
    sourceHandle: record.sourcePortId,
    targetHandle: record.targetPortId,
  }));
}
//#endregion Parsing

//#region Keyboard
function isEditableGraphKeyTarget(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  const tag = target.tagName;
  if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
  if (target.isContentEditable) return true;
  return target.closest("[contenteditable='true'], [role='textbox']") != null;
}

function handleGraphKeyboard(event: KeyboardEvent<HTMLDivElement>, editable: boolean, parsedNodes: readonly MediaGraphNodeRecord[], dispatch: (action: string, args?: Record<string, unknown>) => void) {
  if (!editable || isEditableGraphKeyTarget(event.target)) return;
  const mod = event.metaKey || event.ctrlKey;
  if (mod && event.key.toLowerCase() === "a") {
    event.preventDefault();
    dispatch("setMediaNodeSelection", { nodeIds: parsedNodes.map((node) => node.id) });
    return;
  }
  if (event.key === "Escape") {
    event.preventDefault();
    dispatch("setMediaNodeSelection", { nodeIds: [] });
    return;
  }
  if (event.key === "Delete" || event.key === "Backspace") {
    event.preventDefault();
    dispatch("deleteSelection", {});
  }
}
//#endregion Keyboard

//#region DiagramNode
function MediaGraphDiagramNode({ data }: NodeProps<MediaGraphNodeData>) {
  const inputCount = Math.max(data.inputs.length, 1);
  const outputCount = Math.max(data.outputs.length, 1);
  const rowCount = Math.max(inputCount, outputCount);
  const rowHeight = 18;
  const bodyHeight = Math.max(data.height, 56 + rowCount * rowHeight);
  return (
    <div className="rounded border border-border bg-panel text-panel-foreground shadow-sm" style={{ width: data.width, minHeight: bodyHeight }}>
      <div className="border-b border-border px-2 py-1 text-xs font-medium">{data.label}</div>
      <div className="relative px-2 py-1 text-[10px] leading-[18px]">
        {Array.from({ length: rowCount }, (_, rowIndex) => {
          const input = data.inputs[rowIndex];
          const output = data.outputs[rowIndex];
          const top = 8 + rowIndex * rowHeight;
          return (
            <div key={`${input?.id ?? "in"}:${output?.id ?? "out"}:${rowIndex}`} className="relative h-[18px]">
              {input ? (
                <>
                  <Handle id={input.id} type="target" position={Position.Left} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="pl-3 text-muted-foreground">{portLabel(input)}</span>
                </>
              ) : null}
              {output ? (
                <>
                  <Handle id={output.id} type="source" position={Position.Right} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="absolute right-3 top-0 text-right text-muted-foreground">{portLabel(output)}</span>
                </>
              ) : null}
            </div>
          );
        })}
      </div>
    </div>
  );
}

const mediaGraphNodeTypes: NodeTypes = { mediaGraph: MediaGraphDiagramNode };
//#endregion DiagramNode

//#region WasmGraphSurface
function WasmGraphSurface({
  scene,
  surfaceId,
  controllerId,
  editable,
  contextMenuItems,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly contextMenuItems: readonly GraphContextMenuItem[];
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const sessionRef = useRef<FrameworkGraphSession | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [overlaySize, setOverlaySize] = useState({ w: 0, h: 0 });
  const [paramStateJson, setParamStateJson] = useState("{}");
  const [stepperStateJson, setStepperStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const sceneJson = useMemo(() => sceneToSyncJson(scene), [scene]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    try {
      paintDagLabelOverlays(session.labelOverlayPaintStateJson(), labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredNodeId() ?? null,
        selectedIds: parseDagNodeIdArray(session.selectedNodeIdsJson()),
        preselect: { ids: [], removedIds: [] },
        dimmedIds: [],
      });
    } catch {
      /* gpu not ready */
    }
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), "rectangle"));
    try {
      setParamStateJson(session.paramOverlayPaintStateJson());
      setStepperStateJson(session.stepperOverlayStateJson());
      setSliderStateJson(session.sliderOverlayStateJson());
    } catch {
      /* session not ready */
    }
    setOverlaySize({ w: rect.width, h: rect.height });
  }, []);

  useEffect(() => {
    sessionRef.current?.syncFromSceneJson(sceneJson);
    paintOverlays();
  }, [sceneJson, paintOverlays]);

  const onSessionReady = useCallback(
    (session: GraphWasmSession) => {
      sessionRef.current = session as FrameworkGraphSession;
      sessionRef.current.syncFromSceneJson(sceneJson);
      paintOverlays();
    },
    [sceneJson, paintOverlays],
  );

  const [wasmSession, setWasmSession] = useState<FrameworkGraphSession | null>(null);

  useEffect(() => {
    let cancelled = false;
    void createGraphSession().then((session) => {
      if (!cancelled) setWasmSession(session as FrameworkGraphSession);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  const sessionFactory = useCallback(() => {
    if (wasmSession) return wasmSession;
    return {
      attachCanvas: async () => undefined,
      setSize: () => {},
      renderFrame: () => {},
      syncFromSceneJson: () => {},
      pointerDownScreen: () => {},
      pointerMoveScreen: () => {},
      pointerUpScreen: () => {},
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      paramOverlayPaintStateJson: () => "{}",
      stepperOverlayStateJson: () => "{}",
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => "{}",
      cameraJson: () => scene.viewportJson,
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      fixtureJson: () => "{}",
      takePendingOpenInstanceId: () => null,
    } satisfies FrameworkGraphSession;
  }, [scene.viewportJson, wasmSession]);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedNodeIdsJson()) as string[];
      dispatch(nodeGraphActions.select, { nodeIds });
      const hovered = session.hoveredNodeId();
      dispatch(nodeGraphActions.hover, { hoverJson: hovered ? JSON.stringify({ nodeId: hovered }) : null });
      dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(session.cameraJson()));
      const openId = session.takePendingOpenInstanceId?.();
      if (openId) dispatch("openInstance", { instanceId: openId });
    } catch {
      /* session not ready */
    }
    paintOverlays();
  }, [dispatch, paintOverlays]);

  const commitGraphFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session?.fixtureJson) return;
    try {
      const fixtureJson = session.fixtureJson();
      dispatch(nodeGraphActions.edit, { ops: [{ op: "setFixture", fixtureJson }] });
    } catch {
      /* session not ready */
    }
  }, [dispatch]);

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session?.pickTargetsAtScreenJson || !container) return [];
      const rect = container.getBoundingClientRect();
      const sx = client.x - rect.left;
      const sy = client.y - rect.top;
      try {
        return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const target = focus.target;
      if (!target) {
        session.setHover?.(null);
      } else if (target.portId) {
        session.setHoverChannel?.(target.id, target.portId);
      } else {
        session.setHover?.(target.id);
      }
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onContextMenu={(event) => {
        if (!editable || contextMenuItems.length === 0) return;
        event.preventDefault();
        setContextMenu({ x: event.clientX, y: event.clientY });
      }}
      onPointerUp={emitInteractionState}
    >
      <GraphWasmCanvas className="absolute inset-0" sessionFactory={sessionFactory} onSessionReady={onSessionReady} enablePointer={false} />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      {selectionBounds ? <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} /> : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={(event) => {
          if (!editable) return;
          const session = sessionRef.current;
          if (!session?.pointerDownScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session?.pointerMoveScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session?.pointerUpScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          emitInteractionState();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session?.wheelScreen) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          emitInteractionState();
        }}
      />
      {selectionBounds && editable ? (
        <SelectionAlignChrome
          bounds={selectionBounds}
          onAlign={(mode) => {
            const session = sessionRef.current;
            if (!session?.alignSelection) return;
            session.alignSelection(alignModeToDag(mode));
            commitGraphFixture();
            session.renderFrame();
            emitInteractionState();
          }}
        />
      ) : null}
      <GraphParamOverlays stateJson={paramStateJson} logicalW={overlaySize.w} logicalH={overlaySize.h} editable={editable} onParamChange={(nodeId, portId, value) => dispatch(nodeGraphActions.edit, { op: "setParam", nodeId, portId, value })} />
      <GraphStepperOverlays
        stateJson={stepperStateJson}
        logicalW={overlaySize.w}
        logicalH={overlaySize.h}
        editable={editable}
        onStepperChange={(widgetId, fieldKey, value) => dispatch(nodeGraphActions.edit, { op: "setStepper", widgetId, fieldKey, value })}
      />
      <GraphSliderOverlays stateJson={sliderStateJson} logicalW={overlaySize.w} logicalH={overlaySize.h} editable={editable} onSliderChange={(widgetId, value) => dispatch(nodeGraphActions.edit, { op: "setSlider", widgetId, value })} />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => dispatch(item.action, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion WasmGraphSurface

//#region DiagramFallback
function DiagramGraphFallback({
  scene,
  node,
  editable,
  parsedNodes,
  parsedEdges,
  findItems,
  contextMenuItems,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly node: UiComponentSceneNode;
  readonly editable: boolean;
  readonly parsedNodes: readonly MediaGraphNodeRecord[];
  readonly parsedEdges: readonly MediaGraphEdgeRecord[];
  readonly findItems: readonly GraphFindItem[];
  readonly contextMenuItems: readonly GraphContextMenuItem[];
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const viewport = useMemo(() => parseViewport(scene.viewportJson ?? "{}"), [scene.viewportJson]);
  const initialNodes = useMemo(() => mediaGraphNodesToDiagramNodes(parsedNodes), [parsedNodes]);
  const initialEdges = useMemo(() => mediaGraphEdgesToDiagramEdges(parsedEdges), [parsedEdges]);
  const [nodes, setNodes] = useState(initialNodes);
  const [edges, setEdges] = useState(initialEdges);
  useEffect(() => {
    setNodes(initialNodes);
    setEdges(initialEdges);
  }, [initialNodes, initialEdges]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number } | null>(null);

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onDragOver={(event) => {
        if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
      }}
      onDrop={(event: DragEvent<HTMLDivElement>) => {
        if (!editable) return;
        event.preventDefault();
        const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
        if (!raw) return;
        let payload: { readonly programId?: string; readonly appId?: string };
        try {
          payload = JSON.parse(raw) as { readonly programId?: string; readonly appId?: string };
        } catch {
          return;
        }
        if (!payload.programId || !payload.appId) return;
        const rect = containerRef.current?.getBoundingClientRect();
        if (!rect) return;
        const x = (event.clientX - rect.left - viewport.x) / viewport.zoom;
        const y = (event.clientY - rect.top - viewport.y) / viewport.zoom;
        dispatch("spawnApp", { programId: payload.programId, appId: payload.appId, position: { x, y } });
      }}
      onContextMenu={(event) => {
        if (!editable || contextMenuItems.length === 0) return;
        event.preventDefault();
        setContextMenu({ x: event.clientX, y: event.clientY });
      }}
    >
      <Diagram
        className="h-full w-full"
        nodeTypes={mediaGraphNodeTypes}
        nodes={nodes}
        edges={edges}
        fitView={false}
        defaultViewport={viewport}
        minZoom={0.05}
        maxZoom={32}
        panOnDrag={[0, 1]}
        selectionOnDrag
        elementsSelectable
        nodesDraggable={editable}
        nodesConnectable={editable}
        edgesReconnectable={editable}
        onNodesChange={(nextNodes) => setNodes(nextNodes as Node<MediaGraphNodeData>[])}
        onEdgesChange={(nextEdges) => setEdges(nextEdges)}
        onNodeDragStop={
          editable
            ? (_event, draggedNode) => {
                dispatch(nodeGraphActions.edit, {
                  ops: [{ op: "move", nodeId: draggedNode.id, x: draggedNode.position.x, y: draggedNode.position.y }],
                });
              }
            : undefined
        }
        onConnect={
          editable
            ? (connection) => {
                if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
                dispatch(nodeGraphActions.edit, {
                  ops: [
                    {
                      op: "connect",
                      sourceNodeId: connection.source,
                      sourcePortId: connection.sourceHandle,
                      targetNodeId: connection.target,
                      targetPortId: connection.targetHandle,
                    },
                  ],
                });
              }
            : undefined
        }
        onNodeClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("selectInstance", { instanceId: record.instanceId });
          dispatch(nodeGraphActions.select, { nodeIds: [clickedNode.id] });
        }}
        onNodeDoubleClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
        }}
        onSelectionChange={(selection) => {
          const nodeIds = selection.nodes.map((entry) => entry.id);
          dispatch(nodeGraphActions.select, { nodeIds });
        }}
      />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => dispatch(item.action, item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion DiagramFallback

//#region NodeGraphHost
//#region Helpers
const useClient = () => {
  const [client, setClient] = useState(false);
  useEffect(() => setClient(true), []);
  return client;
};

function PresencePeersOverlay({ peers }: { readonly peers: readonly PresencePeer[] }) {
  if (peers.length === 0) return null;
  return (
    <div className="pointer-events-none absolute right-2 top-2 z-panel flex max-w-[14rem] flex-col gap-1 rounded border border-border/60 bg-window/90 px-2 py-1 text-xs shadow-sm">
      {peers.map((peer) => (
        <div key={peer.clientId} className="flex items-center justify-between gap-2 text-muted-foreground">
          <span className="truncate font-medium text-foreground">{peer.name}</span>
          <span>{peer.selectionCount} selected</span>
        </div>
      ))}
    </div>
  );
}
//#endregion Helpers

//#region Component
export function NodeGraphHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.nodeGraph;
  const editable = scene?.editable ?? true;
  const parsedNodes = useMemo(() => parseJsonArray<MediaGraphNodeRecord>(scene?.nodesJson), [scene?.nodesJson]);
  const parsedEdges = useMemo(() => parseJsonArray<MediaGraphEdgeRecord>(scene?.edgesJson), [scene?.edgesJson]);
  const findItems = useMemo(() => parseJsonArray<GraphFindItem>(scene?.findItemsJson), [scene?.findItemsJson]);
  const contextMenuItems = useMemo(() => parseJsonArray<GraphContextMenuItem>(scene?.contextMenuJson), [scene?.contextMenuJson]);
  const presencePeers = useMemo(() => parseJsonArray<PresencePeer>(scene?.presencePeersJson), [scene?.presencePeersJson]);
  const isClient = useClient();

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const findContext = useUIFindSafe();
  const onFindItemRef = useRef<(itemId: string) => void>(() => {});
  onFindItemRef.current = (itemId: string) => {
    const mediaNode = parsedNodes.find((entry) => entry.instanceId === itemId);
    if (!mediaNode) return;
    dispatch(nodeGraphActions.select, { nodeIds: [mediaNode.id] });
    dispatch("selectInstance", { instanceId: mediaNode.instanceId! });
  };

  useEffect(() => {
    if (!findContext?.setFindItems || findItems.length === 0) return;
    findContext.setFindItems(findItems);
  }, [findContext?.setFindItems, findItems]);

  useEffect(() => {
    if (!findContext?.setOnFindItem || findItems.length === 0) return;
    findContext.setOnFindItem((itemId) => onFindItemRef.current(itemId));
    return () => findContext.setOnFindItem?.(undefined);
  }, [findContext?.setOnFindItem, findItems.length]);

  if (!scene) return <div className="semio-node-graph-empty">No graph scene</div>;

  const useFlowEngine = isFlowGraphScene(scene.capabilitiesJson) || Boolean(scene.fixtureJson);

  return (
    <div className="semio-node-graph-host relative h-full min-h-[24rem] w-full" data-surface-id={node.surfaceId} tabIndex={editable ? 0 : undefined} onKeyDown={(event) => handleGraphKeyboard(event, editable, parsedNodes, dispatch)}>
      {isClient ? (
        useFlowEngine ? (
          <FlowGraphCanvasHost scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} contextMenuItems={contextMenuItems} onAction={onAction} />
        ) : (
          <WasmGraphSurface scene={scene} surfaceId={node.surfaceId} controllerId={node.controllerId} editable={editable} contextMenuItems={contextMenuItems} onAction={onAction} />
        )
      ) : (
        <DiagramGraphFallback scene={scene} node={node} editable={editable} parsedNodes={parsedNodes} parsedEdges={parsedEdges} findItems={findItems} contextMenuItems={contextMenuItems} onAction={onAction} />
      )}
      <PresencePeersOverlay peers={presencePeers} />
    </div>
  );
}
//#endregion Component
//#endregion NodeGraphHost

//#region 🔖graph-canvas-overlays

//#region DagOverlayTypes
export type DagLabelOverlayRow = {
  readonly id: string;
  readonly kind?: "port" | "node" | string;
  readonly text: string;
  readonly layout: "horizontal" | "vertical";
  readonly align?: "left" | "center" | "right";
  readonly x: number;
  readonly y: number;
  readonly nodeW: number;
  readonly nodeH: number;
  readonly fontScreenPx?: number;
  readonly maxScreenH?: number;
  readonly ghost?: boolean;
};

export type DagPreselectSnapshot = {
  readonly ids: readonly string[];
  readonly removedIds: readonly string[];
};

export type DagLabelOverlayInteraction = {
  readonly hoveredId: string | null;
  readonly selectedIds: readonly string[];
  readonly preselect: DagPreselectSnapshot;
  readonly dimmedIds?: readonly string[];
};

export type DagMarqueeOverlay = {
  readonly kind: "rect" | "lasso";
  readonly x?: number;
  readonly y?: number;
  readonly width?: number;
  readonly height?: number;
  readonly points?: readonly { readonly x: number; readonly y: number }[];
  readonly coverage?: "full" | "partial";
};

export type DagCameraState = { readonly x: number; readonly y: number; readonly zoom: number };

export type DagParamEditorRow = {
  readonly nodeId: string;
  readonly portId: string;
  readonly label: string;
  readonly type?: string;
  readonly value?: unknown;
  readonly default?: unknown;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagStepperFieldRow = {
  readonly key: string;
  readonly label: string;
  readonly value: number;
  readonly step?: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagStepperOverlayRow = {
  readonly widgetId: string;
  readonly fields: readonly DagStepperFieldRow[];
};

export type DagSliderOverlayRow = {
  readonly widgetId: string;
  readonly value: number;
  readonly min: number;
  readonly max: number;
  readonly step: number;
  readonly x: number;
  readonly y: number;
  readonly w: number;
  readonly h: number;
};

export type DagSelectionBounds = {
  readonly x: number;
  readonly y: number;
  readonly width: number;
  readonly height: number;
};
//#endregion DagOverlayTypes

//#region DagOverlayGeometry
export function parseDagCameraState(json: string): DagCameraState {
  try {
    const parsed = JSON.parse(json) as Partial<DagCameraState>;
    return { x: Number(parsed.x ?? 0), y: Number(parsed.y ?? 0), zoom: Number(parsed.zoom ?? 1) };
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}

export function worldToScreen(camera: DagCameraState, width: number, height: number, wx: number, wy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (wx - camera.x) * zoom + cx, y: (wy - camera.y) * zoom + cy };
}

export function screenToWorld(camera: DagCameraState, width: number, height: number, sx: number, sy: number): { readonly x: number; readonly y: number } {
  const zoom = camera.zoom > 0 ? camera.zoom : 1;
  const cx = width * 0.5;
  const cy = height * 0.5;
  return { x: (sx - cx) / zoom + camera.x, y: (sy - cy) / zoom + camera.y };
}
//#endregion DagOverlayGeometry

//#region DagOverlayPaint
const DAG_LABEL_SCREEN_PX = 11;
const DAG_LABEL_FONT_FAMILY = "ui-sans-serif, system-ui, sans-serif";

export function parseDagNodeIdArray(json: string): string[] {
  try {
    const parsed = JSON.parse(json) as unknown;
    return Array.isArray(parsed) ? parsed.filter((value): value is string => typeof value === "string") : [];
  } catch {
    return [];
  }
}

export function parseDagPreselectJson(json: string): DagPreselectSnapshot {
  try {
    const parsed = JSON.parse(json) as { ids?: unknown; removedIds?: unknown };
    const ids = Array.isArray(parsed.ids) ? parsed.ids.filter((value): value is string => typeof value === "string") : [];
    const removedIds = Array.isArray(parsed.removedIds) ? parsed.removedIds.filter((value): value is string => typeof value === "string") : [];
    return { ids, removedIds };
  } catch {
    return { ids: [], removedIds: [] };
  }
}

export function dagElementInteractionChrome(selectionIds: Iterable<string>, preselection: DagPreselectSnapshot): { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> } {
  if (!preselection.ids.length && !preselection.removedIds.length) {
    return { selectedIds: new Set(selectionIds), highlightedIds: new Set() };
  }
  return { selectedIds: new Set(preselection.ids), highlightedIds: new Set(preselection.removedIds) };
}

export function parseDagLabelRows(stateJson: string): DagLabelOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as {
      readonly labels?: readonly Record<string, unknown>[];
      readonly rows?: readonly Record<string, unknown>[];
    };
    const raw = parsed.labels ?? parsed.rows ?? [];
    return raw
      .map((row) => {
        const text = typeof row.text === "string" ? row.text.trim() : "";
        if (!text) return null;
        const align = row.align === "left" || row.align === "right" || row.align === "center" ? row.align : undefined;
        return {
          id: String(row.id ?? ""),
          kind: typeof row.kind === "string" ? row.kind : undefined,
          text,
          layout: row.layout === "vertical" ? "vertical" : "horizontal",
          align,
          x: Number(row.x ?? 0),
          y: Number(row.y ?? 0),
          nodeW: Number(row.nodeW ?? row.width ?? 0),
          nodeH: Number(row.nodeH ?? row.height ?? 0),
          fontScreenPx: typeof row.fontScreenPx === "number" ? row.fontScreenPx : undefined,
          maxScreenH: typeof row.maxScreenH === "number" ? row.maxScreenH : undefined,
          ghost: row.ghost === true,
        } satisfies DagLabelOverlayRow;
      })
      .filter((row): row is DagLabelOverlayRow => row !== null);
  } catch {
    return [];
  }
}

function dagClampLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(4, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.2 <= maxH) {
    return px;
  }
  let low = 4;
  let high = px;
  let best = 4;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    const w = ctx.measureText(text).width;
    const h = mid * 1.2;
    if (w <= maxW && h <= maxH) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

function dagClampPortLabelFontPx(ctx: CanvasRenderingContext2D, text: string, targetPx: number, maxW: number, maxH: number): number {
  let px = Math.max(8, Math.round(targetPx));
  ctx.font = `${px}px ${DAG_LABEL_FONT_FAMILY}`;
  if (ctx.measureText(text).width <= maxW && px * 1.25 <= maxH) {
    return px;
  }
  let low = 8;
  let high = px;
  let best = 8;
  while (low <= high) {
    const mid = Math.floor((low + high) / 2);
    ctx.font = `${mid}px ${DAG_LABEL_FONT_FAMILY}`;
    if (ctx.measureText(text).width <= maxW) {
      best = mid;
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  return best;
}

export function parseDagParamEditors(stateJson: string): readonly DagParamEditorRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly editors?: DagParamEditorRow[] };
    return parsed.editors ?? [];
  } catch {
    return [];
  }
}

export function parseDagStepperOverlays(stateJson: string): readonly DagStepperOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly steppers?: DagStepperOverlayRow[] };
    return parsed.steppers ?? [];
  } catch {
    return [];
  }
}

export function parseDagSliderOverlays(stateJson: string): readonly DagSliderOverlayRow[] {
  try {
    const parsed = JSON.parse(stateJson) as { readonly sliders?: DagSliderOverlayRow[] };
    return parsed.sliders ?? [];
  } catch {
    return [];
  }
}

export function parseDagOverlayCamera(stateJson: string): DagCameraState {
  try {
    const parsed = JSON.parse(stateJson) as { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number };
    return parseDagCameraState(JSON.stringify(parsed.camera ?? {}));
  } catch {
    return { x: 0, y: 0, zoom: 1 };
  }
}
export function dagOverlayLabelFill(nodeId: string, ghost: boolean, hoveredId: string | null, chrome: { readonly selectedIds: Set<string>; readonly highlightedIds: Set<string> }, dimmedIds: readonly string[] = []): string {
  if (ghost) return "var(--color-secondary)";
  if (dimmedIds.includes(nodeId)) return "var(--color-border)";
  if (chrome.selectedIds.has(nodeId)) return "var(--color-foreground)";
  if (chrome.highlightedIds.has(nodeId)) return "var(--color-secondary)";
  if (hoveredId === nodeId) return "var(--color-foreground)";
  return "var(--color-muted-foreground)";
}

export function paintDagLabelOverlays(stateJson: string, canvas: HTMLCanvasElement, logicalW: number, logicalH: number, dpr: number, interaction: DagLabelOverlayInteraction): void {
  let state: { readonly camera?: DagCameraState; readonly width?: number; readonly height?: number; readonly labels?: readonly DagLabelOverlayRow[] };
  try {
    state = JSON.parse(stateJson) as typeof state;
  } catch {
    return;
  }
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  const pixelW = Math.max(1, Math.round(logicalW * dpr));
  const pixelH = Math.max(1, Math.round(logicalH * dpr));
  if (canvas.width !== pixelW || canvas.height !== pixelH) {
    canvas.width = pixelW;
    canvas.height = pixelH;
  }
  canvas.style.width = `${logicalW}px`;
  canvas.style.height = `${logicalH}px`;
  ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx.clearRect(0, 0, logicalW, logicalH);
  const zoom = Math.max(0.05, Number(state.camera?.zoom) || 1);
  const camera = {
    x: Number(state.camera?.x) || 0,
    y: Number(state.camera?.y) || 0,
    zoom,
  };
  const viewportW = Number(state.width) || logicalW;
  const viewportH = Number(state.height) || logicalH;
  const chrome = dagElementInteractionChrome(interaction.selectedIds, interaction.preselect);
  const dimmedIds = interaction.dimmedIds ?? [];
  const rows = state.labels ?? parseDagLabelRows(stateJson);
  const inset = 0.88;
  for (const row of rows) {
    const anchor = worldToScreen(camera, viewportW, viewportH, row.x, row.y);
    const isPort = row.kind === "port" || row.align === "left" || row.align === "right";
    const maxW = Math.max(4, Number(row.nodeW) * zoom * inset);
    const maxH = Math.max(4, isPort && Number.isFinite(Number(row.maxScreenH)) && Number(row.maxScreenH) > 0 ? Number(row.maxScreenH) : Number(row.nodeH) * zoom * inset);
    const fontScreenPx = Number(row.fontScreenPx);
    const targetPx = Number.isFinite(fontScreenPx) && fontScreenPx > 0 ? fontScreenPx : DAG_LABEL_SCREEN_PX;
    const fontPx = isPort ? dagClampPortLabelFontPx(ctx, row.text, targetPx, maxW, maxH) : dagClampLabelFontPx(ctx, row.text, targetPx, maxW, maxH);
    ctx.font = `${fontPx}px ${DAG_LABEL_FONT_FAMILY}`;
    ctx.fillStyle = dagOverlayLabelFill(row.id, row.ghost === true, interaction.hoveredId, chrome, dimmedIds);
    ctx.globalAlpha = row.ghost ? 0.85 : dimmedIds.includes(row.id) ? 0.5 : 1;
    if (row.layout === "vertical") {
      ctx.save();
      ctx.translate(anchor.x, anchor.y);
      ctx.rotate(-Math.PI / 2);
      ctx.textAlign = "center";
      ctx.textBaseline = "middle";
      ctx.fillText(row.text, 0, 0);
      ctx.restore();
    } else {
      const align = row.align === "left" || row.align === "right" ? row.align : "center";
      ctx.textAlign = align;
      ctx.textBaseline = "middle";
      ctx.fillText(row.text, anchor.x, anchor.y);
    }
    ctx.globalAlpha = 1;
  }
}

export function parseDagSelectionUnionBoundsScreen(json: string): DagSelectionBounds | null {
  try {
    const parsed = JSON.parse(json) as Partial<DagSelectionBounds>;
    if (parsed.x == null || parsed.y == null || parsed.width == null || parsed.height == null) return null;
    return { x: parsed.x, y: parsed.y, width: parsed.width, height: parsed.height };
  } catch {
    return null;
  }
}

export function computeDagMarqueeOverlay(pointsJson: string, crossing: boolean, method: string): DagMarqueeOverlay | null {
  let points: { readonly x: number; readonly y: number }[] = [];
  try {
    points = JSON.parse(pointsJson) as { readonly x: number; readonly y: number }[];
  } catch {
    return null;
  }
  if (points.length < 2) return null;
  const coverage = crossing ? "partial" : "full";
  if (method === "lasso") return { kind: "lasso", points, coverage };
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const x = Math.min(...xs);
  const y = Math.min(...ys);
  return { kind: "rect", x, y, width: Math.max(...xs) - x, height: Math.max(...ys) - y, coverage };
}

export function sceneToSyncJson(scene: NodeGraphScene): string {
  return JSON.stringify(scene);
}

//#region DagDomOverlays
export function GraphParamOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onParamChange,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onParamChange: (nodeId: string, portId: string, value: unknown) => void;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const editors = parseDagParamEditors(stateJson);
  if (editors.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {editors.map((editor) => {
        const screen = worldToScreen(camera, logicalW, logicalH, editor.x, editor.y);
        const w = editor.w * camera.zoom;
        const h = editor.h * camera.zoom;
        return (
          <input
            key={`${editor.nodeId}:${editor.portId}`}
            className="pointer-events-auto absolute rounded border border-border bg-panel px-1 font-mono text-[10px] text-foreground"
            style={{ left: screen.x - w / 2, top: screen.y - h / 2, width: w, height: h }}
            defaultValue={String(editor.value ?? editor.default ?? "")}
            readOnly={!editable}
            onPointerDown={(event) => event.stopPropagation()}
            onChange={(event) => onParamChange(editor.nodeId, editor.portId, event.target.value)}
          />
        );
      })}
    </div>
  );
}

export function GraphStepperOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onStepperChange,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onStepperChange: (widgetId: string, fieldKey: string, value: number) => void;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const steppers = parseDagStepperOverlays(stateJson);
  if (steppers.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {steppers.flatMap((stepper) =>
        stepper.fields.map((field) => {
          const screen = worldToScreen(camera, logicalW, logicalH, field.x, field.y);
          const w = field.w * camera.zoom;
          const h = field.h * camera.zoom;
          return (
            <input
              key={`${stepper.widgetId}:${field.key}`}
              type="number"
              className="pointer-events-auto absolute rounded border border-border bg-panel px-1 font-mono text-[10px] text-foreground"
              style={{ left: screen.x, top: screen.y - h / 2, width: w, height: h }}
              defaultValue={field.value}
              step={field.step ?? 1}
              readOnly={!editable}
              onPointerDown={(event) => event.stopPropagation()}
              onChange={(event) => onStepperChange(stepper.widgetId, field.key, Number(event.target.value))}
            />
          );
        }),
      )}
    </div>
  );
}

export function GraphSliderOverlays({
  stateJson,
  logicalW,
  logicalH,
  editable,
  onSliderChange,
  onSliderPointerDown,
  onSliderPointerUp,
}: {
  readonly stateJson: string;
  readonly logicalW: number;
  readonly logicalH: number;
  readonly editable: boolean;
  readonly onSliderChange: (widgetId: string, value: number) => void;
  readonly onSliderPointerDown?: () => void;
  readonly onSliderPointerUp?: () => void;
}) {
  const camera = parseDagOverlayCamera(stateJson);
  const sliders = parseDagSliderOverlays(stateJson);
  if (sliders.length === 0) return null;
  return (
    <div className="pointer-events-none absolute inset-0 z-45">
      {sliders.map((slider) => {
        const screen = worldToScreen(camera, logicalW, logicalH, slider.x, slider.y);
        const w = slider.w * camera.zoom;
        const h = Math.max(slider.h * camera.zoom, 16);
        return (
          <div key={slider.widgetId} className="pointer-events-auto absolute flex items-center px-1" style={{ left: screen.x - w / 2, top: screen.y - h / 2, width: w, height: h }} onPointerDown={(event) => event.stopPropagation()}>
            <Slider
              className="w-full min-w-0"
              max={slider.max}
              min={slider.min}
              step={slider.step}
              value={[slider.value]}
              disabled={!editable}
              onValueChange={(values) => onSliderChange(slider.widgetId, values[0] ?? slider.value)}
              onPointerDown={onSliderPointerDown}
              onPointerUp={onSliderPointerUp}
              onPointerCancel={onSliderPointerUp}
            />
          </div>
        );
      })}
    </div>
  );
}

const ALIGN_MODES = [
  { id: "left", label: "⬅" },
  { id: "center-h", label: "↔" },
  { id: "right", label: "➡" },
  { id: "top", label: "⬆" },
  { id: "center-v", label: "↕" },
  { id: "bottom", label: "⬇" },
] as const;

export function alignModeToDag(mode: string): string {
  const map: Record<string, string> = {
    left: "alignLeft",
    right: "alignRight",
    top: "alignTop",
    bottom: "alignBottom",
    "center-h": "alignHorizontal",
    "center-v": "alignVertical",
  };
  return map[mode] ?? mode;
}

export function SelectionAlignChrome({ bounds, onAlign }: { readonly bounds: DagSelectionBounds; readonly onAlign: (mode: string) => void }) {
  return (
    <div className="pointer-events-auto absolute z-50 flex gap-0.5 rounded border border-border bg-panel p-0.5 shadow-sm" style={{ left: bounds.x, top: Math.max(0, bounds.y - 28) }}>
      {ALIGN_MODES.map((mode) => (
        <button key={mode.id} type="button" className="size-5 rounded text-xs hover:bg-active-base" aria-label={mode.id} onPointerDown={(event) => event.stopPropagation()} onClick={() => onAlign(mode.id)}>
          {mode.label}
        </button>
      ))}
    </div>
  );
}
//#endregion DagDomOverlays
//#endregion DagOverlayPaint
//#endregion 🔖graph-canvas-overlays

//#region 🔖flow-graph-canvas-host

//#region Sync
// @emoji 🎥 `applyCamera` must stay false for every resync after the first: FlowWasmSession never
// reports its live camera back into the document (`cameraJson` is unimplemented, see the wheel
// handler below), so `scene.viewportJson` is frozen at its initial value for the whole session —
// applying it on every edit-triggered resync would snap the user's camera back on every commit.
function syncFlowSessionFromScene(session: FlowWasmSession, scene: NodeGraphScene, applyCamera: boolean): void {
  if (scene.operatorsJson) session.setNeuronKindInfosJson(scene.operatorsJson);
  if (scene.fixtureJson) session.loadFixtureJson(scene.fixtureJson);
  if (scene.selectionJson) session.setSelection(scene.selectionJson);
  if (scene.previewOffJson) session.setPreviewOff(scene.previewOffJson);
  if (scene.catalogueJson) session.setCatalogueJson(scene.catalogueJson);
  if (scene.computingJson) session.setComputingProgress(scene.computingJson);
  if (scene.lodJson) {
    try {
      const lod = JSON.parse(scene.lodJson) as { readonly automatic?: boolean; readonly forcedLabel?: string };
      session.setAutomaticLod(lod.automatic !== false);
      if (lod.forcedLabel) session.setForcedDrawLodLabel(lod.forcedLabel);
    } catch {
      /* ignore */
    }
  }
  if (!applyCamera) return;
  try {
    const viewport = JSON.parse(scene.viewportJson) as { readonly x?: number; readonly y?: number; readonly zoom?: number };
    session.setCamera(viewport.x ?? 0, viewport.y ?? 0, viewport.zoom ?? 1);
  } catch {
    /* ignore */
  }
}
//#endregion Sync

//#region Spotlight
function SpotlightOverlay({ previewText, onCommit, onDismiss }: { readonly previewText: string; readonly onCommit: () => void; readonly onDismiss: () => void }) {
  if (!previewText.trim()) return null;
  return (
    <div className="pointer-events-auto absolute inset-x-4 bottom-4 z-60 rounded border border-border bg-panel p-3 shadow-lg">
      <div className="mb-2 text-xs font-medium text-muted-foreground">Preview</div>
      <pre className="max-h-40 overflow-auto whitespace-pre-wrap font-mono text-xs text-foreground">{previewText}</pre>
      <div className="mt-2 flex justify-end gap-2">
        <button type="button" className="rounded px-2 py-1 text-xs hover:bg-active-base" onClick={onDismiss}>
          Dismiss
        </button>
        <button type="button" className="rounded bg-accent px-2 py-1 text-xs text-accent-foreground" onClick={onCommit}>
          Commit
        </button>
      </div>
    </div>
  );
}
//#endregion Spotlight

//#region FlowGraphCanvasHost
export function FlowGraphCanvasHost({
  scene,
  surfaceId,
  controllerId,
  editable,
  contextMenuItems,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly surfaceId: string;
  readonly controllerId: string;
  readonly editable: boolean;
  readonly contextMenuItems: readonly GraphContextMenuItem[];
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const sessionRef = useRef<FlowWasmSession | null>(null);
  const gpuCanvasRef = useRef<HTMLCanvasElement | null>(null);
  const labelCanvasRef = useRef<HTMLCanvasElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<{ readonly x: number; readonly y: number; readonly widgetId?: string } | null>(null);
  const [selectionBounds, setSelectionBounds] = useState<ReturnType<typeof parseDagSelectionUnionBoundsScreen>>(null);
  const [marquee, setMarquee] = useState<ReturnType<typeof computeDagMarqueeOverlay>>(null);
  const [labelStateJson, setLabelStateJson] = useState("{}");
  const [paramStateJson, setParamStateJson] = useState("{}");
  const [stepperStateJson, setStepperStateJson] = useState("{}");
  const [sliderStateJson, setSliderStateJson] = useState("{}");
  const [previewText, setPreviewText] = useState("");
  const [containerSize, setContainerSize] = useState({ w: 800, h: 600 });
  const [sessionReady, setSessionReady] = useState(false);
  const sceneSignature = useMemo(() => JSON.stringify(scene), [scene]);
  // Always holds the latest `scene` without forcing effects to depend on (and re-run per) it.
  const sceneRef = useRef(scene);
  sceneRef.current = scene;

  useEffect(() => {
    console.log("[DEBUG] FlowGraphCanvasHost mounted", { surfaceId, controllerId });
    return () => console.log("[DEBUG] FlowGraphCanvasHost UNMOUNTED", { surfaceId, controllerId });
  }, [surfaceId, controllerId]);

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId, action, args: { surfaceId, ...args } });
    },
    [controllerId, onAction, surfaceId],
  );

  const commitFixture = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const fixtureJson = session.fixtureJson();
      console.log("[DEBUG] commitFixture: dispatching setFixture, isGestureActive=", isGestureActiveRef.current, "len=", fixtureJson.length);
      dispatch(nodeGraphActions.edit, { ops: [{ op: "setFixture", fixtureJson }] });
      session.evaluateSync();
    } catch {
      /* session not ready */
    }
  }, [dispatch]);

  // A continuous gesture (e.g. dragging a slider) fires many onValueChange ticks per second, each
  // committing the whole document through an async plugin round-trip; concurrent in-flight commits
  // can resolve out of order, and the scene-resync effect below would apply whichever one lands
  // last — visibly reverting the drag mid-gesture. isGestureActiveRef suppresses that resync while
  // a gesture is active, and commitFixtureThrottled caps how many concurrent commits are in flight.
  const isGestureActiveRef = useRef(false);
  const lastCommitAtRef = useRef(0);
  const pendingCommitTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const GESTURE_COMMIT_THROTTLE_MS = 80;

  const commitFixtureThrottled = useCallback(() => {
    if (pendingCommitTimeoutRef.current != null) {
      clearTimeout(pendingCommitTimeoutRef.current);
      pendingCommitTimeoutRef.current = null;
    }
    const elapsed = Date.now() - lastCommitAtRef.current;
    if (elapsed >= GESTURE_COMMIT_THROTTLE_MS) {
      lastCommitAtRef.current = Date.now();
      commitFixture();
    } else {
      pendingCommitTimeoutRef.current = setTimeout(() => {
        pendingCommitTimeoutRef.current = null;
        lastCommitAtRef.current = Date.now();
        commitFixture();
      }, GESTURE_COMMIT_THROTTLE_MS - elapsed);
    }
  }, [commitFixture]);

  const handleGesturePointerDown = useCallback(() => {
    console.log("[DEBUG] gesture pointerDown: isGestureActiveRef -> true");
    isGestureActiveRef.current = true;
  }, []);

  const handleGesturePointerUp = useCallback(() => {
    console.log("[DEBUG] gesture pointerUp: isGestureActiveRef -> false, firing final commitFixture");
    isGestureActiveRef.current = false;
    if (pendingCommitTimeoutRef.current != null) {
      clearTimeout(pendingCommitTimeoutRef.current);
      pendingCommitTimeoutRef.current = null;
    }
    lastCommitAtRef.current = Date.now();
    commitFixture();
  }, [commitFixture]);

  useEffect(() => {
    return () => {
      if (pendingCommitTimeoutRef.current != null) clearTimeout(pendingCommitTimeoutRef.current);
    };
  }, []);

  const paintOverlays = useCallback(() => {
    const session = sessionRef.current;
    const labelCanvas = labelCanvasRef.current;
    const container = containerRef.current;
    if (!session || !labelCanvas || !container) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    setContainerSize({ w: rect.width, h: rect.height });
    try {
      const labelJson = session.labelOverlayPaintStateJson();
      setLabelStateJson(labelJson);
      const selectedIds = parseDagNodeIdArray(session.selectedWidgetIds());
      const preselect = parseDagPreselectJson(session.preselectWidgetIdsJson());
      const dimmedIds = parseDagNodeIdArray(session.previewOffWidgetIds());
      paintDagLabelOverlays(labelJson, labelCanvas, rect.width, rect.height, dpr, {
        hoveredId: session.hoveredWidgetId() ?? null,
        selectedIds,
        preselect,
        dimmedIds,
      });
      setParamStateJson(session.paramOverlayPaintStateJson());
      setStepperStateJson(session.stepperOverlayStateJson());
      setSliderStateJson(session.sliderOverlayStateJson());
    } catch {
      /* gpu not ready */
    }
    setSelectionBounds(parseDagSelectionUnionBoundsScreen(session.selectionUnionBoundsScreenJson()));
    setMarquee(computeDagMarqueeOverlay(session.selectionPreviewPointsJson(), session.selectionPreviewCrossing(), "rectangle"));
    try {
      setPreviewText(session.previewText());
    } catch {
      setPreviewText("");
    }
  }, []);

  const emitInteractionState = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    try {
      const nodeIds = JSON.parse(session.selectedWidgetIds()) as string[];
      dispatch(nodeGraphActions.select, { nodeIds });
      const hovered = session.hoveredWidgetId();
      const channelJson = session.hoveredChannelJson();
      dispatch(nodeGraphActions.hover, { hoverJson: hovered ? channelJson : null });
    } catch {
      /* session not ready */
    }
    paintOverlays();
  }, [dispatch, paintOverlays]);

  useEffect(() => {
    let cancelled = false;
    void createFlowSession().then((session) => {
      if (cancelled) return;
      sessionRef.current = session;
      setSessionReady(true);
    });
    return () => {
      cancelled = true;
    };
  }, []);

  // Attaches the GPU canvas exactly once per session (NOT per document edit — `scene` must stay out
  // of this effect's deps). It used to depend on `scene`, so it re-ran `attachCanvas` on every single
  // commit (including every slider tick): the wasm session rejects a second attach ("canvas surface
  // already attached"), and because the cleanup below was returned from inside the `.then()` instead
  // of from the effect itself, React never saw it — every re-run leaked its ResizeObserver/rAF loop
  // and could disrupt the live GPU surface, which is what read as the whole view "resetting".
  useEffect(() => {
    const session = sessionRef.current;
    const canvas = gpuCanvasRef.current;
    const container = containerRef.current;
    if (!session || !canvas || !container || !sessionReady) return;
    const rect = container.getBoundingClientRect();
    const dpr = globalThis.devicePixelRatio || 1;
    let raf = 0;
    let cancelled = false;
    let cleanupAttached: (() => void) | undefined;
    console.log("[DEBUG] attachCanvas effect: attaching (should log ONCE per session)");
    session
      .attachCanvas(canvas, Math.round(rect.width), Math.round(rect.height), dpr)
      .then(() => {
        if (cancelled) return;
        console.log("[DEBUG] attachCanvas effect: attached OK, applying initial camera");
        syncFlowSessionFromScene(session, sceneRef.current, true);
        const resize = () => {
          const next = container.getBoundingClientRect();
          const nextDpr = globalThis.devicePixelRatio || 1;
          session.setSize(Math.round(next.width), Math.round(next.height), nextDpr);
          session.renderFrame();
          paintOverlays();
        };
        resize();
        const ro = new ResizeObserver(resize);
        ro.observe(container);
        const tick = () => {
          session.renderFrame();
          raf = requestAnimationFrame(tick);
        };
        raf = requestAnimationFrame(tick);
        cleanupAttached = () => {
          ro.disconnect();
          if (raf) cancelAnimationFrame(raf);
        };
      })
      .catch((err) => {
        /* already attached (e.g. a stale re-run) or transient failure; nothing to clean up */
        console.log("[DEBUG] attachCanvas effect: attach FAILED/REJECTED", err);
      });
    return () => {
      console.log("[DEBUG] attachCanvas effect: cleanup running (effect re-run or unmount)");
      cancelled = true;
      cleanupAttached?.();
    };
  }, [sessionReady, paintOverlays]);

  useEffect(() => {
    const session = sessionRef.current;
    if (!session || !sessionReady) return;
    // Skip while a gesture (e.g. slider drag) is active: an in-flight commit's response landing
    // mid-gesture would otherwise reload the fixture and visibly revert the live local edit.
    if (isGestureActiveRef.current) {
      console.log("[DEBUG] resync effect: SKIPPED (gesture active), sceneSignature len=", sceneSignature.length);
      return;
    }
    console.log("[DEBUG] resync effect: APPLYING syncFlowSessionFromScene, sceneSignature len=", sceneSignature.length, "fixtureJson len=", scene.fixtureJson?.length);
    syncFlowSessionFromScene(session, scene, false);
    session.renderFrame();
    paintOverlays();
  }, [sceneSignature, paintOverlays, scene, sessionReady]);

  useCanvasAppearanceSync(() => {
    syncSessionCanvasTheme(sessionRef.current);
  });

  const pickInteraction = useCanvasPickInteraction({
    resolveTargetsAtClient: (client) => {
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return [];
      const rect = container.getBoundingClientRect();
      const sx = client.x - rect.left;
      const sy = client.y - rect.top;
      try {
        return JSON.parse(session.pickTargetsAtScreenJson(sx, sy)) as CanvasPickTarget[];
      } catch {
        return [];
      }
    },
    onHoverFocus: (focus) => {
      const session = sessionRef.current;
      if (!session) return;
      const target = focus.target;
      if (!target) {
        session.setHover?.(null);
      } else if (target.portId) {
        session.setHoverChannel?.(target.id, target.portId);
      } else {
        session.setHover?.(target.id);
      }
      session.renderFrame();
      paintOverlays();
    },
    onSelectTarget: () => {
      emitInteractionState();
    },
  });

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const session = sessionRef.current;
      if (!session || !editable) return;
      const mod = event.metaKey || event.ctrlKey;
      if (mod && event.key === "z" && !event.shiftKey) {
        event.preventDefault();
        if (session.undo()) {
          commitFixture();
          emitInteractionState();
        }
        return;
      }
      if (mod && (event.key === "Z" || (event.key === "z" && event.shiftKey))) {
        event.preventDefault();
        if (session.redo()) {
          commitFixture();
          emitInteractionState();
        }
        return;
      }
      if (mod && event.key === "a") {
        event.preventDefault();
        session.selectAll();
        emitInteractionState();
        return;
      }
      if (event.key === "Delete" || event.key === "Backspace") {
        if ((event.target as HTMLElement).tagName === "INPUT" || (event.target as HTMLElement).tagName === "TEXTAREA") return;
        event.preventDefault();
        session.deleteSelection();
        commitFixture();
        emitInteractionState();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [commitFixture, editable, emitInteractionState]);

  const clearGhostPreview = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    session.clearGhostWidget();
    session.renderFrame();
    paintOverlays();
  }, [paintOverlays]);

  const onDragOverCanvas = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const encoded = getActiveCatalogueDragPayload();
      if (!encoded) return;
      const catalogueApp = parseCatalogueAppDragPayload(encoded);
      if (!catalogueApp) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = screenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      session.setGhostWidget(catalogueGhostDescriptorJson(catalogueApp), world.x, world.y);
      session.renderFrame();
      paintOverlays();
    },
    [editable, labelStateJson, paintOverlays],
  );

  const onDrop = useCallback(
    (event: DragEvent<HTMLDivElement>) => {
      if (!editable) return;
      clearGhostPreview();
      const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME) || event.dataTransfer.getData("text/plain") || getActiveCatalogueDragPayload() || "";
      if (!raw) return;
      event.preventDefault();
      const session = sessionRef.current;
      const container = containerRef.current;
      if (!session || !container) return;
      const rect = container.getBoundingClientRect();
      const sx = event.clientX - rect.left;
      const sy = event.clientY - rect.top;
      let world = { x: sx, y: sy };
      try {
        const parsed = JSON.parse(session.worldFromScreen(sx, sy)) as { readonly x?: number; readonly y?: number };
        world = { x: parsed.x ?? sx, y: parsed.y ?? sy };
      } catch {
        const camera = parseDagOverlayCamera(labelStateJson);
        world = screenToWorld(camera, rect.width, rect.height, sx, sy);
      }
      const catalogueApp = parseCatalogueAppDragPayload(raw);
      if (catalogueApp) {
        dispatch("spawnApp", { programId: catalogueApp.programId, appId: catalogueApp.appId, position: { x: world.x, y: world.y } });
        return;
      }
      try {
        const descriptor = raw.startsWith("{") ? raw : JSON.stringify({ kind: raw });
        session.addWidget(descriptor, world.x, world.y);
        commitFixture();
        emitInteractionState();
      } catch {
        /* invalid descriptor */
      }
    },
    [clearGhostPreview, commitFixture, dispatch, editable, emitInteractionState, labelStateJson],
  );

  const openHoveredInstance = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    const instanceId = resolveFixtureWidgetInstanceId(scene.fixtureJson, session.hoveredWidgetId());
    if (instanceId) dispatch("openInstance", { instanceId });
  }, [dispatch, scene.fixtureJson]);

  useEffect(() => clearGhostPreview, [clearGhostPreview]);

  return (
    <div
      ref={containerRef}
      className="relative h-full w-full"
      onDragOver={onDragOverCanvas}
      onDragLeave={() => {
        if (!editable) return;
        clearGhostPreview();
      }}
      onDrop={onDrop}
      onContextMenu={(event) => {
        if (!editable || contextMenuItems.length === 0) return;
        event.preventDefault();
        setContextMenu({ x: event.clientX, y: event.clientY, widgetId: sessionRef.current?.hoveredWidgetId() });
      }}
    >
      <canvas ref={gpuCanvasRef} className="absolute inset-0 block h-full w-full" />
      <canvas ref={labelCanvasRef} className="pointer-events-none absolute inset-0 z-40" />
      <GraphParamOverlays
        stateJson={paramStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        onParamChange={(nodeId, portId, value) => {
          const session = sessionRef.current;
          if (!session) return;
          session.setNeuronParams(nodeId, JSON.stringify({ [portId]: value }));
          commitFixture();
          paintOverlays();
        }}
      />
      <GraphStepperOverlays
        stateJson={stepperStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        onStepperChange={(widgetId, fieldKey, value) => {
          sessionRef.current?.setStepperFieldValue(widgetId, fieldKey, value);
          commitFixture();
          paintOverlays();
        }}
      />
      <GraphSliderOverlays
        stateJson={sliderStateJson}
        logicalW={containerSize.w}
        logicalH={containerSize.h}
        editable={editable}
        onSliderChange={(widgetId, value) => {
          console.log("[DEBUG] onSliderChange (TS handler fired)", widgetId, value, "isGestureActive=", isGestureActiveRef.current);
          sessionRef.current?.setSliderValue(widgetId, value);
          commitFixtureThrottled();
          paintOverlays();
        }}
        onSliderPointerDown={handleGesturePointerDown}
        onSliderPointerUp={handleGesturePointerUp}
      />
      {selectionBounds ? (
        <>
          <div className="pointer-events-none absolute z-20 border-2 border-accent" style={{ left: selectionBounds.x, top: selectionBounds.y, width: selectionBounds.width, height: selectionBounds.height }} />
          {editable ? (
            <SelectionAlignChrome
              bounds={selectionBounds}
              onAlign={(mode) => {
                sessionRef.current?.alignSelection(mode);
                commitFixture();
                paintOverlays();
              }}
            />
          ) : null}
        </>
      ) : null}
      {marquee ? (
        marquee.kind === "lasso" ? (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="polygon" points={marquee.points ?? []} />
        ) : (
          <SelectionMarquee coverage={marquee.coverage ?? "full"} shape="rect" rect={{ x: marquee.x ?? 0, y: marquee.y ?? 0, width: marquee.width ?? 0, height: marquee.height ?? 0 }} />
        )
      ) : null}
      <div
        className="absolute inset-0 z-30"
        onPointerDown={(event) => {
          if (!editable) return;
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerDown(client);
          session.pointerDownScreen(event.clientX - rect.left, event.clientY - rect.top, event.button, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey, event.button === 1 || event.buttons === 4);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerMove={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerMove(client);
          session.pointerMoveScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          paintOverlays();
        }}
        onPointerUp={(event) => {
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const client = { x: event.clientX, y: event.clientY };
          pickInteraction.onCanvasPointerUp(client, { shift: event.shiftKey, ctrlOrMeta: event.metaKey || event.ctrlKey, alt: event.altKey });
          session.pointerUpScreen(event.clientX - rect.left, event.clientY - rect.top, event.shiftKey, event.metaKey || event.ctrlKey, event.altKey);
          session.renderFrame();
          commitFixture();
          emitInteractionState();
        }}
        onPointerLeave={() => pickInteraction.onCanvasPointerLeave()}
        onDoubleClick={openHoveredInstance}
        onWheel={(event) => {
          event.preventDefault();
          const session = sessionRef.current;
          if (!session) return;
          const rect = event.currentTarget.getBoundingClientRect();
          const delta = event.deltaMode === 1 ? event.deltaY * 16 : event.deltaMode === 2 ? event.deltaY * 400 : event.deltaY;
          session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, 0, delta, true);
          session.renderFrame();
          const cameraJson = session.cameraJson?.();
          if (cameraJson) dispatch(nodeGraphActions.viewport, nodeGraphViewportActionArgs(cameraJson));
          paintOverlays();
        }}
      />
      <CanvasPickMenu request={pickInteraction.pickMenu} hoveredKey={pickInteraction.menuHoveredKey} onHoverKey={pickInteraction.onMenuHoverKey} onPick={pickInteraction.onMenuPick} onDismiss={pickInteraction.dismissPickMenu} />
      <SpotlightOverlay previewText={previewText} onCommit={() => dispatch(nodeGraphActions.spotlightCommit, {})} onDismiss={() => setPreviewText("")} />
      <ContextMenuController
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenuItems.map((item) => ({
          id: item.id,
          label: item.label,
          onSelect: () => dispatch(item.action, item.action === "openInstance" ? { ...item.args, instanceId: resolveFixtureWidgetInstanceId(scene.fixtureJson, contextMenu?.widgetId) } : item.args),
        }))}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion FlowGraphCanvasHost
//#endregion 🔖flow-graph-canvas-host
