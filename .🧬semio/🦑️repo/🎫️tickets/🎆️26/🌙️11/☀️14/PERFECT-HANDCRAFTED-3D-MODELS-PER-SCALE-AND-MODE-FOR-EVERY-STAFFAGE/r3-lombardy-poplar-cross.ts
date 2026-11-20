/** 🔭️ Where the body silhouettes of two adjacent scales of the round-3 Lombardy poplar differ: per view the 95th percentile of the boundary distance (both directions, raster of 2 cm) and, per metre band of height, how much of the boundary lies farther than the limit, with the worst places (paper coordinates of the view).
 * Usage: bun r3-lombardy-poplar-cross.ts <fine> <coarse> [limit m] [view] */
import type { P2 } from "./volumes-lib.ts";
import { corePolygons, lobeOutline, tubePolygons, type View } from "./r3-lombardy-poplar-kit.ts";
import { models } from "./r3-lombardy-poplar-model.ts";

const [fine, coarse, limit] = [Number(process.argv[2] ?? 50), Number(process.argv[3] ?? 100), Number(process.argv[4] ?? 0.109)];
const only = process.argv[5];
const CELL = 0.02;

type Grid = { w: number; h: number; x0: number; y0: number; data: Uint8Array };
function raster(scale: number, view: View, box: { x0: number; y0: number; x1: number; y1: number }): Grid {
  const m = models.find((x) => x.scale === scale)!;
  const [w, h] = [Math.ceil((box.x1 - box.x0) / CELL), Math.ceil((box.y1 - box.y0) / CELL)];
  const data = new Uint8Array(w * h);
  const polygons: P2[][] = [...m.lobes.map((l) => lobeOutline(view, l, 48)), ...m.burls.map((l) => lobeOutline(view, l, 16)), ...m.cores.flatMap((c) => corePolygons(view, c)), ...m.tufts.flatMap((t) => tubePolygons(view, t)), ...m.wood.filter((t) => !t.flags?.slender && !t.flags?.texture).flatMap((t) => tubePolygons(view, t))];
  for (const ring of polygons) {
    const ys = ring.map((p) => p.y);
    const [ja, jb] = [Math.max(0, Math.floor((Math.min(...ys) - box.y0) / CELL)), Math.min(h - 1, Math.ceil((Math.max(...ys) - box.y0) / CELL))];
    for (let j = ja; j <= jb; j++) {
      const y = box.y0 + (j + 0.5) * CELL;
      const xs: number[] = [];
      for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
        const [a, b] = [ring[i]!, ring[k]!];
        if (a.y > y !== b.y > y) xs.push(((b.x - a.x) * (y - a.y)) / (b.y - a.y) + a.x);
      }
      xs.sort((p, q) => p - q);
      for (let n = 0; n + 1 < xs.length; n += 2) {
        const [ia, ib] = [Math.max(0, Math.ceil((xs[n]! - box.x0) / CELL - 0.5)), Math.min(w - 1, Math.floor((xs[n + 1]! - box.x0) / CELL - 0.5))];
        for (let i = ia; i <= ib; i++) data[j * w + i] = 1;
      }
    }
  }
  return { w, h, x0: box.x0, y0: box.y0, data };
}
const boundary = (g: Grid): number[] => {
  const out: number[] = [];
  for (let j = 1; j + 1 < g.h; j++) for (let i = 1; i + 1 < g.w; i++) {
    const k = j * g.w + i;
    if (g.data[k] && !(g.data[k - 1] && g.data[k + 1] && g.data[k - g.w] && g.data[k + g.w])) out.push(k);
  }
  return out;
};
function distance(g: Grid, seeds: number[]): Float32Array {
  const d = new Float32Array(g.w * g.h).fill(1e9);
  for (const k of seeds) d[k] = 0;
  const [a, b] = [CELL, CELL * Math.SQRT2];
  for (let j = 1; j + 1 < g.h; j++) for (let i = 1; i + 1 < g.w; i++) {
    const k = j * g.w + i;
    d[k] = Math.min(d[k]!, d[k - 1]! + a, d[k - g.w]! + a, d[k - g.w - 1]! + b, d[k - g.w + 1]! + b);
  }
  for (let j = g.h - 2; j >= 1; j--) for (let i = g.w - 2; i >= 1; i--) {
    const k = j * g.w + i;
    d[k] = Math.min(d[k]!, d[k + 1]! + a, d[k + g.w]! + a, d[k + g.w + 1]! + b, d[k + g.w - 1]! + b);
  }
  return d;
}

const box3 = { elevation: { x0: -4, y0: -0.5, x1: 4, y1: 27 }, front: { x0: -4, y0: -0.5, x1: 4, y1: 27 }, plan: { x0: -4, y0: -4, x1: 4, y1: 4 } };
for (const view of ["elevation", "front", "plan"] as View[]) {
  if (only && only !== view) continue;
  const [a, b] = [raster(fine, view, box3[view]), raster(coarse, view, box3[view])];
  const [ba, bb] = [boundary(a), boundary(b)];
  const [da, db] = [distance(a, ba), distance(b, bb)];
  const all = [...ba.map((k) => ({ k, d: db[k]!, of: fine })), ...bb.map((k) => ({ k, d: da[k]!, of: coarse }))].sort((x, y) => y.d - x.d);
  const padOf = (p: P2): string => (view === "plan" ? `r${Math.floor(Math.hypot(p.x, p.y) * 2) / 2}` : `y${Math.floor(p.y)}`);
  const at = (k: number): P2 => ({ x: a.x0 + ((k % a.w) + 0.5) * CELL, y: a.y0 + (Math.floor(k / a.w) + 0.5) * CELL });
  const bad = all.filter((s) => s.d > limit);
  const count: Record<string, number> = {};
  for (const s of bad) count[padOf(at(s.k))] = (count[padOf(at(s.k))] ?? 0) + 1;
  console.log(`[DEBUG] ${view} 1:${fine} against 1:${coarse}: p95 ${all[Math.floor(all.length * 0.05)]!.d.toFixed(3)} max ${all[0]!.d.toFixed(3)}; beyond ${limit}: ${((100 * bad.length) / all.length).toFixed(1)} % of ${all.length} cells; per plate ${Object.entries(count).map(([k, n]) => `${k} ${((100 * n) / all.length).toFixed(1)}`).join(", ")}`);
  const shown: P2[] = [];
  for (const s of bad) {
    if (shown.length >= 40) break;
    const p = at(s.k);
    if (shown.some((q) => Math.hypot(q.x - p.x, q.y - p.y) < 0.4)) continue;
    shown.push(p);
    console.log(`[DEBUG]   ${s.d.toFixed(2)} on 1:${s.of} at ${p.x.toFixed(2)} ${p.y.toFixed(2)} (${padOf(p)})`);
  }
}
