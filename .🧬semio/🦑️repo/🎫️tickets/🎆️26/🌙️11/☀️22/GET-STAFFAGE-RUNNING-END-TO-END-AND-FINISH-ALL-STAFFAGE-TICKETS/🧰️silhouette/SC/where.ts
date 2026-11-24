/** 🧭️ Where a view of a component fails the gate: the clusters of boundary cells of the drawing (D) and of the volume (V) that lie farther than the limit from the boundary of the other, with their paper coordinates in metres, and the size of what the sharp reference has to rasterise.
 *  bun where.ts <id|file.json> <scale> <view> [limit-mm] */
import { S, fileOf } from "./lib.ts";
const { Mask, boundaryOf, fillSmallHoles, measureView, worldSolids, distanceField } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts`);
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);

const [name, scaleArg, view, limitArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg), feature = featureOf(scale);
const limit = limitArg === undefined ? 2 * feature : Number(limitArg) / 1000;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const { cell, box } = m;
const bytes = (groups: readonly (readonly (readonly unknown[])[])[]): number => groups.reduce((t, g) => t + g.reduce((u, r) => u + 2 + r.length * 16, 0), 0);
console.log(`${asset.id} 1:${scale} ${view}${m.section ? " (section)" : ""} iou ${m.iou.toFixed(4)} H ${(m.hausdorffM * 1000).toFixed(1)} mm cell ${(cell * 1000).toFixed(2)} mm box x ${box.minX.toFixed(3)}..${box.maxX.toFixed(3)} y ${box.minY.toFixed(3)}..${box.maxY.toFixed(3)}; groups filled ${m.filled.length} (${bytes(m.filled)} B) outlines ${m.outlines.length} (${bytes(m.outlines)} B) reference ${m.reference.length} (${bytes(m.reference)} B) visible ${m.visible.length} (${bytes(m.visible)} B) full ${m.full.length} (${bytes(m.full)} B)`);
const drawn = new Mask(box, cell), vol = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.max(1, Math.round((4 * feature / cell) ** 2));
const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
for (const [label, from, to] of [["D", bd, bv], ["V", bv, bd]] as const) {
  const field = distanceField(to, w, h);
  const far: number[] = [];
  for (let k = 0; k < from.length; k++) if (from[k] && Math.sqrt(field[k]!) * cell > limit) far.push(k);
  const seen = new Set<number>();
  const clusters: { n: number; worst: number; wx: number; wy: number; x0: number; x1: number; y0: number; y1: number }[] = [];
  const set = new Set(far);
  for (const start of far) {
    if (seen.has(start)) continue;
    const c = { n: 0, worst: 0, wx: 0, wy: 0, x0: Infinity, x1: -Infinity, y0: Infinity, y1: -Infinity };
    const queue = [start];
    seen.add(start);
    while (queue.length > 0) {
      const k = queue.pop()!;
      const [i, j] = [k % w, Math.floor(k / w)];
      const [x, y] = [box.minX + (i + 0.5) * cell, box.minY + (j + 0.5) * cell];
      const d = Math.sqrt(field[k]!) * cell;
      c.n++;
      if (d > c.worst) [c.worst, c.wx, c.wy] = [d, x, y];
      [c.x0, c.x1, c.y0, c.y1] = [Math.min(c.x0, x), Math.max(c.x1, x), Math.min(c.y0, y), Math.max(c.y1, y)];
      for (let dj = -3; dj <= 3; dj++) for (let di = -3; di <= 3; di++) {
        const [a, b] = [i + di, j + dj];
        if (a < 0 || b < 0 || a >= w || b >= h) continue;
        const n = b * w + a;
        if (set.has(n) && !seen.has(n)) { seen.add(n); queue.push(n); }
      }
    }
    clusters.push(c);
  }
  clusters.sort((p, q) => q.worst - p.worst);
  console.log(`  ${label}-boundary cells beyond ${(limit * 1000).toFixed(1)} mm: ${far.length} in ${clusters.length} clusters`);
  for (const c of clusters.slice(0, Number(process.env.N ?? 12))) console.log(`    ${label} ${(c.worst * 1000).toFixed(1)} mm at (${c.wx.toFixed(4)}, ${c.wy.toFixed(4)}) n ${c.n} x ${c.x0.toFixed(4)}..${c.x1.toFixed(4)} y ${c.y0.toFixed(4)}..${c.y1.toFixed(4)}`);
}
