/** 🦴 Prints the joints of a pose (or clip frame) of `humans/person-cooking` in world coordinates (x ahead, y up, z left) and the anchors.
 * Usage: `bun r3-person-cooking-fk.ts [pose | clip:t]` */
import { rigAt } from "./r3-lib.ts";
import { candidate } from "./r3-person-cooking-io.ts";
const m = candidate(false);
const arg = process.argv[2] ?? m.presentation.defaultPose;
const q = arg.includes(":") ? { kind: "clip" as const, clip: arg.split(":")[0]!, timeS: Number(arg.split(":")[1]) } : { kind: "pose" as const, pose: arg };
const rig = rigAt(m, 50, q);
const f3 = (a: readonly number[]): string => a.map((x) => x.toFixed(3)).join(" ");
for (const b of rig.bones) console.log(String(b.id).padEnd(12), "o", f3(b.o), "tip", f3(b.tip), "deg", ((Math.atan2(rig.frames[b.id]!.u[1], rig.frames[b.id]!.u[0]) * 180) / Math.PI).toFixed(1));
for (const a of m.anchors) { const f = rig.frames[a.bone]!; console.log("anchor", a.id.padEnd(14), f3([0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]! + (a.z ?? 0) * f.w[k]!))); }
