#!/usr/bin/env bun
/** 🔲️ Writes the committed IFC case `🧫️fixtures/🏗️ifc/🔲️ceilings/📸️snapshot/🔣️.json` the three.js mesh case `🧫️fixtures/💡️inferences/🧊️element-solids/🪵️ceilings-meshes/🔣️.json` and the take-off case `🧫️fixtures/💡️inferences/🔲️ceilings/🔲️takeoff` from the committed ceiling solid case (the ceilings that cannot be written, a bare type and an unknown type, are left out). Usage: `bun r11-w06-ceilings-ifc-case.ts`. */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../../../../../..");
const fixtures = join(root, "✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures");
const source = JSON.parse(readFileSync(join(fixtures, "💡️inferences/🧊️element-solids/🔲️ceilings-holes-slope/🔣️.json"), "utf8"));

const writable = (ids: string[]) => {
  const snapshot = structuredClone(source.snapshot);
  for (const id of Object.keys(snapshot.ceilings)) if (!ids.includes(id)) delete snapshot.ceilings[id];
  return snapshot;
};
const write = (path: string, value: unknown) => {
  mkdirSync(join(path, ".."), { recursive: true });
  writeFileSync(path, JSON.stringify(value, null, 2) + "\n", "utf8");
};

const exportable = ["c-holed", "c-sloped", "c-diagonal-slope", "c-curved"];
const complete = Object.fromEntries(["walls", "curtain_walls", "columns", "beams", "slabs", "roofs", "openings", "stairs", "railings", "spaces", "grids", "wall_types", "slab_types", "roof_types", "column_types", "beam_types", "window_types", "door_types"].map((name) => [name, {}]));
write(join(fixtures, "🏗️ifc/🔲️ceilings/📸️snapshot/🔣️.json"), { ...complete, ...writable(exportable) });

const meshed = ["c-holed", "c-sloped", "c-diagonal-slope"];
const expected = Object.fromEntries(meshed.map((id) => [id, source.expected[id]]));
write(join(fixtures, "💡️inferences/🧊️element-solids/🪵️ceilings-meshes/🔣️.json"), { snapshot: writable(meshed), expected, meshes: {} });
write(join(fixtures, "💡️inferences/🔲️ceilings/🔲️takeoff/📸️snapshot/🔣️.json"), source.snapshot);
console.log("wrote the IFC case, the mesh case and the take-off case");
