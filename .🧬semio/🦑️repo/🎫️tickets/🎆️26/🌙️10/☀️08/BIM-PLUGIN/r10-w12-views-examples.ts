#!/usr/bin/env bun
/**
 * 🖼️ Authored views of the bundled examples. `viewsFor(model)` is what the `createView` command of the editor makes for a building: the plan of every storey, the ceiling plan of every storey that has ceilings, the four default building
 * elevations, two sections through the middle of the building and a perspective camera. The planes come from the authored geometry (walls grown by half their thickness, columns, beams, slabs, roofs, railings, stairs) with the margin of the
 * command, so the Rust unit test `the_example_views_are_the_ones_the_command_makes` can check them against `create-view::extents` and `elevation_of`. `bun r10-w12-views-examples.ts` adds the views to the committed demo snapshot source;
 * the house and the office get theirs from `r4-x-examples-gen.ts`.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, subset } from "./r3-f1-paths.ts";

type Pt = { x: number; y: number };
type Json = Record<string, any>;

export const MARGIN = 2.0;

const ends = (axis: Json): Pt[] => {
  const row = axis.Line ?? axis.Arc;
  return [row.start, row.end];
};

/** 📏️ The plan rectangle `[x0, y0, x1, y1]` of the authored elements of a building, mirroring `extents` of the `createView` command. */
export function extents(model: Json, building: string): [number, number, number, number] | undefined {
  const on = (storey: string) => model.storeys?.[storey]?.building === building;
  const points: Pt[] = [];
  let grow = 0;
  for (const wall of Object.values<Json>(model.walls ?? {}).filter((row) => on(row.storey))) {
    points.push(...ends(wall.axis));
    grow = Math.max(grow, (model.wall_types?.[wall.wall_type]?.layers ?? []).reduce((sum: number, layer: Json) => sum + layer.thickness, 0) / 2);
  }
  for (const row of Object.values<Json>(model.curtain_walls ?? {}).filter((row) => on(row.storey))) points.push(...ends(row.axis));
  for (const row of Object.values<Json>(model.columns ?? {}).filter((row) => on(row.storey))) points.push(row.position);
  for (const row of Object.values<Json>(model.beams ?? {}).filter((row) => on(row.storey))) points.push(...ends(row.axis));
  for (const row of Object.values<Json>(model.slabs ?? {}).filter((row) => on(row.storey))) points.push(...row.boundary.map((vertex: Json) => vertex.point));
  for (const row of Object.values<Json>(model.roofs ?? {}).filter((row) => on(row.storey))) points.push(...row.footprint.map((vertex: Json) => vertex.point));
  for (const row of Object.values<Json>(model.railings ?? {}).filter((row) => on(row.storey))) points.push(...row.path);
  for (const row of Object.values<Json>(model.stairs ?? {}).filter((row) => on(row.storey))) points.push(row.start);
  for (const row of Object.values<Json>(model.ramps ?? {}).filter((row) => on(row.storey))) points.push(...row.path.map((vertex: Json) => vertex.point));
  if (points.length === 0) return undefined;
  const x0 = Math.min(...points.map((p) => p.x));
  const y0 = Math.min(...points.map((p) => p.y));
  const x1 = Math.max(...points.map((p) => p.x));
  const y1 = Math.max(...points.map((p) => p.y));
  return [x0 - grow, y0 - grow, x1 + grow, y1 + grow];
}

const at = (x: number, y: number): Pt => ({ x, y });
const base = (building: string, name: string, kind: string, over: Json = {}): Json => ({ building, name, kind, depth: 100, hidden: [], scale: 100, detail: "Medium", ...over });

/** 🔭️ The plane and depth of the elevation looking at a side, mirroring `elevation_of`. */
export function elevationOf(rect: [number, number, number, number], side: "south" | "east" | "north" | "west"): { plane: { start: Pt; end: Pt }; depth: number } {
  const [x0, y0, x1, y1] = rect;
  const [west, east, south, north] = [x0 - MARGIN, x1 + MARGIN, y0 - MARGIN, y1 + MARGIN];
  switch (side) {
    case "south": return { plane: { start: at(west, south), end: at(east, south) }, depth: y1 - y0 + MARGIN + 1 };
    case "east": return { plane: { start: at(east, south), end: at(east, north) }, depth: x1 - x0 + MARGIN + 1 };
    case "north": return { plane: { start: at(east, north), end: at(west, north) }, depth: y1 - y0 + MARGIN + 1 };
    case "west": return { plane: { start: at(west, north), end: at(west, south) }, depth: x1 - x0 + MARGIN + 1 };
  }
}

/** 🖼️ The views of every building of a snapshot source, keyed by id. */
export function viewsFor(model: Json): Record<string, Json> {
  const views: Record<string, Json> = {};
  const single = Object.keys(model.buildings ?? {}).length === 1;
  for (const building of Object.keys(model.buildings ?? {})) {
    const prefix = single ? "" : `${building}-`;
    const storeys = Object.entries<Json>(model.storeys ?? {}).filter(([, row]) => row.building === building).sort((a, b) => a[1].level - b[1].level);
    for (const [id, storey] of storeys) {
      views[`v-${prefix}plan-${id}`] = base(building, `Plan ${storey.name}`, "Plan", { storey: id });
      if (Object.values<Json>(model.ceilings ?? {}).some((row) => row.storey === id)) views[`v-${prefix}ceiling-${id}`] = base(building, `Ceiling plan ${storey.name}`, "CeilingPlan", { storey: id });
    }
    const rect = extents(model, building);
    if (!rect) continue;
    const middle = { x: (rect[0] + rect[2]) / 2, y: (rect[1] + rect[3]) / 2 };
    views[`v-${prefix}section-a`] = base(building, "Section A", "Section", { plane: { start: at(rect[0] - MARGIN, middle.y), end: at(rect[2] + MARGIN, middle.y) }, depth: (rect[3] - rect[1]) / 2 + MARGIN });
    views[`v-${prefix}section-b`] = base(building, "Section B", "Section", { plane: { start: at(middle.x, rect[1] - MARGIN), end: at(middle.x, rect[3] + MARGIN) }, depth: (rect[2] - rect[0]) / 2 + MARGIN });
    for (const [side, name] of [["south", "South"], ["east", "East"], ["north", "North"], ["west", "West"]] as const) {
      const { plane, depth } = elevationOf(rect, side);
      views[`v-${prefix}elevation-${side}`] = base(building, name, "Elevation", { plane, depth });
    }
    const height = storeys.filter(([, row]) => row.level >= 0).reduce((sum, [, row]) => sum + row.height, 0);
    const width = rect[2] - rect[0];
    const depth = rect[3] - rect[1];
    views[`v-${prefix}3d`] = base(building, "3D view", "Perspective", { camera: { target: middle, target_height: height / 2, azimuth: Math.PI / 4, pitch: 0.5, distance: 2 * Math.max(width, depth, height) + 5 } });
  }
  return views;
}

if (import.meta.main) {
  const demo = join(child(subset, "assets"), em(0x1f3ac) + "demo", em(0x1f4f8) + "snapshot.json");
  const model = JSON.parse(readFileSync(demo, "utf8"));
  const keep = { ...model };
  delete keep.views;
  const views = viewsFor(keep);
  writeFileSync(demo, JSON.stringify({ ...keep, views }, null, 2) + "\n");
  console.log(`demo: ${Object.keys(views).length} views`);
}
