/** 🧊️ The catalog-silhouette gate for HW-12 assets: bbox against dimensions within one feature, IoU ≥ 0.92 and Hausdorff ≤ 2 features per view and authored scale.
 *  bun guard.ts <slug|file.json> [scale ...] */
import { featureOf, SCALE_DENOMINATORS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { measureAsset, VIEWPOINTS } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
const [target, ...scaleArgs] = process.argv.slice(2);
const file = target!.endsWith(".json") ? target! : `/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/${target}/🔣️.json`;
const asset = (await Bun.file(file).json()).asset;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let bad = 0;
for (const scale of scaleArgs.length ? scaleArgs.map(Number) : SCALE_DENOMINATORS) {
  if (asset.volumes[String(scale)]?.status !== "authored" || asset.representations[String(scale)]?.status !== "authored") continue;
  const feature = featureOf(scale);
  const t0 = performance.now();
  const m = measureAsset(asset, scale, undefined, undefined, feature);
  const bb = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}/${dims[k]}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`).join(" ");
  const views = VIEWPOINTS.map((v) => { const x = m.views[v]; const ok = x.iou >= 0.92 && x.hausdorffM <= 2 * feature; if (!ok) bad++; return `${v} ${x.iou.toFixed(3)} H${(x.hausdorffM * 1000).toFixed(0)}mm${ok ? "" : "!"}`; }).join(" | ");
  if (m.bbox.some((b: number, k: number) => Math.abs(b - dims[k]!) > feature)) bad++;
  console.log(`1:${scale} bbox ${bb} | ${views} (${((performance.now() - t0) / 1000).toFixed(1)} s)`);
}
console.log(bad ? `[DEBUG] guard failures ${bad}` : "guard ok");
