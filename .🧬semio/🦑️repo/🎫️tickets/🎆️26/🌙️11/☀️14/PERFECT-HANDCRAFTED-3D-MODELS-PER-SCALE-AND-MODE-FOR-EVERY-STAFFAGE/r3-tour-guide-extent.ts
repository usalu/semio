/** 📦 Prints the bounding box of every authored scale's body of `humans/tour-guide` in the default pose (x ahead, y up, z left), from the candidate document of `r3-tour-guide-apply.py`.
 *
 * Usage: `bun r3-tour-guide-extent.ts` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { listsIn, pointsOf, rigAt } from "./r3-lib.ts";

const asset = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-tour-guide", "🔣️.json"), "utf8")).asset;
for (const scale of [50, 100, 200, 500]) {
  const rig = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose });
  const sm = listsIn(asset, scale, rig.frames);
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const p of pointsOf(sm.body)) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); }
  console.log(scale, "x", lo[0]!.toFixed(3), hi[0]!.toFixed(3), "len", (hi[0]! - lo[0]!).toFixed(3), " y", lo[1]!.toFixed(3), hi[1]!.toFixed(3), " z", lo[2]!.toFixed(3), hi[2]!.toFixed(3), "wid", (hi[2]! - lo[2]!).toFixed(3));
}
