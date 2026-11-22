/** 🖨️ Print-suitability violations of the HW-18 humans per scale, with pose, view and paper position (diagnostic for the generators).
 *  bun suit.ts <slug> [scale ...] */
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { HUMANS } from "./lib.ts";

const [slug, ...scales] = process.argv.slice(2);
const asset = (await Bun.file(join(HUMANS, slug!, "🔣️.json")).json()).asset;
for (const s of scales.length ? scales : ["50", "100", "200", "500"]) {
  const rep = asset.representations[s];
  for (const end of [rep.validRange.minDenominator, Number(s), rep.validRange.maxDenominator]) {
    const r = measureSuitability(asset, rep, end, DEFAULT_PRINT_PROFILE);
    console.log(`1:${s} at 1:${end} suitable ${r.suitable} stroke ${r.minStrokeMm.toFixed(3)} gap ${r.minGapMm.toFixed(3)} feature ${r.minFeatureMm.toFixed(3)}`);
    for (const v of r.violations.slice(0, 12)) console.log("   ", JSON.stringify(v).slice(0, 300));
  }
}
if (process.env.PER_POSE) for (const s of scales.length ? scales : ["50", "100", "200", "500"]) {
  const rep = asset.representations[s];
  for (const end of [rep.validRange.minDenominator, Number(s), rep.validRange.maxDenominator])
    for (const pose of asset.poses) {
      const one = { ...asset, poses: [pose], clips: [], presentation: { ...asset.presentation, defaultPose: pose.id } };
      const r = measureSuitability(one, rep, end, DEFAULT_PRINT_PROFILE);
      if (!r.suitable) console.log(`  pose ${pose.id} 1:${s}@${end}:`, r.violations.map((v: any) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`).join("; "));
    }
}
