/** 📂️ Loads the candidate asset of `humans/parent-carrying-toddler` (root fixes applied, `🗑️generated/r3-parent-carrying-toddler/🔣️.json`) with the volumes source merged over it.
 * @see ./r3-parent-carrying-toddler-roots.py */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED, SOURCES, type Json } from "./volumes-lib.ts";

export const OUT = join(GENERATED, "r3-parent-carrying-toddler");
export const SOURCE = join(SOURCES, "humans", "parent-carrying-toddler.json");

export function candidate(withSource = true): Json {
  const asset = JSON.parse(readFileSync(join(OUT, "🔣️.json"), "utf8")).asset as Json;
  if (withSource && existsSync(SOURCE)) asset.volumes = JSON.parse(readFileSync(SOURCE, "utf8")).volumes;
  return asset;
}
