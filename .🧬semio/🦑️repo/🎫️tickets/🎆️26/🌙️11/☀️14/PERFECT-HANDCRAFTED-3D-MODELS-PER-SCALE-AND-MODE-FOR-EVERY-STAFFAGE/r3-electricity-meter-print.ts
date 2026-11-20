/** 🖨️ Print suitability of every authored representation of a candidate or catalog document of appliances/electricity-meter under the default print profile (why `r3-2d.ts` reports "no suitable representation"). Usage: bun r3-electricity-meter-print.ts [asset document json] */
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";

const path = process.argv[2] ?? "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/electricity-meter/🔣️.json";
const asset = loadAsset(path) as any;
for (const scale of ["50", "100", "200"]) {
  const result = measureSuitability(asset, asset.representations[scale], Number(scale), DEFAULT_PRINT_PROFILE as any);
  console.log(scale, result.suitable, "stroke", result.minStrokeMm, "gap", result.minGapMm, "feature", result.minFeatureMm);
  for (const violation of result.violations) console.log("  ", JSON.stringify(violation));
}
