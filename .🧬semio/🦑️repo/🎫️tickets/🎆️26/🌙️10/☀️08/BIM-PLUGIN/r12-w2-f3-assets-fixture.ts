#!/usr/bin/env bun
/**
 * 🪑️ Writes the committed snapshot of the IFC components case (`🧫️fixtures/🏗️ifc/🪑️components/📸️snapshot/🔣️.json`): a one-room model with every category of family, free and wall-hosted components (one
 * rotated, one mirrored, one with overrides), terminals of several systems, and ducts, pipes and trays with straight, mitred and vertical runs. The IfcOpenShell oracle `🏗️export-bim-1-ifc/🐍️.py` reads
 * the file the subject exports from it; the numbers below are the only authored input.
 * Run: `bun T/r12-w2-f3-assets-fixture.ts`.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { child, em, fixtures } from "./r3-f1-paths.ts";
import * as F from "./r3-f1-fixtures.ts";
import { tableFamily, cuboid } from "./r12-w2-f2-families-examples.ts";
import { P, basin, chair, component, diffuser, duct, family, kitchen, lamp, merge, override, pipe, tray, wc } from "./r12-w2-f3-assets-examples.ts";

type Json = Record<string, any>;

const MATERIALS: [string, string, string, [number, number, number]][] = [
  ["m-oak", "Oak", "Wood", [0.62, 0.45, 0.28]],
  ["m-steel", "Steel", "Metal", [0.55, 0.58, 0.62]],
  ["m-ceramic", "Ceramic", "Finish", [0.88, 0.88, 0.85]],
  ["m-paint", "Paint", "Finish", [0.96, 0.95, 0.92]],
  ["m-timber", "Timber", "Wood", [0.8, 0.62, 0.4]],
  ["m-plaster", "Plaster", "Finish", [0.92, 0.92, 0.88]],
];

const panel = () =>
  family("fam-panel", "Distribution board", "Electrical", [
    ["width", "Length", "0.6 m"],
    ["depth", "Length", "0.2 m"],
    ["height", "Length", "1.2 m"],
  ], [["Cabinet", cuboid("0 m - width / 2", "0 m", "0 m", "width", "depth", "height"), "m-steel"]]);

const pillar = () =>
  family("fam-pillar", "Pillar", "Generic", [
    ["side", "Length", "0.3 m"],
    ["height", "Length", "2.2 m"],
  ], [["Shaft", cuboid("0 m - side / 2", "0 m - side / 2", "0 m", "side", "side", "height"), "m-timber"]]);

const rig = () =>
  family("fam-rig", "Fan unit", "Mechanical", [
    ["width", "Length", "0.8 m"],
    ["depth", "Length", "0.5 m"],
    ["height", "Length", "0.4 m"],
  ], [["Housing", cuboid("0 m - width / 2", "0 m - depth / 2", "0 m", "width", "depth", "height"), "m-steel"]]);

const families = merge(tableFamily("m-oak"), chair(), kitchen(), wc(), basin(), lamp(), diffuser(), panel(), pillar(), rig());

const components: Json = {
  "cmp-table": component("st-ground", "fam-table", P(1.5, 1.5), 0, 0.5, "Table"),
  "cmp-chair-free": component("st-ground", "fam-chair", P(3.0, 3.0), 0, Math.PI / 2, "Free Chair"),
  "cmp-chair-mirrored": { ...component("st-ground", "fam-chair", P(4.5, 3.0), 0, 0.3, "Mirrored Chair"), mirrored: true },
  "cmp-kitchen": component("st-ground", "fam-kitchen", P(3.0, 5.7), 0, 0, "Kitchen Unit", { host: "w-north" }),
  "cmp-wc": component("st-ground", "fam-wc", P(0.5, 4.0), 0, 0, "WC", { host: "w-west" }),
  "cmp-basin": component("st-ground", "fam-basin", P(7.5, 3.0), 0.8, 0, "Basin", { host: "w-east", system: "Waste" }),
  "cmp-lamp": component("st-ground", "fam-lamp", P(4.0, 2.0), 2.6, 0, "Lamp", { system: "Lighting" }),
  "cmp-diffuser": component("st-ground", "fam-diffuser", P(6.0, 3.0), 2.8, 0, "Diffuser", { system: "Supply" }),
  "cmp-fan": component("st-ground", "fam-rig", P(2.0, 4.5), 0, 0.2, "Fan Unit"),
  "cmp-pillar": component("st-ground", "fam-pillar", P(6.5, 1.0), 0, 0, "Pillar"),
  "cmp-panel": component("st-ground", "fam-panel", P(7.0, 5.7), 0, 0, "Board", { host: "w-north", system: "Power" }),
  "cmp-table-first": component("st-first", "fam-table", P(2.0, 2.0), 0, 0, "Upper Table"),
};

const overrides = Object.fromEntries([
  override("cmp-table", "width", "1.8 m"),
  override("cmp-table", "depth", "0.9 m"),
  override("cmp-chair-mirrored", "seat_height", "0.5 m"),
  override("cmp-kitchen", "width", "2.1 m"),
]);

const mep: Json = {
  "mep-supply": duct("st-ground", "Supply", 0.4, 0.25, [[1, 3, 2.5], [6, 3, 2.5], [6, 3, 2.8]], "Supply Duct"),
  "mep-return": duct("st-ground", "Return", 0.3, 0.2, [[1, 4.5, 2.6], [7, 4.5, 2.6]], "Return Duct"),
  "mep-water": pipe("st-ground", "DomesticWater", 0.032, [[7.85, 5.0, 0.95], [7.85, 3.0, 0.95], [7.85, 3.0, 0.2]], "Water Pipe"),
  "mep-waste": pipe("st-ground", "Waste", 0.1, [[7.85, 3.0, 0.8], [7.85, 3.0, 0.1], [5.0, 3.0, 0.1]], "Waste Pipe"),
  "mep-gas": pipe("st-ground", "Gas", 0.025, [[7.5, 5.5, 0.0], [7.5, 5.5, 2.5]], "Gas Riser"),
  "mep-tray": tray("st-ground", "Power", 0.3, 0.06, [[1, 1, 2.9], [7, 1, 2.9], [7, 5, 2.9]], "Power Tray"),
  "mep-lighting": tray("st-ground", "Lighting", 0.1, 0.05, [[4, 2, 2.6], [4, 2, 2.9]], "Lighting Stub"),
};

const model = F.scene({
  walls: {
    "w-north": F.wall("st-ground", "wt-300", F.line([8, 6], [0, 6]), F.storeyTop(0), "North"),
    "w-west": F.wall("st-ground", "wt-300", F.line([0, 6], [0, 0]), F.storeyTop(0), "West"),
  },
}) as Json;
Object.assign(model.project, { name: "Components" });
Object.assign(model.materials, Object.fromEntries(MATERIALS.map(([id, name, category, [r, g, b]]) => [id, { name, category, color: { r, g, b }, density: 700, conductivity: 0.2, specific_heat: 1200 }])));
const plan = (storey: string, name: string) => ({ building: "bldg-1", name, kind: "Plan", storey, depth: 100, hidden: [], scale: 100, detail: "Medium" });
Object.assign(model, { ...families, components, component_overrides: overrides, mep_elements: mep, views: { "v-plan-ground": plan("st-ground", "Plan Ground"), "v-plan-first": plan("st-first", "Plan First") } });

const parameters = new Set(Object.values<Json>(model.family_parameters).map((row) => `${row.family}.${row.name}`));
for (const [id, row] of Object.entries<Json>(components)) {
  if (!(row.storey in model.storeys) || !(row.family in model.families)) throw new Error(`components/${id}: dangling storey or family`);
  if (row.host !== undefined && model.walls[row.host]?.storey !== row.storey) throw new Error(`components/${id}: host ${row.host}`);
}
for (const [key, row] of Object.entries<Json>(overrides)) if (!parameters.has(`${components[row.component].family}.${row.name}`)) throw new Error(`component_overrides/${key}: unknown parameter`);

const directory = join(child(fixtures, "ifc"), em(0x1fa91) + "components", em(0x1f4f8) + "snapshot");
mkdirSync(directory, { recursive: true });
writeFileSync(join(directory, em(0x1f523) + ".json"), JSON.stringify(model, null, 2) + "\n");
console.log(`components: ${Object.keys(components).length} components, ${Object.keys(overrides).length} overrides, ${Object.keys(mep).length} mep elements -> ${directory}`);
