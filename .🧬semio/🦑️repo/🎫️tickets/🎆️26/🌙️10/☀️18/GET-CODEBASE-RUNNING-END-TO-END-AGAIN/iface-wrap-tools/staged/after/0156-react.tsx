/** 🔷️ The SSR-safe diagram fallback: workflow nodes and edges mapped onto the diagram element, the workflow diagram node and the fallback surface.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | NodeGraph}, which re-exports every public name below so import specifiers stay unchanged.
 */

import {
  useCallback,
  useEffect,
  useMemo,
  useRef,
  useState,
  type DragEvent
} from "../../⚛️react-port/🟦️.ts";
import {
  CATALOGUE_DRAG_MIME,
  cn,
  ContextMenuController,
  Diagram,
  Handle,
  Position,
  surfaceClass,
  useLabel,
  type Edge,
  type Node,
  type NodeProps,
  type NodeTypes
} from "@semio-tech/ui-react";
import {
  nodeGraphActions,
  type ActionDescriptor,
  type ContextMenuItemSpec,
  type NodeGraphEdgeRecord,
  type NodeGraphFindItem,
  type NodeGraphNodeRecord,
  type NodeGraphScene,
  type PluginContextMenuRequest,
  type UiComponentSceneNode
} from "@semio-tech/framework";
import { openSurfaceContextMenu, useMapContextMenuSpecs, useShellContextMenuFallback, type SurfaceContextMenuResult } from "@semio-tech/framework-os/modules/renderer/engine/elements/Interpreter/surface-menu";
import { nodeDragMoves, nodeDragRow } from "@semio-tech/framework/tool-machine";
import {
  DEFAULT_NODE_GRAPH_VIEWPORT,
  nodeGraphConnectionIsValid,
  portHandleId,
  portLabel,
  publishNodeGraphSelection,
  type WorkflowNodeData
} from "../🧰️graph-core/🟦️.ts";
import { createNodeGraphInteractionLedger, nodeGraphSelectionMarkKey } from "../🫱️interaction-publication/🟦️.ts";

//#region 🔖️NodeGraphHost
//#region Parsing
/** 🎯️ Diagram nodes for the React-Flow fallback board, carrying the PLUGIN's selection as their own
 * `selected` flag — see `DiagramGraphFallback`'s ledger docstring for why an unreflected selection is
 * an erased one. Exported for `🧪️tests/🫱️interaction-publication`. */
export function workflowNodesToDiagramNodes(records: readonly NodeGraphNodeRecord[], selectedNodeIds: readonly string[] = []): Node<WorkflowNodeData>[] {
  const selected = new Set(selectedNodeIds);
  return records.map((record) => ({
    id: record.id,
    type: "workflow",
    selected: selected.has(record.id),
    position: { x: record.x, y: record.y },
    data: {
      label: record.label?.trim() || record.instanceId || record.id,
      inputs: record.inputs,
      outputs: record.outputs,
      width: record.width,
      height: record.height,
    },
  }));
}

function workflowEdgesToDiagramEdges(records: readonly NodeGraphEdgeRecord[]): Edge[] {
  return records.map((record) => ({
    id: record.id,
    source: record.sourceNodeId,
    target: record.targetNodeId,
    sourceHandle: record.sourcePortId,
    targetHandle: record.targetPortId,
  }));
}
//#endregion Parsing

//#region DiagramNode
function WorkflowDiagramNode({ data }: NodeProps<Node<WorkflowNodeData>>) {
  const inputCount = Math.max(data.inputs.length, 1);
  const outputCount = Math.max(data.outputs.length, 1);
  const rowCount = Math.max(inputCount, outputCount);
  const rowHeight = 18;
  const bodyHeight = Math.max(data.height, 56 + rowCount * rowHeight);
  return (
    <div className="rounded border border-border bg-background text-foreground shadow-sm" style={{ width: data.width, minHeight: bodyHeight }}>
      <div className="border-b border-border px-2 py-1 text-xs font-medium">{data.label}</div>
      <div className="relative px-2 py-1 text-[0.625rem] leading-[1.125rem]">
        {Array.from({ length: rowCount }, (_, rowIndex) => {
          const input = data.inputs[rowIndex];
          const output = data.outputs[rowIndex];
          const top = 8 + rowIndex * rowHeight;
          return (
            <div key={`${input?.id ?? "in"}:${output?.id ?? "out"}:${rowIndex}`} className="relative h-[1.125rem]">
              {input ? (
                <>
                  <Handle id={portHandleId(input)} type="target" position={Position.Left} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
                  <span className="pl-3 text-muted-foreground">{portLabel(input)}</span>
                </>
              ) : null}
              {output ? (
                <>
                  <Handle id={portHandleId(output)} type="source" position={Position.Right} className="!size-2 !border-panel !bg-foreground" style={{ top }} />
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

const workflowNodeTypes: NodeTypes = { workflow: WorkflowDiagramNode };
//#endregion DiagramNode

//#region DiagramFallback
export function DiagramGraphFallback({
  scene,
  node,
  editable,
  parsedNodes,
  parsedEdges,
  findItems,
  requestContextMenu,
  onAction,
}: {
  readonly scene: NodeGraphScene;
  readonly node: UiComponentSceneNode;
  readonly editable: boolean;
  readonly parsedNodes: readonly NodeGraphNodeRecord[];
  readonly parsedEdges: readonly NodeGraphEdgeRecord[];
  readonly findItems: readonly NodeGraphFindItem[];
  readonly requestContextMenu?: (request: PluginContextMenuRequest) => Promise<readonly ContextMenuItemSpec[]>;
  readonly onAction: (action: ActionDescriptor) => void;
}) {
  const viewport = scene.viewport ?? DEFAULT_NODE_GRAPH_VIEWPORT;
  /** 🎯️ The selection the PLUGIN holds, as this surface's own baseline. React Flow fires
   * `onSelectionChange` once at mount with an empty list, and publishing that emptiness is how a
   * selection dispatched a few milliseconds BEFORE this surface existed was wiped: measured on
   * generation3d, `interactionSelect targets:[] domainId:graph` left this component 15 ms before
   * `node-graph host mount`, with no pointer event anywhere in the run
   * (`📓️hot-swap-board-remount-2026-09-15.md` §2). Adopted, never published — the ledger's own rule for
   * a mark that arrived FROM the plugin — and reflected onto the mounted nodes, so a selection made
   * before mount is APPLIED at mount instead of dropped. */
  const sceneSelection = useMemo(() => [...(scene.selection ?? [])], [scene.selection]);
  const interactionLedger = useMemo(() => createNodeGraphInteractionLedger(), []);
  const adoptedSelectionRef = useRef<string | null>(null);
  const sceneSelectionKey = nodeGraphSelectionMarkKey({ nodeIds: sceneSelection });
  if (adoptedSelectionRef.current !== sceneSelectionKey) {
    adoptedSelectionRef.current = sceneSelectionKey;
    interactionLedger.adoptSelection({ nodeIds: sceneSelection });
  }
  const initialNodes = useMemo(() => workflowNodesToDiagramNodes(parsedNodes, sceneSelection), [parsedNodes, sceneSelection]);
  const initialEdges = useMemo(() => workflowEdgesToDiagramEdges(parsedEdges), [parsedEdges]);
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

  const mapContextMenu = useMapContextMenuSpecs(dispatch);
  const shellContextMenuFallback = useShellContextMenuFallback();

  const containerRef = useRef<HTMLDivElement>(null);
  const [contextMenu, setContextMenu] = useState<(SurfaceContextMenuResult & { readonly x: number; readonly y: number }) | null>(null);
  const contextMenuTitleLabel = useLabel(contextMenu?.titleKey ?? "ui.surfaceContextMenu.node");

  return (
    <div
      ref={containerRef}
      className={cn("relative h-full w-full", surfaceClass)}
      data-level="base"
      onDragOver={(event) => {
        if (editable && event.dataTransfer.types.includes(CATALOGUE_DRAG_MIME)) event.preventDefault();
      }}
      onDrop={(event: DragEvent<HTMLDivElement>) => {
        if (!editable) return;
        event.preventDefault();
        const raw = event.dataTransfer.getData(CATALOGUE_DRAG_MIME);
        if (!raw) return;
        let payload: { readonly pluginId?: string; readonly appId?: string };
        try {
          payload = JSON.parse(raw) as { readonly pluginId?: string; readonly appId?: string };
        } catch {
          return;
        }
        if (!payload.pluginId || !payload.appId) return;
        const rect = containerRef.current?.getBoundingClientRect();
        if (!rect) return;
        const x = (event.clientX - rect.left - viewport.x) / viewport.zoom;
        const y = (event.clientY - rect.top - viewport.y) / viewport.zoom;
        dispatch("spawnApp", { pluginId: payload.pluginId, appId: payload.appId, x, y });
      }}
      onContextMenu={(event) => {
        if (!editable || !requestContextMenu) return;
        event.preventDefault();
        event.stopPropagation();
        void (async () => {
          const menu = await openSurfaceContextMenu(
            requestContextMenu,
            {
              menu: { id: "nodeGraph", args: null },
              surface: { surfaceId: node.surfaceId, kind: "nodeGraph", hits: [], selection: [] },
              point: { x: event.clientX, y: event.clientY },
            },
            mapContextMenu,
            shellContextMenuFallback,
          );
          setContextMenu({ x: event.clientX, y: event.clientY, ...menu });
        })();
      }}
    >
      <Diagram
        className="h-full w-full"
        nodeTypes={workflowNodeTypes}
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
        isValidConnection={(connection) => nodeGraphConnectionIsValid(parsedNodes, connection)}
        onNodesChange={(nextNodes) => setNodes(nextNodes as Node<WorkflowNodeData>[])}
        onEdgesChange={(nextEdges) => setEdges(nextEdges)}
        onNodeDragStop={
          editable
            ? (_event, draggedNode, draggedNodes) => {
                const origin = parsedNodes.find((record) => record.id === draggedNode.id);
                if (!origin) return;
                const record = { gestureId: `node-drag:${Date.now()}`, nodeIds: (draggedNodes.length > 0 ? draggedNodes : [draggedNode]).map((node) => node.id), dx: draggedNode.position.x - origin.x, dy: draggedNode.position.y - origin.y };
                if (nodeDragMoves(record)) dispatch(nodeGraphActions.edit, { operations: [nodeDragRow(record)] });
              }
            : undefined
        }
        onConnect={
          editable
            ? (connection) => {
                if (!connection.source || !connection.target || !connection.sourceHandle || !connection.targetHandle) return;
                if (!nodeGraphConnectionIsValid(parsedNodes, connection)) {
                  return;
                }
                dispatch(nodeGraphActions.edit, {
                  operations: [
                    {
                      operation: "connect",
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
          if (interactionLedger.publishSelection({ nodeIds: [clickedNode.id] })) publishNodeGraphSelection(dispatch, scene.interactionDomain, { nodeIds: [clickedNode.id] });
        }}
        onNodeDoubleClick={(_event, clickedNode) => {
          const record = parsedNodes.find((entry) => entry.id === clickedNode.id);
          if (record?.instanceId) dispatch("openInstance", { instanceId: record.instanceId });
        }}
        onSelectionChange={(selection) => {
          const nodeIds = selection.nodes.map((entry) => entry.id);
          if (interactionLedger.publishSelection({ nodeIds })) publishNodeGraphSelection(dispatch, scene.interactionDomain, { nodeIds });
        }}
      />
      <ContextMenuController
        title={contextMenuTitleLabel}
        open={contextMenu != null}
        position={contextMenu ?? { x: 0, y: 0 }}
        items={contextMenu?.items ?? []}
        onOpenChange={(open) => {
          if (!open) setContextMenu(null);
        }}
      />
    </div>
  );
}
//#endregion DiagramFallback
//#endregion 🔖️NodeGraphHost
