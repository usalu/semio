/** 🧭️ Prints the bone frames (origin, tip, axes) of animals/sheep in the default pose or the pose given as argv[2]. Usage: bun r3-sheep-frames.ts [pose] */
import { openCtx } from "./kit-lib.ts";
const c = openCtx("animals/sheep", 50);
const f3 = (v: number[]): string => v.map((x) => x.toFixed(4)).join(", ");
for (const b of c.raw.rig.bones) {
  const f = c.state.frames[b.id]!;
  const tip = f.o.map((x, i) => x + f.u[i]! * b.lengthM);
  console.log(b.id.padEnd(14), "o", f3(f.o), "| tip", f3(tip), "| u", f3(f.u), "| v", f3(f.v), "| w", f3(f.w));
}
