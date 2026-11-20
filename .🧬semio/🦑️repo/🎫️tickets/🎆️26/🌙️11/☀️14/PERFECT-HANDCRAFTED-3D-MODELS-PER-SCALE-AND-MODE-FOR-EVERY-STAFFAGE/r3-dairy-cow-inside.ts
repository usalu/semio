/** 🩻 Round 3 dairy cow: which structure solid leaves the body silhouette in which view, and by how much (vertex test on a 5 mm raster of the body projection).
 *
 * Usage: bun r3-dairy-cow-inside.ts 50 */
import { entryOf, loadSource, merged, meshesAt, type Json } from "./volumes-lib.ts";

const scale = Number(process.argv[2]);
const { entry, asset: catalog } = entryOf("animals/dairy-cow");
const asset = merged(catalog, loadSource(entry)!);
const volume = asset.volumes[String(scale)];
const body = meshesAt(asset, scale, volume.body as Json[]);
const structure = meshesAt(asset, scale, (volume.structure as Json[]).filter((s) => s.kind));
const cell = 0.005;
const views: Record<string, [number, number]> = { elevation: [0, 1], front: [2, 1], plan: [0, 2] };
for (const [view, [a, b]] of Object.entries(views)) {
  const filled = new Set<string>();
  for (const m of body) for (let t = 0; t < m.indices.length; t += 3) {
    const p = [0, 1, 2].map((k) => [m.world[3 * m.indices[t + k]! + a]!, m.world[3 * m.indices[t + k]! + b]!]);
    const [x0, x1] = [Math.min(...p.map((q) => q[0]!)), Math.max(...p.map((q) => q[0]!))];
    const [y0, y1] = [Math.min(...p.map((q) => q[1]!)), Math.max(...p.map((q) => q[1]!))];
    for (let i = Math.floor(x0 / cell); i <= Math.floor(x1 / cell); i++) for (let j = Math.floor(y0 / cell); j <= Math.floor(y1 / cell); j++) {
      const [px, py] = [(i + 0.5) * cell, (j + 0.5) * cell];
      const d = (p[1]![0]! - p[0]![0]!) * (p[2]![1]! - p[0]![1]!) - (p[2]![0]! - p[0]![0]!) * (p[1]![1]! - p[0]![1]!);
      if (Math.abs(d) < 1e-14) continue;
      const u = ((p[1]![0]! - p[0]![0]!) * (py - p[0]![1]!) - (px - p[0]![0]!) * (p[1]![1]! - p[0]![1]!)) / d;
      const v = ((px - p[0]![0]!) * (p[2]![1]! - p[0]![1]!) - (p[2]![0]! - p[0]![0]!) * (py - p[0]![1]!)) / d;
      if (u >= -0.05 && v >= -0.05 && u + v <= 1.05) filled.add(`${i},${j}`);
    }
  }
  for (const m of structure) {
    let worst = 0;
    for (let k = 0; k < m.world.length; k += 3) {
      const [i, j] = [Math.floor(m.world[k + a]! / cell), Math.floor(m.world[k + b]! / cell)];
      let near = 99;
      for (let r = 0; r < 12 && near === 99; r++) for (let di = -r; di <= r && near === 99; di++) for (let dj = -r; dj <= r; dj++) if (filled.has(`${i + di},${j + dj}`)) { near = r; break; }
      worst = Math.max(worst, near);
    }
    if (worst > 1) console.log(view, m.id, `about ${(worst * cell * 1000).toFixed(0)} mm outside`);
  }
}
