/** 🖨️ Lists the print-suitability violations of the candidate document of `humans/kneeling-gardener` at a scale under the default print profile.
 * Usage: `bun r3-kneeling-gardener-suit.ts <scale> [file]` */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { CANDIDATE } from "./r3-kneeling-gardener-figure.ts";
const asset = JSON.parse(readFileSync(process.argv[3] ?? CANDIDATE, "utf8")).asset;
const scale = Number(process.argv[2]);
const rep = asset.representations[String(scale)];
for (const end of new Set([scale, rep.validRange.minDenominator, rep.validRange.maxDenominator])) {
  const s = measureSuitability(asset, rep, end, DEFAULT_PRINT_PROFILE as never);
  console.log("1:" + end, s.suitable, s.minStrokeMm, s.minGapMm, s.minFeatureMm);
  for (const v of s.violations) console.log(JSON.stringify(v).slice(0, 400));
}
