/** 🧰️ Round 5 Labrador: the working asset (the catalog asset with the corrected roots and redrawn representations of `🗑️generated/r5-labrador-dog/*.json` and the author's source volumes laid over it), so rig, drawings and solids are developed together before anything is written to the catalog.
 * @see ./r5-labrador-dog-rig.py */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, entryOf, loadSource, type Json } from "./volumes-lib.ts";

export const OUT = join(GENERATED, "r5-labrador-dog");
export const SPEC = "animals/labrador-dog";

export function working(parts: readonly string[] = ["roots", "reps"]): Json {
  const { entry, asset } = entryOf(SPEC);
  const out: Json = { ...asset };
  for (const part of parts) {
    const file = join(OUT, `${part}.json`);
    if (existsSync(file)) Object.assign(out, JSON.parse(readFileSync(file, "utf8")));
  }
  const bones = new Set((out.rig.bones as Json[]).map((b) => b.id));
  const kept = (list: Json[]): Json[] => list.filter((s) => bones.has(s.bone) && (s.solid?.axisBone === undefined || bones.has(s.solid.axisBone)));
  out.representations = Object.fromEntries(Object.entries(out.representations as Record<string, Json>).map(([k, rep]) => [k, rep.geometry === undefined ? rep : { ...rep, geometry: { elevation: kept(rep.geometry.elevation), front: kept(rep.geometry.front), plan: kept(rep.geometry.plan) } }]));
  const source = loadSource(entry);
  if (source !== null) out.volumes = source.volumes;
  return out;
}
