/** 🧱 Tiles of 512 cells of one view in which one mask has boundary cells and the other none (the guard counts such a tile with the full tile distance).
 *  bun machinery-tiles.ts <asset> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, fillSmallHoles, measureView, worldSolids, type Viewpoint } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
const [name, scaleArg, view] = process.argv.slice(2) as [string, string, Viewpoint];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((x) => x.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view, scale, 3000, feature, Infinity, false);
const cell = feature / 4;
const tile = 512 * cell;
const margin = 6 * feature;
const holes = Math.round((4 * feature / cell) ** 2);
for (let y = m.box.minY; y < m.box.maxY; y += tile) for (let x = m.box.minX; x < m.box.maxX; x += tile) {
  const box = { minX: x - margin, maxX: x + tile + margin, minY: y - margin, maxY: y + tile + margin };
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const [nd, nv] = [bd.reduce((s, v) => s + v, 0), bv.reduce((s, v) => s + v, 0)];
  if ((nd === 0) !== (nv === 0)) console.log(`tile (${x.toFixed(2)},${y.toFixed(2)}) drawnBoundary ${nd} volumeBoundary ${nv}`);
}
console.log("done");
