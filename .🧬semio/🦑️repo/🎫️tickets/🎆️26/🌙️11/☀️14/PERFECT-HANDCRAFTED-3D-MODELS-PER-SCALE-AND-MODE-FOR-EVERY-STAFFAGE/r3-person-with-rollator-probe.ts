/** 🔎 Prints the joint positions of a pose or clip frame of `humans/person-with-rollator` (x ahead, y up, z left) and the direction of every bone.
 *
 * Usage: `bun r3-person-with-rollator-probe.ts [pose] [clip t]` */
import { rigAt } from "./r3-lib.ts";
import { ASSET as asset } from "./r3-person-with-rollator-kit.ts";
const [a, b] = process.argv.slice(2);
const q = b !== undefined ? ({ kind: "clip", clip: a!, timeS: Number(b) } as const) : ({ kind: "pose", pose: a ?? asset.presentation.defaultPose } as const);
const rig = rigAt(asset, 50, q);
for (const bone of rig.bones) {
  const f = rig.frames[bone.id]!;
  console.log(bone.id.padEnd(12), bone.o.map((x) => x.toFixed(4)).join(" "), " tip", bone.tip.map((x) => x.toFixed(4)).join(" "), " deg", ((Math.atan2(f.u[1], f.u[0]) * 180) / Math.PI).toFixed(1), " w", f.w.map((x) => x.toFixed(2)).join(","));
}
