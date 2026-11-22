/** 🌓️ IoU and Hausdorff distance of drawing against volume per scale and view (the catalog-silhouette gate): bun sil.ts <slug|file> [pose] */
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { measureAsset, VIEWPOINTS } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { readAsset } from "./lib.ts";
const [src, pose] = process.argv.slice(2);
const asset = await readAsset(src!);
for (const scale of [50, 100, 200, 500]) {
  const f = featureOf(scale as 50);
  const m = measureAsset(asset, scale, pose ?? asset.presentation.defaultPose, undefined, f);
  const d = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
  console.log(`1:${scale} bbox ${m.bbox.map((b, k) => `${b.toFixed(3)}/${d[k]}${Math.abs(b - d[k]!) > f ? "!" : ""}`).join(" ")}  ` + VIEWPOINTS.map((v) => `${v} ${m.views[v].iou.toFixed(3)}${m.views[v].iou < 0.92 ? "!" : ""} h${m.views[v].hausdorffM.toFixed(3)}${m.views[v].hausdorffM > 2 * f ? "!" : ""}`).join("  "));
}
