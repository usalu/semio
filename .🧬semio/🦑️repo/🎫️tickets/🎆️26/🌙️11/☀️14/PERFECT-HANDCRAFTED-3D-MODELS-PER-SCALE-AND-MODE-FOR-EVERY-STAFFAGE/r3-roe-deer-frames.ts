/** 🧭️ Round 3 roe deer: prints the world frame of every bone in the default pose of the work document (origin, axis), the numbers the hand tables are written against. Usage: bun r3-roe-deer-frames.ts */
import { Roe, SPEC } from "./r3-roe-deer-kit.ts";
const S = new Roe(SPEC, 50);
for (const b of S.c.raw.rig.bones) { const f = S.frame(b.id); console.log(b.id.padEnd(12), f.o.map((v: number) => v.toFixed(3)).join(" "), " u", f.u.map((v: number) => v.toFixed(3)).join(" "), " v", f.v.map((v: number) => v.toFixed(3)).join(" "), " len", b.lengthM); }
