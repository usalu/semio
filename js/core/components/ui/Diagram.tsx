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
  Node as RFNode,
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
  //TODO on drag save drag state
  //TODO add copies of all selected nodes with drag position offset

  // Track drag state
  const [dragState, setDragState] = useState<{
    nodeId: string;
    offset: XYPosition;
    position: XYPosition;
  } | null>(null);

  // Mapping the semio design to react flow nodes and edges
  const nodesAndEdges = useMemo(
    () => mapDesignToNodesAndEdges({ kit, designId, selection }),
    [kit, designId, selection],
  );
  if (!nodesAndEdges) return null;
  const { nodes, edges } = nodesAndEdges;

  const { handleNodeDragStart, handleNodeDrag, handleNodeDragStop } =
    useDragHandle(setDragState, dragState);

  // render nodes
  const displayNodes = useDisplayNodes(dragState, nodes);

  return (
    <div id="diagram" className="h-full w-full">
      <ReactFlow
        ref={useDiagramDroppableNodeRef()}
        nodes={displayNodes}
        edges={edges}
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

function useDragHandle(
  setDragState: React.Dispatch<
    React.SetStateAction<{
      nodeId: string;
      offset: XYPosition;
      position: XYPosition;
    } | null>
  >,
  dragState: {
    nodeId: string;
    offset: XYPosition;
    position: XYPosition;
  } | null,
) {
  const handleNodeDragStart = useCallback((event: any, node: RFNode) => {
    setDragState({
      nodeId: node.id,
      offset: { x: 0, y: 0 },
      position: node.position,
    });
  }, []);

  // Handle node drag: update dragState.position in React state
  const handleNodeDrag = useCallback(
    (event: any, node: RFNode) => {
      if (!dragState) return;
      setDragState((prev) =>
        prev
          ? {
              ...prev,
              position: node.position,
            }
          : null,
      );
    },
    [dragState],
  );

  // Handle node drag stop: clear dragState
  const handleNodeDragStop = useCallback(() => {
    setDragState(null);
  }, []);
  return { handleNodeDragStart, handleNodeDrag, handleNodeDragStop };
}

function useDisplayNodes(
  dragState: {
    nodeId: string;
    offset: XYPosition;
    position: XYPosition;
  } | null,
  nodes: PieceNode[],
) {
  return useMemo(() => {
    if (!dragState) return nodes;
    const nodeBeingDragged = nodes.find((node) => node.id === dragState.nodeId);
    if (!nodeBeingDragged) return nodes;

    // Grey out the original node
    const nodesWithGreyedOutNodes = nodes.map((node) => {
      if (node.id !== nodeBeingDragged.id) return node;
      else return { ...node, data: { ...node.data, ghost: true } };
    });

    const ghostNode = {
      ...nodeBeingDragged,
      id: "ghost" + nodeBeingDragged.id,
      position: dragState.position,
      data: { ...nodeBeingDragged.data, ghost: false },
      selected: true,
    };
    return [...nodesWithGreyedOutNodes, ghostNode];
  }, [nodes, dragState]);
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
  data: { piece, type },
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

// Patch PieceNodeComponent to support ghost/alpha rendering
const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(
  ({ id, data, selected }) => {
    const {
      piece: { id_ },
      type: { ports },
      ghost,
    } = data as any;
    return (
      <div style={{ opacity: ghost ? 0.5 : 1 }}>
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
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + +HANDLE_HEIGHT / 2}`;
  return <BaseEdge path={path} />;
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

function mapDesignToNodesAndEdges({
  kit,
  designId,
  selection,
}: {
  kit: Kit;
  designId: DesignId;
  selection?: DesignEditorSelection;
}): { nodes: PieceNode[]; edges: ConnectionEdge[] } | null {
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
