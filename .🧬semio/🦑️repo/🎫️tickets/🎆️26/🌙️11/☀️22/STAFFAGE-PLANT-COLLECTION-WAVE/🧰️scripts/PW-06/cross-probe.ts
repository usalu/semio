/** 🔎️ Where two scales of a species disagree: the boundary distances (both ways) between the unions of the projected body solids of two scales per view, with the worst places, to steer `volumes-cross-scale`.
 * Usage: `repo tools exec bun -- cross-probe.ts <species module> <fine> <coarse>`. */
import type { Species } from "./plant-kit.ts";
import { lobeOutline, outline, tubePolygons } from "../../../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE/r3-apple-tree-kit.ts";

const spec: Species = (await import(`./${process.argv[2]}.ts`)).default;
const [fine, coarse] = [Number(process.argv[3]), Number(process.argv[4])] as [50, 100];
type P = { x: number; y: number };
const rings = (scale: 50 | 100 | 200 | 500, view: "elevation" | "front" | "plan"): P[][] => {
  const m = spec.models[scale];
  return outline([...m.tubes.flatMap((t) => tubePolygons(view, t)), ...[...m.lobes, ...m.clusters].map((l) => lobeOutline(view, l, 72))], 0.005, 0.0001);
};
const seg = (p: P, a: P, b: P): number => {
  const [dx, dy] = [b.x - a.x, b.y - a.y];
  const k = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
  return Math.hypot(p.x - a.x - k * dx, p.y - a.y - k * dy);
};
const dense = (rs: P[][], step: number): P[] => rs.flatMap((r) => r.flatMap((a, i) => {
  const b = r[(i + 1) % r.length]!;
  const n = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / step));
  return Array.from({ length: n }, (_, k) => ({ x: a.x + ((b.x - a.x) * k) / n, y: a.y + ((b.y - a.y) * k) / n }));
}));
for (const view of ["elevation", "front", "plan"] as const) {
  const [A, B] = [rings(fine, view), rings(coarse, view)];
  const d = (pts: P[], to: P[][]) => pts.map((p) => ({ p, d: Math.min(...to.map((r) => Math.min(...r.map((a, i) => seg(p, a, r[(i + 1) % r.length]!))))) }));
  const all = [...d(dense(B, 0.02), A), ...d(dense(A, 0.02), B)].sort((a, b) => a.d - b.d);
  const p95 = all[Math.floor(all.length * 0.95)]!.d;
  const worst = all.slice(-400).filter((_, i) => i % 40 === 0).map((w) => `(${w.p.x.toFixed(2)},${w.p.y.toFixed(2)}) ${(w.d * 1000).toFixed(0)}`);
  const bands: Record<string, number> = {};
  for (const w of all.filter((x) => x.d > p95 * 0.8)) bands[(Math.round(w.p.y * 2) / 2).toFixed(1)] = (bands[(Math.round(w.p.y * 2) / 2).toFixed(1)] ?? 0) + 1;
  console.log(`[DEBUG] ${view}: p95 ${(p95 * 1000).toFixed(0)} mm; worst ${worst.join(" ")}; far points by height ${JSON.stringify(bands)}`);
}
const area = (r: P[]): number => r.reduce((s, p, i) => s + p.x * r[(i + 1) % r.length]!.y - r[(i + 1) % r.length]!.x * p.y, 0) / 2;
for (const scale of [fine, coarse])
  for (const view of ["elevation", "front", "plan"] as const) {
    const holes = rings(scale, view).filter((r) => area(r) < 0).map((r) => `${(-area(r)).toFixed(3)}@(${(r.reduce((s, p) => s + p.x, 0) / r.length).toFixed(2)},${(r.reduce((s, p) => s + p.y, 0) / r.length).toFixed(2)})`);
    const outer = rings(scale, view).filter((r) => area(r) > 0).length;
    console.log(`[DEBUG] 1:${scale} ${view}: ${outer} outer ring(s), ${holes.length} hole(s) ${holes.slice(0, 12).join(" ")}`);
  }
