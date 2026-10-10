#!/usr/bin/env bun
/**
 * 🧗️ Writes the authored input of the committed wall depth inference case `🧫️fixtures/💡️inferences/🧗️wall-depth/🏠️attic/📸️snapshot` (snapshot only: the shapely oracle writes `💡️inference/🧗️wall-depth`, the
 * unit test `bless_the_wall_depth_meshes` writes `🧊️meshes` for the three.js oracle). One model, five areas 10 m apart:
 * * a room of four walls on the first storey, every top attached to a gable roof (gable ends, eaves, mitered corners);
 * * a free wall under a hip roof (a ridge plateau and two hips along its axis);
 * * a free wall on the ground storey whose base follows a sloped slab, with a baseboard;
 * * a free wall with a door (interrupts its baseboard), a window with an authored reveal and a window without;
 * * a hand rail on a free wall of the hip roof area.
 * Usage: `bun r12-w2-wp08-oracle-fixture.ts`.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, fixtures, JSONF } from "./r3-f1-paths.ts";

const V = (x: number, y: number) => ({ point: { x, y }, bulge: 0 });
const rect = (x0: number, y0: number, x1: number, y1: number) => [V(x0, y0), V(x1, y0), V(x1, y1), V(x0, y1)];
const line = (a: [number, number], b: [number, number]) => ({ Line: { start: { x: a[0], y: a[1] }, end: { x: b[0], y: b[1] } } });
const layer = (material: string, thickness: number, fn = "Structure") => ({ material, thickness, function: fn });
const material = (name: string, category: string, density: number) => ({ name, category, color: { r: 0.7, g: 0.6, b: 0.5 }, density, conductivity: 0.8, specific_heat: 900 });
const wall = (storey: string, axis: unknown, top: unknown, name: string, extra: Record<string, unknown> = {}) => ({ storey, wall_type: "wt-300", axis, location: "Center", base_offset: 0, top, phase: "New", name, ...extra });
const toRoof = (roof: string, offset = 0) => ({ Roof: { roof, offset } });
const storeyTop = { StoreyTop: { offset: 0 } };
const baseboard = (host: string, extra: Record<string, unknown> = {}) => ({ host, side: "Left", profile: { Rectangle: { width: 0.02, depth: 0.1 } }, height: 0, inset: 0, material: "m-paint", name: "Baseboard", ...extra });

const snapshot = {
  schema: "s.bim.model@1",
  project: { name: "Attic", description: "Walls under roofs, on a sloped slab, with sweeps and reveals", author: "", organization: "", phase_names: [] },
  materials: { "m-brick": material("Brick", "Masonry", 1800), "m-paint": material("Paint", "Finish", 1200), "m-tile": material("Tile", "Other", 2000), "m-conc": material("Concrete", "Concrete", 2400) },
  wall_types: { "wt-300": { name: "Brick 300", layers: [layer("m-brick", 0.3)] } },
  slab_types: { "slt-200": { name: "Concrete 200", layers: [layer("m-conc", 0.2)] } },
  roof_types: { "rt-tile": { name: "Tile", layers: [layer("m-tile", 0.1, "Finish")] } },
  window_types: { "win-1": { name: "Window", width: 1.2, height: 1.2, sill: 0.9, frame_width: 0.06, frame_depth: 0.08, panes: 1, material: "m-paint" } },
  door_types: { "door-1": { name: "Door", width: 0.9, height: 2.1, frame_width: 0.05, frame_depth: 0.1, leaves: "Single", swing: "Left", material: "m-paint" } },
  sites: { "site-1": { name: "Plot", latitude: 47, longitude: 8, elevation: 100, true_north: 0, boundary: [] } },
  buildings: { "bldg-1": { site: "site-1", name: "House", origin: { x: 0, y: 0 }, rotation: 0, elevation: 0 } },
  storeys: { "st-0": { building: "bldg-1", name: "Ground", level: 0, height: 3 }, "st-1": { building: "bldg-1", name: "First", level: 1, height: 2.8 } },
  walls: {
    "w-east": wall("st-1", line([8, 0], [8, 6]), toRoof("r-gable"), "Gable end east"),
    "w-north": wall("st-1", line([8, 6], [0, 6]), toRoof("r-gable", 0.05), "Eave north"),
    "w-west": wall("st-1", line([0, 6], [0, 0]), toRoof("r-gable"), "Gable end west"),
    "w-south": wall("st-1", line([0, 0], [8, 0]), toRoof("r-gable", 0.05), "Eave south"),
    "w-hip": wall("st-1", line([21, 4], [29, 4]), toRoof("r-hip"), "Under the hip"),
    "w-rail": wall("st-1", line([21, 2], [29, 2]), storeyTop, "Rail wall"),
    "w-base": wall("st-0", line([41, 0], [49, 0]), storeyTop, "On the slope", { base_slab: "sl-slope" }),
    "w-door": wall("st-0", line([60, 0], [68, 0]), storeyTop, "Door and windows"),
  },
  roofs: {
    "r-gable": { storey: "st-1", roof_type: "rt-tile", footprint: rect(0, 0, 8, 6), shape: { Gable: { pitch: 0.6, ridge_direction: 0 } }, overhang: 0, base_offset: 0, phase: "New", name: "Gable roof" },
    "r-hip": { storey: "st-1", roof_type: "rt-tile", footprint: rect(20, 0, 30, 8), shape: { Hip: { pitch: 0.4 } }, overhang: 0, base_offset: 0, phase: "New", name: "Hip roof" },
  },
  slabs: { "sl-slope": { storey: "st-0", slab_type: "slt-200", boundary: rect(40, -5, 50, 5), holes: [], offset: 0, slope: { direction: 0, angle: 0.1 }, phase: "New", name: "Sloped slab" } },
  openings: {
    "o-door": { host: "w-door", kind: { Door: { door_type: "door-1" } }, offset: 2, flip_hand: false, flip_facing: false, name: "Door" },
    "o-reveal": { host: "w-door", kind: { Window: { window_type: "win-1" } }, offset: 5, flip_hand: false, flip_facing: false, reveal_depth: 0.1, reveal_material: "m-paint", name: "Window with a reveal" },
    "o-plain": { host: "w-door", kind: { Window: { window_type: "win-1" } }, offset: 7, flip_hand: false, flip_facing: false, name: "Plain window" },
  },
  wall_sweeps: {
    "ws-door": baseboard("w-door"),
    "ws-slope": baseboard("w-base", { name: "Baseboard on the slope" }),
    "ws-rail": baseboard("w-rail", { side: "Right", profile: { Rectangle: { width: 0.05, depth: 0.04 } }, height: 0.9, inset: 0.01, name: "Hand rail" }),
  },
};

const root = child(fixtures, "inferences");
const base = join(root, em(0x1f9d7) + "wall-depth", em(0x1f3e0) + "attic", em(0x1f4f8) + "snapshot");
mkdirSync(base, { recursive: true });
writeFileSync(join(base, JSONF), JSON.stringify(snapshot, null, 2) + "\n");
console.log(`wrote ${base}`);
