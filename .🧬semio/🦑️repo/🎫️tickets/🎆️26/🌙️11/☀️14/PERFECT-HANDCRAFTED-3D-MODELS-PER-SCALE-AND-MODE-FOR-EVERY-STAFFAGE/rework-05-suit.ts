/** 🖨️ Names the pose and view of every print-suitability violation of a candidate document: rework-05-suit.ts <candidate 🔣️.json> <scale> [requested denominators…]
 * @see ./r3-tools.md */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale, ...requested] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const rep = asset.representations[scale!];
for (const req of requested.length > 0 ? requested.map(Number) : [Number(scale)])
  for (const pose of asset.poses)
    for (const view of ["all"]) {
      const geometry = view === "all" ? rep.geometry : { elevation: view === "elevation" ? rep.geometry.elevation : rep.geometry.elevation.map((x: { id: string }) => ({ ...x, filled: false, role: "texture" })), front: view === "front" ? rep.geometry.front : [], plan: view === "plan" ? rep.geometry.plan : [] };
      const one = { ...asset, poses: [pose], clips: [], presentation: { ...asset.presentation, defaultPose: pose.id, defaultClip: undefined }, representations: { ...asset.representations, [scale!]: { ...rep, geometry } } };
      try {
        const s = measureSuitability(one as never, one.representations[scale!] as never, req, DEFAULT_PRINT_PROFILE);
        for (const v of s.violations) console.log(`1:${req} ${pose.id} ${view} ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(3)}`);
      } catch (e) {
        console.log(`[DEBUG] 1:${req} ${pose.id} ${view} threw ${String(e).slice(0, 160)}`);
      }
    }
