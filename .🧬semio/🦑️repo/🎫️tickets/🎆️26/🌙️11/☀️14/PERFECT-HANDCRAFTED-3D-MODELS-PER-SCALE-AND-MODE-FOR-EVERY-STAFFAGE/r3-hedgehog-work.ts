/** 🦔️ Round 3 hedgehog: the working asset (the catalog asset with the corrected roots `rig-members.json` and the redrawn `reps.json` of `🗑️generated/r3-hedgehog` and the author's source volumes laid over it), so that rig, drawings and solids are developed together and the catalog file is written only when the whole is valid.
 * @see ./r3-hedgehog-rig.ts
 * @see ./r3-animals-hedgehog.md */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { GENERATED, entryOf, loadSource, type Json } from "./volumes-lib.ts";

export const OUT = join(GENERATED, "r3-hedgehog");
export const SPEC = "animals/hedgehog";

export function working(parts: readonly string[] = ["rig-members", "reps"]): Json {
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
