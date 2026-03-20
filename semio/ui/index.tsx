// #region 🔖Header

// 💻 semio/ui/index.tsx

// Specs: Re-export generic ui primitives and override Diagram with a semio design mini-map renderer.
// Summary: Shared semio ui exports plus a minimal design diagram component.
//
// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Shared export surface for semio ui components.

// #endregion 🔖Header

import { applyDesignDiff, findDesignInKit, flattenDesign, type Connection, type Design, type DesignDiff, type Kit, type Piece } from "@semio/js";
import * as React from "react";

// #region 🔖Exports

// Re-export all ui primitives from @elements/ui.

export * from "@elements/ui";

// #endregion 🔖Exports

// #region 🔖Diagram

const DEFAULT_DIAGRAM_PADDING = 12;
const DEFAULT_DIAGRAM_PIECE_RADIUS = 1.75;
const DEFAULT_DIAGRAM_STROKE_WIDTH = 1;
const DEFAULT_DIAGRAM_ZOOM = 1;
const MIN_DIAGRAM_ZOOM = 1;
const MAX_DIAGRAM_ZOOM = 12;
const DIAGRAM_ZOOM_STEP = 0.0015;
const MIN_DIAGRAM_SPAN = 1;

type DiagramEntityStatus = "default" | "removed" | "added" | "modified";

export interface DiagramSelection {
  pieceGuids?: string[];
  connectionGuids?: string[];
}

export interface DiagramPan {
  x: number;
  y: number;
}

export interface SemioDiagramProps {
  kit: Kit;
  designGuid: string;
  designDiff?: DesignDiff;
  defaultDesignDiff?: DesignDiff;
  diffEnabled?: boolean;
  selection?: DiagramSelection;
  defaultSelection?: DiagramSelection;
  selectionEnabled?: boolean;
  pieceSelectionEnabled?: boolean;
  connectionSelectionEnabled?: boolean;
  onSelectionChange?: (selection: DiagramSelection) => void;
  pan?: DiagramPan;
  defaultPan?: DiagramPan;
  panEnabled?: boolean;
  onPanChange?: (pan: DiagramPan) => void;
  zoom?: number;
  defaultZoom?: number;
  zoomEnabled?: boolean;
  onZoomChange?: (zoom: number) => void;
  className?: string;
  padding?: number;
  pieceRadius?: number;
  strokeWidth?: number;
  title?: string;
  onPieceClick?: (piece: Piece) => void;
  onConnectionClick?: (connection: Connection) => void;
}

interface DiagramPoint {
  guid: string;
  piece: Piece;
  u: number;
  v: number;
  status: DiagramEntityStatus;
}

interface DiagramLine {
  guid: string;
  connection: Connection;
  source: DiagramPoint;
  target: DiagramPoint;
  status: DiagramEntityStatus;
}

interface DiagramSnapshot {
  lines: DiagramLine[];
  points: DiagramPoint[];
  minU: number;
  maxU: number;
  minY: number;
  maxY: number;
  width: number;
  height: number;
}

interface DiagramBounds {
  minU: number;
  maxU: number;
  minY: number;
  maxY: number;
  width: number;
  height: number;
}

const buildFlatDesign = (kit: Kit, design: Design): Design => {
  const flattenedKit: Kit = {
    ...kit,
    designs: (kit.designs ?? []).map((candidate) => (candidate.guid === design.guid ? design : candidate)),
  };
  return applyDesignDiff(design, flattenDesign(flattenedKit, design.guid).forward);
};

const getEntityStatusColor = (status: DiagramEntityStatus): string => {
  if (status === "removed") return "var(--color-removed)";
  if (status === "added") return "var(--color-new)";
  if (status === "modified") return "var(--color-modified)";
  return "currentColor";
};

const getInteractiveEntityColor = (status: DiagramEntityStatus, isSelected: boolean, isHovered: boolean): string => {
  if (isSelected) {
    return status === "default" ? "var(--accent)" : "var(--color-changed-selected)";
  }
  if (isHovered) {
    return status === "default" ? "var(--accent-secondary)" : "var(--color-changed-hovered)";
  }
  return getEntityStatusColor(status);
};

const buildDiagramSnapshot = (kit: Kit, designGuid: string, padding: number, designDiff?: DesignDiff): DiagramSnapshot => {
  const baseDesign = findDesignInKit(kit, designGuid);
  const nextDesign = designDiff ? applyDesignDiff(baseDesign, designDiff) : baseDesign;
  const flatBaseDesign = buildFlatDesign(kit, baseDesign);
  const flatNextDesign = designDiff ? buildFlatDesign(kit, nextDesign) : flatBaseDesign;
  const removedPieceGuids = new Set((designDiff?.pieces?.removed ?? []).map((piece) => piece.guid));
  const addedPieceGuids = new Set((designDiff?.pieces?.added ?? []).map((piece) => piece.guid));
  const modifiedPieceGuids = new Set((designDiff?.pieces?.updated ?? []).map((piece) => piece.piece.guid));
  const removedConnectionGuids = new Set((designDiff?.connections?.removed ?? []).map((connection) => connection.guid));
  const addedConnectionGuids = new Set((designDiff?.connections?.added ?? []).map((connection) => connection.guid));
  const modifiedConnectionGuids = new Set((designDiff?.connections?.updated ?? []).map((connection) => connection.connection.guid));

  const pointMap = new Map<string, DiagramPoint>();
  const upsertPoint = (piece: Piece, status: DiagramEntityStatus) => {
    if (!piece.guid || !piece.center) return;
    pointMap.set(piece.guid, {
      guid: piece.guid,
      piece,
      u: piece.center.u,
      v: piece.center.v,
      status,
    });
  };

  (flatBaseDesign.pieces ?? []).forEach((piece) => {
    if (removedPieceGuids.has(piece.guid)) {
      upsertPoint(piece, "removed");
    } else if (!designDiff) {
      upsertPoint(piece, "default");
    }
  });
  (flatNextDesign.pieces ?? []).forEach((piece) => {
    if (addedPieceGuids.has(piece.guid)) {
      upsertPoint(piece, "added");
    } else if (modifiedPieceGuids.has(piece.guid)) {
      upsertPoint(piece, "modified");
    } else {
      upsertPoint(piece, "default");
    }
  });

  const points = Array.from(pointMap.values());
  const pointsByGuid = new Map(points.map((point) => [point.guid, point]));
  const lineMap = new Map<string, DiagramLine>();
  const upsertLine = (connection: Connection, status: DiagramEntityStatus) => {
    if (!connection.guid) return;
    const source = pointsByGuid.get(connection.connected.piece.guid);
    const target = pointsByGuid.get(connection.connecting.piece.guid);
    if (!source || !target) return;
    lineMap.set(connection.guid, {
      guid: connection.guid,
      connection,
      source,
      target,
      status,
    });
  };

  (flatBaseDesign.connections ?? []).forEach((connection) => {
    if (removedConnectionGuids.has(connection.guid)) {
      upsertLine(connection, "removed");
    } else if (!designDiff) {
      upsertLine(connection, "default");
    }
  });
  (flatNextDesign.connections ?? []).forEach((connection) => {
    if (addedConnectionGuids.has(connection.guid)) {
      upsertLine(connection, "added");
    } else if (modifiedConnectionGuids.has(connection.guid)) {
      upsertLine(connection, "modified");
    } else {
      upsertLine(connection, "default");
    }
  });

  const lines = Array.from(lineMap.values());
  const minU = points.length > 0 ? Math.min(...points.map((point) => point.u)) : -0.5;
  const maxU = points.length > 0 ? Math.max(...points.map((point) => point.u)) : 0.5;
  const minY = points.length > 0 ? Math.min(...points.map((point) => -point.v)) : -0.5;
  const maxY = points.length > 0 ? Math.max(...points.map((point) => -point.v)) : 0.5;
  const width = Math.max(maxU - minU, MIN_DIAGRAM_SPAN);
  const height = Math.max(maxY - minY, MIN_DIAGRAM_SPAN);

  return { lines, points, minU, maxU, minY, maxY, width, height };
};

const buildDiagramBounds = (points: Array<{ u: number; v: number }>): DiagramBounds | null => {
  if (points.length === 0) return null;
  const minU = Math.min(...points.map((point) => point.u));
  const maxU = Math.max(...points.map((point) => point.u));
  const minY = Math.min(...points.map((point) => -point.v));
  const maxY = Math.max(...points.map((point) => -point.v));
  return {
    minU,
    maxU,
    minY,
    maxY,
    width: Math.max(maxU - minU, MIN_DIAGRAM_SPAN),
    height: Math.max(maxY - minY, MIN_DIAGRAM_SPAN),
  };
};

const isSelected = (guid: string, guidSet: Set<string>): boolean => guidSet.has(guid);

const useElementSize = <T extends HTMLElement>() => {
  const ref = React.useRef<T | null>(null);
  const [size, setSize] = React.useState({ width: 0, height: 0 });

  React.useLayoutEffect(() => {
    const element = ref.current;
    if (!element) return;
    const update = () => {
      setSize({
        width: element.clientWidth,
        height: element.clientHeight,
      });
    };
    update();
    const observer = new ResizeObserver(update);
    observer.observe(element);
    return () => observer.disconnect();
  }, []);

  return { ref, size };
};

const normalizeSelection = (selection?: DiagramSelection): DiagramSelection => ({
  pieceGuids: selection?.pieceGuids ?? [],
  connectionGuids: selection?.connectionGuids ?? [],
});

const useResolvedValue = <T,>(value: T | undefined, defaultValue: T) => value ?? defaultValue;

const useInteractiveControllableValue = <T,>(value: T | undefined, defaultValue: T, onChange?: (nextValue: T) => void) => {
  const [internalValue, setInternalValue] = React.useState(value ?? defaultValue);
  const isControlled = value !== undefined && onChange !== undefined;
  const lastExternalValueRef = React.useRef(value);

  React.useEffect(() => {
    if (isControlled) return;
    if (value === undefined) return;
    if (Object.is(lastExternalValueRef.current, value)) return;
    lastExternalValueRef.current = value;
    setInternalValue(value);
  }, [isControlled, value]);

  const resolvedValue = isControlled ? value : internalValue;
  const setValue = React.useCallback(
    (nextValue: T) => {
      if (!isControlled) {
        setInternalValue(nextValue);
      }
      onChange?.(nextValue);
    },
    [isControlled, onChange],
  );
  return [resolvedValue, setValue, isControlled] as const;
};

export const Diagram: React.FC<SemioDiagramProps> = ({
  kit,
  designGuid,
  designDiff,
  defaultDesignDiff,
  diffEnabled,
  selection,
  defaultSelection,
  selectionEnabled,
  pieceSelectionEnabled = true,
  connectionSelectionEnabled = true,
  onSelectionChange,
  pan,
  defaultPan,
  panEnabled = true,
  onPanChange,
  zoom,
  defaultZoom,
  zoomEnabled = true,
  onZoomChange,
  className = "",
  padding = DEFAULT_DIAGRAM_PADDING,
  pieceRadius = DEFAULT_DIAGRAM_PIECE_RADIUS,
  strokeWidth = DEFAULT_DIAGRAM_STROKE_WIDTH,
  title = "Design Diagram",
  onPieceClick,
  onConnectionClick,
}) => {
  const effectiveDiffEnabled = diffEnabled ?? true;
  const effectiveSelectionEnabled = selectionEnabled ?? true;
  const effectivePieceSelectionEnabled = effectiveSelectionEnabled && pieceSelectionEnabled;
  const effectiveConnectionSelectionEnabled = effectiveSelectionEnabled && connectionSelectionEnabled;
  const resolvedDesignDiff = useResolvedValue(designDiff, defaultDesignDiff);
  const [resolvedSelection, setResolvedSelection] = useInteractiveControllableValue(selection, normalizeSelection(defaultSelection), onSelectionChange);
  const [resolvedPan, setResolvedPan, isPanControlled] = useInteractiveControllableValue(pan, defaultPan ?? { x: 0, y: 0 }, onPanChange);
  const [resolvedZoom, setResolvedZoom, isZoomControlled] = useInteractiveControllableValue(zoom, defaultZoom ?? DEFAULT_DIAGRAM_ZOOM, onZoomChange);
  const snapshot = React.useMemo(() => buildDiagramSnapshot(kit, designGuid, padding, effectiveDiffEnabled ? resolvedDesignDiff : undefined), [designGuid, effectiveDiffEnabled, kit, padding, resolvedDesignDiff]);
  const selectedPieceGuids = React.useMemo(() => new Set(effectiveSelectionEnabled ? (resolvedSelection.pieceGuids ?? []) : []), [effectiveSelectionEnabled, resolvedSelection.pieceGuids]);
  const selectedConnectionGuids = React.useMemo(() => new Set(effectiveSelectionEnabled ? (resolvedSelection.connectionGuids ?? []) : []), [effectiveSelectionEnabled, resolvedSelection.connectionGuids]);
  const [hoveredPieceGuid, setHoveredPieceGuid] = React.useState<string | null>(null);
  const [hoveredConnectionGuid, setHoveredConnectionGuid] = React.useState<string | null>(null);
  const { ref, size } = useElementSize<HTMLDivElement>();
  const panPointerIdRef = React.useRef<number | null>(null);
  const panOriginRef = React.useRef({ x: 0, y: 0, panX: 0, panY: 0 });
  const [isPanning, setIsPanning] = React.useState(false);
  const innerPadding = padding;
  const drawableWidth = Math.max(size.width - innerPadding * 2, 1);
  const drawableHeight = Math.max(size.height - innerPadding * 2, 1);
  const scale = Math.min(drawableWidth / snapshot.width, drawableHeight / snapshot.height);
  const offsetX = (size.width - snapshot.width * scale) / 2;
  const offsetY = (size.height - snapshot.height * scale) / 2;
  const centerX = size.width / 2;
  const centerY = size.height / 2;
  const toBasePixelX = (u: number) => offsetX + (u - snapshot.minU) * scale;
  const toBasePixelY = (y: number) => offsetY + (y - snapshot.minY) * scale;
  const fittedViewport = React.useMemo(() => {
    const changedPoints = effectiveDiffEnabled
      ? snapshot.points.filter((point) => point.status !== "default")
      : [];
    const changedLinePoints = effectiveDiffEnabled
      ? snapshot.lines.filter((line) => line.status !== "default").flatMap((line) => [line.source, line.target])
      : [];
    const targetBounds =
      buildDiagramBounds([...changedPoints, ...changedLinePoints]) ?? {
        minU: snapshot.minU,
        maxU: snapshot.maxU,
        minY: snapshot.minY,
        maxY: snapshot.maxY,
        width: snapshot.width,
        height: snapshot.height,
      };
    const targetMinX = toBasePixelX(targetBounds.minU);
    const targetMaxX = toBasePixelX(targetBounds.maxU);
    const targetMinY = toBasePixelY(targetBounds.minY);
    const targetMaxY = toBasePixelY(targetBounds.maxY);
    const targetWidth = Math.max(targetMaxX - targetMinX, 1);
    const targetHeight = Math.max(targetMaxY - targetMinY, 1);
    const targetCenterX = (targetMinX + targetMaxX) / 2;
    const targetCenterY = (targetMinY + targetMaxY) / 2;
    const zoomToFit = Math.min(MAX_DIAGRAM_ZOOM, Math.max(MIN_DIAGRAM_ZOOM, Math.min(drawableWidth / targetWidth, drawableHeight / targetHeight)));

    return {
      zoom: defaultZoom ?? zoomToFit,
      pan: defaultPan ?? {
        x: -zoomToFit * (targetCenterX - centerX),
        y: -zoomToFit * (targetCenterY - centerY),
      },
    };
  }, [centerX, centerY, defaultPan, defaultZoom, drawableHeight, drawableWidth, effectiveDiffEnabled, snapshot, toBasePixelX, toBasePixelY]);
  const applyViewportX = (x: number) => centerX + resolvedPan.x + resolvedZoom * (x - centerX);
  const applyViewportY = (y: number) => centerY + resolvedPan.y + resolvedZoom * (y - centerY);
  const toPixelX = (u: number) => applyViewportX(toBasePixelX(u));
  const toPixelY = (y: number) => applyViewportY(toBasePixelY(y));

  React.useEffect(() => {
    if (!isZoomControlled) {
      setResolvedZoom(fittedViewport.zoom);
    }
    if (!isPanControlled) {
      setResolvedPan(fittedViewport.pan);
    }
  }, [designGuid, fittedViewport.pan, fittedViewport.zoom, isPanControlled, isZoomControlled, kit, resolvedDesignDiff, setResolvedPan, setResolvedZoom, size.height, size.width]);

  const handleWheel = React.useCallback(
    (event: React.WheelEvent<HTMLDivElement>) => {
      if (!zoomEnabled) return;
      event.preventDefault();
      if (size.width <= 0 || size.height <= 0) return;
      const nextZoom = Math.min(MAX_DIAGRAM_ZOOM, Math.max(MIN_DIAGRAM_ZOOM, resolvedZoom * Math.exp(-event.deltaY * DIAGRAM_ZOOM_STEP)));
      if (Math.abs(nextZoom - resolvedZoom) < 0.0001) return;
      const rect = event.currentTarget.getBoundingClientRect();
      const cursorX = event.clientX - rect.left;
      const cursorY = event.clientY - rect.top;
      const baseX = centerX + (cursorX - centerX - resolvedPan.x) / resolvedZoom;
      const baseY = centerY + (cursorY - centerY - resolvedPan.y) / resolvedZoom;
      setResolvedZoom(nextZoom);
      setResolvedPan({
        x: cursorX - centerX - nextZoom * (baseX - centerX),
        y: cursorY - centerY - nextZoom * (baseY - centerY),
      });
    },
    [centerX, centerY, resolvedPan.x, resolvedPan.y, resolvedZoom, setResolvedPan, setResolvedZoom, size.height, size.width, zoomEnabled],
  );

  const handleDoubleClick = React.useCallback(() => {
    if (!zoomEnabled && !panEnabled) return;
    if (zoomEnabled) {
      setResolvedZoom(fittedViewport.zoom);
    }
    if (panEnabled) {
      setResolvedPan(fittedViewport.pan);
    }
  }, [fittedViewport.pan, fittedViewport.zoom, panEnabled, setResolvedPan, setResolvedZoom, zoomEnabled]);

  const handlePointerDown = React.useCallback(
    (event: React.PointerEvent<SVGSVGElement>) => {
      if (!panEnabled) return;
      if (event.button !== 0) return;
      if (event.target !== event.currentTarget) return;
      panPointerIdRef.current = event.pointerId;
      panOriginRef.current = {
        x: event.clientX,
        y: event.clientY,
        panX: resolvedPan.x,
        panY: resolvedPan.y,
      };
      setIsPanning(true);
      event.currentTarget.setPointerCapture(event.pointerId);
    },
    [panEnabled, resolvedPan.x, resolvedPan.y],
  );

  const handlePointerMove = React.useCallback(
    (event: React.PointerEvent<SVGSVGElement>) => {
      if (panPointerIdRef.current !== event.pointerId) return;
      const deltaX = event.clientX - panOriginRef.current.x;
      const deltaY = event.clientY - panOriginRef.current.y;
      setResolvedPan({
        x: panOriginRef.current.panX + deltaX,
        y: panOriginRef.current.panY + deltaY,
      });
    },
    [setResolvedPan],
  );

  const handlePointerEnd = React.useCallback((event: React.PointerEvent<SVGSVGElement>) => {
    if (panPointerIdRef.current !== event.pointerId) return;
    panPointerIdRef.current = null;
    setIsPanning(false);
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  }, []);

  const selectPiece = React.useCallback(
    (pieceGuid: string) => {
      if (!effectivePieceSelectionEnabled) return;
      const nextPieceGuids = new Set(resolvedSelection.pieceGuids ?? []);
      if (nextPieceGuids.has(pieceGuid)) {
        nextPieceGuids.delete(pieceGuid);
      } else {
        nextPieceGuids.add(pieceGuid);
      }
      setResolvedSelection({
        pieceGuids: Array.from(nextPieceGuids),
        connectionGuids: resolvedSelection.connectionGuids ?? [],
      });
    },
    [effectivePieceSelectionEnabled, resolvedSelection.connectionGuids, resolvedSelection.pieceGuids, setResolvedSelection],
  );

  const selectConnection = React.useCallback(
    (connectionGuid: string) => {
      if (!effectiveConnectionSelectionEnabled) return;
      const nextConnectionGuids = new Set(resolvedSelection.connectionGuids ?? []);
      if (nextConnectionGuids.has(connectionGuid)) {
        nextConnectionGuids.delete(connectionGuid);
      } else {
        nextConnectionGuids.add(connectionGuid);
      }
      setResolvedSelection({
        pieceGuids: resolvedSelection.pieceGuids ?? [],
        connectionGuids: Array.from(nextConnectionGuids),
      });
    },
    [effectiveConnectionSelectionEnabled, resolvedSelection.connectionGuids, resolvedSelection.pieceGuids, setResolvedSelection],
  );

  const clearSelection = React.useCallback(() => {
    if (!effectiveSelectionEnabled) return;
    setResolvedSelection({
      pieceGuids: [],
      connectionGuids: [],
    });
  }, [effectiveSelectionEnabled, setResolvedSelection]);

  return (
    <div ref={ref} className={`h-full w-full ${className}`} onDoubleClick={handleDoubleClick} onWheel={handleWheel}>
      <svg
        aria-label={title}
        className="h-full w-full overflow-visible text-foreground"
        onPointerCancel={handlePointerEnd}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerEnd}
        role="img"
        style={{ cursor: panEnabled ? (isPanning ? "grabbing" : "grab") : "default", touchAction: panEnabled ? "none" : "auto" }}
        onClick={clearSelection}
      >
        {snapshot.lines.map((line) => {
          const selected = isSelected(line.guid, selectedConnectionGuids);
          const hovered = hoveredConnectionGuid === line.guid;
          return (
            <line
              key={line.guid}
              onClick={
                onConnectionClick || effectiveConnectionSelectionEnabled
                  ? (event) => {
                      event.stopPropagation();
                      selectConnection(line.guid);
                      onConnectionClick?.(line.connection);
                    }
                  : undefined
              }
              pointerEvents="stroke"
              stroke={getInteractiveEntityColor(line.status, selected, hovered)}
              strokeLinecap="round"
              strokeOpacity={selected || hovered ? 1 : line.status === "default" ? 0.45 : 0.8}
              strokeWidth={(selected ? strokeWidth + 1.5 : hovered ? strokeWidth + 0.75 : strokeWidth) * resolvedZoom}
              style={{ cursor: onConnectionClick || effectiveConnectionSelectionEnabled ? "pointer" : "default" }}
              onPointerEnter={() => setHoveredConnectionGuid(line.guid)}
              onPointerLeave={() => setHoveredConnectionGuid((currentGuid) => (currentGuid === line.guid ? null : currentGuid))}
              x1={toPixelX(line.source.u)}
              x2={toPixelX(line.target.u)}
              y1={toPixelY(-line.source.v)}
              y2={toPixelY(-line.target.v)}
            />
          );
        })}
        {snapshot.points.map((point) => {
          const selected = isSelected(point.guid, selectedPieceGuids);
          const hovered = hoveredPieceGuid === point.guid;
          return (
            <circle
              key={point.guid}
              cx={toPixelX(point.u)}
              cy={toPixelY(-point.v)}
              fill={getEntityStatusColor(point.status)}
              onClick={
                onPieceClick || effectivePieceSelectionEnabled
                  ? (event) => {
                      event.stopPropagation();
                      selectPiece(point.guid);
                      onPieceClick?.(point.piece);
                    }
                  : undefined
              }
              onPointerEnter={() => setHoveredPieceGuid(point.guid)}
              onPointerLeave={() => setHoveredPieceGuid((currentGuid) => (currentGuid === point.guid ? null : currentGuid))}
              r={(selected ? pieceRadius + 0.75 : hovered ? pieceRadius + 0.35 : pieceRadius) * resolvedZoom}
              stroke={selected || hovered ? getInteractiveEntityColor(point.status, selected, hovered) : "none"}
              strokeWidth={(selected ? 1.5 : hovered ? 1 : 0) * resolvedZoom}
              style={{ cursor: onPieceClick || effectivePieceSelectionEnabled ? "pointer" : "default" }}
            />
          );
        })}
      </svg>
    </div>
  );
};

// #endregion 🔖Diagram

// #region 🔖PieceSelection

/**
 * PieceSelection is a constrained Diagram configuration that only supports selecting pieces.
 *
 * Specs:
 * - Connection selection is always disabled (no connection hover/click selection state).
 * - Selection callbacks only return `pieceGuids`.
 */
export interface PieceSelectionState {
  pieceGuids?: string[];
}

export interface PieceSelectionProps extends Omit<SemioDiagramProps, "pieceSelectionEnabled" | "connectionSelectionEnabled" | "onConnectionClick" | "selection" | "defaultSelection" | "onSelectionChange"> {
  selection?: PieceSelectionState;
  defaultSelection?: PieceSelectionState;
  onSelectionChange?: (selection: PieceSelectionState) => void;
}

export const PieceSelection: React.FC<PieceSelectionProps> = ({ selection, defaultSelection, onSelectionChange, ...rest }) => {
  const mappedSelection = selection ? { pieceGuids: selection.pieceGuids ?? [], connectionGuids: [] } : undefined;
  const mappedDefaultSelection = defaultSelection ? { pieceGuids: defaultSelection.pieceGuids ?? [], connectionGuids: [] } : undefined;

  return (
    <Diagram
      {...rest}
      pieceSelectionEnabled={true}
      connectionSelectionEnabled={false}
      selection={mappedSelection}
      defaultSelection={mappedDefaultSelection}
      onSelectionChange={
        onSelectionChange
          ? (next) => {
              onSelectionChange({ pieceGuids: next.pieceGuids ?? [] });
            }
          : undefined
      }
    />
  );
};

// #endregion 🔖PieceSelection

// #region 🔖Vec

// Specs: SVG 2D vector input with draggable handle, visible origin and axes.
// Summary: Draggable XY pad mapping pointer position to a {u,v} vector in a bounded domain.

export interface VecValue {
  u: number;
  v: number;
}

export interface VecProps {
  id: string;
  vec: VecValue;
  minU?: number;
  maxU?: number;
  minV?: number;
  maxV?: number;
  showAxes?: boolean;
  showOrigin?: boolean;
  size?: number;
  onVecChange?: (vec: VecValue) => void;
  className?: string;
}

const vecClamp = (val: number, lo: number, hi: number) => Math.min(hi, Math.max(lo, val));

/**
 * Vec displays a 2D vector input as an SVG pad with a draggable handle.
 * The U axis points right, V axis points up. Origin and axes are optionally visible.
 **/
export const Vec: React.FC<VecProps> = ({ id, vec, minU = -1, maxU = 1, minV = -1, maxV = 1, showAxes = true, showOrigin = true, size = 120, onVecChange, className = "" }) => {
  const svgRef = React.useRef<SVGSVGElement>(null);
  const [dragging, setDragging] = React.useState(false);
  const [localVec, setLocalVec] = React.useState<VecValue | null>(null);
  const rafId = React.useRef<number>(0);
  const pendingVec = React.useRef<VecValue | null>(null);
  const pad = 8;
  const inner = size - pad * 2;

  const vecFromEvent = React.useCallback(
    (e: React.PointerEvent | PointerEvent): VecValue => {
      if (!svgRef.current) return { u: 0, v: 0 };
      const rect = svgRef.current.getBoundingClientRect();
      const px = e.clientX - rect.left - pad;
      const py = e.clientY - rect.top - pad;
      const u = vecClamp(minU + (px / inner) * (maxU - minU), minU, maxU);
      const v = vecClamp(maxV - (py / inner) * (maxV - minV), minV, maxV);
      return { u, v };
    },
    [inner, minU, maxU, minV, maxV],
  );

  const toSvgX = (u: number) => pad + ((u - minU) / (maxU - minU)) * inner;
  const toSvgY = (v: number) => pad + ((maxV - v) / (maxV - minV)) * inner;

  const flushPending = React.useCallback(() => {
    if (pendingVec.current !== null) {
      onVecChange?.(pendingVec.current);
      pendingVec.current = null;
    }
  }, [onVecChange]);

  const handlePointerDown = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      e.preventDefault();
      svgRef.current?.setPointerCapture(e.pointerId);
      const v = vecFromEvent(e);
      setDragging(true);
      setLocalVec(v);
      pendingVec.current = null;
      onVecChange?.(v);
    },
    [vecFromEvent, onVecChange],
  );

  const handlePointerMove = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      if (!dragging) return;
      const v = vecFromEvent(e);
      setLocalVec(v);
      pendingVec.current = v;
      if (!rafId.current) {
        rafId.current = requestAnimationFrame(() => {
          rafId.current = 0;
          flushPending();
        });
      }
    },
    [dragging, vecFromEvent, flushPending],
  );

  const handlePointerUp = React.useCallback(
    (e: React.PointerEvent<SVGSVGElement>) => {
      if (!dragging) return;
      if (rafId.current) {
        cancelAnimationFrame(rafId.current);
        rafId.current = 0;
      }
      const v = vecFromEvent(e);
      setLocalVec(null);
      setDragging(false);
      onVecChange?.(v);
    },
    [dragging, vecFromEvent, onVecChange],
  );

  const handlePointerCancel = React.useCallback(() => {
    if (!dragging) return;
    if (rafId.current) {
      cancelAnimationFrame(rafId.current);
      rafId.current = 0;
    }
    setLocalVec(null);
    setDragging(false);
  }, [dragging]);

  React.useEffect(() => {
    return () => {
      if (rafId.current) cancelAnimationFrame(rafId.current);
    };
  }, []);

  const displayVec = localVec ?? vec;
  const handleX = toSvgX(displayVec.u);
  const handleY = toSvgY(displayVec.v);
  const originX = toSvgX(0);
  const originY = toSvgY(0);
  const originInBounds = minU <= 0 && maxU >= 0 && minV <= 0 && maxV >= 0;

  return (
    <svg
      ref={svgRef}
      data-slot="vec"
      id={id}
      width={size}
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      className={`touch-none select-none ${className}`}
      onPointerDown={handlePointerDown}
      onPointerMove={handlePointerMove}
      onPointerUp={handlePointerUp}
      onPointerCancel={handlePointerCancel}
    >
      <rect x={pad} y={pad} width={inner} height={inner} rx={2} className="fill-muted/40 stroke-muted-foreground/20" strokeWidth={0.5} />
      {showAxes && originInBounds && (
        <>
          <line x1={pad} y1={originY} x2={pad + inner} y2={originY} className="stroke-muted-foreground/40" strokeWidth={0.5} strokeDasharray="2 2" />
          <line x1={originX} y1={pad} x2={originX} y2={pad + inner} className="stroke-muted-foreground/40" strokeWidth={0.5} strokeDasharray="2 2" />
        </>
      )}
      {showOrigin && originInBounds && <circle cx={originX} cy={originY} r={2} className="fill-muted-foreground/60" />}
      {originInBounds && <line x1={originX} y1={originY} x2={handleX} y2={handleY} className="stroke-foreground/50" strokeWidth={1} />}
      <circle data-slot="vec-handle" cx={handleX} cy={handleY} r={dragging ? 6 : 5} className={`fill-foreground cursor-grab active:cursor-grabbing ${dragging ? "" : "transition-all duration-150"}`} />
    </svg>
  );
};

// #endregion 🔖Vec
