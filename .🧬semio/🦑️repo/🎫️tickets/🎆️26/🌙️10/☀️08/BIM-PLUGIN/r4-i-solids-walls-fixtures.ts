#!/usr/bin/env bun
/** 🧊️ Writes the authored `snapshot` of every `🧊️element-solids` fixture case (keeps `expected` and `meshes` of an existing file). */
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..", "..", "..", "..");
const cases = join(root, "✏️s", "🔌️plugins", "🏙️bim", "🗿️artifacts", "🏢️model", "🏅️standards", "🔖️1", "🪆️subsets", "✳️any", "🧫️fixtures", "💡️inferences", "🧊️element-solids");

const point = (x: number, y: number) => ({ x, y });
const line = (a: [number, number], b: [number, number]) => ({ Line: { start: point(...a), end: point(...b) } });
const arc = (a: [number, number], b: [number, number], bulge: number) => ({ Arc: { start: point(...a), end: point(...b), bulge } });
const unconnected = (height: number) => ({ Unconnected: { height } });
const wall = (wall_type: string, axis: object, top: object, location = "Center", name = "") => ({ storey: "st-1", wall_type, axis, location, base_offset: 0.0, top, phase: "New", name });
const window = (host: string, window_type: string, offset: number, sill: number, extra: object = {}) => ({ host, kind: { Window: { window_type } }, offset, sill, flip_hand: false, flip_facing: false, name: "", ...extra });
const door = (host: string, door_type: string, offset: number, extra: object = {}) => ({ host, kind: { Door: { door_type } }, offset, sill: 0.0, flip_hand: false, flip_facing: false, name: "", ...extra });
const hole = (host: string, width: number, height: number, offset: number, sill: number) => ({ host, kind: { Void: { width, height } }, offset, sill, flip_hand: false, flip_facing: false, name: "" });

const library = {
  schema: "s.bim.model@1",
  project: { name: "Element Solids", description: "", author: "", organization: "", phase_names: [] },
  materials: {
    "m-brick": { name: "Brick", category: "Masonry", color: { r: 0.7, g: 0.35, b: 0.25 }, density: 1800.0, conductivity: 0.8, specific_heat: 900.0 },
    "m-insulation": { name: "Mineral Wool", category: "Insulation", color: { r: 0.9, g: 0.85, b: 0.3 }, density: 40.0, conductivity: 0.035, specific_heat: 1030.0 },
    "m-wood": { name: "Oak", category: "Wood", color: { r: 0.55, g: 0.4, b: 0.2 }, density: 700.0, conductivity: 0.17, specific_heat: 1600.0 },
    "m-steel": { name: "Steel", category: "Metal", color: { r: 0.6, g: 0.62, b: 0.65 }, density: 7850.0, conductivity: 50.0, specific_heat: 470.0 },
    "m-glass": { name: "Glass", category: "Glass", color: { r: 0.6, g: 0.8, b: 0.9 }, density: 2500.0, conductivity: 1.0, specific_heat: 840.0 },
  },
  wall_types: {
    "wt-200": { name: "Brick 200", layers: [{ material: "m-brick", thickness: 0.2, function: "Structure" }] },
    "wt-300": {
      name: "Brick 300",
      layers: [
        { material: "m-brick", thickness: 0.2, function: "Structure" },
        { material: "m-insulation", thickness: 0.1, function: "Insulation" },
      ],
    },
  },
  window_types: { "wn-1": { name: "Window 1.2 x 1.0", width: 1.2, height: 1.0, sill: 0.0, frame_width: 0.06, frame_depth: 0.12, panes: 2, material: "m-wood" } },
  door_types: {
    "dr-1": { name: "Door 0.9 x 2.1", width: 0.9, height: 2.1, frame_width: 0.05, frame_depth: 0.1, leaves: "Single", swing: "Left", material: "m-wood" },
    "dr-2": { name: "Door 1.6 x 2.1", width: 1.6, height: 2.1, frame_width: 0.05, frame_depth: 0.1, leaves: "Double", swing: "Right", material: "m-wood" },
  },
  sites: { "site-1": { name: "Plot", latitude: 47.0, longitude: 8.0, elevation: 0.0, true_north: 0.0, boundary: [] } },
  buildings: { "bldg-1": { site: "site-1", name: "Building", origin: point(0, 0), rotation: 0.0, elevation: 0.0 } },
  storeys: { "st-1": { building: "bldg-1", name: "Ground", level: 0, height: 3.0 } },
};

const snapshots: Record<string, object> = {
  "straight-openings": {
    ...library,
    walls: {
      "w-plain": wall("wt-200", line([0, 0], [5, 0]), unconnected(2.5)),
      "w-window": wall("wt-200", line([0, 10], [5, 10]), unconnected(2.5)),
      "w-door": wall("wt-200", line([0, 20], [6, 20]), unconnected(2.5)),
      "w-layered": wall("wt-300", line([0, 30], [6, 30]), { StoreyTop: { offset: 0.0 } }),
      "w-diagonal": wall("wt-200", line([0, 40], [4, 43]), unconnected(2.5)),
      "w-exterior": wall("wt-300", line([10, 0], [15, 0]), unconnected(2.5), "Exterior"),
      "w-interior": wall("wt-300", line([10, 5], [15, 5]), unconnected(2.5), "Interior"),
    },
    openings: {
      "o-win-1": window("w-window", "wn-1", 2.5, 0.9),
      "o-void-1": hole("w-window", 0.5, 0.5, 0.8, 1.0),
      "o-door-1": door("w-door", "dr-1", 1.5),
      "o-win-2": window("w-door", "wn-1", 4.0, 0.9),
      "o-door-2": door("w-layered", "dr-2", 2.0),
      "o-win-3": window("w-layered", "wn-1", 4.5, 0.9),
      "o-win-4": window("w-diagonal", "wn-1", 2.5, 0.9),
      "o-bad-1": window("w-plain", "wn-1", 0.2, 0.9),
    },
  },
  "arc-window": {
    ...library,
    walls: { "w-arc": wall("wt-200", arc([4, 0], [0, 4], Math.tan(Math.PI / 8)), unconnected(2.5)) },
    openings: { "o-win-arc": window("w-arc", "wn-1", Math.PI, 0.9) },
  },
  "room-joins": {
    ...library,
    walls: {
      "w-south": wall("wt-200", line([0, 0], [6, 0]), unconnected(2.5)),
      "w-east": wall("wt-200", line([6, 0], [6, 4]), unconnected(2.5)),
      "w-north": wall("wt-200", line([6, 4], [0, 4]), unconnected(2.5)),
      "w-west": wall("wt-200", line([0, 4], [0, 0]), unconnected(2.5)),
      "w-partition": wall("wt-200", line([3, 0], [3, 4]), unconnected(2.5)),
    },
    openings: {},
  },
  "curtain-grid": {
    ...library,
    curtain_walls: {
      "cw-1": {
        storey: "st-1",
        axis: line([0, 0], [6, 0]),
        base_offset: 0.0,
        top: unconnected(3.0),
        u_spacing: 1.5,
        v_spacing: 1.0,
        mullion: { Rectangle: { width: 0.06, depth: 0.12 } },
        panel_material: "m-glass",
        mullion_material: "m-steel",
        name: "Curtain",
      },
    },
  },
};

for (const [name, snapshot] of Object.entries(snapshots)) {
  const dir = join(cases, name);
  mkdirSync(dir, { recursive: true });
  const file = join(dir, "🔣️.json");
  const previous = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : {};
  const head = JSON.stringify({ snapshot, expected: previous.expected ?? {} }, null, 2).slice(0, -2);
  writeFileSync(file, `${head},\n  "meshes": ${JSON.stringify(previous.meshes ?? {})}\n}\n`);
  console.log(`written ${name}`);
}
