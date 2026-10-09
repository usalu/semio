#!/usr/bin/env bun
/**
 * 🖼️ Writes the authored snapshot of the view oracle case (`🧫️fixtures/💡️inferences/🖼️view-linework/🏠️room/📸️snapshot/🔣️.json`): a closed room of four mitred walls and a free wall on the second storey, a rotated column on each storey and eight vertical views
 * (an elevation, a shallow elevation, an oblique elevation, two sections, a cropped section, a section that hides walls, a phase filtered elevation) next to a plan and a camera that the table leaves out. `bun r11-w12-views-oracle-fixture.ts`.
 * The structure of the file is the one of the annotated room; every collection but the ones below is emptied.
 */
import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { child, em, subset } from "./r3-f1-paths.ts";

const inferences = join(child(subset, "fixtures"), em(0x1f4a1) + "inferences");
const snapshot = em(0x1f4f8) + "snapshot";
const json = em(0x1f523) + ".json";
const template = JSON.parse(readFileSync(join(inferences, em(0x1faa7) + "annotation-layout", em(0x1f3e0) + "room", snapshot, json), "utf8"));

const at = (x: number, y: number) => ({ x, y });
const wall = (storey: string, a: [number, number], b: [number, number], phase = "New", name = "") => ({
  storey,
  wall_type: "wt-300",
  axis: { Line: { start: at(...a), end: at(...b) } },
  location: "Center",
  base_offset: 0,
  top: { StoreyTop: { offset: 0 } },
  phase,
  name,
});
const column = (storey: string, x: number, y: number, rotation: number, name: string) => ({ storey, column_type: "ct-40-60", position: at(x, y), rotation, base_offset: 0, top: { StoreyTop: { offset: 0 } }, phase: "New", name });
const view = (name: string, kind: string, over: Record<string, unknown>) => ({ building: "bldg-1", name, kind, depth: 100, hidden: [], scale: 100, detail: "Medium", ...over });
const plane = (a: [number, number], b: [number, number]) => ({ start: at(...a), end: at(...b) });

const model: Record<string, any> = {};
for (const [key, value] of Object.entries(template)) model[key] = key === "schema" ? value : {};
model.project = { name: "View room", description: "Walls and columns on two storeys, seen by sections and elevations", author: "", organization: "", phase_names: [] };
model.materials = template.materials;
model.wall_types = template.wall_types;
model.column_types = { "ct-40-60": { name: "Column 40x60", profile: { Rectangle: { width: 0.4, depth: 0.6 } }, material: "m-brick" } };
model.sites = template.sites;
model.buildings = template.buildings;
model.storeys = {
  "st-ground": { building: "bldg-1", name: "Ground", level: 0, height: 3.0 },
  "st-first": { building: "bldg-1", name: "First", level: 1, height: 2.8 },
};
const corners: [number, number][] = [[0, 0], [6, 0], [6, 4], [0, 4]];
model.walls = Object.fromEntries([
  ...corners.map((start, index) => [`w${index}`, wall("st-ground", start, corners[(index + 1) % 4], "New", `Wall ${index}`)]),
  ["w4", wall("st-first", [1, 3.5], [5, 3.5], "Existing", "Parapet")],
]);
model.columns = { "col-a": column("st-ground", 3, 2, 0.5, "Column A"), "col-b": column("st-first", 1.5, 1.5, 0, "Column B") };
model.views = {
  "v-plan-ground": view("Plan Ground", "Plan", { storey: "st-ground" }),
  "v-3d": view("3D view", "Perspective", { camera: { target: at(3, 2), target_height: 3, azimuth: 0.78, pitch: 0.5, distance: 20 } }),
  "v-south": view("South", "Elevation", { plane: plane([-2, -2], [8, -2]) }),
  "v-south-shallow": view("South shallow", "Elevation", { plane: plane([-2, -2], [8, -2]), depth: 2.2 }),
  "v-oblique": view("Oblique", "Elevation", { plane: plane([-3, 6], [4, -3]) }),
  "v-section-a": view("Section A", "Section", { plane: plane([-1, 1.5], [7, 1.5]) }),
  "v-section-diagonal": view("Section diagonal", "Section", { plane: plane([-1, -1], [7, 5]), depth: 3 }),
  "v-section-crop": view("Section cropped", "Section", { plane: plane([-1, 1.5], [7, 1.5]), crop: { min: at(0.5, 0.5), max: at(5, 4.2) } }),
  "v-section-columns": view("Section without walls", "Section", { plane: plane([-1, 2], [7, 2]), hidden: ["Walls"] }),
  "v-south-new": view("South new", "Elevation", { plane: plane([-2, -2], [8, -2]), phase: "New" }),
};
const out = join(inferences, em(0x1f5bc) + "view-linework", em(0x1f3e0) + "room", snapshot, json);
mkdirSync(dirname(out), { recursive: true });
writeFileSync(out, JSON.stringify(model, null, 2) + "\n");
console.log(`${Object.keys(model.views).length} views`);
