/** 🔎 Prints the joint origins and axes of every bone of `humans/elderly-with-cane` in the default pose (x ahead, y up, z left).
 *
 * Usage: `bun r5-elderly-with-cane-probe.ts` */
import { FRAMES } from "./r3-elderly-with-cane-kit.ts";
for (const [id, f] of Object.entries(FRAMES)) console.log(id.padEnd(10), "o", f.o.map((x) => x.toFixed(3)).join(" "), " u", f.u.map((x) => x.toFixed(3)).join(" "));
