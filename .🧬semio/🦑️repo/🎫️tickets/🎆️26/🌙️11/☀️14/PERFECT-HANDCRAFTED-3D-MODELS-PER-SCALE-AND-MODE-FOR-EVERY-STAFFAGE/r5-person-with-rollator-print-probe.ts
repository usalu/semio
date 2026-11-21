/** 🖨️ Finds which pose and viewpoint of an asset document make a print-profile violation: measures the suitability of a band at a denominator with one pose at a time and one viewpoint's geometry at a time, against the default print profile.
 *
 * Usage: `bun r5-person-with-rollator-print-probe.ts <asset document json> <band> <denominator>`
 * @see ./r3-tools.md */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, band, den] = process.argv.slice(2) as [string, string, string];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
for (const pose of asset.poses) for (const view of ["elevation", "front", "plan"]) {
  const rep = structuredClone(asset.representations[band]);
  for (const v of ["elevation", "front", "plan"]) if (v !== view) rep.geometry[v] = [];
  const a = { ...asset, poses: [pose], representations: { ...asset.representations, [band]: rep } };
  const s = measureSuitability(a, rep, Number(den), DEFAULT_PRINT_PROFILE);
  for (const v of s.violations) console.log(pose.id, view, v.check, v.shapes.join("+"), v.measuredMm.toFixed(3));
}
