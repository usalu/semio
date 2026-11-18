/** 🧱 The fine tiles of the banded measurement of a large object with their Hausdorff distance, the worst first: bun vehicles-tiles.ts <asset> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, TILE_CELLS, WINDOW_FEATURES, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan"];
const scale = Number(scaleArg);
const feature = featureOf(scale);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const coarse = measureView(asset, state, solids, view, scale, 2000, feature, Infinity, true);
const cell = feature / 4;
const [b0, b1] = [{ x: coarse.box.minX + 2 * coarse.cell, y: coarse.box.minY + 2 * coarse.cell }, { x: coarse.box.maxX - 2 * coarse.cell, y: coarse.box.maxY - 2 * coarse.cell }];
const frame = { minX: b0.x - 2 * cell, minY: b0.y - 2 * cell, maxX: b1.x + 2 * cell, maxY: b1.y + 2 * cell };
const side = Number(process.env.TILE ?? TILE_CELLS) * cell;
const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
const out: { i: number; j: number; h: number; x: number; y: number; core: any; box: any }[] = [];
for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
  const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
  const core = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
  const box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
  const tile = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false, { core, box, cell });
  if (tile.hausdorffM > 0) out.push({ core, box, i, j, h: +tile.hausdorffM.toFixed(4), x: +((core.minX + core.maxX) / 2).toFixed(2), y: +((core.minY + core.maxY) / 2).toFixed(2) });
}
out.sort((a, b) => b.h - a.h);
console.log(`limit ${2 * feature}; tile side ${side.toFixed(2)} m; margin ${margin.toFixed(3)}`);
for (const t of out.slice(0, 8)) console.log(`tile (${t.i},${t.j}) centre (${t.x}, ${t.y}) hausdorff ${t.h}`);

for (const t of out.slice(0, Number(process.env.WORST ?? 2))) {
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false, { core: t.core, box: t.box, cell });
  const drawn = new Mask(m.box, m.cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(m.box, m.cell);
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = Math.round((4 * feature / m.cell) ** 2);
  const [bd, bv] = [boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h), boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h)];
  const [toVol, toDrawn] = [distanceField(bv, w, h), distanceField(bd, w, h)];
  const limit = 2 * feature;
  const list: { side: string; x: number; y: number; d: number }[] = [];
  const seen = new Set<string>();
  for (const [side, from, field] of [["drawn-only", bd, toVol], ["volume-only", bv, toDrawn]] as const)
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
      if (!from[j * w + i]) continue;
      const d = Math.sqrt(field[j * w + i]!) * m.cell;
      const [x, y] = [m.box.minX + (i + 0.5) * m.cell, m.box.minY + (j + 0.5) * m.cell];
      if (d <= limit || x < t.core.minX || x > t.core.maxX || y < t.core.minY || y > t.core.maxY) continue;
      const key = `${side}${Math.round(x / 0.3)},${Math.round(y / 0.3)}`;
      if (seen.has(key)) continue;
      seen.add(key);
      list.push({ side, x: +x.toFixed(3), y: +y.toFixed(3), d: +d.toFixed(3) });
    }
  list.sort((a, b) => b.d - a.d);
  console.log(`tile (${t.i},${t.j}):`, JSON.stringify(list.slice(0, 8)));
  if (list.length > 0 && process.env.ASCII !== undefined) {
    const [fd, fv] = [fillSmallHoles(drawn.cells, w, h, holes), fillSmallHoles(vol.cells, w, h, holes)];
    const c = list[0]!, r = 0.25, st = Math.max(1, Math.round(r / m.cell / 40));
    for (let y = c.y + r; y > c.y - r; y -= st * m.cell * 2) {
      let line = "";
      for (let x = c.x - r; x < c.x + r; x += st * m.cell) {
        const k = Math.floor((y - m.box.minY) / m.cell) * w + Math.floor((x - m.box.minX) / m.cell);
        line += fd[k] && fv[k] ? ":" : fd[k] ? "a" : fv[k] ? "b" : ".";
      }
      console.log(line);
    }
  }
}
