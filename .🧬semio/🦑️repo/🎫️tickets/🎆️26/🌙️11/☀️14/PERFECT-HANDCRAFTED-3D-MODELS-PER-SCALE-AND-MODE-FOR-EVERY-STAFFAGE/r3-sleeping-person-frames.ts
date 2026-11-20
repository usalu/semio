/** 🧭️ Prints the supine world frames (origin, axes, tip) of every bone of the rooted sleeping person, the numbers the hand-stated solids are written against.
 * @see ./r3-sleeping-person-kit.ts */
import { POSE, ASSET_ROOTED } from "./r3-sleeping-person-kit.ts";
const f3 = (p: readonly number[]): string => p.map((x) => x.toFixed(3).padStart(7)).join(",");
for (const b of ASSET_ROOTED.rig.bones) {
  const f = POSE[b.id]!;
  const tip = [0, 1, 2].map((k) => f.o[k]! + b.lengthM * f.u[k]!);
  console.log(b.id.padEnd(12), "o", f3(f.o), "tip", f3(tip), "u", f3(f.u), "v", f3(f.v), "w", f3(f.w));
}
