/** 🧩 Area mismatch bins (drawn-only D, volume-only V cells per 0.05 m bin) for one view: bun iou-bins.ts <asset> <scale> <view> [bin]. Original: Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun silhouette-diag.ts <asset> <scale> [view] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

void readFileSync;
const [name, scaleArg, only, px, py, pr] = process.argv.slice(2) as [string, string, string | undefined, string | undefined, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const e = extentOf(solids);
console.log("extent min", e.min.map((x) => +x.toFixed(3)), "max", e.max.map((x) => +x.toFixed(3)), "dims", asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM, "feature", feature);
for (const view of VIEWPOINTS) {
  if (only !== undefined && only !== view) continue;
  const m = measureView(asset, state, solids, view, scale, undefined, feature, Infinity, false);
  const cell = m.cell;
  const box = m.box;
  const drawn = new Mask(box, cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(box, cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = (4 * feature / cell) ** 2;
  const bin = Number(process.argv[6] ?? 0.05);
  const bins = new Map<string, [number, number]>();
  let dOnly = 0, vOnly = 0, both = 0;
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    const a = drawn.cells[j * w + i]!, b = vol.cells[j * w + i]!;
    if (a && b) both++;
    if (a === b) continue;
    const k = `${(Math.floor((box.minX + i * cell) / bin) * bin).toFixed(3)},${(Math.floor((box.minY + j * cell) / bin) * bin).toFixed(3)}`;
    const e = bins.get(k) ?? [0, 0];
    if (a) { e[0]++; dOnly++; } else { e[1]++; vOnly++; }
    bins.set(k, e);
  }
  console.log(`== ${view} 1:${scale} cell ${cell.toFixed(4)} both ${both} drawn-only ${dOnly} volume-only ${vOnly} iou ${(both / (both + dOnly + vOnly)).toFixed(3)}`);
  for (const [k, e] of [...bins].sort((x, y) => y[1][0] + y[1][1] - x[1][0] - x[1][1]).slice(0, 12)) console.log(`  bin ${k} (x,y) drawn-only ${(e[0] * cell * cell * 1e4).toFixed(1)} cm2 volume-only ${(e[1] * cell * cell * 1e4).toFixed(1)} cm2`);
}
void GATE;
