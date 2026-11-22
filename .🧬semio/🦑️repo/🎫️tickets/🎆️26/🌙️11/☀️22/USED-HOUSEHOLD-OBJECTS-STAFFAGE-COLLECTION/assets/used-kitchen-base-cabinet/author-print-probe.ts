/** 🔬 Print probe of the candidate: measures every authored scale at its nominal and band-end denominators pose by pose, so a violation names the pose that causes it.
 *
 * `$BUN author-print-probe.ts [scale]`
 *
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts — `measureSuitability`
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const only = process.argv[2];
for (const key of ["50", "100", "200", "500"]) {
  if (only !== undefined && only !== key) continue;
  const rep = asset.representations[key];
  for (const pose of asset.poses.filter((p: { key?: boolean }) => !p.key)) {
    for (const view of ["elevation", "front", "plan"]) {
      const geometry = { elevation: [], front: [], plan: [], [view]: rep.geometry[view] };
      const one = { ...rep, geometry };
      const single = { ...asset, poses: [pose], representations: { ...asset.representations, [key]: one } };
      for (const at of [...new Set([rep.validRange.minDenominator, Number(key), rep.validRange.maxDenominator])]) {
        const s = measureSuitability(single, one, at, DEFAULT_PRINT_PROFILE);
        for (const v of s.violations) console.log(`[DEBUG] 1:${key} at 1:${at} pose ${pose.id} ${view}: ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}`);
      }
    }
  }
}
console.log("[DEBUG] probe done");
