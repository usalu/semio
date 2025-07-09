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

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = ({
  id,
  data,
  selected,
}) => {
  // const { zoom } = useViewport();
  const {
    piece: { id_ },
    type: { ports },
  } = data;
  return (
    <div>
      <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
        <circle
          cx={ICON_WIDTH / 2}
          cy={ICON_WIDTH / 2}
          r={ICON_WIDTH / 2 - 1}
          className={`stroke-foreground stroke-2 ${selected ? "fill-primary " : "fill-transparent"} `}
        />
        {/* {zoom < 10 && */}
        <text
          x={ICON_WIDTH / 2}
          y={ICON_WIDTH / 2}
          textAnchor="middle"
          dominantBaseline="middle"
          className={`text-xs font-bold fill-foreground`}
        >
          {id_}
        </text>
        {/* } */}
      </svg>
      {/* {zoom > 10 && <Canvas className="w-full h-full">
                <OrbitControls
                    mouseButtons={{
                        LEFT: THREE.MOUSE.ROTATE, // Left mouse button for orbit/pan
                        MIDDLE: undefined,
                        RIGHT: undefined // Right button disabled to allow selection
                    }}
                />
                <Sphere args={[1, 100, 100]} position={[0, 0, 0]}>
                    <meshStandardMaterial color="gold" roughness={0} metalness={1} />
                </Sphere>
                <ambientLight intensity={1} />
            </Canvas>} */}
      {ports?.map((port) => <PortHandle key={port.id_} port={port} />)}
    </div>
  );
};

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

  // const sourceHandle = getNode(source)?.handles.find((handle) => handle.id === sourceHandleId);
  // const targetHandle = getNode(target)?.handles.find((handle) => handle.id === targetHandleId);

  // const centerX = (sourceX + targetX) / 2;
  // const centerY = (sourceY + targetY) / 2;
  // unit vector pointing to center
  // const length = Math.sqrt((centerX) ** 2 + (centerY) ** 2);
  // const x = (centerX / length) * ICON_WIDTH;
  // const y = (centerY / length) * ICON_WIDTH;

  // const path = `M ${sourceX} ${sourceY} C ${sourceX + x} ${sourceY + y}, ${targetX - x} ${targetY - y}, ${targetX} ${targetY}`;
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

function useLogNodeArrayStability(nodes: Node[]) {
  const prevNodesRef = useRef<Node[]>([]);

  useEffect(() => {
    const prevNodes = prevNodesRef.current;
    const currIds = nodes.map((n) => n.id);
    const prevIds = prevNodes.map((n) => n.id);

    // Check for changes
    const sameLength = currIds.length === prevIds.length;
    const sameOrder = sameLength && currIds.every((id, i) => id === prevIds[i]);
    const sameSet =
      sameLength &&
      currIds.every((id) => prevIds.includes(id)) &&
      prevIds.every((id) => currIds.includes(id));

    // Log details
    console.log("---- Node Array Stability Check ----");
    console.log("Current IDs:", currIds);
    console.log("Previous IDs:", prevIds);
    console.log("Same length:", sameLength);
    console.log("Same order:", sameOrder);
    console.log("Same set:", sameSet);
    if (!sameOrder) {
      console.warn("Node order changed!");
    }
    if (!sameSet) {
      console.warn("Node set changed (IDs added/removed)!");
    }
    if (!sameLength) {
      console.warn("Node count changed!");
    }
    prevNodesRef.current = nodes;
  }, [nodes]);
}

interface DiagramProps {
  kit: Kit;
  designId: DesignId;
  fileUrls: Map<string, string>;
  fullscreen?: boolean;
  selection?: DesignEditorSelection;
  onDesignChange?: (design: Design) => void;
  onSelectionChange?: (selection: DesignEditorSelection) => void;
  onPanelDoubleClick?: () => void;
}

const Diagram: FC<DiagramProps> = ({
  kit,
  designId,
  fullscreen,
  selection,
  onPanelDoubleClick,
  onDesignChange,
  onSelectionChange,
}) => {
  const nodeTypes = useMemo(() => ({ piece: PieceNodeComponent }), []);
  const edgeTypes = useMemo(
    () => ({ connection: ConnectionEdgeComponent }),
    [],
  );
  console.log("render Diagram");
  const [dragState, setDragState] = useState<DragState | null>(null);

  const { setNodeRef } = useDroppable({
    id: "diagram",
  });

  const normalize = (value: string | undefined) =>
    value === undefined ? "" : value;
  const design = kit.designs?.find(
    (design) =>
      design.name === designId.name &&
      normalize(design.variant) === normalize(designId.variant) &&
      normalize(design.view) === normalize(designId.view),
  );
  if (!design) {
    return null;
  }
  const flatDesign = design ? flattenDesign(kit, designId) : null;
  const types = kit?.types ?? [];
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

  const onNodeClick = useCallback(
    (e: React.MouseEvent, node: Node) => {
      e.stopPropagation();
      if (selection && onSelectionChange) {
        if (node.selected) {
          onSelectionChange?.({
            ...selection,
            selectedPieceIds: selection?.selectedPieceIds.filter(
              (id) => id !== node.id,
            ),
          });
        } else {
          onSelectionChange?.({
            ...selection,
            selectedPieceIds: [...selection?.selectedPieceIds, node.id],
          });
        }
      }
    },
    [selection, onSelectionChange],
  );

  const onNodeDragStart = useCallback(
    (e: React.MouseEvent, node: Node) => {
      // if dragged node is not part of selection, deselect all pieceNodes, add dragged node to selection
      if (
        !selection?.selectedPieceIds.includes(
          (node as PieceNode).data.piece.id_,
        )
      ) {
        onSelectionChange?.({
          ...selection,
          selectedPieceIds: [(node as PieceNode).data.piece.id_],
          selectedConnections: selection?.selectedConnections ?? [],
        });
      }
      setDragState({
        nodeStartPosition: node.position,
        nodeIntermediatePosition: node.position,
      });
      // No intermediate nodes needed
      // console.log("onNodeDragStart", dragState, e, node);
    },
    [setDragState, dragState, pieceNodes, selection, onSelectionChange],
  );

  const onNodeDrag = useCallback(
    (e: React.MouseEvent, node: Node) => {
      if (dragState) {
        setDragState({
          ...dragState,
          nodeIntermediatePosition: node.position,
        });
      }
      // console.log("onNodeDrag", e, node);
    },
    [dragState],
  );

  const onNodeDragStop = useCallback(
    (e: React.MouseEvent, node: Node) => {
      const selectedFixedPieces = pieceNodes
        ?.filter((n) => n.selected && n.data.piece.center)
        .map((n) => n.data.piece);
      const offset = {
        x: (node.position.x - dragState!.nodeStartPosition!.x) / ICON_WIDTH,
        y: -(node.position.y - dragState!.nodeStartPosition!.y) / ICON_WIDTH,
      };
      const offsettedPieces = selectedFixedPieces?.map((p) => ({
        ...p,
        center: { x: p.center!.x + offset.x, y: p.center!.y + offset.y },
      }));
      onDesignChange?.({
        ...design,
        pieces: design.pieces?.map(
          (p) => offsettedPieces?.find((op) => op.id_ === p.id_) || p,
        ),
      });
      // No intermediate nodes to clear
      setDragState(null);
    },
    [dragState, pieceNodes, selection, onDesignChange, design],
  );

  const onDoubleClickCapture = useCallback(
    (e: React.MouseEvent) => {
      e.stopPropagation();
      onPanelDoubleClick?.();
    },
    [onPanelDoubleClick],
  );

  useLogNodeArrayStability(pieceNodes);

  return (
    <div id="diagram" className="h-full w-full">
      <ReactFlow
        ref={setNodeRef}
        nodes={pieceNodes}
        edges={connectionEdges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        connectionMode={ConnectionMode.Loose}
        elementsSelectable
        minZoom={0.1}
        maxZoom={12}
        // onNodesChange={onNodesChange}
        // onEdgesChange={onEdgesChange}
        // onConnect={onConnect}
        // TODO: Whenever another components updates the selection, onSelectionChange is called. Figure out how to prevent this.
        // onSelectionChange={({ pieceNodes, connectionEdges }) => {
        //     const selectedPieceIds = pieceNodes.map((node) => node.id);
        //     const selectedConnections = connectionEdges.map((edge) => {
        //         return {
        //             connectedPieceId: edge.source,
        //             connectingPieceId: edge.target,
        //         }
        //     });
        //     // When another component updates the selection, pieceNodes and connectionEdges are empty but we don't want this to reset the selection
        //     if (selectedPieceIds.length === 0 && selectedConnections.length === 0) {
        //         return;
        //     }

        //     // Only trigger onSelectionChange if the selection actually changed
        //     const currentSelection = selection || { selectedPieceIds: [], selectedConnections: [] };
        //     const piecesChanged =
        //         selectedPieceIds.length !== currentSelection.selectedPieceIds.length ||
        //         selectedPieceIds.some(id => !currentSelection.selectedPieceIds.includes(id));
        //     const connectionsChanged =
        //         selectedConnections.length !== currentSelection.selectedConnections.length ||
        //         selectedConnections.some(conn =>
        //             !currentSelection.selectedConnections.some(
        //                 currConn => currConn.connectedPieceId === conn.connectedPieceId &&
        //                     currConn.connectingPieceId === conn.connectingPieceId
        //             )
        //         );

        //     if (piecesChanged || connectionsChanged) {
        //         onSelectionChange?.({
        //             selectedPieceIds,
        //             selectedConnections
        //         });
        //     }
        // }}
        // onNodeDragStart={onNodeDragStart}
        // onNodeDrag={onNodeDrag}
        // onNodeDragStop={onNodeDragStop}
        // onNodeClick={onNodeClick}
        zoomOnDoubleClick={false}
        onDoubleClickCapture={onDoubleClickCapture}
        panOnDrag={[0]} //left mouse button
        proOptions={{ hideAttribution: true }}
        multiSelectionKeyCode="Shift"
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

export default Diagram;
