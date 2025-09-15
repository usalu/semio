import { useDroppable } from "@dnd-kit/core";
import {
  BaseEdge,
  ConnectionLineComponentProps,
  ConnectionMode,
  Controls,
  Edge,
  EdgeProps,
  Handle,
  MiniMap,
  MiniMapNodeProps,
  Node,
  NodeProps,
  Position,
  ReactFlow,
  Connection as RFConnection,
  useReactFlow,
  ViewportPortal,
  XYPosition,
} from "@xyflow/react";
import React, { FC, useCallback, useEffect, useState } from "react";

import { arePortsCompatible, Connection, Coord, DesignId, DiffStatus, findAttributeValue, findPortInType, findTypeInKit, getIncludedDesigns, ICON_WIDTH, isPortInUse, isSameConnection, Piece, Port, TOLERANCE, Type } from "../../../semio";

import "@xyflow/react/dist/style.css";
import {
  DesignEditorFullscreenPanel,
  DesignEditorPresenceOther,
  PieceScopeProvider,
  useClusterableGroups,
  useDesign,
  useDesignEditorCommands,
  useDesignEditorFullscreen,
  useDesignEditorOthers,
  useDesignEditorSelection,
  useExplodeableDesignNodes,
  useKit,
  useKitCommands,
} from "../../../store";

type ClusterMenuProps = {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  onCluster: (clusterPieceIds: string[]) => void;
};

const ClusterMenu: FC<ClusterMenuProps> = ({ nodes, edges, onCluster }) => {
  const reactFlowInstance = useReactFlow();
  const clusterableGroups = useClusterableGroups();

  const getBoundingBoxForGroup = useCallback(
    (groupPieceIds: string[]) => {
      const groupNodes = nodes.filter((node) => groupPieceIds.includes(node.data.piece.id_));

      if (groupNodes.length === 0) return null;

      let minX = Infinity;
      let minY = Infinity;
      let maxX = -Infinity;
      let maxY = -Infinity;

      groupNodes.forEach((node) => {
        const x = node.position.x;
        const y = node.position.y;
        const width = ICON_WIDTH;
        const height = ICON_WIDTH;

        minX = Math.min(minX, x);
        minY = Math.min(minY, y);
        maxX = Math.max(maxX, x + width);
        maxY = Math.max(maxY, y + height);
      });

      const padding = 20;
      return {
        x: minX - padding,
        y: minY - padding,
        width: maxX - minX + padding * 2,
        height: maxY - minY + padding * 2,
      };
    },
    [nodes],
  );

  if (clusterableGroups.length === 0) {
    return null;
  }

  return (
    <ViewportPortal>
      {clusterableGroups.map((groupPieceIds, groupIndex) => {
        const boundingBox = getBoundingBoxForGroup(groupPieceIds);
        if (!boundingBox) return null;

        return (
          <div
            key={`cluster-group-${groupIndex}`}
            className="absolute pointer-events-none"
            style={{
              left: boundingBox.x,
              top: boundingBox.y,
              width: boundingBox.width,
              height: boundingBox.height,
            }}
          >
            <div className="absolute inset-0 border-2 border-dashed border-primary/50 rounded-md" style={{ pointerEvents: "none" }} />
            <div className="absolute -top-10 -right-2 pointer-events-auto">
              <button onClick={() => onCluster(groupPieceIds)} className="bg-primary text-primary-foreground px-3 py-1 rounded-md text-sm font-medium shadow-md hover:bg-primary/90 transition-colors">
                Cluster
              </button>
            </div>
          </div>
        );
      })}
    </ViewportPortal>
  );
};

type ExpandMenuProps = {
  nodes: DiagramNode[];
  edges: DiagramEdge[];
  onExpand: (designId: DesignId) => void;
};

const ExpandMenu: FC<ExpandMenuProps> = ({ nodes, edges, onExpand }) => {
  const selection = useDesignEditorSelection();
  const explodeableDesignNodes = useExplodeableDesignNodes(nodes, selection);

  const getBoundingBoxForNode = useCallback((node: DiagramNode) => {
    const x = node.position.x;
    const y = node.position.y;
    const width = ICON_WIDTH;
    const height = ICON_WIDTH;

    const padding = 20;
    return {
      x: x - padding,
      y: y - padding,
      width: width + padding * 2,
      height: height + padding * 2,
    };
  }, []);

  if (explodeableDesignNodes.length === 0) {
    return null;
  }

  return (
    <ViewportPortal>
      {explodeableDesignNodes.map((node) => {
        const boundingBox = getBoundingBoxForNode(node);
        const designName = (node.data.piece as Piece).type?.variant!;

        return (
          <div
            key={`explode-design-${designName}`}
            className="absolute pointer-events-none"
            style={{
              left: boundingBox.x,
              top: boundingBox.y,
              width: boundingBox.width,
              height: boundingBox.height,
            }}
          >
            <div className="absolute inset-0 border-2 border-dashed border-secondary/50 rounded-md" style={{ pointerEvents: "none" }} />
            <div className="absolute -top-10 -right-2 pointer-events-auto">
              <button onClick={() => onExpand({ name: designName })} className="bg-secondary text-secondary-foreground px-3 py-1 rounded-md text-sm font-medium shadow-md hover:bg-secondary/90 transition-colors">
                Expand
              </button>
            </div>
          </div>
        );
      })}
    </ViewportPortal>
  );
};

const PresenceDiagram: FC<DesignEditorPresenceOther> = ({ name, cursor, camera }) => {
  if (!cursor) return null;
  return (
    <ViewportPortal>
      <div
        style={{
          transform: `translate(${cursor.x * ICON_WIDTH}px, ${-cursor.y * ICON_WIDTH}px)`,
          position: "absolute",
          pointerEvents: "none",
          zIndex: 1000,
        }}
      >
        <div className="flex items-center gap-1 bg-primary text-primary-foreground px-2 py-1 rounded-full text-xs shadow-lg">
          <div className="w-2 h-2 bg-primary-foreground rounded-full"></div>
          {name}
        </div>
      </div>
    </ViewportPortal>
  );
};

type HelperLine = {
  type: "horizontal" | "vertical" | "equalDistance";
  position?: number;
  relatedPieceId: string;
  x1?: number;
  y1?: number;
  x2?: number;
  y2?: number;
  distance?: number;
  referencePieceIds?: string[];
};

type PieceNodeProps = {
  piece: Piece;
  type: Type;
};

type DesignNodeProps = {
  piece: Piece;
  externalConnections: Connection[];
};

type PieceNode = Node<PieceNodeProps, "piece">;
type DesignNode = Node<DesignNodeProps, "design">;
type DiagramNode = PieceNode | DesignNode;

type ConnectionEdge = Edge<{ connection: Connection; isParentConnection?: boolean }, "connection">;
type DiagramEdge = ConnectionEdge;

type PortHandleProps = {
  port: Port;
  pieceId: string;
  selected?: boolean;
  onPortClick: (port: Port) => void;
};

const getPortPositionStyle = (port: Port): { x: number; y: number } => {
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

const PortHandle: React.FC<PortHandleProps> = ({ port, pieceId, selected = false, onPortClick }) => {
  const { x, y } = getPortPositionStyle(port);
  const portColor = findAttributeValue(port, "semio.color", "var(--color-foreground)")!;

  const onClick = (event: React.MouseEvent) => {
    event.stopPropagation();
    onPortClick(port);
  };

  return (
    <Handle
      id={port.id_ ?? ""}
      type="source"
      className="left-1/2 top-0 cursor-pointer"
      style={{
        left: x + ICON_WIDTH / 2,
        top: y,
        backgroundColor: selected ? "var(--color-primary)" : portColor,
        border: selected ? "6px solid var(--color-primary)" : "0",
        zIndex: selected ? 20 : 10,
      }}
      position={Position.Top}
      onClick={onClick}
    />
  );
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data, selected }) => {
  const {
    piece,
    piece: { id_, attributes },
    type: { ports },
  } = data as PieceNodeProps & { diffStatus: DiffStatus };

  const { selectPiecePort, deselectPiecePort, addConnection } = useDesignEditorCommands();
  const selection = useDesignEditorSelection();

  const onPortClick = (port: Port) => {
    const currentSelectedPort = selection.port;

    if (currentSelectedPort && (currentSelectedPort.piece.id_ !== piece.id_ || currentSelectedPort.port.id_ !== port.id_)) {
      const connection: Connection = {
        connecting: {
          piece: { id_: currentSelectedPort.piece.id_ },
          port: { id_: currentSelectedPort.port.id_ },
        },
        connected: { piece: { id_: piece.id_ }, port: { id_: port.id_ } },
      };
      addConnection(connection);
      deselectPiecePort();
    } else if (currentSelectedPort && currentSelectedPort.piece.id_ === piece.id_ && currentSelectedPort.port.id_ === port.id_) {
      deselectPiecePort();
    } else if (port.id_) selectPiecePort({ id_: piece.id_ }, { id_: port.id_ });
  };

  let fillClass = "";
  let strokeClass = "stroke-foreground stroke-2";
  let opacity = 1;

  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;

  if (diff === DiffStatus.Added) {
    fillClass = selected ? "fill-[color-mix(in_srgb,theme(colors.success)_50%,theme(colors.primary)_50%)]" : "fill-success";
  } else if (diff === DiffStatus.Removed) {
    fillClass = selected ? "fill-[color-mix(in_srgb,theme(colors.danger)_50%,theme(colors.primary)_50%)]" : "fill-danger";
    strokeClass = "stroke-danger stroke-2";
    opacity = 0.2;
  } else if (diff === DiffStatus.Modified) {
    fillClass = selected ? "fill-[color-mix(in_srgb,theme(colors.warning)_50%,theme(colors.primary)_50%)]" : "fill-warning";
  } else if (selected) {
    fillClass = "fill-primary";
  } else {
    fillClass = "fill-transparent";
  }

  return (
    <PieceScopeProvider id={{ id_ }}>
      <div style={{ opacity }}>
        <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
          <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 1} className={`${strokeClass} ${fillClass}`} />
          <text x={ICON_WIDTH / 2} y={ICON_WIDTH / 2} textAnchor="middle" dominantBaseline="middle" className="text-xs font-bold fill-foreground">
            {id_}
          </text>
        </svg>
        {ports?.map((port: Port, portIndex: number) => (
          <PortHandle key={`${id}-port-${portIndex}-${port.id_}`} port={port} pieceId={id_} selected={selection.port?.piece.id_ === id_ && selection.port?.port.id_ === port.id_} onPortClick={onPortClick} />
        ))}
      </div>
    </PieceScopeProvider>
  );
});

const DesignNodeComponent: React.FC<NodeProps<DesignNode>> = React.memo(({ id, data, selected }) => {
  const {
    piece,
    piece: { id_, attributes },
    externalConnections,
  } = data as DesignNodeProps & { diffStatus: DiffStatus };

  const { selectPiecePort, deselectPiecePort, addConnection } = useDesignEditorCommands();
  const selection = useDesignEditorSelection();

  const ports: Port[] = externalConnections.map((connection, portIndex) => {
    const connectedIsDesignPiece = connection.connected.piece.id_ === piece.id_ || connection.connected.designPiece?.id_ === piece.id_;
    const connectingIsDesignPiece = connection.connecting.piece.id_ === piece.id_ || connection.connecting.designPiece?.id_ === piece.id_;

    const designSide = connectedIsDesignPiece ? connection.connected : connection.connecting;
    const originalSide = connectedIsDesignPiece ? connection.connecting : connection.connected;

    const totalPorts = externalConnections.length;
    const t = portIndex / totalPorts;

    const angle = t * 2 * Math.PI;
    const radius = 0.5;

    const portX = radius * Math.sin(angle);
    const portY = radius * Math.cos(angle);
    const portZ = 0;

    const directionX = Math.sin(angle);
    const directionY = Math.cos(angle);
    const directionZ = 0;

    return {
      id_: `port-${portIndex}`,
      description: `Port for connection to ${originalSide.piece.id_}:${originalSide.port.id_}`,
      family: "default",
      mandatory: false,
      t: t,
      point: { x: portX, y: portY, z: portZ },
      direction: { x: directionX, y: directionY, z: directionZ },
      attributes: [
        {
          key: "semio.originalPieceId",
          value: designSide.piece.id_ || "",
        },
        {
          key: "semio.originalPortId",
          value: designSide.port.id_ || "",
        },
        {
          key: "semio.externalPieceId",
          value: originalSide.piece.id_ || "",
        },
        {
          key: "semio.externalPortId",
          value: originalSide.port.id_ || "",
        },
      ],
    };
  });

  const onPortClick = (port: Port) => {
    const currentSelectedPort = selection.port;

    if (currentSelectedPort && (currentSelectedPort.piece.id_ !== piece.id_ || currentSelectedPort.port.id_ !== port.id_)) {
      const connection: Connection = {
        connecting: {
          piece: { id_: currentSelectedPort.piece.id_ },
          port: { id_: currentSelectedPort.port.id_ },
        },
        connected: { piece: { id_: piece.id_ }, port: { id_: port.id_ } },
      };
      addConnection(connection);
      deselectPiecePort();
    } else if (currentSelectedPort && currentSelectedPort.piece.id_ === piece.id_ && currentSelectedPort.port.id_ === port.id_) {
      deselectPiecePort();
    } else if (port.id_) selectPiecePort({ id_: piece.id_ }, { id_: port.id_ });
  };

  let fillClass = "";
  let strokeClass = "stroke-dark stroke-2";
  let opacity = 1;

  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;

  if (diff === DiffStatus.Added) {
    fillClass = selected ? "fill-[color-mix(in_srgb,theme(colors.success)_50%,theme(colors.primary)_50%)]" : "fill-success";
  } else if (diff === DiffStatus.Removed) {
    fillClass = selected ? "fill-[color-mix(in_srgb,theme(colors.danger)_50%,theme(colors.primary)_50%)]" : "fill-danger";
    strokeClass = "stroke-danger stroke-2";
    opacity = 0.2;
  } else if (diff === DiffStatus.Modified) {
    fillClass = selected ? "fill-[color-mix(in_srgb,theme(colors.warning)_50%,theme(colors.primary)_50%)]" : "fill-warning";
  } else if (selected) {
    fillClass = "fill-primary";
  } else {
    fillClass = "fill-background";
  }

  return (
    <div style={{ opacity }}>
      <svg width={ICON_WIDTH} height={ICON_WIDTH} className="cursor-pointer">
        <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 1} className={`${strokeClass} ${fillClass}`} />
        <text x={ICON_WIDTH / 2} y={ICON_WIDTH / 2} textAnchor="middle" dominantBaseline="middle" className="text-xs font-bold fill-dark">
          {id_}
        </text>
      </svg>
      {ports?.map((port: Port, portIndex: number) => (
        <PortHandle key={`${id}-port-${portIndex}-${port.id_}`} port={port} pieceId={id_} selected={selection.port?.piece.id_ === id_ && selection.port?.port.id_ === port.id_} onPortClick={onPortClick} />
      ))}
    </div>
  );
});
const nodeComponents = { piece: PieceNodeComponent, design: DesignNodeComponent };

const ConnectionEdgeComponent: React.FC<EdgeProps<ConnectionEdge>> = ({ id, source, target, sourceX, sourceY, targetX, targetY, sourceHandleId, targetHandleId, data, selected }) => {
  const HANDLE_HEIGHT = 5;
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`;

  const diff = (data?.connection?.attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isParentConnection = data?.isParentConnection ?? false;

  let stroke = "var(--color-dark)";
  let strokeWidth = 2;
  let dasharray: string | undefined;
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    stroke = selected ? "color-mix(in srgb, var(--color-success) 50%, var(--color-primary) 50%)" : "var(--color-success)";
    dasharray = "5 5";
  } else if (diff === DiffStatus.Removed) {
    stroke = selected ? "color-mix(in srgb, var(--color-danger) 50%, var(--color-primary) 50%)" : "var(--color-danger)";
    opacity = 0.25;
  } else if (diff === DiffStatus.Modified) {
    stroke = selected ? "color-mix(in srgb, var(--color-warning) 50%, var(--color-primary) 50%)" : "var(--color-warning)";
  } else if (selected) {
    stroke = "var(--color-primary)";
  } else if (isParentConnection) {
    stroke = "var(--color-secondary)";
    strokeWidth = 3;
  }

  return (
    <BaseEdge
      path={path}
      style={{
        stroke,
        strokeWidth,
        strokeDasharray: dasharray,
        opacity,
      }}
      className="transition-colors duration-200"
    />
  );
};
const edgeComponents = { connection: ConnectionEdgeComponent };

const ConnectionConnectionLine: React.FC<ConnectionLineComponentProps> = (props: ConnectionLineComponentProps) => {
  const { fromX, fromY, toX, toY } = props;
  const HANDLE_HEIGHT = 5;
  const path = `M ${fromX} ${fromY + HANDLE_HEIGHT / 2} L ${toX} ${toY + HANDLE_HEIGHT / 2}`;
  return <BaseEdge path={path} style={{ stroke: "gray" }} className="opacity-70" />;
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }: MiniMapNodeProps) => {
  return <circle className={`${selected ? "fill-primary" : "fill-foreground"} transition-colors duration-200`} cx={x} cy={y} r="10" />;
};

const HelperLines: React.FC<{
  lines: HelperLine[];
  nodes: { id: string; position: { x: number; y: number } }[];
}> = ({ lines, nodes }) => {
  const { getViewport } = useReactFlow();

  if (lines.length === 0) return null;

  const viewport = getViewport();

  return (
    <div className="absolute inset-0 w-full h-full pointer-events-none z-[1000] overflow-hidden">
      {lines.map((line, index) => {
        if (line.type === "horizontal" && line.position !== undefined) {
          const screenY = line.position * viewport.zoom + viewport.y;
          return <div key={`h-${line.relatedPieceId}-${index}`} className="absolute left-0 w-full h-px border-t border-dashed border-primary opacity-60" style={{ top: screenY }} />;
        } else if (line.type === "vertical" && line.position !== undefined) {
          const screenX = line.position * viewport.zoom + viewport.x;
          return <div key={`v-${line.relatedPieceId}-${index}`} className="absolute top-0 w-px h-full border-l border-dashed border-primary opacity-60" style={{ left: screenX }} />;
        } else if (line.type === "equalDistance" && line.x1 !== undefined && line.y1 !== undefined && line.x2 !== undefined && line.y2 !== undefined) {
          const screenX1 = line.x1 * viewport.zoom + viewport.x;
          const screenY1 = line.y1 * viewport.zoom + viewport.y;
          const screenX2 = line.x2 * viewport.zoom + viewport.x;
          const screenY2 = line.y2 * viewport.zoom + viewport.y;

          const isMidLine = line.relatedPieceId.startsWith("mid-");
          const strokeColor = "var(--color-primary)";
          const strokeWidth = isMidLine ? "3" : "2";
          const opacity = isMidLine ? 1 : 0.7;
          const dashArray = isMidLine ? "4 4" : "8 4";

          return (
            <svg key={`eq-${line.relatedPieceId}-${index}`} className="absolute inset-0 w-full h-full pointer-events-none">
              <line x1={screenX1} y1={screenY1} x2={screenX2} y2={screenY2} stroke={strokeColor} strokeWidth={strokeWidth} strokeDasharray={dashArray} opacity={opacity} />
            </svg>
          );
        }
        return null;
      })}
    </div>
  );
};

const pieceToNode = (piece: Piece, type: Type, center: Coord, selected: boolean, index: number): PieceNode => ({
  type: "piece",
  id: `piece-${index}-${piece.id_}`,
  position: {
    x: center.x * ICON_WIDTH || 0,
    y: -center.y * ICON_WIDTH || 0,
  },
  selected,
  data: { piece, type },
  className: selected ? "selected" : "",
});

const designToNode = (piece: Piece, externalConnections: Connection[], center: Coord, selected: boolean, index: number): DesignNode => ({
  type: "design",
  id: `piece-${index}-${piece.id_}`,
  position: {
    x: center.x * ICON_WIDTH || 0,
    y: -center.y * ICON_WIDTH || 0,
  },
  selected,
  data: { piece, externalConnections },
  className: selected ? "selected" : "",
});

const extractPieceIdFromNodeId = (nodeId: string): string => {
  return nodeId.split("-").slice(2).join("-");
};

const getPieceIdFromNode = (node: DiagramNode): string => {
  return node.data.piece.id_;
};

const connectionToEdge = (connection: Connection, selected: boolean, isParentConnection: boolean = false, pieceIndexMap: Map<string, number>, connectionIndex: number = 0, designPieces?: Piece[], allConnections?: Connection[]): ConnectionEdge => {
  let sourcePieceId = connection.connecting.piece.id_;
  let targetPieceId = connection.connected.piece.id_;
  let sourcePortId = connection.connecting.port.id_ ?? "undefined";
  let targetPortId = connection.connected.port.id_ ?? "undefined";

  if (connection.connecting.designPiece && allConnections) {
    const designPieceId = connection.connecting.designPiece.id_;
    sourcePieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece?.id_ === connection.connecting.designPiece?.id_;
      const connectingToDesign = conn.connecting.designPiece?.id_ === connection.connecting.designPiece?.id_;
      return connectedToDesign || connectingToDesign;
    });

    const portIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece.id_ === connection.connected.piece.id_ &&
        conn.connecting.piece.id_ === connection.connecting.piece.id_ &&
        conn.connected.port.id_ === connection.connected.port.id_ &&
        conn.connecting.port.id_ === connection.connecting.port.id_,
    );
    sourcePortId = portIndex >= 0 ? `port-${portIndex}` : "port-0";
  }

  if (connection.connected.designPiece && allConnections) {
    const designPieceId = connection.connected.designPiece.id_;
    targetPieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece?.id_ === connection.connected.designPiece?.id_;
      const connectingToDesign = conn.connecting.designPiece?.id_ === connection.connected.designPiece?.id_;
      return connectedToDesign || connectingToDesign;
    });

    const portIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece.id_ === connection.connected.piece.id_ &&
        conn.connecting.piece.id_ === connection.connecting.piece.id_ &&
        conn.connected.port.id_ === connection.connected.port.id_ &&
        conn.connecting.port.id_ === connection.connecting.port.id_,
    );
    targetPortId = portIndex >= 0 ? `port-${portIndex}` : "port-0";
  }

  const sourceIndex = pieceIndexMap.get(sourcePieceId) ?? 0;
  const targetIndex = pieceIndexMap.get(targetPieceId) ?? 0;
  const sourceNodeId = `piece-${sourceIndex}-${sourcePieceId}`;
  const targetNodeId = `piece-${targetIndex}-${targetPieceId}`;

  return {
    type: "connection",
    id: `${sourceNodeId}:${sourcePortId} -- ${targetNodeId}:${targetPortId}:${connectionIndex}`,
    source: sourceNodeId,
    sourceHandle: sourcePortId,
    target: targetNodeId,
    targetHandle: targetPortId,
    data: { connection, isParentConnection },
    selected,
  };
};

const designToNodesAndEdges = (design: Design, flattenedDesign: Design, metadata: Map<string, any>, kit: any, selection: any) => {
  if (!design) return null;

  const centerMap = new Map<string, Coord>();
  flattenedDesign.pieces?.forEach((piece) => {
    if (piece.id_ && piece.center) {
      centerMap.set(piece.id_, piece.center);
    }
  });

  const pieceNodes =
    design.pieces
      ?.map((piece, i) => {
        const isSelected = selection?.pieces?.some((p: any) => p.id_ === piece.id_) ?? false;
        const center = centerMap.get(piece.id_) || piece.center || { x: 0, y: 0 };

        if (!piece.type) {
          console.warn(`No type defined for piece ${piece.id_}`);
          return null;
        }

        const type = findTypeInKit(kit, piece.type);
        if (!type) {
          console.warn(`Type not found for piece ${piece.id_}: ${piece.type.name}/${piece.type.variant || "no-variant"}`);
          const fallbackType: Type = {
            name: piece.type.name,
            variant: piece.type.variant,
            unit: "m",
            description: `Missing type: ${piece.type.name}`,
            ports: [],
            representations: [],
          };
          return pieceToNode(piece, fallbackType, center, isSelected, i);
        }
        return pieceToNode(piece, type, center, isSelected, i);
      })
      .filter((node): node is PieceNode => node !== null) ?? [];

  const includedDesigns = getIncludedDesigns(design);

  const designNodes = includedDesigns.map((includedDesign, i) => {
    const isSelected = selection?.pieces.some((p: any) => p.id_ === includedDesign.id) ?? false;

    if (includedDesign.type === "connected") {
      let calculatedCenter = { x: 0, y: 0 };
      if (includedDesign.externalConnections && includedDesign.externalConnections.length > 0) {
        const connectedPieceIds = new Set<string>();
        includedDesign.externalConnections.forEach((conn) => {
          if (conn.connected.designPiece?.id_ === includedDesign.designId.name) {
            connectedPieceIds.add(conn.connecting.piece.id_);
          } else if (conn.connecting.designPiece?.id_ === includedDesign.designId.name) {
            connectedPieceIds.add(conn.connected.piece.id_);
          }
        });

        const connectedPieceCenters: Coord[] = [];
        Array.from(connectedPieceIds).forEach((pieceId) => {
          const center = centerMap.get(pieceId);
          if (center) {
            connectedPieceCenters.push(center);
          }
        });

        if (connectedPieceCenters.length > 0) {
          const avgX = connectedPieceCenters.reduce((sum, center) => sum + center.x, 0) / connectedPieceCenters.length;
          const avgY = connectedPieceCenters.reduce((sum, center) => sum + center.y, 0) / connectedPieceCenters.length;

          calculatedCenter = {
            x: Math.round(avgX),
            y: Math.round(avgY),
          };
        }
      }

      const designPiece: Piece = {
        id_: includedDesign.id,
        type: { name: "design", variant: includedDesign.designId.name },
        center: calculatedCenter,
        description: `Clustered design: ${includedDesign.designId.name}`,
      };

      return designToNode(designPiece, includedDesign.externalConnections || [], calculatedCenter, isSelected, design.pieces!.length + i);
    } else {
      const displayCenter = includedDesign.center || { x: 0, y: 0 };

      const designPiece: Piece = {
        id_: includedDesign.id,
        type: { name: "design", variant: `${includedDesign.designId.name}${includedDesign.designId.variant ? `-${includedDesign.designId.variant}` : ""}${includedDesign.designId.view ? `-${includedDesign.designId.view}` : ""}` },
        center: displayCenter,
        plane: includedDesign.plane,
        description: `Fixed design: ${includedDesign.designId.name}`,
      };

      return designToNode(designPiece, [], displayCenter, isSelected, design.pieces!.length + i);
    }
  });

  const pieceIndexMap = new Map<string, number>();
  design.pieces?.forEach((piece, index) => {
    if (!pieceIndexMap.has(piece.id_)) {
      pieceIndexMap.set(piece.id_, index);
    }
  });

  includedDesigns.forEach((includedDesign, index) => {
    if (!pieceIndexMap.has(includedDesign.id)) {
      pieceIndexMap.set(includedDesign.id, design.pieces!.length + index);
    }
  });

  const nodeIdToPieceIndexMap = new Map<string, number>();
  design.pieces?.forEach((piece, index) => {
    nodeIdToPieceIndexMap.set(`piece-${index}-${piece.id_}`, index);
  });
  includedDesigns.forEach((includedDesign, index) => {
    const nodeIndex = design.pieces!.length + index;
    nodeIdToPieceIndexMap.set(`piece-${nodeIndex}-${includedDesign.id}`, nodeIndex);
  });

  const parentConnectionId =
    selection?.pieces?.length === 1 && (selection?.connections?.length === 0 || !selection?.connections)
      ? (() => {
          const selectedPieceId = selection.pieces[0].id_;
          const pieceMetadata = metadata.get(selectedPieceId);
          if (pieceMetadata?.parentPieceId) {
            return `${pieceMetadata.parentPieceId} -- ${selectedPieceId}`;
          }
          return null;
        })()
      : null;

  const connectionEdges =
    design.connections?.map((connection, connectionIndex) => {
      const isSelected = selection?.connections.some((c: any) => c.connecting.piece.id_ === connection.connecting.piece.id_ && c.connected.piece.id_ === connection.connected.piece.id_) ?? false;

      const connectionId = `${connection.connecting.piece.id_} -- ${connection.connected.piece.id_}`;
      const isParentConnection = parentConnectionId === connectionId || parentConnectionId === `${connection.connected.piece.id_} -- ${connection.connecting.piece.id_}`;

      return connectionToEdge(connection, isSelected, isParentConnection, pieceIndexMap, connectionIndex, design.pieces, design.connections);
    }) ?? [];
  return { nodes: [...pieceNodes, ...designNodes], edges: connectionEdges };
};

const Diagram: FC = () => {
  const {
    deselectAll,
    selectPiece,
    addPieceToSelection,
    removePieceFromSelection,
    selectConnection,
    addConnectionToSelection,
    removeConnectionFromSelection,
    toggleDiagramFullscreen,
    startTransaction,
    finalizeTransaction,
    abortTransaction,
    execute,
    addConnection,
    addConnections,
    updatePieces,
    updateConnections,
  } = useDesignEditorCommands();

  const { updateDesign } = useKitCommands();

  const selection = useDesignEditorSelection();
  const fullscreenPanel = useDesignEditorFullscreen();
  const others = useDesignEditorOthers();

  // const design = useDiffedDesign();
  const design = useDesign();
  // const types = usePortColoredTypes();
  const kit = useKit();
  // const flattenedDesign = useFlatDesign();
  const flattenedDesign = design;
  // const metadata = usePiecesMetadata();
  const metadata = new Map();

  if (!design) return null;
  const { nodes, edges } = designToNodesAndEdges(design, flattenedDesign, metadata, kit, selection) ?? {
    nodes: [],
    edges: [],
  };
  const reactFlowInstance = useReactFlow();
  const [dragState, setDragState] = useState<{
    lastPostition: XYPosition;
  } | null>(null);
  const [helperLines, setHelperLines] = useState<HelperLine[]>([]);
  const fullscreen = fullscreenPanel === DesignEditorFullscreenPanel.Diagram;

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape" && dragState) {
        abortTransaction();
        setDragState(null);
      }
    };

    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [dragState, abortTransaction]);

  const onNodeClick = (e: React.MouseEvent, node: DiagramNode) => {
    e.stopPropagation();
    const pieceId = getPieceIdFromNode(node);
    if (e.ctrlKey || e.metaKey) removePieceFromSelection({ id_: pieceId });
    else if (e.shiftKey) addPieceToSelection({ id_: pieceId });
    else selectPiece({ id_: pieceId });
  };

  const onEdgeClick = (e: React.MouseEvent, edge: DiagramEdge) => {
    e.stopPropagation();
    if (e.ctrlKey || e.metaKey) removeConnectionFromSelection(edge.data!.connection);
    else if (e.shiftKey) addConnectionToSelection(edge.data!.connection);
    else selectConnection(edge.data!.connection);
  };

  const onPaneClick = (e: React.MouseEvent) => {
    if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) deselectAll();
  };

  const onDoubleClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    toggleDiagramFullscreen();
  };

  const onCluster = useCallback(
    (clusterPieceIds: string[]) => {
      execute?.("cluster", { pieceIds: clusterPieceIds }).catch(() => {});
    },
    [execute],
  );

  const onExpand = useCallback(
    (target: DesignId) => {
      execute?.("explode", { designId: target }).catch(() => {});
    },
    [execute],
  );

  const onNodeDragStart = useCallback(
    (event: any, node: Node) => {
      const currentSelectedIds = selection?.pieces ?? [];
      const pieceId = getPieceIdFromNode(node as DiagramNode);
      const isNodeSelected = currentSelectedIds.some((p: any) => p.id_ === pieceId);
      const ctrlKey = event.ctrlKey || event.metaKey;
      const shiftKey = event.shiftKey;

      if (ctrlKey) isNodeSelected ? removePieceFromSelection({ id_: pieceId }) : addPieceToSelection({ id_: pieceId });
      else if (shiftKey) !isNodeSelected ? addPieceToSelection({ id_: pieceId }) : selectPiece({ id_: pieceId });
      else if (!isNodeSelected) selectPiece({ id_: pieceId });

      startTransaction();
      setDragState({ lastPostition: { x: node.position.x, y: node.position.y } });
      setHelperLines([]);
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection, startTransaction],
  );

  const onNodeDrag = useCallback(
    (event: any, node: DiagramNode) => {
      if (node.type !== "piece") return;
      const piece = node.data.piece as Piece;
      const MIN_DISTANCE = 150;
      const SNAP_THRESHOLD = 20;
      if (!dragState) return;
      const { lastPostition } = dragState;

      const altPressed = event.altKey;

      const currentHelperLines: HelperLine[] = [];
      const nonSelectedNodes = nodes.filter((n) => !(selection?.pieces ?? []).some((p: any) => p.id_ === getPieceIdFromNode(n)));
      const draggedCenterX = node.position.x + ICON_WIDTH / 2;
      const draggedCenterY = node.position.y + ICON_WIDTH / 2;

      const addedConnections: Connection[] = [];
      // Note: Piece updates during drag are temporarily disabled for new store architecture
      // let updatedPieces: PieceDiff[] = [];
      // let updatedConnections: ConnectionDiff[] = [];

      for (const selectedNode of nodes.filter((n) => selection?.pieces?.some((p: any) => p.id_ === getPieceIdFromNode(n)))) {
        if (selectedNode.type !== "piece") continue;
        const piece = (selectedNode as PieceNode).data.piece;
        const type = (selectedNode as PieceNode).data.type;
        const fixedPieceId = metadata.get(piece.id_)!.fixedPieceId;
        let closestConnection: Connection | null = null;
        let closestDistance = Number.MAX_VALUE;
        const selectedInternalNode = reactFlowInstance.getInternalNode(selectedNode.id)!;

        let draggedX = node.position.x;
        let draggedY = node.position.y;

        if (!altPressed) {
          const EQUAL_DISTANCE_THRESHOLD = 15;
          let equalDistanceHelperLines: HelperLine[] = [];
          const displayedDistances = new Set<number>();

          for (let i = 0; i < nonSelectedNodes.length; i++) {
            for (let j = i + 1; j < nonSelectedNodes.length; j++) {
              const node1 = nonSelectedNodes[i];
              const node2 = nonSelectedNodes[j];

              const center1 = {
                x: node1.position.x + ICON_WIDTH / 2,
                y: node1.position.y + ICON_WIDTH / 2,
              };
              const center2 = {
                x: node2.position.x + ICON_WIDTH / 2,
                y: node2.position.y + ICON_WIDTH / 2,
              };

              if (Math.abs(center1.x - center2.x) < 5) {
                const distance = Math.abs(center2.y - center1.y);
                const minY = Math.min(center1.y, center2.y);
                const maxY = Math.max(center1.y, center2.y);
                const midY = (center1.y + center2.y) / 2;

                const isDistanceAlreadyDisplayed = Array.from(displayedDistances).some((existingDistance) => Math.abs(existingDistance - distance) < TOLERANCE);

                if (distance > 40 && !isDistanceAlreadyDisplayed) {
                  displayedDistances.add(distance);

                  if (Math.abs(draggedCenterY - midY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = midY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `upper-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: minY,
                        x2: center1.x + 50,
                        y2: minY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `lower-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: maxY,
                        x2: center1.x + 50,
                        y2: maxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `mid-${node1.id}-${node2.id}`,
                        x1: center1.x - 30,
                        y1: midY,
                        x2: center1.x + 30,
                        y2: midY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedMinY = minY - distance;
                  const extendedMaxY = maxY + distance;

                  if (Math.abs(draggedCenterY - extendedMinY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedMinY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `extend-before-${node1.id}-${node2.id}`,
                        x1: center1.x - 30,
                        y1: extendedMinY,
                        x2: center1.x + 30,
                        y2: extendedMinY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: minY,
                        x2: center1.x + 50,
                        y2: minY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: maxY,
                        x2: center1.x + 50,
                        y2: maxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterY - extendedMaxY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedMaxY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `extend-after-${node1.id}-${node2.id}`,
                        x1: center1.x - 30,
                        y1: extendedMaxY,
                        x2: center1.x + 30,
                        y2: extendedMaxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: minY,
                        x2: center1.x + 50,
                        y2: minY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: center1.x - 50,
                        y1: maxY,
                        x2: center1.x + 50,
                        y2: maxY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedLeftX = center1.x - distance;
                  const extendedRightX = center1.x + distance;

                  if (Math.abs(draggedCenterX - extendedLeftX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedLeftX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-left-${node1.id}-${node2.id}`,
                        x1: extendedLeftX,
                        y1: midY - 30,
                        x2: extendedLeftX,
                        y2: midY + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: center1.x,
                        y1: midY - 50,
                        x2: center1.x,
                        y2: midY + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterX - extendedRightX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedRightX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-right-${node1.id}-${node2.id}`,
                        x1: extendedRightX,
                        y1: midY - 30,
                        x2: extendedRightX,
                        y2: midY + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: center1.x,
                        y1: midY - 50,
                        x2: center1.x,
                        y2: midY + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }
                }
              }

              if (Math.abs(center1.y - center2.y) < 5) {
                const distance = Math.abs(center2.x - center1.x);
                const minX = Math.min(center1.x, center2.x);
                const maxX = Math.max(center1.x, center2.x);
                const midX = (center1.x + center2.x) / 2;

                const isDistanceAlreadyDisplayed = Array.from(displayedDistances).some((existingDistance) => Math.abs(existingDistance - distance) < TOLERANCE);

                if (distance > 40 && !isDistanceAlreadyDisplayed) {
                  displayedDistances.add(distance);

                  if (Math.abs(draggedCenterX - midX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = midX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `left-${node1.id}-${node2.id}`,
                        x1: minX,
                        y1: center1.y - 50,
                        x2: minX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `right-${node1.id}-${node2.id}`,
                        x1: maxX,
                        y1: center1.y - 50,
                        x2: maxX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `mid-${node1.id}-${node2.id}`,
                        x1: midX,
                        y1: center1.y - 30,
                        x2: midX,
                        y2: center1.y + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedMinX = minX - distance;
                  const extendedMaxX = maxX + distance;

                  if (Math.abs(draggedCenterX - extendedMinX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedMinX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `extend-before-${node1.id}-${node2.id}`,
                        x1: extendedMinX,
                        y1: center1.y - 30,
                        x2: extendedMinX,
                        y2: center1.y + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: minX,
                        y1: center1.y - 50,
                        x2: minX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: maxX,
                        y1: center1.y - 50,
                        x2: maxX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterX - extendedMaxX) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedX = extendedMaxX - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `extend-after-${node1.id}-${node2.id}`,
                        x1: extendedMaxX,
                        y1: center1.y - 30,
                        x2: extendedMaxX,
                        y2: center1.y + 30,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref1-${node1.id}-${node2.id}`,
                        x1: minX,
                        y1: center1.y - 50,
                        x2: minX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `ref2-${node1.id}-${node2.id}`,
                        x1: maxX,
                        y1: center1.y - 50,
                        x2: maxX,
                        y2: center1.y + 50,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  const extendedUpY = center1.y - distance;
                  const extendedDownY = center1.y + distance;

                  if (Math.abs(draggedCenterY - extendedUpY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedUpY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-up-${node1.id}-${node2.id}`,
                        x1: midX - 30,
                        y1: extendedUpY,
                        x2: midX + 30,
                        y2: extendedUpY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: midX - 50,
                        y1: center1.y,
                        x2: midX + 50,
                        y2: center1.y,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }

                  if (Math.abs(draggedCenterY - extendedDownY) < EQUAL_DISTANCE_THRESHOLD) {
                    draggedY = extendedDownY - ICON_WIDTH / 2;

                    equalDistanceHelperLines.push(
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-down-${node1.id}-${node2.id}`,
                        x1: midX - 30,
                        y1: extendedDownY,
                        x2: midX + 30,
                        y2: extendedDownY,
                        referencePieceIds: [node1.id, node2.id],
                      },
                      {
                        type: "equalDistance",
                        relatedPieceId: `perp-ref-${node1.id}-${node2.id}`,
                        x1: midX - 50,
                        y1: center1.y,
                        x2: midX + 50,
                        y2: center1.y,
                        referencePieceIds: [node1.id, node2.id],
                      },
                    );
                  }
                }
              }
            }
          }

          const updatedDraggedCenterX = draggedX + ICON_WIDTH / 2;
          const updatedDraggedCenterY = draggedY + ICON_WIDTH / 2;

          for (const otherNode of nonSelectedNodes) {
            const centerY = otherNode.position.y + ICON_WIDTH / 2;
            const distance = Math.abs(updatedDraggedCenterY - centerY);
            if (distance < SNAP_THRESHOLD) {
              draggedY = centerY - ICON_WIDTH / 2;
              currentHelperLines.push({
                type: "horizontal",
                position: centerY,
                relatedPieceId: otherNode.id,
              });
              break;
            }
          }

          for (const otherNode of nonSelectedNodes) {
            const centerX = otherNode.position.x + ICON_WIDTH / 2;
            const distance = Math.abs(updatedDraggedCenterX - centerX);
            if (distance < SNAP_THRESHOLD) {
              draggedX = centerX - ICON_WIDTH / 2;
              currentHelperLines.push({
                type: "vertical",
                position: centerX,
                relatedPieceId: otherNode.id,
              });
              break;
            }
          }

          currentHelperLines.push(...equalDistanceHelperLines);

          setHelperLines(currentHelperLines);
        } else {
          setHelperLines([]);
        }

        if (selectedNode.id === node.id) {
          selectedInternalNode.internals.positionAbsolute.x = draggedX;
          selectedInternalNode.internals.positionAbsolute.y = draggedY;
          node.position.x = draggedX;
          node.position.y = draggedY;
        }

        if (!altPressed) {
          for (const otherNode of nodes.filter((n) => !(selection.pieces ?? []).some((p: any) => p.id_ === getPieceIdFromNode(n)))) {
            if (otherNode.type !== "piece") continue;
            const existingConnection = design.connections?.find((c) =>
              isSameConnection(c, {
                connected: { piece: { id_: selectedNode.data.piece.id_ } },
                connecting: { piece: { id_: otherNode.data.piece.id_ } },
              }),
            );
            if (existingConnection) continue;
            const otherInternalNode = reactFlowInstance.getInternalNode(otherNode.id)!;
            for (const handle of selectedInternalNode.internals.handleBounds?.source ?? []) {
              const port = findPortInType(type, { id_: handle.id! });
              for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
                const otherPort = findPortInType((otherNode as PieceNode).data.type, {
                  id_: otherHandle.id!,
                });
                const haveSameFixedPiece = fixedPieceId === metadata.get(otherNode.data.piece.id_)!.fixedPieceId;
                if (haveSameFixedPiece || !arePortsCompatible(port, otherPort) || isPortInUse(design, piece, port) || isPortInUse(design, otherNode.data.piece, otherPort)) continue;
                const dx = selectedInternalNode.internals.positionAbsolute.x + handle.x - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x);
                const dy = selectedInternalNode.internals.positionAbsolute.y + handle.y - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y);
                const distance = Math.sqrt(dx * dx + dy * dy);
                if (distance < closestDistance && distance < MIN_DISTANCE) {
                  closestConnection = {
                    connected: {
                      piece: { id_: otherNode.data.piece.id_ },
                      port: { id_: otherHandle.id! },
                    },
                    connecting: {
                      piece: { id_: selectedNode.data.piece.id_ },
                      port: { id_: handle.id! },
                    },
                    x: (selectedInternalNode.internals.positionAbsolute.x + handle.x - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x)) / ICON_WIDTH,
                    y: -((selectedInternalNode.internals.positionAbsolute.y + handle.y - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y)) / ICON_WIDTH),
                  };
                  closestDistance = distance;
                }
              }
            }
          }
        }

        if (closestConnection) {
          addedConnections.push(closestConnection);
          // Note: Piece updates during drag are temporarily disabled for new store architecture
          // const updatedPiece = {
          //   ...selectedNode.data.piece,
          //   center: undefined,
          //   plane: undefined,
          // };
          // if (updatedPiece.type) {
          //   updatedPieces.push(updatedPiece as Piece);
          // }
        } else {
          // Note: Piece position updates during drag are temporarily disabled for new store architecture
          // if (!piece.center) {
          //   const parentPieceId = metadata.get(selectedNode.data.piece.id_)!.parentPieceId!;
          //   const parentNode = nodes.find((n) => n.data.piece.id_ === parentPieceId);
          //   if (!parentNode) throw new Error(`Parent node not found for piece ${parentPieceId}`);
          //   const parentInternalNode = reactFlowInstance.getInternalNode(parentNode.id);
          //   if (!parentInternalNode) throw new Error(`Internal node not found for ${parentNode.id}`);
          //   const parentConnection = findConnectionInDesign(design, {
          //     connected: { piece: { id_: selectedNode.data.piece.id_ } },
          //     connecting: { piece: { id_: parentPieceId } },
          //   });
          //   updatedConnections.push({
          //     ...parentConnection,
          //     x: (parentConnection.x ?? 0) + (draggedX - lastPostition.x) / ICON_WIDTH,
          //     y: (parentConnection.y ?? 0) - (draggedY - lastPostition.y) / ICON_WIDTH,
          //   });
          // } else {
          //   const scaledOffset = {
          //     x: (draggedX - lastPostition.x) / ICON_WIDTH,
          //     y: -(draggedY - lastPostition.y) / ICON_WIDTH,
          //   };
          //   const updatedPiece = {
          //     ...piece,
          //     center: {
          //       x: piece.center!.x + scaledOffset.x,
          //       y: piece.center!.y + scaledOffset.y,
          //     },
          //   };
          //   if (updatedPiece.type) {
          //     updatedPieces.push(updatedPiece as Piece);
          //   }
          // }
        }
        if (addedConnections.length > 0) {
          addedConnections.forEach((conn) => addConnection(conn));
        }
        setDragState({
          ...dragState!,
          lastPostition: { x: draggedX, y: draggedY },
        });
      }
    },
    [addConnection, design, reactFlowInstance, selection, nodes, metadata],
  );

  const onNodeDragStop = useCallback(() => {
    finalizeTransaction();
    setDragState(null);
    setHelperLines([]);
  }, [finalizeTransaction]);

  const onConnect = useCallback(
    (params: RFConnection) => {
      if (params.source === params.target) return;

      const sourceInternalNode = reactFlowInstance.getInternalNode(params.source);
      const targetInternalNode = reactFlowInstance.getInternalNode(params.target);
      if (!sourceInternalNode || !targetInternalNode) return;

      const sourceHandle = (sourceInternalNode.internals.handleBounds?.source ?? []).find((h) => h.id === params.sourceHandle);
      const targetHandle = (targetInternalNode.internals.handleBounds?.source ?? []).find((h) => h.id === params.targetHandle);
      if (!sourceHandle || !targetHandle) return;

      const sourcePieceId = extractPieceIdFromNodeId(params.source!);
      const targetPieceId = extractPieceIdFromNodeId(params.target!);

      const newConnection = {
        connected: {
          piece: { id_: sourcePieceId },
          port: { id_: params.sourceHandle! },
        },
        connecting: {
          piece: { id_: targetPieceId },
          port: { id_: params.targetHandle! },
        },
        x: (sourceInternalNode.internals.positionAbsolute.x + sourceHandle.x - (targetInternalNode.internals.positionAbsolute.x + targetHandle.x)) / ICON_WIDTH,
        y: -((sourceInternalNode.internals.positionAbsolute.y + sourceHandle.y - (targetInternalNode.internals.positionAbsolute.y + targetHandle.y)) / ICON_WIDTH),
      };

      if ((design.connections ?? []).find((c) => isSameConnection(c, newConnection))) return;
      addConnection(newConnection);
    },
    [addConnection, reactFlowInstance, design],
  );

  return (
    <div id="diagram" className="h-full w-full relative">
      <ReactFlow
        ref={useDroppable({ id: "diagram-drop-zone" }).setNodeRef}
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeComponents}
        edgeTypes={edgeComponents}
        connectionMode={ConnectionMode.Loose}
        elementsSelectable={false}
        minZoom={0.1}
        maxZoom={12}
        fitView
        zoomOnDoubleClick={false}
        panOnDrag={[0]}
        proOptions={{ hideAttribution: true }}
        onNodeClick={onNodeClick}
        onEdgeClick={onEdgeClick}
        onNodeDragStart={onNodeDragStart}
        onNodeDrag={onNodeDrag}
        onNodeDragStop={onNodeDragStop}
        onPaneClick={onPaneClick}
        onDoubleClick={onDoubleClick}
        onConnect={onConnect}
        connectionLineComponent={ConnectionConnectionLine}
        className="bg-background"
      >
        {fullscreen && <Controls className="border border-border bg-background rounded-md shadow-sm" showZoom={false} showInteractive={false} />}
        {fullscreen && <MiniMap className="border border-border bg-background rounded-md shadow-sm" maskColor="var(--accent)" bgColor="var(--background)" nodeComponent={MiniMapNode} />}
        <ViewportPortal>⌞</ViewportPortal>
        {others.map((presence, idx) => (
          <PresenceDiagram key={`presence-${idx}-${presence.name}-${presence.cursor?.x || 0}-${presence.cursor?.y || 0}`} {...presence} />
        ))}
      </ReactFlow>
      <HelperLines lines={helperLines} nodes={nodes} />
      {/* <ClusterMenu nodes={nodes} edges={edges} onCluster={onCluster} /> */}
      {/* <ExpandMenu nodes={nodes} edges={edges} onExpand={onExpand} /> */}
    </div>
  );
};

export default Diagram;
