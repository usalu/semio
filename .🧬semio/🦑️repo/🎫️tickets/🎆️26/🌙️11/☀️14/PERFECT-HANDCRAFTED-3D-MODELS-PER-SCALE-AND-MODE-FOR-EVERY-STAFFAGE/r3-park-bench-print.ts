/** 🖨️ Round 3 park-bench: the product catalog check (`checkAssetDetailed`: validateAsset plus the print suitability of every authored representation under the default profile) of the candidate document, one line per problem and per violation.
 *
 * Usage: `bun r3-park-bench-print.ts [asset document json]` (default 🗑️generated/r3-park-bench/🔣️.json) */
import { readFileSync } from "node:fs";

import { checkAssetDetailed } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts";

const file = process.argv[2] ?? "🗑️generated/r3-park-bench/🔣️.json";
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const report = checkAssetDetailed(asset as never, { samples: 3 });
for (const p of report.problems) console.log("PROBLEM", p);
for (const s of report.scales) {
  console.log(`1:${s.denominator} ${s.status} suitable=${s.suitability?.suitable} minGap=${s.suitability?.minGapMm} minFeature=${s.suitability?.minFeatureMm} ${s.error ?? ""}`);
  for (const v of s.suitability?.violations ?? []) console.log("  ", JSON.stringify(v));
}
