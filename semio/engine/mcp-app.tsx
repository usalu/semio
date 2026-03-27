// #region 🔖Header
// [👤semio📚engine💻mcpapp](repo://p/u/semio/b/l/engine/f/mcp-app.tsx)

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Specs: Standalone MCP App entry point. Receives pre-computed diagram data from
// tool results and renders a lightweight UI with diagram + kit artifact selection.
// Summary: MCP App rendering a design diagram plus kit artifact selector.

// #endregion 🔖Header

import * as React from "react";
import { createRoot } from "react-dom/client";
import { useApp, useHostStyles } from "@modelcontextprotocol/ext-apps/react";
import type { App as McpAppInstance } from "@modelcontextprotocol/ext-apps";

// #region 🔖KitArtifactSelect
// Specs: Inlined kit artifact selector for the MCP App. Standalone version without
// @semio/ui dependency to avoid pulling in Three.js, @semio/js, and other heavy deps.
// Summary: Lightweight kit artifact selector component for the MCP App.

interface KitArtifactSelectPort {
  guid: string;
  typeGuid: string;
  id?: string;
  port?: string;
  name?: string;
  description?: string;
  mandatory?: boolean;
}

interface KitArtifactSelectData {
  designs?: Array<{ guid: string; name?: string; variant?: string; view?: string }>;
  types?: Array<{ guid: string; name?: string; variant?: string }>;
  ports?: KitArtifactSelectPort[];
}

interface KitArtifactSelectSelection {
  designGuids?: string[];
  typeGuids?: string[];
  portGuids?: string[];
}

type KitArtifactSelectGroupKind = "design" | "type" | "port";

const KitArtifactSelect: React.FC<{
  data?: KitArtifactSelectData;
  selection?: KitArtifactSelectSelection;
  onSelectionChange?: (selection: KitArtifactSelectSelection) => void;
  title?: string;
  className?: string;
}> = ({ data, selection, onSelectionChange, title = "Kit Artifacts", className }) => {
  const designs = data?.designs ?? [];
  const types = data?.types ?? [];
  const ports = data?.ports ?? [];
  const sel = selection ?? { designGuids: [], typeGuids: [], portGuids: [] };
  const selectedDesignGuids = React.useMemo(() => new Set(sel.designGuids ?? []), [sel.designGuids]);
  const selectedTypeGuids = React.useMemo(() => new Set(sel.typeGuids ?? []), [sel.typeGuids]);
  const selectedPortGuids = React.useMemo(() => new Set(sel.portGuids ?? []), [sel.portGuids]);

  const toggle = React.useCallback(
    (group: KitArtifactSelectGroupKind, guid: string) => {
      if (!onSelectionChange) return;
      const nextDesigns = new Set(sel.designGuids ?? []);
      const nextTypes = new Set(sel.typeGuids ?? []);
      const nextPorts = new Set(sel.portGuids ?? []);
      const target = group === "design" ? nextDesigns : group === "type" ? nextTypes : nextPorts;
      if (target.has(guid)) target.delete(guid);
      else target.add(guid);
      onSelectionChange({
        designGuids: Array.from(nextDesigns),
        typeGuids: Array.from(nextTypes),
        portGuids: Array.from(nextPorts),
      });
    },
    [onSelectionChange, sel.designGuids, sel.typeGuids, sel.portGuids],
  );

  const clear = React.useCallback(() => {
    onSelectionChange?.({ designGuids: [], typeGuids: [], portGuids: [] });
  }, [onSelectionChange]);

  const itemStyle: React.CSSProperties = { display: "flex", alignItems: "center", gap: 8, padding: "6px 10px", borderRadius: 6, border: "1px solid var(--border)" };

  return (
    <div className={className} style={{ display: "flex", flexDirection: "column", gap: 10 }}>
      <div style={{ display: "flex", alignItems: "center", justifyContent: "space-between", gap: 10 }}>
        <div style={{ display: "flex", flexDirection: "column", gap: 2 }}>
          <div style={{ fontWeight: 600 }}>{title}</div>
          <div style={{ fontSize: 12, opacity: 0.7 }}>
            {designs.length} designs · {types.length} types · {ports.length} ports
          </div>
        </div>
        <button type="button" onClick={clear} style={{ padding: "4px 12px", borderRadius: 6, border: "1px solid var(--border)", background: "transparent", cursor: "pointer", fontSize: 13 }}>
          Clear
        </button>
      </div>
      {designs.length > 0 && (
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          <div style={{ fontSize: 12, fontWeight: 600, opacity: 0.8 }}>Designs</div>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: 6 }}>
            {designs.map((d) => (
              <label key={d.guid} style={itemStyle}>
                <input type="checkbox" checked={selectedDesignGuids.has(d.guid)} onChange={() => toggle("design", d.guid)} />
                <div style={{ display: "flex", flexDirection: "column", gap: 2, minWidth: 0 }}>
                  <div style={{ fontSize: 13, fontWeight: 500, whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>{d.name || d.guid}</div>
                  <div style={{ fontSize: 11, opacity: 0.7 }}>
                    {d.variant || "default"} · {d.view || "default"}
                  </div>
                </div>
              </label>
            ))}
          </div>
        </div>
      )}
      {types.length > 0 && (
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          <div style={{ fontSize: 12, fontWeight: 600, opacity: 0.8 }}>Types</div>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: 6 }}>
            {types.map((t) => (
              <label key={t.guid} style={itemStyle}>
                <input type="checkbox" checked={selectedTypeGuids.has(t.guid)} onChange={() => toggle("type", t.guid)} />
                <div style={{ display: "flex", flexDirection: "column", gap: 2, minWidth: 0 }}>
                  <div style={{ fontSize: 13, fontWeight: 500, whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>{t.name || t.guid}</div>
                  <div style={{ fontSize: 11, opacity: 0.7 }}>{t.variant || "default"}</div>
                </div>
              </label>
            ))}
          </div>
        </div>
      )}
      {ports.length > 0 && (
        <div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
          <div style={{ fontSize: 12, fontWeight: 600, opacity: 0.8 }}>Ports</div>
          <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(220px, 1fr))", gap: 6 }}>
            {ports.map((p) => (
              <label key={p.guid} style={itemStyle}>
                <input type="checkbox" checked={selectedPortGuids.has(p.guid)} onChange={() => toggle("port", p.guid)} />
                <div style={{ display: "flex", flexDirection: "column", gap: 2, minWidth: 0 }}>
                  <div style={{ fontSize: 13, fontWeight: 500, whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>{p.name || p.guid}</div>
                  <div style={{ fontSize: 11, opacity: 0.7 }}>
                    {p.port || "default"} · {p.mandatory ? "mandatory" : "optional"}
                  </div>
                </div>
              </label>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

// #endregion 🔖KitArtifactSelect

// #region 🔖DiagramTypes

interface DiagramPoint {
  guid: string;
  id: string;
  u: number;
  v: number;
  status: "default" | "removed" | "added" | "modified";
}

interface DiagramLine {
  guid: string;
  sourceU: number;
  sourceV: number;
  targetU: number;
  targetV: number;
  status: "default" | "removed" | "added" | "modified";
}

interface DiagramPayload {
  points: DiagramPoint[];
  lines: DiagramLine[];
  capabilities?: {
    pieceSelection?: boolean;
    connectionSelection?: boolean;
  };
  kitArtifacts?: KitArtifactSelectData;
}

// #endregion 🔖DiagramTypes

// #region 🔖DiagramRendering

const PADDING = 12;
const PIECE_RADIUS = 1.75;
const STROKE_WIDTH = 1;
const DEFAULT_ZOOM = 1;
const MIN_ZOOM = 1;
const MAX_ZOOM = 12;
const ZOOM_STEP = 0.0015;
const MIN_SPAN = 1;

type EntityStatus = "default" | "removed" | "added" | "modified";

const statusColor = (s: EntityStatus): string => {
  if (s === "removed") return "var(--color-removed)";
  if (s === "added") return "var(--color-new)";
  if (s === "modified") return "var(--color-modified)";
  return "currentColor";
};

const interactiveColor = (s: EntityStatus, selected: boolean, hovered: boolean): string => {
  if (selected) return s === "default" ? "var(--accent)" : "var(--color-changed-selected)";
  if (hovered) return s === "default" ? "var(--accent-secondary)" : "var(--color-changed-hovered)";
  return statusColor(s);
};

const parseMcpToolResult = (result: unknown): DiagramPayload | null => {
  if (!result || typeof result !== "object") return null;
  const r = result as { content?: Array<{ type: string; text?: string }> };
  const textContent = r.content?.find((c) => c.type === "text");
  if (!textContent?.text) return null;
  try {
    const parsed = JSON.parse(textContent.text) as DiagramPayload;
    if (!parsed || typeof parsed !== "object") return null;
    if (!Array.isArray(parsed.points) || !Array.isArray(parsed.lines)) return null;
    return parsed;
  } catch {
    return null;
  }
};

const McpDesignViewer: React.FC = () => {
  const [payload, setPayload] = React.useState<DiagramPayload | null>(null);
  const [selectedPieces, setSelectedPieces] = React.useState<Set<string>>(new Set());
  const [selectedConnections, setSelectedConnections] = React.useState<Set<string>>(new Set());
  const [artifactSelection, setArtifactSelection] = React.useState<KitArtifactSelectSelection>({ designGuids: [], typeGuids: [], portGuids: [] });
  const [hoveredPiece, setHoveredPiece] = React.useState<string | null>(null);
  const [hoveredConnection, setHoveredConnection] = React.useState<string | null>(null);
  const [pan, setPan] = React.useState({ x: 0, y: 0 });
  const [zoom, setZoom] = React.useState(DEFAULT_ZOOM);
  const [size, setSize] = React.useState({ width: 0, height: 0 });
  const containerRef = React.useRef<HTMLDivElement | null>(null);
  const panPointerIdRef = React.useRef<number | null>(null);
  const panOriginRef = React.useRef({ x: 0, y: 0, panX: 0, panY: 0 });
  const [isPanning, setIsPanning] = React.useState(false);
  const appRef = React.useRef<McpAppInstance | null>(null);
  const [debugState, setDebugState] = React.useState("mounting");

  console.error("[DEBUG] McpDesignViewer render, isInIframe:", window.parent !== window);

  const { app, isConnected, error } = useApp({
    appInfo: { name: "semio design viewer", version: "1.0.0" },
    capabilities: {},
    onAppCreated: (a: McpAppInstance) => {
      console.error("[DEBUG] onAppCreated called");
      setDebugState("app-created");
      appRef.current = a;
      a.ontoolresult = (result: unknown) => {
        console.error("[DEBUG] ontoolresult fired, result type:", typeof result);
        console.error("[DEBUG] ontoolresult raw:", JSON.stringify(result).slice(0, 500));
        const parsed = parseMcpToolResult(result);
        console.error("[DEBUG] parseMcpToolResult returned:", parsed ? `points=${parsed.points.length}, lines=${parsed.lines.length}` : "null");
        if (parsed) {
          setPayload(parsed);
          setSelectedPieces(new Set());
          setSelectedConnections(new Set());
          setArtifactSelection({ designGuids: [], typeGuids: [], portGuids: [] });
          setDebugState(`loaded:${parsed.points.length}p/${parsed.lines.length}l`);
        } else {
          setDebugState("parse-failed");
        }
      };
      a.ontoolinput = (params: unknown) => {
        console.error("[DEBUG] ontoolinput fired:", JSON.stringify(params).slice(0, 500));
        setDebugState("tool-input-received");
      };
      a.ontoolcancelled = (params: { reason?: string }) => {
        console.error("[DEBUG] ontoolcancelled fired:", params.reason);
        setDebugState(`cancelled:${params.reason ?? "unknown"}`);
      };
      a.onteardown = async () => ({});
      a.onerror = (err: unknown) => {
        console.error("[DEBUG] app.onerror:", err);
        setDebugState(`error:${err}`);
      };
    },
  });

  // Track connection state changes
  React.useEffect(() => {
    console.error("[DEBUG] useApp state: isConnected=", isConnected, "error=", error?.message, "app=", !!app);
    if (isConnected && app) {
      setDebugState((prev) => (prev === "app-created" ? "connected" : prev));
    }
    if (error) {
      setDebugState(`connect-error:${error.message}`);
    }
  }, [isConnected, error, app]);

  // Apply host theme CSS variables (background, text colors, fonts, etc.)
  useHostStyles(app, app?.getHostContext());

  // Resize observer
  React.useLayoutEffect(() => {
    const el = containerRef.current;
    if (!el) return;
    const update = () => setSize({ width: el.clientWidth, height: el.clientHeight });
    update();
    const obs = new ResizeObserver(update);
    obs.observe(el);
    return () => obs.disconnect();
  }, []);

  // Compute diagram bounds
  const bounds = React.useMemo(() => {
    if (!payload || payload.points.length === 0) return null;
    const us = payload.points.map((p) => p.u);
    const vs = payload.points.map((p) => -p.v);
    const minU = Math.min(...us);
    const maxU = Math.max(...us);
    const minV = Math.min(...vs);
    const maxV = Math.max(...vs);
    return {
      minU,
      maxU,
      minV,
      maxV,
      width: Math.max(maxU - minU, MIN_SPAN),
      height: Math.max(maxV - minV, MIN_SPAN),
    };
  }, [payload]);

  // Fit viewport on first load
  React.useEffect(() => {
    if (!bounds || size.width === 0 || size.height === 0) return;
    const drawW = Math.max(size.width - PADDING * 2, 1);
    const drawH = Math.max(size.height - PADDING * 2, 1);
    const fitZoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, Math.min(drawW / bounds.width, drawH / bounds.height)));

    const scale = Math.min(drawW / bounds.width, drawH / bounds.height);
    const offsetX = (size.width - bounds.width * scale) / 2;
    const offsetY = (size.height - bounds.height * scale) / 2;
    const cx = size.width / 2;
    const cy = size.height / 2;
    const targetCx = offsetX + (bounds.width * scale) / 2;
    const targetCy = offsetY + (bounds.height * scale) / 2;

    setZoom(fitZoom);
    setPan({
      x: -fitZoom * (targetCx - cx),
      y: -fitZoom * (targetCy - cy),
    });
  }, [bounds, size.width, size.height]);

  const sendSelectionUpdate = React.useCallback((pieces: Set<string>, connections: Set<string>) => {
    if (appRef.current) {
      appRef.current.updateModelContext({
        content: [
          {
            type: "text" as const,
            text: JSON.stringify({
              selectionChange: {
                pieceGuids: Array.from(pieces),
                connectionGuids: Array.from(connections),
              },
            }),
          },
        ],
      });
    }
  }, []);

  const sendKitArtifactSelectionUpdate = React.useCallback((next: KitArtifactSelectSelection) => {
    if (!appRef.current) return;
    appRef.current.updateModelContext({
      content: [
        {
          type: "text" as const,
          text: JSON.stringify({
            kitArtifactSelectionChange: {
              designGuids: next.designGuids ?? [],
              typeGuids: next.typeGuids ?? [],
              portGuids: next.portGuids ?? [],
            },
          }),
        },
      ],
    });
  }, []);

  const togglePiece = React.useCallback(
    (guid: string) => {
      setSelectedPieces((prev) => {
        const next = new Set(prev);
        if (next.has(guid)) next.delete(guid);
        else next.add(guid);
        sendSelectionUpdate(next, selectedConnections);
        return next;
      });
    },
    [selectedConnections, sendSelectionUpdate],
  );

  const toggleConnection = React.useCallback(
    (guid: string) => {
      setSelectedConnections((prev) => {
        const next = new Set(prev);
        if (next.has(guid)) next.delete(guid);
        else next.add(guid);
        sendSelectionUpdate(selectedPieces, next);
        return next;
      });
    },
    [selectedPieces, sendSelectionUpdate],
  );

  const clearSelection = React.useCallback(() => {
    setSelectedPieces(new Set());
    setSelectedConnections(new Set());
    sendSelectionUpdate(new Set(), new Set());
  }, [sendSelectionUpdate]);

  const pieceSelectionEnabled = payload?.capabilities?.pieceSelection ?? false;
  const connectionSelectionEnabled = payload?.capabilities?.connectionSelection ?? false;

  const drawW = Math.max(size.width - PADDING * 2, 1);
  const drawH = Math.max(size.height - PADDING * 2, 1);
  const scale = bounds ? Math.min(drawW / bounds.width, drawH / bounds.height) : 1;
  const offsetX = bounds ? (size.width - bounds.width * scale) / 2 : 0;
  const offsetY = bounds ? (size.height - bounds.height * scale) / 2 : 0;
  const cx = size.width / 2;
  const cy = size.height / 2;

  const toPixelX = (u: number) => (bounds ? cx + pan.x + zoom * (offsetX + (u - bounds.minU) * scale - cx) : 0);
  const toPixelY = (v: number) => (bounds ? cy + pan.y + zoom * (offsetY + (-v - bounds.minV) * scale - cy) : 0);

  const handleWheel = (event: React.WheelEvent<HTMLDivElement>) => {
    event.preventDefault();
    if (size.width <= 0 || size.height <= 0) return;
    const nextZoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, zoom * Math.exp(-event.deltaY * ZOOM_STEP)));
    if (Math.abs(nextZoom - zoom) < 0.0001) return;
    const rect = event.currentTarget.getBoundingClientRect();
    const cursorX = event.clientX - rect.left;
    const cursorY = event.clientY - rect.top;
    const baseX = cx + (cursorX - cx - pan.x) / zoom;
    const baseY = cy + (cursorY - cy - pan.y) / zoom;
    setZoom(nextZoom);
    setPan({
      x: cursorX - cx - nextZoom * (baseX - cx),
      y: cursorY - cy - nextZoom * (baseY - cy),
    });
  };

  const handlePointerDown = (event: React.PointerEvent<SVGSVGElement>) => {
    if (event.button !== 0 || event.target !== event.currentTarget) return;
    panPointerIdRef.current = event.pointerId;
    panOriginRef.current = { x: event.clientX, y: event.clientY, panX: pan.x, panY: pan.y };
    setIsPanning(true);
    event.currentTarget.setPointerCapture(event.pointerId);
  };

  const handlePointerMove = (event: React.PointerEvent<SVGSVGElement>) => {
    if (panPointerIdRef.current !== event.pointerId) return;
    setPan({
      x: panOriginRef.current.panX + event.clientX - panOriginRef.current.x,
      y: panOriginRef.current.panY + event.clientY - panOriginRef.current.y,
    });
  };

  const handlePointerEnd = (event: React.PointerEvent<SVGSVGElement>) => {
    if (panPointerIdRef.current !== event.pointerId) return;
    panPointerIdRef.current = null;
    setIsPanning(false);
    if (event.currentTarget.hasPointerCapture(event.pointerId)) {
      event.currentTarget.releasePointerCapture(event.pointerId);
    }
  };

  const handleSvgClick = (event: React.MouseEvent<SVGSVGElement>) => {
    if (event.target === event.currentTarget) clearSelection();
  };

  const handleDoubleClick = () => {
    if (!bounds || size.width === 0 || size.height === 0) return;
    const fitZoom = Math.min(MAX_ZOOM, Math.max(MIN_ZOOM, Math.min(drawW / bounds.width, drawH / bounds.height)));
    const targetCx = offsetX + (bounds.width * scale) / 2;
    const targetCy = offsetY + (bounds.height * scale) / 2;
    setZoom(fitZoom);
    setPan({ x: -fitZoom * (targetCx - cx), y: -fitZoom * (targetCy - cy) });
  };

  // Overlay content for loading/error states (container div always rendered for ResizeObserver)
  let overlayContent: React.ReactNode = null;
  if (error) {
    overlayContent = (
      <div style={{ position: "absolute", inset: 0, display: "flex", alignItems: "center", justifyContent: "center", color: "#dc2626", zIndex: 20 }}>
        <p>Error: {error.message}</p>
      </div>
    );
  } else if (!isConnected || !app) {
    overlayContent = (
      <div style={{ position: "absolute", inset: 0, display: "flex", alignItems: "center", justifyContent: "center", color: "#737373", zIndex: 20 }}>
        <p>Connecting to host…</p>
      </div>
    );
  } else if (!payload || !bounds) {
    overlayContent = (
      <div style={{ position: "absolute", inset: 0, display: "flex", alignItems: "center", justifyContent: "center", color: "#737373", zIndex: 20 }}>
        <p>Waiting for design data…</p>
      </div>
    );
  }

  // [DEBUG] visible state banner
  const debugBanner = (
    <div style={{ position: "absolute", bottom: 4, left: 4, fontSize: 10, fontFamily: "monospace", color: "#888", zIndex: 30, pointerEvents: "none" }}>
      {debugState} | {size.width}x{size.height} | {isConnected ? "connected" : "disconnected"} | {payload ? `${payload.points.length}p/${payload.lines.length}l` : "no-data"}
    </div>
  );

  return (
    <div
      ref={containerRef}
      style={{ width: "100%", height: "100vh", position: "relative", background: "var(--color-background-primary, #ffffff)", color: "var(--color-text-primary, currentColor)" }}
      onDoubleClick={handleDoubleClick}
      onWheel={handleWheel}
    >
      {debugBanner}
      {overlayContent}
      {payload && bounds && (
        <>
          {payload.kitArtifacts && (
            <div style={{ position: "absolute", left: 12, top: 12, right: 12, pointerEvents: "none", zIndex: 10 }}>
              <div style={{ pointerEvents: "auto", maxHeight: "38vh", overflow: "auto" }}>
                <KitArtifactSelect
                  data={payload.kitArtifacts}
                  selection={artifactSelection}
                  onSelectionChange={(next) => {
                    setArtifactSelection(next);
                    sendKitArtifactSelectionUpdate(next);
                  }}
                  title="Kit Artifacts"
                  className="rounded-md border border-border bg-background/90 p-3 backdrop-blur"
                />
              </div>
            </div>
          )}
          <svg
            aria-label="Design Diagram"
            role="img"
            style={{
              width: "100%",
              height: "100%",
              overflow: "visible",
              color: "var(--foreground)",
              cursor: isPanning ? "grabbing" : "grab",
              touchAction: "none",
            }}
            onClick={handleSvgClick}
            onPointerDown={handlePointerDown}
            onPointerMove={handlePointerMove}
            onPointerUp={handlePointerEnd}
            onPointerCancel={handlePointerEnd}
          >
            {payload.lines.map((line) => {
              const sel = selectedConnections.has(line.guid);
              const hov = hoveredConnection === line.guid;
              return (
                <line
                  key={line.guid}
                  x1={toPixelX(line.sourceU)}
                  y1={toPixelY(line.sourceV)}
                  x2={toPixelX(line.targetU)}
                  y2={toPixelY(line.targetV)}
                  stroke={interactiveColor(line.status, sel, hov)}
                  strokeLinecap="round"
                  strokeOpacity={sel || hov ? 1 : line.status === "default" ? 0.45 : 0.8}
                  strokeWidth={(sel ? STROKE_WIDTH + 1.5 : hov ? STROKE_WIDTH + 0.75 : STROKE_WIDTH) * zoom}
                  pointerEvents="stroke"
                  style={{ cursor: connectionSelectionEnabled ? "pointer" : "default" }}
                  onClick={
                    connectionSelectionEnabled
                      ? (e) => {
                          e.stopPropagation();
                          toggleConnection(line.guid);
                        }
                      : undefined
                  }
                  onPointerEnter={() => setHoveredConnection(line.guid)}
                  onPointerLeave={() => setHoveredConnection((prev) => (prev === line.guid ? null : prev))}
                />
              );
            })}
            {payload.points.map((point) => {
              const sel = selectedPieces.has(point.guid);
              const hov = hoveredPiece === point.guid;
              return (
                <circle
                  key={point.guid}
                  cx={toPixelX(point.u)}
                  cy={toPixelY(point.v)}
                  r={(sel ? PIECE_RADIUS + 0.75 : hov ? PIECE_RADIUS + 0.35 : PIECE_RADIUS) * zoom}
                  fill={statusColor(point.status)}
                  stroke={sel || hov ? interactiveColor(point.status, sel, hov) : "none"}
                  strokeWidth={(sel ? 1.5 : hov ? 1 : 0) * zoom}
                  style={{ cursor: pieceSelectionEnabled ? "pointer" : "default" }}
                  onClick={
                    pieceSelectionEnabled
                      ? (e) => {
                          e.stopPropagation();
                          togglePiece(point.guid);
                        }
                      : undefined
                  }
                  onPointerEnter={() => setHoveredPiece(point.guid)}
                  onPointerLeave={() => setHoveredPiece((prev) => (prev === point.guid ? null : prev))}
                />
              );
            })}
          </svg>
        </>
      )}
    </div>
  );
};

// #endregion 🔖DiagramRendering

const rootEl = document.getElementById("root");
if (rootEl) {
  createRoot(rootEl).render(
    <React.StrictMode>
      <McpDesignViewer />
    </React.StrictMode>,
  );
}
