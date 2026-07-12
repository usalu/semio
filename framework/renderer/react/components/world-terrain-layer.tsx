import { useEffect, useMemo, useRef, useState } from "react";
import { BufferAttribute, BufferGeometry, CanvasTexture, ClampToEdgeWrapping, DoubleSide, MeshStandardMaterial } from "three";
import { createTerrainSession, type TerrainWasmSession } from "../os-shell.tsx";

//#region TerrainStyle
export type WorldTerrainStyle = {
  readonly tileUrlTemplate: string;
  readonly projectOriginLon: number;
  readonly projectOriginLat: number;
  readonly exaggeration: number;
  readonly colorRamp: string;
  readonly minZoom: number;
  readonly maxZoom: number;
};

export function parseWorldTerrainStyle(json: string | undefined): WorldTerrainStyle | null {
  if (!json) return null;
  try {
    const parsed = JSON.parse(json) as Partial<WorldTerrainStyle>;
    if (typeof parsed.tileUrlTemplate !== "string") return null;
    return {
      tileUrlTemplate: parsed.tileUrlTemplate,
      projectOriginLon: parsed.projectOriginLon ?? 0,
      projectOriginLat: parsed.projectOriginLat ?? 0,
      exaggeration: parsed.exaggeration ?? 1,
      colorRamp: parsed.colorRamp ?? "hypsometric",
      minZoom: parsed.minZoom ?? 6,
      maxZoom: parsed.maxZoom ?? 14,
    };
  } catch {
    return null;
  }
}
//#endregion TerrainStyle

//#region TerrainMesh
type TerrainTileMeshPayload = {
  readonly positions: number[];
  readonly normals: number[];
  readonly indices: number[];
  readonly uvs: number[];
};

function geometryFromTerrainMesh(mesh: TerrainTileMeshPayload): BufferGeometry {
  const geometry = new BufferGeometry();
  geometry.setAttribute("position", new BufferAttribute(new Float32Array(mesh.positions), 3));
  geometry.setAttribute("normal", new BufferAttribute(new Float32Array(mesh.normals), 3));
  geometry.setAttribute("uv", new BufferAttribute(new Float32Array(mesh.uvs), 2));
  geometry.setIndex(mesh.indices);
  return geometry;
}

/** 🎨 A vertical hypsometric ramp (low -> green/tan, high -> grey rock, peak -> white) sampled by
 * each terrain vertex's normalized-elevation `uv.y` — generated once client-side rather than
 * round-tripped through Rust, since it's a pure display convenience. */
let hypsometricTexture: CanvasTexture | null = null;
function getHypsometricTexture(): CanvasTexture {
  if (hypsometricTexture) return hypsometricTexture;
  const canvas = document.createElement("canvas");
  canvas.width = 2;
  canvas.height = 256;
  const ctx = canvas.getContext("2d");
  if (ctx) {
    const gradient = ctx.createLinearGradient(0, canvas.height, 0, 0);
    gradient.addColorStop(0, "#4b6b3a");
    gradient.addColorStop(0.5, "#a68a5b");
    gradient.addColorStop(0.85, "#8f8f8f");
    gradient.addColorStop(1, "#ffffff");
    ctx.fillStyle = gradient;
    ctx.fillRect(0, 0, canvas.width, canvas.height);
  }
  hypsometricTexture = new CanvasTexture(canvas);
  hypsometricTexture.wrapS = ClampToEdgeWrapping;
  hypsometricTexture.wrapT = ClampToEdgeWrapping;
  hypsometricTexture.needsUpdate = true;
  return hypsometricTexture;
}
//#endregion TerrainMesh

//#region TerrainTileRenderer
const TERRAIN_TILE_REFRESH_DEBOUNCE_MS = 150;
const MAX_CONCURRENT_TERRAIN_TILE_FETCHES = 8;

type TerrainTileRow = { readonly z: number; readonly x: number; readonly y: number; readonly key: string };

function parseVisibleTerrainTilesJson(raw: string): TerrainTileRow[] {
  try {
    const rows = JSON.parse(raw) as TerrainTileRow[];
    return Array.isArray(rows) ? rows : [];
  } catch {
    return [];
  }
}

/** 🧵 Owns a `TerrainSession`, fetches/uploads/evicts DEM tiles as the camera moves, and reports
 * the current set of tile geometries back to React — the 3D analog of `gis-map-host.tsx`'s
 * `MapRenderer`, except it hands back mesh buffers for three.js instead of driving a canvas. */
class TerrainTileRenderer {
  private disposed = false;
  private session: TerrainWasmSession | null = null;
  private readonly tileMiss = new Set<string>();
  private refreshTimer: ReturnType<typeof setTimeout> | null = null;
  private refreshInFlight: Promise<void> | null = null;
  private readonly geometries = new Map<string, BufferGeometry>();

  constructor(
    private readonly style: WorldTerrainStyle,
    private readonly onGeometriesChanged: (geometries: Map<string, BufferGeometry>) => void,
  ) {}

  async init(): Promise<void> {
    const session = await createTerrainSession();
    if (this.disposed) return;
    session.set_project_origin(this.style.projectOriginLon, this.style.projectOriginLat);
    session.set_exaggeration(this.style.exaggeration);
    this.session = session;
  }

  scheduleRefresh(cameraJson: string): void {
    if (this.disposed) return;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    this.refreshTimer = setTimeout(() => {
      this.refreshTimer = null;
      void this.refresh(cameraJson);
    }, TERRAIN_TILE_REFRESH_DEBOUNCE_MS);
  }

  private async refresh(cameraJson: string): Promise<void> {
    if (this.disposed || !this.session) return;
    if (this.refreshInFlight) await this.refreshInFlight;
    if (this.disposed) return;
    this.refreshInFlight = this.doRefresh(cameraJson).finally(() => {
      this.refreshInFlight = null;
    });
    return this.refreshInFlight;
  }

  private async doRefresh(cameraJson: string): Promise<void> {
    const session = this.session;
    if (!session) return;
    const rows = parseVisibleTerrainTilesJson(session.visible_terrain_tiles_json(cameraJson));
    const visibleKeys = new Set(rows.map((row) => row.key));
    for (const key of [...this.geometries.keys()]) {
      if (visibleKeys.has(key)) continue;
      this.geometries.get(key)?.dispose();
      this.geometries.delete(key);
      const [z, x, y] = key.split("/").map(Number);
      if (z !== undefined && x !== undefined && y !== undefined) session.evict_terrain_tile(z, x, y);
    }
    const missing = rows.filter((row) => !this.geometries.has(row.key) && !this.tileMiss.has(row.key));
    const uploadOne = async (row: TerrainTileRow): Promise<void> => {
      const url = this.style.tileUrlTemplate.replace("{z}", String(row.z)).replace("{x}", String(row.x)).replace("{y}", String(row.y));
      let response: Response;
      try {
        response = await fetch(url);
      } catch {
        this.tileMiss.add(row.key);
        return;
      }
      if (!response.ok) {
        this.tileMiss.add(row.key);
        return;
      }
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (this.disposed) return;
      if (!session.upload_elevation_tile(row.z, row.x, row.y, bytes)) {
        this.tileMiss.add(row.key);
        return;
      }
      const meshJson = session.terrain_tile_mesh_json(row.z, row.x, row.y);
      if (meshJson === "null" || this.disposed) return;
      const mesh = JSON.parse(meshJson) as TerrainTileMeshPayload;
      this.geometries.set(row.key, geometryFromTerrainMesh(mesh));
    };
    for (let i = 0; i < missing.length; i += MAX_CONCURRENT_TERRAIN_TILE_FETCHES) {
      await Promise.all(missing.slice(i, i + MAX_CONCURRENT_TERRAIN_TILE_FETCHES).map((row) => uploadOne(row)));
    }
    if (!this.disposed) this.onGeometriesChanged(new Map(this.geometries));
  }

  dispose(): void {
    this.disposed = true;
    if (this.refreshTimer !== null) clearTimeout(this.refreshTimer);
    for (const geometry of this.geometries.values()) geometry.dispose();
    this.geometries.clear();
  }
}
//#endregion TerrainTileRenderer

//#region WorldTerrainLayer
/** ⛰️ Renders GIS 3D terrain as chunked DEM-tile meshes inside the shared `World3d` viewport —
 * mounted alongside `WorldInstancesLayer` when `scene.terrainJson` is present. */
export function WorldTerrainLayer({
  terrainJson,
  cameraPosition,
  cameraTarget,
}: {
  readonly terrainJson: string | undefined;
  readonly cameraPosition: readonly [number, number, number];
  readonly cameraTarget: readonly [number, number, number];
}) {
  const style = useMemo(() => parseWorldTerrainStyle(terrainJson), [terrainJson]);
  const rendererRef = useRef<TerrainTileRenderer | null>(null);
  const [geometries, setGeometries] = useState<Map<string, BufferGeometry>>(new Map());
  const material = useMemo(() => new MeshStandardMaterial({ map: getHypsometricTexture(), side: DoubleSide, roughness: 1, metalness: 0 }), []);

  useEffect(() => {
    if (!style) {
      rendererRef.current?.dispose();
      rendererRef.current = null;
      setGeometries(new Map());
      return undefined;
    }
    const renderer = new TerrainTileRenderer(style, setGeometries);
    rendererRef.current = renderer;
    void renderer.init().then(() => {
      if (rendererRef.current === renderer) renderer.scheduleRefresh(JSON.stringify({ position: cameraPosition, target: cameraTarget }));
    });
    return () => {
      renderer.dispose();
      if (rendererRef.current === renderer) rendererRef.current = null;
    };
    // eslint-disable-next-line react-hooks/exhaustive-deps -- camera changes are handled by the effect below; this one only (re)creates the session when the terrain source itself changes.
  }, [style?.tileUrlTemplate, style?.projectOriginLon, style?.projectOriginLat, style?.exaggeration]);

  useEffect(() => {
    if (!style) return;
    rendererRef.current?.scheduleRefresh(JSON.stringify({ position: cameraPosition, target: cameraTarget }));
  }, [style, cameraPosition, cameraTarget]);

  if (!style) return null;

  return (
    <group>
      {[...geometries.entries()].map(([key, geometry]) => (
        <mesh key={key} geometry={geometry} material={material} receiveShadow />
      ))}
    </group>
  );
}
//#endregion WorldTerrainLayer
