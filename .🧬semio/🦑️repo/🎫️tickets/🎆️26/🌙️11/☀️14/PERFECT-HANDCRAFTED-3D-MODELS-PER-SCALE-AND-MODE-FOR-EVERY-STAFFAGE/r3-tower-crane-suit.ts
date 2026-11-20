/** 🖨️ Print suitability of every authored scale of `machinery/tower-crane` (catalog file, or a candidate document): lists the profile violations that would make the product refuse to draw a scale.
 *
 * Usage: `bun r3-tower-crane-suit.ts [<asset document json>]`
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const file = process.argv[2] ?? "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🏗️machinery/tower-crane/🔣️.json";
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
for (const d of ["50", "100", "200", "500"]) {
  const s = measureSuitability(asset, asset.representations[d], Number(d), DEFAULT_PRINT_PROFILE as never);
  process.stdout.write(`1:${d} suitable ${s.suitable} minStroke ${s.minStrokeMm} minGap ${s.minGapMm} minFeature ${s.minFeatureMm}\n`);
  for (const v of s.violations) process.stdout.write(`  ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}\n`);
}
