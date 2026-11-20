/** 🦴 Prints the joints of a pose of `humans/parent-carrying-toddler` (candidate when it exists, else the catalog) in figure coordinates (x ahead, y up, z left) with the frame vectors and the anchors.
 * Usage: `bun r3-parent-carrying-toddler-fk.ts [pose] [--catalog]` */
import { existsSync, readFileSync } from "node:fs";
import { rigAt } from "./r3-lib.ts";
const G = "🗑️generated/r3-parent-carrying-toddler";
const file = !process.argv.includes("--catalog") && existsSync(`${G}/🔣️.json`) ? `${G}/🔣️.json` : `${G}/original.json`;
const m = JSON.parse(readFileSync(file, "utf8")).asset;
const pose = process.argv[2] && !process.argv[2].startsWith("--") ? process.argv[2] : m.presentation.defaultPose;
const rig = rigAt(m, 50, { kind: "pose", pose });
const f3 = (a: readonly number[]): string => a.map((x) => x.toFixed(3)).join(" ");
console.log(file, pose);
for (const b of rig.bones) { const f = rig.frames[b.id]!; console.log(String(b.id).padEnd(16), "o", f3(b.o), "tip", f3(b.tip), "u", f3(f.u), "v", f3(f.v), "w", f3(f.w)); }
for (const a of m.anchors) { const f = rig.frames[a.bone]!; console.log("anchor", a.id.padEnd(18), f3([0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]! + (a.z ?? 0) * f.w[k]!))); }
