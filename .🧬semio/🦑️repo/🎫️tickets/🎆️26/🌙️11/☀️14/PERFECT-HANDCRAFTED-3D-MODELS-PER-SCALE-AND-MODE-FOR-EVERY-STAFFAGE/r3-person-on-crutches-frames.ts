/** 🧭 Prints the bone frames of `humans/person-on-crutches` with the root fixes, in a pose (default: the default pose): origin, direction in the side plane and lateral position.
 * Usage: `bun r3-person-on-crutches-frames.ts [pose]` */
import { rigAt } from "./r3-lib.ts";
import { ASSET } from "./r3-person-on-crutches-kit.ts";
const rig = rigAt(ASSET, 50, { kind: "pose", pose: process.argv[2] ?? ASSET.presentation.defaultPose });
for (const b of rig.bones) {
  const f = rig.frames[b.id]!;
  console.log(b.id.padEnd(10), "o", f.o.map((x) => x.toFixed(4)).join(" "), "tip", b.tip.map((x) => x.toFixed(4)).join(" "), "deg", ((Math.atan2(f.u[1], f.u[0]) * 180) / Math.PI).toFixed(2), "u", f.u.map((x) => x.toFixed(3)).join(" "), "w", f.w.map((x) => x.toFixed(3)).join(" "));
}
