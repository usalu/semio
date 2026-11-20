/** 📂️ Loads `humans/person-with-white-cane`: the candidate document `🗑️generated/r3-person-with-white-cane/🔣️.json` (root fixes applied) when it exists, else the catalog asset, optionally with the volumes source merged over it.
 * @see ./r3-person-with-white-cane-roots.ts */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED, SOURCES, entryOf, type Json } from "./volumes-lib.ts";

export const SPEC = "humans/person-with-white-cane";
export const OUT = join(GENERATED, "r3-person-with-white-cane");
export const CANDIDATE = join(OUT, "🔣️.json");
export const SOURCE = join(SOURCES, "humans", "person-with-white-cane.json");

export function candidate(withSource = true): Json {
  const asset = existsSync(CANDIDATE) ? (JSON.parse(readFileSync(CANDIDATE, "utf8")).asset as Json) : (entryOf(SPEC).asset as Json);
  if (withSource && existsSync(SOURCE)) asset.volumes = JSON.parse(readFileSync(SOURCE, "utf8")).volumes;
  return asset;
}
