/** 🖨️ Prints the print suitability of every authored representation of `humans/person-with-white-cane` (catalog asset, or the candidate with `--candidate`, or any asset document with `--file`): violations with their measured values.
 * Usage: `bun r5-person-with-white-cane-suit.ts [--candidate | --file <json>]` */
import { readFileSync } from "node:fs";
import { resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import * as rep from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf, type Json } from "./volumes-lib.ts";
const at = process.argv.indexOf("--file");
const path = at > 0 ? process.argv[at + 1]! : process.argv.includes("--candidate") ? "🗑️generated/r5-person-with-white-cane/🔣️.json" : entryOf("humans/person-with-white-cane").entry.path;
const asset = JSON.parse(readFileSync(path, "utf8")).asset as Json;
const profile = (rep as Json).DEFAULT_PRINT_PROFILE ?? (rep as Json).defaultPrintProfile ?? Object.values(rep as Json).find((v: Json) => v && typeof v === "object" && "minStrokeMm" in v);
for (const scale of [50, 100, 200, 500]) {
  const r = resolveRepresentation(asset as never, { denominator: scale } as never, profile as never) as Json;
  console.log(scale, r.kind, JSON.stringify(r.suitability?.violations ?? []).slice(0, 1500), r.suitability?.minStrokeMm, r.suitability?.minGapMm, r.suitability?.minFeatureMm);
}
