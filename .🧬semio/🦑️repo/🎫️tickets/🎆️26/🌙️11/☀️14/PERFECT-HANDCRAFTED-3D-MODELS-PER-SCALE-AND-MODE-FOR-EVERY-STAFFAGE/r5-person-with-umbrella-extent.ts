/** 📏 Prints the body extents of every scale of `humans/person-with-umbrella` in the default pose (candidate rig, current source), for `dimensions` and the bounds rule.
 *
 * Usage: `bun r5-person-with-umbrella-extent.ts` */
import { listsIn, pointsOf, rigAt } from "./r3-lib.ts";
import { ASSET } from "./r5-person-with-umbrella-kit.ts";
import { volumes } from "./r5-person-with-umbrella-build.ts";
const work = { ...ASSET, volumes: await volumes() };
for (const scale of [50, 100, 200, 500]) {
  const rig = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose });
  const lo = [Infinity, Infinity, Infinity];
  const hi = [-Infinity, -Infinity, -Infinity];
  for (const p of pointsOf(listsIn(work, scale, rig.frames).body)) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, p[k]!); hi[k] = Math.max(hi[k]!, p[k]!); }
  console.log(scale, "x", lo[0]!.toFixed(3), hi[0]!.toFixed(3), "y", lo[1]!.toFixed(3), hi[1]!.toFixed(3), "z", lo[2]!.toFixed(3), hi[2]!.toFixed(3), "| H", (hi[1]! - lo[1]!).toFixed(3), "depth", (hi[0]! - lo[0]!).toFixed(3), "width", (hi[2]! - lo[2]!).toFixed(3));
}
