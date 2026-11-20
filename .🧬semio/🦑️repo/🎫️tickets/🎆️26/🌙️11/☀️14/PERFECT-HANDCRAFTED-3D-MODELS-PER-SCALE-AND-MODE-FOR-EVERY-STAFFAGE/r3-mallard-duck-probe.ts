/** 🔎️ Round 3 mallard: prints the world joints, tips and axes of the default pose of the trial rig. Usage: bun r3-mallard-duck-probe.ts */
import { Bench, type Json, type V3 } from "./r3-mallard-duck-kit.ts";

const S = new Bench(50);
const f3 = (v: V3): string => v.map((x) => x.toFixed(4)).join(" ");
for (const b of S.c.m.rig.bones as Json[]) console.log(b.id.padEnd(9), "o", f3(S.J(b.id)), "tip", f3(S.J(b.id, b.lengthM)), "u", f3(S.frame(b.id).u as V3));
