import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// Coordinate descent over the listed joints: smallest angle changes until the lowest geometry of a pose is above -3 mm. Prints the new angles as JSON.
const [file, aid, pid, list] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const a = doc.assets.find((x: any) => x.id === aid), p = a.poses.find((x: any) => x.id === pid);
const names: string[] = JSON.parse(list!);
const lowest = () => { const s = evaluate(a, { kind: "pose", pose: pid }); let lo = 1e9, id = ""; for (const e of s.shapes.elevation) { if (e.shape.role === "ground") continue; for (const r of e.rings) for (const q of r) if (q.y < lo) { lo = q.y; id = e.shape.id; } } return { lo, id }; };
const ang = (n: string) => (p.bones[n] ??= { angleDeg: 0 }).angleDeg;
const lim = (n: string) => a.rig.bones.find((b: any) => b.id === n).joint;
for (let iter = 0; iter < 12; iter++) {
  const cur = lowest(); if (cur.lo >= -0.003) break;
  let best = { lo: cur.lo, n: "", v: 0 };
  for (const n of names) { const base = ang(n); const j = lim(n);
    for (let d = 2; d <= 40; d += 2) for (const sg of [1, -1]) { const v = base + sg * d; if (v < j.minDeg || v > j.maxDeg) continue; p.bones[n].angleDeg = v; const r = lowest(); if (r.lo > best.lo + 0.001 && (best.n === "" || r.lo > best.lo)) best = { lo: r.lo, n, v }; }
    p.bones[n].angleDeg = base; }
  if (!best.n) break;
  p.bones[best.n].angleDeg = best.v;
}
const fin = lowest();
console.log(JSON.stringify({ lowestMm: Math.round(fin.lo * 1e4) / 10, id: fin.id, bones: Object.fromEntries(names.map((n) => [n, p.bones[n].angleDeg])) }));
