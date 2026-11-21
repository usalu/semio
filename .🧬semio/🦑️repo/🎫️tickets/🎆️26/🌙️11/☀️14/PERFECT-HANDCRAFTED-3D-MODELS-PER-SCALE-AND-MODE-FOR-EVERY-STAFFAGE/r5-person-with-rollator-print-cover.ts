/** 🕵️ Finds which drawn shape makes a print-feature violation: removes one candidate shape from one viewpoint at a time and reports whether the violation of the target shape disappears (band, denominator and pose as given).
 *
 * Usage: `bun r5-print-cover.ts <asset document json> <band> <denominator> <pose> <target shape> <candidate ids, comma separated>`
 * @see ./r5-person-with-rollator-print-probe.ts */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const [file, band, den, pose, target, ids] = process.argv.slice(2) as [string, string, string, string, string, string];
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const p = asset.poses.find((x: { id: string }) => x.id === pose);
const worst = (rep: { geometry: Record<string, { id: string }[]> }): string => {
  const a = { ...asset, poses: [p], representations: { ...asset.representations, [band]: rep } };
  const v = measureSuitability(a, rep as never, Number(den), DEFAULT_PRINT_PROFILE).violations.find((x) => x.shapes.includes(target));
  return v ? v.measuredMm.toFixed(3) : "ok";
};
console.log("as drawn", worst(asset.representations[band]));
for (const id of ids.split(",")) for (const view of ["elevation", "front", "plan"]) {
  const rep = structuredClone(asset.representations[band]);
  const n = rep.geometry[view].length;
  rep.geometry[view] = rep.geometry[view].filter((s: { id: string }) => s.id !== id);
  if (rep.geometry[view].length === n) continue;
  console.log(`without ${id} in ${view}: ${worst(rep)}`);
}
