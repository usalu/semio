// #region 🧲Header
/** @emoji 🗺️ GIS map React host: WASM {@link MapSession}, tile fetch, {@link MapCanvas}. */
// #endregion 🧲Header

// #region 🔌Adapters
import React, { useCallback, useEffect, useLayoutEffect, useMemo, useRef, useState, type ReactNode } from "react";
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
import initGis2dWasm, { MapSession } from "../rs/pkg/gis_2d.js";

const gis2dWasmLoadedSync = Boolean(import.meta.vitest || (typeof process !== "undefined" && process.env.VITEST));

if (gis2dWasmLoadedSync) {
  const { readFileSync } = await import("node:fs");
  const { initSync } = await import("../rs/pkg/gis_2d.js");
  const wasmPath = new URL("../rs/pkg/gis_2d_bg.wasm", import.meta.url).pathname;
  initSync({ module: readFileSync(wasmPath) });
} else {
  await initGis2dWasm();
}

export async function ensureGisMapWasmLoaded(): Promise<void> {
  if (gis2dWasmLoadedSync) {
    return;
  }
  await initGis2dWasm();
}

export { MapSession };
// #endregion 🔌Adapters

// #region 🔖Types
export interface MapCamera {
  x: number;
  y: number;
  zoom: number;
}

export interface MapPositionProps {
  id: string;
  lon: number;
  lat: number;
  label?: string;
  name?: string;
  icon?: IconName;
  sourceUrl?: string;
  kind?: string;
}

export interface MapRouteProps {
  id: string;
  points: ReadonlyArray<readonly [number, number]>;
  strokeWidth?: number;
}

export interface MapRegionProps {
  id: string;
  ring: ReadonlyArray<readonly [number, number]>;
}

export interface MapDescriptor {
  positions: MapPositionProps[];
  routes: MapRouteProps[];
  regions: MapRegionProps[];
}

export type MapRenderMode = "image" | "vector" | "combined";

export type MapVectorStyle = "colored" | "figureGround" | "invertedFigure";

/** @emoji 📶 GIS map LOD band from WASM {@link MapSession.lodScaleJson}. */
export interface GisMapLodEntry {
  id: string;
  name: string;
  description: string;
  maxZoom: number;
  tileZ: number;
}

/** @emoji 📶 Window LOD select value (`automatic` or a pinned band id). */
export const GIS_MAP_LOD_MODE_AUTOMATIC = "automatic" as const;

export type GisMapLodId = GisMapLodEntry["id"];

export type MapLodModeKind = typeof GIS_MAP_LOD_MODE_AUTOMATIC | GisMapLodId;

/** @emoji 👁️ GIS map layer ids for window toggles and {@link MapLayerVisibility}. */
export const GIS_MAP_LAYER_IDS = [
  "raster",
  "water",
  "land",
  "roads",
  "buildings",
  "borders",
  "labels",
  "positions",
  "positionLabels",
  "routes",
  "regions",
] as const;

export type GisMapLayerId = (typeof GIS_MAP_LAYER_IDS)[number];

export interface MapLayerVisibility {
  readonly raster: boolean;
  readonly water: boolean;
  readonly land: boolean;
  readonly roads: boolean;
  readonly buildings: boolean;
  readonly borders: boolean;
  readonly labels: boolean;
  readonly positions: boolean;
  readonly positionLabels: boolean;
  readonly routes: boolean;
  readonly regions: boolean;
}

export const GIS_MAP_LAYER_LABEL: Record<GisMapLayerId, string> = {
  raster: "Raster tiles",
  water: "Water",
  land: "Land",
  roads: "Roads",
  buildings: "Buildings",
  borders: "Borders",
  labels: "Labels",
  positions: "Positions",
  positionLabels: "Position labels",
  routes: "Routes",
  regions: "Regions",
};

/** @emoji 👁️ All GIS map information layers visible. */
export function defaultMapLayerVisibility(): MapLayerVisibility {
  return {
    raster: true,
    water: true,
    land: true,
    roads: true,
    buildings: true,
    borders: true,
    labels: true,
    positions: true,
    positionLabels: true,
    routes: true,
    regions: true,
  };
}

/** @emoji 👁️ Serializes layer visibility for WASM {@link MapSession.setLayerVisibilityJson}. */
export function mapLayerVisibilityToJson(visibility: MapLayerVisibility): string {
  return JSON.stringify(visibility);
}

export function isGisMapLayerId(value: string): value is GisMapLayerId {
  return (GIS_MAP_LAYER_IDS as readonly string[]).includes(value);
}

/** @emoji 🎚️ Layer weight slider bounds (matches WASM {@link clamp_map_layer_weight}). */
export const GIS_MAP_LAYER_WEIGHT_MIN = 0.25;

export const GIS_MAP_LAYER_WEIGHT_MAX = 3;

export const GIS_MAP_LAYER_WEIGHT_STEP = 0.05;

export interface MapLayerStrokeScale {
  readonly raster: number;
  readonly water: number;
  readonly land: number;
  readonly roads: number;
  readonly buildings: number;
  readonly borders: number;
  readonly labels: number;
  readonly positions: number;
  readonly positionLabels: number;
  readonly routes: number;
  readonly regions: number;
}

/** @emoji 🎚️ Default 1× line/label weight for every GIS map layer. */
export function defaultMapLayerStrokeScale(): MapLayerStrokeScale {
  return {
    raster: 1,
    water: 1,
    land: 1,
    roads: 1,
    buildings: 1,
    borders: 1,
    labels: 1,
    positions: 1,
    positionLabels: 1,
    routes: 1,
    regions: 1,
  };
}

/** @emoji 🎚️ Serializes layer weights for WASM {@link MapSession.setLayerStrokeScaleJson}. */
export function mapLayerStrokeScaleToJson(strokeScale: MapLayerStrokeScale): string {
  return JSON.stringify(strokeScale);
}

/** @emoji 🎚️ Layer weight sliders shown in window options for a LOD + render mode (WASM SSOT). */
export function gisMapLayerWeightSlidersAtLod(lodId: GisMapLodId, renderMode: MapRenderMode): readonly GisMapLayerId[] {
  const raw = new MapSession().layerWeightSliderIdsJson(lodId, renderMode);
  try {
    const ids = JSON.parse(raw) as string[];
    if (!Array.isArray(ids)) {
      return [];
    }
    return ids.filter((id): id is GisMapLayerId => isGisMapLayerId(id));
  } catch {
    return [];
  }
}

export interface MapCanvasProps {
  camera?: MapCamera;
  onCamera?: (camera: MapCamera) => void;
  onEffectiveLodChange?: (lodId: GisMapLodId) => void;
  children?: ReactNode;
  className?: string;
  tileUrlTemplate?: string;
  vectorTileUrlTemplate?: string;
  renderMode?: MapRenderMode;
  vectorStyle?: MapVectorStyle;
  lodMode?: MapLodModeKind;
  layerVisibility?: MapLayerVisibility;
  layerStrokeScale?: MapLayerStrokeScale;
  selectedPositionIds?: readonly string[];
  selectedRouteIds?: readonly string[];
  hoveredFeature?: MapHoveredFeature | null;
  selectionMethod?: SelectionMarqueeMethod;
  onSelect?: (payload: MapSelectPayload) => void;
  onHoverChange?: (feature: MapHoveredFeature | null) => void;
  getContextMenuItems?: (context: MapContextMenuContext) => ContextMenuItem[];
  fitWorldRevision?: number;
}

export type MapFeatureKind = "position" | "route";

export interface MapHoveredFeature {
  readonly kind: MapFeatureKind;
  readonly id: string;
}

export interface MapFeatureHit {
  readonly positions: readonly string[];
  readonly routes: readonly string[];
}

export interface MapSelectPayload {
  readonly positions: readonly string[];
  readonly routes: readonly string[];
  readonly mode: SelectionMergeMode;
  readonly crossing: boolean;
}

export interface MapContextMenuContext {
  readonly clientX: number;
  readonly clientY: number;
  readonly feature: MapHoveredFeature | null;
}

const MAP_MARQUEE_THRESHOLD_PX = 6;
// #endregion 🔖Types

// #region 🔖LodScale
let gisMapLodScaleCache: readonly GisMapLodEntry[] | null = null;

function parseGisMapLodScaleJson(raw: string): readonly GisMapLodEntry[] {
  const rows = JSON.parse(raw) as Array<{
    id?: string;
    name?: string;
    description?: string;
    maxZoom?: number;
    tileZ?: number;
  }>;
  if (!Array.isArray(rows)) {
    return [];
  }
  const out: GisMapLodEntry[] = [];
  for (const row of rows) {
    if (typeof row.id !== "string") {
      continue;
    }
    out.push({
      id: row.id,
      name: typeof row.name === "string" ? row.name : row.id,
      description: typeof row.description === "string" ? row.description : "",
      maxZoom: typeof row.maxZoom === "number" ? row.maxZoom : Number.POSITIVE_INFINITY,
      tileZ: typeof row.tileZ === "number" ? row.tileZ : 0,
    });
  }
  return out;
}

/** @emoji 📶 Fixed LOD table declared in GIS map WASM (single source of truth). */
export function getGisMapLodScale(): readonly GisMapLodEntry[] {
  if (!gisMapLodScaleCache) {
    gisMapLodScaleCache = parseGisMapLodScaleJson(new MapSession().lodScaleJson());
  }
  return gisMapLodScaleCache;
}

export function isGisMapLodId(value: string): value is GisMapLodId {
  return getGisMapLodScale().some((lod) => lod.id === value);
}

/** @emoji 📶 Automatic LOD select row label showing the live zoom-derived band. */
export function gisMapLodAutomaticSelectLabel(effectiveLodId: GisMapLodId): string {
  const row = getGisMapLodScale().find((lod) => lod.id === effectiveLodId);
  const name = row?.name ?? effectiveLodId;
  return `Automatic · ${name}`;
}

function parseCurrentLodId(raw: string): GisMapLodId | null {
  try {
    const row = JSON.parse(raw) as { id?: string };
    if (typeof row.id === "string" && isGisMapLodId(row.id)) {
      return row.id;
    }
  } catch {
    return null;
  }
  return null;
}
// #endregion 🔖LodScale

// #region 🔖HostKinds
export const GIS_MAP_HOST_POSITION = "gis.map/position";
export const GIS_MAP_HOST_ROUTE = "gis.map/route";
export const GIS_MAP_HOST_REGION = "gis.map/region";

export function Position(_props: MapPositionProps): null {
  return null;
}
Position.displayName = GIS_MAP_HOST_POSITION;

export function Route(_props: MapRouteProps): null {
  return null;
}
Route.displayName = GIS_MAP_HOST_ROUTE;

export function Region(_props: MapRegionProps): null {
  return null;
}
Region.displayName = GIS_MAP_HOST_REGION;
// #endregion 🔖HostKinds

// #region 🔖Descriptor
function collectMapDescriptor(children: ReactNode): MapDescriptor {
  const positions: MapPositionProps[] = [];
  const routes: MapRouteProps[] = [];
  const regions: MapRegionProps[] = [];
  const visit = (nodes: ReactNode): void => {
    React.Children.forEach(nodes, (child) => {
      if (!React.isValidElement(child)) {
        return;
      }
      const type = child.type as { displayName?: string };
      const name = type.displayName ?? "";
      if (name === GIS_MAP_HOST_POSITION) {
        positions.push(child.props as MapPositionProps);
      } else if (name === GIS_MAP_HOST_ROUTE) {
        routes.push(child.props as MapRouteProps);
      } else if (name === GIS_MAP_HOST_REGION) {
        regions.push(child.props as MapRegionProps);
      } else if (child.props && typeof child.props === "object" && "children" in child.props) {
        visit((child.props as { children?: ReactNode }).children);
      }
    });
  };
  visit(children);
  return { positions, routes, regions };
}

export function mapDescriptorToJson(descriptor: MapDescriptor): string {
  return JSON.stringify({
    positions: descriptor.positions.map((p) => ({
      id: p.id,
      lon: p.lon,
      lat: p.lat,
      label: p.label,
      name: p.name,
      icon: p.icon,
      kind: p.kind,
      source_url: p.sourceUrl,
    })),
    routes: descriptor.routes.map((r) => ({ id: r.id, points: r.points, stroke_width: r.strokeWidth ?? 2 })),
    regions: descriptor.regions.map((reg) => ({ id: reg.id, ring: reg.ring })),
  });
}

function parseMapPositionScreen(raw: string): { x: number; y: number } | null {
  if (raw === "null") {
    return null;
  }
  try {
    const v = JSON.parse(raw) as { x?: number; y?: number };
    if (typeof v.x !== "number" || typeof v.y !== "number") {
      return null;
    }
    return { x: v.x, y: v.y };
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
  if (raw === "null") {
    return null;
  }
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

function mapSelectionToJson(positions: readonly string[], routes: readonly string[]): string {
  return JSON.stringify({ positions: [...positions], routes: [...routes] });
}

function mapHoverToJson(feature: MapHoveredFeature | null): string {
  if (!feature) {
    return "null";
  }
  return JSON.stringify(feature);
}

function screenRectFromPoints(points: readonly SelectionMarqueePoint[]): { x: number; y: number; width: number; height: number } | null {
  if (!points.length) {
    return null;
  }
  const xs = points.map((point) => point.x);
  const ys = points.map((point) => point.y);
  const minX = Math.min(...xs);
  const minY = Math.min(...ys);
  const maxX = Math.max(...xs);
  const maxY = Math.max(...ys);
  return { x: minX, y: minY, width: maxX - minX, height: maxY - minY };
}
// #endregion 🔖Descriptor

// #region 🔖Tiles
const DEFAULT_TILE_URL = "/osm/{z}/{x}/{y}.png";
const DEFAULT_VECTOR_TILE_URL = "/vt/{z}/{x}/{y}.pbf";
const MAX_CONCURRENT_TILE_FETCHES = 12;
const TILE_REFRESH_DEBOUNCE_MS = 120;
let gisMapCameraLimitsCache: { min: number; max: number } | null = null;

/** @emoji 🔭 Wheel/pan zoom bounds from GIS map WASM (cover min depends on viewport size). */
export function getGisMapCameraLimits(session?: MapSession): { min: number; max: number } {
  if (session) {
    return JSON.parse(session.cameraLimitsJson()) as { min: number; max: number };
  }
  if (!gisMapCameraLimitsCache) {
    gisMapCameraLimitsCache = JSON.parse(new MapSession().cameraLimitsJson()) as { min: number; max: number };
  }
  return gisMapCameraLimitsCache;
}

interface VisibleTileRow {
  z: number;
  x: number;
  y: number;
  key: string;
}

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
    if (typeof v.x !== "number" || typeof v.y !== "number" || typeof v.zoom !== "number") {
      return null;
    }
    return { x: v.x, y: v.y, zoom: v.zoom };
  } catch {
    return null;
  }
}
// #endregion 🔖Tiles

// #region 🔖MapTheme
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

function mapParseCssColorToRgba8888(css: string, fallback: [number, number, number, number]): [number, number, number, number] {
  const m = css.match(/rgba?\(\s*([\d.]+)\s*,\s*([\d.]+)\s*,\s*([\d.]+)(?:\s*,\s*([\d.]+%?)\s*)?\)/u);
  if (!m) {
    return fallback;
  }
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
  if (typeof document === "undefined") {
    return "";
  }
  const el = document.createElement("span");
  const key = property === "color" ? "color" : "background-color";
  el.setAttribute("style", `${key}:${value};position:absolute;left:0;top:0;visibility:hidden;pointer-events:none`);
  if (document.documentElement.classList.contains("dark")) {
    el.classList.add("dark");
  }
  document.documentElement.appendChild(el);
  const out = getComputedStyle(el)[property];
  el.remove();
  return out;
}

/** @emoji 🎨 Serializes UI semantic CSS (`--canvas`, `--foreground`, accents) for WASM map Vello paints. */
export function serializeMapCanvasThemeJson(): string {
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
// #endregion 🔖MapTheme

// #region 🔖MapRenderer
export class MapRenderer {
  readonly session: MapSession;
  camera: MapCamera = { x: 0, y: 0, zoom: 1 };
  private raf = 0;
  private disposed = false;
  private canvasEl: HTMLCanvasElement | null = null;
  private tileCache = new Map<string, ArrayBuffer>();
  private vectorTileCache = new Map<string, ArrayBuffer>();
  private tileMiss = new Set<string>();
  private vectorTileMiss = new Set<string>();
  private tileUrlTemplate = DEFAULT_TILE_URL;
  private vectorTileUrlTemplate = DEFAULT_VECTOR_TILE_URL;
  private renderMode: MapRenderMode = "vector";
  private vectorStyle: MapVectorStyle = "colored";
  private lodMode: MapLodModeKind = GIS_MAP_LOD_MODE_AUTOMATIC;
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

  constructor() {
    this.session = new MapSession();
  }

  private applyCanvasPixelSize(lw: number, lh: number, nextDpr: number): void {
    const canvas = this.canvasEl;
    if (!canvas) {
      return;
    }
    const pw = Math.max(1, Math.round(lw * nextDpr));
    const ph = Math.max(1, Math.round(lh * nextDpr));
    if (canvas.width !== pw || canvas.height !== ph) {
      canvas.width = pw;
      canvas.height = ph;
    }
  }

  setTileUrlTemplate(template: string): void {
    this.tileUrlTemplate = template;
  }

  setVectorTileUrlTemplate(template: string): void {
    this.vectorTileUrlTemplate = template;
  }

  setRenderMode(mode: MapRenderMode): void {
    this.renderMode = mode;
    this.session.setRenderMode(mode);
  }

  setVectorStyle(style: MapVectorStyle): void {
    this.vectorStyle = style;
    this.session.setVectorStyle(style);
  }

  setLayerVisibility(visibility: MapLayerVisibility): void {
    this.session.setLayerVisibilityJson(mapLayerVisibilityToJson(visibility));
  }

  setLayerStrokeScale(strokeScale: MapLayerStrokeScale): void {
    this.session.setLayerStrokeScaleJson(mapLayerStrokeScaleToJson(strokeScale));
  }

  setLodMode(mode: MapLodModeKind): void {
    this.lodMode = mode;
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

  readEffectiveLodId(): GisMapLodId | null {
    return parseCurrentLodId(this.session.currentLodJson());
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
    if (!this.canvasEl) {
      return false;
    }
    if (lw === this.logicalWidth && lh === this.logicalHeight && nextDpr === this.dpr) {
      return false;
    }
    this.logicalWidth = lw;
    this.logicalHeight = lh;
    this.dpr = nextDpr;
    this.applyCanvasPixelSize(lw, lh, nextDpr);
    this.session.setSize(lw, lh, nextDpr);
    this.session.reclampCamera();
    const parsed = this.readCameraFromSession();
    if (parsed) {
      this.camera = parsed;
    }
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
    if (this.disposed || !this.canvasEl) {
      return;
    }
    if (this.refreshTimer !== null) {
      clearTimeout(this.refreshTimer);
    }
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = null;
      void this.refreshTiles();
    }, TILE_REFRESH_DEBOUNCE_MS);
  }

  async refreshTiles(): Promise<void> {
    if (!this.canvasEl) {
      return;
    }
    if (this.refreshInFlight) {
      this.tilesRefreshQueued = true;
      return this.refreshInFlight;
    }
    this.refreshInFlight = (async () => {
      const tasks: Promise<void>[] = [];
      if (this.needsRasterTiles()) {
        tasks.push(this.refreshRasterTiles());
      }
      if (this.needsVectorTiles()) {
        tasks.push(this.refreshVectorTiles());
      }
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
    if (!this.canvasEl || !this.session.gpuReady()) {
      return;
    }
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
    const visibleKey = this.session.visibleTilesJson();
    const rows = parseVisibleTilesJson(visibleKey);
    if (rows.length === 0) {
      return;
    }
    if (visibleKey !== this.lastRasterVisibleKey) {
      this.lastRasterVisibleKey = visibleKey;
      this.tileMiss.clear();
    }
    const uploadOne = async (row: VisibleTileRow): Promise<void> => {
      const key = row.key;
      let buf = this.tileCache.get(key);
      if (!buf) {
        if (this.tileMiss.has(key)) {
          return;
        }
        const url = this.tileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const res = await fetch(url);
        if (!res.ok) {
          this.tileMiss.add(key);
          return;
        }
        buf = await res.arrayBuffer();
        this.tileCache.set(key, buf);
      }
      this.session.uploadTile(row.z, row.x, row.y, new Uint8Array(buf));
    };
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => uploadOne(row)));
    }
  }

  async refreshVectorTiles(): Promise<void> {
    const visibleKey = this.session.visibleVectorTilesJson();
    const rows = parseVisibleTilesJson(visibleKey);
    if (rows.length === 0) {
      return;
    }
    if (visibleKey !== this.lastVectorVisibleKey) {
      this.lastVectorVisibleKey = visibleKey;
      this.vectorTileMiss.clear();
    }
    const uploadOne = async (row: VisibleTileRow): Promise<void> => {
      const key = row.key;
      let buf = this.vectorTileCache.get(key);
      if (!buf) {
        if (this.vectorTileMiss.has(key)) {
          return;
        }
        const url = this.vectorTileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
        const res = await fetch(url);
        if (!res.ok) {
          this.vectorTileMiss.add(key);
          return;
        }
        buf = await res.arrayBuffer();
        this.vectorTileCache.set(key, buf);
      }
      this.session.uploadVectorTile(row.z, row.x, row.y, new Uint8Array(buf));
    };
    for (let i = 0; i < rows.length; i += MAX_CONCURRENT_TILE_FETCHES) {
      await Promise.all(rows.slice(i, i + MAX_CONCURRENT_TILE_FETCHES).map((row) => uploadOne(row)));
    }
  }

  private syncMapThemeFromDocument(): void {
    if (typeof document === "undefined") {
      return;
    }
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
      if (this.disposed) {
        return;
      }
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
    if (this.disposed) {
      return;
    }
    this.disposed = true;
    if (this.refreshTimer !== null) {
      clearTimeout(this.refreshTimer);
      this.refreshTimer = null;
    }
    this.stopLoop();
    this.session.free();
    this.canvasEl = null;
  }

  drainEvents(): unknown[] {
    const raw = this.session.drainEventsJson();
    try {
      return JSON.parse(raw) as unknown[];
    } catch {
      return [];
    }
  }

  positionScreen(id: string): { x: number; y: number } | null {
    return parseMapPositionScreen(this.session.positionScreenJson(id));
  }
}
// #endregion 🔖MapRenderer

// #region 🔖MapCanvas
export function MapCanvas({
  camera,
  onCamera,
  onEffectiveLodChange,
  children,
  className,
  tileUrlTemplate,
  vectorTileUrlTemplate,
  renderMode = "vector",
  vectorStyle = "colored",
  lodMode = GIS_MAP_LOD_MODE_AUTOMATIC,
  layerVisibility = defaultMapLayerVisibility(),
  layerStrokeScale = defaultMapLayerStrokeScale(),
  selectedPositionIds = [],
  selectedRouteIds = [],
  hoveredFeature = null,
  selectionMethod = "rectangle",
  onSelect,
  onHoverChange,
  getContextMenuItems,
  fitWorldRevision = 0,
}: MapCanvasProps): React.JSX.Element {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rendererRef = useRef<MapRenderer | null>(null);
  const [internalCamera, setInternalCamera] = useState<MapCamera | null>(camera ?? null);
  const userAdjustedCameraRef = useRef(false);
  const panningRef = useRef(false);
  const descriptor = useMemo(() => collectMapDescriptor(children), [children]);
  const descriptorJson = useMemo(() => mapDescriptorToJson(descriptor), [descriptor]);
  const positionMetaById = useMemo(() => new Map(descriptor.positions.map((row) => [row.id, row])), [descriptor]);
  const layerVisibilityJson = useMemo(() => mapLayerVisibilityToJson(layerVisibility), [layerVisibility]);
  const layerStrokeScaleJson = useMemo(() => mapLayerStrokeScaleToJson(layerStrokeScale), [layerStrokeScale]);
  const selectionJson = useMemo(() => mapSelectionToJson(selectedPositionIds, selectedRouteIds), [selectedPositionIds, selectedRouteIds]);
  const hoverJson = useMemo(() => mapHoverToJson(hoveredFeature), [hoveredFeature]);
  const popupRef = useRef<HTMLDivElement>(null);
  const [marqueeOverlay, setMarqueeOverlay] = useState<
    | { coverage: SelectionMarqueeCoverage; shape: "rect"; rect: { x: number; y: number; width: number; height: number } }
    | { coverage: SelectionMarqueeCoverage; shape: "polygon"; points: readonly SelectionMarqueePoint[] }
    | null
  >(null);
  const [contextMenu, setContextMenu] = useState<{ open: boolean; position: { x: number; y: number } | null; items: ContextMenuItem[] }>({
    open: false,
    position: null,
    items: [],
  });
  const onSelectRef = useRef(onSelect);
  const onHoverChangeRef = useRef(onHoverChange);
  const getContextMenuItemsRef = useRef(getContextMenuItems);
  const selectionMethodRef = useRef(selectionMethod);
  const selectedPositionIdsRef = useRef(selectedPositionIds);
  const selectedRouteIdsRef = useRef(selectedRouteIds);
  onSelectRef.current = onSelect;
  onHoverChangeRef.current = onHoverChange;
  getContextMenuItemsRef.current = getContextMenuItems;
  selectionMethodRef.current = selectionMethod;
  selectedPositionIdsRef.current = selectedPositionIds;
  selectedRouteIdsRef.current = selectedRouteIds;

  const clampMapZoom = useCallback((zoom: number): number => {
    const { min, max } = getGisMapCameraLimits(rendererRef.current?.session);
    return Math.min(max, Math.max(min, zoom));
  }, []);

  const clampCamera = useCallback(
    (next: MapCamera): MapCamera => ({ x: next.x, y: next.y, zoom: clampMapZoom(next.zoom) }),
    [],
  );

  const reportEffectiveLod = useCallback(() => {
    const id = rendererRef.current?.readEffectiveLodId();
    if (id) {
      onEffectiveLodChange?.(id);
    }
  }, [onEffectiveLodChange]);

  const mirrorSessionCameraToReactRef = useRef<() => void>(() => undefined);

  const mirrorSessionCameraToReact = useCallback(() => {
    const parsed = rendererRef.current?.readCameraFromSession();
    if (!parsed) {
      return;
    }
    rendererRef.current!.camera = parsed;
    if (!camera) {
      setInternalCamera(parsed);
    }
    onCamera?.(parsed);
    reportEffectiveLod();
  }, [camera, onCamera, reportEffectiveLod]);

  mirrorSessionCameraToReactRef.current = mirrorSessionCameraToReact;

  const clientToLocal = useCallback((clientX: number, clientY: number): SelectionMarqueePoint => {
    const rect = canvasRef.current?.getBoundingClientRect();
    if (!rect) {
      return { x: 0, y: 0 };
    }
    return { x: clientX - rect.left, y: clientY - rect.top };
  }, []);

  const queryFeatureHits = useCallback((points: readonly SelectionMarqueePoint[], crossing: boolean): MapFeatureHit => {
    const session = rendererRef.current?.session;
    if (!session) {
      return { positions: [], routes: [] };
    }
    if (selectionMethodRef.current === "lasso" && points.length >= 3) {
      return parseMapFeatureHit(session.featuresInPolygonJson(JSON.stringify(points.map((point) => [point.x, point.y])), crossing));
    }
    const rect = screenRectFromPoints(points);
    if (!rect) {
      return { positions: [], routes: [] };
    }
    return parseMapFeatureHit(
      session.featuresInRectJson(rect.x, rect.y, rect.x + rect.width, rect.y + rect.height, crossing),
    );
  }, []);

  const queryHitFeature = useCallback((point: SelectionMarqueePoint): MapHoveredFeature | null => {
    const session = rendererRef.current?.session;
    if (!session) {
      return null;
    }
    return parseMapHoveredFeature(session.hitTestFeatureJson(point.x, point.y));
  }, []);

  const applyCameraToSession = useCallback(
    (next: MapCamera): void => {
      const clamped = clampCamera(next);
      userAdjustedCameraRef.current = true;
      rendererRef.current?.applyCameraToSession(clamped);
      if (!camera) {
        setInternalCamera(clamped);
      }
      onCamera?.(clamped);
    },
    [camera, clampCamera, onCamera],
  );

  const resolveMapPaneElement = useCallback((container: HTMLElement): HTMLElement => {
    let node: HTMLElement | null = container;
    while (node) {
      const slot = node.dataset.slot;
      if (slot === "window" || slot === "mode-dock-stack-body") {
        return node;
      }
      node = node.parentElement;
    }
    return container;
  }, []);

  const readContainerSize = useCallback((): { w: number; h: number } => {
    const container = containerRef.current;
    if (!container) {
      return { w: 1, h: 1 };
    }
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

  useLayoutEffect(() => {
    const canvas = canvasRef.current;
    const container = containerRef.current;
    if (!canvas || !container) {
      return;
    }
    const renderer = new MapRenderer();
    if (tileUrlTemplate) {
      renderer.setTileUrlTemplate(tileUrlTemplate);
    }
    if (vectorTileUrlTemplate) {
      renderer.setVectorTileUrlTemplate(vectorTileUrlTemplate);
    }
    renderer.setRenderMode(renderMode);
    renderer.setVectorStyle(vectorStyle);
    renderer.setLodMode(lodMode);
    renderer.setLayerVisibility(layerVisibility);
    renderer.setLayerStrokeScale(layerStrokeScale);
    rendererRef.current = renderer;
    let disposed = false;
    const dpr = globalThis.devicePixelRatio || 1;

    const applySize = (): void => {
      const nextDpr = globalThis.devicePixelRatio || 1;
      const { w, h } = readContainerSize();
      if (!renderer.setSize(w, h, nextDpr)) {
        return;
      }
      mirrorSessionCameraToReactRef.current();
      renderer.scheduleRefreshTiles();
    };

    let resizeRafId: number | null = null;
    const resizeObserver =
      typeof ResizeObserver === "undefined"
        ? null
        : new ResizeObserver(() => {
            if (resizeRafId !== null) {
              return;
            }
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
              if (disposed) {
                return;
              }
              applySize();
            });
          });
    const pane = resolveMapPaneElement(container);
    resizeObserver?.observe(pane);
    if (pane !== container) {
      resizeObserver?.observe(container);
    }

    const boot = async (): Promise<void> => {
      let { w, h } = readContainerSize();
      for (let attempt = 0; attempt < 240 && (w < 64 || h < 64); attempt += 1) {
        await new Promise<void>((resolve) => {
          if (typeof globalThis.requestAnimationFrame === "function") {
            globalThis.requestAnimationFrame(() => resolve());
          } else {
            queueMicrotask(resolve);
          }
        });
        if (disposed) {
          return;
        }
        ({ w, h } = readContainerSize());
      }
      await renderer.attach(canvas, w, h, dpr);
      if (disposed) {
        renderer.dispose();
        return;
      }
      applySize();
      if (!userAdjustedCameraRef.current) {
        renderer.session.fitWorldCamera();
        const bootCamera = renderer.readCameraFromSession();
        if (bootCamera) {
          renderer.applyCameraToSession(bootCamera);
          if (!camera) {
            setInternalCamera(bootCamera);
          }
          onCamera?.(bootCamera);
        } else if (camera) {
          renderer.applyCameraToSession(clampCamera(camera));
        }
      }
      renderer.syncDescriptor(descriptorJson);
      await renderer.refreshTiles();
      reportEffectiveLod();
      renderer.startLoop();
    };

    void boot();

    return () => {
      disposed = true;
      resizeObserver?.disconnect();
      if (resizeRafId !== null && typeof globalThis.cancelAnimationFrame === "function") {
        globalThis.cancelAnimationFrame(resizeRafId);
      }
      renderer.dispose();
      rendererRef.current = null;
    };
  }, [resolveMapPaneElement, tileUrlTemplate, vectorTileUrlTemplate, readContainerSize]);

  useEffect(() => {
    rendererRef.current?.setRenderMode(renderMode);
    rendererRef.current?.scheduleRefreshTiles();
  }, [renderMode]);

  useEffect(() => {
    rendererRef.current?.setVectorStyle(vectorStyle);
  }, [vectorStyle]);

  useEffect(() => {
    rendererRef.current?.setLodMode(lodMode);
  }, [lodMode]);

  useEffect(() => {
    rendererRef.current?.setLayerVisibility(layerVisibility);
  }, [layerVisibilityJson]);

  useEffect(() => {
    rendererRef.current?.setLayerStrokeScale(layerStrokeScale);
  }, [layerStrokeScaleJson]);

  useEffect(() => {
    rendererRef.current?.syncDescriptor(descriptorJson);
    rendererRef.current?.scheduleRefreshTiles();
  }, [descriptorJson]);

  useEffect(() => {
    rendererRef.current?.session.setSelectionJson(selectionJson);
  }, [selectionJson]);

  useEffect(() => {
    rendererRef.current?.session.setHoverJson(hoverJson);
  }, [hoverJson]);

  useEffect(() => {
    const tooltipFeature = hoveredFeature?.kind === "position" ? hoveredFeature : null;
    if (!tooltipFeature) {
      return undefined;
    }
    let raf = 0;
    const tick = () => {
      const screen = rendererRef.current?.session.featureScreenJson("position", tooltipFeature.id);
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

  useEffect(() => {
    if (!fitWorldRevision) {
      return;
    }
    const renderer = rendererRef.current;
    if (!renderer) {
      return;
    }
    renderer.session.fitWorldCamera();
    mirrorSessionCameraToReact();
    renderer.scheduleRefreshTiles();
    console.log("[DEBUG] gis map canvas fit world applied");
  }, [fitWorldRevision, mirrorSessionCameraToReact]);

  useEffect(() => {
    if (!camera || panningRef.current) {
      return;
    }
    rendererRef.current?.applyCameraToSession(clampCamera(camera));
    rendererRef.current?.scheduleRefreshTiles();
  }, [camera?.x, camera?.y, camera?.zoom, clampCamera]);

  const applyWheelZoom = useCallback(
    (event: WheelEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const r = rendererRef.current;
      const canvas = canvasRef.current;
      if (!r || !canvas) {
        return;
      }
      const rect = canvas.getBoundingClientRect();
      let deltaY =
        event.deltaY *
        (event.deltaMode === WheelEvent.DOM_DELTA_LINE ? 16 : event.deltaMode === WheelEvent.DOM_DELTA_PAGE ? 400 : 1);
      if (event.ctrlKey) {
        deltaY *= 2.5;
      }
      userAdjustedCameraRef.current = true;
      r.session.wheelScreen(event.clientX - rect.left, event.clientY - rect.top, deltaY);
      mirrorSessionCameraToReact();
      r.scheduleRefreshTiles();
    },
    [mirrorSessionCameraToReact],
  );

  useEffect(() => {
    const element = containerRef.current;
    if (!element) {
      return undefined;
    }
    element.addEventListener("wheel", applyWheelZoom, { passive: false });
    return () => element.removeEventListener("wheel", applyWheelZoom);
  }, [applyWheelZoom]);

  const pointer = useRef<{
    leftDown: boolean;
    middleDown: boolean;
    marqueeTracking: boolean;
    marqueeActive: boolean;
    start: SelectionMarqueePoint;
    points: SelectionMarqueePoint[];
  }>({
    leftDown: false,
    middleDown: false,
    marqueeTracking: false,
    marqueeActive: false,
    start: { x: 0, y: 0 },
    points: [],
  });

  const resetMarquee = useCallback(() => {
    pointer.current.marqueeTracking = false;
    pointer.current.marqueeActive = false;
    pointer.current.points = [];
    setMarqueeOverlay(null);
  }, []);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) {
      return undefined;
    }
    const onPointerDown = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 0) {
        pointer.current.leftDown = true;
        pointer.current.marqueeTracking = true;
        pointer.current.marqueeActive = false;
        pointer.current.start = point;
        pointer.current.points = [point];
        if (typeof canvas.setPointerCapture === "function") {
          canvas.setPointerCapture(event.pointerId);
        }
        return;
      }
      if (event.button === 1) {
        event.preventDefault();
        pointer.current.middleDown = true;
        panningRef.current = true;
        userAdjustedCameraRef.current = true;
        if (typeof canvas.setPointerCapture === "function") {
          canvas.setPointerCapture(event.pointerId);
        }
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
        onHoverChangeRef.current?.(hit);
        return;
      }
      event.stopPropagation();
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      if (!pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        pointer.current.marqueeActive = true;
      }
      if (!pointer.current.marqueeActive) {
        return;
      }
      const method = selectionMethodRef.current;
      const points =
        method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
      pointer.current.points = points;
      const coverage = marqueeCoverageFromGesture({
        method,
        startX: pointer.current.start.x,
        endX: point.x,
        path: points,
      });
      const rect = screenRectFromPoints(points);
      setMarqueeOverlay(
        method === "lasso"
          ? { coverage, shape: "polygon", points }
          : { coverage, shape: "rect", rect: rect ?? { x: 0, y: 0, width: 0, height: 0 } },
      );
      queryFeatureHits(points, coverage === "partial");
    };
    const onPointerUp = (event: PointerEvent): void => {
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      if (event.button === 1 && pointer.current.middleDown) {
        pointer.current.middleDown = false;
        panningRef.current = false;
        rendererRef.current?.session.pointerUpScreen(point.x, point.y);
        if (typeof canvas.releasePointerCapture === "function" && canvas.hasPointerCapture(event.pointerId)) {
          canvas.releasePointerCapture(event.pointerId);
        }
        mirrorSessionCameraToReact();
        rendererRef.current?.scheduleRefreshTiles();
        return;
      }
      if (event.button !== 0 || !pointer.current.leftDown) {
        return;
      }
      pointer.current.leftDown = false;
      if (typeof canvas.releasePointerCapture === "function" && canvas.hasPointerCapture(event.pointerId)) {
        canvas.releasePointerCapture(event.pointerId);
      }
      const distance = Math.hypot(point.x - pointer.current.start.x, point.y - pointer.current.start.y);
      const mode = marqueeModeFromModifiers(event);
      const method = selectionMethodRef.current;
      if (pointer.current.marqueeActive && distance >= MAP_MARQUEE_THRESHOLD_PX) {
        const points = method === "lasso" ? [...pointer.current.points, point] : [pointer.current.start, point];
        const coverage = marqueeCoverageFromGesture({
          method,
          startX: pointer.current.start.x,
          endX: point.x,
          path: points,
        });
        const hits = queryFeatureHits(points, coverage === "partial");
        onSelectRef.current?.({
          positions: [...hits.positions],
          routes: [...hits.routes],
          mode,
          crossing: coverage === "partial",
        });
      } else if (distance < MAP_MARQUEE_THRESHOLD_PX) {
        const hit = queryHitFeature(point);
        onSelectRef.current?.({
          positions: hit?.kind === "position" ? [hit.id] : [],
          routes: hit?.kind === "route" ? [hit.id] : [],
          mode,
          crossing: false,
        });
      }
      resetMarquee();
    };
    const onPointerCancel = (event: PointerEvent): void => {
      pointer.current.leftDown = false;
      pointer.current.middleDown = false;
      panningRef.current = false;
      resetMarquee();
      if (typeof canvas.releasePointerCapture === "function" && canvas.hasPointerCapture(event.pointerId)) {
        canvas.releasePointerCapture(event.pointerId);
      }
      mirrorSessionCameraToReact();
    };
    const onContextMenu = (event: MouseEvent): void => {
      event.preventDefault();
      event.stopPropagation();
      const point = clientToLocal(event.clientX, event.clientY);
      const feature = queryHitFeature(point);
      const items = getContextMenuItemsRef.current?.({
        clientX: event.clientX,
        clientY: event.clientY,
        feature,
      }) ?? [];
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
  }, [clientToLocal, mirrorSessionCameraToReact, queryFeatureHits, queryHitFeature, resetMarquee]);

  return (
    <div
      ref={containerRef}
      className={["absolute inset-0 box-border min-h-0 min-w-0 overflow-hidden select-none", className].filter(Boolean).join(" ") || undefined}
      style={{ touchAction: "none" }}
    >
      <canvas ref={canvasRef} className="absolute inset-0 block size-full touch-none outline-none focus:outline-none" />
      {marqueeOverlay?.shape === "rect" ? (
        <SelectionMarquee coverage={marqueeOverlay.coverage} shape="rect" rect={marqueeOverlay.rect} />
      ) : null}
      {marqueeOverlay?.shape === "polygon" ? (
        <SelectionMarquee coverage={marqueeOverlay.coverage} shape="polygon" points={marqueeOverlay.points} />
      ) : null}
      <ContextMenuController
        open={contextMenu.open}
        position={contextMenu.position}
        items={contextMenu.items}
        onOpenChange={(open) => setContextMenu((prev) => ({ ...prev, open }))}
      />
      {hoveredFeature?.kind === "position" ? (
        <div
          ref={popupRef}
          className={cn("pointer-events-none absolute z-10 max-w-56 -translate-x-1/2 -translate-y-[calc(100%+12px)] px-2 py-1.5", floatingMenuSurfaceClass)}
          style={{ left: 0, top: 0 }}
        >
          {(() => {
            const meta = positionMetaById.get(hoveredFeature.id);
            const title = meta?.name ?? meta?.label ?? hoveredFeature.id;
            return (
              <div className="flex items-start gap-1.5">
                {meta?.icon ? <Icon icon={meta.icon} size="small" className="mt-0.5 shrink-0" /> : null}
                <div className="min-w-0">
                  <div className="truncate text-sm font-medium">{title}</div>
                  {meta?.sourceUrl ? (
                    <span className="text-xs text-secondary underline-offset-2">Source available</span>
                  ) : null}
                </div>
              </div>
            );
          })()}
        </div>
      ) : null}
      {children}
    </div>
  );
}
// #endregion 🔖MapCanvas

// #region 🧪Tests
if (import.meta.vitest) {
  const { describe, expect, it } = import.meta.vitest;

  describe("mapDescriptorToJson", () => {
    it("serializes overlays", () => {
      const json = mapDescriptorToJson({
        positions: [{ id: "zurich", lon: 8.54, lat: 47.37, label: "Zürich" }],
        routes: [],
        regions: [],
      });
      expect(json).toContain("zurich");
      expect(json).toContain("Zürich");
    });

    it("serializes rich position metadata", () => {
      const json = mapDescriptorToJson({
        positions: [
          {
            id: "donor-1",
            lon: 8.54,
            lat: 47.37,
            name: "Donor site",
            kind: "donor",
            icon: "box",
            sourceUrl: "https://example.test/donor",
          },
        ],
        routes: [],
        regions: [],
      });
      expect(json).toContain("Donor site");
      expect(json).toContain("source_url");
      expect(json).toContain("https://example.test/donor");
    });
  });

  describe("getGisMapLodScale", () => {
    it("lists every map LOD band from wasm", () => {
      const scale = getGisMapLodScale();
      expect(scale.map((row) => row.id)).toEqual([
        "world",
        "continent",
        "country",
        "region",
        "city",
        "district",
        "street",
        "building",
      ]);
    });
  });

  describe("MapSession tile SSOT", () => {
    it("forced building lod bounds visible tiles at world zoom", () => {
      const session = new MapSession();
      session.setSize(800, 600, 1);
      session.setLodMode("building");
      const rows = parseVisibleTilesJson(session.visibleTilesJson());
      expect(rows.length).toBeGreaterThan(0);
      expect(rows.length).toBeLessThanOrEqual(256);
    });

    it("visibleTilesJson returns a bounded list at world-fit zoom", () => {
      const session = new MapSession();
      session.setSize(800, 600, 1);
      const rows = parseVisibleTilesJson(session.visibleTilesJson());
      expect(rows.length).toBeGreaterThan(0);
      expect(rows.length).toBeLessThan(512);
    });

    it("visibleVectorTilesJson overzooms openfreemap at max camera zoom", () => {
      const session = new MapSession();
      session.setSize(800, 600, 1);
      const { max } = getGisMapCameraLimits();
      session.setCamera(0, 0, max);
      const rows = parseVisibleTilesJson(session.visibleVectorTilesJson());
      expect(rows.length).toBeGreaterThan(0);
      expect(rows.every((r) => r.z <= 14)).toBe(true);
    });

    it("visibleVectorTilesJson lists tiles at world zoom", () => {
      const session = new MapSession();
      session.setSize(800, 600, 1);
      const rows = parseVisibleTilesJson(session.visibleVectorTilesJson());
      expect(rows.length).toBeGreaterThan(0);
      expect(rows.every((r) => r.z <= 14)).toBe(true);
    });
  });

  describe("MapRenderer tile templates", () => {
    it("substitutes vector tile coordinates", () => {
      const url = DEFAULT_VECTOR_TILE_URL.replace("{z}", "2").replace("{x}", "1").replace("{y}", "0");
      expect(url).toBe("/vt/2/1/0.pbf");
    });
  });

  describe("serializeMapCanvasThemeJson", () => {
    it("includes landStroke with zero alpha to avoid tile seams", () => {
      const parsed = JSON.parse(serializeMapCanvasThemeJson()) as { landStroke: number[] };
      expect(parsed.landStroke[3]).toBe(0);
    });

    it("includes selection and hover stroke colors", () => {
      const parsed = JSON.parse(serializeMapCanvasThemeJson()) as { selectionStroke: number[]; hoverStroke: number[] };
      expect(parsed.selectionStroke.length).toBe(4);
      expect(parsed.hoverStroke.length).toBe(4);
    });
  });

  describe("mapSelectionToJson", () => {
    it("serializes positions and routes", () => {
      const json = mapSelectionToJson(["a"], ["b"]);
      const parsed = JSON.parse(json) as { positions: string[]; routes: string[] };
      expect(parsed.positions).toEqual(["a"]);
      expect(parsed.routes).toEqual(["b"]);
    });
  });

}
// #endregion 🧪Tests
