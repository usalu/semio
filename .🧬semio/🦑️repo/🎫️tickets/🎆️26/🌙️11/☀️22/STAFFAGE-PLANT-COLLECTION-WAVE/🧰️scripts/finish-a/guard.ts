/** 🛡️ Finisher A guard: the measures of the catalog-wide silhouette test for one asset document (body box against `dimensions` within one smallest feature per axis; IoU and Hausdorff distance of the body against the drawing in elevation, front and plan at every scale), one line per scale and view and a final `GUARD ok|FAILED n`.
 * Usage: `bun guard.ts <asset document json> [scales, default 500,200,100,50]`
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️catalog-silhouette/🟦️.ts */
import { readFileSync } from "node:fs";
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";

const IOU = 0.92;
const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
const scales = (process.argv[3] ?? "500,200,100,50").split(",").map(Number);
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let failed = 0;
for (const scale of scales) {
  const feature = featureOf(scale as 50);
  const m = measureAsset(asset, scale as 50, undefined, undefined, feature);
  const box = m.bbox.map((b: number, k: number) => {
    const ok = Math.abs(b - dims[k]!) <= feature;
    if (!ok) failed++;
    return `${ok ? "" : "!"}${((b - dims[k]!) * 1000).toFixed(0)}`;
  });
  console.log(`1:${scale} box deviation mm (width, height, depth; limit ${feature * 1000}) ${box.join(" ")}`);
  for (const view of ["elevation", "front", "plan"] as const) {
    const v = m.views[view];
    const ok = v.iou >= IOU && v.hausdorffM <= 2 * feature;
    if (!ok) failed++;
    console.log(`${ok ? " " : "!"} 1:${scale} ${view} iou ${v.iou.toFixed(4)} (min ${IOU}) hausdorff ${(v.hausdorffM * 1000).toFixed(1)} mm (max ${2 * feature * 1000})`);
  }
}
console.log(failed === 0 ? `GUARD ok ${asset.id}` : `GUARD FAILED ${failed} ${asset.id}`);
