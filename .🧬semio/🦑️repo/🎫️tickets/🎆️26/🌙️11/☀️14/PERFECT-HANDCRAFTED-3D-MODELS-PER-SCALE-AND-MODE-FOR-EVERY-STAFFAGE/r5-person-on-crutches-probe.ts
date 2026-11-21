/** 🔎 Prints the joint origins and axes of every bone of `humans/person-on-crutches` in the default pose (x ahead, y up, z left) and the bone lengths.
 *
 * Usage: `bun r5-person-on-crutches-probe.ts` */
import { ASSET, FRAMES } from "./r3-person-on-crutches-kit.ts";
for (const [id, f] of Object.entries(FRAMES)) console.log(id.padEnd(10), "o", f.o.map((x) => x.toFixed(3)).join(" "), " u", f.u.map((x) => x.toFixed(3)).join(" "), " L", (ASSET.rig.bones as any[]).find((b) => b.id === id)?.lengthM);
