/** 🔬 Where the drawing and the volume of one asset disagree: worst mismatch clusters per view at one scale, in the world frame of the view (paper-view x, y in metres), with the drawn shape or volume solid nearest.
 *  bun silhouette-diag.ts <asset> <scale> [view] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GATE, Mask, VIEWPOINTS, boundaryOf, distanceField, fillSmallHoles, extentOf, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

/** 📏 Column or row profile of the drawn and the volume masks of one asset, scale and view: runs of covered cells per line, differing lines flagged.
 *  bun animals-appliances-components-b-prof.ts <asset> <scale> <view> <x|y> [stepM] [from] [to] */
const [name, scaleArg, view, axis, stepArg, fromArg, toArg] = process.argv.slice(2) as [string, string, string, string, string | undefined, string | undefined, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const feature = featureOf(scale);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
const { cell, box } = m;
const drawn = new Mask(box, cell);
for (const g of m.filled) drawn.fill(g);
for (const g of m.outlines) drawn.fill(g);
const vol = new Mask(box, cell);
for (const g of m.reference) vol.fill(g, true);
if (m.section) for (const g of m.visible) vol.fill(g, true);
vol.seal();
const [w, h] = [vol.width, vol.height];
const holes = Math.round((4 * feature / cell) ** 2);
const dc = fillSmallHoles(drawn.cells, w, h, holes);
const vc = fillSmallHoles(vol.cells, w, h, holes);
const step = Math.max(1, Math.round(Number(stepArg ?? 0.05) / cell));
const runs = (get: (k: number) => number, n: number, o: number) => {
  const r: string[] = [];
  let s = -1;
  for (let k = 0; k <= n; k++) {
    const on = k < n && get(k) !== 0;
    if (on && s < 0) s = k;
    if (!on && s >= 0) { r.push(`${(o + s * cell).toFixed(3)}..${(o + k * cell).toFixed(3)}`); s = -1; }
  }
  return r.join(" ");
};
console.log(`${view} 1:${scale} cell ${cell} box x ${box.minX.toFixed(3)} ${box.maxX.toFixed(3)} y ${box.minY.toFixed(3)} ${box.maxY.toFixed(3)} iou ${m.iou.toFixed(3)} H ${m.hausdorffM.toFixed(4)}`);
if (axis === "x") {
  for (let i = 0; i < w; i += step) {
    const x = box.minX + (i + 0.5) * cell;
    if (fromArg !== undefined && x < Number(fromArg)) continue;
    if (toArg !== undefined && x > Number(toArg)) continue;
    const a = runs((j) => dc[j * w + i]!, h, box.minY), b = runs((j) => vc[j * w + i]!, h, box.minY);
    console.log(`x ${x.toFixed(3)} ${a === b ? "=" : "!"} D ${a} | V ${b}`);
  }
} else {
  for (let j = 0; j < h; j += step) {
    const y = box.minY + (j + 0.5) * cell;
    if (fromArg !== undefined && y < Number(fromArg)) continue;
    if (toArg !== undefined && y > Number(toArg)) continue;
    const a = runs((i) => dc[j * w + i]!, w, box.minX), b = runs((i) => vc[j * w + i]!, w, box.minX);
    console.log(`y ${y.toFixed(3)} ${a === b ? "=" : "!"} D ${a} | V ${b}`);
  }
}
