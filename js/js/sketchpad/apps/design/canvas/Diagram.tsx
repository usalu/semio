import { useDroppable } from "@dnd-kit/core";
import type { XYPosition } from "@xyflow/react";
import React, { FC, useCallback, useEffect, useMemo, useRef, useState } from "react";
import BaseDiagram, {
  BaseEdge,
  type ConnectionLineComponentProps,
  type Edge,
  type EdgeProps,
  type EdgeTypes,
  Handle,
  type MiniMapNodeProps,
  type Node,
  type NodeProps,
  type NodeTypes,
  Position,
  type ReactFlowInstance,
  type ReactFlowConnection as RFConnection,
  useReactFlow,
  ViewportPortal,
} from "../../../../elements/windows/Diagram";

// Global state for hover management - shared across all piece nodes
let globalHoverClearTimeout: NodeJS.Timeout | null = null;
let currentHoveredPieceGuid: string | null = null;

import {
  arePortsCompatible,
  areSameConnection,
  Coord,
  Design,
  DiffStatus,
  findAttributeValue,
  findPortInType,
  findTypeInKit,
  getIncludedDesigns,
  Guid,
  ICON_WIDTH,
  isPortInUse,
  Kit,
  Piece,
  Port,
  Connection as SemioConnection,
  TOLERANCE,
  Type,
} from "../../../../semio";

import { Avatar, AvatarFallback } from "../../../../elements/display/Avatar";
import { Button } from "../../../../elements/input/Button";
import { ConnectionScopeProvider, PieceScopeProvider, useDesign, useExplodeableDesignNodes, useKit, useKitCommands } from "../../../kits/store";
import { useClusterableGroups, useDiffedPiece, useIsConnectionHovered, useIsPieceHovered } from "../../../kits/designAppIntegration";
import { useFocusSafe } from "../../../Navbar";
import { ToolType, useAppPanelVisibility, useSketchpadCommands } from "../../../store";
import {
  DesignAppFullscreenWindow,
  DesignAppPresenceOther,
  DesignAppSelection,
  useDesignApp,
  useDesignAppCommands,
  useDesignAppDiagramCenter,
  useDesignAppDiagramScale,
  useDesignAppFullscreen,
  useDesignAppHover,
  useDesignAppOthers,
  useDesignAppPieceColor,
  useDesignAppSelection,
} from "../store";

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
      const groupNodes = nodes.filter((node) => groupPieceIds.includes(node.data.piece.guid));

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
            <div className="absolute inset-0 border-2 border-dashed border-accent/50 rounded-md" style={{ pointerEvents: "none" }} />
            <div className="absolute -top-10 -right-2 pointer-events-auto">
              <Button level="temporary" className="px-3 py-1 text-sm h-auto" onClick={() => onCluster(groupPieceIds)}>
                Cluster
              </Button>
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
  onExpand: (designId: string) => void;
};

const ExpandMenu: FC<ExpandMenuProps> = ({ nodes, edges, onExpand }) => {
  const selection = useDesignAppSelection();
  const kit = useKit() as Kit;
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
        const piece = node.data.piece as Piece;
        const type = piece.type ? findTypeInKit(kit, piece.type) : null;
        const designName = type?.variant ?? type?.name ?? "";

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
            <div className="absolute inset-0 border-2 border-dashed border-accent/50 rounded-md" style={{ pointerEvents: "none" }} />
            <div className="absolute -top-10 -right-2 pointer-events-auto">
              <Button level="temporary" className="px-3 py-1 text-sm h-auto" onClick={() => onExpand(designName)}>
                Expand
              </Button>
            </div>
          </div>
        );
      })}
    </ViewportPortal>
  );
};

const PresenceDiagram: FC<DesignAppPresenceOther> = ({ name, cursor, camera }) => {
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
        <div className="flex items-center gap-1 bg-accent text-accent-foreground px-2 py-1 rounded-full text-xs">
          <div className="w-2 h-2 bg-accent-foreground rounded-full"></div>
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
  externalConnections: SemioConnection[];
};

type PieceNode = Node<PieceNodeProps, "piece">;
type DesignNode = Node<DesignNodeProps, "design">;
type DiagramNode = PieceNode | DesignNode;

type ConnectionEdge = Edge<{ SemioConnection: SemioConnection; isParentConnection?: boolean }, "SemioConnection">;
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
  const portColor = findAttributeValue(port, "semio.color", "var(--foreground)")!;
  const hover = useDesignAppHover();
  const { hoverPort } = useDesignAppCommands();
  const isHovered = hover?.ports?.some((p) => p.piece === pieceId && p.port === port.guid) ?? false;

  const onClick = (event: React.MouseEvent) => {
    event.stopPropagation();
    onPortClick(port);
  };

  return (
    <Handle
      id={port.guid ?? ""}
      type="source"
      className="left-1/2 top-0 cursor-selectable"
      style={{
        left: x + ICON_WIDTH / 2,
        top: y,
        backgroundColor: selected ? "var(--active-base)" : isHovered ? "var(--hover-base)" : portColor,
        border: selected || isHovered ? "2px solid var(--border-color)" : "0",
        zIndex: selected || isHovered ? 20 : 10,
      }}
      position={Position.Top}
      role="button"
      onClick={onClick}
      onPointerEnter={() => {
        if (port.guid) hoverPort(pieceId, port.guid);
      }}
      onPointerLeave={() => {
        // Do nothing - let parent handle hover clear
      }}
    />
  );
};

const PieceNodeComponent: React.FC<NodeProps<PieceNode>> = React.memo(({ id, data }) => {
  const {
    piece,
    piece: { guid, attributes },
    type,
  } = data as PieceNodeProps & { diffStatus: DiffStatus };
  const ports = type.ports;
  const { selectPiecePort, deselectPiecePort, addConnection, hoverPiece, clearHover } = useDesignAppCommands();
  const selection = useDesignAppSelection();
  const isSelected = selection?.pieces?.includes(guid) ?? false;
  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isDesignPiece = !!piece.design;

  const handleMouseEnter = useCallback(() => {
    // Clear any global pending clear hover
    if (globalHoverClearTimeout) {
      clearTimeout(globalHoverClearTimeout);
      globalHoverClearTimeout = null;
    }

    // Only set hover if this is a different piece
    if (currentHoveredPieceGuid !== guid) {
      currentHoveredPieceGuid = guid;
      hoverPiece(guid);
    }
  }, [guid, hoverPiece]);

  const handleMouseLeave = useCallback(() => {
    // Clear any existing global timeout
    if (globalHoverClearTimeout) {
      clearTimeout(globalHoverClearTimeout);
    }

    // Set a global timeout for clearing hover
    // Only clear if this piece is still the currently hovered one
    const pieceGuidAtLeave = guid;
    globalHoverClearTimeout = setTimeout(() => {
      if (currentHoveredPieceGuid === pieceGuidAtLeave) {
        clearHover();
        currentHoveredPieceGuid = null;
      }
      globalHoverClearTimeout = null;
    }, 50);
  }, [guid, clearHover]);

  return (
    <PieceScopeProvider guid={guid}>
      <PieceNodeInner
        id={id}
        piece={piece}
        type={type}
        ports={ports}
        isSelected={isSelected}
        diff={diff}
        isDesignPiece={isDesignPiece}
        selection={selection}
        selectPiecePort={selectPiecePort}
        deselectPiecePort={deselectPiecePort}
        addConnection={addConnection}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
      />
    </PieceScopeProvider>
  );
});

type PieceNodeInnerProps = {
  id: string;
  piece: Piece;
  type: Type;
  ports: Port[] | undefined;
  isSelected: boolean;
  diff: DiffStatus;
  isDesignPiece: boolean;
  selection: DesignAppSelection | undefined;
  selectPiecePort: (piece: Guid, port: Guid) => void;
  deselectPiecePort: () => void;
  addConnection: (SemioConnection: any) => void;
  onMouseEnter: () => void;
  onMouseLeave: () => void;
};

const PieceNodeInner: React.FC<PieceNodeInnerProps> = ({ id, piece, type, ports, isSelected, diff, isDesignPiece, selection, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const { fill, stroke, opacity: colorOpacity } = useDesignAppPieceColor(undefined, piece.guid);
  const isHovered = useIsPieceHovered();

  // Always call the hook to maintain hook order (Rules of Hooks)
  const diffedPiece = useDiffedPiece() as Piece;

  // Check if piece has a center diff - only show ghost if there's an actual position change
  const hasCenterDiff = diff === DiffStatus.Modified && piece.center && diffedPiece.center && (piece.center.x !== diffedPiece.center.x || piece.center.y !== diffedPiece.center.y);

  const typeName = type.name || "";
  const typeVariant = type.variant || "";
  const displayVariant = typeVariant || typeName || piece.guid || "??";
  const initials = displayVariant.substring(0, 2).toUpperCase();
  const backgroundColor = fill === "transparent" ? undefined : fill;
  const showHoverBackground = fill === "var(--hover-base)";
  const textColor = isSelected ? "var(--active-foreground)" : backgroundColor && !showHoverBackground ? "var(--background)" : "var(--foreground)";
  const avatarTitle = typeVariant && typeVariant !== typeName ? (typeName ? `${typeName} (${typeVariant})` : typeVariant) : typeName || typeVariant || piece.guid;
  const ringClass = isSelected ? "ring-1 ring-inset ring-[color:var(--active-base)]" : isHovered ? "ring-1 ring-inset ring-[color:var(--hover-base)]" : "";
  const fallbackStyle = backgroundColor ? { backgroundColor, color: textColor } : { color: textColor };

  const onPortClick = (port: Port) => {
    const currentSelectedPort = selection?.port;

    if (currentSelectedPort && (currentSelectedPort.piece !== piece.guid || currentSelectedPort.port !== port.guid)) {
      const SemioConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connecting: {
          guid: crypto.randomUUID(),
          piece: currentSelectedPort.piece,
          port: currentSelectedPort.port,
        },
        connected: { guid: crypto.randomUUID(), piece: piece.guid, port: port.guid },
      };
      addConnection(SemioConnection);
      deselectPiecePort();
    } else if (currentSelectedPort && currentSelectedPort.piece === piece.guid && currentSelectedPort.port === port.guid) {
      deselectPiecePort();
    } else if (port.guid) selectPiecePort(piece.guid, port.guid);
  };

  // Calculate original position in pixels for the ghost node
  const originalPixelPos = hasCenterDiff
    ? {
        x: (piece.center?.x ?? 0) * ICON_WIDTH,
        y: -(piece.center?.y ?? 0) * ICON_WIDTH,
      }
    : null;

  return (
    <div
      className="cursor-selectable"
      style={{
        opacity: colorOpacity,
        width: ICON_WIDTH,
        height: ICON_WIDTH,
        position: "relative",
        pointerEvents: "all",
      }}
      onPointerEnter={onMouseEnter}
      onPointerLeave={onMouseLeave}
    >
      {/* Original node (muted border only) - rendered at absolute position */}
      {hasCenterDiff && originalPixelPos && (
        <div
          style={{
            position: "absolute",
            left: originalPixelPos.x - (diffedPiece.center?.x ?? 0) * ICON_WIDTH,
            top: originalPixelPos.y - -(diffedPiece.center?.y ?? 0) * ICON_WIDTH,
            pointerEvents: "none",
            width: ICON_WIDTH,
            height: ICON_WIDTH,
          }}
        >
          <svg width={ICON_WIDTH} height={ICON_WIDTH}>
            <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 1} className="stroke-[var(--muted-foreground)] stroke-2 fill-transparent" strokeDasharray="4 4" />
            {isDesignPiece && <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 6} className="stroke-[var(--muted-foreground)] stroke-2 fill-transparent" strokeDasharray="4 4" />}
          </svg>
        </div>
      )}

      {/* Current/diffed node */}
      <Avatar role="button" title={avatarTitle} className={`w-full h-full border-[color:var(--border-color)] ${ringClass}`} style={{ borderColor: stroke }}>
        <AvatarFallback className="select-none text-xs font-bold" style={fallbackStyle}>
          {initials}
        </AvatarFallback>
      </Avatar>
      {ports?.map((port: Port, portIndex: number) => (
        <PortHandle key={`${id}-port-${portIndex}-${port.guid}`} port={port} pieceId={piece.guid} selected={selection?.port?.piece === piece.guid && selection?.port?.port === port.guid} onPortClick={onPortClick} />
      ))}
    </div>
  );
};

const DesignNodeComponent: React.FC<NodeProps<DesignNode>> = React.memo(({ id, data }) => {
  const {
    piece,
    piece: { guid, attributes },
    externalConnections,
  } = data as DesignNodeProps & { diffStatus: DiffStatus };
  const { selectPiecePort, deselectPiecePort, addConnection, hoverPiece, clearHover } = useDesignAppCommands();
  const selection = useDesignAppSelection();
  const isSelected = selection?.pieces?.includes(guid) ?? false;
  const diff = (attributes?.find((q) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;

  const handleMouseEnter = useCallback(() => {
    // Clear any global pending clear hover
    if (globalHoverClearTimeout) {
      clearTimeout(globalHoverClearTimeout);
      globalHoverClearTimeout = null;
    }

    // Only set hover if this is a different piece
    if (currentHoveredPieceGuid !== guid) {
      currentHoveredPieceGuid = guid;
      hoverPiece(guid);
    }
  }, [guid, hoverPiece]);

  const handleMouseLeave = useCallback(() => {
    // Clear any existing global timeout
    if (globalHoverClearTimeout) {
      clearTimeout(globalHoverClearTimeout);
    }

    // Set a global timeout for clearing hover
    // Only clear if this piece is still the currently hovered one
    const pieceGuidAtLeave = guid;
    globalHoverClearTimeout = setTimeout(() => {
      if (currentHoveredPieceGuid === pieceGuidAtLeave) {
        clearHover();
        currentHoveredPieceGuid = null;
      }
      globalHoverClearTimeout = null;
    }, 50);
  }, [guid, clearHover]);

  const ports: Port[] = externalConnections.map((SemioConnection, portIndex) => {
    const connectedIsDesignPiece = SemioConnection.connected.piece === piece.guid || SemioConnection.connected.designPiece === piece.guid;
    const connectingIsDesignPiece = SemioConnection.connecting.piece === piece.guid || SemioConnection.connecting.designPiece === piece.guid;

    const designSide = connectedIsDesignPiece ? SemioConnection.connected : SemioConnection.connecting;
    const originalSide = connectedIsDesignPiece ? SemioConnection.connecting : SemioConnection.connected;

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
      guid: `port-${portIndex}`,
      description: `Port for SemioConnection to ${originalSide.piece}:${originalSide.port}`,
      family: "default",
      mandatory: false,
      t: t,
      point: { x: portX, y: portY, z: portZ },
      direction: { x: directionX, y: directionY, z: directionZ },
      attributes: [
        {
          guid: crypto.randomUUID(),
          key: "semio.originalPieceId",
          value: designSide.piece || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.originalPortId",
          value: designSide.port || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.externalPieceId",
          value: originalSide.piece || "",
        },
        {
          guid: crypto.randomUUID(),
          key: "semio.externalPortId",
          value: originalSide.port || "",
        },
      ],
    };
  });

  return (
    <PieceScopeProvider guid={guid}>
      <DesignNodeInner
        id={id}
        piece={piece}
        ports={ports}
        isSelected={isSelected}
        diff={diff}
        selection={selection}
        selectPiecePort={selectPiecePort}
        deselectPiecePort={deselectPiecePort}
        addConnection={addConnection}
        onMouseEnter={handleMouseEnter}
        onMouseLeave={handleMouseLeave}
      />
    </PieceScopeProvider>
  );
});

type DesignNodeInnerProps = {
  id: string;
  piece: Piece;
  ports: Port[] | undefined;
  isSelected: boolean;
  diff: DiffStatus;
  selection: DesignAppSelection | undefined;
  selectPiecePort: (piece: Guid, port: Guid) => void;
  deselectPiecePort: () => void;
  addConnection: (SemioConnection: any) => void;
  onMouseEnter: () => void;
  onMouseLeave: () => void;
};

const DesignNodeInner: React.FC<DesignNodeInnerProps> = ({ id, piece, ports, isSelected, diff, selection, selectPiecePort, deselectPiecePort, addConnection, onMouseEnter, onMouseLeave }) => {
  const isHovered = useIsPieceHovered();

  const onPortClick = (port: Port) => {
    const currentSelectedPort = selection?.port;

    if (currentSelectedPort && (currentSelectedPort.piece !== piece.guid || currentSelectedPort.port !== port.guid)) {
      const SemioConnection: SemioConnection = {
        guid: crypto.randomUUID(),
        connecting: {
          guid: crypto.randomUUID(),
          piece: currentSelectedPort.piece,
          port: currentSelectedPort.port,
        },
        connected: { guid: crypto.randomUUID(), piece: piece.guid, port: port.guid },
      };
      addConnection(SemioConnection);
      deselectPiecePort();
    } else if (currentSelectedPort && currentSelectedPort.piece === piece.guid && currentSelectedPort.port === port.guid) {
      deselectPiecePort();
    } else if (port.guid) selectPiecePort(piece.guid, port.guid);
  };

  let fillClass = "fill-transparent";
  let strokeClass = "stroke-[var(--foreground)] stroke-2";
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    fillClass = "fill-[var(--color-success)]";
    strokeClass = "stroke-[var(--color-success)] stroke-2";
  } else if (diff === DiffStatus.Removed) {
    fillClass = "fill-[var(--color-danger)]";
    strokeClass = "stroke-[var(--color-danger)] stroke-2";
    opacity = 0.2;
  } else if (diff === DiffStatus.Modified) {
    fillClass = "fill-[var(--color-warning)]";
    strokeClass = "stroke-[var(--color-warning)] stroke-2";
  }
  if (isHovered && !isSelected) {
    fillClass = "fill-[var(--hover-base)]";
    strokeClass = "stroke-[var(--foreground)] stroke-2";
    opacity = 1;
  }
  if (isSelected) {
    fillClass = "fill-[var(--active-base)]";
    strokeClass = "stroke-[var(--foreground)] stroke-2";
    opacity = 1;
  }

  return (
    <div
      className="cursor-selectable"
      style={{
        opacity,
        width: ICON_WIDTH,
        height: ICON_WIDTH,
        position: "relative",
        pointerEvents: "all",
      }}
      onPointerEnter={onMouseEnter}
      onPointerLeave={onMouseLeave}
    >
      <svg width={ICON_WIDTH} height={ICON_WIDTH} role="button" style={{ pointerEvents: "all" }}>
        <circle cx={ICON_WIDTH / 2} cy={ICON_WIDTH / 2} r={ICON_WIDTH / 2 - 1} className={`${strokeClass} ${fillClass}`} />
        <text x={ICON_WIDTH / 2} y={ICON_WIDTH / 2} textAnchor="middle" dominantBaseline="middle" className={`text-xs font-bold ${isSelected ? "fill-[var(--active-foreground)]" : "fill-foreground"}`} style={{ pointerEvents: "none" }}>
          {piece.guid}
        </text>
      </svg>
      {ports?.map((port: Port, portIndex: number) => (
        <PortHandle key={`${id}-port-${portIndex}-${port.guid}`} port={port} pieceId={piece.guid} selected={selection?.port?.piece === piece.guid && selection?.port?.port === port.guid} onPortClick={onPortClick} />
      ))}
    </div>
  );
};
const nodeComponents = { piece: PieceNodeComponent, design: DesignNodeComponent };

const ConnectionEdgeComponent: React.FC<EdgeProps<ConnectionEdge>> = (props) => {
  const connectionGuid = props.data?.SemioConnection?.guid;
  if (!connectionGuid) {
    return <ConnectionEdgeFallback {...props} />;
  }
  return (
    <ConnectionScopeProvider guid={connectionGuid}>
      <ConnectionEdgeInner {...props} connectionGuid={connectionGuid} />
    </ConnectionScopeProvider>
  );
};

const ConnectionEdgeFallback: React.FC<EdgeProps<ConnectionEdge>> = ({ sourceX, sourceY, targetX, targetY, data, selected }) => {
  const HANDLE_HEIGHT = 5;
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`;
  const diff = (data?.SemioConnection?.attributes?.find((q: any) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isParentConnection = data?.isParentConnection ?? false;

  let stroke = "var(--foreground)";
  let strokeWidth = 2;
  let dasharray: string | undefined;
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    stroke = "var(--color-success)";
    dasharray = "5 5";
  } else if (diff === DiffStatus.Removed) {
    stroke = "var(--color-danger)";
    opacity = 0.25;
  } else if (diff === DiffStatus.Modified) {
    stroke = "var(--color-warning)";
  }
  if (isParentConnection) {
    stroke = "var(--accent-secondary)";
    strokeWidth = 3;
  }
  if (selected) {
    stroke = "var(--active-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
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

type ConnectionEdgeInnerProps = EdgeProps<ConnectionEdge> & { connectionGuid: Guid };

const ConnectionEdgeInner: React.FC<ConnectionEdgeInnerProps> = ({ sourceX, sourceY, targetX, targetY, data, selected, connectionGuid }) => {
  const { hoverConnection, clearHover } = useDesignAppCommands();
  const isHovered = useIsConnectionHovered();
  const HANDLE_HEIGHT = 5;
  const path = `M ${sourceX} ${sourceY + HANDLE_HEIGHT / 2} L ${targetX} ${targetY + HANDLE_HEIGHT / 2}`;

  const diff = (data?.SemioConnection?.attributes?.find((q: any) => q.key === "semio.diffStatus")?.value as DiffStatus) || DiffStatus.Unchanged;
  const isParentConnection = data?.isParentConnection ?? false;

  let stroke = "var(--foreground)";
  let strokeWidth = 2;
  let dasharray: string | undefined;
  let opacity = 1;

  if (diff === DiffStatus.Added) {
    stroke = "var(--color-success)";
    dasharray = "5 5";
  } else if (diff === DiffStatus.Removed) {
    stroke = "var(--color-danger)";
    opacity = 0.25;
  } else if (diff === DiffStatus.Modified) {
    stroke = "var(--color-warning)";
  }
  if (isParentConnection) {
    stroke = "var(--accent-secondary)";
    strokeWidth = 3;
  }
  if (isHovered && !selected) {
    stroke = "var(--hover-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }
  if (selected) {
    stroke = "var(--active-base)";
    strokeWidth = Math.max(strokeWidth, 3);
    dasharray = undefined;
    opacity = 1;
  }

  return (
    <g>
      <BaseEdge
        path={path}
        style={{
          stroke,
          strokeWidth,
          strokeDasharray: dasharray,
          opacity,
        }}
        className="transition-colors duration-200 pointer-events-none"
      />
      <path
        d={path}
        fill="none"
        stroke="transparent"
        strokeWidth={Math.max(strokeWidth, 6)}
        onPointerEnter={() => {
          if (connectionGuid) hoverConnection(connectionGuid);
        }}
        onPointerLeave={() => clearHover()}
      />
    </g>
  );
};
const edgeComponents = { SemioConnection: ConnectionEdgeComponent };

const ConnectionConnectionLine: React.FC<ConnectionLineComponentProps> = (props: ConnectionLineComponentProps) => {
  const { fromX, fromY, toX, toY } = props;
  const HANDLE_HEIGHT = 5;
  const path = `M ${fromX} ${fromY + HANDLE_HEIGHT / 2} L ${toX} ${toY + HANDLE_HEIGHT / 2}`;
  return <BaseEdge path={path} style={{ stroke: "gray" }} className="opacity-70" />;
};

export const MiniMapNode: React.FC<MiniMapNodeProps> = ({ x, y, selected }: MiniMapNodeProps) => {
  return <circle className={`${selected ? "fill-accent" : "fill-foreground"} transition-colors duration-200`} cx={x} cy={y} r="10" />;
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
          return <div key={`h-${line.relatedPieceId}-${index}`} className="absolute left-0 w-full h-px border-t border-dashed border-accent opacity-60" style={{ top: screenY }} />;
        } else if (line.type === "vertical" && line.position !== undefined) {
          const screenX = line.position * viewport.zoom + viewport.x;
          return <div key={`v-${line.relatedPieceId}-${index}`} className="absolute top-0 w-px h-full border-l border-dashed border-accent opacity-60" style={{ left: screenX }} />;
        } else if (line.type === "equalDistance" && line.x1 !== undefined && line.y1 !== undefined && line.x2 !== undefined && line.y2 !== undefined) {
          const screenX1 = line.x1 * viewport.zoom + viewport.x;
          const screenY1 = line.y1 * viewport.zoom + viewport.y;
          const screenX2 = line.x2 * viewport.zoom + viewport.x;
          const screenY2 = line.y2 * viewport.zoom + viewport.y;

          const isMidLine = line.relatedPieceId.startsWith("mid-");
          const strokeColor = "var(--accent)";
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
  id: `piece-${index}-${piece.guid}`,
  position: {
    x: center.x * ICON_WIDTH || 0,
    y: -center.y * ICON_WIDTH || 0,
  },
  selected,
  draggable: true,
  data: { piece, type },
  className: selected ? "selected" : "",
});

const designToNode = (piece: Piece, externalConnections: SemioConnection[], center: Coord, selected: boolean, index: number): DesignNode => ({
  type: "design",
  id: `piece-${index}-${piece.guid}`,
  position: {
    x: center.x * ICON_WIDTH || 0,
    y: -center.y * ICON_WIDTH || 0,
  },
  selected,
  draggable: true,
  data: { piece, externalConnections },
  className: selected ? "selected" : "",
});

const extractPieceIdFromNodeId = (nodeId: string): string => {
  return nodeId.split("-").slice(2).join("-");
};

const getPieceIdFromNode = (node: DiagramNode): string => {
  return node.data.piece.guid;
};

const connectionToEdge = (
  SemioConnection: SemioConnection,
  selected: boolean,
  isParentConnection: boolean = false,
  pieceIndexMap: Map<string, number>,
  connectionIndex: number = 0,
  designPieces?: Piece[],
  allConnections?: SemioConnection[],
): ConnectionEdge => {
  let sourcePieceId = SemioConnection.connecting.piece;
  let targetPieceId = SemioConnection.connected.piece;
  let sourcePortId = SemioConnection.connecting.port ?? "undefined";
  let targetPortId = SemioConnection.connected.port ?? "undefined";

  if (SemioConnection.connecting.designPiece && allConnections) {
    const designPieceId = SemioConnection.connecting.designPiece;
    sourcePieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece === SemioConnection.connecting.designPiece;
      const connectingToDesign = conn.connecting.designPiece === SemioConnection.connecting.designPiece;
      return connectedToDesign || connectingToDesign;
    });

    const portIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece === SemioConnection.connected.piece && conn.connecting.piece === SemioConnection.connecting.piece && conn.connected.port === SemioConnection.connected.port && conn.connecting.port === SemioConnection.connecting.port,
    );
    sourcePortId = portIndex >= 0 ? `port-${portIndex}` : "port-0";
  }

  if (SemioConnection.connected.designPiece && allConnections) {
    const designPieceId = SemioConnection.connected.designPiece;
    targetPieceId = designPieceId;

    const externalConnections = allConnections.filter((conn) => {
      const connectedToDesign = conn.connected.designPiece === SemioConnection.connected.designPiece;
      const connectingToDesign = conn.connecting.designPiece === SemioConnection.connected.designPiece;
      return connectedToDesign || connectingToDesign;
    });

    const portIndex = externalConnections.findIndex(
      (conn) =>
        conn.connected.piece === SemioConnection.connected.piece && conn.connecting.piece === SemioConnection.connecting.piece && conn.connected.port === SemioConnection.connected.port && conn.connecting.port === SemioConnection.connecting.port,
    );
    targetPortId = portIndex >= 0 ? `port-${portIndex}` : "port-0";
  }

  const sourceIndex = pieceIndexMap.get(sourcePieceId) ?? 0;
  const targetIndex = pieceIndexMap.get(targetPieceId) ?? 0;
  const sourceNodeId = `piece-${sourceIndex}-${sourcePieceId}`;
  const targetNodeId = `piece-${targetIndex}-${targetPieceId}`;

  return {
    type: "SemioConnection",
    id: SemioConnection.guid,
    source: sourceNodeId,
    sourceHandle: sourcePortId,
    target: targetNodeId,
    targetHandle: targetPortId,
    data: { SemioConnection, isParentConnection },
    selected,
  };
};

const designToNodesAndEdges = (design: Design, flattenedDesign: Design, metadata: Map<string, any>, kit: any, selection: any) => {
  if (!design) return null;

  const centerMap = new Map<string, Coord>();
  flattenedDesign.pieces?.forEach((piece) => {
    if (piece.guid && piece.center) {
      centerMap.set(piece.guid, piece.center);
    }
  });

  const pieceNodes =
    design.pieces
      ?.map((piece, i) => {
        const isSelected = selection?.pieces?.includes(piece.guid) ?? false;
        const center = centerMap.get(piece.guid) || piece.center || { x: 0, y: 0 };

        if (piece.design) {
          const design = kit.designs?.find((d: Design) => d.guid === piece.design);
          if (!design) {
            const fallbackType: Type = {
              guid: `fallback-${piece.design}`,
              name: `Unknown-${piece.design}`,
              variant: undefined,
              unit: "m",
              description: `Missing design: ${piece.design}`,
              ports: [],
              representations: [],
            };
            return pieceToNode(piece, fallbackType, center, isSelected, i);
          }
          const designAsType: Type = {
            guid: design.guid,
            name: design.name,
            variant: design.variant,
            unit: design.unit || "m",
            description: design.description,
            ports: [],
            representations: [],
          };
          return pieceToNode(piece, designAsType, center, isSelected, i);
        }

        if (!piece.type) {
          return null;
        }

        const type = findTypeInKit(kit, piece.type);
        if (!type) {
          const fallbackType: Type = {
            guid: `fallback-${piece.type}`,
            name: `Unknown-${piece.type}`,
            variant: undefined,
            unit: "m",
            description: `Missing type: ${piece.type}`,
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
    const isSelected = selection?.pieces?.includes(includedDesign.designGuid) ?? false;

    if (includedDesign.type === "connected") {
      let calculatedCenter = { x: 0, y: 0 };
      if (includedDesign.externalConnections && includedDesign.externalConnections.length > 0) {
        const connectedPieceIds = new Set<string>();
        includedDesign.externalConnections.forEach((conn) => {
          if (conn.connected.designPiece === includedDesign.designGuid) {
            connectedPieceIds.add(conn.connecting.piece);
          } else if (conn.connecting.designPiece === includedDesign.designGuid) {
            connectedPieceIds.add(conn.connected.piece);
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
        guid: includedDesign.guid,
        type: includedDesign.designGuid,
        center: calculatedCenter,
        description: `Clustered design: ${includedDesign.designGuid}`,
      };

      return designToNode(designPiece, includedDesign.externalConnections || [], calculatedCenter, isSelected, design.pieces!.length + i);
    } else {
      const displayCenter = includedDesign.center || { x: 0, y: 0 };

      const designPiece: Piece = {
        guid: includedDesign.guid,
        type: includedDesign.designGuid,
        center: displayCenter,
        plane: includedDesign.plane,
        description: `Fixed design: ${includedDesign.designGuid}`,
      };

      return designToNode(designPiece, [], displayCenter, isSelected, design.pieces!.length + i);
    }
  });

  const pieceIndexMap = new Map<string, number>();
  design.pieces?.forEach((piece, index) => {
    if (!pieceIndexMap.has(piece.guid)) {
      pieceIndexMap.set(piece.guid, index);
    }
  });

  includedDesigns.forEach((includedDesign, index) => {
    if (!pieceIndexMap.has(includedDesign.guid)) {
      pieceIndexMap.set(includedDesign.guid, design.pieces!.length + index);
    }
  });

  const nodeIdToPieceIndexMap = new Map<string, number>();
  design.pieces?.forEach((piece, index) => {
    nodeIdToPieceIndexMap.set(`piece-${index}-${piece.guid}`, index);
  });
  includedDesigns.forEach((includedDesign, index) => {
    const nodeIndex = design.pieces!.length + index;
    nodeIdToPieceIndexMap.set(`piece-${nodeIndex}-${includedDesign.guid}`, nodeIndex);
  });

  const parentConnectionGuid =
    selection?.pieces?.length === 1 && (selection?.connections?.length === 0 || !selection?.connections)
      ? (() => {
          const selectedPieceGuid = selection.pieces[0];
          const pieceMetadata = metadata.get(selectedPieceGuid);
          if (pieceMetadata?.parentPieceId) {
            const parentConnection = design.connections?.find(
              (c) => (c.connected.piece === selectedPieceGuid && c.connecting.piece === pieceMetadata.parentPieceId) || (c.connecting.piece === selectedPieceGuid && c.connected.piece === pieceMetadata.parentPieceId),
            );
            return parentConnection?.guid ?? null;
          }
          return null;
        })()
      : null;

  const connectionEdges =
    design.connections?.map((SemioConnection, connectionIndex) => {
      const isSelected = selection?.connections?.includes(SemioConnection.guid) ?? false;

      const isParentConnection = parentConnectionGuid === SemioConnection.guid;

      return connectionToEdge(SemioConnection, isSelected, isParentConnection, pieceIndexMap, connectionIndex, design.pieces, design.connections);
    }) ?? [];
  return { nodes: [...pieceNodes, ...designNodes], edges: connectionEdges };
};

interface DiagramProps {
  reactFlowInstanceRef: React.MutableRefObject<ReactFlowInstance | null>;
}

const Diagram: FC<DiagramProps> = ({ reactFlowInstanceRef }) => {
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
    addPiece,
    setDiagramCenter,
    setDiagramScale,
    focusPiece,
  } = useDesignAppCommands();

  const { updateDesign } = useKitCommands();
  const sketchpadCommands = useSketchpadCommands();
  const kit = useKit();
  const activeTool = (useDesignApp((s) => s.activeTool) as ToolType | undefined) ?? ToolType.SELECTION_NORMAL;

  const selection = useDesignAppSelection();
  const fullscreenWindow = useDesignAppFullscreen();
  const others = useDesignAppOthers();
  const savedDiagramCenter = useDesignAppDiagramCenter();
  const savedDiagramScale = useDesignAppDiagramScale();
  const panelVisibility = useAppPanelVisibility();

  // const design = useDiffedDesign();
  const design = useDesign() as Design | null;
  // const types = usePortColoredTypes();
  // const flattenedDesign = useFlatDesign();
  const flattenedDesign = design;
  // const metadata = usePiecesMetadata();
  const metadata = useMemo(() => new Map(), []);

  const { nodes, edges } = useMemo(() => {
    if (!design || !flattenedDesign) return { nodes: [], edges: [] };
    return (
      designToNodesAndEdges(design, flattenedDesign, metadata, kit, selection) ?? {
        nodes: [],
        edges: [],
      }
    );
  }, [design, flattenedDesign, metadata, kit, selection]);

  const focusContext = useFocusSafe();
  const [focusedItemId, setFocusedItemId] = useState<string | undefined>();
  const prevItemsRef = useRef<string>("");

  useEffect(() => {
    if (!focusContext) return;
    const items = [
      ...nodes.map((n) => ({
        id: n.data.piece.guid, // Use actual piece.guid for 3D scene focus
        label: n.data.piece.description || `Piece ${n.data.piece.guid.substring(0, 8)}`,
        category: "Pieces",
      })),
      ...edges.map((e) => ({
        id: e.data?.SemioConnection?.guid || e.id,
        label: e.data?.SemioConnection?.description || `Connection ${e.id}`,
        category: "Connections",
      })),
    ];
    // Only update if the items have actually changed
    const itemsKey = items.map((item) => `${item.id}:${item.label}`).join("|");
    if (prevItemsRef.current !== itemsKey) {
      prevItemsRef.current = itemsKey;
      focusContext.setFocusItems(items);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [nodes, edges]);

  useEffect(() => {
    if (!focusContext) return;
    const handleFocus = (itemId: string) => {
      // itemId is the piece.guid
      // Find the corresponding React Flow node ID for 2D diagram focus
      const node = nodes.find((n) => n.data.piece.guid === itemId);
      if (node) {
        setFocusedItemId(node.id); // Focus 2D diagram with React Flow node ID
      }
      focusPiece(itemId); // Focus 3D scene with piece.guid
    };
    focusContext.setOnFocusItem(handleFocus);
    return () => {
      if (focusContext) focusContext.setOnFocusItem(undefined);
    };
  }, [focusContext, focusPiece, nodes]);

  if (!design) return null;

  const [dragState, setDragState] = useState<{ lastPostition: XYPosition } | null>(null);
  const [helperLines, setHelperLines] = useState<HelperLine[]>([]);
  const fullscreen = fullscreenWindow === DesignAppFullscreenWindow.Diagram;
  const viewportRestoredRef = useRef(false);
  const isUpdatingViewportRef = useRef(false);
  const { setNodeRef: setDroppableRef } = useDroppable({ id: "diagram-drop-zone" });

  useEffect(() => {
    const handleEscape = (event: KeyboardEvent) => {
      if (event.key === "Escape" && dragState) {
        // Abort the transaction and reset drag state
        abortTransaction();
        setDragState(null);
        setHelperLines([]);

        // Reset the node positions to their original state by triggering a re-render
        // The transaction abort will have restored the data, we just need to update the UI
        if (reactFlowInstanceRef.current) {
          reactFlowInstanceRef.current.setNodes((nodes) => nodes.map((node) => ({ ...node })));
        }
      }
    };

    document.addEventListener("keydown", handleEscape);
    return () => document.removeEventListener("keydown", handleEscape);
  }, [dragState, abortTransaction, reactFlowInstanceRef]);

  useEffect(() => {
    if (!viewportRestoredRef.current && savedDiagramCenter && savedDiagramScale !== undefined && reactFlowInstanceRef.current) {
      isUpdatingViewportRef.current = true;
      setTimeout(() => {
        if (reactFlowInstanceRef.current) {
          reactFlowInstanceRef.current.setViewport({ x: savedDiagramCenter.x, y: savedDiagramCenter.y, zoom: savedDiagramScale });
          viewportRestoredRef.current = true;
          setTimeout(() => {
            isUpdatingViewportRef.current = false;
          }, 100);
        }
      }, 0);
    }
  }, [savedDiagramCenter, savedDiagramScale, reactFlowInstanceRef]);

  const onMoveEnd = useCallback(() => {
    if (isUpdatingViewportRef.current || !reactFlowInstanceRef.current) return;
    const viewport = reactFlowInstanceRef.current.getViewport();
    setDiagramCenter({ x: viewport.x, y: viewport.y });
    setDiagramScale(viewport.zoom);
  }, [reactFlowInstanceRef, setDiagramCenter, setDiagramScale]);

  const onNodeClick = (e: React.MouseEvent, node: DiagramNode) => {
    console.log("onNodeClick fired", node.id, "target:", e.target, "currentTarget:", e.currentTarget, "classList:", (e.target as HTMLElement).className);
    e.stopPropagation();
    const pieceId = getPieceIdFromNode(node);
    if (e.ctrlKey || e.metaKey) removePieceFromSelection(pieceId);
    else if (e.shiftKey) addPieceToSelection(pieceId);
    else if (activeTool === ToolType.SELECTION_ADDITIVE) addPieceToSelection(pieceId);
    else if (activeTool === ToolType.SELECTION_SUBTRACTIVE) removePieceFromSelection(pieceId);
    else selectPiece(pieceId);
  };

  const onNodeDoubleClick = (e: React.MouseEvent, node: DiagramNode) => {
    e.stopPropagation();
    const kitData = kit as Kit;
    if (!kitData?.guid) return;
    const piece = node.data.piece;
    if (piece.type) sketchpadCommands.navigateToType(kitData.guid, piece.type);
    else if (piece.design) sketchpadCommands.navigateToDesign(kitData.guid, piece.design);
  };

  const onEdgeClick = (e: React.MouseEvent, edge: DiagramEdge) => {
    e.stopPropagation();
    const connectionId = edge.data!.SemioConnection.guid;
    if (e.ctrlKey || e.metaKey) removeConnectionFromSelection(connectionId);
    else if (e.shiftKey) addConnectionToSelection(connectionId);
    else if (activeTool === ToolType.SELECTION_ADDITIVE) addConnectionToSelection(connectionId);
    else if (activeTool === ToolType.SELECTION_SUBTRACTIVE) removeConnectionFromSelection(connectionId);
    else selectConnection(connectionId);
  };

  const onPaneClick = (e: React.MouseEvent) => {
    console.log("onPaneClick fired", "target:", e.target, "currentTarget:", e.currentTarget, "classList:", (e.target as HTMLElement).className);
    e.stopPropagation();
    if (!(e.ctrlKey || e.metaKey) && !e.shiftKey) {
      deselectAll();
    }
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
    (target: string) => {
      execute?.("explode", { designId: target }).catch(() => {});
    },
    [execute],
  );

  const onNodeDragStart = useCallback(
    (event: any, node: Node) => {
      const currentSelectedIds = selection?.pieces ?? [];
      const pieceId = getPieceIdFromNode(node as DiagramNode);
      const isNodeSelected = currentSelectedIds.includes(pieceId);
      const ctrlKey = event.ctrlKey || event.metaKey;
      const shiftKey = event.shiftKey;

      if (ctrlKey) isNodeSelected ? removePieceFromSelection(pieceId) : addPieceToSelection(pieceId);
      else if (shiftKey) !isNodeSelected ? addPieceToSelection(pieceId) : selectPiece(pieceId);
      else if (activeTool === ToolType.SELECTION_ADDITIVE) addPieceToSelection(pieceId);
      else if (activeTool === ToolType.SELECTION_SUBTRACTIVE) removePieceFromSelection(pieceId);
      else if (!isNodeSelected) selectPiece(pieceId);

      startTransaction();
      setDragState({ lastPostition: { x: node.position.x, y: node.position.y } });
      setHelperLines([]);
    },
    [selectPiece, removePieceFromSelection, addPieceToSelection, startTransaction, selection, activeTool],
  );

  const onNodeDrag = useCallback(
    (event: any, node: DiagramNode) => {
      // Allow dragging for both piece and design nodes
      if (!dragState || !reactFlowInstanceRef.current) return;

      const piece = node.data.piece as Piece;
      const MIN_DISTANCE = 150;
      const SNAP_THRESHOLD = 20;
      const { lastPostition } = dragState;

      const altPressed = event.altKey;

      const currentHelperLines: HelperLine[] = [];
      const nonSelectedNodes = nodes.filter((n) => !(selection?.pieces ?? []).includes(getPieceIdFromNode(n)));
      const draggedCenterX = node.position.x + ICON_WIDTH / 2;
      const draggedCenterY = node.position.y + ICON_WIDTH / 2;

      const addedConnections: SemioConnection[] = [];
      const updatedPieces: Array<{ id: string; diff: any }> = [];

      let draggedX = node.position.x;
      let draggedY = node.position.y;

      for (const selectedNode of nodes.filter((n) => selection?.pieces?.includes(getPieceIdFromNode(n)))) {
        const piece = selectedNode.data.piece;
        const selectedInternalNode = reactFlowInstanceRef.current.getInternalNode(selectedNode.id)!;

        // Design nodes are moved without port snapping
        if (selectedNode.type === "design") {
          if (selectedNode.id === node.id) {
            selectedInternalNode.internals.positionAbsolute.x = draggedX;
            selectedInternalNode.internals.positionAbsolute.y = draggedY;
            node.position.x = draggedX;
            node.position.y = draggedY;
          }

          const scaledOffset = {
            x: (draggedX - lastPostition.x) / ICON_WIDTH,
            y: -(draggedY - lastPostition.y) / ICON_WIDTH,
          };
          updatedPieces.push({
            id: piece.guid,
            diff: {
              center: {
                x: (piece.center?.x ?? 0) + scaledOffset.x,
                y: (piece.center?.y ?? 0) + scaledOffset.y,
              },
            },
          });
          continue;
        }

        // Handle piece nodes with port snapping
        const type = (selectedNode as PieceNode).data.type;
        const fixedPieceId = metadata.get(piece.guid)?.fixedPieceId;
        let closestConnection: SemioConnection | null = null;
        let closestDistance = Number.MAX_VALUE;

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
          for (const otherNode of nodes.filter((n) => !(selection.pieces ?? []).includes(getPieceIdFromNode(n)))) {
            if (otherNode.type !== "piece") continue;
            const existingConnection = design?.connections?.find((c) =>
              areSameConnection(c, {
                connected: { piece: selectedNode.data.piece.guid },
                connecting: { piece: otherNode.data.piece.guid },
              } as SemioConnection),
            );
            if (existingConnection) continue;
            const otherInternalNode = reactFlowInstanceRef.current.getInternalNode(otherNode.id)!;
            for (const handle of selectedInternalNode.internals.handleBounds?.source ?? []) {
              const port = findPortInType(type, handle.id!);
              for (const otherHandle of otherInternalNode.internals.handleBounds?.source ?? []) {
                const otherPort = findPortInType((otherNode as PieceNode).data.type, otherHandle.id!);
                const haveSameFixedPiece = fixedPieceId && fixedPieceId === metadata.get(otherNode.data.piece.guid)?.fixedPieceId;
                if (haveSameFixedPiece || !arePortsCompatible(port, otherPort) || (design && isPortInUse(design, piece.guid, port.guid)) || (design && isPortInUse(design, otherNode.data.piece.guid, otherPort.guid))) continue;
                const dx = selectedInternalNode.internals.positionAbsolute.x + handle.x - (otherInternalNode.internals.positionAbsolute.x + otherHandle.x);
                const dy = selectedInternalNode.internals.positionAbsolute.y + handle.y - (otherInternalNode.internals.positionAbsolute.y + otherHandle.y);
                const distance = Math.sqrt(dx * dx + dy * dy);
                if (distance < closestDistance && distance < MIN_DISTANCE) {
                  closestConnection = {
                    guid: crypto.randomUUID(),
                    connected: {
                      guid: crypto.randomUUID(),
                      piece: otherNode.data.piece.guid,
                      port: otherHandle.id!,
                    },
                    connecting: {
                      guid: crypto.randomUUID(),
                      piece: selectedNode.data.piece.guid,
                      port: handle.id!,
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
          updatedPieces.push({
            id: selectedNode.data.piece.guid,
            diff: {
              center: undefined,
              plane: undefined,
            },
          });
        } else {
          const scaledOffset = {
            x: (draggedX - lastPostition.x) / ICON_WIDTH,
            y: -(draggedY - lastPostition.y) / ICON_WIDTH,
          };
          updatedPieces.push({
            id: piece.guid,
            diff: {
              center: {
                x: (piece.center?.x ?? 0) + scaledOffset.x,
                y: (piece.center?.y ?? 0) + scaledOffset.y,
              },
            },
          });
        }
      }

      if (addedConnections.length > 0) {
        addedConnections.forEach((conn) => addConnection(conn));
      }
      if (updatedPieces.length > 0) {
        updatePieces(updatedPieces);
      }
      setDragState({
        ...dragState!,
        lastPostition: { x: draggedX, y: draggedY },
      });
    },
    [addConnection, updatePieces, design, reactFlowInstanceRef, selection, nodes, metadata, dragState],
  );

  const onNodeDragStop = useCallback(() => {
    finalizeTransaction();
    setDragState(null);
    setHelperLines([]);
  }, [finalizeTransaction]);

  const onConnect = useCallback(
    (params: RFConnection) => {
      if (params.source === params.target || !reactFlowInstanceRef.current) return;

      const sourceInternalNode = reactFlowInstanceRef.current.getInternalNode(params.source);
      const targetInternalNode = reactFlowInstanceRef.current.getInternalNode(params.target);
      if (!sourceInternalNode || !targetInternalNode) return;

      const sourceHandle = (sourceInternalNode.internals.handleBounds?.source ?? []).find((h: any) => h.id === params.sourceHandle);
      const targetHandle = (targetInternalNode.internals.handleBounds?.source ?? []).find((h: any) => h.id === params.targetHandle);
      if (!sourceHandle || !targetHandle) return;

      const sourcePieceId = extractPieceIdFromNodeId(params.source!);
      const targetPieceId = extractPieceIdFromNodeId(params.target!);

      const newConnection = {
        guid: crypto.randomUUID(),
        connected: {
          guid: crypto.randomUUID(),
          piece: sourcePieceId,
          port: params.sourceHandle!,
        },
        connecting: {
          guid: crypto.randomUUID(),
          piece: targetPieceId,
          port: params.targetHandle!,
        },
        x: (sourceInternalNode.internals.positionAbsolute.x + sourceHandle.x - (targetInternalNode.internals.positionAbsolute.x + targetHandle.x)) / ICON_WIDTH,
        y: -((sourceInternalNode.internals.positionAbsolute.y + sourceHandle.y - (targetInternalNode.internals.positionAbsolute.y + targetHandle.y)) / ICON_WIDTH),
      };

      if (!design) return;
      if (((design as Design).connections ?? []).find((c: SemioConnection) => areSameConnection(c, newConnection))) return;
      addConnection(newConnection);
    },
    [addConnection, reactFlowInstanceRef, design],
  );

  return (
    <div id="diagram" className="h-full w-full relative">
      <BaseDiagram
        wrapperRef={setDroppableRef}
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeComponents as NodeTypes}
        edgeTypes={edgeComponents as EdgeTypes}
        connectionMode="loose"
        connectionLineComponent={ConnectionConnectionLine}
        elementsSelectable={false}
        nodesFocusable={false}
        edgesFocusable={false}
        nodesDraggable={true}
        minZoom={0.1}
        maxZoom={12}
        fitView={!savedDiagramCenter && !savedDiagramScale}
        panOnDrag={[0]}
        zoomOnDoubleClick={false}
        onNodeClick={onNodeClick as any}
        onNodeDoubleClick={onNodeDoubleClick as any}
        onEdgeClick={onEdgeClick as any}
        onNodeDragStart={onNodeDragStart as any}
        onNodeDrag={onNodeDrag as any}
        onNodeDragStop={onNodeDragStop as any}
        onPaneClick={onPaneClick}
        onPaneDoubleClick={onDoubleClick}
        onMoveEnd={onMoveEnd}
        onConnect={onConnect}
        reactFlowInstanceRef={reactFlowInstanceRef}
        showControls={fullscreen && panelVisibility.toolbar}
        showMinimap={fullscreen && panelVisibility.toolbar}
        miniMapNodeComponent={MiniMapNode}
        focusedItemId={focusedItemId}
        onFocusComplete={() => setFocusedItemId(undefined)}
        panels={
          <>
            <ViewportPortal>⌞</ViewportPortal>
            {others.map((presence, idx) => (
              <PresenceDiagram key={`presence-${idx}-${presence.name}-${presence.cursor?.x || 0}-${presence.cursor?.y || 0}`} {...presence} />
            ))}
          </>
        }
      />
      <HelperLines lines={helperLines} nodes={nodes} />
      {/* <ClusterMenu nodes={nodes} edges={edges} onCluster={onCluster} /> */}
      {/* <ExpandMenu nodes={nodes} edges={edges} onExpand={onExpand} /> */}
    </div>
  );
};

export default Diagram;
