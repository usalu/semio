/** 🖨️ Lists the print-suitability violations of the candidate document of `humans/seated-office-worker` at every scale and at the ends of its valid range under the default print profile.
 * Usage: `bun r3-seated-office-worker-suit.ts [file]` */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { CANDIDATE } from "./r3-seated-office-worker-build.ts";
const asset = JSON.parse(readFileSync(process.argv[2] ?? CANDIDATE, "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  const rep = asset.representations[String(scale)];
  for (const end of new Set([scale, rep.validRange.minDenominator, rep.validRange.maxDenominator])) {
    const s = measureSuitability(asset, rep, end, DEFAULT_PRINT_PROFILE as never);
    console.log("1:" + end, s.suitable, s.minStrokeMm, s.minGapMm, s.minFeatureMm, s.violations.length);
    for (const v of s.violations.slice(0, 6)) console.log("   ", JSON.stringify(v).slice(0, 300));
  }
}
