/** 🧭️ Prints the bone frames of the candidate document of `humans/running-child` in a pose (default: its default pose; `rest` for the rest pose of the rig), the world position of every anchor and, with `bbox`, the bounding box of the body list of every scale in the default pose.
 * Usage: `bun r3-running-child-frames.ts [pose|rest|bbox]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { poseState, scaleMeshes } from "./volumes-lib.ts";

const m = JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-running-child", "🔣️.json"), "utf8")).asset;
const pose = process.argv[2];
const r = (a: readonly number[]) => a.map((x) => x.toFixed(4)).join(" ");
if (pose === "rest") {
  for (const [id, b] of Object.entries(restFrames(m.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>)) console.log(id.padEnd(12), "o", r(b.frame.o), "u", r(b.frame.u), "v", r(b.frame.v), "w", r(b.frame.w));
} else if (pose === "bbox") {
  for (const scale of [50, 100, 200, 500]) {
    const lo = [Infinity, Infinity, Infinity];
    const hi = [-Infinity, -Infinity, -Infinity];
    for (const mesh of scaleMeshes(m, scale, m.volumes[String(scale)]).body) for (let i = 0; i < mesh.world.length; i++) [lo[i % 3], hi[i % 3]] = [Math.min(lo[i % 3]!, mesh.world[i]!), Math.max(hi[i % 3]!, mesh.world[i]!)];
    console.log(scale, "min", r(lo), "max", r(hi), "extent", r(hi.map((x, k) => x - lo[k]!)));
  }
} else {
  if (pose !== undefined) m.presentation = { ...m.presentation, defaultPose: pose };
  const st = poseState(m, 50);
  for (const [id, f] of Object.entries(st.frames) as [string, any][]) console.log(id.padEnd(12), "o", r(f.o), "u", r(f.u), "v", r(f.v), "w", r(f.w));
  for (const a of m.anchors) {
    const f = (st.frames as any)[a.bone]!;
    console.log("anchor", a.id, r([0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]! + (a.z ?? 0) * f.w[k]!)));
  }
}
