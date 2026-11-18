/** 🗺️ Whole-view ASCII of the drawn and volume masks of one asset, scale and view: bun vegetation-ascii.ts <asset> <scale> <view> [cols] [x0 y0 x1 y1] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, extentOf, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, colsArg, ...win] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const e = extentOf(solids);
console.log("extent min", e.min.map((x) => +x.toFixed(3)), "max", e.max.map((x) => +x.toFixed(3)), "dims", asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const box = m.box;
const cell = m.cell;
const drawn = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const vol = new Mask(box, cell);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const w = vol.width;
const h = vol.height;
if (process.env.GUARD) {
  const holes = Math.round((4 * feature / cell) ** 2);
  drawn.cells.set(fillSmallHoles(drawn.cells, w, h, holes));
  vol.cells.set(fillSmallHoles(vol.cells, w, h, holes));
}
const [x0, y0, x1, y1] = win.length === 4 ? win.map(Number) as [number, number, number, number] : [box.minX, box.minY, box.maxX, box.maxY];
const cols = Number(colsArg ?? 100);
const step = Math.max(1, Math.round((x1! - x0!) / cell / cols));
console.log(`view ${view} 1:${scale} box x ${box.minX.toFixed(2)}..${box.maxX.toFixed(2)} y ${box.minY.toFixed(2)}..${box.maxY.toFixed(2)} cell ${cell} step ${step} (block ${(step * cell).toFixed(3)} m) iou ${m.iou.toFixed(3)} H ${m.hausdorffM.toFixed(3)}`);
const rowStep = process.env.ROW1 ? 1 : 2;
for (let y = y1!; y > y0!; y -= step * cell * rowStep) {
  let line = "";
  for (let x = x0!; x < x1!; x += step * cell) {
    let b = 0, d = 0, v = 0, n = 0;
    for (let dj = 0; dj < step * rowStep; dj++) for (let di = 0; di < step; di++) {
      const i = Math.floor((x - box.minX) / cell) + di;
      const j = Math.floor((y - box.minY) / cell) - dj;
      if (i < 0 || j < 0 || i >= w || j >= h) continue;
      const [a, c] = [drawn.cells[j * w + i], vol.cells[j * w + i]];
      n++;
      if (process.env.SHOW === "drawn") { if (a) b++; continue; }
      if (process.env.SHOW === "vol") { if (c) b++; continue; }
      if (a && c) b++; else if (a) d++; else if (c) v++;
    }
    const k = Math.max(b, d, v);
    line += n === 0 ? " " : k === 0 ? "." : d >= v && d === k && d > b / 3 ? "D" : v === k && v > b / 3 ? "V" : b === k ? "#" : d > v ? "D" : "V";
  }
  console.log(`${y.toFixed(1).padStart(6)} ${line}`);
}
