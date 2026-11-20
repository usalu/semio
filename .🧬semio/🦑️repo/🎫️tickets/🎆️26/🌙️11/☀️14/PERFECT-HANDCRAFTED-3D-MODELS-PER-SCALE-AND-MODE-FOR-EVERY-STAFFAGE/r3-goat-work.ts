/** 🧰️ Round 3 goat: the working asset (the catalog asset with the corrected roots of `🗑️generated/r3-goat/*.json` and the author's source volumes laid over it), so rig, drawings and solids are developed together before anything is written to the catalog.
 * @see ./r3-goat-rig.py */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, entryOf, loadSource, type Json } from "./volumes-lib.ts";

export const OUT = join(GENERATED, "r3-goat");
export const SPEC = "animals/goat";

export function working(parts: readonly string[] = ["roots", "reps", "extra"]): Json {
  const { entry, asset } = entryOf(SPEC);
  const out: Json = { ...asset };
  for (const part of parts) {
    const file = join(OUT, `${part}.json`);
    if (existsSync(file)) Object.assign(out, JSON.parse(readFileSync(file, "utf8")));
  }
  const source = loadSource(entry);
  if (source !== null) out.volumes = source.volumes;
  return out;
}
