#!/usr/bin/env bun
/**
 * 🤝️ WP-16 fixtures: the authored snapshots of the inference cases `🧨️clash-sets/🏢️frame` (an orthogonal frame with a wall, a slab and two ducts, three clash sets) and `⚖️rule-results/🏡️limits`
 * (a house with stairs, doors, a ramp, spaces and zones, one rule per kind) and the committed model of the BCF case `🚪️bcf/🏢️frame`. `bun r12-w2-wp16-fixtures.ts <fixtures root>` writes the three
 * snapshots (`📸️snapshot/🔣️.json`) under the given `🧫️fixtures` root; the meshes, the measures and the expectations are written by the Rust blessing tests and the python oracles, never by hand.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { em } from "./r3-f1-paths.ts";

const sel = (over: Record<string, unknown> = {}) => ({ classes: [], storeys: [], phases: [], ids: [], ...over });
const set = (name: string, a: unknown, b: unknown, tolerance: number, clearance: number) => ({ name, a, b, tolerance, clearance });
const P = F.P;
const line = (a: [number, number], b: [number, number]) => F.line(a, b);

const frame = () => {
  const model: any = F.snap(
    {
      materials: { "m-concrete": F.material("Concrete") },
      wall_types: { "wt-200": F.wallType("Wall 200", [F.layer("m-concrete", 0.2)]) },
      slab_types: { "st-300": F.wallType("Slab 300", [F.layer("m-concrete", 0.3)]) },
      column_types: { "ct-50": { name: "Column 50", profile: { Rectangle: { width: 0.5, depth: 0.5 } }, material: "m-concrete" } },
      beam_types: { "bt-40x60": { name: "Beam 40x60", profile: { Rectangle: { width: 0.4, depth: 0.6 } }, material: "m-concrete" } },
      sites: { "site-1": F.site("Plot") },
      buildings: { "bldg-1": F.building("site-1", "Frame") },
      storeys: { "st-0": F.storey("bldg-1", "Ground", 0, 4) },
      walls: { "w-1": F.wall("st-0", "wt-200", line([3, 2], [3, 6]), F.storeyTop(0), "Wall") },
      columns: {
        "c-1": { storey: "st-0", column_type: "ct-50", position: P(0, 0), rotation: 0, base_offset: 0, top: F.storeyTop(0), name: "C1", phase: "New" },
        "c-2": { storey: "st-0", column_type: "ct-50", position: P(6, 0), rotation: 0, base_offset: 0, top: F.storeyTop(0), name: "C2", phase: "New" },
      },
      beams: {
        "b-1": { storey: "st-0", beam_type: "bt-40x60", axis: line([0, 0], [6, 0]), top_offset: -0.36, name: "B1", phase: "New" },
        "b-2": { storey: "st-0", beam_type: "bt-40x60", axis: line([1, 3], [5, 3]), top_offset: -0.36, name: "B2", phase: "New" },
      },
      slabs: { "sl-1": { storey: "st-0", slab_type: "st-300", boundary: [[-1, -1], [7, -1], [7, 7], [-1, 7]].map(([x, y]) => ({ point: P(x, y), bulge: 0 })), holes: [], offset: 0, name: "Slab", phase: "New" } },
    },
    "Frame",
  );
  model.mep_elements = {
    "d-1": { storey: "st-0", system: "Supply", shape: { Duct: { width: 0.4, height: 0.25 } }, path: [{ x: 0.5, y: 3.5, z: 3 }, { x: 5.5, y: 3.5, z: 3 }], name: "Supply" },
    "d-2": { storey: "st-0", system: "Return", shape: { Duct: { width: 0.4, height: 0.25 } }, path: [{ x: 0.5, y: 5.5, z: 3 }, { x: 5.5, y: 5.5, z: 3 }], name: "Return" },
  };
  model.clash_sets = {
    "cs-structure": set("Structure against walls", sel({ classes: ["Column", "Beam", "Slab"] }), sel({ classes: ["Wall"] }), 0.002, 0),
    "cs-services": set("Services against structure", sel({ classes: ["Mep"] }), sel({ classes: ["Beam", "Column", "Wall"] }), 0.005, 0.15),
    "cs-frame": set("Beams against columns", sel({ classes: ["Beam"] }), sel({ classes: ["Column"] }), 0.001, 0),
  };
  return model;
};

const root = process.argv[2];
if (!root) throw new Error("usage: bun r12-w2-wp16-fixtures.ts <fixtures root>");
const write = (path: string[], model: unknown) => {
  const dir = join(root, ...path, em(0x1f4f8) + "snapshot");
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, em(0x1f523) + ".json"), JSON.stringify(model, null, 2) + "\n");
};
write([em(0x1f4a1) + "inferences", em(0x1f9e8) + "clash-sets", em(0x1f3e2) + "frame"], frame());
console.log("wrote the frame case");
