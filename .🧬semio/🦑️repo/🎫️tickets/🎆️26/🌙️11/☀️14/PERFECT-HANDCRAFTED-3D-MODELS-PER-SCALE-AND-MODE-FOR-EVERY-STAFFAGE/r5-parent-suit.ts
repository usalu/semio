/** 🧾 Round-5 print probe of `humans/parent-with-stroller` and `humans/parent-carrying-toddler`: the product's print suitability of one representation of a candidate document at given denominators, all viewpoints and then per viewpoint (the other two emptied), so a failing band can be traced to the view that makes it thin.
 * Usage: `bun r5-parent-suit.ts <asset document json> <scale> <denominator,...>`
 * @see ./r3-validate.ts */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale, ends] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const rep = asset.representations[scale!];
for (const end of ends!.split(",").map(Number)) for (const view of ["all", "elevation", "front", "plan"]) {
  const geometry = Object.fromEntries(Object.entries(rep.geometry).map(([k, v]) => [k, k === view || view === "all" ? v : []]));
  const r = { ...rep, geometry };
  const s = measureSuitability({ ...asset, representations: { ...asset.representations, [scale!]: r } }, r, end, DEFAULT_PRINT_PROFILE);
  for (const v of s.violations) console.log(`1:${end} ${view} ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}`);
}
