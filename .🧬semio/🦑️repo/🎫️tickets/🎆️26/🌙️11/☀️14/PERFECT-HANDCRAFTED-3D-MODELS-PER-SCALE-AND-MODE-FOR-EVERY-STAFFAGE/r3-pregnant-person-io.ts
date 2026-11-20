/** 📂️ Loads `humans/pregnant-person`: the candidate document `🗑️generated/r3-pregnant-person/🔣️.json` (root fixes applied) when it exists, else the catalog asset, optionally with the volumes source merged over it.
 * @see ./r3-pregnant-person-roots.ts */
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED, SOURCES, entryOf, type Json } from "./volumes-lib.ts";

export const SPEC = "humans/pregnant-person";
export const OUT = join(GENERATED, "r3-pregnant-person");
export const CANDIDATE = join(OUT, "🔣️.json");
export const SOURCE = join(SOURCES, "humans", "pregnant-person.json");

export function candidate(withSource = true): Json {
  const asset = existsSync(CANDIDATE) ? (JSON.parse(readFileSync(CANDIDATE, "utf8")).asset as Json) : (entryOf(SPEC).asset as Json);
  if (withSource && existsSync(SOURCE)) asset.volumes = JSON.parse(readFileSync(SOURCE, "utf8")).volumes;
  return asset;
}
