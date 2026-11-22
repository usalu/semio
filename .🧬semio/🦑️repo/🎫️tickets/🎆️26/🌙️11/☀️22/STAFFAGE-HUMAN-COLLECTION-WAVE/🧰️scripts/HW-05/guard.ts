/** 🧊️ The catalog-silhouette gate of `🧪️tests/🧊️catalog-silhouette` for one asset file: bbox against dimensions within one feature, IoU ≥ 0.92 and Hausdorff ≤ 2 features per view and authored scale, in the default pose (or POSE=).
 *  bun guard.ts <asset.json> [scale ...] */
import { SCALE_DENOMINATORS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { VIEWPOINTS, measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

const asset = (await Bun.file(process.argv[2]!).json()).asset;
const only = process.argv.slice(3).map(Number);
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
let bad = 0;
for (const scale of SCALE_DENOMINATORS) {
  if (only.length && !only.includes(scale)) continue;
  if (asset.representations[`${scale}`].status !== "authored") continue;
  const feature = featureOf(scale);
  const m = measureAsset(asset, scale, process.env.POSE ?? asset.presentation.defaultPose, undefined, feature);
  const box = m.bbox.map((b: number, k: number) => `${b.toFixed(3)}${Math.abs(b - dims[k]!) > feature ? "!" : ""}`);
  if (m.bbox.some((b: number, k: number) => Math.abs(b - dims[k]!) > feature)) bad++;
  const views = VIEWPOINTS.map((v) => {
    const x = m.views[v];
    const fail = x.iou < 0.92 || x.hausdorffM > 2 * feature;
    if (fail) bad++;
    return `${v} iou ${x.iou.toFixed(3)} h ${(x.hausdorffM * 1000).toFixed(0)}mm${fail ? " FAIL" : ""}`;
  });
  console.log(`1:${scale} bbox ${box.join(" ")} | ${views.join(" | ")}`);
}
console.log(bad ? `[DEBUG] guard failing ${bad}` : "guard ok");
