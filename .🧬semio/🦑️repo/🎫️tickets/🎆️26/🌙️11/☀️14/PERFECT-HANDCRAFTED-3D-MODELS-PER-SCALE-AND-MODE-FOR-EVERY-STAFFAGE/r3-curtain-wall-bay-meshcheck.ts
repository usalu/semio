/** 🔍️ Round 3 curtain-wall bay: tessellates every solid of the volumes source and compares the mesh volume (divergence theorem) with the analytic volume of extrudes (profile area minus holes times length), to find profiles the kernel triangulates wrongly. Usage: `bun r3-curtain-wall-bay-meshcheck.ts` */
import { readFileSync } from "node:fs";
import { localGeometry } from "./volumes-lib.ts";

const src = JSON.parse(readFileSync(`${import.meta.dir}/volumes/components/curtain-wall-bay.json`, "utf8")).volumes;
const q = { chordM: 0.002, minSegments: 16, maxSegments: 32 };
const area = (r: number[][]): number => Math.abs(r.reduce((a, p, i) => { const n = r[(i + 1) % r.length]!; return a + p[0]! * n[1]! - n[0]! * p[1]!; }, 0) / 2);
for (const scale of Object.keys(src)) for (const list of ["body", "thermal", "structure"]) for (const s of src[scale][list] ?? []) {
  if (s.kind !== "extrude") continue;
  const g = localGeometry(s, q);
  let v = 0;
  const p = g.positions;
  for (let t = 0; t < g.indices.length; t += 3) {
    const [a, b, c] = [g.indices[t]! * 3, g.indices[t + 1]! * 3, g.indices[t + 2]! * 3];
    v += (p[a]! * (p[b + 1]! * p[c + 2]! - p[b + 2]! * p[c + 1]!) - p[a + 1]! * (p[b]! * p[c + 2]! - p[b + 2]! * p[c]!) + p[a + 2]! * (p[b]! * p[c + 1]! - p[b + 1]! * p[c]!)) / 6;
  }
  const want = (area(s.profile) - (s.holes ?? []).reduce((a: number, h: number[][]) => a + area(h), 0)) * Math.abs(s.to - s.from) * (s.repeat?.count ?? 1);
  const dev = Math.abs(Math.abs(v) - want) / want;
  if (dev > 1e-3) console.log(`[DEBUG] 1:${scale} ${list} ${s.id}: mesh ${Math.abs(v).toExponential(4)} want ${want.toExponential(4)} dev ${(dev * 100).toFixed(2)} %`);
}
console.log("done");
