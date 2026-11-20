/** 🎞️ Samples a clip of `humans/running-child` (catalog asset, or the candidate document with `--candidate`) and prints per sample the pelvis position, the height of the four foot contact anchors and the knee angles, to judge planted feet, flight arcs and ground clearance after a rig change.
 * Usage: `bun r3-running-child-motion-probe.ts <clip> [--candidate] [--step S]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import { rigAt } from "./r3-lib.ts";
import { entryOf } from "./volumes-lib.ts";

const args = process.argv.slice(2);
const candidate = args.includes("--candidate");
const asset = candidate ? JSON.parse(readFileSync(join(import.meta.dir, "🗑️generated", "r3-running-child", "🔣️.json"), "utf8")).asset : entryOf("humans/running-child").asset;
const clip = asset.clips.find((c: any) => c.id === args[0]);
const step = args.includes("--step") ? Number(args[args.indexOf("--step") + 1]) : clip.durationS / 50;
const f = (x: number): string => x.toFixed(4).padStart(8);
console.log("     t   pelvis.x pelvis.y   heel-l   ball-l   heel-r   ball-r   toe-tip-l toe-tip-r");
for (let t = 0; t <= clip.durationS + 1e-9; t += step) {
  const rig = rigAt(asset, 50, { kind: "clip", clip: clip.id, timeS: Math.min(t, clip.durationS) });
  const at = (id: string): number[] => {
    const a = asset.anchors.find((x: any) => x.id === id);
    const fr = rig.frames[a.bone]!;
    return [0, 1, 2].map((k) => fr.o[k]! + a.x * fr.u[k]! + a.y * fr.v[k]!);
  };
  const tip = (bone: string): number => rig.bones.find((b) => b.id === bone)!.tip[1]!;
  console.log(t.toFixed(4), f(rig.frames.pelvis!.o[0]!), f(rig.frames.pelvis!.o[1]!), ...["heel-l", "ball-l", "heel-r", "ball-r"].map((id) => f(at(id)[1]!)), f(tip("toe-l")), f(tip("toe-r")));
}
