/** 🔎️ Round 3 horse: lists the structure solids whose vertices leave the body silhouette of a scale, per view, with the distance. Usage: bun r3-horse-inside.ts <scale> */
import { entryOf, loadSource, merged, meshesAt, type Json } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? 50);
const { entry, asset: catalog } = entryOf("animals/horse");
const asset = merged(catalog, loadSource(entry)!);
const vol = asset.volumes[String(scale)];
const body = meshesAt(asset, scale, vol.body as Json[]);
const structure = meshesAt(asset, scale, vol.structure as Json[]);
const cell = 0.0025;
for (const [view, ax, ay] of [["elevation", 0, 1], ["front", 2, 1], ["plan", 0, 2]] as const) {
  const [ox, oy, w, h] = [-1, -0.7, 1600, 1400];
  const g = new Uint8Array(w * h);
  for (const m of body) for (let t = 0; t < m.indices.length; t += 3) {
    const p = [0, 1, 2].map((k) => [m.world[3 * m.indices[t + k]! + ax]!, m.world[3 * m.indices[t + k]! + ay]!]);
    const d = (p[1]![0]! - p[0]![0]!) * (p[2]![1]! - p[0]![1]!) - (p[2]![0]! - p[0]![0]!) * (p[1]![1]! - p[0]![1]!);
    if (Math.abs(d) < 1e-14) continue;
    const xs = p.map((q) => q[0]!);
    const ys = p.map((q) => q[1]!);
    for (let j = Math.floor((Math.min(...ys) - oy) / cell); j <= Math.ceil((Math.max(...ys) - oy) / cell); j++) for (let i = Math.floor((Math.min(...xs) - ox) / cell); i <= Math.ceil((Math.max(...xs) - ox) / cell); i++) {
      const [px, py] = [ox + (i + 0.5) * cell, oy + (j + 0.5) * cell];
      const u = ((p[1]![0]! - p[0]![0]!) * (py - p[0]![1]!) - (px - p[0]![0]!) * (p[1]![1]! - p[0]![1]!)) / d;
      const v = ((px - p[0]![0]!) * (p[2]![1]! - p[0]![1]!) - (p[2]![0]! - p[0]![0]!) * (py - p[0]![1]!)) / d;
      if (u >= 0 && v >= 0 && u + v <= 1) g[j * w + i] = 1;
    }
  }
  const rows: [number, string][] = [];
  for (const m of structure) {
    let worst = 0;
    for (let k = 0; k < m.world.length; k += 3) {
      const i = Math.floor((m.world[k + ax]! - ox) / cell);
      const j = Math.floor((m.world[k + ay]! - oy) / cell);
      if (g[j * w + i]) continue;
      let best = 40;
      for (let dj = -16; dj <= 16; dj++) for (let di = -16; di <= 16; di++) if (g[(j + dj) * w + i + di]) best = Math.min(best, Math.hypot(di, dj));
      worst = Math.max(worst, best * cell);
    }
    if (worst > 0.004) rows.push([worst, m.id]);
  }
  console.log(view, rows.sort((a, b) => b[0] - a[0]).slice(0, 12).map(([d, id]) => `${id} ${(d * 1000).toFixed(0)}`).join(", "));
}
