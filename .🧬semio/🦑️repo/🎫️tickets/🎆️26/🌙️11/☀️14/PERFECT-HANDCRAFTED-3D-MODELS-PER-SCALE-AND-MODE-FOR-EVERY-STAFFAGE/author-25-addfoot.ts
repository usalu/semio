/** Spec argument: JSON [[frameBone, [bones to hull], id], ...] (default: the dairy-cow lower legs). Prints new 1:500 elevation shapes for lower legs (metapodium + hoof merged) as hull of the 1:200 outlines in the metapodium bone frame. Usage: bun author-25-addfoot.ts animals/dairy-cow */
import { openCtx, drawn } from "./kit-lib.ts";
const c = openCtx(process.argv[2]!, 200);
const hull = (pts: [number, number][]): [number, number][] => {
  const p = [...pts].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cr = (o: number[], a: number[], b: number[]) => (a[0]! - o[0]!) * (b[1]! - o[1]!) - (a[1]! - o[1]!) * (b[0]! - o[0]!);
  const lo: [number, number][] = [];
  for (const q of p) { while (lo.length >= 2 && cr(lo[lo.length - 2]!, lo[lo.length - 1]!, q) <= 0) lo.pop(); lo.push(q); }
  const up: [number, number][] = [];
  for (const q of [...p].reverse()) { while (up.length >= 2 && cr(up[up.length - 2]!, up[up.length - 1]!, q) <= 0) up.pop(); up.push(q); }
  return [...lo.slice(0, -1), ...up.slice(0, -1)];
};
const out: unknown[] = [];
const spec: [string, string[], string][] = process.argv[3] !== undefined ? JSON.parse(process.argv[3]) : [["metatarsus-hf", ["metatarsus-hf", "hoof-hf"], "far-hind-lower"], ["metatarsus-hn", ["metatarsus-hn", "hoof-hn"], "near-hind-lower"], ["cannon-ff", ["cannon-ff", "hoof-ff"], "far-fore-lower"], ["cannon-fn", ["cannon-fn", "hoof-fn"], "near-fore-lower"]];
for (const [bone, group, id] of spec) {
  const f = c.state.frames[bone]!;
  const pts: [number, number][] = [];
  for (const s of drawn(c, "elevation")) if (group.includes(s.bone)) for (const r of s.rings) for (const q of r) {
    const d = [q.x - f.o[0]!, q.y - f.o[1]!];
    pts.push([d[0]! * f.u[0]! + d[1]! * f.u[1]!, d[0]! * f.v[0]! + d[1]! * f.v[1]!]);
  }
  out.push({ id, bone, pts: hull(pts).map((q) => [Math.round(q[0] * 1e5) / 1e5, Math.round(q[1] * 1e5) / 1e5]) });
}
console.log(JSON.stringify(out));
