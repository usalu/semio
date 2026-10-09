#!/usr/bin/env bun
/**
 * 🔲️ Writes the authored input of the committed inference cases of the ceilings (`w06-ceilings`): the solids case
 * `🧫️fixtures/💡️inferences/🧊️element-solids/🔲️ceilings-holes-slope` (snapshot only, the shapely oracle fills `expected` through
 * `🧪️tests/📦️infer-bim-1-solids-rest/🐍️.py` (case `ceilings-holes-slope`), the unit test blesses `meshes`) and the ceilings hung over the committed rooms model `🧫️fixtures/💡️inferences/🛋️spaces/🏡️rooms`
 * (whose `expected` table the shapely oracle of `🏠️infer-bim-1-spaces` rewrites). Usage: `bun r10-w06-ceilings-fixtures.ts`; existing `expected`/`meshes` members survive a rewrite.
 */
import { existsSync, mkdirSync, readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { em, fixtures, JSONF, child } from "./r3-f1-paths.ts";
import { field, parse, print, raw, string_, type Node } from "./r6-z-mutations-rawjson.ts";

const V = (x: number, y: number, bulge = 0) => ({ point: { x, y }, bulge });
const rect = (x0: number, y0: number, x1: number, y1: number) => [V(x0, y0), V(x1, y0), V(x1, y1), V(x0, y1)];
const layer = (material: string, thickness: number, fn = "Finish") => ({ material, thickness, function: fn });
const material = (name: string, category: string, r: number, g: number, b: number, density: number) => ({ name, category, color: { r, g, b }, density, conductivity: 1.0, specific_heat: 900.0 });

const solids = {
  schema: "s.bim.model@1",
  project: { name: "Ceilings", description: "", author: "", organization: "", phase_names: [] },
  materials: {
    "m-board": material("Plasterboard", "Finish", 0.9, 0.9, 0.88, 800),
    "m-wool": material("Mineral wool", "Insulation", 0.9, 0.85, 0.4, 30),
  },
  sites: { "site-1": { name: "Plot", latitude: 47.0, longitude: 8.0, elevation: 100.0, true_north: 0.0, boundary: [] } },
  buildings: { "bldg-1": { site: "site-1", name: "Frame", origin: { x: 0.0, y: 0.0 }, rotation: 0.0, elevation: 2.5 } },
  storeys: { "st-ground": { building: "bldg-1", name: "Ground", level: 0, height: 3.0 }, "st-first": { building: "bldg-1", name: "First", level: 1, height: 2.8 } },
  ceiling_types: {
    "ct-board": { name: "Board on wool", layers: [layer("m-board", 0.0125), layer("m-wool", 0.05, "Insulation")] },
    "ct-tile": { name: "Tile", layers: [layer("m-board", 0.02)] },
    "ct-bare": { name: "Bare", layers: [] },
  },
  ceilings: {
    "c-holed": { storey: "st-ground", ceiling_type: "ct-board", boundary: rect(0, 0, 6, 4), holes: [rect(2, 1, 3, 2)], offset: 0.3, name: "Holed" },
    "c-sloped": { storey: "st-ground", ceiling_type: "ct-tile", boundary: rect(0, 0, 5, 4), holes: [], offset: 0.1, slope: { direction: 0.0, angle: 0.05 }, name: "Sloped" },
    "c-diagonal-slope": { storey: "st-first", ceiling_type: "ct-tile", boundary: rect(0, 0, 4, 3), holes: [], offset: 0.2, slope: { direction: 1.0471975511965976, angle: 0.1 }, name: "Diagonal fall" },
    "c-curved": { storey: "st-first", ceiling_type: "ct-board", boundary: [V(0, 0), V(4, 0, 1), V(4, 4), V(0, 4)], holes: [[V(1, 2, 1), V(2, 2, 1)]], offset: 0.25, name: "Curved with round hole" },
    "c-bare": { storey: "st-ground", ceiling_type: "ct-bare", boundary: rect(0, 0, 1, 1), holes: [], offset: 0.0, name: "No layers" },
    "c-unknown-type": { storey: "st-ground", ceiling_type: "ct-none", boundary: rect(0, 0, 1, 1), holes: [], offset: 0.0, name: "Unknown type" },
  },
};

const element = child(child(fixtures, "inferences"), "element-solids");
const rooms = child(child(child(fixtures, "inferences"), "spaces"), "rooms");

function write(dir: string, name: string, snapshot: unknown) {
  const folder = join(dir, em(0x1f532) + name);
  mkdirSync(folder, { recursive: true });
  const file = join(folder, JSONF);
  const previous = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : {};
  writeFileSync(file, JSON.stringify({ snapshot, expected: previous.expected ?? {}, meshes: previous.meshes ?? {} }, null, 2) + "\n");
  console.log(`wrote ${name}`);
}

const node = (value: unknown): Node => {
  if (typeof value === "number") return raw(Number.isInteger(value) ? `${value}.0` : `${value}`);
  if (typeof value === "string") return string_(value);
  if (typeof value === "boolean") return raw(String(value));
  if (Array.isArray(value)) return { k: "arr", v: value.map(node) };
  return { k: "obj", v: Object.entries(value as Record<string, unknown>).map(([key, item]) => [key, node(item)] as [string, Node]) };
};

function hang() {
  const file = join(rooms, readdirSync(rooms).find((name) => name.endsWith("snapshot"))!, JSONF);
  const document = parse(readFileSync(file, "utf8"));
  if (document.k !== "obj") throw new Error("the rooms model is no object");
  const set = (key: string, value: unknown) => {
    const slot = document.v.find(([name]) => name === key);
    if (slot) slot[1] = node(value);
    else document.v.push([key, node(value)]);
  };
  const materials = field(document, "materials");
  if (materials?.k === "obj" && !field(materials, "m-board")) materials.v.push(["m-board", node(material("Plasterboard", "Finish", 0.9, 0.9, 0.88, 800))]);
  set("ceiling_types", { "cet-board": { name: "Board 12.5", layers: [layer("m-board", 0.0125)] }, "cet-deep": { name: "Board 50", layers: [layer("m-board", 0.05)] } });
  set("ceilings", {
    "ce-living": { storey: "st-ground", ceiling_type: "cet-board", boundary: rect(0, 0, 5, 6), holes: [], offset: 0.4, name: "Living ceiling" },
    "ce-kitchen": { storey: "st-ground", ceiling_type: "cet-deep", boundary: rect(5, 0, 8, 6), holes: [rect(7, 4.5, 7.5, 5.5)], offset: 0.2, slope: { direction: 0.0, angle: 0.1 }, name: "Kitchen sloped ceiling" },
    "ce-upper": { storey: "st-first", ceiling_type: "cet-deep", boundary: rect(0, 0, 4, 3), holes: [], offset: 0.0, name: "Upper ceiling" },
  });
  writeFileSync(file, print(document) + "\n");
  console.log("hung the ceilings over the committed rooms model");
}

write(element, "ceilings-holes-slope", solids);
hang();
