/** 🔬️ Silhouette difference between the bodies of two scales in one view: clusters of cells that only one has, with world position and the solids there. Usage: bun author-32-xdiff.ts cat/slug 100 200 elevation [--source f] [--cell 0.01] */
import { readFileSync } from "node:fs";
import { Mask, entryOf, loadSource, merged, projectWorld, scaleMeshes } from "./volumes-lib.ts";
const a = process.argv.slice(2);
const g = (n: string) => (a.includes(n) ? a[a.indexOf(n) + 1] : undefined);
const [spec, sa, sb, view] = a;
const { entry, asset } = entryOf(spec!);
const src = g("--source") ? JSON.parse(readFileSync(g("--source")!, "utf8")) : loadSource(entry);
const m = merged(asset, src);
const cell = Number(g("--cell") ?? 0.01);
const meshes = (s: string) => (scaleMeshes(m, Number(s), m.volumes[s]) as any).body as { id: string; world: Float64Array; indices: Uint32Array }[];
const A = meshes(sa!), B = meshes(sb!);
const box = { minX: -3, minY: -0.5, maxX: 3, maxY: 3 };
const w = (view === "plan" || view === "front") ? { minX: -1.5, minY: -0.5, maxX: 1.5, maxY: 3 } : box;
const mk = (ms: typeof A) => {
  const mask = new Mask(w, cell);
  const owner = new Map<number, string>();
  for (const s of ms) {
    const n = s.world.length / 3;
    const xy = new Float64Array(n * 2);
    for (let i = 0; i < n; i++) { const p = projectWorld(view as any, [s.world[i * 3]!, s.world[i * 3 + 1]!, s.world[i * 3 + 2]!]); xy[i * 2] = p.x; xy[i * 2 + 1] = p.y; }
    for (let t = 0; t < s.indices.length; t += 3) {
      const tri = [0, 1, 2].map((k) => ({ x: xy[s.indices[t + k]! * 2]!, y: xy[s.indices[t + k]! * 2 + 1]! }));
      mask.fill([tri], (i: number) => owner.set(i, s.id));
    }
  }
  return { mask, owner };
};
const MA = mk(A), MB = mk(B);
const W = Math.round((w.maxX - w.minX) / cell);
const only = (X: typeof MA, Y: typeof MA) => { const l: number[] = []; for (let i = 0; i < X.mask.cells.length; i++) if (X.mask.cells[i] && !Y.mask.cells[i]) l.push(i); return l; };
for (const [name, X, Y] of [[`only ${sa}`, MA, MB], [`only ${sb}`, MB, MA]] as const) {
  const l = only(X, Y);
  const seen = new Set<number>();
  const clusters: { n: number; x0: number; x1: number; y0: number; y1: number; solids: Set<string> }[] = [];
  const set = new Set(l);
  for (const i of l) {
    if (seen.has(i)) continue;
    const st = [i]; seen.add(i);
    const c = { n: 0, x0: 1e9, x1: -1e9, y0: 1e9, y1: -1e9, solids: new Set<string>() };
    while (st.length) {
      const k = st.pop()!;
      const cx = k % W, cy = Math.floor(k / W);
      c.n++; c.x0 = Math.min(c.x0, w.minX + cx * cell); c.x1 = Math.max(c.x1, w.minX + (cx + 1) * cell); c.y0 = Math.min(c.y0, w.minY + cy * cell); c.y1 = Math.max(c.y1, w.minY + (cy + 1) * cell);
      const o = X.owner.get(k); if (o) c.solids.add(o);
      for (const nb of [k - 1, k + 1, k - W, k + W]) if (set.has(nb) && !seen.has(nb)) { seen.add(nb); st.push(nb); }
    }
    clusters.push(c);
  }
  clusters.sort((p, q) => q.n - p.n);
  console.log(name, "cells", l.length, "clusters", clusters.length);
  for (const c of clusters.slice(0, 8)) console.log(`  ${c.n} cells  x ${c.x0.toFixed(2)}..${c.x1.toFixed(2)}  y ${c.y0.toFixed(2)}..${c.y1.toFixed(2)}  ${[...c.solids].join(",")}`);
}
if (g("--ascii")) {
  const [x0, x1, y0, y1] = g("--ascii")!.split(",").map(Number) as [number, number, number, number];
  const step = Number(g("--astep") ?? 0.02);
  for (let y = y1!; y >= y0!; y -= step) {
    let line = y.toFixed(2).padStart(5) + " ";
    for (let x = x0!; x <= x1!; x += step) {
      const k = Math.floor((y - w.minY) / cell) * W + Math.floor((x - w.minX) / cell);
      const p = MA.mask.cells[k], q = MB.mask.cells[k];
      line += p && q ? "#" : p ? "a" : q ? "b" : ".";
    }
    console.log(line);
  }
}
if (a.includes("--bd")) {
  const bnd = (X: typeof MA) => { const o: number[] = []; const H = X.mask.cells.length / W; for (let i = 0; i < X.mask.cells.length; i++) if (X.mask.cells[i]) { const cx = i % W, cy = Math.floor(i / W); if (cx === 0 || cy === 0 || cx === W - 1 || cy === H - 1 || !X.mask.cells[i - 1] || !X.mask.cells[i + 1] || !X.mask.cells[i - W] || !X.mask.cells[i + W]) o.push(i); } return o; };
  const bA = bnd(MA), bB = bnd(MB);
  const dist = (from: number[], to: number[]) => from.map((i) => { const x = i % W, y = Math.floor(i / W); let best = 1e9; for (const j of to) { const dx = (j % W) - x, dy = Math.floor(j / W) - y; const d = dx * dx + dy * dy; if (d < best) best = d; } return { i, d: Math.sqrt(best) * cell }; });
  for (const [nm, f, t] of [["A->B", bA, bB], ["B->A", bB, bA]] as const) {
    const r = dist(f, t).sort((p, q) => q.d - p.d);
    const all = r.map((q) => q.d).sort((p, q) => p - q);
    console.log(nm, "n", all.length, "p95", all[Math.floor(all.length * 0.95)]!.toFixed(3), "max", all[all.length - 1]!.toFixed(3));
    const shown: string[] = [];
    for (const q of r) { const x = w.minX + (q.i % W) * cell, y = w.minY + Math.floor(q.i / W) * cell; if (shown.every((s) => { const [sx, sy] = s.split(",").map(Number); return Math.hypot(sx! - x, sy! - y) > 0.08; })) shown.push(`${x.toFixed(2)},${y.toFixed(2)}`), console.log(`   d=${q.d.toFixed(3)} at x=${x.toFixed(2)} y=${y.toFixed(2)}`); if (shown.length >= 10) break; }
  }
}
