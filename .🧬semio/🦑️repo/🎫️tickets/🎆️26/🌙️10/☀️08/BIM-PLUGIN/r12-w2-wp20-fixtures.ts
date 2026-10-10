#!/usr/bin/env bun
/**
 * 🌡️ Writes the committed cases of the `🌡️energy-envelope` inference oracle under `🧫️fixtures/💡️inferences/🌡️energy-envelope/<case>/📸️snapshot/🔣️.json`: a closed insulated room with a window and a door (`🏠️box`), two
 * rooms with different set points in two zones (`🏘️zoning`) and a stack of cellar, ground floor and upper floor under a roof (`🧱️stack`). The expectation is written by the python oracle (`🐍️.py write`), never here.
 * Run `bun r12-w2-wp20-fixtures.ts`.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { child, em, fixtures } from "./r3-f1-paths.ts";

const P = F.P;
const material = (name: string, conductivity: number, category = "Masonry") => ({ name, category, color: { r: 0.7, g: 0.35, b: 0.25 }, density: 1800, conductivity, specific_heat: 900 });
const layer = (id: string, thickness: number) => ({ material: id, thickness, function: "Structure" });
const corner = (x: number, y: number) => ({ point: P(x, y), bulge: 0 });
const rect = (x0: number, y0: number, x1: number, y1: number) => [corner(x0, y0), corner(x1, y0), corner(x1, y1), corner(x0, y1)];
const wall = (storey: string, a: [number, number], b: [number, number], name: string) => F.wall(storey, "wt", F.line(a, b), F.storeyTop(0), name);
const space = (storey: string, number: string, name: string, seed: [number, number], extra: Record<string, unknown> = {}) => ({ storey, number, name, boundary: { Bounded: { seed: P(...seed) } }, usage: "Office", phase: "New", ...extra });
const slab = (storey: string, boundary: unknown[], name: string) => ({ storey, slab_type: "st", boundary, holes: [], offset: 0, phase: "New", name });
const opening = (host: string, kind: unknown, offset: number, name: string) => ({ host, kind, offset, flip_hand: false, flip_facing: false, name });
const conditions = (heating?: number, cooling?: number) => ({ occupancy: "Office", occupancy_density: 0.1, ...(heating === undefined ? {} : { heating_setpoint: heating }), ...(cooling === undefined ? {} : { cooling_setpoint: cooling }), ventilation_rate: 1.5, lighting_power_density: 8, equipment_power_density: 12, schedule: "Office 08-18" });

const base = (parts: Record<string, unknown>) => ({
  ...F.snap({
    materials: { m: material("Brick", 0.8), ins: material("Insulation", 0.04, "Insulation") },
    wall_types: { wt: F.wallType("Insulated brick", [layer("m", 0.2), layer("ins", 0.1)]) },
    sites: { site: F.site("Plot", 0) },
    buildings: { bldg: F.building("site", "Building") },
    ...(parts as object),
  }),
  slab_types: { st: { name: "Slab 250", layers: [layer("m", 0.25)] } },
  roof_types: { rt: { name: "Flat roof", layers: [layer("ins", 0.2), layer("m", 0.2)] } },
  window_types: { win: { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.05, frame_depth: 0.08, panes: 2, material: "m", u_value: 1.1, g_value: 0.5, frame_fraction: 0.25 } },
  door_types: { dr: { name: "Door", width: 1, height: 2.1, frame_width: 0.05, frame_depth: 0.08, leaves: "Single", swing: "Left", material: "m", u_value: 1.8 } },
});

const box = () => {
  const model: any = base({
    storeys: { "st-0": F.storey("bldg", "Ground", 0, 3) },
    walls: { "w-south": wall("st-0", [0, 0], [4, 0], "South"), "w-east": wall("st-0", [4, 0], [4, 3], "East"), "w-north": wall("st-0", [4, 3], [0, 3], "North"), "w-west": wall("st-0", [0, 3], [0, 0], "West") },
  });
  model.slabs = { sl: slab("st-0", rect(0, 0, 4, 3), "Slab") };
  model.openings = {
    "o-win": opening("w-south", { Window: { window_type: "win" } }, 2, "Window"),
    "o-door": opening("w-north", { Door: { door_type: "dr" } }, 1, "Door"),
  };
  model.spaces = { sp: space("st-0", "0.01", "Room", [2, 1.5]) };
  model.space_conditions = { sp: conditions(20, 26) };
  model.sites.site.true_north = 0.3;
  model.buildings.bldg.rotation = 0.2;
  return model;
};

const zoning = () => {
  const model: any = base({
    storeys: { "st-0": F.storey("bldg", "Ground", 0, 3) },
    walls: {
      "w-south": wall("st-0", [0, 0], [8, 0], "South"), "w-east": wall("st-0", [8, 0], [8, 3], "East"), "w-north": wall("st-0", [8, 3], [0, 3], "North"), "w-west": wall("st-0", [0, 3], [0, 0], "West"),
      "w-part": wall("st-0", [4, 0], [4, 3], "Partition"),
    },
  });
  model.slabs = { sl: slab("st-0", rect(0, 0, 8, 3), "Slab") };
  model.openings = {
    "o-a": opening("w-south", { Window: { window_type: "win" } }, 2, "Window A"),
    "o-b": opening("w-south", { Window: { window_type: "win" } }, 6, "Window B"),
    "o-part": opening("w-part", { Door: { door_type: "dr" } }, 1.5, "Partition door"),
  };
  model.zones = { "z-a": { name: "Zone A", category: "Thermal", occupancy_density: 0.1 }, "z-b": { name: "Zone B", category: "Thermal", occupancy_density: 0.1 } };
  model.spaces = { "sp-a": space("st-0", "0.01", "West room", [2, 1.5], { zone: "z-a" }), "sp-b": space("st-0", "0.02", "East room", [6, 1.5], { zone: "z-b" }) };
  model.space_conditions = { "sp-a": conditions(20, 26), "sp-b": conditions(22, 27) };
  return model;
};

const stack = () => {
  const model: any = base({
    storeys: { "st-b": F.storey("bldg", "Cellar", -1, 2.6), "st-0": F.storey("bldg", "Ground", 0, 3), "st-1": F.storey("bldg", "Upper", 1, 3) },
    walls: Object.fromEntries(
      ["st-b", "st-0", "st-1"].flatMap((storey) => [
        [`w-${storey}-south`, wall(storey, [0, 0], [6, 0], "South")],
        [`w-${storey}-east`, wall(storey, [6, 0], [6, 4], "East")],
        [`w-${storey}-north`, wall(storey, [6, 4], [0, 4], "North")],
        [`w-${storey}-west`, wall(storey, [0, 4], [0, 0], "West")],
      ]),
    ),
  });
  model.slabs = { "sl-b": slab("st-b", rect(0, 0, 6, 4), "Cellar slab"), "sl-0": slab("st-0", rect(0, 0, 6, 4), "Ground slab"), "sl-1": slab("st-1", rect(0, 0, 6, 4), "Upper slab") };
  model.roofs = { rf: { storey: "st-1", roof_type: "rt", footprint: rect(-0.2, -0.2, 6.2, 4.2), shape: "Flat", overhang: 0, base_offset: 0, phase: "New", name: "Roof" } };
  model.openings = { "o-0": opening("w-st-0-east", { Window: { window_type: "win" } }, 2, "Ground window"), "o-1": opening("w-st-1-north", { Window: { window_type: "win" } }, 3, "Upper window") };
  model.spaces = { "sp-b": space("st-b", "B.01", "Cellar", [3, 2], { usage: "Storage" }), "sp-0": space("st-0", "0.01", "Ground", [3, 2]), "sp-1": space("st-1", "1.01", "Upper", [3, 2]) };
  model.space_conditions = { "sp-b": { occupancy: "Storage" }, "sp-0": conditions(20, 26), "sp-1": conditions(20, 26) };
  return model;
};

const root = join(child(fixtures, "inferences"), em(0x1f321) + "energy-envelope");
for (const [dir, model] of [[em(0x1f3e0) + "box", box()], [em(0x1f3d8) + "zoning", zoning()], [em(0x1f9f1) + "stack", stack()]] as const) {
  const target = join(root, dir, em(0x1f4f8) + "snapshot");
  mkdirSync(target, { recursive: true });
  writeFileSync(join(target, em(0x1f523) + ".json"), `${JSON.stringify(model, null, 2)}\n`);
  console.log(`wrote ${dir}`);
}
