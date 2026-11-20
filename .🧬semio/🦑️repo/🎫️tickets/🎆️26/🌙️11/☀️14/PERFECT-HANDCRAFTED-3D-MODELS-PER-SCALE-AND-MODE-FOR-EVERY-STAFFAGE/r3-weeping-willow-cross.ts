/** 🔍️ Where two scales' body outlines of the weeping willow differ: boundary points of each scale's union outline farther than a limit from the other's, binned by height and lateral position.
 * Usage: bun r3-weeping-willow-cross.ts <fine> <coarse> [limit m] */
import type { P2 } from "./volumes-lib.ts";
import { models } from "./r3-weeping-willow-model.ts";
import { outline, partPolygons, type View } from "./r3-weeping-willow-kit.ts";

const [fine, coarse] = [Number(process.argv[2]), Number(process.argv[3])];
const limit = Number(process.argv[4] ?? 0.1);
const all = models();
const segDist = (p: P2, a: P2, b: P2): number => {
  const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy || 1;
  const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / l));
  return Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy);
};
const dist = (p: P2, rings: P2[][]): number => Math.min(...rings.map((r) => Math.min(...r.map((a, i) => segDist(p, a, r[(i + 1) % r.length]!)))));
const sample = (rings: P2[][], step: number): P2[] => rings.flatMap((r) => r.flatMap((a, i) => {
  const b = r[(i + 1) % r.length]!, n = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / step));
  return Array.from({ length: n }, (_, k) => ({ x: a.x + ((b.x - a.x) * k) / n, y: a.y + ((b.y - a.y) * k) / n }));
}));
for (const view of ["elevation", "front", "plan"] as View[]) {
  const [A, B] = [fine, coarse].map((s) => outline(all.find((m) => m.scale === s)!.parts.filter((p) => !p.id.startsWith("inner") && !p.id.startsWith("fringe")).flatMap((p) => partPolygons(view, p)), 0.005, 0.01));
  for (const [name, X, Y] of [["fine->coarse", A!, B!], ["coarse->fine", B!, A!]] as const) {
    const pts = sample(X, 0.05);
    const d = pts.map((p) => ({ p, d: dist(p, Y) }));
    const sorted = d.map((x) => x.d).sort((a, b) => a - b);
    const bins: Record<string, number> = {};
    for (const x of d) if (x.d > limit) { const k = `y${Math.floor(x.p.y)}`; bins[k] = (bins[k] ?? 0) + 1; }
    console.log(view, name, "n", pts.length, "p95", sorted[Math.floor(sorted.length * 0.95)]!.toFixed(3), "over", d.filter((x) => x.d > limit).length, JSON.stringify(bins));
  }
}
