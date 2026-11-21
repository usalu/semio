/** 📦️ Prints the vertical extent of the body of every scale of the sleeping person's candidate document and the solid that reaches the top and the bottom (debug aid of round 5).
 * Usage: `bun r5-sleeping-person-box.ts` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { meshesIn, rigAt } from "./r3-lib.ts";
const asset = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-sleeping-person", "🔣️.json"), "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  const frames = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  let [lo, hi, loId, hiId] = [Infinity, -Infinity, "", ""];
  for (const m of meshesIn(asset, frames, asset.volumes[String(scale)].body.filter((s: any) => s.slender !== true))) for (let i = 1; i < m.world.length; i += 3) {
    if (m.world[i]! < lo) [lo, loId] = [m.world[i]!, (m as any).id];
    if (m.world[i]! > hi) [hi, hiId] = [m.world[i]!, (m as any).id];
  }
  console.log(scale, lo.toFixed(3), loId, hi.toFixed(3), hiId, (hi - lo).toFixed(3));
}
