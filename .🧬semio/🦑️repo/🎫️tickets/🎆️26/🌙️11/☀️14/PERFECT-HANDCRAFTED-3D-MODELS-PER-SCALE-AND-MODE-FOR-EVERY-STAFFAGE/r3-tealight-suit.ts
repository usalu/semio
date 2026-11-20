/** 🖨️ Prints the print-profile findings of a tealight asset document at every preset and range end (the product's `validateRanges`), for judging 2D root fixes.
 *
 * Usage: `bun r3-tealight-suit.ts <asset document json>` */
import { readFileSync } from "node:fs";

import { validateRanges } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
for (const issue of validateRanges(asset, DEFAULT_PRINT_PROFILE)) console.log(issue.path, issue.code, issue.message.en.slice(0, 160));
for (const scale of ["50", "100", "200"]) {
  const s = measureSuitability(asset, asset.representations[scale], Number(scale), DEFAULT_PRINT_PROFILE);
  console.log(scale, s.suitable, "minFeature", s.minFeatureMm.toFixed(3), "minGap", s.minGapMm, "minStroke", s.minStrokeMm);
}
