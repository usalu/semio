/** 🧸️ Canonical JSON builders for `s.bim.model@1` fixtures: every collection is emitted, optional fields only when set. */

export const COLLECTIONS = ["materials", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types", "sites", "buildings", "storeys", "grids", "walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "properties", "classifications"] as const;
type Collection = (typeof COLLECTIONS)[number];

export const P = (x: number, y: number) => ({ x, y });

export const project = (name = "Model") => ({ name, description: "", author: "", organization: "", phase_names: [] as string[] });

export function snap(parts: Partial<Record<Collection, Record<string, unknown>>> = {}, name = "Model") {
  return { schema: "s.bim.model@1", project: project(name), ...Object.fromEntries(COLLECTIONS.map((c) => [c, parts[c] ?? {}])) };
}

export const material = (name: string) => ({ name, category: "Masonry", color: { r: 0.7, g: 0.35, b: 0.25 }, density: 1800, conductivity: 0.8, specific_heat: 900 });
export const layer = (materialId: string, thickness: number, fn = "Structure") => ({ material: materialId, thickness, function: fn });
export const wallType = (name: string, layers: unknown[]) => ({ name, layers });
export const site = (name: string, elevation = 100) => ({ name, latitude: 47, longitude: 8, elevation, true_north: 0, boundary: [] as unknown[] });
export const building = (siteId: string, name: string, elevation = 0) => ({ site: siteId, name, origin: P(0, 0), rotation: 0, elevation });
export const storey = (buildingId: string, name: string, level: number, height: number, cutHeight?: number) => ({ building: buildingId, name, level, height, ...(cutHeight === undefined ? {} : { cut_height: cutHeight }) });
export const line = (a: [number, number], b: [number, number]) => ({ Line: { start: P(...a), end: P(...b) } });
export const unconnected = (height: number) => ({ Unconnected: { height } });
export const storeyTop = (offset = 0) => ({ StoreyTop: { offset } });
export const toStorey = (storeyId: string, offset = 0) => ({ Storey: { storey: storeyId, offset } });
export const wall = (storeyId: string, type: string, axis: unknown, top: unknown, name: string, baseOffset = 0) => ({ storey: storeyId, wall_type: type, axis, location: "Center", base_offset: baseOffset, top, phase: "New", name });
export const opening = (host: string, name: string) => ({ host, kind: { Window: { window_type: "win-1" } }, offset: 1, flip_hand: false, flip_facing: false, name });

/** 🏡️ site-1 / bldg-1 / ground (level 0, 3.0 m) + first (level 1, 2.8 m); two ground walls, one resolved by the storey, one free. */
export function scene(extra: { walls?: Record<string, unknown>; storeys?: Record<string, unknown>; openings?: Record<string, unknown> } = {}) {
  return snap({
    materials: { "m-brick": material("Brick") },
    wall_types: { "wt-300": wallType("Brick 300", [layer("m-brick", 0.3)]) },
    sites: { "site-1": site("Plot") },
    buildings: { "bldg-1": building("site-1", "House") },
    storeys: { "st-ground": storey("bldg-1", "Ground", 0, 3), "st-first": storey("bldg-1", "First", 1, 2.8), ...extra.storeys },
    walls: {
      "w-south": wall("st-ground", "wt-300", line([0, 0], [8, 0]), storeyTop(0), "South"),
      "w-east": wall("st-ground", "wt-300", line([8, 0], [8, 6]), unconnected(2.4), "East"),
      ...extra.walls,
    },
    openings: extra.openings ?? {},
  });
}
