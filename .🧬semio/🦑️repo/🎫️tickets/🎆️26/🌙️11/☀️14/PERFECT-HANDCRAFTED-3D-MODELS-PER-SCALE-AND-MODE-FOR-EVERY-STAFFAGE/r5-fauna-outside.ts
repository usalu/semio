/** 🔍️ Lists the structure solids of an animal scale whose elevation projection leaves the body silhouette, with the largest distance outside (raster of 2 mm).
 * Usage: bun r5-fauna-outside.ts <asset> <scale> */
import { openCtx } from "./kit-lib.ts";
import { meshesAt } from "./volumes-lib.ts";
const [spec, sc] = [process.argv[2]!, Number(process.argv[3])];
const c = openCtx(spec, sc);
const cell = 0.002;
const body = meshesAt(c.m, sc, c.vol.body);
const st = meshesAt(c.m, sc, c.vol.structure);
const fill = (ms: any[]): Set<string> => {
  const g = new Set<string>();
  for (const m of ms) for (let t = 0; t < m.indices.length; t += 3) {
    const p = [0, 1, 2].map((k) => { const i = m.indices[t + k]! * 3; return [m.world[i]! / cell, m.world[i + 1]! / cell]; });
    const [A, B, C] = p as [number[], number[], number[]];
    const d = (B[0]! - A[0]!) * (C[1]! - A[1]!) - (B[1]! - A[1]!) * (C[0]! - A[0]!);
    if (Math.abs(d) < 1e-12) continue;
    for (let y = Math.floor(Math.min(A[1]!, B[1]!, C[1]!)); y <= Math.ceil(Math.max(A[1]!, B[1]!, C[1]!)); y++) for (let x = Math.floor(Math.min(A[0]!, B[0]!, C[0]!)); x <= Math.ceil(Math.max(A[0]!, B[0]!, C[0]!)); x++) {
      const u = ((B[0]! - A[0]!) * (y - A[1]!) - (B[1]! - A[1]!) * (x - A[0]!)) / d;
      const v = ((x - A[0]!) * (C[1]! - A[1]!) - (y - A[1]!) * (C[0]! - A[0]!)) / d;
      if (u >= -0.01 && v >= -0.01 && u + v <= 1.01) g.add(`${x},${y}`);
    }
  }
  return g;
};
const B = fill(body);
const solids = c.vol.structure as any[];
st.forEach((m: any, i: number) => {
  let worst = 0;
  for (let k = 0; k < m.world.length; k += 3) {
    const x = Math.round(m.world[k]! / cell); const y = Math.round(m.world[k + 1]! / cell);
    if (B.has(`${x},${y}`)) continue;
    let r = 1; while (r < 60) { let hit = false; for (let a = -r; a <= r && !hit; a++) for (const b of [-r, r]) if (B.has(`${x + a},${y + b}`) || B.has(`${x + b},${y + a}`)) hit = true; if (hit) break; r++; }
    worst = Math.max(worst, r * cell);
  }
  if (worst > 0.0019) console.log((m.id ?? solids[i]?.id ?? i), worst.toFixed(3));
});
