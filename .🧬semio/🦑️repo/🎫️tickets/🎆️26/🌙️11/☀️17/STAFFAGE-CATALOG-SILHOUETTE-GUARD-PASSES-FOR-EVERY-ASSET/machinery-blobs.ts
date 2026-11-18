/** 🔬 Mismatch blobs of one asset, scale and view: connected groups of boundary cells farther than the factor times the limit from the other boundary, with bounding box, worst distance, the volume solids and drawn shapes that touch it.
 *  bun machinery-blobs.ts <asset> <scale> <view> [factor=1] [patch: px py r] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids, project, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, factorArg, px, py, pr] = process.argv.slice(2) as [string, string, Viewpoint, string | undefined, string | undefined, string | undefined, string | undefined];
const scale = Number(scaleArg);
const factor = Number(factorArg ?? 1);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view, scale, process.env.CELLS === undefined ? undefined : Number(process.env.CELLS), feature, Infinity, false);
const win = process.env.WIN === undefined ? undefined : process.env.WIN.split(",").map(Number);
const cell = win === undefined ? m.cell : feature / 4;
const box = win === undefined ? m.box : { minX: win[0]! - win[2]!, maxX: win[0]! + win[2]!, minY: win[1]! - win[2]!, maxY: win[1]! + win[2]! };
const drawn = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const vol = new Mask(box, cell);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.round((4 * feature / cell) ** 2);
const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
const toVol = distanceField(bv, w, h);
const toDrawn = distanceField(bd, w, h);
const limit = 2 * feature;
const solidBoxes = solids.map((s) => {
  const xs: number[] = [], ys: number[] = [];
  for (let i = 0; i < s.positions.length; i += 3) { const p = project(view, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!); xs.push(p.x); ys.push(p.y); }
  return { id: s.id, slender: s.slender === true, x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys) };
});
const shapes = (state.views?.[view] ?? state.geometry?.[view] ?? state.shapes?.[view] ?? []) as any[];
const shapeBoxes = shapes.map((s) => { const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y); return { id: String(s.shape.id), x0: Math.min(...xs), x1: Math.max(...xs), y0: Math.min(...ys), y1: Math.max(...ys), filled: s.shape.filled !== false, closed: s.closed }; });
for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const) {
  const flag = new Uint8Array(w * h);
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit * factor) flag[j * w + i] = 1;
  const seen = new Uint8Array(w * h);
  const reach = Math.max(1, Math.round((limit * 0.75) / cell));
  const blobs: { i0: number; i1: number; j0: number; j1: number; d: number; n: number }[] = [];
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (!flag[j * w + i] || seen[j * w + i]) continue;
    const b = { i0: i, i1: i, j0: j, j1: j, d: 0, n: 0 };
    const stack = [[i, j]];
    seen[j * w + i] = 1;
    while (stack.length > 0) {
      const [ci, cj] = stack.pop()!;
      b.i0 = Math.min(b.i0, ci!); b.i1 = Math.max(b.i1, ci!); b.j0 = Math.min(b.j0, cj!); b.j1 = Math.max(b.j1, cj!); b.n++;
      b.d = Math.max(b.d, Math.sqrt(field[cj! * w + ci!]!) * cell);
      for (let dj = -reach; dj <= reach; dj++) for (let di = -reach; di <= reach; di++) {
        const [ni, nj] = [ci! + di, cj! + dj];
        if (ni < 0 || nj < 0 || ni >= w || nj >= h || !flag[nj * w + ni] || seen[nj * w + ni]) continue;
        seen[nj * w + ni] = 1;
        stack.push([ni, nj]);
      }
    }
    blobs.push(b);
  }
  blobs.sort((a, b) => b.d - a.d);
  for (const b of blobs.slice(0, 12)) {
    const [x0, x1, y0, y1] = [box.minX + b.i0 * cell, box.minX + (b.i1 + 1) * cell, box.minY + b.j0 * cell, box.minY + (b.j1 + 1) * cell];
    const pad = limit;
    const near = (o: { x0: number; x1: number; y0: number; y1: number }) => o.x1 >= x0 - pad && o.x0 <= x1 + pad && o.y1 >= y0 - pad && o.y0 <= y1 + pad;
    const list = side === "volume-only" ? solidBoxes.filter(near).filter((o) => o.x1 - o.x0 < 1e9).map((o) => `${o.id}${o.slender ? "*" : ""}[${o.x0.toFixed(2)},${o.x1.toFixed(2)}|${o.y0.toFixed(2)},${o.y1.toFixed(2)}]`) : shapeBoxes.filter(near).map((o) => `${o.id}${o.filled ? "" : "~"}[${o.x0.toFixed(2)},${o.x1.toFixed(2)}|${o.y0.toFixed(2)},${o.y1.toFixed(2)}]`);
    console.log(`${side} d=${b.d.toFixed(3)} n=${b.n} x ${x0.toFixed(3)}..${x1.toFixed(3)} y ${y0.toFixed(3)}..${y1.toFixed(3)}  near: ${list.slice(0, 8).join(" ")}`);
  }
}
if (px !== undefined && py !== undefined) {
  const r = Number(pr ?? 0.1);
  const step = Math.max(1, Math.round((2 * r) / cell / 70));
  for (let y = Number(py) + r; y > Number(py) - r; y -= step * cell * 2) {
    let line = "";
    for (let x = Number(px) - r; x < Number(px) + r; x += step * cell) {
      const i = Math.floor((x - box.minX) / cell), j = Math.floor((y - box.minY) / cell);
      const [a, b] = [drawn.cells[j * w + i], vol.cells[j * w + i]];
      line += a && b ? "#" : a ? "D" : b ? "V" : ".";
    }
    console.log(line);
  }
}
console.log(`== ${view} 1:${scale} iou ${m.iou.toFixed(3)} hausdorff ${m.hausdorffM.toFixed(4)} limit ${limit}`);
void extentOf;
