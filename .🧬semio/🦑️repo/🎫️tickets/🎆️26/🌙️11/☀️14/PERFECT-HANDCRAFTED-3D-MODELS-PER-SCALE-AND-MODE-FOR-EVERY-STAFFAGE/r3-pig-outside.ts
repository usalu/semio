/** 🔎️ Lists the structure (or thermal) solids of animals/pig that reach outside the body silhouette of a view, with the distance, on a 2 mm raster. Usage: bun r3-pig-outside.ts <scale> [elevation|front|plan] [structure|thermal] */
import { openCtx } from "./kit-lib.ts";
import { meshesAt, resolveList } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 50);
const view = (process.argv[3] ?? "elevation") as "elevation" | "front" | "plan";
const list = (process.argv[4] ?? "structure") as "structure" | "thermal";
const [a, b] = { elevation: [0, 1], front: [2, 1], plan: [0, 2] }[view];
const c = openCtx("animals/pig", scale);
const cell = 0.002;
const body = meshesAt(c.m, scale, c.vol.body);
const on = new Set<string>();
for (const m of body) for (let t = 0; t < m.indices.length; t += 3) {
  const p = [0, 1, 2].map((k) => [m.world[m.indices[t + k]! * 3 + a]! / cell, m.world[m.indices[t + k]! * 3 + b]! / cell]);
  const xs = p.map((q) => q[0]!); const ys = p.map((q) => q[1]!);
  for (let y = Math.floor(Math.min(...ys)); y <= Math.ceil(Math.max(...ys)); y++) for (let x = Math.floor(Math.min(...xs)); x <= Math.ceil(Math.max(...xs)); x++) {
    const [A, B, C] = p as [number[], number[], number[]];
    const d = (B[0]! - A[0]!) * (C[1]! - A[1]!) - (B[1]! - A[1]!) * (C[0]! - A[0]!);
    if (Math.abs(d) < 1e-12) continue;
    const u = ((B[0]! - A[0]!) * (y - A[1]!) - (B[1]! - A[1]!) * (x - A[0]!)) / d;
    const v = ((x - A[0]!) * (C[1]! - A[1]!) - (y - A[1]!) * (C[0]! - A[0]!)) / d;
    if (u >= -0.05 && v >= -0.05 && u + v <= 1.05) on.add(`${x},${y}`);
  }
}
const near = (x: number, y: number): number => { for (let r = 0; r < 60; r++) for (let dx = -r; dx <= r; dx++) for (const dy of [-r, r]) if (on.has(`${x + dx},${y + dy}`) || on.has(`${x + dy},${y + dx}`)) return r * cell; return 60 * cell; };
const solids = resolveList(c.vol, list);
for (const m of meshesAt(c.m, scale, solids)) {
  let worst = 0;
  for (let i = 0; i < m.world.length; i += 3) { const x = Math.round(m.world[i + a]! / cell); const y = Math.round(m.world[i + b]! / cell); if (!on.has(`${x},${y}`)) worst = Math.max(worst, near(x, y)); }
  if (worst > 0.004) console.log(`${m.id} ${(worst * 1000).toFixed(0)} mm`);
}
