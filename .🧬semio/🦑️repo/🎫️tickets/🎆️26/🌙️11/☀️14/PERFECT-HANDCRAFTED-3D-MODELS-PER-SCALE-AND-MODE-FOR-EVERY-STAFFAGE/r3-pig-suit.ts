/** 🖨️ Lists the print-suitability violations of animals/pig (candidate document or the catalog file) per scale at the nominal denominator and both ends of `validRange`, under the default print profile.
 * Usage: bun r3-pig-suit.ts [file] */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";
const asset = JSON.parse(readFileSync(process.argv[2] ?? entryOf("animals/pig").entry.path, "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  const rep = asset.representations[String(scale)];
  for (const end of new Set([scale, rep.validRange.minDenominator, rep.validRange.maxDenominator])) {
    const s = measureSuitability(asset, rep, end, DEFAULT_PRINT_PROFILE as never);
    console.log(`1:${scale} at 1:${end}`, s.suitable, s.minStrokeMm, s.minGapMm, s.minFeatureMm);
    for (const v of s.violations.slice(0, 8)) console.log("  ", JSON.stringify(v).slice(0, 300));
  }
}
