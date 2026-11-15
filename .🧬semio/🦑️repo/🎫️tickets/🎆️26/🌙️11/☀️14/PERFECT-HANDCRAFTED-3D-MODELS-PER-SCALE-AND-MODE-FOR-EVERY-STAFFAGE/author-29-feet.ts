import { openCtx, drawn } from "./kit-lib.ts";
import { writeFileSync } from "node:fs";
const c = openCtx("animals/pigeon", 50);
const hull = (pts: [number, number][]): [number, number][] => {
  const p = [...pts].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cr = (o: number[], a: number[], b: number[]) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lo: [number, number][] = [], up: [number, number][] = [];
  for (const q of p) { while (lo.length >= 2 && cr(lo[lo.length - 2]!, lo[lo.length - 1]!, q) <= 0) lo.pop(); lo.push(q); }
  for (const q of [...p].reverse()) { while (up.length >= 2 && cr(up[up.length - 2]!, up[up.length - 1]!, q) <= 0) up.pop(); up.push(q); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
};
const out: any[] = [];
for (const side of ["n", "f"]) {
  const f: any = c.state.frames[`tarsus-${side}`];
  const pts: [number, number][] = [];
  for (const b of [`tarsus-${side}`, `toes-${side}`]) for (const s of drawn(c, "elevation", b)) for (const r of s.rings) for (const p of r) {
    const d = [p.x - f.o[0], p.y - f.o[1]];
    pts.push([d[0] * f.u[0] + d[1] * f.u[1], d[0] * f.v[0] + d[1] * f.v[1]]);
  }
  const h = hull(pts).map((q) => [Math.round(q[0] * 1e5) / 1e5, Math.round(q[1] * 1e5) / 1e5]);
  console.log(side, h.length, JSON.stringify(h));
  out.push({ scale: "200", view: "elevation", id: side === "n" ? "near-foot" : "far-foot", set: { points: h, penMm: 0.15 } });
}
out.push({ scale: "200", view: "front", id: "near-foot", set: { penMm: 0.15 } });
writeFileSync("🗑️generated/author-29/pigeon-feet-patch.json", JSON.stringify(out));
