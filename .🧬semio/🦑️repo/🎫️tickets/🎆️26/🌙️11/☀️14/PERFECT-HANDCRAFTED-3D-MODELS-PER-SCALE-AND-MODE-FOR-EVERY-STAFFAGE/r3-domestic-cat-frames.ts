/** 🧭️ Prints the bone frames (origin, tip, world angle) of animals/domestic-cat in the default pose. Usage: bun r3-domestic-cat-frames.ts */
import { openCtx } from "./kit-lib.ts";
const c = openCtx("animals/domestic-cat", 50);
const f3 = (v: number[]): string => v.map((x) => x.toFixed(4)).join(", ");
for (const b of c.raw.rig.bones) {
  const f = c.state.frames[b.id]!;
  const tip = f.o.map((x: number, i: number) => x + f.u[i]! * b.lengthM);
  console.log(b.id.padEnd(14), "o", f3(f.o), "| tip", f3(tip), "| deg", (Math.atan2(f.u[1]!, f.u[0]!) * 180 / Math.PI).toFixed(1), "| w", f3(f.w));
}
