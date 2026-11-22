/** 🔍️ Where a view of one scale disagrees with its volume: the boundary points of the drawing far from the volume outline and the reverse, clustered, in world metres of the view (elevation x/y, front z/y, plan x/z), with the shapes and solids nearest to them.
 *  bun diag.ts <file.json> <scale> <view> [count] */
import { S } from "./lib.ts";
const sil = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const { featureOf } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🟦️.ts`);
const [file, sc, view, count] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const scale = Number(sc);
const m = sil.measureAsset(asset, scale, undefined, undefined, featureOf(scale)).views[view!];
const box = m.box, cell = m.cell / Number(process.env.SUB ?? 1);
const W = Math.ceil((box.maxX - box.minX) / cell), H = Math.ceil((box.maxY - box.minY) / cell);
const drawn = new sil.Mask(box, cell), vol = new sil.Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
for (const g of m.reference) vol.fill(g);
const holes = (4 * featureOf(scale) / cell) ** 2;
const a = sil.fillSmallHoles(drawn.cells, drawn.width, drawn.height, holes), b = sil.fillSmallHoles(vol.cells, vol.width, vol.height, holes);
const ea = sil.boundaryOf(a, drawn.width, drawn.height), eb = sil.boundaryOf(b, vol.width, vol.height);
const da = sil.distanceField(ea, drawn.width, drawn.height), db = sil.distanceField(eb, vol.width, vol.height);
const out: { d: number; x: number; y: number; kind: string }[] = [];
for (let j = 0; j < drawn.height; j++) for (let i = 0; i < drawn.width; i++) {
  const k = j * drawn.width + i;
  const x = box.minX + (i + 0.5) * cell, y = box.minY + (j + 0.5) * cell;
  if (ea[k]) out.push({ d: Math.sqrt(db[k]!) * cell, x, y, kind: b[k] ? "drawing edge inside volume" : "drawing beyond volume" });
  if (eb[k]) out.push({ d: Math.sqrt(da[k]!) * cell, x, y, kind: a[k] ? "volume edge inside drawing" : "volume beyond drawing" });
}
out.sort((p, q) => q.d - p.d);
const picked: typeof out = [];
for (const o of out) if (picked.length < Number(count ?? 8) && picked.every((p) => Math.hypot(p.x - o.x, p.y - o.y) > 0.03)) picked.push(o);
console.log(`${asset.id} 1:${scale} ${view}: iou ${m.iou.toFixed(3)} hausdorff ${(m.hausdorffM * 1000).toFixed(1)} mm (limit ${(2 * featureOf(scale) * 1000).toFixed(1)}), cell ${(cell * 1000).toFixed(1)} mm, ${W}x${H}`);
for (const p of picked) console.log(`  ${(p.d * 1000).toFixed(1).padStart(6)} mm  ${p.kind.padEnd(28)} at (${p.x.toFixed(3)}, ${p.y.toFixed(3)})`);
const { posed } = await import("./trace.ts");
const { state, worlds } = posed(asset, sc!);
for (const p of picked) {
  const near = worlds.map((w: any) => {
    let d = Infinity;
    for (let k = 0; k < w.positions.length; k += 3) {
      const q = sil.project(view, w.positions[k], w.positions[k + 1], w.positions[k + 2]);
      d = Math.min(d, Math.hypot(q.x - p.x, q.y - p.y));
    }
    return [w.id, d] as const;
  }).sort((x: any, y: any) => x[1] - y[1]).slice(0, 4);
  const shp = state.shapes[view!].map((s: any) => [s.shape.id, Math.min(...s.rings.flat().map((q: any) => Math.hypot(q.x - p.x, q.y - p.y)))] as const).sort((x: any, y: any) => x[1] - y[1]).slice(0, 4);
  console.log(`  (${p.x.toFixed(3)}, ${p.y.toFixed(3)}) solids ${near.map(([i, d]: any) => `${i} ${(d * 1000).toFixed(0)}`).join(", ")} | shapes ${shp.map(([i, d]: any) => `${i} ${(d * 1000).toFixed(0)}`).join(", ")}`);
}
