#!/usr/bin/env bun
/**
 * 🧬 Wave W2 `w2-f2-families`: writes the committed snapshots of the family inference corpus under `🧫️fixtures/💡️inferences/🧬️families/`: the `🪑️table` model (a table with every parameter kind, solid shape and
 * kind of fault, a wide-flange profile, a rectangular and a round profile) that the python oracle adjudicates, and the `🏛️frame` model (columns, beams and a railing whose profiles are profile families)
 * the graph tests read. `bun r12-w2-f2-families-fixtures.ts` rewrites both; the expectation `💡️inference/📏️families/🔣️.json` of the oracle case is written by the oracle, never by hand.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import * as F from "./r3-f1-fixtures.ts";
import { em, fixtures, JSONF } from "./r3-f1-paths.ts";

const zero = { x: "0 m", y: "0 m", z: "0 m" };
const offset = (x: string, y: string, z: string) => ({ x, y, z });
const point = (x: string, y: string) => ({ x, y });
const rectangle = (width: string, depth: string) => ({ Rectangle: { width, depth } });
const extrusion = (profile: unknown, base: string, height: string) => ({ Extrusion: { profile, base, height } });
const cuboid = (x: string, y: string, z: string, width: string, depth: string, height: string) => ({ Cuboid: { x, y, z, width, depth, height } });
const family = (name: string, category: string) => ({ name, category });
const parameter = (owner: string, name: string, kind: string, value: string) => [`${owner}.${name}`, { family: owner, name, kind, value }] as const;
const solid = (owner: string, name: string, shape: unknown, extra: Record<string, unknown> = {}) => ({ family: owner, name, shape, material: "\"m-oak\"", visible: "true", offset: zero, ...extra });
const material = (name: string, category: string, density: number) => ({ name, category, color: { r: 0.6, g: 0.45, b: 0.3 }, density, conductivity: 0.2, specific_heat: 1600 });
const rows = (...entries: (readonly [string, unknown])[]) => Object.fromEntries(entries);

const table = rows(
  parameter("fam-table", "width", "Length", "1.6 m"),
  parameter("fam-table", "depth", "Length", "0.8 m"),
  parameter("fam-table", "height", "Length", "0.74 m"),
  parameter("fam-table", "top_thickness", "Length", "30 mm"),
  parameter("fam-table", "leg_section", "Length", "60 mm"),
  parameter("fam-table", "leg_inset", "Length", "50 mm"),
  parameter("fam-table", "leg_height", "Length", "height - top_thickness"),
  parameter("fam-table", "leg_x", "Length", "width - leg_inset - leg_section"),
  parameter("fam-table", "leg_y", "Length", "depth - leg_inset - leg_section"),
  parameter("fam-table", "legs", "Integer", "4"),
  parameter("fam-table", "wide", "Boolean", "width > 1.4 m"),
  parameter("fam-table", "ratio", "Real", "width / depth"),
  parameter("fam-table", "area", "Real", "round(width * depth / 1 m2 * 100) / 100"),
  parameter("fam-table", "tilt", "Angle", "if wide then 15 deg else 0 deg"),
  parameter("fam-table", "finish", "Material", "\"m-oak\""),
  parameter("fam-table", "label", "Text", "\"Dining table\""),
  parameter("fam-hea", "h", "Length", "190 mm"),
  parameter("fam-hea", "b", "Length", "200 mm"),
  parameter("fam-hea", "tw", "Length", "6.5 mm"),
  parameter("fam-hea", "tf", "Length", "10 mm"),
  parameter("fam-rhs", "width", "Length", "100 mm"),
  parameter("fam-rhs", "depth", "Length", "60 mm"),
  parameter("fam-round", "diameter", "Length", "114 mm"),
  parameter("fam-broken", "base", "Length", "1 m"),
  parameter("fam-broken", "loop_a", "Length", "loop_b + 1 m"),
  parameter("fam-broken", "loop_b", "Length", "loop_a + 1 m"),
  parameter("fam-broken", "after_loop", "Length", "loop_a * 2"),
  parameter("fam-broken", "syntax", "Length", "2 *"),
  parameter("fam-broken", "after_syntax", "Length", "syntax + 1 m"),
  parameter("fam-broken", "unknown", "Length", "ghost + 1 m"),
  parameter("fam-broken", "zero", "Real", "1 / 0"),
  parameter("fam-broken", "mixed", "Length", "1 m + 30 deg"),
  parameter("fam-broken", "number_for_length", "Length", "2"),
  parameter("fam-broken", "fraction", "Integer", "2.5"),
  parameter("fam-broken", "no_material", "Material", "\"m-ghost\""),
  parameter("fam-broken", "negative", "Length", "-1 m"),
);

const tableSolids = {
  "s-top": solid("fam-table", "Top", cuboid("0 m", "0 m", "leg_height", "width", "depth", "top_thickness"), { material: "finish" }),
  "s-leg-1": solid("fam-table", "Leg 1", cuboid("leg_inset", "leg_inset", "0 m", "leg_section", "leg_section", "leg_height")),
  "s-leg-2": solid("fam-table", "Leg 2", cuboid("leg_x", "leg_inset", "0 m", "leg_section", "leg_section", "leg_height")),
  "s-leg-3": solid("fam-table", "Leg 3", cuboid("leg_x", "leg_y", "0 m", "leg_section", "leg_section", "leg_height")),
  "s-leg-4": solid("fam-table", "Leg 4", cuboid("leg_inset", "leg_y", "0 m", "leg_section", "leg_section", "leg_height")),
  "s-shelf": solid("fam-table", "Shelf", extrusion(rectangle("width - 2 * leg_inset", "depth - 2 * leg_inset"), "0 m", "20 mm"), { visible: "width > 2 m", offset: offset("width / 2", "depth / 2", "150 mm") }),
  "s-foot": solid("fam-table", "Foot", { Revolution: { profile: { Polygon: { points: [point("0 m", "0 m"), point("30 mm", "0 m"), point("25 mm", "15 mm"), point("0 m", "15 mm")] } }, axis: "Z", angle: "360 deg" } }, { offset: offset("leg_inset + leg_section / 2", "leg_inset + leg_section / 2", "0 m") }),
  "s-rail": solid("fam-table", "Rail", { Sweep: { profile: rectangle("20 mm", "20 mm"), path: [point("0 m", "0 m"), point("width", "0 m"), point("width", "depth")] } }, { offset: offset("0 m", "0 m", "leg_height - 30 mm") }),
  "s-hea": solid("fam-hea", "HEA 200", extrusion({ IShape: { width: "b", depth: "h", web: "tw", flange: "tf" } }, "0 m", "1 m"), { material: "\"m-steel\"" }),
  "s-rhs": solid("fam-rhs", "RHS", extrusion(rectangle("width", "depth"), "0 m", "1 m"), { material: "\"m-steel\"" }),
  "s-round": solid("fam-round", "Pipe", extrusion({ Circle: { diameter: "diameter" } }, "0 m", "1 m"), { material: "\"m-steel\"" }),
  "s-bad-height": solid("fam-broken", "Flat", cuboid("0 m", "0 m", "0 m", "base", "base", "negative")),
  "s-bad-name": solid("fam-broken", "Ghost", cuboid("0 m", "0 m", "0 m", "ghost", "base", "base")),
  "s-bad-loop": solid("fam-broken", "Loop", cuboid("0 m", "0 m", "0 m", "loop_a", "base", "base")),
  "s-bad-material": solid("fam-broken", "Material", cuboid("0 m", "0 m", "0 m", "base", "base", "base"), { material: "\"m-ghost\"" }),
};

const tableModel = () => ({
  ...F.snap({ materials: { "m-oak": material("Oak", "Wood", 700), "m-steel": material("Steel", "Metal", 7850) } }, "Families"),
  families: { "fam-table": family("Table", "Furniture"), "fam-hea": family("HEA 200", "Profile"), "fam-rhs": family("RHS 100x60", "Profile"), "fam-round": family("Pipe 114", "Profile"), "fam-broken": family("Broken", "Generic") },
  family_parameters: table,
  family_solids: tableSolids,
});

const profileFamily = (name: string) => ({ Family: { family: name } });
const frameModel = () => ({
  ...F.scene(),
  materials: { "m-brick": F.material("Brick"), "m-steel": material("Steel", "Metal", 7850), "m-oak": material("Oak", "Wood", 700) },
  families: { "fam-hea": family("HEA 200", "Profile"), "fam-rhs": family("RHS 100x60", "Profile"), "fam-table": family("Table", "Furniture") },
  family_parameters: rows(
    parameter("fam-hea", "h", "Length", "190 mm"), parameter("fam-hea", "b", "Length", "200 mm"), parameter("fam-hea", "tw", "Length", "6.5 mm"), parameter("fam-hea", "tf", "Length", "10 mm"),
    parameter("fam-rhs", "width", "Length", "100 mm"), parameter("fam-rhs", "depth", "Length", "60 mm"),
    parameter("fam-table", "width", "Length", "1.6 m"),
  ),
  family_solids: {
    "s-hea": tableSolids["s-hea"],
    "s-rhs": tableSolids["s-rhs"],
    "s-top": solid("fam-table", "Top", cuboid("0 m", "0 m", "0 m", "width", "width", "30 mm")),
  },
  column_types: { "ct-hea": { name: "HEA column", profile: profileFamily("fam-hea"), material: "m-steel" }, "ct-plain": { name: "Plain column", profile: { Rectangle: { width: 0.2, depth: 0.2 } }, material: "m-steel" } },
  beam_types: { "bt-rhs": { name: "RHS beam", profile: profileFamily("fam-rhs"), material: "m-steel" } },
  columns: {
    "col-hea": { storey: "st-ground", column_type: "ct-hea", position: F.P(1, 1), rotation: 0, base_offset: 0, top: F.storeyTop(0), phase: "New", name: "HEA column" },
    "col-plain": { storey: "st-ground", column_type: "ct-plain", position: F.P(3, 1), rotation: 0, base_offset: 0, top: F.storeyTop(0), phase: "New", name: "Plain column" },
  },
  beams: { "beam-rhs": { storey: "st-ground", beam_type: "bt-rhs", axis: F.line([1, 1], [3, 1]), top_offset: 0, phase: "New", name: "RHS beam" } },
  railings: { "rail-rhs": { storey: "st-ground", path: [F.P(0, 4), F.P(3, 4)], height: 1, post_spacing: 1.5, profile: profileFamily("fam-rhs"), post_profile: { Rectangle: { width: 0.05, depth: 0.05 } }, infill: "None", material: "m-steel", base_offset: 0, phase: "New", name: "Rail" } },
});

const out = join(fixtures, em(0x1f4a1) + "inferences", em(0x1f9ec) + "families");
for (const [name, emoji, model] of [["table", 0x1fa91, tableModel()], ["frame", 0x1f3db, frameModel()]] as const) {
  const dir = join(out, em(emoji) + name, em(0x1f4f8) + "snapshot");
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, JSONF), JSON.stringify(model, null, 2) + "\n");
}
console.log("families fixtures written");
