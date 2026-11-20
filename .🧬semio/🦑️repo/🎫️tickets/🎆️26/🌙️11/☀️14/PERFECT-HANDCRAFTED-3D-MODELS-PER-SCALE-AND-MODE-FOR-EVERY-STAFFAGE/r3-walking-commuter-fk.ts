/** 🦴 Prints the joints of a pose of the candidate `humans/walking-commuter` in figure coordinates (x ahead, y up, z left) and the anchors.
 * Usage: `bun r3-walking-commuter-fk.ts [pose]` */
import { rigAt } from "./r3-lib.ts";
import { candidate } from "./r3-walking-commuter-io.ts";
const m = candidate(false);
const pose = process.argv[2] ?? m.presentation.defaultPose;
const rig = rigAt(m, 50, { kind: "pose", pose });
const f3 = (a: readonly number[]): string => a.map((x) => x.toFixed(3)).join(" ");
for (const b of rig.bones) console.log(String(b.id).padEnd(10), "o", f3(b.o), "tip", f3(b.tip), "deg", ((Math.atan2(rig.frames[b.id]!.u[1], rig.frames[b.id]!.u[0]) * 180) / Math.PI).toFixed(1));
for (const a of m.anchors) { const f = rig.frames[a.bone]!; console.log("anchor", a.id, f3([0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]! + (a.z ?? 0) * f.w[k]!))); }
