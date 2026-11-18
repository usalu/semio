/** 🔭 Scan of one view at fine cell size in windows: per window the worst boundary distance beyond the limit and its position, for the windows that fail.
 *  bun machinery-scan.ts <asset> <scale> <view> [half=1.2] [x0 x1 y0 y1] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view, halfArg, a, b, c, e] = process.argv.slice(2) as [string, string, Viewpoint, string | undefined, string | undefined, string | undefined, string | undefined, string | undefined];
const scale = Number(scaleArg);
const half = Number(halfArg ?? 1.2);
const entry = (await assetPathsFromFiles()).find((x) => x.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view, scale, 3000, feature, Infinity, false);
const cell = feature / 4;
const limit = 2 * feature;
const [bx0, bx1, by0, by1] = a === undefined ? [m.box.minX, m.box.maxX, m.box.minY, m.box.maxY] : [Number(a), Number(b), Number(c), Number(e)];
const step = 2 * half - 0.4;
let worstAll = 0;
for (let cy = by0 + half; cy < by1 + half; cy += step) for (let cx = bx0 + half; cx < bx1 + half; cx += step) {
  const box = { minX: cx - half, maxX: cx + half, minY: cy - half, maxY: cy + half };
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  if (!drawn.cells.some((x) => x) && !vol.cells.some((x) => x)) continue;
  const holes = Math.round((4 * feature / cell) ** 2);
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const toVol = distanceField(bv, w, h);
  const toDrawn = distanceField(bd, w, h);
  let worst = 0, wx = 0, wy = 0, side = "";
  const margin = Math.ceil(0.3 / cell);
  for (const [s, from, field] of [["drawn", bd, toVol], ["vol", bv, toDrawn]] as const) for (let j = margin; j < h - margin; j++) for (let i = margin; i < w - margin; i++) if (from[j * w + i]) {
    const d = Math.sqrt(field[j * w + i]!) * cell;
    if (d > worst && d < 1e6) [worst, wx, wy, side] = [d, box.minX + i * cell, box.minY + j * cell, s];
  }
  worstAll = Math.max(worstAll, worst);
  if (worst > limit) console.log(`window (${cx.toFixed(1)},${cy.toFixed(1)}) worst ${worst.toFixed(3)} ${side}-only at (${wx.toFixed(3)},${wy.toFixed(3)})`);
}
console.log("worst", worstAll.toFixed(4), "limit", limit);
