import React, {
  FC,
  useCallback,
  useMemo,
  useState,
  useEffect,
  useRef,
} from "react";
import {
  addEdge,
  Background,
  BackgroundVariant,
  BaseEdge,
  BuiltInNode,
  ConnectionMode,
  Controls,
  Edge,
  EdgeProps,
  getBezierPath,
  getStraightPath,
  Handle,
  HandleProps,
  MiniMap,
  MiniMapNodeProps,
  Node,
  NodeProps,
  OnConnect,
  OnEdgesChange,
  OnNodesChange,
  Panel,
  Position,
  ReactFlow,
  ReactFlowProvider,
  useEdgesState,
  useNodesState,
  useOnViewportChange,
  useReactFlow,
  useStoreApi,
  useViewport,
  Viewport,
  ViewportPortal,
  useStore,
  XYPosition,
  InternalNode,
  applyNodeChanges,
  NodeChange,
  ReactFlowInstance,
} from "@xyflow/react";
import * as THREE from "three";
import { useDroppable } from "@dnd-kit/core";
import { Canvas } from "@react-three/fiber";
import { OrbitControls, Sphere } from "@react-three/drei";

import {
  Connection,
  Design,
  DesignId,
  DiagramPoint,
  flattenDesign,
  ICON_WIDTH,
  Kit,
  Piece,
  Port,
  Type,
  TypeId,
} from "@semio/js/semio";
import {
  Avatar,
  AvatarFallback,
  AvatarImage,
} from "@semio/js/components/ui/Avatar";

// import '@xyflow/react/dist/base.css';
import "@xyflow/react/dist/style.css";
import "@semio/js/globals.css";
import { DesignEditorSelection } from "../..";
import { normalize } from "path";

const Diagram: FC<DiagramProps> = ({
  kit,
  designId,
  fullscreen,
  selection,
  onPanelDoubleClick,
  onDesignChange,
  onSelectionChange,
}) => {
  console.log("[Diagram] Rendering");
  // Mapping the semio design to react flow nodes and edges
  const nodesAndEdges = useMemo(
    () => mapDesignToNodesAndEdges({ kit, designId, selection }),
    [kit, designId, selection],
  );
  if (!nodesAndEdges) return null;
  const { nodes, edges } = nodesAndEdges;

  // drage state
  const [dragState, setDragState] = useState<{
    origin: XYPosition;
    offset: XYPosition;
  } | null>(null);

  const { handleNodeDragStart, handleNodeDrag, handleNodeDragStop } =
    useDragHandle(setDragState);

  // update nodes state
  const displayNodes = useDisplayNodes(dragState, nodes, selection);

  const reactFlowInstance = useReactFlow();

  const ghostEdge = useMemo(
    () => getProximityEdge(displayNodes, nodes, reactFlowInstance),
    [displayNodes, nodes, reactFlowInstance],
  );

  const displayEdges = useMemo(() => {
    if (ghostEdge) {
      return [...edges, ghostEdge];
    }
    return edges;
  }, [edges, ghostEdge]);

  return (
    <div id="diagram" className="h-full w-full">
      <ReactFlow
        ref={useDiagramDroppableNodeRef()}
        nodes={displayNodes}
        edges={displayEdges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionMode={ConnectionMode.Loose}
        elementsSelectable={false}
        minZoom={0.1}
        maxZoom={12}
        zoomOnDoubleClick={false}
        panOnDrag={[0]} //left mouse button
        proOptions={{ hideAttribution: true }}
        multiSelectionKeyCode="Shift"
        onNodeClick={(event, node) => {
          toggleNodeSelection(node.id, selection, onSelectionChange, event);
        }}
        onNodeDragStart={handleNodeDragStart}
        onNodeDrag={handleNodeDrag}
        onNodeDragStop={handleNodeDragStop}
      >
        {fullscreen && (
          <Controls
            className="border"
            showZoom={false}
            showInteractive={false}
          />
        )}
        {fullscreen && (
          <MiniMap
            className="border"
            maskColor="var(--accent)"
            bgColor="var(--background)"
            nodeComponent={MiniMapNode}
          />
        )}
        <ViewportPortal>⌞</ViewportPortal>
      </ReactFlow>
    </div>
  );
};

function getProximityEdge(
  displayNodes: PieceNode[],
  nodes: PieceNode[],
  reactFlowInstance: ReactFlowInstance,
): Edge | null {
  const ghostNode = displayNodes.find((node) => node.data.isGhost);

  if (!ghostNode) {
    return null;
  }

  const closestEdge = getClosestEdge(
    ghostNode,
    nodes,
    reactFlowInstance.getInternalNode,
  );

  if (!closestEdge) {
    return null;
  }

  return {
    id: "ghost-edge",
    source: closestEdge.source,
    sourceHandle: closestEdge.sourceHandle,
    target: closestEdge.target,
    targetHandle: closestEdge.targetHandle,
    className: "temp",
  };
}

function useDragHandle(
  setDragState: React.Dispatch<
    React.SetStateAction<{
      origin: XYPosition;
      offset: XYPosition;
    } | null>
  >,
) {
  const handleNodeDragStart = useCallback((event: any, node: Node) => {
    setDragState({
      origin: { x: node.position.x, y: node.position.y },
      offset: { x: 0, y: 0 },
    });
  }, []);

  const handleNodeDrag = useCallback(
    (event: any, node: Node) => {
      setDragState((prev) =>
        prev
          ? {
              ...prev,
              offset: {
                x: node.position.x - prev.origin.x,
                y: node.position.y - prev.origin.y,
              },
            }
          : null,
      );
    },
    [setDragState],
  );

  const handleNodeDragStop = useCallback(() => {
    setDragState(null);
  }, []);
  return { handleNodeDragStart, handleNodeDrag, handleNodeDragStop };
}

function useDisplayNodes(
  dragState: {
    origin: XYPosition;
    offset: XYPosition;
  } | null,
  nodes: PieceNode[],
  selection: { selectedPieceIds?: string[] },
) {
  const reactFlowInstance = useReactFlow();
  return useMemo(() => {
    if (!dragState) return nodes;
    const { offset } = dragState;
    const selectedNodeIds = selection.selectedPieceIds ?? [];

    // Grey out all originals being dragged
    const nodesWithGreyedOut = nodes.map((node) =>
      selectedNodeIds.includes(node.id)
        ? { ...node, data: { ...node.data, isBeingDragged: true } }
        : node,
    );
    // Add ghost nodes for all being dragged
    const ghostNodes: PieceNode[] = selectedNodeIds
      .map((id) => {
        const orig = nodes.find((n) => n.id === id);
        if (!orig) return undefined;
        // Copy all properties from the original node, only override what needs to be unique
        return {
          ...orig,
          id: "ghost" + id,
          position: {
            x: orig.position.x + offset.x,
            y: orig.position.y + offset.y,
          },
          data: { ...orig.data, isGhost: true },
          selected: true,
        };
      })
      .filter(Boolean) as PieceNode[];

    return [...nodesWithGreyedOut, ...ghostNodes];
  }, [nodes, dragState, selection]);
}

function toggleNodeSelection(
  nodeId: string,
  selection: { selectedPieceIds?: string[] },
  onSelectionChange: (sel: {
    selectedPieceIds: string[];
    selectedConnections: any[];
  }) => void,
  event?: React.MouseEvent,
) {
  const currentSelectedIds = selection?.selectedPieceIds ?? [];
  const isNodeSelected = currentSelectedIds.includes(nodeId);
  const isMultiSelect =
    event && (event.shiftKey || event.metaKey || event.ctrlKey);

  if (isMultiSelect) {
    // Multi-select: toggle node in selection
    if (isNodeSelected) {
      onSelectionChange({
        selectedPieceIds: currentSelectedIds.filter((id) => id !== nodeId),
        selectedConnections: [],
      });
    } else {
      onSelectionChange({
        selectedPieceIds: [...currentSelectedIds, nodeId],
        selectedConnections: [],
      });
    }
  } else {
    // Single select: only this node
    onSelectionChange({
      selectedPieceIds: [nodeId],
      selectedConnections: [],
    });
  }
}

type PieceNodeProps = {
  piece: Piece;
  type: Type;
  isBeingDragged: boolean;
  isGhost: boolean;
};

type PieceNode = Node<PieceNodeProps, "piece">;
type DiagramNode = PieceNode;

type ConnectionEdge = Edge<{ connection: Connection }, "connection">;
type DiagramEdge = ConnectionEdge;

type PortHandleProps = { port: Port };

const portPositionStyle = (port: Port): { x: number; y: number } => {
  // t is normalized in [0,1[ and clockwise and starts at 12 o'clock
  const { t } = port;
  if (t === undefined) {
    return { x: 0, y: 0 };
  }
  const angle = t * 2 * Math.PI;
  const radius = ICON_WIDTH / 2;
  return {
    x: radius * Math.sin(angle),
    y: -(radius * Math.cos(angle) - radius),
  };
};

function useDiagramDroppableNodeRef() {
  return useDroppable({ id: "diagram" }).setNodeRef;
}

const pieceToNode = (
  piece: Piece,
  type: Type,
  selected: boolean,
): PieceNode => ({
  type: "piece",
  id: piece.id_,
  position: {
    x: piece.center!.x * ICON_WIDTH || 0,
    y: -piece.center!.y * ICON_WIDTH || 0,
  },
  selected,
  data: { piece, type, isBeingDragged: false, isGhost: false },
});

const connectionToEdge = (
  connection: Connection,
  selected: boolean,
): ConnectionEdge => ({
  type: "connection",
  id: `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`,
  source: connection.connecting.piece.id_,
  sourceHandle: connection.connecting.port.id_,
  target: connection.connected.piece.id_,
  targetHandle: connection.connected.port.id_,
  data: { connection }, // removed 'selected' property
});

const PortHandle: React.FC<PortHandleProps> = ({ port }) => {
  const { x, y } = portPositionStyle(port);

  return (
    <Handle
      id={port.id_}
      type="source"
      style={{ left: x + ICON_WIDTH / 2, top: y }}
      position={Position.Top}
    />
  );
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(
  ({ id, data, selected }) => {
    const {
      piece: { id_ },
      type: { ports },
      isBeingDragged,
      isGhost,
    } = data as any;
    return (
      <div style={{ opacity: isBeingDragged ? 0.5 : 1 }}>
        <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
          <circle
            cx={ICON_WIDTH / 2}
            cy={ICON_WIDTH / 2}
            r={ICON_WIDTH / 2 - 1}
            className={`stroke-foreground stroke-2 ${selected ? "fill-primary " : "fill-transparent"} `}
          />
          <text
            x={ICON_WIDTH / 2}
            y={ICON_WIDTH / 2}
            textAnchor="middle"
            dominantBaseline="middle"
            className={`text-xs font-bold fill-foreground`}
          >
            {id_}
          </text>
        </svg>
        {ports?.map((port: Port) => <PortHandle key={port.id_} port={port} />)}
      </div>
    );
  },
);

const ConnectionEdgeComponent: React.FC<EdgeProps<ConnectionEdge>> = ({
  id,
  source,
  target,
  sourceX,
  sourceY,
  targetX,
  targetY,
  sourceHandleId,
  targetHandleId,
}) => {
  const { getNode } = useReactFlow();
  const HANDLE_HEIGHT = 5;
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`;

  const isGhost = id === "ghost-edge";
  console.log("[Drag] Edge:", id, isGhost);
  console.log("qwer");

  return (
    <BaseEdge
      path={path}
      style={{
        strokeDasharray: isGhost ? "5 5" : undefined,
        stroke: isGhost ? "var(--primary)" : "var(--foreground)",
      }}
    />
  );
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }) => {
  return (
    <circle
      className={selected ? "fill-primary" : "fill-foreground"}
      cx={x}
      cy={y}
      r="10"
    />
  );
};

interface DragState {
  nodeStartPosition: XYPosition;
  nodeIntermediatePosition?: XYPosition;
}

function createDummyType(name: string): Type {
  return {
    name: name,
    description: `A dummy type for ${name}`,
    icon: "🧩",
    image: "",
    variant: "",
    unit: "m",
    created: new Date(),
    updated: new Date(),
    authors: [],
    ports: [
      {
        id_: "p1",
        t: 0.25,
        description: "",
        family: "",
        compatibleFamilies: [],
        point: { x: 0, y: 0, z: 0 },
        direction: { x: 0, y: 0, z: 1 },
      },
      {
        id_: "p2",
        t: 0.75,
        description: "",
        family: "",
        compatibleFamilies: [],
        point: { x: 0, y: 0, z: 0 },
        direction: { x: 0, y: 0, z: 1 },
      },
    ],
  };
}

function mapDesignToNodesAndEdges({
  kit,
  designId,
  selection,
}: {
  kit: Kit;
  designId: DesignId;
  selection?: DesignEditorSelection;
}): { nodes: PieceNode[]; edges: ConnectionEdge[] } | null {
  /*
  const types = kit?.types ?? [];
  const normalize = (value: string | undefined) =>
    value === undefined ? "" : value;
  const design = kit.designs?.find(
    (design) =>
      design.name === designId.name &&
      normalize(design.variant) === normalize(designId.variant) &&
      normalize(design.view) === normalize(designId.view),
  );
  if (!design) return null;
  const flatDesign = flattenDesign(kit, designId);
  const pieceNodes =
    flatDesign!.pieces?.map((flatPiece) =>
      pieceToNode(
        flatPiece,
        types.find(
          (t) =>
            t.name === flatPiece.type.name &&
            (t.variant ?? "") === (flatPiece.type.variant ?? ""),
        )!,
        selection?.selectedPieceIds.includes(flatPiece.id_) ?? false,
      ),
    ) ?? [];
  const connectionEdges =
    design.connections?.map((connection) =>
      connectionToEdge(
        connection,
        selection?.selectedConnections.some(
          (c) =>
            c.connectingPieceId === connection.connecting.piece.id_ &&
            c.connectedPieceId === connection.connected.piece.id_,
        ) ?? false,
      ),
    ) ?? [];
  return { nodes: pieceNodes, edges: connectionEdges };
  */
  const nodes: PieceNode[] = [
    {
      id: "A",
      type: "piece" as const,
      position: { x: 100, y: 100 },
      data: {
        piece: {
          id_: "A",
          type: { name: "typeA" },
          center: { x: 100 / ICON_WIDTH, y: -100 / ICON_WIDTH, z: 0 },
        },
        type: createDummyType("typeA"),
        isBeingDragged: false,
        isGhost: false,
      },
    },
    {
      id: "B",
      type: "piece" as const,
      position: { x: 400, y: 100 },
      data: {
        piece: {
          id_: "B",
          type: { name: "typeB" },
          center: { x: 400 / ICON_WIDTH, y: -100 / ICON_WIDTH, z: 0 },
        },
        type: createDummyType("typeB"),
        isBeingDragged: false,
        isGhost: false,
      },
    },
    {
      id: "C",
      type: "piece" as const,
      position: { x: 100, y: 300 },
      data: {
        piece: {
          id_: "C",
          type: { name: "typeC" },
          center: { x: 100 / ICON_WIDTH, y: -300 / ICON_WIDTH, z: 0 },
        },
        type: createDummyType("typeC"),
        isBeingDragged: false,
        isGhost: false,
      },
    },
    {
      id: "D",
      type: "piece" as const,
      position: { x: 400, y: 300 },
      data: {
        piece: {
          id_: "D",
          type: { name: "typeD" },
          center: { x: 400 / ICON_WIDTH, y: -300 / ICON_WIDTH, z: 0 },
        },
        type: createDummyType("typeD"),
        isBeingDragged: false,
        isGhost: false,
      },
    },
  ].map((n) => ({
    ...n,
    selected: selection?.selectedPieceIds.includes(n.id) ?? false,
  }));

  return { nodes, edges: [] };
}

const nodeTypes = { piece: PieceNodeComponent };
const edgeTypes = { connection: ConnectionEdgeComponent };

interface DiagramProps {
  kit: Kit;
  designId: DesignId;
  fileUrls: Map<string, string>;
  fullscreen?: boolean;
  selection: DesignEditorSelection;
  onDesignChange?: (design: Design) => void;
  onSelectionChange: (selection: DesignEditorSelection) => void;
  onPanelDoubleClick?: () => void;
}

export default Diagram;

export const MIN_DISTANCE = 150;

function getClosestEdge(
  node: Node,
  nodes: PieceNode[],
  getInternalNode: (id: string) => InternalNode<Node> | undefined,
): Edge | null {
  let draggedHandles: Array<{ handleId: string; x: number; y: number }>;

  // For ghost nodes, we can't rely on getInternalNode as it might not be updated yet.
  // We calculate handle positions manually based on the node's data.
  if (node.data.isGhost) {
    const pieceNode = node as PieceNode;
    const ports = pieceNode.data.type.ports || [];
    draggedHandles = ports
      .filter((port) => port.id_)
      .map((port) => {
        const { x: portX, y: portY } = portPositionStyle(port);
        return {
          handleId: port.id_ as string,
          x: pieceNode.position.x + portX + ICON_WIDTH / 2,
          y: pieceNode.position.y + portY,
        };
      });
  } else {
    const draggedNode = getInternalNode(node.id);
    if (!draggedNode) return null;
    draggedHandles = getAbsoluteHandlePositions(draggedNode);
  }

  let closest = {
    distance: Number.MAX_VALUE,
    source: null as null | { nodeId: string; handleId: string },
    target: null as null | { nodeId: string; handleId: string },
  };

  const originalDraggedId = node.data.isGhost ? node.id.slice(5) : null;

  // Iterate over all other nodes in the array
  for (const otherNode of nodes) {
    if (otherNode.id === node.id || otherNode.id === originalDraggedId)
      continue;
    const otherInternalNode = getInternalNode(otherNode.id);
    if (!otherInternalNode) continue;
    // Get all handles on the other node
    const otherHandles = getAbsoluteHandlePositions(otherInternalNode);
    // Compare all handles between dragged node and other node
    for (const draggedHandle of draggedHandles) {
      for (const otherHandle of otherHandles) {
        const dx = draggedHandle.x - otherHandle.x;
        const dy = draggedHandle.y - otherHandle.y;
        const distance = Math.sqrt(dx * dx + dy * dy);
        if (distance < closest.distance && distance < MIN_DISTANCE) {
          // Always assign the node with the lower id as source for consistency
          if (node.id < otherNode.id) {
            closest = {
              distance,
              source: {
                nodeId: node.id,
                handleId: draggedHandle.handleId,
              },
              target: { nodeId: otherNode.id, handleId: otherHandle.handleId },
            };
          } else {
            closest = {
              distance,
              source: { nodeId: otherNode.id, handleId: otherHandle.handleId },
              target: {
                nodeId: node.id,
                handleId: draggedHandle.handleId,
              },
            };
          }
        }
      }
    }
  }

  // If no close handle pair found, return null
  if (!closest.source || !closest.target) {
    return null;
  }

  // Return an edge connecting the closest handles
  return {
    id: `${closest.source.nodeId}-${closest.source.handleId}__${closest.target.nodeId}-${closest.target.handleId}`,
    source: closest.source.nodeId,
    sourceHandle: closest.source.handleId,
    target: closest.target.nodeId,
    targetHandle: closest.target.handleId,
  };
}

// Helper to get absolute handle positions for a node (all handles are undirected)
function getAbsoluteHandlePositions(node: InternalNode<Node>): Array<{
  handleId: string;
  x: number;
  y: number;
}> {
  const sourceHandles = node.internals.handleBounds?.source ?? [];
  const targetHandles = node.internals.handleBounds?.target ?? [];
  const handles = [...sourceHandles, ...targetHandles];
  return handles
    .filter((handle) => handle.id !== null && handle.id !== undefined)
    .map((handle) => ({
      handleId: handle.id as string,
      x: node.internals.positionAbsolute.x + handle.x + handle.width / 2,
      y: node.internals.positionAbsolute.y + handle.y + handle.height / 2,
    }));
}
