/** 🔎️ Round 3 pigeon: prints the default-pose bone frames (origin, axis, end) in world metres. */
import { openCtx } from "./kit-lib.ts";
const c = openCtx("animals/pigeon", Number(process.argv[2] ?? 50));
const bones = c.m.rig.bones as { id: string; lengthM: number }[];
for (const b of bones) {
  const f = c.state.frames[b.id]!;
  const o = f.o as number[]; const u = f.u as number[];
  const e = o.map((v, i) => v + u[i]! * b.lengthM);
  const r = (v: number[]) => v.map((x) => x.toFixed(4)).join(" ");
  console.log(b.id.padEnd(10), "o", r(o), "| end", r(e), "| u", r(u), "| v", r(f.v as number[]), "| w", r(f.w as number[]));
}
