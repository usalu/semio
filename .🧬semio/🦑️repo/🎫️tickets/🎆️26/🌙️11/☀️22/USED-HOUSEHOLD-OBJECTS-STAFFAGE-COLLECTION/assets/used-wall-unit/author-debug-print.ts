/** 🔬️ Debug aid: measures the print suitability of one scale of the candidate per named pose and per viewpoint (`bun author-debug-print.ts <scale> <requested>`), so a violation can be traced to the pose and view that causes it. */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")).asset;
const scale = process.argv[2] ?? "200";
const requested = Number(process.argv[3] ?? scale);
const profile = DEFAULT_PRINT_PROFILE;
for (const pose of asset.poses) {
  for (const view of ["elevation", "front", "plan"]) {
    const rep = asset.representations[scale];
    const only = { ...rep, geometry: { elevation: view === "elevation" ? rep.geometry.elevation : [], front: view === "front" ? rep.geometry.front : [], plan: view === "plan" ? rep.geometry.plan : [] } };
    const one = { ...asset, poses: [pose], representations: { ...asset.representations, [scale]: only } };
    const s = measureSuitability(one, only, requested, profile);
    for (const v of s.violations) console.log(`[DEBUG] ${pose.id} ${view}: ${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} mm`);
  }
}
