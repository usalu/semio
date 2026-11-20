/** 🧪️ Where the crown outlines of two adjacent scales of the round-3 small-leaved lime differ most: per view the 95th percentile of the boundary distance and the worst places (paper coordinates of the view).
 * Usage: bun r3-small-leaved-lime-cross.ts <fine> <coarse> */
import type { P2 } from "./volumes-lib.ts";
import { lobeOutline, outline, tubePolygons, type View } from "./r3-small-leaved-lime-kit.ts";
import { models } from "./r3-small-leaved-lime-model.ts";

const [fine, coarse] = [Number(process.argv[2] ?? 50), Number(process.argv[3] ?? 100)];
const rings = (scale: number, view: View): P2[][] => {
  const m = models.find((x) => x.scale === scale)!;
  return outline([...m.lobes.map((l) => lobeOutline(view, l, 96)), ...m.wood.flatMap((t) => tubePolygons(view, t))], 0.006, 0.01);
};
const distance = (p: P2, to: P2[][]): number => {
  let best = Infinity;
  for (const ring of to) for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
    const d = (p.x - a.x - t * dx) ** 2 + (p.y - a.y - t * dy) ** 2;
    if (d < best) best = d;
  }
  return Math.sqrt(best);
};
const samples = (from: P2[][]): P2[] => from.flatMap((ring) => ring.flatMap((a, i) => {
  const b = ring[(i + 1) % ring.length]!;
  const n = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / 0.05));
  return Array.from({ length: n }, (_, k) => ({ x: a.x + ((b.x - a.x) * k) / n, y: a.y + ((b.y - a.y) * k) / n }));
}));
for (const view of ["elevation", "front", "plan"] as View[]) {
  const [a, b] = [rings(fine, view), rings(coarse, view)];
  const all = [...samples(a).map((p) => ({ p, d: distance(p, b), of: fine })), ...samples(b).map((p) => ({ p, d: distance(p, a), of: coarse }))].sort((x, y) => y.d - x.d);
  console.log(`[DEBUG] ${view} 1:${fine} against 1:${coarse}: p95 ${all[Math.floor(all.length * 0.05)]!.d.toFixed(3)} max ${all[0]!.d.toFixed(3)}`);
  const shown: P2[] = [];
  for (const s of all) {
    if (shown.length >= 16 || s.d < 0.1) break;
    if (shown.some((q) => Math.hypot(q.x - s.p.x, q.y - s.p.y) < 0.5)) continue;
    shown.push(s.p);
    console.log(`[DEBUG]   ${s.d.toFixed(3)} on 1:${s.of} at ${s.p.x.toFixed(2)} ${s.p.y.toFixed(2)}`);
  }
}
