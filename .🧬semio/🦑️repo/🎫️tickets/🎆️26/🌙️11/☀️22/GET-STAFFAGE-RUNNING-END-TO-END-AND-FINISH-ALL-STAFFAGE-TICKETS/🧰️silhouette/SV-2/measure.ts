/** 📏️ Every check of the catalog silhouette guard for plants, not only the first: bounding box per axis, IoU and largest boundary distance per authored scale and view, a `FAIL` mark on each one beyond its limit and the count in the last line.
 *  bun measure.ts <slug|file.json>… [--scales 50,100] */
import { parseArgs } from "node:util";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { VIEWPOINTS, bodyDimensions, measureAsset, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

import { fileOf } from "./measure-lib.ts";
const { values, positionals } = parseArgs({ allowPositionals: true, options: { scales: { type: "string" } } });
for (const arg of positionals) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  const scales = (values.scales ?? "500,200,100,50").split(",").map(Number).filter((d) => asset.volumes[String(d)]?.status === "authored");
  let bad = 0;
  for (const scale of scales) {
    const feature = featureOf(scale), t = Date.now();
    const m = asset.representations[String(scale)].status === "authored" ? measureAsset(asset, scale, undefined, undefined, feature) : undefined;
    const bbox = m?.bbox ?? bodyDimensions(worldSolids(asset, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }).bones as never, scale));
    const box = bbox.map((b: number, k: number) => { const off = Math.abs(b - dims[k]) > feature; if (off) bad++; return `${b.toFixed(4)}/${dims[k]}${off ? " FAIL" : ""}`; });
    console.log(`${asset.id} 1:${scale} bbox ${box.join("  ")}  (feature ${feature}, ${((Date.now() - t) / 1000).toFixed(0)} s)`);
    if (m !== undefined) for (const view of VIEWPOINTS) {
      const v = m.views[view], a = v.iou < 0.92, h = v.hausdorffM > 2 * feature;
      bad += Number(a) + Number(h);
      console.log(`${asset.id} 1:${scale} ${view.padEnd(9)} iou ${v.iou.toFixed(4)}${a ? " FAIL" : ""}  H ${v.hausdorffM.toFixed(4)}/${(2 * feature).toFixed(3)}${h ? " FAIL" : ""}`);
    }
  }
  console.log(`${asset.id}: ${bad} failing checks`);
}
