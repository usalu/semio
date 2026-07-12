import { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState } from "react";
import {
  cn,
  ContextMenuController,
  floatingMenuSurfaceClass,
  Icon,
  marqueeCoverageFromGesture,
  marqueeModeFromModifiers,
  SelectionMarquee,
  type ContextMenuItem,
  type IconName,
  type SelectionMarqueeCoverage,
  type SelectionMarqueePoint,
  type SelectionMergeMode,
  type SelectionMarqueeMethod,
} from "@semio-tech/ui-react";
import type { ComponentSceneHostProps, GisMapScene } from "@semio-tech/framework-core";
import type { MapWasmSession } from "../os-shell.tsx";
import { createMapSession } from "../os-shell.tsx";

//#region Types
type MapCamera = { x: number; y: number; zoom: number };

type MapRenderMode = "image" | "vector" | "combined";

type MapVectorStyle = "colored" | "figureGround" | "invertedFigure";

type MapFeatureKind = "position" | "route";

type MapHoveredFeature = { readonly kind: MapFeatureKind; readonly id: string };

type MapFeatureHit = { readonly positions: readonly string[]; readonly routes: readonly string[] };

type MapPositionMeta = {
  readonly id: string;
  readonly label?: string;
  readonly name?: string;
  readonly icon?: IconName;
  readonly sourceUrl?: string;
};

type VisibleTileRow = { z: number; x: number; y: number; key: string };

const DEFAULT_CAMERA_JSON = '{"x":0,"y":0,"zoom":1}';
const MAP_MARQUEE_THRESHOLD_PX = 6;
const MAX_CONCURRENT_TILE_FETCHES = 12;
const TILE_REFRESH_DEBOUNCE_MS = 120;

const MAP_VELLO_THEME_FALLBACK_RGBA = {
  surfaceClear: [12, 28, 33, 255] as [number, number, number, number],
  landFill: [46, 60, 61, 255] as [number, number, number, number],
  landStroke: [51, 64, 65, 107] as [number, number, number, number],
  labelFill: [247, 243, 227, 255] as [number, number, number, number],
  labelHalo: [12, 28, 33, 235] as [number, number, number, number],
  regionFill: [52, 209, 191, 56] as [number, number, number, number],
  regionStroke: [52, 209, 191, 230] as [number, number, number, number],
  routeStroke: [250, 149, 0, 235] as [number, number, number, number],
  positionFill: [255, 52, 79, 255] as [number, number, number, number],
  positionStroke: [247, 243, 227, 255] as [number, number, number, number],
  selectionStroke: [255, 52, 79, 255] as [number, number, number, number],
  hoverStroke: [52, 209, 191, 235] as [number, number, number, number],
};
//#endregion Types

//#region Parsing
function parseVisibleTilesJson(raw: string): VisibleTileRow[] {
  try {
    const rows = JSON.parse(raw) as VisibleTileRow[];
    return Array.isArray(rows) ? rows : [];
  } catch {
    return [];
  }
}

function parseCameraJson(raw: string): MapCamera | null {
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number; zoom?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number" || typeof v.zoom !== "number") return null;
    return { x: v.x, y: v.y, zoom: v.zoom };
  } catch {
    return null;
  }
}

function parseMapFeatureHit(raw: string): MapFeatureHit {
  try {
    const v = JSON.parse(raw) as { positions?: string[]; routes?: string[] };
    const positions = Array.isArray(v.positions) ? v.positions.filter((id): id is string => typeof id === "string") : [];
    const routes = Array.isArray(v.routes) ? v.routes.filter((id): id is string => typeof id === "string") : [];
    return { positions, routes };
  } catch {
    return { positions: [], routes: [] };
  }
}

function parseMapHoveredFeature(raw: string): MapHoveredFeature | null {
  if (raw === "null") return null;
  try {
    const v = JSON.parse(raw) as { kind?: string; id?: string };
    if ((v.kind === "position" || v.kind === "route") && typeof v.id === "string") {
      return { kind: v.kind, id: v.id };
    }
  } catch {
    return null;
  }
  return null;
}

function parseMapPositionScreen(raw: string): { x: number; y: number } | null {
  if (raw === "null") return null;
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number") return null;
    return { x: v.x, y: v.y };
  } catch {
    return null;
  }
}

function parsePositionMeta(mapFixtureJson: string): Map<string, MapPositionMeta> {
  try {
    const descriptor = JSON.parse(mapFixtureJson) as {
      positions?: Array<{ id?: string; label?: string; name?: string; icon?: string; source_url?: string; sourceUrl?: string }>;
    };
    const out = new Map<string, MapPositionMeta>();
    for (const row of descriptor.positions ?? []) {
      if (typeof row.id !== "string") continue;
      out.set(row.id, {
        id: row.id,
        label: row.label,
        name: row.name,
        icon: row.icon as IconName | undefined,
        sourceUrl: row.source_url ?? row.sourceUrl,
      });
    }
    return out;
  } catch {
    return new Map();
  }
}

function parseFeatureSelection(raw: string): { positions: string[]; routes: string[] } {
  try {
    const v = JSON.parse(raw) as { positions?: string[]; routes?: string[] };
    return {
      positions: Array.isArray(v.positions) ? v.positions.filter((id): id is string => typeof id === "string") : [],
      routes: Array.isArray(v.routes) ? v.routes.filter((id): id is string => typeof id === "string") : [],
    };
  } catch {
    return { positions: [], routes: [] };
  }
}

function screenRectFromPoints(points: readonly SelectionMarqueePoint[]): { x: number; y: number; width: number; height: number } | null {
  if (!points.length) return null;
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const minX = Math.min(...xs);
  const minY = Math.min(...ys);
  const maxX = Math.max(...xs);
  const maxY = Math.max(...ys);
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}

function getGisMapCameraLimits(session?: MapWasmSession): { min: number; max: number } {
  if (session) {
    return JSON.parse(session.cameraLimitsJson()) as { min: number; max: number };
  }
  return { min: 0.05, max: 64 };
}
//#endregion Parsing

//#region MapTheme
function mapParseCssColorToRgba8888(css: string, fallback: [number, number, number, number]): [number, number, number, number] {
  const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)(?:\s*,\s*([\d.]+%?)\s*)?\)/u);
  if (!m) return fallback;
  const r = Math.min(255, Math.max(0, Math.round(Number(m[1]))));
  const g = Math.min(255, Math.max(0, Math.round(Number(m[2]))));
  const b = Math.min(255, Math.max(0, Math.round(Number(m[3]))));
  let a = 255;
  if (m[4] !== undefined && m[4] !== "") {
    const raw = m[4];
    if (raw.endsWith("%")) {
      a = Math.min(255, Math.max(0, Math.round((Number(raw.slice(0, -1)) / 100) * 255)));
    } else {
      const n = Number(raw);
      a = Math.min(255, Math.max(0, Math.round(n <= 1 ? n * 255 : n)));
    }
  }
  return [r, g, b, a];
}

function mapProbeCssComputed(property: "color" | "backgroundColor", value: string): string {
  if (typeof document === "undefined") return "";
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (document.documentElement.classList.contains("dark")) el.classList.add("dark");
  document.documentElement.appendChild(el);
  const out = getComputedStyle(el)[property];
  el.remove();
  return out;
}

function serializeMapCanvasThemeJson(): string {
  const fb = MAP_VELLO_THEME_FALLBACK_RGBA;
  const pc = (prop: "color" | "backgroundColor", expr: string, fall: [number, number, number, number]): number[] => {
    const raw = mapProbeCssComputed(prop, expr);
    return [...mapParseCssColorToRgba8888(raw, fall)];
  };
  return JSON.stringify({
    surfaceClear: pc("backgroundColor", "var(--canvas)", fb.surfaceClear),
    landFill: pc("backgroundColor", "color-mix(in oklab, var(--color-muted-foreground) 32%, var(--color-canvas))", fb.landFill),
    landStroke: pc("color", "color-mix(in oklab, var(--color-muted-foreground) 42%, transparent)", fb.landStroke),
    labelFill: pc("color", "var(--foreground)", fb.labelFill),
    labelHalo: pc("backgroundColor", "var(--canvas)", fb.labelHalo),
    regionFill: pc("backgroundColor", "color-mix(in oklab, var(--color-secondary) 22%, transparent)", fb.regionFill),
    regionStroke: pc("color", "var(--color-secondary)", fb.regionStroke),
    routeStroke: pc("color", "var(--color-tertiary)", fb.routeStroke),
    positionFill: pc("backgroundColor", "var(--color-active-base)", fb.positionFill),
    positionStroke: pc("color", "var(--color-active-foreground)", fb.positionStroke),
    selectionStroke: pc("color", "var(--color-active-base)", fb.selectionStroke),
    hoverStroke: pc("color", "var(--color-secondary)", fb.hoverStroke),
  });
}
//#endregion MapTheme

//#region MapRenderer
class MapRenderer {
  readonly session: MapWasmSession;
  camera: MapCamera = { x: 0, y: 0, zoom: 1 };
  private raf = 0;
  private disposed = false;
  private canvasEl: HTMLCanvasElement | null = null;
  private tileCache = new Map<string, ArrayBuffer>();
  private vectorTileCache = new Map<string, ArrayBuffer>();
  private tileMiss = new Set<string>();
  private vectorTileMiss = new Set<string>();
  private tileUrlTemplate: string;
  private vectorTileUrlTemplate: string;
  private renderMode: MapRenderMode = "vector";
  private refreshTimer: ReturnType<typeof setTimeout> | null = null;
  private refreshInFlight: Promise<void> | null = null;
  private lastRasterVisibleKey = "";
  private lastVectorVisibleKey = "";
  private lastPolledRasterVisibleKey = "";
  private lastPolledVectorVisibleKey = "";
  private tilesRefreshQueued = false;
  private lastMapThemeJson = "";
  private logicalWidth = 1;
  private logicalHeight = 1;
  private dpr = 1;

  constructor(tileUrlTemplate: string, vectorTileUrlTemplate: string, session: MapWasmSession) {
    this.session = session;
    this.tileUrlTemplate = tileUrlTemplate;
    this.vectorTileUrlTemplate = vectorTileUrlTemplate;
  }

  private applyCanvasPixelSize(lw: number, lh: number, nextDpr: number): void {
    const canvas = this.canvasEl;
    if (!canvas) return;
    const pw = Math.max(1, Math.round(lw * nextDpr));
    const ph = Math.max(1, Math.round(lh * nextDpr));
    if (canvas.width !== pw || canvas.height !== ph) {
      canvas.width = pw;
      canvas.height = ph;
    }
  }

  setRenderMode(mode: MapRenderMode): void {
    this.renderMode = mode;
    this.session.setRenderMode(mode);
  }

  setVectorStyle(style: MapVectorStyle): void {
    this.session.setVectorStyle(style);
  }

  setLayerVisibilityJson(json: string): void {
    this.session.setLayerVisibilityJson(json);
  }

  setLayerStrokeScaleJson(json: string): void {
    this.session.setLayerStrokeScaleJson(json);
  }

  setLodMode(mode: string): void {
    this.session.setLodMode(mode);
    this.tileCache.clear();
    this.vectorTileCache.clear();
    this.tileMiss.clear();
    this.vectorTileMiss.clear();
    this.lastRasterVisibleKey = "";
    this.lastVectorVisibleKey = "";
    this.lastPolledRasterVisibleKey = "";
    this.lastPolledVectorVisibleKey = "";
    this.scheduleRefreshTiles();
  }

  async attach(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): Promise<void> {
    this.canvasEl = canvas;
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = dpr > 0 ? dpr : 1;
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    await this.session.attachCanvas(canvas, lw, lh, nextDpr);
  }

  setSize(width: number, height: number, dpr: number): boolean {
    const lw = Math.max(1, Math.round(width));
    const lh = Math.max(1, Math.round(height));
    const nextDpr = dpr > 0 ? dpr : 1;
    if (!this.canvasEl) return false;
    if (lw === this.logicalWidth && lh === this.logicalHeight && nextDpr === this.dpr) return false;
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    this.session.setSize(lw, lh, nextDpr);
    this.session.reclampCamera();
    const parsed = this.readCameraFromSession();
    if (parsed) this.camera = parsed;
    return true;
  }

  syncDescriptor(json: string): void {
    this.session.syncMapJson(json);
  }

  readCameraFromSession(): MapCamera | null {
    return parseCameraJson(this.session.cameraJson());
  }

  applyCameraToSession(camera: MapCamera): void {
    this.camera = camera;
    this.session.setCamera(camera.x, camera.y, camera.zoom);
  }

  private needsRasterTiles(): boolean {
    return this.renderMode === "image" || this.renderMode === "combined";
  }

  private needsVectorTiles(): boolean {
    return this.renderMode === "vector" || this.renderMode === "combined";
  }

  scheduleRefreshTiles(): void {
    if (this.disposed || !this.canvasEl) return;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = null;
      void this.refreshTiles();
    }, TILE_REFRESH_DEBOUNCE_MS);
  }

  async refreshTiles(): Promise<void> {
    if (this.disposed || !this.canvasEl) return;
    if (this.refreshInFlight) {
      this.tilesRefreshQueued = true;
      return this.refreshInFlight;
    }
    this.refreshInFlight = (async () => {
      const tasks: Promise<void>[] = [];
      if (this.needsRasterTiles()) tasks.push(this.refreshRasterTiles());
      if (this.needsVectorTiles()) tasks.push(this.refreshVectorTiles());
      await Promise.all(tasks);
    })().finally(() => {
      this.refreshInFlight = null;
      if (this.tilesRefreshQueued) {
        this.tilesRefreshQueued = false;
        void this.refreshTiles();
      }
    });
    return this.refreshInFlight;
  }

  private pollVisibleTilesForRefresh(): void {
    if (!this.canvasEl || !this.session.gpuReady()) return;
    if (this.needsRasterTiles()) {
      const rasterKey = this.session.visibleTilesJson();
      if (rasterKey !== this.lastPolledRasterVisibleKey) {
        this.lastPolledRasterVisibleKey = rasterKey;
        this.scheduleRefreshTiles();
      }
    }
    if (this.needsVectorTiles()) {
      const vectorKey = this.session.visibleVectorTilesJson();
      if (vectorKey !== this.lastPolledVectorVisibleKey) {
        this.lastPolledVectorVisibleKey = vectorKey;
        this.scheduleRefreshTiles();
      }
    }
  }

  private async refreshRasterTiles(): Promise<void> {
    if (this.disposed) return;
    const visibleKey = this.session.visibleTilesJson();
    const rows = parseVisibleTilesJson(visibleKey);
    if (rows.length === 0) return;
    if (visibleKey !== this.lastRasterVisibleKey) {
      this.lastRasterVisibleKey = visibleKey;
      this.tileMiss.clear();
    }
    const uploadOne = async (row: VisibleTileRow): Promise<void> => {
      const key = row.key;
      let buf = this.tileCache.get(key);
      if (!buf) {
        if (this.tileMiss.has(key)) return;
        const url = this.tileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const res = await fetch(url);
        if (!res.ok) {
          this.tileMiss.add(key);
          return;
        }
        buf = await res.arrayBuffer();
        this.tileCache.set(key, buf);
      }
      if (this.disposed) return;
      this.session.uploadTile(row.z, row.x, row.y, new Uint8Array(buf));
    };
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => uploadOne(row)));
    }
  }

  async refreshVectorTiles(): Promise<void> {
    if (this.disposed) return;
    const visibleKey = this.session.visibleVectorTilesJson();
    const rows = parseVisibleTilesJson(visibleKey);
    if (rows.length === 0) return;
    if (visibleKey !== this.lastVectorVisibleKey) {
      this.lastVectorVisibleKey = visibleKey;
      this.vectorTileMiss.clear();
    }
    const uploadOne = async (row: VisibleTileRow): Promise<void> => {
      const key = row.key;
      let buf = this.vectorTileCache.get(key);
      if (!buf) {
        if (this.vectorTileMiss.has(key)) return;
        const url = this.vectorTileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const res = await fetch(url);
        if (!res.ok) {
          this.vectorTileMiss.add(key);
          return;
        }
        buf = await res.arrayBuffer();
        this.vectorTileCache.set(key, buf);
      }
      if (this.disposed) return;
      this.session.uploadVectorTile(row.z, row.x, row.y, new Uint8Array(buf));
    };
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => uploadOne(row)));
    }
  }

  private syncMapThemeFromDocument(): void {
    if (typeof document === "undefined") return;
    try {
      const json = serializeMapCanvasThemeJson();
      if (json !== this.lastMapThemeJson) {
        this.lastMapThemeJson = json;
        this.session.setMapThemeJson(json);
      }
    } catch {
      this.lastMapThemeJson = "";
    }
  }

  startLoop(): void {
    const tick = () => {
      if (this.disposed) return;
      this.syncMapThemeFromDocument();
      this.pollVisibleTilesForRefresh();
      void this.session.renderFrame();
      this.raf = requestAnimationFrame(tick);
    };
    this.raf = requestAnimationFrame(tick);
  }

  stopLoop(): void {
    if (this.raf) {
      cancelAnimationFrame(this.raf);
      this.raf = 0;
    }
  }

  dispose(): void {
    if (this.disposed) return;
    this.disposed = true;
    if (this.refreshTimer !== null) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = null;
    }
    this.stopLoop();
    this.session.free();
    this.canvasEl = null;
  }
}
//#endregion MapRenderer

//#region ContextMenu
function buildGisMapContextMenuItems(scene: GisMapScene, feature: MapHoveredFeature | null, dispatch: (action: string, args?: Record<string, unknown>) => void): ContextMenuItem[] {
  const selection = parseFeatureSelection(scene.selectionJson);
  if (feature) {
    const selected = feature.kind === "position" ? selection.positions.includes(feature.id) : selection.routes.includes(feature.id);
    const items: ContextMenuItem[] = [
      {
        id: "gis-map.ctx.select",
        label: "Select",
        onSelect: () =>
          dispatch("setFeatureSelection", {
            positions: feature.kind === "position" ? [feature.id] : [],
            routes: feature.kind === "route" ? [feature.id] : [],
            mode: "default",
          }),
      },
    ];
    if (selected) {
      items.push({
        id: "gis-map.ctx.deselect",
        label: "Deselect",
        onSelect: () => dispatch("deselect", { featureId: feature.id, featureKind: feature.kind }),
      });
    }
    items.push({
      id: "gis-map.ctx.focus",
      label: "Focus / zoom to",
      onSelect: () => dispatch("focusFeature", { featureId: feature.id, featureKind: feature.kind }),
    });
    if (feature.kind === "position") {
      const meta = parsePositionMeta(scene.mapFixtureJson).get(feature.id);
      if (meta?.sourceUrl) {
        items.push({
          id: "gis-map.ctx.source",
          label: "Open source",
          onSelect: () => dispatch("openSource", { featureId: feature.id }),
        });
      }
    }
    return items;
  }
  return [
    { id: "gis-map.ctx.select-all", label: "Select all", onSelect: () => dispatch("selectAll") },
    {
      id: "gis-map.ctx.clear",
      label: "Clear selection",
      disabled: selection.positions.length + selection.routes.length === 0,
      onSelect: () => dispatch("clearSelection"),
    },
    { id: "gis-map.ctx.fit-world", label: "Fit world", onSelect: () => dispatch("fitWorld") },
  ];
}
//#endregion ContextMenu

//#region GisMapHost
export function GisMapHost({ node, onAction }: ComponentSceneHostProps) {
  const scene = node.gisMap;
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<MapRenderer | null>(null);
  const panningRef = useRef(false);
  const userAdjustedCameraRef = useRef(false);
  const popupRef = useRef<HTMLDivElement>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<
    { coverage: SelectionMarqueeCoverage; shape: "rect"; rect: { x: number; y: number; width: number; height: number } } | { coverage: SelectionMarqueeCoverage; shape: "polygon"; points: readonly SelectionMarqueePoint[] } | null
  >(null);
  const [contextMenu, setContextMenu] = useState<{ open: boolean; position: { x: number; y: number } | null; items: ContextMenuItem[] }>({
    open: false,
    position: null,
    items: [],
  });

  const positionMetaById = useMemo(() => (scene ? parsePositionMeta(scene.mapFixtureJson) : new Map()), [scene?.mapFixtureJson]);
  const hoveredFeature = useMemo(() => (scene ? parseMapHoveredFeature(scene.hoverJson) : null), [scene?.hoverJson]);
  const selectionMethod = (scene?.selectionMethod ?? "rectangle") as SelectionMarqueeMethod;

  const dispatch = useCallback(
    (action: string, args?: Record<string, unknown>) => {
      onAction({ controllerId: node.controllerId, action, args: { surfaceId: node.surfaceId, ...args } });
    },
    [node.controllerId, node.surfaceId, onAction],
  );

  const dispatchCamera = useCallback(
    (camera: MapCamera) => {
      dispatch("setCamera", { camera });
    },
    [dispatch],
  );

  const clampMapZoom = useCallback((zoom: number): number => {
    const { min, max } = getGisMapCameraLimits(rendererRef.current?.session);
    return Math.min(max, Math.max(min, zoom));
  }, []);

  const clampCamera = useCallback((next: MapCamera): MapCamera => ({ x: next.x, y: next.y, zoom: clampMapZoom(next.zoom) }), [clampMapZoom]);

  const mirrorSessionCameraToReact = useCallback(() => {
    const parsed = rendererRef.current?.readCameraFromSession();
    if (!parsed) return;
    rendererRef.current!.camera = parsed;
    dispatchCamera(parsed);
  }, [dispatchCamera]);

  const clientToLocal = useCallback((clientX: number, clientY: number): SelectionMarqueePoint => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) return { x: 0, y: 0 };
    return { x: clientX - rect.left, y: clientY - rect.top };
  }, []);

  const queryFeatureHits = useCallback(
    (points: readonly SelectionMarqueePoint[], crossing: boolean): MapFeatureHit => {
      const session = rendererRef.current?.session;
      if (!session) return { positions: [], routes: [] };
      if (selectionMethod === "lasso" && points.length >= 3) {
        return parseMapFeatureHit(session.featuresInPolygonJson(JSON.stringify(points.map((point) => [point.x, point.y])), crossing));
      }
      const rect = screenRectFromPoints(points);
      if (!rect) return { positions: [], routes: [] };
      return parseMapFeatureHit(session.featuresInRectJson(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height, crossing));
    },
    [selectionMethod],
  );

  const queryHitFeature = useCallback((point: SelectionMarqueePoint): MapHoveredFeature | null => {
    const session = rendererRef.current?.session;
    if (!session) return null;
    return parseMapHoveredFeature(session.hitTestFeatureJson(point.x, point.y));
  }, []);

  const resolveMapPaneElement = useCallback((container: HTMLElement): HTMLElement => {
    let el: HTMLElement | null = container;
    while (el) {
      const slot = el.dataset.slot;
      if (slot === "window" || slot === "mode-dock-stack-body") return el;
      el = el.parentElement;
    }
    return container;
  }, []);

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) return { w: 1, h: 1 };
    const pane = resolveMapPaneElement(container);
    const rect = pane.getBoundingClientRect();
    const style = globalThis.getComputedStyle(pane);
    const padX = Number.parseFloat(style.paddingLeft) + Number.parseFloat(style.paddingRight);
    const padY = Number.parseFloat(style.paddingTop) + Number.parseFloat(style.paddingBottom);
    const innerW = rect.width - (Number.isFinite(padX) ? padX : 0);
    const innerH = rect.height - (Number.isFinite(padY) ? padY : 0);
    return {
      w: Math.max(1, Math.round(innerW || pane.clientWidth || container.clientWidth)),
      h: Math.max(1, Math.round(innerH || pane.clientHeight || container.clientHeight)),
    };
  }, [resolveMapPaneElement]);

  const mirrorSessionCameraToReactRef = useRef(mirrorSessionCameraToReact);
  mirrorSessionCameraToReactRef.current = mirrorSessionCameraToReact;
  const dispatchCameraRef = useRef(dispatchCamera);
  dispatchCameraRef.current = dispatchCamera;

  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container || !scene) return;
    let disposed = false;
    let resizeRafId: number | null = null;
    let resizeObserver: ResizeObserver | null = null;
    const dpr = globalThis.devicePixelRatio || 1;

    void createMapSession().then((session) => {
      if (disposed) {
        session.free();
        return;
      }
      const renderer = new MapRenderer(scene.tileUrlTemplate, scene.vectorTileUrlTemplate, session);
      renderer.setRenderMode(scene.renderMode as MapRenderMode);
      renderer.setVectorStyle(scene.vectorStyle as MapVectorStyle);
      renderer.setLodMode(scene.lodMode);
      renderer.setLayerVisibilityJson(scene.layerVisibilityJson);
      renderer.setLayerStrokeScaleJson(scene.layerStrokeScaleJson);
      rendererRef.current = renderer;

      const applySize = (): void => {
        const nextDpr = globalThis.devicePixelRatio || 1;
        const { w, h } = readContainerSize();
        if (!renderer.setSize(w, h, nextDpr)) return;
        mirrorSessionCameraToReactRef.current();
        renderer.scheduleRefreshTiles();
      };

      resizeObserver =
        typeof ResizeObserver === "undefined"
          ? null
          : new ResizeObserver(() => {
              if (resizeRafId !== null) return;
              const schedule =
                typeof globalThis.requestAnimationFrame === "function"
                  ? (fn: () => void) => {
                      resizeRafId = globalThis.requestAnimationFrame(() => {
                        resizeRafId = null;
                        fn();
                      });
                    }
                  : (fn: () => void) => {
                      queueMicrotask(fn);
                    };
              schedule(() => {
                if (disposed) return;
                applySize();
              });
            });
      const pane = resolveMapPaneElement(container);
      resizeObserver?.observe(pane);
      if (pane !== container) resizeObserver?.observe(container);

      const boot = async (): Promise<void> => {
        let { w, h } = readContainerSize();
        for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
          await new Promise<void>((resolve) => {
            if (typeof globalThis.requestAnimationFrame === "function") globalThis.requestAnimationFrame(() => resolve());
            else queueMicrotask(resolve);
          });
          if (disposed) return;
          ({ w, h } = readContainerSize());
        }
        await renderer.attach(canvas, w, h, dpr);
        if (disposed) {
          renderer.dispose();
          return;
        }
        applySize();
        if (!userAdjustedCameraRef.current) {
          if (scene.cameraJson === DEFAULT_CAMERA_JSON) {
            renderer.session.fitWorldCamera();
          } else {
            const bootCamera = parseCameraJson(scene.cameraJson);
            if (bootCamera) renderer.applyCameraToSession(clampCamera(bootCamera));
          }
          const bootCamera = renderer.readCameraFromSession();
          if (bootCamera) {
            renderer.applyCameraToSession(bootCamera);
            dispatchCameraRef.current(bootCamera);
          }
        }
        renderer.syncDescriptor(scene.mapFixtureJson);
        renderer.session.setSelectionJson(scene.selectionJson);
        renderer.session.setHoverJson(scene.hoverJson);
        await renderer.refreshTiles();
        renderer.startLoop();
      };

      void boot();
    });

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (resizeRafId !== null && typeof globalThis.cancelAnimationFrame === "function") {
        globalThis.cancelAnimationFrame(resizeRafId);
      }
      rendererRef.current?.dispose();
      rendererRef.current = null;
    };
  }, [clampCamera, readContainerSize, resolveMapPaneElement, scene?.tileUrlTemplate, scene?.vectorTileUrlTemplate]);

  useEffect(() => {
    if (!scene) return;
    const renderer = rendererRef.current;
    if (!renderer) return;
    renderer.setRenderMode(scene.renderMode as MapRenderMode);
    renderer.scheduleRefreshTiles();
  }, [scene?.renderMode]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setVectorStyle(scene.vectorStyle as MapVectorStyle);
  }, [scene?.vectorStyle]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLodMode(scene.lodMode);
  }, [scene?.lodMode]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLayerVisibilityJson(scene.layerVisibilityJson);
  }, [scene?.layerVisibilityJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.setLayerStrokeScaleJson(scene.layerStrokeScaleJson);
  }, [scene?.layerStrokeScaleJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.syncDescriptor(scene.mapFixtureJson);
    rendererRef.current?.scheduleRefreshTiles();
  }, [scene?.mapFixtureJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.session.setSelectionJson(scene.selectionJson);
  }, [scene?.selectionJson]);

  useEffect(() => {
    if (!scene) return;
    rendererRef.current?.session.setHoverJson(scene.hoverJson);
  }, [scene?.hoverJson]);

  useEffect(() => {
    if (!scene || panningRef.current) return;
    const camera = parseCameraJson(scene.cameraJson);
    if (!camera) return;
    rendererRef.current?.applyCameraToSession(clampCamera(camera));
    rendererRef.current?.scheduleRefreshTiles();
  }, [clampCamera, scene?.cameraJson]);

  useEffect(() => {
    if (!hoveredFeature || hoveredFeature.kind !== "position") return undefined;
    let raf = 0;
    const tick = () => {
      const screen = rendererRef.current?.session.featureScreenJson("position", hoveredFeature.id);
      const parsed = parseMapPositionScreen(screen ?? "null");
      const popup = popupRef.current;
      if (parsed && popup) {
        popup.style.left = `${parsed.x}px`;
        popup.style.top = `${parsed.y}px`;
      }
      raf = requestAnimationFrame(tick);
    };
    raf = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(raf);
  }, [hoveredFeature]);

  const applyWheelZoom = useCallback(
    (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const r = rendererRef.current;
      const canvas = canvasRef.current;
      if (!r || !canvas) return;
      const rect = canvas.getBoundingClientRect();
      let deltaY = event.deltaY * (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      if (event.ctrlKey) deltaY *= 2.5;
      userAdjustedCameraRef.current = true;
      r.session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, deltaY);
      mirrorSessionCameraToReact();
      r.scheduleRefreshTiles();
    },
    [mirrorSessionCameraToReact],
  );

  useEffect(() => {
    const element = containerRef.current;
    if (!element) return undefined;
    element.addEventListener("wheel", applyWheelZoom, { passive: false });
    return () => element.removeEventListener("wheel", applyWheelZoom);
  }, [applyWheelZoom]);

  const pointer = useRef({
    leftDown: false,
    middleDown: false,
    marqueeTracking: false,
    marqueeActive: false,
    start: { x: 0, y: 0 } as SelectionMarqueePoint,
    points: [] as SelectionMarqueePoint[],
  });

  const resetMarquee = useCallback(() => {
    pointer.current.marqueeTracking = false;
    pointer.current.marqueeActive = false;
    pointer.current.points = [];
    setMarqueeOverlay(null);
  }, []);

  const emitFeatureSelection = useCallback(
    (hits: MapFeatureHit, mode: SelectionMergeMode, crossing: boolean) => {
      dispatch("setFeatureSelection", {
        positions: [...hits.positions],
        routes: [...hits.routes],
        mode,
        crossing,
      });
    },
    [dispatch],
  );

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas || !scene) return undefined;
    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 0) {
        pointer.current.leftDown = true;
        pointer.current.marqueeTracking = true;
        pointer.current.marqueeActive = false;
        pointer.current.start = point;
        pointer.current.points = [point];
        canvas.setPointerCapture?.(event.pointerId);
        return;
      }
      if (event.button === 1) {
        event.preventDefault();
        pointer.current.middleDown = true;
        panningRef.current = true;
        userAdjustedCameraRef.current = true;
        canvas.setPointerCapture?.(event.pointerId);
        rendererRef.current?.session.pointerDownScreen(point.x, point.y, 1);
      }
    };
    const onPointerMove = (event: PointerEvent): void => {
      const point = clientToLocal(event.clientX, event.clientY);
      if (pointer.current.middleDown) {
        event.stopPropagation();
        rendererRef.current?.session.pointerMoveScreen(point.x, point.y);
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (!pointer.current.marqueeTracking) {
        const hit = queryHitFeature(point);
        const nextHover = hit ? { kind: hit.kind, id: hit.id } : null;
        const currentHover = parseMapHoveredFeature(scene.hoverJson);
        if ((currentHover?.id ?? null) !== (nextHover?.id ?? null) || (currentHover?.kind ?? null) !== (nextHover?.kind ?? null)) {
          dispatch("setHover", { hover: nextHover });
        }
        return;
      }
      event.stopPropagation();
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      if (!pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        pointer.current.marqueeActive = true;
      }
      if (!pointer.current.marqueeActive) return;
      const method = selectionMethod;
      const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
      pointer.current.points = points;
      const coverage = marqueeCoverageFromGesture({
        method,
        startX: pointer.current.start.x,
        endX: point.x,
        path: points,
      });
      const rect = screenRectFromPoints(points);
      setMarqueeOverlay(method === "lasso" ? { coverage, shape: "polygon", points } : { coverage, shape: "rect", rect: rect ?? { x: 0, y: 0, width: 0, height: 0 } });
    };
    const onPointerUp = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 1 && pointer.current.middleDown) {
        pointer.current.middleDown = false;
        panningRef.current = false;
        rendererRef.current?.session.pointerUpScreen(point.x, point.y);
        if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (event.button !== 0 || !pointer.current.leftDown) return;
      pointer.current.leftDown = false;
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      const mode = marqueeModeFromModifiers(event);
      const method = selectionMethod;
      if (pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
        const coverage = marqueeCoverageFromGesture({
          method,
          startX: pointer.current.start.x,
          endX: point.x,
          path: points,
        });
        emitFeatureSelection(queryFeatureHits(points, coverage === "partial"), mode, coverage === "partial");
      } else if (distance < MAP_MARQUEE_THRESHOLD_PX) {
        const hit = queryHitFeature(point);
        emitFeatureSelection(
          {
            positions: hit?.kind === "position" ? [hit.id] : [],
            routes: hit?.kind === "route" ? [hit.id] : [],
          },
          mode,
          false,
        );
      }
      resetMarquee();
    };
    const onPointerCancel = (event: PointerEvent): void => {
      pointer.current.leftDown = false;
      pointer.current.middleDown = false;
      panningRef.current = false;
      resetMarquee();
      if (canvas.hasPointerCapture?.(event.pointerId)) canvas.releasePointerCapture(event.pointerId);
      mirrorSessionCameraToReact();
    };
    const onContextMenu = (event: MouseEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      const feature = queryHitFeature(point);
      const items = buildGisMapContextMenuItems(scene, feature, dispatch);
      setContextMenu({ open: items.length > 0, position: { x: event.clientX, y: event.clientY }, items });
    };
    canvas.addEventListener("pointerdown", onPointerDown);
    window.addEventListener("pointermove", onPointerMove);
    window.addEventListener("pointerup", onPointerUp);
    window.addEventListener("pointercancel", onPointerCancel);
    canvas.addEventListener("contextmenu", onContextMenu);
    return () => {
      canvas.removeEventListener("pointerdown", onPointerDown);
      window.removeEventListener("pointermove", onPointerMove);
      window.removeEventListener("pointerup", onPointerUp);
      window.removeEventListener("pointercancel", onPointerCancel);
      canvas.removeEventListener("contextmenu", onContextMenu);
    };
  }, [clientToLocal, dispatch, emitFeatureSelection, mirrorSessionCameraToReact, queryFeatureHits, queryHitFeature, resetMarquee, scene, selectionMethod]);

  if (!scene) return <div className="semio-gis-map-empty text-muted-foreground p-2 text-xs">No map scene</div>;

  return (
    <div ref={containerRef} className="semio-gis-map-host absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none" data-surface-id={node.surfaceId} style={{ touchAction: "none" }}>
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      {marqueeOverlay?.shape === "rect" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} /> : null}
      {marqueeOverlay?.shape === "polygon" ? <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} /> : null}
      <ContextMenuController open={contextMenu.open} position={contextMenu.position} items={contextMenu.items} onOpenChange={(open) => setContextMenu((prev) => ({ ...prev, open }))} />
      {hoveredFeature?.kind === "position" ? (
        <div ref={popupRef} className={cn("pointer-events-none absolute z-10 max-w-56 -translate-x-1/2 -translate-y-[calc(100%+12px)] px-2 py-1.5", floatingMenuSurfaceClass)} style={{ left: 0, top: 0 }}>
          {(() => {
            const meta = positionMetaById.get(hoveredFeature.id);
            const title = meta?.name ?? meta?.label ?? hoveredFeature.id;
            return (
              <div className="flex items-start gap-1.5">
                {meta?.icon ? <Icon icon={meta.icon} size="small" className="mt-0.5 shrink-0" /> : null}
                <div className="min-w-0">
                  <div className="truncate text-sm font-medium">{title}</div>
                  {meta?.sourceUrl ? <span className="text-xs text-secondary underline-offset-2">Source available</span> : null}
                </div>
              </div>
            );
          })()}
        </div>
      ) : null}
    </div>
  );
}
//#endregion GisMapHost
