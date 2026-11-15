/** 🔬️ Boundary distance 3D body vs drawn shapes in one view; worst clusters. Usage: bun author-32-dd.ts cat/slug scale view [cell] */
import { Mask, projectWorld, scaleMeshes } from "./volumes-lib.ts";
import { openCtx, drawn } from "./kit-lib.ts";
const [spec, sc, view, cellArg] = process.argv.slice(2);
const c = openCtx(spec!, Number(sc));
const cell = Number(cellArg ?? c.feature / 4);
const ms = ((scaleMeshes(c.m, Number(sc), c.m.volumes[sc!]) as any).body as any[]).filter((s) => s.slender !== true);
const b = { minX: -1, minY: -1, maxX: 14, maxY: 4 };
const box = view === "elevation" ? b : view === "front" ? { minX: -2, minY: -0.5, maxX: 2, maxY: 4 } : { minX: -1, minY: -2, maxX: 14, maxY: 2 };
const A = new Mask(box, cell), B = new Mask(box, cell);
for (const s of drawn(c, view as any)) if (["body", "detail", "glazing"].includes(s.role)) B.fill(s.rings);
for (const s of ms) {
  const n = s.world.length / 3, xy = new Float64Array(n * 2);
  for (let i = 0; i < n; i++) { const p = projectWorld(view as any, [s.world[i * 3], s.world[i * 3 + 1], s.world[i * 3 + 2]]); xy[i * 2] = p.x; xy[i * 2 + 1] = p.y; }
  for (let t = 0; t < s.indices.length; t += 3) A.fill([[0, 1, 2].map((k) => ({ x: xy[s.indices[t + k] * 2], y: xy[s.indices[t + k] * 2 + 1] }))]);
}
const W = Math.round((box.maxX - box.minX) / cell), H = Math.round((box.maxY - box.minY) / cell);
const bnd = (M: Mask) => { const o: number[] = []; for (let i = 0; i < M.cells.length; i++) if (M.cells[i]) { const x = i % W, y = Math.floor(i / W); if (x === 0 || y === 0 || x === W - 1 || y === H - 1 || !M.cells[i - 1] || !M.cells[i + 1] || !M.cells[i - W] || !M.cells[i + W]) o.push(i); } return o; };
const bA = bnd(A), bB = bnd(B);
const dist = (f: number[], t: number[]) => { const tx = t.map((j) => [j % W, Math.floor(j / W)]); return f.map((i) => { const x = i % W, y = Math.floor(i / W); let best = 1e9; for (const [jx, jy] of tx) { const d = (jx - x) ** 2 + (jy - y) ** 2; if (d < best) best = d; } return { i, d: Math.sqrt(best) * cell }; }); };
for (const [nm, f, t] of [["3D->drawn", bA, bB], ["drawn->3D", bB, bA]] as const) {
  const r = dist(f, t).sort((p, q) => q.d - p.d);
  const all = r.map((q) => q.d).sort((p, q) => p - q);
  console.log(nm, "n", all.length, "p95", all[Math.floor(all.length * 0.95)]!.toFixed(3));
  const shown: number[][] = [];
  for (const q of r) { const x = box.minX + (q.i % W) * cell, y = box.minY + Math.floor(q.i / W) * cell; if (shown.every((s) => Math.hypot(s[0]! - x, s[1]! - y) > 0.3)) { shown.push([x, y]); console.log(`   d=${q.d.toFixed(3)} at (${x.toFixed(2)}, ${y.toFixed(2)})`); } if (shown.length >= 6) break; }
}
