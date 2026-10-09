#!/usr/bin/env bun
/**
 * 🕰️ Wave W04: sets the construction phase of a few elements of the committed IFC export house (`🧫️fixtures/🏗️ifc/🏠️house/📸️snapshot`), so the
 * exported file carries `Semio_Authoring.Phase` rows the IfcOpenShell oracle audits: `bun r10-w04-phase-house.ts`. Idempotent. Re-bless the committed
 * file afterwards: `BIM_BLESS=1 cargo test … projection`.
 */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { field, parse, print, string_, type Node } from "./r6-z-mutations-rawjson.ts";
import { child, fixtures, JSONF } from "./r3-f1-paths.ts";

const PHASES: Record<string, Record<string, string>> = {
  walls: { "w-first-south": "Demolished" },
  curtain_walls: { "cw-1": "Demolished" },
  columns: { "c-2": "Temporary" },
  beams: { "b-2": "Demolished" },
  slabs: { "sl-balcony": "Existing" },
  stairs: { "s-2": "Demolished" },
  railings: { "rl-1": "Temporary" },
  spaces: { "sp-2": "Existing" },
};

const house = join(child(child(child(fixtures, "ifc"), "house"), "snapshot"), JSONF);
const root = parse(readFileSync(house, "utf8"));
let changed = 0;
for (const [collection, rows] of Object.entries(PHASES)) {
  const table = field(root, collection);
  if (!table || table.k !== "obj") throw new Error(`no ${collection}`);
  for (const [id, phase] of Object.entries(rows)) {
    const record = field(table, id);
    if (!record || record.k !== "obj") throw new Error(`no ${id}`);
    const slot = record.v.find(([key]) => key === "phase");
    if (!slot) throw new Error(`${id} carries no phase`);
    const next: Node = string_(phase);
    if (print(slot[1]) !== print(next)) {
      slot[1] = next;
      changed++;
    }
  }
}
writeFileSync(house, print(root) + "\n");
console.log(`phases set on ${changed} elements of ${house}`);
