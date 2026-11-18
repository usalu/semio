/** 🔬 Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
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
if (process.env.OPS) (await import("./animals-appliances-components-a-gen.mjs")).default(asset);
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
  const shapes = ((state as any).views?.[view] ?? (state as any).geometry?.[view] ?? (state as any).shapes?.[view]) as any[];
  const [w, h] = [vol.width, vol.height];
  const rows: [string, number, number][] = [];
  for (const s of shapes) {
    const one = new Mask(box, cell);
    if (s.shape.filled !== false) one.fill(s.rings);
    else continue;
    let n = 0, out = 0;
    for (let k = 0; k < w * h; k++) if (one.cells[k]) { n++; if (!vol.cells[k]) out++; }
    const xs = s.rings.flat().map((q: any) => q.x), ys = s.rings.flat().map((q: any) => q.y);
    rows.push([String(s.shape.id) + ` x ${Math.min(...xs).toFixed(3)}..${Math.max(...xs).toFixed(3)} y ${Math.min(...ys).toFixed(3)}..${Math.max(...ys).toFixed(3)}`, n, out]);
  }
  rows.sort((a, b) => b[2] - a[2]);
  console.log(`== ${view} 1:${scale} cell ${cell} iou ${m.iou.toFixed(3)} H ${m.hausdorffM.toFixed(4)} drawn ${m.counts.drawn} vol ${m.counts.volume} both ${m.counts.both}`);
  for (const r of rows.slice(0, Number(process.env.TOP ?? 14))) console.log("  outside-volume", r[0].padEnd(50), "cells", r[1], "outside", r[2]);
}
