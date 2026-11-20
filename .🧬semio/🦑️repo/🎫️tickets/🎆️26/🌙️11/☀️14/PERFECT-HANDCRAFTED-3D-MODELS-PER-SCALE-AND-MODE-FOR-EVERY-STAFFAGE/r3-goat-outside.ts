/** 🔦️ Round 3 goat: which structure solids leave the body silhouette, per view, with the largest distance outside. Usage: bun r3-goat-outside.ts <scale> */
import { distanceField, entryOf, featureM, loadSource, merged, meshMask, meshesAt, projectedPoints, type Json, type View } from "./volumes-lib.ts";
const scale = Number(process.argv[2] ?? 50);
const { entry, asset: catalog } = entryOf("animals/goat");
const asset: Json = merged(catalog, loadSource(entry)!);
const v = asset.volumes[String(scale)];
const body = meshesAt(asset, scale, v.body);
const own: Json[] = v.structure.filter((s: Json) => s.of === undefined);
const meshes = meshesAt(asset, scale, own);
const cell = featureM(scale, asset) / 8;
for (const view of ["elevation", "front", "plan"] as View[]) {
  const pts = projectedPoints(view, [...body, ...meshes]);
  const box = { minX: Math.min(...pts.map((p) => p.x)) - 4 * cell, minY: Math.min(...pts.map((p) => p.y)) - 4 * cell, maxX: Math.max(...pts.map((p) => p.x)) + 4 * cell, maxY: Math.max(...pts.map((p) => p.y)) + 4 * cell };
  const field = distanceField(meshMask(view, body, box, cell));
  meshes.forEach((m, i) => {
    const mask = meshMask(view, [m], box, cell);
    let worst = 0;
    for (let k = 0; k < mask.cells.length; k++) if (mask.cells[k]! > 0 && field[k]! > worst) worst = field[k]!;
    if (worst > 0.0015) console.log(view, own[i]!.id, worst.toFixed(4));
  });
}
{
  const inside = (p: [number, number, number]): boolean => {
    for (const m of body) {
      let hits = 0;
      const d = [0.5773, 0.5774, 0.5772];
      for (let t = 0; t < m.indices.length; t += 3) {
        const v = [0, 1, 2].map((k) => [m.world[m.indices[t + k]! * 3]!, m.world[m.indices[t + k]! * 3 + 1]!, m.world[m.indices[t + k]! * 3 + 2]!]);
        const e1 = [v[1]![0]! - v[0]![0]!, v[1]![1]! - v[0]![1]!, v[1]![2]! - v[0]![2]!];
        const e2 = [v[2]![0]! - v[0]![0]!, v[2]![1]! - v[0]![1]!, v[2]![2]! - v[0]![2]!];
        const h = [d[1]! * e2[2]! - d[2]! * e2[1]!, d[2]! * e2[0]! - d[0]! * e2[2]!, d[0]! * e2[1]! - d[1]! * e2[0]!];
        const det = e1[0]! * h[0]! + e1[1]! * h[1]! + e1[2]! * h[2]!;
        if (Math.abs(det) < 1e-14) continue;
        const s = [p[0] - v[0]![0]!, p[1] - v[0]![1]!, p[2] - v[0]![2]!];
        const u = (s[0]! * h[0]! + s[1]! * h[1]! + s[2]! * h[2]!) / det;
        if (u < 0 || u > 1) continue;
        const q = [s[1]! * e1[2]! - s[2]! * e1[1]!, s[2]! * e1[0]! - s[0]! * e1[2]!, s[0]! * e1[1]! - s[1]! * e1[0]!];
        const w = (d[0]! * q[0]! + d[1]! * q[1]! + d[2]! * q[2]!) / det;
        if (w < 0 || u + w > 1) continue;
        if ((e2[0]! * q[0]! + e2[1]! * q[1]! + e2[2]! * q[2]!) / det > 1e-9) hits++;
      }
      if (hits % 2 === 1) return true;
    }
    return false;
  };
  meshes.forEach((m, i) => {
    if (own[i]!.part !== "member") return;
    let out = 0;
    const n = m.world.length / 3;
    const step = Math.max(1, Math.floor(n / 150));
    let seen = 0;
    for (let k = 0; k < n; k += step) { seen++; if (!inside([m.world[3 * k]!, m.world[3 * k + 1]!, m.world[3 * k + 2]!])) out++; }
    if (out / seen > 0.08) console.log("3d", own[i]!.id, `${out}/${seen} sampled vertices outside the body`);
  });
}
